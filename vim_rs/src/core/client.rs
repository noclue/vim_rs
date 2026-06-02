use std::sync::Arc;
use std::time::Instant;

use tokio::sync::RwLock;
use super::super::types::structs;
use super::wire_log;
use log::{warn, debug, trace};

use bytes::Bytes;
use std::future::Future;
use std::pin::Pin;
use std::ffi::OsStr;
use crate::mo;
use crate::types::struct_enum::StructType;
use crate::types::structs::{ManagedObjectReference, ServiceContent};

const LIB_NAME: &str = env!("CARGO_PKG_NAME");
const LIB_VERSION: &str = env!("CARGO_PKG_VERSION");
// See build.rs for the RUSTC_VERSION
const RUSTC_VERSION: &str = env!("RUSTC_VERSION");

/// Compatible API releases i.e. current and older API releases that can be negotiated with a server
pub const COMPATIBLE_API_RELEASES: [&str; 4] = ["9.0.0.0", "8.0.3.0", "8.0.2.0", "8.0.1.0"];

/// The default API version found in the OpenAPI specification
pub const API_RELEASE: &str = "9.0.0.0";

/// The header key for the session key
const AUTHN_HEADER: &str = "vmware-api-session-id";

const SERVICE_INSTANCE_MOID: &str = "ServiceInstance";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A typed vSphere API fault returned by the server.
    ///
    /// For SOAP transport this is produced *only* when the server's
    /// `<soapenv:Fault>` carries a typed element inside `<detail>`
    /// (e.g. `RequestCanceled`, `InvalidLogin`, `VAppPropertyFault`). For
    /// JSON transport this is produced when the error body deserializes as
    /// a `MethodFault`. Envelope-level SOAP errors that are *not* part of
    /// the vSphere type hierarchy are surfaced as [`Error::SoapFault`].
    #[error("MethodFault: {0:?}")]
    MethodFault(structs::MethodFault),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    /// A parse error returned by the server.
    ///
    /// This is produced when the server's response is not valid XML or JSON.
    /// This is also produced when the server's response is a SOAP envelope fault.
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Missing or Invalid session key")]
    MissingOrInvalidSessionKey,
    #[error("Invalid object type {0} expected: {1}")]
    InvalidObjectType(String, String),
    #[error("Cannot negotiate compatible API release. Attempted with: {0:?}")]
    CannotNegotiateAPIRelease(Vec<String>),
}

/// Convenience alias used in generated MO stubs for error construction.
pub type VimError = Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Returns `true` if `err` is a `RequestCanceled` SOAP fault (for example after
/// [`crate::mo::PropertyCollector::cancel_wait_for_updates`]).
#[must_use]
pub fn is_request_canceled_error(err: &Error) -> bool {
    matches!(
        err,
        Error::MethodFault(f) if f.type_ == Some(StructType::RequestCanceled)
    )
}

#[cfg(test)]
mod is_request_canceled_tests {
    use super::{is_request_canceled_error, Error};
    use crate::types::struct_enum::StructType;
    use crate::types::structs::MethodFault;

    #[test]
    fn detects_request_canceled_fault() {
        let err = Error::MethodFault(MethodFault {
            fault_cause: None,
            fault_message: None,
            type_: Some(StructType::RequestCanceled),
            extra_fields_: Default::default(),
        });
        assert!(is_request_canceled_error(&err));
    }

    #[test]
    fn rejects_non_fault_errors() {
        assert!(!is_request_canceled_error(&Error::ParseError("x".into())));
    }
}

/// A boxed future used by the object-safe `VimClient` trait.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Transport protocol used by a `VimClient` instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Json,
    #[cfg(feature = "xml")]
    Soap,
}

/// Wire representation of a managed-object property value before it is narrowed to a concrete type.
///
/// JSON transport returns raw bytes from the VI JSON API. SOAP transport returns the value already
/// parsed as [`crate::types::vim_any::VimAny`] from `RetrievePropertiesEx` (no re-serialization).
#[derive(Debug)]
pub enum PropertyValue {
    Json(Bytes),
    #[cfg(feature = "xml")]
    Parsed(crate::types::vim_any::VimAny),
}

/// Narrow a [`PropertyValue`] to `T`: JSON deserializes once; SOAP downcasts the in-memory `VimAny`.
pub fn extract_property<T: miniserde::Deserialize + 'static>(pv: PropertyValue) -> Result<T> {
    match pv {
        PropertyValue::Json(bytes) => {
            let text = std::str::from_utf8(&bytes)
                .map_err(|e| Error::ParseError(e.to_string()))?;
            miniserde::json::from_str(text).map_err(|_| {
                Error::ParseError(format!(
                    "JSON property decode failed for {}",
                    std::any::type_name::<T>()
                ))
            })
        }
        #[cfg(feature = "xml")]
        PropertyValue::Parsed(any) => any
            .into_any()
            .downcast::<T>()
            .map(|b| *b)
            .map_err(|_| {
                Error::ParseError(format!(
                    "SOAP property type mismatch for {}",
                    std::any::type_name::<T>()
                ))
            }),
    }
}

/// Object-safe client abstraction for generated managed-object stubs (`crate::mo::*`).
///
/// The trait exposes high-level operations that are transport-agnostic.
/// Generated MO proxies use [`extract_property`] for GET properties and the free [`unmarshal`] /
/// [`unmarshal_array`] functions for POST method bodies, keyed on [`Self::transport`].
pub trait VimClient: Send + Sync {
    /// Access vSphere `ServiceContent` (root managed object references).
    fn service_content(&self) -> &ServiceContent;

    /// The transport protocol this client uses.
    fn transport(&self) -> Transport;

    /// API version string (negotiated release for JSON, about.api_version for SOAP).
    fn api_release(&self) -> String;

    /// Invoke a method and return the response body bytes.
    fn invoke<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Bytes>>;

    /// Invoke a method that may return an empty body (`None`).
    fn invoke_optional<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Option<Bytes>>>;

    /// Invoke a void method (no response body expected).
    fn invoke_void<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<()>>;

    /// Fetch a single property by name (or `None` if empty).
    ///
    /// Managed-object stubs call [`extract_property`] on the result. Application code typically uses
    /// [`Client::fetch_property`] on the public facade instead.
    fn fetch_property_raw<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        property: &'a str,
    ) -> BoxFuture<'a, Result<Option<PropertyValue>>>;
}

/// Deserialize response bytes according to the transport format.
///
/// This is a free function (not a trait method) so that it can be generic on `T`
/// without breaking object-safety of `VimClient`.
#[cfg(not(feature = "xml"))]
pub fn unmarshal<T: miniserde::Deserialize>(transport: Transport, bytes: &[u8]) -> Result<T> {
    let text = std::str::from_utf8(bytes)
        .map_err(|e| Error::ParseError(e.to_string()))?;
    match transport {
        Transport::Json => miniserde::json::from_str(text)
            .map_err(|_| Error::ParseError(format!(
                "JSON deserialization failed for {}", std::any::type_name::<T>()))),
    }
}

/// Deserialize response bytes according to the transport format.
///
/// With the `xml` feature, SOAP/XML unmarshalling requires [`crate::types::data_type_aware::DataTypeAware`]
/// at the root type (same contract as [`crate::xml::de::from_xml`]).
#[cfg(feature = "xml")]
pub fn unmarshal<T>(transport: Transport, bytes: &[u8]) -> Result<T>
where
    T: miniserde::Deserialize + crate::types::data_type_aware::DataTypeAware,
{
    let text = std::str::from_utf8(bytes)
        .map_err(|e| Error::ParseError(e.to_string()))?;
    match transport {
        Transport::Json => miniserde::json::from_str(text)
            .map_err(|_| Error::ParseError(format!(
                "JSON deserialization failed for {}", std::any::type_name::<T>()))),
        Transport::Soap => {
            crate::xml::soap::vim_response_internal(text)
                .or_else(|_| crate::xml::de::from_xml_internal(text))
                .map_err(|_| Error::ParseError(format!(
                    "XML deserialization failed for {}", std::any::type_name::<T>())))
        },
    }
}

/// Deserialize results encoded as a **list of sibling** `<returnval>` elements per item (SOAP) or
/// the equivalent JSON array (JSON).
///
/// vSphere uses this shape for paginated reads such as *ReadNextEvents*, *ReadPreviousEvents*,
/// *QueryEvents*, *ReadNextTasks*, *ReadPreviousTasks*, and *ReadNextTasksByViewSpec*. For those
/// APIs, do **not** use [`unmarshal`] with `Vec<…>` on SOAP: [`unmarshal`] only drives the first
/// `<returnval>` via [`crate::xml::soap::vim_response`].
#[cfg(not(feature = "xml"))]
pub fn unmarshal_array<U: miniserde::Deserialize>(
    transport: Transport,
    bytes: &[u8],
) -> Result<Vec<U>> {
    let text = std::str::from_utf8(bytes)
        .map_err(|e| Error::ParseError(e.to_string()))?;
    match transport {
        Transport::Json => miniserde::json::from_str(text).map_err(|_| {
            Error::ParseError(format!(
                "JSON deserialization failed for Vec<{}>",
                std::any::type_name::<U>()
            ))
        }),
    }
}

