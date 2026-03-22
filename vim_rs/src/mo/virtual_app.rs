use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Represents a multi-tiered software solution.
/// 
/// A vApp is a collection of
/// virtual machines (and potentially other vApp containers) that are operated and
/// monitored as a unit. From a manage perspective, a multi-tiered vApp acts a
/// lot like a virtual machine object. It has power operations, networks, datastores,
/// and its resource usage can be configured.
/// 
/// From a technical perspective, a vApp container is a specialized resource pool that
/// has been extended with the following capabilities:
/// - Product information such as product name, vendor, properties,
///   and licenses.
/// - A power-on/power-off sequence specification
/// - Support for import/export vApps as OVF packages
/// - An OVF environment that allows for application-level customization
///   
/// **Destroying a vApp**
/// 
/// When a vApp is destroyed, all of its virtual machines are destroyed,
/// as well as any child vApps.
/// 
/// The VApp.Delete privilege must be held on the vApp as well as the
/// parent folder of the vApp. Also, the VApp.Delete privilege must
/// be held on any child vApps that would be destroyed by the operation.
#[derive(Clone)]
pub struct VirtualApp {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VirtualApp {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Creates a clone of this vApp.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped, unless it
    /// is used to start an escape sequence. Clients may also escape any other characters
    /// in this name parameter.
    /// 
    /// When invoking this method, the following privilege checks occur:
    /// - The privilege VApp.Clone is required on this vApp.
    /// - If the target is a resource pool, the privilege
    ///   Resource.AssignVAppToPool is required on it.
    /// - If the target is a vApp, the privileges VApp.Clone and
    ///   VApp.AssignVApp are required on it.
    ///   
    /// Additional privileges are required by the clone spec provided. See *VAppCloneSpec* for details.
    /// 
    /// ***Required privileges:*** VApp.Clone
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the new vApp.
    ///
    /// ### target
    /// The parent entity of the new vApp. Must be of type
    /// *ResourcePool* or *VirtualApp*.
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ### spec
    /// Specifies how to clone the vApp.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the vApp is powered on.
    /// 
    /// ***TaskInProgress***: if the vApp is busy.
    /// 
    /// ***NotSupported***: if the operation is not supported by the current agent.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// vApp's current state. For example, if the virtual machine
    /// configuration information is not available, or if the vApp
    /// is running.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// target datastores.
    /// 
    /// ***FileFault***: if there was an error accessing one of the virtual machine files.
    /// 
    /// ***VmConfigFault***: if one of the virtual machines are not compatible with a
    /// destination host. Typically, a specific subclass of this exception is
    /// thrown, such as IDEDiskNotSupported.
    /// 
    /// ***MigrationFault***: if it is not possible to migrate one of the virtual machines
    /// to the destination. This is typically due to hosts being incompatible,
    /// such as mismatch in network polices or access to networks and datastores.
    /// Typically, a more specific subclass is thrown.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    pub async fn clone_v_app_task(&self, name: &str, target: &crate::types::structs::ManagedObjectReference, spec: &crate::types::structs::VAppCloneSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CloneVAppRequestType {name, target, spec, };
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "CloneVApp_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a new resource pool.
    /// 
    /// ***Required privileges:*** Resource.CreatePool
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the ResourcePool. Any % (percent) character
    /// used in this parameter must be escaped, unless it is used
    /// to start an escape sequence. Clients may also escape any
    /// other characters in this parameter.
    ///
    /// ### spec
    /// The spec for the ResourcePool.
    /// All values in ResourceAllocationInfo must be specified and
    /// are not optional.
    ///
    /// ## Returns:
    ///
    /// A reference to the new resource pool.
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the ComputeResource does not support
    /// nested resource pools.
    /// 
    /// ***InvalidName***: if the name is not a valid entity name.
    /// 
    /// ***DuplicateName***: if this pool already contains an object
    /// with the given name.
    /// 
    /// ***InvalidArgument***: if the pool specification is invalid.
    /// 
    /// ***InsufficientResourcesFault***: if the operation would violate a resource
    /// usage policy. Typically, a more specific subclass, such as
    /// InsufficientCpuResourcesFault will be thrown.
    pub async fn create_resource_pool(&self, name: &str, spec: &crate::types::structs::ResourceConfigSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateResourcePoolRequestType {name, spec, };
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "CreateResourcePool", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a new vApp container.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped, unless it
    /// is used to start an escape sequence. Clients may also escape any other characters
    /// in this name parameter.
    /// 
    /// ***Required privileges:*** VApp.Create
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the vApp container in the inventory
    ///
    /// ### res_spec
    /// The resource configuration for the vApp container (same as for a
    /// regular resource pool).
    ///
    /// ### config_spec
    /// The specification of the vApp specific meta-data.
    ///
    /// ### vm_folder
    /// The parent folder for the vApp. This must be null if this is
    /// a child vApp.
    /// 
    /// Refers instance of *Folder*.
    ///
    /// ## Returns:
    ///
    /// The created vApp object.
    /// 
    /// Refers instance of *VirtualApp*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the ComputeResource does not support
    /// nested resource pools.
    /// 
    /// ***InvalidName***: if the name is not a valid entity name.
    /// 
    /// ***DuplicateName***: if this pool already contains an object
    /// with the given name.
    /// 
    /// ***InvalidArgument***: if the pool specification is invalid.
    /// 
    /// ***InsufficientResourcesFault***: if the operation would violate a resource
    /// usage policy. Typically, a more specific subclass, such as
    /// InsufficientCpuResourcesFault will be thrown.
    /// 
    /// ***InvalidState***: if the resource pool does not support the operation in
    /// its current state. This will typically be a subclass such
    /// as *NoActiveHostInCluster*.
    /// 
    /// ***VmConfigFault***: or a more specific subclass, if errors are found in
    /// the supplied in VApp configuration.
    pub async fn create_v_app(&self, name: &str, res_spec: &crate::types::structs::ResourceConfigSpec, config_spec: &crate::types::structs::VAppConfigSpec, vm_folder: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateVAppRequestType {name, res_spec, config_spec, vm_folder, };
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "CreateVApp", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a new virtual machine in a vApp container.
    /// 
    /// This method supports creating a virtual machine directly in a vApp. A
    /// virtual machine in a vApp is not associated with a VM folder and therefore
    /// cannot be created using the method on a *Folder*.
    /// 
    /// This method can only be called directly on a *vApp*
    /// or on a resource pool that is a child of a vApp.
    /// 
    /// The privilege VirtualMachine.Inventory.Create is required on this entity. Further,
    /// if this is a resource pool, the privilege Resource.AssignVMToPool is required. If
    /// this is a vApp, the privilege VApp.AssignVM is required.
    /// 
    /// Depending on the properties of the virtual machine bring created, additional
    /// privileges may be required. See *Folder.CreateVM_Task* for a description of
    /// these privileges.
    /// 
    /// ***Required privileges:*** VirtualMachine.Inventory.Create
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// The configuration of the virtual machine hardware.
    ///
    /// ### host
    /// The target host on which the virtual machine will run. This must
    /// specify a host that is a member of the ComputeResource indirectly
    /// specified by the pool. For a stand-alone host or a cluster with DRS,
    /// host can be omitted, and the system selects a default.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the newly created *VirtualMachine*
    /// upon success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VmConfigFault***: if the configSpec has incorrect values. Typically, a more
    /// specific subclass is thrown.
    /// 
    /// ***OutOfBounds***: if Host.capability.maxSupportedVMs is exceeded.
    /// 
    /// ***FileAlreadyExists***: if the requested cfgPath for the virtual machine's
    /// configuration file already exists.
    /// 
    /// ***FileFault***: if there is a problem creating the virtual machine on disk.
    /// Typically, a more specific subclass, such as NoDiskSpace, will be thrown.
    /// 
    /// ***InvalidName***: if the name is not a valid entity name.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// target datastores.
    /// 
    /// ***VmWwnConflict***: if the WWN of the virtual machine has been used by
    /// other virtual machines.
    /// 
    /// ***NotSupported***: if this resource pool is not a vApp or is a child
    /// of a vApp.
    pub async fn create_child_vm_task(&self, config: &crate::types::structs::VirtualMachineConfigSpec, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateChildVmRequestType {config, host, };
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "CreateChildVM_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// ***Required privileges:*** VApp.Delete
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
    pub async fn destroy_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "Destroy_Task", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Removes all child resource pools recursively.
    /// 
    /// All virtual machines and vApps
    /// associated with the child resource pools get associated with this resource pool.
    /// 
    /// Note that resource pools contained in child vApps are not affected.
    /// 
    /// The privilege checks performed are the following.
    /// - Resource.DeletePool privilege must be held on this object and each of it's
    ///   immediate children to be destroyed.
    /// - If VMs are being moved, the privilege Resource.AssignVMToPool must be held
    ///   on this resource pool as well as on any virtual machines being moved.
    /// - If vApps are being moved, the privilege Resource.AssignVAppToPool
    ///   must be held on this resource pool as well as on any vApps being
    ///   moved.
    pub async fn destroy_children(&self) -> Result<()> {
        self.client.invoke_void("", "VirtualApp", &self.mo_id, "DestroyChildren", None).await
    }
    /// Obtains an export lease on this vApp.
    /// 
    /// The export lease contains a list
    /// of URLs for the disks of the virtual machines in this vApp, as well as
    /// a ticket that gives access to these URLs.
    /// 
    /// See *HttpNfcLease* for information on how to use the lease.
    /// 
    /// ***Required privileges:*** VApp.Export
    ///
    /// ## Returns:
    ///
    /// the export lease on this vApp. The export task continues
    /// running until the lease is completed or aborted.
    /// 
    /// Refers instance of *HttpNfcLease*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the vApp is powered on.
    /// 
    /// ***TaskInProgress***: if the vApp is busy.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// vApp's current state. For example, if the virtual machine
    /// configuration information is not available, or if the vApp
    /// is running or already powering on.
    /// 
    /// ***FileFault***: if there was an error accessing one of the virtual machine files.
    pub async fn export_v_app(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "ExportVApp", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a new entity in this resource pool.
    /// 
    /// The import process consists of two
    /// steps:
    /// 1. Create the VMs and/or vApps that make up the entity.
    /// 2. Upload virtual disk contents.
    ///    
    /// In step 1, the client must wait for the server to create all inventory
    /// objects. It does that by monitoring the *HttpNfcLease.state*
    /// property on the *HttpNfcLease* object returned from this call.
    /// When the server is done creating objects, the lease will change to the
    /// ready state, and step 2 begins. If an error occurs while the server is
    /// creating inventory objects, the lease will change to the error state, and
    /// the import process is aborted.
    /// 
    /// In step 2, the client uploads disk contents using the URLs provided in the
    /// *HttpNfcLease.info* property of the lease. The client must call
    /// *HttpNfcLease.HttpNfcLeaseProgress* on the lease periodically to keep the
    /// lease alive and report progress to the server. Failure to do so will cause
    /// the lease to time out, and the import process will be aborted.
    /// 
    /// When the client is done uploading disks, it completes the lease by calling
    /// *HttpNfcLease.HttpNfcLeaseComplete*. The client can also abort the import
    /// process by calling *HttpNfcLease.HttpNfcLeaseAbort*.
    /// 
    /// If the import process fails, is aborted, or times out, all created inventory
    /// objects are removed, including all virtual disks.
    /// 
    /// This operation only works if the folder's childType includes VirtualMachine.
    /// 
    /// Depending on the properties of the virtual machine bring imported, additional
    /// privileges may be required. See *Folder.CreateVM_Task* for a description of
    /// these privileges.
    /// 
    /// ***Required privileges:*** VApp.Import
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// An *ImportSpec* describing what to import.
    ///
    /// ### folder
    /// The folder to which the entity will be attached.
    /// 
    /// ***Required privileges:*** VApp.Import
    /// 
    /// Refers instance of *Folder*.
    ///
    /// ### host
    /// The target host on which the entity will run. This must
    /// specify a host that is a member of the ComputeResource indirectly
    /// specified by the pool. For a stand-alone host or a cluster with DRS,
    /// host can be omitted, and the system selects a default.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// a *HttpNfcLease* object which is used to drive the import
    /// session.
    /// 
    /// Refers instance of *HttpNfcLease*.
    ///
    /// ## Errors:
    ///
    /// ***VmConfigFault***: if a VM configSpec has incorrect values. Typically, a more
    /// specific subclass is thrown.
    /// 
    /// ***OutOfBounds***: if Host.capability.maxSupportedVMs is exceeded.
    /// 
    /// ***FileAlreadyExists***: if the requested cfgPath for the virtual machine's
    /// configuration file already exists.
    /// 
    /// ***FileFault***: if there is a problem creating the virtual machine on disk.
    /// Typically, a more specific subclass, such as NoDiskSpace, will be thrown.
    /// 
    /// ***DuplicateName***: if another virtual machine in the same folder already has
    /// the specified target name.
    /// 
    /// ***InvalidName***: if the name is not a valid entity name.
    /// 
    /// ***NotSupported***: if the virtual machine is being created within a folder
    /// whose *Folder.childType* property is not set to "VirtualMachine",
    /// a vApp is being imported into a resource pool that does not support
    /// nested resource pools, or a virtual machine is being imported into a resource
    /// pool and no folder is given.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// target datastores.
    /// 
    /// ***VmWwnConflict***: if the WWN of the virtual machine has been used by
    /// other virtual machines.
    pub async fn import_v_app(&self, spec: &dyn crate::types::traits::ImportSpecTrait, folder: Option<&crate::types::structs::ManagedObjectReference>, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ImportVAppRequestType {spec, folder, host, };
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "ImportVApp", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Moves a set of resource pools, vApps or virtual machines into this pool.
    /// 
    /// The
    /// pools, vApps and virtual machines must be part of the cluster or standalone
    /// host that contains this pool.
    /// 
    /// For each entity being moved, the move is subject to the following privilege
    /// checks:
    /// - If the object being moved is a ResourcePool, then Resource.MovePool must be
    ///   held on the pool being moved and it's former parent pool or vApp. If the
    ///   target is a vApp, the privilege VApp.AssignResourcePool must be held on
    ///   it. If the target is a ResourcePool, Resource.MovePool must be held on it.
    /// - If the object being moved is a VirtualApp, VApp.Move must be held on
    ///   the vApp being moved and it's former parent pool or vApp. If the target
    ///   entity is a resource pool, Resource.AssignVAppToPool must be held on the
    ///   target. If the target is a vApp, the privilege VApp.AssignVApp must
    ///   be held on the target vApp.
    /// - If the object being moved is a VirtualMachine, then if the target is a
    ///   ResourcePool, Resource.AssignVMToPool is required on the VirtualMachine and the
    ///   target pool. If the target is a vApp, VApp.AssignVM is required on both
    ///   the VirtualMachine and the target pool.
    ///   
    /// This operation is typically used by clients when they implement a drag-and-drop
    /// interface to move a set of objects into a folder.
    /// 
    /// This operation is only transactional with respect to each individual entity.
    /// The set of entities is moved sequentially, as specified in the list,
    /// and committed one at a time. If a failure is detected, then the method
    /// terminates with an exception.
    /// 
    /// The root resource pool cannot be moved.
    ///
    /// ## Parameters:
    ///
    /// ### list
    /// A list of ResourcePool and VirtualMachine objects.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if this pool already contains an object with
    /// the given name.
    /// 
    /// ***InvalidArgument***: if an ancestor of this pool is in the list.
    /// 
    /// ***InsufficientResourcesFault***: if the move would violate the resource usage
    /// policy. Typically, a more specific subclass, such as
    /// InsufficientMemoryResourcesFault.
    pub async fn move_into_resource_pool(&self, list: &[crate::types::structs::ManagedObjectReference]) -> Result<()> {
        let input = MoveIntoResourcePoolRequestType {list, };
        self.client.invoke_void("", "VirtualApp", &self.mo_id, "MoveIntoResourcePool", Some(&input)).await
    }
    /// Stops this vApp.
    /// 
    /// The virtual machines (or child vApps) will be stopped in the order
    /// specified in the vApp configuration, if force is false. If force is set
    /// to true, all virtual machines are powered-off (in no specific order and
    /// possibly in parallel) regardless of the vApp auto-start configuration.
    /// 
    /// While a vApp is stopping, all power operations performed on sub entities are
    /// disabled through the VIM API. They will throw TaskInProgress.
    /// 
    /// ***Required privileges:*** VApp.PowerOff
    ///
    /// ## Parameters:
    ///
    /// ### force
    /// If force is false, the shutdown order in the vApp is
    /// executed. If force is true, all virtual machines are powered-off
    /// (regardless of shutdown order).
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the vApp is not running
    /// 
    /// ***TaskInProgress***: if the vApp is busy.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// vApp's current state. For example, if the vApp is in the
    /// process of being started.
    /// 
    /// ***MissingPowerOffConfiguration***: if no vApp powerOff configuration
    /// has been specified.
    pub async fn power_off_v_app_task(&self, force: bool) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = PowerOffVAppRequestType {force, };
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "PowerOffVApp_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Starts this vApp.
    /// 
    /// The virtual machines (or sub vApps) will be started in the order
    /// specified in the vApp configuration. If the vApp is suspended
    /// (@see vim.VirtualApp.Summary#suspended), all suspended virtual machines
    /// will be powered-on based on the defined start-up order.
    /// 
    /// While a vApp is starting, all power operations performed on sub entities are
    /// disabled through the VIM API. They will throw TaskInProgress.
    /// 
    /// In case of a failure to power-on a virtual machine, the exception from the virtual
    /// machine power on is returned, and the power-on sequence will be terminated. In
    /// case of a failure, virtual machines that are already started will remain
    /// powered-on.
    /// 
    /// ***Required privileges:*** VApp.PowerOn
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the vApp is already running
    /// 
    /// ***TaskInProgress***: if the vApp is busy
    /// 
    /// ***NotEnoughLicenses***: if there are not enough licenses to power on one or more
    /// virtual machines.
    /// 
    /// ***NotSupported***: if the vApp is marked as a template.
    /// 
    /// ***InvalidState***: if it fails to power on a virtual machine due to no host
    /// availability, or unable to access the configuration file of a VM.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***VmConfigFault***: if a configuration issue on the vApp or a virtual
    /// machine in the vApp prevents the power-on to complete. Typically, a
    /// more specific fault, such as InvalidPropertyType is thrown.
    /// 
    /// ***VAppConfigFault***: if a configuration issue on a vApp prevents the
    /// power-on. Typically, a more specific fault, MissingPowerOnConfiguration,
    /// is thrown.
    /// 
    /// ***FileFault***: if there is a problem accessing the virtual machine on the
    /// filesystem.
    /// 
    /// ***MissingNetworkIpConfig***: if no network configuration exists for the primary
    /// network for the vApp.
    pub async fn power_on_v_app_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "PowerOnVApp_Task", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 6.5.
    /// 
    /// Get a value range and default values for *ResourceConfigSpec*.
    /// 
    /// This API was never implemented, and there is no replacement for it.
    /// 
    /// ***Required privileges:*** Resource.EditPool
    ///
    /// ## Returns:
    ///
    /// *ResourceConfigOption* object.
    pub async fn query_resource_config_option(&self) -> Result<crate::types::structs::ResourceConfigOption> {
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "QueryResourceConfigOption", None).await?;
        let result: crate::types::structs::ResourceConfigOption = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Refreshes the resource usage data that is available in
    /// *ResourcePoolRuntimeInfo*.
    /// 
    /// The latest runtime resource usage of this resource pool may not be
    /// available immediately after operations that alter resource usage,
    /// such as powering on a virtual machine. Invoke this method when resource
    /// usage may have recently changed, and the most up-to-date value in the
    /// *ResourcePoolRuntimeInfo* is needed.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn refresh_runtime(&self) -> Result<()> {
        self.client.invoke_void("", "VirtualApp", &self.mo_id, "RefreshRuntime", None).await
    }
    /// Adds an existing virtual machine to this resource pool or vApp.
    /// 
    /// This operation only works for vApps or resource pools that are children of
    /// vApps. To register a VM in a folder, see *Folder.RegisterVM_Task*.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped, unless it
    /// is used to start an escape sequence. Clients may also escape any other characters
    /// in this name parameter.
    /// In addition to the VirtualMachine.Inventory.Register privilege, it
    /// requires System.Read privilege on the datastore that the existing virtual
    /// machine resides on.
    /// 
    /// ***Required privileges:*** VirtualMachine.Inventory.Register
    ///
    /// ## Parameters:
    ///
    /// ### path
    /// A datastore path to the virtual machine. If the path ends with
    /// ".vmtx", indicating that it refers to a VM template, an InvalidArgument
    /// fault is thrown.
    ///
    /// ### name
    /// The name to be assigned to the virtual machine. If this parameter is
    /// not set, the displayName configuration parameter of the virtual machine is
    /// used. An entity name must be a non-empty string of less than 80
    /// characters. The slash (/), backslash (\\) and percent (%) will be
    /// escaped using the URL syntax. For example, %2F.
    ///
    /// ### host
    /// The target host on which the virtual machine will run. This parameter
    /// must specify a host that is a member of the ComputeResource to which this
    /// resource pool belongs. For a stand-alone host or a cluster with DRS,
    /// the parameter can be omitted, and the system selects a default.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the newly registered *VirtualMachine*
    /// upon success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the operation is not supported. For example, if the
    /// operation is invoked on a resource pool that is unrelated to a vApp.
    /// 
    /// ***OutOfBounds***: if the maximum number of VMs has been exceeded.
    /// 
    /// ***AlreadyExists***: if the virtual machine is already registered.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// target datastores.
    /// 
    /// ***NotFound***: if the configuration file is not found on the system.
    /// 
    /// ***InvalidName***: if the entity name is invalid.
    /// 
    /// ***InvalidArgument***: if any of the arguments are invalid and a more specific
    /// fault type does not apply.
    /// 
    /// ***VmConfigFault***: if the format / configuration of the virtual machine
    /// is invalid. Typically, a more specific fault is thrown such as
    /// InvalidFormat if the configuration file cannot be read, or
    /// InvalidDiskFormat if the disks cannot be read.
    /// 
    /// ***FileFault***: if there is an error accessing the files on disk.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    pub async fn register_child_vm_task(&self, path: &str, name: Option<&str>, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RegisterChildVmRequestType {path, name, host, };
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "RegisterChildVM_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "VirtualApp", &self.mo_id, "Reload", None).await
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
    /// ***Required privileges:*** VApp.Rename
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
    pub async fn rename_task(&self, new_name: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RenameRequestType {new_name, };
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "Rename_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "VirtualApp", &self.mo_id, "setCustomValue", Some(&input)).await
    }
    /// Suspends this vApp.
    /// 
    /// Suspends all powered-on virtual machines in a vApp, including virtual machines
    /// in child vApps. The virtual machines are suspended in the same order as
    /// used for a power-off operation (reverse power-on sequence).
    /// 
    /// While a vApp is being suspended, all power operations performed on sub entities
    /// are disabled through the VIM API. They will throw TaskInProgress.
    /// 
    /// ***Required privileges:*** VApp.Suspend
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the vApp is not running
    /// 
    /// ***TaskInProgress***: if the vApp is busy.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// vApp's current state. For example, if the vApp is in the
    /// process of being started.
    pub async fn suspend_v_app_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "SuspendVApp_Task", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Removes this vApp from the inventory without removing
    /// any of the virtual machine's files on disk.
    /// 
    /// All high-level information
    /// stored with the management server (ESX Server or VirtualCenter) is
    /// removed, including information such as vApp configuration, statistics,
    /// permissions, and alarms.
    /// 
    /// ***Required privileges:*** VApp.Unregister
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the vApp is running.
    pub async fn unregister_v_app_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let bytes = self.client.invoke("", "VirtualApp", &self.mo_id, "unregisterVApp_Task", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Changes resource configuration of a set of children of this resource pool.
    /// 
    /// The
    /// method allows bulk modifications of the set of the direct children
    /// (virtual machines and resource pools).
    /// 
    /// Bulk modifications are not transactional. Each modification is made individually.
    /// If a failure is encountered while applying the changes, then the processing stops,
    /// meaning at least one and as many as all of the changes are not applied.
    /// 
    /// A set can include a subset of the resources. Children that are not
    /// mentioned in the list are not changed.
    /// 
    /// For each ResourceConfigSpec, the following privilege checks apply:
    /// - If the ResourceConfigSpec refers to a child resource pool or a child
    ///   vApp, the privileges required are the same as would be required for
    ///   calling *ResourcePool.UpdateConfig* on that entity.
    /// - If the ResourceConfigSpec refers to a virtual machine,
    ///   VirtualMachine.Config.Resource must be held on the virtual machine.
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// -
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if a managed entity that is not a child of this group
    /// is included.
    /// 
    /// ***InsufficientResourcesFault***: if the operation would violate a resource
    /// usage policy. Typically, a more specific subclass, such as
    /// InsufficientMemoryResourcesFault will be thrown.
    pub async fn update_child_resource_configuration(&self, spec: &[crate::types::structs::ResourceConfigSpec]) -> Result<()> {
        let input = UpdateChildResourceConfigurationRequestType {spec, };
        self.client.invoke_void("", "VirtualApp", &self.mo_id, "UpdateChildResourceConfiguration", Some(&input)).await
    }
    /// Updates the configuration of the resource pool.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped, unless it
    /// is used to start an escape sequence. Clients may also escape any other characters
    /// in this name parameter.
    /// 
    /// The privilege checks for this operation are as follows:
    /// - If this is a resource pool, the privilege Resource.EditPool is required on
    ///   this and on the parent pool or vApp.
    /// - If this is a vApp, the privilege VApp.ResourceConfig is required on
    ///   this and on the parent pool or vApp.
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// If set, then the new name of the resource pool.
    ///
    /// ### config
    /// If set, then the new resource allocation for this
    /// resource pool.
    ///
    /// ## Errors:
    ///
    /// ***InvalidName***: if the name is not a valid entity name.
    /// 
    /// ***DuplicateName***: if the name is changed to an already existing name.
    /// 
    /// ***InvalidArgument***: if the parameters are out of range,
    /// or if the reservationLimit field is set.
    /// 
    /// ***InsufficientResourcesFault***: if the pool specification cannot be
    /// supported by the parent resource pool or vApp.
    /// 
    /// ***ConcurrentAccess***: if the changeVersion does not match the server's
    /// changeVersion for the configuration.
    pub async fn update_config(&self, name: Option<&str>, config: Option<&crate::types::structs::ResourceConfigSpec>) -> Result<()> {
        let input = UpdateConfigRequestType {name, config, };
        self.client.invoke_void("", "VirtualApp", &self.mo_id, "UpdateConfig", Some(&input)).await
    }
    /// Deprecated as of vSphere API 5.1.
    /// 
    /// Reconfigure the set of linked children.
    /// 
    /// A VirtualMachine and vApp can be added as a linked child as long as it
    /// is not a direct child of another vApp. In case it is a linked child, the
    /// existing link is removed and replaced with the new link specified in this
    /// call.
    /// 
    /// An InvalidArgument fault is thrown if a link target is a direct child
    /// of another vApp, or if the addition of the link will result in a vApp
    /// with a cycle. For example, a vApp cannot be linked to itself.
    /// 
    /// The removeSet must refer to managed entities that are currently linked
    /// children. Otherwise, an InvalidArgument exception is thrown.
    /// 
    /// For each entity being linked, the operation is subject to the following privilege
    /// checks:
    /// - If the object being linked is a vApp, VApp.Move must be held on
    ///   the vApp being linked and its former parent vApp (if any). The privilege
    ///   VApp.AssignVApp must be held on this vApp.
    /// - If the object being linked is a VirtualMachine, VApp.AssignVM is required on
    ///   both the target vApp, the VirtualMachine, and its former parent vApp (if any).
    ///   
    /// Privilege checks for each entity in the removeSet are similar to the entities
    /// in the addChangeSet, except that there is no target vApp.
    /// 
    /// This operation is only transactional with respect to each individual link change.
    /// The changes are processed sequentially and committed one at a time. The
    /// addChangeSet is processed first, followed by the removeSet. If a failure is
    /// detected, then the method terminates with an exception.
    ///
    /// ## Parameters:
    ///
    /// ### add_change_set
    /// a set of LinkInfo objects that either add a new link
    /// or modify an exisiting link.
    ///
    /// ### remove_set
    /// a set of entities that should no longer link to this vApp.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: See above description.
    /// 
    /// ***ConcurrentAccess***: If a concurrent modification happens while adding the link.
    /// 
    /// ***NotSupported***: If the target of the link is not in the same datacenter.
    pub async fn update_linked_children(&self, add_change_set: Option<&[crate::types::structs::VirtualAppLinkInfo]>, remove_set: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<()> {
        let input = UpdateLinkedChildrenRequestType {add_change_set, remove_set, };
        self.client.invoke_void("", "VirtualApp", &self.mo_id, "UpdateLinkedChildren", Some(&input)).await
    }
    /// Updates the vApp configuration.
    /// 
    /// Updates in different areas require different privileges. See
    /// *VAppConfigSpec* for a full description.
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// contains the updates to the current configuration. Any set element,
    /// is changed. All values in the spec that is left unset, will not be
    /// modified.
    ///
    /// ## Errors:
    ///
    /// ***InvalidPowerState***: if the reconfiguration is not possible given the
    /// current powerState of the vApp.
    /// 
    /// ***TaskInProgress***: if the vApp is busy.
    /// 
    /// ***ConcurrentAccess***: if another operation conflicted with this operation.
    /// 
    /// ***InvalidArgument***: for wrong input.
    /// 
    /// ***InvalidIndexArgument***: if a wrong key is used in one of the arrays. For
    /// example, for duplicated entries in entityConfig.
    /// 
    /// ***VmConfigFault***: for bad configuration, such as invalid
    /// property types.
    pub async fn update_v_app_config(&self, spec: &crate::types::structs::VAppConfigSpec) -> Result<()> {
        let input = UpdateVAppConfigRequestType {spec, };
        self.client.invoke_void("", "VirtualApp", &self.mo_id, "UpdateVAppConfig", Some(&input)).await
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "alarmActionsEnabled").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "availableField").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// The resource configuration of all direct children (VirtualMachine and
    /// ResourcePool) of this resource group.
    /// 
    /// Property collector update notifications might not be generated for this
    /// property. To listen for the child configuration change, please create
    /// PropertyCollector filter on the child entities directly.
    pub async fn child_configuration(&self) -> Result<Option<Vec<crate::types::structs::ResourceConfigSpec>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "childConfiguration").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Deprecated as of vSphere API 5.1.
    /// 
    /// List of linked children.
    pub async fn child_link(&self) -> Result<Option<Vec<crate::types::structs::VirtualAppLinkInfo>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "childLink").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Configuration of this resource pool.
    pub async fn config(&self) -> Result<crate::types::structs::ResourceConfigSpec> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "config").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property config was empty".to_string()))?;
        let result: crate::types::structs::ResourceConfigSpec = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Current configuration issues that have been detected for this entity.
    /// 
    /// Typically,
    /// these issues have already been logged as events. The entity stores these
    /// events as long as they are still current. The
    /// *configStatus* property provides an overall status
    /// based on these events.
    pub async fn config_issue(&self) -> Result<Option<Vec<crate::types::structs::Event>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "configIssue").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
    pub async fn config_status(&self) -> Result<crate::types::enums::ManagedEntityStatusEnum> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "configStatus").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property configStatus was empty".to_string()))?;
        let result: crate::types::enums::ManagedEntityStatusEnum = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "customValue").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// A collection of references to the subset of datastore objects used by this
    /// vApp.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Datastore*.
    pub async fn datastore(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "datastore").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
    pub async fn declared_alarm_state(&self) -> Result<Option<Vec<crate::types::structs::AlarmState>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "declaredAlarmState").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "disabledMethod").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "effectiveRole").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "name").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property name was empty".to_string()))?;
        let result: String = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// The namespace with which the ResourcePool is associated.
    /// 
    /// Namespace is a
    /// vAPI resource which divides cluster resources and allows administrators
    /// to give Kubernetes environments to their development teams.
    /// This property is set only at the time of creation and cannot change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn namespace(&self) -> Result<Option<String>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "namespace").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// A collection of references to the subset of network objects that
    /// is used by this virtual machine.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Network*.
    pub async fn network(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "network").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
    pub async fn overall_status(&self) -> Result<crate::types::enums::ManagedEntityStatusEnum> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "overallStatus").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property overallStatus was empty".to_string()))?;
        let result: crate::types::enums::ManagedEntityStatusEnum = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// The ComputeResource to which this set of one or more nested resource pools
    /// belong.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ComputeResource*.
    pub async fn owner(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "owner").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property owner was empty".to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::extract_property(pv)?;
        Ok(result)
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
    pub async fn parent(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "parent").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// A reference to the parent folder in the VM and Template folder hierarchy.
    /// 
    /// This
    /// is only set for a root vApp. A root vApp is a vApp that is not a child of
    /// another vApp.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Folder*.
    pub async fn parent_folder(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "parentFolder").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Reference to the parent vApp.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ManagedEntity*.
    pub async fn parent_v_app(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "parentVApp").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// List of permissions defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<crate::types::structs::Permission>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "permission").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
    pub async fn recent_task(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "recentTask").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// The set of child resource pools.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *ResourcePool*.
    pub async fn resource_pool(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "resourcePool").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Runtime information about a resource pool.
    /// 
    /// The *ResourcePoolResourceUsage* information within
    /// *ResourcePoolRuntimeInfo* can be transiently stale.
    /// Use *ResourcePool.RefreshRuntime* method to
    /// update the information.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    pub async fn runtime(&self) -> Result<crate::types::structs::ResourcePoolRuntimeInfo> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "runtime").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property runtime was empty".to_string()))?;
        let result: crate::types::structs::ResourcePoolRuntimeInfo = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Basic information about a resource pool.
    /// 
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    pub async fn summary(&self) -> Result<Box<dyn crate::types::traits::ResourcePoolSummaryTrait>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "summary").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property summary was empty".to_string()))?;
        let result: Box<dyn crate::types::traits::ResourcePoolSummaryTrait> = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<crate::types::structs::Tag>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "tag").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
    pub async fn triggered_alarm_state(&self) -> Result<Option<Vec<crate::types::structs::AlarmState>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "triggeredAlarmState").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Configuration of this package.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn v_app_config(&self) -> Result<Option<crate::types::structs::VAppConfigInfo>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "vAppConfig").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "value").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// The set of virtual machines associated with this resource pool.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *VirtualMachine*.
    pub async fn vm(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "VirtualApp", &self.mo_id, "vm").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct CloneVAppRequestType<'a> {
    name: &'a str,
    target: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::VAppCloneSpec,
}

