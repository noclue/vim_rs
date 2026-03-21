use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object manages the assignable hardware state of the host.
#[derive(Clone)]
pub struct HostAssignableHardwareManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostAssignableHardwareManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let bytes = self.client.invoke("", "HostAssignableHardwareManager", &self.mo_id, "DownloadDescriptionTree", None).await?;
        let result: Vec<u8> = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Retrieve PCI Dynamic Passthrough info.
    /// 
    /// Returns the list of PCI devices that may be used as a Dynamic
    /// DirectPath IO device.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_dynamic_passthrough_info(&self) -> Result<Option<Vec<crate::types::structs::VirtualMachineDynamicPassthroughInfo>>> {
        let bytes_opt = self.client.invoke_optional("", "HostAssignableHardwareManager", &self.mo_id, "RetrieveDynamicPassthroughInfo", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Retrieve VendorDeviceGroup info.
    /// 
    /// Returns the list of Vendor Device Group deviceTypes present.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_vendor_device_group_info(&self) -> Result<Option<Vec<crate::types::structs::VirtualMachineVendorDeviceGroupInfo>>> {
        let bytes_opt = self.client.invoke_optional("", "HostAssignableHardwareManager", &self.mo_id, "RetrieveVendorDeviceGroupInfo", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn update_assignable_hardware_config(&self, config: &crate::types::structs::HostAssignableHardwareConfig) -> Result<()> {
        let input = UpdateAssignableHardwareConfigRequestType {config, };
        self.client.invoke_void("", "HostAssignableHardwareManager", &self.mo_id, "UpdateAssignableHardwareConfig", Some(&input)).await
    }
    /// Assignable Hardware bindings
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn binding(&self) -> Result<Option<Vec<crate::types::structs::HostAssignableHardwareBinding>>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostAssignableHardwareManager", &self.mo_id, "binding").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Assignable Hardware configuration
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn config(&self) -> Result<crate::types::structs::HostAssignableHardwareConfig> {
        let bytes_opt = self.client.fetch_property_raw("", "HostAssignableHardwareManager", &self.mo_id, "config").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property config was empty".to_string()))?;
        let result: crate::types::structs::HostAssignableHardwareConfig = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct UpdateAssignableHardwareConfigRequestType<'a> {
    config: &'a crate::types::structs::HostAssignableHardwareConfig,
}

impl<'a> miniserde::Serialize for UpdateAssignableHardwareConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateAssignableHardwareConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateAssignableHardwareConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateAssignableHardwareConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateAssignableHardwareConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateAssignableHardwareConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
