use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostProfileValidationFailureInfo;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::ProfileDeferredPolicyOptionParameter;
use crate::types::structs::ProfileDescription;
/// A host profile describes ESX Server configuration.
/// 
/// The *HostProfile* managed object provides access to profile data and
/// it defines methods to manipulate the profile.
/// A host profile is a combination of subprofiles, each of which contains
/// configuration data for a specific capability. Some examples of host capabilities are
/// authentication, memory, networking, and security. For access to individual subprofiles,
/// see the *HostApplyProfile* data object
/// (*HostProfile*.*Profile.config*.*HostProfileConfigInfo.applyProfile*).
/// 
/// Host profiles are part of the stateless configuration architecture.
/// In the stateless environment, a Profile Engine runs on each ESX host,
/// but an ESX host does not store its own configuration state. Instead,
/// host configuration data is stored on vCenter Servers. Every time a host
/// boots or reboots, it obtains its profile from the vCenter Server.
/// - To create a base host profile use the
///   *HostProfileManager*.*ProfileManager.CreateProfile*
///   method. To create a profile from an ESX host, specify a
///   *HostProfileHostBasedConfigSpec*. To create a profile from a file,
///   specify a *HostProfileSerializedHostProfileSpec*.
/// - To create a subprofile for a particular host capability, use the
///   *HostProfileManager*.*HostProfileManager.CreateDefaultProfile*
///   method. After you create the default profile you can modify it and save it in the base profile.
/// - To update an existing profile, use the
///   *HostProfile*.*HostProfile.UpdateHostProfile* method.
/// - To apply a host profile to an ESX host, use the *HostProfile.ExecuteHostProfile* method
///   to generate configuration changes, then call the
///   *HostProfileManager*.*HostProfileManager.ApplyHostConfig_Task*
///   method to apply them.
///   
/// <u>Host-Specific Configuration</u>
/// 
/// An individual host or a set of hosts may have some configuration settings
/// that are different from the settings specified in the host profile.
/// For example, the IP configuration for the host's virtual network adapters
/// must be unique.
/// - To verify host-specific data, use the <code>deferredParam</code> parameter
///   to the *HostProfile.ExecuteHostProfile* method.
///   The Profile Engine will determine if you have specified all of the required
///   parameters for the host configuration. If additional data is required,
///   call the *HostProfile.ExecuteHostProfile* method again as many times as necessary
///   to verify a complete set of parameters.
/// - To apply host-specific data, use the <code>userInput</code> parameter to the
///   *HostProfileManager*.*HostProfileManager.ApplyHostConfig_Task*
///   method.
///   
///   
/// The Profile Engine saves host-specific data in an *AnswerFile*
/// that is stored on the vCenter Server.
/// The *HostProfileManager* provides several methods to manipulate
/// answer files.
/// 
/// <u>Profile Compliance</u>
/// 
/// You can create associations between hosts and profiles to support compliance checking.
/// When you perform compliance checking, you can determine if a host configuration
/// conforms to a host profile.
/// - To create an association between a host and a profile, use the
///   *Profile.AssociateProfile* method.
///   The method adds the host to the
///   *HostProfile*.*Profile.entity*\[\] list.
/// - To retrieve the list of profiles associated with a host, use the
///   *HostProfileManager*.*ProfileManager.FindAssociatedProfile*
///   method.
/// - To check host compliance, use the
///   *Profile.CheckProfileCompliance_Task* method.
///   If you do not specify any hosts, the method will check the compliance of all hosts
///   that are associated with the profile.
///   
///   
/// You can also use the Profile Compliance Manager to check compliance by specifying
/// profiles, entities (hosts), or both. See
/// *ProfileComplianceManager*.*ProfileComplianceManager.CheckCompliance_Task*.
/// 
/// <u>Profile Plug-Ins</u>
/// 
/// The vSphere architecture uses VMware profile plug-ins to define profile extensions.
/// For information about using a plug-in to extend a host profile, see the VMware Technical Note
/// _Developing a Host Profile Extension Plug-in_.
/// 
/// For access to host configuration data that is defined by plug-ins, use the
/// *ApplyProfile*.*ApplyProfile.policy*\[\] and
/// *ApplyProfile*.*ApplyProfile.property*\[\] lists.
/// The *HostApplyProfile* and its subprofiles, which collectively
/// define host configuration data, are derived from the *ApplyProfile*.
/// - Policies store ESX configuration data in *PolicyOption* objects.
/// - Profile property lists contain subprofiles defined by plug-ins. Subprofiles can be nested.
///   - Subprofile lists are available as an extension to the base host profile
///     (*HostProfile*.*Profile.config*.*HostProfileConfigInfo.applyProfile*.*ApplyProfile.property*\[\]).
///   - Subprofile lists are available as extensions to the host subprofiles - for example,
///     the network subprofile
///     (*HostApplyProfile*.*HostApplyProfile.network*.*ApplyProfile.property*\[\]).
///     
///   
/// If you make changes to host profile data, later versions of profile plug-ins may not support
/// the host configuration implied by the changes that you make. When a subsequent vSphere
/// version becomes available, you must verify that the new version supports any previous
/// configuration changes that you have made.
pub struct HostProfile {
    client: Arc<Client>,
    mo_id: String,
}
impl HostProfile {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// This API will update the validationState to Ready
    /// from Failed, invalidate the validationFailureInfo
    /// and reset the validationStateUpdateTime.
    /// 
    /// This API will return error if the validationState
    /// is Running.
    /// 
    /// ***Required privileges:*** Profile.Edit
    pub async fn host_profile_reset_validation_state(&self) -> Result<()> {
        let path = format!("/HostProfile/{moId}/HostProfileResetValidationState", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Associate a profile with a managed entity.
    /// 
    /// You can check the compliance of
    /// entities associated with a profile by calling the
    /// *Profile.CheckProfileCompliance_Task* method.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The entity(s) to associate with the profile.
    /// If an entity is already associated with the profile, the association is
    /// maintained and the vCenter Server does not perform any action.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn associate_profile(&self, entity: &[ManagedObjectReference]) -> Result<()> {
        let input = AssociateProfileRequestType {entity, };
        let path = format!("/HostProfile/{moId}/AssociateProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Check compliance of an entity against a Profile.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// If specified, the compliance check is performed on this entity.
    /// If the entity is not specified, the vCenter Server runs a compliance check on all the
    /// entities associated with the profile. The entity does not have to be associated with the
    /// profile.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn check_profile_compliance_task(&self, entity: Option<&[ManagedObjectReference]>) -> Result<ManagedObjectReference> {
        let input = CheckProfileComplianceRequestType {entity, };
        let path = format!("/HostProfile/{moId}/CheckProfileCompliance_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Destroy the profile.
    /// 
    /// ***Required privileges:*** Profile.Delete
    pub async fn destroy_profile(&self) -> Result<()> {
        let path = format!("/HostProfile/{moId}/DestroyProfile", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Remove the association between a profile and a managed entity.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// List of entities. The vCenter Server will remove the associations
    /// that the profile has with the entities in the list. If unset,
    /// the Server removes all the associations that the profile has with any
    /// managed entities in the inventory.
    /// If the specified entity is not associated with the profile,
    /// the Server does not perform any action.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn dissociate_profile(&self, entity: Option<&[ManagedObjectReference]>) -> Result<()> {
        let input = DissociateProfileRequestType {entity, };
        let path = format!("/HostProfile/{moId}/DissociateProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Run the Profile Engine to determine the list of configuration changes
    /// needed for the specified host.
    /// 
    /// The method generates configuration changes
    /// based on the host profile.
    /// 
    /// You can also specify deferred parameters to verify additional host-specific data.
    /// The Profile Engine uses the policy options
    /// (*HostProfile*.*Profile.config*.*HostProfileConfigInfo.applyProfile*.*ApplyProfile.policy*\[\])
    /// to determine the required parameters
    /// (*PolicyOption*.*PolicyOption.parameter*\[\])
    /// for host configuration. If you do not provide all of the required parameters,
    /// you must call the method again to provide the complete list to the Profile Engine.
    /// After successful profile execution, when you apply the profile,
    /// the Profile Engine will save the host-specific data in an *AnswerFile*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host on which to execute the profile.
    /// The host does not have to be associated with the profile.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### deferred_param
    /// Additional configuration data to be applied to the host.
    /// This should contain all of the host-specific data, including data from from
    /// previous calls to the method.
    ///
    /// ## Returns:
    ///
    /// Result of the execution. If the operation is successful
    /// (*ProfileExecuteResult*.*ProfileExecuteResult.status*<code>=success</code>),
    /// the result object includes a valid host configuration specification that you can pass to the
    /// *HostProfileManager*.*HostProfileManager.ApplyHostConfig_Task*
    /// method.
    /// 
    /// If the operation is not successful, the object contains error information
    /// or information about additional data required for the host configuration.
    /// If additional data is required, the required input list
    /// (*ProfileExecuteResult*.*ProfileExecuteResult.requireInput*\[\])
    /// contains both the <code>deferredParam</code> data and paths to missing parameters.
    /// After you fill in the missing parameters, pass the complete required input
    /// list via the <code>deferredParam</code> parameter in another call to the execute method
    /// to finish input verification. After successful profile execution, you can pass
    /// the verified required input list to the *HostProfileManager.ApplyHostConfig_Task*
    /// method.
    pub async fn execute_host_profile(&self, host: &ManagedObjectReference, deferred_param: Option<&[ProfileDeferredPolicyOptionParameter]>) -> Result<Box<dyn crate::types::traits::ProfileExecuteResultTrait>> {
        let input = ExecuteHostProfileRequestType {host, deferred_param, };
        let path = format!("/HostProfile/{moId}/ExecuteHostProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Export the profile in a serialized form.
    /// 
    /// To use the serialized string to create a profile,
    /// specify a *ProfileSerializedCreateSpec* when you call the
    /// *HostProfileManager*.*ProfileManager.CreateProfile*
    /// method.
    /// 
    /// ***Required privileges:*** Profile.Export
    ///
    /// ## Returns:
    ///
    /// Serialized form of the profile.
    pub async fn export_profile(&self) -> Result<String> {
        let path = format!("/HostProfile/{moId}/ExportProfile", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Returns the localizable description for the profile.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Profile divided into sections containing element descriptions and messages.
    pub async fn retrieve_description(&self) -> Result<Option<ProfileDescription>> {
        let path = format!("/HostProfile/{moId}/RetrieveDescription", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
    /// Update the <code>HostProfile</code> with the specified configuration data.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// Specification containing the new data.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: If the profile with the new name already exists.
    /// 
    /// ***ProfileUpdateFailed***: if errors were encountered when updating
    /// the profile.
    pub async fn update_host_profile(&self, config: &dyn crate::types::traits::HostProfileConfigSpecTrait) -> Result<()> {
        let input = UpdateHostProfileRequestType {config, };
        let path = format!("/HostProfile/{moId}/UpdateHostProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Sets the *HostProfile*.*HostProfile.referenceHost* property.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Reference host to use. If unset, the *HostProfile.referenceHost*
    /// property is cleared.
    /// 
    /// Refers instance of *HostSystem*.
    pub async fn update_reference_host(&self, host: Option<&ManagedObjectReference>) -> Result<()> {
        let input = UpdateReferenceHostRequestType {host, };
        let path = format!("/HostProfile/{moId}/UpdateReferenceHost", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// The latest compliance check time.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    pub async fn compliance_check_time(&self) -> Result<Option<String>> {
        let path = format!("/HostProfile/{moId}/complianceCheckTime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// Overall compliance of entities associated with this profile.
    /// 
    /// If one of the entities is out of compliance, the profile is <code>nonCompliant</code>.
    /// If all entities are in compliance, the profile is <code>compliant</code>.
    /// If the compliance status of one of the entities is not known, compliance status
    /// of the profile is <code>unknown</code>.
    /// See *ComplianceResultStatus_enum*.
    pub async fn compliance_status(&self) -> Result<String> {
        let path = format!("/HostProfile/{moId}/complianceStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Configuration data for the profile.
    /// 
    /// ***Required privileges:*** Profile.Edit
    pub async fn config(&self) -> Result<Box<dyn crate::types::traits::ProfileConfigInfoTrait>> {
        let path = format!("/HostProfile/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Time at which the profile was created.
    pub async fn created_time(&self) -> Result<String> {
        let path = format!("/HostProfile/{moId}/createdTime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Deprecated as of vSphere API 5.0. use *Profile.RetrieveDescription* instead.
    /// 
    /// Localizable description of the profile
    pub async fn description(&self) -> Result<Option<ProfileDescription>> {
        let path = format!("/HostProfile/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// List of managed entities associated with the profile.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *ManagedEntity*.
    pub async fn entity(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/HostProfile/{moId}/entity", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// Time at which the profile was last modified.
    pub async fn modified_time(&self) -> Result<String> {
        let path = format!("/HostProfile/{moId}/modifiedTime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Name of the profile.
    pub async fn name(&self) -> Result<String> {
        let path = format!("/HostProfile/{moId}/name", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Reference host in use for this host profile.
    /// 
    /// To set this property,
    /// use the *HostProfile.UpdateReferenceHost*
    /// method. If you do not specify a host for validation
    /// (*HostProfileCompleteConfigSpec*.*HostProfileCompleteConfigSpec.validatorHost*),
    /// the Profile Engine uses the reference host to validate the profile.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *HostSystem*.
    pub async fn reference_host(&self) -> Result<Option<ManagedObjectReference>> {
        let path = format!("/HostProfile/{moId}/referenceHost", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// This object is created or updated if the *HostProfileValidationState_enum*
    /// is Failed.
    /// 
    /// This object captures the most recent validation
    /// result for the host profile object in case of failure.
    pub async fn validation_failure_info(&self) -> Result<Option<HostProfileValidationFailureInfo>> {
        let path = format!("/HostProfile/{moId}/validationFailureInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// State of the host profile validation operation.
    /// 
    /// The values
    /// of the state will be one of *HostProfileValidationState_enum* enumerations.
    pub async fn validation_state(&self) -> Result<Option<String>> {
        let path = format!("/HostProfile/{moId}/validationState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// Update time of the validation operation.
    pub async fn validation_state_update_time(&self) -> Result<Option<String>> {
        let path = format!("/HostProfile/{moId}/validationStateUpdateTime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AssociateProfileRequestType<'a> {
    entity: &'a [ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckProfileComplianceRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DissociateProfileRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ExecuteHostProfileRequestType<'a> {
    host: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deferredParam")]
    deferred_param: Option<&'a [ProfileDeferredPolicyOptionParameter]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateHostProfileRequestType<'a> {
    config: &'a dyn crate::types::traits::HostProfileConfigSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateReferenceHostRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
