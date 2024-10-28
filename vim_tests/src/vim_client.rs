use tokio::sync::RwLock;
use vim::vim;

const AUTHN_HEADER: &str = "vmware-api-session-id";
const API_RELEASE: &str = "8.0.2.0";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("MethodFault: {0:?}")]
    MethodFault(Box<dyn vim::MethodFaultTrait>),
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

pub struct VimClient {
    pub http_client: reqwest::Client,
    pub session_key: RwLock<Option<String>>,
    pub base_url: String,
}

/// Client for the VI JSON API that handles basic HTTP requests and authentication headers
impl VimClient {

    /// Create a new client for the VI/JSON API
    /// 
    /// * `http_client` - reqwest::Client instance with preset TCP, TLS and HTTP options
    /// * `server_address` - vCenter server FQDN or IP address
    /// * `release` - vCenter API release version e.g. "8.0.3.0". If not provided, the default is 
    ///              API_RELEASE
    pub fn new(http_client: reqwest::Client, server_address: &str, release: Option<&str>) -> Self {
        let release = release.unwrap_or(API_RELEASE);
        // From the VI/JSON OpenAPI spec https://{vcenter-host}/sdk/vim25/{release}
        let base_url = format!("https://{}/sdk/vim25/{}", server_address, release);
        let session_key = RwLock::new(None);
        Self {
            http_client,
            session_key,
            base_url: base_url.to_string(),
        }
    }

    /// Prepare GET request
    pub fn get_request(&self, path: &str) -> reqwest::RequestBuilder
    {
        let url = format!("{}{}", self.base_url, path);
        self.http_client.get(&url)
    }

    /// Prepare POST request with a body
    pub fn post_request<B>(&self, path: &str, payload: &B) -> reqwest::RequestBuilder
    where
        B: serde::Serialize,
    {
        let url = format!("{}{}", self.base_url, path);
        let req = self.http_client.post(&url);
        req.header("Content-Type", "application/json").json(payload)
    }

    /// Prepare POST request without a body
    pub fn post_bare(&self, path: &str) -> reqwest::RequestBuilder
    {
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

    /// Execute a request that returns a response body
    pub async fn execute_option<T>(&self, mut req: reqwest::RequestBuilder) -> Result<Option<T>> 
    where T: serde::de::DeserializeOwned 
    {
        req = self.prepare(req).await;
        let res = req.send().await?;
        let res = self.process_response(res).await?;
        let bytes = res.bytes().await?;
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
            let fault: Box<dyn vim::MethodFaultTrait> = res.json().await?;
            return Err(Error::MethodFault(fault));
        }
        Ok(res)
    }

}
