use std::sync::Arc;

use tokio::sync::RwLock;
use super::super::types::structs;
use log::{warn, debug, trace, log_enabled};
use log::Level::Trace;

use std::ffi::OsStr;
use crate::mo;
use crate::types::structs::ServiceContent;

const LIB_NAME: &str = env!("CARGO_PKG_NAME");
const LIB_VERSION: &str = env!("CARGO_PKG_VERSION");
// See build.rs for the RUSTC_VERSION
const RUSTC_VERSION: &str = env!("RUSTC_VERSION");

/// Compatible API releases i.e. current and older API releases that can be negotiated with a server
pub const COMPATIBLE_API_RELEASES: [&str; 2] = ["8.0.2.0", "8.0.1.0"];

/// The default API version found in the OpenAPI specification
pub const API_RELEASE: &str = "8.0.2.0";

/// The header key for the session key
const AUTHN_HEADER: &str = "vmware-api-session-id";

const SERVICE_INSTANCE_MOID: &str = "ServiceInstance";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("MethodFault: {0:?}")]
    MethodFault(structs::MethodFault),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Missing or Invalid session key")]
    MissingOrInvalidSessionKey,
    #[error("Invalid object type {0} expected: {1}")]
    InvalidObjectType(String, String),
    #[error("Cannot negotiate compatible API release. Attempted with: {0:?}")]
    CannotNegotiateAPIRelease(Vec<String>),
}

pub type Result<T> = std::result::Result<T, Error>;

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
                let req = http_client.post(&path)
                    .header("Content-Type", "application/json")
                    .header("User-Agent", &user_agent)
                    .json(&spec);
                let res = req.send().await?;
                let res = res.error_for_status()?;
                let result: HelloResult = res.json().await?;
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

    /// Get the service instance content
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

    /// Prepare GET request
    pub fn get_request(&self, path: &str) -> reqwest::RequestBuilder
    {
        debug!("GET request: {}", path);
        let url = format!("{}{}", self.base_url, path);
        self.http_client.get(&url)
    }

    /// Prepare POST request with a body
    pub fn post_request<B>(&self, path: &str, payload: &B) -> reqwest::RequestBuilder
    where
        B: serde::Serialize,
    {
        debug!("POST request: {}", path);
        let url = format!("{}{}", self.base_url, path);
        let req = self.http_client.post(&url);
        req.header("Content-Type", "application/json").json(payload)
    }

    /// Prepare POST request without a body
    pub fn post_bare(&self, path: &str) -> reqwest::RequestBuilder
    {
        debug!("POST request (void): {}", path);
        let url = format!("{}{}", self.base_url, path);
        self.http_client.post(&url)
    }

    /// Execute a request that returns a response body
    pub async fn execute<T>(&self, mut req: reqwest::RequestBuilder) -> Result<T> 
    where T: serde::de::DeserializeOwned 
    {
        req = self.prepare(req).await;
        let res = req.send().await?;
        let res = self.process_response(res).await?;
        let content: T = res.json().await?;
        Ok(content)
    }

    /// Execute a request that optionally returns a response body
    pub async fn execute_option<T>(&self, mut req: reqwest::RequestBuilder) -> Result<Option<T>> 
    where T: serde::de::DeserializeOwned 
    {
        req = self.prepare(req).await;
        let res = req.send().await?;
        let res = self.process_response(res).await?;
        let bytes = res.bytes().await?;
        if log_enabled!(Trace) {
            trace!("Response body: {}", std::str::from_utf8(&bytes).unwrap());
        }
        let r: serde_json::Result<T> = serde_json::from_slice(&bytes);
        let content = match r {
            Ok(c) => Some(c),
            Err(e) => {
                if e.is_eof() {
                    None
                } else {
                    return Err(Error::SerdeError(e));
                }
            },
        };
        Ok(content)
    }

    /// Execute a request that does not return a response body
    pub async fn execute_void(&self, mut req: reqwest::RequestBuilder) -> Result<()> 
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
            let fault: structs::MethodFault = res.json().await?;
            return Err(Error::MethodFault(fault));
        }
        Ok(res)
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
                            resp.json::<structs::MethodFault>().await.map(|fault| {
                                warn!("Failed to logout session(HTTP code: {}). MethodFault: {:?}", status, fault);
                            }).unwrap_or_else(|e| {
                                warn!("Failed to logout session(HTTP code: {}). Cannot parse MethodFault: {}", status, e);
                            });
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
#[derive(serde::Serialize, Debug)]
struct HelloSpec<'a> {
    /// List of API release IDs that the client can work with in order of preference. The server will select the first mutually supported release ID.
    api_releases: &'a Vec<String>,
}

/// The Hello System API response. This is not full-fledged binding but a simple response to
/// negotiate the API release version.
#[derive(serde::Deserialize, Debug)]
struct HelloResult {
    /// The ID of a mutually-supported API release. This ID should be used in subsequent API calls
    /// to the current vCenter system. If there is no mutually-supported API release, the value will
    /// be an empty string, e.g. "". Typically, this is a case where one of the parties is much
    /// older than the other party.
    api_release: String,
}