use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This is the interface for managing the lifecycle of volumes that are consumed
/// by containers or pods, in case of Kubernetes.
/// 
/// This managed interface can be
/// accessed through MOID of cns-volume-manager, through vSAN service in vCenter.
///   
/// Lifecycle of a container volume includes creation, update, query, attach,
/// detach and delete operations. This interface and its related classes are the entry
/// point for Cloud Native Storage (abbreviated to CNS) service.
///   
/// These requests could come from different container orchestrator clusters running
/// on same vSphere as associated with this VolumeManager. VolumeManager is not
/// aware of presence and topology of container orchestrator clusters, except
/// for the weak association via *CnsContainerCluster*. This is a weak
/// association because it's client's responsibility to provide unique identity
/// for this container orchestrator cluster. VolumeManager will not impose any
/// uniqueness verification on cluster identification.
///   
/// Provisioning APIs of this interface return vim.Task which is vCenter Task
/// object to track the progress of operation. In case of either partial or
/// complete success, the state of the task would be set to success. In case
/// of complete failure of the task when the individual specs couldn't be scheduled,
/// the task status would be set to error. The corresponding fault, if any, will be
/// set in the fault field. For a successfully scheduled task, result of this
/// operation will be a list of *CnsVolumeOperationResult* instances.
/// The client needs to go through the result and check the successful and failed
/// instances.
///   
/// The Task returned by provisioning APIs is a vim.Task object. Client needs to
/// connect to vim endpoint on vCenter using the latest VSAN VMODL version
/// (not latest VIM version) to monitor task status. After the task is complete,
/// clients can refer to *CnsVolumeOperationResult* set as result field in
/// task's *TaskInfo* field.
///   
/// Please refer to the required privileges in the individual API documentation and ignore
/// the **Required Privileges** section which is not used.
#[derive(Clone)]
pub struct CnsVolumeManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl CnsVolumeManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Attaches volumes(block volumes only) to specified VM instances, to make
    /// volumes ready for mount and consumption by respective containers.
    /// 
    ///   
    /// For each volume in input, this API will attach block backing for this volume
    /// to the VirtualMachine specified in input, via one of the available slots on
    /// SCSI controller. This API will transparently add new SCSI controller to the
    /// VirtualMachine, if needed.
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Datastore.FileManagement on datastores specified in input, required for
    ///   block volume only
    /// - VirtualMachine.Config.AddExistingDisk on VM specified in the input
    /// - VirtualMachine.Config.AddRemoveDevice on VM specified in the input
    ///   
    /// Faults that can be set in individual result entry, corresponding to each
    /// VolumeAttachDetachSpec instance in input:
    /// - vmodl.fault.InvalidArgument set in case of invalid input arguments, like
    ///   empty strings, invalid formats, invalid
    ///   combination of inputs.
    /// - vmodl.fault.ManagedObjectNotFound set in case of the VM can not be
    ///   found.
    /// - vim.fault.NotFound set in case of the volume can not be
    ///   found.
    /// - vim.fault.ResourceInUse set when volume has been attached to a VM
    ///   and is in use, client needs to first detach
    ///   the volume from that VM and then retry this
    ///   operation.
    /// - vim.fault.CnsMissingControllerFault set if the virtual machine has no
    ///   available controller when controllerKey is
    ///   unset, it is inherited from
    ///   vim.fault.CnsFault.
    /// - vim.fault.CnsFault set in case of any other failure
    ///   scenario.
    ///
    /// ## Parameters:
    ///
    /// ### attach_specs
    /// Specification for attach operation
    ///
    /// ## Returns:
    ///
    /// *Task* to track the progress and overall state of this
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: This API supports input size of 1 only. If
    /// more or less than one entries are passed as
    /// input, this exception will be thrown and
    /// operation will fail. This fault will occur in
    /// cases where the volume ID is empty, VM
    /// is not present, volume type is FILE etc.
    /// 
    /// ***ManagedObjectNotFound***: if the VM can not be found.
    /// 
    /// ***NotFound***: if the volume can not be found.
    /// 
    /// ***ResourceInUse***: if the volume has been attached a VM and is in
    /// use, client needs to first detach the volume
    /// from that VM and then retry this operation.
    /// 
    /// ***CnsFault***: Thrown for all other failure scenario.
    pub async fn cns_attach_volume(&self, attach_specs: &[crate::types::structs::CnsVolumeAttachDetachSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsAttachVolumeRequestType {attach_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsAttachVolume", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Modify the ACL configurations for existing volumes.
    /// 
    /// Multiple requests
    /// configuring the same volume at the same time will be serialized in CNS.
    ///   
    /// Note that this API is currently supported for file volumes only.
    ///   
    /// Following privileges will be required on file volumes, to perform
    /// this operation:
    /// - Host.Config.Storage on vSAN file service enabled vSAN cluster,
    ///   required for file volume only
    ///   
    /// Faults that can be set in individual result entry, corresponding to each
    /// *CnsVolumeACLConfigureSpec* instance in input:
    /// - vmodl.fault.InvalidArgument Set if the input spec has invalid field.
    /// - vim.fault.NotFound Set in case of the volume can not be found.
    /// - vim.fault.CnsFault Set in case of any other failure scenarios.
    ///
    /// ## Parameters:
    ///
    /// ### acl_config_specs
    /// Specifications for volumes ACL configuration.
    ///
    /// ## Returns:
    ///
    /// *Task* to track the progress and overall state of this
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: Thrown if: 1) two or more ACLConfigSpec instances
    /// are passed; 2) the volume ID is empty; 3) file
    /// service reports the invalid inputs.
    /// 
    /// ***NotFound***: Thrown if the volume or cluster can not be found
    /// by CNS.
    /// 
    /// ***CnsFault***: Thrown for any other authorization failure scenrios.
    pub async fn cns_configure_volume_ac_ls(&self, acl_config_specs: &[crate::types::structs::CnsVolumeAclConfigureSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsConfigureVolumeAcLsRequestType {acl_config_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsConfigureVolumeACLs", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates container volume with given specifications.
    /// 
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation. For dynamic provisioning, the datastores that does not
    /// have the necessary privileges will be ignored and other datastores that
    /// have the necessary privileges will be considered for volume placement.
    /// - Datastore.FileManagement on datastores specified in input, required for
    ///   block volume only
    /// - Host.Config.Storage on vSAN file service enabled vSAN cluster,
    ///   required for file volume only
    /// - StorageProfile.View on RootFolder to access storage policy specified in input
    ///   
    /// Faults that can be set in individual result entry, corresponding to each
    /// VolumeCreateSpec instance in input:
    /// - vmodl.fault.InvalidArgument set in case of invalid input arguments,
    ///   invalid formats, invalid combination of
    ///   inputs.
    /// - vim.fault.NotFound set in case the existing disk that should
    ///   be used to back the container volume cannot be found.
    /// - vim.fault.CnsFault set in case of any other failure scenario.
    /// - vim.fault.CnsVolumeAlreadyExistsFault
    ///   set in case volume already exists on a different datastore.
    ///
    /// ## Parameters:
    ///
    /// ### create_specs
    /// Specifications for volumes to be created.
    ///
    /// ## Returns:
    ///
    /// *Task* to track the progress and overall state of this
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: For block volume, if the input spec is invalid like
    /// createSpecs size is not equal to 1, backing disk ID
    /// in backing object details is empty and datastores is
    /// empty, volume metadata in input spec is invalid,
    /// backing disk ID in backing object details is not
    /// present, datastore is invalid, entityMetadata
    /// containing duplicate entity types, profile size
    /// is not equal to 1 etc.
    ///   
    /// For file volume, if the input spec is invalid like
    /// createSpecs size is not equal to 1, volume metadata
    /// in input spec is invalid, backing disk ID in backing
    /// object details is not present, entityMetadata
    /// containing duplicate entity types, profile size
    /// is not equal to 1 etc.
    /// 
    /// ***NotFound***: if the volume can not be found.
    /// 
    /// ***CnsFault***: Thrown for all other failure scenario.
    pub async fn cns_create_volume(&self, create_specs: &[crate::types::structs::CnsVolumeCreateSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsCreateVolumeRequestType {create_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsCreateVolume", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Create snapshots of given volumes
    ///   
    /// A volume with snapshot created on it, is considered in use
    /// and cannot be deleted.
    /// 
    /// Create snapshot operation should be called by providing at least one SnapshotCreateSpec.
    /// If an array of empty spec is passed, the operation will fail.
    /// Return a task that tracks the status and result of snapshot
    /// operation.
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Datastore.FileManagement on all involved Datastores
    ///
    /// ## Parameters:
    ///
    /// ### snapshot_specs
    /// -
    ///
    /// ## Returns:
    ///
    /// *Task* to track the progress and result
    /// of this operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: This API supports input size of 1 only. If
    /// more or less than one entries are passed as
    /// input, this exception will be thrown and
    /// operation will fail.
    /// This exception will be thrown when invalid
    /// format for VolumeId *CnsVolumeId.id*
    /// is passed, or volume IDs are empty etc.
    /// 
    /// ***NotFound***: if the volume can not be found.
    /// 
    /// ***CnsSnapshotCreatedFault***: If the snapshot is created but CNS
    /// failed to persist it into DB. Clean-up using
    /// lower layer api is advised
    /// 
    /// ***CnsFault***: Thrown for all other failure scenario.
    pub async fn cns_create_snapshots(&self, snapshot_specs: &[crate::types::structs::CnsSnapshotCreateSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsCreateSnapshotsRequestType {snapshot_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsCreateSnapshots", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deletes given container volumes.
    /// 
    ///   
    /// Multiple requests for a volume, when deletion is already in progress, will
    /// not return any fault. This operation will make the volume unavailable for
    /// any attach, update and query operation.
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Datastore.FileManagement on all involved Datastores, required for block
    ///   volume only
    /// - Host.Config.Storage on vSAN file service enabled vSAN cluster,
    ///   required for file volume only
    ///   
    /// Faults that can be set in individual result entry, corresponding to each
    /// volume ID instance in input:
    /// - vmodl.fault.InvalidArgument set in case of empty volume ID
    /// - vim.fault.NotFound set in case of the volume can not be found.
    /// - vim.fault.ResourceInUse set when volume has been attached to a VM
    ///   and is in use, client needs to first detach
    ///   the volume from that VM and then retry this
    ///   operation.
    /// - vim.fault.CnsFault set in case of any other failure scenario.
    ///
    /// ## Parameters:
    ///
    /// ### volume_ids
    /// List of *CnsVolumeId* for the volumes to be
    /// deleted.
    ///
    /// ### delete_disk
    /// Disk is the backing object for each container volume
    /// specified in volumeIds list. If set to true, the backing
    /// objects specified in volumeIds list will be deleted. If
    /// set to false, the backing objects specified in volumeIds
    /// list will not be deleted but will no longer be a
    /// container volume.
    ///
    /// ## Returns:
    ///
    /// *Task* to track the progress and overall state of this
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: This API supports input size of 1 only. If
    /// more or less than one entries are passed as
    /// input, this exception will be thrown and
    /// operation will fail.  
    /// This exception will be thrown when invalid
    /// format for VolumeId *CnsVolumeId.id*
    /// is passed, or volume IDs are empty.
    /// 
    /// ***NotFound***: if the volume can not be found.
    /// 
    /// ***ResourceInUse***: if the volume has been attached to a VM and is in
    /// use, client needs to first detach the volume
    /// from that VM and then retry this operation.
    /// 
    /// ***CnsFault***: Thrown for all other failure scenario.
    pub async fn cns_delete_volume(&self, volume_ids: &[crate::types::structs::CnsVolumeId], delete_disk: bool) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsDeleteVolumeRequestType {volume_ids, delete_disk, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsDeleteVolume", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Delete snapshots with given volumeIds and snapshotIds.
    /// 
    ///   
    /// Delete snapshot operation should be called by providing at least one SnapshotDeleteSpec
    /// If an array of empty spec is passed, the operation will fail.
    /// Return a task that tracks the status and result of delete operation
    /// per given volume.
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Datastore.FileManagement on all involved Datastores
    ///
    /// ## Parameters:
    ///
    /// ### snapshot_delete_specs
    /// -
    ///
    /// ## Returns:
    ///
    /// *Task* to track the progress and result
    /// of this operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: This API supports input size of 1 only. If
    /// more or less than one entries are passed as
    /// input, this exception will be thrown and
    /// operation will fail.  
    /// This exception will be thrown when invalid
    /// format for VolumeId *CnsVolumeId.id*
    /// is passed, or volume IDs is empty etc.
    /// 
    /// ***NotFound***: if the volume or snapshot can not be found.
    /// 
    /// ***CnsFault***: Thrown for all other failure scenario.
    pub async fn cns_delete_snapshots(&self, snapshot_delete_specs: &[crate::types::structs::CnsSnapshotDeleteSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsDeleteSnapshotsRequestType {snapshot_delete_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsDeleteSnapshots", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Detaches volumes(block volumes only) and makes those volumes unavailable
    /// for consumption.
    /// 
    /// If a volume is already detached, this operation will pass,
    /// without any failure for that volume.
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Datastore.FileManagement on datastores specified in input, required for
    ///   block volume only
    /// - VirtualMachine.Config.RemoveDisk on VM specified in the input
    ///   
    /// Faults that can be set in individual result entry, corresponding to each
    /// VolumeAttachDetachSpec instance in input:
    /// - vmodl.fault.InvalidArgument set in case of invalid input arguments, like
    ///   empty strings, invalid formats, invalid
    ///   combination of inputs, such as the volume is
    ///   not attached to any VM, or the volume is not
    ///   attached to the VM specified, volume type if
    ///   FILE etc.
    /// - vmodl.fault.ManagedObjectNotFound set in case of the VM can not be
    ///   found.
    /// - vim.fault.NotFound set in case of the volume can not be found.
    /// - vim.fault.CnsFault set in case of any other failure scenario.
    ///
    /// ## Parameters:
    ///
    /// ### detach_specs
    /// Specification for detach operation
    ///
    /// ## Returns:
    ///
    /// *Task* to track the progress and overall state of this
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: This API supports input size of 1 only. If
    /// more or less than one entries are passed as
    /// input, this exception will be thrown and
    /// operation will fail. This fault will occur
    /// when the volume is not attached to any VM or
    /// the volume is not attached to the VM specified
    /// or the volume ID is empty etc.
    /// 
    /// ***ManagedObjectNotFound***: if the VM can not be found.
    /// 
    /// ***NotFound***: if the volume can not be found.
    /// 
    /// ***CnsFault***: Thrown for all other failure scenario.
    pub async fn cns_detach_volume(&self, detach_specs: &[crate::types::structs::CnsVolumeAttachDetachSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsDetachVolumeRequestType {detach_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsDetachVolume", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Extend the capacity for the container volumes.
    /// 
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Datastore.FileManagement on datastore where the volume is placed,
    ///   required for block volume only
    /// - Host.Config.Storage on vSAN file service enabled vSAN cluster,
    ///   required for file volume only
    ///   
    /// Faults that can be set in individual result entry, corresponding to each
    /// VolumeExtendSpec instance in input:
    /// - vmodl.fault.InvalidArgument set in case of invalid input arguments,
    ///   like empty strings, invalid formats,
    ///   invalid combination of inputs.
    /// - vim.fault.NotFound set in case of the volume can not be found.
    /// - vim.fault.CnsFault set in case of any other failure scenario.
    ///
    /// ## Parameters:
    ///
    /// ### extend_specs
    /// Specifications for volumes to be extended.
    ///
    /// ## Returns:
    ///
    /// *Task* to track the progress and overall state of this
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: This API supports input size of 1 only. If
    /// more or less than one entries are passed as
    /// input, this exception will be thrown and
    /// operation will fail.
    /// This API requires the new capacity of the
    /// volume specified in ExtendSpec is bigger
    /// than 0, this exception will be thrown and
    /// operation will fail.
    /// 
    /// ***NotFound***: if the volume can not be found.
    /// 
    /// ***CnsFault***: Thrown for all other failure scenario.
    pub async fn cns_extend_volume(&self, extend_specs: &[crate::types::structs::CnsVolumeExtendSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsExtendVolumeRequestType {extend_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsExtendVolume", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Returns container volumes matching criteria set in the filter.
    /// 
    ///   
    /// This API will return partial results even when some of the volume IDs
    /// are invalid or non-existent. Invalid inputs, like empty volume ID or
    /// unknown volume ID, will be excluded from the results. For valid inputs,
    /// if the output doesn't contain information for that volume that would
    /// mean that CNS is not aware of the existence of that volume. Note that
    /// there could be duplicate volumes or missing volumes across multiple
    /// pages returned by this API when there are parallel volume provisioning
    /// operations like create, delete are in
    /// progress.
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Cns.Searchable on RootFolder to search over all container volumes
    ///   
    /// ***Required privileges:*** Cns.Searchable
    ///
    /// ## Parameters:
    ///
    /// ### filter
    /// All container volumes matching the criteria set in the filter
    /// will be returned. A maximum of 1000 volume ids can be provided.
    /// See *CnsQueryFilter*
    ///
    /// ### selection
    /// Selection spec for the query entities to return.
    /// This is an optional parameter. All volume fields would be returned
    /// if the parameter is not specified. See *CnsQuerySelection*
    ///
    /// ## Returns:
    ///
    /// array of *CnsVolume* matching the input criteria
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: Thrown in case of invalid input arguments, like empty
    /// strings, invalid formats, invalid combination of
    /// inputs
    /// 
    /// ***CnsFault***: Thrown for all other failure scenarios
    pub async fn cns_query_volume(&self, filter: &dyn crate::types::traits::CnsQueryFilterTrait, selection: Option<&crate::types::structs::CnsQuerySelection>) -> Result<crate::types::structs::CnsQueryResult> {
        let input = CnsQueryVolumeRequestType {filter, selection, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsQueryVolume", Some(&input)).await?;
        let result: crate::types::structs::CnsQueryResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Returns container volumes matching criteria set in the filter.
    /// 
    ///   
    /// This API will return partial results even when some of the volume IDs
    /// are invalid or non-existent. Invalid inputs, like empty volume ID or
    /// unknown volume ID, will be excluded from the results. For valid inputs,
    /// if the output doesn't contain information for that volume that would
    /// mean that CNS is not aware of the existence of that volume. Note that
    /// there could be duplicate volumes or missing volumes across multiple
    /// pages returned by this API when there are parallel volume provisioning
    /// operations like create, delete are in progress.
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Cns.Searchable on RootFolder to search over all container volumes
    ///   
    /// ***Required privileges:*** Cns.Searchable
    ///
    /// ## Parameters:
    ///
    /// ### filter
    /// All container volumes matching the criteria set in the filter
    /// will be returned. A maximum of 1000 volume ids can be provided.
    /// See *CnsQueryFilter*
    ///
    /// ### selection
    /// Selection spec for the query entities to return.
    /// This is an optional parameter. All volume fields would be returned
    /// if the parameter is not specified. See *CnsQuerySelection*
    ///
    /// ## Returns:
    ///
    /// *Task* to track the progress and result of this operation.
    /// For result type in task, see *CnsAsyncQueryResult*
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: Thrown in case of invalid input arguments, like empty
    /// strings, invalid formats, invalid combination of
    /// inputs
    /// 
    /// ***CnsFault***: Thrown for all other failure scenarios
    pub async fn cns_query_async(&self, filter: &dyn crate::types::traits::CnsQueryFilterTrait, selection: Option<&crate::types::structs::CnsQuerySelection>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsQueryAsyncRequestType {filter, selection, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsQueryAsync", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Reconfigures the volume with the storage policy.
    /// 
    /// Currently the API is supported for only block volumes and only supports applying
    /// the currently associated policy.
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Datastore.FileManagement on datastores specified in input, required for
    ///   block volume only
    /// - StoragePolicy.View on policy in the spec
    ///
    /// ## Parameters:
    ///
    /// ### volume_policy_reconfig_specs
    /// An array of spec ,
    /// currently only array of size 1 is supported.
    ///
    /// ## Returns:
    ///
    /// *Task* vCenter Task to track the progress and overall state
    /// of this operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if:
    /// - Input size is not equal to 1
    /// - Volume id in input spec is empty.
    /// - Policy string is empty.
    ///   
    /// ***NotFound***: if the volume can not be found.
    /// 
    /// ***CnsFault***: Thrown for all other failure scenario.
    pub async fn cns_reconfig_volume_policy(&self, volume_policy_reconfig_specs: Option<&[crate::types::structs::CnsVolumePolicyReconfigSpec]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsReconfigVolumePolicyRequestType {volume_policy_reconfig_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsReconfigVolumePolicy", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Relocate container volume from the current source datastore to another
    /// destination datastore.
    /// 
    /// Currently, it supports for a single block volume only.
    ///   
    /// This API comes with the following limitations:
    /// - Any CNS control operations like attach, detach, update, expand, etc on a
    ///   volume being relocated or any other volume attached
    ///   to the same VM as the volume being relocated will
    ///   fail with CnsFault, see *CnsFault*.
    /// - VMC is not fully supported for this API.
    /// - Please see <a href="https://kb.vmware.com/s/article/90607">guidelines and
    ///   limitations of CNS relocate on vSphere</a>.
    ///   
    ///   
    /// If an array of empty spec is passed or the size of the input spec is not
    /// equal to 1, the operation will fail.
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation.
    /// - Datastore.FileManagement on both the source datastore, and the
    ///   destination datastore specified in input,
    ///   which is required for block volume only
    /// - Resource.ColdMigrate on the virtual machine that volume is attached to,
    ///   which is required for attached block volume only.
    /// - Datastore.AllocateSpace on the target datastore
    /// - Resource.HotMigrate on the virtual machine that volume is attached to,
    ///   if the vm is powered on, which is required for
    ///   attached block volume only
    ///   
    /// Faults that can be set in individual result entry, corresponding to each
    /// VolumeRelocateSpec instance in input:
    /// - vmodl.fault.InvalidArgument set in case of invalid input arguments,
    ///   invalid formats.
    /// - vim.fault.NotFound set in case the volume or datastore in the spec
    ///   can not be found.
    /// - vim.fault.CnsFault set in case of any other failure scenarios.
    ///
    /// ## Parameters:
    ///
    /// ### relocate_specs
    /// Specifications for volumes to be relocated. Block volume
    /// relocation should use the child class spec, see
    /// *CnsBlockVolumeRelocateSpec*.
    ///
    /// ## Returns:
    ///
    /// *Task* to track the progress and overall state of this
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: Thrown in case volume is already migrated to the
    /// specified destination datastore.
    /// 
    /// ***InvalidArgument***: Thrown in case of invalid input arguments, such as
    /// invalid volume or datastore format, or empty volume
    /// IDs
    /// 
    /// ***NotFound***: if the volume or datastore can not be found.
    /// 
    /// ***CnsFault***: Thrown for all other failure scenario.
    pub async fn cns_relocate_volume(&self, relocate_specs: &[Box<dyn crate::types::traits::CnsVolumeRelocateSpecTrait>]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsRelocateVolumeRequestType {relocate_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsRelocateVolume", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Initiates a task to synchronize one or more volumes based on the provided specifications.
    /// 
    /// This method allows for syncing volume properties for a single volume only.
    /// The operation is asynchronous and returns a \`Task\` object that can be used
    /// to monitor the progress and outcome of the synchronization. This API supported only
    /// for block volume.
    ///
    /// ## Parameters:
    ///
    /// ### sync_specs
    /// Specification for volume to be synchronized.
    /// At most one specification is supported for this operation.
    ///
    /// ## Returns:
    ///
    /// \`vim.Task\` vCenter Task to track the progress and overall state
    /// of this operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if:
    /// - The volume ID in an input spec is empty or malformed
    /// - This API supports input size of 1 only. If
    ///   more or less than one entries are passed as
    ///   input, this exception will be thrown.
    ///   
    /// ***NotFound***: if a volume specified in \`syncSpecs\` does not exist on the system.
    /// 
    /// ***CnsFault***: Thrown for all other CNS-related failure scenarios, such as issues
    /// with the CNS infrastructure or backend storage.
    pub async fn cns_sync_volume(&self, sync_specs: Option<&[crate::types::structs::CnsSyncVolumeSpec]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsSyncVolumeRequestType {sync_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsSyncVolume", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Initiates an asynchronous operation to unregister volume.
    /// 
    ///   
    /// Unregistration removes the CNS metadata associated with the specified volumes,
    /// it also optionally converts to a targer volume type as specified in the
    /// *CnsUnregisterVolumeSpec.targetVolumeType*. This API is only
    /// supported for the block volume.
    ///   
    /// This is an asynchronous operation, it returns a *Task* object
    /// which can be used to monitor the progress and completion status of the
    /// unregistration process.
    ///
    /// ## Parameters:
    ///
    /// ### unregister_spec
    /// An array of *CnsUnregisterVolumeSpec* objects,
    /// each specifying a unique volume or PVC to be unregistered
    /// along with optional parameters for its post-unregistration
    /// state. At most one specification is supported for this operation.
    ///
    /// ## Returns:
    ///
    /// *Task* to track the progress and result of this operation.
    /// The task's result will indicate the success of the unregistration
    /// for all specified volumes.
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Datastore.FileManagement on datastores specified in input, required for
    ///   block volume only
    ///   
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: Thrown if any of the `volumeId`s specified in the
    /// `unregisterSpec` do not correspond to an
    /// existing PVC in the CNS inventory.
    /// 
    /// ***InvalidState***: Thrown if a volume specified is in a state that prevents
    /// unregistration.
    /// 
    /// ***InvalidDatastore***: Thrown if the operation cannot be performed on the datastore.
    /// 
    /// ***TaskInProgress***: Thrown if the virtual storage object is busy.
    /// 
    /// ***InvalidArgument***: if:
    /// - This API supports input size of 1 only. If
    ///   more or less than one entries are passed as
    ///   input, this exception will be thrown.
    /// - This exception will be thrown when invalid
    ///   format for VolumeId *CnsVolumeId.id*
    ///   is passed, or volume IDs are empty.
    /// - This exception will be thrown when volume Id does
    ///   not belong to block volume. 
    ///   
    /// ***CnsFault***: Thrown if a general CNS-specific error occurs during the
    /// unregistration process that is not covered by more
    /// specific faults.
    /// 
    /// ***CnsNotRegisteredFault***: if the volume exists in VC but not registered as CNS
    /// volume.
    pub async fn cns_unregister_volume(&self, unregister_spec: Option<&[crate::types::structs::CnsUnregisterVolumeSpec]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsUnregisterVolumeRequestType {unregister_spec, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsUnregisterVolume", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Updates volume crypto, namely encrypt, deep recrypt, shallow recrypt,
    /// and decrypt for the container block volumes and all the disks in the chain.
    /// 
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Datastore.FileManagement on all involved datastores, required for
    ///   block volumes only
    /// - Cryptographer.Encrypt on all involved datastores and on the virtual machines volumes
    ///   are attached to, required for block volumes update encrypt
    /// - Cryptographer.Recrypt on all involved datastores and on the virtual machines volumes
    ///   are attached to, required for block volumes update deep or shallow recrypt
    /// - Cryptographer.Decrypt on all involved datastores and on the virtual machines volumes
    ///   are attached to, required for block volumes update decrypt
    ///   
    /// Faults that can be set in individual result entry, corresponding to each
    /// VolumeCryptoUpdateSpec instance in input:
    /// - vmodl.fault.InvalidArgument set in case of invalid but non-empty
    ///   volume id.
    /// - vim.fault.NotFound set in case of the volume can not be found.
    /// - vim.fault.CnsFault set in case of any other failure scenario.
    ///
    /// ## Parameters:
    ///
    /// ### update_specs
    /// Specifications for volumes to be crypted.
    ///
    /// ## Returns:
    ///
    /// *Task* vCenter Task to track the progress and overall state
    /// of this operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if:
    /// - Input size is not equal to 1
    /// - Volume id in input spec is empty.
    /// - *VolumeCryptoUpdateSpec.disksCrypto.crypto*
    ///   is unset.
    /// - *VolumeCryptoUpdateSpec.disksCrypto.parent*
    ///   is set.
    ///   
    /// ***NotFound***: if the volume can not be found.
    /// 
    /// ***CnsNotRegisteredFault***: if the volume exists in VC but not registered as CNS
    /// volume.
    /// 
    /// ***CnsFault***: Thrown for all other failure scenario.
    pub async fn cns_update_volume_crypto(&self, update_specs: Option<&[crate::types::structs::CnsVolumeCryptoUpdateSpec]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsUpdateVolumeCryptoRequestType {update_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsUpdateVolumeCrypto", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Updates volume metadata, namely labels and container cluster information for the
    /// container volumes.
    /// 
    ///   
    /// Following privileges will be required on specified entities, to perform
    /// this operation:
    /// - Datastore.FileManagement on datastores specified in input, required for
    ///   block volume only
    /// - Host.Config.Storage on vSAN file service enabled vSAN cluster, required
    ///   for file volume only
    ///   
    /// Faults that can be set in individual result entry, corresponding to each
    /// VolumeMetadataUpdateSpec instance in input:
    /// - vmodl.fault.InvalidArgument set in case of invalid but non-empty
    ///   volume id.
    /// - vim.fault.NotFound set in case of the volume can not be found.
    /// - vim.fault.CnsFault set in case of any other failure scenario.
    ///
    /// ## Parameters:
    ///
    /// ### update_specs
    /// Specifications for volumes to be updated.
    ///
    /// ## Returns:
    ///
    /// *Task* vCenter Task to track the progress and overall state
    /// of this operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if:
    /// - Input size is not equal to 1
    /// - Volume id in input spec is empty.
    /// - Volume metadata in input spec has empty
    ///   cluster info.
    /// - Entity objects in volume metadata has empty or
    ///   invalid attributes.
    /// - EntityMetadata in volume metadata contains
    ///   duplicate entity types.
    ///   
    /// ***NotFound***: if the volume can not be found.
    /// 
    /// ***CnsNotRegisteredFault***: if the volume exists in VC but not registered as CNS
    /// volume.
    /// 
    /// ***CnsFault***: Thrown for all other failure scenario.
    pub async fn cns_update_volume_metadata(&self, update_specs: &[crate::types::structs::CnsVolumeMetadataUpdateSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CnsUpdateVolumeMetadataRequestType {update_specs, };
        let bytes = self.client.invoke("vsan", "CnsVolumeManager", &self.mo_id, "CnsUpdateVolumeMetadata", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct CnsAttachVolumeRequestType<'a> {
    attach_specs: &'a [crate::types::structs::CnsVolumeAttachDetachSpec],
}

impl<'a> miniserde::Serialize for CnsAttachVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsAttachVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsAttachVolumeRequestTypeSer<'b, 'a> {
    data: &'b CnsAttachVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsAttachVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsAttachVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("attachSpecs"), &self.data.attach_specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CnsConfigureVolumeAcLsRequestType<'a> {
    acl_config_specs: &'a [crate::types::structs::CnsVolumeAclConfigureSpec],
}

impl<'a> miniserde::Serialize for CnsConfigureVolumeAcLsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsConfigureVolumeAcLsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsConfigureVolumeAcLsRequestTypeSer<'b, 'a> {
    data: &'b CnsConfigureVolumeAcLsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsConfigureVolumeAcLsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsConfigureVolumeACLsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("ACLConfigSpecs"), &self.data.acl_config_specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CnsCreateVolumeRequestType<'a> {
    create_specs: &'a [crate::types::structs::CnsVolumeCreateSpec],
}

impl<'a> miniserde::Serialize for CnsCreateVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsCreateVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsCreateVolumeRequestTypeSer<'b, 'a> {
    data: &'b CnsCreateVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsCreateVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsCreateVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("createSpecs"), &self.data.create_specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CnsCreateSnapshotsRequestType<'a> {
    snapshot_specs: &'a [crate::types::structs::CnsSnapshotCreateSpec],
}

impl<'a> miniserde::Serialize for CnsCreateSnapshotsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsCreateSnapshotsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsCreateSnapshotsRequestTypeSer<'b, 'a> {
    data: &'b CnsCreateSnapshotsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsCreateSnapshotsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsCreateSnapshotsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("snapshotSpecs"), &self.data.snapshot_specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CnsDeleteVolumeRequestType<'a> {
    volume_ids: &'a [crate::types::structs::CnsVolumeId],
    delete_disk: bool,
}

impl<'a> miniserde::Serialize for CnsDeleteVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsDeleteVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsDeleteVolumeRequestTypeSer<'b, 'a> {
    data: &'b CnsDeleteVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsDeleteVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsDeleteVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("volumeIds"), &self.data.volume_ids as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("deleteDisk"), &self.data.delete_disk as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CnsDeleteSnapshotsRequestType<'a> {
    snapshot_delete_specs: &'a [crate::types::structs::CnsSnapshotDeleteSpec],
}

impl<'a> miniserde::Serialize for CnsDeleteSnapshotsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsDeleteSnapshotsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsDeleteSnapshotsRequestTypeSer<'b, 'a> {
    data: &'b CnsDeleteSnapshotsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsDeleteSnapshotsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsDeleteSnapshotsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("snapshotDeleteSpecs"), &self.data.snapshot_delete_specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CnsDetachVolumeRequestType<'a> {
    detach_specs: &'a [crate::types::structs::CnsVolumeAttachDetachSpec],
}

impl<'a> miniserde::Serialize for CnsDetachVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsDetachVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsDetachVolumeRequestTypeSer<'b, 'a> {
    data: &'b CnsDetachVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsDetachVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsDetachVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("detachSpecs"), &self.data.detach_specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CnsExtendVolumeRequestType<'a> {
    extend_specs: &'a [crate::types::structs::CnsVolumeExtendSpec],
}

impl<'a> miniserde::Serialize for CnsExtendVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsExtendVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsExtendVolumeRequestTypeSer<'b, 'a> {
    data: &'b CnsExtendVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsExtendVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsExtendVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("extendSpecs"), &self.data.extend_specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CnsQueryVolumeRequestType<'a> {
    filter: &'a dyn crate::types::traits::CnsQueryFilterTrait,
    selection: Option<&'a crate::types::structs::CnsQuerySelection>,
}

impl<'a> miniserde::Serialize for CnsQueryVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsQueryVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsQueryVolumeRequestTypeSer<'b, 'a> {
    data: &'b CnsQueryVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsQueryVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsQueryVolumeRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("filter"), &self.data.filter as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.selection else { continue; };
                    return Some((std::borrow::Cow::Borrowed("selection"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CnsQueryAsyncRequestType<'a> {
    filter: &'a dyn crate::types::traits::CnsQueryFilterTrait,
    selection: Option<&'a crate::types::structs::CnsQuerySelection>,
}

impl<'a> miniserde::Serialize for CnsQueryAsyncRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsQueryAsyncRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsQueryAsyncRequestTypeSer<'b, 'a> {
    data: &'b CnsQueryAsyncRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsQueryAsyncRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsQueryAsyncRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("filter"), &self.data.filter as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.selection else { continue; };
                    return Some((std::borrow::Cow::Borrowed("selection"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CnsReconfigVolumePolicyRequestType<'a> {
    volume_policy_reconfig_specs: Option<&'a [crate::types::structs::CnsVolumePolicyReconfigSpec]>,
}

impl<'a> miniserde::Serialize for CnsReconfigVolumePolicyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsReconfigVolumePolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsReconfigVolumePolicyRequestTypeSer<'b, 'a> {
    data: &'b CnsReconfigVolumePolicyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsReconfigVolumePolicyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsReconfigVolumePolicyRequestType")),
                1 => {
                    let Some(ref val) = self.data.volume_policy_reconfig_specs else { continue; };
                    return Some((std::borrow::Cow::Borrowed("volumePolicyReconfigSpecs"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CnsRelocateVolumeRequestType<'a> {
    relocate_specs: &'a [Box<dyn crate::types::traits::CnsVolumeRelocateSpecTrait>],
}

impl<'a> miniserde::Serialize for CnsRelocateVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsRelocateVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsRelocateVolumeRequestTypeSer<'b, 'a> {
    data: &'b CnsRelocateVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsRelocateVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsRelocateVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("relocateSpecs"), &self.data.relocate_specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CnsSyncVolumeRequestType<'a> {
    sync_specs: Option<&'a [crate::types::structs::CnsSyncVolumeSpec]>,
}

impl<'a> miniserde::Serialize for CnsSyncVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsSyncVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsSyncVolumeRequestTypeSer<'b, 'a> {
    data: &'b CnsSyncVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsSyncVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsSyncVolumeRequestType")),
                1 => {
                    let Some(ref val) = self.data.sync_specs else { continue; };
                    return Some((std::borrow::Cow::Borrowed("syncSpecs"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CnsUnregisterVolumeRequestType<'a> {
    unregister_spec: Option<&'a [crate::types::structs::CnsUnregisterVolumeSpec]>,
}

impl<'a> miniserde::Serialize for CnsUnregisterVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsUnregisterVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsUnregisterVolumeRequestTypeSer<'b, 'a> {
    data: &'b CnsUnregisterVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsUnregisterVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsUnregisterVolumeRequestType")),
                1 => {
                    let Some(ref val) = self.data.unregister_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("unregisterSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CnsUpdateVolumeCryptoRequestType<'a> {
    update_specs: Option<&'a [crate::types::structs::CnsVolumeCryptoUpdateSpec]>,
}

impl<'a> miniserde::Serialize for CnsUpdateVolumeCryptoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsUpdateVolumeCryptoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsUpdateVolumeCryptoRequestTypeSer<'b, 'a> {
    data: &'b CnsUpdateVolumeCryptoRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsUpdateVolumeCryptoRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsUpdateVolumeCryptoRequestType")),
                1 => {
                    let Some(ref val) = self.data.update_specs else { continue; };
                    return Some((std::borrow::Cow::Borrowed("updateSpecs"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CnsUpdateVolumeMetadataRequestType<'a> {
    update_specs: &'a [crate::types::structs::CnsVolumeMetadataUpdateSpec],
}

impl<'a> miniserde::Serialize for CnsUpdateVolumeMetadataRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CnsUpdateVolumeMetadataRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CnsUpdateVolumeMetadataRequestTypeSer<'b, 'a> {
    data: &'b CnsUpdateVolumeMetadataRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CnsUpdateVolumeMetadataRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CnsUpdateVolumeMetadataRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("updateSpecs"), &self.data.update_specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
