use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type is used to configure agent virtual machine resource
/// configuration, such as what network and datastore to use for agent virtual
/// machines.
#[derive(Clone)]
pub struct HostEsxAgentHostManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostEsxAgentHostManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn esx_agent_host_manager_update_config(&self, config_info: &crate::types::structs::HostEsxAgentHostManagerConfigInfo) -> Result<()> {
        let input = EsxAgentHostManagerUpdateConfigRequestType {config_info, };
        let path = format!("/HostEsxAgentHostManager/{moId}/EsxAgentHostManagerUpdateConfig", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Configuration of agent virtual machine resources
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn config_info(&self) -> Result<crate::types::structs::HostEsxAgentHostManagerConfigInfo> {
        let path = format!("/HostEsxAgentHostManager/{moId}/configInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::HostEsxAgentHostManagerConfigInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct EsxAgentHostManagerUpdateConfigRequestType<'a> {
    config_info: &'a crate::types::structs::HostEsxAgentHostManagerConfigInfo,
}

impl<'a> miniserde::Serialize for EsxAgentHostManagerUpdateConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EsxAgentHostManagerUpdateConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EsxAgentHostManagerUpdateConfigRequestTypeSer<'b, 'a> {
    data: &'b EsxAgentHostManagerUpdateConfigRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for EsxAgentHostManagerUpdateConfigRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EsxAgentHostManagerUpdateConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("configInfo"), &self.data.config_info as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
