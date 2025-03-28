use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostSnmpConfigSpec;
use crate::types::structs::HostSnmpSystemAgentLimits;
/// Provision the SNMP Version 1,2c agent.
/// 
/// This object is accessed through the
/// *HostConfigManager* object.
pub struct HostSnmpSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostSnmpSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// -
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn reconfigure_snmp_agent(&self, spec: &HostSnmpConfigSpec) -> Result<()> {
        let input = ReconfigureSnmpAgentRequestType {spec, };
        let path = format!("/HostSnmpSystem/{moId}/ReconfigureSnmpAgent", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn send_test_notification(&self) -> Result<()> {
        let path = format!("/HostSnmpSystem/{moId}/SendTestNotification", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// ***Required privileges:*** Global.Settings
    pub async fn configuration(&self) -> Result<HostSnmpConfigSpec> {
        let path = format!("/HostSnmpSystem/{moId}/configuration", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// ***Required privileges:*** Global.Settings
    pub async fn limits(&self) -> Result<HostSnmpSystemAgentLimits> {
        let path = format!("/HostSnmpSystem/{moId}/limits", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigureSnmpAgentRequestType<'a> {
    spec: &'a HostSnmpConfigSpec,
}
