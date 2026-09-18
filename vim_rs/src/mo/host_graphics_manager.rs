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
        let bytes = self.client.invoke("", "HostGraphicsManager", &self.mo_id, "IsSharedGraphicsActive", None).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Refresh the available graphics information.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn refresh_graphics_manager(&self) -> Result<()> {
        self.client.invoke_void("", "HostGraphicsManager", &self.mo_id, "RefreshGraphicsManager", None).await
    }
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_vgpu_device_info(&self) -> Result<Option<Vec<crate::types::structs::VirtualMachineVgpuDeviceInfo>>> {
        let bytes_opt = self.client.invoke_optional("", "HostGraphicsManager", &self.mo_id, "RetrieveVgpuDeviceInfo", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_vgpu_profile_info(&self) -> Result<Option<Vec<crate::types::structs::VirtualMachineVgpuProfileInfo>>> {
        let bytes_opt = self.client.invoke_optional("", "HostGraphicsManager", &self.mo_id, "RetrieveVgpuProfileInfo", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        self.client.invoke_void("", "HostGraphicsManager", &self.mo_id, "setCustomValue", Some(&input)).await
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
        self.client.invoke_void("", "HostGraphicsManager", &self.mo_id, "UpdateGraphicsConfig", Some(&input)).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let pv_opt = self.client.fetch_property_raw("", "HostGraphicsManager", &self.mo_id, "availableField").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Graphics Configuration
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn graphics_config(&self) -> Result<Option<crate::types::structs::HostGraphicsConfig>> {
        let pv_opt = self.client.fetch_property_raw("", "HostGraphicsManager", &self.mo_id, "graphicsConfig").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Array of graphics information
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn graphics_info(&self) -> Result<Option<Vec<crate::types::structs::HostGraphicsInfo>>> {
        let pv_opt = self.client.fetch_property_raw("", "HostGraphicsManager", &self.mo_id, "graphicsInfo").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Array of shared passthru GPU capabilities.
    /// 
    /// See also *HostSharedGpuCapabilities*.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn shared_gpu_capabilities(&self) -> Result<Option<Vec<crate::types::structs::HostSharedGpuCapabilities>>> {
        let pv_opt = self.client.fetch_property_raw("", "HostGraphicsManager", &self.mo_id, "sharedGpuCapabilities").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
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
        let pv_opt = self.client.fetch_property_raw("", "HostGraphicsManager", &self.mo_id, "sharedPassthruGpuTypes").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
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
        let pv_opt = self.client.fetch_property_raw("", "HostGraphicsManager", &self.mo_id, "value").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> miniserde::Serialize for SetCustomValueRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetCustomValueRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetCustomValueRequestTypeSer<'b, 'a> {
    data: &'b SetCustomValueRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetCustomValueRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"setCustomValueRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("value"), &self.data.value as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateGraphicsConfigRequestType<'a> {
    config: &'a crate::types::structs::HostGraphicsConfig,
}

impl<'a> miniserde::Serialize for UpdateGraphicsConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateGraphicsConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateGraphicsConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateGraphicsConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateGraphicsConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateGraphicsConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