#[cfg(feature = "xml")]
pub fn unmarshal_array<U>(transport: Transport, bytes: &[u8]) -> Result<Vec<U>>
where
    U: miniserde::Deserialize + crate::types::data_type_aware::DataTypeAware,
{
    let text = std::str::from_utf8(bytes)
        .map_err(|e| Error::ParseError(e.to_string()))?;
    match transport {
        Transport::Json => miniserde::json::from_str(text).map_err(|_| {
            Error::ParseError(format!(
                "JSON deserialization failed for Vec<{}>",
                std::any::type_name::<U>()
            ))
        }),
        Transport::Soap => crate::xml::soap::vim_response_list_internal(text).map_err(|_| {
            Error::ParseError(format!(
                "XML deserialization failed for Vec<{}>",
                std::any::type_name::<U>()
            ))
        }),
    }
}

/// Convenience handle type used by generated bindings.
pub type VimClientHandle = Arc<dyn VimClient>;

/// Transport selection for [`ClientBuilder::build`].
///
/// Requires the `xml` feature for `Soap` and `Auto` variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransportMode {
    /// Use the VI JSON API (vCenter only). This is the default.
    #[default]
    Json,
    /// Use SOAP/XML transport (works on both ESXi and vCenter).
    #[cfg(feature = "xml")]
    Soap,
    /// Auto-detect: try the Hello System API (JSON). If it fails, fall back to SOAP.
    #[cfg(feature = "xml")]
    Auto,
}

/// Explicit wire capture mode for VI JSON / SOAP traffic.
///
/// Emits on log targets `vim_rs::wire::json` and `vim_rs::wire::soap`. Summary metadata uses
/// [`log::Level::Debug`]; full request/response bodies use [`log::Level::Trace`] when
/// [`WireLoggingMode::Detailed`] applies and the managed object type is not denylisted.
///
/// **Denylist:** traffic for managed object type `SessionManager` (login, logout, session APIs)
/// never logs bodies, even in detailed mode; emitted lines use summary-style fields and may include
/// `body_logging=denylisted`.
///
/// When any wire mode other than [`WireLoggingMode::Off`] is active, legacy transport `trace!` lines
/// that duplicate full request/response dumps are suppressed for those paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WireLoggingMode {
    /// Default: no dedicated wire log lines; existing library `debug!` / `trace!` behavior unchanged.
    #[default]
    Off,
    /// Request/response metadata only (sizes, paths, status, duration) at [`log::Level::Debug`].
    Summary,
    /// Adds full bodies at [`log::Level::Trace`] where allowed (`SessionManager` remains summary-only).
    Detailed,
}

impl WireLoggingMode {
    /// `true` when dedicated wire logs may be emitted (Summary or Detailed).
    pub const fn is_enabled(self) -> bool {
        !matches!(self, Self::Off)
    }
    /// `true` when full bodies may be logged at [`log::Level::Trace`] (subject to denylist).
    pub const fn is_detailed(self) -> bool {
        matches!(self, Self::Detailed)
    }
}

pub struct ClientBuilder {
    server_address: String,
    compatible_api_releases: Option<Vec<String>>,
    api_release: Option<String>,
    http_client: Option<reqwest::Client>,
    #[cfg(feature = "default-client")]
    insecure: Option<bool>,
    app_name: Option<String>,
    app_version: Option<String>,
    user_name: Option<String>,
    password: Option<String>,
    locale: Option<String>,
    transport_mode: TransportMode,
    wire_logging: WireLoggingMode,
}

/// Build a `reqwest::Client` when the `default-client` feature is enabled (turnkey path).
#[cfg(feature = "default-client")]
fn build_default_http_client(
    insecure: Option<bool>,
    _cookie_store: bool,
) -> Result<reqwest::Client> {
    let mut builder = reqwest::Client::builder();
    #[cfg(feature = "xml")]
    if _cookie_store {
        builder = builder.cookie_store(true);
    }
    if let Some(insecure) = insecure {
        builder = builder
            .danger_accept_invalid_certs(insecure)
            .danger_accept_invalid_hostnames(insecure);
    }
    Ok(builder.build()?)
}

#[cfg(feature = "default-client")]
fn obtain_http_client(
    existing: Option<reqwest::Client>,
    insecure: Option<bool>,
    cookie_store: bool,
) -> Result<reqwest::Client> {
    match existing {
        Some(client) => Ok(client),
        None => build_default_http_client(insecure, cookie_store),
    }
}

#[cfg(not(feature = "default-client"))]
fn obtain_http_client(existing: Option<reqwest::Client>) -> Result<reqwest::Client> {
    existing.ok_or_else(|| {
        Error::ParseError(
            "ClientBuilder requires a reqwest::Client from new(server, http_client)".into(),
        )
    })
}

impl ClientBuilder {
    fn new_common(server_address: &str, http_client: Option<reqwest::Client>) -> Self {
        Self {
            server_address: server_address.to_string(),
            compatible_api_releases: None,
            api_release: None,
            http_client,
            #[cfg(feature = "default-client")]
            insecure: None,
            app_name: None,
            app_version: None,
            user_name: None,
            password: None,
            locale: None,
            transport_mode: TransportMode::default(),
            wire_logging: WireLoggingMode::default(),
        }
    }
}

#[cfg(feature = "default-client")]
impl ClientBuilder {
    /// Create a client builder for a VI/JSON (or SOAP with `xml`) API at the given FQDN or IP.
    ///
    /// When the `default-client` feature is enabled (default), a `reqwest::Client` is created
    /// automatically at [`Self::build`] unless one is supplied via [`Self::http_client`].
    pub fn new(server_address: &str) -> Self {
        Self::new_common(server_address, None)
    }

    /// Set the reqwest::Client instance to use for HTTP requests.
    ///
    /// Resets the [`Self::insecure`] flag; configure TLS on the supplied client instead.
    pub fn http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self.insecure = None;
        self
    }

    /// Allow invalid TLS certificates and hostnames on an auto-created client.
    ///
    /// Resets a previously set [`Self::http_client`]; a new client is built at [`Self::build`].
    pub fn insecure(mut self, insecure: bool) -> Self {
        warn!("!!! WARNING !!! Insecure mode enabled. TLS certificate and hostname verification is disabled. !!! WARNING !!!");
        self.insecure = Some(insecure);
        self.http_client = None;
        self
    }
}

#[cfg(not(feature = "default-client"))]
impl ClientBuilder {
    /// Create a client builder with a caller-supplied `reqwest::Client`.
    ///
    /// Requires `default-features = false` on the `vim_rs` dependency (disables `default-client`).
    /// Configure TLS, proxies, and (for SOAP) [`reqwest::ClientBuilder::cookie_store`] on your
    /// client before passing it here.
    pub fn new(server_address: &str, http_client: reqwest::Client) -> Self {
        Self::new_common(server_address, Some(http_client))
    }
}

impl ClientBuilder {

    /// Opt-in transport logging on dedicated log targets (`vim_rs::wire::json`, `vim_rs::wire::soap`).
    ///
    /// Defaults to [`WireLoggingMode::Off`]. Summary records use [`log::Level::Debug`]; full bodies in
    /// [`WireLoggingMode::Detailed`] use [`log::Level::Trace`] except for `SessionManager` traffic,
    /// which stays summary-only. Prefer filtering by target (e.g. `RUST_LOG=vim_rs::wire::json=debug`)
    /// instead of raising global trace for the entire crate.
    pub fn wire_logging(mut self, mode: WireLoggingMode) -> Self {
        self.wire_logging = mode;
        self
    }

    /// Set the transport mode. Requires the `xml` feature for `Soap` and `Auto`.
    pub fn transport(mut self, mode: TransportMode) -> Self {
        self.transport_mode = mode;
        self
    }

