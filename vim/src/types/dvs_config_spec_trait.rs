use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *DVSConfigSpec*
/// data object contains configuration data for a
/// *DistributedVirtualSwitch*.
/// 
/// Use the *DistributedVirtualSwitch.ReconfigureDvs_Task*
/// method to apply the configuration to the
/// switch.
pub trait DvsConfigSpecTrait : super::data_object_trait::DataObjectTrait {
    /// Set of dynamic properties.
    /// 
    /// This property is optional because only the
    /// properties of an object that are unknown to a client will be part of this set.
    /// This property is not readonly just in case we want to send such properties
    /// from a client in the future.
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>>;
    /// The version string of the configuration that this spec is trying to
    /// change.
    /// 
    /// This property is required in reconfiguring a switch
    /// and should be set to the same value as
    /// *DVSConfigInfo.configVersion*.
    /// This property is ignored during switch creation.
    fn get_config_version(&self) -> &Option<String>;
    /// The name of the switch.
    /// 
    /// Must be unique in the parent folder.
    fn get_name(&self) -> &Option<String>;
    /// The number of standalone ports in the switch.
    /// 
    /// Standalone ports are
    /// ports that do not belong to any portgroup. If set to a number larger
    /// than number of existing standalone ports in the switch, new ports get
    /// created to meet the number. If set to a number smaller than the number
    /// of existing standalone ports, free ports (uplink ports excluded) are
    /// deleted to meet the number. If the set number cannot be met by
    /// deleting free standalone ports, a fault is raised.
    fn get_num_standalone_ports(&self) -> Option<i32>;
    /// Deprecated as of vSphere API 5.0
    /// The default value of this propoerty is maxint and there is no reason
    /// for users to change it to a lower value.
    /// 
    /// The maximum number of DistributedVirtualPorts allowed in the switch.
    /// 
    /// If specified in a reconfigure operation, this number cannot be smaller
    /// than the number of existing DistributedVirtualPorts.
    fn get_max_ports(&self) -> Option<i32>;
    /// The uplink port policy.
    fn get_uplink_port_policy(&self) -> &Option<Box<dyn super::dvs_uplink_port_policy_trait::DvsUplinkPortPolicyTrait>>;
    /// The uplink portgroups.
    /// 
    /// Refers instances of *DistributedVirtualPortgroup*.
    fn get_uplink_portgroup(&self) -> &Option<Vec<ManagedObjectReference>>;
    /// The default configuration for ports.
    fn get_default_port_config(&self) -> &Option<Box<dyn super::dv_port_setting_trait::DvPortSettingTrait>>;
    /// The host member specification.
    /// 
    /// A particular host should have only one entry
    /// in this array. Duplicate entries for the same host will raise a fault.
    /// The host version should be compatible with the version of
    /// *DistributedVirtualSwitch*. Use
    /// *DistributedVirtualSwitchManager.QueryDvsCheckCompatibility*
    /// to check for compatibility.
    fn get_host(&self) -> &Option<Vec<DistributedVirtualSwitchHostMemberConfigSpec>>;
    /// The key of the extension registered by a remote server that
    /// controls the switch.
    fn get_extension_key(&self) -> &Option<String>;
    /// Set the description string of the switch.
    fn get_description(&self) -> &Option<String>;
    /// The usage policy of the switch.
    fn get_policy(&self) -> &Option<DvsPolicy>;
    /// Set the opaque blob that stores vendor specific configuration.
    fn get_vendor_specific_config(&self) -> &Option<Vec<DistributedVirtualSwitchKeyedOpaqueBlob>>;
    /// Set the human operator contact information.
    fn get_contact(&self) -> &Option<DvsContactInfo>;
    /// IP address for the switch, specified using IPv4 dot notation.
    /// 
    /// IPv6 address is not supported for this property.
    /// The utility of this address is defined by other switch features.
    /// switchIpAddress would be ignored when IPFIX collector uses IPv6.
    fn get_switch_ip_address(&self) -> &Option<String>;
    /// The default host proxy switch maximum port number
    fn get_default_proxy_switch_max_num_ports(&self) -> Option<i32>;
    /// The host infrastructure traffic resource allocation specification.
    /// 
    /// Only the traffic class resource allocations identified in the list
    /// will be updated. The other traffic class resource allocations that are not
    /// specified will not change.
    fn get_infrastructure_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>>;
    /// The dynamic host infrastructure traffic resource allocation
    /// specification.
    fn get_net_resource_pool_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>>;
    /// Indicates the Network Resource Control APIs that are supported on the switch.
    /// 
    /// Possible value can be of
    /// *DistributedVirtualSwitchNetworkResourceControlVersion_enum*.
    fn get_network_resource_control_version(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn DvsConfigSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsConfigSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsConfigSpecVisitor)
            }
        }

