use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Deprecated as of VI API 4.0, use *HostConfigManager.virtualNicManager*.
/// 
/// The VMotionSystem managed object describes the VMotion configuration
/// of the host.
#[derive(Clone)]
pub struct HostVMotionSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostVMotionSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        self.client.invoke_void("", "HostVMotionSystem", &self.mo_id, "DeselectVnic", None).await
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
        self.client.invoke_void("", "HostVMotionSystem", &self.mo_id, "SelectVnic", Some(&input)).await
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
        self.client.invoke_void("", "HostVMotionSystem", &self.mo_id, "setCustomValue", Some(&input)).await
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
    pub async fn update_ip_config(&self, ip_config: &dyn crate::types::traits::HostIpConfigTrait) -> Result<()> {
        let input = UpdateIpConfigRequestType {ip_config, };
        self.client.invoke_void("", "HostVMotionSystem", &self.mo_id, "UpdateIpConfig", Some(&input)).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let pv_opt = self.client.fetch_property_raw("", "HostVMotionSystem", &self.mo_id, "availableField").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// IP configuration of the VMotion VirtualNic.
    pub async fn ip_config(&self) -> Result<Option<Box<dyn crate::types::traits::HostIpConfigTrait>>> {
        let pv_opt = self.client.fetch_property_raw("", "HostVMotionSystem", &self.mo_id, "ipConfig").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// VMotion network configuration.
    pub async fn net_config(&self) -> Result<Option<crate::types::structs::HostVMotionNetConfig>> {
        let pv_opt = self.client.fetch_property_raw("", "HostVMotionSystem", &self.mo_id, "netConfig").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "HostVMotionSystem", &self.mo_id, "value").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct SelectVnicRequestType<'a> {
    device: &'a str,
}

impl<'a> miniserde::Serialize for SelectVnicRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SelectVnicRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SelectVnicRequestTypeSer<'b, 'a> {
    data: &'b SelectVnicRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SelectVnicRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SelectVnicRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("device"), &self.data.device as &dyn miniserde::Serialize)),
            _ => return None,
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
struct UpdateIpConfigRequestType<'a> {
    ip_config: &'a dyn crate::types::traits::HostIpConfigTrait,
}

impl<'a> miniserde::Serialize for UpdateIpConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateIpConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateIpConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateIpConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateIpConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateIpConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("ipConfig"), &self.data.ip_config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
