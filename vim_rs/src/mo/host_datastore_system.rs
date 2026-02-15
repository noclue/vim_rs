use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
#[derive(Clone)]
pub struct HostDatastoreSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostDatastoreSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let req = self.client.post_json(&path, &input);
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
    pub async fn create_local_datastore(&self, name: &str, path: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateLocalDatastoreRequestType {name, path, };
        let path = format!("/HostDatastoreSystem/{moId}/CreateLocalDatastore", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn create_nas_datastore(&self, spec: &crate::types::structs::HostNasVolumeSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateNasDatastoreRequestType {spec, };
        let path = format!("/HostDatastoreSystem/{moId}/CreateNasDatastore", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn create_vmfs_datastore(&self, spec: &crate::types::structs::VmfsDatastoreCreateSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateVmfsDatastoreRequestType {spec, };
        let path = format!("/HostDatastoreSystem/{moId}/CreateVmfsDatastore", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn create_vvol_datastore(&self, spec: &crate::types::structs::HostDatastoreSystemVvolDatastoreSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateVvolDatastoreRequestType {spec, };
        let path = format!("/HostDatastoreSystem/{moId}/CreateVvolDatastore", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn disable_clustered_vmdk_support(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<()> {
        let input = DisableClusteredVmdkSupportRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/DisableClusteredVmdkSupport", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
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
    pub async fn enable_clustered_vmdk_support(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<()> {
        let input = EnableClusteredVmdkSupportRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/EnableClusteredVmdkSupport", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
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
    pub async fn expand_vmfs_datastore(&self, datastore: &crate::types::structs::ManagedObjectReference, spec: &crate::types::structs::VmfsDatastoreExpandSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ExpandVmfsDatastoreRequestType {datastore, spec, };
        let path = format!("/HostDatastoreSystem/{moId}/ExpandVmfsDatastore", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn extend_vmfs_datastore(&self, datastore: &crate::types::structs::ManagedObjectReference, spec: &crate::types::structs::VmfsDatastoreExtendSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ExtendVmfsDatastoreRequestType {datastore, spec, };
        let path = format!("/HostDatastoreSystem/{moId}/ExtendVmfsDatastore", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn query_available_disks_for_vmfs(&self, datastore: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::HostScsiDisk>>> {
        let input = QueryAvailableDisksForVmfsRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/QueryAvailableDisksForVmfs", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::HostScsiDisk>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn query_max_queue_depth(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<i64> {
        let input = QueryMaxQueueDepthRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/QueryMaxQueueDepth", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: i64 = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn query_unresolved_vmfs_volumes(&self) -> Result<Option<Vec<crate::types::structs::HostUnresolvedVmfsVolume>>> {
        let path = format!("/HostDatastoreSystem/{moId}/QueryUnresolvedVmfsVolumes", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::HostUnresolvedVmfsVolume>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn query_vmfs_datastore_create_options(&self, device_path: &str, vmfs_major_version: Option<i32>) -> Result<Option<Vec<crate::types::structs::VmfsDatastoreOption>>> {
        let input = QueryVmfsDatastoreCreateOptionsRequestType {device_path, vmfs_major_version, };
        let path = format!("/HostDatastoreSystem/{moId}/QueryVmfsDatastoreCreateOptions", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VmfsDatastoreOption>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn query_vmfs_datastore_expand_options(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::VmfsDatastoreOption>>> {
        let input = QueryVmfsDatastoreExpandOptionsRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/QueryVmfsDatastoreExpandOptions", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VmfsDatastoreOption>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn query_vmfs_datastore_extend_options(&self, datastore: &crate::types::structs::ManagedObjectReference, device_path: &str, suppress_expand_candidates: Option<bool>) -> Result<Option<Vec<crate::types::structs::VmfsDatastoreOption>>> {
        let input = QueryVmfsDatastoreExtendOptionsRequestType {datastore, device_path, suppress_expand_candidates, };
        let path = format!("/HostDatastoreSystem/{moId}/QueryVmfsDatastoreExtendOptions", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VmfsDatastoreOption>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn remove_datastore(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<()> {
        let input = RemoveDatastoreRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/RemoveDatastore", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
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
    pub async fn remove_datastore_ex_task(&self, datastore: &[crate::types::structs::ManagedObjectReference]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RemoveDatastoreExRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/RemoveDatastoreEx_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn resignature_unresolved_vmfs_volume_task(&self, resolution_spec: &crate::types::structs::HostUnresolvedVmfsResignatureSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ResignatureUnresolvedVmfsVolumeRequestType {resolution_spec, };
        let path = format!("/HostDatastoreSystem/{moId}/ResignatureUnresolvedVmfsVolume_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn set_max_queue_depth(&self, datastore: &crate::types::structs::ManagedObjectReference, max_qdepth: i64) -> Result<()> {
        let input = SetMaxQueueDepthRequestType {datastore, max_qdepth, };
        let path = format!("/HostDatastoreSystem/{moId}/SetMaxQueueDepth", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
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
    pub async fn update_local_swap_datastore(&self, datastore: Option<&crate::types::structs::ManagedObjectReference>) -> Result<()> {
        let input = UpdateLocalSwapDatastoreRequestType {datastore, };
        let path = format!("/HostDatastoreSystem/{moId}/UpdateLocalSwapDatastore", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Capability vector indicating the available product features.
    pub async fn capabilities(&self) -> Result<crate::types::structs::HostDatastoreSystemCapabilities> {
        let path = format!("/HostDatastoreSystem/{moId}/capabilities", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::HostDatastoreSystemCapabilities = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// List of datastores on this host.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Datastore*.
    pub async fn datastore(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/HostDatastoreSystem/{moId}/datastore", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ManagedObjectReference>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct ConfigureDatastorePrincipalRequestType<'a> {
    user_name: &'a str,
    password: Option<&'a str>,
}

impl<'a> miniserde::Serialize for ConfigureDatastorePrincipalRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ConfigureDatastorePrincipalRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ConfigureDatastorePrincipalRequestTypeSer<'b, 'a> {
    data: &'b ConfigureDatastorePrincipalRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ConfigureDatastorePrincipalRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ConfigureDatastorePrincipalRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("userName"), &self.data.user_name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.password else { continue; };
                    return Some((std::borrow::Cow::Borrowed("password"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CreateLocalDatastoreRequestType<'a> {
    name: &'a str,
    path: &'a str,
}

impl<'a> miniserde::Serialize for CreateLocalDatastoreRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateLocalDatastoreRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateLocalDatastoreRequestTypeSer<'b, 'a> {
    data: &'b CreateLocalDatastoreRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CreateLocalDatastoreRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateLocalDatastoreRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("path"), &self.data.path as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateNasDatastoreRequestType<'a> {
    spec: &'a crate::types::structs::HostNasVolumeSpec,
}

impl<'a> miniserde::Serialize for CreateNasDatastoreRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateNasDatastoreRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateNasDatastoreRequestTypeSer<'b, 'a> {
    data: &'b CreateNasDatastoreRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CreateNasDatastoreRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateNasDatastoreRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateVmfsDatastoreRequestType<'a> {
    spec: &'a crate::types::structs::VmfsDatastoreCreateSpec,
}

impl<'a> miniserde::Serialize for CreateVmfsDatastoreRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateVmfsDatastoreRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateVmfsDatastoreRequestTypeSer<'b, 'a> {
    data: &'b CreateVmfsDatastoreRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CreateVmfsDatastoreRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateVmfsDatastoreRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateVvolDatastoreRequestType<'a> {
    spec: &'a crate::types::structs::HostDatastoreSystemVvolDatastoreSpec,
}

impl<'a> miniserde::Serialize for CreateVvolDatastoreRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateVvolDatastoreRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateVvolDatastoreRequestTypeSer<'b, 'a> {
    data: &'b CreateVvolDatastoreRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CreateVvolDatastoreRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateVvolDatastoreRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DisableClusteredVmdkSupportRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for DisableClusteredVmdkSupportRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DisableClusteredVmdkSupportRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DisableClusteredVmdkSupportRequestTypeSer<'b, 'a> {
    data: &'b DisableClusteredVmdkSupportRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DisableClusteredVmdkSupportRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DisableClusteredVmdkSupportRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct EnableClusteredVmdkSupportRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for EnableClusteredVmdkSupportRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EnableClusteredVmdkSupportRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EnableClusteredVmdkSupportRequestTypeSer<'b, 'a> {
    data: &'b EnableClusteredVmdkSupportRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for EnableClusteredVmdkSupportRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EnableClusteredVmdkSupportRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ExpandVmfsDatastoreRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::VmfsDatastoreExpandSpec,
}

impl<'a> miniserde::Serialize for ExpandVmfsDatastoreRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ExpandVmfsDatastoreRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ExpandVmfsDatastoreRequestTypeSer<'b, 'a> {
    data: &'b ExpandVmfsDatastoreRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ExpandVmfsDatastoreRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ExpandVmfsDatastoreRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ExtendVmfsDatastoreRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::VmfsDatastoreExtendSpec,
}

impl<'a> miniserde::Serialize for ExtendVmfsDatastoreRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ExtendVmfsDatastoreRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ExtendVmfsDatastoreRequestTypeSer<'b, 'a> {
    data: &'b ExtendVmfsDatastoreRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ExtendVmfsDatastoreRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ExtendVmfsDatastoreRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryAvailableDisksForVmfsRequestType<'a> {
    datastore: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryAvailableDisksForVmfsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryAvailableDisksForVmfsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryAvailableDisksForVmfsRequestTypeSer<'b, 'a> {
    data: &'b QueryAvailableDisksForVmfsRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for QueryAvailableDisksForVmfsRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryAvailableDisksForVmfsRequestType")),
                1 => {
                    let Some(ref val) = self.data.datastore else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datastore"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryMaxQueueDepthRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryMaxQueueDepthRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryMaxQueueDepthRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryMaxQueueDepthRequestTypeSer<'b, 'a> {
    data: &'b QueryMaxQueueDepthRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for QueryMaxQueueDepthRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryMaxQueueDepthRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryVmfsDatastoreCreateOptionsRequestType<'a> {
    device_path: &'a str,
    vmfs_major_version: Option<i32>,
}

impl<'a> miniserde::Serialize for QueryVmfsDatastoreCreateOptionsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVmfsDatastoreCreateOptionsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVmfsDatastoreCreateOptionsRequestTypeSer<'b, 'a> {
    data: &'b QueryVmfsDatastoreCreateOptionsRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for QueryVmfsDatastoreCreateOptionsRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVmfsDatastoreCreateOptionsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("devicePath"), &self.data.device_path as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.vmfs_major_version else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vmfsMajorVersion"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryVmfsDatastoreExpandOptionsRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryVmfsDatastoreExpandOptionsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVmfsDatastoreExpandOptionsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVmfsDatastoreExpandOptionsRequestTypeSer<'b, 'a> {
    data: &'b QueryVmfsDatastoreExpandOptionsRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for QueryVmfsDatastoreExpandOptionsRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVmfsDatastoreExpandOptionsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryVmfsDatastoreExtendOptionsRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
    device_path: &'a str,
    suppress_expand_candidates: Option<bool>,
}

impl<'a> miniserde::Serialize for QueryVmfsDatastoreExtendOptionsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVmfsDatastoreExtendOptionsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVmfsDatastoreExtendOptionsRequestTypeSer<'b, 'a> {
    data: &'b QueryVmfsDatastoreExtendOptionsRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for QueryVmfsDatastoreExtendOptionsRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVmfsDatastoreExtendOptionsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("devicePath"), &self.data.device_path as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.suppress_expand_candidates else { continue; };
                    return Some((std::borrow::Cow::Borrowed("suppressExpandCandidates"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveDatastoreRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for RemoveDatastoreRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveDatastoreRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveDatastoreRequestTypeSer<'b, 'a> {
    data: &'b RemoveDatastoreRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RemoveDatastoreRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveDatastoreRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveDatastoreExRequestType<'a> {
    datastore: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for RemoveDatastoreExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveDatastoreExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveDatastoreExRequestTypeSer<'b, 'a> {
    data: &'b RemoveDatastoreExRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RemoveDatastoreExRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveDatastoreExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ResignatureUnresolvedVmfsVolumeRequestType<'a> {
    resolution_spec: &'a crate::types::structs::HostUnresolvedVmfsResignatureSpec,
}

impl<'a> miniserde::Serialize for ResignatureUnresolvedVmfsVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ResignatureUnresolvedVmfsVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ResignatureUnresolvedVmfsVolumeRequestTypeSer<'b, 'a> {
    data: &'b ResignatureUnresolvedVmfsVolumeRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ResignatureUnresolvedVmfsVolumeRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ResignatureUnresolvedVmfsVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("resolutionSpec"), &self.data.resolution_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SetMaxQueueDepthRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
    max_qdepth: i64,
}

impl<'a> miniserde::Serialize for SetMaxQueueDepthRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetMaxQueueDepthRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetMaxQueueDepthRequestTypeSer<'b, 'a> {
    data: &'b SetMaxQueueDepthRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for SetMaxQueueDepthRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetMaxQueueDepthRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("maxQdepth"), &self.data.max_qdepth as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateLocalSwapDatastoreRequestType<'a> {
    datastore: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for UpdateLocalSwapDatastoreRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateLocalSwapDatastoreRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateLocalSwapDatastoreRequestTypeSer<'b, 'a> {
    data: &'b UpdateLocalSwapDatastoreRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for UpdateLocalSwapDatastoreRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateLocalSwapDatastoreRequestType")),
                1 => {
                    let Some(ref val) = self.data.datastore else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datastore"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
