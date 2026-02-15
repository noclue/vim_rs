use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The FirewallSystem managed object describes the firewall configuration
/// of the host.
/// 
/// The firewall should be configured first by setting the default policy and
/// then by making exceptions to the policy to get the desired openness.
#[derive(Clone)]
pub struct HostFirewallSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostFirewallSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Blocks the firewall ports belonging to the specified ruleset.
    /// 
    /// If the ruleset has a managed service with a policy of 'auto'
    /// and all other rulesets used by the service are blocked, stops
    /// the service.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the ruleset ID is unknown.
    /// 
    /// ***HostConfigFault***: if an internal error happened when reconfigure the
    /// ruleset.
    pub async fn disable_ruleset(&self, id: &str) -> Result<()> {
        let input = DisableRulesetRequestType {id, };
        let path = format!("/HostFirewallSystem/{moId}/DisableRuleset", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Opens the firewall ports belonging to the specified ruleset.
    /// 
    /// If the ruleset has a managed service with a policy of 'auto'
    /// that is not running, starts the service.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the ruleset ID is unknown.
    /// 
    /// ***HostConfigFault***: if an internal error happened when reconfigure the
    /// ruleset.
    pub async fn enable_ruleset(&self, id: &str) -> Result<()> {
        let input = EnableRulesetRequestType {id, };
        let path = format!("/HostFirewallSystem/{moId}/EnableRuleset", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Refresh the firewall information and settings to pick up any changes
    /// made directly on the host.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    pub async fn refresh_firewall(&self) -> Result<()> {
        let path = format!("/HostFirewallSystem/{moId}/RefreshFirewall", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Assigns a value to a custom field.
    /// 
    /// The setCustomValue method requires
    /// whichever updatePrivilege is defined as one of the
    /// *CustomFieldDef.fieldInstancePrivileges*
    /// for the CustomFieldDef whose value is being changed.
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The name of the field whose value is to be updated.
    ///
    /// ### value
    /// Value to be assigned to the custom field.
    pub async fn set_custom_value(&self, key: &str, value: &str) -> Result<()> {
        let input = SetCustomValueRequestType {key, value, };
        let path = format!("/HostFirewallSystem/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Updates the default firewall policy; unset fields are left unchanged.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### default_policy
    /// -
    pub async fn update_default_policy(&self, default_policy: &crate::types::structs::HostFirewallDefaultPolicy) -> Result<()> {
        let input = UpdateDefaultPolicyRequestType {default_policy, };
        let path = format!("/HostFirewallSystem/{moId}/UpdateDefaultPolicy", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Update the firewall ruleset specification.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// -
    ///
    /// ### spec
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the ruleset ID is unknown
    /// 
    /// ***HostConfigFault***: if the update of the ruleset failed.
    pub async fn update_ruleset(&self, id: &str, spec: &crate::types::structs::HostFirewallRulesetRulesetSpec) -> Result<()> {
        let input = UpdateRulesetRequestType {id, spec, };
        let path = format!("/HostFirewallSystem/{moId}/UpdateRuleset", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let path = format!("/HostFirewallSystem/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::CustomFieldDef>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Firewall configuration.
    pub async fn firewall_info(&self) -> Result<Option<crate::types::structs::HostFirewallInfo>> {
        let path = format!("/HostFirewallSystem/{moId}/firewallInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::HostFirewallInfo>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let path = format!("/HostFirewallSystem/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct DisableRulesetRequestType<'a> {
    id: &'a str,
}

impl<'a> miniserde::Serialize for DisableRulesetRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DisableRulesetRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DisableRulesetRequestTypeSer<'b, 'a> {
    data: &'b DisableRulesetRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DisableRulesetRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DisableRulesetRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct EnableRulesetRequestType<'a> {
    id: &'a str,
}

impl<'a> miniserde::Serialize for EnableRulesetRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EnableRulesetRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EnableRulesetRequestTypeSer<'b, 'a> {
    data: &'b EnableRulesetRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for EnableRulesetRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EnableRulesetRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> miniserde::Serialize for SetCustomValueRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetCustomValueRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetCustomValueRequestTypeSer<'b, 'a> {
    data: &'b SetCustomValueRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for SetCustomValueRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"setCustomValueRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("value"), &self.data.value as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateDefaultPolicyRequestType<'a> {
    default_policy: &'a crate::types::structs::HostFirewallDefaultPolicy,
}

impl<'a> miniserde::Serialize for UpdateDefaultPolicyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateDefaultPolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateDefaultPolicyRequestTypeSer<'b, 'a> {
    data: &'b UpdateDefaultPolicyRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for UpdateDefaultPolicyRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateDefaultPolicyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("defaultPolicy"), &self.data.default_policy as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateRulesetRequestType<'a> {
    id: &'a str,
    spec: &'a crate::types::structs::HostFirewallRulesetRulesetSpec,
}

impl<'a> miniserde::Serialize for UpdateRulesetRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateRulesetRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateRulesetRequestTypeSer<'b, 'a> {
    data: &'b UpdateRulesetRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for UpdateRulesetRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateRulesetRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
