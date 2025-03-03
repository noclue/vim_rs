use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostDatastoreSystemCapabilities;
use crate::types::structs::HostDatastoreSystemVvolDatastoreSpec;
use crate::types::structs::HostNasVolumeSpec;
use crate::types::structs::HostScsiDisk;
use crate::types::structs::HostUnresolvedVmfsResignatureSpec;
use crate::types::structs::HostUnresolvedVmfsVolume;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::VmfsDatastoreCreateSpec;
use crate::types::structs::VmfsDatastoreExpandSpec;
use crate::types::structs::VmfsDatastoreExtendSpec;
use crate::types::structs::VmfsDatastoreOption;
/// This managed object creates and removes datastores from the host.
/// 
/// To a host, a datastore is a storage abstraction that is backed by one
/// of several types of storage volumes:
/// <dl>
/// <dt>**Local file system**</dt>
/// <dd>
/// A datastore that is backed by a local file system volume uses a host native
/// local file system such as NTFS or ext3. The datastore is created by
/// identifying a file path for a directory in which virtual machine data will
/// be stored. When the datastore is deleted, the mapping from the datastore to
/// the file is deleted. The contents of the directory are not deleted.
/// </dd>
/// 
/// <dt>**NAS Volume**</dt>
/// <dd>
/// A datastore that is backed by a network-attached storage device is created
/// by specifying the required data needed to attach the volume to the host.
/// Destroying the datastore detaches the volume from the host.
/// </dd>
/// 
/// <dt>**VMFS**</dt>
/// <dd>
/// A datastore that is backed by a VMware File System (VMFS) is created by
/// specifying a disk with unpartitioned space, the desired disk partition
/// format on the disk, and some VMFS attributes.
/// 
/// An ESX Server system automatically discovers the VMFS volume on attached Logical
/// Unit Numbers (LUNs) on startup and after re-scanning the host bus adapter.
/// Datastores are automatically created. The datastore label is based on the
/// VMFS volume label. If there is a conflict with an existing datastore,
/// it is made unique by appending a suffix. The VMFS volume label will
/// be unchanged.
/// 
/// Destroying the datastore removes the partitions that compose the VMFS volume.
/// </dd>
/// </dl>
/// Datastores are never automatically removed because transient storage
/// connection outages may occur. They must be removed from the host using
/// this interface.
/// 
/// See also *Datastore*.
pub struct HostDatastoreSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostDatastoreSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Configures datastore principal user for the host.
    /// 
    /// All virtual machine-related file I/O is performed under
    /// this user. Configuring datastore principal user
    /// will result in all virtual machine files (configuration, disk,
    /// and so on) being checked for proper access. If necessary, ownership
    /// and permissions are modified. Note that in some environments,
    /// file ownership and permissions modification may not be possible.
    /// For example, virtual machine files stored on NFS cannot be
    /// modified for ownership and permissions if root squashing is
    /// enabled. Ownership and permissions for these files must be
    /// manually changed by a system administrator. In general, if
    /// server process does not have rights to change ownership
    /// and file permissions of virtual machine files, they must
    /// be modified manually. If a virtual machine files are not
    /// read/writeable by this user, virtual machine related operations such as
    /// power on/off, configuration, and so on will fail. This operation
    /// must be performed while in maintenance mode and requires host
    /// reboot.
    /// 
    /// ***Required privileges:*** Host.Config.Maintenance
    ///
    /// ## Parameters:
    ///
    /// ### user_name
    /// Datastore principal user name.
    ///
    /// ### password
    /// Optional password for systems that require password for
    /// user impersonation.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is not in maintenance mode.
    /// 
    /// ***InvalidArgument***: if userName or password is not valid.
    /// 
    /// ***NotSupported***: if this feature is not supported on the host.
    /// 
    /// ***HostConfigFault***: if unable to configure the datastore principal.
    pub async fn configure_datastore_principal(&self, user_name: &str, password: Option<&str>) -> Result<()> {
        let input = ConfigureDatastorePrincipalRequestType {user_name, password, };
        let path = format!("/HostDatastoreSystem/{moId}/ConfigureDatastorePrincipal", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Creates a new local datastore.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of a datastore to create on the local host.
    ///
    /// ### path
    /// The file path for a directory in which the virtual machine data
    /// will be stored.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if a datastore with the same name already exists.
    /// 
    /// ***HostConfigFault***: if unable to create the datastore on host.
    /// 
    /// ***InvalidName***: if name is not valid datastore name
    /// 
    /// ***FileNotFound***: if path doesn't exist
    pub async fn create_local_datastore(&self, name: &str, path: &str) -> Result<ManagedObjectReference> {
        let input = CreateLocalDatastoreRequestType {name, path, };
        let path = format!("/HostDatastoreSystem/{moId}/CreateLocalDatastore", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Creates a new network-attached storage datastore.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The specification for creating a network-attached storage volume.
    ///
    /// ## Returns:
    ///
    /// The newly created datastore.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if a datastore with the same name already exists.
    /// 
    /// ***InvalidArgument***: if the datastore name is invalid, or the spec
    /// is invalid.
    /// 
    /// ***NoVirtualNic***: if VMkernel TCPIP stack is not configured.
    /// 
    /// ***NoGateway***: if VMkernel gateway is not configured.
    /// 
    /// ***AlreadyExists***: if the local path already exists on the host, or
    /// the remote path is already mounted on the host.
    /// 
    /// ***HostConfigFault***: if unable to mount the NAS volume.
    pub async fn create_nas_datastore(&self, spec: &HostNasVolumeSpec) -> Result<ManagedObjectReference> {
        let input = CreateNasDatastoreRequestType {spec, };
        let path = format!("/HostDatastoreSystem/{moId}/CreateNasDatastore", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Creates a new VMFS datastore.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The specification for creating a datastore backed by a VMFS.
    ///
    /// ## Returns:
    ///
    /// The newly created datastore.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if a datastore with the same name already exists.
    /// 
    /// ***InvalidArgument***: if the datastore name is invalid, or the spec
    /// is invalid.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server system.
    /// 
    /// ***HostConfigFault***: if unable to format the VMFS volume or
    /// gather information about the created volume.
    pub async fn create_vmfs_datastore(&self, spec: &VmfsDatastoreCreateSpec) -> Result<ManagedObjectReference> {
        let input = CreateVmfsDatastoreRequestType {spec, };
        let path = format!("/HostDatastoreSystem/{moId}/CreateVmfsDatastore", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Create a Virtual-Volume based datastore
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Specification for creating a Virtual-Volume based datastore.
    ///
    /// ## Returns:
    ///
    /// The newly created datastore.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the storage container could not be found.
    /// 
    /// ***DuplicateName***: if a datastore with the same name already exists.
    /// 
    /// ***HostConfigFault***: if unable to create the datastore on host.
    /// 
    /// ***InvalidName***: if name is not valid datastore name
    pub async fn create_vvol_datastore(&self, spec: &HostDatastoreSystemVvolDatastoreSpec) -> Result<ManagedObjectReference> {
        let input = CreateVvolDatastoreRequestType {spec, };
        let path = format!("/HostDatastoreSystem/{moId}/CreateVvolDatastore", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Disable the clustered vmdk support on specified datastore.
    /// 
    /// This API will fail if there are running VMs on the datastore
    /// which are configured to use clustered VMDK feature.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// Datastore on which clustered vmdk should be
    /// disabled.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if a datastore with the name could not be found.
    /// 
    /// ***HostConfigFault***: if unable to disable clustered vmdk support.
    pub async fn disable_clustered_vmdk_support(&self, datastore: &ManagedObjectReference) -> Result<()> {
        let input = DisableClusteredVmdkSupportRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/DisableClusteredVmdkSupport", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Enable the clustered vmdk support on specified datastore.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// Datastore on which clustered vmdk should be
    /// enabled
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if a datastore with the name could not be found.
    /// 
    /// ***HostConfigFault***: if unable to enable clustered vmdk support.
    pub async fn enable_clustered_vmdk_support(&self, datastore: &ManagedObjectReference) -> Result<()> {
        let input = EnableClusteredVmdkSupportRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/EnableClusteredVmdkSupport", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Increases the capacity of an existing VMFS datastore by expanding
    /// (increasing the size of) an existing extent of the datastore.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The datastore whose capacity should be increased.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### spec
    /// The specification describing which extent of the VMFS
    /// datastore to expand.
    ///
    /// ## Returns:
    ///
    /// The expanded datastore.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if a datastore with the name could not be found.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server.
    /// 
    /// ***HostConfigFault***: if unable to expand the VMFS volume.
    pub async fn expand_vmfs_datastore(&self, datastore: &ManagedObjectReference, spec: &VmfsDatastoreExpandSpec) -> Result<ManagedObjectReference> {
        let input = ExpandVmfsDatastoreRequestType {datastore, spec, };
        let path = format!("/HostDatastoreSystem/{moId}/ExpandVmfsDatastore", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Increases the capacity of an existing VMFS datastore by adding new
    /// extents to the datastore.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The datastore whose capacity should be increased.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### spec
    /// The specification describing what extents to add to a
    /// VMFS datastore.
    ///
    /// ## Returns:
    ///
    /// The extended datastore.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if a datastore with the name could not be found.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server.
    /// 
    /// ***HostConfigFault***: if unable to extend the VMFS volume.
    pub async fn extend_vmfs_datastore(&self, datastore: &ManagedObjectReference, spec: &VmfsDatastoreExtendSpec) -> Result<ManagedObjectReference> {
        let input = ExtendVmfsDatastoreRequestType {datastore, spec, };
        let path = format!("/HostDatastoreSystem/{moId}/ExtendVmfsDatastore", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query to list disks that can be used to contain VMFS datastore extents.
    /// 
    /// If the optional parameter name is supplied, queries for disks that can be
    /// used to contain extents for a VMFS datastore identified by the supplied
    /// name. Otherwise, the method retrieves disks that can be used to contain
    /// new VMFS datastores.
    /// 
    /// This operation will filter out disks that are currently in use by an
    /// existing VMFS unless the VMFS using the disk is one being extended.
    /// It will also filter out management LUNs and disks that are referenced by
    /// RDMs. These disk LUNs are also unsuited for use by a VMFS.
    /// 
    /// Disk LUNs referenced by RDMs are found by examining all virtual machines
    /// known to the system and visiting their virtual disk backends. If a
    /// virtual disk backend uses an RDM that is referencing a disk LUN, the disk
    /// LUN becomes ineligible for use by a VMFS datastore.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The managed object reference of the VMFS datastore
    /// you want extents for.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// An array of data objects describing SCSI disks.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the host is not an ESX Server.
    /// 
    /// ***NotFound***: if the named VMFS datastore is not found.
    /// 
    /// ***InvalidArgument***: if named VMFS datastore is not a VMFS datastore.
    /// 
    /// ***HostConfigFault***: if unable to query disk information.
    pub async fn query_available_disks_for_vmfs(&self, datastore: Option<&ManagedObjectReference>) -> Result<Option<Vec<HostScsiDisk>>> {
        let input = QueryAvailableDisksForVmfsRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/QueryAvailableDisksForVmfs", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Query max queue depth for a specified NFS datastore.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The NFS datastore which need to query max queue depth
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the datastore could not be found.
    pub async fn query_max_queue_depth(&self, datastore: &ManagedObjectReference) -> Result<i64> {
        let input = QueryMaxQueueDepthRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/QueryMaxQueueDepth", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Get the list of unbound VMFS volumes.
    /// 
    /// For sharing a volume across hosts, a VMFS volume is bound to its
    /// underlying block device storage. When a low level block copy is
    /// performed to copy or move the VMFS volume, the copied volume will
    /// be unbound.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// An array of unbound VMFS datastore
    pub async fn query_unresolved_vmfs_volumes(&self) -> Result<Option<Vec<HostUnresolvedVmfsVolume>>> {
        let path = format!("/HostDatastoreSystem/{moId}/QueryUnresolvedVmfsVolumes", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
    /// Queries options for creating a new VMFS datastore for a disk.
    /// 
    /// See also *HostScsiDisk.devicePath*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### device_path
    /// The devicePath of the disk on which datastore creation
    /// options are generated.
    ///
    /// ### vmfs_major_version
    /// major version of VMFS to be used for
    /// formatting the datastore. If this
    /// parameter is not specified, then the highest
    /// *supported VMFS major version* for the host
    /// is used.
    ///
    /// ## Returns:
    ///
    /// An array of VMFS datastore provisioning options that can be
    /// applied on a disk.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the host is not an ESX Server.
    /// 
    /// ***NotFound***: if the device is not found.
    /// 
    /// ***HostConfigFault***: if unable to get the current partition information for
    /// the device.
    pub async fn query_vmfs_datastore_create_options(&self, device_path: &str, vmfs_major_version: Option<i32>) -> Result<Option<Vec<VmfsDatastoreOption>>> {
        let input = QueryVmfsDatastoreCreateOptionsRequestType {device_path, vmfs_major_version, };
        let path = format!("/HostDatastoreSystem/{moId}/QueryVmfsDatastoreCreateOptions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Queries for options for increasing the capacity of an existing VMFS
    /// datastore by expanding (increasing the size of) an existing extent of
    /// the datastore.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The datastore to be expanded.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// An array of VMFS datastore expansion options that can be applied.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified datastore could not be found or is unmounted.
    /// 
    /// ***HostConfigFault***: if unable to get partition information for the
    /// devices on which the extents reside
    /// 
    /// ***NotSupported***: if the host is not an ESX Server.
    pub async fn query_vmfs_datastore_expand_options(&self, datastore: &ManagedObjectReference) -> Result<Option<Vec<VmfsDatastoreOption>>> {
        let input = QueryVmfsDatastoreExpandOptionsRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/QueryVmfsDatastoreExpandOptions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Queries for options for increasing the capacity of an existing VMFS
    /// datastore by adding new extents using space from the specified disk.
    /// 
    /// See also *HostScsiDisk.devicePath*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The datastore to be extended.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### device_path
    /// The devicePath of the disk on which datastore extension
    /// options are generated.
    ///
    /// ### suppress_expand_candidates
    /// Indicates whether to exclude options that can be
    /// used for extent expansion also.
    /// Free space can be used for adding an extent or expanding an existing
    /// extent. If this parameter is set to true, the list of options
    /// returned will not include free space that can be used for expansion.
    ///
    /// ## Returns:
    ///
    /// An array of VMFS datastore provisioning options that can be applied
    /// on a disk.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if a datastore or device with the given name could not be found
    /// or if the datastore is unmounted.
    /// 
    /// ***HostConfigFault***: if unable to get the current partition information for
    /// the device.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server.
    pub async fn query_vmfs_datastore_extend_options(&self, datastore: &ManagedObjectReference, device_path: &str, suppress_expand_candidates: Option<bool>) -> Result<Option<Vec<VmfsDatastoreOption>>> {
        let input = QueryVmfsDatastoreExtendOptionsRequestType {datastore, device_path, suppress_expand_candidates, };
        let path = format!("/HostDatastoreSystem/{moId}/QueryVmfsDatastoreExtendOptions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Removes a datastore from a host.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The datastore to be removed.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the datastore could not be found.
    /// 
    /// ***HostConfigFault***: if unable to umount the NAS volume for NAS
    /// datastore, or gather the existing volume information.
    /// 
    /// ***ResourceInUse***: for a VMFS volume if there is any VM registered
    /// on any host attached to this datastore.
    /// 
    /// ***ResourceInUse***: for a NFS volume if there is any VM residing on
    /// this datastore and registered on this host.
    pub async fn remove_datastore(&self, datastore: &ManagedObjectReference) -> Result<()> {
        let input = RemoveDatastoreRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/RemoveDatastore", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Remove one or more datastores.
    /// 
    /// This is an asynchronous, batch operation of
    /// removeDatastore. Please see *HostDatastoreSystem.RemoveDatastore*
    /// for operational details.
    /// Note: This API currently supports removal of only NFS datastores.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// each element specifies one datastore to be removed.
    /// 
    /// Refers instances of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for host configuration failures.
    pub async fn remove_datastore_ex_task(&self, datastore: &[ManagedObjectReference]) -> Result<ManagedObjectReference> {
        let input = RemoveDatastoreExRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/RemoveDatastoreEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Resignature an unbound VMFS volume.
    /// 
    /// To safely enable sharing of the volume across hosts, a VMFS volume
    /// is bound to its underlying block device storage. When a low level
    /// block copy is performed to copy or move the VMFS volume, the copied
    /// volume will be unbound. In order for the VMFS volume to be usable,
    /// a resolution operation is needed to determine whether the VMFS volume
    /// should be treated as a new volume or not and what extents compose
    /// that volume in the event there is more than one unbound volume.
    /// 
    /// With 'Resignature' operation, a new Vmfs Uuid is assigned to the
    /// volume but its contents are kept intact. Resignature results in a
    /// new Vmfs volume on the host. Users can specify a list of hosts on which
    /// the volume will be auto-mounted.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### resolution_spec
    /// A data object that describes what the disk
    /// extents to be used for creating the new
    /// VMFS volume.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. The task result
    /// (*Task.info*.*TaskInfo.result*) contains a
    /// *HostResignatureRescanResult* object that identifies
    /// the newly created VMFS datastore.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VmfsAmbiguousMount***: when ESX is unable to resolve the extents
    /// of a VMFS volume unambiguously. This is thrown only when
    /// a VMFS volume has multiple extents and multiple copies of
    /// non-head extents are detected, and the user has not
    /// specified one copy of every extent. Please note that some
    /// versions of ESX may not support resolving the situation
    /// where multiple copies of non-head extents are detected,
    /// even if one copy of every extent is specified in the
    /// method parameter. To resolve such a situation, the user
    /// is expected to change the configuration (for example,
    /// using array management tools) so that only one copy of
    /// each non-head extent is presented to ESX.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn resignature_unresolved_vmfs_volume_task(&self, resolution_spec: &HostUnresolvedVmfsResignatureSpec) -> Result<ManagedObjectReference> {
        let input = ResignatureUnresolvedVmfsVolumeRequestType {resolution_spec, };
        let path = format!("/HostDatastoreSystem/{moId}/ResignatureUnresolvedVmfsVolume_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Set max queue depth for a specified NFS datastore.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The NFS datastore which need to set max queue depth
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### max_qdepth
    /// Max queue depth value for a datastore
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the datastore could not be found.
    /// 
    /// ***InvalidArgument***: if max queue depth is not within range.
    pub async fn set_max_queue_depth(&self, datastore: &ManagedObjectReference, max_qdepth: i64) -> Result<()> {
        let input = SetMaxQueueDepthRequestType {datastore, max_qdepth, };
        let path = format!("/HostDatastoreSystem/{moId}/SetMaxQueueDepth", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Choose the
    /// *localSwapDatastore*
    /// for this host.
    /// 
    /// Any change to this setting will affect virtual machines
    /// that subsequently power on or resume from a suspended state at this host,
    /// or that migrate to this host while powered on; virtual machines that are
    /// currently powered on at this host will not yet be affected.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The selected datastore. If this argument is unset, then
    /// the *localSwapDatastore*
    /// property becomes unset. Otherwise, the host must have read/write
    /// access to the indicated datastore.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the datastore argument is set and the
    /// *localSwapDatastoreSupported*
    /// capability is not true for the host.
    /// 
    /// ***InaccessibleDatastore***: if the datastore argument is set and
    /// the host cannot access the indicated datastore.
    /// 
    /// ***DatastoreNotWritableOnHost***: if the datastore argument is set and
    /// the host cannot write to the indicated datastore.
    pub async fn update_local_swap_datastore(&self, datastore: Option<&ManagedObjectReference>) -> Result<()> {
        let input = UpdateLocalSwapDatastoreRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/UpdateLocalSwapDatastore", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Capability vector indicating the available product features.
    pub async fn capabilities(&self) -> Result<HostDatastoreSystemCapabilities> {
        let path = format!("/HostDatastoreSystem/{moId}/capabilities", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// List of datastores on this host.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Datastore*.
    pub async fn datastore(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/HostDatastoreSystem/{moId}/datastore", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConfigureDatastorePrincipalRequestType<'a> {
    #[serde(rename = "userName")]
    user_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    password: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateLocalDatastoreRequestType<'a> {
    name: &'a str,
    path: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateNasDatastoreRequestType<'a> {
    spec: &'a HostNasVolumeSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateVmfsDatastoreRequestType<'a> {
    spec: &'a VmfsDatastoreCreateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateVvolDatastoreRequestType<'a> {
    spec: &'a HostDatastoreSystemVvolDatastoreSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DisableClusteredVmdkSupportRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EnableClusteredVmdkSupportRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ExpandVmfsDatastoreRequestType<'a> {
    datastore: &'a ManagedObjectReference,
    spec: &'a VmfsDatastoreExpandSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ExtendVmfsDatastoreRequestType<'a> {
    datastore: &'a ManagedObjectReference,
    spec: &'a VmfsDatastoreExtendSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryAvailableDisksForVmfsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datastore: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryMaxQueueDepthRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVmfsDatastoreCreateOptionsRequestType<'a> {
    #[serde(rename = "devicePath")]
    device_path: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vmfsMajorVersion")]
    vmfs_major_version: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVmfsDatastoreExpandOptionsRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVmfsDatastoreExtendOptionsRequestType<'a> {
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "devicePath")]
    device_path: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "suppressExpandCandidates")]
    suppress_expand_candidates: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveDatastoreRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveDatastoreExRequestType<'a> {
    datastore: &'a [ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ResignatureUnresolvedVmfsVolumeRequestType<'a> {
    #[serde(rename = "resolutionSpec")]
    resolution_spec: &'a HostUnresolvedVmfsResignatureSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetMaxQueueDepthRequestType<'a> {
    datastore: &'a ManagedObjectReference,
    #[serde(rename = "maxQdepth")]
    max_qdepth: i64,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateLocalSwapDatastoreRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datastore: Option<&'a ManagedObjectReference>,
}
