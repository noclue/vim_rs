use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Managed object responsible for getting and setting host
/// power management policies.
#[derive(Clone)]
pub struct HostPowerSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostPowerSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Configure host power policy.
    /// 
    /// ***Required privileges:*** Host.Config.Power
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// A key from one of the policies in
    /// *PowerSystemCapability.availablePolicy*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if an invalid power policy key is provided.
    /// 
    /// ***HostConfigFault***: for any other failure.
    pub async fn configure_power_policy(&self, key: i32) -> Result<()> {
        let input = ConfigurePowerPolicyRequestType {key, };
        let path = format!("/HostPowerSystem/{moId}/ConfigurePowerPolicy", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Power system capabilities object.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn capability(&self) -> Result<crate::types::structs::PowerSystemCapability> {
        let path = format!("/HostPowerSystem/{moId}/capability", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::PowerSystemCapability = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Power system state info object.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn info(&self) -> Result<crate::types::structs::PowerSystemInfo> {
        let path = format!("/HostPowerSystem/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::PowerSystemInfo = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConfigurePowerPolicyRequestType {
    key: i32,
}
