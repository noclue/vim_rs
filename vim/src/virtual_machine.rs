use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::AlarmState;
use crate::types::CustomFieldDef;
use crate::types::CustomFieldValueTrait;
use crate::types::CustomizationSpec;
use crate::types::DiskChangeInfo;
use crate::types::EventTrait;
use crate::types::FaultToleranceConfigSpec;
use crate::types::GuestInfo;
use crate::types::HostFeatureMask;
use crate::types::Id;
use crate::types::ManagedEntityStatusEnum;
use crate::types::ManagedObjectReference;
use crate::types::MethodFaultTrait;
use crate::types::Permission;
use crate::types::ResourceConfigSpec;
use crate::types::Tag;
use crate::types::UsbScanCodeSpec;
use crate::types::VirtualDisk;
use crate::types::VirtualMachineCapability;
use crate::types::VirtualMachineCloneSpec;
use crate::types::VirtualMachineConfigInfo;
use crate::types::VirtualMachineConfigSpec;
use crate::types::VirtualMachineConnectionTrait;
use crate::types::VirtualMachineDisplayTopology;
use crate::types::VirtualMachineFileLayout;
use crate::types::VirtualMachineFileLayoutEx;
use crate::types::VirtualMachineGuestQuiesceSpecTrait;
use crate::types::VirtualMachineInstantCloneSpec;
use crate::types::VirtualMachineMksTicket;
use crate::types::VirtualMachineMovePriorityEnum;
use crate::types::VirtualMachinePowerStateEnum;
use crate::types::VirtualMachineRelocateSpec;
use crate::types::VirtualMachineRuntimeInfo;
use crate::types::VirtualMachineSnapshotInfo;
use crate::types::VirtualMachineStorageInfo;
use crate::types::VirtualMachineSummary;
use crate::types::VirtualMachineTicket;
/// VirtualMachine is the managed object type for manipulating virtual machines,
/// including templates that can be deployed (repeatedly) as new virtual machines.
/// 
/// This type provides methods for configuring and controlling a virtual machine.
/// 
/// VirtualMachine extends the ManagedEntity type because virtual machines are
/// part of a virtual infrastructure inventory. The parent of a virtual machine
/// must be a folder, and a virtual machine has no children.
/// 
/// Destroying a virtual machine disposes of all associated storage, including
/// the virtual disks. To remove a virtual machine while retaining its
/// virtual disk storage, a client must remove the virtual disks
/// from the virtual machine before destroying it.
pub struct VirtualMachine {
    client: Arc<VimClient>,
    mo_id: String,
}
impl VirtualMachine {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of vSphere API 4.1, use *VirtualMachine.AcquireTicket* instead.
    /// 
    /// Creates and returns a one-time credential used in establishing a
    /// remote mouse-keyboard-screen connection to this virtual
    /// machine.
    /// 
    /// The correct function of this method depends on being able to
    /// retrieve TCP binding information about the server end of the
    /// client connection that is requesting the ticket. If such
    /// information is not available, the NotSupported fault is thrown.
    /// This method is appropriate for SOAP and authenticated connections,
    /// which are both TCP-based connections.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.ConsoleInteract
    ///
    /// ## Returns:
    ///
    /// A one-time credential used in establishing a remote
    /// mouse-keyboard-screen connection.
    pub async fn acquire_mks_ticket(&self) -> Result<VirtualMachineMksTicket> {
        let path = format!("/VirtualMachine/{moId}/AcquireMksTicket", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Creates and returns a one-time credential used in establishing a
    /// specific connection to this virtual machine, for example, a ticket
    /// type of mks can be used to establish a remote mouse-keyboard-screen
    /// connection.
    /// 
    /// A client using this ticketing mechanism must have network
    /// connectivity to the ESX server where the virtual machine is running,
    /// and the ESX server must be reachable to the management client from
    /// the address made available to the client via the ticket.
    /// 
    /// Acquiring a virtual machine ticket requires different privileges
    /// depending on the types of ticket:
    /// - VirtualMachine.Interact.DeviceConnection if requesting a device
    ///   ticket.
    /// - VirtualMachine.Interact.GuestControl if requesting a guestControl
    ///   or guestIntegrity ticket.
    /// - VirtualMachine.Interact.ConsoleInteract if requesting an mks
    ///   or webmks ticket.
    /// - VirtualMachine.Interact.DnD if requesting a drag and drop
    ///   ticket.
    ///
    /// ## Parameters:
    ///
    /// ### ticket_type
    /// The type of service to acquire, the set of possible
    /// values is described in *VirtualMachineTicketType_enum*.
    ///
    /// ## Returns:
    ///
    /// A one-time credential used in establishing a remote
    /// connection to this virtual machine.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the virtual machine is not connected.
    pub async fn acquire_ticket(&self, ticket_type: &str) -> Result<VirtualMachineTicket> {
        let input = AcquireTicketRequestType {ticket_type, };
        let path = format!("/VirtualMachine/{moId}/AcquireTicket", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Responds to a question that is blocking this virtual machine.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.AnswerQuestion
    ///
    /// ## Parameters:
    ///
    /// ### question_id
    /// The value from QuestionInfo.id that identifies the question
    /// to answer.
    ///
    /// ### answer_choice
    /// The contents of the QuestionInfo.choice.value array element
    /// that identifies the desired answer.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the questionId does not apply to this virtual machine.
    /// For example, this can happen if another client already answered the message.
    /// 
    /// ***ConcurrentAccess***: if the question has been or is being answered by
    /// another thread or user.
    pub async fn answer_vm(&self, question_id: &str, answer_choice: &str) -> Result<()> {
        let input = AnswerVmRequestType {question_id, answer_choice, };
        let path = format!("/VirtualMachine/{moId}/AnswerVM", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Applies the EVC mode masks to the virtual machine.
    /// 
    /// Existing masks will be replaced by the input masks.
    /// If the mask parameter is not set, then the masks on the virtual machine
    /// are removed.
    /// See *EVCMode.featureMask* for the list of masks to provide.
    /// These can be retrieved from *Capability.supportedEVCMode*,
    /// which is accessible in *ServiceInstance.capability*.
    /// 
    /// This operation is only supported if
    /// *VirtualMachineCapability.perVmEvcSupported* is true.
    /// 
    /// ***Required privileges:*** VirtualMachine.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### mask
    /// The feature masks to apply to the virtual machine.
    /// An empty set of masks will clear EVC settings.
    ///
    /// ### complete_masks
    /// Defaults to true if not set. A true value implies
    /// that any unspecified feature will not be exposed to the guest.
    /// A false value will expose any unspecified feature to the guest
    /// with the value of the host.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the power state is not poweredOff.
    pub async fn apply_evc_mode_vm_task(&self, mask: Option<&[HostFeatureMask]>, complete_masks: Option<bool>) -> Result<ManagedObjectReference> {
        let input = ApplyEvcModeVmRequestType {mask, complete_masks, };
        let path = format!("/VirtualMachine/{moId}/ApplyEvcModeVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Attach an existing disk to this virtual machine.
    /// 
    /// A minimum virtual machine version of 'vmx-13' is required for this
    /// operation to succeed. If a compatible VM version is not satisfied,
    /// a *DeviceUnsupportedForVmVersion* fault will be thrown.
    /// 
    /// ***Required privileges:*** VirtualMachine.Config.AddExistingDisk
    ///
    /// ## Parameters:
    ///
    /// ### disk_id
    /// The ID of the virtual disk to be operated. See
    /// *ID*
    ///
    /// ### datastore
    /// The datastore where the virtual disk is located.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### controller_key
    /// Key of the controller the disk will connect to.
    /// It can be unset if there is only one controller
    /// (SCSI or SATA) with the available slot in the
    /// virtual machine. If there are multiple SCSI or
    /// SATA controllers available, user must specify
    /// the controller; if there is no available
    /// controllers, a *MissingController*
    /// fault will be thrown.
    ///
    /// ### unit_number
    /// The unit number of the attached disk on its controller.
    /// If unset, the next available slot on the specified
    /// controller or the only available controller will be
    /// assigned to the attached disk.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the disk object cannot be found.
    /// 
    /// ***VmConfigFault***: if the virtual machine's configuration is invalid.
    /// 
    /// ***FileFault***: if there is a problem creating or accessing the virtual
    /// machine's files for this operation.
    /// 
    /// ***InvalidState***: if the operation cannot be performed in the current
    /// state of the virtual machine. For example, because the virtual
    /// machine's configuration is not available.
    /// 
    /// ***InvalidDatastore***: If the datastore cannot be found or inaccessible.
    /// 
    /// ***InvalidController***: If the specified controller cannot be found or
    /// the specified unitNumber is already taken, or
    /// the controller has no free slots.
    /// 
    /// ***MissingController***: If the virtual machine has no or more than one
    /// available controllers when controllerKey is
    /// unset.
    /// 
    /// ***DeviceUnsupportedForVmVersion***: If the virtual machine's version is
    /// incompatible for the given device.
    pub async fn attach_disk_task(&self, disk_id: &Id, datastore: &ManagedObjectReference, controller_key: Option<i32>, unit_number: Option<i32>) -> Result<ManagedObjectReference> {
        let input = AttachDiskRequestType {disk_id, datastore, controller_key, unit_number, };
        let path = format!("/VirtualMachine/{moId}/AttachDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Checks the customization specification against the virtual machine configuration.
    /// 
    /// For example, this is used on a source virtual machine before a clone operation to
    /// catch customization failure before the disk copy. This checks the specification's
    /// internal consistency as well as for compatibility with this virtual machine's
    /// configuration.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.Customize
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The customization specification to check.
    ///
    /// ## Errors:
    ///
    /// ***CustomizationFault***: A subclass of CustomizationFault is thrown.
    pub async fn check_customization_spec(&self, spec: &CustomizationSpec) -> Result<()> {
        let input = CheckCustomizationSpecRequestType {spec, };
        let path = format!("/VirtualMachine/{moId}/CheckCustomizationSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Creates a clone of this virtual machine.
    /// 
    /// If the virtual machine
    /// is used as a template, this method corresponds to the deploy command.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped, unless it
    /// is used to start an escape sequence. Clients may also escape any other characters
    /// in this name parameter.
    /// 
    /// The privilege required on the source virtual machine depends on the source
    /// and destination types:
    /// - source is virtual machine, destination is virtual machine -
    ///   VirtualMachine.Provisioning.Clone
    /// - source is virtual machine, destination is template -
    ///   VirtualMachine.Provisioning.CreateTemplateFromVM
    /// - source is template, destination is virtual machine -
    ///   VirtualMachine.Provisioning.DeployTemplate
    /// - source is template, destination is template -
    ///   VirtualMachine.Provisioning.CloneTemplate
    /// - source is encrypted virtual machine - Cryptographer.Clone
    ///   
    /// If customization is requested in the CloneSpec, then the
    /// VirtualMachine.Provisioning.Customize privilege must also be
    /// held on the source virtual machine.
    /// 
    /// The Resource.AssignVMToPool privilege is also required for the
    /// resource pool specified in the CloneSpec, if the destination is not a
    /// template.
    /// The Datastore.AllocateSpace privilege is required on all datastores
    /// where the clone is created.
    ///
    /// ## Parameters:
    ///
    /// ### folder
    /// The location of the new virtual machine.
    /// 
    /// ***Required privileges:*** VirtualMachine.Inventory.CreateFromExisting
    /// 
    /// Refers instance of *Folder*.
    ///
    /// ### name
    /// The name of the new virtual machine.
    ///
    /// ### spec
    /// Specifies how to clone the virtual machine. The folder
    /// specified in the spec takes precedence over the folder parameter.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the newly added *VirtualMachine* upon
    /// success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the host cannot run this virtual machine.
    /// 
    /// ***CustomizationFault***: if a customization error happens.
    /// Typically, a specific subclass of this exception is thrown.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the operation is not supported by the current agent.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// target datastores.
    /// 
    /// ***FileFault***: if there is an error accessing the virtual machine files.
    /// 
    /// ***VmConfigFault***: if the virtual machine is not compatible with the
    /// destination host. Typically, a specific subclass of this exception is
    /// thrown, such as IDEDiskNotSupported.
    /// 
    /// ***MigrationFault***: if it is not possible to migrate the virtual machine to
    /// the destination host. This is typically due to hosts being incompatible,
    /// such as mismatch in network polices or access to networks and datastores.
    /// Typically, a more specific subclass is thrown.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***NoPermission***: if the virtual machine is encrypted, but encryption
    /// is not enabled on the destination and the user does not have
    /// Cryptographer.RegisterHost permission on the host.
    /// 
    /// ***NoPermission***: if source virtual machine is encrypted, but the
    /// the user does not have Cryptographer.Clone permission on it.
    pub async fn clone_vm_task(&self, folder: &ManagedObjectReference, name: &str, spec: &VirtualMachineCloneSpec) -> Result<ManagedObjectReference> {
        let input = CloneVmRequestType {folder, name, spec, };
        let path = format!("/VirtualMachine/{moId}/CloneVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Consolidate the virtual disk files of the virtual machine by finding hierarchies
    /// of redo logs that can be combined without violating data dependency.
    /// 
    /// The
    /// redundant redo logs after merging are then deleted.
    /// Consolidation improves I/O performance since less number of virtual disk
    /// files need to be traversed; it also reduces the storage usage. However
    /// additional space is temporarily required to perform the operation. Use *VirtualMachine.EstimateStorageForConsolidateSnapshots_Task* to estimate the
    /// temporary space required.
    /// Consolidation can be I/O intensive, it is advisable to invoke this operation
    /// when guest is not under heavy I/O usage.
    /// 
    /// ***Required privileges:*** VirtualMachine.State.RemoveSnapshot
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
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***FileFault***: if if there is a problem creating or accessing the
    /// virtual machine's files for this operation. Typically a more
    /// specific fault for example *NoDiskSpace* is
    /// thrown.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual
    /// machine configuration information is not available.
    /// 
    /// ***VmConfigFault***: if a virtual machine configuration issue prevents
    /// consolidation. Typically, a more specific fault is thrown
    /// such as *InvalidDiskFormat* if a disk cannot
    /// be read, or *InvalidSnapshotFormat* if the
    /// snapshot configuration is invalid.
    pub async fn consolidate_vm_disks_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/ConsolidateVMDisks_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Create a screen shot of a virtual machine.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.CreateScreenshot
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***FileFault***: if there is a problem with creating or accessing one
    /// or more files needed for this operation.
    /// 
    /// ***InvalidPowerState***: if the virtual machine is not powered on.
    /// 
    /// ***InvalidState***: if the virtual machine is not ready to respond to
    /// such requests.
    pub async fn create_screenshot_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/CreateScreenshot_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 6.0, use *VirtualMachine.CreateSecondaryVMEx_Task* instead.
    /// 
    /// Creates a secondary virtual machine to be part of this fault tolerant group.
    /// 
    /// If a host is specified, the secondary virtual machine will be created on it.
    /// Otherwise, a host will be selected by the system.
    /// 
    /// If the primary virtual machine (i.e., this virtual machine) is powered on when
    /// the secondary is created, an attempt will be made to power on the secondary on
    /// a system selected host. If the cluster is a DRS cluster, DRS will be
    /// invoked to obtain a placement for the new secondary virtual machine. If the DRS
    /// recommendation (see *ClusterRecommendation*)
    /// is automatic, it will be automatically executed. Otherwise, the recommendation will
    /// be returned to the caller of this method and the secondary will remain powered off
    /// until the recommendation is approved using *ClusterComputeResource.ApplyRecommendation*.
    /// Failure to power on the secondary virtual machine will not fail the creation of the secondary.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.CreateSecondary
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The host where the secondary virtual machine is to be
    /// created and powered on. If no host is specified, a compatible host will be
    /// selected by the system. If a host cannot be found for the secondary or the specified
    /// host is not suitable, the secondary will not be created and a fault will be returned.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* returns an instance of the
    /// *FaultToleranceSecondaryOpResult* data object, which
    /// contains a reference to the created *VirtualMachine*
    /// and the status of powering it on, if attempted.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the virtual machine is marked as a template, or
    /// it is not in a vSphere HA enabled cluster.
    /// 
    /// ***InvalidState***: if the virtual machine's configuration information
    /// is not available.
    /// 
    /// ***ManagedObjectNotFound***: if a host is specified and it does not exist.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***VmFaultToleranceIssue***: if any error is encountered with the
    /// fault tolerance configuration of the virtual machine. Typically,
    /// a more specific fault like FaultToleranceNotLicensed is thrown.
    /// 
    /// ***FileFault***: if there is a problem accessing the virtual machine on the
    /// filesystem.
    /// 
    /// ***VmConfigFault***: if a configuration issue prevents creating the secondary.
    /// Typically, a more specific fault such as
    /// VmConfigIncompatibleForFaultTolerance is thrown.
    pub async fn create_secondary_vm_task(&self, host: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = CreateSecondaryVmRequestType {host, };
        let path = format!("/VirtualMachine/{moId}/CreateSecondaryVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a secondary virtual machine to be part of this fault tolerant group.
    /// 
    /// If a host is specified, the secondary virtual machine will be created on it.
    /// Otherwise, a host will be selected by the system.
    /// 
    /// If a FaultToleranceConfigSpec is specified, the virtual machine's
    /// configuration files and disks will be created in the specified datastores.
    /// 
    /// If the primary virtual machine (i.e., this virtual machine) is powered on when
    /// the secondary is created, an attempt will be made to power on the secondary on
    /// a system selected host. If the cluster is a DRS cluster, DRS will be
    /// invoked to obtain a placement for the new secondary virtual machine. If the DRS
    /// recommendation (see *ClusterRecommendation*)
    /// is automatic, it will be automatically executed. Otherwise, the recommendation will
    /// be returned to the caller of this method and the secondary will remain powered off
    /// until the recommendation is approved using *ClusterComputeResource.ApplyRecommendation*.
    /// Failure to power on the secondary virtual machine will not fail the creation of the secondary.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.CreateSecondary
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The host where the secondary virtual machine is to be
    /// created and powered on. If no host is specified, a compatible host will be
    /// selected by the system. If a host cannot be found for the secondary or the specified
    /// host is not suitable, the secondary will not be created and a fault will be returned.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### spec
    /// This parameter *FaultToleranceVMConfigSpec* can
    /// be used to specify the storage location of the fault tolerance
    /// tie-breaker file, secondary configuration file and secondary
    /// disks.
    /// 
    /// If the virtual machine is on a vSAN datastore, then the
    /// Fault Tolerance secondary virtual machine and the tie-breaker
    /// file also have to be placed on that same vSAN datastore.
    /// Conversely, if a primary VM is not using vSAN datastore,
    /// then its Fault Tolerance secondary virtual machine can
    /// not be placed on a vSAN datastore. Fault Tolerance is not
    /// supported for VMs that are using both vSAN and non-vSAN
    /// datastores for its configuration and disks.
    /// 
    /// If the virtual machine is using persistent memory for any of
    /// its disks, then its corresponding secondary disk placement
    /// entry should not be specified in the
    /// *FaultToleranceVMConfigSpec*. The system will
    /// automatically place the corresponding secondary disk on
    /// persistent memory.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* returns an instance of the
    /// *FaultToleranceSecondaryOpResult* data object, which
    /// contains a reference to the created *VirtualMachine*
    /// and the status of powering it on, if attempted.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the virtual machine is marked as a template, or
    /// it is not in a vSphere HA enabled cluster.
    /// 
    /// ***InvalidState***: if the virtual machine's configuration information
    /// is not available.
    /// 
    /// ***ManagedObjectNotFound***: if a host is specified and it does not exist.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***VmFaultToleranceIssue***: if any error is encountered with the
    /// fault tolerance configuration of the virtual machine. Typically,
    /// a more specific fault like FaultToleranceNotLicensed is thrown.
    /// 
    /// ***FileFault***: if there is a problem accessing the virtual machine on the
    /// filesystem.
    /// 
    /// ***VmConfigFault***: if a configuration issue prevents creating the secondary.
    /// Typically, a more specific fault such as
    /// VmConfigIncompatibleForFaultTolerance is thrown.
    pub async fn create_secondary_vm_ex_task(&self, host: Option<&ManagedObjectReference>, spec: Option<&FaultToleranceConfigSpec>) -> Result<ManagedObjectReference> {
        let input = CreateSecondaryVmExRequestType {host, spec, };
        let path = format!("/VirtualMachine/{moId}/CreateSecondaryVMEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere 8.0GA, this method is deprecated. Please
    /// use *VirtualMachine.CreateSnapshotEx_Task* instead.
    /// 
    /// Creates a new snapshot of this virtual machine.
    /// 
    /// As a side effect,
    /// this updates the current snapshot.
    /// 
    /// Snapshots are not supported for Fault Tolerance primary and secondary
    /// virtual machines.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped, unless it
    /// is used to start an escape sequence. Clients may also escape any other characters
    /// in this name parameter.
    /// 
    /// ***Required privileges:*** VirtualMachine.State.CreateSnapshot
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name for this snapshot. The name need not be unique for
    /// this virtual machine.
    ///
    /// ### description
    /// A description for this snapshot. If omitted, a default
    /// description may be provided.
    ///
    /// ### memory
    /// If TRUE, a dump of the internal state of the virtual machine
    /// (basically a memory dump) is included in the snapshot. Memory snapshots
    /// consume time and resources, and thus take longer to create. When set to FALSE,
    /// the power state of the snapshot is set to powered off.
    /// 
    /// *capabilities*
    /// indicates whether or not this virtual machine supports this operation.
    /// For a virtual machine in suspended state we always include memory
    /// unless *VirtualMachineCapability.diskOnlySnapshotOnSuspendedVMSupported* is
    /// true.
    ///
    /// ### quiesce
    /// If TRUE and the virtual machine is powered on when the
    /// snapshot is taken, VMware Tools is used to quiesce the file
    /// system in the virtual machine. This assures that a disk snapshot
    /// represents a consistent state of the guest file systems. If the virtual machine
    /// is powered off or VMware Tools are not available, the quiesce flag is ignored.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the newly created *VirtualMachineSnapshot* upon
    /// success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the host product does not support snapshots or if the host
    /// does not support quiesced snapshots and the quiesce parameter is set to true; or
    /// if the virtual machine is a Fault Tolerance primary or secondary
    /// 
    /// ***SnapshotFault***: if an error occurs during the snapshot operation.
    /// Typically a more specific fault like MultipleSnapshotsNotSupported
    /// is thrown.
    /// 
    /// ***FileFault***: if there is a problem with creating or accessing one
    /// or more files needed for this operation.
    /// 
    /// ***VmConfigFault***: if the virtual machine's configuration is invalid.
    /// Typically, a more specific fault like InvalidSnapshotState is thrown.
    /// 
    /// ***InvalidName***: if the specified snapshot name is invalid.
    /// 
    /// ***InvalidPowerState***: if the operation cannot be performed in the current
    /// power state of the virtual machine.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, the virtual machine
    /// configuration information is not available.
    pub async fn create_snapshot_task(&self, name: &str, description: Option<&str>, memory: bool, quiesce: bool) -> Result<ManagedObjectReference> {
        let input = CreateSnapshotRequestType {name, description, memory, quiesce, };
        let path = format!("/VirtualMachine/{moId}/CreateSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a new snapshot of this virtual machine.
    /// 
    /// As a side effect,
    /// this updates the current snapshot.
    /// 
    /// Snapshots are not supported for Fault Tolerance primary and secondary
    /// virtual machines.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped,
    /// unless it is used to start an escape sequence. Clients may also escape
    /// any other characters in this name parameter.
    /// 
    /// ***Required privileges:*** VirtualMachine.State.CreateSnapshot
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name for this snapshot. The name need not be unique for
    /// this virtual machine.
    ///
    /// ### description
    /// A description for this snapshot. If omitted, a default
    /// description may be provided.
    ///
    /// ### memory
    /// If TRUE, a dump of the internal state of the virtual machine
    /// (basically a memory dump) is included in the snapshot. Memory snapshots
    /// consume time and resources, and thus take longer to create.
    /// When set to FALSE, the power state of the snapshot is set to powered off.
    /// 
    /// *capabilities*
    /// indicates whether or not this virtual machine supports this operation.
    /// For a virtual machine in suspended state we always include memory
    /// unless *VirtualMachineCapability.diskOnlySnapshotOnSuspendedVMSupported* is
    /// true.
    ///
    /// ### quiesce_spec
    /// Spec for granular control over quiesce details.
    /// If quiesceSpec is set and the virtual machine is powered on when the
    /// snapshot is taken, VMware Tools is used to quiesce the file
    /// system in the virtual machine. This assures that a disk snapshot
    /// represents a consistent state of the guest file systems. If the virtual
    /// machine is powered off or VMware Tools are not available, the quiesce
    /// spec is ignored. If the spec type is *VirtualMachineGuestQuiesceSpec*, the
    /// default quiescing process will be applied. If the spec type is
    /// *VirtualMachineWindowsQuiesceSpec* and Guest OS is Windows, the parameters
    /// will control the VSS process.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. The *info.result* property
    /// in the *Task* contains the newly created
    /// *VirtualMachineSnapshot* upon success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if quiesceSpec is invalid.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the host product does not support snapshots or
    /// if the host does not support quiesced snapshots and the quiesce
    /// spec is set; or if the virtual machine is a Fault
    /// Tolerance primary or secondary; or if an unsupported quiesce
    /// spec is set.
    /// 
    /// ***SnapshotFault***: if an error occurs during the snapshot operation.
    /// Typically a more specific fault like MultipleSnapshotsNotSupported
    /// is thrown.
    /// 
    /// ***FileFault***: if there is a problem with creating or accessing one
    /// or more files needed for this operation.
    /// 
    /// ***VmConfigFault***: if the virtual machine's configuration is invalid.
    /// Typically, a more specific fault like InvalidSnapshotState is
    /// thrown.
    /// 
    /// ***InvalidName***: if the specified snapshot name is invalid.
    /// 
    /// ***InvalidPowerState***: if the operation cannot be performed in the
    /// current power state of the virtual machine.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, the virtual machine
    /// configuration information is not available.
    pub async fn create_snapshot_ex_task(&self, name: &str, description: Option<&str>, memory: bool, quiesce_spec: Option<&dyn VirtualMachineGuestQuiesceSpecTrait>) -> Result<ManagedObjectReference> {
        let input = CreateSnapshotExRequestType {name, description, memory, quiesce_spec, };
        let path = format!("/VirtualMachine/{moId}/CreateSnapshotEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Unlocks an encrypted virtual machine by sending the encryption keys for
    /// the Virtual Machine Home and all the Virtual Disks to the ESX Server.
    /// 
    /// ***Required privileges:*** Cryptographer.RegisterVM
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: when the required Key Management Server is not
    /// configured.
    /// 
    /// ***InvalidVmState***: when the virtual machine failed to unlock.
    /// 
    /// ***NotSupported***: if the ESX server doesn't support encryption.
    pub async fn crypto_unlock_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/CryptoUnlock_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Customizes a virtual machine's guest operating system.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.Customize
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The customization specification object.
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
    /// ***CustomizationFault***: A subclass of CustomizationFault is thrown.
    pub async fn customize_vm_task(&self, spec: &CustomizationSpec) -> Result<ManagedObjectReference> {
        let input = CustomizeVmRequestType {spec, };
        let path = format!("/VirtualMachine/{moId}/CustomizeVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Defragment all virtual disks attached to this virtual machine.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.DefragmentAllDisks
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the virtual machine is not connected.
    /// 
    /// ***InvalidPowerState***: if the virtual machine is poweredOn.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***FileFault***: if there is an error accessing the disk files.
    pub async fn defragment_all_disks(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/DefragmentAllDisks", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Destroys this object, deleting its contents and removing it from its parent
    /// folder (if any).
    /// 
    /// NOTE: The appropriate privilege must be held on the parent of the destroyed
    /// entity as well as the entity itself.
    /// This method can throw one of several exceptions. The exact set of exceptions
    /// depends on the kind of entity that is being removed. See comments for
    /// each entity for more information on destroy behavior.
    /// 
    /// ***Required privileges:*** VirtualMachine.Inventory.Delete
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
    /// Failure
    pub async fn destroy_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/Destroy_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Detach a disk from this virtual machine.
    /// 
    /// ***Required privileges:*** VirtualMachine.Config.RemoveDisk
    ///
    /// ## Parameters:
    ///
    /// ### disk_id
    /// The ID of the virtual disk to be operated. See
    /// *ID*
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the disk object cannot be found.
    /// 
    /// ***VmConfigFault***: if the virtual machine's configuration is invalid.
    /// 
    /// ***FileFault***: if there is a problem creating or accessing the virtual
    /// machine's files for this operation.
    /// 
    /// ***InvalidState***: if the operation cannot be performed in the current
    /// state of the virtual machine. For example, because the virtual
    /// machine's configuration is not available.
    pub async fn detach_disk_task(&self, disk_id: &Id) -> Result<ManagedObjectReference> {
        let input = DetachDiskRequestType {disk_id, };
        let path = format!("/VirtualMachine/{moId}/DetachDisk_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Disables the specified secondary virtual machine in this fault tolerant group.
    /// 
    /// The specified secondary will not be automatically started on a subsequent
    /// power-on of the primary virtual machine.
    /// This operation could leave the primary virtual machine in a non-fault
    /// tolerant state.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.DisableSecondary
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The secondary virtual machine specified will be disabed.
    /// This field must specify a secondary virtual machine that is part of the fault
    /// tolerant group that this virtual machine is currently associated with. It can
    /// only be invoked from the primary virtual machine in the group.
    /// 
    /// Refers instance of *VirtualMachine*.
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
    /// ***VmFaultToleranceIssue***: if any error is encountered with the
    /// fault tolerance configuration of the virtual machine. Typically,
    /// a more specific fault like InvalidOperationOnSecondaryVm is thrown.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidState***: if the host is in maintenance mode or if
    /// the virtual machine's configuration information is not available.
    pub async fn disable_secondary_vm_task(&self, vm: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = DisableSecondaryVmRequestType {vm, };
        let path = format!("/VirtualMachine/{moId}/DisableSecondaryVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Force the virtual machine to drop the specified connections.
    /// 
    /// Attempt to drop the specified virtual machine connections. An attempt
    /// will be made to drop all of the specified connections before returning.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.0
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.ConsoleInteract
    ///
    /// ## Parameters:
    ///
    /// ### list_of_connections
    /// -
    ///
    /// ## Returns:
    ///
    /// true All of the specified connections have been dropped.
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: If the virtual machine is not powered on.
    /// No connection drop actions will have been
    /// attempted if this is thrown.
    pub async fn drop_connections(&self, list_of_connections: Option<&[Box<dyn VirtualMachineConnectionTrait>]>) -> Result<bool> {
        let input = DropConnectionsRequestType {list_of_connections, };
        let path = format!("/VirtualMachine/{moId}/DropConnections", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Enables the specified secondary virtual machine in this fault tolerant group.
    /// 
    /// This operation is used to enable a secondary virtual machine that was
    /// previously disabled by the *VirtualMachine.DisableSecondaryVM_Task*
    /// call. The specified secondary will be automatically started whenever the
    /// primary is powered on.
    /// 
    /// If the primary virtual machine (i.e., this virtual machine) is powered on when
    /// the secondary is enabled, an attempt will be made to power on the secondary. If
    /// a host was specified in the method call, this host will be used. If a host is
    /// not specified, one will be selected by the system. In the latter case, if the cluster
    /// is a DRS cluster, DRS will be invoked to obtain a placement for the new secondary
    /// virtual machine. If the DRS recommendation (see *ClusterRecommendation*)
    /// is automatic, it will be executed. Otherwise, the recommendation will be
    /// returned to the caller of this method and the secondary will remain powered off
    /// until the recommendation is approved using *ClusterComputeResource.ApplyRecommendation*.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.EnableSecondary
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The secondary virtual machine specified will be enabled.
    /// This field must specify a secondary virtual machine that is part of the fault
    /// tolerant group that this virtual machine is currently associated with. It can
    /// only be invoked from the primary virtual machine in the group.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### host
    /// The host on which the secondary virtual machine is to be
    /// enabled and possibly powered on. If no host is specified, a compatible host
    /// will be selected by the system. If the secondary virtual machine is not
    /// compatible with the specified host, the secondary will not be re-enabled
    /// and a fault will be returned.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* returns an instance of the
    /// *FaultToleranceSecondaryOpResult* data object, which
    /// contains a reference to the *VirtualMachine*
    /// and the status of powering it on, if attempted.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VmConfigFault***: if a configuration issue prevents enabling the secondary.
    /// Typically, a more specific fault such as
    /// VmConfigIncompatibleForFaultTolerance is thrown.
    /// 
    /// ***VmFaultToleranceIssue***: if any error is encountered with the
    /// fault tolerance configuration of the virtual machine. Typically,
    /// a more specific fault like InvalidOperationOnSecondaryVm is thrown.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***ManagedObjectNotFound***: if a host is specified and it does not exist.
    /// 
    /// ***InvalidState***: if the virtual machine's configuration information is not
    /// available, if the secondary virtual machine is not disabled, or if a
    /// power-on is attempted and one is already in progress.
    pub async fn enable_secondary_vm_task(&self, vm: &ManagedObjectReference, host: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = EnableSecondaryVmRequestType {vm, host, };
        let path = format!("/VirtualMachine/{moId}/EnableSecondaryVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Estimate the temporary space required to consolidation disk
    /// files.
    /// 
    /// The estimation is a lower bound if the childmost writable disk
    /// file will be consolidated for an online virtual machine, it is
    /// accurate for all other situations. This is because the space
    /// requirement depending on the size of the childmost disk file and how
    /// write intensive the guest is.
    /// 
    /// This method can be used prior to invoke consolidation via
    /// *VirtualMachine.ConsolidateVMDisks_Task*.
    /// 
    /// ***Required privileges:*** VirtualMachine.State.RemoveSnapshot
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
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    /// 
    /// ***FileFault***: if if there is a problem accessing the
    /// virtual machine's files for this operation. Typically a more
    /// specific fault *FileLocked* is thrown.
    /// 
    /// ***VmConfigFault***: if a virtual machine configuration issue prevents
    /// the estimation. Typically, a more specific fault is thrown.
    pub async fn estimate_storage_for_consolidate_snapshots_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/EstimateStorageForConsolidateSnapshots_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Obtains an export lease on this virtual machine.
    /// 
    /// The export lease contains
    /// a list of URLs for the virtual disks for this virtual machine, as well as
    /// a ticket giving access to the URLs.
    /// 
    /// See *HttpNfcLease* for information on how to use the lease.
    /// 
    /// ***Required privileges:*** VApp.Export
    ///
    /// ## Returns:
    ///
    /// The export lease on this *VirtualMachine*. The
    /// export task continues running until the lease is completed by the
    /// caller.
    /// 
    /// Refers instance of *HttpNfcLease*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the virtual machine is powered on.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    /// 
    /// ***FileFault***: if there is an error accessing the virtual machine files.
    pub async fn export_vm(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/ExportVm", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Returns the OVF environment for a virtual machine.
    /// 
    /// If the virtual machine has no
    /// vApp configuration, an empty string is returned. Also, sensitive information
    /// is omitted, so this method is not guaranteed to return the complete OVF
    /// environment.
    /// 
    /// ***Required privileges:*** VApp.ExtractOvfEnvironment
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the virtual machine is not running
    pub async fn extract_ovf_environment(&self) -> Result<String> {
        let path = format!("/VirtualMachine/{moId}/ExtractOvfEnvironment", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a powered-on Instant Clone of a virtual machine.
    /// 
    /// The new
    /// virtual machine will be created on the same host and start with the
    /// identical running point as the original virtual machine, sharing memory
    /// state when possible and sharing disk state.
    /// The original virtual machine must be in a powered-on state.
    /// The privilege required for Instant Clone operation are:
    /// - VirtualMachine.Provisioning.Clone
    /// - VirtualMachine.Interact.PowerOn
    /// - VirtualMachine.Inventory.CreateFromExisting
    /// - Datastore.AllocateSpace
    /// - Resource.AssignVMToPool
    ///   
    /// ***Required privileges:*** VirtualMachine.Provisioning.Clone
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Is a *VirtualMachineInstantCloneSpec*. It specifies the
    /// cloned virtual machine's configuration.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: in the following cases:
    /// - Source virtual machine is not powered on
    /// - Source virtual machine configuration is not supported for
    ///   Instant Clone operation
    /// - Relocation specification has unsupported settings 
    ///   
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// host or virtual machine's current state. For example, if the host
    /// is in maintenance mode or if the source virtual machine is not
    /// powered on.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// target datastores.
    /// 
    /// ***FileFault***: if there is an error accessing the virtual machine
    /// files.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a
    /// resource usage policy.
    /// 
    /// ***DisallowedMigrationDeviceAttached***: if any of the devices attached
    /// to the source virtual machine are not supported for the Instant
    /// Clone operation or if device change specification contains
    /// changes that are not supported.
    pub async fn instant_clone_task(&self, spec: &VirtualMachineInstantCloneSpec) -> Result<ManagedObjectReference> {
        let input = InstantCloneRequestType {spec, };
        let path = format!("/VirtualMachine/{moId}/InstantClone_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Makes the specified secondary virtual machine from this fault tolerant group as
    /// the primary virtual machine.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.MakePrimary
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The secondary virtual machine specified will be made the primary
    /// virtual machine.
    /// This field must specify a secondary virtual machine that is part of the fault
    /// tolerant group that this virtual machine is currently associated with. It can
    /// only be invoked from the primary virtual machine in the group.
    /// 
    /// Refers instance of *VirtualMachine*.
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
    /// ***VmFaultToleranceIssue***: if any error is encountered with the
    /// fault tolerance configuration of the virtual machine. Typically,
    /// a more specific fault like InvalidOperationOnSecondaryVm is thrown.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidState***: if the host is in maintenance mode or if
    /// the virtual machine's configuration information is not available.
    pub async fn make_primary_vm_task(&self, vm: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = MakePrimaryVmRequestType {vm, };
        let path = format!("/VirtualMachine/{moId}/MakePrimaryVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Marks a VirtualMachine object as being used as a template.
    /// 
    /// Note: A VirtualMachine marked as a template cannot be powered on.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.MarkAsTemplate
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if marking a virtual machine as a template is not supported.
    /// 
    /// ***InvalidPowerState***: if the virtual machine is not powered off.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    /// 
    /// ***VmConfigFault***: if the template is incompatible with the host, such
    /// as the files are not accessible.
    /// 
    /// ***FileFault***: if there is an error accessing the virtual machine files.
    pub async fn mark_as_template(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/MarkAsTemplate", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Clears the 'isTemplate' flag and reassociates the virtual machine with
    /// a resource pool and host.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.MarkAsVM
    ///
    /// ## Parameters:
    ///
    /// ### pool
    /// Resource pool to associate with the virtual machine.
    /// 
    /// ***Required privileges:*** Resource.AssignVMToPool
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ### host
    /// The target host on which the virtual machine is intended to run. The
    /// host
    /// parameter must specify a host that is a member of the ComputeResource
    /// indirectly specified by the pool. For a stand-alone host or a cluster with
    /// DRS, it can be omitted and the system selects a default.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if marking a template as a virtual machine is not
    /// supported.
    /// 
    /// ***InvalidState***: if the virtual machine is not marked as a template.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// target datastores.
    /// 
    /// ***VmConfigFault***: if the virtual machine is not compatible with the
    /// host. For example, a DisksNotSupported fault if the destination host
    /// does not support the disk backings of the template.
    /// 
    /// ***FileFault***: if there is an error accessing the virtual machine files.
    pub async fn mark_as_virtual_machine(&self, pool: &ManagedObjectReference, host: Option<&ManagedObjectReference>) -> Result<()> {
        let input = MarkAsVirtualMachineRequestType {pool, host, };
        let path = format!("/VirtualMachine/{moId}/MarkAsVirtualMachine", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Deprecated as of vSphere 6.5, use *VirtualMachine.RelocateVM_Task*
    /// instead.
    /// 
    /// Migrates a virtual machine's execution to a specific resource pool or host.
    /// 
    /// Requires Resource.HotMigrate privilege if the virtual machine is powered on or
    /// Resource.ColdMigrate privilege if the virtual machine is powered off or
    /// suspended.
    ///
    /// ## Parameters:
    ///
    /// ### pool
    /// The target resource pool for the virtual machine. If the pool
    /// parameter is left unset, the virtual machine's current pool is used
    /// as the target pool.
    /// 
    /// ***Required privileges:*** Resource.AssignVMToPool
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ### host
    /// The target host to which the virtual machine is intended to migrate.
    /// The host parameter
    /// may be left unset if the compute resource associated with the
    /// target pool represents a stand-alone host or a DRS-enabled cluster.
    /// In the former case the stand-alone host is used as the target host.
    /// In the latter case, the DRS system selects an appropriate
    /// target host from the cluster.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### priority
    /// The task priority (@see vim.VirtualMachine.MovePriority).
    ///
    /// ### state
    /// If specified, the virtual machine migrates only if
    /// its state matches the specified state.
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
    /// ***NotSupported***: if the virtual machine is marked as a template.
    /// 
    /// ***InvalidArgument***: in the following cases:
    /// - the target host and target pool are not associated with the
    ///   same compute resource
    /// - the host parameter is left unset when the target pool is
    ///   associated with a non-DRS cluster
    ///   
    /// ***InvalidPowerState***: if the state argument is set and the virtual
    /// machine does not have that power state.
    /// 
    /// ***FileFault***: if, in a case where the virtual machine
    /// configuration file must be copied, the destination location for
    /// that file does not have the necessary file access permissions.
    /// 
    /// ***VmConfigFault***: if the virtual machine is not compatible with the
    /// destination host. Typically, a specific subclass of this exception is
    /// thrown, such as IDEDiskNotSupported.
    /// 
    /// ***MigrationFault***: if it is not possible to migrate the virtual machine to
    /// the destination host. This is typically due to hosts being incompatible,
    /// such as mismatch in network polices or access to networks and datastores.
    /// Typically, a more specific subclass is thrown.
    /// 
    /// ***Timedout***: if one of the phases of the migration process times out.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state or the target host's current state.
    /// For example, if the virtual machine configuration information is not
    /// available or if the target host is disconnected or in maintenance mode.
    /// 
    /// ***NoActiveHostInCluster***: if a target host is not specified and the
    /// cluster associated with the target pool does not contain at least one
    /// potential target host. A host must be connected and not in maintenance
    /// mode in order to be considered as a potential target host.
    /// 
    /// ***NoPermission***: if the virtual machine is encrypted, but encryption is
    /// not enabled on the destination host and the user does not have
    /// Cryptographer.RegisterHost permission on it.
    /// 
    /// ***NoPermission***: if the virtual machine is encrypted, but the
    /// the user does not have Cryptographer.Migrate permission on the VM.
    pub async fn migrate_vm_task(&self, pool: Option<&ManagedObjectReference>, host: Option<&ManagedObjectReference>, priority: VirtualMachineMovePriorityEnum, state: Option<VirtualMachinePowerStateEnum>) -> Result<ManagedObjectReference> {
        let input = MigrateVmRequestType {pool, host, priority, state, };
        let path = format!("/VirtualMachine/{moId}/MigrateVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Mounts the VMware Tools CD installer as a CD-ROM for the guest operating system.
    /// 
    /// To monitor the status of the tools install, clients should check the tools status,
    /// *GuestInfo.toolsVersionStatus* and
    /// *GuestInfo.toolsRunningStatus*
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.ToolsInstall
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the virtual machine is not running,
    /// or the VMware Tools CD is already mounted.
    /// 
    /// ***VmToolsUpgradeFault***: if the VMware Tools CD failed to mount.
    pub async fn mount_tools_installer(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/MountToolsInstaller", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Powers off this virtual machine.
    /// 
    /// If this virtual machine is a fault tolerant primary virtual machine, this
    /// will result in the secondary virtual machine(s) getting powered off as well.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.PowerOff
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
    /// ***InvalidPowerState***: if the power state is not poweredOn.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the virtual machine is marked as a template.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    pub async fn power_off_vm_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/PowerOffVM_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Powers on this virtual machine.
    /// 
    /// If the virtual machine is suspended,
    /// this method resumes execution from the suspend point.
    /// 
    /// When powering on a virtual machine in a cluster, the system might implicitly
    /// or due to the host argument, do an implicit relocation of the virtual machine
    /// to another host. Hence, errors related to this relocation can be thrown. If the
    /// cluster is a DRS cluster, DRS will be invoked if the virtual machine can be
    /// automatically placed by DRS (see *DrsBehavior_enum*).
    /// Because this method does not return a DRS *ClusterRecommendation*, no
    /// vmotion nor host power operations will be done as part of a DRS-facilitated power
    /// on. To have DRS consider such operations use *Datacenter.PowerOnMultiVM_Task*.
    /// As of vSphere API 5.1, use of this method with vCenter Server is deprecated;
    /// use *Datacenter.PowerOnMultiVM_Task* instead.
    /// 
    /// If this virtual machine is a fault tolerant primary virtual machine, its
    /// secondary virtual machines will be started on system-selected
    /// hosts. If the virtual machines are in a VMware DRS enabled cluster,
    /// then DRS will be invoked to obtain placements for the secondaries but
    /// no vmotion nor host power operations will be considered for these power ons.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.PowerOn
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// (optional) The host where the virtual machine is to be powered on.
    /// If no host is specified, the current associated host is used. This field must
    /// specify a host that is part of the same compute resource that the virtual machine
    /// is currently associated with. If this host is not compatible, the current host
    /// association is used.
    /// 
    /// Refers instance of *HostSystem*.
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
    /// ***InvalidPowerState***: if the power state is poweredOn.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotEnoughLicenses***: if there are not enough licenses to power on this
    /// virtual machine.
    /// 
    /// ***NotSupported***: if the virtual machine is marked as a template.
    /// 
    /// ***InvalidState***: if the host is in maintenance mode or if
    /// the virtual machine's configuration information is not available
    /// or if the virtual machine is already powering on
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***VmConfigFault***: if a configuration issue prevents the power-on.
    /// Typically, a more specific fault, such as UnsupportedVmxLocation, is
    /// thrown.
    /// 
    /// ***FileFault***: if there is a problem accessing the virtual machine on the
    /// filesystem.
    /// 
    /// ***DisallowedOperationOnFailoverHost***: if the host specified is a failover
    /// host. See *ClusterFailoverHostAdmissionControlPolicy*.
    pub async fn power_on_vm_task(&self, host: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = PowerOnVmRequestType {host, };
        let path = format!("/VirtualMachine/{moId}/PowerOnVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Promotes disks on this virtual machine that have delta disk backings.
    /// 
    /// A delta disk backing is a way to preserve a virtual disk backing
    /// at some point in time. A delta disk backing is a file backing which in
    /// turn points to the original virtual disk backing (the parent). After a delta
    /// disk backing is added, all writes go to the delta disk backing. All reads
    /// first try the delta disk backing and then try the parent backing if needed.
    /// 
    /// Promoting does two things
    /// 1. If the unlink parameter is true, any disk backing which is shared
    ///    shared by multiple virtual machines is copied so that this virtual machine
    ///    has its own unshared version. Copied files always end up in the virtual
    ///    machine's home directory. To promote the disks of a powered on VM,
    ///    the VM cannot have snapshots.
    /// 2. Any disk backing which is not shared between multiple virtual
    ///    machines and is not associated with a snapshot is consolidated
    ///    with its child backing.
    ///    
    /// If the unlink parameter is true, the net effect of this operation is improved
    /// read performance, at the cost of disk space. If the unlink parameter is
    /// false the net effect is improved read performance at the cost of inhibiting
    /// future sharing.
    /// 
    /// This operation is only supported if
    /// *HostCapability.deltaDiskBackingsSupported* is true.
    /// 
    /// This operation is only supported on VirtualCenter. If no work is required,
    /// an invocation completes successfully.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.PromoteDisks
    ///
    /// ## Parameters:
    ///
    /// ### unlink
    /// If true, then these disks will be unlinked before consolidation.
    ///
    /// ### disks
    /// The set of disks that are to be promoted.
    /// If this value is unset or the array is empty,
    /// all disks which have delta disk backings are promoted.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the host doesn't support disk promotion APIs.
    /// 
    /// ***InvalidState***: if the virtual machine's power state changes
    /// during the execution of this method.
    /// 
    /// ***InvalidState***: if the virtual machine is not ready to respond to
    /// such requests.
    pub async fn promote_disks_task(&self, unlink: bool, disks: Option<&[VirtualDisk]>) -> Result<ManagedObjectReference> {
        let input = PromoteDisksRequestType {unlink, disks, };
        let path = format!("/VirtualMachine/{moId}/PromoteDisks_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Inject a sequence of USB HID scan codes into the keyboard.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.PutUsbScanCodes
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// -
    ///
    /// ## Returns:
    ///
    /// Number of keys injected.
    pub async fn put_usb_scan_codes(&self, spec: &UsbScanCodeSpec) -> Result<i32> {
        let input = PutUsbScanCodesRequestType {spec, };
        let path = format!("/VirtualMachine/{moId}/PutUsbScanCodes", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Get a list of areas of a virtual disk belonging to this VM that have
    /// been modified since a well-defined point in the past.
    /// 
    /// The beginning of
    /// the change interval is identified by "changeId", while the end of the
    /// change interval is implied by the snapshot ID passed in.
    /// 
    /// Note that the result of this function may contain "false positives"
    /// (i.e: flag areas of the disk as modified that are not). However, it is
    /// guaranteed that no changes will be missed.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.DiskRandomRead
    ///
    /// ## Parameters:
    ///
    /// ### snapshot
    /// Snapshot for which changes that have been made sine
    /// "changeId" should be computed. If not set, changes are computed
    /// against the "current" snapshot of the virtual machine. However,
    /// using the "current" snapshot will only work for virtual machines
    /// that are powered off.
    /// 
    /// Refers instance of *VirtualMachineSnapshot*.
    ///
    /// ### device_key
    /// Identifies the virtual disk for which to compute changes.
    ///
    /// ### start_offset
    /// Start Offset in bytes at which to start computing changes.
    /// Typically, callers will make multiple calls to this function, starting
    /// with startOffset 0 and then examine the "length" property in the
    /// returned DiskChangeInfo structure, repeatedly calling queryChangedDiskAreas
    /// until a map forthe entire virtual disk has been obtained.
    ///
    /// ### change_id
    /// Identifyer referring to a point in the past that should be used
    /// as the point in time at which to begin including changes to the disk in
    /// the result. A typical use case would be a backup application obtaining a
    /// changeId from a virtual disk's backing info when performing a
    /// backup. When a subsequent incremental backup is to be performed, this
    /// change Id can be used to obtain a list of changed areas on disk.
    ///
    /// ## Returns:
    ///
    /// Returns a data structure specifying extents of the virtual disk that
    /// have changed since the thime the changeId string was obtained.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the snapshot specified does not exist.
    /// 
    /// ***InvalidArgument***: if deviceKey does not specify a virtual disk, startOffset
    /// is beyond the end of the virtual disk or changeId is invalid or change
    /// tracking is not supported for this particular disk.
    /// 
    /// ***FileFault***: if the virtual disk files cannot be accessed/queried.
    pub async fn query_changed_disk_areas(&self, snapshot: Option<&ManagedObjectReference>, device_key: i32, start_offset: i64, change_id: &str) -> Result<DiskChangeInfo> {
        let input = QueryChangedDiskAreasRequestType {snapshot, device_key, start_offset, change_id, };
        let path = format!("/VirtualMachine/{moId}/QueryChangedDiskAreas", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Ask the virtual machine for a list of connections.
    /// 
    /// The virtual machine returns a list of connections.
    /// It is possible for the array returned to be empty - a virtual machine
    /// may have no connections.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.0
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.ConsoleInteract
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: If the virtual machine is not powered on.
    /// 
    /// ***Timedout***: If the the virtual machine did not respond
    /// to the request in a timely manner.
    /// 
    /// ***VmConfigFault***: If an error occurred.
    pub async fn query_connections(&self) -> Result<Option<Vec<Box<dyn VirtualMachineConnectionTrait>>>> {
        let path = format!("/VirtualMachine/{moId}/QueryConnections", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Deprecated as of vSphere API 6.0.
    /// 
    /// This API can be invoked to determine whether a virtual machine is
    /// compatible for legacy Fault Tolerance.
    /// 
    /// The API only checks for
    /// VM-specific factors that impact compatibility for RecordReplay based
    /// Fault Tolerance. Other requirements for Fault Tolerance such as host
    /// processor compatibility, logging nic configuration and licensing are
    /// not covered by this API.
    /// The query returns a list of faults, each fault corresponding to a
    /// specific incompatibility. If a given virtual machine is
    /// compatible for Fault Tolerance, then the fault list returned will be
    /// empty.
    /// 
    /// ***Required privileges:*** VirtualMachine.Config.QueryFTCompatibility
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the operation cannot be performed because of
    /// the virtual machine's current state.
    /// 
    /// ***VmConfigFault***: if the virtual machine's configuration is invalid.
    /// 
    /// ***NotSupported***: if the virtual machine is a template or this operation
    /// is not supported.
    pub async fn query_fault_tolerance_compatibility(&self) -> Result<Option<Vec<Box<dyn MethodFaultTrait>>>> {
        let path = format!("/VirtualMachine/{moId}/QueryFaultToleranceCompatibility", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// This API can be invoked to determine whether a virtual machine is
    /// compatible for Fault Tolerance.
    /// 
    /// The API only checks for VM-specific
    /// factors that impact compatibility for Fault Tolerance. Other
    /// requirements for Fault Tolerance such as host processor compatibility,
    /// logging nic configuration and licensing are not covered by this API.
    /// The query returns a list of faults, each fault corresponding to a
    /// specific incompatibility. If a given virtual machine is
    /// compatible for Fault Tolerance, then the fault list returned will be
    /// empty.
    /// 
    /// ***Required privileges:*** VirtualMachine.Config.QueryFTCompatibility
    ///
    /// ## Parameters:
    ///
    /// ### for_legacy_ft
    /// checks for legacy record-replay FT compatibility only
    /// if this is set to true.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the operation cannot be performed because of
    /// the virtual machine's current state.
    /// 
    /// ***VmConfigFault***: if the virtual machine's configuration is invalid.
    /// 
    /// ***NotSupported***: if the virtual machine is a template or this operation
    /// is not supported.
    pub async fn query_fault_tolerance_compatibility_ex(&self, for_legacy_ft: Option<bool>) -> Result<Option<Vec<Box<dyn MethodFaultTrait>>>> {
        let input = QueryFaultToleranceCompatibilityExRequestType {for_legacy_ft, };
        let path = format!("/VirtualMachine/{moId}/QueryFaultToleranceCompatibilityEx", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// For all files that belong to the vm, check that the file owner
    /// is set to the current datastore principal user, as set by
    /// *HostDatastoreSystem.ConfigureDatastorePrincipal*
    /// 
    /// ***Required privileges:*** VirtualMachine.Config.QueryUnownedFiles
    ///
    /// ## Returns:
    ///
    /// The list of file paths for vm files whose ownership is
    /// not correct.
    /// Use *FileManager.ChangeOwner* to set the file ownership.
    pub async fn query_unowned_files(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/VirtualMachine/{moId}/QueryUnownedFiles", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Issues a command to the guest operating system asking it to perform
    /// a reboot.
    /// 
    /// Returns immediately and does not wait for the guest operating system
    /// to complete the operation.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.Reset
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the power state is not powered on.
    /// 
    /// ***ToolsUnavailable***: if VMware Tools is not running.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    pub async fn reboot_guest(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/RebootGuest", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Reconfigures this virtual machine.
    /// 
    /// All the changes in the given configuration
    /// are applied to the virtual machine as an atomic operation.
    /// 
    /// Reconfiguring the virtual machine may require any of the following privileges
    /// depending on what is being changed:
    /// - VirtualMachine.Interact.DeviceConnection if changing the runtime connection
    ///   state of a device as embodied by the Connectable property.
    /// - VirtualMachine.Interact.SetCDMedia if changing the backing of a CD-ROM
    ///   device
    /// - VirtualMachine.Interact.SetFloppyMedia if changing the backing of a
    ///   floppy device
    /// - VirtualMachine.Config.Rename if renaming the virtual machine
    /// - VirtualMachine.Config.Annotation if setting annotation a value
    /// - VirtualMachine.Config.AddExistingDisk if adding a virtual disk device
    ///   that is backed by an existing virtual disk file
    /// - VirtualMachine.Config.AddNewDisk if adding a virtual disk device for which
    ///   the backing virtual disk file is to be created
    /// - VirtualMachine.Config.RemoveDisk if removing a virtual disk device that
    ///   refers to a virtual disk file
    /// - VirtualMachine.Config.CPUCount if changing the number of CPUs
    /// - VirtualMachine.Config.Memory if changing the amount of memory
    /// - VirtualMachine.Config.RawDevice if adding, removing or editing a raw
    ///   device mapping (RDM) or SCSI passthrough device
    /// - VirtualMachine.Config.AddRemoveDevice if adding or removing any
    ///   device other than disk, raw, or USB device
    /// - VirtualMachine.Config.EditDevice if changing the settings of any
    ///   device
    /// - VirtualMachine.Config.Settings if changing any basic settings such as
    ///   those in ToolsConfigInfo, FlagInfo, or DefaultPowerOpInfo
    /// - VirtualMachine.Config.Resource if changing resource allocations,
    ///   affinities, or setting network traffic shaping or virtual disk shares
    /// - VirtualMachine.Config.AdvancedConfig if changing values in
    ///   extraConfig
    /// - VirtualMachine.Config.SwapPlacement if changing swapPlacement
    /// - VirtualMachine.Config.HostUSBDevice if adding, removing or editing a
    ///   VirtualUSB device backed by the host USB device.
    /// - VirtualMachine.Config.DiskExtend if extending an existing VirtualDisk
    ///   device.
    /// - VirtualMachine.Config.ChangeTracking if enabling/disabling changed
    ///   block tracking for the virtual machine's disks.
    /// - VirtualMachine.Config.MksControl if toggling display connection
    ///   limits or the guest auto-lock feature.
    /// - DVSwitch.CanUse if connecting a VirtualEthernetAdapter to a port
    ///   in a DistributedVirtualSwitch.
    /// - DVPortgroup.CanUse if connecting a VirtualEthernetAdapter to a
    ///   DistributedVirtualPortgroup.
    /// - Cryptographer.Encrypt if vm home folder is encrypted or existing
    ///   disk is encryted.
    /// - Cryptographer.Decrypt if vm home folder is decrypted or existing
    ///   disk is decryted.
    /// - Cryptographer.Recrypt if vm home folder is recrypted or existing
    ///   disk is recryted.
    /// - Cryptographer.AddDisk if encrypted disk is attached to the vm.
    /// - Cryptographer.RegisterHost on the host if the virtual machine is
    ///   encrypted, but encryption is not enabled on the host.
    ///   
    /// Creating a virtual machine may require the following privileges:
    /// - VirtualMachine.Config.RawDevice if adding a raw device
    /// - VirtualMachine.Config.AddExistingDisk if adding a VirtualDisk and
    ///   the fileOperation is unset
    /// - VirtualMachine.Config.AddNewDisk if adding a VirtualDisk and
    ///   the fileOperation is set
    /// - VirtualMachine.Config.HostUSBDevice if adding a VirtualUSB device
    ///   backed by the host USB device.
    ///   
    /// In addition, this operation may require the following privileges:
    /// - Datastore.AllocateSpace on any datastore where virtual disks will
    ///   be created or extended.
    /// - Network.Assign on any network the virtual machine will be
    ///   connected to.
    ///   
    /// To create a VirtualDisk on a persistent memory storage, the storage
    /// must be specified via
    /// *profile* while the datastore
    /// property of corresponding VirtualDisk backing must be unset.
    /// 
    /// To create a VirtualNVDIMM device, the storage
    /// *profile* must be set to the
    /// default persistent memory storage profile while the datastore property of
    /// *the device backing* must be
    /// unset.
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The new configuration values.
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
    /// ***InvalidPowerState***: if the power state is poweredOn and the virtual hardware
    /// cannot support the configuration changes.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***TooManyDevices***: if the device specifications exceed the allowed limits.
    /// 
    /// ***ConcurrentAccess***: if the changeVersion does not match the server's
    /// changeVersion for the configuration.
    /// 
    /// ***FileFault***: if there is a problem creating or accessing the virtual machine's
    /// files for this operation. Typically a more specific fault like NoDiskSpace
    /// or FileAlreadyExists is thrown.
    /// 
    /// ***InvalidName***: if the specified name is invalid.
    /// 
    /// ***DuplicateName***: if the specified name already exists in the parent folder.
    /// 
    /// ***InvalidState***: if the operation cannot be performed in the current state
    /// of the virtual machine. For example, because the virtual machine's
    /// configuration is not available.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***VmConfigFault***: if the spec is invalid. Typically, a more specific subclass
    /// is thrown.
    /// 
    /// ***CpuHotPlugNotSupported***: if the current configuration of the VM does not
    /// support hot-plugging of CPUs.
    /// 
    /// ***MemoryHotPlugNotSupported***: if the current configuration of the VM does not
    /// support hot-plugging of memory.
    /// 
    /// ***VmWwnConflict***: if the WWN of the virtual machine has been used by
    /// other virtual machines.
    /// 
    /// ***NoPermission***: if crypto operation is requested on the vm home
    /// folder, but the user does not have the corresponding crypto
    /// privilege on the virtual machine:
    /// Encrypt - Cryptographer.Encrypt
    /// Decrypt - Cryptographer.Decrypt
    /// Recrypt - Cryptographer.Recrypt
    /// 
    /// ***NoPermission***: if crypto operation is requested on the vms disks,
    /// but the user does not have the corresponding crypto privilege
    /// on the virtual machine:
    /// Encrypt - Cryptographer.Encrypt
    /// Decrypt - Cryptographer.Decrypt
    /// Recrypt - Cryptographer.Recrypt
    /// AddDisk - Cryptographer.AddDisk
    /// 
    /// ***NoPermission***: if the virtual machine is encrypted and the
    /// encryption is not enabled on the host, but the user does not have
    /// Cryptographer.RegisterHost privilege on the host.
    pub async fn reconfig_vm_task(&self, spec: &VirtualMachineConfigSpec) -> Result<ManagedObjectReference> {
        let input = ReconfigVmRequestType {spec, };
        let path = format!("/VirtualMachine/{moId}/ReconfigVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Explicitly refreshes the storage information of this virtual machine,
    /// updating properties *VirtualMachine.storage*, *VirtualMachine.layoutEx*
    /// and *VirtualMachineSummary.storage*.
    /// 
    /// This is an asynchronous operation which will return immediately; changes
    /// may not be reflected in vCenter for some time.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn refresh_storage_info(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/RefreshStorageInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Reload the entity state.
    /// 
    /// Clients only need to call this method
    /// if they changed some external state that affects the service
    /// without using the Web service interface to perform the change.
    /// For example, hand-editing a virtual machine configuration file
    /// affects the configuration of the associated virtual machine but
    /// the service managing the virtual machine might not monitor the
    /// file for changes. In this case, after such an edit, a client
    /// would call "reload" on the associated virtual machine to ensure
    /// the service and its clients have current data for the
    /// virtual machine.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn reload(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/Reload", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Reloads the configuration for this virtual machine from a given
    /// datastore path.
    /// 
    /// This is equivalent to unregistering and registering the
    /// virtual machine from a different path. The virtual machine's hardware
    /// configuration, snapshots, guestinfo variables etc. will be
    /// replaced based on the new configuration file. Other information
    /// associated with the virtual machine object, such as events and
    /// permissions, will be preserved.
    /// 
    /// This method is only supported on vCenter Server. It can be invoked on
    /// inaccessible or orphaned virtual machines, but it cannot be invoked on
    /// powered on, connected virtual machines. Both the source virtual machine
    /// object and the destination path should be of the same type i.e. virtual
    /// machine or template. Reloading a virtual machine with a template or
    /// vice-versa is not supported.
    /// 
    /// _Note:_ Since the API replaces the source configuration with that
    /// of the destination, if the destination configuration does not refer to a
    /// valid virtual machine, it will create an invalid virtual machine object.
    /// This API should not be invoked on fault tolerant virtual machines since
    /// doing so will leave the original virtual machine's configuration in an
    /// invalid state. It is recommended that you turn off fault tolerance before
    /// invoking this API.
    /// 
    /// ***Required privileges:*** VirtualMachine.Config.ReloadFromPath
    ///
    /// ## Parameters:
    ///
    /// ### configuration_path
    /// -
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if invoked on ESX server or if invoked on a virtual
    /// machine with the destination path for a template and vice-versa.
    /// 
    /// ***InvalidPowerState***: if the virtual machine is powered on.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***FileFault***: if there is a problem creating or accessing the files
    /// needed for this operation.
    /// 
    /// ***InvalidState***: if the virtual machine is busy or not ready to
    /// respond to such requests.
    /// 
    /// ***VmConfigFault***: if the format / configuration of the virtual machine
    /// is invalid. Typically, a more specific fault is thrown such as
    /// InvalidFormat if the configuration file cannot be read, or
    /// InvalidDiskFormat if the disks cannot be read.
    /// 
    /// ***AlreadyExists***: if the virtual machine is already registered.
    pub async fn reload_virtual_machine_from_path_task(&self, configuration_path: &str) -> Result<ManagedObjectReference> {
        let input = ReloadVirtualMachineFromPathRequestType {configuration_path, };
        let path = format!("/VirtualMachine/{moId}/reloadVirtualMachineFromPath_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Relocates a virtual machine to the location specified by
    /// *VirtualMachineRelocateSpec*.
    /// 
    /// Starting from VCenter 5.1, this API also supports relocating a template
    /// to a new host should the current host become inactive.
    /// Starting from vCenter 6.0 this API also supports relocating a VM to a new
    /// vCenter service.
    /// 
    /// Requires the following additional permissions:
    /// - Resource.HotMigrate if the virtual machine is powered on.
    /// - Datastore.AllocateSpec if the virtual machine or its disks are
    ///   being relocated to a new datastore.
    /// - Resource.AssignVMToPool if the resource pool is changing.
    /// - VirtualMachine.Inventory.Register against the destination folder if
    ///   the virtual machine is moving to a new vCenter service.
    /// - VirtualMachine.Inventory.Move against the virtual machine, source
    ///   folder, and destination folder if the virtual machine is changing
    ///   folders within the same vCenter service.
    /// - Network.Assign against the new network if the virtual machine is
    ///   changing networks.
    ///   
    /// If this virtual machine is configured with a VirtualNVDIMM device, and if
    /// the virtual machine will be moved to a different host, the VirtualNVDIMM
    /// will be automatically relocated to the destination host's Non-Volatile
    /// Memory storage.
    /// If this Virtual machine is configured with virtual disks via
    /// persistent memory storage profile:
    /// - If spec specifies only compute location change, these virtual disks
    ///   will be automatically moved to a persistent memory storage in
    ///   destination host that supports the profile.
    /// - If spec specifies primary datastore change via
    ///   *datastore*, unlike regular
    ///   virtual disks, these disks will not be automatically moved to the
    ///   specified datastore, instead they will stay on a persistent
    ///   memory storage in destination host that supports the profile.
    /// - To explicityly move these disks to a location other than
    ///   persistent memory storage, use disk locator to specify the
    ///   new destination datastore along with a storage profile that removes
    ///   the persistent memory storage requirement. Note that this
    ///   downgrades the disk I/O performance.
    /// - On the other hand, to move a virtual disk from a regular storage to
    ///   persistent memory, use
    ///   *deviceChange*
    ///   to specify a storage profile of persistent memory storage. Note
    ///   that this upgrades the disk I/O performance.
    ///   
    /// ***Required privileges:*** Resource.ColdMigrate
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The specification of where to relocate the virtual machine
    /// (see *VirtualMachineRelocateSpec*).
    ///
    /// ### priority
    /// The task priority
    /// (see *VirtualMachineMovePriority_enum*).
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
    /// ***InvalidArgument***: in the following cases:
    /// - the target host and target pool are not associated with the
    ///   same compute resource
    /// - the target pool represents a cluster without DRS enabled,
    ///   and the host is not specified
    /// - the virtual machine is powered on, its home or any of its disks
    ///   will change storage location, and the host is not specified
    /// - Datastore is not accessible in a cross-datacenter move
    /// - Datastore in a diskLocator entry is not specified
    /// - the specified device ID cannot be found in the virtual machine's current
    ///   configuration
    ///   
    /// ***NotSupported***: if the virtual machine is marked as template and
    /// the datastore is changing or if it is a cross vCenter vMotion operation.
    /// 
    /// ***Timedout***: if one of the phases of the relocate process times out.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// host or virtual machine's current state. For example, if the host is in
    /// maintenance mode, or if the virtual machine's configuration information
    /// is not available.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// target datastores.
    /// 
    /// ***FileFault***: if there is an error accessing the virtual machine files.
    /// 
    /// ***VmConfigFault***: if the virtual machine is not compatible with the
    /// destination host. Typically, a specific subclass of this exception is
    /// thrown, such as IDEDiskNotSupported.
    /// 
    /// ***MigrationFault***: if it is not possible to migrate the virtual machine to
    /// the destination host. This is typically due to hosts being incompatible,
    /// such as mismatch in network polices or access to networks and datastores.
    /// Typically, a more specific subclass is thrown.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***DisallowedOperationOnFailoverHost***: if the virtual machine is powered on
    /// and is being migrated to a failover host. See
    /// *ClusterFailoverHostAdmissionControlPolicy*.
    pub async fn relocate_vm_task(&self, spec: &VirtualMachineRelocateSpec, priority: Option<VirtualMachineMovePriorityEnum>) -> Result<ManagedObjectReference> {
        let input = RelocateVmRequestType {spec, priority, };
        let path = format!("/VirtualMachine/{moId}/RelocateVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Remove all the snapshots associated with this virtual machine.
    /// 
    /// If the virtual
    /// machine
    /// does not have any snapshots, then this operation simply returns successfully.
    /// 
    /// ***Required privileges:*** VirtualMachine.State.RemoveSnapshot
    ///
    /// ## Parameters:
    ///
    /// ### consolidate
    /// (optional) If set to true, the virtual disks of the deleted
    /// snapshot will be merged with other disk if possible. Default to true.
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
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the host product does not support snapshots.
    /// 
    /// ***InvalidPowerState***: if the operation cannot be performed in the current
    /// power state of the virtual machine.
    /// 
    /// ***SnapshotFault***: if an error occurs during the snapshot operation.
    /// Typically, a more specific fault like InvalidSnapshotFormat
    /// is thrown.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    pub async fn remove_all_snapshots_task(&self, consolidate: Option<bool>) -> Result<ManagedObjectReference> {
        let input = RemoveAllSnapshotsRequestType {consolidate, };
        let path = format!("/VirtualMachine/{moId}/RemoveAllSnapshots_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Renames this managed entity.
    /// 
    /// Any % (percent) character used in this name parameter
    /// must be escaped, unless it is used to start an escape
    /// sequence. Clients may also escape any other characters in
    /// this name parameter.
    /// 
    /// See also *ManagedEntity.name*.
    /// 
    /// ***Required privileges:*** VirtualMachine.Config.Rename
    ///
    /// ## Parameters:
    ///
    /// ### new_name
    /// -
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
    /// ***DuplicateName***: If another object in the same folder has the target name.
    /// 
    /// ***InvalidName***: If the new name is not a valid entity name.
    pub async fn rename_task(&self, new_name: &str) -> Result<ManagedObjectReference> {
        let input = RenameRequestType {new_name, };
        let path = format!("/VirtualMachine/{moId}/Rename_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Resets power on this virtual machine.
    /// 
    /// If the current state is poweredOn,
    /// then this method first performs powerOff(hard). Once the power state
    /// is poweredOff, then this method performs powerOn(option).
    /// 
    /// Although this method functions as a powerOff followed by a powerOn, the
    /// two operations are atomic with respect to other clients, meaning that
    /// other power operations cannot be performed until the reset method completes.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.Reset
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
    /// ***InvalidPowerState***: if the power state is suspended or poweredOff.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotEnoughLicenses***: if there are not enough licenses to reset
    /// this virtual machine.
    /// 
    /// ***NotSupported***: if the virtual machine is marked as a template.
    /// 
    /// ***InvalidState***: if the host is in maintenance mode.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    pub async fn reset_vm_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/ResetVM_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Clears cached guest information.
    /// 
    /// Guest information can be cleared
    /// only if the virtual machine is powered off.
    /// 
    /// This method can be useful if stale information is cached,
    /// preventing an IP address or MAC address from being reused.
    /// 
    /// ***Required privileges:*** VirtualMachine.Config.ResetGuestInfo
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the virtual machine is not powered off.
    /// 
    /// ***NotSupported***: if the virtual machine is marked as a template.
    pub async fn reset_guest_information(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/ResetGuestInformation", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Reverts the virtual machine to the current snapshot.
    /// 
    /// This is equivalent to
    /// doing snapshot.currentSnapshot.revert.
    /// 
    /// If no snapshot exists, then the operation does nothing,
    /// and the virtual machine state remains unchanged.
    /// 
    /// ***Required privileges:*** VirtualMachine.State.RevertToSnapshot
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// (optional) Choice of host for the virtual machine,
    /// in case this operation causes the virtual machine to power on.
    /// 
    /// If a snapshot was taken while a virtual machine was powered on,
    /// and this operation is invoked after the virtual machine was
    /// powered off, the operation causes the virtual machine to power
    /// on to reach the snapshot state. This parameter can be used to
    /// specify a choice of host where the virtual machine should power
    /// on.
    /// 
    /// If this parameter is not set, and the vBalance feature is
    /// configured for automatic load balancing, a host is
    /// automatically selected. Otherwise, the virtual machine keeps
    /// its existing host affiliation.
    /// 
    /// This is not supported for virtual machines associated with hosts on ESX 2.x
    /// servers.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### suppress_power_on
    /// (optional) If set to true, the virtual
    /// machine will not be powered on regardless of the power state when
    /// the current snapshot was created. Default to false.
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
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the host product does not support snapshots.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***SnapshotFault***: if an error occurs during the snapshot operation.
    /// Typically, a more specific fault like InvalidSnapshotFormat
    /// is thrown.
    /// 
    /// ***InvalidPowerState***: if the operation cannot be performed in the current
    /// power state of the virtual machine.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available or if an OVF consumer is
    /// blocking the operation.
    /// 
    /// ***VmConfigFault***: if a configuration issue prevents the power-on. Typically, a
    /// more specific fault, such as UnsupportedVmxLocation, is thrown.
    /// 
    /// ***FileFault***: if there is a problem accessing the virtual machine on the
    /// filesystem.
    /// 
    /// ***NotFound***: if the virtual machine does not have a current snapshot.
    /// 
    /// ***DisallowedOperationOnFailoverHost***: if the virtual machine is being
    /// reverted to a powered on state and the host specified is a failover host.
    /// See *ClusterFailoverHostAdmissionControlPolicy*.
    pub async fn revert_to_current_snapshot_task(&self, host: Option<&ManagedObjectReference>, suppress_power_on: Option<bool>) -> Result<ManagedObjectReference> {
        let input = RevertToCurrentSnapshotRequestType {host, suppress_power_on, };
        let path = format!("/VirtualMachine/{moId}/RevertToCurrentSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Send a non-maskable interrupt (NMI).
    /// 
    /// Currently, there is no way to verify if the NMI was actually
    /// received by the guest OS.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.GuestControl
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the virtual machine is not powered on.
    pub async fn send_nmi(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/SendNMI", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
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
        let path = format!("/VirtualMachine/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Sets the console window's display topology as specified.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.ConsoleInteract
    ///
    /// ## Parameters:
    ///
    /// ### displays
    /// The topology for each monitor that the
    /// virtual machine's display must span.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the Guest Operating system does
    /// not support setting the display topology
    /// 
    /// ***InvalidPowerState***: if the power state is not poweredOn.
    /// 
    /// ***InvalidState***: if the virtual machine is not connected.
    /// 
    /// ***ToolsUnavailable***: if VMware Tools is not running.
    pub async fn set_display_topology(&self, displays: &[VirtualMachineDisplayTopology]) -> Result<()> {
        let input = SetDisplayTopologyRequestType {displays, };
        let path = format!("/VirtualMachine/{moId}/SetDisplayTopology", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Sets the console window's resolution as specified.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.ConsoleInteract
    ///
    /// ## Parameters:
    ///
    /// ### width
    /// The screen width that should be set.
    ///
    /// ### height
    /// The screen height that should be set.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the Guest Operating system does
    /// not support setting the screen resolution.
    /// 
    /// ***InvalidPowerState***: if the power state is not poweredOn.
    /// 
    /// ***InvalidState***: if the virtual machine is not connected.
    /// 
    /// ***ToolsUnavailable***: if VMware Tools is not running.
    pub async fn set_screen_resolution(&self, width: i32, height: i32) -> Result<()> {
        let input = SetScreenResolutionRequestType {width, height, };
        let path = format!("/VirtualMachine/{moId}/SetScreenResolution", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Issues a command to the guest operating system asking it to perform
    /// a clean shutdown of all services.
    /// 
    /// Returns immediately and does not wait for the guest operating system
    /// to complete the operation.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.PowerOff
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the power state is not powered on.
    /// 
    /// ***ToolsUnavailable***: if VMware Tools is not running.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    pub async fn shutdown_guest(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/ShutdownGuest", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Issues a command to the guest operating system asking it to prepare for
    /// a suspend operation.
    /// 
    /// Returns immediately and does not wait for the guest operating system
    /// to complete the operation.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.Suspend
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the power state is not powered on.
    /// 
    /// ***ToolsUnavailable***: if VMware Tools is not running.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    pub async fn standby_guest(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/StandbyGuest", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Deprecated as of vsphere API 5.1.
    /// 
    /// Initiates a recording session on this virtual machine.
    /// 
    /// As a side effect,
    /// this operation creates a snapshot on the virtual machine, which in turn
    /// becomes the current snapshot.
    /// 
    /// This is an experimental interface that is not intended for use in production code.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.Record
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name for the snapshot associated with this recording.
    /// The name need not be unique for this virtual machine.
    ///
    /// ### description
    /// A description for the snapshot associated with this
    /// recording. If omitted, a default description may be provided.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. The *info.result* property
    /// in the *Task* contains the newly created *VirtualMachineSnapshot*
    /// associated with the recording on success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the host product does not support record
    /// functionality or if the virtual machine does not support this
    /// 
    /// ***VmConfigIncompatibleForRecordReplay***: if the virtual machine
    /// configuration is incompatible for recording.
    /// 
    /// ***SnapshotFault***: if an error occurs during the snapshot operation.
    /// Typically, a more specific fault like MultipleSnapshotsNotSupported
    /// is thrown.
    /// 
    /// ***InvalidName***: if the specified snapshot name is invalid.
    /// 
    /// ***FileFault***: if there is a problem with creating or accessing one
    /// or more files needed for this operation.
    /// 
    /// ***InvalidPowerState***: if the operation cannot be performed in the
    /// current power state of the virtual machine.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, the virtual machine
    /// configuration information is not available.
    /// 
    /// ***RecordReplayDisabled***: if the record/replay config flag has not been
    /// enabled for this virtual machine.
    /// 
    /// ***HostIncompatibleForRecordReplay***: if the virtual machine is located
    /// on a host that does not support record/replay.
    pub async fn start_recording_task(&self, name: &str, description: Option<&str>) -> Result<ManagedObjectReference> {
        let input = StartRecordingRequestType {name, description, };
        let path = format!("/VirtualMachine/{moId}/StartRecording_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vsphere API 5.1.
    /// 
    /// Starts a replay session on this virtual machine.
    /// 
    /// As a side effect,
    /// this operation updates the current snapshot of the virtual machine.
    /// 
    /// This is an experimental interface that is not intended for use in production code.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.Replay
    ///
    /// ## Parameters:
    ///
    /// ### replay_snapshot
    /// The snapshot from which to start the replay. This
    /// snapshot must have been created by a record operation on the
    /// virtual machine.
    /// 
    /// Refers instance of *VirtualMachineSnapshot*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the host product does not support record/replay
    /// functionality or if the virtual machine does not support this
    /// capability.
    /// 
    /// ***InvalidArgument***: if replaySnapshot is not a valid snapshot
    /// associated with a recorded session on this virtual machine.
    /// 
    /// ***SnapshotFault***: if an error occurs during the snapshot operation.
    /// Typically, a more specific fault like InvalidSnapshotFormat
    /// is thrown.
    /// 
    /// ***FileFault***: if there is a problem with creating or accessing one
    /// or more files needed for this operation.
    /// 
    /// ***VmConfigIncompatibleForRecordReplay***: if the virtual machine
    /// configuration is incompatible for replaying.
    /// 
    /// ***InvalidPowerState***: if the operation cannot be performed in the
    /// current power state of the virtual machine.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, the virtual machine
    /// configuration information is not available.
    /// 
    /// ***NotFound***: if replaySnapshot is no longer present.
    /// 
    /// ***RecordReplayDisabled***: if the record/replay config flag has not been
    /// enabled for this virtual machine.
    /// 
    /// ***HostIncompatibleForRecordReplay***: if the virtual machine is located
    /// on a host that does not support record/replay.
    pub async fn start_replaying_task(&self, replay_snapshot: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = StartReplayingRequestType {replay_snapshot, };
        let path = format!("/VirtualMachine/{moId}/StartReplaying_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vsphere API 5.1.
    /// 
    /// Stops a currently active recording session on this virtual machine.
    /// 
    /// This is an experimental interface that is not intended for use in production code.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.Record
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the host product does not support record/replay
    /// functionality or if the virtual machine does not support this
    /// capability.
    /// 
    /// ***SnapshotFault***: if an error occurs during the snapshot operation.
    /// Typically, a more specific fault like InvalidSnapshotFormat
    /// is thrown.
    /// 
    /// ***FileFault***: if there is a problem with creating or accessing one
    /// or more files needed for this operation.
    /// 
    /// ***InvalidPowerState***: if the operation cannot be performed in the current
    /// power state of the virtual machine.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, the virtual machine
    /// does not have an active recording session.
    pub async fn stop_recording_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/StopRecording_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vsphere API 5.1.
    /// 
    /// Stops a replay session on this virtual machine.
    /// 
    /// This is an experimental interface that is not intended for use in production code.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.Replay
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the host product does not support record/replay
    /// functionality or if the virtual machine does not support this
    /// capability.
    /// 
    /// ***SnapshotFault***: if an error occurs during the snapshot operation.
    /// Typically, a more specific fault like InvalidSnapshotFormat
    /// is thrown.
    /// 
    /// ***FileFault***: if there is a problem with creating or accessing one
    /// or more files needed for this operation.
    /// 
    /// ***InvalidPowerState***: if the operation cannot be performed in the
    /// current power state of the virtual machine.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, the virtual machine
    /// does not have an active recording session.
    pub async fn stop_replaying_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/StopReplaying_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Suspends execution in this virtual machine.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.Suspend
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
    /// ***InvalidPowerState***: if the power state is not poweredOn.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the virtual machine is marked as a template.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    pub async fn suspend_vm_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/SuspendVM_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Do an immediate power off of a VM.
    /// 
    /// This API issues a SIGKILL to the vmx process of the VM.
    /// Pending synchronous I/Os may not be written out before the vmx
    /// process dies depending on accessibility of the datastore.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.PowerOff
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if this operation is not supported.
    /// 
    /// ***InvalidState***: if the VM is not powered on or another issue prevents the
    /// operation from being performed.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    pub async fn terminate_vm(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/TerminateVM", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Terminates the specified secondary virtual machine in a fault tolerant group.
    /// 
    /// This
    /// can be used to test fault tolerance on a given virtual machine, and should
    /// be used with care.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.TerminateFaultTolerantVM
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The secondary virtual machine specified will be terminated, allowing
    /// fault tolerance to activate. If no virtual machine is specified,
    /// all secondary virtual machines will be terminated. If vm is a
    /// primary, InvalidArgument exception is thrown.
    /// This field must specify a virtual machine that is part of the fault
    /// tolerant group that this virtual machine is currently associated with. It can
    /// only be invoked from the primary virtual machine in the group. If the primary
    /// virtual machine is terminated, an available secondary virtual machine will be
    /// promoted to primary. If no secondary exists, an exception will be thrown and
    /// the primary virtual machine will not be terminated. If a secondary virtual
    /// machine is terminated, it may be respawned on a potentially different host.
    /// 
    /// Refers instance of *VirtualMachine*.
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
    /// ***VmFaultToleranceIssue***: if any error is encountered with the
    /// fault tolerance configuration of the virtual machine. Typically,
    /// a more specific fault like InvalidOperationOnSecondaryVm is thrown.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidState***: if the host is in maintenance mode or if
    /// the virtual machine's configuration information is not available.
    pub async fn terminate_fault_tolerant_vm_task(&self, vm: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = TerminateFaultTolerantVmRequestType {vm, };
        let path = format!("/VirtualMachine/{moId}/TerminateFaultTolerantVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Removes all secondary virtual machines associated with the fault tolerant
    /// group and turns off protection for this virtual machine.
    /// 
    /// This operation can only be invoked from the primary virtual machine in
    /// the group.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.TurnOffFaultTolerance
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
    /// ***VmFaultToleranceIssue***: if any error is encountered with the
    /// fault tolerance configuration of the virtual machine. Typically,
    /// a more specific fault like InvalidOperationOnSecondaryVm is thrown.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidState***: if the host is in maintenance mode or if
    /// the virtual machine's configuration information is not available.
    pub async fn turn_off_fault_tolerance_for_vm_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/TurnOffFaultToleranceForVM_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Unmounts VMware Tools installer CD.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.ToolsInstall
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the virtual machine is not running,
    /// VMware Tools is not running or the VMware Tools CD is already mounted.
    pub async fn unmount_tools_installer(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/UnmountToolsInstaller", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Removes this virtual machine from the inventory without removing
    /// any of the virtual machine's files on disk.
    /// 
    /// All high-level information
    /// stored with the management server (ESX Server or VirtualCenter) is
    /// removed, including information such as statistics, resource pool association,
    /// permissions, and alarms.
    /// 
    /// Use the Folder.RegisterVM method to recreate a
    /// VirtualMachine object from the set of virtual machine files by passing in
    /// the path to the configuration file. However, the VirtualMachine managed object
    /// that results typically has different objects ID and may inherit a different
    /// set of permissions.
    /// 
    /// ***Required privileges:*** VirtualMachine.Inventory.Unregister
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidPowerState***: if the virtual machine is powered on.
    pub async fn unregister_vm(&self) -> Result<()> {
        let path = format!("/VirtualMachine/{moId}/UnregisterVM", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Begins the tools upgrade process.
    /// 
    /// To monitor the status of the tools install, clients should check the tools status,
    /// *GuestInfo.toolsVersionStatus* and
    /// *GuestInfo.toolsRunningStatus*.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.ToolsInstall
    ///
    /// ## Parameters:
    ///
    /// ### installer_options
    /// Command line options passed to the installer to modify
    /// the installation procedure for tools.
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
    /// ***InvalidState***: if the virtual machine is not running
    /// or is suspended.
    /// 
    /// ***NotSupported***: if upgrading tools is not supported.
    /// 
    /// ***TaskInProgress***: if an upgrade is already taking place.
    /// 
    /// ***VmToolsUpgradeFault***: if the upgrade failed.
    /// 
    /// ***ToolsUnavailable***: if VMware Tools is not running.
    pub async fn upgrade_tools_task(&self, installer_options: Option<&str>) -> Result<ManagedObjectReference> {
        let input = UpgradeToolsRequestType {installer_options, };
        let path = format!("/VirtualMachine/{moId}/UpgradeTools_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Upgrades this virtual machine's virtual hardware to the latest revision
    /// that is supported by the virtual machine's current host.
    /// 
    /// ***Required privileges:*** VirtualMachine.Config.UpgradeVirtualHardware
    ///
    /// ## Parameters:
    ///
    /// ### version
    /// If specified, upgrade to that specified version. If not specified,
    /// upgrade to the most current virtual hardware supported on the host.
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
    /// ***InvalidPowerState***: if the power state is not poweredOff.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***AlreadyUpgraded***: if the virtual machine's hardware is already up-to-date.
    /// 
    /// ***NoDiskFound***: if no virtual disks are attached to this virtual machine.
    /// 
    /// ***InvalidState***: if the host is in maintenance mode,
    /// if an invalid version string is specified, or
    /// if the virtual machine is in a state in which the operation
    /// cannot be performed. For example, if the configuration
    /// information is not available.
    pub async fn upgrade_vm_task(&self, version: Option<&str>) -> Result<ManagedObjectReference> {
        let input = UpgradeVmRequestType {version, };
        let path = format!("/VirtualMachine/{moId}/UpgradeVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let path = format!("/VirtualMachine/{moId}/alarmActionsEnabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/VirtualMachine/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Information about the runtime capabilities of this virtual machine.
    pub async fn capability(&self) -> Result<VirtualMachineCapability> {
        let path = format!("/VirtualMachine/{moId}/capability", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Configuration of this virtual machine, including the name and UUID.
    /// 
    /// This property is set when a virtual machine is created or when
    /// the *reconfigVM* method is called.
    /// 
    /// The virtual machine configuration is not guaranteed to be available.
    /// For example, the configuration information would be unavailable
    /// if the server is unable to access the virtual machine files on disk,
    /// and is often also unavailable during the initial phases of
    /// virtual machine creation.
    pub async fn config(&self) -> Result<VirtualMachineConfigInfo> {
        let path = format!("/VirtualMachine/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Current configuration issues that have been detected for this entity.
    /// 
    /// Typically,
    /// these issues have already been logged as events. The entity stores these
    /// events as long as they are still current. The
    /// *configStatus* property provides an overall status
    /// based on these events.
    pub async fn config_issue(&self) -> Result<Option<Vec<Box<dyn EventTrait>>>> {
        let path = format!("/VirtualMachine/{moId}/configIssue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The configStatus indicates whether or not the system has detected a configuration
    /// issue involving this entity.
    /// 
    /// For example, it might have detected a
    /// duplicate IP address or MAC address, or a host in a cluster
    /// might be out of compliance. The meanings of the configStatus values are:
    /// - red: A problem has been detected involving the entity.
    /// - yellow: A problem is about to occur or a transient condition
    ///   has occurred (For example, reconfigure fail-over policy).
    /// - green: No configuration issues have been detected.
    /// - gray: The configuration status of the entity is not being monitored.
    ///   
    /// A green status indicates only that a problem has not been detected;
    /// it is not a guarantee that the entity is problem-free.
    /// 
    /// The *configIssue* property contains a list of the
    /// problems that have been detected.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    pub async fn config_status(&self) -> Result<ManagedEntityStatusEnum> {
        let path = format!("/VirtualMachine/{moId}/configStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn CustomFieldValueTrait>>>> {
        let path = format!("/VirtualMachine/{moId}/customValue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A collection of references to the subset of datastore objects in the datacenter
    /// that is used by this virtual machine.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Datastore*.
    pub async fn datastore(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/VirtualMachine/{moId}/datastore", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A set of alarm states for alarms that apply to this managed entity.
    /// 
    /// The set includes alarms defined on this entity
    /// and alarms inherited from the parent entity,
    /// or from any ancestors in the inventory hierarchy.
    /// 
    /// Alarms are inherited if they can be triggered by this entity or its descendants.
    /// This set does not include alarms that are defined on descendants of this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn declared_alarm_state(&self) -> Result<Option<Vec<AlarmState>>> {
        let path = format!("/VirtualMachine/{moId}/declaredAlarmState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of operations that are disabled, given the current runtime
    /// state of the entity.
    /// 
    /// For example, a power-on operation always fails if a
    /// virtual machine is already powered on. This list can be used by clients to
    /// enable or disable operations in a graphical user interface.
    /// 
    /// Note: This list is determined by the current runtime state of an entity,
    /// not by its permissions.
    /// 
    /// This list may include the following operations for a HostSystem:
    /// - *HostSystem.EnterMaintenanceMode_Task*
    /// - *HostSystem.ExitMaintenanceMode_Task*
    /// - *HostSystem.RebootHost_Task*
    /// - *HostSystem.ShutdownHost_Task*
    /// - *HostSystem.ReconnectHost_Task*
    /// - *HostSystem.DisconnectHost_Task*
    ///   
    /// This list may include the following operations for a VirtualMachine:
    /// - *VirtualMachine.AnswerVM*
    /// - *ManagedEntity.Rename_Task*
    /// - *VirtualMachine.CloneVM_Task*
    /// - *VirtualMachine.PowerOffVM_Task*
    /// - *VirtualMachine.PowerOnVM_Task*
    /// - *VirtualMachine.SuspendVM_Task*
    /// - *VirtualMachine.ResetVM_Task*
    /// - *VirtualMachine.ReconfigVM_Task*
    /// - *VirtualMachine.RelocateVM_Task*
    /// - *VirtualMachine.MigrateVM_Task*
    /// - *VirtualMachine.CustomizeVM_Task*
    /// - *VirtualMachine.ShutdownGuest*
    /// - *VirtualMachine.StandbyGuest*
    /// - *VirtualMachine.RebootGuest*
    /// - *VirtualMachine.CreateSnapshot_Task*
    /// - *VirtualMachine.RemoveAllSnapshots_Task*
    /// - *VirtualMachine.RevertToCurrentSnapshot_Task*
    /// - *VirtualMachine.MarkAsTemplate*
    /// - *VirtualMachine.MarkAsVirtualMachine*
    /// - *VirtualMachine.ResetGuestInformation*
    /// - *VirtualMachine.MountToolsInstaller*
    /// - *VirtualMachine.UnmountToolsInstaller*
    /// - *ManagedEntity.Destroy_Task*
    /// - *VirtualMachine.UpgradeVM_Task*
    /// - *VirtualMachine.ExportVm*
    ///   
    /// This list may include the following operations for a ResourcePool:
    /// - *ResourcePool.ImportVApp*
    /// - *ResourcePool.CreateChildVM_Task*
    /// - *ResourcePool.UpdateConfig*
    /// - *Folder.CreateVM_Task*
    /// - *ManagedEntity.Destroy_Task*
    /// - *ManagedEntity.Rename_Task*
    ///   
    /// This list may include the following operations for a VirtualApp:
    /// - *ManagedEntity.Destroy_Task*
    /// - *VirtualApp.CloneVApp_Task*
    /// - *VirtualApp.unregisterVApp_Task*
    /// - *VirtualApp.ExportVApp*
    /// - *VirtualApp.PowerOnVApp_Task*
    /// - *VirtualApp.PowerOffVApp_Task*
    /// - *VirtualApp.UpdateVAppConfig*
    ///   
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    pub async fn disabled_method(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/VirtualMachine/{moId}/disabledMethod", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let path = format!("/VirtualMachine/{moId}/effectiveRole", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The current virtual machine's environment browser object.
    /// 
    /// This contains
    /// information on all the configurations that can be used on the
    /// virtual machine. This is identical to the environment browser on
    /// the *ComputeResource* to which this virtual machine belongs.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *EnvironmentBrowser*.
    pub async fn environment_browser(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/environmentBrowser", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Information about VMware Tools and about the virtual machine
    /// from the perspective of VMware Tools.
    /// 
    /// Information about the guest operating system is available in VirtualCenter. Guest
    /// operating system information reflects the last known state of the virtual machine.
    /// For powered on machines, this is current information. For powered off machines,
    /// this is the last recorded state before the virtual machine was powered off.
    pub async fn guest(&self) -> Result<GuestInfo> {
        let path = format!("/VirtualMachine/{moId}/guest", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The guest heartbeat.
    /// 
    /// The heartbeat status is classified as:
    /// - gray - VMware Tools are not installed or not running.
    /// - red - No heartbeat. Guest operating system may have stopped responding.
    /// - yellow - Intermittent heartbeat. May be due to guest load.
    /// - green - Guest operating system is responding normally.
    ///   
    /// The guest heartbeat is a statistics metric. Alarms can be configured on
    /// this metric to trigger emails or other actions.
    pub async fn guest_heartbeat_status(&self) -> Result<ManagedEntityStatusEnum> {
        let path = format!("/VirtualMachine/{moId}/guestHeartbeatStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 4.0, use *VirtualMachine.layoutEx* instead.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    /// 
    /// Detailed information about the files that comprise this virtual machine.
    pub async fn layout(&self) -> Result<VirtualMachineFileLayout> {
        let path = format!("/VirtualMachine/{moId}/layout", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Detailed information about the files that comprise this virtual machine.
    /// 
    /// Can be explicitly refreshed by the *VirtualMachine.RefreshStorageInfo* operation.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    pub async fn layout_ex(&self) -> Result<VirtualMachineFileLayoutEx> {
        let path = format!("/VirtualMachine/{moId}/layoutEx", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Name of this entity, unique relative to its parent.
    /// 
    /// Any / (slash), \\ (backslash), character used in this
    /// name element will be escaped. Similarly, any % (percent) character used in
    /// this name element will be escaped, unless it is used to start an escape
    /// sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
    /// %5c, and a percent is escaped as %25.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn name(&self) -> Result<String> {
        let path = format!("/VirtualMachine/{moId}/name", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// A collection of references to the subset of network objects in the datacenter that
    /// is used by this virtual machine.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Network*.
    pub async fn network(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/VirtualMachine/{moId}/network", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// General health of this managed entity.
    /// 
    /// The overall status of the managed entity is computed as the worst status
    /// among its alarms and the configuration issues detected on the entity.
    /// The status is reported as one of the following values:
    /// - red: The entity has alarms or configuration issues with a red status.
    /// - yellow: The entity does not have alarms or configuration issues with a
    ///   red status, and has at least one with a yellow status.
    /// - green: The entity does not have alarms or configuration issues with a
    ///   red or yellow status, and has at least one with a green status.
    /// - gray: All of the entity's alarms have a gray status and the
    ///   configuration status of the entity is not being monitored.
    ///   
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    pub async fn overall_status(&self) -> Result<ManagedEntityStatusEnum> {
        let path = format!("/VirtualMachine/{moId}/overallStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Parent of this entity.
    /// 
    /// This value is null for the root object and for
    /// *VirtualMachine* objects that are part of
    /// a *VirtualApp*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ManagedEntity*.
    pub async fn parent(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/parent", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Reference to the parent vApp.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ManagedEntity*.
    pub async fn parent_v_app(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/parentVApp", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// List of permissions defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<Permission>>> {
        let path = format!("/VirtualMachine/{moId}/permission", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The set of recent tasks operating on this managed entity.
    /// 
    /// This is a subset
    /// of *TaskManager.recentTask* belong to this entity. A task in this
    /// list could be in one of the four states: pending, running, success or error.
    /// 
    /// This property can be used to deduce intermediate power states for
    /// a virtual machine entity. For example, if the current powerState is "poweredOn"
    /// and there is a running task performing the "suspend" operation, then the virtual
    /// machine's intermediate state might be described as "suspending."
    /// 
    /// Most tasks (such as power operations) obtain exclusive access to the virtual
    /// machine, so it is unusual for this list to contain more than one running task.
    /// One exception, however, is the task of cloning a virtual machine.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Task*.
    pub async fn recent_task(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/VirtualMachine/{moId}/recentTask", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The resource configuration for a virtual machine.
    /// 
    /// The shares
    /// in this specification are evaluated relative to the resource pool
    /// to which it is assigned. This will return null if the product
    /// the virtual machine is registered on does not support resource
    /// configuration.
    /// 
    /// To retrieve the configuration, you typically use
    /// *childConfiguration*.
    /// 
    /// To change the configuration, use
    /// *ResourcePool.UpdateChildResourceConfiguration*.
    pub async fn resource_config(&self) -> Result<ResourceConfigSpec> {
        let path = format!("/VirtualMachine/{moId}/resourceConfig", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The current resource pool that specifies resource allocation
    /// for this virtual machine.
    /// 
    /// This property is set when a virtual machine is created or associated with
    /// a different resource pool.
    /// 
    /// Returns null if the virtual machine is a template or the session has no access
    /// to the resource pool.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ResourcePool*.
    pub async fn resource_pool(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachine/{moId}/resourcePool", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The roots of all snapshot trees for the virtual machine.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *VirtualMachineSnapshot*.
    pub async fn root_snapshot(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/VirtualMachine/{moId}/rootSnapshot", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Execution state and history for this virtual machine.
    /// 
    /// The contents of this property change when:
    /// - the virtual machine's power state changes.
    /// - an execution message is pending.
    /// - an event occurs.
    pub async fn runtime(&self) -> Result<VirtualMachineRuntimeInfo> {
        let path = format!("/VirtualMachine/{moId}/runtime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Current snapshot and tree.
    /// 
    /// The property is valid if snapshots have been created
    /// for this virtual machine.
    /// 
    /// The contents of this property change in response to the methods:
    /// - *createSnapshot*
    /// - *revertToCurrentSnapshot*
    /// - *remove*
    /// - *revert*
    /// - *removeAllSnapshots*
    pub async fn snapshot(&self) -> Result<VirtualMachineSnapshotInfo> {
        let path = format!("/VirtualMachine/{moId}/snapshot", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Storage space used by the virtual machine, split by datastore.
    /// 
    /// Can be explicitly refreshed by the *VirtualMachine.RefreshStorageInfo* operation.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    pub async fn storage(&self) -> Result<VirtualMachineStorageInfo> {
        let path = format!("/VirtualMachine/{moId}/storage", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Basic information about this virtual machine.
    /// 
    /// This includes:
    /// - runtimeInfo
    /// - guest
    /// - basic configuration
    /// - alarms
    /// - performance information
    pub async fn summary(&self) -> Result<VirtualMachineSummary> {
        let path = format!("/VirtualMachine/{moId}/summary", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<Tag>>> {
        let path = format!("/VirtualMachine/{moId}/tag", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A set of alarm states for alarms triggered by this entity
    /// or by its descendants.
    /// 
    /// Triggered alarms are propagated up the inventory hierarchy
    /// so that a user can readily tell when a descendant has triggered an alarm.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn triggered_alarm_state(&self) -> Result<Option<Vec<AlarmState>>> {
        let path = format!("/VirtualMachine/{moId}/triggeredAlarmState", moId = &self.mo_id);
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
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn CustomFieldValueTrait>>>> {
        let path = format!("/VirtualMachine/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AcquireTicketRequestType<'a> {
    #[serde(rename = "ticketType")]
    ticket_type: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "AnswerVMRequestType", tag = "_typeName")]
struct AnswerVmRequestType<'a> {
    #[serde(rename = "questionId")]
    question_id: &'a str,
    #[serde(rename = "answerChoice")]
    answer_choice: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "ApplyEvcModeVMRequestType", tag = "_typeName")]
struct ApplyEvcModeVmRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mask: Option<&'a [HostFeatureMask]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completeMasks")]
    complete_masks: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AttachDiskRequestType<'a> {
    #[serde(rename = "diskId")]
    disk_id: &'a Id,
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "controllerKey")]
    controller_key: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unitNumber")]
    unit_number: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckCustomizationSpecRequestType<'a> {
    spec: &'a CustomizationSpec,
}
#[derive(serde::Serialize)]
#[serde(rename = "CloneVMRequestType", tag = "_typeName")]
struct CloneVmRequestType<'a> {
    folder: &'a ManagedObjectReference,
    name: &'a str,
    spec: &'a VirtualMachineCloneSpec,
}
#[derive(serde::Serialize)]
#[serde(rename = "CreateSecondaryVMRequestType", tag = "_typeName")]
struct CreateSecondaryVmRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(rename = "CreateSecondaryVMExRequestType", tag = "_typeName")]
struct CreateSecondaryVmExRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a FaultToleranceConfigSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateSnapshotRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    memory: bool,
    quiesce: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateSnapshotExRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    memory: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quiesceSpec")]
    quiesce_spec: Option<&'a dyn VirtualMachineGuestQuiesceSpecTrait>,
}
#[derive(serde::Serialize)]
#[serde(rename = "CustomizeVMRequestType", tag = "_typeName")]
struct CustomizeVmRequestType<'a> {
    spec: &'a CustomizationSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DetachDiskRequestType<'a> {
    #[serde(rename = "diskId")]
    disk_id: &'a Id,
}
#[derive(serde::Serialize)]
#[serde(rename = "DisableSecondaryVMRequestType", tag = "_typeName")]
struct DisableSecondaryVmRequestType<'a> {
    vm: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DropConnectionsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "listOfConnections")]
    list_of_connections: Option<&'a [Box<dyn VirtualMachineConnectionTrait>]>,
}
#[derive(serde::Serialize)]
#[serde(rename = "EnableSecondaryVMRequestType", tag = "_typeName")]
struct EnableSecondaryVmRequestType<'a> {
    vm: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct InstantCloneRequestType<'a> {
    spec: &'a VirtualMachineInstantCloneSpec,
}
#[derive(serde::Serialize)]
#[serde(rename = "MakePrimaryVMRequestType", tag = "_typeName")]
struct MakePrimaryVmRequestType<'a> {
    vm: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MarkAsVirtualMachineRequestType<'a> {
    pool: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(rename = "MigrateVMRequestType", tag = "_typeName")]
struct MigrateVmRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pool: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
    priority: VirtualMachineMovePriorityEnum,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    state: Option<VirtualMachinePowerStateEnum>,
}
#[derive(serde::Serialize)]
#[serde(rename = "PowerOnVMRequestType", tag = "_typeName")]
struct PowerOnVmRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PromoteDisksRequestType<'a> {
    unlink: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    disks: Option<&'a [VirtualDisk]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PutUsbScanCodesRequestType<'a> {
    spec: &'a UsbScanCodeSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryChangedDiskAreasRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    snapshot: Option<&'a ManagedObjectReference>,
    #[serde(rename = "deviceKey")]
    device_key: i32,
    #[serde(rename = "startOffset")]
    start_offset: i64,
    #[serde(rename = "changeId")]
    change_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryFaultToleranceCompatibilityExRequestType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "forLegacyFt")]
    for_legacy_ft: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(rename = "ReconfigVMRequestType", tag = "_typeName")]
struct ReconfigVmRequestType<'a> {
    spec: &'a VirtualMachineConfigSpec,
}
#[derive(serde::Serialize)]
#[serde(rename = "reloadVirtualMachineFromPathRequestType", tag = "_typeName")]
struct ReloadVirtualMachineFromPathRequestType<'a> {
    #[serde(rename = "configurationPath")]
    configuration_path: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "RelocateVMRequestType", tag = "_typeName")]
struct RelocateVmRequestType<'a> {
    spec: &'a VirtualMachineRelocateSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    priority: Option<VirtualMachineMovePriorityEnum>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveAllSnapshotsRequestType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    consolidate: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameRequestType<'a> {
    #[serde(rename = "newName")]
    new_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RevertToCurrentSnapshotRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "suppressPowerOn")]
    suppress_power_on: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetDisplayTopologyRequestType<'a> {
    displays: &'a [VirtualMachineDisplayTopology],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetScreenResolutionRequestType {
    width: i32,
    height: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct StartRecordingRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct StartReplayingRequestType<'a> {
    #[serde(rename = "replaySnapshot")]
    replay_snapshot: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(rename = "TerminateFaultTolerantVMRequestType", tag = "_typeName")]
struct TerminateFaultTolerantVmRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    vm: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpgradeToolsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "installerOptions")]
    installer_options: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(rename = "UpgradeVMRequestType", tag = "_typeName")]
struct UpgradeVmRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<&'a str>,
}
