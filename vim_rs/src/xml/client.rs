use std::borrow::Cow;
use std::time::Instant;

use bytes::Bytes;
use log::{debug, trace, warn};

use crate::core::client::WireLoggingMode;
use crate::core::wire_log;
use miniserde::ser::{Fragment, Map as SerMap, Serialize};

use crate::core::client::{BoxFuture, Error, PropertyValue, Result, Transport, VimClient};
use crate::types::enums::MoTypesEnum;
use crate::types::structs::{
    ManagedObjectReference, MethodFault, ObjectSpec, PropertyFilterSpec, PropertySpec,
    RetrieveOptions, ServiceContent,
};
use crate::types::vim_any::VimAny;
const SOAP_ACTION: &str = "urn:vim25/9.1.0.0";
const CONTENT_TYPE: &str = "text/xml; charset=utf-8";

// ============================================================================
// SOAP XML body builder
// ============================================================================

/// Build the XML body for a VIM SOAP method call.
///
/// Produces:
/// ```xml
/// <MethodName xmlns="urn:vim25" xsi:type="MethodNameRequestType">
///   <_this type="MoType">mo-id</_this>
///   <field1>...</field1>
///   ...
/// </MethodName>
/// ```
///
/// The `params` serializer (generated `*RequestType`) emits `_typeName` as its
/// first Map key. We extract it for the `xsi:type` attribute, inject the
/// `_this` element, then stream the remaining fields as XML children.
fn build_soap_method_xml(
    method_name: &str,
    mo_type: &str,
    mo_id: &str,
    params: Option<&(dyn Serialize + Send + Sync)>,
) -> String {
    let mut out = String::new();
    out.push('<');
    out.push_str(method_name);
    out.push_str(" xmlns=\"urn:vim25\"");

    if let Some(p) = params {
        if let Fragment::Map(mut map) = p.begin() {
            if let Some((key, val)) = map.next() {
                if &*key == "_typeName" {
                    if let Fragment::Str(s) = val.begin() {
                        out.push_str(" xsi:type=\"");
                        out.push_str(&s);
                        out.push('"');
                    }
                }
            }
            out.push('>');
            push_this(&mut out, mo_type, mo_id);
            while let Some((key, val)) = map.next() {
                super::ser::stream_xml(val, &key, &mut out);
            }
        } else {
            out.push('>');
            push_this(&mut out, mo_type, mo_id);
        }
    } else {
        out.push('>');
        push_this(&mut out, mo_type, mo_id);
    }

    out.push_str("</");
    out.push_str(method_name);
    out.push('>');
    out
}

fn push_this(out: &mut String, mo_type: &str, mo_id: &str) {
    out.push_str("<_this type=\"");
    out.push_str(mo_type);
    out.push_str("\">");
    out.push_str(mo_id);
    out.push_str("</_this>");
}

// ============================================================================
// SOAP fault extraction
// ============================================================================

fn extract_soap_fault(body: &str) -> Option<String> {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    let mut reader = Reader::from_str(body);
    let mut in_faultstring = false;
    let mut faultstring = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name_bytes = e.name();
                let name = std::str::from_utf8(name_bytes.as_ref()).unwrap_or("");
                if name == "faultstring" {
                    in_faultstring = true;
                }
            }
            Ok(Event::Text(e)) if in_faultstring => {
                if let Ok(s) = std::str::from_utf8(&e) {
                    faultstring.push_str(s);
                }
            }
            Ok(Event::End(_)) if in_faultstring => {
                return Some(faultstring);
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    None
}

