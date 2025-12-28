use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Deprecated as of vSphere 9.0. Please refer to vLCM APIs.
/// 
/// An <code>Agency</code> handles the deployment of a single type of agent
/// virtual machine and any associated VIB bundle, on a set of compute resources.
/// 
/// For a solution to deploy multiple types of agents, it must create multiple
/// agencies by using <code>createAgency</code> on *EsxAgentManager* (see
/// *EsxAgentManager.CreateAgency*).
/// 
/// Creating an agency is a long-running process. vSphere ESX Agent Manager must
/// install VIBs, configure hosts, install agent virtual machines and do many
/// more things. Each of these steps can take a considerable amount of time.
/// vSphere ESX Agent Manager can also encounter problems when creating the
/// agency. In this case, the solution must remediate the problem. See
/// *Issue* for a description of the kinds of issue that vSphere ESX Agent
/// Manager will raise. Similarly, removing an agency from vSphere ESX Agent
/// Manager is also a long-running process that involves many steps. Removing an
/// agency can also raise issues.
/// 
/// Use the <code>goalState</code> and <code>status</code> properties to show the
/// progress of creating or removing an Agency. The <code>goalState</code> and
/// <code>status</code> properties are found in the runtime information of an
/// <code>Agency</code> (see *EamObjectRuntimeInfo.status* in
/// *Agency.runtime*):
/// - <code>goalState</code>. The goal state describes the overall goal of an
///   <code>Agency</code>. The goal state can be <code>enabled</code> or
///   <code>uninstalled</code>:
///   - <code>enabled</code>. The <code>Agency</code> continuously deploys VIBs
///     and agent virtual machines, powers on agent virtual machines, and monitors
///     agents for issues.
///   - <code>uninstalled</code>. The <code>Agency</code> uninstalls any
///     installed VIBs and powers off and deletes any deployed agent virtual
///     machines.
/// - <code>status</code>. The status of the <code>Agency</code> regarding the
///   given goal state. Status can be either red, yellow or green:
///   - <code>red</code>. An issue is preventing the <code>Agency</code> from
///     reaching its desired goal state. See *EamObjectRuntimeInfo.issue* in
///     *Agency.runtime* for the types of issues that can block this
///     <code>Agency</code>.
///   - <code>yellow</code>. The <code>Agency</code> is actively working to reach
///     the desired goal state. For the <code>enabled</code> goal state, this means
///     that this <code>Agency</code> is currently installing VIBs, deploying agent
///     virtual machines, and powering them on.
///   - <code>green</code>. The <code>Agency</code> has reached the desired goal
///     state. The <code>Agency</code> is no longer actively scheduling new tasks but
///     is monitoring the vCenter Server for changes that might conflict with this
///     <code>Agency</code>'s goal state.
///     
/// The following image shows in general terms how the status changes in the
/// life-cycle of an <code>Agency</code>.
/// 
/// "Agency degraded" means that something has happened in the vCenter Server
/// that causes this <code>Agency</code> to actively schedule new tasks to reach
/// the goal state. For example, adding a host to a cluster covered by the scope
/// of the <code>Agency</code>, which causes ESX Agent Manager to install a VIB
/// and deploy an agent virtual machine on the new host.
/// A solution should monitor the list of issues associated with this
/// <code>Agency</code>.
/// 
/// The solution can poll *Agency.runtime*.
#[derive(Clone)]
pub struct Agency {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl Agency {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated.
    /// 
    /// Adds an issue to this agency.
    /// 
    /// *Issue.key* and *Issue.time* is
    /// overwritten so that *Issue.key* becomes unique on this server and
    /// *Issue.time* is the current time.
    /// 
    /// Requires modify privileges.
    ///
    /// ## Parameters:
    ///
    /// ### issue
    /// A new issue.
    ///
    /// ## Returns:
    ///
    /// The same issue where the key and time is set.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: Thrown if issue typeId is unknown.
    pub async fn add_issue(&self, issue: &dyn crate::types::traits::IssueTrait) -> Result<Box<dyn crate::types::traits::IssueTrait>> {
        let input = AddIssueRequestType {issue, };
        let path = format!("/eam/Agency/{moId}/AddIssue", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: Box<dyn crate::types::traits::IssueTrait> = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Destroys this Agency.
    /// 
    /// Any agents that the <code>Agency</code> has are
    /// removed. Until the agents have been removed, it is possible to view the
    /// runtime state of this <code>Agency</code> but it is not possible to modify
    /// its configuration or change its goal state. After all agents have been
    /// removed, any subsequent call on this <code>Agency</code> will throw a
    /// <code>ManagedObjectNotFound</code> exception.
    /// 
    /// Requires modify privileges.
    pub async fn destroy_agency(&self) -> Result<()> {
        let path = format!("/eam/Agency/{moId}/DestroyAgency", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Deprecated its definition is not consistent across agent VMs and VIBs.
    /// It is impossible to be defined since there is no corresponding
    /// state of ESXi vibs.
    /// 
    /// Sets the goal state of this <code>Agency</code> to <code>disabled</code>.
    /// 
    /// This powers off any powered on agent virtual machines, but continues
    /// provisioning agents to hosts that are added to the compute resources in
    /// the agency's scope, and removes agents from hosts that are taken out of
    /// the scope.
    /// 
    /// Requires modify privileges.
    pub async fn agency_disable(&self) -> Result<()> {
        let path = format!("/eam/Agency/{moId}/Agency_Disable", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Deprecated since agencies are always created as enabled. In addition,
    /// enabling already uninstalled agency is not supported.
    /// 
    /// Sets the goal state of this <code>Agency</code> to <code>enabled</code>.
    /// 
    /// This causes the agency to continuously deploy and monitor agents.
    /// 
    /// Requires modify privileges.
    pub async fn agency_enable(&self) -> Result<()> {
        let path = format!("/eam/Agency/{moId}/Agency_Enable", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Deprecated use *Agency.agent* instead.
    /// 
    /// An array of agents deployed by this agent manager.
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Agent*.
    pub async fn query_agent(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/eam/Agency/{moId}/QueryAgent", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::ManagedObjectReference>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Deprecated use *Agency.config* instead.
    /// 
    /// The configuration of this <code>Agency</code>.
    /// 
    /// Specifies how this
    /// <code>Agency</code> deploys its agents and VIBs.
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// The configuration of this <code>Agency</code>.
    pub async fn query_config(&self) -> Result<crate::types::structs::AgencyConfigInfo> {
        let path = format!("/eam/Agency/{moId}/QueryConfig", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::AgencyConfigInfo = serde_json::from_slice(bytes.as_ref())?;
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
        let path = format!("/eam/Agency/{moId}/QueryIssue", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<Box<dyn crate::types::traits::IssueTrait>>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Deprecated use *Agency.runtime* instead.
    /// 
    /// Gets the runtime information for this agency.
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// The runtime information.
    pub async fn agency_query_runtime(&self) -> Result<Box<dyn crate::types::traits::EamObjectRuntimeInfoTrait>> {
        let path = format!("/eam/Agency/{moId}/AgencyQueryRuntime", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: Box<dyn crate::types::traits::EamObjectRuntimeInfoTrait> = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Deprecated use *Agency.solutionId* instead.
    /// 
    /// The ID of the solution that owns this <code>Agency</code>.
    /// 
    /// If the agency
    /// is owned by a VC extension, this is the extension's key. Otherwise, this
    /// is same as *Agency.owner*. The users in the latter case are
    /// either regular or solution users.
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// The solution ID.
    pub async fn query_solution_id(&self) -> Result<String> {
        let path = format!("/eam/Agency/{moId}/QuerySolutionId", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: String = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Deprecated use automatically provisioned VMs and register hooks to have
    /// control post provisioning and power on.
    /// 
    /// Adds an agent VM to this agency.
    /// 
    /// Used if
    /// *AgencyConfigInfo.manuallyProvisioned* is set to true. The method
    /// does nothing if the agent VM is already registered with this agency.
    /// 
    /// Requires modify privileges.
    ///
    /// ## Parameters:
    ///
    /// ### agent_vm
    /// The managed object reference to the agent VM.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Agent*.
    ///
    /// ## Errors:
    ///
    /// ***ManagedObjectNotFound***: Thrown if agentVm does not exist in vCenter.
    pub async fn register_agent_vm(&self, agent_vm: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RegisterAgentVmRequestType {agent_vm, };
        let path = format!("/eam/Agency/{moId}/RegisterAgentVm", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
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
        let path = format!("/eam/Agency/{moId}/Resolve", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<i32>>(bytes.as_ref())?)),
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
        let path = format!("/eam/Agency/{moId}/ResolveAll", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Sets the goal state of this <code>Agency</code> to
    /// <code>uninstalled</code>.
    /// 
    /// This initiates the uninstallation of this
    /// <code>Agency</code>, which causes all agents to be removed.
    /// 
    /// The best practice when destroying an agency is to call
    /// <code>uninstall</code>, wait for the runtime status to turn green, and
    /// then invoke *Agency.DestroyAgency*. When waiting for this
    /// <code>Agency</code> to be uninstalled the solution can then attend to and
    /// resolve any raised issues.
    /// 
    /// Requires modify privileges.
    pub async fn uninstall(&self) -> Result<()> {
        let path = format!("/eam/Agency/{moId}/Uninstall", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Deprecated use automatically provisioned VMs and register hooks to have
    /// control post provisioning and power on.
    /// 
    /// Removes an agent VM to this agency.
    /// 
    /// Used if
    /// *AgencyConfigInfo.manuallyProvisioned* is set to true. The method
    /// does nothing if the agent VM is not registered with this agency.
    /// 
    /// Requires modify privileges.
    ///
    /// ## Parameters:
    ///
    /// ### agent_vm
    /// The managed object reference to the agent VM.
    /// 
    /// Refers instance of *VirtualMachine*.
    pub async fn unregister_agent_vm(&self, agent_vm: &crate::types::structs::ManagedObjectReference) -> Result<()> {
        let input = UnregisterAgentVmRequestType {agent_vm, };
        let path = format!("/eam/Agency/{moId}/UnregisterAgentVm", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Updates the agency configuration used by this <code>Agency</code> to
    /// deploy agents and VIBs.
    /// 
    /// vSphere ESX Agent Manager generates a diff between
    /// the old configuration and the new one and updates the <code>Agency</code>
    /// accordingly.
    /// 
    /// Requires modify privileges.
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// The new configuration for this <code>Agency</code>
    ///
    /// ## Errors:
    ///
    /// ***InvalidAgentConfiguration***: Thrown if one or more agent configurations are invalid.
    /// 
    /// ***InvalidAgencyScope***: Thrown if one or more compute resources in the scope cannot be
    /// found in vCenter or there is no configured resource pool or
    /// folder where the VMs to be deployed.
    /// 
    /// ***EamInvalidUrl***: Thrown if either the agent virtual machine URL or VIB URL
    /// cannot be parsed or if the resource refered to cannot be
    /// downloaded.
    pub async fn update(&self, config: &crate::types::structs::AgencyConfigInfo) -> Result<()> {
        let input = UpdateRequestType {config, };
        let path = format!("/eam/Agency/{moId}/Update", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// An array of agents deployed by this agent manager.
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Agent*.
    pub async fn agent(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/eam/Agency/{moId}/agent", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::ManagedObjectReference>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// The configuration of this <code>Agency</code>.
    /// 
    /// Specifies how this
    /// <code>Agency</code> deploys its agents and VIBs.
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// The configuration of this <code>Agency</code>.
    pub async fn config(&self) -> Result<crate::types::structs::AgencyConfigInfo> {
        let path = format!("/eam/Agency/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::AgencyConfigInfo = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// The principal name of the user that owns this <code>Agency</code>.
    /// 
    /// If the
    /// agency is owned by a VC extension, this method returns null.
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// the owner's principal name
    pub async fn owner(&self) -> Result<Option<String>> {
        let path = format!("/eam/Agency/{moId}/owner", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<String>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Gets the runtime information for this agency.
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// The runtime information.
    pub async fn runtime(&self) -> Result<Box<dyn crate::types::traits::EamObjectRuntimeInfoTrait>> {
        let path = format!("/eam/Agency/{moId}/runtime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: Box<dyn crate::types::traits::EamObjectRuntimeInfoTrait> = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// The ID of the solution that owns this <code>Agency</code>.
    /// 
    /// If the agency
    /// is owned by a VC extension, this is the extension's key. Otherwise, this
    /// is same as *Agency.owner*. The users in the latter case are
    /// either regular or solution users.
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// The solution ID.
    pub async fn solution_id(&self) -> Result<String> {
        let path = format!("/eam/Agency/{moId}/solutionId", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: String = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddIssueRequestType<'a> {
    issue: &'a dyn crate::types::traits::IssueTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryIssueRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "issueKey")]
    issue_key: Option<&'a [i32]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RegisterAgentVmRequestType<'a> {
    #[serde(rename = "agentVm")]
    agent_vm: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ResolveRequestType<'a> {
    #[serde(rename = "issueKey")]
    issue_key: &'a [i32],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnregisterAgentVmRequestType<'a> {
    #[serde(rename = "agentVm")]
    agent_vm: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateRequestType<'a> {
    config: &'a crate::types::structs::AgencyConfigInfo,
}
