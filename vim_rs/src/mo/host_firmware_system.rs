use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *HostFirmwareSystem* managed object type provides access to the firmware
/// of an embedded ESX host.
/// 
/// It provides operations to backup, restore, and reset the
/// configuration of an embedded ESX host.
#[derive(Clone)]
pub struct HostFirmwareSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostFirmwareSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let bytes = self.client.invoke("", "HostFirmwareSystem", &self.mo_id, "BackupFirmwareConfiguration", None).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("", "HostFirmwareSystem", &self.mo_id, "QueryFirmwareConfigUploadURL", None).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "HostFirmwareSystem", &self.mo_id, "ResetFirmwareToFactoryDefaults", None).await
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
        self.client.invoke_void("", "HostFirmwareSystem", &self.mo_id, "RestoreFirmwareConfiguration", Some(&input)).await
    }
}
struct RestoreFirmwareConfigurationRequestType {
    force: bool,
}

impl miniserde::Serialize for RestoreFirmwareConfigurationRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RestoreFirmwareConfigurationRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RestoreFirmwareConfigurationRequestTypeSer<'b> {
    data: &'b RestoreFirmwareConfigurationRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for RestoreFirmwareConfigurationRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RestoreFirmwareConfigurationRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("force"), &self.data.force as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
