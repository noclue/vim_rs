mod url_template;

use std::sync::Arc;

use tokio::sync::RwLock;
use url_template::substitute;
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
        Self {http_client, session_key, base_url}
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

pub struct ServiceInstance {
    client: Arc<VimClient>,
    mo_id: String,
}

impl ServiceInstance {
    pub async fn get_content(&self) -> Result<vim::ServiceContent> {
        let path = substitute("/ServiceInstance/{moId}/content", &vec![("moId", &self.mo_id)]);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}


pub struct SessionManager {
    client: Arc<VimClient>,
    mo_id: String,
}

impl SessionManager {
    pub fn new(client: Arc<VimClient>, mo_ref: &vim::ManagedObjectReference) -> Result<Self> {
        // TODO Add PartialEq to enums in vim.rs
        // if mo_ref.r#type != vim::MoTypesEnum::SessionManager {
        //     return Err(Error::InvalidObjectType(mo_ref.type_.clone(), "SessionManager".to_string()));
        // }
        Ok(Self {
            client,
            mo_id: mo_ref.value.clone(),
        })
    }
    pub async fn login(&self, user_name: &str, password: &str, locale: Option<&str>) -> Result<vim::UserSession> {
        let login = vim::LoginRequestType {
            user_name: user_name.to_string(),
            password: password.to_string(),
            locale: locale.map(|s| s.to_string()),
        };
        let path = substitute("/SessionManager/{moId}/Login", &vec![("moId", &self.mo_id)]);
        let req = self.client.post_request(&path, &login);
        let session: vim::UserSession = self.client.execute(req).await?;
        Ok(session)
    }
    pub async fn logout(&self) -> Result<()> {
        let path = substitute("/SessionManager/{moId}/Logout", &vec![("moId", &self.mo_id)]);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await?;
        Ok(())
    }

}



#[cfg(test)]
mod tests {
    use std::{env, sync::Arc};
    use super::{VimClient, ServiceInstance, SessionManager};

    use reqwest::ClientBuilder;
    use vim::vim;
    use ::vim::vim::{CastInto, VirtualDeviceTrait};

    #[test]
    fn it_works() {
        let login = vim::LoginRequestType{
            user_name: "administrator@vsphere.local".to_string(),
            password: "secret!".to_string(),
            locale: None,
        };
        let s = serde_json::to_string(&login).unwrap();
        println!("{}", s);
        let login1 = serde_json::from_str::<vim::LoginRequestType>(&s).unwrap();
        assert_eq!(login.user_name, login1.user_name);
        print!("login1: {:?}", login1);
    }

    #[test]
    fn test_ethernet() {
        let e1000 = vim::VirtualE1000 {
            mac_address: Some("00:50:56:aa:bb:cc".to_string()),
            wake_on_lan_enabled: Some(true),
            address_type: Some("Generated".to_string()),
            numa_node: None,
            device_group_info: None,
            resource_allocation: None,
            upt_compatibility_enabled: Some(false),
            external_id: None,
            unit_number: None,
            backing: None,
            controller_key: None,
            slot_info: None,
            key: 1000,
            device_info: None,
            connectable: None,
            dynamic_property: None,
        };
        let s = serde_json::to_string(&e1000).unwrap();
        println!("{}", s);
        let vd: Box<dyn VirtualDeviceTrait> = serde_json::from_str(&s).unwrap();
        assert_eq!(vd.get_key(), 1000);
        let eth: Box<dyn vim::VirtualEthernetCardTrait> = vd.into_box().unwrap();
        assert_eq!(*eth.get_mac_address(), Some("00:50:56:aa:bb:cc".to_string()));
        println!("{:?}", eth);
    }

    #[tokio::test]
    async fn get_content() {
        let builder = ClientBuilder::new();
        let client = builder.danger_accept_invalid_certs(true)
                                .danger_accept_invalid_hostnames(true)
                                .build()
                                .unwrap();
        let res = client.get("https://vc8.home/sdk/vim25/8.0.3.0/ServiceInstance/ServiceInstance/content").send().await.unwrap();
        if res.status() != 200 {
            let fault: Box<dyn vim::MethodFaultTrait> = res.json().await.unwrap();
            panic!("Failed to get content: {:?}", fault);
        }
        let content: vim::ServiceContent = res.json().await.unwrap();
        println!("{:?}", content.about);
    }

    #[tokio::test]
    async fn login() {
        let client = ClientBuilder::new()
                                .danger_accept_invalid_certs(true)
                                .danger_accept_invalid_hostnames(true)
                                .build()
                                .unwrap();
        let client = Arc::new(VimClient::new(client, "vc8.home", None));
        let service_instance = ServiceInstance {
            client: client.clone(),
            mo_id: "ServiceInstance".to_string(),
        };
        let content = service_instance.get_content().await.unwrap();
        let session_manager_mo_ref = content.session_manager.unwrap();
        let session_manager = SessionManager::new(client.clone(), &session_manager_mo_ref).unwrap();
        let session = session_manager.login(
                            &env::var("VC_USERNAME").unwrap(),
                            &env::var("VC_PASSWORD").unwrap(), 
                            None).await.unwrap();
        println!("{:?}", session);
        session_manager.logout().await.unwrap();
        println!("Logged out");
    }


}
