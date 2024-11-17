mod url_template;
mod vim_client;

use std::sync::Arc;
use url_template::substitute;
use vim::types;
use vim::types::ManagedObjectReference;
use vim_client::{VimClient, Result};
pub struct ServiceInstance {
    client: Arc<VimClient>,
    mo_id: String,
}

impl ServiceInstance {
    pub async fn get_content(&self) -> Result<types::ServiceContent> {
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
    pub fn new(client: Arc<VimClient>, mo_ref: &ManagedObjectReference) -> Result<Self> {
        // TODO Add PartialEq to enums in vim.rs
        // if mo_ref.r#type != vim::MoTypesEnum::SessionManager {
        //     return Err(Error::InvalidObjectType(mo_ref.type_.clone(), "SessionManager".to_string()));
        // }
        Ok(Self {
            client,
            mo_id: mo_ref.value.clone(),
        })
    }
    pub async fn login(&self, user_name: &str, password: &str, locale: Option<&str>) -> Result<types::UserSession> {
        let login = LoginRequestType {user_name,password,locale,};
        let path = format!("/SessionManager/{moId}/Login", moId = &self.mo_id);
        let req = self.client.post_request(&path, &login);
        let session: types::UserSession = self.client.execute(req).await?;
        Ok(session)
    }
    pub async fn logout(&self) -> Result<()> {
        let path = format!("/SessionManager/{moId}/Logout", moId =&self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await?;
        Ok(())
    }

}

/// The parameters of *SessionManager.Login*.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag="_typeName")]
pub struct LoginRequestType<'a> {
    // Fields of LoginRequestType
    /// The *ID*
    /// of the user who is logging on to the server.
    #[serde(rename = "userName")]
    pub user_name: &'a str,
    /// The *HostAccountSpec.password*
    /// of the user who is logging on to the server.
    pub password: &'a str,
    /// A two-character ISO-639 language ID (like "en")
    /// optionally followed by an
    /// underscore and a two-character ISO 3166 country ID (like "US").
    /// 
    /// Examples are "de", "fr\_CA", "zh", "zh\_CN", and "zh\_TW".
    /// Note: The method uses the server default locale when
    /// a locale is not provided. This default can be configured in the
    /// server configuration file. If unspecified, it defaults to the
    /// locale of the server environment or English ("en") if unsupported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<&'a str>,
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
    pub entity: &'a Option<types::ManagedObjectReference>,
}

impl AlarmManager {
    pub fn new(client: Arc<VimClient>, mo_ref: &types::ManagedObjectReference) -> Result<Self> {
        Ok(Self {
            client,
            mo_id: mo_ref.value.clone(),
        })
    }
    pub async fn get_alarm(&self, entity: &Option<types::ManagedObjectReference>) -> Result<Option<Vec<types::ManagedObjectReference>>> {
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
    use vim::types;
    use vim::types::{CastInto, VirtualDeviceTrait};

    #[test]
    fn it_works() {
        let login = super::LoginRequestType{
            user_name: "administrator@vsphere.local",
            password: "secret!",
            locale: None,
        };
        let s = serde_json::to_string(&login).unwrap();
        println!("{}", s);
        let login1 = serde_json::from_str::<super::LoginRequestType>(&s).unwrap();
        assert_eq!(login.user_name, login1.user_name);
        print!("login1: {:?}", login1);
    }

    #[test]
    fn test_ethernet() {
        let e1000 = types::VirtualE1000 {
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
        let eth: Box<dyn types::VirtualEthernetCardTrait> = vd.into_box().unwrap();
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
            let fault: Box<dyn types::MethodFaultTrait> = res.json().await.unwrap();
            panic!("Failed to get content: {:?}", fault);
        }
        let content: types::ServiceContent = res.json().await.unwrap();
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
        let entity = Some(types::ManagedObjectReference {
            r#type: types::MoTypesEnum::VirtualMachine,
            value: "vm-1".to_string(),
        });
        let alarm = alarm_manager.get_alarm(&entity).await.unwrap();
        println!("{:?}", alarm);

        session_manager.logout().await.unwrap();
        println!("Logged out");
    }


}
