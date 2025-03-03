use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::AnswerFile;
use crate::types::structs::AnswerFileStatusResult;
use crate::types::structs::ApplyHostProfileConfigurationSpec;
use crate::types::structs::HostApplyProfile;
use crate::types::structs::HostConfigSpec;
use crate::types::structs::HostProfileManagerConfigTaskList;
use crate::types::structs::HostProfileManagerHostToConfigSpecMap;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::ProfileDeferredPolicyOptionParameter;
use crate::types::structs::ProfileMetadata;
use crate::types::structs::ProfilePolicyMetadata;
use crate::types::structs::ProfileProfileStructure;
use crate::types::structs::StructuredCustomizations;
/// The *HostProfileManager* provides access to a list of
/// *HostProfile*s and it defines methods to manipulate
/// profiles and *AnswerFile*s.
pub struct HostProfileManager {
    client: Arc<Client>,
    mo_id: String,
}
impl HostProfileManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    pub async fn apply_entities_config_task(&self, apply_config_specs: Option<&[ApplyHostProfileConfigurationSpec]>) -> Result<ManagedObjectReference> {
        let input = ApplyEntitiesConfigRequestType {apply_config_specs, };
        let path = format!("/HostProfileManager/{moId}/ApplyEntitiesConfig_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn apply_host_config_task(&self, host: &ManagedObjectReference, config_spec: &HostConfigSpec, user_input: Option<&[ProfileDeferredPolicyOptionParameter]>) -> Result<ManagedObjectReference> {
        let input = ApplyHostConfigRequestType {host, config_spec, user_input, };
        let path = format!("/HostProfileManager/{moId}/ApplyHostConfig_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn check_answer_file_status_task(&self, host: &[ManagedObjectReference]) -> Result<ManagedObjectReference> {
        let input = CheckAnswerFileStatusRequestType {host, };
        let path = format!("/HostProfileManager/{moId}/CheckAnswerFileStatus_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn composite_host_profile_task(&self, source: &ManagedObjectReference, targets: Option<&[ManagedObjectReference]>, to_be_merged: Option<&HostApplyProfile>, to_be_replaced_with: Option<&HostApplyProfile>, to_be_deleted: Option<&HostApplyProfile>, enable_status_to_be_copied: Option<&HostApplyProfile>) -> Result<ManagedObjectReference> {
        let input = CompositeHostProfileRequestType {source, targets, to_be_merged, to_be_replaced_with, to_be_deleted, enable_status_to_be_copied, };
        let path = format!("/HostProfileManager/{moId}/CompositeHostProfile_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn create_default_profile(&self, profile_type: &str, profile_type_name: Option<&str>, profile: Option<&ManagedObjectReference>) -> Result<Box<dyn crate::types::traits::ApplyProfileTrait>> {
        let input = CreateDefaultProfileRequestType {profile_type, profile_type_name, profile, };
        let path = format!("/HostProfileManager/{moId}/CreateDefaultProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn create_profile(&self, create_spec: &dyn crate::types::traits::ProfileCreateSpecTrait) -> Result<ManagedObjectReference> {
        let input = CreateProfileRequestType {create_spec, };
        let path = format!("/HostProfileManager/{moId}/CreateProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn export_answer_file_task(&self, host: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = ExportAnswerFileRequestType {host, };
        let path = format!("/HostProfileManager/{moId}/ExportAnswerFile_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn find_associated_profile(&self, entity: &ManagedObjectReference) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = FindAssociatedProfileRequestType {entity, };
        let path = format!("/HostProfileManager/{moId}/FindAssociatedProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn generate_config_task_list(&self, config_spec: &HostConfigSpec, host: &ManagedObjectReference) -> Result<HostProfileManagerConfigTaskList> {
        let input = GenerateConfigTaskListRequestType {config_spec, host, };
        let path = format!("/HostProfileManager/{moId}/GenerateConfigTaskList", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn generate_host_config_task_spec_task(&self, hosts_info: Option<&[StructuredCustomizations]>) -> Result<ManagedObjectReference> {
        let input = GenerateHostConfigTaskSpecRequestType {hosts_info, };
        let path = format!("/HostProfileManager/{moId}/GenerateHostConfigTaskSpec_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn generate_host_profile_task_list_task(&self, config_spec: &HostConfigSpec, host: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = GenerateHostProfileTaskListRequestType {config_spec, host, };
        let path = format!("/HostProfileManager/{moId}/GenerateHostProfileTaskList_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn query_answer_file_status(&self, host: &[ManagedObjectReference]) -> Result<Option<Vec<AnswerFileStatusResult>>> {
        let input = QueryAnswerFileStatusRequestType {host, };
        let path = format!("/HostProfileManager/{moId}/QueryAnswerFileStatus", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn query_policy_metadata(&self, policy_name: Option<&[String]>, profile: Option<&ManagedObjectReference>) -> Result<Option<Vec<ProfilePolicyMetadata>>> {
        let input = QueryPolicyMetadataRequestType {policy_name, profile, };
        let path = format!("/HostProfileManager/{moId}/QueryPolicyMetadata", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn query_host_profile_metadata(&self, profile_name: Option<&[String]>, profile: Option<&ManagedObjectReference>) -> Result<Option<Vec<ProfileMetadata>>> {
        let input = QueryHostProfileMetadataRequestType {profile_name, profile, };
        let path = format!("/HostProfileManager/{moId}/QueryHostProfileMetadata", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn query_profile_structure(&self, profile: Option<&ManagedObjectReference>) -> Result<ProfileProfileStructure> {
        let input = QueryProfileStructureRequestType {profile, };
        let path = format!("/HostProfileManager/{moId}/QueryProfileStructure", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn retrieve_answer_file(&self, host: &ManagedObjectReference) -> Result<Option<AnswerFile>> {
        let input = RetrieveAnswerFileRequestType {host, };
        let path = format!("/HostProfileManager/{moId}/RetrieveAnswerFile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn retrieve_answer_file_for_profile(&self, host: &ManagedObjectReference, apply_profile: &HostApplyProfile) -> Result<Option<AnswerFile>> {
        let input = RetrieveAnswerFileForProfileRequestType {host, apply_profile, };
        let path = format!("/HostProfileManager/{moId}/RetrieveAnswerFileForProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn retrieve_host_customizations(&self, hosts: Option<&[ManagedObjectReference]>) -> Result<Option<Vec<StructuredCustomizations>>> {
        let input = RetrieveHostCustomizationsRequestType {hosts, };
        let path = format!("/HostProfileManager/{moId}/RetrieveHostCustomizations", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn retrieve_host_customizations_for_profile(&self, hosts: Option<&[ManagedObjectReference]>, apply_profile: &HostApplyProfile) -> Result<Option<Vec<StructuredCustomizations>>> {
        let input = RetrieveHostCustomizationsForProfileRequestType {hosts, apply_profile, };
        let path = format!("/HostProfileManager/{moId}/RetrieveHostCustomizationsForProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn update_answer_file_task(&self, host: &ManagedObjectReference, config_spec: &dyn crate::types::traits::AnswerFileCreateSpecTrait) -> Result<ManagedObjectReference> {
        let input = UpdateAnswerFileRequestType {host, config_spec, };
        let path = format!("/HostProfileManager/{moId}/UpdateAnswerFile_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// This is the batch version of
    /// vim.profile.host.ProfileManager@updateAnswerFile.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### host_to_config_spec_map
    /// A map that contains the hosts with which
    /// the answer files are associated and the corresponding
    /// host-specific configuration data. If the configuration
    /// specification does not contain any host-specific user input
    /// (<code>configSpec</code>.*AnswerFileOptionsCreateSpec.userInput*),
    /// the method does not perform any operation on the answer file.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation with a result of *AnswerFileValidationResultMap* array.
    /// 
    /// Refers instance of *Task*.
    pub async fn update_host_customizations_task(&self, host_to_config_spec_map: Option<&[HostProfileManagerHostToConfigSpecMap]>) -> Result<ManagedObjectReference> {
        let input = UpdateHostCustomizationsRequestType {host_to_config_spec_map, };
        let path = format!("/HostProfileManager/{moId}/UpdateHostCustomizations_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn validate_host_profile_composition_task(&self, source: &ManagedObjectReference, targets: Option<&[ManagedObjectReference]>, to_be_merged: Option<&HostApplyProfile>, to_replace_with: Option<&HostApplyProfile>, to_be_deleted: Option<&HostApplyProfile>, enable_status_to_be_copied: Option<&HostApplyProfile>, error_only: Option<bool>) -> Result<ManagedObjectReference> {
        let input = ValidateHostProfileCompositionRequestType {source, targets, to_be_merged, to_replace_with, to_be_deleted, enable_status_to_be_copied, error_only, };
        let path = format!("/HostProfileManager/{moId}/ValidateHostProfileComposition_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// A list of profiles known to this ProfileManager.
    /// 
    /// ***Required privileges:*** Profile.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Profile*.
    pub async fn profile(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/HostProfileManager/{moId}/profile", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ApplyEntitiesConfigRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "applyConfigSpecs")]
    apply_config_specs: Option<&'a [ApplyHostProfileConfigurationSpec]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ApplyHostConfigRequestType<'a> {
    host: &'a ManagedObjectReference,
    #[serde(rename = "configSpec")]
    config_spec: &'a HostConfigSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "userInput")]
    user_input: Option<&'a [ProfileDeferredPolicyOptionParameter]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckAnswerFileStatusRequestType<'a> {
    host: &'a [ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CompositeHostProfileRequestType<'a> {
    source: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    targets: Option<&'a [ManagedObjectReference]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "toBeMerged")]
    to_be_merged: Option<&'a HostApplyProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "toBeReplacedWith")]
    to_be_replaced_with: Option<&'a HostApplyProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "toBeDeleted")]
    to_be_deleted: Option<&'a HostApplyProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enableStatusToBeCopied")]
    enable_status_to_be_copied: Option<&'a HostApplyProfile>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateDefaultProfileRequestType<'a> {
    #[serde(rename = "profileType")]
    profile_type: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "profileTypeName")]
    profile_type_name: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateProfileRequestType<'a> {
    #[serde(rename = "createSpec")]
    create_spec: &'a dyn crate::types::traits::ProfileCreateSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ExportAnswerFileRequestType<'a> {
    host: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindAssociatedProfileRequestType<'a> {
    entity: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GenerateConfigTaskListRequestType<'a> {
    #[serde(rename = "configSpec")]
    config_spec: &'a HostConfigSpec,
    host: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GenerateHostConfigTaskSpecRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hostsInfo")]
    hosts_info: Option<&'a [StructuredCustomizations]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GenerateHostProfileTaskListRequestType<'a> {
    #[serde(rename = "configSpec")]
    config_spec: &'a HostConfigSpec,
    host: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryAnswerFileStatusRequestType<'a> {
    host: &'a [ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPolicyMetadataRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "policyName")]
    policy_name: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryHostProfileMetadataRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "profileName")]
    profile_name: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryProfileStructureRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveAnswerFileRequestType<'a> {
    host: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveAnswerFileForProfileRequestType<'a> {
    host: &'a ManagedObjectReference,
    #[serde(rename = "applyProfile")]
    apply_profile: &'a HostApplyProfile,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveHostCustomizationsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hosts: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveHostCustomizationsForProfileRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hosts: Option<&'a [ManagedObjectReference]>,
    #[serde(rename = "applyProfile")]
    apply_profile: &'a HostApplyProfile,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateAnswerFileRequestType<'a> {
    host: &'a ManagedObjectReference,
    #[serde(rename = "configSpec")]
    config_spec: &'a dyn crate::types::traits::AnswerFileCreateSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateHostCustomizationsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hostToConfigSpecMap")]
    host_to_config_spec_map: Option<&'a [HostProfileManagerHostToConfigSpecMap]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ValidateHostProfileCompositionRequestType<'a> {
    source: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    targets: Option<&'a [ManagedObjectReference]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "toBeMerged")]
    to_be_merged: Option<&'a HostApplyProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "toReplaceWith")]
    to_replace_with: Option<&'a HostApplyProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "toBeDeleted")]
    to_be_deleted: Option<&'a HostApplyProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enableStatusToBeCopied")]
    enable_status_to_be_copied: Option<&'a HostApplyProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errorOnly")]
    error_only: Option<bool>,
}