    /// Set the compatible API releases. The default is set from the openapi spec. If `api_release`
    /// is not explicitly set then this value or `COMPATIBLE_API_RELEASES` will be used to call the
    /// vCenter [Hello System](https://developer.broadcom.com/xapis/vsphere-automation-api/latest/vcenter/api/vcenter/system__action=hello/post/index)
    /// API to negotiate an API release.
    /// * `compatible_api_releases` - List of compatible API releases
    pub fn compatible_api_releases(mut self, releases: Vec<&str>) -> Self {
        self.compatible_api_releases = Some(releases.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Set the vCenter API release version. The default value from the OpenAPI spec can be used
    /// by setting here the `API_RELEASE` constant. If this is set then the Hello System API will
    /// not be called to negotiate the API release.
    /// * `api_release` - API release version
    pub fn api_release(mut self, api_release: &str) -> Self {
        self.api_release = Some(api_release.to_string());
        self
    }

    /// Set app name and version. This will be used to compose the User-Agent header. User Agent
    /// value is seen in the vSphere UI under Monitoring for the vCenter system for troubleshooting.
    /// The easiest is to use cargo environment variables during build time.
    /// ```rust
    /// const APP_NAME: &str = env!("CARGO_PKG_NAME");
    /// const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
    /// ```
    /// * `app_name` - Name of the application
    /// * `app_version` - Version of the application
    pub fn app_details(mut self, app_name: &str, app_version: &str) -> Self {
        self.app_name = Some(app_name.to_string());
        self.app_version = Some(app_version.to_string());
        self
    }

    /// Set the username and password for basic login.
    /// * `user_name` - Username for login
    /// * `password` - Password for login
    pub fn basic_authn(mut self, user_name: &str, password: &str) -> Self {
        self.user_name = Some(user_name.to_string());
        self.password = Some(password.to_string());
        self
    }

    /// Set the locale for the session. The default is "en".
    /// * `locale` - Locale for the session
    pub fn locale(mut self, locale: &str) -> Self {
        self.locale = Some(locale.to_string());
        self
    }

    /// Build a connected [`Client`] facade for the configured [`TransportMode`].
    ///
    /// - **Json** (default): VI JSON API via Hello System negotiation (unless `api_release` is set).
    /// - **Soap** (`xml` feature): SOAP/XML to `/sdk`.
    /// - **Auto** (`xml` feature): Hello System first; falls back to SOAP if unavailable.
    pub async fn build(self) -> Result<Arc<Client>> {
        #[cfg(feature = "xml")]
        {
            match self.transport_mode {
                TransportMode::Soap => return Self::build_soap_facade(self).await,
                TransportMode::Auto => return Self::build_auto_facade(self).await,
                TransportMode::Json => {}
            }
        }
        Self::build_json(self).await
    }

    async fn build_json(self) -> Result<Arc<Client>> {
        let wire_logging = self.wire_logging;
        #[cfg(feature = "default-client")]
        let http_client = obtain_http_client(self.http_client, self.insecure, false)?;
        #[cfg(not(feature = "default-client"))]
        let http_client = obtain_http_client(self.http_client)?;
        let session_key = Arc::new(RwLock::new(None));

        let user_agent = user_agent(self.app_name.as_deref(), self.app_version.as_deref());

        // Negotiate the API release if not set
        let api_release = match self.api_release {
            Some(release) => release,
            None => {
                let releases = self.compatible_api_releases
                    .unwrap_or_else(|| COMPATIBLE_API_RELEASES.iter().map(|s| s.to_string()).collect());
                let spec = HelloSpec {
                    api_releases: &releases,
                };
                let path = format!("https://{}/api/vcenter/system?action=hello", self.server_address);
                let json_body = miniserde::json::to_string(&spec);
                let started = Instant::now();
                if wire_logging.is_enabled() {
                    let mode_l = wire_log::wire_mode_label(wire_logging, "");
                    let msg = format!(
                        "wire=json mode={} phase=request kind=negotiate path={} body_bytes={}",
                        mode_l,
                        path,
                        json_body.len()
                    );
                    wire_log::log_json_line(wire_logging, "", false, &msg);
                }
                let req = http_client.post(&path)
                    .header("Content-Type", "application/json")
                    .header("User-Agent", &user_agent)
                    .body(json_body);
                let res = match req.send().await {
                    Ok(r) => r,
                    Err(e) => {
                        wire_log::log_json_negotiate_transport_failure(
                            wire_logging,
                            &path,
                            started.elapsed(),
                            &e,
                        );
                        return Err(Error::ReqwestError(e));
                    }
                };
                let status = res.status();
                // Same classification as `Response::error_for_status()` (4xx/5xx → reqwest::Error).
                let http_err = res.error_for_status_ref().err();
                let body = res.text().await.map_err(Error::ReqwestError)?;
                let elapsed = started.elapsed();
                if wire_logging.is_enabled() {
                    let mode_l = wire_log::wire_mode_label(wire_logging, "");
                    let detailed = wire_logging.is_detailed();
                    let mut msg = format!(
                        "wire=json mode={} phase=response kind=negotiate path={} status={} body_bytes={} duration_ms={}",
                        mode_l,
                        path,
                        status.as_u16(),
                        body.len(),
                        elapsed.as_millis()
                    );
                    if detailed {
                        msg.push_str(&format!(" body={}", body));
                    }
                    wire_log::log_json_line(wire_logging, "", detailed, &msg);
                }
                if let Some(e) = http_err {
                    return Err(Error::ReqwestError(e));
                }
                let result: HelloResult = miniserde::json::from_str(&body)
                    .map_err(|_| Error::ParseError("Failed to parse HelloResult".to_string()))?;
                let api_release = result.api_release;
                // Throw error if api_release is empty string indicating no compatible API release
                // was found.
                if api_release.is_empty() {
                    return Err(Error::CannotNegotiateAPIRelease(releases));
                }
                debug!("Negotiated API release: {}", api_release);
                api_release
            },
        };

        let base_url = format!("https://{}/sdk/vim25/{}", self.server_address, api_release);

        let bootstrap = Arc::new(JsonClient {
            http_client: http_client.clone(),
            session_key: session_key.clone(),
            api_release: api_release.clone(),
            base_url: base_url.clone(),
            user_agent: user_agent.clone(),
            service_content: None,
            wire_logging: wire_logging,
        });

        let service_instance = mo::ServiceInstance::new(bootstrap.clone(), SERVICE_INSTANCE_MOID);
        let content = service_instance.content().await?;
        debug!("ServiceInstance content obtained from: {}", content.about.full_name);
        trace!("ServiceInstance content: {:?}", content);

        let sm_id = content.session_manager.as_ref().map(|moid| moid.value.clone());
        let json = Arc::new(JsonClient {
            http_client: http_client.clone(),
            session_key: session_key.clone(),
            api_release: api_release.clone(),
            base_url: base_url.clone(),
            user_agent: user_agent.clone(),
            service_content: Some(content),
            wire_logging: wire_logging,
        });

        if let (Some(ref sm_id), Some(ref user_name), Some(ref password)) = (sm_id, self.user_name, self.password) {
            let sm = mo::SessionManager::new(json.clone(), sm_id);
            let session = sm.login(user_name, password, self.locale.as_deref()).await?;
            debug!("Session created for: {:?}", session.user_name);
        }
        Ok(Arc::new(Client { inner: json }))
    }

    #[cfg(feature = "xml")]
    async fn build_soap_facade(self) -> Result<Arc<Client>> {
        #[cfg(feature = "default-client")]
        let http_client = obtain_http_client(self.http_client, self.insecure, true)?;
        #[cfg(not(feature = "default-client"))]
        let http_client = obtain_http_client(self.http_client)?;
        let ua = user_agent(self.app_name.as_deref(), self.app_version.as_deref());
        let api_release = match self.api_release {
            Some(release) => release.clone(),
            None => API_RELEASE.to_string(),
        };
        let mut soap = crate::xml::client::SoapClient::new(
            http_client,
            &self.server_address,
            &api_release,
            &ua,
            self.wire_logging,
        );
        soap.bootstrap().await?;

        let sm_id = soap.service_content().session_manager.as_ref().map(|m| m.value.clone());
        let soap = Arc::new(soap);

        if let (Some(ref sm_id), Some(ref user_name), Some(ref password)) = (sm_id, self.user_name, self.password) {
            let sm = mo::SessionManager::new(soap.clone(), sm_id);
            let session = sm.login(user_name, password, self.locale.as_deref()).await?;
            debug!("SOAP session created for: {:?}", session.user_name);
        }
        Ok(Arc::new(Client { inner: soap }))
    }

    #[cfg(feature = "xml")]
    async fn build_auto_facade(self) -> Result<Arc<Client>> {
        let wl = self.wire_logging;
        let ua = user_agent(self.app_name.as_deref(), self.app_version.as_deref());
        #[cfg(feature = "default-client")]
        let http_client_for_probe =
            obtain_http_client(self.http_client.clone(), self.insecure, false)?;
        #[cfg(not(feature = "default-client"))]
        let http_client_for_probe = obtain_http_client(self.http_client.clone())?;

        let releases = self.compatible_api_releases.clone()
            .unwrap_or_else(|| COMPATIBLE_API_RELEASES.iter().map(|s| s.to_string()).collect());
        let hello_url = format!("https://{}/api/vcenter/system?action=hello", self.server_address);
        let spec = HelloSpec { api_releases: &releases };
        let json_body = miniserde::json::to_string(&spec);
        let started = Instant::now();
        if wl.is_enabled() {
            let mode_l = wire_log::wire_mode_label(wl, "");
            let msg = format!(
                "wire=json mode={} phase=request kind=probe path={} body_bytes={}",
                mode_l,
                hello_url,
                json_body.len()
            );
            wire_log::log_json_line(wl, "", false, &msg);
        }

        let hello_ok = match http_client_for_probe
            .post(&hello_url)
            .header("Content-Type", "application/json")
            .header("User-Agent", &ua)
            .body(json_body)
            .send()
            .await
        {
            Ok(res) => {
                let status = res.status();
                let body = res.text().await.unwrap_or_default();
                let elapsed = started.elapsed();
                if wl.is_enabled() {
                    let mode_l = wire_log::wire_mode_label(wl, "");
                    let detailed = wl.is_detailed();
                    let mut msg = format!(
                        "wire=json mode={} phase=response kind=probe path={} status={} body_bytes={} duration_ms={}",
                        mode_l,
                        hello_url,
                        status.as_u16(),
                        body.len(),
                        elapsed.as_millis()
                    );
                    if detailed {
                        msg.push_str(&format!(" body={}", body));
                    }
                    wire_log::log_json_line(wl, "", detailed, &msg);
                }
                if status.is_success() {
                    miniserde::json::from_str::<HelloResult>(&body)
                        .ok()
                        .filter(|r| !r.api_release.is_empty())
                } else {
                    None
                }
            }
            Err(e) => {
                if wl.is_enabled() {
                    let mode_l = wire_log::wire_mode_label(wl, "");
                    let msg = format!(
                        "wire=json mode={} phase=response kind=probe path={} error=transport body_bytes=0 duration_ms={} detail={}",
                        mode_l,
                        hello_url,
                        started.elapsed().as_millis(),
                        e
                    );
                    wire_log::log_json_line(wl, "", false, &msg);
                }
                None
            }
        };

        if hello_ok.is_some() {
            debug!("Auto mode: Hello API succeeded, using JSON transport");
            Self::build_json(self.transport(TransportMode::Json)).await
        } else {
            debug!("Auto mode: Hello API unavailable, falling back to SOAP");
            Self::build_soap_facade(self).await
        }
    }
}

/// User-facing session handle: **only** the public methods that existed on `Client` in vim_rs **0.4.0**
/// (`service_content`, `api_release`, `fetch_property`), independent of wire format.
///
/// Internally holds an [`Arc`] to a [`VimClient`] implementation (crate-private JSON or SOAP
/// client). Logout runs when the last strong reference to that inner client is dropped.
pub struct Client {
    inner: Arc<dyn VimClient>,
}

impl Client {
    /// Get the VIM service instance content. This is the main part of the VI JSON API that contains
    /// the core virtualization objects and services. There are additional storage APIs under VSAN,
    /// PBM, VSLM and SMS. There is also ESX Agent Manager API for managing agent virtual machines.
    pub fn service_content(&self) -> &ServiceContent {
        self.inner.service_content()
    }

    /// Get the currently used API release. This may be lower than `API_RELEASE` and should be used
    /// to downgrade client expectations. For example if client is using library 8.0.3.0 with
    /// vCenter 8.0.1.0 the negotiated release will be 8.0.1.0 and the client should not call APIs
    /// or set parameters that are only available in 8.0.3.0.
    pub fn api_release(&self) -> String {
        self.inner.api_release()
    }

    /// Fetch a managed object property by name into the requested type.
    ///
    /// Works for both JSON and SOAP transports via [`extract_property`].
    pub async fn fetch_property<T>(&self, obj: ManagedObjectReference, property: &str) -> Result<T>
    where
        T: miniserde::Deserialize + 'static,
    {
        let type_name: &str = obj.r#type.as_str();
        let id = &obj.value;
        let pv_opt = self
            .inner
            .fetch_property_raw("", type_name, id, property)
            .await?;
        let pv = pv_opt.ok_or_else(|| {
            Error::ParseError(format!("property {property} was empty"))
        })?;
        extract_property(pv)
    }
}

impl VimClient for Client {
    fn service_content(&self) -> &ServiceContent {
        self.inner.service_content()
    }

    fn transport(&self) -> Transport {
        self.inner.transport()
    }

    fn api_release(&self) -> String {
        self.inner.api_release()
    }

    fn invoke<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Bytes>> {
        self.inner.invoke(svc, mo_type, mo_id, method_name, params)
    }

    fn invoke_optional<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Option<Bytes>>> {
        self.inner
            .invoke_optional(svc, mo_type, mo_id, method_name, params)
    }

    fn invoke_void<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<()>> {
        self.inner.invoke_void(svc, mo_type, mo_id, method_name, params)
    }

    fn fetch_property_raw<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        property: &'a str,
    ) -> BoxFuture<'a, Result<Option<PropertyValue>>> {
        self.inner
            .fetch_property_raw(svc, mo_type, mo_id, property)
    }
}

