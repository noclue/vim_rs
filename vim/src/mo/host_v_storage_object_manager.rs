use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::Id;
use crate::types::structs::KeyValue;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::VStorageObject;
use crate::types::structs::VStorageObjectSnapshotInfo;
use crate::types::structs::VStorageObjectStateInfo;
use crate::types::structs::VslmCloneSpec;
use crate::types::structs::VslmCreateSpec;
use crate::types::structs::VslmInfrastructureObjectPolicy;
use crate::types::structs::VslmRelocateSpec;
use crate::types::structs::VslmVClockInfo;
/// Interface to manage virtual storage object on an ESXi host.
pub struct HostVStorageObjectManager {
    client: Arc<Client>,
    mo_id: String,
}
impl HostVStorageObjectManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Reverts to a given snapshot of a VStorageObject.
    /// 
    /// This operation is supported on detached VirtualDisks
    /// During revert all the snapshots which were taken after the specified
    /// snapshot would get deleted.
    /// 
    /// E.g. Consider Disk with 4 snapshots
    /// 
    /// BaseDisk -&gt; Snap-2 -&gt; Snap-3 -&gt; Snap-4 -&gt; Running-Point
    /// 
    /// If user chooses to revert to snap-2 then snap-4 and snap-3 would also
    /// be deleted. After revert operation disk would have below configuration:
    /// 
    /// BaseDisk -&gt; Snap-2 -&gt; Running-Point
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage object
    /// is located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_v_storage_object_revert_task(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: &Id) -> Result<ManagedObjectReference> {
        let input = HostVStorageObjectRevertRequestType {id, datastore, snapshot_id, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostVStorageObjectRevert_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Clear control flags on VStorageObject.
    /// 
    /// The control flags are defined in
    /// *vslmVStorageObjectControlFlag_enum*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage
    /// object is located.
    /// 
    /// ***Required privileges:*** Datastore.FileManagement
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_clear_v_storage_object_control_flags(&self, id: &Id, datastore: &ManagedObjectReference, control_flags: Option<&[String]>) -> Result<()> {
        let input = HostClearVStorageObjectControlFlagsRequestType {id, datastore, control_flags, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostClearVStorageObjectControlFlags", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Clone a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on both source and
    /// destination datastore.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage
    /// object is located.
    /// 
    /// Refers instance of *Datastore*.
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
    /// ***InvalidDatastore***: If the operation cannot be performed on
    /// the datastore.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be
    /// found.
    pub async fn host_clone_v_storage_object_task(&self, id: &Id, datastore: &ManagedObjectReference, spec: &VslmCloneSpec) -> Result<ManagedObjectReference> {
        let input = HostCloneVStorageObjectRequestType {id, datastore, spec, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostCloneVStorageObject_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Create a virtual disk, which is a storage object with
    /// *disk*
    /// as consumption type.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk object is created.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The specification of the virtual storage object
    /// to be created.
    /// 2
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
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
    pub async fn host_create_disk_task(&self, spec: &VslmCreateSpec) -> Result<ManagedObjectReference> {
        let input = HostCreateDiskRequestType {spec, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostCreateDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a new Disk from given snapshot of a VStorageObject.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage object
    /// is located.
    /// 
    /// Refers instance of *Datastore*.
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
    /// to be created. If not specified disk gets created at defualt
    /// VStorageObject location on the specified datastore
    ///
    /// ### provisioning_type
    /// Provisioining type of the disk as specified in above
    /// mentioned profile. The list of supported values can be found in
    /// *BaseConfigInfoDiskFileBackingInfoProvisioningType_enum*
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
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
    pub async fn host_v_storage_object_create_disk_from_snapshot_task(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: &Id, name: &str, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>, crypto: Option<&dyn crate::types::traits::CryptoSpecTrait>, path: Option<&str>, provisioning_type: Option<&str>) -> Result<ManagedObjectReference> {
        let input = HostVStorageObjectCreateDiskFromSnapshotRequestType {id, datastore, snapshot_id, name, profile, crypto, path, provisioning_type, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostVStorageObjectCreateDiskFromSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a snapshot of a given VStorageObject.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage object
    /// is located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_v_storage_object_create_snapshot_task(&self, id: &Id, datastore: &ManagedObjectReference, description: &str) -> Result<ManagedObjectReference> {
        let input = HostVStorageObjectCreateSnapshotRequestType {id, datastore, description, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostVStorageObjectCreateSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a snapshot of a given VStorageObject.
    /// 
    /// Requires Datastore.FileManagement privilege.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage object
    /// is located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn v_storage_object_create_snapshot_ex_task(&self, id: &Id, datastore: &ManagedObjectReference, description: &str) -> Result<ManagedObjectReference> {
        let input = VStorageObjectCreateSnapshotExRequestType {id, datastore, description, };
        let path = format!("/HostVStorageObjectManager/{moId}/VStorageObjectCreateSnapshotEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deletes a given snapshot of a VStorageObject.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage object
    /// is located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_v_storage_object_delete_snapshot_task(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: &Id) -> Result<ManagedObjectReference> {
        let input = HostVStorageObjectDeleteSnapshotRequestType {id, datastore, snapshot_id, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostVStorageObjectDeleteSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deletes a given snapshot of a VStorageObject.
    /// 
    /// Requires Datastore.FileManagement privilege.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage object
    /// is located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn v_storage_object_delete_snapshot_ex_task(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: &Id) -> Result<ManagedObjectReference> {
        let input = VStorageObjectDeleteSnapshotExRequestType {id, datastore, snapshot_id, };
        let path = format!("/HostVStorageObjectManager/{moId}/VStorageObjectDeleteSnapshotEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Delete a virtual storage object and its assoicated backings.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object to be deleted.
    ///
    /// ### datastore
    /// The datastore where the virtual storage object is
    /// located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_delete_v_storage_object_task(&self, id: &Id, datastore: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = HostDeleteVStorageObjectRequestType {id, datastore, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostDeleteVStorageObject_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Delete a virtual storage object and its assoicated backings.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object to be deleted.
    ///
    /// ### datastore
    /// The datastore where the virtual storage object is
    /// located.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Returns:
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
    pub async fn host_delete_v_storage_object_ex_task(&self, id: &Id, datastore: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = HostDeleteVStorageObjectExRequestType {id, datastore, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostDeleteVStorageObjectEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual disk to be extended.
    ///
    /// ### datastore
    /// The datastore where the virtual disk is located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_extend_disk_task(&self, id: &Id, datastore: &ManagedObjectReference, new_capacity_in_mb: i64) -> Result<ManagedObjectReference> {
        let input = HostExtendDiskRequestType {id, datastore, new_capacity_in_mb, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostExtendDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    /// ***Since:*** vSphere API Release 8.0.2.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual disk to be extended.
    ///
    /// ### datastore
    /// The datastore where the virtual disk is located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn v_storage_object_extend_disk_ex_task(&self, id: &Id, datastore: &ManagedObjectReference, new_capacity_in_mb: i64) -> Result<ManagedObjectReference> {
        let input = VStorageObjectExtendDiskExRequestType {id, datastore, new_capacity_in_mb, };
        let path = format!("/HostVStorageObjectManager/{moId}/VStorageObjectExtendDiskEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Inflate a sparse or thin-provisioned virtual disk up to the full size.
    /// 
    /// Additional space allocated to the disk as a result of this operation
    /// will be filled with zeroes.
    /// 
    /// Currently inflateDisk API only supports the following combinations:
    /// Valid provisioning type: THIN;
    /// Valid Datastore: VMFS, NFS.
    /// Inflating a disk is not applicable for VVol/VSAN datastore.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual disk to be inflated.
    ///
    /// ### datastore
    /// The datastore where the virtual disk is located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_inflate_disk_task(&self, id: &Id, datastore: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = HostInflateDiskRequestType {id, datastore, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostInflateDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// List all virtual storage objects located on a datastore.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The datastore to query for the virtual storage objects.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// The list of IDs of the virtual storage objects located on the
    /// datastore.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore, such as datastore cannot be found
    /// or inaccessible.
    pub async fn host_list_v_storage_object(&self, datastore: &ManagedObjectReference) -> Result<Option<Vec<Id>>> {
        let input = HostListVStorageObjectRequestType {datastore, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostListVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Reconcile the datastore inventory info of virtual storage objects.
    /// 
    /// Requires Datastore.FileManagement privilege.
    /// 
    /// ***Required privileges:*** System.View
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
    pub async fn host_reconcile_datastore_inventory_task(&self, datastore: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = HostReconcileDatastoreInventoryRequestType {datastore, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostReconcileDatastoreInventory_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Promote a virtual disk to a First Class Disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### path
    /// URL or datastore path to the virtual disk.
    ///
    /// ### name
    /// The descriptive name of the disk object. If
    /// unset the name will be automatically determined
    /// from the path. @see vim.vslm.BaseConfigInfo#name
    ///
    /// ### modify_control_flags
    /// Optional Parameter describing if the control Flags should be changed to default values
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
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
    pub async fn host_register_disk(&self, path: &str, name: Option<&str>, modify_control_flags: Option<bool>) -> Result<VStorageObject> {
        let input = HostRegisterDiskRequestType {path, name, modify_control_flags, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostRegisterDisk", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Relocate a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on both source and
    /// destination datastore.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage
    /// object is located.
    /// 
    /// Refers instance of *Datastore*.
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
    /// ***InvalidDatastore***: If the operation cannot be performed on
    /// the datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// The disk may be consumed and cannot be relocated.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot
    /// be found.
    pub async fn host_relocate_v_storage_object_task(&self, id: &Id, datastore: &ManagedObjectReference, spec: &VslmRelocateSpec) -> Result<ManagedObjectReference> {
        let input = HostRelocateVStorageObjectRequestType {id, datastore, spec, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostRelocateVStorageObject_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Rename a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object to be renamed.
    ///
    /// ### datastore
    /// The datastore where the virtual storage object is
    /// located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_rename_v_storage_object(&self, id: &Id, datastore: &ManagedObjectReference, name: &str) -> Result<()> {
        let input = HostRenameVStorageObjectRequestType {id, datastore, name, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostRenameVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Rename a virtual storage object.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object to be renamed.
    ///
    /// ### datastore
    /// The datastore where the virtual storage object is
    /// located.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### name
    /// The new name for the virtual storage object.
    ///
    /// ## Returns:
    ///
    /// The vclock info of this operation
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
    pub async fn rename_v_storage_object_ex(&self, id: &Id, datastore: &ManagedObjectReference, name: &str) -> Result<VslmVClockInfo> {
        let input = RenameVStorageObjectExRequestType {id, datastore, name, };
        let path = format!("/HostVStorageObjectManager/{moId}/RenameVStorageObjectEx", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieves snapshot information of a given VStorageObject.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage object
    /// is located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_v_storage_object_retrieve_snapshot_info(&self, id: &Id, datastore: &ManagedObjectReference) -> Result<VStorageObjectSnapshotInfo> {
        let input = HostVStorageObjectRetrieveSnapshotInfoRequestType {id, datastore, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostVStorageObjectRetrieveSnapshotInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieve virtual storage infrastructure object SBPM policy on given
    /// datastore.
    /// 
    /// Only support VSAN datastore.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore specified.
    /// 
    /// ***Required privileges:*** System.View
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
    pub async fn host_retrieve_v_storage_infrastructure_object_policy(&self, datastore: &ManagedObjectReference) -> Result<Option<Vec<VslmInfrastructureObjectPolicy>>> {
        let input = HostRetrieveVStorageInfrastructureObjectPolicyRequestType {datastore, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostRetrieveVStorageInfrastructureObjectPolicy", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Retrieve a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object to be retrieved.
    ///
    /// ### datastore
    /// The datastore where the virtual storage object is
    /// located.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### disk_info_flags
    /// Flags indicating the FCD information to be
    /// retrieved. If diskInfoFlags is unset, then all FCD
    /// information will be retrieved. See
    /// *vslmDiskInfoFlag_enum* for the list of
    /// supported values.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
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
    pub async fn host_retrieve_v_storage_object(&self, id: &Id, datastore: &ManagedObjectReference, disk_info_flags: Option<&[String]>) -> Result<VStorageObject> {
        let input = HostRetrieveVStorageObjectRequestType {id, datastore, disk_info_flags, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostRetrieveVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieve metadata KV pairs from a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore to query for the virtual storage objects.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_retrieve_v_storage_object_metadata(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: Option<&Id>, prefix: Option<&str>) -> Result<Option<Vec<KeyValue>>> {
        let input = HostRetrieveVStorageObjectMetadataRequestType {id, datastore, snapshot_id, prefix, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostRetrieveVStorageObjectMetadata", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Retrieve the metadata value by key from a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore to query for the virtual storage objects.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_retrieve_v_storage_object_metadata_value(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: Option<&Id>, key: &str) -> Result<String> {
        let input = HostRetrieveVStorageObjectMetadataValueRequestType {id, datastore, snapshot_id, key, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostRetrieveVStorageObjectMetadataValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieve a virtual storage object state.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object the state to be retrieved.
    ///
    /// ### datastore
    /// The datastore where the virtual storage object is
    /// located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_retrieve_v_storage_object_state(&self, id: &Id, datastore: &ManagedObjectReference) -> Result<VStorageObjectStateInfo> {
        let input = HostRetrieveVStorageObjectStateRequestType {id, datastore, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostRetrieveVStorageObjectState", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Reverts to a given snapshot of a VStorageObject.
    /// 
    /// This operation is supported on detached VirtualDisks
    /// During revert all the snapshots which were taken after the specified
    /// snapshot would get deleted.
    /// 
    /// E.g. Consider Disk with 4 snapshots
    /// 
    /// BaseDisk -&gt; Snap-2 -&gt; Snap-3 -&gt; Snap-4 -&gt; Running-Point
    /// 
    /// If user chooses to revert to snap-2 then snap-4 and snap-3 would also
    /// be deleted. After revert operation disk would have below configuration:
    /// 
    /// BaseDisk -&gt; Snap-2 -&gt; Running-Point
    /// 
    /// Requires Datastore.FileManagement privilege.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage object
    /// is located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn revert_v_storage_object_ex_task(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: &Id) -> Result<ManagedObjectReference> {
        let input = RevertVStorageObjectExRequestType {id, datastore, snapshot_id, };
        let path = format!("/HostVStorageObjectManager/{moId}/RevertVStorageObjectEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Schedules reconcile of the datastore inventory info of virtual storage
    /// objects.
    /// 
    /// This method just schedules the reconcile operation for the
    /// nearby future and returns. Note that since the reconcile operation will
    /// be executed after this method already returns the success of this method
    /// should not be considered as success of the actual reconcile operation.
    /// 
    /// Requires Datastore.FileManagement privilege.
    /// 
    /// ***Required privileges:*** System.View
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
    pub async fn host_schedule_reconcile_datastore_inventory(&self, datastore: &ManagedObjectReference) -> Result<()> {
        let input = HostScheduleReconcileDatastoreInventoryRequestType {datastore, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostScheduleReconcileDatastoreInventory", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Set control flags on VStorageObject.
    /// 
    /// The control flags are defined in
    /// *vslmVStorageObjectControlFlag_enum*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore where the source virtual storage
    /// object is located.
    /// 
    /// ***Required privileges:*** Datastore.FileManagement
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_set_v_storage_object_control_flags(&self, id: &Id, datastore: &ManagedObjectReference, control_flags: Option<&[String]>) -> Result<()> {
        let input = HostSetVStorageObjectControlFlagsRequestType {id, datastore, control_flags, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostSetVStorageObjectControlFlags", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore to query for the virtual storage objects.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_update_v_storage_object_metadata_task(&self, id: &Id, datastore: &ManagedObjectReference, metadata: Option<&[KeyValue]>, delete_keys: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = HostUpdateVStorageObjectMetadataRequestType {id, datastore, metadata, delete_keys, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostUpdateVStorageObjectMetadata_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    /// ***Since:*** vSphere API Release 7.0.2.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object.
    ///
    /// ### datastore
    /// The datastore to query for the virtual storage objects.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn host_update_v_storage_object_metadata_ex_task(&self, id: &Id, datastore: &ManagedObjectReference, metadata: Option<&[KeyValue]>, delete_keys: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = HostUpdateVStorageObjectMetadataExRequestType {id, datastore, metadata, delete_keys, };
        let path = format!("/HostVStorageObjectManager/{moId}/HostUpdateVStorageObjectMetadataEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostVStorageObjectRevertRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostClearVStorageObjectControlFlagsRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "controlFlags")]
    control_flags: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostCloneVStorageObjectRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    spec: &'a VslmCloneSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostCreateDiskRequestType<'a> {
    spec: &'a VslmCreateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostVStorageObjectCreateDiskFromSnapshotRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a Id,
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    crypto: Option<&'a dyn crate::types::traits::CryptoSpecTrait>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    path: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "provisioningType")]
    provisioning_type: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostVStorageObjectCreateSnapshotRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    description: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VStorageObjectCreateSnapshotExRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    description: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostVStorageObjectDeleteSnapshotRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VStorageObjectDeleteSnapshotExRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostDeleteVStorageObjectRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostDeleteVStorageObjectExRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostExtendDiskRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "newCapacityInMB")]
    new_capacity_in_mb: i64,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VStorageObjectExtendDiskExRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "newCapacityInMB")]
    new_capacity_in_mb: i64,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostInflateDiskRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostListVStorageObjectRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostReconcileDatastoreInventoryRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostRegisterDiskRequestType<'a> {
    path: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modifyControlFlags")]
    modify_control_flags: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostRelocateVStorageObjectRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    spec: &'a VslmRelocateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostRenameVStorageObjectRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameVStorageObjectExRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostVStorageObjectRetrieveSnapshotInfoRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostRetrieveVStorageInfrastructureObjectPolicyRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostRetrieveVStorageObjectRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "diskInfoFlags")]
    disk_info_flags: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostRetrieveVStorageObjectMetadataRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "snapshotId")]
    snapshot_id: Option<&'a Id>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    prefix: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostRetrieveVStorageObjectMetadataValueRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "snapshotId")]
    snapshot_id: Option<&'a Id>,
    key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostRetrieveVStorageObjectStateRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RevertVStorageObjectExRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostScheduleReconcileDatastoreInventoryRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostSetVStorageObjectControlFlagsRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "controlFlags")]
    control_flags: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostUpdateVStorageObjectMetadataRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a [KeyValue]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deleteKeys")]
    delete_keys: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostUpdateVStorageObjectMetadataExRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a [KeyValue]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deleteKeys")]
    delete_keys: Option<&'a [String]>,
}
