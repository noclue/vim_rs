use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::HostDiskDimensionsChs;
use crate::types::ManagedObjectReference;
use crate::types::VirtualMachineProfileSpecTrait;
use crate::types::VirtualDiskSpecTrait;
/// Most VirtualDiskManager APIs will be DEPRECATED as of vSphere 6.5.
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
pub struct VirtualDiskManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl VirtualDiskManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
    pub async fn copy_virtual_disk_task(&self, source_name: &str, source_datacenter: Option<&ManagedObjectReference>, dest_name: &str, dest_datacenter: Option<&ManagedObjectReference>, dest_spec: Option<&dyn VirtualDiskSpecTrait>, force: Option<bool>) -> Result<ManagedObjectReference> {
        let input = CopyVirtualDiskRequestType {source_name, source_datacenter, dest_name, dest_datacenter, dest_spec, force, };
        let path = format!("/VirtualDiskManager/{moId}/CopyVirtualDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn create_virtual_disk_task(&self, name: &str, datacenter: Option<&ManagedObjectReference>, spec: &dyn VirtualDiskSpecTrait) -> Result<ManagedObjectReference> {
        let input = CreateVirtualDiskRequestType {name, datacenter, spec, };
        let path = format!("/VirtualDiskManager/{moId}/CreateVirtualDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn defragment_virtual_disk_task(&self, name: &str, datacenter: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = DefragmentVirtualDiskRequestType {name, datacenter, };
        let path = format!("/VirtualDiskManager/{moId}/DefragmentVirtualDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn delete_virtual_disk_task(&self, name: &str, datacenter: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = DeleteVirtualDiskRequestType {name, datacenter, };
        let path = format!("/VirtualDiskManager/{moId}/DeleteVirtualDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn eager_zero_virtual_disk_task(&self, name: &str, datacenter: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = EagerZeroVirtualDiskRequestType {name, datacenter, };
        let path = format!("/VirtualDiskManager/{moId}/EagerZeroVirtualDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn extend_virtual_disk_task(&self, name: &str, datacenter: Option<&ManagedObjectReference>, new_capacity_kb: i64, eager_zero: Option<bool>) -> Result<ManagedObjectReference> {
        let input = ExtendVirtualDiskRequestType {name, datacenter, new_capacity_kb, eager_zero, };
        let path = format!("/VirtualDiskManager/{moId}/ExtendVirtualDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Import an unmanaged-snapshot from Virtual-Volume(VVol) enabled
    /// Storage Array.
    /// 
    /// Storage Array may support users to take snapshots indepedent of
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
    pub async fn import_unmanaged_snapshot(&self, vdisk: &str, datacenter: Option<&ManagedObjectReference>, vvol_id: &str) -> Result<()> {
        let input = ImportUnmanagedSnapshotRequestType {vdisk, datacenter, vvol_id, };
        let path = format!("/VirtualDiskManager/{moId}/ImportUnmanagedSnapshot", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn inflate_virtual_disk_task(&self, name: &str, datacenter: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = InflateVirtualDiskRequestType {name, datacenter, };
        let path = format!("/VirtualDiskManager/{moId}/InflateVirtualDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    /// If true, overwrite any indentically named disk at the destination.
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
    pub async fn move_virtual_disk_task(&self, source_name: &str, source_datacenter: Option<&ManagedObjectReference>, dest_name: &str, dest_datacenter: Option<&ManagedObjectReference>, force: Option<bool>, profile: Option<&[Box<dyn VirtualMachineProfileSpecTrait>]>) -> Result<ManagedObjectReference> {
        let input = MoveVirtualDiskRequestType {source_name, source_datacenter, dest_name, dest_datacenter, force, profile, };
        let path = format!("/VirtualDiskManager/{moId}/MoveVirtualDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn query_virtual_disk_fragmentation(&self, name: &str, datacenter: Option<&ManagedObjectReference>) -> Result<i32> {
        let input = QueryVirtualDiskFragmentationRequestType {name, datacenter, };
        let path = format!("/VirtualDiskManager/{moId}/QueryVirtualDiskFragmentation", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn query_virtual_disk_geometry(&self, name: &str, datacenter: Option<&ManagedObjectReference>) -> Result<HostDiskDimensionsChs> {
        let input = QueryVirtualDiskGeometryRequestType {name, datacenter, };
        let path = format!("/VirtualDiskManager/{moId}/QueryVirtualDiskGeometry", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn query_virtual_disk_uuid(&self, name: &str, datacenter: Option<&ManagedObjectReference>) -> Result<String> {
        let input = QueryVirtualDiskUuidRequestType {name, datacenter, };
        let path = format!("/VirtualDiskManager/{moId}/QueryVirtualDiskUuid", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn release_managed_snapshot(&self, vdisk: &str, datacenter: Option<&ManagedObjectReference>) -> Result<()> {
        let input = ReleaseManagedSnapshotRequestType {vdisk, datacenter, };
        let path = format!("/VirtualDiskManager/{moId}/ReleaseManagedSnapshot", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn set_virtual_disk_uuid(&self, name: &str, datacenter: Option<&ManagedObjectReference>, uuid: &str) -> Result<()> {
        let input = SetVirtualDiskUuidRequestType {name, datacenter, uuid, };
        let path = format!("/VirtualDiskManager/{moId}/SetVirtualDiskUuid", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn shrink_virtual_disk_task(&self, name: &str, datacenter: Option<&ManagedObjectReference>, copy: Option<bool>) -> Result<ManagedObjectReference> {
        let input = ShrinkVirtualDiskRequestType {name, datacenter, copy, };
        let path = format!("/VirtualDiskManager/{moId}/ShrinkVirtualDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn zero_fill_virtual_disk_task(&self, name: &str, datacenter: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = ZeroFillVirtualDiskRequestType {name, datacenter, };
        let path = format!("/VirtualDiskManager/{moId}/ZeroFillVirtualDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CopyVirtualDiskRequestType<'a> {
    #[serde(rename = "sourceName")]
    source_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceDatacenter")]
    source_datacenter: Option<&'a ManagedObjectReference>,
    #[serde(rename = "destName")]
    dest_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "destDatacenter")]
    dest_datacenter: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "destSpec")]
    dest_spec: Option<&'a dyn VirtualDiskSpecTrait>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateVirtualDiskRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    spec: &'a dyn VirtualDiskSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DefragmentVirtualDiskRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteVirtualDiskRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EagerZeroVirtualDiskRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ExtendVirtualDiskRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    #[serde(rename = "newCapacityKb")]
    new_capacity_kb: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eagerZero")]
    eager_zero: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ImportUnmanagedSnapshotRequestType<'a> {
    vdisk: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    #[serde(rename = "vvolId")]
    vvol_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct InflateVirtualDiskRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MoveVirtualDiskRequestType<'a> {
    #[serde(rename = "sourceName")]
    source_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceDatacenter")]
    source_datacenter: Option<&'a ManagedObjectReference>,
    #[serde(rename = "destName")]
    dest_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "destDatacenter")]
    dest_datacenter: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a [Box<dyn VirtualMachineProfileSpecTrait>]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVirtualDiskFragmentationRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVirtualDiskGeometryRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVirtualDiskUuidRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReleaseManagedSnapshotRequestType<'a> {
    vdisk: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetVirtualDiskUuidRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ShrinkVirtualDiskRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    copy: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ZeroFillVirtualDiskRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
}