/// Extract a typed [`MethodFault`] from a SOAP fault body by locating the
/// `<detail>` element and handing its first child element (with `xsi:type`)
/// to the generic XML deserializer.
///
/// Returns `None` if the body is not a SOAP fault, the `<detail>` element is
/// missing/empty, or the inner fault element fails to deserialize.
///
/// SOAP 1.1 puts the typed fault info inside `<detail>` (unqualified in the
/// SOAP namespace), e.g.:
///
/// ```xml
/// <soapenv:Fault>
///   <faultcode>ServerFaultCode</faultcode>
///   <faultstring></faultstring>
///   <detail>
///     <RequestCanceledFault xmlns="urn:vim25" xsi:type="RequestCanceled"/>
///   </detail>
/// </soapenv:Fault>
/// ```
///
/// The `<faultstring>` is often empty (notably on `vcsim`), so relying on it
/// to surface a typed fault is not viable. Reading `<detail>` makes
/// [`crate::core::client::is_request_canceled_error`] work for XML transport.
fn extract_soap_method_fault(body: &str) -> Option<MethodFault> {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    let mut reader = Reader::from_str(body);
    let mut in_detail = false;
    let mut inner_depth: i32 = 0;
    let mut inner_start: Option<usize> = None;
    let mut inner_end: Option<usize> = None;

    loop {
        let before = reader.buffer_position() as usize;
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let local = e.name().local_name();
                let local_str = std::str::from_utf8(local.as_ref()).unwrap_or("");
                if !in_detail {
                    if local_str == "detail" {
                        in_detail = true;
                    }
                } else {
                    if inner_start.is_none() {
                        inner_start = Some(before);
                    }
                    inner_depth += 1;
                }
            }
            Ok(Event::Empty(_)) if in_detail && inner_start.is_none() => {
                let after = reader.buffer_position() as usize;
                inner_start = Some(before);
                inner_end = Some(after);
                break;
            }
            Ok(Event::End(_)) if in_detail => {
                if inner_depth == 0 {
                    break;
                }
                inner_depth -= 1;
                if inner_depth == 0 {
                    inner_end = Some(reader.buffer_position() as usize);
                    break;
                }
            }
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
    }

    let (start, end) = (inner_start?, inner_end?);
    let slice = body.get(start..end)?.trim();
    if slice.is_empty() {
        return None;
    }
    super::de::from_xml::<MethodFault>(slice).ok()
}

/// Classify a SOAP HTTP error body into the most specific `Error` variant
/// that honestly describes the failure:
///
/// 1. Typed fault element inside `<detail>` → [`Error::MethodFault`].
///    This is a genuine vSphere API fault from the VIM hierarchy
///    (e.g. `RequestCanceled`, `InvalidLogin`, `VAppPropertyFault`), so
///    `type_` is preserved and matchers like
///    [`crate::core::client::is_request_canceled_error`] work.
/// 2. `<faultstring>` only, no typed `<detail>` child → [`Error::SoapFault`].
///    This is a SOAP 1.1 envelope-level fault (e.g. `VersionMismatch`,
///    `MustUnderstand`, front-end auth rejection before the API dispatcher
///    runs, or an infrastructure error). Synthesizing a [`MethodFault`]
///    here would lie about the value coming from the API.
/// 3. Neither recognisable → [`Error::ParseError`] — the body isn't a SOAP
///    fault we can interpret.
///
/// `context` is a short prefix used for the `ParseError` fallback so
/// operators can tell apart bootstrap failures from ordinary method calls.
fn soap_error_from_body(body: &str, status: reqwest::StatusCode, context: &str) -> Error {
    if let Some(fault) = extract_soap_method_fault(body) {
        return Error::MethodFault(fault);
    }
    if let Some(faultstring) = extract_soap_fault(body) {
        return Error::ParseError(faultstring);
    }
    Error::ParseError(format!(
        "HTTP {status} ({context}): no SOAP fault recognised"
    ))
}

// ============================================================================
// SoapClient
// ============================================================================

/// VIM SOAP/XML client. Uses cookie-based session management and communicates
/// with the `/sdk` endpoint.
pub(crate) struct SoapClient {
    http_client: reqwest::Client,
    endpoint: String,
    user_agent: String,
    api_release: String,
    service_content: Option<ServiceContent>,
    wire_logging: WireLoggingMode,
}

