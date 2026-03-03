use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The MemoryManagerSystem managed object provides an interface through which
/// the host memory management policies that affect the performance of running
/// virtual machines can be gathered and configured.
#[derive(Clone)]
pub struct HostMemorySystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostMemorySystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Sets the configured service console memory reservation.
    /// 
    /// This change
    /// affects only the serviceConsoleReservedCfg property. The
    /// configuration change
    /// propagates to the other properties after the next boot.
    /// 
    /// ***Required privileges:*** Host.Config.Memory
    ///
    /// ## Parameters:
    ///
    /// ### cfg_bytes
    /// -
    pub async fn reconfigure_service_console_reservation(&self, cfg_bytes: i64) -> Result<()> {
        let input = ReconfigureServiceConsoleReservationRequestType {cfg_bytes, };
        let path = format!("/HostMemorySystem/{moId}/ReconfigureServiceConsoleReservation", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Updates the virtual machine reservation information.
    /// 
    /// ***Required privileges:*** Host.Config.Memory
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// -
    pub async fn reconfigure_virtual_machine_reservation(&self, spec: &crate::types::structs::VirtualMachineMemoryReservationSpec) -> Result<()> {
        let input = ReconfigureVirtualMachineReservationRequestType {spec, };
        let path = format!("/HostMemorySystem/{moId}/ReconfigureVirtualMachineReservation", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
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
        let path = format!("/HostMemorySystem/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let path = format!("/HostMemorySystem/{moId}/availableField", moId = &self.mo_id);
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
    /// Service console reservation information for the memory manager.
    /// 
    /// The
    /// existence of this data object indicates if the service console memory
    /// reservation must be configured for this host.
    pub async fn console_reservation_info(&self) -> Result<Option<crate::types::structs::ServiceConsoleReservationInfo>> {
        let path = format!("/HostMemorySystem/{moId}/consoleReservationInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::ServiceConsoleReservationInfo>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
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
        let path = format!("/HostMemorySystem/{moId}/value", moId = &self.mo_id);
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
    /// Virtual machine reservation information for the memory manager.
    /// 
    /// The
    /// existence of this data object indicates if the virtual machine memory
    /// reservation must be configured for this host.
    pub async fn virtual_machine_reservation_info(&self) -> Result<Option<crate::types::structs::VirtualMachineMemoryReservationInfo>> {
        let path = format!("/HostMemorySystem/{moId}/virtualMachineReservationInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::VirtualMachineMemoryReservationInfo>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct ReconfigureServiceConsoleReservationRequestType {
    cfg_bytes: i64,
}

impl miniserde::Serialize for ReconfigureServiceConsoleReservationRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigureServiceConsoleReservationRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigureServiceConsoleReservationRequestTypeSer<'b> {
    data: &'b ReconfigureServiceConsoleReservationRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for ReconfigureServiceConsoleReservationRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigureServiceConsoleReservationRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cfgBytes"), &self.data.cfg_bytes as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ReconfigureVirtualMachineReservationRequestType<'a> {
    spec: &'a crate::types::structs::VirtualMachineMemoryReservationSpec,
}

impl<'a> miniserde::Serialize for ReconfigureVirtualMachineReservationRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigureVirtualMachineReservationRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigureVirtualMachineReservationRequestTypeSer<'b, 'a> {
    data: &'b ReconfigureVirtualMachineReservationRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReconfigureVirtualMachineReservationRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigureVirtualMachineReservationRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
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
