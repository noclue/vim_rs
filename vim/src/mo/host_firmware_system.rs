use std::sync::Arc;
use crate::core::client::{Client, Result};
/// The *HostFirmwareSystem* managed object type provides access to the firmware
/// of an embedded ESX host.
/// 
/// It provides operations to backup, restore, and reset the
/// configuration of an embedded ESX host.
pub struct HostFirmwareSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostFirmwareSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Backup the configuration of the host.
    /// 
    /// The method generates a bundle containing the host configuration.
    /// You can use an HTTP GET operation to download the bundle from the returned URL.
    /// 
    /// ***Required privileges:*** Host.Config.Firmware
    ///
    /// ## Returns:
    ///
    /// URL that identifies the location of the backup bundle.
    pub async fn backup_firmware_configuration(&self) -> Result<String> {
        let path = format!("/HostFirmwareSystem/{moId}/BackupFirmwareConfiguration", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Return the URL on the host to which the configuration bundle must be
    /// uploaded for a restore operation.
    /// 
    /// See *HostFirmwareSystem.RestoreFirmwareConfiguration*.
    /// 
    /// ***Required privileges:*** Host.Config.Firmware
    ///
    /// ## Returns:
    ///
    /// URL that identifies the location for the restore operation.
    pub async fn query_firmware_config_upload_url(&self) -> Result<String> {
        let path = format!("/HostFirmwareSystem/{moId}/QueryFirmwareConfigUploadURL", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Reset the configuration to factory defaults.
    /// 
    /// This method will reset all configuration options, including the "admin"
    /// password, to the factory defaults. The host will be rebooted immediately.
    /// The host needs to be in maintenance mode before this operation can be
    /// performed.
    /// 
    /// ***Required privileges:*** Host.Config.Firmware
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is not in maintenance mode.
    pub async fn reset_firmware_to_factory_defaults(&self) -> Result<()> {
        let path = format!("/HostFirmwareSystem/{moId}/ResetFirmwareToFactoryDefaults", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Restore the configuration of the host to that specified in the bundle.
    /// 
    /// Upload the bundle to the URL returned by the
    /// *HostFirmwareSystem.QueryFirmwareConfigUploadURL* method.
    /// The *HostFirmwareSystem.RestoreFirmwareConfiguration* method
    /// will restore all configuration options,
    /// including the "admin" password, to the values in the bundle.
    /// The host will be rebooted immediately.
    /// The host must be in maintenance mode before this operation can be
    /// performed.
    /// 
    /// ***Required privileges:*** Host.Config.Firmware
    ///
    /// ## Parameters:
    ///
    /// ### force
    /// Forces application of the configuration even if the bundle
    /// is mismatched.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is not in maintenance mode.
    /// 
    /// ***FileFault***: if the file was not accessible.
    /// 
    /// ***MismatchedBundle***: if the uuid / build number in the bundle
    /// does not match the uuid / build number of the host and
    /// parameter 'force' is set to false.
    /// 
    /// ***InvalidBundle***: if the bundle does not have the expected contents.
    pub async fn restore_firmware_configuration(&self, force: bool) -> Result<()> {
        let input = RestoreFirmwareConfigurationRequestType {force, };
        let path = format!("/HostFirmwareSystem/{moId}/RestoreFirmwareConfiguration", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RestoreFirmwareConfigurationRequestType {
    force: bool,
}
