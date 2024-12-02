use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::CustomFieldDef;
use crate::types::CustomFieldValueTrait;
use crate::types::HostGraphicsConfig;
use crate::types::HostGraphicsInfo;
use crate::types::HostSharedGpuCapabilities;
use crate::types::VirtualMachineVgpuDeviceInfo;
use crate::types::VirtualMachineVgpuProfileInfo;
/// This managed object manages the graphics state of the host.
pub struct HostGraphicsManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HostGraphicsManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Indicate if shared graphics device is active on the host.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn is_shared_graphics_active(&self) -> Result<bool> {
        let path = format!("/HostGraphicsManager/{moId}/IsSharedGraphicsActive", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Refresh the available graphics information.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn refresh_graphics_manager(&self) -> Result<()> {
        let path = format!("/HostGraphicsManager/{moId}/RefreshGraphicsManager", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_vgpu_device_info(&self) -> Result<Option<Vec<VirtualMachineVgpuDeviceInfo>>> {
        let path = format!("/HostGraphicsManager/{moId}/RetrieveVgpuDeviceInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_vgpu_profile_info(&self) -> Result<Option<Vec<VirtualMachineVgpuProfileInfo>>> {
        let path = format!("/HostGraphicsManager/{moId}/RetrieveVgpuProfileInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
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
        let path = format!("/HostGraphicsManager/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update graphics configuration
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// -
    pub async fn update_graphics_config(&self, config: &HostGraphicsConfig) -> Result<()> {
        let input = UpdateGraphicsConfigRequestType {config, };
        let path = format!("/HostGraphicsManager/{moId}/UpdateGraphicsConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/HostGraphicsManager/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Graphics Configuration
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn graphics_config(&self) -> Result<HostGraphicsConfig> {
        let path = format!("/HostGraphicsManager/{moId}/graphicsConfig", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Array of graphics information
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn graphics_info(&self) -> Result<Option<Vec<HostGraphicsInfo>>> {
        let path = format!("/HostGraphicsManager/{moId}/graphicsInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Array of shared passthru GPU capablities.
    /// 
    /// See also *HostSharedGpuCapabilities*.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn shared_gpu_capabilities(&self) -> Result<Option<Vec<HostSharedGpuCapabilities>>> {
        let path = format!("/HostGraphicsManager/{moId}/sharedGpuCapabilities", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Array of shared passthru GPU types.
    /// 
    /// These GPU types may be enabled
    /// when specific host hardware is present. Example values are "grid\_k120q"
    /// and "grid\_k240q".
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn shared_passthru_gpu_types(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/HostGraphicsManager/{moId}/sharedPassthruGpuTypes", moId = &self.mo_id);
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
        let path = format!("/HostGraphicsManager/{moId}/value", moId = &self.mo_id);
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
struct UpdateGraphicsConfigRequestType<'a> {
    config: &'a HostGraphicsConfig,
}