impl<'a> miniserde::Serialize for CloneVAppRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CloneVAppRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CloneVAppRequestTypeSer<'b, 'a> {
    data: &'b CloneVAppRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CloneVAppRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CloneVAppRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("target"), &self.data.target as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateResourcePoolRequestType<'a> {
    name: &'a str,
    spec: &'a crate::types::structs::ResourceConfigSpec,
}

impl<'a> miniserde::Serialize for CreateResourcePoolRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateResourcePoolRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateResourcePoolRequestTypeSer<'b, 'a> {
    data: &'b CreateResourcePoolRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateResourcePoolRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateResourcePoolRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateVAppRequestType<'a> {
    name: &'a str,
    res_spec: &'a crate::types::structs::ResourceConfigSpec,
    config_spec: &'a crate::types::structs::VAppConfigSpec,
    vm_folder: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for CreateVAppRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateVAppRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateVAppRequestTypeSer<'b, 'a> {
    data: &'b CreateVAppRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateVAppRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateVAppRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("resSpec"), &self.data.res_spec as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("configSpec"), &self.data.config_spec as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.vm_folder else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vmFolder"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CreateChildVmRequestType<'a> {
    config: &'a crate::types::structs::VirtualMachineConfigSpec,
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for CreateChildVmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateChildVmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateChildVmRequestTypeSer<'b, 'a> {
    data: &'b CreateChildVmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateChildVmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateChildVMRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ImportVAppRequestType<'a> {
    spec: &'a dyn crate::types::traits::ImportSpecTrait,
    folder: Option<&'a crate::types::structs::ManagedObjectReference>,
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for ImportVAppRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ImportVAppRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ImportVAppRequestTypeSer<'b, 'a> {
    data: &'b ImportVAppRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ImportVAppRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ImportVAppRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.folder else { continue; };
                    return Some((std::borrow::Cow::Borrowed("folder"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct MoveIntoResourcePoolRequestType<'a> {
    list: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for MoveIntoResourcePoolRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MoveIntoResourcePoolRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MoveIntoResourcePoolRequestTypeSer<'b, 'a> {
    data: &'b MoveIntoResourcePoolRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MoveIntoResourcePoolRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MoveIntoResourcePoolRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("list"), &self.data.list as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PowerOffVAppRequestType {
    force: bool,
}

impl miniserde::Serialize for PowerOffVAppRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PowerOffVAppRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PowerOffVAppRequestTypeSer<'b> {
    data: &'b PowerOffVAppRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for PowerOffVAppRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PowerOffVAppRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("force"), &self.data.force as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RegisterChildVmRequestType<'a> {
    path: &'a str,
    name: Option<&'a str>,
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for RegisterChildVmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RegisterChildVmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RegisterChildVmRequestTypeSer<'b, 'a> {
    data: &'b RegisterChildVmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RegisterChildVmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RegisterChildVMRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("path"), &self.data.path as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("name"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RenameRequestType<'a> {
    new_name: &'a str,
}

impl<'a> miniserde::Serialize for RenameRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RenameRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RenameRequestTypeSer<'b, 'a> {
    data: &'b RenameRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RenameRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RenameRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("newName"), &self.data.new_name as &dyn miniserde::Serialize)),
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
struct UpdateChildResourceConfigurationRequestType<'a> {
    spec: &'a [crate::types::structs::ResourceConfigSpec],
}

impl<'a> miniserde::Serialize for UpdateChildResourceConfigurationRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateChildResourceConfigurationRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateChildResourceConfigurationRequestTypeSer<'b, 'a> {
    data: &'b UpdateChildResourceConfigurationRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateChildResourceConfigurationRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateChildResourceConfigurationRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateConfigRequestType<'a> {
    name: Option<&'a str>,
    config: Option<&'a crate::types::structs::ResourceConfigSpec>,
}

impl<'a> miniserde::Serialize for UpdateConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateConfigRequestType")),
                1 => {
                    let Some(ref val) = self.data.name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("name"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.config else { continue; };
                    return Some((std::borrow::Cow::Borrowed("config"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UpdateLinkedChildrenRequestType<'a> {
    add_change_set: Option<&'a [crate::types::structs::VirtualAppLinkInfo]>,
    remove_set: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for UpdateLinkedChildrenRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateLinkedChildrenRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateLinkedChildrenRequestTypeSer<'b, 'a> {
    data: &'b UpdateLinkedChildrenRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateLinkedChildrenRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateLinkedChildrenRequestType")),
                1 => {
                    let Some(ref val) = self.data.add_change_set else { continue; };
                    return Some((std::borrow::Cow::Borrowed("addChangeSet"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.remove_set else { continue; };
                    return Some((std::borrow::Cow::Borrowed("removeSet"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UpdateVAppConfigRequestType<'a> {
    spec: &'a crate::types::structs::VAppConfigSpec,
}

impl<'a> miniserde::Serialize for UpdateVAppConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateVAppConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateVAppConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateVAppConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateVAppConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateVAppConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
