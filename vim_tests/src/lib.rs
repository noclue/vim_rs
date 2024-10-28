mod url_template;
mod vim_client;

use std::sync::Arc;
use url_template::substitute;
use vim::vim;
use ::vim::vim::ManagedObjectReference;
use vim_client::{VimClient, Result};
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


pub struct AlarmManager {
    client: Arc<VimClient>,
    mo_id: String,
}

/// The parameters of *AlarmManager.GetAlarm*.
#[derive(Debug, serde::Serialize)]
#[serde(tag="_typeName")]
pub struct GetAlarmRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: &'a Option<vim::ManagedObjectReference>,
}

impl AlarmManager {
    pub fn new(client: Arc<VimClient>, mo_ref: &vim::ManagedObjectReference) -> Result<Self> {
        Ok(Self {
            client,
            mo_id: mo_ref.value.clone(),
        })
    }
    pub async fn get_alarm(&self, entity: &Option<vim::ManagedObjectReference>) -> Result<Option<Vec<vim::ManagedObjectReference>>> {
        let body = GetAlarmRequestType {
            entity: entity,
        };
        let path = substitute("/AlarmManager/{moId}/GetAlarm", &vec![("moId", &self.mo_id)]);
        let req = self.client.post_request(&path, &body);
        Ok(self.client.execute_option(req).await?)
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

        let alarm_manager_mo_ref = content.alarm_manager.unwrap();
        let alarm_manager = super::AlarmManager::new(client.clone(), &alarm_manager_mo_ref).unwrap();
        let entity = Some(vim::ManagedObjectReference {
            r#type: vim::MoTypesEnum::VirtualMachine,
            value: "vm-1".to_string(),
        });
        let alarm = alarm_manager.get_alarm(&entity).await.unwrap();
        println!("{:?}", alarm);

        session_manager.logout().await.unwrap();
        println!("Logged out");
    }


}
