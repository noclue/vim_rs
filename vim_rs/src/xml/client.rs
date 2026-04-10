use std::borrow::Cow;
use std::time::Instant;

use bytes::Bytes;
use log::{debug, trace, warn};

use crate::core::client::WireLoggingMode;
use crate::core::wire_log;
use miniserde::ser::{Fragment, Map as SerMap, Serialize};

use crate::core::client::{BoxFuture, Error, PropertyValue, Result, Transport, VimClient};
use crate::types::vim_any::VimAny;
use crate::types::enums::MoTypesEnum;
use crate::types::structs::{
    ManagedObjectReference, ObjectSpec, PropertyFilterSpec, PropertySpec, RetrieveOptions,
    ServiceContent,
};
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
            let fault_msg = extract_soap_fault(&body).unwrap_or_else(|| format!("HTTP {status}"));
            return Err(Error::ParseError(format!(
                "SOAP fault (bootstrap): {fault_msg}"
            )));
        }
        let sc: ServiceContent = super::soap::vim_response(&body)
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
            let fault_msg = extract_soap_fault(&body).unwrap_or_else(|| format!("HTTP {status}"));
            warn!("SOAP fault from {}: {}", method_name, fault_msg);
            return Err(Error::ParseError(format!("SOAP fault: {fault_msg}")));
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
        let body = self.soap_invoke("RetrievePropertiesEx", "PropertyCollector", &pc.value, Some(&req)).await?;

        let result: crate::types::structs::RetrieveResult = super::soap::vim_response(&body)
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
                return Err(Error::ParseError(format!("Service name not supported for SOAP: {}", svc)));
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
                return Err(Error::ParseError(format!("Service name not supported for SOAP: {}", svc)));
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
                return Err(Error::ParseError(format!("Service name not supported for SOAP: {}", svc)));
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
                return Err(Error::ParseError(format!("Service name not supported for SOAP: {}", svc)));
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
    use crate::core::client::{WireLoggingMode, API_RELEASE, TEST_WIRE_DEAD_ADDR};
    use crate::core::client::{test_dead_port_http_client, test_minimal_service_content_for_tests};
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

