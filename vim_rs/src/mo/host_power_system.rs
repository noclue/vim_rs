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
        self.client.invoke_void("", "HostPowerSystem", &self.mo_id, "ConfigurePowerPolicy", Some(&input)).await
    }
    /// Power system capabilities object.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn capability(&self) -> Result<crate::types::structs::PowerSystemCapability> {
        let bytes_opt = self.client.fetch_property_raw("", "HostPowerSystem", &self.mo_id, "capability").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property capability was empty".to_string()))?;
        let result: crate::types::structs::PowerSystemCapability = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Power system state info object.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn info(&self) -> Result<crate::types::structs::PowerSystemInfo> {
        let bytes_opt = self.client.fetch_property_raw("", "HostPowerSystem", &self.mo_id, "info").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property info was empty".to_string()))?;
        let result: crate::types::structs::PowerSystemInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct ConfigurePowerPolicyRequestType {
    key: i32,
}

impl miniserde::Serialize for ConfigurePowerPolicyRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ConfigurePowerPolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ConfigurePowerPolicyRequestTypeSer<'b> {
    data: &'b ConfigurePowerPolicyRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for ConfigurePowerPolicyRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ConfigurePowerPolicyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
