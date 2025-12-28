use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object manages the graphics state of the host.
#[derive(Clone)]
pub struct HostGraphicsManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostGraphicsManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let bytes = self.client.execute_bytes(req).await?;
        let result: bool = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Refresh the available graphics information.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn refresh_graphics_manager(&self) -> Result<()> {
        let path = format!("/HostGraphicsManager/{moId}/RefreshGraphicsManager", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_vgpu_device_info(&self) -> Result<Option<Vec<crate::types::structs::VirtualMachineVgpuDeviceInfo>>> {
        let path = format!("/HostGraphicsManager/{moId}/RetrieveVgpuDeviceInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::VirtualMachineVgpuDeviceInfo>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_vgpu_profile_info(&self) -> Result<Option<Vec<crate::types::structs::VirtualMachineVgpuProfileInfo>>> {
        let path = format!("/HostGraphicsManager/{moId}/RetrieveVgpuProfileInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::VirtualMachineVgpuProfileInfo>>(bytes.as_ref())?)),
            None => Ok(None),
        }
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
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Update graphics configuration
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// -
    pub async fn update_graphics_config(&self, config: &crate::types::structs::HostGraphicsConfig) -> Result<()> {
        let input = UpdateGraphicsConfigRequestType {config, };
        let path = format!("/HostGraphicsManager/{moId}/UpdateGraphicsConfig", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let path = format!("/HostGraphicsManager/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::CustomFieldDef>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Graphics Configuration
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn graphics_config(&self) -> Result<Option<crate::types::structs::HostGraphicsConfig>> {
        let path = format!("/HostGraphicsManager/{moId}/graphicsConfig", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::HostGraphicsConfig>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Array of graphics information
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn graphics_info(&self) -> Result<Option<Vec<crate::types::structs::HostGraphicsInfo>>> {
        let path = format!("/HostGraphicsManager/{moId}/graphicsInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::HostGraphicsInfo>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Array of shared passthru GPU capablities.
    /// 
    /// See also *HostSharedGpuCapabilities*.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn shared_gpu_capabilities(&self) -> Result<Option<Vec<crate::types::structs::HostSharedGpuCapabilities>>> {
        let path = format!("/HostGraphicsManager/{moId}/sharedGpuCapabilities", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::HostSharedGpuCapabilities>>(bytes.as_ref())?)),
            None => Ok(None),
        }
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
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<String>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let path = format!("/HostGraphicsManager/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>(bytes.as_ref())?)),
            None => Ok(None),
        }
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
    config: &'a crate::types::structs::HostGraphicsConfig,
}
