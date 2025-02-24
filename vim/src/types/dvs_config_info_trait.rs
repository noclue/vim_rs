use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Configuration of a *DistributedVirtualSwitch*.
pub trait DvsConfigInfoTrait : super::data_object_trait::DataObjectTrait {
    /// Generated UUID of the switch.
    /// 
    /// Unique across vCenter Server
    /// inventory and instances.
    fn get_uuid(&self) -> &str;
    /// Name of the switch.
    fn get_name(&self) -> &str;
    /// Number of standalone ports in the switch.
    /// 
    /// Standalone ports are
    /// ports that do not belong to any portgroup.
    fn get_num_standalone_ports(&self) -> i32;
    /// Current number of ports, not including conflict ports.
    fn get_num_ports(&self) -> i32;
    /// Maximum number of ports allowed in the switch,
    /// not including conflict ports.
    fn get_max_ports(&self) -> i32;
    /// Uplink port policy.
    fn get_uplink_port_policy(&self) -> &Box<dyn super::dvs_uplink_port_policy_trait::DvsUplinkPortPolicyTrait>;
    /// List of uplink portgroups.
    /// 
    /// When adding host members, the server
    /// uses the *DVSConfigInfo.uplinkPortPolicy* to create a number of
    /// uplink ports for the host. If portgroups are shown here,
    /// those uplink ports will be added to the portgroups, with uplink ports
    /// evenly spread among the portgroups.
    /// 
    /// Refers instances of *DistributedVirtualPortgroup*.
    fn get_uplink_portgroup(&self) -> &Option<Vec<ManagedObjectReference>>;
    /// Default configuration for the ports in the switch, if the port
    /// does not inherit configuration from the parent portgroup or has
    /// its own configuration.
    fn get_default_port_config(&self) -> &Box<dyn super::dv_port_setting_trait::DvPortSettingTrait>;
    /// Hosts that join the switch.
    fn get_host(&self) -> &Option<Vec<DistributedVirtualSwitchHostMember>>;
    /// Vendor, product, and version information for the implementation
    /// module of the switch.
    fn get_product_info(&self) -> &DistributedVirtualSwitchProductSpec;
    /// Intended vendor, product, and version information for the
    /// implementation module of the switch.
    fn get_target_info(&self) -> &Option<DistributedVirtualSwitchProductSpec>;
    /// Key of the extension registered by the remote server that
    /// controls the switch.
    fn get_extension_key(&self) -> &Option<String>;
    /// Opaque binary blob that stores vendor specific configuration.
    fn get_vendor_specific_config(&self) -> &Option<Vec<DistributedVirtualSwitchKeyedOpaqueBlob>>;
    /// Usage policy of the switch.
    fn get_policy(&self) -> &Option<DvsPolicy>;
    /// Description string for the switch.
    fn get_description(&self) -> &Option<String>;
    /// Version string of the configuration.
    fn get_config_version(&self) -> &str;
    /// Human operator contact information.
    fn get_contact(&self) -> &DvsContactInfo;
    /// IP address for the switch, specified using IPv4 dot notation.
    /// 
    /// The
    /// utility of this address is defined by other switch features.
    fn get_switch_ip_address(&self) -> &Option<String>;
    /// Create time of the switch.
    fn get_create_time(&self) -> &str;
    /// Boolean to indicate if network I/O control is enabled on the
    /// switch.
    fn get_network_resource_management_enabled(&self) -> bool;
    /// Default host proxy switch maximum port number
    fn get_default_proxy_switch_max_num_ports(&self) -> Option<i32>;
    /// VDS health check configuration.
    fn get_health_check_config(&self) -> &Option<Vec<Box<dyn super::dvs_health_check_config_trait::DvsHealthCheckConfigTrait>>>;
    /// Host infrastructure traffic class resource configuration.
    fn get_infrastructure_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>>;
    /// Dynamic Host infrastructure traffic class resource configuration.
    fn get_net_resource_pool_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>>;
    /// Network resource control version of the switch.
    /// 
    /// Possible value can be of
    /// *DistributedVirtualSwitchNetworkResourceControlVersion_enum*.
    fn get_network_resource_control_version(&self) -> &Option<String>;
    /// The Virtual NIC network resource pool information for the switch.
    fn get_vm_vnic_network_resource_pool(&self) -> &Option<Vec<DvsVmVnicNetworkResourcePool>>;
    /// The percentage of physical nic link speed
    /// *PhysicalNicLinkInfo.speedMb*
    /// available for infrastructure traffic reservation.
    /// 
    /// If this value is 75, then for a 1Gbps physical nic, only
    /// 750Mbps is allowed for all infrastructure traffic reservations.
    fn get_pnic_capacity_ratio_for_reservation(&self) -> Option<i32>;
}
impl<'s> serde::Serialize for dyn DvsConfigInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsConfigInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsConfigInfoVisitor)
            }
        }

