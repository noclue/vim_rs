use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *StoragePod* data object aggregates the storage
/// resources of associated *Datastore* objects into a single
/// storage resource for use by virtual machines.
/// 
/// The storage services
/// such as Storage DRS (Distributed Resource Scheduling),
/// enhance the utility of the storage pod.
/// 
/// Use the *Folder*.*Folder.CreateStoragePod* method
/// to create an instance of this object.
#[derive(Clone)]
pub struct StoragePod {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl StoragePod {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Creates a new single-host compute resource.
    /// 
    /// The name provided can be an
    /// IP address, such as 192.168.0.120, or a string, such as esx120.
    /// If a name is specified, a DNS lookup is used to resolve it to a fully-qualified
    /// name, such as esx120.vmware.com. If the DNS lookup fails, the string is
    /// stored as specified.
    /// 
    /// Licenses for the host are allocated when making the first connection to
    /// the host. This is because the license needed typically depends on the type
    /// of host and the number of CPUs.
    /// 
    /// In addition to the Host.Inventory.AddStandaloneHost privilege, it
    /// requires System.View privilege on the VM folder that the VMs of the
    /// host will be placed on.
    /// 
    /// ***Required privileges:*** Host.Inventory.AddStandaloneHost
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Specifies the parameters needed to add a single host.
    ///
    /// ### comp_res_spec
    /// Optionally specify the configuration for the compute
    /// resource that will be created to contain the host.
    ///
    /// ### add_connected
    /// Flag to specify whether or not the host should be
    /// connected as soon as it is added. The host will not
    /// be added if a connection attempt is made and fails.
    ///
    /// ### license
    /// Provide a licenseKey or licenseKeyType. See *LicenseManager*
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the newly added *ComputeResource* upon
    /// success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidLogin***: if authentication with the host fails.
    /// 
    /// ***InvalidArgument***: if an argument is specified incorrectly.
    /// 
    /// ***AlreadyBeingManaged***: if the host is already being managed by a
    /// vCenter server. If the host is being managed by a different
    /// vCenter server, this can be overridden by the "force" flag in the
    /// connection specification.
    /// 
    /// ***NotEnoughLicenses***: if there are not enough licenses to add the host.
    /// 
    /// ***NoHost***: if the host cannot be contacted.
    /// 
    /// ***NotSupported***: if the host is being added to a folder whose
    /// *Folder.childType* property does not contain
    /// "ComputeResource".
    /// 
    /// ***NotSupportedHost***: if the host is running a software version that is not
    /// supported.
    /// 
    /// ***AgentInstallFailed***: if there is an error installing the vCenter
    /// agent on the new host.
    /// 
    /// ***AlreadyConnected***: if addConnected is true and the host is already
    /// connected to vCenter.
    /// 
    /// ***HostConnectFault***: if an error occurred when attempting to connect
    /// to a host. Typically, a more specific subclass, such as
    /// AlreadyBeingManaged, is thrown.
    /// 
    /// ***SSLVerifyFault***: if the host certificate could not be authenticated
    /// 
    /// ***DuplicateName***: if another host in the same folder has the name.
    /// 
    /// ***NoPermission***: if there are crypto keys to be sent to the host,
    /// but the user does not have Cryptographer.RegisterHost privilege
    /// on the Folder.
    pub async fn add_standalone_host_task(&self, spec: &crate::types::structs::HostConnectSpec, comp_res_spec: Option<&dyn crate::types::traits::ComputeResourceConfigSpecTrait>, add_connected: bool, license: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = AddStandaloneHostRequestType {spec, comp_res_spec, add_connected, license, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "AddStandaloneHost_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Adds a set of new and existing hosts to the cluster.
    /// 
    /// This API is a composite API and performs the following tasks before hosts
    /// become part of the specified cluster -
    /// - Adds all new hosts as standalone hosts.
    /// - Move each host to the desired state.
    /// - Move each host to the cluster.
    ///   
    /// The dynamic privilege check will ensure that appropriate privileges
    /// are acquired to allow this API to perform multiple actions on hosts
    /// and cluster. Required privileges -
    /// - Host.Inventory.EditCluster on cluster
    /// - Host.Config.Maintenance on the hosts if desiredState is set
    /// - Privileges for *Folder.BatchAddStandaloneHosts_Task* if newHosts is
    ///   set
    /// - Host.Inventory.EditCluster on the hosts' source ComputeResource
    /// - Host.Inventory.MoveHost on the hosts
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Specifies the cluster to which hosts need to be
    /// added.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### new_hosts
    /// Specifies a list of new hosts to be added to
    /// the cluster. Hosts are first added as standalone hosts.
    ///
    /// ### existing_hosts
    /// Specifies a list of existing hosts to be
    /// added to the cluster. Hosts are first moved to the desired state
    /// before moving them to cluster.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ### comp_res_spec
    /// Specifies the configuration for the compute
    /// resource that will be created to contain all the hosts.
    ///
    /// ### desired_state
    /// Specifies desired state for hosts once added to
    /// the cluster. If not specified, hosts are added to the cluster in their
    /// current state. See *FolderDesiredHostState_enum* for valid values.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn batch_add_hosts_to_cluster_task(&self, cluster: &crate::types::structs::ManagedObjectReference, new_hosts: Option<&[crate::types::structs::FolderNewHostSpec]>, existing_hosts: Option<&[crate::types::structs::ManagedObjectReference]>, comp_res_spec: Option<&dyn crate::types::traits::ComputeResourceConfigSpecTrait>, desired_state: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = BatchAddHostsToClusterRequestType {cluster, new_hosts, existing_hosts, comp_res_spec, desired_state, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "BatchAddHostsToCluster_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Adds a list of hosts to inventory, as standalone hosts,
    /// in a single invocation.
    /// 
    /// The operation returns a result containing
    /// a list of hosts that are successfully added.
    /// 
    /// In addition to the Host.Inventory.AddStandaloneHost privilege, the operation
    /// requires System.View privilege on the VM folder that the VMs of the
    /// host will be placed on.
    /// 
    /// ***Required privileges:*** Host.Inventory.AddStandaloneHost
    ///
    /// ## Parameters:
    ///
    /// ### new_hosts
    /// Specifies a list of host specifications for new hosts.
    ///
    /// ### comp_res_spec
    /// Specifies the configuration for the compute
    /// resource that will be created to contain all the
    /// hosts.
    ///
    /// ### add_connected
    /// Flag to specify whether or not hosts should be
    /// connected at the time they are added. A host will not
    /// be added if a connection attempt is made and fails.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn batch_add_standalone_hosts_task(&self, new_hosts: Option<&[crate::types::structs::FolderNewHostSpec]>, comp_res_spec: Option<&dyn crate::types::traits::ComputeResourceConfigSpecTrait>, add_connected: bool) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = BatchAddStandaloneHostsRequestType {new_hosts, comp_res_spec, add_connected, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "BatchAddStandaloneHosts_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of VI API 2.5, use *Folder.CreateClusterEx*.
    /// 
    /// Creates a new cluster compute resource in this folder.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped, unless it
    /// is used to start an escape sequence. Clients may also escape any other characters
    /// in this name parameter.
    /// 
    /// ***Required privileges:*** Host.Inventory.CreateCluster
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// Name for the new cluster.
    ///
    /// ### spec
    /// Specification for the cluster.
    ///
    /// ## Returns:
    ///
    /// A new ClusterComputeResource instance.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if an entity with that name already exists.
    /// 
    /// ***InvalidArgument***: if the cluster configuration specification parameter is
    /// invalid.
    /// 
    /// ***InvalidName***: if the name is not a valid entity name.
    /// 
    /// ***NotSupported***: if the cluster is being added to a folder whose
    /// *Folder.childType* property value does not contain
    /// "ComputeResource" or "ClusterComputeResource".
    pub async fn create_cluster(&self, name: &str, spec: &crate::types::structs::ClusterConfigSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateClusterRequestType {name, spec, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "CreateCluster", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a new cluster compute resource in this folder.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped, unless it
    /// is used to start an escape sequence. Clients may also escape any other characters
    /// in this name parameter.
    /// 
    /// ***Required privileges:*** Host.Inventory.CreateCluster
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// Name for the new cluster.
    ///
    /// ### spec
    /// Specification for the cluster.
    ///
    /// ## Returns:
    ///
    /// A new ClusterComputeResource instance.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if an entity with that name already exists.
    /// 
    /// ***InvalidArgument***: if the cluster configuration specification parameter is
    /// invalid.
    /// 
    /// ***InvalidName***: if the name is not a valid entity name.
    /// 
    /// ***NotSupported***: if the cluster is being added to a folder whose
    /// *Folder.childType* property value does not contain
    /// "ComputeResource" or "ClusterComputeResource".
    pub async fn create_cluster_ex(&self, name: &str, spec: &crate::types::structs::ClusterConfigSpecEx) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateClusterExRequestType {name, spec, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "CreateClusterEx", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a new datacenter with the given name.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped, unless it
    /// is used to start an escape sequence. Clients may also escape any other characters
    /// in this name parameter.
    /// 
    /// ***Required privileges:*** Datacenter.Create
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// Name for the new datacenter. An entity name
    /// must be a non-empty string of less than 80 characters.
    /// The slash (/), backslash (\\) and percent (%) will be escaped
    /// using the URL syntax. For example, %2F.
    ///
    /// ## Returns:
    ///
    /// A new Datacenter instance.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if an entity with that name already exists.
    /// 
    /// ***InvalidName***: if the new name is not a valid entity name.
    /// 
    /// ***NotSupported***: if the datacenter is being created within a folder whose
    /// *Folder.childType* property value does not contain
    /// "Datacenter".
    pub async fn create_datacenter(&self, name: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateDatacenterRequestType {name, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "CreateDatacenter", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Create a *DistributedVirtualSwitch* in the folder according to the
    /// specified *DVSCreateSpec*.
    /// 
    /// The specified Folder
    /// must be a Network entity folder.
    /// 
    /// ***Required privileges:*** DVSwitch.Create
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The *DVSCreateSpec*
    /// to create the distributed virtual switch.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. After successful completion, the
    /// *Task*.*Task.info*.*TaskInfo.result* property
    /// contains the newly registered *DistributedVirtualSwitch*.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if called directly on a host.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// (*DVSConfigInfo.extensionKey*).
    pub async fn create_dvs_task(&self, spec: &crate::types::structs::DvsCreateSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateDvsRequestType {spec, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "CreateDVS_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a new sub-folder with the specified name.
    /// 
    /// The *Folder.childType* property of the new folder is the same as
    /// the *Folder.childType* property of the current folder.
    /// 
    /// ***Required privileges:*** Folder.Create
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name to be given the new folder. An entity name
    /// must be a non-empty string of less than 80 characters.
    /// The slash (/), backslash (\\) and percent (%) will be escaped
    /// using the URL syntax. For example, %2F. Any percent (%)
    /// character used in this parameter must be escaped, unless
    /// it is used to start an escape sequence. Clients may also
    /// escape any other characters in this parameter.
    ///
    /// ## Returns:
    ///
    /// A reference to the new folder.
    /// 
    /// Refers instance of *Folder*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if another object in the same folder has the
    /// target name.
    /// 
    /// ***InvalidName***: if the name is not a valid entity name.
    pub async fn create_folder(&self, name: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateFolderRequestType {name, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "CreateFolder", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a new storage pod in this folder.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped, unless it
    /// is used to start an escape sequence. Clients may also escape any other characters
    /// in this name parameter.
    /// 
    /// ***Required privileges:*** Folder.Create
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// Name for the new storage pod.
    ///
    /// ## Returns:
    ///
    /// A new StoragePod instance.
    /// 
    /// Refers instance of *StoragePod*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if an entity with that name already exists.
    /// 
    /// ***InvalidName***: if the name is not a valid entity name.
    /// 
    /// ***NotSupported***: if the storage pod is being added to a folder whose
    /// *Folder.childType* property value does not contain
    /// "StoragePod".
    pub async fn create_storage_pod(&self, name: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateStoragePodRequestType {name, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "CreateStoragePod", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a new virtual machine in the current folder and attaches it to the
    /// specified resource pool.
    /// 
    /// This operation creates a virtual machine,
    /// instead of cloning a virtual machine from an existing one.
    /// 
    /// The server does not support creating templates using this method.
    /// Instead, you should create templates by marking existing virtual
    /// machines as templates, or by cloning an existing virtual machine or
    /// template.
    /// 
    /// This operation only works if the folder's childType includes VirtualMachine.
    /// In addition to the VirtualMachine.Inventory.Create privilege, may also require
    /// any of the following privileges depending on the properties of the virtual
    /// machine bring created:
    /// - VirtualMachine.Config.AddExistingDisk if including a virtual disk device
    ///   that refers to an existing virtual disk file (not RDM)
    /// - VirtualMachine.Config.AddNewDisk if including a virtual disk device that
    ///   creates a new virtual disk file (not RDM)
    /// - VirtualMachine.Config.RawDevice if including a raw device mapping
    ///   (RDM) or SCSI passthrough device
    /// - VirtualMachine.Config.HostUSBDevice if including a VirtualUSB device
    ///   backed by a host USB device
    /// - VirtualMachine.Config.AdvancedConfig if setting values in
    ///   ConfigSpec.extraConfig
    /// - VirtualMachine.Config.SwapPlacement if setting swapPlacement
    /// - VirtualMachine.Config.ChangeTracking if setting changed
    ///   block tracking for the virtual machine's disks.
    /// - Datastore.AllocateSpace required on all datastores where the
    ///   virtual machine and its virtual disks will be created
    /// - Network.Assign required on the network which is assigned to the
    ///   new virtual machine that is being created
    /// - Cryptographer.EncryptNew on the folder if the created virtual
    ///   machine is encrypted
    /// - Cryptographer.RegisterHost on the host if the created virtual
    ///   machine is encrypted, but encryption is not enabled on the host
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
    /// ***Required privileges:*** VirtualMachine.Inventory.Create
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// The configuration of the virtual machine hardware.
    ///
    /// ### pool
    /// The resource pool to which the virtual machine will be attached.
    /// 
    /// ***Required privileges:*** Resource.AssignVMToPool
    /// 
    /// Refers instance of *ResourcePool*.
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
    /// ***DuplicateName***: if another virtual machine in the same folder already has
    /// the specified target name.
    /// 
    /// ***InvalidName***: if the name is not a valid entity name.
    /// 
    /// ***NotSupported***: if the virtual machine is being created within a folder
    /// whose *Folder.childType* property is not set to
    /// "VirtualMachine".
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
    /// ***AlreadyExists***: if the requested cfgPath (or the default cfgPath)
    /// for the virtual machine's configuration file is already loaded
    /// in the inventory.
    /// 
    /// ***InvalidState***: if the operation is not allowed in current state of
    /// the entities involved.
    /// 
    /// ***NoPermission***: if the created virtual machine is encrypted but the
    /// user does not have Cryptographer.EncryptNew on the folder.
    pub async fn create_vm_task(&self, config: &crate::types::structs::VirtualMachineConfigSpec, pool: &crate::types::structs::ManagedObjectReference, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateVmRequestType {config, pool, host, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "CreateVM_Task", Some(&input)).await?;
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
    /// ***Required privileges:*** Folder.Delete
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
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "Destroy_Task", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Moves a set of managed entities into this folder.
    /// 
    /// This operation is typically used by clients when they implement a drag-and-drop
    /// interface to move a set of objects into a folder.
    /// 
    /// This operation is transactional only with respect to each individual entity.
    /// The set of entities is moved sequentially as specified in the list, and
    /// committed one at a time. If the *Folder.MoveIntoFolder_Task* method fails on an object, the
    /// method terminates at that point with an exception, leaving the rest of the
    /// managed entities in their original location.
    /// 
    /// The objects that can be moved into a folder depends on the folder's
    /// type (as defined by the folder's *Folder.childType* property).
    /// For a datacenter folder, only datacenters and datacenter folders can be
    /// moved into the folder. For a virtual machine folder, only virtual machines
    /// and virtual machine folders can be moved into the folder.
    /// For a host folder, ComputeResource objects, host folder objects, and
    /// HostSystem objects can be moved into the folder.
    /// 
    /// Moving a HostSystem into a host folder creates a stand-alone host from a
    /// host that is currently part of a ClusterComputeResource. The host must be part
    /// of a ClusterComputeResource in the same datacenter and the host must be in
    /// maintenance mode. Otherwise, the operation fails.
    /// 
    /// A ComputeResource with a single root resource pool is created for each
    /// HostSystem. The name of the ComputeResource is the DNS or IP address of the
    /// host. This operation moves the (physical) host resources out of a cluster.
    /// It does not move or change the ResourcePool configuration that is part of the
    /// ClusterComputeResource with which the host was associated.
    /// 
    /// Note that all virtual machines associated with a host are moved with the host
    /// into the folder. If there are virtual machines that should not be moved
    /// with the host, then migrate them from the host before initiating this operation.
    /// 
    /// vSphere Lifecycle Manager baselines (previously called vSphere Update
    /// Manager VUM) is
    /// <a href="https://kb.vmware.com/s/article/89519">deprecated</a> in vCenter 8.0.
    /// You can instead manage the lifecycle of the
    /// hosts in your environment by using vSphere Lifecycle Manager images (vLCM).
    /// A Host moved from image managed cluster to datacenter/host folder will become
    /// baseline managed stand-alone host.
    /// 
    /// For a HostSystem move, the privileges required are Host.Inventory.EditCluster
    /// on the source ClusterComputeResource, Host.Inventory.MoveHost on the HostSystem,
    /// and Host.Inventory.AddStandaloneHost on the target Folder.
    /// 
    /// Otherwise, the privilege required for this operation varies depending on this
    /// folder's type and is checked against the source container, destination container,
    /// and the object:
    /// - Folder.Move if the object is a Folder
    /// - Datacenter.Move if the object is a Datacenter
    /// - Host.Inventory.MoveCluster if the object is a ComputeResource
    /// - VirtualMachine.Inventory.Move if the object is a virtual machine
    ///   or virtual machine template
    /// - DVSwitch.Move if the object is a DistributedVirtualSwitch
    /// - Datastore.Move if the object is a datastore
    /// - Network.Move if the object is a network
    ///   
    /// If the object is a HostSystem, the privileges required are
    /// Host.Inventory.AddStandaloneHost on the folder, Host.Inventory.MoveHost on
    /// the HostSystem, and Host.Inventory.EditCluster on the host's original
    /// ComputeResource.
    ///
    /// ## Parameters:
    ///
    /// ### list
    /// The list of objects to be moved into the folder.
    /// 
    /// Refers instances of *ManagedEntity*.
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
    /// ***DuplicateName***: if this folder already contains an object with
    /// the specified name.
    /// 
    /// ***InvalidFolder***: if a Folder that is a parent of this Folder is part
    /// of the list of objects.
    /// 
    /// ***InvalidState***: if a HostSystem is not part of the same
    /// datacenter, not part of a ClusterComputeResource, or not in
    /// maintenance mode.
    /// 
    /// ***NotSupported***: if the entity is being moved into a folder
    /// whose *Folder.childType* property is not set to
    /// the appropriate value. For example, a VirtualMachine entity
    /// cannot be moved into a folder whose ChildType property value
    /// does not contain "VirtualMachine".
    /// 
    /// ***DisallowedOperationOnFailoverHost***: if the host is being moved
    /// out of a cluster and was configured as a failover host in that
    /// cluster. See *ClusterFailoverHostAdmissionControlPolicy*.
    /// 
    /// ***VmAlreadyExistsInDatacenter***: if moving a standalone host between
    /// datacenters, and one or more of the host's virtual machines is
    /// already registered to a host in the destination datacenter.
    pub async fn move_into_folder_task(&self, list: &[crate::types::structs::ManagedObjectReference]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = MoveIntoFolderRequestType {list, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "MoveIntoFolder_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Adds an existing virtual machine to the folder.
    /// 
    /// Any % (percent) character used in this name parameter must be escaped, unless it
    /// is used to start an escape sequence. Clients may also escape any other characters
    /// in this name parameter.
    /// 
    /// This operation only works if the folder's type is VirtualMachine.
    /// In addition to the VirtualMachine.Inventory.Register and
    /// Resource.AssignVMToPool privileges, it requires System.Read privilege
    /// on the datastore that the existing virtual machine resides on. If the
    /// virtual machine is encrypted Cryptographer.RegisterVM is required on the
    /// folder, in which the virtual machine is registered. Otherwise, the VM is
    /// registered successfully, but is left in the locked state.
    /// 
    /// ***Required privileges:*** VirtualMachine.Inventory.Register
    ///
    /// ## Parameters:
    ///
    /// ### path
    /// A datastore path to the virtual machine.
    ///
    /// ### name
    /// The name to be assigned to the virtual machine. If this parameter is
    /// not set, the displayName configuration parameter of the virtual machine is
    /// used. An entity name must be a non-empty string of less than 80
    /// characters. The slash (/), backslash (\\) and percent (%) will be
    /// escaped using the URL syntax. For example, %2F.
    ///
    /// ### as_template
    /// Flag to specify whether or not the virtual machine
    /// should be marked as a template.
    ///
    /// ### pool
    /// The resource pool to which the virtual machine should be attached.
    /// If imported as a template, this parameter is not set.
    /// 
    /// ***Required privileges:*** Resource.AssignVMToPool
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ### host
    /// The target host on which the virtual machine will run. This parameter
    /// must specify a host that is a member of the ComputeResource indirectly
    /// specified by the pool. For a stand-alone host or a cluster,
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
    /// ***NotSupported***: if the operation is not supported. For example,
    /// templates are not supported directly on hosts. Also, if the operation
    /// is invoked on a folder whose *Folder.childType* property is
    /// not set to "VirtualMachine".
    /// 
    /// ***OutOfBounds***: if the maximum number of VMs for this folder has been
    /// exceeded. The maximum number is determined by
    /// Host.capability.maxSupportedVMs.
    /// 
    /// ***DuplicateName***: if another virtual machine in the same folder has
    /// the target name.
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
    /// ***InvalidArgument***: if any of the arguments such as host or resource pool
    /// are not set to valid values.
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
    /// 
    /// ***InvalidState***: if the operation is not allowed in current state of
    /// the entities involved.
    pub async fn register_vm_task(&self, path: &str, name: Option<&str>, as_template: bool, pool: Option<&crate::types::structs::ManagedObjectReference>, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RegisterVmRequestType {path, name, as_template, pool, host, };
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "RegisterVM_Task", Some(&input)).await?;
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
        self.client.invoke_void("", "StoragePod", &self.mo_id, "Reload", None).await
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
    /// ***Required privileges:*** Folder.Rename
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
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "Rename_Task", Some(&input)).await?;
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
        self.client.invoke_void("", "StoragePod", &self.mo_id, "setCustomValue", Some(&input)).await
    }
    /// Recursively unregisters all virtual machines and vApps, and destroys
    /// all child virtual machine folders.
    /// 
    /// This is similar to the Destroy\_Task method,
    /// but this method calls UnregisterAndDestroy\_Task on each virtual machine
    /// object instead of calling Destroy\_Task.
    /// This operation applies only to VirtualMachine folders.
    /// 
    /// UnregisterAndDestroy\_Task is a recursive operation that destroys the specified
    /// virtual machine folder, unregisters all child virtual machine objects, and destroys
    /// all child virtual machine folders. When you call UnregisterAndDestroy\_Task
    /// to destroy a virtual machine folder, the system uses the specified folder
    /// as a root and traverses its descendant hierarchy, calling UnregisterAndDestroy\_Task
    /// on each virtual machine object and Destroy\_Task on each virtual machine folder.
    /// UnregisterAndDestroy\_Task is a single operation that treats each recursive call
    /// as a single transaction, committing each call to remove an object individually.
    /// If a failure occurs, the method terminates at that point with an exception, leaving
    /// some or all objects unaffected.
    /// 
    /// If you are removing virtual machines, you must hold the VirtualMachine.Delete
    /// privilege on all of the virtual machines to be unregistered, and on their parent folders.
    /// If you are removing virtual applications, you must hold the VApp.Delete
    /// privilege on all of the virtual applications to be unregistered, and on their
    /// parent folders.
    /// 
    /// ***Required privileges:*** Folder.Delete
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
    /// ***InvalidState***: if a virtual machine is not powered off or suspended.
    /// 
    /// ***ConcurrentAccess***: if another client modifies the folder contents
    /// before this operation completes.
    /// 
    /// ***NotSupported***: if the *Folder.childType* property of the
    /// folder is not set to "VirtualMachine".
    pub async fn unregister_and_destroy_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let bytes = self.client.invoke("", "StoragePod", &self.mo_id, "UnregisterAndDestroy_Task", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "alarmActionsEnabled").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "availableField").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// An array of managed object references.
    /// 
    /// Each entry is a reference to a child entity.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *ManagedEntity*.
    pub async fn child_entity(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "childEntity").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Specifies the object types a folder may contain.
    /// 
    /// When you create a folder, it inherits its childType from the parent folder
    /// in which it is created. childType is an array of strings. Each array entry
    /// identifies a set of object types - Folder and one or more managed object
    /// types. The following list shows childType values for the different folders:
    /// - { "vim.Folder", "vim.Datacenter" } - Identifies the root folder
    ///   and its descendant folders. Data center folders can contain
    ///   child data center folders and Datacenter managed objects.
    ///   Datacenter objects contain virtual machine, compute resource,
    ///   network entity, and datastore folders.
    /// - { "vim.Folder", "vim.Virtualmachine", "vim.VirtualApp" } - Identifies
    ///   a virtual machine folder. A virtual machine folder may contain child
    ///   virtual machine folders. It also can contain VirtualMachine managed objects,
    ///   templates, and VirtualApp managed objects.
    /// - { "vim.Folder", "vim.ComputeResource" } - Identifies a
    ///   compute resource folder, which contains child compute resource folders
    ///   and ComputeResource hierarchies.
    /// - { "vim.Folder", "vim.Network" } - Identifies a network entity folder.
    ///   Network entity folders on a vCenter Server can contain Network,
    ///   DistributedVirtualSwitch, and DistributedVirtualPortgroup managed
    ///   objects. Network entity folders on an ESXi host can contain only
    ///   Network objects.
    /// - { "vim.Folder", "vim.Datastore" } - Identifies a datastore folder.
    ///   Datastore folders can contain child datastore folders and Datastore
    ///   managed objects.
    ///   
    /// ***Required privileges:*** System.View
    pub async fn child_type(&self) -> Result<Option<Vec<String>>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "childType").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Current configuration issues that have been detected for this entity.
    /// 
    /// Typically,
    /// these issues have already been logged as events. The entity stores these
    /// events as long as they are still current. The
    /// *configStatus* property provides an overall status
    /// based on these events.
    pub async fn config_issue(&self) -> Result<Option<Vec<crate::types::structs::Event>>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "configIssue").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "configStatus").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property configStatus was empty".to_string()))?;
        let result: crate::types::enums::ManagedEntityStatusEnum = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "customValue").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "declaredAlarmState").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "disabledMethod").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "effectiveRole").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// The information of externally managed folder.
    /// 
    /// This property is only set for the externally managed folder.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    pub async fn externally_managed_folder_info(&self) -> Result<Option<crate::types::structs::FolderExternallyManagedFolderInfo>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "externallyManagedFolderInfo").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "name").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property name was empty".to_string()))?;
        let result: String = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// The namespace with which the Folder is associated.
    /// 
    /// Namespace is a vAPI
    /// resource which divides cluster resources and allows administrators to
    /// give Kubernetes environments to their development teams.
    /// This property is set only at the time of creation and cannot change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn namespace(&self) -> Result<Option<String>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "namespace").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "overallStatus").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property overallStatus was empty".to_string()))?;
        let result: crate::types::enums::ManagedEntityStatusEnum = crate::core::client::extract_property(pv)?;
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
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "parent").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// List of the permissions explicitly defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<crate::types::structs::Permission>>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "permission").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Storage DRS related attributes of the Storage Pod.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn pod_storage_drs_entry(&self) -> Result<Option<crate::types::structs::PodStorageDrsEntry>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "podStorageDrsEntry").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "recentTask").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Storage pod summary.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn summary(&self) -> Result<Option<crate::types::structs::StoragePodSummary>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "summary").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<crate::types::structs::Tag>>> {
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "tag").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "triggeredAlarmState").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "StoragePod", &self.mo_id, "value").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct AddStandaloneHostRequestType<'a> {
    spec: &'a crate::types::structs::HostConnectSpec,
    comp_res_spec: Option<&'a dyn crate::types::traits::ComputeResourceConfigSpecTrait>,
    add_connected: bool,
    license: Option<&'a str>,
}

impl<'a> miniserde::Serialize for AddStandaloneHostRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddStandaloneHostRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddStandaloneHostRequestTypeSer<'b, 'a> {
    data: &'b AddStandaloneHostRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddStandaloneHostRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddStandaloneHostRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.comp_res_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("compResSpec"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("addConnected"), &self.data.add_connected as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.license else { continue; };
                    return Some((std::borrow::Cow::Borrowed("license"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct BatchAddHostsToClusterRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    new_hosts: Option<&'a [crate::types::structs::FolderNewHostSpec]>,
    existing_hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    comp_res_spec: Option<&'a dyn crate::types::traits::ComputeResourceConfigSpecTrait>,
    desired_state: Option<&'a str>,
}

impl<'a> miniserde::Serialize for BatchAddHostsToClusterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(BatchAddHostsToClusterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct BatchAddHostsToClusterRequestTypeSer<'b, 'a> {
    data: &'b BatchAddHostsToClusterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for BatchAddHostsToClusterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"BatchAddHostsToClusterRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.new_hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("newHosts"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.existing_hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("existingHosts"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.comp_res_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("compResSpec"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.desired_state else { continue; };
                    return Some((std::borrow::Cow::Borrowed("desiredState"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct BatchAddStandaloneHostsRequestType<'a> {
    new_hosts: Option<&'a [crate::types::structs::FolderNewHostSpec]>,
    comp_res_spec: Option<&'a dyn crate::types::traits::ComputeResourceConfigSpecTrait>,
    add_connected: bool,
}

impl<'a> miniserde::Serialize for BatchAddStandaloneHostsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(BatchAddStandaloneHostsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct BatchAddStandaloneHostsRequestTypeSer<'b, 'a> {
    data: &'b BatchAddStandaloneHostsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for BatchAddStandaloneHostsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"BatchAddStandaloneHostsRequestType")),
                1 => {
                    let Some(ref val) = self.data.new_hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("newHosts"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.comp_res_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("compResSpec"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("addConnected"), &self.data.add_connected as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct CreateClusterRequestType<'a> {
    name: &'a str,
    spec: &'a crate::types::structs::ClusterConfigSpec,
}

impl<'a> miniserde::Serialize for CreateClusterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateClusterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateClusterRequestTypeSer<'b, 'a> {
    data: &'b CreateClusterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateClusterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateClusterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateClusterExRequestType<'a> {
    name: &'a str,
    spec: &'a crate::types::structs::ClusterConfigSpecEx,
}

impl<'a> miniserde::Serialize for CreateClusterExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateClusterExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateClusterExRequestTypeSer<'b, 'a> {
    data: &'b CreateClusterExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateClusterExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateClusterExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateDatacenterRequestType<'a> {
    name: &'a str,
}

impl<'a> miniserde::Serialize for CreateDatacenterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateDatacenterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateDatacenterRequestTypeSer<'b, 'a> {
    data: &'b CreateDatacenterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateDatacenterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateDatacenterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateDvsRequestType<'a> {
    spec: &'a crate::types::structs::DvsCreateSpec,
}

impl<'a> miniserde::Serialize for CreateDvsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateDvsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateDvsRequestTypeSer<'b, 'a> {
    data: &'b CreateDvsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateDvsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateDVSRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateFolderRequestType<'a> {
    name: &'a str,
}

impl<'a> miniserde::Serialize for CreateFolderRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateFolderRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateFolderRequestTypeSer<'b, 'a> {
    data: &'b CreateFolderRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateFolderRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateFolderRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateStoragePodRequestType<'a> {
    name: &'a str,
}

impl<'a> miniserde::Serialize for CreateStoragePodRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateStoragePodRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateStoragePodRequestTypeSer<'b, 'a> {
    data: &'b CreateStoragePodRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateStoragePodRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateStoragePodRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateVmRequestType<'a> {
    config: &'a crate::types::structs::VirtualMachineConfigSpec,
    pool: &'a crate::types::structs::ManagedObjectReference,
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for CreateVmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateVmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateVmRequestTypeSer<'b, 'a> {
    data: &'b CreateVmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateVmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateVMRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("pool"), &self.data.pool as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct MoveIntoFolderRequestType<'a> {
    list: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for MoveIntoFolderRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MoveIntoFolderRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MoveIntoFolderRequestTypeSer<'b, 'a> {
    data: &'b MoveIntoFolderRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MoveIntoFolderRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MoveIntoFolderRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("list"), &self.data.list as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RegisterVmRequestType<'a> {
    path: &'a str,
    name: Option<&'a str>,
    as_template: bool,
    pool: Option<&'a crate::types::structs::ManagedObjectReference>,
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for RegisterVmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RegisterVmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RegisterVmRequestTypeSer<'b, 'a> {
    data: &'b RegisterVmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RegisterVmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RegisterVMRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("path"), &self.data.path as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("name"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("asTemplate"), &self.data.as_template as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.pool else { continue; };
                    return Some((std::borrow::Cow::Borrowed("pool"), val as &dyn miniserde::Serialize));
                }
                5 => {
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
