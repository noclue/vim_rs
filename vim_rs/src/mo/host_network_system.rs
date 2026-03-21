use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type describes networking host configuration and
/// serves as the top level container for relevant networking
/// data objects.
#[derive(Clone)]
pub struct HostNetworkSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostNetworkSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Adds a port group to the virtual switch.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### portgrp
    /// -
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: if the port group already exists.
    /// 
    /// ***NotFound***: if the virtual switch does not exist.
    /// 
    /// ***InvalidArgument***: if the PortGroup vlanId is invalid. Valid vlanIds
    /// range from \[0,4095\], where 0 means no vlan tagging. Exception is
    /// also thrown if network policy is invalid.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn add_port_group(&self, portgrp: &crate::types::structs::HostPortGroupSpec) -> Result<()> {
        let input = AddPortGroupRequestType {portgrp, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "AddPortGroup", Some(&input)).await
    }
    /// Adds a virtual service console network adapter.
    /// 
    /// Returns the device of the
    /// VirtualNic.
    /// 
    /// IP configuration is required although it does not have to be enabled
    /// if the host is an ESX Server system.
    /// The dynamic privilege check will ensure that users have Host.Config.Network
    /// privilege on the host, and Network.Assign privilege on the connecting
    /// DVPortGroup, or DVS if connecting to a standalone DVPort.
    /// Network.Assign privilege is not required for operations on standard network
    /// or for operations performed directly on the host
    /// 
    /// See also *HostNetCapabilities.usesServiceConsoleNic*.
    ///
    /// ## Parameters:
    ///
    /// ### portgroup
    /// -
    ///
    /// ### nic
    /// -
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the IP address or subnet mask in the IP
    /// configuration are invalid or the named PortGroup does not exist.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server system.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn add_service_console_virtual_nic(&self, portgroup: &str, nic: &crate::types::structs::HostVirtualNicSpec) -> Result<String> {
        let input = AddServiceConsoleVirtualNicRequestType {portgroup, nic, };
        let bytes = self.client.invoke("", "HostNetworkSystem", &self.mo_id, "AddServiceConsoleVirtualNic", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Adds a virtual host/VMkernel network adapter.
    /// 
    /// Returns the device of the virtual
    /// network adapter.
    /// 
    /// IP configuration is required although it does not have to be enabled
    /// if the host is an ESX Server system.
    /// The dynamic privilege check will ensure that users have Host.Config.Network
    /// privilege on the host, and Network.Assign privilege on the connecting
    /// DVPortGroup, or DVS if connecting to a standalone DVPort.
    /// Network.Assign privilege is not required for operations on standard network
    /// or for operations performed directly on the host.
    ///
    /// ## Parameters:
    ///
    /// ### portgroup
    /// Note: Must be the empty string in case nic.distributedVirtualPort
    /// is set.
    ///
    /// ### nic
    /// -
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: if the portgroup already has a virtual network
    /// adapter.
    /// 
    /// ***InvalidArgument***: if the IP address or subnet mask in the IP
    /// configuration are invalid. In the case of an ESX Server system, DHCP is
    /// not supported and this exception will be thrown if DHCP is
    /// specified. Exception may also be thrown if the named PortGroup
    /// does not exist.
    /// 
    /// ***InvalidState***: if the an ipv6 address is specified in an ipv4 only
    /// system
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn add_virtual_nic(&self, portgroup: &str, nic: &crate::types::structs::HostVirtualNicSpec) -> Result<String> {
        let input = AddVirtualNicRequestType {portgroup, nic, };
        let bytes = self.client.invoke("", "HostNetworkSystem", &self.mo_id, "AddVirtualNic", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Adds a new virtual switch to the system with the given name.
    /// 
    /// The
    /// name must be unique with respect to other virtual switches on the
    /// host and is limited to 32 characters.
    /// 
    /// See also *HostNetworkSystem.UpdateVirtualSwitch*.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### vswitch_name
    /// -
    ///
    /// ### spec
    /// -
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: if the virtual switch already exists.
    /// 
    /// ***InvalidArgument***: if network vswitchName exceeds the maximum allowed
    /// length, or the number of ports specified falls out of valid range,
    /// or the network policy is invalid, or beacon configuration is invalid.
    /// 
    /// ***ResourceInUse***: if the physical network adapter being bridged
    /// is already in use.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn add_virtual_switch(&self, vswitch_name: &str, spec: Option<&crate::types::structs::HostVirtualSwitchSpec>) -> Result<()> {
        let input = AddVirtualSwitchRequestType {vswitch_name, spec, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "AddVirtualSwitch", Some(&input)).await
    }
    /// Requests network hint information for a physical network adapter.
    /// 
    /// A network hint is
    /// some information about the network to which the physical network
    /// adapter is attached. The method receives in a list of physical
    /// network adapter devices and returns an equal number of hints
    /// if some devices are provided. If the list of devices is empty,
    /// then the method accesses hints for all physical
    /// network adapters.
    /// 
    /// See also *HostNetCapabilities.supportsNetworkHints*, *PhysicalNic.device*.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### device
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if a specified physical network adapter does not exist.
    /// 
    /// ***InvalidArgument***: if the speed and duplexity combination is not valid
    /// for the current link driver.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server system.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn query_network_hint(&self, device: Option<&[String]>) -> Result<Option<Vec<crate::types::structs::PhysicalNicHintInfo>>> {
        let input = QueryNetworkHintRequestType {device, };
        let bytes_opt = self.client.invoke_optional("", "HostNetworkSystem", &self.mo_id, "QueryNetworkHint", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Refresh the network information and settings to pick up any changes
    /// that might have occurred.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    pub async fn refresh_network_system(&self) -> Result<()> {
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "RefreshNetworkSystem", None).await
    }
    /// Removes port group from the virtual switch.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### pg_name
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the port group or virtual switch does not exist.
    /// 
    /// ***ResourceInUse***: if the port group can not be removed because there
    /// are virtual network adapters associated with it.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn remove_port_group(&self, pg_name: &str) -> Result<()> {
        let input = RemovePortGroupRequestType {pg_name, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "RemovePortGroup", Some(&input)).await
    }
    /// Removes a virtual service console network adapter.
    /// 
    /// See also *HostNetCapabilities.usesServiceConsoleNic*.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### device
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the virtual network adapter cannot be found.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server system.
    /// 
    /// ***ResourceInUse***: if the network adapter is currently used
    /// by DHCP DNS.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn remove_service_console_virtual_nic(&self, device: &str) -> Result<()> {
        let input = RemoveServiceConsoleVirtualNicRequestType {device, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "RemoveServiceConsoleVirtualNic", Some(&input)).await
    }
    /// Removes a virtual host/VMkernel network adapter.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### device
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the virtual network adapter cannot be found.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn remove_virtual_nic(&self, device: &str) -> Result<()> {
        let input = RemoveVirtualNicRequestType {device, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "RemoveVirtualNic", Some(&input)).await
    }
    /// Removes an existing virtual switch from the system.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### vswitch_name
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the virtual switch does not exist.
    /// 
    /// ***ResourceInUse***: if there are virtual network adapters associated
    /// with the virtual switch.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn remove_virtual_switch(&self, vswitch_name: &str) -> Result<()> {
        let input = RemoveVirtualSwitchRequestType {vswitch_name, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "RemoveVirtualSwitch", Some(&input)).await
    }
    /// Restart the service console virtual network adapter interface.
    /// 
    /// If the service console virtual network adapter uses DHCP, restarting
    /// the interface may result it with a different IP configuration, or
    /// even fail to be brought up depending on the host system network
    /// configuration.
    /// 
    /// See also *HostNetCapabilities.usesServiceConsoleNic*.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### device
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the virtual network adapter cannot be found.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server system.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn restart_service_console_virtual_nic(&self, device: &str) -> Result<()> {
        let input = RestartServiceConsoleVirtualNicRequestType {device, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "RestartServiceConsoleVirtualNic", Some(&input)).await
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
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "setCustomValue", Some(&input)).await
    }
    /// Launch DPU(Data Processing Unit) failover for a given distributed virtual switch.
    /// 
    /// ***Since:*** vSphere API Release 8.0.3.0
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### dvs_name
    /// The name of the distributed virtual switch.
    ///
    /// ### target_dpu_alias
    /// The alias of the DPU to failover to.
    /// If not set, it will be determined by the system.
    /// At least one vmnic backed by the DPU needs to be associated to the
    /// standby uplink of the distributed virtual switch.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the given distributed virtual switch is not configured
    /// in network offloading mode or the standby DPU is not available.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn start_dpu_failover(&self, dvs_name: &str, target_dpu_alias: Option<&str>) -> Result<()> {
        let input = StartDpuFailoverRequestType {dvs_name, target_dpu_alias, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "startDpuFailover", Some(&input)).await
    }
    /// Applies the IP route configuration for the service console.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// -
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if any of the IP addresses are invalid.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server system.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn update_console_ip_route_config(&self, config: &dyn crate::types::traits::HostIpRouteConfigTrait) -> Result<()> {
        let input = UpdateConsoleIpRouteConfigRequestType {config, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "UpdateConsoleIpRouteConfig", Some(&input)).await
    }
    /// Deprecated as of vSphere API 5.5, which is moved to
    /// each NetStackInstance. This API only works on the default NetStackInstance.
    /// 
    /// Applies the client-side DNS configuration.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// -
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if any of the IP addresses are invalid, or
    /// for a DHCP DNS, if the DHCP virtual network adapter is not specified
    /// or the virtual network adapter specified is not DHCP enabled.
    /// 
    /// ***NotFound***: when the DHCP virtual network adapter specified does
    /// not exist.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server system.
    /// 
    /// ***HostInDomain***: if an attempt is made to change the host or domain name
    /// while the host is part of a Windows domain.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_dns_config(&self, config: &dyn crate::types::traits::HostDnsConfigTrait) -> Result<()> {
        let input = UpdateDnsConfigRequestType {config, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "UpdateDnsConfig", Some(&input)).await
    }
    /// Deprecated as of vSphere API 5.5, which is moved to
    /// each NetStackInstance. This API only works on the default NetStackInstance.
    /// 
    /// Applies the IP route configuration.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// -
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if any of the IP addresses are invalid.
    /// 
    /// ***InvalidState***: if the an ipv6 address is specified in an ipv4 only
    /// system
    /// 
    /// ***NotSupported***: if the host is not an ESX Server system.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_ip_route_config(&self, config: &dyn crate::types::traits::HostIpRouteConfigTrait) -> Result<()> {
        let input = UpdateIpRouteConfigRequestType {config, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "UpdateIpRouteConfig", Some(&input)).await
    }
    /// Deprecated as of vSphere API 5.5, which is moved to
    /// each NetStackInstance. This API only works on the default NetStackInstance.
    /// 
    /// Applies the IP route table configuration.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// -
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if any of the IP addresses are invalid.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server system.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    pub async fn update_ip_route_table_config(&self, config: &crate::types::structs::HostIpRouteTableConfig) -> Result<()> {
        let input = UpdateIpRouteTableConfigRequestType {config, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "UpdateIpRouteTableConfig", Some(&input)).await
    }
    /// Applies the network configuration.
    /// 
    /// This method operates primarily
    /// in two modes: **replace** or **modify** mode.
    /// 
    /// **replace**  
    /// When called in **replace** mode, this method applies the fully
    /// specified networking configuration to the networking system.
    /// 
    /// Upon successful completion of the call, the state of networking will
    /// match the configuration specified in **config**. In general, objects
    /// are created or destroyed to match the elements in the array of
    /// configurations. The identifier field in each element in an array of
    /// configurations is used to match an existing network entity.
    /// The state of existing network entities is patched to match that
    /// of the configuration.
    /// 
    /// An exception to this approach applies to the array of PhysicalNic.Config
    /// objects. The cardinality of physical network adapters cannot be
    /// changed through this
    /// operation. Thus, the identifier of every element in the array must match an
    /// existing PhysicalNic. If there are fewer elements in the array than
    /// there are existing PhysicalNics, then no change is made on the
    /// unreferenced PhysicalNic objects.
    /// 
    /// If the call fails, the networking error is returned as an exception
    /// and the state of networking reverts to the state prior to the start
    /// of the call.
    /// 
    /// **modify**
    /// When called in **modify** mode, only changes that are specified are
    /// made. For singleton entities like DnsConfig, the state is
    /// changed only if the data object is set. For array elements, there is
    /// an Operation field that indicates if the element should be added,
    /// removed, or edited. In the case of editing or removal, the entity
    /// must exist or an exception is thrown. In the case of adding, a
    /// specification needs to be provided.
    /// 
    /// It returns device names of vmkernel and service console virtual network
    /// adapter added to the system.
    /// 
    /// Currently, the only mode that is implemented is incremental mode.
    /// Only add operations are supported for instances. Singleton
    /// configuration is not supported.
    /// The dynamic privilege check will ensure that users have Host.Config.Network
    /// privilege on the host, and Network.Assign privilege on the connecting
    /// DVPortGroup, or DVS if connecting to a standalone DVPort.
    /// Network.Assign privilege is not required for operations on standard network
    /// or for operations performed directly on the host
    /// 
    /// See also *HostConfigChangeMode_enum*.
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// -
    ///
    /// ### change_mode
    /// -
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: when a network entity specified in the configuration
    /// already exists.
    /// 
    /// ***NotFound***: when a network entity specified in the configuration
    /// already exists.
    /// 
    /// ***InvalidArgument***: if an invalid parameter is passed in for one
    /// of the networking objects.
    /// 
    /// ***NotSupported***: if modify mode is not used, a remove or set
    /// operation is specified for an instance, or a singleton entity
    /// is configured.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn update_network_config(&self, config: &crate::types::structs::HostNetworkConfig, change_mode: &str) -> Result<crate::types::structs::HostNetworkConfigResult> {
        let input = UpdateNetworkConfigRequestType {config, change_mode, };
        let bytes = self.client.invoke("", "HostNetworkSystem", &self.mo_id, "UpdateNetworkConfig", Some(&input)).await?;
        let result: crate::types::structs::HostNetworkConfigResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Configures link speed and duplexity.
    /// 
    /// If linkSpeed is not specified,
    /// physical network adapter will be set to autonegotiate.
    /// 
    /// See also *HostNetCapabilities.canSetPhysicalNicLinkSpeed*.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### device
    /// -
    ///
    /// ### link_speed
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the physical network adapter does not exist.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server system.
    /// 
    /// ***InvalidArgument***: if the speed and duplexity is not one of the valid
    /// configurations.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn update_physical_nic_link_speed(&self, device: &str, link_speed: Option<&crate::types::structs::PhysicalNicLinkInfo>) -> Result<()> {
        let input = UpdatePhysicalNicLinkSpeedRequestType {device, link_speed, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "UpdatePhysicalNicLinkSpeed", Some(&input)).await
    }
    /// Reconfigures a port group on the virtual switch.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### pg_name
    /// -
    ///
    /// ### portgrp
    /// -
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: if the update causes the port group to conflict
    /// with an existing port group.
    /// 
    /// ***NotFound***: if the port group or virtual switch does not exist.
    /// 
    /// ***InvalidArgument***: if the PortGroup vlanId is invalid. Valid vlanIds
    /// range from \[0,4095\], where 0 means no vlan tagging. Exception is
    /// also thrown if network policy is invalid.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn update_port_group(&self, pg_name: &str, portgrp: &crate::types::structs::HostPortGroupSpec) -> Result<()> {
        let input = UpdatePortGroupRequestType {pg_name, portgrp, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "UpdatePortGroup", Some(&input)).await
    }
    /// Configures the IP configuration for a virtual service console network
    /// adapter.
    /// 
    /// IP configuration is required although it does not have to be enabled
    /// if the host is an ESX Server system.
    /// The dynamic privilege check will check that the users
    /// have Network.Assign privilege on the DVPortGroup
    /// or the DVS if the port resides on a DVPortGroup or is a stand-alone DVS port.
    /// 
    /// See also *HostNetCapabilities.usesServiceConsoleNic*.
    ///
    /// ## Parameters:
    ///
    /// ### device
    /// -
    ///
    /// ### nic
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the virtual network adapter cannot be found.
    /// 
    /// ***InvalidArgument***: if the IP address or subnet mask in the IP
    /// configuration are invalid or the named PortGroup does not exist.
    /// 
    /// ***NotSupported***: if the host is not an ESX Server system.
    /// 
    /// ***ResourceInUse***: if tries to turn of DHCP while the network
    /// adapter is currently used by DHCP DNS.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn update_service_console_virtual_nic(&self, device: &str, nic: &crate::types::structs::HostVirtualNicSpec) -> Result<()> {
        let input = UpdateServiceConsoleVirtualNicRequestType {device, nic, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "UpdateServiceConsoleVirtualNic", Some(&input)).await
    }
    /// Configures virtual host/VMkernel network adapter.
    /// 
    /// IP configuration is required although it does not have to be enabled
    /// if the host is an ESX Server system.
    /// The dynamic privilege check will ensure that users have Host.Config.Network
    /// privilege on the host, and Network.Assign privilege on the connecting
    /// DVPortGroup, or DVS if connecting to a standalone DVPort.
    /// Network.Assign privilege is not required for operations on standard network
    /// or for operations performed directly on the host.
    ///
    /// ## Parameters:
    ///
    /// ### device
    /// -
    ///
    /// ### nic
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the virtual network adapter cannot be found.
    /// 
    /// ***InvalidArgument***: if the IP address or subnet mask in the IP
    /// configuration are invalid. In the case of an ESX Server
    /// system, DHCP is
    /// not supported and this exception is thrown if DHCP is
    /// specified. Exception may also be thrown if the named PortGroup
    /// does not exist.
    /// 
    /// ***InvalidState***: if the an ipv6 address is specified in an ipv4 only
    /// system
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn update_virtual_nic(&self, device: &str, nic: &crate::types::structs::HostVirtualNicSpec) -> Result<()> {
        let input = UpdateVirtualNicRequestType {device, nic, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "UpdateVirtualNic", Some(&input)).await
    }
    /// Updates the properties of the virtual switch.
    /// 
    /// If the bridge is NULL, the configuration will be unset.
    /// 
    /// If a network adapter is listed in the active or standby list, then
    /// changing the set of network adapters to which the physical network
    /// adapter is associated may have a side effect of changing the network
    /// adapter order policy. If a network adapter is removed from
    /// the bridge configuration, then the network adapter is removed
    /// from the network
    /// adapter teaming order.
    /// 
    /// The BondBridge configuration is the only valid bridge configuration for
    /// an ESX Server system.
    /// 
    /// See also *HostNicOrderPolicy*.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    ///
    /// ## Parameters:
    ///
    /// ### vswitch_name
    /// -
    ///
    /// ### spec
    /// -
    ///
    /// ## Errors:
    ///
    /// ***ResourceInUse***: if the physical network adapter being bridged is
    /// already in use.
    /// 
    /// ***NotFound***: if the virtual switch does not exist.
    /// 
    /// ***InvalidArgument***: if the bridge parameter is bad or the network policy
    /// is invalid or does not exist or the number of ports specified falls
    /// out of valid range, or the beacon configuration is invalid.
    /// 
    /// ***NotSupported***: if network adapter teaming policy is set but
    /// is not supported.
    /// 
    /// ***HostConfigFault***: for all other configuration failures.
    /// 
    /// ***MethodDisabled***: if the host's current flavor of partial
    /// maintenance mode has been configured to block this operation.
    pub async fn update_virtual_switch(&self, vswitch_name: &str, spec: &crate::types::structs::HostVirtualSwitchSpec) -> Result<()> {
        let input = UpdateVirtualSwitchRequestType {vswitch_name, spec, };
        self.client.invoke_void("", "HostNetworkSystem", &self.mo_id, "UpdateVirtualSwitch", Some(&input)).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostNetworkSystem", &self.mo_id, "availableField").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Capability vector indicating the available product features.
    pub async fn capabilities(&self) -> Result<Option<crate::types::structs::HostNetCapabilities>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostNetworkSystem", &self.mo_id, "capabilities").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// IP route configuration for the service console.
    /// 
    /// The IP route
    /// configuration is global to the entire host. This property is
    /// set only if
    /// IP routing can be configured for the service console.
    pub async fn console_ip_route_config(&self) -> Result<Option<Box<dyn crate::types::traits::HostIpRouteConfigTrait>>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostNetworkSystem", &self.mo_id, "consoleIpRouteConfig").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Deprecated as of vSphere API 5.5, which is moved to
    /// each NetStackInstance. This only works on the default NetStackInstance.
    /// 
    /// Client-side DNS configuration.
    pub async fn dns_config(&self) -> Result<Option<Box<dyn crate::types::traits::HostDnsConfigTrait>>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostNetworkSystem", &self.mo_id, "dnsConfig").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Deprecated as of vSphere API 5.5, which is moved to
    /// each NetStackInstance. This only works on the default NetStackInstance.
    /// 
    /// The IP route configuration.
    pub async fn ip_route_config(&self) -> Result<Option<Box<dyn crate::types::traits::HostIpRouteConfigTrait>>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostNetworkSystem", &self.mo_id, "ipRouteConfig").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Network configuration information.
    /// 
    /// This information can be applied
    /// using the *updateNetworkConfig()* method. The
    /// information is a strict subset of the information available in NetworkInfo.
    /// 
    /// See also *HostNetworkInfo*.
    pub async fn network_config(&self) -> Result<Option<crate::types::structs::HostNetworkConfig>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostNetworkSystem", &self.mo_id, "networkConfig").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// The network configuration and runtime information.
    pub async fn network_info(&self) -> Result<Option<crate::types::structs::HostNetworkInfo>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostNetworkSystem", &self.mo_id, "networkInfo").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Deprecated as of VI API 4.0, the system defaults will be used.
    /// 
    /// The offload capabilities available on this server.
    pub async fn offload_capabilities(&self) -> Result<Option<crate::types::structs::HostNetOffloadCapabilities>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostNetworkSystem", &self.mo_id, "offloadCapabilities").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.fetch_property_raw("", "HostNetworkSystem", &self.mo_id, "value").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct AddPortGroupRequestType<'a> {
    portgrp: &'a crate::types::structs::HostPortGroupSpec,
}

