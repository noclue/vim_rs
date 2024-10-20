mod url_template;

use hyper::HeaderMap;
use url_template::substitute;
use vim::vim;

const AUTHN_HEADER: &str = "vmware-api-session-id";

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
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct VimClient {
    pub http_client: reqwest::Client,
    pub session_key: Option<String>,
    pub base_url: String,
}


impl VimClient {
    pub fn get(&self, path: &str) -> reqwest::RequestBuilder
    {
        let url = format!("{}{}", self.base_url, path);
        self.http_client.get(&url)
    }

    pub fn post<B>(&self, path: &str, payload: &B) -> reqwest::RequestBuilder
    where
        B: serde::Serialize,
    {
        let url = format!("{}{}", self.base_url, path);
        let req = self.http_client.post(&url);
        req.header("Content-Type", "application/json").json(payload)
    }

    pub fn post_bare(&self, path: &str) -> reqwest::RequestBuilder
    {
        let url = format!("{}{}", self.base_url, path);
        self.http_client.post(&url)
    }

    async fn execute<T>(&self, mut req: reqwest::RequestBuilder) -> Result<(T, HeaderMap)> 
    where T: serde::de::DeserializeOwned 
    {
        if let Some(session_key) = &self.session_key {
            req = req.header(AUTHN_HEADER, session_key);
        }
        let res = req.send().await?;
        if !res.status().is_success() {
            let fault: Box<dyn vim::MethodFaultTrait> = res.json().await?;
            return Err(Error::MethodFault(fault));
        }
        let headers = res.headers().clone();
        let content: T = res.json().await?;
        Ok((content, headers))
    }

    async fn execute_void(&self, mut req: reqwest::RequestBuilder) -> Result<()> 
    {
        if let Some(session_key) = &self.session_key {
            req = req.header(AUTHN_HEADER, session_key);
        }
        let res = req.send().await?;
        if !res.status().is_success() {
            let fault: Box<dyn vim::MethodFaultTrait> = res.json().await?;
            return Err(Error::MethodFault(fault));
        }
        Ok(())
    }

}

pub struct ServiceInstance<'a> {
    client: &'a VimClient,
    mo_id: String,
}

impl ServiceInstance<'_> {
    pub async fn get_content(&self) -> Result<vim::ServiceContent> {
        let path = substitute("/ServiceInstance/{moId}/content", &vec![("moId", &self.mo_id)]);
        let req = self.client.get(&path);
        Ok(self.client.execute(req).await?.0)
    }
}


pub struct SessionManager<'a> {
    client: &'a VimClient,
    mo_id: String,
}

impl SessionManager<'_> {
    pub async fn login(&self, user_name: &str, password: &str, locale: Option<&str>) -> Result<(vim::UserSession,VimClient)> {
        let login = vim::LoginRequestType {
            user_name: user_name.to_string(),
            password: password.to_string(),
            locale: locale.map(|s| s.to_string()),
        };
        let path = substitute("/SessionManager/{moId}/Login", &vec![("moId", &self.mo_id)]);
        let req = self.client.post(&path, &login);
        let (session, headers): (vim::UserSession, HeaderMap) = self.client.execute(req).await?;
        let Some(session_key) = headers.get(AUTHN_HEADER) else {
            return Err(Error::MissingOrInvalidSessionKey);
        };
        let client = VimClient {
            http_client: self.client.http_client.clone(),
            session_key: Some(session_key.to_str().map_err(|_| Error::MissingOrInvalidSessionKey)?.to_string()),
            base_url: self.client.base_url.clone(),
        };
        Ok((session, client))
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
    use std::env;

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
        let anonymous_client = super::VimClient {
            http_client: client,
            session_key: None,
            base_url: "https://vc8.home/sdk/vim25/8.0.2.0".to_string(),
        };
        let service_instance = super::ServiceInstance {
            client: &anonymous_client,
            mo_id: "ServiceInstance".to_string(),
        };
        let content = service_instance.get_content().await.unwrap();
        let session_manager_mo = content.session_manager.unwrap().value;
        let anon_session_manager = super::SessionManager {
            client: &anonymous_client,
            mo_id: session_manager_mo.clone(),
        };
        let (session, vim_client) = anon_session_manager.login(
                            &env::var("VC_USERNAME").unwrap(),
                            &env::var("VC_PASSWORD").unwrap(), 
                            None).await.unwrap();
        println!("{:?}", session);
        let session_manager = super::SessionManager {
            client: &vim_client,
            mo_id: session_manager_mo.clone(),
        };
        session_manager.logout().await.unwrap();
    }


}
