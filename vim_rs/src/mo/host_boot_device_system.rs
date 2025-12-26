use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *HostBootDeviceSystem* managed object provides methods to query and update
/// a host boot device configuration.
#[derive(Clone)]
pub struct HostBootDeviceSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostBootDeviceSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Retrieves a list of the available boot devices for the host system.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// The boot device information for the host. The returned object has a
    /// list of *HostBootDevice* data objects; each boot
    /// device object defines a description and a key to identify the device.
    /// The order of devices in the list is unpredictable.
    /// The returned *HostBootDeviceInfo* data object also contains
    /// the key of the current boot device.
    pub async fn query_boot_devices(&self) -> Result<Option<crate::types::structs::HostBootDeviceInfo>> {
        let path = format!("/HostBootDeviceSystem/{moId}/QueryBootDevices", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::HostBootDeviceInfo>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Sets the current boot device for the host system.
    /// 
    /// ***Required privileges:*** Host.Config.Maintenance
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The *HostBootDevice.key* of the
    /// *HostBootDevice* from which the host will boot.
    pub async fn update_boot_device(&self, key: &str) -> Result<()> {
        let input = UpdateBootDeviceRequestType {key, };
        let path = format!("/HostBootDeviceSystem/{moId}/UpdateBootDevice", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateBootDeviceRequestType<'a> {
    key: &'a str,
}
