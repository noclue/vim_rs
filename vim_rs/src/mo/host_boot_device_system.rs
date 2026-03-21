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
        let bytes_opt = self.client.invoke_optional("", "HostBootDeviceSystem", &self.mo_id, "QueryBootDevices", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
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
        self.client.invoke_void("", "HostBootDeviceSystem", &self.mo_id, "UpdateBootDevice", Some(&input)).await
    }
}
struct UpdateBootDeviceRequestType<'a> {
    key: &'a str,
}

impl<'a> miniserde::Serialize for UpdateBootDeviceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateBootDeviceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateBootDeviceRequestTypeSer<'b, 'a> {
    data: &'b UpdateBootDeviceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateBootDeviceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateBootDeviceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
