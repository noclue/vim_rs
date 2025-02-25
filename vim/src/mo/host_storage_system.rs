use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::CustomFieldDef;
use crate::types::structs::FcoeConfigFcoeSpecification;
use crate::types::structs::HostDiskPartitionBlockRange;
use crate::types::structs::HostDiskPartitionInfo;
use crate::types::structs::HostDiskPartitionLayout;
use crate::types::structs::HostDiskPartitionSpec;
use crate::types::structs::HostFileSystemVolumeInfo;
use crate::types::structs::HostInternetScsiHbaAuthenticationProperties;
use crate::types::structs::HostInternetScsiHbaDigestProperties;
use crate::types::structs::HostInternetScsiHbaDiscoveryProperties;
use crate::types::structs::HostInternetScsiHbaIpProperties;
use crate::types::structs::HostInternetScsiHbaParamValue;
use crate::types::structs::HostInternetScsiHbaSendTarget;
use crate::types::structs::HostInternetScsiHbaStaticTarget;
use crate::types::structs::HostInternetScsiHbaTargetSet;
use crate::types::structs::HostMultipathInfoHppLogicalUnitPolicy;
use crate::types::structs::HostMultipathStateInfo;
use crate::types::structs::HostNasVolumeUserInfo;
use crate::types::structs::HostNvmeConnectSpec;
use crate::types::structs::HostNvmeDisconnectSpec;
use crate::types::structs::HostNvmeDiscoverSpec;
use crate::types::structs::HostNvmeDiscoveryLog;
use crate::types::structs::HostPathSelectionPolicyOption;
use crate::types::structs::HostScsiDisk;
use crate::types::structs::HostScsiDiskPartition;
use crate::types::structs::HostStorageArrayTypePolicyOption;
use crate::types::structs::HostStorageDeviceInfo;
use crate::types::structs::HostUnresolvedVmfsResolutionResult;
use crate::types::structs::HostUnresolvedVmfsResolutionSpec;
use crate::types::structs::HostUnresolvedVmfsVolume;
use crate::types::structs::HostVffsSpec;
use crate::types::structs::HostVffsVolume;
use crate::types::structs::HostVmfsSpec;
use crate::types::structs::HostVmfsVolume;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::VmfsConfigOption;
use crate::types::structs::VmfsUnmapBandwidthSpec;
/// This managed object gets and sets configuration information
/// about the host's storage subsystem.
/// 
/// The properties and operations are
/// used to configure the host to make storage available for virtual machines.
/// This object contains properties that are specific to ESX Server and
/// general to both the ESX Server system and the hosted architecture.
pub struct HostStorageSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostStorageSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Adds Send Target entries to the host bus adapter discovery list.
    /// 
    /// The DiscoveryProperties.sendTargetsDiscoveryEnabled flag
    /// must be set to true.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_device
    /// The device of the Internet SCSI HBA adapter.
    ///
    /// ### targets
    /// An array of iSCSI send targets.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***NotFound***: if the discovery list could not be found.
    pub async fn add_internet_scsi_send_targets(&self, i_scsi_hba_device: &str, targets: &[HostInternetScsiHbaSendTarget]) -> Result<()> {
        let input = AddInternetScsiSendTargetsRequestType {i_scsi_hba_device, targets, };
        let path = format!("/HostStorageSystem/{moId}/AddInternetScsiSendTargets", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Adds Static Target entries to the host bus adapter discovery list.
    /// 
    /// The DiscoveryProperty.staticTargetDiscoveryEnabled must be set to true.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_device
    /// The device of the Internet SCSI HBA adapter.
    ///
    /// ### targets
    /// An array of iSCSI static targets to add.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the host bus adaptor discovery list was not found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn add_internet_scsi_static_targets(&self, i_scsi_hba_device: &str, targets: &[HostInternetScsiHbaStaticTarget]) -> Result<()> {
        let input = AddInternetScsiStaticTargetsRequestType {i_scsi_hba_device, targets, };
        let path = format!("/HostStorageSystem/{moId}/AddInternetScsiStaticTargets", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Allow I/O issue to the specified detached ScsiLun.
    /// 
    /// The ScsiLun is
    /// administratively configured on, if the attach operation is successful.
    /// See *HostStorageSystem.DetachScsiLun*.
    /// 
    /// attachScsiLun is part of the Unmount, Detach workflow used
    /// when a device will be permanently removed.
    /// See also *HostStorageSystem.UnmountVmfsVolume*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### lun_uuid
    /// The uuid of the ScsiLun to update.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the device could not be found.
    /// 
    /// ***InvalidState***: if
    /// - The device is already attached.
    ///   i.e. Device state is not 'off' in *ScsiLun.operationalState*.
    /// - The device is unreachable. See *ScsiLun.operationalState*.
    ///   
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn attach_scsi_lun(&self, lun_uuid: &str) -> Result<()> {
        let input = AttachScsiLunRequestType {lun_uuid, };
        let path = format!("/HostStorageSystem/{moId}/AttachScsiLun", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Attach one or more SCSI LUNs.
    /// 
    /// This is an asynchronous, batch operation of
    /// attachScisLun. Please see *HostStorageSystem.AttachScsiLun*
    /// for operational details.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### lun_uuid
    /// each element specifies UUID of LUN to be attached.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for host configuration failures.
    pub async fn attach_scsi_lun_ex_task(&self, lun_uuid: &[String]) -> Result<ManagedObjectReference> {
        let input = AttachScsiLunExRequestType {lun_uuid, };
        let path = format!("/HostStorageSystem/{moId}/AttachScsiLunEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Extends a VMFS by attaching a disk partition as an extent.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_path
    /// The path of the VMFS to extend. See *FileSystemMountInfo*.
    ///
    /// ### extent
    /// A data object that describes the specification of a
    /// Disk partition.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the VMFS cannot be found or is unmounted.
    /// 
    /// ***InvalidArgument***: if the new extent is already used by another
    /// vmfs volume, does not exist, or is of an invalid partition
    /// type.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn attach_vmfs_extent(&self, vmfs_path: &str, extent: &HostScsiDiskPartition) -> Result<()> {
        let input = AttachVmfsExtentRequestType {vmfs_path, extent, };
        let path = format!("/HostStorageSystem/{moId}/AttachVmfsExtent", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Change password for existing NFS user.
    /// 
    /// This method shall be called after
    /// the NFS user has been created on the host.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### password
    /// New password.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: NFS user is not set.
    /// 
    /// ***HostConfigFault***: Unable to set passwords due to host configuration
    /// problem.
    pub async fn change_nfs_user_password(&self, password: &str) -> Result<()> {
        let input = ChangeNfsUserPasswordRequestType {password, };
        let path = format!("/HostStorageSystem/{moId}/ChangeNFSUserPassword", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Clear the NFS user configured on the esx host
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: Unable to clear NFS user due to host configuration
    /// problem.
    pub async fn clear_nfs_user(&self) -> Result<()> {
        let path = format!("/HostStorageSystem/{moId}/ClearNFSUser", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Computes the disk partition information given the desired disk layout.
    /// 
    /// The server computes a new partition information object for a specific
    /// disk representing the desired layout.
    /// 
    /// See also *HostDiskPartitionInfoPartitionFormat_enum*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### device_path
    /// The name of the device path for the specific disk.
    ///
    /// ### layout
    /// A data object that describes the disk partition layout.
    ///
    /// ### partition_format
    /// Specifies the desired partition format to be
    /// computed from the block range.
    /// If partitionFormat is not specified, the existing partitionFormat
    /// on disk is used, if the disk is not blank and mbr otherwise.
    ///
    /// ## Returns:
    ///
    /// A data object that contains information about
    /// the partitions on a disk
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the device could not be found.
    /// 
    /// ***InvalidArgument***: if the layout is invalid.
    /// 
    /// ***HostConfigFault***: if unable to get the current partition information for
    /// the device.
    pub async fn compute_disk_partition_info(&self, device_path: &str, layout: &HostDiskPartitionLayout, partition_format: Option<&str>) -> Result<HostDiskPartitionInfo> {
        let input = ComputeDiskPartitionInfoRequestType {device_path, layout, partition_format, };
        let path = format!("/HostStorageSystem/{moId}/ComputeDiskPartitionInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Computes the disk partition information for the purpose of resizing
    /// a given partition.
    /// 
    /// See also *HostDiskPartitionInfoPartitionFormat_enum*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### partition
    /// The disk partition to resize.
    ///
    /// ### block_range
    /// Specifies the desired block range for the resized
    /// partition. The start of the block range specified should match
    /// that of the current partition.
    ///
    /// ### partition_format
    /// Specifies the desired partition format to be
    /// computed from the block range.
    /// If partitionFormat is not specified, the existing partitionFormat
    /// on disk is used, if the disk is not blank and mbr otherwise.
    ///
    /// ## Returns:
    ///
    /// resized disk partition information
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the device could not be found.
    /// 
    /// ***InvalidArgument***: if blockRange or partition is invalid.
    /// 
    /// ***HostConfigFault***: if unable to get the current partition
    /// information for the device.
    pub async fn compute_disk_partition_info_for_resize(&self, partition: &HostScsiDiskPartition, block_range: &HostDiskPartitionBlockRange, partition_format: Option<&str>) -> Result<HostDiskPartitionInfo> {
        let input = ComputeDiskPartitionInfoForResizeRequestType {partition, block_range, partition_format, };
        let path = format!("/HostStorageSystem/{moId}/ComputeDiskPartitionInfoForResize", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Establish a connection to an NVME controller.
    /// 
    /// As a result, all the namespaces attached to the controller
    /// will be accessible through the adapter.
    /// For more details, see:
    /// - "NVM Express over Fabrics 1.0", Section 3.3,
    ///   "Connect command and response"
    ///   
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### connect_spec
    /// A data object that specifies the parameters
    /// necessary to connect to the controller.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the given HBA or transport target could not be found.
    /// 
    /// ***InvalidArgument***: if the provided spec is not valid.
    /// 
    /// ***NotSupported***: if the adapter does not support the provided
    /// combination of parameters.
    /// 
    /// ***HostConfigFault***: if the host is unable to establish
    /// the connection.
    pub async fn connect_nvme_controller(&self, connect_spec: &HostNvmeConnectSpec) -> Result<()> {
        let input = ConnectNvmeControllerRequestType {connect_spec, };
        let path = format!("/HostStorageSystem/{moId}/ConnectNvmeController", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Establish a connection to one or more NVMe controllers.
    /// 
    /// This is an asynchronous, batch version of the connectNvmeController API.
    /// See *HostStorageSystem.ConnectNvmeController* for details. If supported
    /// on the host in question, *HostCapability.nvmeBatchOperationsSupported*
    /// will be set to true.
    /// An attempt will be made to establish a connection using each of the provided
    /// specifications. There are no transactional guarantees - some of the connections
    /// may succeed and some may fail. In case of any failures, a fault containing
    /// information about the failed attempts to establish a connection will be thrown.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### connect_spec
    /// A list of data objects, each specifying the parameters
    /// necessary to connect to an NVMe controller.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the batch API is not supported on the host in question.
    /// 
    /// ***HostConfigFault***: if any of the attempted connections failed.
    pub async fn connect_nvme_controller_ex_task(&self, connect_spec: Option<&[HostNvmeConnectSpec]>) -> Result<ManagedObjectReference> {
        let input = ConnectNvmeControllerExRequestType {connect_spec, };
        let path = format!("/HostStorageSystem/{moId}/ConnectNvmeControllerEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a software NVME over RDMA adapter.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### rdma_device_name
    /// The device name of the RDMA device
    /// to be used to create the software adapter.
    /// Can be obtained from *HostRdmaDevice.device*.
    ///
    /// ## Errors:
    ///
    /// ***ResourceInUse***: if the RDMA device is already in use
    /// by an NVME over RDMA adapter.
    /// 
    /// ***NotFound***: if the given RDMA device could not be found.
    /// 
    /// ***NotSupported***: if the current configuration of the RDMA
    /// device does not permit the creation of the adapter.
    /// 
    /// ***HostConfigFault***: if the host is unable to create the adapter.
    pub async fn create_nvme_over_rdma_adapter(&self, rdma_device_name: &str) -> Result<()> {
        let input = CreateNvmeOverRdmaAdapterRequestType {rdma_device_name, };
        let path = format!("/HostStorageSystem/{moId}/CreateNvmeOverRdmaAdapter", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Creates a software host bus adapter based on the provided spec.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// A data object that specifices the parameters necessary
    /// to create a software host bus adapter of a specific type.
    ///
    /// ## Errors:
    ///
    /// ***ResourceInUse***: if some of the resources specified in the spec
    /// and needed for adapter creation is in use
    /// 
    /// ***NotFound***: if any of the resources specified in the spec could
    /// not be found.
    /// 
    /// ***NotSupported***: if the configuration requested by the spec
    /// is not supported.
    /// 
    /// ***HostConfigFault***: if the host is unable to create the adapter.
    pub async fn create_software_adapter(&self, spec: &dyn crate::types::traits::HostHbaCreateSpecTrait) -> Result<()> {
        let input = CreateSoftwareAdapterRequestType {spec, };
        let path = format!("/HostStorageSystem/{moId}/CreateSoftwareAdapter", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// For previously detached SCSI Lun, remove the state information from
    /// host.
    /// 
    /// Detach SCSI Lun marks the device where I/Os are not allowed.
    /// Host still maintains the entry of this device and its state.
    /// If a LUN is detached using detachScsiLun, ESX will not automatically
    /// attach this LUN durng a rescan or after a reboot.
    /// See *HostStorageSystem.DetachScsiLun*.
    /// Please note: The API takes 'canonicalName' of the ScsiLun object
    /// instead of the ScsiLun.uuid.
    /// 
    /// See also *ScsiLun.canonicalName*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### lun_canonical_name
    /// The 'canonicalName' of the ScsiLun
    /// whose state needs to be deleted.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for any configuration failures.
    pub async fn delete_scsi_lun_state(&self, lun_canonical_name: &str) -> Result<()> {
        let input = DeleteScsiLunStateRequestType {lun_canonical_name, };
        let path = format!("/HostStorageSystem/{moId}/DeleteScsiLunState", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// For previously unmounted VFFS volume, remove the state information from
    /// host.
    /// 
    /// VFFS volumes mount state is maintained by host.
    /// 
    /// deleteVffsVolumeState is part of the Unmount/Detach workflow used
    /// when the device will be permanently removed.
    /// See also *HostStorageSystem.UnmountVffsVolume*.
    /// If the VFFS volume is unmounted using unmountVffsVolume, ESX maintains
    /// the state of VFFS volume. This API will remove the state from the host.
    /// If the underlying storage device is going to be un-provisioned on the
    /// array side, please detach the storage device.
    /// See also *HostStorageSystem.DetachScsiLun*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vffs_uuid
    /// The VFFS UUID.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for any configuration failures.
    pub async fn delete_vffs_volume_state(&self, vffs_uuid: &str) -> Result<()> {
        let input = DeleteVffsVolumeStateRequestType {vffs_uuid, };
        let path = format!("/HostStorageSystem/{moId}/DeleteVffsVolumeState", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// For previously unmounted VMFS volume, remove the state information from
    /// host.
    /// 
    /// VMFS volumes mount state is maintained by host.
    /// 
    /// deleteVmfsVolumeState is part of the Unmount/Detach workflow used
    /// when the device will be permanently removed.
    /// See also *HostStorageSystem.UnmountVmfsVolume*.
    /// If the VMFS volume is unmounted using unmountVmfsVolume, ESX maintains
    /// the state of VMFS volume. This API will remove the state from the host.
    /// If the underlying storage device is going to be un-provisioned on the
    /// array side, please detach the storage device.
    /// See also *HostStorageSystem.DetachScsiLun*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_uuid
    /// The VMFS UUID.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for any configuration failures.
    pub async fn delete_vmfs_volume_state(&self, vmfs_uuid: &str) -> Result<()> {
        let input = DeleteVmfsVolumeStateRequestType {vmfs_uuid, };
        let path = format!("/HostStorageSystem/{moId}/DeleteVmfsVolumeState", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Destroy a VFFS volume.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vffs_path
    /// The path of the VFFS to destroy. See *FileSystemMountInfo*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the VFFS cannot be found or is unmounted.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***ResourceInUse***: VFFS volume is being used.
    pub async fn destroy_vffs(&self, vffs_path: &str) -> Result<()> {
        let input = DestroyVffsRequestType {vffs_path, };
        let path = format!("/HostStorageSystem/{moId}/DestroyVffs", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Disallow I/O issue to the specified ScsiLun.
    /// 
    /// The ScsiLun is
    /// detached, i.e. administratively configured off until a subsequent
    /// attachScsiLun is performed, if the operation is successful.
    /// See *HostStorageSystem.AttachScsiLun*.
    /// 
    /// detachScsiLun is part of the Unmount / Detach workflow used
    /// when a device will be permanently removed.
    /// See also *HostStorageSystem.UnmountVmfsVolume*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### lun_uuid
    /// The uuid of the ScsiLun device to detach.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the device could not be found.
    /// 
    /// ***InvalidState***: if
    /// - The device is already detached(turned 'off').
    ///   See *ScsiLun.operationalState*.
    ///   
    /// ***ResourceInUse***: if
    /// - This device is backing a Vm with a Raw Device Mapped Virtual
    ///   Disk.
    /// - 1 or more programs have I/O outstanding on this device.
    /// - 1 or more mounted vmfs volumes have extents residing on this
    ///   device.
    /// - The device is configured for use by the host e.g. diagnostic
    ///   partition is configured on this device.
    ///   
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn detach_scsi_lun(&self, lun_uuid: &str) -> Result<()> {
        let input = DetachScsiLunRequestType {lun_uuid, };
        let path = format!("/HostStorageSystem/{moId}/DetachScsiLun", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Detach one or more SCSI LUNs.
    /// 
    /// This is an asynchronous, batch operation of
    /// detachScisLun. Please see *HostStorageSystem.DetachScsiLun*
    /// for operational details.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### lun_uuid
    /// each element specifies UUID of LUN to be detached.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for host configuration failures.
    pub async fn detach_scsi_lun_ex_task(&self, lun_uuid: &[String]) -> Result<ManagedObjectReference> {
        let input = DetachScsiLunExRequestType {lun_uuid, };
        let path = format!("/HostStorageSystem/{moId}/DetachScsiLunEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Disables an enabled path for a Logical Unit.
    /// 
    /// Use the path name from *HostMultipathStateInfoPath*
    /// or *HostMultipathInfoPath*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### path_name
    /// The name of the path to disable.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the LUN could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn disable_multipath_path(&self, path_name: &str) -> Result<()> {
        let input = DisableMultipathPathRequestType {path_name, };
        let path = format!("/HostStorageSystem/{moId}/DisableMultipathPath", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Disconnect from an NVME controller.
    /// 
    /// As a result, all the namespaces attached to the controller
    /// will no longer be accessible through the adapter.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### disconnect_spec
    /// A data object that specifies the parameters
    /// necessary to perform the disconnection.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the given HBA or controller could not be found.
    /// 
    /// ***InvalidArgument***: if the provided spec is not valid.
    /// 
    /// ***NotSupported***: if the adapter does not support the provided
    /// combination of parameters.
    /// 
    /// ***HostConfigFault***: if the host is unable to perform
    /// the disconnect.
    pub async fn disconnect_nvme_controller(&self, disconnect_spec: &HostNvmeDisconnectSpec) -> Result<()> {
        let input = DisconnectNvmeControllerRequestType {disconnect_spec, };
        let path = format!("/HostStorageSystem/{moId}/DisconnectNvmeController", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Disconnect from one or more NVMe controllers.
    /// 
    /// This is an asynchronous, batch version of the disconnectNvmeController API.
    /// See *HostStorageSystem.DisconnectNvmeController* for details. If supported
    /// on the host in question, *HostCapability.nvmeBatchOperationsSupported*
    /// will be set to true.
    /// An attempt will be made to disconnect a controller using each of the provided
    /// specifications. There are no transactional guarantees - some of the disconnections
    /// may succeed and some may fail. In case of any failures, a fault containing
    /// information about the failed attempts to disconnect a controller will be thrown.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### disconnect_spec
    /// A list of data objects, each specifying the parameters
    /// necessary to disconnect an NVMe controller.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the batch API is not supported on the host in question.
    /// 
    /// ***HostConfigFault***: if any of the attempts to disconnect a controller fails.
    pub async fn disconnect_nvme_controller_ex_task(&self, disconnect_spec: Option<&[HostNvmeDisconnectSpec]>) -> Result<ManagedObjectReference> {
        let input = DisconnectNvmeControllerExRequestType {disconnect_spec, };
        let path = format!("/HostStorageSystem/{moId}/DisconnectNvmeControllerEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 8.0. Software FCoE not supported.
    /// 
    /// Initiates FCoE discovery using the given FcoeSpecification.
    /// 
    /// Upon success, discovered VNPorts will have registered with the
    /// system as FCoE HBAs.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### fcoe_spec
    /// -
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if any parameter in the given FcoeSpecification
    /// is invalid.
    /// 
    /// ***HostConfigFault***: if the host is unable to issue FCoE discovery.
    /// 
    /// ***NotFound***: if the given HBA could not be found.
    pub async fn discover_fcoe_hbas(&self, fcoe_spec: &FcoeConfigFcoeSpecification) -> Result<()> {
        let input = DiscoverFcoeHbasRequestType {fcoe_spec, };
        let path = format!("/HostStorageSystem/{moId}/DiscoverFcoeHbas", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Connects to a Discovery Controller and retrieves the Discovery Log
    /// using the provided NvmeDiscoverSpec.
    /// 
    /// For more details, see:
    /// - "NVM Express over Fabrics 1.0", Section 5, "Discovery service"
    ///   
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### discover_spec
    /// A data object that specifies the parameters
    /// necessary to retrieve the Discovery Log.
    ///
    /// ## Returns:
    ///
    /// discoveryLog A data object that represents
    /// the Discovery Log.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the given HBA or transport target could not be found.
    /// 
    /// ***InvalidArgument***: if the provided spec is not valid.
    /// 
    /// ***NotSupported***: if the adapter does not support the provided
    /// combination of parameters.
    /// 
    /// ***HostConfigFault***: if the host is unable to retrieve
    /// the discovery log.
    pub async fn discover_nvme_controllers(&self, discover_spec: &HostNvmeDiscoverSpec) -> Result<HostNvmeDiscoveryLog> {
        let input = DiscoverNvmeControllersRequestType {discover_spec, };
        let path = format!("/HostStorageSystem/{moId}/DiscoverNvmeControllers", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Enables a disabled path for a Logical Unit.
    /// 
    /// Use the path name from *HostMultipathStateInfoPath*
    /// or *HostMultipathInfoPath*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### path_name
    /// The name of the path to enable.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the LUN could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn enable_multipath_path(&self, path_name: &str) -> Result<()> {
        let input = EnableMultipathPathRequestType {path_name, };
        let path = format!("/HostStorageSystem/{moId}/EnableMultipathPath", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Expands a VMFS extent as specified by the Disk partition specification.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_path
    /// The path of the VMFS to expand. See *FileSystemMountInfo*.
    ///
    /// ### extent
    /// The disk partition corresponding to the extent to be
    /// expanded.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the VMFS cannot be found or is unmounted.
    /// 
    /// ***InvalidArgument***: if the extent is not part of the VMFS volume.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn expand_vmfs_extent(&self, vmfs_path: &str, extent: &HostScsiDiskPartition) -> Result<()> {
        let input = ExpandVmfsExtentRequestType {vmfs_path, extent, };
        let path = format!("/HostStorageSystem/{moId}/ExpandVmfsExtent", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Extends a VFFS by attaching a SSD.
    /// 
    /// See also *HostScsiDisk.devicePath*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vffs_path
    /// The path of the VFFS to extend. See *FileSystemMountInfo*.
    ///
    /// ### device_path
    /// Device path of the SSD disk.
    ///
    /// ### spec
    /// A data object that describes the SSD disk partition
    /// information. If this property is not provided, partition
    /// information will be computed and generated.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the VFFS cannot be found or is unmounted.
    /// 
    /// ***InvalidArgument***: if the new SSD is already used by another
    /// VFFS volume, does not exist, or is of an invalid partition
    /// type.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***ResourceInUse***: VFFS volume is being used.
    pub async fn extend_vffs(&self, vffs_path: &str, device_path: &str, spec: Option<&HostDiskPartitionSpec>) -> Result<()> {
        let input = ExtendVffsRequestType {vffs_path, device_path, spec, };
        let path = format!("/HostStorageSystem/{moId}/ExtendVffs", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Format a new VFFS on a SSD disk
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### create_spec
    /// A data object that describes the VFFS volume
    /// creation specification.
    ///
    /// ## Returns:
    ///
    /// A data object that represents the VFFS file system.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if VFFS version is invalid, the SSD disk
    /// does not exist or is of an invalid type.
    /// 
    /// ***AlreadyExists***: if the volume name is already being used
    /// by another volume on the host.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***ResourceInUse***: VFFS volume is being used.
    pub async fn format_vffs(&self, create_spec: &HostVffsSpec) -> Result<HostVffsVolume> {
        let input = FormatVffsRequestType {create_spec, };
        let path = format!("/HostStorageSystem/{moId}/FormatVffs", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Formats a new VMFS on a disk partition.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### create_spec
    /// A data object that describes the VMware File System
    /// (VMFS) creation specification.
    ///
    /// ## Returns:
    ///
    /// A data object that represents the VMFS file system.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if VMFS version specified is not 2 or 3,
    /// if blocksize, lock mode, or volume label are invalid,
    /// the partition does not exist or is of an invalid type.
    /// 
    /// ***AlreadyExists***: if the volume name is already being used
    /// by another volume on the host.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn format_vmfs(&self, create_spec: &HostVmfsSpec) -> Result<HostVmfsVolume> {
        let input = FormatVmfsRequestType {create_spec, };
        let path = format!("/HostStorageSystem/{moId}/FormatVmfs", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Mark a disk to local disk, due to the reason that local disks
    /// behind some controllers might not be recongized as local correctly.
    /// 
    /// Task failure might lose existing claim rules on the disk.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### scsi_disk_uuid
    /// The SCSI disk UUID.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the device could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn mark_as_local_task(&self, scsi_disk_uuid: &str) -> Result<ManagedObjectReference> {
        let input = MarkAsLocalRequestType {scsi_disk_uuid, };
        let path = format!("/HostStorageSystem/{moId}/MarkAsLocal_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Mark a disk to remote disk, which is the opposite operation of
    /// *HostStorageSystem.MarkAsLocal_Task*
    /// Task failure might lose existing claim rules on the disk.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### scsi_disk_uuid
    /// The SCSI disk UUID.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the device could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn mark_as_non_local_task(&self, scsi_disk_uuid: &str) -> Result<ManagedObjectReference> {
        let input = MarkAsNonLocalRequestType {scsi_disk_uuid, };
        let path = format!("/HostStorageSystem/{moId}/MarkAsNonLocal_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Mark a disk to Non-SSD, which is the opposite operation of
    /// *HostStorageSystem.MarkAsSsd_Task*
    /// Task failure might lose existing claim rules on the disk.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### scsi_disk_uuid
    /// The SCSI disk UUID.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the device could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn mark_as_non_ssd_task(&self, scsi_disk_uuid: &str) -> Result<ManagedObjectReference> {
        let input = MarkAsNonSsdRequestType {scsi_disk_uuid, };
        let path = format!("/HostStorageSystem/{moId}/MarkAsNonSsd_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Mark a disk to SSD, due to the reason that SSDs behind some controllers
    /// might not be recongized as SSD correctly.
    /// 
    /// Task failure might lose existing claim rules on the disk.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### scsi_disk_uuid
    /// The SCSI disk UUID.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the device could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn mark_as_ssd_task(&self, scsi_disk_uuid: &str) -> Result<ManagedObjectReference> {
        let input = MarkAsSsdRequestType {scsi_disk_uuid, };
        let path = format!("/HostStorageSystem/{moId}/MarkAsSsd_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 8.0. Software FCoE not supported.
    /// 
    /// Mark or unmark the given FCoE HBA for removal from the host system.
    /// 
    /// Marking an FCoE HBA for removal will result in the HBA
    /// not being discovered upon host reboot. Until reboot,
    /// the HBA remains visible in the storage topology.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### hba_name
    /// -
    ///
    /// ### remove
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the given HBA could not be found.
    /// 
    /// ***InvalidArgument***: if the given HBA is not an FCoE HBA.
    /// 
    /// ***HostConfigFault***: if the host does not support removing the given
    /// HBA.
    pub async fn mark_for_removal(&self, hba_name: &str, remove: bool) -> Result<()> {
        let input = MarkForRemovalRequestType {hba_name, remove, };
        let path = format!("/HostStorageSystem/{moId}/MarkForRemoval", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Marks the specified LUN as perennially reserved.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### lun_uuid
    /// The UUID of the ScsiLun device to be marked as perennially
    /// reserved.
    ///
    /// ### state
    /// State of the ScsiLun perennially reserved flag to be set.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if unable to change perennially reserved state.
    /// 
    /// ***NotFound***: if the device could not be found.
    pub async fn mark_perennially_reserved(&self, lun_uuid: &str, state: bool) -> Result<()> {
        let input = MarkPerenniallyReservedRequestType {lun_uuid, state, };
        let path = format!("/HostStorageSystem/{moId}/MarkPerenniallyReserved", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Marks the specified one or more SCSI LUN's perennially reserved based
    /// on the sate.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### lun_uuid
    /// The UUIDs of the ScsiLun devices that need a change in
    /// the perennially reserved flag state.
    ///
    /// ### state
    /// State of the ScsiLun perennially reserved flag to be set.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn mark_perennially_reserved_ex_task(&self, lun_uuid: Option<&[String]>, state: bool) -> Result<ManagedObjectReference> {
        let input = MarkPerenniallyReservedExRequestType {lun_uuid, state, };
        let path = format!("/HostStorageSystem/{moId}/MarkPerenniallyReservedEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Mount the unmounted VFFS volume.
    /// 
    /// See *HostStorageSystem.UnmountVffsVolume*.
    /// 
    /// mountVffsVolume is part of the Unmount / Detach workflow used
    /// when a device will be permanently removed.
    /// See also *HostStorageSystem.DetachScsiLun*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vffs_uuid
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if VFFS uuid is not found on the host.
    /// 
    /// ***InvalidState***: if
    /// - The volume is already mounted.
    /// - The volume is inaccessible. 
    ///   
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***ResourceInUse***: VFFS volume is being used.
    pub async fn mount_vffs_volume(&self, vffs_uuid: &str) -> Result<()> {
        let input = MountVffsVolumeRequestType {vffs_uuid, };
        let path = format!("/HostStorageSystem/{moId}/MountVffsVolume", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Mount the unmounted Vmfs volume.
    /// 
    /// A newly discovered vmfs volume will be
    /// mounted unless, it has been explicitly unmounted. The default mount
    /// behavior of Vmfs volumes is auto-mount. See *HostStorageSystem.UnmountVmfsVolume*.
    /// 
    /// mountVmfsVolume is part of the Unmount / Detach workflow used
    /// when a device will be permanently removed.
    /// See also *HostStorageSystem.DetachScsiLun*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_uuid
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if VMFS Uuid is not found on the host.
    /// 
    /// ***InvalidState***: if
    /// - The volume is already mounted.
    /// - The volume is inaccessible. 
    ///   
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn mount_vmfs_volume(&self, vmfs_uuid: &str) -> Result<()> {
        let input = MountVmfsVolumeRequestType {vmfs_uuid, };
        let path = format!("/HostStorageSystem/{moId}/MountVmfsVolume", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Mount one or more VMFS volumes.
    /// 
    /// This is an asynchronous, batch operation of
    /// mountVmfsVolume. Please see *HostStorageSystem.MountVmfsVolume*
    /// for operational details.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_uuid
    /// each element specifies the UUID of a VMFS volume to be unmounted.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for host configuration failures.
    pub async fn mount_vmfs_volume_ex_task(&self, vmfs_uuid: &[String]) -> Result<ManagedObjectReference> {
        let input = MountVmfsVolumeExRequestType {vmfs_uuid, };
        let path = format!("/HostStorageSystem/{moId}/MountVmfsVolumeEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Query the list SSD disks that can be used to contain a VFFS volume.
    /// 
    /// If the optional parameter name is supplied, queries for the SSD
    /// disks that can be used to contain extents of the specified VFFS volume. Otherwise,
    /// the method retrieves the SSD disks that can be used to contain the new VFFS volume.
    /// 
    /// This operation will filter out SSD disks that are currently in use by an existing VFFS volume.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vffs_path
    /// The path of the VFFS to extend. See *FileSystemMountInfo*.
    ///
    /// ## Returns:
    ///
    /// An array of data objects descrbing SSD disks.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the named VFFS volume is not found.
    /// 
    /// ***InvalidArgument***: if named VFFS volume is not a VFFS volume
    /// 
    /// ***HostConfigFault***: if unable to query disk information.
    pub async fn query_available_ssds(&self, vffs_path: Option<&str>) -> Result<Option<Vec<HostScsiDisk>>> {
        let input = QueryAvailableSsdsRequestType {vffs_path, };
        let path = format!("/HostStorageSystem/{moId}/QueryAvailableSsds", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Query the NFS user configured on the esx host
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Returns:
    ///
    /// UserInfo objects.
    /// See *HostNasVolumeUserInfo*
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: Unable to get NFS user due to host configuration
    /// problem.
    pub async fn query_nfs_user(&self) -> Result<Option<HostNasVolumeUserInfo>> {
        let path = format!("/HostStorageSystem/{moId}/QueryNFSUser", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Queries the set of path selection policy options.
    /// 
    /// The set of policy
    /// options indicates what path selection policies can be used by a
    /// device managed by native multipathing. Devices managed through native
    /// multipathing are described in the *HostMultipathInfo* data
    /// object.
    /// 
    /// Filtering capabilities are not currently present but may be added in
    /// the future.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// The list of path selection policy descriptions that match the
    /// search criteria. Details about the policies will also be
    /// provided in accordance to the query specification.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for system configuration failures.
    pub async fn query_path_selection_policy_options(&self) -> Result<Option<Vec<HostPathSelectionPolicyOption>>> {
        let path = format!("/HostStorageSystem/{moId}/QueryPathSelectionPolicyOptions", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Queries the set of storage array type policy options.
    /// 
    /// The set of policy
    /// options indicates what storage array type policies can be used by a
    /// device managed by native multipathing. Devices managed through native
    /// multipathing are described in the *HostMultipathInfo* data
    /// object.
    /// 
    /// Filtering capabilities are not currently present but may be added in
    /// the future.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// The list of storage array type policy descriptions.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for system configuration failures.
    pub async fn query_storage_array_type_policy_options(&self) -> Result<Option<Vec<HostStorageArrayTypePolicyOption>>> {
        let path = format!("/HostStorageSystem/{moId}/QueryStorageArrayTypePolicyOptions", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
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
    /// An array of unbound VMFS volumes.
    pub async fn query_unresolved_vmfs_volume(&self) -> Result<Option<Vec<HostUnresolvedVmfsVolume>>> {
        let path = format!("/HostStorageSystem/{moId}/QueryUnresolvedVmfsVolume", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Get the VMFS configuration options, including block size,
    /// unmap granularity.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Returns:
    ///
    /// VMFS configuration options.
    pub async fn query_vmfs_config_option(&self) -> Result<Option<Vec<VmfsConfigOption>>> {
        let path = format!("/HostStorageSystem/{moId}/QueryVmfsConfigOption", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Obtains the latest host storage information related to storage devices, topology,
    /// and file systems.
    /// 
    /// The ESX host updates its storage information asynchronously.
    /// 
    /// This method may update the following inventory elements:
    /// - Devices and storage topology
    ///   (*HostSystem*.*HostSystem.config*.*HostConfigInfo.storageDevice*).
    /// - VMFS and NFS datastores (*HostSystem*.*HostSystem.datastore*).
    /// - File system volumes
    ///   (*HostSystem*.*HostSystem.config*.*HostConfigInfo.fileSystemVolume*).
    ///   
    /// The Server performs asynchronous updates to the inventory. Use the
    /// *PropertyCollector*.*PropertyCollector.WaitForUpdatesEx*
    /// method to obtain the property changes.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    pub async fn refresh_storage_system(&self) -> Result<()> {
        let path = format!("/HostStorageSystem/{moId}/RefreshStorageSystem", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Removes Send Target entries from the host bus adapter discovery list.
    /// 
    /// The DiscoveryProperty.sendTargetsDiscoveryEnabled must be set to true.
    /// If any of the targets provided as parameters are not found in
    /// the existing list, the other targets are removed and an exception
    /// is thrown.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_device
    /// The device of the Internet SCSI HBA adapter.
    ///
    /// ### targets
    /// An array of iSCSI send targets to remove.
    ///
    /// ### force
    /// flag for forced removal of iSCSI send targets.
    /// If unset, force flag will be treated as false.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.0
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if at least one target was not found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn remove_internet_scsi_send_targets(&self, i_scsi_hba_device: &str, targets: &[HostInternetScsiHbaSendTarget], force: Option<bool>) -> Result<()> {
        let input = RemoveInternetScsiSendTargetsRequestType {i_scsi_hba_device, targets, force, };
        let path = format!("/HostStorageSystem/{moId}/RemoveInternetScsiSendTargets", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Removes static target entries from the host bus adapter discovery list.
    /// 
    /// The DiscoveryProperty.staticTargetDiscoveryEnabled must be set to true.
    /// If any of the targets provided as parameters are not found in
    /// the existing list, the other targets are removed and an exception
    /// is thrown.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_device
    /// The device of the Internet SCSI HBA adapter.
    ///
    /// ### targets
    /// An array of iSCSI static targets to remove.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if at least one target was not found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn remove_internet_scsi_static_targets(&self, i_scsi_hba_device: &str, targets: &[HostInternetScsiHbaStaticTarget]) -> Result<()> {
        let input = RemoveInternetScsiStaticTargetsRequestType {i_scsi_hba_device, targets, };
        let path = format!("/HostStorageSystem/{moId}/RemoveInternetScsiStaticTargets", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Removes a software NVME over RDMA adapter.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### hba_device_name
    /// The device name of the NVME over RDMA adapter to be removed.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the given HBA could not be found.
    /// 
    /// ***InvalidArgument***: if the given HBA is not an NVMe over RDMA HBA.
    /// 
    /// ***ResourceInUse***: if the given HBA is in use.
    /// 
    /// ***HostConfigFault***: if the host is unable to remove the given HBA.
    pub async fn remove_nvme_over_rdma_adapter(&self, hba_device_name: &str) -> Result<()> {
        let input = RemoveNvmeOverRdmaAdapterRequestType {hba_device_name, };
        let path = format!("/HostStorageSystem/{moId}/RemoveNvmeOverRdmaAdapter", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Removes a software host bus adapter, if the adapter type allows it.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### hba_device_name
    /// The device name of the adapter to be removed.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the given HBA could not be found.
    /// 
    /// ***InvalidArgument***: if the given adapter type cannot be removed.
    /// 
    /// ***ResourceInUse***: if the given HBA is in use.
    /// 
    /// ***HostConfigFault***: if the host is unable to remove the given HBA.
    pub async fn remove_software_adapter(&self, hba_device_name: &str) -> Result<()> {
        let input = RemoveSoftwareAdapterRequestType {hba_device_name, };
        let path = format!("/HostStorageSystem/{moId}/RemoveSoftwareAdapter", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Scans all host bus adapters to obtain the current list of devices and device topology.
    /// 
    /// The *HostStorageSystem.RescanAllHba* method looks for new devices,
    /// removed devices, and path changes.
    /// 
    /// This method may update the following inventory elements:
    /// - Devices and storage topology
    ///   (*HostSystem*.*HostSystem.config*.*HostConfigInfo.storageDevice*).
    /// - VMFS and NFS datastores (*HostSystem*.*HostSystem.datastore*).
    /// - File system volumes (*HostSystem*.*HostSystem.config*.*HostConfigInfo.fileSystemVolume*).
    ///   
    /// The Server performs asynchronous updates to the inventory. Use the
    /// *PropertyCollector*.*PropertyCollector.WaitForUpdatesEx*
    /// method to obtain the property changes.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if rescan failed.
    pub async fn rescan_all_hba(&self) -> Result<()> {
        let path = format!("/HostStorageSystem/{moId}/RescanAllHba", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Issues a request to rescan a specific host bus adapter
    /// for new storage devices.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### hba_device
    /// The device of the host bus adapter.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the host bus adapter cannot be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn rescan_hba(&self, hba_device: &str) -> Result<()> {
        let input = RescanHbaRequestType {hba_device, };
        let path = format!("/HostStorageSystem/{moId}/RescanHba", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Rescans for new VFFS.
    /// 
    /// The *HostStorageSystem.RefreshStorageSystem* method also performs a VFFS rescan.
    /// 
    /// *HostStorageSystem.RescanVffs* may update the
    /// *HostSystem*.*HostSystem.config*.*HostConfigInfo.fileSystemVolume* property.
    /// The Server performs asynchronous updates to the inventory. Use the
    /// *PropertyCollector*.*PropertyCollector.WaitForUpdatesEx*
    /// method to obtain the property changes.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if configuration fails.
    pub async fn rescan_vffs(&self) -> Result<()> {
        let path = format!("/HostStorageSystem/{moId}/RescanVffs", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Rescans for new Virtual Machine File Systems (VMFS).
    /// 
    /// The *HostStorageSystem.RefreshStorageSystem* method also performs a VMFS rescan.
    /// 
    /// *HostStorageSystem.RescanVmfs* may update the
    /// *HostSystem*.*HostSystem.config*.*HostConfigInfo.fileSystemVolume* property.
    /// The Server performs asynchronous updates to the inventory. Use the
    /// *PropertyCollector*.*PropertyCollector.WaitForUpdatesEx*
    /// method to obtain the property changes.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if configuration fails.
    pub async fn rescan_vmfs(&self) -> Result<()> {
        let path = format!("/HostStorageSystem/{moId}/RescanVmfs", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Resignature or 'Force Mount' list of unbound VMFS volumes.
    /// 
    /// To safely enable sharing of the volume across hosts, a VMFS volume
    /// is bound to its underlying block device storage. When a low level
    /// block copy is performed to copy or move the VMFS volume, the copied
    /// volume will be unbound. In order for the VMFS volume to be usable,
    /// a resolution operation is needed to determine whether the VMFS volume
    /// should be treated as a new volume or not and what extents compose
    /// that volume in the event there is more than one unbound volume.
    /// 
    /// Resignature results in a new VMFS volume on the host.
    /// Operations performed at the StorageSystem interface apply only to a
    /// specific host. Hence, callers of this method are responsible for
    /// issuing rescan operations to detect the new VMFS volume on other hosts.
    /// Alternatively, callers that want VirtualCenter to handle rescanning
    /// the necessary hosts should use the DatastoreSystem interface.
    /// 
    /// When user wants to keep the original Vmfs Uuid and mount it
    /// on the host, set the 'resolutionSpec.uuidResolution' to 'forceMounted'
    /// This is per-host operation.
    /// It will return an array of ResolutionResult describing success or failure
    /// associated with each specification.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### resolution_spec
    /// List of data object that describes what the disk
    /// extents to be used for creating the new
    /// VMFS volume.
    ///
    /// ## Returns:
    ///
    /// A data object that represents the VMFS file system and return status value.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if batch operation fails on the host.
    /// Because the returned array of ResolutionResult contains the new VMFS volume
    /// or fault for each operation, as of vSphere API 5.x, we won't throw fault when
    /// batch operation fails.
    pub async fn resolve_multiple_unresolved_vmfs_volumes(&self, resolution_spec: &[HostUnresolvedVmfsResolutionSpec]) -> Result<Option<Vec<HostUnresolvedVmfsResolutionResult>>> {
        let input = ResolveMultipleUnresolvedVmfsVolumesRequestType {resolution_spec, };
        let path = format!("/HostStorageSystem/{moId}/ResolveMultipleUnresolvedVmfsVolumes", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Resignature or 'Force Mount' list of unbound VMFS volumes.
    /// 
    /// To safely enable sharing of the volume across hosts, a VMFS volume
    /// is bound to its underlying block device storage. When a low level
    /// block copy is performed to copy or move the VMFS volume, the copied
    /// volume will be unbound. In order for the VMFS volume to be usable,
    /// a resolution operation is needed to determine whether the VMFS volume
    /// should be treated as a new volume or not and what extents compose
    /// that volume in the event there is more than one unbound volume.
    /// 
    /// Resignature results in a new VMFS volume on the host.
    /// Operations performed at the *HostStorageSystem* interface apply only to a
    /// specific host. Hence, callers of this method are responsible for
    /// issuing rescan operations to detect the new VMFS volume on other hosts.
    /// Alternatively, callers that want VirtualCenter to handle rescanning
    /// the necessary hosts should use the *HostDatastoreSystem* interface.
    /// 
    /// When user wants to keep the original VMFS UUID and mount it
    /// on the host, set the resolutionSpec.uuidResolution
    /// (*HostUnresolvedVmfsResolutionSpec.uuidResolution*)
    /// to *forceMount*.
    /// This is per-host operation.
    /// 
    /// It will return an array of *HostUnresolvedVmfsResolutionResult*
    /// describing success or failure associated with each specification.
    /// 
    /// This method behaves the same as *HostStorageSystem.ResolveMultipleUnresolvedVmfsVolumes*
    /// except that it returns a task to support monitoring the operation.
    /// This is important for operations with large number of
    /// unresolved volumes which may take potentially dozens of minutes to complete.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### resolution_spec
    /// List of data object that describes what the disk
    /// extents to be used for creating the new
    /// VMFS volume.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains an array of *HostUnresolvedVmfsResolutionResult*
    /// describing success or failure associated with each specification.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if batch operation fails on the host.
    /// Because the returned array of ResolutionResult contains the new VMFS volume
    /// or fault for each operation, as of vSphere API 5.x, we won't throw fault when
    /// batch operation fails.
    pub async fn resolve_multiple_unresolved_vmfs_volumes_ex_task(&self, resolution_spec: &[HostUnresolvedVmfsResolutionSpec]) -> Result<ManagedObjectReference> {
        let input = ResolveMultipleUnresolvedVmfsVolumesExRequestType {resolution_spec, };
        let path = format!("/HostStorageSystem/{moId}/ResolveMultipleUnresolvedVmfsVolumesEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Gets the partition information for the disks named by the device names.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### device_path
    /// An array of device path names that identify disks.
    /// See *ScsiDisk*.
    ///
    /// ## Returns:
    ///
    /// An array of information about the partitions.
    pub async fn retrieve_disk_partition_info(&self, device_path: &[String]) -> Result<Option<Vec<HostDiskPartitionInfo>>> {
        let input = RetrieveDiskPartitionInfoRequestType {device_path, };
        let path = format!("/HostStorageSystem/{moId}/RetrieveDiskPartitionInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Assigns a value to a custom field.
    /// 
    /// The setCustomValue method requires
    /// whichever updatePrivilege is defined as one of the
    /// *CustomFieldDef.fieldInstancePrivileges*
    /// for the CustomFieldDef whose value is being changed.
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The name of the field whose value is to be updated.
    ///
    /// ### value
    /// Value to be assigned to the custom field.
    pub async fn set_custom_value(&self, key: &str, value: &str) -> Result<()> {
        let input = SetCustomValueRequestType {key, value, };
        let path = format!("/HostStorageSystem/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the path selection policy for a Logical Unit.
    /// 
    /// Use the LUN uuid from *HostMultipathInfoLogicalUnit*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### lun_id
    /// The logical unit ID
    ///
    /// ### policy
    /// A data object that describes a path selection policy for
    /// the logical unit.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the LUN could not be found.
    /// 
    /// ***InvalidArgument***: if the policy is invalid.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn set_multipath_lun_policy(&self, lun_id: &str, policy: &dyn crate::types::traits::HostMultipathInfoLogicalUnitPolicyTrait) -> Result<()> {
        let input = SetMultipathLunPolicyRequestType {lun_id, policy, };
        let path = format!("/HostStorageSystem/{moId}/SetMultipathLunPolicy", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Set NFS username and password on the host.
    /// 
    /// The specified password is
    /// stored encrypted at the host and overwrites any previous password
    /// configuration. This information is only needed when the host has
    /// mounted NFS volumes with security types that require user
    /// credentials for accessing data. The password is used to acquire
    /// credentials that the NFS client needs to use in order to secure NFS
    /// traffic using RPCSECGSS. The client will access files on all volumes
    /// mounted on this host (that are mounted with the relevant security
    /// type) on behalf of specified user.
    /// 
    /// At present, this API supports only file system NFSv4.1.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### user
    /// User to be saved on the the esx host
    ///
    /// ### password
    /// Password for the user.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: Unable to set user/passwords due to host configuration
    /// problem.
    pub async fn set_nfs_user(&self, user: &str, password: &str) -> Result<()> {
        let input = SetNfsUserRequestType {user, password, };
        let path = format!("/HostStorageSystem/{moId}/SetNFSUser", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Turn off one or more disk locator LEDs.
    /// 
    /// This is a batch operation to turn off one or more disk locator LEDs,
    /// which is the opposite operation of *HostStorageSystem.TurnDiskLocatorLedOn_Task*
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### scsi_disk_uuids
    /// The SCSI disk UUIDs for which the disk locator LED
    /// should be turned off.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for host configuration failures.
    pub async fn turn_disk_locator_led_off_task(&self, scsi_disk_uuids: &[String]) -> Result<ManagedObjectReference> {
        let input = TurnDiskLocatorLedOffRequestType {scsi_disk_uuids, };
        let path = format!("/HostStorageSystem/{moId}/TurnDiskLocatorLedOff_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Turn on one or more disk locator LEDs, duration is the maximum that
    /// hardware can support.
    /// 
    /// This is a batch operation to turn on one or more disk locator LEDs,
    /// so that user can easily locate the ScsiDisk on physical infrastructure.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### scsi_disk_uuids
    /// The SCSI disk UUIDs for which the disk locator LED
    /// should be turned on.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for host configuration failures.
    pub async fn turn_disk_locator_led_on_task(&self, scsi_disk_uuids: &[String]) -> Result<ManagedObjectReference> {
        let input = TurnDiskLocatorLedOnRequestType {scsi_disk_uuids, };
        let path = format!("/HostStorageSystem/{moId}/TurnDiskLocatorLedOn_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Unmap one or more VMFS volumes.
    /// 
    /// This is an asynchronous, batch operation.
    /// The operation unmaps free blocks in each VMFS volume.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_uuid
    /// each element specifies the UUID of a VMFS volume to be unmapped.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for host configuration failures.
    pub async fn unmap_vmfs_volume_ex_task(&self, vmfs_uuid: &[String]) -> Result<ManagedObjectReference> {
        let input = UnmapVmfsVolumeExRequestType {vmfs_uuid, };
        let path = format!("/HostStorageSystem/{moId}/UnmapVmfsVolumeEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Unmount the 'forceMounted' Vmfs volume.
    /// 
    /// When a low level block copy is performed to copy or move the
    /// VMFS volume, the copied volume is unresolved. For the VMFS
    /// volume to be usable, a resolution operation is applied. As
    /// part of resolution operation, user may decide to keep the
    /// original VMFS Uuid. Once the resolution is applied, the VMFS
    /// volume is mounted on the host for its use. User can unmount
    /// the VMFS volume if it is not being used by any registered
    /// VMs.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_uuid
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if VMFS Uuid is not found on the host.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn unmount_force_mounted_vmfs_volume(&self, vmfs_uuid: &str) -> Result<()> {
        let input = UnmountForceMountedVmfsVolumeRequestType {vmfs_uuid, };
        let path = format!("/HostStorageSystem/{moId}/UnmountForceMountedVmfsVolume", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Unmount the VFFS volume.
    /// 
    /// An unmounted volume cannot be used for any
    /// filesystem operation requiring I/O. In contrast to removal, this
    /// operation does not destroy or alter partitions on which VFFS volumes
    /// reside. The mountState will be persisted across filesystem rescans and
    /// host reboots. See *HostStorageSystem.MountVffsVolume*.
    /// 
    /// unmountVffsVolume is part of the Unmount / Detach workflow used
    /// when a device will be permanently removed.
    /// See also *HostStorageSystem.DetachScsiLun*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vffs_uuid
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if VFFS uuid is not found on the host.
    /// 
    /// ***InvalidState***: if
    /// - The volume is already unmounted.
    /// - The volume is inaccessible.
    ///   
    /// ***ResourceInUse***: if
    /// - 1 or more programs have I/O outstanding on this volume.
    ///   
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***ResourceInUse***: VFFS volume is being used.
    pub async fn unmount_vffs_volume(&self, vffs_uuid: &str) -> Result<()> {
        let input = UnmountVffsVolumeRequestType {vffs_uuid, };
        let path = format!("/HostStorageSystem/{moId}/UnmountVffsVolume", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Unmount the Vmfs volume.
    /// 
    /// An unmounted volume cannot be used for any
    /// filesystem operation requiring I/O. In contrast to removal, this
    /// operation does not destroy or alter partitions on which vmfs volumes
    /// reside. The mountState will be persisted across filesystem rescans and
    /// host reboots. See *HostStorageSystem.MountVmfsVolume*.
    /// 
    /// unmountVmfsVolume is part of the Unmount / Detach workflow used
    /// when a device will be permanently removed.
    /// 
    ///                          Mounted Vmfs Volume
    ///         unmountVmfsVolume  |  ^ mountVmfsVolume
    ///                            V  |
    ///                          Unmounted Vmfs Volume
    ///     
    ///              Attached Scsi Device (honors I/O)
    ///         detachScsiLun      |  ^ attachScsiLun
    ///                            V  |
    ///       Detached Scsi Device (does not honor I/O)
    /// 
    /// It is safe to unprovision a Lun from the Storage array \*only\*
    /// after a Scsi device is detached.
    /// 
    /// The best practice for decommisioning a Lun would be to find
    /// out the set of subsystems that a Lun is being used for.
    /// Many of the systems are listed as exceptions in the
    /// function documentation.
    /// 
    /// One typical workflow could be:
    /// - Find out if the device is used as a Vmfs Extent. (See VmfsVolume.Extent API)
    /// - Unmount the Vmfs Volume.
    /// - Find out if device is used by the Diagnostic system (See Diagnostic System API).
    /// - Deactivate the diagnostic system, if it is being used.
    /// - Find out if this device is used to back a VM's RDM (See VirtualMachine API).
    /// - Remove this device from the VM.
    /// - Detach the Scsi device.
    /// - On success, it is safe to decommision the Lun at this point.
    ///   
    /// See also *HostStorageSystem.DetachScsiLun*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_uuid
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if VMFS Uuid is not found on the host.
    /// 
    /// ***InvalidState***: if
    /// - The volume is already unmounted.
    /// - The volume is inaccessible.
    ///   
    /// ***ResourceInUse***: if
    /// - There is any VM registered on this volume.
    /// - 1 or more programs have I/O outstanding on this volume.
    ///   
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn unmount_vmfs_volume(&self, vmfs_uuid: &str) -> Result<()> {
        let input = UnmountVmfsVolumeRequestType {vmfs_uuid, };
        let path = format!("/HostStorageSystem/{moId}/UnmountVmfsVolume", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Unmount one or more VMFS volumes.
    /// 
    /// This is an asynchronous, batch operation of
    /// unmountVmfsVolume. Please see *HostStorageSystem.UnmountVmfsVolume*
    /// for operational details.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_uuid
    /// each element specifies the UUID of a VMFS volume to be unmounted.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for host configuration failures.
    pub async fn unmount_vmfs_volume_ex_task(&self, vmfs_uuid: &[String]) -> Result<ManagedObjectReference> {
        let input = UnmountVmfsVolumeExRequestType {vmfs_uuid, };
        let path = format!("/HostStorageSystem/{moId}/UnmountVmfsVolumeEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Changes the partitions on the disk by supplying a partition specification
    /// and the device name.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### device_path
    /// The name of the device path for the specific disk.
    ///
    /// ### spec
    /// A data object that describes the disk partition table
    /// specification used to configure the partitions on a disk.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the device could not be found.
    /// 
    /// ***InvalidArgument***: if the spec is invalid.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_disk_partitions(&self, device_path: &str, spec: &HostDiskPartitionSpec) -> Result<()> {
        let input = UpdateDiskPartitionsRequestType {device_path, spec, };
        let path = format!("/HostStorageSystem/{moId}/UpdateDiskPartitions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the path selection policy for a HPP claimed Logical Unit.
    /// 
    /// Use the LUN uuid from *HostMultipathInfoLogicalUnit*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### lun_id
    /// The logical unit ID
    ///
    /// ### policy
    /// A data object that describes a path selection policy and
    /// its configuration for the logical unit.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the LUN could not be found.
    /// 
    /// ***InvalidArgument***: if the policy is invalid.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_hpp_multipath_lun_policy(&self, lun_id: &str, policy: &HostMultipathInfoHppLogicalUnitPolicy) -> Result<()> {
        let input = UpdateHppMultipathLunPolicyRequestType {lun_id, policy, };
        let path = format!("/HostStorageSystem/{moId}/UpdateHppMultipathLunPolicy", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the advanced options the iSCSI host bus adapter or the
    /// discovery addresses and targets associated with it.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_device
    /// The device of the Internet SCSI HBA adapter.
    ///
    /// ### target_set
    /// The set the targets to configure. If not provided,
    /// the settings will be applied to the host bus adapter itself.
    ///
    /// ### options
    /// The list of options to set.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the host bus adapter could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_internet_scsi_advanced_options(&self, i_scsi_hba_device: &str, target_set: Option<&HostInternetScsiHbaTargetSet>, options: &[HostInternetScsiHbaParamValue]) -> Result<()> {
        let input = UpdateInternetScsiAdvancedOptionsRequestType {i_scsi_hba_device, target_set, options, };
        let path = format!("/HostStorageSystem/{moId}/UpdateInternetScsiAdvancedOptions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the alias of an iSCSI host bus adapter.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_device
    /// The device of the Internet SCSI HBA adapter.
    ///
    /// ### i_scsi_alias
    /// The new value for the alias of the adapter.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the host bus adapter could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_internet_scsi_alias(&self, i_scsi_hba_device: &str, i_scsi_alias: &str) -> Result<()> {
        let input = UpdateInternetScsiAliasRequestType {i_scsi_hba_device, i_scsi_alias, };
        let path = format!("/HostStorageSystem/{moId}/UpdateInternetScsiAlias", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the authentication properties for one or more targets or
    /// discovery addresses associated with an iSCSI host bus adapter.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_device
    /// The device of the Internet SCSI HBA adapter.
    /// associated with the target.
    ///
    /// ### authentication_properties
    /// The data object that represents
    /// the authentication settings to set.
    ///
    /// ### target_set
    /// The set the targets to configure. Optional,
    /// when obmitted will configura the authentication properties
    /// for the adapter instead.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the host bus adapter could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_internet_scsi_authentication_properties(&self, i_scsi_hba_device: &str, authentication_properties: &HostInternetScsiHbaAuthenticationProperties, target_set: Option<&HostInternetScsiHbaTargetSet>) -> Result<()> {
        let input = UpdateInternetScsiAuthenticationPropertiesRequestType {i_scsi_hba_device, authentication_properties, target_set, };
        let path = format!("/HostStorageSystem/{moId}/UpdateInternetScsiAuthenticationProperties", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the digest properties for the iSCSI host bus adapter or the
    /// discovery addresses and targets associated with it.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_device
    /// The device of the Internet SCSI HBA adapter.
    ///
    /// ### target_set
    /// The set the targets to configure. If not provided,
    /// the settings will be applied to the host bus adapter itself.
    ///
    /// ### digest_properties
    /// The data object that represents the digest
    /// settings to set.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the host bus adapter could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_internet_scsi_digest_properties(&self, i_scsi_hba_device: &str, target_set: Option<&HostInternetScsiHbaTargetSet>, digest_properties: &HostInternetScsiHbaDigestProperties) -> Result<()> {
        let input = UpdateInternetScsiDigestPropertiesRequestType {i_scsi_hba_device, target_set, digest_properties, };
        let path = format!("/HostStorageSystem/{moId}/UpdateInternetScsiDigestProperties", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the Discovery properties for an iSCSI host bus adapter.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_device
    /// The device of the Internet SCSI HBA adapter.
    ///
    /// ### discovery_properties
    /// The discovery settings for this host bus adapter.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the host bus adapter could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_internet_scsi_discovery_properties(&self, i_scsi_hba_device: &str, discovery_properties: &HostInternetScsiHbaDiscoveryProperties) -> Result<()> {
        let input = UpdateInternetScsiDiscoveryPropertiesRequestType {i_scsi_hba_device, discovery_properties, };
        let path = format!("/HostStorageSystem/{moId}/UpdateInternetScsiDiscoveryProperties", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the IP properties for an iSCSI host bus adapter.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_device
    /// The device of the Internet SCSI HBA adapter.
    ///
    /// ### ip_properties
    /// A data object representing the IP properties
    /// for the host bus adapter
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the host bus adapter could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_internet_scsi_ip_properties(&self, i_scsi_hba_device: &str, ip_properties: &HostInternetScsiHbaIpProperties) -> Result<()> {
        let input = UpdateInternetScsiIpPropertiesRequestType {i_scsi_hba_device, ip_properties, };
        let path = format!("/HostStorageSystem/{moId}/UpdateInternetScsiIPProperties", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the name of an iSCSI host bus adapter.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_device
    /// The current name of the Internet SCSI HBA adapter.
    ///
    /// ### i_scsi_name
    /// The new name for the Internet SCSI HBA adapter
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the host bus adapter could not be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_internet_scsi_name(&self, i_scsi_hba_device: &str, i_scsi_name: &str) -> Result<()> {
        let input = UpdateInternetScsiNameRequestType {i_scsi_hba_device, i_scsi_name, };
        let path = format!("/HostStorageSystem/{moId}/UpdateInternetScsiName", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update the mutable display name associated with a ScsiLun.
    /// 
    /// The ScsiLun
    /// to be updated is identified using the specified uuid.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### lun_uuid
    /// The uuid of the ScsiLun to update.
    ///
    /// ### display_name
    /// The new name to assign to the ScsiLun object.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the device could not be found.
    /// 
    /// ***InvalidName***: if the name does not meet name restrictions such
    /// as an 80 character limit.
    /// 
    /// ***DuplicateName***: if the name does not name uniqueness restrictions.
    /// Name uniqueness restrictions will vary based on the context in
    /// which this method is invoked.
    /// 
    /// When this method is invoked on a host directly, no uniqueness
    /// checks will be performed on the name.
    /// 
    /// When this method is invoked on a VC server, uniqueness checks
    /// will be performed on the name. The uniqueness check will
    /// ensure that the name is unique with respect to the entire
    /// VC instance.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_scsi_lun_display_name(&self, lun_uuid: &str, display_name: &str) -> Result<()> {
        let input = UpdateScsiLunDisplayNameRequestType {lun_uuid, display_name, };
        let path = format!("/HostStorageSystem/{moId}/UpdateScsiLunDisplayName", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Enables or disables Software iSCSI.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### enabled
    /// True to enable the iSCSI; false to disable it
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: for any configuration failure.
    pub async fn update_software_internet_scsi_enabled(&self, enabled: bool) -> Result<()> {
        let input = UpdateSoftwareInternetScsiEnabledRequestType {enabled, };
        let path = format!("/HostStorageSystem/{moId}/UpdateSoftwareInternetScsiEnabled", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update VMFS unmap bandwidth.
    /// 
    /// This API updates the value of
    /// *VmfsUnmapBandwidthSpec.policy*,
    /// *VmfsUnmapBandwidthSpec.fixedValue*,
    /// *VmfsUnmapBandwidthSpec.dynamicMin*,
    /// *VmfsUnmapBandwidthSpec.dynamicMax*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_uuid
    /// The VMFS UUID.
    ///
    /// ### unmap_bandwidth_spec
    /// Unmap bandwidth specification. See
    /// *VmfsUnmapBandwidthSpec*
    pub async fn update_vmfs_unmap_bandwidth(&self, vmfs_uuid: &str, unmap_bandwidth_spec: &VmfsUnmapBandwidthSpec) -> Result<()> {
        let input = UpdateVmfsUnmapBandwidthRequestType {vmfs_uuid, unmap_bandwidth_spec, };
        let path = format!("/HostStorageSystem/{moId}/UpdateVmfsUnmapBandwidth", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update VMFS unmap priority.
    /// 
    /// This API updates the value of *HostVmfsVolume.unmapPriority*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_uuid
    /// The VMFS UUID.
    ///
    /// ### unmap_priority
    /// Unmap priority. The supported values are defined in
    /// *HostVmfsVolumeUnmapPriority_enum*.
    pub async fn update_vmfs_unmap_priority(&self, vmfs_uuid: &str, unmap_priority: &str) -> Result<()> {
        let input = UpdateVmfsUnmapPriorityRequestType {vmfs_uuid, unmap_priority, };
        let path = format!("/HostStorageSystem/{moId}/UpdateVmfsUnmapPriority", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Iterates over all registered virtual machines.
    /// 
    /// For each VM which .vmx file
    /// is located on the service console and all disks are available on VMFS3 or NAS,
    /// it will relocate the disks into directories if stored in the ROOT, and
    /// relocate the VMX file into the directory too. Events are logged for each
    /// virtual machine that is relocated.
    /// 
    /// On ESXi systems, this operation has no effect.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    pub async fn upgrade_vm_layout(&self) -> Result<()> {
        let path = format!("/HostStorageSystem/{moId}/UpgradeVmLayout", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Upgrades the VMFS to the *latest
    /// supported VMFS version*.
    /// 
    /// Prerequisite:  
    /// All hosts that have mounted the volume must support the VMFS
    /// version to which the volume will be upgraded.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vmfs_path
    /// The path of the VMFS.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the VMFS cannot be found or is unmounted.
    /// 
    /// ***HostConfigFault***: if the prerequisite is not satisfied
    /// or for all other configuration
    /// failures.
    pub async fn upgrade_vmfs(&self, vmfs_path: &str) -> Result<()> {
        let input = UpgradeVmfsRequestType {vmfs_path, };
        let path = format!("/HostStorageSystem/{moId}/UpgradeVmfs", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/HostStorageSystem/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// File system volume information for the host.
    /// 
    /// See the
    /// *FileSystemVolumeInfo* data
    /// object type for more information.
    pub async fn file_system_volume_info(&self) -> Result<HostFileSystemVolumeInfo> {
        let path = format!("/HostStorageSystem/{moId}/fileSystemVolumeInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Runtime information about the state of a multipath path.
    /// 
    /// A null value will be returned if path state information is not available
    /// for this system.
    /// 
    /// In systems prior to the plug-store architecture, the state of a path
    /// may be accessible on the *HostMultipathInfo* data object
    /// of the *HostStorageSystem.storageDeviceInfo* property.
    pub async fn multipath_state_info(&self) -> Result<Option<HostMultipathStateInfo>> {
        let path = format!("/HostStorageSystem/{moId}/multipathStateInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Host storage information up to the device level.
    pub async fn storage_device_info(&self) -> Result<Option<HostStorageDeviceInfo>> {
        let path = format!("/HostStorageSystem/{moId}/storageDeviceInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Datastore paths of files used by the host system on
    /// mounted volumes, for instance, the COS vmdk file of the
    /// host.
    /// 
    /// For information on datastore paths, see *Datastore*.
    pub async fn system_file(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/HostStorageSystem/{moId}/systemFile", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let path = format!("/HostStorageSystem/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddInternetScsiSendTargetsRequestType<'a> {
    #[serde(rename = "iScsiHbaDevice")]
    i_scsi_hba_device: &'a str,
    targets: &'a [HostInternetScsiHbaSendTarget],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddInternetScsiStaticTargetsRequestType<'a> {
    #[serde(rename = "iScsiHbaDevice")]
    i_scsi_hba_device: &'a str,
    targets: &'a [HostInternetScsiHbaStaticTarget],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AttachScsiLunRequestType<'a> {
    #[serde(rename = "lunUuid")]
    lun_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AttachScsiLunExRequestType<'a> {
    #[serde(rename = "lunUuid")]
    lun_uuid: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AttachVmfsExtentRequestType<'a> {
    #[serde(rename = "vmfsPath")]
    vmfs_path: &'a str,
    extent: &'a HostScsiDiskPartition,
}
#[derive(serde::Serialize)]
#[serde(rename = "ChangeNFSUserPasswordRequestType", tag = "_typeName")]
struct ChangeNfsUserPasswordRequestType<'a> {
    password: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ComputeDiskPartitionInfoRequestType<'a> {
    #[serde(rename = "devicePath")]
    device_path: &'a str,
    layout: &'a HostDiskPartitionLayout,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partitionFormat")]
    partition_format: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ComputeDiskPartitionInfoForResizeRequestType<'a> {
    partition: &'a HostScsiDiskPartition,
    #[serde(rename = "blockRange")]
    block_range: &'a HostDiskPartitionBlockRange,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "partitionFormat")]
    partition_format: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConnectNvmeControllerRequestType<'a> {
    #[serde(rename = "connectSpec")]
    connect_spec: &'a HostNvmeConnectSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConnectNvmeControllerExRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "connectSpec")]
    connect_spec: Option<&'a [HostNvmeConnectSpec]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateNvmeOverRdmaAdapterRequestType<'a> {
    #[serde(rename = "rdmaDeviceName")]
    rdma_device_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateSoftwareAdapterRequestType<'a> {
    spec: &'a dyn crate::types::traits::HostHbaCreateSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteScsiLunStateRequestType<'a> {
    #[serde(rename = "lunCanonicalName")]
    lun_canonical_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteVffsVolumeStateRequestType<'a> {
    #[serde(rename = "vffsUuid")]
    vffs_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteVmfsVolumeStateRequestType<'a> {
    #[serde(rename = "vmfsUuid")]
    vmfs_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DestroyVffsRequestType<'a> {
    #[serde(rename = "vffsPath")]
    vffs_path: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DetachScsiLunRequestType<'a> {
    #[serde(rename = "lunUuid")]
    lun_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DetachScsiLunExRequestType<'a> {
    #[serde(rename = "lunUuid")]
    lun_uuid: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DisableMultipathPathRequestType<'a> {
    #[serde(rename = "pathName")]
    path_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DisconnectNvmeControllerRequestType<'a> {
    #[serde(rename = "disconnectSpec")]
    disconnect_spec: &'a HostNvmeDisconnectSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DisconnectNvmeControllerExRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "disconnectSpec")]
    disconnect_spec: Option<&'a [HostNvmeDisconnectSpec]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DiscoverFcoeHbasRequestType<'a> {
    #[serde(rename = "fcoeSpec")]
    fcoe_spec: &'a FcoeConfigFcoeSpecification,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DiscoverNvmeControllersRequestType<'a> {
    #[serde(rename = "discoverSpec")]
    discover_spec: &'a HostNvmeDiscoverSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EnableMultipathPathRequestType<'a> {
    #[serde(rename = "pathName")]
    path_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ExpandVmfsExtentRequestType<'a> {
    #[serde(rename = "vmfsPath")]
    vmfs_path: &'a str,
    extent: &'a HostScsiDiskPartition,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ExtendVffsRequestType<'a> {
    #[serde(rename = "vffsPath")]
    vffs_path: &'a str,
    #[serde(rename = "devicePath")]
    device_path: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a HostDiskPartitionSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FormatVffsRequestType<'a> {
    #[serde(rename = "createSpec")]
    create_spec: &'a HostVffsSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FormatVmfsRequestType<'a> {
    #[serde(rename = "createSpec")]
    create_spec: &'a HostVmfsSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MarkAsLocalRequestType<'a> {
    #[serde(rename = "scsiDiskUuid")]
    scsi_disk_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MarkAsNonLocalRequestType<'a> {
    #[serde(rename = "scsiDiskUuid")]
    scsi_disk_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MarkAsNonSsdRequestType<'a> {
    #[serde(rename = "scsiDiskUuid")]
    scsi_disk_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MarkAsSsdRequestType<'a> {
    #[serde(rename = "scsiDiskUuid")]
    scsi_disk_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MarkForRemovalRequestType<'a> {
    #[serde(rename = "hbaName")]
    hba_name: &'a str,
    remove: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MarkPerenniallyReservedRequestType<'a> {
    #[serde(rename = "lunUuid")]
    lun_uuid: &'a str,
    state: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MarkPerenniallyReservedExRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lunUuid")]
    lun_uuid: Option<&'a [String]>,
    state: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MountVffsVolumeRequestType<'a> {
    #[serde(rename = "vffsUuid")]
    vffs_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MountVmfsVolumeRequestType<'a> {
    #[serde(rename = "vmfsUuid")]
    vmfs_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MountVmfsVolumeExRequestType<'a> {
    #[serde(rename = "vmfsUuid")]
    vmfs_uuid: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryAvailableSsdsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vffsPath")]
    vffs_path: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveInternetScsiSendTargetsRequestType<'a> {
    #[serde(rename = "iScsiHbaDevice")]
    i_scsi_hba_device: &'a str,
    targets: &'a [HostInternetScsiHbaSendTarget],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveInternetScsiStaticTargetsRequestType<'a> {
    #[serde(rename = "iScsiHbaDevice")]
    i_scsi_hba_device: &'a str,
    targets: &'a [HostInternetScsiHbaStaticTarget],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveNvmeOverRdmaAdapterRequestType<'a> {
    #[serde(rename = "hbaDeviceName")]
    hba_device_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveSoftwareAdapterRequestType<'a> {
    #[serde(rename = "hbaDeviceName")]
    hba_device_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RescanHbaRequestType<'a> {
    #[serde(rename = "hbaDevice")]
    hba_device: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ResolveMultipleUnresolvedVmfsVolumesRequestType<'a> {
    #[serde(rename = "resolutionSpec")]
    resolution_spec: &'a [HostUnresolvedVmfsResolutionSpec],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ResolveMultipleUnresolvedVmfsVolumesExRequestType<'a> {
    #[serde(rename = "resolutionSpec")]
    resolution_spec: &'a [HostUnresolvedVmfsResolutionSpec],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveDiskPartitionInfoRequestType<'a> {
    #[serde(rename = "devicePath")]
    device_path: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetMultipathLunPolicyRequestType<'a> {
    #[serde(rename = "lunId")]
    lun_id: &'a str,
    policy: &'a dyn crate::types::traits::HostMultipathInfoLogicalUnitPolicyTrait,
}
#[derive(serde::Serialize)]
#[serde(rename = "SetNFSUserRequestType", tag = "_typeName")]
struct SetNfsUserRequestType<'a> {
    user: &'a str,
    password: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct TurnDiskLocatorLedOffRequestType<'a> {
    #[serde(rename = "scsiDiskUuids")]
    scsi_disk_uuids: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct TurnDiskLocatorLedOnRequestType<'a> {
    #[serde(rename = "scsiDiskUuids")]
    scsi_disk_uuids: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnmapVmfsVolumeExRequestType<'a> {
    #[serde(rename = "vmfsUuid")]
    vmfs_uuid: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnmountForceMountedVmfsVolumeRequestType<'a> {
    #[serde(rename = "vmfsUuid")]
    vmfs_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnmountVffsVolumeRequestType<'a> {
    #[serde(rename = "vffsUuid")]
    vffs_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnmountVmfsVolumeRequestType<'a> {
    #[serde(rename = "vmfsUuid")]
    vmfs_uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnmountVmfsVolumeExRequestType<'a> {
    #[serde(rename = "vmfsUuid")]
    vmfs_uuid: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateDiskPartitionsRequestType<'a> {
    #[serde(rename = "devicePath")]
    device_path: &'a str,
    spec: &'a HostDiskPartitionSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateHppMultipathLunPolicyRequestType<'a> {
    #[serde(rename = "lunId")]
    lun_id: &'a str,
    policy: &'a HostMultipathInfoHppLogicalUnitPolicy,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateInternetScsiAdvancedOptionsRequestType<'a> {
    #[serde(rename = "iScsiHbaDevice")]
    i_scsi_hba_device: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetSet")]
    target_set: Option<&'a HostInternetScsiHbaTargetSet>,
    options: &'a [HostInternetScsiHbaParamValue],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateInternetScsiAliasRequestType<'a> {
    #[serde(rename = "iScsiHbaDevice")]
    i_scsi_hba_device: &'a str,
    #[serde(rename = "iScsiAlias")]
    i_scsi_alias: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateInternetScsiAuthenticationPropertiesRequestType<'a> {
    #[serde(rename = "iScsiHbaDevice")]
    i_scsi_hba_device: &'a str,
    #[serde(rename = "authenticationProperties")]
    authentication_properties: &'a HostInternetScsiHbaAuthenticationProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetSet")]
    target_set: Option<&'a HostInternetScsiHbaTargetSet>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateInternetScsiDigestPropertiesRequestType<'a> {
    #[serde(rename = "iScsiHbaDevice")]
    i_scsi_hba_device: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetSet")]
    target_set: Option<&'a HostInternetScsiHbaTargetSet>,
    #[serde(rename = "digestProperties")]
    digest_properties: &'a HostInternetScsiHbaDigestProperties,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateInternetScsiDiscoveryPropertiesRequestType<'a> {
    #[serde(rename = "iScsiHbaDevice")]
    i_scsi_hba_device: &'a str,
    #[serde(rename = "discoveryProperties")]
    discovery_properties: &'a HostInternetScsiHbaDiscoveryProperties,
}
#[derive(serde::Serialize)]
#[serde(rename = "UpdateInternetScsiIPPropertiesRequestType", tag = "_typeName")]
struct UpdateInternetScsiIpPropertiesRequestType<'a> {
    #[serde(rename = "iScsiHbaDevice")]
    i_scsi_hba_device: &'a str,
    #[serde(rename = "ipProperties")]
    ip_properties: &'a HostInternetScsiHbaIpProperties,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateInternetScsiNameRequestType<'a> {
    #[serde(rename = "iScsiHbaDevice")]
    i_scsi_hba_device: &'a str,
    #[serde(rename = "iScsiName")]
    i_scsi_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateScsiLunDisplayNameRequestType<'a> {
    #[serde(rename = "lunUuid")]
    lun_uuid: &'a str,
    #[serde(rename = "displayName")]
    display_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateSoftwareInternetScsiEnabledRequestType {
    enabled: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateVmfsUnmapBandwidthRequestType<'a> {
    #[serde(rename = "vmfsUuid")]
    vmfs_uuid: &'a str,
    #[serde(rename = "unmapBandwidthSpec")]
    unmap_bandwidth_spec: &'a VmfsUnmapBandwidthSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateVmfsUnmapPriorityRequestType<'a> {
    #[serde(rename = "vmfsUuid")]
    vmfs_uuid: &'a str,
    #[serde(rename = "unmapPriority")]
    unmap_priority: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpgradeVmfsRequestType<'a> {
    #[serde(rename = "vmfsPath")]
    vmfs_path: &'a str,
}