struct DvsConfigSpecVisitor;

impl<'de> de::Visitor<'de> for DvsConfigSpecVisitor {
    type Value = Box<dyn DvsConfigSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsConfigSpecTrait JSON object with a _typeName field")
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

impl DvsConfigSpecTrait for DvsConfigSpec {
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>> { &self.dynamic_property }
    fn get_config_version(&self) -> &Option<String> { &self.config_version }
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_num_standalone_ports(&self) -> Option<i32> { self.num_standalone_ports }
    fn get_max_ports(&self) -> Option<i32> { self.max_ports }
    fn get_uplink_port_policy(&self) -> &Option<Box<dyn super::dvs_uplink_port_policy_trait::DvsUplinkPortPolicyTrait>> { &self.uplink_port_policy }
    fn get_uplink_portgroup(&self) -> &Option<Vec<ManagedObjectReference>> { &self.uplink_portgroup }
    fn get_default_port_config(&self) -> &Option<Box<dyn super::dv_port_setting_trait::DvPortSettingTrait>> { &self.default_port_config }
    fn get_host(&self) -> &Option<Vec<DistributedVirtualSwitchHostMemberConfigSpec>> { &self.host }
    fn get_extension_key(&self) -> &Option<String> { &self.extension_key }
    fn get_description(&self) -> &Option<String> { &self.description }
    fn get_policy(&self) -> &Option<DvsPolicy> { &self.policy }
    fn get_vendor_specific_config(&self) -> &Option<Vec<DistributedVirtualSwitchKeyedOpaqueBlob>> { &self.vendor_specific_config }
    fn get_contact(&self) -> &Option<DvsContactInfo> { &self.contact }
    fn get_switch_ip_address(&self) -> &Option<String> { &self.switch_ip_address }
    fn get_default_proxy_switch_max_num_ports(&self) -> Option<i32> { self.default_proxy_switch_max_num_ports }
    fn get_infrastructure_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>> { &self.infrastructure_traffic_resource_config }
    fn get_net_resource_pool_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>> { &self.net_resource_pool_traffic_resource_config }
    fn get_network_resource_control_version(&self) -> &Option<String> { &self.network_resource_control_version }
}
impl DvsConfigSpecTrait for VMwareDvsConfigSpec {
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>> { &self.dynamic_property }
    fn get_config_version(&self) -> &Option<String> { &self.config_version }
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_num_standalone_ports(&self) -> Option<i32> { self.num_standalone_ports }
    fn get_max_ports(&self) -> Option<i32> { self.max_ports }
    fn get_uplink_port_policy(&self) -> &Option<Box<dyn super::dvs_uplink_port_policy_trait::DvsUplinkPortPolicyTrait>> { &self.uplink_port_policy }
    fn get_uplink_portgroup(&self) -> &Option<Vec<ManagedObjectReference>> { &self.uplink_portgroup }
    fn get_default_port_config(&self) -> &Option<Box<dyn super::dv_port_setting_trait::DvPortSettingTrait>> { &self.default_port_config }
    fn get_host(&self) -> &Option<Vec<DistributedVirtualSwitchHostMemberConfigSpec>> { &self.host }
    fn get_extension_key(&self) -> &Option<String> { &self.extension_key }
    fn get_description(&self) -> &Option<String> { &self.description }
    fn get_policy(&self) -> &Option<DvsPolicy> { &self.policy }
    fn get_vendor_specific_config(&self) -> &Option<Vec<DistributedVirtualSwitchKeyedOpaqueBlob>> { &self.vendor_specific_config }
    fn get_contact(&self) -> &Option<DvsContactInfo> { &self.contact }
    fn get_switch_ip_address(&self) -> &Option<String> { &self.switch_ip_address }
    fn get_default_proxy_switch_max_num_ports(&self) -> Option<i32> { self.default_proxy_switch_max_num_ports }
    fn get_infrastructure_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>> { &self.infrastructure_traffic_resource_config }
    fn get_net_resource_pool_traffic_resource_config(&self) -> &Option<Vec<DvsHostInfrastructureTrafficResource>> { &self.net_resource_pool_traffic_resource_config }
    fn get_network_resource_control_version(&self) -> &Option<String> { &self.network_resource_control_version }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsConfigSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsConfigSpec => Some(from.as_any_ref().downcast_ref::<DvsConfigSpec>()?),
            StructType::VMwareDvsConfigSpec => Some(from.as_any_ref().downcast_ref::<VMwareDvsConfigSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsConfigSpec => Ok(from.as_any_box().downcast::<DvsConfigSpec>()?),
            StructType::VMwareDvsConfigSpec => Ok(from.as_any_box().downcast::<VMwareDvsConfigSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