impl<'a> miniserde::Serialize for AddPortGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddPortGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddPortGroupRequestTypeSer<'b, 'a> {
    data: &'b AddPortGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddPortGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddPortGroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("portgrp"), &self.data.portgrp as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AddServiceConsoleVirtualNicRequestType<'a> {
    portgroup: &'a str,
    nic: &'a crate::types::structs::HostVirtualNicSpec,
}

impl<'a> miniserde::Serialize for AddServiceConsoleVirtualNicRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddServiceConsoleVirtualNicRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddServiceConsoleVirtualNicRequestTypeSer<'b, 'a> {
    data: &'b AddServiceConsoleVirtualNicRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddServiceConsoleVirtualNicRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddServiceConsoleVirtualNicRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("portgroup"), &self.data.portgroup as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("nic"), &self.data.nic as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AddVirtualNicRequestType<'a> {
    portgroup: &'a str,
    nic: &'a crate::types::structs::HostVirtualNicSpec,
}

impl<'a> miniserde::Serialize for AddVirtualNicRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddVirtualNicRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddVirtualNicRequestTypeSer<'b, 'a> {
    data: &'b AddVirtualNicRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddVirtualNicRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddVirtualNicRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("portgroup"), &self.data.portgroup as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("nic"), &self.data.nic as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AddVirtualSwitchRequestType<'a> {
    vswitch_name: &'a str,
    spec: Option<&'a crate::types::structs::HostVirtualSwitchSpec>,
}

