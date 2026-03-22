use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object manages the health state of the host.
/// 
/// See also *HostCapability.ipmiSupported*.
#[derive(Clone)]
pub struct HostHealthStatusSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostHealthStatusSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Hardware System Event Log (SEL) information
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn fetch_system_event_log(&self) -> Result<Option<Vec<crate::types::structs::SystemEventInfo>>> {
        let bytes_opt = self.client.invoke_optional("", "HostHealthStatusSystem", &self.mo_id, "FetchSystemEventLog", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Clear the the IPMI System Event Log.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn clear_system_event_log(&self) -> Result<()> {
        self.client.invoke_void("", "HostHealthStatusSystem", &self.mo_id, "ClearSystemEventLog", None).await
    }
    /// Refresh the available runtime hardware health information.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn refresh_health_status_system(&self) -> Result<()> {
        self.client.invoke_void("", "HostHealthStatusSystem", &self.mo_id, "RefreshHealthStatusSystem", None).await
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
        self.client.invoke_void("", "HostHealthStatusSystem", &self.mo_id, "ResetSystemHealthInfo", None).await
    }
    pub async fn runtime(&self) -> Result<crate::types::structs::HealthSystemRuntime> {
        let pv_opt = self.client.fetch_property_raw("", "HostHealthStatusSystem", &self.mo_id, "runtime").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property runtime was empty".to_string()))?;
        let result: crate::types::structs::HealthSystemRuntime = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
}
