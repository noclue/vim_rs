use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *PbmProfileProfileManager* supports operations on virtual machine storage profiles.
/// 
/// A Storage Policy API profile consists of a set of _subprofiles_.
/// A subprofile corresponds to a _rule set_ in the vSphere Web Client.
/// 
/// Virtual machine storage profiles specify the storage requirements
/// for virtual machine files. You use the vSphere Web Client to define virtual machine
/// storage profiles. The requirements
/// (*PbmCapabilityProfile*.*PbmCapabilityProfile.constraints*)
/// impose constraints on the placement of virtual machine files.
/// 
/// The Storage Policy Server also supports datastore profiles. Datastore profiles
/// define storage capabilities. Storage capabilities are resources defined by
/// storage providers. Storage requirements are based on storage capabilities.
/// When you associate a storage profile with a virtual machine or virtual disk,
/// the Server sends the profile to the storage provider. When you perform compliance
/// checking (*PbmComplianceManager*), the storage provider
/// compares the requirements with the capabilities.
/// 
/// The *PbmProfileProfileManager* supports the following operations on
/// virtual machine storage profiles.
/// - Create, update, and delete storage profiles.
/// - Retrieve profile data based on specified criteria.
/// - Retrieve storage vendor data.
///   
/// The following figure shows the set of data objects that comprise
/// a storage profile specification (*PbmCapabilityProfileCreateSpec*).
/// You pass a storage profile specification to the Storage Policy Server
/// when you call the following methods:
/// - *PbmProfileProfileManager*.*PbmProfileProfileManager.PbmCreate*
/// - *PbmPlacementSolver*.*PbmPlacementSolver.PbmCheckCompatibilityWithSpec*
/// - *PbmPlacementSolver*.*PbmPlacementSolver.PbmQueryMatchingHubWithSpec*
/// <!-- -->
///      +---------------------------------+
///      |  PbmCapabilityProfileCreateSpec |
///      |                            name |     +-------------------------+
///      |                     description |     |  PbmProfileResourceType |
///      |                    resourceType ------|    resourceType=STORAGE |
///      |                     constraints ---   +-------------------------+
///      +---------------------------------+ |
///                                          |
///                                          |
///             +------------------------------------+
///             | PbmCapabilitySubProfileConstraints |
///             |                        subprofiles ---
///             +------------------------------------+ |
///                                                    | 1..n
///                                                    |
///                               +-------------------------+
///                               | PbmCapabilitySubProfile |
///                               |                    name |
///                               |          forceProvision |
///                               |              capability ---
///                               +-------------------------+ |
///                                                           | 1..n            +-------------------------------+
///                                                           |                 | PbmCapabilityMetadataUniqueId |
///                                         +-----------------------+           |                            id |
///                                         | PbmCapabilityInstance |           |                     namespace |
///                                         |                    id ------------+-------------------------------+
///                                         |            constraint ---
///                                         +-----------------------+ |
///                                                                   | 1..n
///                                                                   |                +-------------------------------+
///                                        +---------------------------------+         | PbmCapabilityPropertyInstance |
///                                        | PbmCapabilityConstraintInstance |  1..n   |                            id |
///                                        |                propertyInstance ----------|                         value |
///                                        +---------------------------------+         +-------------------------------+
#[derive(Clone)]
pub struct PbmProfileProfileManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl PbmProfileProfileManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Assign the given profile as the default profile for the given datastores.
    /// 
    /// This is an atomic operation. Either all the datastores will be assigned
    /// the default profile or none will be.
    /// In addition to StorageProfile.Update privilege, it requires
    /// Datastore.UpdateVirtualMachineFiles privilege on the given datastores to
    /// change the default profile for the datastores. Otherwise a NoPermission
    /// fault is thrown.
    /// 
    /// ***Required privileges:*** StorageProfile.Update
    ///
    /// ## Parameters:
    ///
    /// ### profile
    /// The profile that needs to be made default profile.
    ///
    /// ### datastores
    /// The datastores for which the profile needs to be made as default profile.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If one of the hub is not a datastore or profile cannot be
    /// used as default requirement profile for any of the hub.
    /// 
    /// ***PbmLegacyHubsNotSupported***: If any of the hub in datastores argument is legacy (VMFS
    /// or NFS) datastores.
    /// 
    /// ***PbmNonExistentHubs***: If any of the hub in datastores argument is non existent.
    /// 
    /// ***PbmFault***: Internal service error
    /// 
    /// ***PbmFaultNoPermission***: If user does not have Datastore.UpdateVirtualMachineFiles
    /// privilege on the given datastores.
    pub async fn pbm_assign_default_requirement_profile(&self, profile: &crate::types::structs::PbmProfileId, datastores: &[crate::types::structs::PbmPlacementHub]) -> Result<()> {
        let input = PbmAssignDefaultRequirementProfileRequestType {profile, datastores, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmAssignDefaultRequirementProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Creates a capability-based storage profile.
    /// 
    /// A capability-based profile
    /// contains requirements that are derived from tag-defined capabilities
    /// or from VMware VSAN capabilities.
    /// - Use the vSphere Web Client to define tags for capabilities.
    /// - VSAN storage capabilities are system-defined.
    ///   
    /// A profile is a collection of subprofiles
    /// (*PbmCapabilitySubProfile*).
    /// A subprofile references storage capabilities and defines requirements
    /// based on those capabilities.
    /// 
    /// To define a storage requirement, you specify constraint property instance values
    /// (*PbmCapabilityPropertyInstance*) that use Storage Policy API builtin
    /// types (*PbmBuiltinType_enum*) to create expressions
    /// for compliance checking.
    /// 
    /// The profile specification contains lists of constraint property instances
    /// (*PbmCapabilityProfileCreateSpec*.*PbmCapabilityProfileCreateSpec.constraints*.*PbmCapabilitySubProfileConstraints.subProfiles*\[\].*PbmCapabilitySubProfile.capability*\[\].*PbmCapabilityInstance.constraint*\[\].*PbmCapabilityConstraintInstance.propertyInstance*\[\]).
    /// The constraints are based on storage capabilities described in metadata
    /// (*PbmCapabilityPropertyMetadata*) and in the datastore profiles.
    /// 
    /// ***Required privileges:*** StorageProfile.Update
    ///
    /// ## Parameters:
    ///
    /// ### create_spec
    /// Capability-based profile specification.
    ///
    /// ## Returns:
    ///
    /// Identifier for the new profile.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if
    /// *PbmCapabilityProfileCreateSpec* is invalid.
    /// 
    /// ***PbmFaultProfileStorageFault***: if there is an error in persisting the profile.
    /// 
    /// ***PbmDuplicateName***: if a profile with the same name already exists.
    pub async fn pbm_create(&self, create_spec: &crate::types::structs::PbmCapabilityProfileCreateSpec) -> Result<crate::types::structs::PbmProfileId> {
        let input = PbmCreateRequestType {create_spec, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmCreate", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::PbmProfileId = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Deletes one or more profiles.
    /// 
    /// If the method successfully deletes a
    /// profile, its identifier is no longer valid.
    /// 
    /// ***Required privileges:*** StorageProfile.Update
    ///
    /// ## Parameters:
    ///
    /// ### profile_id
    /// Array of profile identifiers.
    ///
    /// ## Returns:
    ///
    /// Array of result objects, one for each profile specified in the
    /// call to the <code>PbmDelete</code> method.
    /// 
    /// The result object contains the profile ID and, if an error
    /// occurred, it also describes the fault. The method can return one
    /// of the following faults if the profile cannot be deleted:
    /// - *InvalidArgument* - Profile is not
    ///   recognized by the system.
    /// - *PbmFaultProfileStorageFault* - Internal service
    ///   error.
    /// - *PbmResourceInUse* - Profile is still associated
    ///   with an entity.
    pub async fn pbm_delete(&self, profile_id: &[crate::types::structs::PbmProfileId]) -> Result<Option<Vec<crate::types::structs::PbmProfileOperationOutcome>>> {
        let input = PbmDeleteRequestType {profile_id, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmDelete", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmProfileOperationOutcome>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Retrieves capability metadata.
    /// 
    /// Each capability metadata object has a unique identifier
    /// (*PbmCapabilityMetadata*.*PbmCapabilityMetadata.id*).
    /// The identifier object (*PbmCapabilityMetadataUniqueId*)
    /// contains the unique ID and it identifies the namespace to which
    /// the capability metadata object belongs.
    /// 
    /// Each registered namespace is required to be globally unique.
    /// You can associate a capability metadata object with a unique vendor and
    /// resource type by using the namespace and the
    /// *PbmCapabilityVendorResourceTypeInfo*
    /// data returned by the *PbmProfileProfileManager.PbmFetchVendorInfo* method.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### resource_type
    /// Type of profile resource. The Server supports the "STORAGE" resource
    /// type only. If not specified, this method will return capability metadata for the storage
    /// resources. Any other <code>resourceType</code> is considered invalid.
    ///
    /// ### vendor_uuid
    /// Unique identifier for the vendor/owner of capability
    /// metadata. The specified vendor ID must match
    /// *PbmCapabilitySchemaVendorInfo*.*PbmCapabilitySchemaVendorInfo.vendorUuid*.
    /// If omitted, the Server searchs all capability metadata registered with the system. If a
    /// <code>vendorUuid</code> unknown to the Server is specified, empty results will be returned.
    ///
    /// ## Returns:
    ///
    /// Array of capability metadata objects, classified by category
    /// (*PbmCapabilityMetadataPerCategory*.*PbmCapabilityMetadataPerCategory.subCategory*).
    pub async fn pbm_fetch_capability_metadata(&self, resource_type: Option<&crate::types::structs::PbmProfileResourceType>, vendor_uuid: Option<&str>) -> Result<Option<Vec<crate::types::structs::PbmCapabilityMetadataPerCategory>>> {
        let input = PbmFetchCapabilityMetadataRequestType {resource_type, vendor_uuid, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmFetchCapabilityMetadata", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmCapabilityMetadataPerCategory>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Returns the capability schema objects registered in the system.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### vendor_uuid
    /// Unique identifier for the vendor/owner of capability metadata.
    /// If omitted, the server searchs all capability metadata registered
    /// with the system. The specified vendor ID must match
    /// *PbmCapabilitySchemaVendorInfo*.*PbmCapabilitySchemaVendorInfo.vendorUuid*.
    ///
    /// ### line_of_service
    /// Optional line of service that must match *PbmLineOfServiceInfoLineOfServiceEnum_enum*.
    /// If specified, the capability schema objects
    /// are returned for the given lineOfServices. If null, then all
    /// capability schema objects that may or may not have data service capabilities
    /// are returned.
    ///
    /// ## Returns:
    ///
    /// Array of *PbmCapabilitySchema*
    ///
    /// ## Errors:
    ///
    /// ***PbmFault***: If there is an internal server error.
    /// 
    /// ***InvalidArgument***: If input lineOfServices has unknown/invalid line of service.
    pub async fn pbm_fetch_capability_schema(&self, vendor_uuid: Option<&str>, line_of_service: Option<&[String]>) -> Result<Option<Vec<crate::types::structs::PbmCapabilitySchema>>> {
        let input = PbmFetchCapabilitySchemaRequestType {vendor_uuid, line_of_service, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmFetchCapabilitySchema", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmCapabilitySchema>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Retrieves information about various resource types registered with the system.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Returns:
    ///
    /// Array of resource types.
    pub async fn pbm_fetch_resource_type(&self) -> Result<Option<Vec<crate::types::structs::PbmProfileResourceType>>> {
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmFetchResourceType", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmProfileResourceType>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Retrieve information about various capability metadata owners/vendors
    /// registered with the system, the resource type for which they are registered,
    /// and schema namespaces to which they belong.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### resource_type
    /// Specifies the resource type. The Server supports the STORAGE resource
    /// type only. If not specified, server defaults to STORAGE resource type. Any other
    /// <code>resourceType</code> is considered invalid.
    ///
    /// ## Returns:
    ///
    /// Vendor and namespace information.
    pub async fn pbm_fetch_vendor_info(&self, resource_type: Option<&crate::types::structs::PbmProfileResourceType>) -> Result<Option<Vec<crate::types::structs::PbmCapabilityVendorResourceTypeInfo>>> {
        let input = PbmFetchVendorInfoRequestType {resource_type, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmFetchVendorInfo", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmCapabilityVendorResourceTypeInfo>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Returns the profiles that can be made as default profile for all the given datastores.
    /// 
    /// A profile can be made as a default profile for a datastore only if it contains a ruleset
    /// from the namespace the datastore belongs to.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### datastores
    /// Datastores for which the default profile is found out. Note that
    /// the datastore pods/clusters are not supported.
    ///
    /// ## Returns:
    ///
    /// Profile\[\]
    /// Returns all the requirements profiles that can be made as default profile for the given datastores.
    /// If no profile can be made as default for all datastores, then an empty array is returned.
    /// Note that the profiles returned may or may not be compatible with the datastores.
    ///
    /// ## Errors:
    ///
    /// ***PbmLegacyHubsNotSupported***: If any of the hubs in datastores argument are legacy (VMFS or NFS) datastores.
    /// 
    /// ***PbmNonExistentHubs***: If any of the hubs in datastores argument are non existent.
    /// 
    /// ***PbmFault***: Internal service error.
    /// 
    /// ***InvalidArgument***: If the datastores argument contains a non-datastore, example storage pod.
    pub async fn pbm_find_applicable_default_profile(&self, datastores: &[crate::types::structs::PbmPlacementHub]) -> Result<Option<Vec<Box<dyn crate::types::traits::PbmProfileTrait>>>> {
        let input = PbmFindApplicableDefaultProfileRequestType {datastores, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmFindApplicableDefaultProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<Box<dyn crate::types::traits::PbmProfileTrait>>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Returns the virtual machine and disks that are associated with the given
    /// storage policies.
    /// 
    /// If the profiles parameter is empty, then this API returns
    /// all the virtual machine and disks that are associated with some storage
    /// policy.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### profiles
    /// Storage policy array.
    ///
    /// ## Returns:
    ///
    /// Array of QueryProfileResult
    ///
    /// ## Errors:
    ///
    /// ***PbmFault***: If there is an internal service error.
    pub async fn pbm_query_associated_entities(&self, profiles: Option<&[crate::types::structs::PbmProfileId]>) -> Result<Option<Vec<crate::types::structs::PbmQueryProfileResult>>> {
        let input = PbmQueryAssociatedEntitiesRequestType {profiles, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmQueryAssociatedEntities", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmQueryProfileResult>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Retrieves entities associated with the specified profile.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### profile
    /// Profile identifier.
    ///
    /// ### entity_type
    /// If specified, the method returns only those entities
    /// which match the type. The <code>entityType</code> string value must match
    /// one of the *PbmObjectType_enum* values.
    /// If not specified, the method returns all entities associated with the profile.
    ///
    /// ## Returns:
    ///
    /// Array of entities associated with the profile.
    ///
    /// ## Errors:
    ///
    /// ***PbmFault***: If there is an internal server error.
    pub async fn pbm_query_associated_entity(&self, profile: &crate::types::structs::PbmProfileId, entity_type: Option<&str>) -> Result<Option<Vec<crate::types::structs::PbmServerObjectRef>>> {
        let input = PbmQueryAssociatedEntityRequestType {profile, entity_type, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmQueryAssociatedEntity", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmServerObjectRef>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Returns identifiers for profiles associated with a virtual machine,
    /// virtual disk, or datastore.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// Reference to a virtual machine, virtual disk, or datastore.
    ///
    /// ## Returns:
    ///
    /// Array of profiles associated with the entity.
    ///
    /// ## Errors:
    ///
    /// ***PbmFault***: If there is an internal server error.
    pub async fn pbm_query_associated_profile(&self, entity: &crate::types::structs::PbmServerObjectRef) -> Result<Option<Vec<crate::types::structs::PbmProfileId>>> {
        let input = PbmQueryAssociatedProfileRequestType {entity, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmQueryAssociatedProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmProfileId>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Returns profiles associated with the specified entities.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### entities
    /// Array of server object references.
    ///
    /// ## Returns:
    ///
    /// Array of query result objects. Each *PbmQueryProfileResult*
    /// object identifies a virtual machine, virtual disk, or datastore
    /// and it contains a list of the profiles associated with that entity.
    /// It also describes the fault, if there is an error associated
    /// with one of the profiles.
    ///
    /// ## Errors:
    ///
    /// ***PbmFault***: If there is an internal server error.
    pub async fn pbm_query_associated_profiles(&self, entities: &[crate::types::structs::PbmServerObjectRef]) -> Result<Option<Vec<crate::types::structs::PbmQueryProfileResult>>> {
        let input = PbmQueryAssociatedProfilesRequestType {entities, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmQueryAssociatedProfiles", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmQueryProfileResult>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Returns the default requirement profile ID for the given datastore.
    /// 
    /// For
    /// legacy hub the API returns `null`.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### hub
    /// Placement hub (i.e. datastore).
    ///
    /// ## Returns:
    ///
    /// Profile Id of the Default Requirement Profile. For legacy hub the
    /// API returns `null`.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If hub is invalid (does not denote a datastore).
    /// 
    /// ***PbmNonExistentHubs***: If hub is non existent.
    /// 
    /// ***PbmFault***: Internal service error.
    pub async fn pbm_query_default_requirement_profile(&self, hub: &crate::types::structs::PbmPlacementHub) -> Result<Option<crate::types::structs::PbmProfileId>> {
        let input = PbmQueryDefaultRequirementProfileRequestType {hub, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmQueryDefaultRequirementProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::PbmProfileId>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Returns the default profiles for the given datastores.
    /// 
    /// For legacy
    /// datastores we set `DefaultProfileInfo.defaultProfile` to
    /// `null`.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### datastores
    /// The datastores for which the default profiles are requested. For
    /// legacy datastores we set
    /// `DefaultProfileInfo.defaultProfile` to `null`.
    ///
    /// ## Returns:
    ///
    /// DefaultProfileInfo Default profile information.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If one of the datastore is invalid (does not denote a
    /// datastore).
    /// 
    /// ***PbmNonExistentHubs***: If any of the datastore in datastores argument are non
    /// existent.
    /// 
    /// ***PbmFault***: Internal service error.
    pub async fn pbm_query_default_requirement_profiles(&self, datastores: &[crate::types::structs::PbmPlacementHub]) -> Result<Vec<crate::types::structs::PbmDefaultProfileInfo>> {
        let input = PbmQueryDefaultRequirementProfilesRequestType {datastores, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmQueryDefaultRequirementProfiles", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: Vec<crate::types::structs::PbmDefaultProfileInfo> = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Returns requirement profile ids or resource profile ids, or both.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### resource_type
    /// Type of resource. You can specify only STORAGE.
    ///
    /// ### profile_category
    /// Profile category. The string value must correspond
    /// to one of the *PbmProfileCategoryEnum_enum* values.
    /// If you do not specify a profile category, the method returns profiles in all
    /// categories.
    ///
    /// ## Returns:
    ///
    /// Array of storage profile identifiers.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the Server does not recognize the specified
    /// resourceType or profileCategory.
    pub async fn pbm_query_profile(&self, resource_type: &crate::types::structs::PbmProfileResourceType, profile_category: Option<&str>) -> Result<Option<Vec<crate::types::structs::PbmProfileId>>> {
        let input = PbmQueryProfileRequestType {resource_type, profile_category, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmQueryProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmProfileId>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Retrieves space statistics of a datastore.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// Entity for which space statistics are being requested i.e datastore.
    ///
    /// ### capability_profile_id
    /// \- capability profile Ids.
    /// If omitted, the statistics for the container
    /// as a whole would be returned.
    ///
    /// ## Returns:
    ///
    /// Array of Space stats of datastore for each capabilityProfileId.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: - Thrown if the input datastore parameter is null
    /// or its type is not datastore or its key is empty.
    /// 
    /// ***PbmFault***: - Thrown if server internal error occurred or
    /// if storage container does not support the profile.
    pub async fn pbm_query_space_stats_for_storage_container(&self, datastore: &crate::types::structs::PbmServerObjectRef, capability_profile_id: Option<&[crate::types::structs::PbmProfileId]>) -> Result<Option<Vec<crate::types::structs::PbmDatastoreSpaceStatistics>>> {
        let input = PbmQuerySpaceStatsForStorageContainerRequestType {datastore, capability_profile_id, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmQuerySpaceStatsForStorageContainer", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmDatastoreSpaceStatistics>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Deprecated since it is not supported.
    /// 
    /// Not supported in this release.
    /// 
    /// ***Required privileges:*** StorageProfile.Update
    ///
    /// ## Parameters:
    ///
    /// ### profile
    /// Profile to reset.
    pub async fn pbm_reset_default_requirement_profile(&self, profile: Option<&crate::types::structs::PbmProfileId>) -> Result<()> {
        let input = PbmResetDefaultRequirementProfileRequestType {profile, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmResetDefaultRequirementProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Resets the system pre-created VSAN default profile to factory defaults.
    /// 
    /// ***Required privileges:*** StorageProfile.Update
    pub async fn pbm_reset_v_san_default_profile(&self) -> Result<()> {
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmResetVSanDefaultProfile", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Returns one or more storage profiles.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### profile_ids
    /// Array of storage profile identifiers.
    ///
    /// ## Returns:
    ///
    /// Array of storage profiles.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the Server does not recognize any of the profileIds.
    pub async fn pbm_retrieve_content(&self, profile_ids: &[crate::types::structs::PbmProfileId]) -> Result<Vec<Box<dyn crate::types::traits::PbmProfileTrait>>> {
        let input = PbmRetrieveContentRequestType {profile_ids, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmRetrieveContent", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: Vec<Box<dyn crate::types::traits::PbmProfileTrait>> = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Updates a storage profile.
    /// 
    /// ***Required privileges:*** StorageProfile.Update
    ///
    /// ## Parameters:
    ///
    /// ### profile_id
    /// Profile identifier.
    ///
    /// ### update_spec
    /// Capability-based update specification.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the Server does not recognize *PbmProfileId*.
    /// 
    /// ***PbmFaultProfileStorageFault***: in case of internal service error.
    pub async fn pbm_update(&self, profile_id: &crate::types::structs::PbmProfileId, update_spec: &crate::types::structs::PbmCapabilityProfileUpdateSpec) -> Result<()> {
        let input = PbmUpdateRequestType {profile_id, update_spec, };
        let path = format!("/pbm/PbmProfileProfileManager/{moId}/PbmUpdate", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmAssignDefaultRequirementProfileRequestType<'a> {
    profile: &'a crate::types::structs::PbmProfileId,
    datastores: &'a [crate::types::structs::PbmPlacementHub],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmCreateRequestType<'a> {
    #[serde(rename = "createSpec")]
    create_spec: &'a crate::types::structs::PbmCapabilityProfileCreateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmDeleteRequestType<'a> {
    #[serde(rename = "profileId")]
    profile_id: &'a [crate::types::structs::PbmProfileId],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmFetchCapabilityMetadataRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType")]
    resource_type: Option<&'a crate::types::structs::PbmProfileResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vendorUuid")]
    vendor_uuid: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmFetchCapabilitySchemaRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vendorUuid")]
    vendor_uuid: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lineOfService")]
    line_of_service: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmFetchVendorInfoRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType")]
    resource_type: Option<&'a crate::types::structs::PbmProfileResourceType>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmFindApplicableDefaultProfileRequestType<'a> {
    datastores: &'a [crate::types::structs::PbmPlacementHub],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQueryAssociatedEntitiesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profiles: Option<&'a [crate::types::structs::PbmProfileId]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQueryAssociatedEntityRequestType<'a> {
    profile: &'a crate::types::structs::PbmProfileId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entityType")]
    entity_type: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQueryAssociatedProfileRequestType<'a> {
    entity: &'a crate::types::structs::PbmServerObjectRef,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQueryAssociatedProfilesRequestType<'a> {
    entities: &'a [crate::types::structs::PbmServerObjectRef],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQueryDefaultRequirementProfileRequestType<'a> {
    hub: &'a crate::types::structs::PbmPlacementHub,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQueryDefaultRequirementProfilesRequestType<'a> {
    datastores: &'a [crate::types::structs::PbmPlacementHub],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQueryProfileRequestType<'a> {
    #[serde(rename = "resourceType")]
    resource_type: &'a crate::types::structs::PbmProfileResourceType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "profileCategory")]
    profile_category: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQuerySpaceStatsForStorageContainerRequestType<'a> {
    datastore: &'a crate::types::structs::PbmServerObjectRef,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "capabilityProfileId")]
    capability_profile_id: Option<&'a [crate::types::structs::PbmProfileId]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmResetDefaultRequirementProfileRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a crate::types::structs::PbmProfileId>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmRetrieveContentRequestType<'a> {
    #[serde(rename = "profileIds")]
    profile_ids: &'a [crate::types::structs::PbmProfileId],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmUpdateRequestType<'a> {
    #[serde(rename = "profileId")]
    profile_id: &'a crate::types::structs::PbmProfileId,
    #[serde(rename = "updateSpec")]
    update_spec: &'a crate::types::structs::PbmCapabilityProfileUpdateSpec,
}
