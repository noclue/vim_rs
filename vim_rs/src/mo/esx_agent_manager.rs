use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The <code>EsxAgentManager</code> is the main entry point for a solution to
/// create agencies in the vSphere ESX Agent Manager server.
/// 
/// In vCenter 6.0, a _solution_ is either a vCenter extension or a regular
/// user.
/// 
/// The vSphere ESX Agent Manager VMODL API distinguishes between two types of
/// users: VC extensions and regular vCenter users. These users have different
/// privileges in the vSphere ESX Agent Manager VMODL API:
/// - VC extensions have the privileges to call anything in the vSphere ESX
///   Agent Manager VMODL API.
/// - Regular vCenter users have restrictions on what methods they can call.
///   The methods that a vCenter user that is not an extension can call are
///   annotated with two types of privileges, <code>Eam.View</code> and
///   <code>Eam.Modify</code>:
///   - <code>Eam.View</code>. If a method has the <code>Eam.View</code>
///     privilege, a user can call that method if they have the <code>EAM.View</code>
///     privilege in vCenter.
///   - <code>Eam.Modify</code>. Similarly to <code>Eam.View</code>, if a method
///     has the <code>Eam.Modify</code> privilege, a user can call that method if
///     they have the <code>EAM.Modify</code> privilege in vCenter. If a user has the
///     <code>EAM.Modify</code> privilege, they automatically have
///     <code>EAM.View</code>.
///     
/// In vCenter 6.5 every _solution_, which is making VMODL API calls to
/// EsxAgentManager, should be aware of the posibility, that the data from
/// vCenter database might not be fully loaded. In all such cases the clients
/// will receive an _ESX Agent Manager_ runtime fault:
/// _EamServiceNotInitialized_.
/// NOTE: No issues are associated with <code>EsxAgentManager</code> any longer.
#[derive(Clone)]
pub struct EsxAgentManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl EsxAgentManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of vSphere 9.0. Please refer to vLCM APIs.
    /// 
    /// Creates an Agency.
    /// 
    /// The initial goal state is given to the method by the
    /// second parameter. Throws <code>InvalidArgument</code> if the
    /// <code>agencyName</code> is not set in the <code>agencyConfigInfo</code>.
    /// 
    /// Requires modify privileges.
    ///
    /// ## Parameters:
    ///
    /// ### agency_config_info
    /// The configuration that describes how to deploy the agents in the
    /// created agency.
    ///
    /// ### initial_goal_state
    /// Deprecated. No sence to create agency in other state than
    /// <code>enabled</code>. <code>disabled</code> is deprecated
    /// whereas <code>uninstalled</code> is useless.
    /// The initial goal state of the agency. See
    /// *EamObjectRuntimeInfoGoalState_enum*.
    ///
    /// ## Returns:
    ///
    /// The created agency.
    /// 
    /// Refers instance of *Agency*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidAgentConfiguration***: Thrown if the agent configuration is empty or if one or more
    /// agent configurations are invalid.
    /// 
    /// ***InvalidAgencyScope***: Thrown if one or more compute resources in the scope cannot be
    /// found in vCenter or there is no configured resource pool or
    /// folder where the VMs to be deployed.
    /// 
    /// ***EamInvalidUrl***: Thrown if either the agent virtual machine URL or VIB URL
    /// cannot be parsed or if the resource refered to cannot be
    /// downloaded.
    pub async fn create_agency(&self, agency_config_info: &crate::types::structs::AgencyConfigInfo, initial_goal_state: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateAgencyRequestType {agency_config_info, initial_goal_state, };
        let bytes = self.client.invoke("eam", "EsxAgentManager", &self.mo_id, "CreateAgency", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of vSphere 9.0. Please refer to vLCM Image APIs.
    /// 
    /// Obtains maintenance policy for for clusters not managed by vSphere
    /// Lifecycle Manasger.
    /// 
    /// See also *EsxAgentManagerMaintenanceModePolicy_enum*.
    /// 
    /// ***Since:*** vEAM API 7.4
    ///
    /// ## Returns:
    ///
    /// Currently configured policy.
    pub async fn get_maintenance_mode_policy(&self) -> Result<String> {
        let bytes = self.client.invoke("eam", "EsxAgentManager", &self.mo_id, "GetMaintenanceModePolicy", None).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated use *EsxAgentManager.agency* instead.
    /// 
    /// An array of all *Agency* objects.
    /// 
    /// If called by a vCenter user with
    /// the vCenter <code>EAM.View</code> or <code>EAM.Modify</code> privilege,
    /// this returns all agencies registered in vSphere ESX Agent Manager. If
    /// called by a solution, this property only returns the agencies created by
    /// the solution (and only those which have not been destroyed).
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// The possibly empty set of agencies registered in the vSphere ESX
    /// Agent Manager server.
    /// 
    /// Refers instances of *Agency*.
    pub async fn query_agency(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let bytes_opt = self.client.invoke_optional("eam", "EsxAgentManager", &self.mo_id, "QueryAgency", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        let bytes_opt = self.client.invoke_optional("eam", "EsxAgentManager", &self.mo_id, "QueryIssue", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        let bytes_opt = self.client.invoke_optional("eam", "EsxAgentManager", &self.mo_id, "Resolve", Some(&input)).await?;
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
        self.client.invoke_void("eam", "EsxAgentManager", &self.mo_id, "ResolveAll", None).await
    }
    /// Deprecated presence of unknown VMs is no more acceptable.
    /// 
    /// Scans the vCenter inventory for any unknown agent virtual machine.
    /// 
    /// An
    /// issue is generated for each unknown agent virtual machine found during
    /// this scan.
    /// 
    /// Requires view privileges.
    pub async fn scan_for_unknown_agent_vm(&self) -> Result<()> {
        self.client.invoke_void("eam", "EsxAgentManager", &self.mo_id, "ScanForUnknownAgentVm", None).await
    }
    /// Deprecated as of vSphere 9.0. Please refer to vLCM Image APIs.
    /// 
    /// Configures maintenance mode policy for clusters not managed by vSphere
    /// Lifecycle Manasger.
    /// 
    /// See also *EsxAgentManagerMaintenanceModePolicy_enum*.
    /// 
    /// ***Since:*** vEAM API 7.4
    ///
    /// ## Parameters:
    ///
    /// ### policy
    /// The policy to use.
    pub async fn set_maintenance_mode_policy(&self, policy: &str) -> Result<()> {
        let input = SetMaintenanceModePolicyRequestType {policy, };
        self.client.invoke_void("eam", "EsxAgentManager", &self.mo_id, "SetMaintenanceModePolicy", Some(&input)).await
    }
    /// Deprecated as of vSphere 9.0. Please refer to vLCM APIs.
    /// 
    /// An array of all *Agency* objects.
    /// 
    /// If called by a vCenter user with
    /// the vCenter <code>EAM.View</code> or <code>EAM.Modify</code> privilege,
    /// this returns all agencies registered in vSphere ESX Agent Manager. If
    /// called by a solution, this property only returns the agencies created by
    /// the solution (and only those which have not been destroyed).
    /// 
    /// Requires view privileges.
    ///
    /// ## Returns:
    ///
    /// The possibly empty set of agencies registered in the vSphere ESX
    /// Agent Manager server.
    /// 
    /// Refers instances of *Agency*.
    pub async fn agency(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("eam", "EsxAgentManager", &self.mo_id, "agency").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Deprecated this method is deprecated since the EAM object does not
    /// handles issue anymore. At the moment the implementation
    /// returns an empty array of issues.
    /// 
    /// Current issues that have been detected for this entity.
    /// 
    /// Each issue can be
    /// remediated by invoking *EamObject.Resolve* or
    /// *EamObject.ResolveAll*.
    /// 
    /// Requires view privileges.
    pub async fn issue(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::IssueTrait>>>> {
        let pv_opt = self.client.fetch_property_raw("eam", "EsxAgentManager", &self.mo_id, "issue").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct CreateAgencyRequestType<'a> {
    agency_config_info: &'a crate::types::structs::AgencyConfigInfo,
    initial_goal_state: &'a str,
}

impl<'a> miniserde::Serialize for CreateAgencyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateAgencyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateAgencyRequestTypeSer<'b, 'a> {
    data: &'b CreateAgencyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateAgencyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateAgencyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("agencyConfigInfo"), &self.data.agency_config_info as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("initialGoalState"), &self.data.initial_goal_state as &dyn miniserde::Serialize)),
            _ => return None,
        }
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
struct SetMaintenanceModePolicyRequestType<'a> {
    policy: &'a str,
}

impl<'a> miniserde::Serialize for SetMaintenanceModePolicyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetMaintenanceModePolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetMaintenanceModePolicyRequestTypeSer<'b, 'a> {
    data: &'b SetMaintenanceModePolicyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetMaintenanceModePolicyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetMaintenanceModePolicyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("policy"), &self.data.policy as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