impl SoapClient {
    /// Create a new SoapClient. The `http_client` must be built with
    /// `.cookie_store(true)` for session management.
    pub(crate) fn new(
        http_client: reqwest::Client,
        server_address: &str,
        api_release: &str,
        user_agent: &str,
        wire_logging: WireLoggingMode,
    ) -> Self {
        Self {
            http_client,
            endpoint: format!("https://{server_address}/sdk"),
            user_agent: user_agent.to_string(),
            api_release: api_release.to_string(),
            service_content: None,
            wire_logging,
        }
    }

    /// Bootstrap the client by fetching ServiceContent via SOAP.
    /// This must be called before using the client through `VimClient` methods.
    pub(crate) async fn bootstrap(&mut self) -> Result<()> {
        let method_xml = build_soap_method_xml(
            "RetrieveServiceContent",
            "ServiceInstance",
            "ServiceInstance",
            None,
        );
        let soap_body = super::soap::envelope(&method_xml);
        let started = Instant::now();
        if self.wire_logging.is_enabled() {
            let mode_l = wire_log::wire_mode_label(self.wire_logging, "ServiceInstance");
            let msg = format!(
                "wire=soap mode={} phase=request mo=ServiceInstance id=ServiceInstance method=RetrieveServiceContent endpoint={} body_bytes={}",
                mode_l,
                self.endpoint,
                soap_body.len()
            );
            wire_log::log_soap_line(self.wire_logging, "ServiceInstance", false, &msg);
        }
        let resp = match self.soap_send_raw(&soap_body).await {
            Ok(r) => r,
            Err(e) => {
                if self.wire_logging.is_enabled() {
                    wire_log::log_soap_transport_failure(
                        self.wire_logging,
                        "ServiceInstance",
                        "ServiceInstance",
                        "RetrieveServiceContent",
                        started.elapsed(),
                        &e,
                    );
                }
                return Err(Error::ReqwestError(e));
            }
        };
        let status = resp.status();
        let body = resp.text().await.map_err(Error::ReqwestError)?;
        if self.wire_logging.is_enabled() {
            let mode_l = wire_log::wire_mode_label(self.wire_logging, "ServiceInstance");
            let mut msg = format!(
                "wire=soap mode={} phase=response mo=ServiceInstance id=ServiceInstance method=RetrieveServiceContent status={} body_bytes={} duration_ms={}",
                mode_l,
                status.as_u16(),
                body.len(),
                started.elapsed().as_millis()
            );
            if wire_log::bodies_allowed(self.wire_logging, "ServiceInstance") {
                msg.push_str(&format!(" body={}", body));
            }
            wire_log::log_soap_line(
                self.wire_logging,
                "ServiceInstance",
                wire_log::bodies_allowed(self.wire_logging, "ServiceInstance"),
                &msg,
            );
        }
        if !status.is_success() {
            let err = soap_error_from_body(&body, status, "bootstrap");
            if let Error::MethodFault(f) = &err {
                warn!("SOAP bootstrap fault: typed={:?}", f.type_);
            }
            return Err(err);
        }
        let sc: ServiceContent = super::soap::vim_response_internal(&body)
            .map_err(|_| Error::ParseError("Failed to parse ServiceContent".to_string()))?;
        debug!("SOAP ServiceContent obtained from: {}", sc.about.full_name);
        self.service_content = Some(sc);
        Ok(())
    }

    async fn soap_send_raw(
        &self,
        soap_body: &str,
    ) -> std::result::Result<reqwest::Response, reqwest::Error> {
        self.http_client
            .post(&self.endpoint)
            .header("Content-Type", CONTENT_TYPE)
            .header("SOAPAction", format!("urn:vim25/{}", self.api_release))
            .header("User-Agent", &self.user_agent)
            .body(soap_body.to_owned())
            .send()
            .await
    }

