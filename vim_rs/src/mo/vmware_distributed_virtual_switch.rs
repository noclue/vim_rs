use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
#[derive(Clone)]
pub struct VmwareDistributedVirtualSwitch {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VmwareDistributedVirtualSwitch {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn add_network_resource_pool(&self, config_spec: &[crate::types::structs::DvsNetworkResourcePoolConfigSpec]) -> Result<()> {
        let input = AddNetworkResourcePoolRequestType {config_spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/AddNetworkResourcePool", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn create_dv_portgroup_task(&self, spec: &crate::types::structs::DvPortgroupConfigSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateDvPortgroupRequestType {spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/CreateDVPortgroup_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn add_dv_portgroup_task(&self, spec: &[crate::types::structs::DvPortgroupConfigSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = AddDvPortgroupRequestType {spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/AddDVPortgroup_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
    pub async fn destroy_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/Destroy_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn fetch_dv_port_keys(&self, criteria: Option<&crate::types::structs::DistributedVirtualSwitchPortCriteria>) -> Result<Option<Vec<String>>> {
        let input = FetchDvPortKeysRequestType {criteria, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/FetchDVPortKeys", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<String>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn fetch_dv_ports(&self, criteria: Option<&crate::types::structs::DistributedVirtualSwitchPortCriteria>) -> Result<Option<Vec<crate::types::structs::DistributedVirtualPort>>> {
        let input = FetchDvPortsRequestType {criteria, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/FetchDVPorts", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::DistributedVirtualPort>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn lookup_dv_port_group(&self, portgroup_key: &str) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = LookupDvPortGroupRequestType {portgroup_key, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/LookupDvPortGroup", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::ManagedObjectReference>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn merge_dvs_task(&self, dvs: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = MergeDvsRequestType {dvs, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/MergeDvs_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn move_dv_port_task(&self, port_key: &[String], destination_portgroup_key: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = MoveDvPortRequestType {port_key, destination_portgroup_key, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/MoveDVPort_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn perform_dvs_product_spec_operation_task(&self, operation: &str, product_spec: Option<&crate::types::structs::DistributedVirtualSwitchProductSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = PerformDvsProductSpecOperationRequestType {operation, product_spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/PerformDvsProductSpecOperation_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Return the used VLAN ID (PVLAN excluded) in the switch.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn query_used_vlan_id_in_dvs(&self) -> Result<Option<Vec<i32>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/QueryUsedVlanIdInDvs", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<i32>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn reconfigure_dvs_task(&self, spec: &dyn crate::types::traits::DvsConfigSpecTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ReconfigureDvsRequestType {spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/ReconfigureDvs_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn reconfigure_dv_port_task(&self, port: &[crate::types::structs::DvPortConfigSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ReconfigureDvPortRequestType {port, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/ReconfigureDVPort_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn dvs_reconfigure_vm_vnic_network_resource_pool_task(&self, config_spec: &[crate::types::structs::DvsVmVnicResourcePoolConfigSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DvsReconfigureVmVnicNetworkResourcePoolRequestType {config_spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/DvsReconfigureVmVnicNetworkResourcePool_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn rectify_dvs_host_task(&self, hosts: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RectifyDvsHostRequestType {hosts, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/RectifyDvsHost_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
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
        self.client.execute_void(req).await
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
        let req = self.client.post_json(&path, &input);
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
    pub async fn rename_task(&self, new_name: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RenameRequestType {new_name, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/Rename_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn dvs_rollback_task(&self, entity_backup: Option<&crate::types::structs::EntityBackupConfig>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DvsRollbackRequestType {entity_backup, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/DVSRollback_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn update_dvs_capability(&self, capability: &crate::types::structs::DvsCapability) -> Result<()> {
        let input = UpdateDvsCapabilityRequestType {capability, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/UpdateDvsCapability", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn update_dvs_health_check_config_task(&self, health_check_config: &[Box<dyn crate::types::traits::DvsHealthCheckConfigTrait>]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UpdateDvsHealthCheckConfigRequestType {health_check_config, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/UpdateDVSHealthCheckConfig_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn update_dvs_lacp_group_config_task(&self, lacp_group_spec: &[crate::types::structs::VMwareDvsLacpGroupSpec]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UpdateDvsLacpGroupConfigRequestType {lacp_group_spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/UpdateDVSLacpGroupConfig_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn update_network_resource_pool(&self, config_spec: &[crate::types::structs::DvsNetworkResourcePoolConfigSpec]) -> Result<()> {
        let input = UpdateNetworkResourcePoolRequestType {config_spec, };
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/UpdateNetworkResourcePool", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/alarmActionsEnabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<bool>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::CustomFieldDef>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Capability of the switch.
    /// 
    /// Capabilities are indicated at the port,
    /// portgroup and switch levels, and for version-specific features.
    /// When you retrieve this property from an ESXi host,
    /// *DistributedVirtualSwitch.capability*.*DVSCapability.dvsOperationSupported*
    /// should always be set to false.
    pub async fn capability(&self) -> Result<crate::types::structs::DvsCapability> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/capability", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::DvsCapability = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Switch configuration data.
    pub async fn config(&self) -> Result<Box<dyn crate::types::traits::DvsConfigInfoTrait>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Box<dyn crate::types::traits::DvsConfigInfoTrait> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/configIssue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::Event>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/configStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::enums::ManagedEntityStatusEnum = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/customValue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/declaredAlarmState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::AlarmState>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/disabledMethod", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<String>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/effectiveRole", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<i32>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/name", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 6.0
    /// Use *DVSConfigInfo.vmVnicNetworkResourcePool*
    /// to get the Virtual NIC resource pool information.
    /// Use *DVSConfigInfo.infrastructureTrafficResourceConfig*
    /// to get the host infrastructure resource information.
    /// 
    /// Network resource pool information for the switch.
    pub async fn network_resource_pool(&self) -> Result<Option<Vec<crate::types::structs::DvsNetworkResourcePool>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/networkResourcePool", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::DvsNetworkResourcePool>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/overallStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::enums::ManagedEntityStatusEnum = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/parent", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::ManagedObjectReference>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// List of permissions defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<crate::types::structs::Permission>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/permission", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::Permission>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Portgroups that are defined on the switch.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *DistributedVirtualPortgroup*.
    pub async fn portgroup(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/portgroup", moId = &self.mo_id);
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/recentTask", moId = &self.mo_id);
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
    /// Runtime information of the distributed virtual switch.
    pub async fn runtime(&self) -> Result<Option<crate::types::structs::DvsRuntimeInfo>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/runtime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::DvsRuntimeInfo>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Summary of the switch.
    pub async fn summary(&self) -> Result<crate::types::structs::DvsSummary> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/summary", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::DvsSummary = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<crate::types::structs::Tag>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/tag", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::Tag>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/triggeredAlarmState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::AlarmState>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Generated UUID of the switch.
    /// 
    /// Unique across vCenter Server
    /// inventory and instances.
    pub async fn uuid(&self) -> Result<String> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/uuid", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let path = format!("/VmwareDistributedVirtualSwitch/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct AddNetworkResourcePoolRequestType<'a> {
    config_spec: &'a [crate::types::structs::DvsNetworkResourcePoolConfigSpec],
}

impl<'a> miniserde::Serialize for AddNetworkResourcePoolRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddNetworkResourcePoolRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddNetworkResourcePoolRequestTypeSer<'b, 'a> {
    data: &'b AddNetworkResourcePoolRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for AddNetworkResourcePoolRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddNetworkResourcePoolRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("configSpec"), &self.data.config_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateDvPortgroupRequestType<'a> {
    spec: &'a crate::types::structs::DvPortgroupConfigSpec,
}

impl<'a> miniserde::Serialize for CreateDvPortgroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateDvPortgroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateDvPortgroupRequestTypeSer<'b, 'a> {
    data: &'b CreateDvPortgroupRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CreateDvPortgroupRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateDVPortgroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AddDvPortgroupRequestType<'a> {
    spec: &'a [crate::types::structs::DvPortgroupConfigSpec],
}

impl<'a> miniserde::Serialize for AddDvPortgroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddDvPortgroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddDvPortgroupRequestTypeSer<'b, 'a> {
    data: &'b AddDvPortgroupRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for AddDvPortgroupRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddDVPortgroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct EnableNetworkResourceManagementRequestType {
    enable: bool,
}

impl miniserde::Serialize for EnableNetworkResourceManagementRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EnableNetworkResourceManagementRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EnableNetworkResourceManagementRequestTypeSer<'b> {
    data: &'b EnableNetworkResourceManagementRequestType,
    seq: usize,
}

impl miniserde::ser::Map for EnableNetworkResourceManagementRequestTypeSer<'_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EnableNetworkResourceManagementRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("enable"), &self.data.enable as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct FetchDvPortKeysRequestType<'a> {
    criteria: Option<&'a crate::types::structs::DistributedVirtualSwitchPortCriteria>,
}

impl<'a> miniserde::Serialize for FetchDvPortKeysRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FetchDvPortKeysRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FetchDvPortKeysRequestTypeSer<'b, 'a> {
    data: &'b FetchDvPortKeysRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for FetchDvPortKeysRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FetchDVPortKeysRequestType")),
                1 => {
                    let Some(ref val) = self.data.criteria else { continue; };
                    return Some((std::borrow::Cow::Borrowed("criteria"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct FetchDvPortsRequestType<'a> {
    criteria: Option<&'a crate::types::structs::DistributedVirtualSwitchPortCriteria>,
}

impl<'a> miniserde::Serialize for FetchDvPortsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FetchDvPortsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FetchDvPortsRequestTypeSer<'b, 'a> {
    data: &'b FetchDvPortsRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for FetchDvPortsRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FetchDVPortsRequestType")),
                1 => {
                    let Some(ref val) = self.data.criteria else { continue; };
                    return Some((std::borrow::Cow::Borrowed("criteria"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct LookupDvPortGroupRequestType<'a> {
    portgroup_key: &'a str,
}

impl<'a> miniserde::Serialize for LookupDvPortGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(LookupDvPortGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct LookupDvPortGroupRequestTypeSer<'b, 'a> {
    data: &'b LookupDvPortGroupRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for LookupDvPortGroupRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"LookupDvPortGroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("portgroupKey"), &self.data.portgroup_key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MergeDvsRequestType<'a> {
    dvs: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for MergeDvsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MergeDvsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MergeDvsRequestTypeSer<'b, 'a> {
    data: &'b MergeDvsRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for MergeDvsRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MergeDvsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("dvs"), &self.data.dvs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MoveDvPortRequestType<'a> {
    port_key: &'a [String],
    destination_portgroup_key: Option<&'a str>,
}

impl<'a> miniserde::Serialize for MoveDvPortRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MoveDvPortRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MoveDvPortRequestTypeSer<'b, 'a> {
    data: &'b MoveDvPortRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for MoveDvPortRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MoveDVPortRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("portKey"), &self.data.port_key as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.destination_portgroup_key else { continue; };
                    return Some((std::borrow::Cow::Borrowed("destinationPortgroupKey"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct PerformDvsProductSpecOperationRequestType<'a> {
    operation: &'a str,
    product_spec: Option<&'a crate::types::structs::DistributedVirtualSwitchProductSpec>,
}

impl<'a> miniserde::Serialize for PerformDvsProductSpecOperationRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PerformDvsProductSpecOperationRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PerformDvsProductSpecOperationRequestTypeSer<'b, 'a> {
    data: &'b PerformDvsProductSpecOperationRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PerformDvsProductSpecOperationRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PerformDvsProductSpecOperationRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("operation"), &self.data.operation as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.product_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("productSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ReconfigureDvsRequestType<'a> {
    spec: &'a dyn crate::types::traits::DvsConfigSpecTrait,
}

impl<'a> miniserde::Serialize for ReconfigureDvsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigureDvsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigureDvsRequestTypeSer<'b, 'a> {
    data: &'b ReconfigureDvsRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ReconfigureDvsRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigureDvsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ReconfigureDvPortRequestType<'a> {
    port: &'a [crate::types::structs::DvPortConfigSpec],
}

impl<'a> miniserde::Serialize for ReconfigureDvPortRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigureDvPortRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigureDvPortRequestTypeSer<'b, 'a> {
    data: &'b ReconfigureDvPortRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ReconfigureDvPortRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigureDVPortRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("port"), &self.data.port as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DvsReconfigureVmVnicNetworkResourcePoolRequestType<'a> {
    config_spec: &'a [crate::types::structs::DvsVmVnicResourcePoolConfigSpec],
}

impl<'a> miniserde::Serialize for DvsReconfigureVmVnicNetworkResourcePoolRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DvsReconfigureVmVnicNetworkResourcePoolRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DvsReconfigureVmVnicNetworkResourcePoolRequestTypeSer<'b, 'a> {
    data: &'b DvsReconfigureVmVnicNetworkResourcePoolRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DvsReconfigureVmVnicNetworkResourcePoolRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DvsReconfigureVmVnicNetworkResourcePoolRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("configSpec"), &self.data.config_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RectifyDvsHostRequestType<'a> {
    hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for RectifyDvsHostRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RectifyDvsHostRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RectifyDvsHostRequestTypeSer<'b, 'a> {
    data: &'b RectifyDvsHostRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RectifyDvsHostRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RectifyDvsHostRequestType")),
                1 => {
                    let Some(ref val) = self.data.hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hosts"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RefreshDvPortStateRequestType<'a> {
    port_keys: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for RefreshDvPortStateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RefreshDvPortStateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RefreshDvPortStateRequestTypeSer<'b, 'a> {
    data: &'b RefreshDvPortStateRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RefreshDvPortStateRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RefreshDVPortStateRequestType")),
                1 => {
                    let Some(ref val) = self.data.port_keys else { continue; };
                    return Some((std::borrow::Cow::Borrowed("portKeys"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveNetworkResourcePoolRequestType<'a> {
    key: &'a [String],
}

impl<'a> miniserde::Serialize for RemoveNetworkResourcePoolRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveNetworkResourcePoolRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveNetworkResourcePoolRequestTypeSer<'b, 'a> {
    data: &'b RemoveNetworkResourcePoolRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RemoveNetworkResourcePoolRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveNetworkResourcePoolRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            _ => return None,
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

impl miniserde::ser::Map for RenameRequestTypeSer<'_, '_> {
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
struct DvsRollbackRequestType<'a> {
    entity_backup: Option<&'a crate::types::structs::EntityBackupConfig>,
}

impl<'a> miniserde::Serialize for DvsRollbackRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DvsRollbackRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DvsRollbackRequestTypeSer<'b, 'a> {
    data: &'b DvsRollbackRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DvsRollbackRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DVSRollbackRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity_backup else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entityBackup"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
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

impl miniserde::ser::Map for SetCustomValueRequestTypeSer<'_, '_> {
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
struct UpdateDvsCapabilityRequestType<'a> {
    capability: &'a crate::types::structs::DvsCapability,
}

impl<'a> miniserde::Serialize for UpdateDvsCapabilityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateDvsCapabilityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateDvsCapabilityRequestTypeSer<'b, 'a> {
    data: &'b UpdateDvsCapabilityRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for UpdateDvsCapabilityRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateDvsCapabilityRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("capability"), &self.data.capability as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateDvsHealthCheckConfigRequestType<'a> {
    health_check_config: &'a [Box<dyn crate::types::traits::DvsHealthCheckConfigTrait>],
}

impl<'a> miniserde::Serialize for UpdateDvsHealthCheckConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateDvsHealthCheckConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateDvsHealthCheckConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateDvsHealthCheckConfigRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for UpdateDvsHealthCheckConfigRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateDVSHealthCheckConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("healthCheckConfig"), &self.data.health_check_config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateDvsLacpGroupConfigRequestType<'a> {
    lacp_group_spec: &'a [crate::types::structs::VMwareDvsLacpGroupSpec],
}

impl<'a> miniserde::Serialize for UpdateDvsLacpGroupConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateDvsLacpGroupConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateDvsLacpGroupConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateDvsLacpGroupConfigRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for UpdateDvsLacpGroupConfigRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateDVSLacpGroupConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("lacpGroupSpec"), &self.data.lacp_group_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateNetworkResourcePoolRequestType<'a> {
    config_spec: &'a [crate::types::structs::DvsNetworkResourcePoolConfigSpec],
}

impl<'a> miniserde::Serialize for UpdateNetworkResourcePoolRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateNetworkResourcePoolRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateNetworkResourcePoolRequestTypeSer<'b, 'a> {
    data: &'b UpdateNetworkResourcePoolRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for UpdateNetworkResourcePoolRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateNetworkResourcePoolRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("configSpec"), &self.data.config_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
