use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The VsanSystem managed object type exposes VSAN configuration
/// primitives and serves as a host-level access point for relevant
/// VSAN data objects.
#[derive(Clone)]
pub struct HostVsanSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostVsanSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn add_disks_task(&self, disk: &[crate::types::structs::HostScsiDisk]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = AddDisksRequestType {disk, };
        let path = format!("/vsan/HostVsanSystem/{moId}/AddDisks_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn evacuate_vsan_node_task(&self, maintenance_spec: &crate::types::structs::HostMaintenanceSpec, timeout: i32) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = EvacuateVsanNodeRequestType {maintenance_spec, timeout, };
        let path = format!("/vsan/HostVsanSystem/{moId}/EvacuateVsanNode_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn initialize_disks_task(&self, mapping: &[crate::types::structs::VsanHostDiskMapping]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = InitializeDisksRequestType {mapping, };
        let path = format!("/vsan/HostVsanSystem/{moId}/InitializeDisks_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn query_disks_for_vsan(&self, canonical_name: Option<&[String]>) -> Result<Option<Vec<Box<dyn crate::types::traits::VsanHostDiskResultTrait>>>> {
        let input = QueryDisksForVsanRequestType {canonical_name, };
        let path = format!("/vsan/HostVsanSystem/{moId}/QueryDisksForVsan", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::VsanHostDiskResultTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Queries this host's current runtime status for the VSAN service.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// a populated *VsanHostClusterStatus* entry
    pub async fn query_host_status(&self) -> Result<crate::types::structs::VsanHostClusterStatus> {
        let path = format!("/vsan/HostVsanSystem/{moId}/QueryHostStatus", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VsanHostClusterStatus = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn recommission_vsan_node_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/vsan/HostVsanSystem/{moId}/RecommissionVsanNode_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn remove_disk_task(&self, disk: &[crate::types::structs::HostScsiDisk], maintenance_spec: Option<&crate::types::structs::HostMaintenanceSpec>, timeout: Option<i32>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RemoveDiskRequestType {disk, maintenance_spec, timeout, };
        let path = format!("/vsan/HostVsanSystem/{moId}/RemoveDisk_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn remove_disk_mapping_task(&self, mapping: &[crate::types::structs::VsanHostDiskMapping], maintenance_spec: Option<&crate::types::structs::HostMaintenanceSpec>, timeout: Option<i32>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RemoveDiskMappingRequestType {mapping, maintenance_spec, timeout, };
        let path = format!("/vsan/HostVsanSystem/{moId}/RemoveDiskMapping_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn unmount_disk_mapping_task(&self, mapping: &[crate::types::structs::VsanHostDiskMapping]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UnmountDiskMappingRequestType {mapping, };
        let path = format!("/vsan/HostVsanSystem/{moId}/UnmountDiskMapping_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn update_vsan_task(&self, config: &dyn crate::types::traits::VsanHostConfigInfoTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UpdateVsanRequestType {config, };
        let path = format!("/vsan/HostVsanSystem/{moId}/UpdateVsan_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// The current VSAN service configuration information for this host.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn config(&self) -> Result<Box<dyn crate::types::traits::VsanHostConfigInfoTrait>> {
        let path = format!("/vsan/HostVsanSystem/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Box<dyn crate::types::traits::VsanHostConfigInfoTrait> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct AddDisksRequestType<'a> {
    disk: &'a [crate::types::structs::HostScsiDisk],
}

impl<'a> miniserde::Serialize for AddDisksRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddDisksRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddDisksRequestTypeSer<'b, 'a> {
    data: &'b AddDisksRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for AddDisksRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddDisksRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("disk"), &self.data.disk as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct EvacuateVsanNodeRequestType<'a> {
    maintenance_spec: &'a crate::types::structs::HostMaintenanceSpec,
    timeout: i32,
}

impl<'a> miniserde::Serialize for EvacuateVsanNodeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EvacuateVsanNodeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EvacuateVsanNodeRequestTypeSer<'b, 'a> {
    data: &'b EvacuateVsanNodeRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for EvacuateVsanNodeRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EvacuateVsanNodeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("maintenanceSpec"), &self.data.maintenance_spec as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("timeout"), &self.data.timeout as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct InitializeDisksRequestType<'a> {
    mapping: &'a [crate::types::structs::VsanHostDiskMapping],
}

impl<'a> miniserde::Serialize for InitializeDisksRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(InitializeDisksRequestTypeSer { data: self, seq: 0 }))
    }
}

struct InitializeDisksRequestTypeSer<'b, 'a> {
    data: &'b InitializeDisksRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for InitializeDisksRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"InitializeDisksRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("mapping"), &self.data.mapping as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryDisksForVsanRequestType<'a> {
    canonical_name: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for QueryDisksForVsanRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDisksForVsanRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDisksForVsanRequestTypeSer<'b, 'a> {
    data: &'b QueryDisksForVsanRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for QueryDisksForVsanRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDisksForVsanRequestType")),
                1 => {
                    let Some(ref val) = self.data.canonical_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("canonicalName"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveDiskRequestType<'a> {
    disk: &'a [crate::types::structs::HostScsiDisk],
    maintenance_spec: Option<&'a crate::types::structs::HostMaintenanceSpec>,
    timeout: Option<i32>,
}

impl<'a> miniserde::Serialize for RemoveDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveDiskRequestTypeSer<'b, 'a> {
    data: &'b RemoveDiskRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RemoveDiskRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveDiskRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("disk"), &self.data.disk as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.maintenance_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("maintenanceSpec"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.timeout else { continue; };
                    return Some((std::borrow::Cow::Borrowed("timeout"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveDiskMappingRequestType<'a> {
    mapping: &'a [crate::types::structs::VsanHostDiskMapping],
    maintenance_spec: Option<&'a crate::types::structs::HostMaintenanceSpec>,
    timeout: Option<i32>,
}

impl<'a> miniserde::Serialize for RemoveDiskMappingRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveDiskMappingRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveDiskMappingRequestTypeSer<'b, 'a> {
    data: &'b RemoveDiskMappingRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RemoveDiskMappingRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveDiskMappingRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("mapping"), &self.data.mapping as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.maintenance_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("maintenanceSpec"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.timeout else { continue; };
                    return Some((std::borrow::Cow::Borrowed("timeout"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UnmountDiskMappingRequestType<'a> {
    mapping: &'a [crate::types::structs::VsanHostDiskMapping],
}

impl<'a> miniserde::Serialize for UnmountDiskMappingRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnmountDiskMappingRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnmountDiskMappingRequestTypeSer<'b, 'a> {
    data: &'b UnmountDiskMappingRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for UnmountDiskMappingRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnmountDiskMappingRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("mapping"), &self.data.mapping as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateVsanRequestType<'a> {
    config: &'a dyn crate::types::traits::VsanHostConfigInfoTrait,
}

impl<'a> miniserde::Serialize for UpdateVsanRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateVsanRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateVsanRequestTypeSer<'b, 'a> {
    data: &'b UpdateVsanRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for UpdateVsanRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateVsanRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
