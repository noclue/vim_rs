use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type provides the service interface for setting the
/// storage policy to one vSAN object, querying the vSAN object
/// status information, i.e., its storage profile, its health status.  
/// When the ManagedEntity is accessed with MOID of 'vsan-cluster-object-system'
/// through vSAN service at vCenter server and ESXi host side, it acts as
/// cluster-level APIs.
/// 
/// When it accessed with MOID of 'vsan-object-system' through vSAN service
/// at ESXi host side, its scope is only limited to that host.
#[derive(Clone)]
pub struct VsanObjectSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanObjectSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Delete vSAN Objects on vSAN cluster.
    /// 
    /// When the 'cluster' parameter is
    /// specified, this is a cluster level asynchronous API. The delete operation
    /// will return a task result which tracks progress, please check the task
    /// state when use.
    /// When it's called against host, it's a host level asynchronous API, which
    /// deprecates the 'DeleteVsanObjects' in *HostVsanInternalSystem*.
    /// WARNING: This API can be slow because we do IOs to all the objects.
    /// This API can be used to delete vSAN objects. vSAN won't allow access to
    /// objects which have lost quorum. Such objects can be deleted with the
    /// optional "force" flag.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### obj_uuids
    /// The vSAN object UUIDs to delete.
    ///
    /// ### force
    /// The force flag, inaccessible objects can't be deleted without
    /// this flag. WARNING: it's at your own risk.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_delete_objects_task(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, obj_uuids: &[String], force: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanDeleteObjectsRequestType {cluster, obj_uuids, force, };
        let path = format!("/vsan/VsanObjectSystem/{moId}/VsanDeleteObjects_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Query all inaccessible vmswap objects on a vSAN cluster.
    /// 
    /// It works on
    /// both cluster level and host level.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// list of inaccessible vmswap objects.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_inaccessible_vm_swap_objects(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<String>>> {
        let input = VsanQueryInaccessibleVmSwapObjectsRequestType {cluster, };
        let path = format!("/vsan/VsanObjectSystem/{moId}/VsanQueryInaccessibleVmSwapObjects", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<String>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Map from vSAN object UUIDs to their identities.
    /// 
    /// The results contain the identities of all objects that could be resolved. The
    /// identity includes things like the object type, references to the VM the object
    /// belongs to and such. If called against a host with MOID 'vsan-object-system',
    /// the cluster argument is ignored and, only those objects are resolved which
    /// can be resolved by that host. For VM related objects a host can only resolve
    /// the objects if the VM they belong to is registered on that host.
    /// If called against a host with MOID 'vsan-cluster-object-system', the cluster
    /// argument is ignored and cluster wide object identities would be returned
    /// except template VM, and in identities, the VM property would be left unset.
    /// If called against vCenter, all connected hosts will be contacted for
    /// this information and the results will be aggregated. Any VMs registered with
    /// hosts which are disconnected will hence not be resolved.
    /// **It's not recommended to use this API to get all of vSAN object identities
    /// at once to avoid potential performance issue due to large response in large
    /// scale cluster. Instead, we should limit the output by specifying various
    /// filter parameters like the objects UUID, object type, only including
    /// health or identity result etc.**
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### obj_uuids
    /// The vSAN object UUIDs for querying. Unset to query
    /// all of vSAN objects
    ///
    /// ### obj_types
    /// Querying the vSAN objects with given types, only the object whose
    /// type in this list will be returned. All of available object types
    /// include 'vmswap', 'vdisk', 'namespace', 'vmem', 'stats', 'other',
    /// 'iscsiHome', 'iscsiLun', 'iscsiTarget' and 'fileShare'.
    /// Unset to query all of vSAN objects with any type
    ///
    /// ### include_health
    /// If the result includes the object health status.
    /// Default is False
    ///
    /// ### include_obj_identity
    /// If the result includes all of the object identity.
    /// Default is True if it's unset
    ///
    /// ### include_space_summary
    /// If the result includes the vSAN space consumption
    /// summarizing by object type. Currently, it's not
    /// supported to include both of space summary and
    /// object identity in the result. Default is false.
    ///
    /// ### extra_query_spec
    /// This parameter only takes effect for host level API.
    /// The vSAN object identity query spec to specify
    /// detailed query specification like remote vSAN or VMFS
    /// datastores. If no extraQquerySpec is specified, the
    /// original logic to return all identities of local vSAN
    /// datastores will be applied. Note for any nonlocal
    /// vsan datastores provided, all the specified filter like
    /// objUuids, objTypes, include\* won't take affect.
    ///
    /// ## Returns:
    ///
    /// Object information structure
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_object_identities(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, obj_uuids: Option<&[String]>, obj_types: Option<&[String]>, include_health: Option<bool>, include_obj_identity: Option<bool>, include_space_summary: Option<bool>, extra_query_spec: Option<&crate::types::structs::VsanObjIdentityQuerySpec>) -> Result<Option<crate::types::structs::VsanObjectIdentityAndHealth>> {
        let input = VsanQueryObjectIdentitiesRequestType {cluster, obj_uuids, obj_types, include_health, include_obj_identity, include_space_summary, extra_query_spec, };
        let path = format!("/vsan/VsanObjectSystem/{moId}/VsanQueryObjectIdentities", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::VsanObjectIdentityAndHealth>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Query summary information about the syncing objects in this cluster.
    /// 
    /// It will retrieve information about syncing objects based on object
    /// filter options.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### syncing_object_filter
    /// Filter spec for summary.
    ///
    /// ## Returns:
    ///
    /// result VsanSyncingObjectQueryResult.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn query_syncing_vsan_objects_summary(&self, cluster: &crate::types::structs::ManagedObjectReference, syncing_object_filter: Option<&crate::types::structs::VsanSyncingObjectFilter>) -> Result<crate::types::structs::VsanHostVsanObjectSyncQueryResult> {
        let input = QuerySyncingVsanObjectsSummaryRequestType {cluster, syncing_object_filter, };
        let path = format!("/vsan/VsanObjectSystem/{moId}/QuerySyncingVsanObjectsSummary", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VsanHostVsanObjectSyncQueryResult = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Get information about the vSAN object/directory.
    /// 
    ///   
    /// If it's called in VC, the spbmProfileGenerationId can be safely ignored in
    /// vim.cluster.VsanObjectQuerySpec, it will be automatically set to the correct
    /// value by using SPBM service. If it's called in host with MOID
    /// 'vsan-object-system', both fields in
    /// *VsanObjectQuerySpec* should be provided, if
    /// spbmProfileGenerationId is not provided, the one from vSAN CMMDS will
    /// be used here, as a result, the mismatch field for storage compliance result
    /// is not reliable in some cases, e.g. profile is updated or deleted in vCenter
    /// after the vSAN object creation. If it's called in host with MOID
    /// 'vsan-cluster-object-system', the spbmProfileGenerationId in
    /// *VsanObjectQuerySpec* would be ignored and mismatch field
    /// is not reliable. Note this API can query maximum 500 vSAN objects
    /// at once when it's called from ESXi host for performance consideration,
    /// otherwise, the *InvalidArgument* exception will
    /// be thrown.
    ///   
    /// If the vSAN object can not be found,
    /// *FileNotFound* exception will be
    /// raised. If the vSAN object failed to read the policy, e.g. because it
    /// is not accessible, FileNotReadable exception will be raised.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### vsan_object_query_specs
    /// vSAN object query spec list.
    ///
    /// ## Returns:
    ///
    /// Object information list.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vos_query_vsan_object_information(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, vsan_object_query_specs: &[crate::types::structs::VsanObjectQuerySpec]) -> Result<Option<Vec<crate::types::structs::VsanObjectInformation>>> {
        let input = VosQueryVsanObjectInformationRequestType {cluster, vsan_object_query_specs, };
        let path = format!("/vsan/VsanObjectSystem/{moId}/VosQueryVsanObjectInformation", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VsanObjectInformation>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// The re-layout task for the objects which need change in object format.
    /// 
    /// This
    /// may involve physical on-disk re-layout of objects which will cause data
    /// movement in the cluster. Objects in the old format might not be able to use
    /// some of the new features.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The task object which represents the vSan
    /// RelayoutObjects task.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    pub async fn relayout_objects(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RelayoutObjectsRequestType {cluster, };
        let path = format!("/vsan/VsanObjectSystem/{moId}/RelayoutObjects", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Set the policy of the vSAN object.
    /// 
    /// The cluster parameters is ignored if called on ESX.  
    /// Profile can be 3 formats:  
    /// 1\. <code>VirtualMachineEmptyProfileSpec</code> means to use the empty vSAN
    /// policy. This is not the default policy, but a policy where all fields
    /// have default values.  
    /// 2\. <code>VirtualMachineDefinedProfileSpec</code> where profileId is set, in
    /// which case this profileId will be looked up in SPBM for the detailed
    /// policy information.  
    /// 3\. <code>VirtualMachineDefinedProfileSpec</code> where profileId is an empty
    /// string and instead the profileData is set for extensionKey
    /// 'com.vmware.vim.sps'. In this case the objectData field can be either the
    /// vSAN expression format, or a SPBM XML string. If no profile is
    /// supplied, and the call is executed against vCenter, then SPBM will be
    /// consulted for the vSAN datastore's default profile.  
    /// When this method returns successfully, the profile has been applied, but
    /// vSAN may still be remediating in order to implement the new policy.
    /// The health state of the object and resync information should be monitored to
    /// check on the progress.  
    /// Only the third option for profile is available when called on ESXi, other
    /// formats will raise InvalidArgument exception. If SPBM needs to be contacted,
    /// but SPBM is not available, RuntimeFault exception will be raised. If the
    /// profileId can not be resolved with SPBM, InvalidArgument exception will be
    /// raised. If objectData was provided but is neither of the two supported
    /// formats, InvalidArgument exception will be raised. If the vSAN object
    /// can not be found, FileNotFound exception will be raised. If the vSAN
    /// object failed to set the policy, e.g. because it is not accessible,
    /// FileNotWritable exception will be raised. If called against vCenter, but no
    /// ESXi host could be contacted to perform the operation, the exception is
    /// logged and the False is returned by this method.
    /// NOTE: This API requires additional Global.Settings privilege if called
    /// against vSphere 65P02 release.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** Global.Settings
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### vsan_object_uuid
    /// vSAN object UUID.
    ///
    /// ### profile
    /// See above description for all possible options.
    ///
    /// ## Returns:
    ///
    /// True when the operation successfully, False when it is failed and
    /// there is no exception raised.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vos_set_vsan_object_policy(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, vsan_object_uuid: &str, profile: Option<&dyn crate::types::traits::VirtualMachineProfileSpecTrait>) -> Result<bool> {
        let input = VosSetVsanObjectPolicyRequestType {cluster, vsan_object_uuid, profile, };
        let path = format!("/vsan/VsanObjectSystem/{moId}/VosSetVsanObjectPolicy", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: bool = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct VsanDeleteObjectsRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    obj_uuids: &'a [String],
    force: Option<bool>,
}

impl<'a> miniserde::Serialize for VsanDeleteObjectsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanDeleteObjectsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanDeleteObjectsRequestTypeSer<'b, 'a> {
    data: &'b VsanDeleteObjectsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanDeleteObjectsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanDeleteObjectsRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("objUuids"), &self.data.obj_uuids as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.force else { continue; };
                    return Some((std::borrow::Cow::Borrowed("force"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQueryInaccessibleVmSwapObjectsRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanQueryInaccessibleVmSwapObjectsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryInaccessibleVmSwapObjectsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryInaccessibleVmSwapObjectsRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryInaccessibleVmSwapObjectsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryInaccessibleVmSwapObjectsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryInaccessibleVmSwapObjectsRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQueryObjectIdentitiesRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    obj_uuids: Option<&'a [String]>,
    obj_types: Option<&'a [String]>,
    include_health: Option<bool>,
    include_obj_identity: Option<bool>,
    include_space_summary: Option<bool>,
    extra_query_spec: Option<&'a crate::types::structs::VsanObjIdentityQuerySpec>,
}

impl<'a> miniserde::Serialize for VsanQueryObjectIdentitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryObjectIdentitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryObjectIdentitiesRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryObjectIdentitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryObjectIdentitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryObjectIdentitiesRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.obj_uuids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("objUuids"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.obj_types else { continue; };
                    return Some((std::borrow::Cow::Borrowed("objTypes"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.include_health else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeHealth"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.include_obj_identity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeObjIdentity"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.include_space_summary else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeSpaceSummary"), val as &dyn miniserde::Serialize));
                }
                7 => {
                    let Some(ref val) = self.data.extra_query_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("extraQuerySpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QuerySyncingVsanObjectsSummaryRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    syncing_object_filter: Option<&'a crate::types::structs::VsanSyncingObjectFilter>,
}

impl<'a> miniserde::Serialize for QuerySyncingVsanObjectsSummaryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QuerySyncingVsanObjectsSummaryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QuerySyncingVsanObjectsSummaryRequestTypeSer<'b, 'a> {
    data: &'b QuerySyncingVsanObjectsSummaryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QuerySyncingVsanObjectsSummaryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QuerySyncingVsanObjectsSummaryRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.syncing_object_filter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("syncingObjectFilter"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VosQueryVsanObjectInformationRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    vsan_object_query_specs: &'a [crate::types::structs::VsanObjectQuerySpec],
}

impl<'a> miniserde::Serialize for VosQueryVsanObjectInformationRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VosQueryVsanObjectInformationRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VosQueryVsanObjectInformationRequestTypeSer<'b, 'a> {
    data: &'b VosQueryVsanObjectInformationRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VosQueryVsanObjectInformationRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VosQueryVsanObjectInformationRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("vsanObjectQuerySpecs"), &self.data.vsan_object_query_specs as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct RelayoutObjectsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for RelayoutObjectsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RelayoutObjectsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RelayoutObjectsRequestTypeSer<'b, 'a> {
    data: &'b RelayoutObjectsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RelayoutObjectsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RelayoutObjectsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VosSetVsanObjectPolicyRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    vsan_object_uuid: &'a str,
    profile: Option<&'a dyn crate::types::traits::VirtualMachineProfileSpecTrait>,
}

impl<'a> miniserde::Serialize for VosSetVsanObjectPolicyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VosSetVsanObjectPolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VosSetVsanObjectPolicyRequestTypeSer<'b, 'a> {
    data: &'b VosSetVsanObjectPolicyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VosSetVsanObjectPolicyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VosSetVsanObjectPolicyRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("vsanObjectUuid"), &self.data.vsan_object_uuid as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