    async fn soap_invoke(
        &self,
        method_name: &str,
        mo_type: &str,
        mo_id: &str,
        params: Option<&(dyn Serialize + Send + Sync)>,
    ) -> Result<String> {
        let method_xml = build_soap_method_xml(method_name, mo_type, mo_id, params);
        let soap_body = super::soap::envelope(&method_xml);
        let started = Instant::now();
        if self.wire_logging.is_enabled() {
            let mode_l = wire_log::wire_mode_label(self.wire_logging, mo_type);
            let deny = wire_log::body_logging_note(self.wire_logging, mo_type);
            let deny_s = deny.unwrap_or("");
            let deny_sep = if deny_s.is_empty() { "" } else { " " };
            let mut msg = format!(
                "wire=soap mode={} phase=request mo={} id={} method={} endpoint={} body_bytes={}{}{}",
                mode_l,
                mo_type,
                mo_id,
                method_name,
                self.endpoint,
                soap_body.len(),
                deny_sep,
                deny_s
            );
            if wire_log::bodies_allowed(self.wire_logging, mo_type) {
                msg.push_str(&format!(" body={}", soap_body));
            }
            wire_log::log_soap_line(
                self.wire_logging,
                mo_type,
                wire_log::bodies_allowed(self.wire_logging, mo_type),
                &msg,
            );
        } else if log::log_enabled!(log::Level::Trace)
            && !wire_log::suppress_legacy_transport_trace(self.wire_logging)
        {
            trace!("SOAP request for {}: {}", method_name, &soap_body);
        }
        let resp = match self.soap_send_raw(&soap_body).await {
            Ok(r) => r,
            Err(e) => {
                if self.wire_logging.is_enabled() {
                    wire_log::log_soap_transport_failure(
                        self.wire_logging,
                        mo_type,
                        mo_id,
                        method_name,
                        started.elapsed(),
                        &e,
                    );
                }
                return Err(Error::ReqwestError(e));
            }
        };
        let status = resp.status();
        let body = resp.text().await.map_err(Error::ReqwestError)?;
        if self.wire_logging.is_enabled() {
            let mode_l = wire_log::wire_mode_label(self.wire_logging, mo_type);
            let deny = wire_log::body_logging_note(self.wire_logging, mo_type);
            let deny_s = deny.unwrap_or("");
            let deny_sep = if deny_s.is_empty() { "" } else { " " };
            let mut msg = format!(
                "wire=soap mode={} phase=response mo={} id={} method={} status={} body_bytes={} duration_ms={}{}{}",
                mode_l,
                mo_type,
                mo_id,
                method_name,
                status.as_u16(),
                body.len(),
                started.elapsed().as_millis(),
                deny_sep,
                deny_s
            );
            if wire_log::bodies_allowed(self.wire_logging, mo_type) {
                msg.push_str(&format!(" body={}", body));
            }
            wire_log::log_soap_line(
                self.wire_logging,
                mo_type,
                wire_log::bodies_allowed(self.wire_logging, mo_type),
                &msg,
            );
        } else if log::log_enabled!(log::Level::Trace)
            && !wire_log::suppress_legacy_transport_trace(self.wire_logging)
        {
            trace!("SOAP response {} from {}: {}", status, method_name, &body);
        }
        if !status.is_success() {
            let err = soap_error_from_body(&body, status, method_name);
            match &err {
                Error::MethodFault(f) => {
                    warn!("SOAP fault from {}: typed={:?}", method_name, f.type_);
                }
                Error::ParseError(s) => {
                    warn!(
                        "Parse Error or SOAP envelope fault from {}: {}",
                        method_name, s
                    );
                }
                _ => {}
            }
            return Err(err);
        }
        Ok(body)
    }

    fn pc_mor(&self) -> Result<&ManagedObjectReference> {
        self.service_content
            .as_ref()
            .map(|sc| &sc.property_collector)
            .ok_or_else(|| Error::ParseError("PropertyCollector MOR not available".to_string()))
    }

