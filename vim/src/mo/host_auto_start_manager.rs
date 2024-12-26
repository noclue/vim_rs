use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostAutoStartManagerConfig;
/// The AutoStartManager allows clients to invoke and set up the auto-start/auto-stop
/// order of virtual machines on a single host.
/// 
/// Virtual machines configured to use
/// auto-start are automatically started or stopped when the host is started or shut
/// down. The AutoStartManager is available when clients connect directly to a host,
/// such as an ESX Server machine or through VirtualCenter.
pub struct HostAutoStartManager {
    client: Arc<Client>,
    mo_id: String,
}
impl HostAutoStartManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
        let path = format!("/HostAutoStartManager/{moId}/AutoStartPowerOff", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Powers-on virtual machines according to the current AutoStart configuration.
    /// 
    /// See the description of the (@link vim.host.AutoStartManager.AutoPowerInfo)
    /// data object type for more information on Auto power-on behavior.
    /// 
    /// ***Required privileges:*** Host.Config.AutoStart
    pub async fn auto_start_power_on(&self) -> Result<()> {
        let path = format!("/HostAutoStartManager/{moId}/AutoStartPowerOn", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn reconfigure_autostart(&self, spec: &HostAutoStartManagerConfig) -> Result<()> {
        let input = ReconfigureAutostartRequestType {spec, };
        let path = format!("/HostAutoStartManager/{moId}/ReconfigureAutostart", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    pub async fn config(&self) -> Result<HostAutoStartManagerConfig> {
        let path = format!("/HostAutoStartManager/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigureAutostartRequestType<'a> {
    spec: &'a HostAutoStartManagerConfig,
}
