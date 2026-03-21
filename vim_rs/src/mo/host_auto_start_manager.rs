use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The AutoStartManager allows clients to invoke and set up the auto-start/auto-stop
/// order of virtual machines on a single host.
/// 
/// Virtual machines configured to use
/// auto-start are automatically started or stopped when the host is started or shut
/// down. The AutoStartManager is available when clients connect directly to a host,
/// such as an ESX Server machine or through VirtualCenter.
#[derive(Clone)]
pub struct HostAutoStartManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostAutoStartManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Powers-off virtual machines according to the current AutoStart configuration.
    /// 
    /// See the description of the (@link vim.host.AutoStartManager.AutoPowerInfo)
    /// data object type for more information on Auto power-off behavior.
    /// 
    /// ***Required privileges:*** Host.Config.AutoStart
    pub async fn auto_start_power_off(&self) -> Result<()> {
        self.client.invoke_void("", "HostAutoStartManager", &self.mo_id, "AutoStartPowerOff", None).await
    }
    /// Powers-on virtual machines according to the current AutoStart configuration.
    /// 
    /// See the description of the (@link vim.host.AutoStartManager.AutoPowerInfo)
    /// data object type for more information on Auto power-on behavior.
    /// 
    /// ***Required privileges:*** Host.Config.AutoStart
    pub async fn auto_start_power_on(&self) -> Result<()> {
        self.client.invoke_void("", "HostAutoStartManager", &self.mo_id, "AutoStartPowerOn", None).await
    }
    /// Changes the power-on or power-off sequence and system defaults.
    /// 
    /// The specification
    /// is an incremental change to the current configuration.
    /// 
    /// If systemDefaults are included, only values that are specified in the
    /// specification are changed.
    /// 
    /// For the spec.powerInfo array, each element is interpreted as an incremental
    /// change and the changes are processed sequentially. It is not an error to remove a
    /// non-existing virtual machine. If both startAction and stopAction are set to
    /// none, then the virtual machine is removed from the configuration.
    /// 
    /// A virtual machine's position in the order can be changed either by assigning the
    /// virtual machine a different position in the order or removing the machine from
    /// the order. When a virtual machine's position changes, all other virtual machines'
    /// positions may be affected as they move to new positions relative to each other.
    /// 
    /// ***Required privileges:*** Host.Config.AutoStart
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// List of changes to defaults and auto-start/auto-stop order.
    pub async fn reconfigure_autostart(&self, spec: &crate::types::structs::HostAutoStartManagerConfig) -> Result<()> {
        let input = ReconfigureAutostartRequestType {spec, };
        self.client.invoke_void("", "HostAutoStartManager", &self.mo_id, "ReconfigureAutostart", Some(&input)).await
    }
    pub async fn config(&self) -> Result<crate::types::structs::HostAutoStartManagerConfig> {
        let bytes_opt = self.client.fetch_property_raw("", "HostAutoStartManager", &self.mo_id, "config").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property config was empty".to_string()))?;
        let result: crate::types::structs::HostAutoStartManagerConfig = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct ReconfigureAutostartRequestType<'a> {
    spec: &'a crate::types::structs::HostAutoStartManagerConfig,
}

impl<'a> miniserde::Serialize for ReconfigureAutostartRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigureAutostartRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigureAutostartRequestTypeSer<'b, 'a> {
    data: &'b ReconfigureAutostartRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReconfigureAutostartRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigureAutostartRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
