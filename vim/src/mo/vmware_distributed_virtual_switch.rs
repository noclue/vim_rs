use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::AlarmState;
use crate::types::structs::CustomFieldDef;
use crate::types::structs::DistributedVirtualPort;
use crate::types::structs::DistributedVirtualSwitchPortCriteria;
use crate::types::structs::DistributedVirtualSwitchProductSpec;
use crate::types::structs::DvPortConfigSpec;
use crate::types::structs::DvPortgroupConfigSpec;
use crate::types::structs::DvsCapability;
use crate::types::structs::DvsNetworkResourcePool;
use crate::types::structs::DvsNetworkResourcePoolConfigSpec;
use crate::types::structs::DvsRuntimeInfo;
use crate::types::structs::DvsSummary;
use crate::types::structs::DvsVmVnicResourcePoolConfigSpec;
use crate::types::structs::EntityBackupConfig;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::Permission;
use crate::types::structs::Tag;
use crate::types::structs::VMwareDvsLacpGroupSpec;
/// The *VmwareDistributedVirtualSwitch* managed object
/// is the VMware implementation of a distributed virtual switch.
/// 
/// The functionality listed here is for a VMware distributed virtual switch only.
/// 
/// When you use a VMware distributed virtual switch, you can perform
/// backup and restore operations on the VMware switch. You can also
/// perform rollback operations on the switch and on portgroups
/// associated with the VMware switch. See the description for the
/// following methods:
/// - *DistributedVirtualSwitchManager.DVSManagerExportEntity_Task*
/// - *DistributedVirtualSwitchManager.DVSManagerImportEntity_Task*
/// - *DistributedVirtualSwitch.DVSRollback_Task*
/// - *DistributedVirtualPortgroup.DVPortgroupRollback_Task*
pub struct VmwareDistributedVirtualSwitch {
    client: Arc<Client>,
    mo_id: String,
}
impl VmwareDistributedVirtualSwitch {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of vSphere API 6.0
    /// Use *DistributedVirtualSwitch.DvsReconfigureVmVnicNetworkResourcePool_Task* instead
    /// to add a Virtual NIC network resource pool.
    /// 
    /// Add a network resource pool.
    /// 
    /// ***Required privileges:*** DVSwitch.ResourceManagement
    ///
    /// ## Parameters:
    ///
    /// ### config_spec
    /// the network resource pool configuration specification.
    ///
    /// ## Errors:
    ///
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***NotSupported***: if network I/O control is not supported on
    /// the vSphere Distributed Switch.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *extensionKey*.
    pub async fn add_network_resource_pool(&self, config_spec: &[DvsNetworkResourcePoolConfigSpec]) -> Result<()> {
        let input = AddNetworkResourcePoolRequestType {config_spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/AddNetworkResourcePool", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Creates a single *DistributedVirtualPortgroup* and adds it
    /// to the distributed virtual switch.
    /// 
    /// ***Required privileges:*** DVPortgroup.Create
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The specification for the portgroup.
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object. The
    /// *Task*.*Task.info*.*TaskInfo.result* property
    /// contains a managed object reference to the new portgroup.
    /// The *DistributedVirtualSwitch.portgroup* property also contains
    /// the reference.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if a portgroup with the same name already exists
    /// 
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***InvalidName***: if name of the portgroup is invalid
    pub async fn create_dv_portgroup_task(&self, spec: &DvPortgroupConfigSpec) -> Result<ManagedObjectReference> {
        let input = CreateDvPortgroupRequestType {spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/CreateDVPortgroup_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates one or more *DistributedVirtualPortgroup*s and adds them to
    /// the distributed virtual switch.
    /// 
    /// ***Required privileges:*** DVPortgroup.Create
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The specification for the portgroup.
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object with which to monitor the operation.
    /// The method does not return a value in the
    /// *Task*.*Task.info*.*TaskInfo.result* property.
    /// Use the *DistributedVirtualSwitch.portgroup* property to obtain
    /// managed object references to the new portgroups.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: If called directly on a host.
    /// 
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *extensionKey*.
    pub async fn add_dv_portgroup_task(&self, spec: &[DvPortgroupConfigSpec]) -> Result<ManagedObjectReference> {
        let input = AddDvPortgroupRequestType {spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/AddDVPortgroup_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    /// ***Required privileges:*** DVSwitch.Delete
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/Destroy_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Enable/Disable network I/O control on the vSphere Distributed Switch.
    /// 
    /// ***Required privileges:*** DVSwitch.ResourceManagement
    ///
    /// ## Parameters:
    ///
    /// ### enable
    /// If true, enables I/O control. If false,
    /// disables network I/O control.
    ///
    /// ## Errors:
    ///
    /// ***DvsFault***: if the enabling/disabling fails.
    /// 
    /// ***NotSupported***: if network I/O control is not supported on
    /// the vSphere Distributed Switch.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *extensionKey*.
    pub async fn enable_network_resource_management(&self, enable: bool) -> Result<()> {
        let input = EnableNetworkResourceManagementRequestType {enable, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/EnableNetworkResourceManagement", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Return the keys of ports that meet the criteria.
    /// 
    /// On an ESXi host,
    /// the property shows only the connected ports currently on the host.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### criteria
    /// The port selection criteria. If unset, the operation
    /// returns the keys of all the ports in the switch.
    pub async fn fetch_dv_port_keys(&self, criteria: Option<&DistributedVirtualSwitchPortCriteria>) -> Result<Option<Vec<String>>> {
        let input = FetchDvPortKeysRequestType {criteria, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/FetchDVPortKeys", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Return the ports that meet the criteria.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### criteria
    /// The port selection criteria. If unset, the operation
    /// returns the keys of all the ports in the portgroup.
    pub async fn fetch_dv_ports(&self, criteria: Option<&DistributedVirtualSwitchPortCriteria>) -> Result<Option<Vec<DistributedVirtualPort>>> {
        let input = FetchDvPortsRequestType {criteria, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/FetchDVPorts", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Returns the portgroup identified by the key within this VDS.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### portgroup_key
    /// The key that identifies a portgroup of this VDS.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *DistributedVirtualPortgroup*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the portgroup for the specified key is not found.
    /// 
    /// ***NotSupported***: If the operation is not supported.
    pub async fn lookup_dv_port_group(&self, portgroup_key: &str) -> Result<Option<ManagedObjectReference>> {
        let input = LookupDvPortGroupRequestType {portgroup_key, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/LookupDvPortGroup", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Deprecated as of vSphere API 5.5.
    /// 
    /// Merge an existing DistributedVirtualSwitch (source) to this switch
    /// (destination).
    /// 
    /// The host members and the connected entity of the source
    /// switch will be transferred to the destination switch. This operation
    /// disconnects the entities from the source switch, tears down its host
    /// proxy switches, creates new proxies for the destination switch,
    /// and reconnects the entities to the destination switch.
    /// 
    /// In summary, this operation does the following:
    /// - Adds the
    ///   <code>config</code>.*DVSConfigInfo.maxPorts*
    ///   of the source switch to the <code>maxPorts</code> of the
    ///   destination switch.
    /// - The host members of the source switch leave the source switch
    ///   and join the destination switch with the same Physical NIC and
    ///   VirtualSwitch (if applicable). A set of new uplink ports,
    ///   compliant with the
    ///   *DVSConfigSpec.uplinkPortPolicy*,
    ///   is created as the hosts join the destination switch.
    /// - The portgroups on the source switch are copied over to destination
    ///   switch, by calculating the effective default port config and
    ///   creating a portgroup of the same name in the destination switch. If
    ///   the name already exists, the copied portgroup uses names following a
    ///   "Copy of switch-portgroup-name" scheme to avoid conflict. The same
    ///   number of ports are created inside each copied portgroup.
    /// - The standalone distributed virtual ports are not copied,
    ///   unless there is a virtual
    ///   machine or host virtual NIC connecting to it. In that case, the
    ///   operation calculates the effective port config and creates a port
    ///   in the destination switch with the same name. Name conflict is
    ///   resolved using numbers like "original-port-name(1)". The uplink ports
    ///   are not copied over.
    /// - The virtual machine and host virtual NICs are disconnected from the source
    ///   switch and reconnected with the destination switch, to the
    ///   copied standalone port or portgroup.
    /// - If you are using a *VmwareDistributedVirtualSwitch* -
    ///   Unless the PVLAN map contains exactly the same entries between
    ///   the source and destination VMware distributed virtual switches,
    ///   the method raises a fault if
    ///   *VmwareDistributedVirtualSwitchPvlanSpec.pvlanId*
    ///   is set in any port, portgroup, or switch that will be copied.
    ///   
    /// ***Required privileges:*** DVSwitch.Modify
    ///
    /// ## Parameters:
    ///
    /// ### dvs
    /// The switch (source) to be merged
    /// 
    /// ***Required privileges:*** DVSwitch.Delete
    /// 
    /// Refers instance of *DistributedVirtualSwitch*.
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object with which to monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: If called directly on a host.
    /// 
    /// ***ResourceInUse***: If failed to delete the source switch
    /// 
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *extensionKey*.
    pub async fn merge_dvs_task(&self, dvs: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = MergeDvsRequestType {dvs, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/MergeDvs_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 6.0.
    /// 
    /// Move the ports out of their current portgroup into the specified portgroup.
    /// 
    /// If the moving of any of the ports results in a violation of the portgroup
    /// policy, or type of the source or destination portgroup, the operation
    /// raises a fault. A conflict port cannot be moved.
    /// 
    /// ***Required privileges:*** DVSwitch.Modify
    ///
    /// ## Parameters:
    ///
    /// ### port_key
    /// The keys of the ports to be moved into the portgroup.
    ///
    /// ### destination_portgroup_key
    /// The key of the portgroup to be moved into.
    /// If unset, the port will be moved under the switch.
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object with which to monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: If called directly on a host.
    /// 
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *extensionKey*.
    pub async fn move_dv_port_task(&self, port_key: &[String], destination_portgroup_key: Option<&str>) -> Result<ManagedObjectReference> {
        let input = MoveDvPortRequestType {port_key, destination_portgroup_key, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/MoveDVPort_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// This method updates the *DistributedVirtualSwitch* product specifications.
    /// 
    /// ***Required privileges:*** DVSwitch.Modify
    ///
    /// ## Parameters:
    ///
    /// ### operation
    /// The operation. See *DistributedVirtualSwitchProductSpecOperationType_enum* for
    /// valid values. For
    /// *VmwareDistributedVirtualSwitch*,
    /// only *upgrade*
    /// is valid.
    ///
    /// ### product_spec
    /// The product info of the implementation.
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object with which to monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: If called directly on a host.
    /// 
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *extensionKey*.
    pub async fn perform_dvs_product_spec_operation_task(&self, operation: &str, product_spec: Option<&DistributedVirtualSwitchProductSpec>) -> Result<ManagedObjectReference> {
        let input = PerformDvsProductSpecOperationRequestType {operation, product_spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/PerformDvsProductSpecOperation_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Return the used VLAN ID (PVLAN excluded) in the switch.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn query_used_vlan_id_in_dvs(&self) -> Result<Option<Vec<i32>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/QueryUsedVlanIdInDvs", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Reconfigures a distributed virtual switch.
    /// 
    /// You can use this method
    /// to set switch properties or to reset the switch to a previous state.
    /// 
    /// **Reconfiguring a Standard Distributed Virtual Switch**
    /// 
    /// To reconfigure a *DistributedVirtualSwitch*,
    /// use a *DVSConfigSpec*
    /// to set the switch properties.
    /// 
    /// **Reconfiguring a VMware Distributed Virtual Switch**
    /// 
    /// If you use a *VmwareDistributedVirtualSwitch*,
    /// you can perform the following switch reconfiguration:
    /// - Use a *VMwareDVSConfigSpec*
    ///   to set the switch properties.
    /// - Use the *VMwareDVSConfigSpec*
    ///   returned by *DistributedVirtualSwitch.DVSRollback_Task*
    ///   to reset the switch to a previous state.
    ///   
    /// Reconfiguring the switch may require any of the following privileges,
    /// depending on what is being changed:
    /// - DVSwitch.PolicyOp if *DVSConfigSpec.policy*
    ///   is set.
    /// - DVSwitch.PortSetting if *DVSConfigSpec.defaultPortConfig*
    ///   is set.
    /// - DVSwitch.HostOp if *DVSConfigSpec.policy*
    ///   is set. The
    ///   user will also need the Host.Config.Network
    ///   privilege on the host.
    /// - DVSwitch.Vspan if *VMwareDVSConfigSpec.vspanConfigSpec*
    ///   is set.
    /// - DVSwitch.Modify for anything else.
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The configuration of the switch
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object with which to monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if called directly on a host or if the spec
    /// includes settings for any vNetwork Distributed
    /// Switch feature that is not supported on this
    /// switch.
    /// 
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *DVSConfigInfo.extensionKey*.
    /// 
    /// ***ResourceNotAvailable***: If there is no port available in the portgroup
    /// 
    /// ***VspanPortConflict***: if dvPort is used as both the transmitted source and destination ports in Distributed Port Mirroring sessions.
    /// 
    /// ***VspanPromiscuousPortNotSupported***: if a promiscuous port is used as transmitted source or destination in the Distributed Port Mirroring sessions.
    /// 
    /// ***VspanSameSessionPortConflict***: if a dvPort is used as both the source and destination in the same Distributed Port Mirroring session.
    /// 
    /// ***VspanDestPortConflict***: if a dvPort is used as desination ports in multiple Distributed Port Mirroring sessions.
    pub async fn reconfigure_dvs_task(&self, spec: &dyn crate::types::dvs_config_spec_trait::DvsConfigSpecTrait) -> Result<ManagedObjectReference> {
        let input = ReconfigureDvsRequestType {spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/ReconfigureDvs_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Reconfigure individual ports.
    /// 
    /// ***Required privileges:*** DVSwitch.PortConfig
    ///
    /// ## Parameters:
    ///
    /// ### port
    /// The specification of the ports.
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object with which to monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: If called directly on a host or if the switch
    /// implementation doesn't support this API or if the spec
    /// includes settings for any vSphere Distributed Switch
    /// feature that is not supported on this switch.
    /// 
    /// ***InvalidArgument***: If the array have different elements for the
    /// same port.
    /// 
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *extensionKey*.
    pub async fn reconfigure_dv_port_task(&self, port: &[DvPortConfigSpec]) -> Result<ManagedObjectReference> {
        let input = ReconfigureDvPortRequestType {port, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/ReconfigureDVPort_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// reconfigure the Virtual NIC network resource pool configuration.
    /// 
    /// ***Required privileges:*** DVSwitch.ResourceManagement
    ///
    /// ## Parameters:
    ///
    /// ### config_spec
    /// The Virtual NIC network resource pool configuration specification and operation type.
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object with which to monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***DvsFault***: if operation fails on any host or if there are other reconfigure failures.
    /// 
    /// ***NotFound***: if the resource pool does not exist on the dvs.
    /// 
    /// ***DuplicateName***: if a virtual NIC network resource pool with the same name already exists.
    /// 
    /// ***ConcurrentAccess***: if a Virtual NIC network resource pool is modified by
    /// two or more clients at the same time.
    /// 
    /// ***ResourceInUse***: If Virtual NIC network resource pool being removed
    /// is associated with a network entity
    /// 
    /// ***NotSupported***: if network I/O control is not supported on
    /// the vSphere Distributed Switch.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *extensionKey*.
    /// 
    /// ***ConflictingConfiguration***: if the any property being set is in conflict.
    pub async fn dvs_reconfigure_vm_vnic_network_resource_pool_task(&self, config_spec: &[DvsVmVnicResourcePoolConfigSpec]) -> Result<ManagedObjectReference> {
        let input = DvsReconfigureVmVnicNetworkResourcePoolRequestType {config_spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/DvsReconfigureVmVnicNetworkResourcePool_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 5.0.
    /// Use
    /// *DistributedVirtualSwitchManager*.*DistributedVirtualSwitchManager.RectifyDvsOnHost_Task* instead.
    /// 
    /// Update the switch configuration on the host to bring them in sync with the
    /// current configuration in vCenter Server.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The hosts to be rectified.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    pub async fn rectify_dvs_host_task(&self, hosts: Option<&[ManagedObjectReference]>) -> Result<ManagedObjectReference> {
        let input = RectifyDvsHostRequestType {hosts, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/RectifyDvsHost_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Refresh port states.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### port_keys
    /// The keys of the ports to be refreshed. If not specified, all port
    /// states are refreshed.
    ///
    /// ## Errors:
    ///
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    pub async fn refresh_dv_port_state(&self, port_keys: Option<&[String]>) -> Result<()> {
        let input = RefreshDvPortStateRequestType {port_keys, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/RefreshDVPortState", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/Reload", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Deprecated as of vSphere API 6.0
    /// Use *DistributedVirtualSwitch.DvsReconfigureVmVnicNetworkResourcePool_Task* instead
    /// to remove a Virtual NIC network resource pool.
    /// 
    /// Remove a network resource pool.
    /// 
    /// ***Required privileges:*** DVSwitch.ResourceManagement
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The network resource pool key.
    ///
    /// ## Errors:
    ///
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***NotFound***: if the resource pool does not exist on the dvs.
    /// 
    /// ***InvalidName***: if the name of the resource pool is invalid.
    /// 
    /// ***ResourceInUse***: If network resource pool is associated with a network entity
    /// 
    /// ***NotSupported***: if network I/O control is not supported on
    /// the vSphere Distributed Switch.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *extensionKey*.
    pub async fn remove_network_resource_pool(&self, key: &[String]) -> Result<()> {
        let input = RemoveNetworkResourcePoolRequestType {key, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/RemoveNetworkResourcePool", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    /// ***Required privileges:*** DVSwitch.Modify
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/Rename_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// This method determines if the distributed virtual switch configuration
    /// has changed.
    /// 
    /// If it has changed, the method returns a
    /// *VMwareDVSConfigSpec*.
    /// Use the *DistributedVirtualSwitch.ReconfigureDvs_Task* method to apply
    /// the rollback configuration to the switch.
    /// You can use the rollback method only on a *VmwareDistributedVirtualSwitch*.
    /// - If you specify the <code>entityBackup</code> parameter, the returned
    ///   configuration specification represents the exported switch configuration.
    ///   If the <code>entityBackup</code> matches the current switch
    ///   configuration, the method does not return a configuration specification.
    /// - If <code>entityBackup</code> is not specified, the returned configuration
    ///   specification represents a previous state of the switch, if available.
    ///   When you use a VMware distributed virtual switch, each time you reconfigure
    ///   the switch, the Server saves the switch configuration before applying the updates.
    ///   If the vCenter Server is restarted, the saved configuration is not preserved
    ///   and the method does not return a configuration specification.
    ///   
    /// To use the rollback method, you must have the DVSwitch.Read privilege.
    ///
    /// ## Parameters:
    ///
    /// ### entity_backup
    /// Backup of a distributed virtual switch, returned by
    /// the *DistributedVirtualSwitchManager.DVSManagerExportEntity_Task*
    /// method.
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object with which to monitor the operation.
    /// If the distributed virtual switch configuration has changed, the
    /// *Task*.*Task.info*.*TaskInfo.result*
    /// property contains the *DVSConfigSpec* object.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***RollbackFailure***: if there is no configuration specified in entityBackup and
    /// the previous configuration does not exist either.
    /// 
    /// ***DvsFault***: if operation fails.
    pub async fn dvs_rollback_task(&self, entity_backup: Option<&EntityBackupConfig>) -> Result<ManagedObjectReference> {
        let input = DvsRollbackRequestType {entity_backup, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/DVSRollback_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Set the capability of the switch.
    /// 
    /// ***Required privileges:*** DVSwitch.Modify
    ///
    /// ## Parameters:
    ///
    /// ### capability
    /// The capability of the switch.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: If called directly on a host or if the switch
    /// implementation doesn't support this API.
    /// 
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *extensionKey*.
    pub async fn update_dvs_capability(&self, capability: &DvsCapability) -> Result<()> {
        let input = UpdateDvsCapabilityRequestType {capability, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/UpdateDvsCapability", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update health check configuration.
    /// 
    /// ***Required privileges:*** DVSwitch.Modify
    ///
    /// ## Parameters:
    ///
    /// ### health_check_config
    /// The health check configuration.
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object with which to monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***NotSupported***: if health check is not supported on the switch.
    pub async fn update_dvs_health_check_config_task(&self, health_check_config: &[Box<dyn crate::types::dvs_health_check_config_trait::DvsHealthCheckConfigTrait>]) -> Result<ManagedObjectReference> {
        let input = UpdateDvsHealthCheckConfigRequestType {health_check_config, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/UpdateDVSHealthCheckConfig_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Update Link Aggregation Control Protocol groups.
    /// 
    /// It can be called if the value of
    /// *VMwareDVSConfigInfo.lacpApiVersion* is
    /// *multipleLag*
    /// else an exception ConflictingConfiguration will be thrown.
    /// 
    /// ***Required privileges:*** DVSwitch.Modify
    ///
    /// ## Parameters:
    ///
    /// ### lacp_group_spec
    /// The Link Aggregation Control Protocol groups to be configured.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***NotSupported***: if multiple Link Aggregation Control Protocol
    /// is not supported on the switch.
    pub async fn update_dvs_lacp_group_config_task(&self, lacp_group_spec: &[VMwareDvsLacpGroupSpec]) -> Result<ManagedObjectReference> {
        let input = UpdateDvsLacpGroupConfigRequestType {lacp_group_spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/UpdateDVSLacpGroupConfig_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 6.0
    /// Use *DistributedVirtualSwitch.DvsReconfigureVmVnicNetworkResourcePool_Task* instead
    /// to update the Virtual NIC network resource pool.
    /// 
    /// Update the network resource pool configuration.
    /// 
    /// ***Required privileges:*** DVSwitch.ResourceManagement
    ///
    /// ## Parameters:
    ///
    /// ### config_spec
    /// The network resource pool configuration specification.
    ///
    /// ## Errors:
    ///
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    /// 
    /// ***NotFound***: if the resource pool does not exist on the dvs.
    /// 
    /// ***InvalidName***: if the name of the resource pool is invalid.
    /// 
    /// ***ConcurrentAccess***: if a network resource pool is modified by
    /// two or more clients at the same time.
    /// 
    /// ***NotSupported***: if network I/O control is not supported on
    /// the vSphere Distributed Switch.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *extensionKey*.
    pub async fn update_network_resource_pool(&self, config_spec: &[DvsNetworkResourcePoolConfigSpec]) -> Result<()> {
        let input = UpdateNetworkResourcePoolRequestType {config_spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/UpdateNetworkResourcePool", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/alarmActionsEnabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Capability of the switch.
    /// 
    /// Capabilities are indicated at the port,
    /// portgroup and switch levels, and for version-specific features.
    /// When you retrieve this property from an ESXi host,
    /// *DistributedVirtualSwitch.capability*.*DVSCapability.dvsOperationSupported*
    /// should always be set to false.
    pub async fn capability(&self) -> Result<DvsCapability> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/capability", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Switch configuration data.
    pub async fn config(&self) -> Result<Box<dyn crate::types::dvs_config_info_trait::DvsConfigInfoTrait>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/config", moId = &self.mo_id);
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
    pub async fn config_issue(&self) -> Result<Option<Vec<Box<dyn crate::types::event_trait::EventTrait>>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/configIssue", moId = &self.mo_id);
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
    pub async fn config_status(&self) -> Result<crate::types::enums::ManagedEntityStatusEnum> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/configStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn crate::types::custom_field_value_trait::CustomFieldValueTrait>>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/customValue", moId = &self.mo_id);
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/declaredAlarmState", moId = &self.mo_id);
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/disabledMethod", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/effectiveRole", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/name", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 6.0
    /// Use *DVSConfigInfo.vmVnicNetworkResourcePool*
    /// to get the Virtual NIC resource pool information.
    /// Use *DVSConfigInfo.infrastructureTrafficResourceConfig*
    /// to get the host infrastructure resource information.
    /// 
    /// Network resource pool information for the switch.
    pub async fn network_resource_pool(&self) -> Result<Option<Vec<DvsNetworkResourcePool>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/networkResourcePool", moId = &self.mo_id);
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
    pub async fn overall_status(&self) -> Result<crate::types::enums::ManagedEntityStatusEnum> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/overallStatus", moId = &self.mo_id);
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
    pub async fn parent(&self) -> Result<Option<ManagedObjectReference>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/parent", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of permissions defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<Permission>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/permission", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Portgroups that are defined on the switch.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *DistributedVirtualPortgroup*.
    pub async fn portgroup(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/portgroup", moId = &self.mo_id);
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/recentTask", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Runtime information of the distributed virtual switch.
    pub async fn runtime(&self) -> Result<Option<DvsRuntimeInfo>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/runtime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Summary of the switch.
    pub async fn summary(&self) -> Result<DvsSummary> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/summary", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<Tag>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/tag", moId = &self.mo_id);
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/triggeredAlarmState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Generated UUID of the switch.
    /// 
    /// Unique across vCenter Server
    /// inventory and instances.
    pub async fn uuid(&self) -> Result<String> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/uuid", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::custom_field_value_trait::CustomFieldValueTrait>>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddNetworkResourcePoolRequestType<'a> {
    #[serde(rename = "configSpec")]
    config_spec: &'a [DvsNetworkResourcePoolConfigSpec],
}
#[derive(serde::Serialize)]
#[serde(rename = "CreateDVPortgroupRequestType", tag = "_typeName")]
struct CreateDvPortgroupRequestType<'a> {
    spec: &'a DvPortgroupConfigSpec,
}
#[derive(serde::Serialize)]
#[serde(rename = "AddDVPortgroupRequestType", tag = "_typeName")]
struct AddDvPortgroupRequestType<'a> {
    spec: &'a [DvPortgroupConfigSpec],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EnableNetworkResourceManagementRequestType {
    enable: bool,
}
#[derive(serde::Serialize)]
#[serde(rename = "FetchDVPortKeysRequestType", tag = "_typeName")]
struct FetchDvPortKeysRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    criteria: Option<&'a DistributedVirtualSwitchPortCriteria>,
}
#[derive(serde::Serialize)]
#[serde(rename = "FetchDVPortsRequestType", tag = "_typeName")]
struct FetchDvPortsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    criteria: Option<&'a DistributedVirtualSwitchPortCriteria>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct LookupDvPortGroupRequestType<'a> {
    #[serde(rename = "portgroupKey")]
    portgroup_key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MergeDvsRequestType<'a> {
    dvs: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(rename = "MoveDVPortRequestType", tag = "_typeName")]
struct MoveDvPortRequestType<'a> {
    #[serde(rename = "portKey")]
    port_key: &'a [String],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "destinationPortgroupKey")]
    destination_portgroup_key: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PerformDvsProductSpecOperationRequestType<'a> {
    operation: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "productSpec")]
    product_spec: Option<&'a DistributedVirtualSwitchProductSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigureDvsRequestType<'a> {
    spec: &'a dyn crate::types::dvs_config_spec_trait::DvsConfigSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(rename = "ReconfigureDVPortRequestType", tag = "_typeName")]
struct ReconfigureDvPortRequestType<'a> {
    port: &'a [DvPortConfigSpec],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DvsReconfigureVmVnicNetworkResourcePoolRequestType<'a> {
    #[serde(rename = "configSpec")]
    config_spec: &'a [DvsVmVnicResourcePoolConfigSpec],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RectifyDvsHostRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hosts: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(rename = "RefreshDVPortStateRequestType", tag = "_typeName")]
struct RefreshDvPortStateRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "portKeys")]
    port_keys: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveNetworkResourcePoolRequestType<'a> {
    key: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameRequestType<'a> {
    #[serde(rename = "newName")]
    new_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "DVSRollbackRequestType", tag = "_typeName")]
struct DvsRollbackRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entityBackup")]
    entity_backup: Option<&'a EntityBackupConfig>,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateDvsCapabilityRequestType<'a> {
    capability: &'a DvsCapability,
}
#[derive(serde::Serialize)]
#[serde(rename = "UpdateDVSHealthCheckConfigRequestType", tag = "_typeName")]
struct UpdateDvsHealthCheckConfigRequestType<'a> {
    #[serde(rename = "healthCheckConfig")]
    health_check_config: &'a [Box<dyn crate::types::dvs_health_check_config_trait::DvsHealthCheckConfigTrait>],
}
#[derive(serde::Serialize)]
#[serde(rename = "UpdateDVSLacpGroupConfigRequestType", tag = "_typeName")]
struct UpdateDvsLacpGroupConfigRequestType<'a> {
    #[serde(rename = "lacpGroupSpec")]
    lacp_group_spec: &'a [VMwareDvsLacpGroupSpec],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateNetworkResourcePoolRequestType<'a> {
    #[serde(rename = "configSpec")]
    config_spec: &'a [DvsNetworkResourcePoolConfigSpec],
}
