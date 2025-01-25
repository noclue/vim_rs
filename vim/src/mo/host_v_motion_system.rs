use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::CustomFieldDef;
use crate::types::structs::CustomFieldValueTrait;
use crate::types::structs::HostIpConfig;
use crate::types::structs::HostVMotionNetConfig;
/// Deprecated as of VI API 4.0, use *HostConfigManager.virtualNicManager*.
/// 
/// The VMotionSystem managed object describes the VMotion configuration
/// of the host.
pub struct HostVMotionSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostVMotionSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Indicate that no VirtualNic should be used for VMotion.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: is a failure occurred
    pub async fn deselect_vnic(&self) -> Result<()> {
        let path = format!("/HostVMotionSystem/{moId}/DeselectVnic", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Select the VirtualNic to be used for VMotion.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### device
    /// The device that uniquely identifies the VirtualNic.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if key represents a nonexistent or invalid VirtualNic.
    /// 
    /// ***HostConfigFault***: for any other failure
    pub async fn select_vnic(&self, device: &str) -> Result<()> {
        let input = SelectVnicRequestType {device, };
        let path = format!("/HostVMotionSystem/{moId}/SelectVnic", moId = &self.mo_id);
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
        let path = format!("/HostVMotionSystem/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update the IP configuration of VMotion VirtualNic.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### ip_config
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if no VirtualNic is selected for VMotion.
    /// 
    /// ***InvalidArgument***: if the IpConfig is invalid or cannot be used.
    /// 
    /// ***HostConfigFault***: for any other failure
    pub async fn update_ip_config(&self, ip_config: &HostIpConfig) -> Result<()> {
        let input = UpdateIpConfigRequestType {ip_config, };
        let path = format!("/HostVMotionSystem/{moId}/UpdateIpConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/HostVMotionSystem/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// IP configuration of the VMotion VirtualNic.
    pub async fn ip_config(&self) -> Result<Option<HostIpConfig>> {
        let path = format!("/HostVMotionSystem/{moId}/ipConfig", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// VMotion network configuration.
    pub async fn net_config(&self) -> Result<Option<HostVMotionNetConfig>> {
        let path = format!("/HostVMotionSystem/{moId}/netConfig", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn CustomFieldValueTrait>>>> {
        let path = format!("/HostVMotionSystem/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SelectVnicRequestType<'a> {
    device: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateIpConfigRequestType<'a> {
    #[serde(rename = "ipConfig")]
    ip_config: &'a HostIpConfig,
}
