use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::HostMaintenanceSpec;
use crate::types::HostScsiDisk;
use crate::types::ManagedObjectReference;
use crate::types::VsanHostClusterStatus;
use crate::types::VsanHostConfigInfo;
use crate::types::VsanHostDiskMapping;
use crate::types::VsanHostDiskResult;
/// The VsanSystem managed object type exposes VSAN configuration
/// primitives and serves as a host-level access point for relevant
/// VSAN data objects.
pub struct HostVsanSystem {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HostVsanSystem {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Add the set of given disks for use by the VSAN service on this host.
    /// 
    /// Users may use this API to manually add disks for use by VSAN, without
    /// specifying an explicit *VsanHostDiskMapping*, when the VSAN service
    /// not configured to automatically claim storage. Any ineligible disk in
    /// the set of given disks and disks which would have exceeded
    /// the capacity will be ignored and will not be published in
    /// returned *TaskInfo.result*.
    /// 
    /// Mount a *VsanHostDiskMapping* if the specified disk belongs to the
    /// unmounted mapping and is of type *VsanHostDiskMapping.ssd*.
    /// 
    /// Upon successful completion of the returned *Task*, its
    /// *TaskInfo.result* field will be populated with a
    /// *VsanHostDiskMapResult*\[\] and caller must inspect
    /// *VsanHostDiskMapResult*\[\] to check result for individual
    /// *VsanHostDiskMapping*.
    /// 
    /// See also *HostVsanSystem.QueryDisksForVsan*, *VsanHostConfigInfoStorageInfo.autoClaimStorage*, *VsanHostDiskMapInfo.mounted*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### disk
    /// list of disks to add for use by the VSAN service
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn add_disks_task(&self, disk: &[HostScsiDisk]) -> Result<ManagedObjectReference> {
        let input = AddDisksRequestType {disk, };
        let path = format!("/HostVsanSystem/{moId}/AddDisks_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Evacuate this host from VSAN cluster.
    /// 
    /// The task is cancellable.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### maintenance_spec
    /// Specifies the data evacuation mode. See *HostMaintenanceSpec*.
    /// If unspecified, the default mode chosen will be
    /// *ensureObjectAccessibility*.
    ///
    /// ### timeout
    /// Time to wait for the task to complete in seconds.
    /// If the value is less than or equal to zero, there
    /// is no timeout. The operation fails with a Timedout
    /// exception if it timed out.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: If the host is entering maintenance mode or evacuating data.
    /// 
    /// ***RequestCanceled***: if the operation is canceled.
    /// 
    /// ***Timedout***: if the operation timed out.
    /// 
    /// ***VsanFault***: if operation fails with VSAN-specific error.
    pub async fn evacuate_vsan_node_task(&self, maintenance_spec: &HostMaintenanceSpec, timeout: i32) -> Result<ManagedObjectReference> {
        let input = EvacuateVsanNodeRequestType {maintenance_spec, timeout, };
        let path = format!("/HostVsanSystem/{moId}/EvacuateVsanNode_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Initialize and use the sets of disks in the given *VsanHostDiskMapping*
    /// list for the VSAN service on this host.
    /// 
    /// Users may use this API to specify or
    /// change disk mappings when the VSAN service is not configured to
    /// automatically claim storage. For appending new non-SSDs to an existing
    /// *VsanHostDiskMapping*, users need to specify only the new non-SSDs
    /// with its *VsanHostDiskMapping.ssd*.
    /// 
    /// Mount a *VsanHostDiskMapping* if the specified
    /// *VsanHostDiskMapping* is not mounted in this host.
    /// 
    /// Upon successful completion of the returned *Task*, its
    /// *TaskInfo.result* field will be populated with a
    /// *VsanHostDiskMapResult*\[\] and caller must inspect
    /// *VsanHostDiskMapResult*\[\] to check result for individual
    /// *VsanHostDiskMapping*.
    /// 
    /// See also *HostVsanSystem.QueryDisksForVsan*, *VsanHostConfigInfoStorageInfo.autoClaimStorage*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### mapping
    /// list of disk mappings to initialize
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn initialize_disks_task(&self, mapping: &[VsanHostDiskMapping]) -> Result<ManagedObjectReference> {
        let input = InitializeDisksRequestType {mapping, };
        let path = format!("/HostVsanSystem/{moId}/InitializeDisks_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Queries disks on this host for suitability to use with the VSAN service,
    /// and returns the result.
    /// 
    /// See also *ScsiLun.canonicalName*.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### canonical_name
    /// may be set to restrict the query to the list of
    /// *HostScsiDisk* objects named by the
    /// given paths
    ///
    /// ## Returns:
    ///
    /// a list of populated *VsanHostDiskResult* entries
    pub async fn query_disks_for_vsan(&self, canonical_name: Option<&[String]>) -> Result<Option<Vec<VsanHostDiskResult>>> {
        let input = QueryDisksForVsanRequestType {canonical_name, };
        let path = format!("/HostVsanSystem/{moId}/QueryDisksForVsan", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Queries this host's current runtime status for the VSAN service.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// a populated *VsanHostClusterStatus* entry
    pub async fn query_host_status(&self) -> Result<VsanHostClusterStatus> {
        let path = format!("/HostVsanSystem/{moId}/QueryHostStatus", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Recommission this host to VSAN cluster.
    /// 
    /// Users may use this API to recommission a node that has been
    /// evacuated in *VsanHostDecommissionMode*.
    /// 
    /// See also *HostVsanSystem.EvacuateVsanNode_Task*, *VsanHostDecommissionMode*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is not evacuated.
    /// 
    /// ***VsanFault***: if operation fails with VSAN-specific error.
    pub async fn recommission_vsan_node_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/HostVsanSystem/{moId}/RecommissionVsanNode_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Remove the set of given disks from use by the VSAN service on this host.
    /// 
    /// Users
    /// may use this API to manually remove a *VsanHostDiskMapping.nonSsd*
    /// from a *VsanHostDiskMapping*. This operation is only permitted if the
    /// VSAN service on this host is not configured to automatically claim storage.
    /// 
    /// The task is cancellable.
    /// 
    /// This method may not be used to remove the last
    /// *VsanHostDiskMapping.nonSsd* from any given
    /// *VsanHostDiskMapping*. Removal of the last
    /// *VsanHostDiskMapping.nonSsd* can be accomplished by using
    /// *HostVsanSystem.RemoveDiskMapping_Task*.
    /// 
    /// Upon successful completion of the returned *Task*, its
    /// *TaskInfo.result* field will be populated with a
    /// *VsanHostDiskResult*\[\]. Sets DiskIsLastRemainingNonSSD fault
    /// in returned task if specified disk is the last
    /// *VsanHostDiskMapping.nonSsd* member of
    /// *VsanHostDiskMapping*.
    /// 
    /// See also *HostVsanSystem.RemoveDiskMapping_Task*, *HostVsanSystem.UpdateVsan_Task*, *VsanHostConfigInfoStorageInfo.autoClaimStorage*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### disk
    /// list of disks to be removed from use by the VSAN service.
    ///
    /// ### maintenance_spec
    /// Any additional actions to move data out of the disk
    /// before removing it. See *HostMaintenanceSpec*.
    /// If unspecified, there is no action taken to move
    /// data from the disk.
    ///
    /// ### timeout
    /// Time to wait for the task to complete in seconds.
    /// If the value is less than or equal to zero, there
    /// is no timeout. The operation fails with a Timedout
    /// exception if it timed out.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn remove_disk_task(&self, disk: &[HostScsiDisk], maintenance_spec: Option<&HostMaintenanceSpec>, timeout: Option<i32>) -> Result<ManagedObjectReference> {
        let input = RemoveDiskRequestType {disk, maintenance_spec, timeout, };
        let path = format!("/HostVsanSystem/{moId}/RemoveDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Delete given set of disk mappings from use by the VSAN service on this host.
    /// 
    /// This API may be used to remove all disks in a given mapping, including its
    /// *VsanHostDiskMapping.ssd*. This operation is only permitted
    /// if the VSAN service on this host is not configured to automatically
    /// claim storage.
    /// 
    /// The task is cancellable.
    /// 
    /// Upon successful completion of the returned *Task*, its
    /// *TaskInfo.result* field will be populated with an empty
    /// *VsanHostDiskMapResult*\[\]. If any errors are encountered,
    /// the returned field will instead contain populated error information.
    /// 
    /// See also *HostVsanSystem.RemoveDisk_Task*, *HostVsanSystem.UpdateVsan_Task*, *VsanHostConfigInfoStorageInfo.autoClaimStorage*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### mapping
    /// list of disk mappings to be removed from VSAN usage.
    ///
    /// ### maintenance_spec
    /// Any additional actions to move data out of the disk
    /// before removing it. See *HostMaintenanceSpec*.
    /// If unspecified, there is no action taken to move
    /// data from the disk.
    ///
    /// ### timeout
    /// Time to wait for the task to complete in seconds.
    /// If the value is less than or equal to zero, there
    /// is no timeout. The operation fails with a Timedout
    /// exception if it timed out.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn remove_disk_mapping_task(&self, mapping: &[VsanHostDiskMapping], maintenance_spec: Option<&HostMaintenanceSpec>, timeout: Option<i32>) -> Result<ManagedObjectReference> {
        let input = RemoveDiskMappingRequestType {mapping, maintenance_spec, timeout, };
        let path = format!("/HostVsanSystem/{moId}/RemoveDiskMapping_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Unmount the mounted *VsanHostDiskMapping*.
    /// 
    /// An unmounted volume cannot
    /// be used for any VSAN operations. In contrast to *HostVsanSystem.RemoveDiskMapping_Task*,
    /// this operation does not destroy or alter VSAN data on the disks.
    /// *HostVsanSystem.AddDisks_Task* and *HostVsanSystem.InitializeDisks_Task* can be used to
    /// re-mount the diskMapping.
    /// 
    /// In case of shared-SAS, where diskMappings are visible to more than one VSAN hosts,
    /// Users may use this API to manually unmount and re-mount diskMappings.
    /// 
    /// Upon successful completion of the returned *Task*, its
    /// *TaskInfo.result* field will be populated with
    /// *VsanHostDiskMapResult*\[\]. If any errors are encountered,
    /// the returned field will instead contain populated error information.
    /// 
    /// See also *HostVsanSystem.RemoveDiskMapping_Task*, *HostVsanSystem.AddDisks_Task*, *HostVsanSystem.InitializeDisks_Task*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### mapping
    /// -
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: If the *VsanHostDiskMapping* is already unmounted.
    /// 
    /// ***VsanFault***: if operation fails with VSAN-specific error.
    pub async fn unmount_disk_mapping_task(&self, mapping: &[VsanHostDiskMapping]) -> Result<ManagedObjectReference> {
        let input = UnmountDiskMappingRequestType {mapping, };
        let path = format!("/HostVsanSystem/{moId}/UnmountDiskMapping_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Update the VSAN service on this host according to the given host
    /// configuration specification.
    /// 
    /// Enabling and disabling the VSAN service can be achieved by using
    /// the *VsanHostConfigInfo.enabled* flag.
    /// Host storage settings can be specified through use of
    /// *VsanHostConfigInfo.storageInfo*. If this value is omitted,
    /// changes will not be made to the existing storage configuration.
    /// Host cluster settings can be specified through use of
    /// *VsanHostConfigInfo.clusterInfo*. If this value is omitted,
    /// changes will not be made to the existing cluster configuration.
    /// Host network settings can be specified through use of
    /// *VsanHostConfigInfo.networkInfo*. If this value is omitted,
    /// changes will not be made to the existing network configuration.
    /// 
    /// See also *VsanHostConfigInfo*, *VsanHostConfigInfo.storageInfo*, *VsanHostConfigInfo.clusterInfo*, *VsanHostConfigInfo.networkInfo*, *HostVsanSystem.QueryDisksForVsan*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// host configuration settings to use for the VSAN service.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn update_vsan_task(&self, config: &VsanHostConfigInfo) -> Result<ManagedObjectReference> {
        let input = UpdateVsanRequestType {config, };
        let path = format!("/HostVsanSystem/{moId}/UpdateVsan_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// The current VSAN service configuration information for this host.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn config(&self) -> Result<VsanHostConfigInfo> {
        let path = format!("/HostVsanSystem/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddDisksRequestType<'a> {
    disk: &'a [HostScsiDisk],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EvacuateVsanNodeRequestType<'a> {
    #[serde(rename = "maintenanceSpec")]
    maintenance_spec: &'a HostMaintenanceSpec,
    timeout: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct InitializeDisksRequestType<'a> {
    mapping: &'a [VsanHostDiskMapping],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDisksForVsanRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canonicalName")]
    canonical_name: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveDiskRequestType<'a> {
    disk: &'a [HostScsiDisk],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maintenanceSpec")]
    maintenance_spec: Option<&'a HostMaintenanceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timeout: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveDiskMappingRequestType<'a> {
    mapping: &'a [VsanHostDiskMapping],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maintenanceSpec")]
    maintenance_spec: Option<&'a HostMaintenanceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timeout: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnmountDiskMappingRequestType<'a> {
    mapping: &'a [VsanHostDiskMapping],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateVsanRequestType<'a> {
    config: &'a VsanHostConfigInfo,
}
