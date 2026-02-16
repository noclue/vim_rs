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
        let path = format!("/HostAssignableHardwareManager/{moId}/DownloadDescriptionTree", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Vec<u8> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Retrieve PCI Dynamic Passthrough info.
    /// 
    /// Returns the list of PCI devices that may be used as a Dynamic
    /// DirectPath IO device.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_dynamic_passthrough_info(&self) -> Result<Option<Vec<crate::types::structs::VirtualMachineDynamicPassthroughInfo>>> {
        let path = format!("/HostAssignableHardwareManager/{moId}/RetrieveDynamicPassthroughInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VirtualMachineDynamicPassthroughInfo>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let path = format!("/HostAssignableHardwareManager/{moId}/RetrieveVendorDeviceGroupInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VirtualMachineVendorDeviceGroupInfo>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let path = format!("/HostAssignableHardwareManager/{moId}/UpdateAssignableHardwareConfig", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Assignable Hardware bindings
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn binding(&self) -> Result<Option<Vec<crate::types::structs::HostAssignableHardwareBinding>>> {
        let path = format!("/HostAssignableHardwareManager/{moId}/binding", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::HostAssignableHardwareBinding>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Assignable Hardware configuration
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn config(&self) -> Result<crate::types::structs::HostAssignableHardwareConfig> {
        let path = format!("/HostAssignableHardwareManager/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::HostAssignableHardwareConfig = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
