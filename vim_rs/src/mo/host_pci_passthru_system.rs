use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object manages the PciPassthru state of the host.
#[derive(Clone)]
pub struct HostPciPassthruSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostPciPassthruSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        self.client.execute_void(req).await
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
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn update_passthru_config(&self, config: &[Box<dyn crate::types::traits::HostPciPassthruConfigTrait>]) -> Result<()> {
        let input = UpdatePassthruConfigRequestType {config, };
        let path = format!("/HostPciPassthruSystem/{moId}/UpdatePassthruConfig", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let path = format!("/HostPciPassthruSystem/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::CustomFieldDef>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Array of PciPassthru information
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn pci_passthru_info(&self) -> Result<Vec<Box<dyn crate::types::traits::HostPciPassthruInfoTrait>>> {
        let path = format!("/HostPciPassthruSystem/{moId}/pciPassthruInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Vec<Box<dyn crate::types::traits::HostPciPassthruInfoTrait>> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Array of Sriov Device Pool information
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn sriov_device_pool_info(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::HostSriovDevicePoolInfoTrait>>>> {
        let path = format!("/HostPciPassthruSystem/{moId}/sriovDevicePoolInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::HostSriovDevicePoolInfoTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let path = format!("/HostPciPassthruSystem/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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

impl miniserde::ser::Map for SetCustomValueRequestTypeSer<'_, '_> {
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
struct UpdatePassthruConfigRequestType<'a> {
    config: &'a [Box<dyn crate::types::traits::HostPciPassthruConfigTrait>],
}

impl<'a> miniserde::Serialize for UpdatePassthruConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdatePassthruConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdatePassthruConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdatePassthruConfigRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for UpdatePassthruConfigRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdatePassthruConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