pub(crate) struct JsonWireCtx<'a> {
    pub svc: &'a str,
    pub mo_type: &'a str,
    pub mo_id: &'a str,
    pub name: &'a str,
    pub path: &'a str,
    pub is_property_get: bool,
}

fn json_method_path(svc: &str, mo_type: &str, mo_id: &str, method_name: &str) -> String {
    if svc.is_empty() {
        format!("/{mo_type}/{mo_id}/{method_name}")
    } else {
        format!("/{svc}/{mo_type}/{mo_id}/{method_name}")
    }
}

pub(crate) struct JsonClient {
    http_client: reqwest::Client,
    session_key: Arc<RwLock<Option<String>>>,
    api_release: String,
    base_url: String,
    user_agent: String,
    service_content: Option<ServiceContent>,
    wire_logging: WireLoggingMode,
}

/// VI JSON API implementation (Hello + `/sdk/vim25/{release}`). Crate-private; users hold [`Client`].
///
/// Manages the session key header and logs out when the last [`Arc`] to this value is dropped.
impl JsonClient {
    pub(crate) fn service_content(&self) -> &ServiceContent {
        self.service_content.as_ref().expect("JsonClient missing ServiceContent")
    }

    pub(crate) fn api_release(&self) -> String {
        self.api_release.clone()
    }

    fn get_request(&self, path: &str) -> reqwest::RequestBuilder
    {
        debug!("GET request: {}", path);
        let url = format!("{}{}", self.base_url, path);
        self.http_client.get(&url)
    }

    /// Prepare POST request without a body
    pub(crate) fn post_bare(&self, path: &str) -> reqwest::RequestBuilder
    {
        debug!("POST request (void): {}", path);
        let url = format!("{}{}", self.base_url, path);
        self.http_client.post(&url)
    }

    fn build_post_request(
        &self,
        svc: &str,
        mo_type: &str,
        mo_id: &str,
        method_name: &str,
        params: Option<&(dyn miniserde::Serialize + Send + Sync)>,
    ) -> reqwest::RequestBuilder {
        let path = if svc.is_empty() {
            format!("/{mo_type}/{mo_id}/{method_name}")
        } else {
            format!("/{svc}/{mo_type}/{mo_id}/{method_name}")
        };
        match params {
            Some(payload) => {
                debug!("POST request: {}", path);
                let json_body = miniserde::json::to_string(payload);
                if self.wire_logging.is_enabled() {
                    let mode_l = wire_log::wire_mode_label(self.wire_logging, mo_type);
                    let deny = wire_log::body_logging_note(self.wire_logging, mo_type);
                    let deny_s = deny.unwrap_or("");
                    let deny_sep = if deny_s.is_empty() { "" } else { " " };
                    let mut msg = format!(
                        "wire=json mode={} phase=request svc=\"{}\" mo={} id={} method={} path={} body_bytes={}{}{}",
                        mode_l,
                        svc,
                        mo_type,
                        mo_id,
                        method_name,
                        path,
                        json_body.len(),
                        deny_sep,
                        deny_s
                    );
                    if wire_log::bodies_allowed(self.wire_logging, mo_type) {
                        msg.push_str(&format!(" body={}", json_body));
                    }
                    wire_log::log_json_line(
                        self.wire_logging,
                        mo_type,
                        wire_log::bodies_allowed(self.wire_logging, mo_type),
                        &msg,
                    );
                } else if log::log_enabled!(log::Level::Trace)
                    && !wire_log::suppress_legacy_transport_trace(self.wire_logging)
                {
                    trace!("POST payload: {}", json_body);
                }
                let url = format!("{}{}", self.base_url, path);
                self.http_client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .body(json_body)
            }
            None => {
                if self.wire_logging.is_enabled() {
                    let mode_l = wire_log::wire_mode_label(self.wire_logging, mo_type);
                    let deny = wire_log::body_logging_note(self.wire_logging, mo_type);
                    let deny_s = deny.unwrap_or("");
                    let deny_sep = if deny_s.is_empty() { "" } else { " " };
                    let msg = format!(
                        "wire=json mode={} phase=request svc=\"{}\" mo={} id={} method={} path={} body_bytes=0{}{}",
                        mode_l,
                        svc,
                        mo_type,
                        mo_id,
                        method_name,
                        path,
                        deny_sep,
                        deny_s
                    );
                    wire_log::log_json_line(self.wire_logging, mo_type, false, &msg);
                }
                self.post_bare(&path)
            }
        }
    }

    /// Execute a request that does not return a response body
    pub(crate) async fn execute_void(
        &self,
        mut req: reqwest::RequestBuilder,
        ctx: Option<JsonWireCtx<'_>>,
        started: Instant,
    ) -> Result<()> {
        req = self.prepare(req).await;
        let res = match req.send().await {
            Ok(r) => r,
            Err(e) => {
                if self.wire_logging.is_enabled() {
                    if let Some(c) = ctx.as_ref() {
                        wire_log::log_json_transport_failure(
                            self.wire_logging,
                            c.svc,
                            c.mo_type,
                            c.mo_id,
                            c.name,
                            c.path,
                            c.is_property_get,
                            started.elapsed(),
                            &e,
                        );
                    }
                }
                return Err(Error::ReqwestError(e));
            }
        };
        let res = self
            .process_response(res, Some(started), ctx.as_ref())
            .await?;
        if self.wire_logging.is_enabled() {
            if let Some(c) = ctx.as_ref() {
                let mode_l = wire_log::wire_mode_label(self.wire_logging, c.mo_type);
                let deny = wire_log::body_logging_note(self.wire_logging, c.mo_type);
                let deny_s = deny.unwrap_or("");
                let deny_sep = if deny_s.is_empty() { "" } else { " " };
                let name_field = format!("method={}", c.name);
                let msg = format!(
                    "wire=json mode={} phase=response svc=\"{}\" mo={} id={} {} path={} status={} body_bytes=0 duration_ms={}{}{}",
                    mode_l,
                    c.svc,
                    c.mo_type,
                    c.mo_id,
                    name_field,
                    c.path,
                    res.status().as_u16(),
                    started.elapsed().as_millis(),
                    deny_sep,
                    deny_s
                );
                wire_log::log_json_line(self.wire_logging, c.mo_type, false, &msg);
            }
        }
        Ok(())
    }

