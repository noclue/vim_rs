use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::CustomFieldDef;
use crate::types::ServiceConsoleReservationInfo;
use crate::types::VirtualMachineMemoryReservationInfo;
use crate::types::VirtualMachineMemoryReservationSpec;
use crate::types::CustomFieldValueTrait;
/// The MemoryManagerSystem managed object provides an interface through which
/// the host memory management policies that affect the performance of running
/// virtual machines can be gathered and configured.
pub struct HostMemorySystem {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HostMemorySystem {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the virtual machine reservation information.
    /// 
    /// ***Required privileges:*** Host.Config.Memory
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// -
    pub async fn reconfigure_virtual_machine_reservation(&self, spec: &VirtualMachineMemoryReservationSpec) -> Result<()> {
        let input = ReconfigureVirtualMachineReservationRequestType {spec, };
        let path = format!("/HostMemorySystem/{moId}/ReconfigureVirtualMachineReservation", moId = &self.mo_id);
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
        let path = format!("/HostMemorySystem/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/HostMemorySystem/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Service console reservation information for the memory manager.
    /// 
    /// The
    /// existence of this data object indicates if the service console memory
    /// reservation must be configured for this host.
    pub async fn console_reservation_info(&self) -> Result<ServiceConsoleReservationInfo> {
        let path = format!("/HostMemorySystem/{moId}/consoleReservationInfo", moId = &self.mo_id);
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
        let path = format!("/HostMemorySystem/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Virtual machine reservation information for the memory manager.
    /// 
    /// The
    /// existence of this data object indicates if the virtual machine memory
    /// reservation must be configured for this host.
    pub async fn virtual_machine_reservation_info(&self) -> Result<VirtualMachineMemoryReservationInfo> {
        let path = format!("/HostMemorySystem/{moId}/virtualMachineReservationInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigureServiceConsoleReservationRequestType {
    #[serde(rename = "cfgBytes")]
    cfg_bytes: i64,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigureVirtualMachineReservationRequestType<'a> {
    spec: &'a VirtualMachineMemoryReservationSpec,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
