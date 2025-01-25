use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::CustomFieldDef;
use crate::types::structs::CustomFieldValueTrait;
use crate::types::structs::HostVirtualNicManagerInfo;
use crate::types::structs::VirtualNicManagerNetConfig;
/// The VirtualNicManager managed object describes the special Virtual NIC
/// configuration of the host.
pub struct HostVirtualNicManager {
    client: Arc<Client>,
    mo_id: String,
}
impl HostVirtualNicManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deselect the VirtualNic to be a special type.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### nic_type
    /// The type of VirtualNic that would be deselected
    ///
    /// ### device
    /// The device that uniquely identifies the VirtualNic.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if nicType is invalid, device represents
    /// a nonexistent or invalid VirtualNic, or the VirtualNic is
    /// not selected
    /// 
    /// ***HostConfigFault***: for any other failure.
    pub async fn deselect_vnic_for_nic_type(&self, nic_type: &str, device: &str) -> Result<()> {
        let input = DeselectVnicForNicTypeRequestType {nic_type, device, };
        let path = format!("/HostVirtualNicManager/{moId}/DeselectVnicForNicType", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Get the NetConfig for the specified nicType
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### nic_type
    /// The *HostVirtualNicManagerNicType_enum*
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if nicType is invalid
    /// 
    /// ***HostConfigFault***: for any other failure.
    pub async fn query_net_config(&self, nic_type: &str) -> Result<Option<VirtualNicManagerNetConfig>> {
        let input = QueryNetConfigRequestType {nic_type, };
        let path = format!("/HostVirtualNicManager/{moId}/QueryNetConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Select the NicType of the VirtualNic.
    /// 
    /// Selecting a device automatically
    /// deselects the previous selection if *VirtualNicManagerNetConfig.multiSelectAllowed*
    /// is false for the specified nicType.
    /// Else, the device is added to the list of selected nics.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### nic_type
    /// The type of VirtualNic that would be selected
    ///
    /// ### device
    /// The device that uniquely identifies the VirtualNic.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if nicType is invalid, or device represents a
    /// nonexistent or invalid VirtualNic
    /// 
    /// ***HostConfigFault***: for any other failure.
    pub async fn select_vnic_for_nic_type(&self, nic_type: &str, device: &str) -> Result<()> {
        let input = SelectVnicForNicTypeRequestType {nic_type, device, };
        let path = format!("/HostVirtualNicManager/{moId}/SelectVnicForNicType", moId = &self.mo_id);
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
        let path = format!("/HostVirtualNicManager/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/HostVirtualNicManager/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Network configuration.
    pub async fn info(&self) -> Result<HostVirtualNicManagerInfo> {
        let path = format!("/HostVirtualNicManager/{moId}/info", moId = &self.mo_id);
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
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn CustomFieldValueTrait>>>> {
        let path = format!("/HostVirtualNicManager/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeselectVnicForNicTypeRequestType<'a> {
    #[serde(rename = "nicType")]
    nic_type: &'a str,
    device: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryNetConfigRequestType<'a> {
    #[serde(rename = "nicType")]
    nic_type: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SelectVnicForNicTypeRequestType<'a> {
    #[serde(rename = "nicType")]
    nic_type: &'a str,
    device: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
