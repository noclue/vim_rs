use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::CustomFieldDef;
use crate::types::CustomFieldValueTrait;
use crate::types::HostDnsConfigTrait;
use crate::types::HostIpRouteConfigTrait;
use crate::types::HostIpRouteTableConfig;
use crate::types::HostNetCapabilities;
use crate::types::HostNetOffloadCapabilities;
use crate::types::HostNetworkConfig;
use crate::types::HostNetworkConfigResult;
use crate::types::HostNetworkInfo;
use crate::types::HostPortGroupSpec;
use crate::types::HostVirtualNicSpec;
use crate::types::HostVirtualSwitchSpec;
use crate::types::PhysicalNicHintInfo;
use crate::types::PhysicalNicLinkInfo;
/// This managed object type describes networking host configuration and
/// serves as the top level container for relevant networking
/// data objects.
pub struct HostNetworkSystem {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HostNetworkSystem {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
    pub async fn add_port_group(&self, portgrp: &HostPortGroupSpec) -> Result<()> {
        let input = AddPortGroupRequestType {portgrp, };
        let path = format!("/HostNetworkSystem/{moId}/AddPortGroup", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn add_service_console_virtual_nic(&self, portgroup: &str, nic: &HostVirtualNicSpec) -> Result<String> {
        let input = AddServiceConsoleVirtualNicRequestType {portgroup, nic, };
        let path = format!("/HostNetworkSystem/{moId}/AddServiceConsoleVirtualNic", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn add_virtual_nic(&self, portgroup: &str, nic: &HostVirtualNicSpec) -> Result<String> {
        let input = AddVirtualNicRequestType {portgroup, nic, };
        let path = format!("/HostNetworkSystem/{moId}/AddVirtualNic", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn add_virtual_switch(&self, vswitch_name: &str, spec: Option<&HostVirtualSwitchSpec>) -> Result<()> {
        let input = AddVirtualSwitchRequestType {vswitch_name, spec, };
        let path = format!("/HostNetworkSystem/{moId}/AddVirtualSwitch", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn query_network_hint(&self, device: Option<&[String]>) -> Result<Option<Vec<PhysicalNicHintInfo>>> {
        let input = QueryNetworkHintRequestType {device, };
        let path = format!("/HostNetworkSystem/{moId}/QueryNetworkHint", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Refresh the network information and settings to pick up any changes
    /// that might have occurred.
    /// 
    /// ***Required privileges:*** Host.Config.Network
    pub async fn refresh_network_system(&self) -> Result<()> {
        let path = format!("/HostNetworkSystem/{moId}/RefreshNetworkSystem", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn remove_port_group(&self, pg_name: &str) -> Result<()> {
        let input = RemovePortGroupRequestType {pg_name, };
        let path = format!("/HostNetworkSystem/{moId}/RemovePortGroup", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn remove_service_console_virtual_nic(&self, device: &str) -> Result<()> {
        let input = RemoveServiceConsoleVirtualNicRequestType {device, };
        let path = format!("/HostNetworkSystem/{moId}/RemoveServiceConsoleVirtualNic", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn remove_virtual_nic(&self, device: &str) -> Result<()> {
        let input = RemoveVirtualNicRequestType {device, };
        let path = format!("/HostNetworkSystem/{moId}/RemoveVirtualNic", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn remove_virtual_switch(&self, vswitch_name: &str) -> Result<()> {
        let input = RemoveVirtualSwitchRequestType {vswitch_name, };
        let path = format!("/HostNetworkSystem/{moId}/RemoveVirtualSwitch", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn restart_service_console_virtual_nic(&self, device: &str) -> Result<()> {
        let input = RestartServiceConsoleVirtualNicRequestType {device, };
        let path = format!("/HostNetworkSystem/{moId}/RestartServiceConsoleVirtualNic", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
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
        let path = format!("/HostNetworkSystem/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_console_ip_route_config(&self, config: &dyn HostIpRouteConfigTrait) -> Result<()> {
        let input = UpdateConsoleIpRouteConfigRequestType {config, };
        let path = format!("/HostNetworkSystem/{moId}/UpdateConsoleIpRouteConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_dns_config(&self, config: &dyn HostDnsConfigTrait) -> Result<()> {
        let input = UpdateDnsConfigRequestType {config, };
        let path = format!("/HostNetworkSystem/{moId}/UpdateDnsConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_ip_route_config(&self, config: &dyn HostIpRouteConfigTrait) -> Result<()> {
        let input = UpdateIpRouteConfigRequestType {config, };
        let path = format!("/HostNetworkSystem/{moId}/UpdateIpRouteConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_ip_route_table_config(&self, config: &HostIpRouteTableConfig) -> Result<()> {
        let input = UpdateIpRouteTableConfigRequestType {config, };
        let path = format!("/HostNetworkSystem/{moId}/UpdateIpRouteTableConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_network_config(&self, config: &HostNetworkConfig, change_mode: &str) -> Result<HostNetworkConfigResult> {
        let input = UpdateNetworkConfigRequestType {config, change_mode, };
        let path = format!("/HostNetworkSystem/{moId}/UpdateNetworkConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn update_physical_nic_link_speed(&self, device: &str, link_speed: Option<&PhysicalNicLinkInfo>) -> Result<()> {
        let input = UpdatePhysicalNicLinkSpeedRequestType {device, link_speed, };
        let path = format!("/HostNetworkSystem/{moId}/UpdatePhysicalNicLinkSpeed", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_port_group(&self, pg_name: &str, portgrp: &HostPortGroupSpec) -> Result<()> {
        let input = UpdatePortGroupRequestType {pg_name, portgrp, };
        let path = format!("/HostNetworkSystem/{moId}/UpdatePortGroup", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_service_console_virtual_nic(&self, device: &str, nic: &HostVirtualNicSpec) -> Result<()> {
        let input = UpdateServiceConsoleVirtualNicRequestType {device, nic, };
        let path = format!("/HostNetworkSystem/{moId}/UpdateServiceConsoleVirtualNic", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_virtual_nic(&self, device: &str, nic: &HostVirtualNicSpec) -> Result<()> {
        let input = UpdateVirtualNicRequestType {device, nic, };
        let path = format!("/HostNetworkSystem/{moId}/UpdateVirtualNic", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_virtual_switch(&self, vswitch_name: &str, spec: &HostVirtualSwitchSpec) -> Result<()> {
        let input = UpdateVirtualSwitchRequestType {vswitch_name, spec, };
        let path = format!("/HostNetworkSystem/{moId}/UpdateVirtualSwitch", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/HostNetworkSystem/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Capability vector indicating the available product features.
    pub async fn capabilities(&self) -> Result<HostNetCapabilities> {
        let path = format!("/HostNetworkSystem/{moId}/capabilities", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// IP route configuration for the service console.
    /// 
    /// The IP route
    /// configuration is global to the entire host. This property is
    /// set only if
    /// IP routing can be configured for the service console.
    pub async fn console_ip_route_config(&self) -> Result<Box<dyn HostIpRouteConfigTrait>> {
        let path = format!("/HostNetworkSystem/{moId}/consoleIpRouteConfig", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 5.5, which is moved to
    /// each NetStackInstance. This only works on the default NetStackInstance.
    /// 
    /// Client-side DNS configuration.
    pub async fn dns_config(&self) -> Result<Box<dyn HostDnsConfigTrait>> {
        let path = format!("/HostNetworkSystem/{moId}/dnsConfig", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 5.5, which is moved to
    /// each NetStackInstance. This only works on the default NetStackInstance.
    /// 
    /// The IP route configuration.
    pub async fn ip_route_config(&self) -> Result<Box<dyn HostIpRouteConfigTrait>> {
        let path = format!("/HostNetworkSystem/{moId}/ipRouteConfig", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Network configuration information.
    /// 
    /// This information can be applied
    /// using the *updateNetworkConfig()* method. The
    /// information is a strict subset of the information available in NetworkInfo.
    /// 
    /// See also *HostNetworkInfo*.
    pub async fn network_config(&self) -> Result<HostNetworkConfig> {
        let path = format!("/HostNetworkSystem/{moId}/networkConfig", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The network configuration and runtime information.
    pub async fn network_info(&self) -> Result<HostNetworkInfo> {
        let path = format!("/HostNetworkSystem/{moId}/networkInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of VI API 4.0, the system defaults will be used.
    /// 
    /// The offload capabilities available on this server.
    pub async fn offload_capabilities(&self) -> Result<HostNetOffloadCapabilities> {
        let path = format!("/HostNetworkSystem/{moId}/offloadCapabilities", moId = &self.mo_id);
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
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn CustomFieldValueTrait>>>> {
        let path = format!("/HostNetworkSystem/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddPortGroupRequestType<'a> {
    portgrp: &'a HostPortGroupSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddServiceConsoleVirtualNicRequestType<'a> {
    portgroup: &'a str,
    nic: &'a HostVirtualNicSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddVirtualNicRequestType<'a> {
    portgroup: &'a str,
    nic: &'a HostVirtualNicSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddVirtualSwitchRequestType<'a> {
    #[serde(rename = "vswitchName")]
    vswitch_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a HostVirtualSwitchSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryNetworkHintRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    device: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemovePortGroupRequestType<'a> {
    #[serde(rename = "pgName")]
    pg_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveServiceConsoleVirtualNicRequestType<'a> {
    device: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveVirtualNicRequestType<'a> {
    device: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveVirtualSwitchRequestType<'a> {
    #[serde(rename = "vswitchName")]
    vswitch_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RestartServiceConsoleVirtualNicRequestType<'a> {
    device: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateConsoleIpRouteConfigRequestType<'a> {
    config: &'a dyn HostIpRouteConfigTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateDnsConfigRequestType<'a> {
    config: &'a dyn HostDnsConfigTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateIpRouteConfigRequestType<'a> {
    config: &'a dyn HostIpRouteConfigTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateIpRouteTableConfigRequestType<'a> {
    config: &'a HostIpRouteTableConfig,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateNetworkConfigRequestType<'a> {
    config: &'a HostNetworkConfig,
    #[serde(rename = "changeMode")]
    change_mode: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdatePhysicalNicLinkSpeedRequestType<'a> {
    device: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "linkSpeed")]
    link_speed: Option<&'a PhysicalNicLinkInfo>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdatePortGroupRequestType<'a> {
    #[serde(rename = "pgName")]
    pg_name: &'a str,
    portgrp: &'a HostPortGroupSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateServiceConsoleVirtualNicRequestType<'a> {
    device: &'a str,
    nic: &'a HostVirtualNicSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateVirtualNicRequestType<'a> {
    device: &'a str,
    nic: &'a HostVirtualNicSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateVirtualSwitchRequestType<'a> {
    #[serde(rename = "vswitchName")]
    vswitch_name: &'a str,
    spec: &'a HostVirtualSwitchSpec,
}
