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
        let path = format!("/HostHealthStatusSystem/{moId}/FetchSystemEventLog", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::SystemEventInfo>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Clear the the IPMI System Event Log.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn clear_system_event_log(&self) -> Result<()> {
        let path = format!("/HostHealthStatusSystem/{moId}/ClearSystemEventLog", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Refresh the available runtime hardware health information.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn refresh_health_status_system(&self) -> Result<()> {
        let path = format!("/HostHealthStatusSystem/{moId}/RefreshHealthStatusSystem", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
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
        self.client.execute_void(req).await
    }
    pub async fn runtime(&self) -> Result<crate::types::structs::HealthSystemRuntime> {
        let path = format!("/HostHealthStatusSystem/{moId}/runtime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::HealthSystemRuntime = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
