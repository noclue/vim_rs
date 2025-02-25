use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::DiskChangeInfo;
use crate::types::structs::DiskCryptoSpec;
use crate::types::structs::Id;
use crate::types::structs::KeyValue;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::RetrieveVStorageObjSpec;
use crate::types::structs::VStorageObject;
use crate::types::structs::VStorageObjectAssociations;
use crate::types::structs::VStorageObjectSnapshotDetails;
use crate::types::structs::VStorageObjectSnapshotInfo;
use crate::types::structs::VStorageObjectStateInfo;
use crate::types::structs::VslmCloneSpec;
use crate::types::structs::VslmCreateSpec;
use crate::types::structs::VslmInfrastructureObjectPolicy;
use crate::types::structs::VslmInfrastructureObjectPolicySpec;
use crate::types::structs::VslmRelocateSpec;
use crate::types::structs::VslmTagEntry;
use crate::types::structs::VslmVClockInfo;
/// Interface to manage virtual storage object on a vCenter.
/// 
/// VStorageObjectManager and SPBM policy support:
/// All of the VStorageObjectManager APIs requiring ESXi host
/// uses "Programatically selected" host to perform the actual operation.
/// If the selected host is of 6.5 version then policy would not be passed
/// down to host. In that case, user operation would succeed but if user checks
/// SPBM Entity Compliance, it will show "Mismatch" / "Non Compliant" as a
/// compliance result.
pub struct VcenterVStorageObjectManager {
    client: Arc<Client>,
    mo_id: String,
}
impl VcenterVStorageObjectManager {
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
    /// Requires Datastore.FileManagement privilege.
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
    pub async fn revert_v_storage_object_task(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: &Id) -> Result<ManagedObjectReference> {
        let input = RevertVStorageObjectRequestType {id, datastore, snapshot_id, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/RevertVStorageObject_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Attach a tag to a virtual storage object.
    /// 
    /// Requires privilege InventoryService.Tagging.AttachTag on root folder
    /// 
    /// ***Required privileges:*** System.Read
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
    pub async fn attach_tag_to_v_storage_object(&self, id: &Id, category: &str, tag: &str) -> Result<()> {
        let input = AttachTagToVStorageObjectRequestType {id, category, tag, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/AttachTagToVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn clear_v_storage_object_control_flags(&self, id: &Id, datastore: &ManagedObjectReference, control_flags: Option<&[String]>) -> Result<()> {
        let input = ClearVStorageObjectControlFlagsRequestType {id, datastore, control_flags, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/ClearVStorageObjectControlFlags", moId = &self.mo_id);
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
    /// The datastore where the source virtual storage object
    /// is located.
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
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be
    /// found.
    pub async fn clone_v_storage_object_task(&self, id: &Id, datastore: &ManagedObjectReference, spec: &VslmCloneSpec) -> Result<ManagedObjectReference> {
        let input = CloneVStorageObjectRequestType {id, datastore, spec, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/CloneVStorageObject_Task", moId = &self.mo_id);
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
    pub async fn create_disk_task(&self, spec: &VslmCreateSpec) -> Result<ManagedObjectReference> {
        let input = CreateDiskRequestType {spec, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/CreateDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a new Disk from given snapshot of a VStorageObject.
    /// 
    /// Requires Datastore.FileManagement privilege.
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
    /// If unset and if profile contains an encryption iofilter and
    /// if snapshto is unencrypted, then cyrpto will be of
    /// type CryptoSpecEncrypt, and filled with keyId that is
    /// automatically generated and keyProviderId that is the
    /// default kms cluster.
    /// If unset and if profile is a default policy and if snapshot
    /// is unenrypted, then crypto is treated as CryptoSpecNoOp.
    /// If unset and if profile contains an encryption iofilter and
    /// if snapshot is encrypted, then cyrpto is treated as
    /// CryptoSpecNoOp.
    /// If unset and if profile is a default policy and if
    /// snapshot is encrypted, then cyrpto is treated as
    /// CryptoSpecDecrypt.
    /// To recrypt the disk during creating disk, crypto has to be
    /// present.
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
    pub async fn create_disk_from_snapshot_task(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: &Id, name: &str, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>, crypto: Option<&dyn crate::types::traits::CryptoSpecTrait>, path: Option<&str>) -> Result<ManagedObjectReference> {
        let input = CreateDiskFromSnapshotRequestType {id, datastore, snapshot_id, name, profile, crypto, path, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/CreateDiskFromSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a snapshot of a given VStorageObject.
    /// 
    /// Requires Datastore.FileManagement privilege.
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
    pub async fn v_storage_object_create_snapshot_task(&self, id: &Id, datastore: &ManagedObjectReference, description: &str) -> Result<ManagedObjectReference> {
        let input = VStorageObjectCreateSnapshotRequestType {id, datastore, description, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/VStorageObjectCreateSnapshot_Task", moId = &self.mo_id);
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
        let path = format!("/VcenterVStorageObjectManager/{moId}/VStorageObjectCreateSnapshotEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deletes a given snapshot of a VStorageObject.
    /// 
    /// Requires Datastore.FileManagement privilege.
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
    pub async fn delete_snapshot_task(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: &Id) -> Result<ManagedObjectReference> {
        let input = DeleteSnapshotRequestType {id, datastore, snapshot_id, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/DeleteSnapshot_Task", moId = &self.mo_id);
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
        let path = format!("/VcenterVStorageObjectManager/{moId}/VStorageObjectDeleteSnapshotEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Delete a virtual storage object and its associated backings.
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
    /// The datastore where the virtual storage object
    /// is located.
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
    pub async fn delete_v_storage_object_task(&self, id: &Id, datastore: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = DeleteVStorageObjectRequestType {id, datastore, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/DeleteVStorageObject_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Delete a virtual storage object and its associated backings.
    /// 
    /// Returns
    /// the corresponding vclock upon succeess.
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
    /// The datastore where the virtual storage object
    /// is located.
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
    pub async fn delete_v_storage_object_ex_task(&self, id: &Id, datastore: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = DeleteVStorageObjectExRequestType {id, datastore, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/DeleteVStorageObjectEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Detach a tag from a virtual storage object.
    /// 
    /// Requires privilege InventoryService.Tagging.AttachTag on root folder
    /// 
    /// ***Required privileges:*** System.Read
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
    pub async fn detach_tag_from_v_storage_object(&self, id: &Id, category: &str, tag: &str) -> Result<()> {
        let input = DetachTagFromVStorageObjectRequestType {id, category, tag, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/DetachTagFromVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn extend_disk_task(&self, id: &Id, datastore: &ManagedObjectReference, new_capacity_in_mb: i64) -> Result<ManagedObjectReference> {
        let input = ExtendDiskRequestType {id, datastore, new_capacity_in_mb, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/ExtendDisk_Task", moId = &self.mo_id);
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
        let path = format!("/VcenterVStorageObjectManager/{moId}/VStorageObjectExtendDiskEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn inflate_disk_task(&self, id: &Id, datastore: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = InflateDiskRequestType {id, datastore, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/InflateDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Lists all tags attached to virtual storage object.
    /// 
    /// ***Required privileges:*** System.Read
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
    pub async fn list_tags_attached_to_v_storage_object(&self, id: &Id) -> Result<Option<Vec<VslmTagEntry>>> {
        let input = ListTagsAttachedToVStorageObjectRequestType {id, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/ListTagsAttachedToVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    /// datastore, such as datastore cannot be found or
    /// is inaccessible.
    pub async fn list_v_storage_object(&self, datastore: &ManagedObjectReference) -> Result<Option<Vec<Id>>> {
        let input = ListVStorageObjectRequestType {datastore, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/ListVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Lists all virtual storage objects attached to the tag.
    /// 
    /// ***Required privileges:*** System.Read
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
    pub async fn list_v_storage_objects_attached_to_tag(&self, category: &str, tag: &str) -> Result<Option<Vec<Id>>> {
        let input = ListVStorageObjectsAttachedToTagRequestType {category, tag, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/ListVStorageObjectsAttachedToTag", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn vstorage_object_v_center_query_changed_disk_areas(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: &Id, start_offset: i64, change_id: &str) -> Result<DiskChangeInfo> {
        let input = VstorageObjectVCenterQueryChangedDiskAreasRequestType {id, datastore, snapshot_id, start_offset, change_id, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/VstorageObjectVCenterQueryChangedDiskAreas", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn reconcile_datastore_inventory_task(&self, datastore: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = ReconcileDatastoreInventoryRequestType {datastore, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/ReconcileDatastoreInventory_Task", moId = &self.mo_id);
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
    /// URL path to the virtual disk.
    ///
    /// ### name
    /// The descriptive name of the disk object. If
    /// unset the name will be automatically determined
    /// from the path. @see vim.vslm.BaseConfigInfo#name
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
    pub async fn register_disk(&self, path: &str, name: Option<&str>) -> Result<VStorageObject> {
        let input = RegisterDiskRequestType {path, name, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/RegisterDisk", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Relocate a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on both source and
    /// destination datastore.
    /// 
    /// Supports only detached virtual storage object.
    /// Requires a host that has access to both source and destination datastores.
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
    pub async fn relocate_v_storage_object_task(&self, id: &Id, datastore: &ManagedObjectReference, spec: &VslmRelocateSpec) -> Result<ManagedObjectReference> {
        let input = RelocateVStorageObjectRequestType {id, datastore, spec, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/RelocateVStorageObject_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Rename a virtual storage object.
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
    pub async fn rename_v_storage_object(&self, id: &Id, datastore: &ManagedObjectReference, name: &str) -> Result<()> {
        let input = RenameVStorageObjectRequestType {id, datastore, name, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/RenameVStorageObject", moId = &self.mo_id);
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
        let path = format!("/VcenterVStorageObjectManager/{moId}/RenameVStorageObjectEx", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieves snapshot disk details of a given snapshot.
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
    pub async fn retrieve_snapshot_details(&self, id: &Id, datastore: &ManagedObjectReference, snapshot_id: &Id) -> Result<VStorageObjectSnapshotDetails> {
        let input = RetrieveSnapshotDetailsRequestType {id, datastore, snapshot_id, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/RetrieveSnapshotDetails", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieves snapshot information of a given VStorageObject.
    /// 
    /// Requires Datastore.FileManagement privilege.
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
    pub async fn retrieve_snapshot_info(&self, id: &Id, datastore: &ManagedObjectReference) -> Result<VStorageObjectSnapshotInfo> {
        let input = RetrieveSnapshotInfoRequestType {id, datastore, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/RetrieveSnapshotInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieve virtual storage infrastructure object SBPM policy on given
    /// datastore.
    /// 
    /// Only support VSAN datastore.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage infrastructure object is located.
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
    pub async fn retrieve_v_storage_infrastructure_object_policy(&self, datastore: &ManagedObjectReference) -> Result<Option<Vec<VslmInfrastructureObjectPolicy>>> {
        let input = RetrieveVStorageInfrastructureObjectPolicyRequestType {datastore, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/RetrieveVStorageInfrastructureObjectPolicy", moId = &self.mo_id);
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
    pub async fn retrieve_v_storage_object(&self, id: &Id, datastore: &ManagedObjectReference, disk_info_flags: Option<&[String]>) -> Result<VStorageObject> {
        let input = RetrieveVStorageObjectRequestType {id, datastore, disk_info_flags, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/RetrieveVStorageObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieve vm associations for each virtual storage object in the query.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### ids
    /// The IDs of the virtual storage objects of the query.
    ///
    /// ## Returns:
    ///
    /// The list of VStorageObjectVmAssociations which provides virtual
    /// storage object id to vm associations mapping.
    pub async fn retrieve_v_storage_object_associations(&self, ids: Option<&[RetrieveVStorageObjSpec]>) -> Result<Option<Vec<VStorageObjectAssociations>>> {
        let input = RetrieveVStorageObjectAssociationsRequestType {ids, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/RetrieveVStorageObjectAssociations", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn retrieve_v_storage_object_state(&self, id: &Id, datastore: &ManagedObjectReference) -> Result<VStorageObjectStateInfo> {
        let input = RetrieveVStorageObjectStateRequestType {id, datastore, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/RetrieveVStorageObjectState", moId = &self.mo_id);
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
        let path = format!("/VcenterVStorageObjectManager/{moId}/RevertVStorageObjectEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn schedule_reconcile_datastore_inventory(&self, datastore: &ManagedObjectReference) -> Result<()> {
        let input = ScheduleReconcileDatastoreInventoryRequestType {datastore, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/ScheduleReconcileDatastoreInventory", moId = &self.mo_id);
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
    pub async fn set_v_storage_object_control_flags(&self, id: &Id, datastore: &ManagedObjectReference, control_flags: Option<&[String]>) -> Result<()> {
        let input = SetVStorageObjectControlFlagsRequestType {id, datastore, control_flags, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/SetVStorageObjectControlFlags", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Assigns specified SBPM policy to the given virtual storage
    /// infrastructure object.
    /// 
    /// Only support VSAN datastore.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage infrastructure object is located.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// specification to assign a SPBM policy to virtual storage
    /// infrastructure object.
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
    pub async fn update_v_storage_infrastructure_object_policy_task(&self, spec: &VslmInfrastructureObjectPolicySpec) -> Result<ManagedObjectReference> {
        let input = UpdateVStorageInfrastructureObjectPolicyRequestType {spec, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/UpdateVStorageInfrastructureObjectPolicy_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Update the crypto on a virtual storage object.
    /// 
    /// This is also intended for disk encryption, decryption and re-encryption.
    /// To encrypt the disk, profile must contain an encryption component.
    /// disksCrypto can be left as blank, which means caller doesn't care
    /// which key is used to encrypt the disk. If it's not blank, it has to
    /// be of type CryptoSpecEncrypt.
    /// To decrypt the disk, profile must not contain an encryption component.
    /// disksCrypto can be left as blank, if not, it has be of type
    /// CryptoSpecDecrypt.
    /// To re-encrypt the disk, profile must contain an encryption component.
    /// disksCrypto cannot be left as blank. It has to be of type either
    /// CryptoSpecShallowRecrypt or CryptoSpecDeepRecrypt.
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
    /// The datastore where the virtual storage object is
    /// located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn update_v_storage_object_crypto_task(&self, id: &Id, datastore: &ManagedObjectReference, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>, disks_crypto: Option<&DiskCryptoSpec>) -> Result<ManagedObjectReference> {
        let input = UpdateVStorageObjectCryptoRequestType {id, datastore, profile, disks_crypto, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/UpdateVStorageObjectCrypto_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Update metadata KV pairs to a virtual storage object and
    /// returns the corresponding vclock upon success.
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
    /// or is inaccessible.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***NotFound***: If specified virtual storage object cannot be found.
    pub async fn v_center_update_v_storage_object_metadata_ex_task(&self, id: &Id, datastore: &ManagedObjectReference, metadata: Option<&[KeyValue]>, delete_keys: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = VCenterUpdateVStorageObjectMetadataExRequestType {id, datastore, metadata, delete_keys, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/VCenterUpdateVStorageObjectMetadataEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Update the storage policy on a virtual storage object.
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
    /// The datastore where the virtual storage object is
    /// located.
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn update_v_storage_object_policy_task(&self, id: &Id, datastore: &ManagedObjectReference, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>) -> Result<ManagedObjectReference> {
        let input = UpdateVStorageObjectPolicyRequestType {id, datastore, profile, };
        let path = format!("/VcenterVStorageObjectManager/{moId}/UpdateVStorageObjectPolicy_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RevertVStorageObjectRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AttachTagToVStorageObjectRequestType<'a> {
    id: &'a Id,
    category: &'a str,
    tag: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ClearVStorageObjectControlFlagsRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "controlFlags")]
    control_flags: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CloneVStorageObjectRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    spec: &'a VslmCloneSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateDiskRequestType<'a> {
    spec: &'a VslmCreateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateDiskFromSnapshotRequestType<'a> {
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
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VStorageObjectCreateSnapshotRequestType<'a> {
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
struct DeleteSnapshotRequestType<'a> {
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
struct DeleteVStorageObjectRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteVStorageObjectExRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DetachTagFromVStorageObjectRequestType<'a> {
    id: &'a Id,
    category: &'a str,
    tag: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ExtendDiskRequestType<'a> {
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
struct InflateDiskRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListTagsAttachedToVStorageObjectRequestType<'a> {
    id: &'a Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListVStorageObjectRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListVStorageObjectsAttachedToTagRequestType<'a> {
    category: &'a str,
    tag: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VstorageObjectVCenterQueryChangedDiskAreasRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a Id,
    #[serde(rename = "startOffset")]
    start_offset: i64,
    #[serde(rename = "changeId")]
    change_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconcileDatastoreInventoryRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RegisterDiskRequestType<'a> {
    path: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RelocateVStorageObjectRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    spec: &'a VslmRelocateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameVStorageObjectRequestType<'a> {
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
struct RetrieveSnapshotDetailsRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "snapshotId")]
    snapshot_id: &'a Id,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveSnapshotInfoRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveVStorageInfrastructureObjectPolicyRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveVStorageObjectRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "diskInfoFlags")]
    disk_info_flags: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveVStorageObjectAssociationsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ids: Option<&'a [RetrieveVStorageObjSpec]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveVStorageObjectStateRequestType<'a> {
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
struct ScheduleReconcileDatastoreInventoryRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetVStorageObjectControlFlagsRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "controlFlags")]
    control_flags: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateVStorageInfrastructureObjectPolicyRequestType<'a> {
    spec: &'a VslmInfrastructureObjectPolicySpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateVStorageObjectCryptoRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "disksCrypto")]
    disks_crypto: Option<&'a DiskCryptoSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VCenterUpdateVStorageObjectMetadataExRequestType<'a> {
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
struct UpdateVStorageObjectPolicyRequestType<'a> {
    id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
}
