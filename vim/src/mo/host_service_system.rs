use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::CustomFieldDef;
use crate::types::structs::HostServiceInfo;
/// The *HostServiceSystem* managed object describes the configuration
/// of host services.
/// 
/// This managed object operates in conjunction
/// with the *HostFirewallSystem*
/// managed object.
pub struct HostServiceSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostServiceSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Refresh the service information and settings to pick up any changes
    /// made directly on the host.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    pub async fn refresh_services(&self) -> Result<()> {
        let path = format!("/HostServiceSystem/{moId}/RefreshServices", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Restarts the service.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Service identifier
    /// (*HostServiceSystem.serviceInfo*.*HostServiceInfo.service*.*HostService.key*).
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the service ID is unknown.
    /// 
    /// ***InvalidState***: if the service is not running.
    /// Only hosts with ESX/ESXi 4.1 or earlier software use this fault.
    /// Hosts running later versions of ESXi do not throw a fault in this case.
    /// 
    /// ***HostConfigFault***: for all other failures.
    pub async fn restart_service(&self, id: &str) -> Result<()> {
        let input = RestartServiceRequestType {id, };
        let path = format!("/HostServiceSystem/{moId}/RestartService", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Assigns a value to a custom field.
    /// 
    /// The setCustomValue method requires
    /// whichever updatePrivilege is defined as one of the
    /// *CustomFieldDef.fieldInstancePrivileges*
    /// for the CustomFieldDef whose value is being changed.
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The name of the field whose value is to be updated.
    ///
    /// ### value
    /// Value to be assigned to the custom field.
    pub async fn set_custom_value(&self, key: &str, value: &str) -> Result<()> {
        let input = SetCustomValueRequestType {key, value, };
        let path = format!("/HostServiceSystem/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Starts the service.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Service identifier
    /// (*HostServiceSystem.serviceInfo*.*HostServiceInfo.service*.*HostService.key*).
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the service ID is unknown.
    /// 
    /// ***InvalidState***: if the service is already running.
    /// Only hosts with ESX/ESXi 4.1 or earlier software use this fault.
    /// Hosts running later versions of ESXi do not throw a fault in this case.
    /// 
    /// ***HostConfigFault***: for all other failures.
    pub async fn start_service(&self, id: &str) -> Result<()> {
        let input = StartServiceRequestType {id, };
        let path = format!("/HostServiceSystem/{moId}/StartService", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Stops the service.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Service identifier
    /// (*HostServiceSystem.serviceInfo*.*HostServiceInfo.service*.*HostService.key*).
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the service ID is unknown.
    /// 
    /// ***InvalidState***: if the service is not running.
    /// Only hosts with ESX/ESXi 4.1 or earlier software use this fault.
    /// Hosts running later versions of ESXi do not throw a fault in this case.
    /// 
    /// ***HostConfigFault***: for all other failures.
    pub async fn stop_service(&self, id: &str) -> Result<()> {
        let input = StopServiceRequestType {id, };
        let path = format!("/HostServiceSystem/{moId}/StopService", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Uninstalls the service.
    /// 
    /// If the service is running, it is
    /// stopped before being uninstalled.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Service identifier
    /// (*HostServiceSystem.serviceInfo*.*HostServiceInfo.service*.*HostService.key*).
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the service ID is unknown.
    /// 
    /// ***InvalidArgument***: if the service is a required service.
    /// 
    /// ***NotSupported***: if the service doesn't support uninstallation.
    /// 
    /// ***HostConfigFault***: for all other failures.
    pub async fn uninstall_service(&self, id: &str) -> Result<()> {
        let input = UninstallServiceRequestType {id, };
        let path = format!("/HostServiceSystem/{moId}/UninstallService", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the activation policy of the service.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Service identifier
    /// (*HostServiceSystem.serviceInfo*.*HostServiceInfo.service*.*HostService.key*).
    ///
    /// ### policy
    /// Specifies the condition for service activation.
    /// Use one of the *HostServicePolicy_enum* values.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the service ID is unknown.
    /// 
    /// ***InvalidArgument***: if the service ID represents a required service,
    /// or if the specified policy is undefined.
    /// 
    /// ***HostConfigFault***: for all other failures.
    pub async fn update_service_policy(&self, id: &str, policy: &str) -> Result<()> {
        let input = UpdateServicePolicyRequestType {id, policy, };
        let path = format!("/HostServiceSystem/{moId}/UpdateServicePolicy", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/HostServiceSystem/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Service configuration.
    pub async fn service_info(&self) -> Result<HostServiceInfo> {
        let path = format!("/HostServiceSystem/{moId}/serviceInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let path = format!("/HostServiceSystem/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RestartServiceRequestType<'a> {
    id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct StartServiceRequestType<'a> {
    id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct StopServiceRequestType<'a> {
    id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UninstallServiceRequestType<'a> {
    id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateServicePolicyRequestType<'a> {
    id: &'a str,
    policy: &'a str,
}
