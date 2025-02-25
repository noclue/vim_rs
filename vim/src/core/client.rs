use std::sync::Arc;

use tokio::sync::RwLock;
use super::super::types::structs;
use super::super::types::traits::MethodFaultTrait;
use log::{warn, debug, trace, log_enabled};
use log::Level::Trace;

use std::ffi::OsStr;

const LIB_NAME: &str = env!("CARGO_PKG_NAME");
const LIB_VERSION: &str = env!("CARGO_PKG_VERSION");
const RUSTC_VERSION: &str = env!("RUSTC_VERSION");


/// The API version found in the OpenAPI specification
const API_RELEASE: &str = "8.0.2.0";
/// The header key for the session key
const AUTHN_HEADER: &str = "vmware-api-session-id";


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("MethodFault: {0:?}")]
    MethodFault(Box<dyn MethodFaultTrait>),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Missing or Invalid session key")]
    MissingOrInvalidSessionKey,
    #[error("Invalid object type {0} expected: {1}")]
    InvalidObjectType(String, String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Client {
    http_client: Arc<reqwest::Client>,
    session_key: Arc<RwLock<Option<String>>>,
    base_url: String,
    user_agent: Arc<RwLock<String>>,
}

/// Client for the VI JSON API that handles basic HTTP requests and authentication headers.
/// 
/// The client is responsible for managing the session key header and logging out the session when
/// the client is dropped.
impl Client {

    /// Create a new client for the VI/JSON API
    /// 
    /// * `http_client` - reqwest::Client instance with preset TCP, TLS and HTTP options
    /// * `server_address` - vCenter server FQDN or IP address
    /// * `release` - vCenter API release version e.g. "8.0.3.0". If not provided, the default is 
    ///              API_RELEASE
    pub fn new(http_client: reqwest::Client, server_address: &str, release: Option<&str>) -> Arc<Self> {
        let release = release.unwrap_or(API_RELEASE);
        // From the VI/JSON OpenAPI spec https://{vcenter-host}/sdk/vim25/{release}
        let base_url = format!("https://{}/sdk/vim25/{}", server_address, release);
        let session_key = RwLock::new(None);
        let user_agent = user_agent(None, None);
        let res = Self {
            http_client: Arc::new(http_client),
            session_key: Arc::new(session_key),
            base_url: base_url.to_string(),
            user_agent: Arc::new(RwLock::new(user_agent)),
        };
        Arc::new(res)
    }

    pub async fn set_app_name(&self, app_name: &str) {
        let mut key_holder = self.user_agent.write().await;
        *key_holder = user_agent(Some(app_name), None);
    }

    pub async fn set_app_details(&self, app_name: &str, app_version: &str) {
        let mut key_holder = self.user_agent.write().await;
        *key_holder = user_agent(Some(app_name), Some(app_version));
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
        let user_agent = self.user_agent.read().await;
        let user_agent = user_agent.clone();
        req = req.header("User-Agent", &user_agent);
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
            let fault: Box<dyn MethodFaultTrait> = res.json().await?;
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
        let http_client = Arc::clone(&self.http_client);
        let base_url = self.base_url.clone();

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                debug!("Terminating VIM session as needed.");
                let key = {
                    let session_key = session_key.read().await;
                    session_key.clone()
                };
                if let Some(key) = key {
                    debug!("Session is present. Sending logout request...");

                    // TODO: get the session manager moId from the service instance
                    let path = format!("{base_url}/SessionManager/{moId}/Logout",
                                        base_url = base_url,
                                        moId = "SessionManager");
                    let req = http_client.post(&path)
                                            .header(AUTHN_HEADER, key);

                    match req.send().await {
                        Ok(resp) => {
                            let status = resp.status();
                            debug!("Session logout request sent. Response code is: {}", status);
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
                }
            });
        });
    }
}

fn user_agent(app_name: Option<&str>, app_version: Option<&str>) -> String {
    let app_name: String = if app_name.is_some() {
        app_name.unwrap().into()
    } else {
        get_executable_name().unwrap_or_else(|| "unknown".into())
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