    /// Add authn header to request
    async fn prepare(&self, mut req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let session_key = self.session_key.read().await;
        if let Some(value) = session_key.as_ref() {
            req = req.header(AUTHN_HEADER, value);
        }
        req = req.header("User-Agent", &self.user_agent);
        req
    }

    /// Handle authn header update and error unmarsalling
    async fn process_response(
        &self,
        res: reqwest::Response,
        started: Option<Instant>,
        ctx: Option<&JsonWireCtx<'_>>,
    ) -> Result<reqwest::Response> {
        if res.status().is_success() && res.headers().contains_key(AUTHN_HEADER) {
            let session_key = res.headers().get(AUTHN_HEADER).unwrap().to_str().map_err(|_| Error::MissingOrInvalidSessionKey)?.to_string();
            let mut key_holder = self.session_key.write().await;
            *key_holder = Some(session_key);
        }
        if !res.status().is_success() {
            warn!("HTTP error: {}", res.status());
            let status = res.status();
            let body = res.text().await?;
            if let (Some(start), Some(c)) = (started, ctx) {
                if self.wire_logging.is_enabled() {
                    wire_log::log_json_http_error(
                        self.wire_logging,
                        c.svc,
                        c.mo_type,
                        c.mo_id,
                        c.name,
                        c.path,
                        c.is_property_get,
                        status,
                        &body,
                        start.elapsed(),
                    );
                }
            }
            let fault: structs::MethodFault = miniserde::json::from_str(&body)
                .map_err(|_| Error::ParseError(format!("Failed to parse MethodFault from error response: {}", &body[..body.len().min(200)])))?;
            return Err(Error::MethodFault(fault));
        }
        Ok(res)
    }
}

impl VimClient for JsonClient {
    fn service_content(&self) -> &ServiceContent {
        JsonClient::service_content(self)
    }

    fn transport(&self) -> Transport {
        Transport::Json
    }

    fn api_release(&self) -> String {
        JsonClient::api_release(self)
    }

    fn invoke<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Bytes>> {
        Box::pin(async move {
            let path_str = json_method_path(svc, mo_type, mo_id, method_name);
            let ctx = JsonWireCtx {
                svc,
                mo_type,
                mo_id,
                name: method_name,
                path: path_str.as_str(),
                is_property_get: false,
            };
            let started = Instant::now();
            let req = self.build_post_request(svc, mo_type, mo_id, method_name, params);
            let req = self.prepare(req).await;
            let res = match req.send().await {
                Ok(r) => r,
                Err(e) => {
                    wire_log::log_json_transport_failure(
                        self.wire_logging,
                        svc,
                        mo_type,
                        mo_id,
                        method_name,
                        path_str.as_str(),
                        false,
                        started.elapsed(),
                        &e,
                    );
                    return Err(Error::ReqwestError(e));
                }
            };
            let res = self
                .process_response(res, Some(started), Some(&ctx))
                .await?;
            let http_status = res.status();
            let bytes = res.bytes().await?;
            if self.wire_logging.is_enabled() {
                let mode_l = wire_log::wire_mode_label(self.wire_logging, mo_type);
                let deny = wire_log::body_logging_note(self.wire_logging, mo_type);
                let deny_s = deny.unwrap_or("");
                let deny_sep = if deny_s.is_empty() { "" } else { " " };
                let body_lossy = wire_log::sanitize_utf8(&bytes);
                let mut msg = format!(
                    "wire=json mode={} phase=response svc=\"{}\" mo={} id={} method={} path={} status={} body_bytes={} duration_ms={}{}{}",
                    mode_l,
                    svc,
                    mo_type,
                    mo_id,
                    method_name,
                    path_str,
                    http_status.as_u16(),
                    bytes.len(),
                    started.elapsed().as_millis(),
                    deny_sep,
                    deny_s
                );
                if wire_log::bodies_allowed(self.wire_logging, mo_type) {
                    msg.push_str(&format!(" body={}", body_lossy));
                }
                wire_log::log_json_line(
                    self.wire_logging,
                    mo_type,
                    wire_log::bodies_allowed(self.wire_logging, mo_type),
                    &msg,
                );
            } else if log::log_enabled!(log::Level::Trace)
                && !wire_log::suppress_legacy_transport_trace(self.wire_logging)
            {
                let body = String::from_utf8_lossy(&bytes);
                trace!(
                    "JSON response from {}/{}: {}...",
                    mo_type,
                    method_name,
                    &body[..body.len().min(2000)]
                );
            }
            Ok(bytes)
        })
    }

    fn invoke_optional<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Option<Bytes>>> {
        Box::pin(async move {
            let path_str = json_method_path(svc, mo_type, mo_id, method_name);
            let ctx = JsonWireCtx {
                svc,
                mo_type,
                mo_id,
                name: method_name,
                path: path_str.as_str(),
                is_property_get: false,
            };
            let started = Instant::now();
            let req = self.build_post_request(svc, mo_type, mo_id, method_name, params);
            let req = self.prepare(req).await;
            let res = match req.send().await {
                Ok(r) => r,
                Err(e) => {
                    wire_log::log_json_transport_failure(
                        self.wire_logging,
                        svc,
                        mo_type,
                        mo_id,
                        method_name,
                        path_str.as_str(),
                        false,
                        started.elapsed(),
                        &e,
                    );
                    return Err(Error::ReqwestError(e));
                }
            };
            let res = self
                .process_response(res, Some(started), Some(&ctx))
                .await?;
            let http_status = res.status();
            let bytes = res.bytes().await?;
            if self.wire_logging.is_enabled() && !bytes.is_empty() {
                let mode_l = wire_log::wire_mode_label(self.wire_logging, mo_type);
                let deny = wire_log::body_logging_note(self.wire_logging, mo_type);
                let deny_s = deny.unwrap_or("");
                let deny_sep = if deny_s.is_empty() { "" } else { " " };
                let body_lossy = wire_log::sanitize_utf8(&bytes);
                let mut msg = format!(
                    "wire=json mode={} phase=response svc=\"{}\" mo={} id={} method={} path={} status={} body_bytes={} duration_ms={}{}{}",
                    mode_l,
                    svc,
                    mo_type,
                    mo_id,
                    method_name,
                    path_str,
                    http_status.as_u16(),
                    bytes.len(),
                    started.elapsed().as_millis(),
                    deny_sep,
                    deny_s
                );
                if wire_log::bodies_allowed(self.wire_logging, mo_type) {
                    msg.push_str(&format!(" body={}", body_lossy));
                }
                wire_log::log_json_line(
                    self.wire_logging,
                    mo_type,
                    wire_log::bodies_allowed(self.wire_logging, mo_type),
                    &msg,
                );
            } else if log::log_enabled!(log::Level::Trace)
                && !bytes.is_empty()
                && !wire_log::suppress_legacy_transport_trace(self.wire_logging)
            {
                let body = String::from_utf8_lossy(&bytes);
                trace!(
                    "JSON response from {}/{}: {}...",
                    mo_type,
                    method_name,
                    &body[..body.len().min(2000)]
                );
            }
            if bytes.is_empty() {
                Ok(None)
            } else {
                Ok(Some(bytes))
            }
        })
    }

