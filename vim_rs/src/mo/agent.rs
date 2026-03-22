use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Deprecated as of vSphere 9.0. Please refer to vLCM APIs.
/// 
/// An <code>Agent</code> is the vSphere ESX Agent Manager managed object
/// responsible for deploying an <code>Agency</code> on a single host.
/// 
/// The
/// <code>Agent</code> maintains the state of the current deployment in its
/// runtime information (see *Agent.runtime*).
/// 
/// An <code>Agent</code> has the same <code>goalState</code> and
/// <code>status</code> properties as an <code>Agency</code>, that are used in
/// the same way:
/// - <code>goalState</code>. The goal state describes the overall goal of this
///   <code>Agent</code>. It can be <code>enabled</code> or
///   <code>uninstalled</code>:
///   - <code>enabled</code>. The <code>Agent</code> deploys its VIB, its agent
///     virtual machine, and powers on the agent virtual machine. The
///     <code>Agent</code> monitors in vCenter the status of the installation of the
///     VIB (in case it is uninstalled outside of vSphere ESX Agent Manager) and of
///     the agent virtual machine (in case an operation is performed on it outside of
///     vSphere ESX Agent Manager).
///   - <code>uninstalled</code>. The <code>Agent</code> uninstalls any installed
///     VIB and power off and delete the deployed agent virtual machine.
/// - <code>status</code>. The status of the <code>Agent</code> regarding the
///   given goal state. Status can be either red, yellow or green:
///   - <code>red</code>. An issue is preventing the <code>Agent</code> from
///     reaching its desired goal state. See *EamObjectRuntimeInfo.issue* in
///     *Agent.runtime* for the types of issues that can block this
///     <code>Agent</code>.
///   - <code>yellow</code>. The <code>Agent</code> is actively working to reach
///     the desired goal state. For the <code>enabled</code> goal state, this means
///     that this <code>Agent</code> is installing its VIB, deploying the agent
///     virtual machine, and powering it on.
///   - <code>green</code>. The <code>Agent</code> has reached the desired goal
///     state. The <code>Agent</code> is no longer actively scheduling new tasks but
///     is monitoring vCenter for changes that might conflict with this
///     <code>Agent</code>'s goal state.
#[derive(Clone)]
pub struct Agent {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl Agent {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Mark this agent's VM as available.
    /// 
    /// Used when the agency this agent belongs to
    /// has
    /// *AgencyConfigInfo.manuallyMarkAgentVmAvailableAfterProvisioning* or
    /// *AgencyConfigInfo.manuallyMarkAgentVmAvailableAfterPowerOn* set to
    /// <code>true</code> and *AgentRuntimeInfo.vmHook* is present.
    /// See *AgentRuntimeInfo.vmHook*
    pub async fn mark_as_available(&self) -> Result<()> {
        self.client.invoke_void("eam", "Agent", &self.mo_id, "MarkAsAvailable", None).await
    }
    /// Deprecated use *Agent.config* instead.
    /// 
    /// The configuration of this <code>Agent</code>.
    pub async fn agent_query_config(&self) -> Result<crate::types::structs::AgentConfigInfo> {
        let bytes = self.client.invoke("eam", "Agent", &self.mo_id, "AgentQueryConfig", None).await?;
        let result: crate::types::structs::AgentConfigInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Current issues that have been detected for this entity.
    /// 
    /// Each issue can be remediated
    /// by invoking *EamObject.Resolve* or *EamObject.ResolveAll*.
    /// 
    /// Requires view privileges.
    ///
    /// ## Parameters:
    ///
    /// ### issue_key
    /// An optional array of issue keys. If not set, all issues for this
    /// entity are returned.
    ///
    /// ## Returns:
    ///
    /// A possibly empty array of issues that match the input <code>issueKey</code> array. Note
    /// that the returned array can be smaller than <code>issueKey</code> if one or more
    /// issue keys refers to issues that this entity does not have.
    pub async fn query_issue(&self, issue_key: Option<&[i32]>) -> Result<Option<Vec<Box<dyn crate::types::traits::IssueTrait>>>> {
        let input = QueryIssueRequestType {issue_key, };
        let bytes_opt = self.client.invoke_optional("eam", "Agent", &self.mo_id, "QueryIssue", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Deprecated use *Agent.runtime* instead.
    /// 
    /// Runtime information for the agent.
    /// 
    /// This includes important information
    /// about the current deployment of the agent's VIB, virtual machine, and
    /// host.
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// The <code>Agent</code>'s runtime information.
    pub async fn agent_query_runtime(&self) -> Result<crate::types::structs::AgentRuntimeInfo> {
        let bytes = self.client.invoke("eam", "Agent", &self.mo_id, "AgentQueryRuntime", None).await?;
        let result: crate::types::structs::AgentRuntimeInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Resolves the issues specified in the input.
    /// 
    /// If an issue is remediable, ESX
    /// Agent Manager
    /// tries to resolve the misconfiguration that caused the issue. If it is not
    /// remediable, the offending issue is removed and ESX Agent Manager assumes that the issue has been
    /// resolved.
    /// 
    /// Requires modify privileges.
    /// 
    /// See also *Issue*.
    ///
    /// ## Parameters:
    ///
    /// ### issue_key
    /// A non-empty array of issue keys.
    ///
    /// ## Returns:
    ///
    /// A possibly empty array of issue keys for the issues that were not found on the
    /// entity. This can happen if <code>resolve</code> is called with issue keys that were
    /// resolved just prior to calling <code>resolve</code> or if an issue is currenly not resolvable.
    pub async fn resolve(&self, issue_key: &[i32]) -> Result<Option<Vec<i32>>> {
        let input = ResolveRequestType {issue_key, };
        let bytes_opt = self.client.invoke_optional("eam", "Agent", &self.mo_id, "Resolve", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Resolve all outstanding issues.
    /// 
    /// The method calls *EamObject.Resolve*
    /// with all issues the <code>EsxAgentManager</code>, <code>Agency</code>, or
    /// <code>Agent</code> have encountered. It is the equivalent of calling the following methods:
    /// - <code>agent.resolve(getIssueKeys(agent.getRuntime().getIssue()));</code>
    ///   for <code>Agent</code> objects
    /// - <code>agency.resolve(getIssueKeys(agency.getRuntime().getIssue()));</code>
    ///   for <code>Agency</code> objects
    /// - <code>esxAgentManager.resolve(getIssueKeys(esxAgentManager.getIssue()));</code>
    ///   for the <code>EsxAgentManager</code> object.
    ///   
    /// Requires modify privileges.
    /// 
    /// See also *Issue*.
    pub async fn resolve_all(&self) -> Result<()> {
        self.client.invoke_void("eam", "Agent", &self.mo_id, "ResolveAll", None).await
    }
    /// The configuration of this <code>Agent</code>.
    pub async fn config(&self) -> Result<crate::types::structs::AgentConfigInfo> {
        let pv_opt = self.client.fetch_property_raw("eam", "Agent", &self.mo_id, "config").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property config was empty".to_string()))?;
        let result: crate::types::structs::AgentConfigInfo = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Runtime information for this agent.
    /// 
    /// This includes important information
    /// about the current deployment of the agent's VIB, virtual machine, and
    /// host.
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// This <code>Agent</code>'s runtime information.
    pub async fn runtime(&self) -> Result<crate::types::structs::AgentRuntimeInfo> {
        let pv_opt = self.client.fetch_property_raw("eam", "Agent", &self.mo_id, "runtime").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property runtime was empty".to_string()))?;
        let result: crate::types::structs::AgentRuntimeInfo = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
}
struct QueryIssueRequestType<'a> {
    issue_key: Option<&'a [i32]>,
}

impl<'a> miniserde::Serialize for QueryIssueRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryIssueRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryIssueRequestTypeSer<'b, 'a> {
    data: &'b QueryIssueRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryIssueRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryIssueRequestType")),
                1 => {
                    let Some(ref val) = self.data.issue_key else { continue; };
                    return Some((std::borrow::Cow::Borrowed("issueKey"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ResolveRequestType<'a> {
    issue_key: &'a [i32],
}

impl<'a> miniserde::Serialize for ResolveRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ResolveRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ResolveRequestTypeSer<'b, 'a> {
    data: &'b ResolveRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ResolveRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ResolveRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("issueKey"), &self.data.issue_key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
