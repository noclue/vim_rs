use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostAssignableHardwareBinding;
use crate::types::structs::HostAssignableHardwareConfig;
use crate::types::structs::VirtualMachineDynamicPassthroughInfo;
use crate::types::structs::VirtualMachineVendorDeviceGroupInfo;
/// This managed object manages the assignable hardware state of the host.
pub struct HostAssignableHardwareManager {
    client: Arc<Client>,
    mo_id: String,
}
impl HostAssignableHardwareManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Download Assignable Hardware description tree.
    /// 
    /// The size of the downloaded description tree is dependent on the
    /// type and number of assignable devices found on the host. As a
    /// rough estimate, each device might require approximate 256 bytes
    /// to represent.
    /// The number of assignable devices on a host may vary from none to
    /// 60 or more. A host with 3 SRIOV type devices consisting
    /// of a PF and 16 VFs would have a total of 51 devices and the
    /// description tree would be approximately 51 \* 256 bytes = 13,056 bytes.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn download_description_tree(&self) -> Result<Vec<u8>> {
        let path = format!("/HostAssignableHardwareManager/{moId}/DownloadDescriptionTree", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieve PCI Dynamic Passthrough info.
    /// 
    /// Returns the list of PCI devices that may be used as a Dynamic
    /// DirectPath IO device.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_dynamic_passthrough_info(&self) -> Result<Option<Vec<VirtualMachineDynamicPassthroughInfo>>> {
        let path = format!("/HostAssignableHardwareManager/{moId}/RetrieveDynamicPassthroughInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Retrieve VendorDeviceGroup info.
    /// 
    /// Returns the list of Vendor Device Group deviceTypes present.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_vendor_device_group_info(&self) -> Result<Option<Vec<VirtualMachineVendorDeviceGroupInfo>>> {
        let path = format!("/HostAssignableHardwareManager/{moId}/RetrieveVendorDeviceGroupInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Update Assignable Hardware configuration.
    /// 
    /// Entries are updated as described in *HostAssignableHardwareConfig*.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// -
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn update_assignable_hardware_config(&self, config: &HostAssignableHardwareConfig) -> Result<()> {
        let input = UpdateAssignableHardwareConfigRequestType {config, };
        let path = format!("/HostAssignableHardwareManager/{moId}/UpdateAssignableHardwareConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Assignable Hardware bindings
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn binding(&self) -> Result<Option<Vec<HostAssignableHardwareBinding>>> {
        let path = format!("/HostAssignableHardwareManager/{moId}/binding", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Assignable Hardware configuration
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn config(&self) -> Result<HostAssignableHardwareConfig> {
        let path = format!("/HostAssignableHardwareManager/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateAssignableHardwareConfigRequestType<'a> {
    config: &'a HostAssignableHardwareConfig,
}