    fn invoke_void<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            let path_str = json_method_path(svc, mo_type, mo_id, method_name);
            let ctx = JsonWireCtx {
                svc,
                mo_type,
                mo_id,
                name: method_name,
                path: path_str.as_str(),
                is_property_get: false,
            };
            let started = Instant::now();
            let req = self.build_post_request(svc, mo_type, mo_id, method_name, params);
            JsonClient::execute_void(self, req, Some(ctx), started).await
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
            let path = if svc.is_empty() {
                format!("/{mo_type}/{mo_id}/{property}")
            } else {
                format!("/{svc}/{mo_type}/{mo_id}/{property}")
            };
            let ctx = JsonWireCtx {
                svc,
                mo_type,
                mo_id,
                name: property,
                path: path.as_str(),
                is_property_get: true,
            };
            let started = Instant::now();
            if self.wire_logging.is_enabled() {
                let mode_l = wire_log::wire_mode_label(self.wire_logging, mo_type);
                let deny = wire_log::body_logging_note(self.wire_logging, mo_type);
                let deny_s = deny.unwrap_or("");
                let deny_sep = if deny_s.is_empty() { "" } else { " " };
                let msg = format!(
                    "wire=json mode={} phase=request svc=\"{}\" mo={} id={} property={} path={} body_bytes=0{}{}",
                    mode_l,
                    svc,
                    mo_type,
                    mo_id,
                    property,
                    path,
                    deny_sep,
                    deny_s
                );
                wire_log::log_json_line(self.wire_logging, mo_type, false, &msg);
            }
            let req = self.get_request(&path);
            let req = self.prepare(req).await;
            let res = match req.send().await {
                Ok(r) => r,
                Err(e) => {
                    wire_log::log_json_transport_failure(
                        self.wire_logging,
                        svc,
                        mo_type,
                        mo_id,
                        property,
                        path.as_str(),
                        true,
                        started.elapsed(),
                        &e,
                    );
                    return Err(Error::ReqwestError(e));
                }
            };
            let res = self
                .process_response(res, Some(started), Some(&ctx))
                .await?;
            let http_status = res.status();
            let bytes = res.bytes().await?;
            if self.wire_logging.is_enabled() && !bytes.is_empty() {
                let mode_l = wire_log::wire_mode_label(self.wire_logging, mo_type);
                let deny = wire_log::body_logging_note(self.wire_logging, mo_type);
                let deny_s = deny.unwrap_or("");
                let deny_sep = if deny_s.is_empty() { "" } else { " " };
                let body_lossy = wire_log::sanitize_utf8(&bytes);
                let mut msg = format!(
                    "wire=json mode={} phase=response svc=\"{}\" mo={} id={} property={} path={} status={} body_bytes={} duration_ms={}{}{}",
                    mode_l,
                    svc,
                    mo_type,
                    mo_id,
                    property,
                    path,
                    http_status.as_u16(),
                    bytes.len(),
                    started.elapsed().as_millis(),
                    deny_sep,
                    deny_s
                );
                if wire_log::bodies_allowed(self.wire_logging, mo_type) {
                    msg.push_str(&format!(" body={}", body_lossy));
                }
                wire_log::log_json_line(
                    self.wire_logging,
                    mo_type,
                    wire_log::bodies_allowed(self.wire_logging, mo_type),
                    &msg,
                );
            } else if log::log_enabled!(log::Level::Trace)
                && !bytes.is_empty()
                && !wire_log::suppress_legacy_transport_trace(self.wire_logging)
            {
                let body = String::from_utf8_lossy(&bytes);
                trace!(
                    "JSON fetch_property_raw {}/{}: {}...",
                    mo_type,
                    property,
                    &body[..body.len().min(2000)]
                );
            }
            if bytes.is_empty() {
                Ok(None)
            } else {
                Ok(Some(PropertyValue::Json(bytes)))
            }
        })
    }
}


/// Log out the JSON session when the last strong reference to this [`JsonClient`] is dropped.
impl Drop for JsonClient {
    fn drop(&mut self) {
        debug!("Disposing VIM client.");

        let session_key = Arc::clone(&self.session_key);
        let http_client = &self.http_client.clone();
        let base_url = self.base_url.clone();
        let wire_logging = self.wire_logging;

        let sm_id = self.service_content.as_ref().and_then(|content| content.session_manager.as_ref().map(|moid| moid.value.clone()));
        let sm_id = match sm_id {
            Some(id) => id,
            None => {
                debug!("No session manager found. Skipping logout.");
                return;
            },
        };

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                debug!("Terminating VIM session as needed.");
                let key = {
                    let session_key = session_key.read().await;
                    session_key.clone()
                };
                let Some(key) = key else {
                    debug!("No session key present. Skipping logout.");
                    return;
                };
                debug!("Session is present. Sending logout request...");

                let path = format!("{base_url}/SessionManager/{moId}/Logout",
                                    base_url = base_url,
                                    moId = sm_id);
                if wire_logging.is_enabled() {
                    let mode_l = wire_log::wire_mode_label(wire_logging, "SessionManager");
                    let msg = format!(
                        "wire=json mode={} phase=request kind=logout mo=SessionManager id={} method=Logout path={} body_bytes=0 body_logging=denylisted",
                        mode_l,
                        sm_id,
                        path
                    );
                    wire_log::log_json_line(wire_logging, "SessionManager", false, &msg);
                }
                let req = http_client.post(&path)
                                        .header(AUTHN_HEADER, key);
                let started = Instant::now();
                match req.send().await {
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
                                "wire=json mode={} phase=response kind=logout mo=SessionManager id={} method=Logout status={} body_bytes={} duration_ms={} body_logging=denylisted{}",
                                mode_l,
                                sm_id,
                                status.as_u16(),
                                body.len(),
                                dur.as_millis(),
                                http_note
                            );
                            wire_log::log_json_line(wire_logging, "SessionManager", false, &msg);
                        }
                        if status.is_success() {
                            debug!("Session logged out successfully");
                        } else {
                            match miniserde::json::from_str::<structs::MethodFault>(&body) {
                                Ok(fault) => warn!("Failed to logout session(HTTP code: {}). MethodFault: {:?}", status, fault),
                                Err(_) => warn!("Failed to logout session(HTTP code: {}). Cannot parse MethodFault: {}", status, &body[..body.len().min(200)]),
                            }
                        }
                    }
                    Err(e) => {
                        if wire_logging.is_enabled() {
                            let mode_l = wire_log::wire_mode_label(wire_logging, "SessionManager");
                            let msg = format!(
                                "wire=json mode={} phase=response kind=logout mo=SessionManager id={} method=Logout error=transport duration_ms={} body_logging=denylisted detail={}",
                                mode_l,
                                sm_id,
                                started.elapsed().as_millis(),
                                e
                            );
                            wire_log::log_json_line(wire_logging, "SessionManager", false, &msg);
                        }
                        warn!("Failed to logout session. Cannot execute logout request: {}", e);
                    }
                }
            });
        });
    }
}

fn user_agent(app_name: Option<&str>, app_version: Option<&str>) -> String {
    let app_name: String = if app_name.is_some() {
        app_name.unwrap().to_string()
    } else {
        get_executable_name().unwrap_or_else(|| "unknown".to_string())
    };
    let Some(appv) = app_version else {
        return format!(
            "{} ({}/{}; {}; {}; rustc/{})",
            app_name,
            LIB_NAME,
            LIB_VERSION,
            std::env::consts::OS,
            std::env::consts::ARCH,
            RUSTC_VERSION
        );
    };
    format!(
        "{}/{} ({}/{}; {}; {}; rustc/{})",
        app_name,
        appv,
        LIB_NAME,
        LIB_VERSION,
        std::env::consts::OS,
        std::env::consts::ARCH,
        RUSTC_VERSION
    )
}

fn get_executable_name() -> Option<String> {
    std::env::current_exe()
        .ok()
        .as_ref()
        .and_then(|path| path.file_name())
        .and_then(OsStr::to_str)
        .map(|s| s.to_owned())
}


/// The Hello System API request. This is not full-fledged binding but a simple request to
/// negotiate the API release version.
/// See [Hello System](https://developer.broadcom.com/xapis/vsphere-automation-api/latest/vcenter/api/vcenter/system__action=hello/post/index)
struct HelloSpec<'a> {
    /// List of API release IDs that the client can work with in order of preference. The server will select the first mutually supported release ID.
    api_releases: &'a Vec<String>,
}

impl miniserde::Serialize for HelloSpec<'_> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        use miniserde::ser::Fragment;
        Fragment::Map(Box::new(HelloSpecSerializer { data: self, seq: 0 }))
    }
}

struct HelloSpecSerializer<'a> {
    data: &'a HelloSpec<'a>,
    seq: usize,
}

impl miniserde::ser::Map for HelloSpecSerializer<'_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let result = match self.seq {
            0 => Some((std::borrow::Cow::Borrowed("api_releases"), &self.data.api_releases as &dyn miniserde::Serialize)),
            _ => None,
        };
        self.seq += 1;
        result
    }
}

impl std::fmt::Debug for HelloSpec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HelloSpec")
            .field("api_releases", &self.api_releases)
            .finish()
    }
}

/// The Hello System API response. This is not full-fledged binding but a simple response to
/// negotiate the API release version.
struct HelloResult {
    /// The ID of a mutually-supported API release. This ID should be used in subsequent API calls
    /// to the current vCenter system. If there is no mutually-supported API release, the value will
    /// be an empty string, e.g. "". Typically, this is a case where one of the parties is much
    /// older than the other party.
    api_release: String,
}

miniserde::make_place!(Place);

impl miniserde::Deserialize for HelloResult {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        Place::new(out)
    }
}

impl miniserde::de::Visitor for Place<HelloResult> {
    fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {
        Ok(Box::new(HelloResultFields {
            api_release: None,
            __out: &mut self.out,
        }))
    }
}

struct HelloResultFields<'a> {
    api_release: Option<String>,
    __out: &'a mut Option<HelloResult>,
}

impl miniserde::de::Map for HelloResultFields<'_> {
    fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn miniserde::de::Visitor> {
        match k {
            "api_release" => Ok(miniserde::Deserialize::begin(&mut self.api_release)),
            _ => Ok(<dyn miniserde::de::Visitor>::ignore()),
        }
    }

    fn finish(&mut self) -> miniserde::Result<()> {
        let api_release = self.api_release.take().ok_or(miniserde::Error)?;
        *self.__out = Some(HelloResult { api_release });
        Ok(())
    }
}

