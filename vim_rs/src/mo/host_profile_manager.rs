use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *HostProfileManager* provides access to a list of
/// *HostProfile*s and it defines methods to manipulate
/// profiles and *AnswerFile*s.
#[derive(Clone)]
pub struct HostProfileManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostProfileManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// The task for applying host configuration on a list of hosts.
    /// 
    /// This is the
    /// batch version of <code>applyHostConfiguration</code>. The implementation
    /// of this method will:
    /// When a host is in a DRS cluster but doesn't satisfy the state requirement
    /// such as that the host is not in the required maintenance mode, this
    /// method uses DRS feature to put the host into maintenance mode.
    /// This method will apply a host profile to a stateful host or stateless
    /// host; or apply a host profile to a stateless host by reboot.
    /// After a host is reboot, a check compliance is done to update the latest
    /// compliance status.
    ///
    /// ## Parameters:
    ///
    /// ### apply_config_specs
    /// An array of
    /// *ApplyHostProfileConfigurationSpec*
    /// objects. Each applyConfigSpecs object contains the data objects
    /// required to remediate a host. The API caller should expand
    /// a cluster to all its hosts for the purpose of providing the
    /// required data object for configuration apply of each host.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. If the task is successful, the
    /// *Task*.*Task.info*.*TaskInfo.result*
    /// property is an array of
    /// *ApplyHostProfileConfigurationResult*
    /// objects. Each
    /// *ApplyHostProfileConfigurationResult*
    /// is for each host in the provided host list.
    /// 
    /// Refers instance of *Task*.
    pub async fn apply_entities_config_task(&self, apply_config_specs: Option<&[crate::types::structs::ApplyHostProfileConfigurationSpec]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ApplyEntitiesConfigRequestType {apply_config_specs, };
        let path = format!("/HostProfileManager/{moId}/ApplyEntitiesConfig_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Apply the configuration to the host.
    /// 
    /// If you specify any user input,
    /// the configuration will be saved in the *AnswerFile*
    /// associated with the host. If there is no answer file, the Profile Engine
    /// will create one.
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host to be updated. User must have sufficient credentials and privileges
    /// to satisfy the contents of the <code>configSpec</code>.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### config_spec
    /// Set of configuration changes to be applied to the host.
    /// The changes are returned by the
    /// *HostProfile*.*HostProfile.ExecuteHostProfile*
    /// method in the
    /// *ProfileExecuteResult*.*ProfileExecuteResult.configSpec*
    /// property.
    ///
    /// ### user_input
    /// Additional host-specific data to be applied to the host.
    /// This data is the complete list of deferred parameters verified by the
    /// *HostProfile*.*HostProfile.ExecuteHostProfile*
    /// method, contained in the *ProfileExecuteResult* object
    /// returned by the method.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is not in maintenance mode and the
    /// configuration specification requires it.
    /// 
    /// ***HostConfigFailed***: if the ESX Server cannot apply the configuration changes.
    pub async fn apply_host_config_task(&self, host: &crate::types::structs::ManagedObjectReference, config_spec: &crate::types::structs::HostConfigSpec, user_input: Option<&[crate::types::structs::ProfileDeferredPolicyOptionParameter]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ApplyHostConfigRequestType {host, config_spec, user_input, };
        let path = format!("/HostProfileManager/{moId}/ApplyHostConfig_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Check the validity of the answer files for the specified hosts.
    /// 
    /// The Profile Engine uses the profile associated with a host to check
    /// the answer file.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Set of hosts for which the answer file status will be checked.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. If the task is successful, the
    /// *Task*.*Task.info*.*TaskInfo.result*
    /// property contains a list of *AnswerFileStatusResult* objects.
    /// 
    /// Refers instance of *Task*.
    pub async fn check_answer_file_status_task(&self, host: &[crate::types::structs::ManagedObjectReference]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CheckAnswerFileStatusRequestType {host, };
        let path = format!("/HostProfileManager/{moId}/CheckAnswerFileStatus_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Composes (merge, replace, delete, disable)
    /// the selected configurations into the target host profiles.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### source
    /// Refers instance of *Profile*.
    ///
    /// ### targets
    /// Refers instances of *Profile*.
    ///
    /// ### to_be_merged
    /// -
    ///
    /// ### to_be_replaced_with
    /// -
    ///
    /// ### to_be_deleted
    /// -
    ///
    /// ### enable_status_to_be_copied
    /// -
    ///
    /// ## Returns:
    ///
    /// This method will returns a *Task* object with which to
    /// monitor the operation. The
    /// *Task*.*Task.info*.*TaskInfo.result*
    /// will contain a
    /// *HostProfileManagerCompositionResult*
    /// object containing the status of the operation, and details about
    /// any composition errors.
    /// The definitions of all the parameters are same as those in
    /// *HostProfileManager.ValidateHostProfileComposition_Task*.
    /// 
    /// Refers instance of *Task*.
    pub async fn composite_host_profile_task(&self, source: &crate::types::structs::ManagedObjectReference, targets: Option<&[crate::types::structs::ManagedObjectReference]>, to_be_merged: Option<&crate::types::structs::HostApplyProfile>, to_be_replaced_with: Option<&crate::types::structs::HostApplyProfile>, to_be_deleted: Option<&crate::types::structs::HostApplyProfile>, enable_status_to_be_copied: Option<&crate::types::structs::HostApplyProfile>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CompositeHostProfileRequestType {source, targets, to_be_merged, to_be_replaced_with, to_be_deleted, enable_status_to_be_copied, };
        let path = format!("/HostProfileManager/{moId}/CompositeHostProfile_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Create a default subprofile of a given type (for example, a
    /// *VirtualSwitchProfile*).
    /// 
    /// After you create
    /// the subprofile, you can add it to a configuration specification
    /// and update the host profile:
    /// - Call the <code>CreateDefaultProfile</code> method.
    /// - Create a *HostProfileCompleteConfigSpec* object.
    /// - Copy the existing profile from the host configuration information
    ///   (*HostProfile*.*Profile.config*.*HostProfileConfigInfo.applyProfile*) to the configuration specification.
    /// - Add the new subprofile to the configuration specification. For example,
    ///   if you create a <code>VirtualSwitchProfile</code>, you would add it to the list
    ///   of virtual switches in the network profile for the configuration specification
    ///   (*NetworkProfile*.*NetworkProfile.vswitch*\[\]).
    /// - Call *HostProfile*.*HostProfile.UpdateHostProfile*
    ///   to save the new subprofile.  
    ///   
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### profile_type
    /// Type of profile to create. The profile types
    /// are system-defined
    /// (*ApplyProfile*.*ApplyProfile.profileTypeName*).
    ///
    /// ### profile_type_name
    /// If specified, the method returns a profile object
    /// containing data for the named profile. The type name does not have
    /// to be system-defined. A user-defined profile can include various
    /// dynamically-defined profiles.
    ///
    /// ### profile
    /// Base profile used during the operation.
    /// 
    /// Refers instance of *Profile*.
    ///
    /// ## Returns:
    ///
    /// Derived subprofile of type <code>profileType</code>.
    pub async fn create_default_profile(&self, profile_type: &str, profile_type_name: Option<&str>, profile: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Box<dyn crate::types::traits::ApplyProfileTrait>> {
        let input = CreateDefaultProfileRequestType {profile_type, profile_type_name, profile, };
        let path = format!("/HostProfileManager/{moId}/CreateDefaultProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Box<dyn crate::types::traits::ApplyProfileTrait> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Create a profile from the specified CreateSpec.
    /// 
    /// ***Required privileges:*** Profile.Create
    ///
    /// ## Parameters:
    ///
    /// ### create_spec
    /// Specification for the profile being created.
    /// Usually a derived class CreateSpec can be used to create the Profile.
    ///
    /// ## Returns:
    ///
    /// Profile created from the specified createSpec.
    /// 
    /// Refers instance of *Profile*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: If a profile with the specified name already
    /// exists.
    /// 
    /// ***InvalidProfileReferenceHost***: if the specified reference host is
    /// incompatible or no reference host has been specified.
    pub async fn create_profile(&self, create_spec: &dyn crate::types::traits::ProfileCreateSpecTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateProfileRequestType {create_spec, };
        let path = format!("/HostProfileManager/{moId}/CreateProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Export a host's answer file into a serialized form.
    /// 
    /// The method returns a string
    /// that contains only the list of user input options.
    /// See *AnswerFile*.*AnswerFile.userInput*.
    /// 
    /// ***Required privileges:*** Profile.Export
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host with which the answer file is associated.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. If the task is successful, the
    /// *Task*.*Task.info*.*TaskInfo.result*
    /// property is a string that contains a serialized form of the answer file.
    /// 
    /// Refers instance of *Task*.
    pub async fn export_answer_file_task(&self, host: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ExportAnswerFileRequestType {host, };
        let path = format!("/HostProfileManager/{moId}/ExportAnswerFile_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Get the profile(s) to which this entity is associated.
    /// 
    /// The list of profiles will only include profiles known to this
    /// profileManager.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// Entity for which profile is being looked up.
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Profile*.
    pub async fn find_associated_profile(&self, entity: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = FindAssociatedProfileRequestType {entity, };
        let path = format!("/HostProfileManager/{moId}/FindAssociatedProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ManagedObjectReference>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Deprecated as of vSphere API 6.0 use
    /// *HostProfileManager.GenerateHostProfileTaskList_Task*.
    /// 
    /// Generate a list of configuration tasks that will be performed on the
    /// host during HostProfile application.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### config_spec
    /// ConfigSpec which was proposed by
    /// *HostProfile.ExecuteHostProfile* method.
    ///
    /// ### host
    /// Host on which the HostProfile application needs to be
    /// carried out.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// List of Configuration tasks.
    pub async fn generate_config_task_list(&self, config_spec: &crate::types::structs::HostConfigSpec, host: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::HostProfileManagerConfigTaskList> {
        let input = GenerateConfigTaskListRequestType {config_spec, host, };
        let path = format!("/HostProfileManager/{moId}/GenerateConfigTaskList", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::HostProfileManagerConfigTaskList = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// This method generates *ApplyHostProfileConfigurationSpec* data object
    /// for each host which can be passed as input to
    /// *HostProfileManager.ApplyEntitiesConfig_Task*
    /// to configure that host.
    /// 
    /// For each host, this method goes through two stages,
    /// *HostProfile.ExecuteHostProfile* stage
    /// *HostProfileManager.GenerateHostProfileTaskList_Task* stage. If the
    /// *HostProfile.ExecuteHostProfile* stage completes
    /// successfully then the method invokes the
    /// *HostProfileManager.GenerateHostProfileTaskList_Task*
    /// stage to generate the list of configuration tasks that are needed
    /// to configure the host.
    /// This method will return a task to monitor the progress of the operation.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### hosts_info
    /// List of host data for which configuration task list
    /// needs to be generated. The
    /// *StructuredCustomizations.customizations* value should be
    /// provided only if the host customization data for that host is
    /// invalid. If this property is not provided, the API will use the
    /// host customization data stored in VC and generate task list.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. If the task is successful, the
    /// *Task*.*Task.info*.*TaskInfo.result*
    /// property is a
    /// *ApplyHostProfileConfigurationSpec* object.
    /// 
    /// Refers instance of *Task*.
    pub async fn generate_host_config_task_spec_task(&self, hosts_info: Option<&[crate::types::structs::StructuredCustomizations]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = GenerateHostConfigTaskSpecRequestType {hosts_info, };
        let path = format!("/HostProfileManager/{moId}/GenerateHostConfigTaskSpec_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Generate a list of configuration tasks that will be performed on the
    /// host during HostProfile application.
    /// 
    /// This differs from the
    /// *HostProfileManager.GenerateConfigTaskList* method in
    /// that it returns a task to monitor the progress of the operation.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### config_spec
    /// ConfigSpec which was proposed by
    /// *HostProfile.ExecuteHostProfile* method.
    ///
    /// ### host
    /// Host on which the HostProfile application needs to be
    /// carried out.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. If the task is successful, the
    /// *Task*.*Task.info*.*TaskInfo.result*
    /// property is a *HostProfileManagerConfigTaskList*
    /// object.
    /// 
    /// Refers instance of *Task*.
    pub async fn generate_host_profile_task_list_task(&self, config_spec: &crate::types::structs::HostConfigSpec, host: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = GenerateHostProfileTaskListRequestType {config_spec, host, };
        let path = format!("/HostProfileManager/{moId}/GenerateHostProfileTaskList_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Returns the status of the answer files associated with specified hosts.
    /// 
    /// This method returns the most recent status determined by
    /// *HostProfileManager.CheckAnswerFileStatus_Task*.
    /// See *HostProfileManagerAnswerFileStatus_enum* for valid values.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The hosts the answer file is associated with.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// List of answer file status objects.
    pub async fn query_answer_file_status(&self, host: &[crate::types::structs::ManagedObjectReference]) -> Result<Option<Vec<crate::types::structs::AnswerFileStatusResult>>> {
        let input = QueryAnswerFileStatusRequestType {host, };
        let path = format!("/HostProfileManager/{moId}/QueryAnswerFileStatus", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::AnswerFileStatusResult>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Get the Metadata information for the policyNames.
    /// 
    /// PolicyNames are available with the defaultProfile obtained by invoking the
    /// method createDefaultProfile.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### policy_name
    /// Retrieve metadata for the specified policyNames.
    /// If policyName is not specified, metadata for all policies will be returned.
    ///
    /// ### profile
    /// Base profile whose context needs to be used during the operation
    /// 
    /// Refers instance of *Profile*.
    ///
    /// ## Returns:
    ///
    /// The metadata information for the policy.
    pub async fn query_policy_metadata(&self, policy_name: Option<&[String]>, profile: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::ProfilePolicyMetadata>>> {
        let input = QueryPolicyMetadataRequestType {policy_name, profile, };
        let path = format!("/HostProfileManager/{moId}/QueryPolicyMetadata", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ProfilePolicyMetadata>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Retrieve the metadata for a set of profiles.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### profile_name
    /// Names of the profiles for which metadata is requested.
    /// If not set, the method returns metadata for all the profiles.
    ///
    /// ### profile
    /// Base profile whose context needs to be used during the operation
    /// 
    /// Refers instance of *Profile*.
    ///
    /// ## Returns:
    ///
    /// List of profile metadata objects.
    pub async fn query_host_profile_metadata(&self, profile_name: Option<&[String]>, profile: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::ProfileMetadata>>> {
        let input = QueryHostProfileMetadataRequestType {profile_name, profile, };
        let path = format!("/HostProfileManager/{moId}/QueryHostProfileMetadata", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ProfileMetadata>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Get information about the structure of the profile.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### profile
    /// Base profile whose context needs to be used during the operation
    /// 
    /// Refers instance of *Profile*.
    ///
    /// ## Returns:
    ///
    /// The profile structure.
    pub async fn query_profile_structure(&self, profile: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ProfileProfileStructure> {
        let input = QueryProfileStructureRequestType {profile, };
        let path = format!("/HostProfileManager/{moId}/QueryProfileStructure", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ProfileProfileStructure = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Returns the answer file associated with a particular host.
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host with which the answer file is associated.
    /// 
    /// ***Required privileges:*** Profile.Edit
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Answer file object will be returned if it exists.
    pub async fn retrieve_answer_file(&self, host: &crate::types::structs::ManagedObjectReference) -> Result<Option<crate::types::structs::AnswerFile>> {
        let input = RetrieveAnswerFileRequestType {host, };
        let path = format!("/HostProfileManager/{moId}/RetrieveAnswerFile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::AnswerFile>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Returns the answer file associated with a particular host, augmented
    /// with whatever answer file values are required for the supplied host
    /// profile.
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host with which the answer file is associated.
    /// 
    /// ***Required privileges:*** Profile.Edit
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### apply_profile
    /// Profile configuration used to generate answer file
    ///
    /// ## Returns:
    ///
    /// Answer file object will be returned.
    pub async fn retrieve_answer_file_for_profile(&self, host: &crate::types::structs::ManagedObjectReference, apply_profile: &crate::types::structs::HostApplyProfile) -> Result<Option<crate::types::structs::AnswerFile>> {
        let input = RetrieveAnswerFileForProfileRequestType {host, apply_profile, };
        let path = format!("/HostProfileManager/{moId}/RetrieveAnswerFileForProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::AnswerFile>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// This is the batch version of
    /// vim.profile.host.ProfileManager@retrieveAnswerFile.
    /// 
    /// Returns a map that contains the hosts and their answer file data
    /// objects.
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// Hosts with which the answer files are associated.
    /// 
    /// ***Required privileges:*** Profile.Edit
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// A map that contains the hosts and their answer files.
    pub async fn retrieve_host_customizations(&self, hosts: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<Option<Vec<crate::types::structs::StructuredCustomizations>>> {
        let input = RetrieveHostCustomizationsRequestType {hosts, };
        let path = format!("/HostProfileManager/{moId}/RetrieveHostCustomizations", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::StructuredCustomizations>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// This is the batch version of
    /// vim.profile.host.ProfileManager@retrieveAnswerFileForProfile.
    /// 
    /// Returns a map that contains the hosts and their answer files associated
    /// with these hosts, augmented with whatever answer file values are
    /// required for the supplied host profile.
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// Hosts with which the answer files are associated.
    /// 
    /// ***Required privileges:*** Profile.Edit
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ### apply_profile
    /// Profile configuration used to generate answer file
    ///
    /// ## Returns:
    ///
    /// A map contains the hosts and their answer files.
    pub async fn retrieve_host_customizations_for_profile(&self, hosts: Option<&[crate::types::structs::ManagedObjectReference]>, apply_profile: &crate::types::structs::HostApplyProfile) -> Result<Option<Vec<crate::types::structs::StructuredCustomizations>>> {
        let input = RetrieveHostCustomizationsForProfileRequestType {hosts, apply_profile, };
        let path = format!("/HostProfileManager/{moId}/RetrieveHostCustomizationsForProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::StructuredCustomizations>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Update the *AnswerFile* for the specified host.
    /// 
    /// If there is no answer file associated with the host, the Profile Engine
    /// uses the answer file configuration specification to create a new one.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host with which the answer file is associated.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### config_spec
    /// Host-specific configuration data. If the configuration
    /// specification does not contain any host-specific user input
    /// (<code>configSpec</code>.*AnswerFileOptionsCreateSpec.userInput*),
    /// the method does not perform any operation on the answer file.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***AnswerFileUpdateFailed***: If the answer file could not be updated.
    /// 
    /// ***InvalidArgument***: If the input parameters are incorrect.
    pub async fn update_answer_file_task(&self, host: &crate::types::structs::ManagedObjectReference, config_spec: &dyn crate::types::traits::AnswerFileCreateSpecTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UpdateAnswerFileRequestType {host, config_spec, };
        let path = format!("/HostProfileManager/{moId}/UpdateAnswerFile_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Validates the proposed host profile composition.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### source
    /// The source host profile of the configurations for
    /// composition.
    /// 
    /// Refers instance of *Profile*.
    ///
    /// ### targets
    /// The array of target host profiles that the configurations
    /// composite into.
    /// 
    /// Refers instances of *Profile*.
    ///
    /// ### to_be_merged
    /// A *HostApplyProfile* object
    /// contains the sub profiles that will be merged from the source to
    /// the target host profiles, and all the ancestors of these sub
    /// profiles. For singleton sub profile, it will be added into a
    /// target host profile if it doesn't exist in the target; otherwise,
    /// it replaces the one in the target.
    /// The member variable
    /// *ApplyProfile.toBeMerged* of these sub profiles
    /// should have a value of <code>true</code>. The member variables
    /// *ApplyProfile.toBeMerged*
    /// *ApplyProfile.toReplaceWith*,
    /// *ApplyProfile.toBeDeleted*
    /// of the ancestors should have a value of <code>false</code>.
    ///
    /// ### to_replace_with
    /// A *HostApplyProfile* object
    /// contains the sub profiles that will be used to replace the array
    /// in the target host profiles, and all the ancestors of these sub
    /// profiles.
    /// Similar to above except that the member variable
    /// *ApplyProfile.toReplaceWith*
    /// is turned on.
    ///
    /// ### to_be_deleted
    /// A *HostApplyProfile* object
    /// contains the sub profiles that will be deleted from the source
    /// **and** the target host profiles, and all the ancestors of
    /// these sub profiles.
    /// Similar to above except that the member variable
    /// *ApplyProfile.toBeDeleted*
    /// is turned on.
    ///
    /// ### enable_status_to_be_copied
    /// A *HostApplyProfile*
    /// object contains the sub profiles that the member variable
    /// *ApplyProfile.enabled* will be copied from the
    /// source host profile to all the target host profiles, and all the
    /// ancestors of these sub profiles.
    /// The member variable
    /// *ApplyProfile.copyEnableStatus*
    /// of these sub profiles is turned on. The member variable
    /// *ApplyProfile.copyEnableStatus* of the
    /// *ApplyProfile.copyEnableStatus* of the
    /// ancestors should have a value of <code>false</code>.
    ///
    /// ### error_only
    /// Indicates that the validation result for each target
    /// don't contain the source-target difference.
    ///
    /// ## Returns:
    ///
    /// This method will returns a *Task* object with which to
    /// monitor the operation. The
    /// *Task*.*Task.info*.*TaskInfo.result*
    /// will contain a
    /// *HostProfileManagerCompositionValidationResult*
    /// object containing the status of the operation, any validation errors
    /// and the validation results.
    /// 
    /// Refers instance of *Task*.
    pub async fn validate_host_profile_composition_task(&self, source: &crate::types::structs::ManagedObjectReference, targets: Option<&[crate::types::structs::ManagedObjectReference]>, to_be_merged: Option<&crate::types::structs::HostApplyProfile>, to_replace_with: Option<&crate::types::structs::HostApplyProfile>, to_be_deleted: Option<&crate::types::structs::HostApplyProfile>, enable_status_to_be_copied: Option<&crate::types::structs::HostApplyProfile>, error_only: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ValidateHostProfileCompositionRequestType {source, targets, to_be_merged, to_replace_with, to_be_deleted, enable_status_to_be_copied, error_only, };
        let path = format!("/HostProfileManager/{moId}/ValidateHostProfileComposition_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// A list of profiles known to this ProfileManager.
    /// 
    /// ***Required privileges:*** Profile.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Profile*.
    pub async fn profile(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/HostProfileManager/{moId}/profile", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ManagedObjectReference>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct ApplyEntitiesConfigRequestType<'a> {
    apply_config_specs: Option<&'a [crate::types::structs::ApplyHostProfileConfigurationSpec]>,
}

impl<'a> miniserde::Serialize for ApplyEntitiesConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ApplyEntitiesConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ApplyEntitiesConfigRequestTypeSer<'b, 'a> {
    data: &'b ApplyEntitiesConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ApplyEntitiesConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ApplyEntitiesConfigRequestType")),
                1 => {
                    let Some(ref val) = self.data.apply_config_specs else { continue; };
                    return Some((std::borrow::Cow::Borrowed("applyConfigSpecs"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ApplyHostConfigRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
    config_spec: &'a crate::types::structs::HostConfigSpec,
    user_input: Option<&'a [crate::types::structs::ProfileDeferredPolicyOptionParameter]>,
}

impl<'a> miniserde::Serialize for ApplyHostConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ApplyHostConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ApplyHostConfigRequestTypeSer<'b, 'a> {
    data: &'b ApplyHostConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ApplyHostConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ApplyHostConfigRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("configSpec"), &self.data.config_spec as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.user_input else { continue; };
                    return Some((std::borrow::Cow::Borrowed("userInput"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CheckAnswerFileStatusRequestType<'a> {
    host: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for CheckAnswerFileStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CheckAnswerFileStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CheckAnswerFileStatusRequestTypeSer<'b, 'a> {
    data: &'b CheckAnswerFileStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CheckAnswerFileStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CheckAnswerFileStatusRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CompositeHostProfileRequestType<'a> {
    source: &'a crate::types::structs::ManagedObjectReference,
    targets: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    to_be_merged: Option<&'a crate::types::structs::HostApplyProfile>,
    to_be_replaced_with: Option<&'a crate::types::structs::HostApplyProfile>,
    to_be_deleted: Option<&'a crate::types::structs::HostApplyProfile>,
    enable_status_to_be_copied: Option<&'a crate::types::structs::HostApplyProfile>,
}

impl<'a> miniserde::Serialize for CompositeHostProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CompositeHostProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CompositeHostProfileRequestTypeSer<'b, 'a> {
    data: &'b CompositeHostProfileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CompositeHostProfileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CompositeHostProfileRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("source"), &self.data.source as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.targets else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targets"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.to_be_merged else { continue; };
                    return Some((std::borrow::Cow::Borrowed("toBeMerged"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.to_be_replaced_with else { continue; };
                    return Some((std::borrow::Cow::Borrowed("toBeReplacedWith"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.to_be_deleted else { continue; };
                    return Some((std::borrow::Cow::Borrowed("toBeDeleted"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.enable_status_to_be_copied else { continue; };
                    return Some((std::borrow::Cow::Borrowed("enableStatusToBeCopied"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CreateDefaultProfileRequestType<'a> {
    profile_type: &'a str,
    profile_type_name: Option<&'a str>,
    profile: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for CreateDefaultProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateDefaultProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateDefaultProfileRequestTypeSer<'b, 'a> {
    data: &'b CreateDefaultProfileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateDefaultProfileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateDefaultProfileRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("profileType"), &self.data.profile_type as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.profile_type_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profileTypeName"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CreateProfileRequestType<'a> {
    create_spec: &'a dyn crate::types::traits::ProfileCreateSpecTrait,
}

impl<'a> miniserde::Serialize for CreateProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateProfileRequestTypeSer<'b, 'a> {
    data: &'b CreateProfileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateProfileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateProfileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("createSpec"), &self.data.create_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ExportAnswerFileRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for ExportAnswerFileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ExportAnswerFileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ExportAnswerFileRequestTypeSer<'b, 'a> {
    data: &'b ExportAnswerFileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ExportAnswerFileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ExportAnswerFileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct FindAssociatedProfileRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for FindAssociatedProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FindAssociatedProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FindAssociatedProfileRequestTypeSer<'b, 'a> {
    data: &'b FindAssociatedProfileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FindAssociatedProfileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FindAssociatedProfileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct GenerateConfigTaskListRequestType<'a> {
    config_spec: &'a crate::types::structs::HostConfigSpec,
    host: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for GenerateConfigTaskListRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GenerateConfigTaskListRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GenerateConfigTaskListRequestTypeSer<'b, 'a> {
    data: &'b GenerateConfigTaskListRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GenerateConfigTaskListRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GenerateConfigTaskListRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("configSpec"), &self.data.config_spec as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct GenerateHostConfigTaskSpecRequestType<'a> {
    hosts_info: Option<&'a [crate::types::structs::StructuredCustomizations]>,
}

impl<'a> miniserde::Serialize for GenerateHostConfigTaskSpecRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GenerateHostConfigTaskSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GenerateHostConfigTaskSpecRequestTypeSer<'b, 'a> {
    data: &'b GenerateHostConfigTaskSpecRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GenerateHostConfigTaskSpecRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GenerateHostConfigTaskSpecRequestType")),
                1 => {
                    let Some(ref val) = self.data.hosts_info else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hostsInfo"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct GenerateHostProfileTaskListRequestType<'a> {
    config_spec: &'a crate::types::structs::HostConfigSpec,
    host: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for GenerateHostProfileTaskListRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GenerateHostProfileTaskListRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GenerateHostProfileTaskListRequestTypeSer<'b, 'a> {
    data: &'b GenerateHostProfileTaskListRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GenerateHostProfileTaskListRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GenerateHostProfileTaskListRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("configSpec"), &self.data.config_spec as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryAnswerFileStatusRequestType<'a> {
    host: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for QueryAnswerFileStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryAnswerFileStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryAnswerFileStatusRequestTypeSer<'b, 'a> {
    data: &'b QueryAnswerFileStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryAnswerFileStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryAnswerFileStatusRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryPolicyMetadataRequestType<'a> {
    policy_name: Option<&'a [String]>,
    profile: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryPolicyMetadataRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPolicyMetadataRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPolicyMetadataRequestTypeSer<'b, 'a> {
    data: &'b QueryPolicyMetadataRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPolicyMetadataRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPolicyMetadataRequestType")),
                1 => {
                    let Some(ref val) = self.data.policy_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("policyName"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryHostProfileMetadataRequestType<'a> {
    profile_name: Option<&'a [String]>,
    profile: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryHostProfileMetadataRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryHostProfileMetadataRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryHostProfileMetadataRequestTypeSer<'b, 'a> {
    data: &'b QueryHostProfileMetadataRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryHostProfileMetadataRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryHostProfileMetadataRequestType")),
                1 => {
                    let Some(ref val) = self.data.profile_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profileName"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryProfileStructureRequestType<'a> {
    profile: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryProfileStructureRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryProfileStructureRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryProfileStructureRequestTypeSer<'b, 'a> {
    data: &'b QueryProfileStructureRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryProfileStructureRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryProfileStructureRequestType")),
                1 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RetrieveAnswerFileRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for RetrieveAnswerFileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveAnswerFileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveAnswerFileRequestTypeSer<'b, 'a> {
    data: &'b RetrieveAnswerFileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveAnswerFileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveAnswerFileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveAnswerFileForProfileRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
    apply_profile: &'a crate::types::structs::HostApplyProfile,
}

impl<'a> miniserde::Serialize for RetrieveAnswerFileForProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveAnswerFileForProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveAnswerFileForProfileRequestTypeSer<'b, 'a> {
    data: &'b RetrieveAnswerFileForProfileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveAnswerFileForProfileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveAnswerFileForProfileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("applyProfile"), &self.data.apply_profile as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveHostCustomizationsRequestType<'a> {
    hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for RetrieveHostCustomizationsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveHostCustomizationsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveHostCustomizationsRequestTypeSer<'b, 'a> {
    data: &'b RetrieveHostCustomizationsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveHostCustomizationsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveHostCustomizationsRequestType")),
                1 => {
                    let Some(ref val) = self.data.hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hosts"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RetrieveHostCustomizationsForProfileRequestType<'a> {
    hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    apply_profile: &'a crate::types::structs::HostApplyProfile,
}

impl<'a> miniserde::Serialize for RetrieveHostCustomizationsForProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveHostCustomizationsForProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveHostCustomizationsForProfileRequestTypeSer<'b, 'a> {
    data: &'b RetrieveHostCustomizationsForProfileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveHostCustomizationsForProfileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveHostCustomizationsForProfileRequestType")),
                1 => {
                    let Some(ref val) = self.data.hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hosts"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("applyProfile"), &self.data.apply_profile as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct UpdateAnswerFileRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
    config_spec: &'a dyn crate::types::traits::AnswerFileCreateSpecTrait,
}

impl<'a> miniserde::Serialize for UpdateAnswerFileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateAnswerFileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateAnswerFileRequestTypeSer<'b, 'a> {
    data: &'b UpdateAnswerFileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateAnswerFileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateAnswerFileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("configSpec"), &self.data.config_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ValidateHostProfileCompositionRequestType<'a> {
    source: &'a crate::types::structs::ManagedObjectReference,
    targets: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    to_be_merged: Option<&'a crate::types::structs::HostApplyProfile>,
    to_replace_with: Option<&'a crate::types::structs::HostApplyProfile>,
    to_be_deleted: Option<&'a crate::types::structs::HostApplyProfile>,
    enable_status_to_be_copied: Option<&'a crate::types::structs::HostApplyProfile>,
    error_only: Option<bool>,
}

impl<'a> miniserde::Serialize for ValidateHostProfileCompositionRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ValidateHostProfileCompositionRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ValidateHostProfileCompositionRequestTypeSer<'b, 'a> {
    data: &'b ValidateHostProfileCompositionRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ValidateHostProfileCompositionRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ValidateHostProfileCompositionRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("source"), &self.data.source as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.targets else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targets"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.to_be_merged else { continue; };
                    return Some((std::borrow::Cow::Borrowed("toBeMerged"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.to_replace_with else { continue; };
                    return Some((std::borrow::Cow::Borrowed("toReplaceWith"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.to_be_deleted else { continue; };
                    return Some((std::borrow::Cow::Borrowed("toBeDeleted"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.enable_status_to_be_copied else { continue; };
                    return Some((std::borrow::Cow::Borrowed("enableStatusToBeCopied"), val as &dyn miniserde::Serialize));
                }
                7 => {
                    let Some(ref val) = self.data.error_only else { continue; };
                    return Some((std::borrow::Cow::Borrowed("errorOnly"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
