use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Interface to manage virtual storage object on a vCenter.
/// 
/// VStorageObjectManager and SPBM policy support:
/// All of the VStorageObjectManager APIs requiring ESXi host
/// uses "Programmatically selected" host to perform the actual operation.
/// If the selected host is of 6.5 version then policy would not be passed
/// down to host. In that case, user operation would succeed but if user checks
/// SPBM Entity Compliance, it will show "Mismatch" / "Non Compliant" as a
/// compliance result.
#[derive(Clone)]
pub struct VcenterVStorageObjectManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VcenterVStorageObjectManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn revert_v_storage_object_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: &crate::types::structs::Id) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RevertVStorageObjectRequestType {id, datastore, snapshot_id, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "RevertVStorageObject_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn attach_tag_to_v_storage_object(&self, id: &crate::types::structs::Id, category: &str, tag: &str) -> Result<()> {
        let input = AttachTagToVStorageObjectRequestType {id, category, tag, };
        self.client.invoke_void("", "VcenterVStorageObjectManager", &self.mo_id, "AttachTagToVStorageObject", Some(&input)).await
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
    pub async fn clear_v_storage_object_control_flags(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, control_flags: Option<&[String]>) -> Result<()> {
        let input = ClearVStorageObjectControlFlagsRequestType {id, datastore, control_flags, };
        self.client.invoke_void("", "VcenterVStorageObjectManager", &self.mo_id, "ClearVStorageObjectControlFlags", Some(&input)).await
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
    pub async fn clone_v_storage_object_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, spec: &crate::types::structs::VslmCloneSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CloneVStorageObjectRequestType {id, datastore, spec, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "CloneVStorageObject_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn create_disk_task(&self, spec: &crate::types::structs::VslmCreateSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateDiskRequestType {spec, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "CreateDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// if snapshto is unencrypted, then crypto will be of
    /// type CryptoSpecEncrypt, and filled with keyId that is
    /// automatically generated and keyProviderId that is the
    /// default kms cluster.
    /// If unset and if profile is a default policy and if snapshot
    /// is unenrypted, then crypto is treated as CryptoSpecNoOp.
    /// If unset and if profile contains an encryption iofilter and
    /// if snapshot is encrypted, then crypto is treated as
    /// CryptoSpecNoOp.
    /// If unset and if profile is a default policy and if
    /// snapshot is encrypted, then crypto is treated as
    /// CryptoSpecDecrypt.
    /// To recrypt the disk during creating disk, crypto has to be
    /// present.
    ///
    /// ### path
    /// Relative location where disk has to be created, used in
    /// `targetDatastore` and `datastore` parameters.
    /// If not specified disk gets created at default *VStorageObject*
    /// location of `targetDatastore` or `datastore`.
    ///
    /// ### is_linked_clone
    /// Indicates whether a linkedClone Disk needs to be created
    /// from the snapshot.
    /// 
    /// ***Since:*** vSphere API Release 9.1.0.0
    ///
    /// ### target_id
    /// The ID of the target virtual storage object.
    /// For each new request, the ID should be a unique ID
    /// complying to RFC4122V4 (generated randomly).
    /// For retry requests with the same ID, all the other input
    /// parameters are expected to remain same.
    /// If not specified, a system generated ID will be assigned
    /// to the newly created virtual storage object.
    /// 
    /// ***Since:*** vSphere API Release 9.1.0.0
    ///
    /// ### target_datastore
    /// The target datastore where the new disk needs to be created.
    /// if not specified, the new disk will be created where
    /// the source virtual storage object is located.
    /// 
    /// ***Since:*** vSphere API Release 9.1.0.0
    /// 
    /// Refers instance of *Datastore*.
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
    pub async fn create_disk_from_snapshot_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: &crate::types::structs::Id, name: &str, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>, crypto: Option<&dyn crate::types::traits::CryptoSpecTrait>, path: Option<&str>, is_linked_clone: Option<bool>, target_id: Option<&crate::types::structs::Id>, target_datastore: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateDiskFromSnapshotRequestType {id, datastore, snapshot_id, name, profile, crypto, path, is_linked_clone, target_id, target_datastore, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "CreateDiskFromSnapshot_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn v_storage_object_create_snapshot_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, description: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VStorageObjectCreateSnapshotRequestType {id, datastore, description, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "VStorageObjectCreateSnapshot_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// ### snapshot_id
    /// The ID of the snapshot of the virtual storage object,
    /// For each new request, the ID should be a unique ID
    /// complying to RFC4122V4 (generated randomly).
    /// For retry requests with the same ID, all the other input
    /// parameters are expected to remain same.
    /// If not specified, a random system generated snapshot ID
    /// will be assigned to the snapshot.
    /// 
    /// ***Since:*** vSphere API Release 9.1.0.0
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
    pub async fn v_storage_object_create_snapshot_ex_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, description: &str, snapshot_id: Option<&crate::types::structs::Id>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VStorageObjectCreateSnapshotExRequestType {id, datastore, description, snapshot_id, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "VStorageObjectCreateSnapshotEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn delete_snapshot_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: &crate::types::structs::Id) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DeleteSnapshotRequestType {id, datastore, snapshot_id, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "DeleteSnapshot_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn v_storage_object_delete_snapshot_ex_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: &crate::types::structs::Id) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VStorageObjectDeleteSnapshotExRequestType {id, datastore, snapshot_id, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "VStorageObjectDeleteSnapshotEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deletes a given snapshot of a VStorageObject.
    /// 
    /// Requires Datastore.FileManagement privilege.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
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
    /// ***NotFound***: If specified virtual storage object or snapshot cannot be found.
    pub async fn v_storage_object_delete_snapshot_ex_2_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: &crate::types::structs::Id) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VStorageObjectDeleteSnapshotEx2RequestType {id, datastore, snapshot_id, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "VStorageObjectDeleteSnapshotEx2_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// ***Required privileges:*** Datastore.FileManagement
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
    pub async fn delete_v_storage_object_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DeleteVStorageObjectRequestType {id, datastore, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "DeleteVStorageObject_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Delete a virtual storage object and its associated backings.
    /// 
    /// Returns
    /// the corresponding vclock upon success.
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
    pub async fn delete_v_storage_object_ex_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DeleteVStorageObjectExRequestType {id, datastore, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "DeleteVStorageObjectEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn detach_tag_from_v_storage_object(&self, id: &crate::types::structs::Id, category: &str, tag: &str) -> Result<()> {
        let input = DetachTagFromVStorageObjectRequestType {id, category, tag, };
        self.client.invoke_void("", "VcenterVStorageObjectManager", &self.mo_id, "DetachTagFromVStorageObject", Some(&input)).await
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
    pub async fn extend_disk_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, new_capacity_in_mb: i64) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ExtendDiskRequestType {id, datastore, new_capacity_in_mb, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "ExtendDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn v_storage_object_extend_disk_ex_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, new_capacity_in_mb: i64) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VStorageObjectExtendDiskExRequestType {id, datastore, new_capacity_in_mb, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "VStorageObjectExtendDiskEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn inflate_disk_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = InflateDiskRequestType {id, datastore, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "InflateDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn list_tags_attached_to_v_storage_object(&self, id: &crate::types::structs::Id) -> Result<Option<Vec<crate::types::structs::VslmTagEntry>>> {
        let input = ListTagsAttachedToVStorageObjectRequestType {id, };
        let bytes_opt = self.client.invoke_optional("", "VcenterVStorageObjectManager", &self.mo_id, "ListTagsAttachedToVStorageObject", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn list_v_storage_object(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::Id>>> {
        let input = ListVStorageObjectRequestType {datastore, };
        let bytes_opt = self.client.invoke_optional("", "VcenterVStorageObjectManager", &self.mo_id, "ListVStorageObject", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn list_v_storage_objects_attached_to_tag(&self, category: &str, tag: &str) -> Result<Option<Vec<crate::types::structs::Id>>> {
        let input = ListVStorageObjectsAttachedToTagRequestType {category, tag, };
        let bytes_opt = self.client.invoke_optional("", "VcenterVStorageObjectManager", &self.mo_id, "ListVStorageObjectsAttachedToTag", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    /// that have changed since the time the changeId string was
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
    pub async fn vstorage_object_v_center_query_changed_disk_areas(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: &crate::types::structs::Id, start_offset: i64, change_id: &str) -> Result<crate::types::structs::DiskChangeInfo> {
        let input = VstorageObjectVCenterQueryChangedDiskAreasRequestType {id, datastore, snapshot_id, start_offset, change_id, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "VstorageObjectVCenterQueryChangedDiskAreas", Some(&input)).await?;
        let result: crate::types::structs::DiskChangeInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Get the virtual disk UUID.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where virtualDisk is present.
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// A URL has the form
    /// > _scheme_://_authority_/folder/_path_?dcPath=_dcPath_&amp;dsName=_dsName_
    /// 
    /// where
    /// - _scheme_ is <code>http</code> or <code>https</code>.
    /// - _authority_ specifies the hostname or IP address of the VirtualCenter or
    ///   ESX server and optionally the port.
    /// - _dcPath_ is the inventory path to the Datacenter containing the
    ///   Datastore.
    /// - _dsName_ is the name of the Datastore.
    /// - _path_ is a slash-delimited path from the root of the datastore.
    /// 
    /// ***Since:*** vSphere API Release 8.0.3.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk whose uuid for the DDB entry needs to be queried.
    /// A datastore path has the form
    /// > \[_datastore_\] _path_
    /// 
    /// where
    /// - _datastore_ is the datastore name.
    /// - _path_ is a slash-delimited path from the root of the datastore.
    /// 
    /// An example datastore path is "\[storage\] path/to/file.extension".
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path is mandatory. Not needed when invoked directly on ESX.
    /// If not specified on a call from VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ## Returns:
    ///
    /// The hex representation of the unique ID for this virtual disk.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: if an error occurs reading the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn query_virtual_disk_uuid_ex(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>) -> Result<String> {
        let input = QueryVirtualDiskUuidExRequestType {name, datacenter, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "QueryVirtualDiskUuidEx", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// ### deep_cleansing
    /// If set true, the reconcile task will check for the
    /// extent files and the disk descriptor file content
    /// as part of reconciliation. Note that this is a time
    /// consuming process.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on
    /// the datastore.
    pub async fn reconcile_datastore_inventory_task(&self, datastore: &crate::types::structs::ManagedObjectReference, deep_cleansing: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ReconcileDatastoreInventoryRequestType {datastore, deep_cleansing, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "ReconcileDatastoreInventory_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Reconciles/scans datastore for the virtual storage objects and returns
    /// the result.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The specification to reconcile/scan a datastore for virtual
    /// storage objects.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. The *info.result*
    /// property in the *Task* contains the result
    /// *VStorageObjectReconcileResult* upon success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: If an error occurs while reconciling the virtual
    /// storage object.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***InvalidState***: If the operation cannot be performed on the disk.
    /// 
    /// ***InvalidArgument***: If there is invalid argument in
    /// *VStorageObjectReconcileSpec*.
    pub async fn reconcile_datastore_inventory_ex_task(&self, spec: &crate::types::structs::VStorageObjectReconcileSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ReconcileDatastoreInventoryExRequestType {spec, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "ReconcileDatastoreInventoryEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// URL path to the running point of the virtual disk.
    ///
    /// ### name
    /// The descriptive name of the disk object. If
    /// unset the name will be automatically determined
    /// from the path. @see vim.vslm.BaseConfigInfo#name
    ///
    /// ### id
    /// The ID of the newly registered virtual storage object.
    /// For each new request, the ID should be a unique ID complying
    /// to RFC4122V4 (generated randomly).
    /// For retry requests with the same ID, all the other input
    /// parameters are expected to remain same.
    /// If not specified, a system generated ID will be assigned.
    /// 
    /// ***Since:*** vSphere API Release 9.1.0.0
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
    pub async fn register_disk(&self, path: &str, name: Option<&str>, id: Option<&crate::types::structs::Id>) -> Result<crate::types::structs::VStorageObject> {
        let input = RegisterDiskRequestType {path, name, id, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "RegisterDisk", Some(&input)).await?;
        let result: crate::types::structs::VStorageObject = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Relocate a virtual storage object.
    /// 
    /// Requires Datastore.FileManagement privilege on both source and
    /// destination datastore.
    /// 
    /// If there is no host that has access to both source and destination datastore,
    /// then limited number of concurrent relocations are supported. This number is
    /// set to 10.
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
    pub async fn relocate_v_storage_object_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, spec: &crate::types::structs::VslmRelocateSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RelocateVStorageObjectRequestType {id, datastore, spec, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "RelocateVStorageObject_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn rename_v_storage_object(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, name: &str) -> Result<()> {
        let input = RenameVStorageObjectRequestType {id, datastore, name, };
        self.client.invoke_void("", "VcenterVStorageObjectManager", &self.mo_id, "RenameVStorageObject", Some(&input)).await
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
    pub async fn rename_v_storage_object_ex(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, name: &str) -> Result<crate::types::structs::VslmVClockInfo> {
        let input = RenameVStorageObjectExRequestType {id, datastore, name, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "RenameVStorageObjectEx", Some(&input)).await?;
        let result: crate::types::structs::VslmVClockInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Repair a virtual disk having broken chain.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where each
    /// virtual disk resides.
    /// 
    /// ***Since:*** vSphere API Release 9.1.0.0
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual disk to be repaired.
    ///
    /// ### datastore
    /// The datastore where the virtual disk is located.
    /// 
    /// ***Required privileges:*** Datastore.FileManagement
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
    /// ***FileFault***: If an error occurs while repairing the virtual disk.
    /// 
    /// ***NotFound***: If the specified virtual storage object cannot be found.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***TaskInProgress***: If the virtual storage object is busy.
    pub async fn repair_v_storage_object_chain_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RepairVStorageObjectChainRequestType {id, datastore, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "RepairVStorageObjectChain_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn retrieve_snapshot_details(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: &crate::types::structs::Id) -> Result<crate::types::structs::VStorageObjectSnapshotDetails> {
        let input = RetrieveSnapshotDetailsRequestType {id, datastore, snapshot_id, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "RetrieveSnapshotDetails", Some(&input)).await?;
        let result: crate::types::structs::VStorageObjectSnapshotDetails = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn retrieve_snapshot_info(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::VStorageObjectSnapshotInfo> {
        let input = RetrieveSnapshotInfoRequestType {id, datastore, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "RetrieveSnapshotInfo", Some(&input)).await?;
        let result: crate::types::structs::VStorageObjectSnapshotInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn retrieve_v_storage_infrastructure_object_policy(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::VslmInfrastructureObjectPolicy>>> {
        let input = RetrieveVStorageInfrastructureObjectPolicyRequestType {datastore, };
        let bytes_opt = self.client.invoke_optional("", "VcenterVStorageObjectManager", &self.mo_id, "RetrieveVStorageInfrastructureObjectPolicy", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn retrieve_v_storage_object(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, disk_info_flags: Option<&[String]>) -> Result<crate::types::structs::VStorageObject> {
        let input = RetrieveVStorageObjectRequestType {id, datastore, disk_info_flags, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "RetrieveVStorageObject", Some(&input)).await?;
        let result: crate::types::structs::VStorageObject = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Retrieve vm associations for each virtual storage object in the query.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual storage object is located.
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
    pub async fn retrieve_v_storage_object_associations(&self, ids: Option<&[crate::types::structs::RetrieveVStorageObjSpec]>) -> Result<Option<Vec<crate::types::structs::VStorageObjectAssociations>>> {
        let input = RetrieveVStorageObjectAssociationsRequestType {ids, };
        let bytes_opt = self.client.invoke_optional("", "VcenterVStorageObjectManager", &self.mo_id, "RetrieveVStorageObjectAssociations", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn retrieve_v_storage_object_state(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::VStorageObjectStateInfo> {
        let input = RetrieveVStorageObjectStateRequestType {id, datastore, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "RetrieveVStorageObjectState", Some(&input)).await?;
        let result: crate::types::structs::VStorageObjectStateInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn revert_v_storage_object_ex_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: &crate::types::structs::Id) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RevertVStorageObjectExRequestType {id, datastore, snapshot_id, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "RevertVStorageObjectEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// ### deep_cleansing
    /// If set true, the reconcile task will check for the
    /// extent files and the disk descriptor file content
    /// as part of reconciliation. Note that this is a time
    /// consuming process.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on
    /// the datastore.
    pub async fn schedule_reconcile_datastore_inventory(&self, datastore: &crate::types::structs::ManagedObjectReference, deep_cleansing: Option<bool>) -> Result<()> {
        let input = ScheduleReconcileDatastoreInventoryRequestType {datastore, deep_cleansing, };
        self.client.invoke_void("", "VcenterVStorageObjectManager", &self.mo_id, "ScheduleReconcileDatastoreInventory", Some(&input)).await
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
    pub async fn set_v_storage_object_control_flags(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, control_flags: Option<&[String]>) -> Result<()> {
        let input = SetVStorageObjectControlFlagsRequestType {id, datastore, control_flags, };
        self.client.invoke_void("", "VcenterVStorageObjectManager", &self.mo_id, "SetVStorageObjectControlFlags", Some(&input)).await
    }
    /// Set the virtual disk Uuid.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where virtualDisk is present.
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// A URL has the form
    /// > _scheme_://_authority_/folder/_path_?dcPath=_dcPath_&amp;dsName=_dsName_
    /// 
    /// where
    /// - _scheme_ is <code>http</code> or <code>https</code>.
    /// - _authority_ specifies the hostname or IP address of the VirtualCenter or
    ///   ESX server and optionally the port.
    /// - _dcPath_ is the inventory path to the Datacenter containing the
    ///   Datastore.
    /// - _dsName_ is the name of the Datastore.
    /// - _path_ is a slash-delimited path from the root of the datastore.
    /// 
    /// ***Since:*** vSphere API Release 8.0.3.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk whose uuid for the DDB entry needs to be set.
    /// A datastore path has the form
    /// > \[_datastore_\] _path_
    /// 
    /// where
    /// - _datastore_ is the datastore name.
    /// - _path_ is a slash-delimited path from the root of the datastore.
    /// 
    /// An example datastore path is "\[storage\] path/to/file.extension".
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path is mandatory. Not needed when invoked directly on ESX.
    /// If not specified on a call from VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### uuid
    /// The hex representation of the unique ID for this virtual disk. If uuid is not set or missing,
    /// a random UUID is generated and assigned.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: if an error occurs updating the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn set_virtual_disk_uuid_ex_task(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>, uuid: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = SetVirtualDiskUuidExRequestType {name, datacenter, uuid, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "SetVirtualDiskUuidEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Convert FCD disk to legacy disk.
    /// 
    /// ***Since:*** vSphere API Release 9.1.0.0
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The ID of the virtual storage object which
    /// needs to be unregistered.
    ///
    /// ### datastore
    /// The datastore where the virtual storage object is
    /// located.
    /// 
    /// ***Required privileges:*** Datastore.FileManagement
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
    /// ***InvalidState***: If unregister disk operation could not be performed.
    /// 
    /// ***NotFound***: If a given vstorage object id was not found.
    /// 
    /// ***InvalidDatastore***: If the operation cannot be performed on the datastore.
    /// 
    /// ***TaskInProgress***: If the virtual storage object is busy.
    /// 
    /// ***NotSupported***: If operation is not supported because of some underlying condition.
    pub async fn unregister_disk_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UnregisterDiskRequestType {id, datastore, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "UnregisterDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn update_v_storage_infrastructure_object_policy_task(&self, spec: &crate::types::structs::VslmInfrastructureObjectPolicySpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UpdateVStorageInfrastructureObjectPolicyRequestType {spec, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "UpdateVStorageInfrastructureObjectPolicy_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn update_v_storage_object_crypto_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>, disks_crypto: Option<&crate::types::structs::DiskCryptoSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UpdateVStorageObjectCryptoRequestType {id, datastore, profile, disks_crypto, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "UpdateVStorageObjectCrypto_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// ***Required privileges:*** Datastore.FileManagement
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
    pub async fn v_center_update_v_storage_object_metadata_ex_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, metadata: Option<&[crate::types::structs::KeyValue]>, delete_keys: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VCenterUpdateVStorageObjectMetadataExRequestType {id, datastore, metadata, delete_keys, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "VCenterUpdateVStorageObjectMetadataEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn update_v_storage_object_policy_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UpdateVStorageObjectPolicyRequestType {id, datastore, profile, };
        let bytes = self.client.invoke("", "VcenterVStorageObjectManager", &self.mo_id, "UpdateVStorageObjectPolicy_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct RevertVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: &'a crate::types::structs::Id,
}

impl<'a> miniserde::Serialize for RevertVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RevertVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RevertVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b RevertVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RevertVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RevertVStorageObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("snapshotId"), &self.data.snapshot_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AttachTagToVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    category: &'a str,
    tag: &'a str,
}

impl<'a> miniserde::Serialize for AttachTagToVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AttachTagToVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AttachTagToVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b AttachTagToVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AttachTagToVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AttachTagToVStorageObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("category"), &self.data.category as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("tag"), &self.data.tag as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ClearVStorageObjectControlFlagsRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    control_flags: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for ClearVStorageObjectControlFlagsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ClearVStorageObjectControlFlagsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ClearVStorageObjectControlFlagsRequestTypeSer<'b, 'a> {
    data: &'b ClearVStorageObjectControlFlagsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ClearVStorageObjectControlFlagsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ClearVStorageObjectControlFlagsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.control_flags else { continue; };
                    return Some((std::borrow::Cow::Borrowed("controlFlags"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CloneVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::VslmCloneSpec,
}

impl<'a> miniserde::Serialize for CloneVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CloneVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CloneVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b CloneVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CloneVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CloneVStorageObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateDiskRequestType<'a> {
    spec: &'a crate::types::structs::VslmCreateSpec,
}

impl<'a> miniserde::Serialize for CreateDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateDiskRequestTypeSer<'b, 'a> {
    data: &'b CreateDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateDiskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateDiskFromSnapshotRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: &'a crate::types::structs::Id,
    name: &'a str,
    profile: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
    crypto: Option<&'a dyn crate::types::traits::CryptoSpecTrait>,
    path: Option<&'a str>,
    is_linked_clone: Option<bool>,
    target_id: Option<&'a crate::types::structs::Id>,
    target_datastore: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for CreateDiskFromSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateDiskFromSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateDiskFromSnapshotRequestTypeSer<'b, 'a> {
    data: &'b CreateDiskFromSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateDiskFromSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateDiskFromSnapshotRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("snapshotId"), &self.data.snapshot_id as &dyn miniserde::Serialize)),
                4 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                5 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.crypto else { continue; };
                    return Some((std::borrow::Cow::Borrowed("crypto"), val as &dyn miniserde::Serialize));
                }
                7 => {
                    let Some(ref val) = self.data.path else { continue; };
                    return Some((std::borrow::Cow::Borrowed("path"), val as &dyn miniserde::Serialize));
                }
                8 => {
                    let Some(ref val) = self.data.is_linked_clone else { continue; };
                    return Some((std::borrow::Cow::Borrowed("isLinkedClone"), val as &dyn miniserde::Serialize));
                }
                9 => {
                    let Some(ref val) = self.data.target_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targetId"), val as &dyn miniserde::Serialize));
                }
                10 => {
                    let Some(ref val) = self.data.target_datastore else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targetDatastore"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VStorageObjectCreateSnapshotRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    description: &'a str,
}

impl<'a> miniserde::Serialize for VStorageObjectCreateSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VStorageObjectCreateSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VStorageObjectCreateSnapshotRequestTypeSer<'b, 'a> {
    data: &'b VStorageObjectCreateSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VStorageObjectCreateSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VStorageObjectCreateSnapshotRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("description"), &self.data.description as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VStorageObjectCreateSnapshotExRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    description: &'a str,
    snapshot_id: Option<&'a crate::types::structs::Id>,
}

impl<'a> miniserde::Serialize for VStorageObjectCreateSnapshotExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VStorageObjectCreateSnapshotExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VStorageObjectCreateSnapshotExRequestTypeSer<'b, 'a> {
    data: &'b VStorageObjectCreateSnapshotExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VStorageObjectCreateSnapshotExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VStorageObjectCreateSnapshotExRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("description"), &self.data.description as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.snapshot_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("snapshotId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct DeleteSnapshotRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: &'a crate::types::structs::Id,
}

impl<'a> miniserde::Serialize for DeleteSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteSnapshotRequestTypeSer<'b, 'a> {
    data: &'b DeleteSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeleteSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteSnapshotRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("snapshotId"), &self.data.snapshot_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VStorageObjectDeleteSnapshotExRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: &'a crate::types::structs::Id,
}

impl<'a> miniserde::Serialize for VStorageObjectDeleteSnapshotExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VStorageObjectDeleteSnapshotExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VStorageObjectDeleteSnapshotExRequestTypeSer<'b, 'a> {
    data: &'b VStorageObjectDeleteSnapshotExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VStorageObjectDeleteSnapshotExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VStorageObjectDeleteSnapshotExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("snapshotId"), &self.data.snapshot_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VStorageObjectDeleteSnapshotEx2RequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: &'a crate::types::structs::Id,
}

impl<'a> miniserde::Serialize for VStorageObjectDeleteSnapshotEx2RequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VStorageObjectDeleteSnapshotEx2RequestTypeSer { data: self, seq: 0 }))
    }
}

struct VStorageObjectDeleteSnapshotEx2RequestTypeSer<'b, 'a> {
    data: &'b VStorageObjectDeleteSnapshotEx2RequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VStorageObjectDeleteSnapshotEx2RequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VStorageObjectDeleteSnapshotEx2RequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("snapshotId"), &self.data.snapshot_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DeleteVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for DeleteVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b DeleteVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeleteVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteVStorageObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DeleteVStorageObjectExRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for DeleteVStorageObjectExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteVStorageObjectExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteVStorageObjectExRequestTypeSer<'b, 'a> {
    data: &'b DeleteVStorageObjectExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeleteVStorageObjectExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteVStorageObjectExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DetachTagFromVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    category: &'a str,
    tag: &'a str,
}

impl<'a> miniserde::Serialize for DetachTagFromVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DetachTagFromVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DetachTagFromVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b DetachTagFromVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DetachTagFromVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DetachTagFromVStorageObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("category"), &self.data.category as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("tag"), &self.data.tag as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ExtendDiskRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    new_capacity_in_mb: i64,
}

impl<'a> miniserde::Serialize for ExtendDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ExtendDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ExtendDiskRequestTypeSer<'b, 'a> {
    data: &'b ExtendDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ExtendDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ExtendDiskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("newCapacityInMB"), &self.data.new_capacity_in_mb as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VStorageObjectExtendDiskExRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    new_capacity_in_mb: i64,
}

impl<'a> miniserde::Serialize for VStorageObjectExtendDiskExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VStorageObjectExtendDiskExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VStorageObjectExtendDiskExRequestTypeSer<'b, 'a> {
    data: &'b VStorageObjectExtendDiskExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VStorageObjectExtendDiskExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VStorageObjectExtendDiskExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("newCapacityInMB"), &self.data.new_capacity_in_mb as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct InflateDiskRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for InflateDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(InflateDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct InflateDiskRequestTypeSer<'b, 'a> {
    data: &'b InflateDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for InflateDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"InflateDiskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ListTagsAttachedToVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
}

impl<'a> miniserde::Serialize for ListTagsAttachedToVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListTagsAttachedToVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListTagsAttachedToVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b ListTagsAttachedToVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ListTagsAttachedToVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListTagsAttachedToVStorageObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ListVStorageObjectRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for ListVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b ListVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ListVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListVStorageObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ListVStorageObjectsAttachedToTagRequestType<'a> {
    category: &'a str,
    tag: &'a str,
}

impl<'a> miniserde::Serialize for ListVStorageObjectsAttachedToTagRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListVStorageObjectsAttachedToTagRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListVStorageObjectsAttachedToTagRequestTypeSer<'b, 'a> {
    data: &'b ListVStorageObjectsAttachedToTagRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ListVStorageObjectsAttachedToTagRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListVStorageObjectsAttachedToTagRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("category"), &self.data.category as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("tag"), &self.data.tag as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VstorageObjectVCenterQueryChangedDiskAreasRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: &'a crate::types::structs::Id,
    start_offset: i64,
    change_id: &'a str,
}

impl<'a> miniserde::Serialize for VstorageObjectVCenterQueryChangedDiskAreasRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VstorageObjectVCenterQueryChangedDiskAreasRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VstorageObjectVCenterQueryChangedDiskAreasRequestTypeSer<'b, 'a> {
    data: &'b VstorageObjectVCenterQueryChangedDiskAreasRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VstorageObjectVCenterQueryChangedDiskAreasRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VstorageObjectVCenterQueryChangedDiskAreasRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("snapshotId"), &self.data.snapshot_id as &dyn miniserde::Serialize)),
            4 => return Some((std::borrow::Cow::Borrowed("startOffset"), &self.data.start_offset as &dyn miniserde::Serialize)),
            5 => return Some((std::borrow::Cow::Borrowed("changeId"), &self.data.change_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryVirtualDiskUuidExRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryVirtualDiskUuidExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVirtualDiskUuidExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVirtualDiskUuidExRequestTypeSer<'b, 'a> {
    data: &'b QueryVirtualDiskUuidExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryVirtualDiskUuidExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVirtualDiskUuidExRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ReconcileDatastoreInventoryRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
    deep_cleansing: Option<bool>,
}

impl<'a> miniserde::Serialize for ReconcileDatastoreInventoryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconcileDatastoreInventoryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconcileDatastoreInventoryRequestTypeSer<'b, 'a> {
    data: &'b ReconcileDatastoreInventoryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReconcileDatastoreInventoryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconcileDatastoreInventoryRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.deep_cleansing else { continue; };
                    return Some((std::borrow::Cow::Borrowed("deepCleansing"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ReconcileDatastoreInventoryExRequestType<'a> {
    spec: &'a crate::types::structs::VStorageObjectReconcileSpec,
}

impl<'a> miniserde::Serialize for ReconcileDatastoreInventoryExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconcileDatastoreInventoryExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconcileDatastoreInventoryExRequestTypeSer<'b, 'a> {
    data: &'b ReconcileDatastoreInventoryExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReconcileDatastoreInventoryExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconcileDatastoreInventoryExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RegisterDiskRequestType<'a> {
    path: &'a str,
    name: Option<&'a str>,
    id: Option<&'a crate::types::structs::Id>,
}

impl<'a> miniserde::Serialize for RegisterDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RegisterDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RegisterDiskRequestTypeSer<'b, 'a> {
    data: &'b RegisterDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RegisterDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RegisterDiskRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("path"), &self.data.path as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("name"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("id"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RelocateVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::VslmRelocateSpec,
}

impl<'a> miniserde::Serialize for RelocateVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RelocateVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RelocateVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b RelocateVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RelocateVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RelocateVStorageObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RenameVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    name: &'a str,
}

impl<'a> miniserde::Serialize for RenameVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RenameVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RenameVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b RenameVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RenameVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RenameVStorageObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RenameVStorageObjectExRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    name: &'a str,
}

impl<'a> miniserde::Serialize for RenameVStorageObjectExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RenameVStorageObjectExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RenameVStorageObjectExRequestTypeSer<'b, 'a> {
    data: &'b RenameVStorageObjectExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RenameVStorageObjectExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RenameVStorageObjectExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RepairVStorageObjectChainRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for RepairVStorageObjectChainRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RepairVStorageObjectChainRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RepairVStorageObjectChainRequestTypeSer<'b, 'a> {
    data: &'b RepairVStorageObjectChainRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RepairVStorageObjectChainRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RepairVStorageObjectChainRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveSnapshotDetailsRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: &'a crate::types::structs::Id,
}

impl<'a> miniserde::Serialize for RetrieveSnapshotDetailsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveSnapshotDetailsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveSnapshotDetailsRequestTypeSer<'b, 'a> {
    data: &'b RetrieveSnapshotDetailsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveSnapshotDetailsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveSnapshotDetailsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("snapshotId"), &self.data.snapshot_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveSnapshotInfoRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for RetrieveSnapshotInfoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveSnapshotInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveSnapshotInfoRequestTypeSer<'b, 'a> {
    data: &'b RetrieveSnapshotInfoRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveSnapshotInfoRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveSnapshotInfoRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveVStorageInfrastructureObjectPolicyRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for RetrieveVStorageInfrastructureObjectPolicyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveVStorageInfrastructureObjectPolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveVStorageInfrastructureObjectPolicyRequestTypeSer<'b, 'a> {
    data: &'b RetrieveVStorageInfrastructureObjectPolicyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveVStorageInfrastructureObjectPolicyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveVStorageInfrastructureObjectPolicyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    disk_info_flags: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for RetrieveVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b RetrieveVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveVStorageObjectRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.disk_info_flags else { continue; };
                    return Some((std::borrow::Cow::Borrowed("diskInfoFlags"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RetrieveVStorageObjectAssociationsRequestType<'a> {
    ids: Option<&'a [crate::types::structs::RetrieveVStorageObjSpec]>,
}

impl<'a> miniserde::Serialize for RetrieveVStorageObjectAssociationsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveVStorageObjectAssociationsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveVStorageObjectAssociationsRequestTypeSer<'b, 'a> {
    data: &'b RetrieveVStorageObjectAssociationsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveVStorageObjectAssociationsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveVStorageObjectAssociationsRequestType")),
                1 => {
                    let Some(ref val) = self.data.ids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("ids"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RetrieveVStorageObjectStateRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for RetrieveVStorageObjectStateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveVStorageObjectStateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveVStorageObjectStateRequestTypeSer<'b, 'a> {
    data: &'b RetrieveVStorageObjectStateRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveVStorageObjectStateRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveVStorageObjectStateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RevertVStorageObjectExRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: &'a crate::types::structs::Id,
}

impl<'a> miniserde::Serialize for RevertVStorageObjectExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RevertVStorageObjectExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RevertVStorageObjectExRequestTypeSer<'b, 'a> {
    data: &'b RevertVStorageObjectExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RevertVStorageObjectExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RevertVStorageObjectExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("snapshotId"), &self.data.snapshot_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ScheduleReconcileDatastoreInventoryRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
    deep_cleansing: Option<bool>,
}

impl<'a> miniserde::Serialize for ScheduleReconcileDatastoreInventoryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ScheduleReconcileDatastoreInventoryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ScheduleReconcileDatastoreInventoryRequestTypeSer<'b, 'a> {
    data: &'b ScheduleReconcileDatastoreInventoryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ScheduleReconcileDatastoreInventoryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ScheduleReconcileDatastoreInventoryRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.deep_cleansing else { continue; };
                    return Some((std::borrow::Cow::Borrowed("deepCleansing"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct SetVStorageObjectControlFlagsRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    control_flags: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for SetVStorageObjectControlFlagsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetVStorageObjectControlFlagsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetVStorageObjectControlFlagsRequestTypeSer<'b, 'a> {
    data: &'b SetVStorageObjectControlFlagsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetVStorageObjectControlFlagsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetVStorageObjectControlFlagsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.control_flags else { continue; };
                    return Some((std::borrow::Cow::Borrowed("controlFlags"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct SetVirtualDiskUuidExRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    uuid: Option<&'a str>,
}

impl<'a> miniserde::Serialize for SetVirtualDiskUuidExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetVirtualDiskUuidExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetVirtualDiskUuidExRequestTypeSer<'b, 'a> {
    data: &'b SetVirtualDiskUuidExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetVirtualDiskUuidExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetVirtualDiskUuidExRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.uuid else { continue; };
                    return Some((std::borrow::Cow::Borrowed("uuid"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UnregisterDiskRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for UnregisterDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnregisterDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnregisterDiskRequestTypeSer<'b, 'a> {
    data: &'b UnregisterDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnregisterDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnregisterDiskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateVStorageInfrastructureObjectPolicyRequestType<'a> {
    spec: &'a crate::types::structs::VslmInfrastructureObjectPolicySpec,
}

impl<'a> miniserde::Serialize for UpdateVStorageInfrastructureObjectPolicyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateVStorageInfrastructureObjectPolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateVStorageInfrastructureObjectPolicyRequestTypeSer<'b, 'a> {
    data: &'b UpdateVStorageInfrastructureObjectPolicyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateVStorageInfrastructureObjectPolicyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateVStorageInfrastructureObjectPolicyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateVStorageObjectCryptoRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    profile: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
    disks_crypto: Option<&'a crate::types::structs::DiskCryptoSpec>,
}

impl<'a> miniserde::Serialize for UpdateVStorageObjectCryptoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateVStorageObjectCryptoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateVStorageObjectCryptoRequestTypeSer<'b, 'a> {
    data: &'b UpdateVStorageObjectCryptoRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateVStorageObjectCryptoRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateVStorageObjectCryptoRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.disks_crypto else { continue; };
                    return Some((std::borrow::Cow::Borrowed("disksCrypto"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VCenterUpdateVStorageObjectMetadataExRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    metadata: Option<&'a [crate::types::structs::KeyValue]>,
    delete_keys: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for VCenterUpdateVStorageObjectMetadataExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VCenterUpdateVStorageObjectMetadataExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VCenterUpdateVStorageObjectMetadataExRequestTypeSer<'b, 'a> {
    data: &'b VCenterUpdateVStorageObjectMetadataExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VCenterUpdateVStorageObjectMetadataExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VCenterUpdateVStorageObjectMetadataExRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.metadata else { continue; };
                    return Some((std::borrow::Cow::Borrowed("metadata"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.delete_keys else { continue; };
                    return Some((std::borrow::Cow::Borrowed("deleteKeys"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UpdateVStorageObjectPolicyRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    profile: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
}

impl<'a> miniserde::Serialize for UpdateVStorageObjectPolicyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateVStorageObjectPolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateVStorageObjectPolicyRequestTypeSer<'b, 'a> {
    data: &'b UpdateVStorageObjectPolicyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateVStorageObjectPolicyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateVStorageObjectPolicyRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
