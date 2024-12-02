use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::CustomFieldDef;
use crate::types::CustomFieldValueTrait;
use crate::types::HostFirewallDefaultPolicy;
use crate::types::HostFirewallInfo;
use crate::types::HostFirewallRulesetRulesetSpec;
/// The FirewallSystem managed object describes the firewall configuration
/// of the host.
/// 
/// The firewall should be configured first by setting the default policy and
/// then by making exceptions to the policy to get the desired openness.
pub struct HostFirewallSystem {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HostFirewallSystem {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Refresh the firewall information and settings to pick up any changes
    /// made directly on the host.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    pub async fn refresh_firewall(&self) -> Result<()> {
        let path = format!("/HostFirewallSystem/{moId}/RefreshFirewall", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the default firewall policy; unset fields are left unchanged.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### default_policy
    /// -
    pub async fn update_default_policy(&self, default_policy: &HostFirewallDefaultPolicy) -> Result<()> {
        let input = UpdateDefaultPolicyRequestType {default_policy, };
        let path = format!("/HostFirewallSystem/{moId}/UpdateDefaultPolicy", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_ruleset(&self, id: &str, spec: &HostFirewallRulesetRulesetSpec) -> Result<()> {
        let input = UpdateRulesetRequestType {id, spec, };
        let path = format!("/HostFirewallSystem/{moId}/UpdateRuleset", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/HostFirewallSystem/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Firewall configuration.
    pub async fn firewall_info(&self) -> Result<HostFirewallInfo> {
        let path = format!("/HostFirewallSystem/{moId}/firewallInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn CustomFieldValueTrait>>>> {
        let path = format!("/HostFirewallSystem/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DisableRulesetRequestType<'a> {
    id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EnableRulesetRequestType<'a> {
    id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateDefaultPolicyRequestType<'a> {
    #[serde(rename = "defaultPolicy")]
    default_policy: &'a HostFirewallDefaultPolicy,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateRulesetRequestType<'a> {
    id: &'a str,
    spec: &'a HostFirewallRulesetRulesetSpec,
}
