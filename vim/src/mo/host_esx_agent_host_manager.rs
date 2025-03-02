use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostEsxAgentHostManagerConfigInfo;
/// This managed object type is used to configure agent virtual machine resource
/// configuration, such as what network and datastore to use for agent virtual
/// machines.
pub struct HostEsxAgentHostManager {
    client: Arc<Client>,
    mo_id: String,
}
impl HostEsxAgentHostManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Update the host's ESX agent configuration.
    /// 
    /// The entire configuration must be set each time since all values are
    /// overwritten. E.g. a field set to null clears the value on the host.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### config_info
    /// configuration of agent virtual machine resources
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if an error occurs.
    pub async fn esx_agent_host_manager_update_config(&self, config_info: &HostEsxAgentHostManagerConfigInfo) -> Result<()> {
        let input = EsxAgentHostManagerUpdateConfigRequestType {config_info, };
        let path = format!("/HostEsxAgentHostManager/{moId}/EsxAgentHostManagerUpdateConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Configuration of agent virtual machine resources
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn config_info(&self) -> Result<HostEsxAgentHostManagerConfigInfo> {
        let path = format!("/HostEsxAgentHostManager/{moId}/configInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EsxAgentHostManagerUpdateConfigRequestType<'a> {
    #[serde(rename = "configInfo")]
    config_info: &'a HostEsxAgentHostManagerConfigInfo,
}
