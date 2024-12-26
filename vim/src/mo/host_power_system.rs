use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::PowerSystemCapability;
use crate::types::structs::PowerSystemInfo;
/// Managed object responsible for getting and setting host
/// power management policies.
pub struct HostPowerSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostPowerSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Power system capabilities object.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn capability(&self) -> Result<PowerSystemCapability> {
        let path = format!("/HostPowerSystem/{moId}/capability", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Power system state info object.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn info(&self) -> Result<PowerSystemInfo> {
        let path = format!("/HostPowerSystem/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConfigurePowerPolicyRequestType {
    key: i32,
}