impl std::fmt::Debug for HelloResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HelloResult")
            .field("api_release", &self.api_release)
            .finish()
    }
}

/// Shared by wire-logging transport tests (`core` + `xml` client).
#[cfg(test)]
pub(crate) fn test_dead_port_http_client() -> reqwest::Client {
    #[cfg(feature = "xml")]
    {
        return reqwest::Client::builder()
            .cookie_store(true)
            .connect_timeout(std::time::Duration::from_millis(500))
            .timeout(std::time::Duration::from_secs(3))
            .build()
            .expect("reqwest test client");
    }
    #[cfg(not(feature = "xml"))]
    {
        reqwest::Client::builder()
            .connect_timeout(std::time::Duration::from_millis(500))
            .timeout(std::time::Duration::from_secs(3))
            .build()
            .expect("reqwest test client")
    }
}

/// Unbound TCP address used to force `send()` transport failures quickly.
#[cfg(test)]
pub(crate) const TEST_WIRE_DEAD_ADDR: &str = "127.0.0.1:65433";

#[cfg(test)]
pub(crate) fn test_minimal_service_content_for_tests() -> ServiceContent {
    let json = r#"{
        "_typeName": "ServiceContent",
        "rootFolder": {"_typeName":"ManagedObjectReference","type":"Folder","value":"root-1"},
        "propertyCollector": {"_typeName":"ManagedObjectReference","type":"PropertyCollector","value":"pc-1"},
        "viewManager": {"_typeName":"ManagedObjectReference","type":"ViewManager","value":"vmgr-1"},
        "about": {
            "_typeName":"AboutInfo",
            "name":"n",
            "fullName":"f",
            "vendor":"v",
            "version":"1",
            "build":"b",
            "osType":"o",
            "productLineId":"p",
            "apiType":"VirtualCenter",
            "apiVersion":"1"
        }
    }"#;
    miniserde::json::from_str(json).expect("fixture ServiceContent")
}

#[cfg(test)]
pub(crate) fn test_json_client_wire_transport() -> Arc<JsonClient> {
    let base_url = format!("https://{}/sdk/vim25/{}", TEST_WIRE_DEAD_ADDR, API_RELEASE);
    Arc::new(JsonClient {
        http_client: test_dead_port_http_client(),
        session_key: Arc::new(RwLock::new(None)),
        api_release: API_RELEASE.to_string(),
        base_url,
        user_agent: "wire-transport-test".to_string(),
        service_content: Some(test_minimal_service_content_for_tests()),
        wire_logging: WireLoggingMode::Summary,
    })
}

/// [`ServiceContent`] with `sessionManager` set (for JSON logout-on-`Drop` tests).
#[cfg(test)]
pub(crate) fn test_service_content_with_session_manager_for_tests() -> ServiceContent {
    let json = r#"{
        "_typeName": "ServiceContent",
        "rootFolder": {"_typeName":"ManagedObjectReference","type":"Folder","value":"root-1"},
        "propertyCollector": {"_typeName":"ManagedObjectReference","type":"PropertyCollector","value":"pc-1"},
        "viewManager": {"_typeName":"ManagedObjectReference","type":"ViewManager","value":"vmgr-1"},
        "sessionManager": {"_typeName":"ManagedObjectReference","type":"SessionManager","value":"sm-wire-test"},
        "about": {
            "_typeName":"AboutInfo",
            "name":"n",
            "fullName":"f",
            "vendor":"v",
            "version":"1",
            "build":"b",
            "osType":"o",
            "productLineId":"p",
            "apiType":"VirtualCenter",
            "apiVersion":"1"
        }
    }"#;
    miniserde::json::from_str(json).expect("fixture ServiceContent with session manager")
}

/// JSON VI client pointing at `http_origin` (e.g. `http://127.0.0.1:PORT`) for local stub servers.
#[cfg(test)]
pub(crate) fn test_json_client_http_origin(
    http_origin: &str,
    session_key: Option<String>,
) -> Arc<JsonClient> {
    let base_url = format!("{http_origin}/sdk/vim25/{API_RELEASE}");
    Arc::new(JsonClient {
        http_client: reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .expect("reqwest test client"),
        session_key: Arc::new(RwLock::new(session_key)),
        api_release: API_RELEASE.to_string(),
        base_url,
        user_agent: "wire-http-test".to_string(),
        service_content: Some(test_service_content_with_session_manager_for_tests()),
        wire_logging: WireLoggingMode::Summary,
    })
}

/// Wire logging integration tests: transport failures, HTTP error responses (`log_json_http_error`), and
/// logout-on-`Drop` paths (JSON + SOAP). Uses a multi-threaded Tokio runtime for every async test because
/// `JsonClient` / `SoapClient` `Drop` uses `block_in_place` + `block_on`. Serializes tests that share the
/// global `log` sink and a localhost HTTP stub on a background OS thread for deterministic responses.
#[cfg(test)]
mod wire_logging_transport_tests {
    use std::sync::{Mutex, Once};

    use std::time::Instant;

    use super::super::wire_log;
    use super::{
        test_dead_port_http_client, test_json_client_wire_transport, ClientBuilder, Error, JsonClient,
        VimClient, WireLoggingMode, TEST_WIRE_DEAD_ADDR,
    };
    use miniserde::Serialize;

    /// Matches [`crate::core::wire_log::TARGET_SOAP`] without importing the `xml`-gated symbol.
    const SOAP_WIRE_TARGET: &str = "vim_rs::wire::soap";

    static LOG_INIT: Once = Once::new();
    static SERIAL: Mutex<()> = Mutex::new(());
    static WIRE_LINES: Mutex<Vec<String>> = Mutex::new(Vec::new());

    struct CaptureWireLogger;

    impl log::Log for CaptureWireLogger {
        fn enabled(&self, _: &log::Metadata<'_>) -> bool {
            true
        }

        fn log(&self, record: &log::Record<'_>) {
            let t = record.target();
            if t == wire_log::TARGET_JSON || t == SOAP_WIRE_TARGET {
                WIRE_LINES
                    .lock()
                    .expect("wire log capture")
                    .push(record.args().to_string());
            }
        }

        fn flush(&self) {}
    }

    fn init_wire_capture() {
        LOG_INIT.call_once(|| {
            let _ = log::set_logger(&CaptureWireLogger);
            log::set_max_level(log::LevelFilter::Trace);
        });
    }

    fn clear_wire_lines() {
        WIRE_LINES.lock().expect("wire lines").clear();
    }

    fn joined_wire_output() -> String {
        WIRE_LINES.lock().expect("wire lines").join("\n")
    }

    /// Empty JSON object `{}` for `invoke` paths that use a POST body.
    struct EmptyJsonObject;
    struct EmptyJsonMap;