    async fn fetch_property_via_pc(
        &self,
        mo_type: &str,
        mo_id: &str,
        property: &str,
    ) -> Result<Option<VimAny>> {
        let pc = self.pc_mor()?.clone();
        let obj = ManagedObjectReference {
            r#type: MoTypesEnum::from_str(mo_type),
            value: mo_id.to_string(),
        };
        let spec = PropertyFilterSpec {
            prop_set: vec![PropertySpec {
                all: Some(false),
                path_set: Some(vec![property.to_string()]),
                r#type: mo_type.to_string(),
            }],
            object_set: vec![ObjectSpec {
                obj,
                skip: Some(false),
                select_set: None,
            }],
            report_missing_objects_in_results: None,
        };
        let options = RetrieveOptions {
            max_objects: Some(1),
        };
        let req = FetchPropertyRequest {
            spec_set: &[spec],
            options: &options,
        };
        let body = self
            .soap_invoke(
                "RetrievePropertiesEx",
                "PropertyCollector",
                &pc.value,
                Some(&req),
            )
            .await?;

        let result: crate::types::structs::RetrieveResult =
            super::soap::vim_response_internal(&body)
                .map_err(|_| Error::ParseError("Failed to parse RetrieveResult".to_string()))?;

        if result.objects.is_empty() {
            return Ok(None);
        }
        let obj_content = result.objects.into_iter().next().unwrap();
        let prop_set = match obj_content.prop_set {
            Some(ps) if !ps.is_empty() => ps,
            _ => return Ok(None),
        };
        let val = prop_set.into_iter().next().unwrap().val;
        Ok(Some(val))
    }
}

impl VimClient for SoapClient {
    fn service_content(&self) -> &ServiceContent {
        self.service_content
            .as_ref()
            .expect("ServiceContent not initialized")
    }

    fn transport(&self) -> Transport {
        Transport::Soap
    }

    fn api_release(&self) -> String {
        self.api_release.clone()
    }

    fn invoke<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Bytes>> {
        Box::pin(async move {
            if !svc.is_empty() {
                return Err(Error::ParseError(format!(
                    "Service name not supported for SOAP: {}",
                    svc
                )));
            }
            let body = self
                .soap_invoke(method_name, mo_type, mo_id, params)
                .await?;
            Ok(Bytes::from(body))
        })
    }

    fn invoke_optional<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Option<Bytes>>> {
        Box::pin(async move {
            if !svc.is_empty() {
                return Err(Error::ParseError(format!(
                    "Service name not supported for SOAP: {}",
                    svc
                )));
            }
            let body = self
                .soap_invoke(method_name, mo_type, mo_id, params)
                .await?;
            if super::soap::vim_response_is_empty(&body)
                .map_err(|_| Error::ParseError("Failed to check SOAP response".to_string()))?
            {
                Ok(None)
            } else {
                Ok(Some(Bytes::from(body)))
            }
        })
    }

    fn invoke_void<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            if !svc.is_empty() {
                return Err(Error::ParseError(format!(
                    "Service name not supported for SOAP: {}",
                    svc
                )));
            }
            let _ = self
                .soap_invoke(method_name, mo_type, mo_id, params)
                .await?;
            Ok(())
        })
    }

    fn fetch_property_raw<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        property: &'a str,
    ) -> BoxFuture<'a, Result<Option<PropertyValue>>> {
        Box::pin(async move {
            if !svc.is_empty() {
                return Err(Error::ParseError(format!(
                    "Service name not supported for SOAP: {}",
                    svc
                )));
            }
            let maybe = self.fetch_property_via_pc(mo_type, mo_id, property).await?;
            Ok(maybe.map(PropertyValue::Parsed))
        })
    }
}

