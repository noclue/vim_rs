use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object gets and sets configuration information
/// about the host's storage subsystem.
/// 
/// The properties and operations are
/// used to configure the host to make storage available for virtual machines.
/// This object contains properties that are specific to ESX Server and
/// general to both the ESX Server system and the hosted architecture.
#[derive(Clone)]
pub struct HostStorageSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostStorageSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn add_internet_scsi_send_targets(&self, i_scsi_hba_device: &str, targets: &[crate::types::structs::HostInternetScsiHbaSendTarget]) -> Result<()> {
        let input = AddInternetScsiSendTargetsRequestType {i_scsi_hba_device, targets, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "AddInternetScsiSendTargets", Some(&input)).await
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
    pub async fn add_internet_scsi_static_targets(&self, i_scsi_hba_device: &str, targets: &[crate::types::structs::HostInternetScsiHbaStaticTarget]) -> Result<()> {
        let input = AddInternetScsiStaticTargetsRequestType {i_scsi_hba_device, targets, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "AddInternetScsiStaticTargets", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "AttachScsiLun", Some(&input)).await
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
    pub async fn attach_scsi_lun_ex_task(&self, lun_uuid: &[String]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = AttachScsiLunExRequestType {lun_uuid, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "AttachScsiLunEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn attach_vmfs_extent(&self, vmfs_path: &str, extent: &crate::types::structs::HostScsiDiskPartition) -> Result<()> {
        let input = AttachVmfsExtentRequestType {vmfs_path, extent, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "AttachVmfsExtent", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "ChangeNFSUserPassword", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "ClearNFSUser", None).await
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
    pub async fn compute_disk_partition_info(&self, device_path: &str, layout: &crate::types::structs::HostDiskPartitionLayout, partition_format: Option<&str>) -> Result<crate::types::structs::HostDiskPartitionInfo> {
        let input = ComputeDiskPartitionInfoRequestType {device_path, layout, partition_format, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "ComputeDiskPartitionInfo", Some(&input)).await?;
        let result: crate::types::structs::HostDiskPartitionInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn compute_disk_partition_info_for_resize(&self, partition: &crate::types::structs::HostScsiDiskPartition, block_range: &crate::types::structs::HostDiskPartitionBlockRange, partition_format: Option<&str>) -> Result<crate::types::structs::HostDiskPartitionInfo> {
        let input = ComputeDiskPartitionInfoForResizeRequestType {partition, block_range, partition_format, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "ComputeDiskPartitionInfoForResize", Some(&input)).await?;
        let result: crate::types::structs::HostDiskPartitionInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn connect_nvme_controller(&self, connect_spec: &crate::types::structs::HostNvmeConnectSpec) -> Result<()> {
        let input = ConnectNvmeControllerRequestType {connect_spec, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "ConnectNvmeController", Some(&input)).await
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
    pub async fn connect_nvme_controller_ex_task(&self, connect_spec: Option<&[crate::types::structs::HostNvmeConnectSpec]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ConnectNvmeControllerExRequestType {connect_spec, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "ConnectNvmeControllerEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "CreateNvmeOverRdmaAdapter", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "CreateSoftwareAdapter", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "DeleteScsiLunState", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "DeleteVffsVolumeState", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "DeleteVmfsVolumeState", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "DestroyVffs", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "DetachScsiLun", Some(&input)).await
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
    pub async fn detach_scsi_lun_ex_task(&self, lun_uuid: &[String]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DetachScsiLunExRequestType {lun_uuid, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "DetachScsiLunEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "DisableMultipathPath", Some(&input)).await
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
    pub async fn disconnect_nvme_controller(&self, disconnect_spec: &crate::types::structs::HostNvmeDisconnectSpec) -> Result<()> {
        let input = DisconnectNvmeControllerRequestType {disconnect_spec, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "DisconnectNvmeController", Some(&input)).await
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
    pub async fn disconnect_nvme_controller_ex_task(&self, disconnect_spec: Option<&[crate::types::structs::HostNvmeDisconnectSpec]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DisconnectNvmeControllerExRequestType {disconnect_spec, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "DisconnectNvmeControllerEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn discover_fcoe_hbas(&self, fcoe_spec: &crate::types::structs::FcoeConfigFcoeSpecification) -> Result<()> {
        let input = DiscoverFcoeHbasRequestType {fcoe_spec, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "DiscoverFcoeHbas", Some(&input)).await
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
    pub async fn discover_nvme_controllers(&self, discover_spec: &crate::types::structs::HostNvmeDiscoverSpec) -> Result<crate::types::structs::HostNvmeDiscoveryLog> {
        let input = DiscoverNvmeControllersRequestType {discover_spec, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "DiscoverNvmeControllers", Some(&input)).await?;
        let result: crate::types::structs::HostNvmeDiscoveryLog = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "EnableMultipathPath", Some(&input)).await
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
    pub async fn expand_vmfs_extent(&self, vmfs_path: &str, extent: &crate::types::structs::HostScsiDiskPartition) -> Result<()> {
        let input = ExpandVmfsExtentRequestType {vmfs_path, extent, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "ExpandVmfsExtent", Some(&input)).await
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
    pub async fn extend_vffs(&self, vffs_path: &str, device_path: &str, spec: Option<&crate::types::structs::HostDiskPartitionSpec>) -> Result<()> {
        let input = ExtendVffsRequestType {vffs_path, device_path, spec, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "ExtendVffs", Some(&input)).await
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
    pub async fn format_vffs(&self, create_spec: &crate::types::structs::HostVffsSpec) -> Result<crate::types::structs::HostVffsVolume> {
        let input = FormatVffsRequestType {create_spec, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "FormatVffs", Some(&input)).await?;
        let result: crate::types::structs::HostVffsVolume = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn format_vmfs(&self, create_spec: &crate::types::structs::HostVmfsSpec) -> Result<crate::types::structs::HostVmfsVolume> {
        let input = FormatVmfsRequestType {create_spec, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "FormatVmfs", Some(&input)).await?;
        let result: crate::types::structs::HostVmfsVolume = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn mark_as_local_task(&self, scsi_disk_uuid: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = MarkAsLocalRequestType {scsi_disk_uuid, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "MarkAsLocal_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn mark_as_non_local_task(&self, scsi_disk_uuid: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = MarkAsNonLocalRequestType {scsi_disk_uuid, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "MarkAsNonLocal_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn mark_as_non_ssd_task(&self, scsi_disk_uuid: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = MarkAsNonSsdRequestType {scsi_disk_uuid, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "MarkAsNonSsd_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn mark_as_ssd_task(&self, scsi_disk_uuid: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = MarkAsSsdRequestType {scsi_disk_uuid, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "MarkAsSsd_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn host_storage_system_mark_for_removal(&self, hba_name: &str, remove: bool) -> Result<()> {
        let input = HostStorageSystemMarkForRemovalRequestType {hba_name, remove, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "HostStorageSystem_MarkForRemoval", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "MarkPerenniallyReserved", Some(&input)).await
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
    pub async fn mark_perennially_reserved_ex_task(&self, lun_uuid: Option<&[String]>, state: bool) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = MarkPerenniallyReservedExRequestType {lun_uuid, state, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "MarkPerenniallyReservedEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "MountVffsVolume", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "MountVmfsVolume", Some(&input)).await
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
    pub async fn mount_vmfs_volume_ex_task(&self, vmfs_uuid: &[String]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = MountVmfsVolumeExRequestType {vmfs_uuid, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "MountVmfsVolumeEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn query_available_ssds(&self, vffs_path: Option<&str>) -> Result<Option<Vec<crate::types::structs::HostScsiDisk>>> {
        let input = QueryAvailableSsdsRequestType {vffs_path, };
        let bytes_opt = self.client.invoke_optional("", "HostStorageSystem", &self.mo_id, "QueryAvailableSsds", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn query_nfs_user(&self) -> Result<Option<crate::types::structs::HostNasVolumeUserInfo>> {
        let bytes_opt = self.client.invoke_optional("", "HostStorageSystem", &self.mo_id, "QueryNFSUser", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn query_path_selection_policy_options(&self) -> Result<Option<Vec<crate::types::structs::HostPathSelectionPolicyOption>>> {
        let bytes_opt = self.client.invoke_optional("", "HostStorageSystem", &self.mo_id, "QueryPathSelectionPolicyOptions", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn query_storage_array_type_policy_options(&self) -> Result<Option<Vec<crate::types::structs::HostStorageArrayTypePolicyOption>>> {
        let bytes_opt = self.client.invoke_optional("", "HostStorageSystem", &self.mo_id, "QueryStorageArrayTypePolicyOptions", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn query_unresolved_vmfs_volume(&self) -> Result<Option<Vec<crate::types::structs::HostUnresolvedVmfsVolume>>> {
        let bytes_opt = self.client.invoke_optional("", "HostStorageSystem", &self.mo_id, "QueryUnresolvedVmfsVolume", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Get the VMFS configuration options, including block size,
    /// unmap granularity.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Returns:
    ///
    /// VMFS configuration options.
    pub async fn query_vmfs_config_option(&self) -> Result<Option<Vec<crate::types::structs::VmfsConfigOption>>> {
        let bytes_opt = self.client.invoke_optional("", "HostStorageSystem", &self.mo_id, "QueryVmfsConfigOption", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "RefreshStorageSystem", None).await
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
    pub async fn remove_internet_scsi_send_targets(&self, i_scsi_hba_device: &str, targets: &[crate::types::structs::HostInternetScsiHbaSendTarget], force: Option<bool>) -> Result<()> {
        let input = RemoveInternetScsiSendTargetsRequestType {i_scsi_hba_device, targets, force, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "RemoveInternetScsiSendTargets", Some(&input)).await
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
    pub async fn remove_internet_scsi_static_targets(&self, i_scsi_hba_device: &str, targets: &[crate::types::structs::HostInternetScsiHbaStaticTarget]) -> Result<()> {
        let input = RemoveInternetScsiStaticTargetsRequestType {i_scsi_hba_device, targets, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "RemoveInternetScsiStaticTargets", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "RemoveNvmeOverRdmaAdapter", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "RemoveSoftwareAdapter", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "RescanAllHba", None).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "RescanHba", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "RescanVffs", None).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "RescanVmfs", None).await
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
    pub async fn resolve_multiple_unresolved_vmfs_volumes(&self, resolution_spec: &[crate::types::structs::HostUnresolvedVmfsResolutionSpec]) -> Result<Option<Vec<crate::types::structs::HostUnresolvedVmfsResolutionResult>>> {
        let input = ResolveMultipleUnresolvedVmfsVolumesRequestType {resolution_spec, };
        let bytes_opt = self.client.invoke_optional("", "HostStorageSystem", &self.mo_id, "ResolveMultipleUnresolvedVmfsVolumes", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn resolve_multiple_unresolved_vmfs_volumes_ex_task(&self, resolution_spec: &[crate::types::structs::HostUnresolvedVmfsResolutionSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ResolveMultipleUnresolvedVmfsVolumesExRequestType {resolution_spec, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "ResolveMultipleUnresolvedVmfsVolumesEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn retrieve_disk_partition_info(&self, device_path: &[String]) -> Result<Option<Vec<crate::types::structs::HostDiskPartitionInfo>>> {
        let input = RetrieveDiskPartitionInfoRequestType {device_path, };
        let bytes_opt = self.client.invoke_optional("", "HostStorageSystem", &self.mo_id, "RetrieveDiskPartitionInfo", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "setCustomValue", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "SetMultipathLunPolicy", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "SetNFSUser", Some(&input)).await
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
    pub async fn turn_disk_locator_led_off_task(&self, scsi_disk_uuids: &[String]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = TurnDiskLocatorLedOffRequestType {scsi_disk_uuids, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "TurnDiskLocatorLedOff_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn turn_disk_locator_led_on_task(&self, scsi_disk_uuids: &[String]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = TurnDiskLocatorLedOnRequestType {scsi_disk_uuids, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "TurnDiskLocatorLedOn_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn unmap_vmfs_volume_ex_task(&self, vmfs_uuid: &[String]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UnmapVmfsVolumeExRequestType {vmfs_uuid, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "UnmapVmfsVolumeEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UnmountForceMountedVmfsVolume", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UnmountVffsVolume", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UnmountVmfsVolume", Some(&input)).await
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
    pub async fn unmount_vmfs_volume_ex_task(&self, vmfs_uuid: &[String]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UnmountVmfsVolumeExRequestType {vmfs_uuid, };
        let bytes = self.client.invoke("", "HostStorageSystem", &self.mo_id, "UnmountVmfsVolumeEx_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn update_disk_partitions(&self, device_path: &str, spec: &crate::types::structs::HostDiskPartitionSpec) -> Result<()> {
        let input = UpdateDiskPartitionsRequestType {device_path, spec, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateDiskPartitions", Some(&input)).await
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
    pub async fn update_hpp_multipath_lun_policy(&self, lun_id: &str, policy: &crate::types::structs::HostMultipathInfoHppLogicalUnitPolicy) -> Result<()> {
        let input = UpdateHppMultipathLunPolicyRequestType {lun_id, policy, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateHppMultipathLunPolicy", Some(&input)).await
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
    pub async fn update_internet_scsi_advanced_options(&self, i_scsi_hba_device: &str, target_set: Option<&crate::types::structs::HostInternetScsiHbaTargetSet>, options: &[crate::types::structs::HostInternetScsiHbaParamValue]) -> Result<()> {
        let input = UpdateInternetScsiAdvancedOptionsRequestType {i_scsi_hba_device, target_set, options, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateInternetScsiAdvancedOptions", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateInternetScsiAlias", Some(&input)).await
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
    pub async fn update_internet_scsi_authentication_properties(&self, i_scsi_hba_device: &str, authentication_properties: &crate::types::structs::HostInternetScsiHbaAuthenticationProperties, target_set: Option<&crate::types::structs::HostInternetScsiHbaTargetSet>) -> Result<()> {
        let input = UpdateInternetScsiAuthenticationPropertiesRequestType {i_scsi_hba_device, authentication_properties, target_set, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateInternetScsiAuthenticationProperties", Some(&input)).await
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
    pub async fn update_internet_scsi_digest_properties(&self, i_scsi_hba_device: &str, target_set: Option<&crate::types::structs::HostInternetScsiHbaTargetSet>, digest_properties: &crate::types::structs::HostInternetScsiHbaDigestProperties) -> Result<()> {
        let input = UpdateInternetScsiDigestPropertiesRequestType {i_scsi_hba_device, target_set, digest_properties, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateInternetScsiDigestProperties", Some(&input)).await
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
    pub async fn update_internet_scsi_discovery_properties(&self, i_scsi_hba_device: &str, discovery_properties: &crate::types::structs::HostInternetScsiHbaDiscoveryProperties) -> Result<()> {
        let input = UpdateInternetScsiDiscoveryPropertiesRequestType {i_scsi_hba_device, discovery_properties, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateInternetScsiDiscoveryProperties", Some(&input)).await
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
    pub async fn update_internet_scsi_ip_properties(&self, i_scsi_hba_device: &str, ip_properties: &crate::types::structs::HostInternetScsiHbaIpProperties) -> Result<()> {
        let input = UpdateInternetScsiIpPropertiesRequestType {i_scsi_hba_device, ip_properties, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateInternetScsiIPProperties", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateInternetScsiName", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateScsiLunDisplayName", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateSoftwareInternetScsiEnabled", Some(&input)).await
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
    pub async fn update_vmfs_unmap_bandwidth(&self, vmfs_uuid: &str, unmap_bandwidth_spec: &crate::types::structs::VmfsUnmapBandwidthSpec) -> Result<()> {
        let input = UpdateVmfsUnmapBandwidthRequestType {vmfs_uuid, unmap_bandwidth_spec, };
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateVmfsUnmapBandwidth", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpdateVmfsUnmapPriority", Some(&input)).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpgradeVmLayout", None).await
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
        self.client.invoke_void("", "HostStorageSystem", &self.mo_id, "UpgradeVmfs", Some(&input)).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let pv_opt = self.client.fetch_property_raw("", "HostStorageSystem", &self.mo_id, "availableField").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// File system volume information for the host.
    /// 
    /// See the
    /// *FileSystemVolumeInfo* data
    /// object type for more information.
    pub async fn file_system_volume_info(&self) -> Result<crate::types::structs::HostFileSystemVolumeInfo> {
        let pv_opt = self.client.fetch_property_raw("", "HostStorageSystem", &self.mo_id, "fileSystemVolumeInfo").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property fileSystemVolumeInfo was empty".to_string()))?;
        let result: crate::types::structs::HostFileSystemVolumeInfo = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Runtime information about the state of a multipath path.
    /// 
    /// A null value will be returned if path state information is not available
    /// for this system.
    /// 
    /// In systems prior to the plug-store architecture, the state of a path
    /// may be accessible on the *HostMultipathInfo* data object
    /// of the *HostStorageSystem.storageDeviceInfo* property.
    pub async fn multipath_state_info(&self) -> Result<Option<crate::types::structs::HostMultipathStateInfo>> {
        let pv_opt = self.client.fetch_property_raw("", "HostStorageSystem", &self.mo_id, "multipathStateInfo").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Host storage information up to the device level.
    pub async fn storage_device_info(&self) -> Result<Option<crate::types::structs::HostStorageDeviceInfo>> {
        let pv_opt = self.client.fetch_property_raw("", "HostStorageSystem", &self.mo_id, "storageDeviceInfo").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Datastore paths of files used by the host system on
    /// mounted volumes, for instance, the COS vmdk file of the
    /// host.
    /// 
    /// For information on datastore paths, see *Datastore*.
    pub async fn system_file(&self) -> Result<Option<Vec<String>>> {
        let pv_opt = self.client.fetch_property_raw("", "HostStorageSystem", &self.mo_id, "systemFile").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let pv_opt = self.client.fetch_property_raw("", "HostStorageSystem", &self.mo_id, "value").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct AddInternetScsiSendTargetsRequestType<'a> {
    i_scsi_hba_device: &'a str,
    targets: &'a [crate::types::structs::HostInternetScsiHbaSendTarget],
}

impl<'a> miniserde::Serialize for AddInternetScsiSendTargetsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddInternetScsiSendTargetsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddInternetScsiSendTargetsRequestTypeSer<'b, 'a> {
    data: &'b AddInternetScsiSendTargetsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddInternetScsiSendTargetsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddInternetScsiSendTargetsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaDevice"), &self.data.i_scsi_hba_device as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("targets"), &self.data.targets as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AddInternetScsiStaticTargetsRequestType<'a> {
    i_scsi_hba_device: &'a str,
    targets: &'a [crate::types::structs::HostInternetScsiHbaStaticTarget],
}

impl<'a> miniserde::Serialize for AddInternetScsiStaticTargetsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddInternetScsiStaticTargetsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddInternetScsiStaticTargetsRequestTypeSer<'b, 'a> {
    data: &'b AddInternetScsiStaticTargetsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddInternetScsiStaticTargetsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddInternetScsiStaticTargetsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaDevice"), &self.data.i_scsi_hba_device as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("targets"), &self.data.targets as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AttachScsiLunRequestType<'a> {
    lun_uuid: &'a str,
}

impl<'a> miniserde::Serialize for AttachScsiLunRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AttachScsiLunRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AttachScsiLunRequestTypeSer<'b, 'a> {
    data: &'b AttachScsiLunRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AttachScsiLunRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AttachScsiLunRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("lunUuid"), &self.data.lun_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AttachScsiLunExRequestType<'a> {
    lun_uuid: &'a [String],
}

impl<'a> miniserde::Serialize for AttachScsiLunExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AttachScsiLunExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AttachScsiLunExRequestTypeSer<'b, 'a> {
    data: &'b AttachScsiLunExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AttachScsiLunExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AttachScsiLunExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("lunUuid"), &self.data.lun_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AttachVmfsExtentRequestType<'a> {
    vmfs_path: &'a str,
    extent: &'a crate::types::structs::HostScsiDiskPartition,
}

impl<'a> miniserde::Serialize for AttachVmfsExtentRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AttachVmfsExtentRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AttachVmfsExtentRequestTypeSer<'b, 'a> {
    data: &'b AttachVmfsExtentRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AttachVmfsExtentRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AttachVmfsExtentRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsPath"), &self.data.vmfs_path as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("extent"), &self.data.extent as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ChangeNfsUserPasswordRequestType<'a> {
    password: &'a str,
}

impl<'a> miniserde::Serialize for ChangeNfsUserPasswordRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ChangeNfsUserPasswordRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ChangeNfsUserPasswordRequestTypeSer<'b, 'a> {
    data: &'b ChangeNfsUserPasswordRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ChangeNfsUserPasswordRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ChangeNFSUserPasswordRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("password"), &self.data.password as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ComputeDiskPartitionInfoRequestType<'a> {
    device_path: &'a str,
    layout: &'a crate::types::structs::HostDiskPartitionLayout,
    partition_format: Option<&'a str>,
}

impl<'a> miniserde::Serialize for ComputeDiskPartitionInfoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ComputeDiskPartitionInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ComputeDiskPartitionInfoRequestTypeSer<'b, 'a> {
    data: &'b ComputeDiskPartitionInfoRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ComputeDiskPartitionInfoRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ComputeDiskPartitionInfoRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("devicePath"), &self.data.device_path as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("layout"), &self.data.layout as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.partition_format else { continue; };
                    return Some((std::borrow::Cow::Borrowed("partitionFormat"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ComputeDiskPartitionInfoForResizeRequestType<'a> {
    partition: &'a crate::types::structs::HostScsiDiskPartition,
    block_range: &'a crate::types::structs::HostDiskPartitionBlockRange,
    partition_format: Option<&'a str>,
}

impl<'a> miniserde::Serialize for ComputeDiskPartitionInfoForResizeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ComputeDiskPartitionInfoForResizeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ComputeDiskPartitionInfoForResizeRequestTypeSer<'b, 'a> {
    data: &'b ComputeDiskPartitionInfoForResizeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ComputeDiskPartitionInfoForResizeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ComputeDiskPartitionInfoForResizeRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("partition"), &self.data.partition as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("blockRange"), &self.data.block_range as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.partition_format else { continue; };
                    return Some((std::borrow::Cow::Borrowed("partitionFormat"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ConnectNvmeControllerRequestType<'a> {
    connect_spec: &'a crate::types::structs::HostNvmeConnectSpec,
}

impl<'a> miniserde::Serialize for ConnectNvmeControllerRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ConnectNvmeControllerRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ConnectNvmeControllerRequestTypeSer<'b, 'a> {
    data: &'b ConnectNvmeControllerRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ConnectNvmeControllerRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ConnectNvmeControllerRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("connectSpec"), &self.data.connect_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ConnectNvmeControllerExRequestType<'a> {
    connect_spec: Option<&'a [crate::types::structs::HostNvmeConnectSpec]>,
}

impl<'a> miniserde::Serialize for ConnectNvmeControllerExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ConnectNvmeControllerExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ConnectNvmeControllerExRequestTypeSer<'b, 'a> {
    data: &'b ConnectNvmeControllerExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ConnectNvmeControllerExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ConnectNvmeControllerExRequestType")),
                1 => {
                    let Some(ref val) = self.data.connect_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("connectSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CreateNvmeOverRdmaAdapterRequestType<'a> {
    rdma_device_name: &'a str,
}

impl<'a> miniserde::Serialize for CreateNvmeOverRdmaAdapterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateNvmeOverRdmaAdapterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateNvmeOverRdmaAdapterRequestTypeSer<'b, 'a> {
    data: &'b CreateNvmeOverRdmaAdapterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateNvmeOverRdmaAdapterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateNvmeOverRdmaAdapterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("rdmaDeviceName"), &self.data.rdma_device_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateSoftwareAdapterRequestType<'a> {
    spec: &'a dyn crate::types::traits::HostHbaCreateSpecTrait,
}

impl<'a> miniserde::Serialize for CreateSoftwareAdapterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateSoftwareAdapterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateSoftwareAdapterRequestTypeSer<'b, 'a> {
    data: &'b CreateSoftwareAdapterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateSoftwareAdapterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateSoftwareAdapterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DeleteScsiLunStateRequestType<'a> {
    lun_canonical_name: &'a str,
}

impl<'a> miniserde::Serialize for DeleteScsiLunStateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteScsiLunStateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteScsiLunStateRequestTypeSer<'b, 'a> {
    data: &'b DeleteScsiLunStateRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeleteScsiLunStateRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteScsiLunStateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("lunCanonicalName"), &self.data.lun_canonical_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DeleteVffsVolumeStateRequestType<'a> {
    vffs_uuid: &'a str,
}

impl<'a> miniserde::Serialize for DeleteVffsVolumeStateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteVffsVolumeStateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteVffsVolumeStateRequestTypeSer<'b, 'a> {
    data: &'b DeleteVffsVolumeStateRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeleteVffsVolumeStateRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteVffsVolumeStateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vffsUuid"), &self.data.vffs_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DeleteVmfsVolumeStateRequestType<'a> {
    vmfs_uuid: &'a str,
}

impl<'a> miniserde::Serialize for DeleteVmfsVolumeStateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteVmfsVolumeStateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteVmfsVolumeStateRequestTypeSer<'b, 'a> {
    data: &'b DeleteVmfsVolumeStateRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeleteVmfsVolumeStateRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteVmfsVolumeStateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsUuid"), &self.data.vmfs_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DestroyVffsRequestType<'a> {
    vffs_path: &'a str,
}

impl<'a> miniserde::Serialize for DestroyVffsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DestroyVffsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DestroyVffsRequestTypeSer<'b, 'a> {
    data: &'b DestroyVffsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DestroyVffsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DestroyVffsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vffsPath"), &self.data.vffs_path as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DetachScsiLunRequestType<'a> {
    lun_uuid: &'a str,
}

impl<'a> miniserde::Serialize for DetachScsiLunRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DetachScsiLunRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DetachScsiLunRequestTypeSer<'b, 'a> {
    data: &'b DetachScsiLunRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DetachScsiLunRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DetachScsiLunRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("lunUuid"), &self.data.lun_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DetachScsiLunExRequestType<'a> {
    lun_uuid: &'a [String],
}

impl<'a> miniserde::Serialize for DetachScsiLunExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DetachScsiLunExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DetachScsiLunExRequestTypeSer<'b, 'a> {
    data: &'b DetachScsiLunExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DetachScsiLunExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DetachScsiLunExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("lunUuid"), &self.data.lun_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DisableMultipathPathRequestType<'a> {
    path_name: &'a str,
}

impl<'a> miniserde::Serialize for DisableMultipathPathRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DisableMultipathPathRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DisableMultipathPathRequestTypeSer<'b, 'a> {
    data: &'b DisableMultipathPathRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DisableMultipathPathRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DisableMultipathPathRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("pathName"), &self.data.path_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DisconnectNvmeControllerRequestType<'a> {
    disconnect_spec: &'a crate::types::structs::HostNvmeDisconnectSpec,
}

impl<'a> miniserde::Serialize for DisconnectNvmeControllerRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DisconnectNvmeControllerRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DisconnectNvmeControllerRequestTypeSer<'b, 'a> {
    data: &'b DisconnectNvmeControllerRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DisconnectNvmeControllerRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DisconnectNvmeControllerRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("disconnectSpec"), &self.data.disconnect_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DisconnectNvmeControllerExRequestType<'a> {
    disconnect_spec: Option<&'a [crate::types::structs::HostNvmeDisconnectSpec]>,
}

impl<'a> miniserde::Serialize for DisconnectNvmeControllerExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DisconnectNvmeControllerExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DisconnectNvmeControllerExRequestTypeSer<'b, 'a> {
    data: &'b DisconnectNvmeControllerExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DisconnectNvmeControllerExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DisconnectNvmeControllerExRequestType")),
                1 => {
                    let Some(ref val) = self.data.disconnect_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("disconnectSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct DiscoverFcoeHbasRequestType<'a> {
    fcoe_spec: &'a crate::types::structs::FcoeConfigFcoeSpecification,
}

impl<'a> miniserde::Serialize for DiscoverFcoeHbasRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DiscoverFcoeHbasRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DiscoverFcoeHbasRequestTypeSer<'b, 'a> {
    data: &'b DiscoverFcoeHbasRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DiscoverFcoeHbasRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DiscoverFcoeHbasRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("fcoeSpec"), &self.data.fcoe_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DiscoverNvmeControllersRequestType<'a> {
    discover_spec: &'a crate::types::structs::HostNvmeDiscoverSpec,
}

impl<'a> miniserde::Serialize for DiscoverNvmeControllersRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DiscoverNvmeControllersRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DiscoverNvmeControllersRequestTypeSer<'b, 'a> {
    data: &'b DiscoverNvmeControllersRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DiscoverNvmeControllersRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DiscoverNvmeControllersRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("discoverSpec"), &self.data.discover_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct EnableMultipathPathRequestType<'a> {
    path_name: &'a str,
}

impl<'a> miniserde::Serialize for EnableMultipathPathRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EnableMultipathPathRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EnableMultipathPathRequestTypeSer<'b, 'a> {
    data: &'b EnableMultipathPathRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for EnableMultipathPathRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EnableMultipathPathRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("pathName"), &self.data.path_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ExpandVmfsExtentRequestType<'a> {
    vmfs_path: &'a str,
    extent: &'a crate::types::structs::HostScsiDiskPartition,
}

impl<'a> miniserde::Serialize for ExpandVmfsExtentRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ExpandVmfsExtentRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ExpandVmfsExtentRequestTypeSer<'b, 'a> {
    data: &'b ExpandVmfsExtentRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ExpandVmfsExtentRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ExpandVmfsExtentRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsPath"), &self.data.vmfs_path as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("extent"), &self.data.extent as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ExtendVffsRequestType<'a> {
    vffs_path: &'a str,
    device_path: &'a str,
    spec: Option<&'a crate::types::structs::HostDiskPartitionSpec>,
}

impl<'a> miniserde::Serialize for ExtendVffsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ExtendVffsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ExtendVffsRequestTypeSer<'b, 'a> {
    data: &'b ExtendVffsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ExtendVffsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ExtendVffsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vffsPath"), &self.data.vffs_path as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("devicePath"), &self.data.device_path as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct FormatVffsRequestType<'a> {
    create_spec: &'a crate::types::structs::HostVffsSpec,
}

impl<'a> miniserde::Serialize for FormatVffsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FormatVffsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FormatVffsRequestTypeSer<'b, 'a> {
    data: &'b FormatVffsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FormatVffsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FormatVffsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("createSpec"), &self.data.create_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct FormatVmfsRequestType<'a> {
    create_spec: &'a crate::types::structs::HostVmfsSpec,
}

impl<'a> miniserde::Serialize for FormatVmfsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FormatVmfsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FormatVmfsRequestTypeSer<'b, 'a> {
    data: &'b FormatVmfsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FormatVmfsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FormatVmfsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("createSpec"), &self.data.create_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MarkAsLocalRequestType<'a> {
    scsi_disk_uuid: &'a str,
}

impl<'a> miniserde::Serialize for MarkAsLocalRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MarkAsLocalRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MarkAsLocalRequestTypeSer<'b, 'a> {
    data: &'b MarkAsLocalRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MarkAsLocalRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MarkAsLocalRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("scsiDiskUuid"), &self.data.scsi_disk_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MarkAsNonLocalRequestType<'a> {
    scsi_disk_uuid: &'a str,
}

impl<'a> miniserde::Serialize for MarkAsNonLocalRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MarkAsNonLocalRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MarkAsNonLocalRequestTypeSer<'b, 'a> {
    data: &'b MarkAsNonLocalRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MarkAsNonLocalRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MarkAsNonLocalRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("scsiDiskUuid"), &self.data.scsi_disk_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MarkAsNonSsdRequestType<'a> {
    scsi_disk_uuid: &'a str,
}

impl<'a> miniserde::Serialize for MarkAsNonSsdRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MarkAsNonSsdRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MarkAsNonSsdRequestTypeSer<'b, 'a> {
    data: &'b MarkAsNonSsdRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MarkAsNonSsdRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MarkAsNonSsdRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("scsiDiskUuid"), &self.data.scsi_disk_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MarkAsSsdRequestType<'a> {
    scsi_disk_uuid: &'a str,
}

impl<'a> miniserde::Serialize for MarkAsSsdRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MarkAsSsdRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MarkAsSsdRequestTypeSer<'b, 'a> {
    data: &'b MarkAsSsdRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MarkAsSsdRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MarkAsSsdRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("scsiDiskUuid"), &self.data.scsi_disk_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HostStorageSystemMarkForRemovalRequestType<'a> {
    hba_name: &'a str,
    remove: bool,
}

impl<'a> miniserde::Serialize for HostStorageSystemMarkForRemovalRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostStorageSystemMarkForRemovalRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostStorageSystemMarkForRemovalRequestTypeSer<'b, 'a> {
    data: &'b HostStorageSystemMarkForRemovalRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostStorageSystemMarkForRemovalRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostStorageSystem_MarkForRemovalRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hbaName"), &self.data.hba_name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("remove"), &self.data.remove as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MarkPerenniallyReservedRequestType<'a> {
    lun_uuid: &'a str,
    state: bool,
}

impl<'a> miniserde::Serialize for MarkPerenniallyReservedRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MarkPerenniallyReservedRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MarkPerenniallyReservedRequestTypeSer<'b, 'a> {
    data: &'b MarkPerenniallyReservedRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MarkPerenniallyReservedRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MarkPerenniallyReservedRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("lunUuid"), &self.data.lun_uuid as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("state"), &self.data.state as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MarkPerenniallyReservedExRequestType<'a> {
    lun_uuid: Option<&'a [String]>,
    state: bool,
}

impl<'a> miniserde::Serialize for MarkPerenniallyReservedExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MarkPerenniallyReservedExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MarkPerenniallyReservedExRequestTypeSer<'b, 'a> {
    data: &'b MarkPerenniallyReservedExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MarkPerenniallyReservedExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MarkPerenniallyReservedExRequestType")),
                1 => {
                    let Some(ref val) = self.data.lun_uuid else { continue; };
                    return Some((std::borrow::Cow::Borrowed("lunUuid"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("state"), &self.data.state as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct MountVffsVolumeRequestType<'a> {
    vffs_uuid: &'a str,
}

impl<'a> miniserde::Serialize for MountVffsVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MountVffsVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MountVffsVolumeRequestTypeSer<'b, 'a> {
    data: &'b MountVffsVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MountVffsVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MountVffsVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vffsUuid"), &self.data.vffs_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MountVmfsVolumeRequestType<'a> {
    vmfs_uuid: &'a str,
}

impl<'a> miniserde::Serialize for MountVmfsVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MountVmfsVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MountVmfsVolumeRequestTypeSer<'b, 'a> {
    data: &'b MountVmfsVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MountVmfsVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MountVmfsVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsUuid"), &self.data.vmfs_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MountVmfsVolumeExRequestType<'a> {
    vmfs_uuid: &'a [String],
}

impl<'a> miniserde::Serialize for MountVmfsVolumeExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MountVmfsVolumeExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MountVmfsVolumeExRequestTypeSer<'b, 'a> {
    data: &'b MountVmfsVolumeExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MountVmfsVolumeExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MountVmfsVolumeExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsUuid"), &self.data.vmfs_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryAvailableSsdsRequestType<'a> {
    vffs_path: Option<&'a str>,
}

impl<'a> miniserde::Serialize for QueryAvailableSsdsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryAvailableSsdsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryAvailableSsdsRequestTypeSer<'b, 'a> {
    data: &'b QueryAvailableSsdsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryAvailableSsdsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryAvailableSsdsRequestType")),
                1 => {
                    let Some(ref val) = self.data.vffs_path else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vffsPath"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveInternetScsiSendTargetsRequestType<'a> {
    i_scsi_hba_device: &'a str,
    targets: &'a [crate::types::structs::HostInternetScsiHbaSendTarget],
    force: Option<bool>,
}

impl<'a> miniserde::Serialize for RemoveInternetScsiSendTargetsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveInternetScsiSendTargetsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveInternetScsiSendTargetsRequestTypeSer<'b, 'a> {
    data: &'b RemoveInternetScsiSendTargetsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveInternetScsiSendTargetsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveInternetScsiSendTargetsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaDevice"), &self.data.i_scsi_hba_device as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("targets"), &self.data.targets as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.force else { continue; };
                    return Some((std::borrow::Cow::Borrowed("force"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveInternetScsiStaticTargetsRequestType<'a> {
    i_scsi_hba_device: &'a str,
    targets: &'a [crate::types::structs::HostInternetScsiHbaStaticTarget],
}

impl<'a> miniserde::Serialize for RemoveInternetScsiStaticTargetsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveInternetScsiStaticTargetsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveInternetScsiStaticTargetsRequestTypeSer<'b, 'a> {
    data: &'b RemoveInternetScsiStaticTargetsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveInternetScsiStaticTargetsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveInternetScsiStaticTargetsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaDevice"), &self.data.i_scsi_hba_device as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("targets"), &self.data.targets as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveNvmeOverRdmaAdapterRequestType<'a> {
    hba_device_name: &'a str,
}

impl<'a> miniserde::Serialize for RemoveNvmeOverRdmaAdapterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveNvmeOverRdmaAdapterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveNvmeOverRdmaAdapterRequestTypeSer<'b, 'a> {
    data: &'b RemoveNvmeOverRdmaAdapterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveNvmeOverRdmaAdapterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveNvmeOverRdmaAdapterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hbaDeviceName"), &self.data.hba_device_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveSoftwareAdapterRequestType<'a> {
    hba_device_name: &'a str,
}

impl<'a> miniserde::Serialize for RemoveSoftwareAdapterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveSoftwareAdapterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveSoftwareAdapterRequestTypeSer<'b, 'a> {
    data: &'b RemoveSoftwareAdapterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveSoftwareAdapterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveSoftwareAdapterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hbaDeviceName"), &self.data.hba_device_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RescanHbaRequestType<'a> {
    hba_device: &'a str,
}

impl<'a> miniserde::Serialize for RescanHbaRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RescanHbaRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RescanHbaRequestTypeSer<'b, 'a> {
    data: &'b RescanHbaRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RescanHbaRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RescanHbaRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hbaDevice"), &self.data.hba_device as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ResolveMultipleUnresolvedVmfsVolumesRequestType<'a> {
    resolution_spec: &'a [crate::types::structs::HostUnresolvedVmfsResolutionSpec],
}

impl<'a> miniserde::Serialize for ResolveMultipleUnresolvedVmfsVolumesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ResolveMultipleUnresolvedVmfsVolumesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ResolveMultipleUnresolvedVmfsVolumesRequestTypeSer<'b, 'a> {
    data: &'b ResolveMultipleUnresolvedVmfsVolumesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ResolveMultipleUnresolvedVmfsVolumesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ResolveMultipleUnresolvedVmfsVolumesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("resolutionSpec"), &self.data.resolution_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ResolveMultipleUnresolvedVmfsVolumesExRequestType<'a> {
    resolution_spec: &'a [crate::types::structs::HostUnresolvedVmfsResolutionSpec],
}

impl<'a> miniserde::Serialize for ResolveMultipleUnresolvedVmfsVolumesExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ResolveMultipleUnresolvedVmfsVolumesExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ResolveMultipleUnresolvedVmfsVolumesExRequestTypeSer<'b, 'a> {
    data: &'b ResolveMultipleUnresolvedVmfsVolumesExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ResolveMultipleUnresolvedVmfsVolumesExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ResolveMultipleUnresolvedVmfsVolumesExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("resolutionSpec"), &self.data.resolution_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveDiskPartitionInfoRequestType<'a> {
    device_path: &'a [String],
}

impl<'a> miniserde::Serialize for RetrieveDiskPartitionInfoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveDiskPartitionInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveDiskPartitionInfoRequestTypeSer<'b, 'a> {
    data: &'b RetrieveDiskPartitionInfoRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveDiskPartitionInfoRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveDiskPartitionInfoRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("devicePath"), &self.data.device_path as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> miniserde::Serialize for SetCustomValueRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetCustomValueRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetCustomValueRequestTypeSer<'b, 'a> {
    data: &'b SetCustomValueRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetCustomValueRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"setCustomValueRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("value"), &self.data.value as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SetMultipathLunPolicyRequestType<'a> {
    lun_id: &'a str,
    policy: &'a dyn crate::types::traits::HostMultipathInfoLogicalUnitPolicyTrait,
}

impl<'a> miniserde::Serialize for SetMultipathLunPolicyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetMultipathLunPolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetMultipathLunPolicyRequestTypeSer<'b, 'a> {
    data: &'b SetMultipathLunPolicyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetMultipathLunPolicyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetMultipathLunPolicyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("lunId"), &self.data.lun_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("policy"), &self.data.policy as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SetNfsUserRequestType<'a> {
    user: &'a str,
    password: &'a str,
}

impl<'a> miniserde::Serialize for SetNfsUserRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetNfsUserRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetNfsUserRequestTypeSer<'b, 'a> {
    data: &'b SetNfsUserRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetNfsUserRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetNFSUserRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("user"), &self.data.user as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("password"), &self.data.password as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct TurnDiskLocatorLedOffRequestType<'a> {
    scsi_disk_uuids: &'a [String],
}

impl<'a> miniserde::Serialize for TurnDiskLocatorLedOffRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(TurnDiskLocatorLedOffRequestTypeSer { data: self, seq: 0 }))
    }
}

struct TurnDiskLocatorLedOffRequestTypeSer<'b, 'a> {
    data: &'b TurnDiskLocatorLedOffRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for TurnDiskLocatorLedOffRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"TurnDiskLocatorLedOffRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("scsiDiskUuids"), &self.data.scsi_disk_uuids as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct TurnDiskLocatorLedOnRequestType<'a> {
    scsi_disk_uuids: &'a [String],
}

impl<'a> miniserde::Serialize for TurnDiskLocatorLedOnRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(TurnDiskLocatorLedOnRequestTypeSer { data: self, seq: 0 }))
    }
}

struct TurnDiskLocatorLedOnRequestTypeSer<'b, 'a> {
    data: &'b TurnDiskLocatorLedOnRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for TurnDiskLocatorLedOnRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"TurnDiskLocatorLedOnRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("scsiDiskUuids"), &self.data.scsi_disk_uuids as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UnmapVmfsVolumeExRequestType<'a> {
    vmfs_uuid: &'a [String],
}

impl<'a> miniserde::Serialize for UnmapVmfsVolumeExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnmapVmfsVolumeExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnmapVmfsVolumeExRequestTypeSer<'b, 'a> {
    data: &'b UnmapVmfsVolumeExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnmapVmfsVolumeExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnmapVmfsVolumeExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsUuid"), &self.data.vmfs_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UnmountForceMountedVmfsVolumeRequestType<'a> {
    vmfs_uuid: &'a str,
}

impl<'a> miniserde::Serialize for UnmountForceMountedVmfsVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnmountForceMountedVmfsVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnmountForceMountedVmfsVolumeRequestTypeSer<'b, 'a> {
    data: &'b UnmountForceMountedVmfsVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnmountForceMountedVmfsVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnmountForceMountedVmfsVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsUuid"), &self.data.vmfs_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UnmountVffsVolumeRequestType<'a> {
    vffs_uuid: &'a str,
}

impl<'a> miniserde::Serialize for UnmountVffsVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnmountVffsVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnmountVffsVolumeRequestTypeSer<'b, 'a> {
    data: &'b UnmountVffsVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnmountVffsVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnmountVffsVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vffsUuid"), &self.data.vffs_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UnmountVmfsVolumeRequestType<'a> {
    vmfs_uuid: &'a str,
}

impl<'a> miniserde::Serialize for UnmountVmfsVolumeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnmountVmfsVolumeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnmountVmfsVolumeRequestTypeSer<'b, 'a> {
    data: &'b UnmountVmfsVolumeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnmountVmfsVolumeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnmountVmfsVolumeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsUuid"), &self.data.vmfs_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UnmountVmfsVolumeExRequestType<'a> {
    vmfs_uuid: &'a [String],
}

impl<'a> miniserde::Serialize for UnmountVmfsVolumeExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnmountVmfsVolumeExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnmountVmfsVolumeExRequestTypeSer<'b, 'a> {
    data: &'b UnmountVmfsVolumeExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnmountVmfsVolumeExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnmountVmfsVolumeExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsUuid"), &self.data.vmfs_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateDiskPartitionsRequestType<'a> {
    device_path: &'a str,
    spec: &'a crate::types::structs::HostDiskPartitionSpec,
}

impl<'a> miniserde::Serialize for UpdateDiskPartitionsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateDiskPartitionsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateDiskPartitionsRequestTypeSer<'b, 'a> {
    data: &'b UpdateDiskPartitionsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateDiskPartitionsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateDiskPartitionsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("devicePath"), &self.data.device_path as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateHppMultipathLunPolicyRequestType<'a> {
    lun_id: &'a str,
    policy: &'a crate::types::structs::HostMultipathInfoHppLogicalUnitPolicy,
}

impl<'a> miniserde::Serialize for UpdateHppMultipathLunPolicyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateHppMultipathLunPolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateHppMultipathLunPolicyRequestTypeSer<'b, 'a> {
    data: &'b UpdateHppMultipathLunPolicyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateHppMultipathLunPolicyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateHppMultipathLunPolicyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("lunId"), &self.data.lun_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("policy"), &self.data.policy as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateInternetScsiAdvancedOptionsRequestType<'a> {
    i_scsi_hba_device: &'a str,
    target_set: Option<&'a crate::types::structs::HostInternetScsiHbaTargetSet>,
    options: &'a [crate::types::structs::HostInternetScsiHbaParamValue],
}

impl<'a> miniserde::Serialize for UpdateInternetScsiAdvancedOptionsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateInternetScsiAdvancedOptionsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateInternetScsiAdvancedOptionsRequestTypeSer<'b, 'a> {
    data: &'b UpdateInternetScsiAdvancedOptionsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateInternetScsiAdvancedOptionsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateInternetScsiAdvancedOptionsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaDevice"), &self.data.i_scsi_hba_device as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.target_set else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targetSet"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("options"), &self.data.options as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct UpdateInternetScsiAliasRequestType<'a> {
    i_scsi_hba_device: &'a str,
    i_scsi_alias: &'a str,
}

impl<'a> miniserde::Serialize for UpdateInternetScsiAliasRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateInternetScsiAliasRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateInternetScsiAliasRequestTypeSer<'b, 'a> {
    data: &'b UpdateInternetScsiAliasRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateInternetScsiAliasRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateInternetScsiAliasRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaDevice"), &self.data.i_scsi_hba_device as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("iScsiAlias"), &self.data.i_scsi_alias as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateInternetScsiAuthenticationPropertiesRequestType<'a> {
    i_scsi_hba_device: &'a str,
    authentication_properties: &'a crate::types::structs::HostInternetScsiHbaAuthenticationProperties,
    target_set: Option<&'a crate::types::structs::HostInternetScsiHbaTargetSet>,
}

impl<'a> miniserde::Serialize for UpdateInternetScsiAuthenticationPropertiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateInternetScsiAuthenticationPropertiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateInternetScsiAuthenticationPropertiesRequestTypeSer<'b, 'a> {
    data: &'b UpdateInternetScsiAuthenticationPropertiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateInternetScsiAuthenticationPropertiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateInternetScsiAuthenticationPropertiesRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaDevice"), &self.data.i_scsi_hba_device as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("authenticationProperties"), &self.data.authentication_properties as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.target_set else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targetSet"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UpdateInternetScsiDigestPropertiesRequestType<'a> {
    i_scsi_hba_device: &'a str,
    target_set: Option<&'a crate::types::structs::HostInternetScsiHbaTargetSet>,
    digest_properties: &'a crate::types::structs::HostInternetScsiHbaDigestProperties,
}

impl<'a> miniserde::Serialize for UpdateInternetScsiDigestPropertiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateInternetScsiDigestPropertiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateInternetScsiDigestPropertiesRequestTypeSer<'b, 'a> {
    data: &'b UpdateInternetScsiDigestPropertiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateInternetScsiDigestPropertiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateInternetScsiDigestPropertiesRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaDevice"), &self.data.i_scsi_hba_device as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.target_set else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targetSet"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("digestProperties"), &self.data.digest_properties as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct UpdateInternetScsiDiscoveryPropertiesRequestType<'a> {
    i_scsi_hba_device: &'a str,
    discovery_properties: &'a crate::types::structs::HostInternetScsiHbaDiscoveryProperties,
}

impl<'a> miniserde::Serialize for UpdateInternetScsiDiscoveryPropertiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateInternetScsiDiscoveryPropertiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateInternetScsiDiscoveryPropertiesRequestTypeSer<'b, 'a> {
    data: &'b UpdateInternetScsiDiscoveryPropertiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateInternetScsiDiscoveryPropertiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateInternetScsiDiscoveryPropertiesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaDevice"), &self.data.i_scsi_hba_device as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("discoveryProperties"), &self.data.discovery_properties as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateInternetScsiIpPropertiesRequestType<'a> {
    i_scsi_hba_device: &'a str,
    ip_properties: &'a crate::types::structs::HostInternetScsiHbaIpProperties,
}

impl<'a> miniserde::Serialize for UpdateInternetScsiIpPropertiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateInternetScsiIpPropertiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateInternetScsiIpPropertiesRequestTypeSer<'b, 'a> {
    data: &'b UpdateInternetScsiIpPropertiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateInternetScsiIpPropertiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateInternetScsiIPPropertiesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaDevice"), &self.data.i_scsi_hba_device as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("ipProperties"), &self.data.ip_properties as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateInternetScsiNameRequestType<'a> {
    i_scsi_hba_device: &'a str,
    i_scsi_name: &'a str,
}

impl<'a> miniserde::Serialize for UpdateInternetScsiNameRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateInternetScsiNameRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateInternetScsiNameRequestTypeSer<'b, 'a> {
    data: &'b UpdateInternetScsiNameRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateInternetScsiNameRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateInternetScsiNameRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaDevice"), &self.data.i_scsi_hba_device as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("iScsiName"), &self.data.i_scsi_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateScsiLunDisplayNameRequestType<'a> {
    lun_uuid: &'a str,
    display_name: &'a str,
}

impl<'a> miniserde::Serialize for UpdateScsiLunDisplayNameRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateScsiLunDisplayNameRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateScsiLunDisplayNameRequestTypeSer<'b, 'a> {
    data: &'b UpdateScsiLunDisplayNameRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateScsiLunDisplayNameRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateScsiLunDisplayNameRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("lunUuid"), &self.data.lun_uuid as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("displayName"), &self.data.display_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateSoftwareInternetScsiEnabledRequestType {
    enabled: bool,
}

impl miniserde::Serialize for UpdateSoftwareInternetScsiEnabledRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateSoftwareInternetScsiEnabledRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateSoftwareInternetScsiEnabledRequestTypeSer<'b> {
    data: &'b UpdateSoftwareInternetScsiEnabledRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for UpdateSoftwareInternetScsiEnabledRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateSoftwareInternetScsiEnabledRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("enabled"), &self.data.enabled as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateVmfsUnmapBandwidthRequestType<'a> {
    vmfs_uuid: &'a str,
    unmap_bandwidth_spec: &'a crate::types::structs::VmfsUnmapBandwidthSpec,
}

impl<'a> miniserde::Serialize for UpdateVmfsUnmapBandwidthRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateVmfsUnmapBandwidthRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateVmfsUnmapBandwidthRequestTypeSer<'b, 'a> {
    data: &'b UpdateVmfsUnmapBandwidthRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateVmfsUnmapBandwidthRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateVmfsUnmapBandwidthRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsUuid"), &self.data.vmfs_uuid as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("unmapBandwidthSpec"), &self.data.unmap_bandwidth_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateVmfsUnmapPriorityRequestType<'a> {
    vmfs_uuid: &'a str,
    unmap_priority: &'a str,
}

impl<'a> miniserde::Serialize for UpdateVmfsUnmapPriorityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateVmfsUnmapPriorityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateVmfsUnmapPriorityRequestTypeSer<'b, 'a> {
    data: &'b UpdateVmfsUnmapPriorityRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateVmfsUnmapPriorityRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateVmfsUnmapPriorityRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsUuid"), &self.data.vmfs_uuid as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("unmapPriority"), &self.data.unmap_priority as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpgradeVmfsRequestType<'a> {
    vmfs_path: &'a str,
}

impl<'a> miniserde::Serialize for UpgradeVmfsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpgradeVmfsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpgradeVmfsRequestTypeSer<'b, 'a> {
    data: &'b UpgradeVmfsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpgradeVmfsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpgradeVmfsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vmfsPath"), &self.data.vmfs_path as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