    impl Serialize for EmptyJsonObject {
        fn begin(&self) -> miniserde::ser::Fragment<'_> {
            miniserde::ser::Fragment::Map(Box::new(EmptyJsonMap))
        }
    }

    impl miniserde::ser::Map for EmptyJsonMap {
        fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn Serialize)> {
            None
        }
    }

    fn client_builder_for_wire_dead_addr() -> ClientBuilder {
        let http = test_dead_port_http_client();
        #[cfg(feature = "default-client")]
        {
            ClientBuilder::new(TEST_WIRE_DEAD_ADDR).http_client(http)
        }
        #[cfg(not(feature = "default-client"))]
        {
            ClientBuilder::new(TEST_WIRE_DEAD_ADDR, http)
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_hello_negotiate_and_invoke_paths_log_transport_failure() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        // Hello System negotiation (no fixed api_release → must POST hello).
        let hello_err = client_builder_for_wire_dead_addr()
            .wire_logging(WireLoggingMode::Summary)
            .build()
            .await;
        assert!(
            matches!(hello_err.as_ref(), Err(Error::ReqwestError(_))),
            "expected transport error from hello, got {:?}",
            hello_err.as_ref().err()
        );
        let out = joined_wire_output();
        assert!(
            out.contains("phase=request kind=negotiate"),
            "hello request line missing: {out}"
        );
        assert!(
            out.contains("phase=response kind=negotiate") && out.contains("error=transport"),
            "hello transport response line missing: {out}"
        );

        clear_wire_lines();
        let jc = test_json_client_wire_transport();
        let empty = EmptyJsonObject;

        let invoke_err = jc
            .invoke(
                "",
                "VirtualMachine",
                "vm-1",
                "RefreshStorageInfo",
                Some(&empty as &(dyn Serialize + Send + Sync)),
            )
            .await;
        assert!(matches!(invoke_err, Err(Error::ReqwestError(_))));
        let out = joined_wire_output();
        assert!(
            out.contains("phase=request") && out.contains("method=RefreshStorageInfo"),
            "invoke request missing: {out}"
        );
        assert!(
            out.contains("phase=response") && out.contains("error=transport"),
            "invoke transport response missing: {out}"
        );

        clear_wire_lines();
        let opt_err = jc
            .invoke_optional(
                "",
                "VirtualMachine",
                "vm-1",
                "SomeMethod",
                Some(&empty as &(dyn Serialize + Send + Sync)),
            )
            .await;
        assert!(matches!(opt_err, Err(Error::ReqwestError(_))));
        let out = joined_wire_output();
        assert!(out.contains("error=transport"), "invoke_optional: {out}");

        clear_wire_lines();
        let void_err = jc
            .invoke_void("", "VirtualMachine", "vm-1", "Destroy", None)
            .await;
        assert!(matches!(void_err, Err(Error::ReqwestError(_))));
        let out = joined_wire_output();
        assert!(
            out.contains("phase=request") && out.contains("body_bytes=0"),
            "void request: {out}"
        );
        assert!(out.contains("error=transport"), "void transport: {out}");

        clear_wire_lines();
        let fetch_err = jc
            .fetch_property_raw("", "VirtualMachine", "vm-1", "name")
            .await;
        assert!(matches!(fetch_err, Err(Error::ReqwestError(_))));
        let out = joined_wire_output();
        assert!(
            out.contains("property=name") && out.contains("error=transport"),
            "fetch_property_raw: {out}"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_execute_void_helper_logs_transport_failure() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        let jc = test_json_client_wire_transport();
        let path_str = super::json_method_path("", "Folder", "group-d1", "Destroy");
        let ctx = super::JsonWireCtx {
            svc: "",
            mo_type: "Folder",
            mo_id: "group-d1",
            name: "Destroy",
            path: path_str.as_str(),
            is_property_get: false,
        };
        let started = Instant::now();
        let req = jc.build_post_request("", "Folder", "group-d1", "Destroy", None);
        let req = jc.prepare(req).await;
        let err = JsonClient::execute_void(jc.as_ref(), req, Some(ctx), started).await;
        assert!(matches!(err, Err(Error::ReqwestError(_))));
        let out = joined_wire_output();
        assert!(out.contains("method=Destroy") && out.contains("error=transport"), "{out}");
    }

    #[cfg(feature = "xml")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_auto_probe_logs_transport_on_hello_send_failure() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        let err = client_builder_for_wire_dead_addr()
            .wire_logging(WireLoggingMode::Summary)
            .transport(super::TransportMode::Auto)
            .build()
            .await;
        assert!(err.is_err(), "expected auto build to fail");
        let out = joined_wire_output();
        assert!(
            out.contains("kind=probe") && out.contains("error=transport"),
            "auto probe transport line missing: {out}"
        );
    }

    /// One-shot HTTP/1.1 stub on a background thread (avoids deadlocks with `Drop` + `block_on`).
    fn spawn_http_stub_once(status: u16, body: &[u8]) -> (String, std::thread::JoinHandle<()>) {
        use std::io::{Read, Write};
        use std::net::TcpListener;
        use std::thread;
        use std::time::Duration;

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind http stub");
        let port = listener.local_addr().expect("stub addr").port();
        let body = body.to_vec();
        let h = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("stub accept");
            let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
            let mut buf = vec![0u8; 32768];
            let _ = stream.read(&mut buf);
            let status_text = match status {
                200 => "OK",
                403 => "Forbidden",
                500 => "Internal Server Error",
                503 => "Service Unavailable",
                _ => "Error",
            };
            let head = format!(
                "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                status,
                status_text,
                body.len()
            );
            let mut out = head.into_bytes();
            out.extend_from_slice(&body);
            let _ = stream.write_all(&out);
        });
        (format!("http://127.0.0.1:{port}"), h)
    }

    /// Minimal [`MethodFault`] JSON (parses via `process_response`).
    const SAMPLE_FAULT_JSON: &str = r#"{"_typeName":"VAppPropertyFault","id":"x","category":"string","label":"l","type":"string","value":"v"}"#;

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_invoke_non_success_emits_wire_http_error_line() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        let (origin, stub) = spawn_http_stub_once(500, SAMPLE_FAULT_JSON.as_bytes());
        let jc = super::test_json_client_http_origin(&origin, Some("sess".into()));
        let empty = EmptyJsonObject;
        let err = jc
            .invoke(
                "",
                "VirtualMachine",
                "vm-1",
                "SomeMethod",
                Some(&empty as &(dyn Serialize + Send + Sync)),
            )
            .await;
        assert!(matches!(err, Err(Error::MethodFault(_))));
        stub.join().expect("stub thread");
        let out = joined_wire_output();
        assert!(out.contains("phase=request") && out.contains("method=SomeMethod"), "{out}");
        assert!(
            out.contains("status=500") && out.contains("phase=response"),
            "expected log_json_http_error-style line: {out}"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_fetch_property_non_success_emits_wire_http_error_line() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        let (origin, stub) = spawn_http_stub_once(403, SAMPLE_FAULT_JSON.as_bytes());
        let jc = super::test_json_client_http_origin(&origin, Some("sess".into()));
        let err = jc
            .fetch_property_raw("", "HostSystem", "host-9", "name")
            .await;
        assert!(matches!(err, Err(Error::MethodFault(_))));
        stub.join().expect("stub thread");
        let out = joined_wire_output();
        assert!(out.contains("property=name") && out.contains("status=403"), "{out}");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_drop_logout_emits_wire_lines_on_http_success() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        let (origin, stub) = spawn_http_stub_once(200, b"");
        let jc = super::test_json_client_http_origin(&origin, Some("sk-drop-ok".into()));
        drop(jc);
        stub.join().expect("stub thread");
        let out = joined_wire_output();
        assert!(out.contains("kind=logout") && out.contains("phase=request"), "{out}");
        assert!(
            out.contains("status=200") && !out.contains("error=http_failure"),
            "{out}"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_drop_logout_emits_wire_lines_on_http_non_success() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        let (origin, stub) = spawn_http_stub_once(503, b"x");
        let jc = super::test_json_client_http_origin(&origin, Some("sk-drop-bad".into()));
        drop(jc);
        stub.join().expect("stub thread");
        let out = joined_wire_output();
        assert!(
            out.contains("kind=logout") && out.contains("error=http_failure"),
            "{out}"
        );
        assert!(out.contains("status=503"), "{out}");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_drop_logout_emits_wire_transport_line() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        let origin = format!("http://{}", super::TEST_WIRE_DEAD_ADDR);
        let jc = super::test_json_client_http_origin(&origin, Some("sk-drop-tr".into()));
        drop(jc);
        let out = joined_wire_output();
        assert!(
            out.contains("kind=logout") && out.contains("error=transport"),
            "{out}"
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn soap_bootstrap_logs_transport_failure() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        use crate::xml::client::SoapClient;

        let mut soap = SoapClient::new(
            test_dead_port_http_client(),
            TEST_WIRE_DEAD_ADDR,
            super::API_RELEASE,
            "soap-bootstrap-wire-test",
            WireLoggingMode::Summary,
        );
        let err = soap.bootstrap().await;
        assert!(matches!(err, Err(Error::ReqwestError(_))));
        let out = joined_wire_output();
        assert!(
            out.contains("RetrieveServiceContent")
                && out.contains("phase=request")
                && out.contains("error=transport"),
            "SOAP bootstrap wire: {out}"
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn soap_invoke_logs_transport_failure() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        use crate::xml::client::soap_test_client_with_service_content;

        let soap = soap_test_client_with_service_content();
        let err = VimClient::invoke(
            &soap,
            "",
            "VirtualMachine",
            "vm-wire",
            "RefreshStorageInfo",
            None,
        )
        .await;
        assert!(matches!(err, Err(Error::ReqwestError(_))));
        let out = joined_wire_output();
        assert!(
            out.contains("RefreshStorageInfo") && out.contains("error=transport"),
            "SOAP invoke wire: {out}"
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn soap_drop_logout_emits_wire_on_http_success() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        let (origin, stub) = spawn_http_stub_once(200, b"");
        let endpoint = format!("{}/sdk", origin.trim_end_matches('/'));
        let soap = crate::xml::client::soap_test_client_for_logout_drop(
            reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap(),
            endpoint,
        );
        drop(soap);
        stub.join().expect("stub thread");
        let out = joined_wire_output();
        assert!(
            out.contains("wire=soap")
                && out.contains("kind=logout")
                && out.contains("status=200")
                && !out.contains("error=http_failure"),
            "{out}"
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn soap_drop_logout_emits_wire_on_http_non_success() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        let (origin, stub) = spawn_http_stub_once(502, b"err");
        let endpoint = format!("{}/sdk", origin.trim_end_matches('/'));
        let soap = crate::xml::client::soap_test_client_for_logout_drop(
            reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap(),
            endpoint,
        );
        drop(soap);
        stub.join().expect("stub thread");
        let out = joined_wire_output();
        assert!(
            out.contains("error=http_failure") && out.contains("status=502"),
            "{out}"
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn soap_drop_logout_emits_wire_transport_line() {
        let _serial = SERIAL.lock().expect("serial");
        init_wire_capture();
        clear_wire_lines();

        let endpoint = format!("http://{}/sdk", super::TEST_WIRE_DEAD_ADDR);
        let soap = crate::xml::client::soap_test_client_for_logout_drop(
            reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(2))
                .build()
                .unwrap(),
            endpoint,
        );
        drop(soap);
        let out = joined_wire_output();
        assert!(
            out.contains("kind=logout") && out.contains("error=transport"),
            "{out}"
        );
    }
}