impl Drop for SoapClient {
    fn drop(&mut self) {
        debug!("Disposing SOAP client.");
        let Some(ref sc) = self.service_content else {
            return;
        };
        let Some(ref sm) = sc.session_manager else {
            debug!("No session manager. Skipping logout.");
            return;
        };
        let sm_value = sm.value.clone();
        let http_client = self.http_client.clone();
        let endpoint = self.endpoint.clone();
        let user_agent = self.user_agent.clone();
        let wire_logging = self.wire_logging;

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                let method_xml = build_soap_method_xml("Logout", "SessionManager", &sm_value, None);
                let soap_body = super::soap::envelope(&method_xml);
                if wire_logging.is_enabled() {
                    let mode_l = wire_log::wire_mode_label(wire_logging, "SessionManager");
                    let msg = format!(
                        "wire=soap mode={} phase=request kind=logout mo=SessionManager id={} method=Logout endpoint={} body_bytes={} body_logging=denylisted",
                        mode_l,
                        sm_value,
                        endpoint,
                        soap_body.len()
                    );
                    wire_log::log_soap_line(wire_logging, "SessionManager", false, &msg);
                }
                let started = Instant::now();
                match http_client
                    .post(&endpoint)
                    .header("Content-Type", CONTENT_TYPE)
                    .header("SOAPAction", SOAP_ACTION)
                    .header("User-Agent", &user_agent)
                    .body(soap_body)
                    .send()
                    .await
                {
                    Ok(resp) => {
                        let status = resp.status();
                        let body = resp.text().await.unwrap_or_default();
                        let dur = started.elapsed();
                        if wire_logging.is_enabled() {
                            let mode_l = wire_log::wire_mode_label(wire_logging, "SessionManager");
                            let http_note = if status.is_success() {
                                ""
                            } else {
                                " error=http_failure"
                            };
                            let msg = format!(
                                "wire=soap mode={} phase=response kind=logout mo=SessionManager id={} method=Logout status={} body_bytes={} duration_ms={} body_logging=denylisted{}",
                                mode_l,
                                sm_value,
                                status.as_u16(),
                                body.len(),
                                dur.as_millis(),
                                http_note
                            );
                            wire_log::log_soap_line(wire_logging, "SessionManager", false, &msg);
                        }
                        if status.is_success() {
                            debug!("SOAP session logged out successfully");
                        } else {
                            warn!("SOAP logout failed (HTTP {})", status);
                        }
                    }
                    Err(e) => {
                        if wire_logging.is_enabled() {
                            let mode_l = wire_log::wire_mode_label(wire_logging, "SessionManager");
                            let msg = format!(
                                "wire=soap mode={} phase=response kind=logout mo=SessionManager id={} method=Logout error=transport duration_ms={} body_logging=denylisted detail={}",
                                mode_l,
                                sm_value,
                                started.elapsed().as_millis(),
                                e
                            );
                            wire_log::log_soap_line(wire_logging, "SessionManager", false, &msg);
                        }
                        warn!("SOAP logout request failed: {}", e);
                    }
                }
            });
        });
    }
}

// ============================================================================
// Helper: RetrievePropertiesEx request serializer for fetch_property
// ============================================================================

/// Minimal serializer for the RetrievePropertiesEx body fields.
/// The SOAP wrapper (`@xmlns`, `_typeName`, `_this`) is added by
/// `build_soap_method_xml`.
struct FetchPropertyRequest<'a> {
    spec_set: &'a [PropertyFilterSpec],
    options: &'a RetrieveOptions,
}

impl<'a> Serialize for FetchPropertyRequest<'a> {
    fn begin(&self) -> Fragment<'_> {
        Fragment::Map(Box::new(FetchPropertyRequestMap { data: self, seq: 0 }))
    }
}

struct FetchPropertyRequestMap<'a> {
    data: &'a FetchPropertyRequest<'a>,
    seq: usize,
}