impl<'a> miniserde::Serialize for AddVirtualSwitchRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddVirtualSwitchRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddVirtualSwitchRequestTypeSer<'b, 'a> {
    data: &'b AddVirtualSwitchRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddVirtualSwitchRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddVirtualSwitchRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vswitchName"), &self.data.vswitch_name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryNetworkHintRequestType<'a> {
    device: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for QueryNetworkHintRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryNetworkHintRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryNetworkHintRequestTypeSer<'b, 'a> {
    data: &'b QueryNetworkHintRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryNetworkHintRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryNetworkHintRequestType")),
                1 => {
                    let Some(ref val) = self.data.device else { continue; };
                    return Some((std::borrow::Cow::Borrowed("device"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemovePortGroupRequestType<'a> {
    pg_name: &'a str,
}

impl<'a> miniserde::Serialize for RemovePortGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemovePortGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemovePortGroupRequestTypeSer<'b, 'a> {
    data: &'b RemovePortGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemovePortGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemovePortGroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("pgName"), &self.data.pg_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveServiceConsoleVirtualNicRequestType<'a> {
    device: &'a str,
}

impl<'a> miniserde::Serialize for RemoveServiceConsoleVirtualNicRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveServiceConsoleVirtualNicRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveServiceConsoleVirtualNicRequestTypeSer<'b, 'a> {
    data: &'b RemoveServiceConsoleVirtualNicRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveServiceConsoleVirtualNicRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveServiceConsoleVirtualNicRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("device"), &self.data.device as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveVirtualNicRequestType<'a> {
    device: &'a str,
}

impl<'a> miniserde::Serialize for RemoveVirtualNicRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveVirtualNicRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveVirtualNicRequestTypeSer<'b, 'a> {
    data: &'b RemoveVirtualNicRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveVirtualNicRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveVirtualNicRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("device"), &self.data.device as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveVirtualSwitchRequestType<'a> {
    vswitch_name: &'a str,
}

impl<'a> miniserde::Serialize for RemoveVirtualSwitchRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveVirtualSwitchRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveVirtualSwitchRequestTypeSer<'b, 'a> {
    data: &'b RemoveVirtualSwitchRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveVirtualSwitchRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveVirtualSwitchRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vswitchName"), &self.data.vswitch_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RestartServiceConsoleVirtualNicRequestType<'a> {
    device: &'a str,
}

impl<'a> miniserde::Serialize for RestartServiceConsoleVirtualNicRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RestartServiceConsoleVirtualNicRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RestartServiceConsoleVirtualNicRequestTypeSer<'b, 'a> {
    data: &'b RestartServiceConsoleVirtualNicRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RestartServiceConsoleVirtualNicRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RestartServiceConsoleVirtualNicRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("device"), &self.data.device as &dyn miniserde::Serialize)),
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
struct StartDpuFailoverRequestType<'a> {
    dvs_name: &'a str,
    target_dpu_alias: Option<&'a str>,
}

impl<'a> miniserde::Serialize for StartDpuFailoverRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(StartDpuFailoverRequestTypeSer { data: self, seq: 0 }))
    }
}

struct StartDpuFailoverRequestTypeSer<'b, 'a> {
    data: &'b StartDpuFailoverRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for StartDpuFailoverRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"startDpuFailoverRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("dvsName"), &self.data.dvs_name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.target_dpu_alias else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targetDpuAlias"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UpdateConsoleIpRouteConfigRequestType<'a> {
    config: &'a dyn crate::types::traits::HostIpRouteConfigTrait,
}

impl<'a> miniserde::Serialize for UpdateConsoleIpRouteConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateConsoleIpRouteConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateConsoleIpRouteConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateConsoleIpRouteConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateConsoleIpRouteConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateConsoleIpRouteConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateDnsConfigRequestType<'a> {
    config: &'a dyn crate::types::traits::HostDnsConfigTrait,
}

impl<'a> miniserde::Serialize for UpdateDnsConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateDnsConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateDnsConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateDnsConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateDnsConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateDnsConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateIpRouteConfigRequestType<'a> {
    config: &'a dyn crate::types::traits::HostIpRouteConfigTrait,
}

impl<'a> miniserde::Serialize for UpdateIpRouteConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateIpRouteConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateIpRouteConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateIpRouteConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateIpRouteConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateIpRouteConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateIpRouteTableConfigRequestType<'a> {
    config: &'a crate::types::structs::HostIpRouteTableConfig,
}

impl<'a> miniserde::Serialize for UpdateIpRouteTableConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateIpRouteTableConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateIpRouteTableConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateIpRouteTableConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateIpRouteTableConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateIpRouteTableConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateNetworkConfigRequestType<'a> {
    config: &'a crate::types::structs::HostNetworkConfig,
    change_mode: &'a str,
}

impl<'a> miniserde::Serialize for UpdateNetworkConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateNetworkConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateNetworkConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateNetworkConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateNetworkConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateNetworkConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("changeMode"), &self.data.change_mode as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdatePhysicalNicLinkSpeedRequestType<'a> {
    device: &'a str,
    link_speed: Option<&'a crate::types::structs::PhysicalNicLinkInfo>,
}

impl<'a> miniserde::Serialize for UpdatePhysicalNicLinkSpeedRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdatePhysicalNicLinkSpeedRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdatePhysicalNicLinkSpeedRequestTypeSer<'b, 'a> {
    data: &'b UpdatePhysicalNicLinkSpeedRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdatePhysicalNicLinkSpeedRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdatePhysicalNicLinkSpeedRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("device"), &self.data.device as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.link_speed else { continue; };
                    return Some((std::borrow::Cow::Borrowed("linkSpeed"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UpdatePortGroupRequestType<'a> {
    pg_name: &'a str,
    portgrp: &'a crate::types::structs::HostPortGroupSpec,
}

impl<'a> miniserde::Serialize for UpdatePortGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdatePortGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdatePortGroupRequestTypeSer<'b, 'a> {
    data: &'b UpdatePortGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdatePortGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdatePortGroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("pgName"), &self.data.pg_name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("portgrp"), &self.data.portgrp as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateServiceConsoleVirtualNicRequestType<'a> {
    device: &'a str,
    nic: &'a crate::types::structs::HostVirtualNicSpec,
}

impl<'a> miniserde::Serialize for UpdateServiceConsoleVirtualNicRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateServiceConsoleVirtualNicRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateServiceConsoleVirtualNicRequestTypeSer<'b, 'a> {
    data: &'b UpdateServiceConsoleVirtualNicRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateServiceConsoleVirtualNicRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateServiceConsoleVirtualNicRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("device"), &self.data.device as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("nic"), &self.data.nic as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateVirtualNicRequestType<'a> {
    device: &'a str,
    nic: &'a crate::types::structs::HostVirtualNicSpec,
}

impl<'a> miniserde::Serialize for UpdateVirtualNicRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateVirtualNicRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateVirtualNicRequestTypeSer<'b, 'a> {
    data: &'b UpdateVirtualNicRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateVirtualNicRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateVirtualNicRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("device"), &self.data.device as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("nic"), &self.data.nic as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateVirtualSwitchRequestType<'a> {
    vswitch_name: &'a str,
    spec: &'a crate::types::structs::HostVirtualSwitchSpec,
}

impl<'a> miniserde::Serialize for UpdateVirtualSwitchRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateVirtualSwitchRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateVirtualSwitchRequestTypeSer<'b, 'a> {
    data: &'b UpdateVirtualSwitchRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateVirtualSwitchRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateVirtualSwitchRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vswitchName"), &self.data.vswitch_name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
