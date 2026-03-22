use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The VirtualNicManager managed object describes the special Virtual NIC
/// configuration of the host.
#[derive(Clone)]
pub struct HostVirtualNicManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostVirtualNicManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        self.client.invoke_void("", "HostVirtualNicManager", &self.mo_id, "DeselectVnicForNicType", Some(&input)).await
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
    pub async fn query_net_config(&self, nic_type: &str) -> Result<Option<crate::types::structs::VirtualNicManagerNetConfig>> {
        let input = QueryNetConfigRequestType {nic_type, };
        let bytes_opt = self.client.invoke_optional("", "HostVirtualNicManager", &self.mo_id, "QueryNetConfig", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        self.client.invoke_void("", "HostVirtualNicManager", &self.mo_id, "SelectVnicForNicType", Some(&input)).await
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
        self.client.invoke_void("", "HostVirtualNicManager", &self.mo_id, "setCustomValue", Some(&input)).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let pv_opt = self.client.fetch_property_raw("", "HostVirtualNicManager", &self.mo_id, "availableField").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Network configuration.
    pub async fn info(&self) -> Result<crate::types::structs::HostVirtualNicManagerInfo> {
        let pv_opt = self.client.fetch_property_raw("", "HostVirtualNicManager", &self.mo_id, "info").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property info was empty".to_string()))?;
        let result: crate::types::structs::HostVirtualNicManagerInfo = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let pv_opt = self.client.fetch_property_raw("", "HostVirtualNicManager", &self.mo_id, "value").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct DeselectVnicForNicTypeRequestType<'a> {
    nic_type: &'a str,
    device: &'a str,
}

impl<'a> miniserde::Serialize for DeselectVnicForNicTypeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeselectVnicForNicTypeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeselectVnicForNicTypeRequestTypeSer<'b, 'a> {
    data: &'b DeselectVnicForNicTypeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeselectVnicForNicTypeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeselectVnicForNicTypeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("nicType"), &self.data.nic_type as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("device"), &self.data.device as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryNetConfigRequestType<'a> {
    nic_type: &'a str,
}

impl<'a> miniserde::Serialize for QueryNetConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryNetConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryNetConfigRequestTypeSer<'b, 'a> {
    data: &'b QueryNetConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryNetConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryNetConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("nicType"), &self.data.nic_type as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SelectVnicForNicTypeRequestType<'a> {
    nic_type: &'a str,
    device: &'a str,
}

impl<'a> miniserde::Serialize for SelectVnicForNicTypeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SelectVnicForNicTypeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SelectVnicForNicTypeRequestTypeSer<'b, 'a> {
    data: &'b SelectVnicForNicTypeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SelectVnicForNicTypeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SelectVnicForNicTypeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("nicType"), &self.data.nic_type as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("device"), &self.data.device as &dyn miniserde::Serialize)),
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
