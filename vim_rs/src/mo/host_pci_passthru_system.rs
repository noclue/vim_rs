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
        self.client.invoke_void("", "HostPciPassthruSystem", &self.mo_id, "Refresh", None).await
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
        self.client.invoke_void("", "HostPciPassthruSystem", &self.mo_id, "setCustomValue", Some(&input)).await
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
        self.client.invoke_void("", "HostPciPassthruSystem", &self.mo_id, "UpdatePassthruConfig", Some(&input)).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostPciPassthruSystem", &self.mo_id, "availableField").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Array of PciPassthru information
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn pci_passthru_info(&self) -> Result<Vec<Box<dyn crate::types::traits::HostPciPassthruInfoTrait>>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostPciPassthruSystem", &self.mo_id, "pciPassthruInfo").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property pciPassthruInfo was empty".to_string()))?;
        let result: Vec<Box<dyn crate::types::traits::HostPciPassthruInfoTrait>> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Array of Sriov Device Pool information
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn sriov_device_pool_info(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::HostSriovDevicePoolInfoTrait>>>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostPciPassthruSystem", &self.mo_id, "sriovDevicePoolInfo").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.fetch_property_raw("", "HostPciPassthruSystem", &self.mo_id, "value").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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

impl<'b, 'a> miniserde::ser::Map for UpdatePassthruConfigRequestTypeSer<'b, 'a> {
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