struct DvsConfigInfoVisitor;

impl<'de> de::Visitor<'de> for DvsConfigInfoVisitor {
    type Value = Box<dyn DvsConfigInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsConfigInfoTrait JSON object with a _typeName field")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let deserializer = de::value::MapAccessDeserializer::new(&mut map);
        let any: VimAny = de::Deserialize::deserialize(deserializer)?;
        match any {
            VimAny::Object(obj) => Ok(CastFrom::from_box(obj)
                .map_err(|_| de::Error::custom("Internal error converting to trait type"))?),
            VimAny::Value(value) => Err(de::Error::custom(format!(
                "expected object not wrapped value: {:?}",
                value))),
        }
    }
}

impl DvsConfigInfoTrait for DvsConfigInfo {
    fn get_uuid(&self) -> &str { &self.uuid }
    fn get_name(&self) -> &str { &self.name }
    fn get_num_standalone_ports(&self) -> i32 { self.num_standalone_ports }
    fn get_num_ports(&self) -> i32 { self.num_ports }
    fn get_max_ports(&self) -> i32 { self.max_ports }
    fn get_uplink_port_policy(&self) -> &Box<dyn super::dvs_uplink_port_policy_trait::DvsUplinkPortPolicyTrait> { &self.uplink_port_policy }
    fn get_uplink_portgroup(&self) -> &Option<Vec<ManagedObjectReference>> { &self.uplink_portgroup }
    fn get_default_port_config(&self) -> &Box<dyn super::dv_port_setting_trait::DvPortSettingTrait> { &self.default_port_config }
    fn get_host(&self) -> &Option<Vec<DistributedVirtualSwitchHostMember>> { &self.host }
    fn get_product_info(&self) -> &DistributedVirtualSwitchProductSpec { &self.product_info }
    fn get_target_info(&self) -> &Option<DistributedVirtualSwitchProductSpec> { &self.target_info }
    fn get_extension_key(&self) -> &Option<String> { &self.extension_key }
    fn get_vendor_specific_config(&self) -> &Option<Vec<DistributedVirtualSwitchKeyedOpaqueBlob>> { &self.vendor_specific_config }
    fn get_policy(&self) -> &Option<DvsPolicy> { &self.policy }
    fn get_description(&self) -> &Option<String> { &self.description }
    fn get_config_version(&self) -> &str { &self.config_version }
    fn get_contact(&self) -> &DvsContactInfo { &self.contact }
    fn get_switch_ip_address(&self) -> &Option<String> { &self.switch_ip_address }
    fn get_create_time(&self) -> &str { &self.create_time }
    fn get_network_resource_management_enabled(&self) -> bool { self.network_resource_management_enabled }
    fn get_default_proxy_switch_max_num_ports(&self) -> Option<i32> { self.default_proxy_switch_max_num_ports }
    fn get_health_check_config(&self) -> &Option<Vec<Box<dyn super::dvs_health_check_config_trait::DvsHealthCheckConfigTrait>>> { &self.health_check_config }
    fn get_infrastructure_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>> { &self.infrastructure_traffic_resource_config }
    fn get_net_resource_pool_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>> { &self.net_resource_pool_traffic_resource_config }
    fn get_network_resource_control_version(&self) -> &Option<String> { &self.network_resource_control_version }
    fn get_vm_vnic_network_resource_pool(&self) -> &Option<Vec<DvsVmVnicNetworkResourcePool>> { &self.vm_vnic_network_resource_pool }
    fn get_pnic_capacity_ratio_for_reservation(&self) -> Option<i32> { self.pnic_capacity_ratio_for_reservation }
}
impl DvsConfigInfoTrait for VMwareDvsConfigInfo {
    fn get_uuid(&self) -> &str { &self.uuid }
    fn get_name(&self) -> &str { &self.name }
    fn get_num_standalone_ports(&self) -> i32 { self.num_standalone_ports }
    fn get_num_ports(&self) -> i32 { self.num_ports }
    fn get_max_ports(&self) -> i32 { self.max_ports }
    fn get_uplink_port_policy(&self) -> &Box<dyn super::dvs_uplink_port_policy_trait::DvsUplinkPortPolicyTrait> { &self.uplink_port_policy }
    fn get_uplink_portgroup(&self) -> &Option<Vec<ManagedObjectReference>> { &self.uplink_portgroup }
    fn get_default_port_config(&self) -> &Box<dyn super::dv_port_setting_trait::DvPortSettingTrait> { &self.default_port_config }
    fn get_host(&self) -> &Option<Vec<DistributedVirtualSwitchHostMember>> { &self.host }
    fn get_product_info(&self) -> &DistributedVirtualSwitchProductSpec { &self.product_info }
    fn get_target_info(&self) -> &Option<DistributedVirtualSwitchProductSpec> { &self.target_info }
    fn get_extension_key(&self) -> &Option<String> { &self.extension_key }
    fn get_vendor_specific_config(&self) -> &Option<Vec<DistributedVirtualSwitchKeyedOpaqueBlob>> { &self.vendor_specific_config }
    fn get_policy(&self) -> &Option<DvsPolicy> { &self.policy }
    fn get_description(&self) -> &Option<String> { &self.description }
    fn get_config_version(&self) -> &str { &self.config_version }
    fn get_contact(&self) -> &DvsContactInfo { &self.contact }
    fn get_switch_ip_address(&self) -> &Option<String> { &self.switch_ip_address }
    fn get_create_time(&self) -> &str { &self.create_time }
    fn get_network_resource_management_enabled(&self) -> bool { self.network_resource_management_enabled }
    fn get_default_proxy_switch_max_num_ports(&self) -> Option<i32> { self.default_proxy_switch_max_num_ports }
    fn get_health_check_config(&self) -> &Option<Vec<Box<dyn super::dvs_health_check_config_trait::DvsHealthCheckConfigTrait>>> { &self.health_check_config }
    fn get_infrastructure_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>> { &self.infrastructure_traffic_resource_config }
    fn get_net_resource_pool_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>> { &self.net_resource_pool_traffic_resource_config }
    fn get_network_resource_control_version(&self) -> &Option<String> { &self.network_resource_control_version }
    fn get_vm_vnic_network_resource_pool(&self) -> &Option<Vec<DvsVmVnicNetworkResourcePool>> { &self.vm_vnic_network_resource_pool }
    fn get_pnic_capacity_ratio_for_reservation(&self) -> Option<i32> { self.pnic_capacity_ratio_for_reservation }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsConfigInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsConfigInfo => Some(from.as_any_ref().downcast_ref::<DvsConfigInfo>()?),
            StructType::VMwareDvsConfigInfo => Some(from.as_any_ref().downcast_ref::<VMwareDvsConfigInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsConfigInfo => Ok(from.as_any_box().downcast::<DvsConfigInfo>()?),
            StructType::VMwareDvsConfigInfo => Ok(from.as_any_box().downcast::<VMwareDvsConfigInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
