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

/// Object-safe client abstraction for generated managed-object stubs (`crate::mo::*`).
///
/// This trait intentionally mirrors the subset of `Client` used by the generated bindings.
/// It is **object-safe** so managed objects can store `Arc<dyn VimClient>`, enabling
/// wrappers/mocks for testing and instrumentation.
pub trait VimClient: Send + Sync {
    /// Access vSphere `ServiceContent` (root managed object references).
    fn service_content(&self) -> &ServiceContent;

    /// Prepare GET request.
    fn get_request(&self, path: &str) -> reqwest::RequestBuilder;

    /// Prepare POST request with a JSON payload.
    fn post_json(&self, path: &str, payload: &dyn miniserde::Serialize) -> reqwest::RequestBuilder;

    /// Prepare POST request without a body.
    fn post_bare(&self, path: &str) -> reqwest::RequestBuilder;

    /// Execute a request and return the raw response body.
    ///
    /// We return `Bytes` (not `Vec<u8>`) to avoid an extra copy of the response body.
    fn execute_bytes<'a>(&'a self, req: reqwest::RequestBuilder) -> BoxFuture<'a, Result<Bytes>>;

    /// Execute a request and return an optional JSON value (empty body -> `None`).
    fn execute_option_bytes<'a>(
        &'a self,
        req: reqwest::RequestBuilder,
    ) -> BoxFuture<'a, Result<Option<Bytes>>>;

    /// Execute a request that returns no response body.
    fn execute_void<'a>(&'a self, req: reqwest::RequestBuilder) -> BoxFuture<'a, Result<()>>;
}

/// Convenience handle type used by generated bindings.
pub type VimClientHandle = Arc<dyn VimClient>;

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
        }
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

    /// Build the client instance
    pub async fn build(self) -> Result<Arc<Client>> {
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

        let bootstrap = Arc::new(Client {
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
        let client = Arc::new(Client {
            http_client: http_client.clone(),
            session_key: session_key.clone(),
            api_release: api_release.clone(),
            base_url: base_url.clone(),
            user_agent: user_agent.clone(),
            service_content: Some(content),
        });


        if let (Some(ref sm_id), Some(ref user_name), Some(ref password)) = (sm_id, self.user_name, self.password) {
            let sm = mo::SessionManager::new(client.clone(), sm_id);
            let session = sm.login(user_name, password, self.locale.as_deref()).await?;
            debug!("Session created for: {:?}", session.user_name);
        }
        Ok(client)
    }
}

pub struct Client {
    http_client: reqwest::Client,
    session_key: Arc<RwLock<Option<String>>>,
    api_release: String,
    base_url: String,
    user_agent: String,
    service_content: Option<ServiceContent>,
}

/// Client for the VI JSON API that handles basic HTTP requests and authentication headers.
/// 
/// The client is responsible for managing the session key header and logging out the session when
/// the client is dropped.
impl Client {

    /// Get the VIM service instance content. This is the main part of the VI JSON API that contains
    /// the core virtualization objects and services. There are additional storage APIs under VSAN,
    /// PBM, VSLM and SMS. There is also ESX Agent Manager API for managing agent virtual machines.
    pub fn service_content(&self) -> &ServiceContent {
        // Safe to unwrap as the service_content is set during construction
        self.service_content.as_ref().unwrap()
    }

    /// Get the currently used API release. This may be lower than `API_RELEASE` and should be used
    /// to downgrade client expectations. For example if client is using library 8.0.3.0 with
    /// vCenter 8.0.1.0 the negotiated release will be 8.0.1.0 and the client should not call APIs
    /// or set parameters that are only available in 8.0.3.0.
    pub fn api_release(&self) -> String {
        self.api_release.clone()
    }

    /// Fetch a managed object property by name into user provided type. This method can be used
    /// with [`miniserde::json::Value`] to fetch the property as a dynamic JSON value. This enables
    /// lightweight albeit unsafe approach to explore the API and extract relevant pieces of data.
    pub async fn fetch_property<T>(&self, obj: ManagedObjectReference, property: &str) -> Result<T>
    where
        T: miniserde::Deserialize
    {
        let type_name: &str = obj.r#type.as_str();
        let id = &obj.value;
        let path = format!("/{type_name}/{id}/{property}");
        let req = self.get_request(&path);
        self.execute(req).await
    }

    /// Prepare GET request
    pub(crate) fn get_request(&self, path: &str) -> reqwest::RequestBuilder
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

    /// Execute a request that returns a response body
    pub(crate) async fn execute<T>(&self, mut req: reqwest::RequestBuilder) -> Result<T>
    where T: miniserde::Deserialize
    {
        req = self.prepare(req).await;
        let res = req.send().await?;
        let res = self.process_response(res).await?;
        let body = res.text().await?;
        let content: T = miniserde::json::from_str(&body)
            .map_err(|_| Error::ParseError(format!("Failed to parse {}", std::any::type_name::<T>())))?;
        Ok(content)
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

impl VimClient for Client {
    fn service_content(&self) -> &ServiceContent {
        Client::service_content(self)
    }

    fn get_request(&self, path: &str) -> reqwest::RequestBuilder {
        Client::get_request(self, path)
    }

    fn post_json(&self, path: &str, payload: &dyn miniserde::Serialize) -> reqwest::RequestBuilder {
        debug!("POST request: {}", path);
        let json_body = miniserde::json::to_string(payload);
        // Avoid logging the payload unless TRACE is enabled (can be huge).
        if log::log_enabled!(log::Level::Trace) {
            trace!("POST payload: {}", json_body);
        };
        let url = format!("{}{}", self.base_url, path);
        self.http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(json_body)
    }

    fn post_bare(&self, path: &str) -> reqwest::RequestBuilder {
        Client::post_bare(self, path)
    }

    fn execute_bytes<'a>(&'a self, mut req: reqwest::RequestBuilder) -> BoxFuture<'a, Result<Bytes>> {
        Box::pin(async move {
            req = self.prepare(req).await;
            let res = req.send().await?;
            let res = self.process_response(res).await?;
            let bytes = res.bytes().await?;
            Ok(bytes)
        })
    }

    fn execute_option_bytes<'a>(
        &'a self,
        mut req: reqwest::RequestBuilder,
    ) -> BoxFuture<'a, Result<Option<Bytes>>> {
        Box::pin(async move {
            req = self.prepare(req).await;
            let res = req.send().await?;
            let res = self.process_response(res).await?;
            let bytes = res.bytes().await?;
            if bytes.is_empty() {
                Ok(None)
            } else {
                Ok(Some(bytes))
            }
        })
    }

    fn execute_void<'a>(&'a self, req: reqwest::RequestBuilder) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move { Client::execute_void(self, req).await })
    }
}


/// Task called asynchronously during drop of the VimClient instance to logout the session if one
/// was created.
impl Drop for Client {
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