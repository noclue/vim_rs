use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::HostSriovDevicePoolInfoTrait;
use crate::types::CustomFieldDef;
use crate::types::HostPciPassthruInfoTrait;
use crate::types::CustomFieldValueTrait;
use crate::types::HostPciPassthruConfigTrait;
/// This managed object manages the PciPassthru state of the host.
pub struct HostPciPassthruSystem {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HostPciPassthruSystem {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Refresh the available PciPassthru information.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn refresh(&self) -> Result<()> {
        let path = format!("/HostPciPassthruSystem/{moId}/Refresh", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
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
        let path = format!("/HostPciPassthruSystem/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the PciPassthru configuration, this will
    /// get called for the dependent device with the enabled
    /// bool set
    /// 
    /// ***Required privileges:*** Host.Config.PciPassthru
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// The new PciPassthru configuration information.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if an error occurs.
    pub async fn update_passthru_config(&self, config: &[Box<dyn HostPciPassthruConfigTrait>]) -> Result<()> {
        let input = UpdatePassthruConfigRequestType {config, };
        let path = format!("/HostPciPassthruSystem/{moId}/UpdatePassthruConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/HostPciPassthruSystem/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Array of PciPassthru information
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn pci_passthru_info(&self) -> Result<Vec<Box<dyn HostPciPassthruInfoTrait>>> {
        let path = format!("/HostPciPassthruSystem/{moId}/pciPassthruInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Array of Sriov Device Pool information
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn sriov_device_pool_info(&self) -> Result<Option<Vec<Box<dyn HostSriovDevicePoolInfoTrait>>>> {
        let path = format!("/HostPciPassthruSystem/{moId}/sriovDevicePoolInfo", moId = &self.mo_id);
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
        let path = format!("/HostPciPassthruSystem/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdatePassthruConfigRequestType<'a> {
    config: &'a [Box<dyn HostPciPassthruConfigTrait>],
}