impl<'a> SerMap for FetchPropertyRequestMap<'a> {
    fn next(&mut self) -> Option<(Cow<'_, str>, &dyn Serialize)> {
        let s = self.seq;
        self.seq += 1;
        match s {
            0 => Some((
                Cow::Borrowed("_typeName"),
                &"RetrievePropertiesExRequestType" as &dyn Serialize,
            )),
            1 => Some((
                Cow::Borrowed("specSet"),
                &self.data.spec_set as &dyn Serialize,
            )),
            2 => Some((
                Cow::Borrowed("options"),
                &self.data.options as &dyn Serialize,
            )),
            _ => None,
        }
    }
}

#[cfg(all(test, feature = "xml"))]
pub(crate) fn soap_test_client_with_service_content() -> SoapClient {
    use crate::core::client::{test_dead_port_http_client, test_minimal_service_content_for_tests};
    use crate::core::client::{WireLoggingMode, API_RELEASE, TEST_WIRE_DEAD_ADDR};
    let mut c = SoapClient::new(
        test_dead_port_http_client(),
        TEST_WIRE_DEAD_ADDR,
        API_RELEASE,
        "wire-soap-transport-test",
        WireLoggingMode::Summary,
    );
    c.service_content = Some(test_minimal_service_content_for_tests());
    c
}

/// SOAP client with `service_content` + `sessionManager` and a caller-supplied endpoint (for logout `Drop` tests).
#[cfg(all(test, feature = "xml"))]
pub(crate) fn soap_test_client_for_logout_drop(
    http_client: reqwest::Client,
    endpoint: String,
) -> SoapClient {
    use crate::core::client::WireLoggingMode;
    SoapClient {
        http_client,
        endpoint,
        user_agent: "soap-logout-wire-test".to_string(),
        api_release: crate::core::client::API_RELEASE.to_string(),
        service_content: Some(
            crate::core::client::test_service_content_with_session_manager_for_tests(),
        ),
        wire_logging: WireLoggingMode::Summary,
    }
}

#[cfg(all(test, feature = "xml"))]
mod fault_extraction_tests {
    use super::{extract_soap_fault, extract_soap_method_fault};
    use crate::types::struct_enum::StructType;

    /// Real `vcsim` payload for an interrupted `WaitForUpdatesEx`: empty
    /// `faultstring`, typed `RequestCanceled` in `<detail>`.
    const VCSIM_REQUEST_CANCELED: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenc="http://schemas.xmlsoap.org/soap/encoding/" xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"><soapenv:Body><soapenv:Fault><faultcode>ServerFaultCode</faultcode><faultstring></faultstring><detail><RequestCanceledFault xmlns="urn:vim25" xsi:type="RequestCanceled"></RequestCanceledFault></detail></soapenv:Fault></soapenv:Body></soapenv:Envelope>"#;

    #[test]
    fn extracts_request_canceled_from_vcsim_fault() {
        let fault = extract_soap_method_fault(VCSIM_REQUEST_CANCELED)
            .expect("detail/RequestCanceled must parse as MethodFault");
        assert_eq!(fault.type_, Some(StructType::RequestCanceled));
    }

    #[test]
    fn extracts_request_canceled_from_self_closing_detail() {
        let body = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"><soapenv:Body><soapenv:Fault><faultcode>ServerFaultCode</faultcode><faultstring/><detail><RequestCanceledFault xmlns="urn:vim25" xsi:type="RequestCanceled"/></detail></soapenv:Fault></soapenv:Body></soapenv:Envelope>"#;
        let fault = extract_soap_method_fault(body).expect("self-closing fault must parse");
        assert_eq!(fault.type_, Some(StructType::RequestCanceled));
    }

    #[test]
    fn returns_none_when_detail_missing() {
        let body = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/"><soapenv:Body><soapenv:Fault><faultcode>ServerFaultCode</faultcode><faultstring>boom</faultstring></soapenv:Fault></soapenv:Body></soapenv:Envelope>"#;
        assert!(extract_soap_method_fault(body).is_none());
        assert_eq!(extract_soap_fault(body).as_deref(), Some("boom"));
    }

