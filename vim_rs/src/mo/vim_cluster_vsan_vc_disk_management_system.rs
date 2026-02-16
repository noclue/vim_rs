use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// VimClusterVsanVcDiskManagementSystem enhances vSAN disk
/// management part, provides vSAN all flash disk group support,
/// exposes more detailed information of vSAN disk group, helps
/// on configure and manage vSAN disk group.
/// 
///   
/// It deprecates APIs *HostVsanSystem.AddDisks_Task* and
/// *HostVsanSystem.InitializeDisks_Task*, exposes
/// *VimVsanHostDiskMapInfoEx* through API
/// *VimClusterVsanVcDiskManagementSystem.QueryDiskMappings*,
/// to present whether a disk group is all flash, and whether
/// deduplication and compression is enabled on this disk group,
/// and deprecates *VsanHostConfigInfoStorageInfo.diskMapping*
/// exposed by *HostVsanSystem.config*.  
/// The ManagedEntity can be accessed through MOID of
/// vsan-disk-management-system, through vSAN service at vCenter
/// server side.
#[derive(Clone)]
pub struct VimClusterVsanVcDiskManagementSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VimClusterVsanVcDiskManagementSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// This API can be used to add new disk(s) to the storage pool for vSAN ESA
    /// consumption.
    /// 
    /// If StoragePoolDiskType is specified as 'SingleTier', disks will be added to
    /// 'Single Tier' storagepool.
    /// Eligible disks for storagepool can be queried from 'QueryDisksForVsan'
    /// *HostVsanSystem.QueryDisksForVsan*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### specs
    /// The specifications to add disks to vSAN storage pool. Please
    /// reference to VimVsanHostAddStoragePoolDiskSpec.
    /// Multiple specs can be used by AddStoragePoolDisks to add disks
    /// on different hosts in one API call.
    /// All hosts should in same vSAN cluster.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If below issues exist:
    /// 1. Specified host doesn't exist;
    /// 2. User doesn't have Host.Config.Storage
    ///    privilege on specified host;
    /// 3. Specified host is not connected to
    ///    vCenter server;
    /// 4. If none of specified disks for storage
    ///    pool is eligible for vSAN ESA;
    pub async fn vsan_add_storage_pool_disk(&self, specs: Option<&[crate::types::structs::VsanAddStoragePoolDiskSpec]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanAddStoragePoolDiskRequestType {specs, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/VsanAddStoragePoolDisk", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// This API can be used to delete a single or multiple disks from storage pool.
    /// 
    /// The disk(s) can be specified by their uniquely identifiable disk uuid.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vSAN cluster which owns the given storage pool disk(s).
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### spec
    /// The specification to delete vSAN storage pool. Please
    /// reference to VimVsanHostDeleteStoragePoolDiskSpec.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If any error happens during storage pool removal stage.
    pub async fn vsan_delete_storage_pool_disk(&self, cluster: &crate::types::structs::ManagedObjectReference, spec: &crate::types::structs::VsanDeleteStoragePoolDiskSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanDeleteStoragePoolDiskRequestType {cluster, spec, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/VsanDeleteStoragePoolDisk", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// This API is used to create new vSAN disk groups or attach
    /// more disks into existing vSAN disk group on specified host.
    /// 
    /// It deprecates APIs *HostVsanSystem.AddDisks_Task* and
    /// *HostVsanSystem.InitializeDisks_Task*, to support creating both hybrid
    /// and all flash disk group.
    /// Scsi disks eligible for vSAN service, could be retrieved through
    /// API *HostVsanSystem.QueryDisksForVsan*.
    /// If the "creationType" of the given spec is set to "vsandirect", then
    /// it will create vSAN direct datastores on each specified SSD and HDD.
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The specification to create vSAN disk group. Please
    /// reference to VimVsanHostDiskMappingCreationSpec;
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If below issues exist:
    /// 1. Specified host doesn't exist;
    /// 2. User doesn't have Host.Config.Storage
    ///    privilege on specified host;
    /// 3. Specified host is not connected to
    ///    vCenter server;
    /// 4. If requests to create all flash disk group
    ///    but host's ESX software doesn't support this
    ///    feature;
    /// 5. If requests to create all flash disk group
    ///    but host is not licensed to support this
    ///    feature;
    /// 6. If requests to append disks to existing
    ///    disk group, but specified more than one disks
    ///    for cache tier;
    /// 7. If none of specified disks for capacity tier
    ///    is eligible for vSAN;
    pub async fn initialize_disk_mappings(&self, spec: &crate::types::structs::VimVsanHostDiskMappingCreationSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = InitializeDiskMappingsRequestType {spec, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/InitializeDiskMappings", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Retrieve logical capacity, logical capacity used, physical capacity, physical
    /// capacity used and data efficiency metadata of a data efficiency enabled
    /// cluster.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn query_cluster_data_efficiency_capacity_state(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::VimVsanDataEfficiencyCapacityState> {
        let input = QueryClusterDataEfficiencyCapacityStateRequestType {cluster, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/QueryClusterDataEfficiencyCapacityState", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VimVsanDataEfficiencyCapacityState = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Get detailed information of vSAN disk groups managed by specified host,
    /// for each disk group, includes:
    /// Disk structure, presents as disk for cache tier, and disks for capacity tier;
    /// Whether is in-use for vSAN I/O by this host;
    /// Whether is all flash disk group;
    /// Whether vSAN deduplication and compression are enabled on this disk group.
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Target host to query.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// List of DiskMapInfoEx, please reference to VimVsanHostDiskMapInfoEx.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If below issues exist:
    /// 1. Specified host doesn't exist;
    /// 2. Specified host is not connected to
    ///    vCenter server;
    /// 3. Any unexpected runtime error;
    pub async fn query_disk_mappings(&self, host: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::VimVsanHostDiskMapInfoEx>>> {
        let input = QueryDiskMappingsRequestType {host, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/QueryDiskMappings", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VimVsanHostDiskMapInfoEx>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Get detailed information of all vSAN managed disks, which
    /// include disk groups, storage pool disks in vSAN datastore and all of
    /// vSAN direct storages for the specified host.
    /// 
    /// The output can be filtered
    /// to show only disk group or storage pool disks or vSAN direct disks. It can
    /// also be filtered to show details for an individual disk on the host by
    /// specifying filter criteria through *VimVsanHostQueryVsanDisksSpec*.
    /// The API
    /// *VimClusterVsanVcDiskManagementSystem.QueryDiskMappings*
    /// will be deprecated and replaced with this API.
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Target host to query.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### filter_spec
    /// Specification to filter vSAN disks.
    /// This spec can be used to filter query results by disktype types
    /// (diskGroup/storagePool/directDisk) or by diskName. If this spec is
    /// not specified, it will return all vSAN disks in the cluster.
    /// Refer *VimVsanHostQueryVsanDisksSpec* for more details.
    ///
    /// ## Returns:
    ///
    /// All of vSAN managed disks including vSAN disk groups and vSAN
    /// direct storages
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If below issues exist:
    /// 1. Specified host doesn't exist;
    /// 2. Specified host is not connected to
    ///    vCenter server;
    /// 3. Any unexpected runtime error;
    pub async fn query_vsan_managed_disks(&self, host: &crate::types::structs::ManagedObjectReference, filter_spec: Option<&crate::types::structs::VimVsanHostQueryVsanDisksSpec>) -> Result<Option<crate::types::structs::VimVsanHostVsanManagedDisksInfo>> {
        let input = QueryVsanManagedDisksRequestType {host, filter_spec, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/QueryVsanManagedDisks", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::VimVsanHostVsanManagedDisksInfo>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// This API is used to rebuild an existing vSAN disk mapping on the
    /// specified host.
    /// 
    /// Please note that the API does not support rebuilding unmounted DGs.
    /// Rebuild disk mapping is done in two steps:
    /// 1\. remove the existing disk mapping with user specified data evacuation mode;
    /// 2\. perform a trim operation on every disk in DG.
    /// 3\. re-create a new disk mapping with the same set of disks.
    /// Removal of a disk mapping may cause data movement so a disk data evacuation
    /// resource check *VsanResourceCheckSystem.VsanPerformResourceCheck*
    /// will be done before removing the disk mapping. Before re-creating new disk
    /// mapping, the disks will be trimmed for refreshing possible bad blocks in the
    /// meta-data section of the disks.
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Target host to rebuild.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### mapping
    /// The disk mapping to be rebuilt from VSAN usage.
    ///
    /// ### maintenance_spec
    /// Specifies the data evacuation mode.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If below issues exist:
    /// 1. Specified host doesn't exist;
    /// 2. User doesn't have Host.Config.Storage
    ///    privilege on specified host;
    /// 3. Specified host is not connected to
    ///    vCenter server;
    /// 4. Resource check completes with "red"
    ///    status;
    pub async fn rebuild_disk_mapping(&self, host: &crate::types::structs::ManagedObjectReference, mapping: &crate::types::structs::VsanHostDiskMapping, maintenance_spec: &crate::types::structs::HostMaintenanceSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RebuildDiskMappingRequestType {host, mapping, maintenance_spec, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/RebuildDiskMapping", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Remove vSAN capacity-tier disk(s) from use in a vSAN cluster with the
    /// specified data evacuation mode or remove vSAN direct disk(s).
    /// 
    /// It will auto-detect whether the disk(s) provided belong to
    /// vSAN capacity-tier disk or vSAN direct.
    /// If the disk(s) belong to vSAN capacity-tier disk, then vSAN disk
    /// will be removed. If the disk(s) belong to vSAN direct, then the
    /// vSAN direct disk(s) will be removed.
    /// For vSAN disk(s) removal, disk(s) will be removed one by one
    /// with the specified data evacuation mode.
    /// Removing a disk may cause data movement so a
    /// disk data evacuation resource check
    /// *VsanResourceCheckSystem.VsanPerformResourceCheck*
    /// will be done before removing each disk.
    /// Note: This method cannot be used to remove the cache-tier disk or the last
    /// capacity-tier disk of a disk mapping since that will cause the whole disk
    /// mapping to be removed. Use
    /// *VimClusterVsanVcDiskManagementSystem.RemoveDiskMappingEx* instead if that is
    /// the case.
    /// For vSAN direct disk(s) removal, the storage system build on top of
    /// these vSAN direct disk(s) will be destroyed one by one with the specified
    /// data evacuation mode.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vSAN cluster which owns the given vSAN disk(s)
    /// and(or) vSAN direct disk(s).
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### disks
    /// List of vSAN disk(s) to be removed.
    ///
    /// ### maintenance_spec
    /// Specifies the data evacuation mode.
    ///
    /// ## Returns:
    ///
    /// A task to monitor the progress of disk(s) removal operation.
    /// If the removal operation includes vSAN disk(s) removal,
    /// then upon successful completion of the returned task, its
    /// *TaskInfo.result* field will be populated with a
    /// *VsanHostDiskResult*\[\] filled with details for each
    /// vSAN disk.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If resource check completes with "red" status or any other
    /// error happens during the resource check stage or actual
    /// disk(s) removal stage for vSAN disk(s).
    /// If the specified disk is the last capacity-tier disk in a
    /// disk mapping, a more specific
    /// vim.fault.DiskIsLastRemainingNonSSD (which is a subclass
    /// of VsanFault) will be thrown instead.
    pub async fn remove_disk_ex(&self, cluster: &crate::types::structs::ManagedObjectReference, disks: &[crate::types::structs::HostScsiDisk], maintenance_spec: &crate::types::structs::HostMaintenanceSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RemoveDiskExRequestType {cluster, disks, maintenance_spec, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/RemoveDiskEx", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Remove vSAN disk mapping(s) from use in a vSAN cluster with the
    /// specified data evacuation mode.
    /// 
    /// Disk mapping(s) will be removed one by one with the specified data
    /// evacuation mode. Removing a disk mapping may cause data movement so a
    /// disk data evacuation resource check
    /// *VsanResourceCheckSystem.VsanPerformResourceCheck*
    /// will be done before removing each disk mapping.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vSAN cluster which owns the given disk mapping(s).
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### mappings
    /// List of disk mapping(s) to be removed.
    ///
    /// ### maintenance_spec
    /// Specifies the data evacuation mode.
    ///
    /// ## Returns:
    ///
    /// A task to monitor the progress of disk mapping(s) removal operation.
    /// Upon successful completion of the returned task, its
    /// *TaskInfo.result* field will be populated with a
    /// *VsanHostDiskMapResult*\[\] filled with details for each
    /// disk mapping.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If resource check completes with "red" status or any error
    /// happens during the resource check stage or actual
    /// disk mapping(s) removal stage.
    pub async fn remove_disk_mapping_ex(&self, cluster: &crate::types::structs::ManagedObjectReference, mappings: &[crate::types::structs::VsanHostDiskMapping], maintenance_spec: &crate::types::structs::HostMaintenanceSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RemoveDiskMappingExRequestType {cluster, mappings, maintenance_spec, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/RemoveDiskMappingEx", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Retrieve capabilities for hosts reside in specified cluster,
    /// to figure out whether all flash disk group is supported by
    /// hosts' ESXi softwares, and whether they are licensed.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Target cluster, of which hosts' capabilities
    /// will be queried;
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// vim.vsan.host.VsanHostCapability\[\]
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn retrieve_all_flash_capabilities(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::VimVsanHostVsanHostCapability>>> {
        let input = RetrieveAllFlashCapabilitiesRequestType {cluster, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/RetrieveAllFlashCapabilities", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VimVsanHostVsanHostCapability>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Unmount vSAN disk mapping(s) in a vSAN cluster with the specified
    /// data evacuation mode.
    /// 
    /// Disk mapping(s) will be unmounted one by one with the specified data
    /// evacuation mode. Unmount a disk mapping may cause data movement so a
    /// disk data evacuation resource check
    /// *VsanResourceCheckSystem.VsanPerformResourceCheck*
    /// will be done before unmounting each disk mapping.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vSAN cluster which owns the given disk mapping(s).
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### mappings
    /// List of disk mapping(s) to be unmounted.
    ///
    /// ### maintenance_spec
    /// Specifies the data evacuation mode.
    ///
    /// ## Returns:
    ///
    /// A task to monitor the progress of disk mapping(s) unmount operation.
    /// Upon successful completion of the returned task, its
    /// *TaskInfo.result* field will be populated with a
    /// *VsanHostDiskMapResult*\[\] filled with details for each
    /// disk mapping.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If resource check completes with "red" status or any error
    /// happens during the resource check stage or actual
    /// disk mapping(s) unmount stage.
    pub async fn unmount_disk_mapping_ex(&self, cluster: &crate::types::structs::ManagedObjectReference, mappings: &[crate::types::structs::VsanHostDiskMapping], maintenance_spec: &crate::types::structs::HostMaintenanceSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UnmountDiskMappingExRequestType {cluster, mappings, maintenance_spec, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/UnmountDiskMappingEx", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// This API can be used to unmount a single or multiple disks from the storage
    /// pool.
    /// 
    /// The disk(s) can be specified by their uniquely identifiable disk UUID.
    /// Storage pool disk(s) will be unmounted with the specified data evacuation
    /// mode. Unmount a storage pool disk may cause data movement so a disk data
    /// evacuation resource check
    /// *VsanResourceCheckSystem.VsanPerformResourceCheck* will be done
    /// before unmounting each storage pool disk.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vSAN cluster which owns the given storage pool disk(s).
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### spec
    /// The specification to unmount vSAN storage pool. Please
    /// reference to VimVsanHostDeleteStoragePoolDiskSpec.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If resource check completes with "red" status or any error
    /// happens during the resource check stage or actual storage
    /// disk unmount stage.
    pub async fn vsan_unmount_storage_pool_disks(&self, cluster: &crate::types::structs::ManagedObjectReference, spec: &crate::types::structs::VsanDeleteStoragePoolDiskSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanUnmountStoragePoolDisksRequestType {cluster, spec, };
        let path = format!("/vsan/VimClusterVsanVcDiskManagementSystem/{moId}/VsanUnmountStoragePoolDisks", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct VsanAddStoragePoolDiskRequestType<'a> {
    specs: Option<&'a [crate::types::structs::VsanAddStoragePoolDiskSpec]>,
}

impl<'a> miniserde::Serialize for VsanAddStoragePoolDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanAddStoragePoolDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanAddStoragePoolDiskRequestTypeSer<'b, 'a> {
    data: &'b VsanAddStoragePoolDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanAddStoragePoolDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanAddStoragePoolDiskRequestType")),
                1 => {
                    let Some(ref val) = self.data.specs else { continue; };
                    return Some((std::borrow::Cow::Borrowed("specs"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanDeleteStoragePoolDiskRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::VsanDeleteStoragePoolDiskSpec,
}

impl<'a> miniserde::Serialize for VsanDeleteStoragePoolDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanDeleteStoragePoolDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanDeleteStoragePoolDiskRequestTypeSer<'b, 'a> {
    data: &'b VsanDeleteStoragePoolDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanDeleteStoragePoolDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanDeleteStoragePoolDiskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct InitializeDiskMappingsRequestType<'a> {
    spec: &'a crate::types::structs::VimVsanHostDiskMappingCreationSpec,
}

impl<'a> miniserde::Serialize for InitializeDiskMappingsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(InitializeDiskMappingsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct InitializeDiskMappingsRequestTypeSer<'b, 'a> {
    data: &'b InitializeDiskMappingsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for InitializeDiskMappingsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"InitializeDiskMappingsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryClusterDataEfficiencyCapacityStateRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryClusterDataEfficiencyCapacityStateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryClusterDataEfficiencyCapacityStateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryClusterDataEfficiencyCapacityStateRequestTypeSer<'b, 'a> {
    data: &'b QueryClusterDataEfficiencyCapacityStateRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryClusterDataEfficiencyCapacityStateRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryClusterDataEfficiencyCapacityStateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryDiskMappingsRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryDiskMappingsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDiskMappingsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDiskMappingsRequestTypeSer<'b, 'a> {
    data: &'b QueryDiskMappingsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDiskMappingsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDiskMappingsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryVsanManagedDisksRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
    filter_spec: Option<&'a crate::types::structs::VimVsanHostQueryVsanDisksSpec>,
}

impl<'a> miniserde::Serialize for QueryVsanManagedDisksRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVsanManagedDisksRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVsanManagedDisksRequestTypeSer<'b, 'a> {
    data: &'b QueryVsanManagedDisksRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryVsanManagedDisksRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVsanManagedDisksRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.filter_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("filterSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RebuildDiskMappingRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
    mapping: &'a crate::types::structs::VsanHostDiskMapping,
    maintenance_spec: &'a crate::types::structs::HostMaintenanceSpec,
}

impl<'a> miniserde::Serialize for RebuildDiskMappingRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RebuildDiskMappingRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RebuildDiskMappingRequestTypeSer<'b, 'a> {
    data: &'b RebuildDiskMappingRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RebuildDiskMappingRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RebuildDiskMappingRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("mapping"), &self.data.mapping as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("maintenanceSpec"), &self.data.maintenance_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveDiskExRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    disks: &'a [crate::types::structs::HostScsiDisk],
    maintenance_spec: &'a crate::types::structs::HostMaintenanceSpec,
}

impl<'a> miniserde::Serialize for RemoveDiskExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveDiskExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveDiskExRequestTypeSer<'b, 'a> {
    data: &'b RemoveDiskExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveDiskExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveDiskExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("disks"), &self.data.disks as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("maintenanceSpec"), &self.data.maintenance_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveDiskMappingExRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    mappings: &'a [crate::types::structs::VsanHostDiskMapping],
    maintenance_spec: &'a crate::types::structs::HostMaintenanceSpec,
}

impl<'a> miniserde::Serialize for RemoveDiskMappingExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveDiskMappingExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveDiskMappingExRequestTypeSer<'b, 'a> {
    data: &'b RemoveDiskMappingExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveDiskMappingExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveDiskMappingExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("mappings"), &self.data.mappings as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("maintenanceSpec"), &self.data.maintenance_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveAllFlashCapabilitiesRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for RetrieveAllFlashCapabilitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveAllFlashCapabilitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveAllFlashCapabilitiesRequestTypeSer<'b, 'a> {
    data: &'b RetrieveAllFlashCapabilitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveAllFlashCapabilitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveAllFlashCapabilitiesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UnmountDiskMappingExRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    mappings: &'a [crate::types::structs::VsanHostDiskMapping],
    maintenance_spec: &'a crate::types::structs::HostMaintenanceSpec,
}

impl<'a> miniserde::Serialize for UnmountDiskMappingExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnmountDiskMappingExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnmountDiskMappingExRequestTypeSer<'b, 'a> {
    data: &'b UnmountDiskMappingExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnmountDiskMappingExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnmountDiskMappingExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("mappings"), &self.data.mappings as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("maintenanceSpec"), &self.data.maintenance_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanUnmountStoragePoolDisksRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::VsanDeleteStoragePoolDiskSpec,
}

impl<'a> miniserde::Serialize for VsanUnmountStoragePoolDisksRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanUnmountStoragePoolDisksRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanUnmountStoragePoolDisksRequestTypeSer<'b, 'a> {
    data: &'b VsanUnmountStoragePoolDisksRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanUnmountStoragePoolDisksRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanUnmountStoragePoolDisksRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
