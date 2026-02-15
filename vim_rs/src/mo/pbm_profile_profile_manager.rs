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
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::PbmProfileId = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::PbmProfileOperationOutcome>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::PbmCapabilityMetadataPerCategory>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::PbmCapabilitySchema>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::PbmProfileResourceType>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::PbmCapabilityVendorResourceTypeInfo>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::PbmProfileTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::PbmQueryProfileResult>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::PbmServerObjectRef>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::PbmProfileId>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::PbmQueryProfileResult>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::PbmProfileId>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Vec<crate::types::structs::PbmDefaultProfileInfo> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::PbmProfileId>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::PbmDatastoreSpaceStatistics>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Vec<Box<dyn crate::types::traits::PbmProfileTrait>> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
struct PbmAssignDefaultRequirementProfileRequestType<'a> {
    profile: &'a crate::types::structs::PbmProfileId,
    datastores: &'a [crate::types::structs::PbmPlacementHub],
}

impl<'a> miniserde::Serialize for PbmAssignDefaultRequirementProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmAssignDefaultRequirementProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmAssignDefaultRequirementProfileRequestTypeSer<'b, 'a> {
    data: &'b PbmAssignDefaultRequirementProfileRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmAssignDefaultRequirementProfileRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmAssignDefaultRequirementProfileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("profile"), &self.data.profile as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastores"), &self.data.datastores as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PbmCreateRequestType<'a> {
    create_spec: &'a crate::types::structs::PbmCapabilityProfileCreateSpec,
}

impl<'a> miniserde::Serialize for PbmCreateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmCreateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmCreateRequestTypeSer<'b, 'a> {
    data: &'b PbmCreateRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmCreateRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmCreateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("createSpec"), &self.data.create_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PbmDeleteRequestType<'a> {
    profile_id: &'a [crate::types::structs::PbmProfileId],
}

impl<'a> miniserde::Serialize for PbmDeleteRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmDeleteRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmDeleteRequestTypeSer<'b, 'a> {
    data: &'b PbmDeleteRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmDeleteRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmDeleteRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("profileId"), &self.data.profile_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PbmFetchCapabilityMetadataRequestType<'a> {
    resource_type: Option<&'a crate::types::structs::PbmProfileResourceType>,
    vendor_uuid: Option<&'a str>,
}

