use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::AlarmState;
use crate::types::structs::CustomFieldDef;
use crate::types::structs::Event;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::Permission;
use crate::types::structs::ResourceConfigOption;
use crate::types::structs::ResourceConfigSpec;
use crate::types::structs::ResourcePoolRuntimeInfo;
use crate::types::structs::Tag;
use crate::types::structs::VAppConfigSpec;
use crate::types::structs::VirtualMachineConfigSpec;
/// Represents a set of physical resources: a single host,
/// a subset of a host's resources, or resources spanning multiple hosts.
/// 
/// Resource pools can be subdivided by creating child resource pools. In
/// order to run, a virtual machine must be associated as a child of a resource
/// pool.
/// 
/// In a parent/child hierarchy of resource pools and virtual machines, the
/// single resource pool that has no parent pool is known as the _root resource
/// pool_.
/// 
/// **Configuration**
/// 
/// A resource pool is configured with a set of CPU (in MHz) and memory (in MB)
/// resources. These resources are specified in absolute terms with a resource
/// reservation and a resource limit, along with a shares setting. The shares
/// are used during resource contention, to ensure graceful degradation.
/// 
/// For the root resource pool, the values of the reservation and
/// the limit are set by the system and are not configurable. The
/// reservation and limit are set to the same value, indicating the total amount
/// of resources the system has available to run virtual machines. This is
/// computed as the aggregated CPU and memory resources provided by the set
/// of current available hosts in the parent compute resource minus the
/// overhead of the virtualization layer.
/// 
/// Since the resource pool configuration is absolute (in MHz or MB), the
/// configuration can become invalid when resources are removed. This can
/// happen if a host is removed from the cluster, if a host becomes
/// unavailable, or if a host is placed in maintenance mode. When this
/// happens, the system flags misconfigured resource pools and displays the
/// reservations and limits that are in effect. Further, in a DRS enabled cluster,
/// the tree can be misconfigured if the user bypasses VirtualCenter and powers on
/// VMs directly on the host.
/// 
/// **A General Discussion of Resource pool states and admission control**
/// There are three states that the resource pool tree can be in: undercommited
/// (green), overcommited (yellow), and inconsistent (red). Depending on the
/// state, different resource pool configuration policies are enforced. The
/// states are described in more detail below:
/// - **GREEN (aka undercommitted)**: We have a tree that is
///   in a _good_ state. Every node has a reservation greater than the sum of
///   the reservations for its children. We have enough capacity at the root to
///   satisfy all the resources reserved by the children. All operations
///   performed on the tree, such as powering on virtual machines, creating
///   new resource pools, or reconfiguring resource settings, will ensure
///   that the above constraints are maintained.
/// - **RED (aka. inconsistent)**: One or more nodes in the
///   tree has children whose reservations are greater than the node is configured to
///   support. For example, i) a resource pool with a fixed reservation has a running
///   virtual machine with a reservation that is higher than the reservation on
///   resource pool itself., or ii) the child reservations are greater than the limit.
///   
///   In this state, the DRS algorithm is disabled until the resource pool tree's
///   configuration has been brought back into a consistent state. We also restrict
///   the resources that such invalid nodes request from their parents to the
///   configured reservation/limit, in an attempt to isolate the problem to a small
///   subtree. For the rest of the tree, we determine whether the cluster is
///   undercommitted or overcommitted according to the existing rules and perform
///   admission control accordingly.
///   
///   Note that since all changes to the resource settings are validated on the
///   VirtualCenter server, the system cannot be brought into this state by simply
///   manipulating a cluster resource pool tree through VirtualCenter. It can only
///   happen if a virtual machine gets powered on directly on a host that is part of
///   a DRS cluster.
/// - **YELLOW (aka overcommitted)**: In this state, the tree is
///   consistent internally, but the root resource pool does not have the capacity at
///   to meet the reservation of its children. We can only go from GREEN -&gt; YELLOW if
///   we lose resources at the root. For example, hosts becomes unavailable or is
///   put into maintenance mode. Note that we will always have enough capacity at the root
///   to run all currently powered on VMs. However, we may not be able to satisfy all
///   resource pool reservations in the tree. In this state, the reservation configured for
///   a resource pool is no longer guaranteed, but the limits are still enforced.
///   This provides additional flexibility for bringing the tree back into a
///   consistent state, without risking bringing the tree into a RED state. In
///   more detail:
///   - **Resource Pool** The root is considered to have unlimited
///     capacity. You can reserve resources without any check except the
///     requirement that the tree remains consistent. This means that
///     nodes whose parents are all configured with expandable reservations and no limit
///     will have unlimited available resources. However, if there is an ancestor with
///     a fixed reservation or an expandable reservation with a limit somewhere, then the
///     node will be limited by the reservation/limit of the ancestor.
///   - **Virtual Machine** Virtual machines are limited by ancestors
///     with a fixed reservation and the capacity at the root.
///     
/// **Destroying a ResourcePool**
/// 
/// When a ResourcePool is destroyed, all the virtual machines are reassigned to its
/// parent pool. The root resource pool cannot be destroyed, and invoking destroy
/// on it will throw an InvalidType fault.
/// 
/// Any vApps in the ResourcePool will be moved to the ResourcePool's parent
/// before the pool is destroyed.
/// 
/// The Resource.DeletePool privilege must be held on the pool as well as the parent
/// of the resource pool. Also, the Resource.AssignVMToPool privilege must be held
/// on the resource pool's parent pool and any virtual machines that are reassigned.
pub struct ResourcePool {
    client: Arc<Client>,
    mo_id: String,
}
impl ResourcePool {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
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
    pub async fn create_resource_pool(&self, name: &str, spec: &ResourceConfigSpec) -> Result<ManagedObjectReference> {
        let input = CreateResourcePoolRequestType {name, spec, };
        let path = format!("/ResourcePool/{moId}/CreateResourcePool", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn create_v_app(&self, name: &str, res_spec: &ResourceConfigSpec, config_spec: &VAppConfigSpec, vm_folder: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = CreateVAppRequestType {name, res_spec, config_spec, vm_folder, };
        let path = format!("/ResourcePool/{moId}/CreateVApp", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn create_child_vm_task(&self, config: &VirtualMachineConfigSpec, host: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = CreateChildVmRequestType {config, host, };
        let path = format!("/ResourcePool/{moId}/CreateChildVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    /// ***Required privileges:*** Resource.DeletePool
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
        let path = format!("/ResourcePool/{moId}/Destroy_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
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
        let path = format!("/ResourcePool/{moId}/DestroyChildren", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
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
    pub async fn import_v_app(&self, spec: &dyn crate::types::traits::ImportSpecTrait, folder: Option<&ManagedObjectReference>, host: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = ImportVAppRequestType {spec, folder, host, };
        let path = format!("/ResourcePool/{moId}/ImportVApp", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn move_into_resource_pool(&self, list: &[ManagedObjectReference]) -> Result<()> {
        let input = MoveIntoResourcePoolRequestType {list, };
        let path = format!("/ResourcePool/{moId}/MoveIntoResourcePool", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn query_resource_config_option(&self) -> Result<ResourceConfigOption> {
        let path = format!("/ResourcePool/{moId}/QueryResourceConfigOption", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
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
        let path = format!("/ResourcePool/{moId}/RefreshRuntime", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
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
    pub async fn register_child_vm_task(&self, path: &str, name: Option<&str>, host: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = RegisterChildVmRequestType {path, name, host, };
        let path = format!("/ResourcePool/{moId}/RegisterChildVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
        let path = format!("/ResourcePool/{moId}/Reload", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
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
    /// ***Required privileges:*** Resource.RenamePool
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
        let path = format!("/ResourcePool/{moId}/Rename_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
        let path = format!("/ResourcePool/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn update_child_resource_configuration(&self, spec: &[ResourceConfigSpec]) -> Result<()> {
        let input = UpdateChildResourceConfigurationRequestType {spec, };
        let path = format!("/ResourcePool/{moId}/UpdateChildResourceConfiguration", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn update_config(&self, name: Option<&str>, config: Option<&ResourceConfigSpec>) -> Result<()> {
        let input = UpdateConfigRequestType {name, config, };
        let path = format!("/ResourcePool/{moId}/UpdateConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let path = format!("/ResourcePool/{moId}/alarmActionsEnabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/ResourcePool/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// The resource configuration of all direct children (VirtualMachine and
    /// ResourcePool) of this resource group.
    /// 
    /// Property collector update notifications might not be generated for this
    /// property. To listen for the child configuration change, please create
    /// PropertyCollector filter on the child entities directly.
    pub async fn child_configuration(&self) -> Result<Option<Vec<ResourceConfigSpec>>> {
        let path = format!("/ResourcePool/{moId}/childConfiguration", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// Configuration of this resource pool.
    pub async fn config(&self) -> Result<ResourceConfigSpec> {
        let path = format!("/ResourcePool/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Current configuration issues that have been detected for this entity.
    /// 
    /// Typically,
    /// these issues have already been logged as events. The entity stores these
    /// events as long as they are still current. The
    /// *configStatus* property provides an overall status
    /// based on these events.
    pub async fn config_issue(&self) -> Result<Option<Vec<Event>>> {
        let path = format!("/ResourcePool/{moId}/configIssue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
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
        let path = format!("/ResourcePool/{moId}/configStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let path = format!("/ResourcePool/{moId}/customValue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
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
        let path = format!("/ResourcePool/{moId}/declaredAlarmState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
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
        let path = format!("/ResourcePool/{moId}/disabledMethod", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let path = format!("/ResourcePool/{moId}/effectiveRole", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
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
        let path = format!("/ResourcePool/{moId}/name", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
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
        let path = format!("/ResourcePool/{moId}/namespace", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
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
        let path = format!("/ResourcePool/{moId}/overallStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// The ComputeResource to which this set of one or more nested resource pools
    /// belong.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ComputeResource*.
    pub async fn owner(&self) -> Result<ManagedObjectReference> {
        let path = format!("/ResourcePool/{moId}/owner", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
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
    pub async fn parent(&self) -> Result<Option<ManagedObjectReference>> {
        let path = format!("/ResourcePool/{moId}/parent", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// List of permissions defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<Permission>>> {
        let path = format!("/ResourcePool/{moId}/permission", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
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
        let path = format!("/ResourcePool/{moId}/recentTask", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// The set of child resource pools.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *ResourcePool*.
    pub async fn resource_pool(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ResourcePool/{moId}/resourcePool", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
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
    pub async fn runtime(&self) -> Result<ResourcePoolRuntimeInfo> {
        let path = format!("/ResourcePool/{moId}/runtime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
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
        let path = format!("/ResourcePool/{moId}/summary", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<Tag>>> {
        let path = format!("/ResourcePool/{moId}/tag", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
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
        let path = format!("/ResourcePool/{moId}/triggeredAlarmState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let path = format!("/ResourcePool/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// The set of virtual machines associated with this resource pool.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *VirtualMachine*.
    pub async fn vm(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ResourcePool/{moId}/vm", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateResourcePoolRequestType<'a> {
    name: &'a str,
    spec: &'a ResourceConfigSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateVAppRequestType<'a> {
    name: &'a str,
    #[serde(rename = "resSpec")]
    res_spec: &'a ResourceConfigSpec,
    #[serde(rename = "configSpec")]
    config_spec: &'a VAppConfigSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vmFolder")]
    vm_folder: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(rename = "CreateChildVMRequestType", tag = "_typeName")]
struct CreateChildVmRequestType<'a> {
    config: &'a VirtualMachineConfigSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ImportVAppRequestType<'a> {
    spec: &'a dyn crate::types::traits::ImportSpecTrait,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    folder: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MoveIntoResourcePoolRequestType<'a> {
    list: &'a [ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(rename = "RegisterChildVMRequestType", tag = "_typeName")]
struct RegisterChildVmRequestType<'a> {
    path: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameRequestType<'a> {
    #[serde(rename = "newName")]
    new_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateChildResourceConfigurationRequestType<'a> {
    spec: &'a [ResourceConfigSpec],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateConfigRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    config: Option<&'a ResourceConfigSpec>,
}
