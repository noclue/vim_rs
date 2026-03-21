use std::sync::Arc;

use tokio::sync::RwLock;
use super::super::types::structs;
use log::{warn, debug, trace};

use bytes::Bytes;
use std::future::Future;
use std::pin::Pin;
use std::ffi::OsStr;
use crate::mo;
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
    #[error("MethodFault: {0:?}")]
    MethodFault(structs::MethodFault),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
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

/// A boxed future used by the object-safe `VimClient` trait.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Transport protocol used by a `VimClient` instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Json,
    #[cfg(feature = "xml")]
    Soap,
}

/// Object-safe client abstraction for generated managed-object stubs (`crate::mo::*`).
///
/// The trait exposes high-level operations that are transport-agnostic.
/// Generated MO proxies call these methods and then use the free `unmarshal`
/// function for deserialization, keyed on `transport()`.
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

    /// Fetch a single property by name, returning the raw wire bytes (or `None`).
    ///
    /// Managed-object stubs call this and then [`unmarshal`] with [`Self::transport`].
    /// Application code typically uses [`Client::fetch_property`] on the public facade instead.
    fn fetch_property_raw<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        property: &'a str,
    ) -> BoxFuture<'a, Result<Option<Bytes>>>;
}

/// Deserialize response bytes according to the transport format.
///
/// This is a free function (not a trait method) so that it can be generic on `T`
/// without breaking object-safety of `VimClient`.
pub fn unmarshal<T: miniserde::Deserialize>(transport: Transport, bytes: &[u8]) -> Result<T> {
    let text = std::str::from_utf8(bytes)
        .map_err(|e| Error::ParseError(e.to_string()))?;
    match transport {
        Transport::Json => miniserde::json::from_str(text)
            .map_err(|_| Error::ParseError(format!(
                "JSON deserialization failed for {}", std::any::type_name::<T>()))),
        #[cfg(feature = "xml")]
        Transport::Soap => {
            // Method responses are SOAP envelopes with `<returnval>`; property fetch returns raw object XML.
            crate::xml::soap::vim_response(text)
                .or_else(|_| crate::xml::de::from_xml(text))
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
        #[cfg(feature = "xml")]
        Transport::Soap => crate::xml::soap::vim_response_list(text).map_err(|_| {
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

pub struct ClientBuilder {
    server_address: String,
    compatible_api_releases: Option<Vec<String>>,
    api_release: Option<String>,
    http_client: Option<reqwest::Client>,
    insecure: Option<bool>,
    app_name: Option<String>,
    app_version: Option<String>,
    user_name: Option<String>,
    password: Option<String>,
    locale: Option<String>,
    transport_mode: TransportMode,
}

impl ClientBuilder {
    /// Create a new client builder for a VI/JSON API at given FQDN or IP address
    ///
    /// * `server_address` - vCenter server FQDN or IP address
    pub fn new(server_address: &str) -> Self {
        Self {
            server_address: server_address.to_string(),
            compatible_api_releases: None,
            api_release: None,
            http_client: None,
            insecure: None,
            app_name: None,
            app_version: None,
            user_name: None,
            password: None,
            locale: None,
            transport_mode: TransportMode::default(),
        }
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

    /// Set the reqwest::Client instance to use for HTTP requests.
    /// This resets the insecure flag. Use the http_client methods to set the certificate and
    /// hostname verification behavior.
    /// * `http_client` - preconfigured reqwest::Client instance
    pub fn http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self.insecure = None;
        self
    }

    /// Set the insecure flag to allow invalid certificates and hostnames.
    /// This resets the http_client. A new reqwest::Client instance will be created instead.
    /// * `insecure` - Allow invalid certificates and hostnames
    pub fn insecure(mut self, insecure: bool) -> Self {
        warn!("!!! WARNING !!! Insecure mode enabled. TLS certificate and hostname verification is disabled. !!! WARNING !!!");
        self.insecure = Some(insecure);
        self.http_client = None;
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
        let http_client = match self.http_client {
            Some(client) => client,
            None => {
                let mut builder = reqwest::ClientBuilder::new();
                if let Some(insecure) = self.insecure {
                    builder = builder.danger_accept_invalid_certs(insecure)
                                     .danger_accept_invalid_hostnames(insecure);
                }
                builder.build()?
            },
        };
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
                let req = http_client.post(&path)
                    .header("Content-Type", "application/json")
                    .header("User-Agent", &user_agent)
                    .body(json_body);
                let res = req.send().await?;
                let res = res.error_for_status()?;
                let body = res.text().await?;
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
        let http_client = match self.http_client {
            Some(client) => client,
            None => {
                let mut builder = reqwest::ClientBuilder::new()
                    .cookie_store(true);
                if let Some(insecure) = self.insecure {
                    builder = builder.danger_accept_invalid_certs(insecure)
                                     .danger_accept_invalid_hostnames(insecure);
                }
                builder.build()?
            },
        };
        let ua = user_agent(self.app_name.as_deref(), self.app_version.as_deref());
        let api_release = match self.api_release {
            Some(release) => release.clone(),
            None => API_RELEASE.to_string(),
        };
        let mut soap = crate::xml::client::SoapClient::new(
            http_client, &self.server_address, &api_release, &ua,
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
        let ua = user_agent(self.app_name.as_deref(), self.app_version.as_deref());
        let http_client_for_probe = {
            let mut builder = reqwest::ClientBuilder::new();
            if let Some(insecure) = self.insecure {
                builder = builder.danger_accept_invalid_certs(insecure)
                                 .danger_accept_invalid_hostnames(insecure);
            }
            builder.build()?
        };

        let releases = self.compatible_api_releases.clone()
            .unwrap_or_else(|| COMPATIBLE_API_RELEASES.iter().map(|s| s.to_string()).collect());
        let hello_url = format!("https://{}/api/vcenter/system?action=hello", self.server_address);
        let spec = HelloSpec { api_releases: &releases };
        let json_body = miniserde::json::to_string(&spec);

        let hello_ok = match http_client_for_probe
            .post(&hello_url)
            .header("Content-Type", "application/json")
            .header("User-Agent", &ua)
            .body(json_body)
            .send()
            .await
        {
            Ok(res) if res.status().is_success() => {
                let body = res.text().await.unwrap_or_default();
                miniserde::json::from_str::<HelloResult>(&body)
                    .ok()
                    .filter(|r| !r.api_release.is_empty())
            }
            _ => None,
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
    /// Works for both JSON and SOAP transports (via [`unmarshal`] and the inner client’s
    /// [`VimClient::transport`]).
    pub async fn fetch_property<T>(&self, obj: ManagedObjectReference, property: &str) -> Result<T>
    where
        T: miniserde::Deserialize,
    {
        let type_name: &str = obj.r#type.as_str();
        let id = &obj.value;
        let bytes_opt = self
            .inner
            .fetch_property_raw("", type_name, id, property)
            .await?;
        let bytes = bytes_opt.ok_or_else(|| {
            Error::ParseError(format!("property {property} was empty"))
        })?;
        unmarshal(self.inner.transport(), &bytes)
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
    ) -> BoxFuture<'a, Result<Option<Bytes>>> {
        self.inner
            .fetch_property_raw(svc, mo_type, mo_id, property)
    }
}

pub(crate) struct JsonClient {
    http_client: reqwest::Client,
    session_key: Arc<RwLock<Option<String>>>,
    api_release: String,
    base_url: String,
    user_agent: String,
    service_content: Option<ServiceContent>,
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
        let path =  if svc.is_empty() {
            format!("/{mo_type}/{mo_id}/{method_name}")
        } else {
            format!("/{svc}/{mo_type}/{mo_id}/{method_name}")
        };
        match params {
            Some(payload) => {
                debug!("POST request: {}", path);
                let json_body = miniserde::json::to_string(payload);
                if log::log_enabled!(log::Level::Trace) {
                    trace!("POST payload: {}", json_body);
                }
                let url = format!("{}{}", self.base_url, path);
                self.http_client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .body(json_body)
            }
            None => self.post_bare(&path),
        }
    }

    /// Execute a request that does not return a response body
    pub(crate) async fn execute_void(&self, mut req: reqwest::RequestBuilder) -> Result<()>
    {
        req = self.prepare(req).await;
        let res = req.send().await?;
        let _ = self.process_response(res).await?;
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
    async fn process_response(&self, res: reqwest::Response) -> Result<reqwest::Response> {
        if res.status().is_success() && res.headers().contains_key(AUTHN_HEADER) {
            let session_key = res.headers().get(AUTHN_HEADER).unwrap().to_str().map_err(|_| Error::MissingOrInvalidSessionKey)?.to_string();
            let mut key_holder = self.session_key.write().await;
            *key_holder = Some(session_key);
        }
        if !res.status().is_success() {
            warn!("HTTP error: {}", res.status());
            let body = res.text().await?;
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
            let req = self.build_post_request(svc, mo_type, mo_id, method_name, params);
            let req = self.prepare(req).await;
            let res = req.send().await?;
            let res = self.process_response(res).await?;
            let bytes = res.bytes().await?;
            if log::log_enabled!(log::Level::Trace) {
                let body = String::from_utf8_lossy(&bytes);
                trace!("JSON response from {}/{}: {}...", mo_type, method_name, &body[..body.len().min(2000)]);
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
            let req = self.build_post_request(svc, mo_type, mo_id, method_name, params);
            let req = self.prepare(req).await;
            let res = req.send().await?;
            let res = self.process_response(res).await?;
            let bytes = res.bytes().await?;
            if log::log_enabled!(log::Level::Trace) && !bytes.is_empty() {
                let body = String::from_utf8_lossy(&bytes);
                trace!("JSON response from {}/{}: {}...", mo_type, method_name, &body[..body.len().min(2000)]);
            }
            if bytes.is_empty() { Ok(None) } else { Ok(Some(bytes)) }
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
            let req = self.build_post_request(svc, mo_type, mo_id, method_name, params);
            JsonClient::execute_void(self, req).await
        })
    }

    fn fetch_property_raw<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        property: &'a str,
    ) -> BoxFuture<'a, Result<Option<Bytes>>> {
        Box::pin(async move {
            let path = if svc.is_empty() {
                format!("/{mo_type}/{mo_id}/{property}")
            } else {
                format!("/{svc}/{mo_type}/{mo_id}/{property}")
            };
            let req = self.get_request(&path);
            let req = self.prepare(req).await;
            let res = req.send().await?;
            let res = self.process_response(res).await?;
            let bytes = res.bytes().await?;
            if log::log_enabled!(log::Level::Trace) && !bytes.is_empty() {
                let body = String::from_utf8_lossy(&bytes);
                trace!("JSON fetch_property_raw {}/{}: {}...", mo_type, property, &body[..body.len().min(2000)]);
            }
            if bytes.is_empty() { Ok(None) } else { Ok(Some(bytes)) }
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
                let req = http_client.post(&path)
                                        .header(AUTHN_HEADER, key);
                match req.send().await {
                    Ok(resp) => {
                        let status = resp.status();
                        if status.is_success() {
                            debug!("Session logged out successfully");
                        } else {
                            match resp.text().await {
                                Ok(body) => {
                                    match miniserde::json::from_str::<structs::MethodFault>(&body) {
                                        Ok(fault) => warn!("Failed to logout session(HTTP code: {}). MethodFault: {:?}", status, fault),
                                        Err(_) => warn!("Failed to logout session(HTTP code: {}). Cannot parse MethodFault: {}", status, &body[..body.len().min(200)]),
                                    }
                                },
                                Err(e) => warn!("Failed to logout session(HTTP code: {}). Cannot read response: {}", status, e),
                            }
                        }
                    },
                    Err(e) => warn!("Failed to logout session. Cannot execute logout request: {}", e),
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