use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Most VirtualDiskManager APIs will be DEPRECATED as of vSphere 6.5.
/// 
/// Please use VStorageObjectManager APIs to manage Virtual disks.
/// 
/// This managed object type provides a way to manage and manipulate virtual disks
/// on datastores. The source and the destination names are in the form of
/// a URL or a datastore path.
/// 
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
/// A datastore path has the form
/// > \[_datastore_\] _path_
/// 
/// where
/// - _datastore_ is the datastore name.
/// - _path_ is a slash-delimited path from the root of the datastore.
/// 
/// An example datastore path is "\[storage\] path/to/file.extension".
/// A listing of all the files, disks and folders on
/// a datastore can be obtained from the datastore browser.
/// 
/// See also *HostDatastoreBrowser*.
#[derive(Clone)]
pub struct VirtualDiskManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VirtualDiskManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Copy a virtual disk, performing conversions as specified in the spec.
    /// 
    /// If source (or destination) name is specified as a URL, then the
    /// corresponding datacenter parameter may be omitted.
    /// 
    /// If source and destination resolve to the same file system location,
    /// the call has no effect, regardless of destSpec content.
    /// 
    /// Requires Datastore.FileManagement privilege on both source and destination
    /// datastores.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### source_name
    /// The name of the source, either a datastore path
    /// or a URL referring to the virtual disk to be copied.
    ///
    /// ### source_datacenter
    /// If <code>sourceName</code> is a datastore path, the
    /// datacenter for that datastore path.
    /// Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>sourceName</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### dest_name
    /// The name of the destination, either a datastore path
    /// or a URL referring to the virtual disk to be created.
    ///
    /// ### dest_datacenter
    /// If <code>destName</code> is a datastore
    /// path, the datacenter for that datastore path.
    /// Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter, it is assumed that
    /// the destination path belongs to the source datacenter.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### dest_spec
    /// The specification of the virtual disk to be created.
    /// If not specified, a preallocated format and busLogic adapter type is assumed.
    ///
    /// ### force
    /// The force flag is currently ignored. The FileAlreadyExists fault is thrown if
    /// the destination file already exists.
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
    /// ***FileFault***: if an error occurs cloning the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the source
    /// or destination datastore.
    /// 
    /// ***InvalidDiskFormat***: if the destination's format is not supported.
    pub async fn copy_virtual_disk_task(&self, source_name: &str, source_datacenter: Option<&crate::types::structs::ManagedObjectReference>, dest_name: &str, dest_datacenter: Option<&crate::types::structs::ManagedObjectReference>, dest_spec: Option<&dyn crate::types::traits::VirtualDiskSpecTrait>, force: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CopyVirtualDiskRequestType {source_name, source_datacenter, dest_name, dest_datacenter, dest_spec, force, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "CopyVirtualDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of vSphere 6.5, use
    /// *HostVStorageObjectManager.HostCreateDisk_Task* instead.
    /// 
    /// Create a virtual disk.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk is created.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a
    /// URL referring to the virtual disk to be created.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### spec
    /// The specification of the virtual disk to be created.
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
    /// ***FileFault***: if an error occurs creating the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn create_virtual_disk_task(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>, spec: &dyn crate::types::traits::VirtualDiskSpecTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateVirtualDiskRequestType {name, datacenter, spec, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "CreateVirtualDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of vSphere 6.5, use
    /// *VirtualMachine.DefragmentAllDisks* instead.
    /// 
    /// Defragment a sparse virtual disk.
    /// 
    /// This is defragmentation of the virtual disk file(s) in the host operating
    /// system, not defragmentation of the guest operating system filesystem inside
    /// the virtual disk.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk that should be defragmented.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
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
    /// ***FileFault***: if an error occurs defragmenting the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn defragment_virtual_disk_task(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DefragmentVirtualDiskRequestType {name, datacenter, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "DefragmentVirtualDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of vSphere 6.5, use
    /// *HostVStorageObjectManager.HostDeleteVStorageObject_Task* instead.
    /// 
    /// Delete a virtual disk.
    /// 
    /// All files relating to the disk
    /// will be deleted.
    /// 
    /// Deletion of virtual disk is prohibited if it is attached to VMs.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk is removed.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk to be deleted.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
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
    /// ***FileFault***: if an error occurs deleting the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn delete_virtual_disk_task(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DeleteVirtualDiskRequestType {name, datacenter, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "DeleteVirtualDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Explicitly zero out unaccessed parts zeroedthick disk.
    /// 
    /// Effectively a no-op if the disk is already eagerZeroedThick.
    /// Unlike zeroFillVirtualDisk, which wipes the entire disk, this
    /// operation only affects previously unaccessed parts of the disk.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk that should be inflated.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
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
    /// ***FileFault***: if an error occurs while eager-zeroing the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn eager_zero_virtual_disk_task(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = EagerZeroVirtualDiskRequestType {name, datacenter, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "EagerZeroVirtualDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of vSphere 6.5, use
    /// *HostVStorageObjectManager.HostExtendDisk_Task* instead.
    /// 
    /// Expand the capacity of a virtual disk to the new capacity.
    /// 
    /// If the eagerZero flag is not specified,
    /// \- the extended disk region of a zerothick disk will be zeroedthick
    /// \- the extended disk region of a eagerzerothick disk will be eagerzeroedthick
    /// \- a thin-provisioned disk will always be extended as a thin-provisioned disk.
    /// If the eagerZero flag TRUE, the extended region of the disk will
    /// always be eagerly zeroed.
    /// If the eagerZero flag FALSE, the extended region of a zeroedthick or
    /// eagerzeroedthick the disk will not be eagerly zeroed. This condition has
    /// no effect on a thin source disk.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk whose capacity should be expanded.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### new_capacity_kb
    /// The new capacty of the virtual disk in Kb.
    ///
    /// ### eager_zero
    /// If true, the extended part of the disk will be
    /// explicitly filled with zeroes.
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
    /// ***FileFault***: if an error occurs extending the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn extend_virtual_disk_task(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>, new_capacity_kb: i64, eager_zero: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ExtendVirtualDiskRequestType {name, datacenter, new_capacity_kb, eager_zero, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "ExtendVirtualDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Import an unmanaged-snapshot from Virtual-Volume(VVol) enabled
    /// Storage Array.
    /// 
    /// Storage Array may support users to take snapshots independent of
    /// VMware stack. Such copies or snapshots are known as
    /// 'Unmanaged-Snapshots'.
    /// We are providing an ability to end-users to import such
    /// unmanaged-snapshots as Virtual Disks.
    /// 
    /// End-user needs to know the VVol-Identifier to import unmanaged
    /// snapshot as VirtualDisk.
    /// 
    /// Once VirtualDisk is created, user can use 'Datastore Browser' to use
    /// with rest of Virtual Machine provisioning APIs.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vdisk
    /// \- The name of the disk to import, either a datastore path or a URL
    /// referring to the virtual disk from which to get geometry information.
    ///
    /// ### datacenter
    /// If <code>vdisk</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>vdisk</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### vvol_id
    /// \- unmanaged snapshot identifier
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if VVol is not found
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// datastore.
    pub async fn import_unmanaged_snapshot(&self, vdisk: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>, vvol_id: &str) -> Result<()> {
        let input = ImportUnmanagedSnapshotRequestType {vdisk, datacenter, vvol_id, };
        self.client.invoke_void("", "VirtualDiskManager", &self.mo_id, "ImportUnmanagedSnapshot", Some(&input)).await
    }
    /// Deprecated as of vSphere 6.5, use
    /// *HostVStorageObjectManager.HostInflateDisk_Task* instead.
    /// 
    /// Inflate a sparse or thin-provisioned virtual disk up to the full size.
    /// 
    /// Additional space allocated to the disk as a result of this operation
    /// will be filled with zeroes.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk that should be inflated.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
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
    /// ***FileFault***: if an error occurs inflating the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn inflate_virtual_disk_task(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = InflateVirtualDiskRequestType {name, datacenter, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "InflateVirtualDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Move a virtual disk and all related files from the source location specified
    /// by <code>sourceName</code> and <code>sourceDatacenter</code> to the destination
    /// location specified by <code>destName</code> and <code>destDatacenter</code>.
    /// 
    /// If source (or destination) name is specified as a URL, then the
    /// corresponding datacenter parameter may be omitted.
    /// 
    /// If source and destination resolve to the same file system location,
    /// the call has no effect.
    /// 
    /// Requires Datastore.FileManagement privilege on both source and destination
    /// datastores.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### source_name
    /// The name of the source, either a datastore path
    /// or a URL referring to the virtual disk to be moved.
    ///
    /// ### source_datacenter
    /// If <code>sourceName</code> is a datastore path, the
    /// datacenter for that datastore path.
    /// Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>sourceName</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### dest_name
    /// The name of the destination, either a datastore path
    /// or a URL referring to the destination virtual disk.
    ///
    /// ### dest_datacenter
    /// If <code>destName</code> is a datastore
    /// path, the datacenter for that datastore path.
    /// Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter, it is assumed that
    /// the destination path belongs to the source datacenter.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### force
    /// If true, overwrite any identically named disk at the destination.
    /// If not specified, it is assumed to be false
    ///
    /// ### profile
    /// User can specify new set of profile when moving virtual disk.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: if an error occurs renaming the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the source
    /// or destination datastore.
    pub async fn move_virtual_disk_task(&self, source_name: &str, source_datacenter: Option<&crate::types::structs::ManagedObjectReference>, dest_name: &str, dest_datacenter: Option<&crate::types::structs::ManagedObjectReference>, force: Option<bool>, profile: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = MoveVirtualDiskRequestType {source_name, source_datacenter, dest_name, dest_datacenter, force, profile, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "MoveVirtualDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Return the percentage of fragmentation of the sparse virtual disk.
    /// 
    /// This is the fragmentation of virtual disk file(s) in the host operating
    /// system, not the fragmentation of the guest operating systemS filesystem
    /// inside the virtual disk.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk for which to return the
    /// percentage of fragmentation.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ## Returns:
    ///
    /// the percentage of fragmentation (as an integer between 0 and 100)
    /// of the sparse virtual disk.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: if an error occurs reading the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn query_virtual_disk_fragmentation(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>) -> Result<i32> {
        let input = QueryVirtualDiskFragmentationRequestType {name, datacenter, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "QueryVirtualDiskFragmentation", Some(&input)).await?;
        let result: i32 = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Get the disk geometry information for the virtual disk.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk from which to get geometry information.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ## Returns:
    ///
    /// The geometry information for this virtual disk.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: if an error occurs reading the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn query_virtual_disk_geometry(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::HostDiskDimensionsChs> {
        let input = QueryVirtualDiskGeometryRequestType {name, datacenter, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "QueryVirtualDiskGeometry", Some(&input)).await?;
        let result: crate::types::structs::HostDiskDimensionsChs = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of vSphere 6.5, use
    /// *HostVStorageObjectManager.HostRetrieveVStorageObject*
    /// instead.
    /// 
    /// Get the virtual disk SCSI inquiry page 0x83 data.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk from which to get SCSI inquiry
    /// page 0x83 data.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
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
    pub async fn query_virtual_disk_uuid(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>) -> Result<String> {
        let input = QueryVirtualDiskUuidRequestType {name, datacenter, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "QueryVirtualDiskUuid", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Release a snapshot previously imported with importUnmanagedSnapshot
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vdisk
    /// \- The name of the disk to release, either a datastore path or a URL
    /// referring to the virtual disk.
    ///
    /// ### datacenter
    /// If <code>vdisk</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>vdisk</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ## Errors:
    ///
    /// ***FileNotFound***: if vdisk is not found
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// datastore.
    pub async fn release_managed_snapshot(&self, vdisk: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>) -> Result<()> {
        let input = ReleaseManagedSnapshotRequestType {vdisk, datacenter, };
        self.client.invoke_void("", "VirtualDiskManager", &self.mo_id, "ReleaseManagedSnapshot", Some(&input)).await
    }
    /// Deprecated as of vSphere 6.5, use
    /// *HostVStorageObjectManager.HostRegisterDisk* to register
    /// a disk as vStorageObject with new unique UUID.
    /// 
    /// Set the virtual disk SCSI inquiry page 0x83 data.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk whose SCSI inquiry page 0x83
    /// data should be set.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### uuid
    /// The hex representation of the unique ID for this virtual disk.
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: if an error occurs updating the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn set_virtual_disk_uuid(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>, uuid: &str) -> Result<()> {
        let input = SetVirtualDiskUuidRequestType {name, datacenter, uuid, };
        self.client.invoke_void("", "VirtualDiskManager", &self.mo_id, "SetVirtualDiskUuid", Some(&input)).await
    }
    /// Deprecated as of vSphere 6.5, use
    /// *VirtualMachine.ShrinkDisk_Task* instead.
    /// 
    /// Shrink a sparse virtual disk.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// The optional parameter <code>copy</code> specifies whether to shrink the
    /// disk in copy-shrink mode or in-place mode. In copy-shrink mode,
    /// additional space is required, but will result in a shrunk disk that is
    /// also defragmented. In-place shrink does not require additional space,
    /// but will increase fragmentation. The default behavior is to perform
    /// copy-shrink if the parameter is not specified.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk that should be shrink.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### copy
    /// If true or omitted, performs shrink in copy-shrink mode, otherwise
    /// shrink in in-place mode.
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
    /// ***FileFault***: if an error occurs shrinking the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn shrink_virtual_disk_task(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>, copy: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ShrinkVirtualDiskRequestType {name, datacenter, copy, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "ShrinkVirtualDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Overwrite all blocks of the virtual disk with zeros.
    /// 
    /// All data will be lost.
    /// 
    /// The datacenter parameter may be omitted if a URL is used to name the disk.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore where the
    /// virtual disk resides.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the disk, either a datastore path or a URL
    /// referring to the virtual disk whose blocks should be overwritten
    /// with zeroes.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// Refers instance of *Datacenter*.
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
    /// ***FileFault***: if an error occurs zero filling the virtual disk.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the datastore.
    pub async fn zero_fill_virtual_disk_task(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ZeroFillVirtualDiskRequestType {name, datacenter, };
        let bytes = self.client.invoke("", "VirtualDiskManager", &self.mo_id, "ZeroFillVirtualDisk_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct CopyVirtualDiskRequestType<'a> {
    source_name: &'a str,
    source_datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    dest_name: &'a str,
    dest_datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    dest_spec: Option<&'a dyn crate::types::traits::VirtualDiskSpecTrait>,
    force: Option<bool>,
}

impl<'a> miniserde::Serialize for CopyVirtualDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CopyVirtualDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CopyVirtualDiskRequestTypeSer<'b, 'a> {
    data: &'b CopyVirtualDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CopyVirtualDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CopyVirtualDiskRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("sourceName"), &self.data.source_name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.source_datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("sourceDatacenter"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("destName"), &self.data.dest_name as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.dest_datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("destDatacenter"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.dest_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("destSpec"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.force else { continue; };
                    return Some((std::borrow::Cow::Borrowed("force"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CreateVirtualDiskRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    spec: &'a dyn crate::types::traits::VirtualDiskSpecTrait,
}

impl<'a> miniserde::Serialize for CreateVirtualDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateVirtualDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateVirtualDiskRequestTypeSer<'b, 'a> {
    data: &'b CreateVirtualDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateVirtualDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateVirtualDiskRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct DefragmentVirtualDiskRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for DefragmentVirtualDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DefragmentVirtualDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DefragmentVirtualDiskRequestTypeSer<'b, 'a> {
    data: &'b DefragmentVirtualDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DefragmentVirtualDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DefragmentVirtualDiskRequestType")),
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
struct DeleteVirtualDiskRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for DeleteVirtualDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteVirtualDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteVirtualDiskRequestTypeSer<'b, 'a> {
    data: &'b DeleteVirtualDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeleteVirtualDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteVirtualDiskRequestType")),
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
struct EagerZeroVirtualDiskRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for EagerZeroVirtualDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EagerZeroVirtualDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EagerZeroVirtualDiskRequestTypeSer<'b, 'a> {
    data: &'b EagerZeroVirtualDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for EagerZeroVirtualDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EagerZeroVirtualDiskRequestType")),
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
struct ExtendVirtualDiskRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    new_capacity_kb: i64,
    eager_zero: Option<bool>,
}

impl<'a> miniserde::Serialize for ExtendVirtualDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ExtendVirtualDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ExtendVirtualDiskRequestTypeSer<'b, 'a> {
    data: &'b ExtendVirtualDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ExtendVirtualDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ExtendVirtualDiskRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("newCapacityKb"), &self.data.new_capacity_kb as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.eager_zero else { continue; };
                    return Some((std::borrow::Cow::Borrowed("eagerZero"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ImportUnmanagedSnapshotRequestType<'a> {
    vdisk: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    vvol_id: &'a str,
}

impl<'a> miniserde::Serialize for ImportUnmanagedSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ImportUnmanagedSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ImportUnmanagedSnapshotRequestTypeSer<'b, 'a> {
    data: &'b ImportUnmanagedSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ImportUnmanagedSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ImportUnmanagedSnapshotRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vdisk"), &self.data.vdisk as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("vvolId"), &self.data.vvol_id as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct InflateVirtualDiskRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for InflateVirtualDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(InflateVirtualDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct InflateVirtualDiskRequestTypeSer<'b, 'a> {
    data: &'b InflateVirtualDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for InflateVirtualDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"InflateVirtualDiskRequestType")),
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
struct MoveVirtualDiskRequestType<'a> {
    source_name: &'a str,
    source_datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    dest_name: &'a str,
    dest_datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    force: Option<bool>,
    profile: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
}

impl<'a> miniserde::Serialize for MoveVirtualDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MoveVirtualDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MoveVirtualDiskRequestTypeSer<'b, 'a> {
    data: &'b MoveVirtualDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MoveVirtualDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MoveVirtualDiskRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("sourceName"), &self.data.source_name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.source_datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("sourceDatacenter"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("destName"), &self.data.dest_name as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.dest_datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("destDatacenter"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.force else { continue; };
                    return Some((std::borrow::Cow::Borrowed("force"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryVirtualDiskFragmentationRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryVirtualDiskFragmentationRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVirtualDiskFragmentationRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVirtualDiskFragmentationRequestTypeSer<'b, 'a> {
    data: &'b QueryVirtualDiskFragmentationRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryVirtualDiskFragmentationRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVirtualDiskFragmentationRequestType")),
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
struct QueryVirtualDiskGeometryRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryVirtualDiskGeometryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVirtualDiskGeometryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVirtualDiskGeometryRequestTypeSer<'b, 'a> {
    data: &'b QueryVirtualDiskGeometryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryVirtualDiskGeometryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVirtualDiskGeometryRequestType")),
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
struct QueryVirtualDiskUuidRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryVirtualDiskUuidRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVirtualDiskUuidRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVirtualDiskUuidRequestTypeSer<'b, 'a> {
    data: &'b QueryVirtualDiskUuidRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryVirtualDiskUuidRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVirtualDiskUuidRequestType")),
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
struct ReleaseManagedSnapshotRequestType<'a> {
    vdisk: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for ReleaseManagedSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReleaseManagedSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReleaseManagedSnapshotRequestTypeSer<'b, 'a> {
    data: &'b ReleaseManagedSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReleaseManagedSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReleaseManagedSnapshotRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vdisk"), &self.data.vdisk as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct SetVirtualDiskUuidRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    uuid: &'a str,
}

impl<'a> miniserde::Serialize for SetVirtualDiskUuidRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetVirtualDiskUuidRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetVirtualDiskUuidRequestTypeSer<'b, 'a> {
    data: &'b SetVirtualDiskUuidRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetVirtualDiskUuidRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetVirtualDiskUuidRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("uuid"), &self.data.uuid as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct ShrinkVirtualDiskRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    copy: Option<bool>,
}

impl<'a> miniserde::Serialize for ShrinkVirtualDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ShrinkVirtualDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ShrinkVirtualDiskRequestTypeSer<'b, 'a> {
    data: &'b ShrinkVirtualDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ShrinkVirtualDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ShrinkVirtualDiskRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.copy else { continue; };
                    return Some((std::borrow::Cow::Borrowed("copy"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ZeroFillVirtualDiskRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for ZeroFillVirtualDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ZeroFillVirtualDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ZeroFillVirtualDiskRequestTypeSer<'b, 'a> {
    data: &'b ZeroFillVirtualDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ZeroFillVirtualDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ZeroFillVirtualDiskRequestType")),
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