    #[test]
    fn returns_none_when_detail_empty() {
        let body = r#"<soapenv:Envelope xmlns:soapenv="x"><soapenv:Body><soapenv:Fault><faultcode>ServerFaultCode</faultcode><faultstring></faultstring><detail></detail></soapenv:Fault></soapenv:Body></soapenv:Envelope>"#;
        assert!(extract_soap_method_fault(body).is_none());
    }

    /// Nested fault inside `<detail>` with children must slice the outer
    /// element correctly so the inner types survive deserialization.
    #[test]
    fn extracts_typed_fault_with_children() {
        let body = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"><soapenv:Body><soapenv:Fault><faultcode>ServerFaultCode</faultcode><faultstring>missing</faultstring><detail><VAppPropertyFaultFault xmlns="urn:vim25" xsi:type="VAppPropertyFault"><id>config.product.version</id><category>string</category><label>Product Version</label><type>string</type><value>1.0.0</value></VAppPropertyFaultFault></detail></soapenv:Fault></soapenv:Body></soapenv:Envelope>"#;
        let fault = extract_soap_method_fault(body).expect("typed VAppPropertyFault must parse");
        assert_eq!(fault.type_, Some(StructType::VAppPropertyFault));
    }

    /// Faultstring-only body (no typed `<detail>`): SOAP envelope-level
    /// fault, **not** a vSphere API `MethodFault`. Must surface as
    /// [`Error::SoapFault`] so callers matching on `MethodFault` don't
    /// see synthesized values, and `is_request_canceled_error` stays
    /// strictly scoped to the typed VIM hierarchy.
    #[test]
    fn soap_error_faultstring_only_maps_to_soap_envelope_fault() {
        use super::soap_error_from_body;
        use crate::core::client::{is_request_canceled_error, Error};

        let body = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/"><soapenv:Body><soapenv:Fault><faultcode>ServerFaultCode</faultcode><faultstring>invalid argument</faultstring></soapenv:Fault></soapenv:Body></soapenv:Envelope>"#;
        let err = soap_error_from_body(body, reqwest::StatusCode::INTERNAL_SERVER_ERROR, "test");
        match &err {
            Error::ParseError(s) => assert_eq!(s, "invalid argument"),
            other => panic!("expected SoapFault, got {:?}", other),
        }
        assert!(
            !is_request_canceled_error(&err),
            "envelope-level fault must not match typed VIM fault predicates"
        );
    }

    /// Body with neither `<faultstring>` nor `<detail>` → `ParseError`.
    /// That's the one path where we genuinely cannot characterise the
    /// failure as a method fault.
    #[test]
    fn soap_error_unrecognised_body_maps_to_parse_error() {
        use super::soap_error_from_body;
        use crate::core::client::Error;

        let body = "<html><body>Bad Gateway</body></html>";
        let err = soap_error_from_body(body, reqwest::StatusCode::BAD_GATEWAY, "test");
        assert!(matches!(err, Error::ParseError(_)));
    }

    /// Typed fault still yields `MethodFault` with preserved discriminator —
    /// makes `is_request_canceled_error` work through the unified entry.
    #[test]
    fn soap_error_typed_detail_preserves_type() {
        use super::soap_error_from_body;
        use crate::core::client::{is_request_canceled_error, Error};

        let err = soap_error_from_body(
            VCSIM_REQUEST_CANCELED,
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            "WaitForUpdatesEx",
        );
        match &err {
            Error::MethodFault(f) => {
                assert_eq!(f.type_, Some(StructType::RequestCanceled));
            }
            other => panic!("expected MethodFault(RequestCanceled), got {:?}", other),
        }
        assert!(is_request_canceled_error(&err));
    }
}
