use std::sync::Arc;
use crate::core::client::{Client, Result};
/// Interface to manage virtual storage object on a vCenter.
/// 
/// VStorageObjectManager and SPBM policy support:
/// All of the VStorageObjectManager APIs requiring ESXi host
/// uses "Programatically selected" host to perform the actual operation.
/// If the selected host is of 6.5 version then policy would not be passed
/// down to host. In that case, user operation would succeed but if user checks
/// SPBM Entity Compliance, it will show "Mismatch" / "Non Compliant" as a
/// compliance result.
#[derive(Clone)]
pub struct VslmVStorageObjectManager {
    client: Arc<Client>,
    mo_id: String,
}
impl VslmVStorageObjectManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Attach an existing disk to this virtual machine.
    /// 
    /// A minimum virtual machine version of 'vmx-13' is required for this
    /// operation to succeed. If a compatible VM version is not satisfied,
    /// a *DeviceUnsupportedForVmVersion* fault will be thrown.
    /// 
    /// *VslmSyncFault* will set in the task error if the
    /// the FCD was attached successfully in the backend, however, there
    /// was a failure in syncing the datastore or FCD. The attach disk
    /// operation is not idempotent, implying, any attempt to re-attach
    /// the FCD to the same VM would cause an error. The sync fault can
    /// be ignored safely as Pandora DB does not track FCDs attached
    /// to VMs as of vSphere 7.0.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual disk to be operated. See
    /// *ID*
    ///
    /// ### vm
    /// The virtual machine where the virtual disk is to be attached.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### controller_key
    /// Key of the controller the disk will connect to.
    /// It can be unset if there is only one controller
    /// (SCSI or SATA) with the available slot in the
    /// virtual machine. If there are multiple SCSI or
    /// SATA controllers available, user must specify
    /// the controller; if there is no available
    /// controllers, a *MissingController*
    /// fault will be thrown.
    ///
    /// ### unit_number
    /// The unit number of the attached disk on its controller.
    /// If unset, the next available slot on the specified
    /// controller or the only available controller will be
    /// assigned to the attached disk.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the disk object cannot be found.
    /// 
    /// ***VmConfigFault***: if the virtual machine's configuration is invalid.
    /// 
    /// ***FileFault***: if there is a problem creating or accessing the virtual
    /// machine's files for this operation.
    /// 
    /// ***InvalidState***: if the operation cannot be performed in the current
    /// state of the virtual machine. For example, because the virtual
    /// machine's configuration is not available.
    /// 
    /// ***InvalidDatastore***: If the datastore cannot be found or inaccessible.
    /// 
    /// ***InvalidController***: If the specified controller cannot be found or
    /// the specified unitNumber is already taken, or
    /// the controller has no free slots.
    /// 
    /// ***MissingController***: If the virtual machine has no or more than one
    /// available controllers when controllerKey is
    /// unset.
    /// 
    /// ***DeviceUnsupportedForVmVersion***: If the virtual machine's version is
    /// incompatible for the given device.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_attach_disk_task(&self, id: &crate::types::structs::Id, vm: &crate::types::structs::ManagedObjectReference, controller_key: Option<i32>, unit_number: Option<i32>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmAttachDiskRequestType {id, vm, controller_key, unit_number, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmAttachDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Attach a tag to a virtual storage object.
    /// 
    /// Requires privilege InventoryService.Tagging.AttachTag on root folder
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The identifier(ID) of the virtual storage object.
    ///
    /// ### category
    /// The category to which the tag belongs.
    ///
    /// ### tag
    /// The tag which has to be associated with the virtual storage
    /// object.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the specified category or tag cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_attach_tag_to_v_storage_object(&self, id: &crate::types::structs::Id, category: &str, tag: &str) -> Result<()> {
        let input = VslmAttachTagToVStorageObjectRequestType {id, category, tag, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmAttachTagToVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Clear control flags on VStorageObject.
    /// 
    /// The control flags are defined in
    /// *vslmVStorageObjectControlFlag_enum*.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### control_flags
    /// control flags enum array to be cleared on the
    /// VStorageObject. All control flags not included
    /// in the array remain intact.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on
    /// the datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// The disk may be consumed.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot
    /// be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_clear_v_storage_object_control_flags(&self, id: &crate::types::structs::Id, control_flags: Option<&[String]>) -> Result<()> {
        let input = VslmClearVStorageObjectControlFlagsRequestType {id, control_flags, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmClearVStorageObjectControlFlags", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Clone a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on both source and
    /// destination datastore.
    /// 
    /// *VslmSyncFault* will set in the task error if the
    /// cloneVStorageObject completed successfully but the datastore
    /// synchronization failed.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### spec
    /// The specification for cloning the virtual storage
    /// object.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while cloning the virtual
    /// storage object.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be
    /// found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_clone_v_storage_object_task(&self, id: &crate::types::structs::Id, spec: &crate::types::structs::VslmCloneSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmCloneVStorageObjectRequestType {id, spec, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmCloneVStorageObject_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Create a virtual disk, which is a storage object with
    /// *disk*
    /// as consumption type.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk object is created.
    /// 
    /// *VslmSyncFault* will set in the task error if the
    /// createDisk completed successfully in the backed but the datastore
    /// synchronization or FCD retrieval failed. The sync fault can be ignored
    /// with the risk of Pandora not recognizing the FCD or Pandora DB
    /// missing the FCD, consequently, affecting the return of
    /// *VslmVStorageObjectManager.VslmListVStorageObjectForSpec* and *VslmVStorageObjectManager.VslmRetrieveVStorageObjects*
    /// APIs.
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The specification of the virtual storage object
    /// to be created.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// monitor the operation. The *info.result*
    /// property in the *Task* contains the newly created
    /// *VStorageObject* upon success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs when creating the virtual disk.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_create_disk_task(&self, spec: &crate::types::structs::VslmCreateSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmCreateDiskRequestType {spec, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmCreateDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Creates a new Disk from given snapshot of a VStorageObject.
    /// 
    /// Requires Datastore.FileManagement and Cryptographer.Decrypt privilege.
    /// 
    /// *VslmSyncFault* will set in the task error if the
    /// createDiskFromSnapshot completed successfully but the datastore
    /// synchronization failed.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### snapshot_id
    /// The ID of the snapshot of the virtual storage object.
    ///
    /// ### name
    /// A user friendly name to be associated with the new disk.
    ///
    /// ### profile
    /// SPBM Profile requirement on the new virtual storage object.
    /// If not specified datastore default policy would be
    /// assigned.
    ///
    /// ### crypto
    /// Crypto information of the new disk.
    ///
    /// ### path
    /// Relative location in the specified datastore where disk needs
    /// to be created. If not specified disk gets created at the
    /// defualt VStorageObject location on the specified datastore.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while snapshotting the virtual
    /// storage object.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_create_disk_from_snapshot_task(&self, id: &crate::types::structs::Id, snapshot_id: &crate::types::structs::Id, name: &str, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>, crypto: Option<&dyn crate::types::traits::CryptoSpecTrait>, path: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmCreateDiskFromSnapshotRequestType {id, snapshot_id, name, profile, crypto, path, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmCreateDiskFromSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Creates a snapshot of a given VStorageObject.
    /// 
    /// Requires Datastore.FileManagement privilege.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### description
    /// A short description to be associated with the snapshot.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while snapshotting the virtual
    /// storage object.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_create_snapshot_task(&self, id: &crate::types::structs::Id, description: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmCreateSnapshotRequestType {id, description, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmCreateSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Deletes a given snapshot of a VStorageObject.
    /// 
    /// Requires Datastore.FileManagement privilege.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### snapshot_id
    /// The ID of the snapshot of a virtual storage object.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while snapshotting the virtual
    /// storage object.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_delete_snapshot_task(&self, id: &crate::types::structs::Id, snapshot_id: &crate::types::structs::Id) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmDeleteSnapshotRequestType {id, snapshot_id, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmDeleteSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Delete a virtual storage object and its associated backings.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// *VslmSyncFault* will set in the task error if the
    /// delete of the FCD succeeded in the backend, however, there was a
    /// issue with syncing the datastore. The error can be ignored with
    /// the anticipation of Pandora resolving this automatically and the
    /// risk of Pandora DB having stale FCDs consequently affecting the
    /// return of *VslmVStorageObjectManager.VslmListVStorageObjectForSpec* and
    /// *VslmVStorageObjectManager.VslmRetrieveVStorageObjects* APIs.
    /// Any subsequent operation on the FCD is guranteed to fail.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object to be deleted.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs when deleting the virtual storage
    /// object.
    /// 
    /// ***NotFound***: If the specified virtual storage object cannot be
    /// found.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// The disk may be consumed and cannot be deleted.
    /// 
    /// ***TaskInProgress***: If the virtual storage object is busy.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_delete_v_storage_object_task(&self, id: &crate::types::structs::Id) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmDeleteVStorageObjectRequestType {id, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmDeleteVStorageObject_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Detach a tag from a virtual storage object.
    /// 
    /// Requires privilege InventoryService.Tagging.AttachTag on root folder
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The identifier(ID) of the virtual storage object.
    ///
    /// ### category
    /// The category to which the tag belongs.
    ///
    /// ### tag
    /// The tag which has to be disassociated with the virtual storage
    /// object.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the specified category or tag cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_detach_tag_from_v_storage_object(&self, id: &crate::types::structs::Id, category: &str, tag: &str) -> Result<()> {
        let input = VslmDetachTagFromVStorageObjectRequestType {id, category, tag, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmDetachTagFromVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Expand the capacity of a virtual disk, which is a storage object with
    /// *disk*, to the new
    /// capacity.
    /// 
    /// If new capacity is smaller than current disk capacity, then
    /// operation fails due to invalid capacity. If new capacity is greater
    /// than current disk capacity, then operation proceeds. If new capacity
    /// is equal to current disk ccapcity, then operation succeeds without
    /// any actual extension.
    /// The extended disk region will be the same as the original disk:
    /// \- For a zerothick disk, the extended disk region will be zeroedthick.
    /// \- For an eagerzerothick disk, the extended disk region will be
    /// eagerzeroedthick
    /// \- A thin-provisioned disk will always be extended as a thin-provisioned
    /// disk.
    /// 
    /// *VslmSyncFault* will set in the task error if the
    /// extendDisk completed successfully but the datastore
    /// synchronization failed.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual disk to be extended.
    ///
    /// ### new_capacity_in_mb
    /// The new capacity of the virtual disk in MB.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while extending the virtual disk.
    /// 
    /// ***NotFound***: If the specified virtual storage object cannot be
    /// found.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// The disk may be consumed and cannot be extended.
    /// 
    /// ***TaskInProgress***: If the virtual storage object is busy.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_extend_disk_task(&self, id: &crate::types::structs::Id, new_capacity_in_mb: i64) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmExtendDiskRequestType {id, new_capacity_in_mb, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmExtendDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Inflate a sparse or thin-provisioned virtual disk up to the full size.
    /// 
    /// Additional space allocated to the disk as a result of this operation
    /// will be filled with zeros.
    /// 
    /// Currently inflateDisk API only supports the following combinations:
    /// Valid provisioning type: THIN;
    /// Valid Datastore: VMFS, NFS.
    /// Inflating a disk is not applicable for VVol/VSAN datastore.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual disk to be inflated.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while inflating the virtual disk.
    /// 
    /// ***NotFound***: If the specified virtual storage object cannot be
    /// found.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// The disk may be consumed and cannot be extended.
    /// 
    /// ***TaskInProgress***: If the virtual storage object is busy.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_inflate_disk_task(&self, id: &crate::types::structs::Id) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmInflateDiskRequestType {id, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmInflateDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Lists all tags attached to virtual storage object.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ## Returns:
    ///
    /// The list of Tag-association tuples associated with the
    /// virtual storage object.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the specified category or tag cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_list_tags_attached_to_v_storage_object(&self, id: &crate::types::structs::Id) -> Result<Option<Vec<crate::types::structs::VslmTagEntry>>> {
        let input = VslmListTagsAttachedToVStorageObjectRequestType {id, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmListTagsAttachedToVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Lists all virtual storage objects attached to the tag.
    ///
    /// ## Parameters:
    ///
    /// ### category
    /// The category to which the tag belongs.
    ///
    /// ### tag
    /// The tag to be queried.
    ///
    /// ## Returns:
    ///
    /// The list of IDs of the virtual storage objects.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the specified category or tag cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_list_v_storage_objects_attached_to_tag(&self, category: &str, tag: &str) -> Result<Option<Vec<crate::types::structs::Id>>> {
        let input = VslmListVStorageObjectsAttachedToTagRequestType {category, tag, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmListVStorageObjectsAttachedToTag", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// List virtual storage objects matching all the
    /// *VslmVsoVStorageObjectQuerySpec*.
    /// 
    /// The results are determined
    /// by ANDing the *VslmVsoVStorageObjectQuerySpec* while ORing each
    /// of the values specified as a part of the
    /// *VslmVsoVStorageObjectQuerySpec.queryValue* field. Currently,
    /// only a single value in *VslmVsoVStorageObjectQuerySpec.queryValue*
    /// is supported.
    /// Expect that not all results will be returned. If all results are not
    /// returned, *VslmVsoVStorageObjectQueryResult.allRecordsReturned*
    /// flag will be set to false. Results will be returned in
    /// *VslmVsoVStorageObjectQueryResult.id* order. To query for addtional
    /// objects, "add ID &gt; last ID returned" to the query and call
    /// *VslmVStorageObjectManager.VslmListVStorageObjectForSpec* again.
    ///
    /// ## Parameters:
    ///
    /// ### query
    /// Query defined using array of
    /// *VslmVsoVStorageObjectQuerySpec* objects.
    ///
    /// ### max_result
    /// Maximum number of virtual storage object IDs to return.
    ///
    /// ## Returns:
    ///
    /// *VslmVsoVStorageObjectQueryResult* array containing the
    /// list of IDs of the virtual storage objects sorted in ascending
    /// order and matching the query.
    pub async fn vslm_list_v_storage_object_for_spec(&self, query: Option<&[crate::types::structs::VslmVsoVStorageObjectQuerySpec]>, max_result: i32) -> Result<Option<crate::types::structs::VslmVsoVStorageObjectQueryResult>> {
        let input = VslmListVStorageObjectForSpecRequestType {query, max_result, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmListVStorageObjectForSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Get a list of areas of a virtual disk that have been modified since a
    /// well-defined point in the past.
    /// 
    /// The beginning of the change interval is
    /// identified by "changeId", while the end of the change interval is implied
    /// by the snapshot ID passed in.
    /// 
    /// Note that the result of this function may contain "false positives"
    /// (i.e: flag areas of the disk as modified that are not). However, it is
    /// guaranteed that no changes will be missed.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### snapshot_id
    /// The ID of the snapshot of a virtual storage object for
    /// which changes that have been made since "changeId"
    /// should be computed.
    ///
    /// ### start_offset
    /// Start Offset in bytes at which to start computing
    /// changes. Typically, callers will make multiple calls
    /// to this function, starting with startOffset 0 and then
    /// examine the "length" property in the returned
    /// DiskChangeInfo structure, repeatedly calling
    /// queryChangedDiskAreas until a map for the entire
    /// virtual disk has been obtained.
    ///
    /// ### change_id
    /// Identifier referring to a point in the past that should
    /// be used as the point in time at which to begin including
    /// changes to the disk in the result. A typical use case
    /// would be a backup application obtaining a changeId from
    /// a virtual disk's backing info when performing a backup.
    /// When a subsequent incremental backup is to be performed,
    /// this change Id can be used to obtain a list of changed
    /// areas on disk.
    ///
    /// ## Returns:
    ///
    /// Returns a data structure specifying extents of the virtual disk
    /// that have changed since the thime the changeId string was
    /// obtained.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***NotFound***: If specified virtual storage object or snapshot
    /// cannot be found.
    /// 
    /// ***FileFault***: if the virtual disk files cannot be accessed/queried.
    /// 
    /// ***InvalidState***: if change tracking is not supported for this
    /// particular disk.
    /// 
    /// ***InvalidArgument***: if startOffset is beyond the end of the virtual
    /// disk or changeId is invalid.
    pub async fn vslm_query_changed_disk_areas(&self, id: &crate::types::structs::Id, snapshot_id: &crate::types::structs::Id, start_offset: i64, change_id: &str) -> Result<crate::types::structs::DiskChangeInfo> {
        let input = VslmQueryChangedDiskAreasRequestType {id, snapshot_id, start_offset, change_id, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmQueryChangedDiskAreas", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Gets the synchronization status of the Global Catalog.
    /// 
    /// This API returns an
    /// array of DatastoreSyncStatuses showing the synchronization status for each
    /// datastore currently being tracked by the Global Catalog. This can be used
    /// to see if the Global Catalog search results are fully up-to-date and also to
    /// check if the Global Catalog is making progress on bringing a datastore's
    /// information up to date.
    pub async fn vslm_query_global_catalog_sync_status(&self) -> Result<Option<Vec<crate::types::structs::VslmDatastoreSyncStatus>>> {
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmQueryGlobalCatalogSyncStatus", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
    /// Gets the synchronization state of the Global Catalog for the specified datastore.
    ///
    /// ## Parameters:
    ///
    /// ### datastore_url
    /// URL of the datastore to check synchronization status for
    ///
    /// ## Returns:
    ///
    /// The sync status of the datastore
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the specified datastore cannot be found
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_query_global_catalog_sync_status_for_datastore(&self, datastore_url: &str) -> Result<Option<crate::types::structs::VslmDatastoreSyncStatus>> {
        let input = VslmQueryGlobalCatalogSyncStatusForDatastoreRequestType {datastore_url, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmQueryGlobalCatalogSyncStatusForDatastore", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Reconcile the datastore inventory info of virtual storage objects.
    /// 
    /// Requires Datastore.FileManagement privilege.
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The datastore that needs to be reconciled.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on
    /// the datastore.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_reconcile_datastore_inventory_task(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmReconcileDatastoreInventoryRequestType {datastore, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmReconcileDatastoreInventory_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Promote a virtual disk to a First Class Disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    ///
    /// ## Parameters:
    ///
    /// ### path
    /// URL path to the virtual disk.
    ///
    /// ### name
    /// The descriptive name of the disk object. If
    /// unset the name will be automatically determined
    /// from the path. @see vim.vslm.BaseConfigInfo.name
    ///
    /// ## Returns:
    ///
    /// The registered virtual storage object for the disk.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while registering the virtual disk.
    /// 
    /// ***InvalidDatastore***: If datastore cannot be found or the operation
    /// cannot be performed on the datastore.
    /// 
    /// ***AlreadyExists***: If disk is already registered as a
    /// virtual storage object.
    /// 
    /// ***VslmSyncFault***: If an error occurs during datastore synchornization,
    /// implies the registerDisk completed successfully,
    /// however, there was an exception during datastore
    /// synchornization.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_register_disk(&self, path: &str, name: Option<&str>) -> Result<crate::types::structs::VStorageObject> {
        let input = VslmRegisterDiskRequestType {path, name, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRegisterDisk", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Relocate a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on both source and
    /// destination datastore.
    /// 
    /// *VslmSyncFault* will set in the task error if the
    /// relocateVStorageObject completed successfully but the datastore
    /// synchronization failed.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### spec
    /// The specification for relocation of the virtual
    /// storage object.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while relocating the virtual
    /// storage object.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// The disk may be consumed and cannot be relocated.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot
    /// be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_relocate_v_storage_object_task(&self, id: &crate::types::structs::Id, spec: &crate::types::structs::VslmRelocateSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmRelocateVStorageObjectRequestType {id, spec, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRelocateVStorageObject_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Rename a virtual storage object.
    /// 
    /// *VslmSyncFault* is thrown when
    /// the underlying rename operation of the FCD succeeded, however, there
    /// was a issue with syncing the datastore or FCD. Rename operation of the
    /// FCD is not idempotent, implying, any attempt to rename the FCD with it's
    /// current name would result in a error. The sync fault can be ignored
    /// with the risk of Pandora DB having stale name, consequently,
    /// affecting the return of *VslmVStorageObjectManager.VslmListVStorageObjectForSpec*
    /// and *VslmVStorageObjectManager.VslmRetrieveVStorageObjects* APIs.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object to be renamed.
    ///
    /// ### name
    /// The new name for the virtual storage object.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while renaming the virtual storage
    /// object.
    /// 
    /// ***NotFound***: If the specified virtual storage object cannot be
    /// found.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***VslmSyncFault***: If an error occurs during datastore synchornization,
    /// implies the renameVStorageObject completed
    /// successfully, however, there was an exception during
    /// datastore synchornization.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_rename_v_storage_object(&self, id: &crate::types::structs::Id, name: &str) -> Result<()> {
        let input = VslmRenameVStorageObjectRequestType {id, name, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRenameVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Retrieves snapshot disk details of a given snapshot.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### snapshot_id
    /// The ID of the snapshot of a virtual storage object.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while snapshotting the virtual
    /// storage object.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_retrieve_snapshot_details(&self, id: &crate::types::structs::Id, snapshot_id: &crate::types::structs::Id) -> Result<crate::types::structs::VStorageObjectSnapshotDetails> {
        let input = VslmRetrieveSnapshotDetailsRequestType {id, snapshot_id, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRetrieveSnapshotDetails", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Retrieves snapshot information of a given VStorageObject.
    /// 
    /// Requires Datastore.FileManagement privilege.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while snapshotting the virtual
    /// storage object.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_retrieve_snapshot_info(&self, id: &crate::types::structs::Id) -> Result<crate::types::structs::VStorageObjectSnapshotInfo> {
        let input = VslmRetrieveSnapshotInfoRequestType {id, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRetrieveSnapshotInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Retrieve FCD infrastructure object SBPM policy on given datastore.
    /// 
    /// Only support VSAN datastore.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage infrastructure object is located.
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// Datastore on which policy needs to be retrieved.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// The policy object of virtual storage object.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_retrieve_v_storage_infrastructure_object_policy(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::VslmInfrastructureObjectPolicy>>> {
        let input = VslmRetrieveVStorageInfrastructureObjectPolicyRequestType {datastore, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRetrieveVStorageInfrastructureObjectPolicy", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Retrieve a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object to be retrieved.
    ///
    /// ## Returns:
    ///
    /// The required virtual storage object.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs when retrieving the virtual object.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_retrieve_v_storage_object(&self, id: &crate::types::structs::Id) -> Result<crate::types::structs::VStorageObject> {
        let input = VslmRetrieveVStorageObjectRequestType {id, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRetrieveVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Retrieve vm associations for each virtual storage object in the query.
    ///
    /// ## Parameters:
    ///
    /// ### ids
    /// The IDs of the virtual storage objects of the query.
    ///
    /// ## Returns:
    ///
    /// The list of VStorageObjectVmAssociations which provides FCD id
    /// to vm associations mapping.
    pub async fn vslm_retrieve_v_storage_object_associations(&self, ids: Option<&[crate::types::structs::Id]>) -> Result<Option<Vec<crate::types::structs::VslmVsoVStorageObjectAssociations>>> {
        let input = VslmRetrieveVStorageObjectAssociationsRequestType {ids, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRetrieveVStorageObjectAssociations", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Retrieve metadata KV pairs from a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### snapshot_id
    /// The ID of the snapshot of virtual storage object.
    ///
    /// ### prefix
    /// The prefix of the metadata key that needs to be retrieved
    ///
    /// ## Returns:
    ///
    /// returns the array of key value pair
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore, such as datastore cannot be found
    /// or inaccessible.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_retrieve_v_storage_object_metadata(&self, id: &crate::types::structs::Id, snapshot_id: Option<&crate::types::structs::Id>, prefix: Option<&str>) -> Result<Option<Vec<crate::types::structs::KeyValue>>> {
        let input = VslmRetrieveVStorageObjectMetadataRequestType {id, snapshot_id, prefix, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRetrieveVStorageObjectMetadata", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Retrieve the metadata value by key from a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### snapshot_id
    /// The ID of the snapshot of virtual storage object.
    ///
    /// ### key
    /// The key for the the virtual storage object
    ///
    /// ## Returns:
    ///
    /// returns the value for the key
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore, such as datastore cannot be found
    /// or inaccessible.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***KeyNotFound***: If specified key cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_retrieve_v_storage_object_metadata_value(&self, id: &crate::types::structs::Id, snapshot_id: Option<&crate::types::structs::Id>, key: &str) -> Result<String> {
        let input = VslmRetrieveVStorageObjectMetadataValueRequestType {id, snapshot_id, key, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRetrieveVStorageObjectMetadataValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Retrieve a virtual storage object state.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object the state to be retrieved.
    ///
    /// ## Returns:
    ///
    /// The required virtual storage object state.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs when retrieving the virtual object
    /// state.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_retrieve_v_storage_object_state(&self, id: &crate::types::structs::Id) -> Result<crate::types::structs::VStorageObjectStateInfo> {
        let input = VslmRetrieveVStorageObjectStateRequestType {id, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRetrieveVStorageObjectState", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Retrieves details of a list of virtual storage objects from cache.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    ///
    /// ## Parameters:
    ///
    /// ### ids
    /// The array of IDs of the virtual storage object to be
    /// retrieved.
    ///
    /// ## Returns:
    ///
    /// The array of *VslmVsoVStorageObjectResult* virtual
    /// storage objects corresponding to the input.
    pub async fn vslm_retrieve_v_storage_objects(&self, ids: Option<&[crate::types::structs::Id]>) -> Result<Option<Vec<crate::types::structs::VslmVsoVStorageObjectResult>>> {
        let input = VslmRetrieveVStorageObjectsRequestType {ids, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRetrieveVStorageObjects", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Reverts to a given snapshot of a VStorageObject.
    /// 
    /// This operation is supported on detached VirtualDisks
    /// During revert all the snapshots which were taken after the specified
    /// snapshot would get deleted.
    /// 
    /// E.g. Consider Disk with 4 snapshots
    /// 
    /// BaseDisk -$gt; Snap-2 -$gt; Snap-3 -$gt; Snap-4 -$gt; Running-Point
    /// 
    /// If user chooses to revert to snap-2 then snap-4 and snap-3 would also
    /// be deleted. After revert operation disk would have below configuration:
    /// 
    /// BaseDisk -$gt; Snap-2 -$gt; Running-Point
    /// 
    /// Requires Datastore.FileManagement privilege.
    /// 
    /// *VslmSyncFault* will set in the task error if the
    /// revertVStorageObject completed successfully but the datastore
    /// synchronization failed.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### snapshot_id
    /// The ID of the snapshot of a virtual storage object.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while snapshotting the virtual
    /// storage object.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_revert_v_storage_object_task(&self, id: &crate::types::structs::Id, snapshot_id: &crate::types::structs::Id) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmRevertVStorageObjectRequestType {id, snapshot_id, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmRevertVStorageObject_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Schedules reconcile of the inventory info of virtual storage objects on
    /// one of the hosts that is connected with the datastore.
    /// 
    /// This method just
    /// schedules the reconcile operation for the nearby future and returns. Note
    /// that since the reconcile operation will be executed after this method
    /// already returns the success of this method should not be considered as
    /// success of the actual reconcile operation.
    /// 
    /// Requires Datastore.FileManagement privilege.
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The datastore that needs to be reconciled.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on
    /// the datastore.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_schedule_reconcile_datastore_inventory(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<()> {
        let input = VslmScheduleReconcileDatastoreInventoryRequestType {datastore, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmScheduleReconcileDatastoreInventory", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Set control flags on VStorageObject.
    /// 
    /// The control flags are defined in
    /// *vslmVStorageObjectControlFlag_enum*.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### control_flags
    /// control flags enum array to be set on the
    /// VStorageObject. All control flags not included
    /// in the array remain intact.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on
    /// the datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// The disk may be consumed.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot
    /// be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_set_v_storage_object_control_flags(&self, id: &crate::types::structs::Id, control_flags: Option<&[String]>) -> Result<()> {
        let input = VslmSetVStorageObjectControlFlagsRequestType {id, control_flags, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmSetVStorageObjectControlFlags", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Assigns specified SBPM policy to the given FCD infrastructure object.
    /// 
    /// Only support VSAN datastore.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage infrastructure object is located.
    /// 
    /// *VslmSyncFault* will set in the task error if the
    /// updateVStorageInfrastructureObjectPolicy completed successfully
    /// but the datastore synchronization failed.
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// specification to assign a SPBM policy to FCD infrastructure
    /// object.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If there is issue with profile spec.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_update_v_storage_infrastructure_object_policy_task(&self, spec: &crate::types::structs::VslmInfrastructureObjectPolicySpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmUpdateVStorageInfrastructureObjectPolicyRequestType {spec, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmUpdateVStorageInfrastructureObjectPolicy_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Update the storage crypto on a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### profile
    /// New profile requirement on the virtual storage object.
    ///
    /// ### disks_crypto
    /// The crypto information of each disk on the chain.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while updating the virtual storage
    /// object policy.
    /// 
    /// ***NotFound***: If the specified virtual storage object cannot be
    /// found.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***TaskInProgress***: If the virtual storage object is busy.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_update_vstorage_object_crypto_task(&self, id: &crate::types::structs::Id, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>, disks_crypto: Option<&crate::types::structs::DiskCryptoSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmUpdateVstorageObjectCryptoRequestType {id, profile, disks_crypto, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmUpdateVstorageObjectCrypto_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Update metadata KV pairs to a virtual storage object.
    /// 
    /// And this API is by
    /// design supposed to be used for all of the addition, modification and
    /// deletion operations of metadata KV pairs.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// *VslmSyncFault* will set in the task error if the
    /// updateVStorageObjectMetadata completed successfully but the datastore
    /// synchronization or FCD retrieval failed. The sync fault can
    /// be ignored with the risk of Pandora DB having stale metadata information
    /// consequently affecting the return of *VslmVStorageObjectManager.VslmListVStorageObjectForSpec*
    /// and *VslmVStorageObjectManager.VslmRetrieveVStorageObjects* APIs.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### metadata
    /// array of key/value strings. (keys must be unique
    /// within the list)
    ///
    /// ### delete_keys
    /// array of keys need to be deleted
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore, such as datastore cannot be found
    /// or inaccessible.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_update_v_storage_object_metadata_task(&self, id: &crate::types::structs::Id, metadata: Option<&[crate::types::structs::KeyValue]>, delete_keys: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmUpdateVStorageObjectMetadataRequestType {id, metadata, delete_keys, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmUpdateVStorageObjectMetadata_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Update the storage policy on a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### profile
    /// New profile requirement on the virtual storage object.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while updating the virtual storage
    /// object policy.
    /// 
    /// ***NotFound***: If the specified virtual storage object cannot be
    /// found.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***TaskInProgress***: If the virtual storage object is busy.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_update_vstorage_object_policy_task(&self, id: &crate::types::structs::Id, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VslmUpdateVstorageObjectPolicyRequestType {id, profile, };
        let path = format!("/vslm/VslmVStorageObjectManager/{moId}/VslmUpdateVstorageObjectPolicy_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmAttachDiskRequestType<'a> {
    id: &'a crate::types::structs::Id,
    vm: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "controllerKey")]
    controller_key: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unitNumber")]
    unit_number: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmAttachTagToVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    category: &'a str,
    tag: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmClearVStorageObjectControlFlagsRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "controlFlags")]
    control_flags: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmCloneVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    spec: &'a crate::types::structs::VslmCloneSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmCreateDiskRequestType<'a> {
    spec: &'a crate::types::structs::VslmCreateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmCreateDiskFromSnapshotRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a crate::types::structs::Id,
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    crypto: Option<&'a dyn crate::types::traits::CryptoSpecTrait>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    path: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmCreateSnapshotRequestType<'a> {
    id: &'a crate::types::structs::Id,
    description: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmDeleteSnapshotRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a crate::types::structs::Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmDeleteVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmDetachTagFromVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    category: &'a str,
    tag: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmExtendDiskRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(rename = "newCapacityInMB")]
    new_capacity_in_mb: i64,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmInflateDiskRequestType<'a> {
    id: &'a crate::types::structs::Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmListTagsAttachedToVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmListVStorageObjectsAttachedToTagRequestType<'a> {
    category: &'a str,
    tag: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmListVStorageObjectForSpecRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    query: Option<&'a [crate::types::structs::VslmVsoVStorageObjectQuerySpec]>,
    #[serde(rename = "maxResult")]
    max_result: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmQueryChangedDiskAreasRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a crate::types::structs::Id,
    #[serde(rename = "startOffset")]
    start_offset: i64,
    #[serde(rename = "changeId")]
    change_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmQueryGlobalCatalogSyncStatusForDatastoreRequestType<'a> {
    #[serde(rename = "datastoreURL")]
    datastore_url: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmReconcileDatastoreInventoryRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRegisterDiskRequestType<'a> {
    path: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRelocateVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    spec: &'a crate::types::structs::VslmRelocateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRenameVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRetrieveSnapshotDetailsRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a crate::types::structs::Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRetrieveSnapshotInfoRequestType<'a> {
    id: &'a crate::types::structs::Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRetrieveVStorageInfrastructureObjectPolicyRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRetrieveVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRetrieveVStorageObjectAssociationsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ids: Option<&'a [crate::types::structs::Id]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRetrieveVStorageObjectMetadataRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "snapshotId")]
    snapshot_id: Option<&'a crate::types::structs::Id>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    prefix: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRetrieveVStorageObjectMetadataValueRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "snapshotId")]
    snapshot_id: Option<&'a crate::types::structs::Id>,
    key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRetrieveVStorageObjectStateRequestType<'a> {
    id: &'a crate::types::structs::Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRetrieveVStorageObjectsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ids: Option<&'a [crate::types::structs::Id]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmRevertVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a crate::types::structs::Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmScheduleReconcileDatastoreInventoryRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmSetVStorageObjectControlFlagsRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "controlFlags")]
    control_flags: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmUpdateVStorageInfrastructureObjectPolicyRequestType<'a> {
    spec: &'a crate::types::structs::VslmInfrastructureObjectPolicySpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmUpdateVstorageObjectCryptoRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "disksCrypto")]
    disks_crypto: Option<&'a crate::types::structs::DiskCryptoSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmUpdateVStorageObjectMetadataRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a [crate::types::structs::KeyValue]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deleteKeys")]
    delete_keys: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmUpdateVstorageObjectPolicyRequestType<'a> {
    id: &'a crate::types::structs::Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
}