impl<'a> miniserde::Serialize for PbmFetchCapabilityMetadataRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmFetchCapabilityMetadataRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmFetchCapabilityMetadataRequestTypeSer<'b, 'a> {
    data: &'b PbmFetchCapabilityMetadataRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmFetchCapabilityMetadataRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmFetchCapabilityMetadataRequestType")),
                1 => {
                    let Some(ref val) = self.data.resource_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("resourceType"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.vendor_uuid else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vendorUuid"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct PbmFetchCapabilitySchemaRequestType<'a> {
    vendor_uuid: Option<&'a str>,
    line_of_service: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for PbmFetchCapabilitySchemaRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmFetchCapabilitySchemaRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmFetchCapabilitySchemaRequestTypeSer<'b, 'a> {
    data: &'b PbmFetchCapabilitySchemaRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmFetchCapabilitySchemaRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmFetchCapabilitySchemaRequestType")),
                1 => {
                    let Some(ref val) = self.data.vendor_uuid else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vendorUuid"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.line_of_service else { continue; };
                    return Some((std::borrow::Cow::Borrowed("lineOfService"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct PbmFetchVendorInfoRequestType<'a> {
    resource_type: Option<&'a crate::types::structs::PbmProfileResourceType>,
}

impl<'a> miniserde::Serialize for PbmFetchVendorInfoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmFetchVendorInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmFetchVendorInfoRequestTypeSer<'b, 'a> {
    data: &'b PbmFetchVendorInfoRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmFetchVendorInfoRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmFetchVendorInfoRequestType")),
                1 => {
                    let Some(ref val) = self.data.resource_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("resourceType"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct PbmFindApplicableDefaultProfileRequestType<'a> {
    datastores: &'a [crate::types::structs::PbmPlacementHub],
}

impl<'a> miniserde::Serialize for PbmFindApplicableDefaultProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmFindApplicableDefaultProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmFindApplicableDefaultProfileRequestTypeSer<'b, 'a> {
    data: &'b PbmFindApplicableDefaultProfileRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmFindApplicableDefaultProfileRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmFindApplicableDefaultProfileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastores"), &self.data.datastores as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PbmQueryAssociatedEntitiesRequestType<'a> {
    profiles: Option<&'a [crate::types::structs::PbmProfileId]>,
}

impl<'a> miniserde::Serialize for PbmQueryAssociatedEntitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmQueryAssociatedEntitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmQueryAssociatedEntitiesRequestTypeSer<'b, 'a> {
    data: &'b PbmQueryAssociatedEntitiesRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmQueryAssociatedEntitiesRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmQueryAssociatedEntitiesRequestType")),
                1 => {
                    let Some(ref val) = self.data.profiles else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profiles"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct PbmQueryAssociatedEntityRequestType<'a> {
    profile: &'a crate::types::structs::PbmProfileId,
    entity_type: Option<&'a str>,
}

impl<'a> miniserde::Serialize for PbmQueryAssociatedEntityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmQueryAssociatedEntityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmQueryAssociatedEntityRequestTypeSer<'b, 'a> {
    data: &'b PbmQueryAssociatedEntityRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmQueryAssociatedEntityRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmQueryAssociatedEntityRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("profile"), &self.data.profile as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.entity_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entityType"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct PbmQueryAssociatedProfileRequestType<'a> {
    entity: &'a crate::types::structs::PbmServerObjectRef,
}

impl<'a> miniserde::Serialize for PbmQueryAssociatedProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmQueryAssociatedProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmQueryAssociatedProfileRequestTypeSer<'b, 'a> {
    data: &'b PbmQueryAssociatedProfileRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmQueryAssociatedProfileRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmQueryAssociatedProfileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PbmQueryAssociatedProfilesRequestType<'a> {
    entities: &'a [crate::types::structs::PbmServerObjectRef],
}

impl<'a> miniserde::Serialize for PbmQueryAssociatedProfilesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmQueryAssociatedProfilesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmQueryAssociatedProfilesRequestTypeSer<'b, 'a> {
    data: &'b PbmQueryAssociatedProfilesRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmQueryAssociatedProfilesRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmQueryAssociatedProfilesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entities"), &self.data.entities as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PbmQueryDefaultRequirementProfileRequestType<'a> {
    hub: &'a crate::types::structs::PbmPlacementHub,
}

impl<'a> miniserde::Serialize for PbmQueryDefaultRequirementProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmQueryDefaultRequirementProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmQueryDefaultRequirementProfileRequestTypeSer<'b, 'a> {
    data: &'b PbmQueryDefaultRequirementProfileRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmQueryDefaultRequirementProfileRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmQueryDefaultRequirementProfileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hub"), &self.data.hub as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PbmQueryDefaultRequirementProfilesRequestType<'a> {
    datastores: &'a [crate::types::structs::PbmPlacementHub],
}

impl<'a> miniserde::Serialize for PbmQueryDefaultRequirementProfilesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmQueryDefaultRequirementProfilesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmQueryDefaultRequirementProfilesRequestTypeSer<'b, 'a> {
    data: &'b PbmQueryDefaultRequirementProfilesRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmQueryDefaultRequirementProfilesRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmQueryDefaultRequirementProfilesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastores"), &self.data.datastores as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PbmQueryProfileRequestType<'a> {
    resource_type: &'a crate::types::structs::PbmProfileResourceType,
    profile_category: Option<&'a str>,
}

impl<'a> miniserde::Serialize for PbmQueryProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmQueryProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmQueryProfileRequestTypeSer<'b, 'a> {
    data: &'b PbmQueryProfileRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmQueryProfileRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmQueryProfileRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("resourceType"), &self.data.resource_type as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.profile_category else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profileCategory"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct PbmQuerySpaceStatsForStorageContainerRequestType<'a> {
    datastore: &'a crate::types::structs::PbmServerObjectRef,
    capability_profile_id: Option<&'a [crate::types::structs::PbmProfileId]>,
}

impl<'a> miniserde::Serialize for PbmQuerySpaceStatsForStorageContainerRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmQuerySpaceStatsForStorageContainerRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmQuerySpaceStatsForStorageContainerRequestTypeSer<'b, 'a> {
    data: &'b PbmQuerySpaceStatsForStorageContainerRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmQuerySpaceStatsForStorageContainerRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmQuerySpaceStatsForStorageContainerRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.capability_profile_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("capabilityProfileId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct PbmResetDefaultRequirementProfileRequestType<'a> {
    profile: Option<&'a crate::types::structs::PbmProfileId>,
}

impl<'a> miniserde::Serialize for PbmResetDefaultRequirementProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmResetDefaultRequirementProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmResetDefaultRequirementProfileRequestTypeSer<'b, 'a> {
    data: &'b PbmResetDefaultRequirementProfileRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmResetDefaultRequirementProfileRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmResetDefaultRequirementProfileRequestType")),
                1 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct PbmRetrieveContentRequestType<'a> {
    profile_ids: &'a [crate::types::structs::PbmProfileId],
}

impl<'a> miniserde::Serialize for PbmRetrieveContentRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmRetrieveContentRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmRetrieveContentRequestTypeSer<'b, 'a> {
    data: &'b PbmRetrieveContentRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmRetrieveContentRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmRetrieveContentRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("profileIds"), &self.data.profile_ids as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PbmUpdateRequestType<'a> {
    profile_id: &'a crate::types::structs::PbmProfileId,
    update_spec: &'a crate::types::structs::PbmCapabilityProfileUpdateSpec,
}

impl<'a> miniserde::Serialize for PbmUpdateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmUpdateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmUpdateRequestTypeSer<'b, 'a> {
    data: &'b PbmUpdateRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PbmUpdateRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmUpdateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("profileId"), &self.data.profile_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("updateSpec"), &self.data.update_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
