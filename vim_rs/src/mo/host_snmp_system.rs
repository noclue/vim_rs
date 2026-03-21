use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Provision the SNMP Version 1,2c agent.
/// 
/// This object is accessed through the
/// *HostConfigManager* object.
#[derive(Clone)]
pub struct HostSnmpSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostSnmpSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn reconfigure_snmp_agent(&self, spec: &crate::types::structs::HostSnmpConfigSpec) -> Result<()> {
        let input = ReconfigureSnmpAgentRequestType {spec, };
        self.client.invoke_void("", "HostSnmpSystem", &self.mo_id, "ReconfigureSnmpAgent", Some(&input)).await
    }
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn send_test_notification(&self) -> Result<()> {
        self.client.invoke_void("", "HostSnmpSystem", &self.mo_id, "SendTestNotification", None).await
    }
    /// ***Required privileges:*** Global.Settings
    pub async fn configuration(&self) -> Result<crate::types::structs::HostSnmpConfigSpec> {
        let bytes_opt = self.client.fetch_property_raw("", "HostSnmpSystem", &self.mo_id, "configuration").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property configuration was empty".to_string()))?;
        let result: crate::types::structs::HostSnmpConfigSpec = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// ***Required privileges:*** Global.Settings
    pub async fn limits(&self) -> Result<crate::types::structs::HostSnmpSystemAgentLimits> {
        let bytes_opt = self.client.fetch_property_raw("", "HostSnmpSystem", &self.mo_id, "limits").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property limits was empty".to_string()))?;
        let result: crate::types::structs::HostSnmpSystemAgentLimits = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct ReconfigureSnmpAgentRequestType<'a> {
    spec: &'a crate::types::structs::HostSnmpConfigSpec,
}

impl<'a> miniserde::Serialize for ReconfigureSnmpAgentRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigureSnmpAgentRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigureSnmpAgentRequestTypeSer<'b, 'a> {
    data: &'b ReconfigureSnmpAgentRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReconfigureSnmpAgentRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigureSnmpAgentRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
