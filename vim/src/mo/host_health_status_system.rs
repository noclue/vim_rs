use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HealthSystemRuntime;
use crate::types::structs::SystemEventInfo;
/// This managed object manages the health state of the host.
/// 
/// See also *HostCapability.ipmiSupported*.
pub struct HostHealthStatusSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostHealthStatusSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Hardware System Event Log (SEL) information
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn fetch_system_event_log(&self) -> Result<Option<Vec<SystemEventInfo>>> {
        let path = format!("/HostHealthStatusSystem/{moId}/FetchSystemEventLog", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Clear the the IPMI System Event Log.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn clear_system_event_log(&self) -> Result<()> {
        let path = format!("/HostHealthStatusSystem/{moId}/ClearSystemEventLog", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Refresh the available runtime hardware health information.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn refresh_health_status_system(&self) -> Result<()> {
        let path = format!("/HostHealthStatusSystem/{moId}/RefreshHealthStatusSystem", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Resets the state of the sensors of the IPMI subsystem.
    /// 
    /// On certain types
    /// of hardware IPMI sensor states latch onto unhealthy states and will stay
    /// in an unhealth state until the sensor state is reset. This method will
    /// explicitly reset the sensors state.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn reset_system_health_info(&self) -> Result<()> {
        let path = format!("/HostHealthStatusSystem/{moId}/ResetSystemHealthInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    pub async fn runtime(&self) -> Result<HealthSystemRuntime> {
        let path = format!("/HostHealthStatusSystem/{moId}/runtime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
