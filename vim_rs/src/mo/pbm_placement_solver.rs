use std::sync::Arc;
use crate::core::client::{Client, Result};
/// The *PbmPlacementSolver* data object provides methods to identify
/// placement hubs that support the capabilities to store virtual
/// machine files.
/// 
/// A placement hub is a datastore or a storage pod.
/// A vSphere API <code>StoragePod</code> corresponds to Storage DRS in the vSphere Web Client.
#[derive(Clone)]
pub struct PbmPlacementSolver {
    client: Arc<Client>,
    mo_id: String,
}
impl PbmPlacementSolver {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of vSphere 2016, use *PbmPlacementSolver.PbmCheckRequirements* instead in order to retrieve compatibility status on both compute and storage
    /// location.
    /// 
    /// Performs placement compatibility checking based on a storage requirement
    /// profile.
    /// 
    /// If compatibility checking for a hub does not produce any errors,
    /// the hub is considered a viable candidate for virtual machine file storage.
    /// If this method is invoked for
    /// *VVolDefaultProfile*
    /// profile, then all the VVOL containers are returned as matching.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### hubs_to_search
    /// Candidate list of hubs, either datastores or storage pods or a
    /// mix. If this parameter is not specified, the Server uses all
    /// of the datastores and storage pods for placement compatibility
    /// checking.
    ///
    /// ### profile
    /// Storage requirement profile.
    ///
    /// ## Returns:
    ///
    /// Array of compatibility result objects. The results array contains
    /// one entry for each entry in the <code>hubsToSearch</code> list. If
    /// a hubs list is not specified, the results array contains one
    /// entry for each datastore and storage pod in your vSphere
    /// environment. Any errors are described in the results array.
    /// - If there is an invalid argument error, the compatibility
    ///   results will contain <code>InvalidArgument</code> faults
    ///   indicating that the profile does not exist or that it does not
    ///   match the requirement type.
    /// - If there are errors or warnings during compatibility checking,
    ///   the compatibility results will contain faults derived from
    ///   *PbmCompatibilityCheckFault*.
    /// - If this method is invoked for
    ///   *VVolDefaultProfile*
    ///   then the compatibility results will contain faults derived from
    ///   *PbmCompatibilityCheckFault* for non-vvol datastores.
    pub async fn pbm_check_compatibility(&self, hubs_to_search: Option<&[crate::types::structs::PbmPlacementHub]>, profile: &crate::types::structs::PbmProfileId) -> Result<Option<Vec<crate::types::structs::PbmPlacementCompatibilityResult>>> {
        let input = PbmCheckCompatibilityRequestType {hubs_to_search, profile, };
        let path = format!("/pbm/PbmPlacementSolver/{moId}/PbmCheckCompatibility", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Deprecated as of vSphere 2016, use *PbmPlacementSolver.PbmCheckRequirements* instead in order to retrieve compatibility status for both
    /// compute and storage location.
    /// 
    /// Performs placement compatibility checking based on a storage profile
    /// specification.
    /// 
    /// If compatibility checking for a hub does not produce
    /// any errors, the hub is considered a viable candidate for virtual
    /// machine file storage.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### hubs_to_search
    /// Candidate list of hubs, either datastores or storage pods
    /// or a mix. If this parameter is not specified, the Server uses all of the
    /// datastores and storage pods for placement compatibility checking.
    ///
    /// ### profile_spec
    /// Specification for a capability based profile.
    ///
    /// ## Returns:
    ///
    /// Array of compatibility result objects. The results array contains
    /// one entry for each entry in the <code>hubsToSearch</code> list.
    /// If a hubs list is not specified, the results array contains one entry
    /// for each datastore and storage pod in your vSphere environment.
    /// Any errors are described in the results array.
    /// - If there is an invalid argument error, the compatibility results
    ///   will contain <code>InvalidArgument</code> faults indicating that the
    ///   profile does not exist or that it does not match the requirement type.
    /// - If there are errors or warnings during compatibility checking,
    ///   the compatibility results will contain faults derived from
    ///   *PbmCompatibilityCheckFault*.
    pub async fn pbm_check_compatibility_with_spec(&self, hubs_to_search: Option<&[crate::types::structs::PbmPlacementHub]>, profile_spec: &crate::types::structs::PbmCapabilityProfileCreateSpec) -> Result<Option<Vec<crate::types::structs::PbmPlacementCompatibilityResult>>> {
        let input = PbmCheckCompatibilityWithSpecRequestType {hubs_to_search, profile_spec, };
        let path = format!("/pbm/PbmPlacementSolver/{moId}/PbmCheckCompatibilityWithSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Performs placement compatibility checking for the specified object to be placed based on its
    /// specified set of requirements.
    /// 
    /// If compatibility checking for a hub does not produce any
    /// errors, the hub is considered a viable candidate for virtual machine storage.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### hubs_to_search
    /// Candidate list of hubs, either datastores or storage pods
    /// or a mix. If this parameter is not specified, the Server uses all of the
    /// datastores and storage pods for placement compatibility checking.
    ///
    /// ### placement_subject_ref
    /// reference to the object being placed. Should be null when a new
    /// object is being provisioned. Should be specified when placement compatibility is being checked
    /// for an existing object. Supported objects are
    /// *virtualMachine*,
    /// *virtualMachineAndDisks*,
    /// *virtualDiskId*,
    /// *virtualDiskUUID*
    ///
    /// ### placement_subject_requirement
    /// Requirements including the policy requirements, compute
    /// requirements and capacity requirements. It is invalid to specify no requirements. It is also
    /// invalid to specify duplicate requirements or multiple conflicting requirements such as
    /// specifying both *PbmPlacementCapabilityConstraintsRequirement* and
    /// *PbmPlacementCapabilityProfileRequirement*.
    ///
    /// ## Returns:
    ///
    /// Array of compatibility result objects. The results array contains
    /// one entry for each entry in the <code>hubsToSearch</code> list.
    /// If hubs list is not specified, the results array contains one entry
    /// for each datastore and storage pod in your vSphere environment.
    /// Any errors are returned in the results array.
    /// - If there is an invalid argument error, the compatibility results
    ///   will contain <code>InvalidArgument</code> faults indicating that the
    ///   profile does not exist or that it does not match the requirement type.
    /// - If there are errors or warnings during compatibility checking,
    ///   the compatibility results will contain faults derived from
    ///   *PbmCompatibilityCheckFault*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if <code>placementSubjectRequirement</code> is null or empty or if
    /// there are duplicate or multiple conflicting requirements such as
    /// *PbmPlacementCapabilityConstraintsRequirement* and *PbmPlacementCapabilityProfileRequirement* both being
    /// specified.
    /// 
    /// ***PbmFault***: If there is an internal server error.
    pub async fn pbm_check_requirements(&self, hubs_to_search: Option<&[crate::types::structs::PbmPlacementHub]>, placement_subject_ref: Option<&crate::types::structs::PbmServerObjectRef>, placement_subject_requirement: Option<&[Box<dyn crate::types::traits::PbmPlacementRequirementTrait>]>) -> Result<Option<Vec<crate::types::structs::PbmPlacementCompatibilityResult>>> {
        let input = PbmCheckRequirementsRequestType {hubs_to_search, placement_subject_ref, placement_subject_requirement, };
        let path = format!("/pbm/PbmPlacementSolver/{moId}/PbmCheckRequirements", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Deprecated as of vSphere 2016, use *PbmPlacementSolver.PbmCheckRequirements*
    /// instead in order to retrieve both compatible compute and storage location.
    /// 
    /// Finds matching placement hubs for the specified requirements profile.
    /// 
    /// This
    /// method returns only those hubs that match the profile. If this method is
    /// invoked for
    /// *VVolDefaultProfile*
    /// profile, then all the VVOL containers are returned as matching.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### hubs_to_search
    /// Candidate list of hubs, either datastores or storage pods or a
    /// mix. If this parameter is not specified, the Server uses all
    /// of the datastores and storage pods.
    ///
    /// ### profile
    /// Storage requirement profile.
    ///
    /// ## Returns:
    ///
    /// Subset of the <code>hubsToSearch</code> list that satisfies the
    /// profile requirements. A storage pod is returned if and only if all
    /// its member datastores satisfy the profile requirements, whether
    /// the hubs list contains any of the member datastores or not. If a
    /// datastore and its storage pod are in the hubs list, and both
    /// satisfy the requirements, both are returned.
    ///
    /// ## Errors:
    ///
    /// ***PbmFault***: If there is an internal server error.
    pub async fn pbm_query_matching_hub(&self, hubs_to_search: Option<&[crate::types::structs::PbmPlacementHub]>, profile: &crate::types::structs::PbmProfileId) -> Result<Option<Vec<crate::types::structs::PbmPlacementHub>>> {
        let input = PbmQueryMatchingHubRequestType {hubs_to_search, profile, };
        let path = format!("/pbm/PbmPlacementSolver/{moId}/PbmQueryMatchingHub", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Deprecated as of vSphere 2016, use *PbmPlacementSolver.PbmCheckRequirements* instead in order to retrieve both compatible compute and storage location.
    /// 
    /// Finds matching placement hubs based on a profile creation specification.
    /// 
    /// This method returns only those hubs that match the specification.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### hubs_to_search
    /// Candidate list of hubs, either datastores or storage
    /// pods or a mix. If this parameter is not specified, the Server uses
    /// all of the datastores and storage pods for placement compatibility checking.
    ///
    /// ### create_spec
    /// Storage profile creation specification.
    ///
    /// ## Returns:
    ///
    /// Subset of the <code>hubsToSearch</code> list that satisfies the profile
    /// requirements. A storage pod is returned if and only if all its member datastores
    /// satisfy the profile requirements, whether the hubs list contains
    /// any of the member datastores or not. If a datastore and its storage pod
    /// are in the hubs list, and both satisfy the requirements, both are returned.
    ///
    /// ## Errors:
    ///
    /// ***PbmFault***: If there is an internal server error.
    pub async fn pbm_query_matching_hub_with_spec(&self, hubs_to_search: Option<&[crate::types::structs::PbmPlacementHub]>, create_spec: &crate::types::structs::PbmCapabilityProfileCreateSpec) -> Result<Option<Vec<crate::types::structs::PbmPlacementHub>>> {
        let input = PbmQueryMatchingHubWithSpecRequestType {hubs_to_search, create_spec, };
        let path = format!("/pbm/PbmPlacementSolver/{moId}/PbmQueryMatchingHubWithSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmCheckCompatibilityRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hubsToSearch")]
    hubs_to_search: Option<&'a [crate::types::structs::PbmPlacementHub]>,
    profile: &'a crate::types::structs::PbmProfileId,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmCheckCompatibilityWithSpecRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hubsToSearch")]
    hubs_to_search: Option<&'a [crate::types::structs::PbmPlacementHub]>,
    #[serde(rename = "profileSpec")]
    profile_spec: &'a crate::types::structs::PbmCapabilityProfileCreateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmCheckRequirementsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hubsToSearch")]
    hubs_to_search: Option<&'a [crate::types::structs::PbmPlacementHub]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "placementSubjectRef")]
    placement_subject_ref: Option<&'a crate::types::structs::PbmServerObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "placementSubjectRequirement")]
    placement_subject_requirement: Option<&'a [Box<dyn crate::types::traits::PbmPlacementRequirementTrait>]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQueryMatchingHubRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hubsToSearch")]
    hubs_to_search: Option<&'a [crate::types::structs::PbmPlacementHub]>,
    profile: &'a crate::types::structs::PbmProfileId,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQueryMatchingHubWithSpecRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hubsToSearch")]
    hubs_to_search: Option<&'a [crate::types::structs::PbmPlacementHub]>,
    #[serde(rename = "createSpec")]
    create_spec: &'a crate::types::structs::PbmCapabilityProfileCreateSpec,
}
