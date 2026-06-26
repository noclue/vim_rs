use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Interface to manage virtual storage object on an ESXi host.
#[derive(Clone)]
pub struct HostVStorageObjectManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostVStorageObjectManager {
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
    pub async fn host_v_storage_object_revert_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: &crate::types::structs::Id) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostVStorageObjectRevertRequestType {id, datastore, snapshot_id, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostVStorageObjectRevert_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_clear_v_storage_object_control_flags(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, control_flags: Option<&[String]>) -> Result<()> {
        let input = HostClearVStorageObjectControlFlagsRequestType {id, datastore, control_flags, };
        self.client.invoke_void("", "HostVStorageObjectManager", &self.mo_id, "HostClearVStorageObjectControlFlags", Some(&input)).await
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
    pub async fn host_clone_v_storage_object_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, spec: &crate::types::structs::VslmCloneSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostCloneVStorageObjectRequestType {id, datastore, spec, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostCloneVStorageObject_Task", Some(&input)).await?;
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
    pub async fn host_create_disk_task(&self, spec: &crate::types::structs::VslmCreateSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostCreateDiskRequestType {spec, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostCreateDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// Relative location where disk has to be created, used in
    /// `targetDatastore` and `datastore` parameters.
    /// If not specified disk gets created at default *VStorageObject*
    /// location of `targetDatastore` or `datastore`.
    ///
    /// ### provisioning_type
    /// Provisioining type of the disk as specified in above
    /// mentioned profile. The list of supported values can be found in
    /// *BaseConfigInfoDiskFileBackingInfoProvisioningType_enum*
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ///
    /// ### is_linked_clone
    /// Indicates whether a linkedClone Disk needs to be created from the snapshot.
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
    pub async fn host_v_storage_object_create_disk_from_snapshot_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: &crate::types::structs::Id, name: &str, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>, crypto: Option<&dyn crate::types::traits::CryptoSpecTrait>, path: Option<&str>, provisioning_type: Option<&str>, is_linked_clone: Option<bool>, target_id: Option<&crate::types::structs::Id>, target_datastore: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostVStorageObjectCreateDiskFromSnapshotRequestType {id, datastore, snapshot_id, name, profile, crypto, path, provisioning_type, is_linked_clone, target_id, target_datastore, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostVStorageObjectCreateDiskFromSnapshot_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_v_storage_object_create_snapshot_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, description: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostVStorageObjectCreateSnapshotRequestType {id, datastore, description, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostVStorageObjectCreateSnapshot_Task", Some(&input)).await?;
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
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "VStorageObjectCreateSnapshotEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_v_storage_object_delete_snapshot_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: &crate::types::structs::Id) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostVStorageObjectDeleteSnapshotRequestType {id, datastore, snapshot_id, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostVStorageObjectDeleteSnapshot_Task", Some(&input)).await?;
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
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "VStorageObjectDeleteSnapshotEx_Task", Some(&input)).await?;
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
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "VStorageObjectDeleteSnapshotEx2_Task", Some(&input)).await?;
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
    /// The datastore where the virtual storage object is
    /// located.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### is_lc_parent_attached
    /// Set to true if parent of linked clone is attached
    /// to VM, set to false if parent of linked clone is
    /// detached, unset is considered as false.
    /// 
    /// ***Since:*** vSphere API Release 9.1.0.0
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
    pub async fn host_delete_v_storage_object_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, is_lc_parent_attached: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostDeleteVStorageObjectRequestType {id, datastore, is_lc_parent_attached, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostDeleteVStorageObject_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Delete a virtual storage object and its associated backings.
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
    /// ### is_lc_parent_attached
    /// Set to true if parent of linked clone is attached
    /// to VM, set to false if parent of linked clone is
    /// detached, unset is considered as false.
    /// 
    /// ***Since:*** vSphere API Release 9.1.0.0
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
    pub async fn host_delete_v_storage_object_ex_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, is_lc_parent_attached: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostDeleteVStorageObjectExRequestType {id, datastore, is_lc_parent_attached, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostDeleteVStorageObjectEx_Task", Some(&input)).await?;
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
    pub async fn host_extend_disk_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, new_capacity_in_mb: i64) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostExtendDiskRequestType {id, datastore, new_capacity_in_mb, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostExtendDisk_Task", Some(&input)).await?;
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
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "VStorageObjectExtendDiskEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_inflate_disk_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostInflateDiskRequestType {id, datastore, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostInflateDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_list_v_storage_object(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::Id>>> {
        let input = HostListVStorageObjectRequestType {datastore, };
        let bytes_opt = self.client.invoke_optional("", "HostVStorageObjectManager", &self.mo_id, "HostListVStorageObject", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Get the virtual disk UUID.
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
    /// A URL has the form
    /// > _scheme_://_authority_/folder/_path_?dsName=_dsName_
    /// 
    /// where
    /// - _scheme_ is <code>http</code> or <code>https</code>.
    /// - _authority_ specifies the hostname or IP address of the VirtualCenter or
    ///   ESX server and optionally the port.
    /// - _dsName_ is the name of the Datastore.
    /// - _path_ is a slash-delimited path from the root of the datastore.
    /// 
    /// A datastore path has the form
    /// > \[_datastore_\] _path_
    /// 
    /// where
    /// - _datastore_ is the datastore name.
    /// - _path_ is a slash-delimited path from the root of the datastore.
    /// 
    /// An example datastore path is "\[storage\] path/to/file.extension".
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
    pub async fn host_query_virtual_disk_uuid(&self, name: &str) -> Result<String> {
        let input = HostQueryVirtualDiskUuidRequestType {name, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostQueryVirtualDiskUuid", Some(&input)).await?;
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
    pub async fn host_reconcile_datastore_inventory_task(&self, datastore: &crate::types::structs::ManagedObjectReference, deep_cleansing: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostReconcileDatastoreInventoryRequestType {datastore, deep_cleansing, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostReconcileDatastoreInventory_Task", Some(&input)).await?;
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
    /// URL or datastore path to the running point of the virtual disk.
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
    pub async fn host_register_disk(&self, path: &str, name: Option<&str>, modify_control_flags: Option<bool>, id: Option<&crate::types::structs::Id>) -> Result<crate::types::structs::VStorageObject> {
        let input = HostRegisterDiskRequestType {path, name, modify_control_flags, id, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostRegisterDisk", Some(&input)).await?;
        let result: crate::types::structs::VStorageObject = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// ### is_lc_parent_attached
    /// ***Since:*** vSphere API Release 9.1.0.0
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
    pub async fn host_relocate_v_storage_object_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, spec: &crate::types::structs::VslmRelocateSpec, is_lc_parent_attached: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostRelocateVStorageObjectRequestType {id, datastore, spec, is_lc_parent_attached, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostRelocateVStorageObject_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_rename_v_storage_object(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, name: &str) -> Result<()> {
        let input = HostRenameVStorageObjectRequestType {id, datastore, name, };
        self.client.invoke_void("", "HostVStorageObjectManager", &self.mo_id, "HostRenameVStorageObject", Some(&input)).await
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
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "RenameVStorageObjectEx", Some(&input)).await?;
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
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "RepairVStorageObjectChain_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_v_storage_object_retrieve_snapshot_info(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::VStorageObjectSnapshotInfo> {
        let input = HostVStorageObjectRetrieveSnapshotInfoRequestType {id, datastore, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostVStorageObjectRetrieveSnapshotInfo", Some(&input)).await?;
        let result: crate::types::structs::VStorageObjectSnapshotInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_retrieve_v_storage_infrastructure_object_policy(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::VslmInfrastructureObjectPolicy>>> {
        let input = HostRetrieveVStorageInfrastructureObjectPolicyRequestType {datastore, };
        let bytes_opt = self.client.invoke_optional("", "HostVStorageObjectManager", &self.mo_id, "HostRetrieveVStorageInfrastructureObjectPolicy", Some(&input)).await?;
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
    pub async fn host_retrieve_v_storage_object(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, disk_info_flags: Option<&[String]>) -> Result<crate::types::structs::VStorageObject> {
        let input = HostRetrieveVStorageObjectRequestType {id, datastore, disk_info_flags, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostRetrieveVStorageObject", Some(&input)).await?;
        let result: crate::types::structs::VStorageObject = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_retrieve_v_storage_object_metadata(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: Option<&crate::types::structs::Id>, prefix: Option<&str>) -> Result<Option<Vec<crate::types::structs::KeyValue>>> {
        let input = HostRetrieveVStorageObjectMetadataRequestType {id, datastore, snapshot_id, prefix, };
        let bytes_opt = self.client.invoke_optional("", "HostVStorageObjectManager", &self.mo_id, "HostRetrieveVStorageObjectMetadata", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn host_retrieve_v_storage_object_metadata_value(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, snapshot_id: Option<&crate::types::structs::Id>, key: &str) -> Result<String> {
        let input = HostRetrieveVStorageObjectMetadataValueRequestType {id, datastore, snapshot_id, key, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostRetrieveVStorageObjectMetadataValue", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_retrieve_v_storage_object_state(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::VStorageObjectStateInfo> {
        let input = HostRetrieveVStorageObjectStateRequestType {id, datastore, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostRetrieveVStorageObjectState", Some(&input)).await?;
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
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "RevertVStorageObjectEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_schedule_reconcile_datastore_inventory(&self, datastore: &crate::types::structs::ManagedObjectReference, deep_cleansing: Option<bool>) -> Result<()> {
        let input = HostScheduleReconcileDatastoreInventoryRequestType {datastore, deep_cleansing, };
        self.client.invoke_void("", "HostVStorageObjectManager", &self.mo_id, "HostScheduleReconcileDatastoreInventory", Some(&input)).await
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
    pub async fn host_set_v_storage_object_control_flags(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, control_flags: Option<&[String]>) -> Result<()> {
        let input = HostSetVStorageObjectControlFlagsRequestType {id, datastore, control_flags, };
        self.client.invoke_void("", "HostVStorageObjectManager", &self.mo_id, "HostSetVStorageObjectControlFlags", Some(&input)).await
    }
    /// Set the virtual disk Uuid.
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
    /// A URL has the form
    /// > _scheme_://_authority_/folder/_path_?dsName=_dsName_
    /// 
    /// where
    /// - _scheme_ is <code>http</code> or <code>https</code>.
    /// - _authority_ specifies the hostname or IP address of the VirtualCenter or
    ///   ESX server and optionally the port.
    /// - _dsName_ is the name of the Datastore.
    /// - _path_ is a slash-delimited path from the root of the datastore.
    /// 
    /// A datastore path has the form
    /// > \[_datastore_\] _path_
    /// 
    /// where
    /// - _datastore_ is the datastore name.
    /// - _path_ is a slash-delimited path from the root of the datastore.
    /// 
    /// An example datastore path is "\[storage\] path/to/file.extension".
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
    pub async fn host_set_virtual_disk_uuid_task(&self, name: &str, uuid: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostSetVirtualDiskUuidRequestType {name, uuid, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostSetVirtualDiskUuid_Task", Some(&input)).await?;
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
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "UnregisterDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_update_v_storage_object_metadata_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, metadata: Option<&[crate::types::structs::KeyValue]>, delete_keys: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostUpdateVStorageObjectMetadataRequestType {id, datastore, metadata, delete_keys, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostUpdateVStorageObjectMetadata_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_update_v_storage_object_metadata_ex_task(&self, id: &crate::types::structs::Id, datastore: &crate::types::structs::ManagedObjectReference, metadata: Option<&[crate::types::structs::KeyValue]>, delete_keys: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HostUpdateVStorageObjectMetadataExRequestType {id, datastore, metadata, delete_keys, };
        let bytes = self.client.invoke("", "HostVStorageObjectManager", &self.mo_id, "HostUpdateVStorageObjectMetadataEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct HostVStorageObjectRevertRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: &'a crate::types::structs::Id,
}

impl<'a> miniserde::Serialize for HostVStorageObjectRevertRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostVStorageObjectRevertRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostVStorageObjectRevertRequestTypeSer<'b, 'a> {
    data: &'b HostVStorageObjectRevertRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostVStorageObjectRevertRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostVStorageObjectRevertRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("snapshotId"), &self.data.snapshot_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HostClearVStorageObjectControlFlagsRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    control_flags: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for HostClearVStorageObjectControlFlagsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostClearVStorageObjectControlFlagsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostClearVStorageObjectControlFlagsRequestTypeSer<'b, 'a> {
    data: &'b HostClearVStorageObjectControlFlagsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostClearVStorageObjectControlFlagsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostClearVStorageObjectControlFlagsRequestType")),
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
struct HostCloneVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::VslmCloneSpec,
}

impl<'a> miniserde::Serialize for HostCloneVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostCloneVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostCloneVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b HostCloneVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostCloneVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostCloneVStorageObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HostCreateDiskRequestType<'a> {
    spec: &'a crate::types::structs::VslmCreateSpec,
}

impl<'a> miniserde::Serialize for HostCreateDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostCreateDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostCreateDiskRequestTypeSer<'b, 'a> {
    data: &'b HostCreateDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostCreateDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostCreateDiskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HostVStorageObjectCreateDiskFromSnapshotRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: &'a crate::types::structs::Id,
    name: &'a str,
    profile: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
    crypto: Option<&'a dyn crate::types::traits::CryptoSpecTrait>,
    path: Option<&'a str>,
    provisioning_type: Option<&'a str>,
    is_linked_clone: Option<bool>,
    target_id: Option<&'a crate::types::structs::Id>,
    target_datastore: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for HostVStorageObjectCreateDiskFromSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostVStorageObjectCreateDiskFromSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostVStorageObjectCreateDiskFromSnapshotRequestTypeSer<'b, 'a> {
    data: &'b HostVStorageObjectCreateDiskFromSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostVStorageObjectCreateDiskFromSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostVStorageObjectCreateDiskFromSnapshotRequestType")),
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
                    let Some(ref val) = self.data.provisioning_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("provisioningType"), val as &dyn miniserde::Serialize));
                }
                9 => {
                    let Some(ref val) = self.data.is_linked_clone else { continue; };
                    return Some((std::borrow::Cow::Borrowed("isLinkedClone"), val as &dyn miniserde::Serialize));
                }
                10 => {
                    let Some(ref val) = self.data.target_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targetId"), val as &dyn miniserde::Serialize));
                }
                11 => {
                    let Some(ref val) = self.data.target_datastore else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targetDatastore"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct HostVStorageObjectCreateSnapshotRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    description: &'a str,
}

impl<'a> miniserde::Serialize for HostVStorageObjectCreateSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostVStorageObjectCreateSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostVStorageObjectCreateSnapshotRequestTypeSer<'b, 'a> {
    data: &'b HostVStorageObjectCreateSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostVStorageObjectCreateSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostVStorageObjectCreateSnapshotRequestType")),
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
struct HostVStorageObjectDeleteSnapshotRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: &'a crate::types::structs::Id,
}

impl<'a> miniserde::Serialize for HostVStorageObjectDeleteSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostVStorageObjectDeleteSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostVStorageObjectDeleteSnapshotRequestTypeSer<'b, 'a> {
    data: &'b HostVStorageObjectDeleteSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostVStorageObjectDeleteSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostVStorageObjectDeleteSnapshotRequestType")),
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
struct HostDeleteVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    is_lc_parent_attached: Option<bool>,
}

impl<'a> miniserde::Serialize for HostDeleteVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostDeleteVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostDeleteVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b HostDeleteVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostDeleteVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostDeleteVStorageObjectRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.is_lc_parent_attached else { continue; };
                    return Some((std::borrow::Cow::Borrowed("isLcParentAttached"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct HostDeleteVStorageObjectExRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    is_lc_parent_attached: Option<bool>,
}

impl<'a> miniserde::Serialize for HostDeleteVStorageObjectExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostDeleteVStorageObjectExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostDeleteVStorageObjectExRequestTypeSer<'b, 'a> {
    data: &'b HostDeleteVStorageObjectExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostDeleteVStorageObjectExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostDeleteVStorageObjectExRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.is_lc_parent_attached else { continue; };
                    return Some((std::borrow::Cow::Borrowed("isLcParentAttached"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct HostExtendDiskRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    new_capacity_in_mb: i64,
}

impl<'a> miniserde::Serialize for HostExtendDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostExtendDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostExtendDiskRequestTypeSer<'b, 'a> {
    data: &'b HostExtendDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostExtendDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostExtendDiskRequestType")),
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
struct HostInflateDiskRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for HostInflateDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostInflateDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostInflateDiskRequestTypeSer<'b, 'a> {
    data: &'b HostInflateDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostInflateDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostInflateDiskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HostListVStorageObjectRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for HostListVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostListVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostListVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b HostListVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostListVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostListVStorageObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HostQueryVirtualDiskUuidRequestType<'a> {
    name: &'a str,
}

impl<'a> miniserde::Serialize for HostQueryVirtualDiskUuidRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostQueryVirtualDiskUuidRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostQueryVirtualDiskUuidRequestTypeSer<'b, 'a> {
    data: &'b HostQueryVirtualDiskUuidRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostQueryVirtualDiskUuidRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostQueryVirtualDiskUuidRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HostReconcileDatastoreInventoryRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
    deep_cleansing: Option<bool>,
}

impl<'a> miniserde::Serialize for HostReconcileDatastoreInventoryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostReconcileDatastoreInventoryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostReconcileDatastoreInventoryRequestTypeSer<'b, 'a> {
    data: &'b HostReconcileDatastoreInventoryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostReconcileDatastoreInventoryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostReconcileDatastoreInventoryRequestType")),
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
struct HostRegisterDiskRequestType<'a> {
    path: &'a str,
    name: Option<&'a str>,
    modify_control_flags: Option<bool>,
    id: Option<&'a crate::types::structs::Id>,
}

impl<'a> miniserde::Serialize for HostRegisterDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostRegisterDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostRegisterDiskRequestTypeSer<'b, 'a> {
    data: &'b HostRegisterDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostRegisterDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostRegisterDiskRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("path"), &self.data.path as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("name"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.modify_control_flags else { continue; };
                    return Some((std::borrow::Cow::Borrowed("modifyControlFlags"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("id"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct HostRelocateVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::VslmRelocateSpec,
    is_lc_parent_attached: Option<bool>,
}

impl<'a> miniserde::Serialize for HostRelocateVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostRelocateVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostRelocateVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b HostRelocateVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostRelocateVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostRelocateVStorageObjectRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.is_lc_parent_attached else { continue; };
                    return Some((std::borrow::Cow::Borrowed("isLcParentAttached"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct HostRenameVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    name: &'a str,
}

impl<'a> miniserde::Serialize for HostRenameVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostRenameVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostRenameVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b HostRenameVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostRenameVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostRenameVStorageObjectRequestType")),
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
struct HostVStorageObjectRetrieveSnapshotInfoRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for HostVStorageObjectRetrieveSnapshotInfoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostVStorageObjectRetrieveSnapshotInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostVStorageObjectRetrieveSnapshotInfoRequestTypeSer<'b, 'a> {
    data: &'b HostVStorageObjectRetrieveSnapshotInfoRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostVStorageObjectRetrieveSnapshotInfoRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostVStorageObjectRetrieveSnapshotInfoRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HostRetrieveVStorageInfrastructureObjectPolicyRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for HostRetrieveVStorageInfrastructureObjectPolicyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostRetrieveVStorageInfrastructureObjectPolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostRetrieveVStorageInfrastructureObjectPolicyRequestTypeSer<'b, 'a> {
    data: &'b HostRetrieveVStorageInfrastructureObjectPolicyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostRetrieveVStorageInfrastructureObjectPolicyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostRetrieveVStorageInfrastructureObjectPolicyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HostRetrieveVStorageObjectRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    disk_info_flags: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for HostRetrieveVStorageObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostRetrieveVStorageObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostRetrieveVStorageObjectRequestTypeSer<'b, 'a> {
    data: &'b HostRetrieveVStorageObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostRetrieveVStorageObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostRetrieveVStorageObjectRequestType")),
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
struct HostRetrieveVStorageObjectMetadataRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: Option<&'a crate::types::structs::Id>,
    prefix: Option<&'a str>,
}

impl<'a> miniserde::Serialize for HostRetrieveVStorageObjectMetadataRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostRetrieveVStorageObjectMetadataRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostRetrieveVStorageObjectMetadataRequestTypeSer<'b, 'a> {
    data: &'b HostRetrieveVStorageObjectMetadataRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostRetrieveVStorageObjectMetadataRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostRetrieveVStorageObjectMetadataRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.snapshot_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("snapshotId"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.prefix else { continue; };
                    return Some((std::borrow::Cow::Borrowed("prefix"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct HostRetrieveVStorageObjectMetadataValueRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    snapshot_id: Option<&'a crate::types::structs::Id>,
    key: &'a str,
}

impl<'a> miniserde::Serialize for HostRetrieveVStorageObjectMetadataValueRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostRetrieveVStorageObjectMetadataValueRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostRetrieveVStorageObjectMetadataValueRequestTypeSer<'b, 'a> {
    data: &'b HostRetrieveVStorageObjectMetadataValueRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostRetrieveVStorageObjectMetadataValueRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostRetrieveVStorageObjectMetadataValueRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.snapshot_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("snapshotId"), val as &dyn miniserde::Serialize));
                }
                4 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct HostRetrieveVStorageObjectStateRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for HostRetrieveVStorageObjectStateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostRetrieveVStorageObjectStateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostRetrieveVStorageObjectStateRequestTypeSer<'b, 'a> {
    data: &'b HostRetrieveVStorageObjectStateRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostRetrieveVStorageObjectStateRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostRetrieveVStorageObjectStateRequestType")),
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
struct HostScheduleReconcileDatastoreInventoryRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
    deep_cleansing: Option<bool>,
}

impl<'a> miniserde::Serialize for HostScheduleReconcileDatastoreInventoryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostScheduleReconcileDatastoreInventoryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostScheduleReconcileDatastoreInventoryRequestTypeSer<'b, 'a> {
    data: &'b HostScheduleReconcileDatastoreInventoryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostScheduleReconcileDatastoreInventoryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostScheduleReconcileDatastoreInventoryRequestType")),
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
struct HostSetVStorageObjectControlFlagsRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    control_flags: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for HostSetVStorageObjectControlFlagsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostSetVStorageObjectControlFlagsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostSetVStorageObjectControlFlagsRequestTypeSer<'b, 'a> {
    data: &'b HostSetVStorageObjectControlFlagsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostSetVStorageObjectControlFlagsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostSetVStorageObjectControlFlagsRequestType")),
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
struct HostSetVirtualDiskUuidRequestType<'a> {
    name: &'a str,
    uuid: Option<&'a str>,
}

impl<'a> miniserde::Serialize for HostSetVirtualDiskUuidRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostSetVirtualDiskUuidRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostSetVirtualDiskUuidRequestTypeSer<'b, 'a> {
    data: &'b HostSetVirtualDiskUuidRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostSetVirtualDiskUuidRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostSetVirtualDiskUuidRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
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
struct HostUpdateVStorageObjectMetadataRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    metadata: Option<&'a [crate::types::structs::KeyValue]>,
    delete_keys: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for HostUpdateVStorageObjectMetadataRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostUpdateVStorageObjectMetadataRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostUpdateVStorageObjectMetadataRequestTypeSer<'b, 'a> {
    data: &'b HostUpdateVStorageObjectMetadataRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostUpdateVStorageObjectMetadataRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostUpdateVStorageObjectMetadataRequestType")),
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
struct HostUpdateVStorageObjectMetadataExRequestType<'a> {
    id: &'a crate::types::structs::Id,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    metadata: Option<&'a [crate::types::structs::KeyValue]>,
    delete_keys: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for HostUpdateVStorageObjectMetadataExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostUpdateVStorageObjectMetadataExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostUpdateVStorageObjectMetadataExRequestTypeSer<'b, 'a> {
    data: &'b HostUpdateVStorageObjectMetadataExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostUpdateVStorageObjectMetadataExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostUpdateVStorageObjectMetadataExRequestType")),
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
