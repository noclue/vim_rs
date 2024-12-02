use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::Capability;
use crate::types::EventTrait;
use crate::types::HostVMotionCompatibility;
use crate::types::ManagedObjectReference;
use crate::types::ProductComponentInfo;
use crate::types::ServiceContent;
use crate::types::VirtualMachinePowerStateEnum;
/// The *ServiceInstance* managed object is the singleton root object of the inventory
/// on both vCenter servers and servers running on standalone host agents.
/// 
/// The server creates the *ServiceInstance* automatically, and also automatically
/// creates the various manager entities that provide services in the virtual
/// environment. Some examples of manager entities are *LicenseManager*,
/// *PerformanceManager*, and *ViewManager*. You can
/// access the manager entities through the *ServiceInstance.content* property.
/// 
/// A vSphere API client application begins by connecting to a server
/// and obtaining a reference to the *ServiceInstance*. The client can then use
/// the *ServiceInstance.RetrieveServiceContent* method to gain
/// access to the various vSphere manager entities and to the root folder
/// of the inventory.
/// 
/// When you create managed objects, the server adds them to the inventory.
/// The inventory of managed objects includes instances the following object types:
/// - *ServiceInstance* -- Root of the inventory; created by vSphere.
/// - *Datacenter* -- A container that represents a virtual
///   data center. It contains hosts, network entities,
///   virtual machines and virtual applications,
///   and datastores.
/// - *Folder* -- A container used for hierarchical
///   organization of the inventory.
/// - *VirtualMachine* -- A virtual machine.
/// - *VirtualApp* -- A virtual application.
/// - *ComputeResource* -- A compute resource
///   (either a cluster or a stand-alone host).
/// - *ResourcePool* -- A subset of resources provided by a ComputeResource.
/// - *HostSystem* -- A single host (ESX Server or VMware Server).
/// - *Network* -- A network available to either hosts or virtual
///   machines.
/// - *DistributedVirtualSwitch* -- A distributed virtual switch.
/// - *DistributedVirtualPortgroup* -- A distributed virtual port group.
/// - *Datastore* -- Platform-independent, host-independent storage
///   for virtual machine files.
///   
/// The following figure shows the organization of managed objects in the
/// vCenter hierarchy:
/// 
/// Every Datacenter has the following set of dedicated folders.
/// These folders are empty until you create entities for the Datacenter.
/// - A folder for any combination of *VirtualMachine*
///   and/or *VirtualApp* objects. *VirtualApp* objects can be nested,
///   but only the parent *VirtualApp* can be visible in the folder.
///   Virtual machines that are children of virtual applications are not
///   associated with a VirtualMachine/VirtualApp folder.
/// - A folder for a *ComputeResource* hierarchy.
/// - A folder for network entities - any combination
///   of *Network*, *DistributedVirtualSwitch*, and/or
///   *DistributedVirtualPortgroup* objects.
/// - A folder for *Datastore* objects.
///   
/// The host agent hierarchy has the same general form as the vCenter hierarchy,
/// but most of the objects are limited to one instance:
pub struct ServiceInstance {
    client: Arc<VimClient>,
    mo_id: String,
}
impl ServiceInstance {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Returns the current time on the server.
    /// 
    /// To monitor non-linear time changes,
    /// use the *serverClock* property.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// The date and time on the server.
    pub async fn current_time(&self) -> Result<String> {
        let path = format!("/ServiceInstance/{moId}/CurrentTime", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 4.0, use
    /// *VirtualMachineProvisioningChecker.QueryVMotionCompatibilityEx_Task* instead.
    /// 
    /// Investigates the general VMotion compatibility of a virtual machine with
    /// a set of hosts.
    /// 
    /// The virtual machine may be in any power state. Hosts
    /// may be in any connection state and also may be in maintenance mode.
    /// 
    /// ***Required privileges:*** Resource.QueryVMotion
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The virtual machine that is the designated VMotion candidate.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### host
    /// The group of hosts to analyze for compatibility.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ### compatibility
    /// The set of compatibility types to investigate.
    /// Each is a string chosen from VMotionCompatibilityType. If this
    /// argument is not set, then all compatibility types are
    /// investigated.
    ///
    /// ## Returns:
    ///
    /// An array where each element, associated with one of the input
    /// hosts, specifies which of the requested compatibility types
    /// applies to that host. If an input host has never been connected
    /// and therefore has no information available for determining its
    /// compatibility, it is omitted from the return list.
    pub async fn query_v_motion_compatibility(&self, vm: &ManagedObjectReference, host: &[ManagedObjectReference], compatibility: Option<&[String]>) -> Result<Option<Vec<HostVMotionCompatibility>>> {
        let input = QueryVMotionCompatibilityRequestType {vm, host, compatibility, };
        let path = format!("/ServiceInstance/{moId}/QueryVMotionCompatibility", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Retrieves the properties of the service instance.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// The properties belonging to the service instance,
    /// including various object managers.
    pub async fn retrieve_service_content(&self) -> Result<ServiceContent> {
        let path = format!("/ServiceInstance/{moId}/RetrieveServiceContent", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Component information for bundled products
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn retrieve_product_components(&self) -> Result<Option<Vec<ProductComponentInfo>>> {
        let path = format!("/ServiceInstance/{moId}/RetrieveProductComponents", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Deprecated as of vSphere API 4.0, use *VirtualMachineProvisioningChecker*
    /// instead.
    /// 
    /// Checks the validity of a set of proposed migrations.
    /// 
    /// A migration
    /// is the act of changing the assigned execution host of a virtual
    /// machine, which can result from invoking
    /// *VirtualMachine.MigrateVM_Task* or
    /// *VirtualMachine.RelocateVM_Task*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The set of virtual machines intended for migration.
    /// 
    /// Refers instances of *VirtualMachine*.
    ///
    /// ### state
    /// The power state that the virtual machines must have. If
    /// this argument is not set, each virtual machine is evaluated
    /// according to its current power state.
    ///
    /// ### test_type
    /// The set of tests to run. If this argument is not set, all
    /// tests will be run.
    ///
    /// ### pool
    /// The target resource pool for the virtual machines. If the
    /// pool parameter is left unset, the target pool for each particular
    /// virtual machine's migration will be that virtual machine's current
    /// pool. If the virtual machine is a template then either this
    /// parameter or the host parameter must be set; additionally if
    /// resource tests are requested then this parameter is required.
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ### host
    /// The target host on which the virtual machines will run. The host
    /// parameter may be left unset if the compute resource associated with
    /// the target pool represents a stand-alone host or a DRS-enabled
    /// cluster. In the former case the stand-alone host is used as the
    /// target host. In the latter case, each connected host in the cluster
    /// that is not in maintenance mode is tested as a target host.
    /// If the virtual machine is a template then either this
    /// parameter or the pool parameter must be set.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// A set of events that describe the warnings or errors that would
    /// apply if the proposed set of migrations were executed.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the target host(s) and target pool for a
    /// migration are not associated with the same compute resource,
    /// or if the host parameter is left unset when the target pool is
    /// associated with a non-DRS cluster.
    /// 
    /// ***InvalidPowerState***: if the state argument is set and at least one
    /// of the specified virtual machines is not in that power state.
    /// 
    /// ***NoActiveHostInCluster***: if a target host is not specified and a
    /// cluster associated with a target pool does not contain at least one
    /// potential target host. A host must be connected and not in maintenance
    /// mode in order to be considered as a potential target host.
    pub async fn validate_migration(&self, vm: &[ManagedObjectReference], state: Option<VirtualMachinePowerStateEnum>, test_type: Option<&[String]>, pool: Option<&ManagedObjectReference>, host: Option<&ManagedObjectReference>) -> Result<Option<Vec<Box<dyn EventTrait>>>> {
        let input = ValidateMigrationRequestType {vm, state, test_type, pool, host, };
        let path = format!("/ServiceInstance/{moId}/ValidateMigration", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// API-wide capabilities.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn capability(&self) -> Result<Capability> {
        let path = format!("/ServiceInstance/{moId}/capability", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The properties of the ServiceInstance managed object.
    /// 
    /// The content property
    /// is identical to the return value from the
    /// *ServiceInstance.RetrieveServiceContent* method.
    /// 
    /// Use the content property with the *PropertyCollector*
    /// to perform inventory traversal that includes the ServiceInstance.
    /// (In the absence of a content property, a traversal that encounters
    /// the *ServiceInstance* would require calling
    /// the *ServiceInstance.RetrieveServiceContent* method,
    /// and then invoking a second traversal to continue.)
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn content(&self) -> Result<ServiceContent> {
        let path = format!("/ServiceInstance/{moId}/content", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Contains the time most recently obtained from the server.
    /// 
    /// The time is not necessarily current. This property is intended for use
    /// with the PropertyCollector *PropertyCollector.WaitForUpdates*
    /// method. The PropertyCollector will provide notification if some event occurs
    /// that changes the server clock time in a non-linear fashion.
    /// 
    /// You should not rely on the serverClock property to get the current time
    /// on the server; instead, use the *ServiceInstance.CurrentTime* method.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn server_clock(&self) -> Result<String> {
        let path = format!("/ServiceInstance/{moId}/serverClock", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVMotionCompatibilityRequestType<'a> {
    vm: &'a ManagedObjectReference,
    host: &'a [ManagedObjectReference],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    compatibility: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ValidateMigrationRequestType<'a> {
    vm: &'a [ManagedObjectReference],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    state: Option<VirtualMachinePowerStateEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "testType")]
    test_type: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pool: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
