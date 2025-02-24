use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *DVPortSetting* data object
/// describes the network configuration of a *DistributedVirtualPort*.
pub trait DvPortSettingTrait : super::data_object_trait::DataObjectTrait {
    /// Indicates whether this port is blocked.
    /// 
    /// If a port is blocked,
    /// packet forwarding is stopped.
    fn get_blocked(&self) -> &Option<BoolPolicy>;
    /// Deprecated as of vSphere API 8.0. VMDirectPath Gen 2 is no longer supported and
    /// there is no replacement.
    /// 
    /// Indicates whether this port is allowed to do VMDirectPath Gen2 network passthrough.
    /// 
    /// Direct path capability is defined at host, switch, and device levels.
    /// See the <code>vmDirectPathGen2Supported</code> properties on the
    /// *DVSFeatureCapability*,
    /// *HostCapability*, *PhysicalNic*,
    /// and *VirtualEthernetCardOption* objects.
    fn get_vm_direct_path_gen_2_allowed(&self) -> &Option<BoolPolicy>;
    /// Network shaping policy for controlling throughput of inbound traffic.
    fn get_in_shaping_policy(&self) -> &Option<DvsTrafficShapingPolicy>;
    /// Network shaping policy for controlling throughput of outbound traffic.
    fn get_out_shaping_policy(&self) -> &Option<DvsTrafficShapingPolicy>;
    /// Opaque binary blob that stores vendor specific configuration.
    fn get_vendor_specific_config(&self) -> &Option<DvsVendorSpecificConfig>;
    /// Deprecated as of vSphere API 6.0
    /// Use *DVPortgroupConfigInfo.vmVnicNetworkResourcePoolKey* instead
    /// to reference the virtual NIC network resource pool.
    /// 
    /// The key of user defined network resource pool to be associated with a port.
    /// 
    /// The default value for this property is "-1", indicating that
    /// this port is not associated with any network resource pool.
    fn get_network_resource_pool_key(&self) -> &Option<StringPolicy>;
    /// Configuration for Network Filter Policy.
    fn get_filter_policy(&self) -> &Option<DvsFilterPolicy>;
}
impl<'s> serde::Serialize for dyn DvPortSettingTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvPortSettingTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvPortSettingVisitor)
            }
        }

struct DvPortSettingVisitor;

impl<'de> de::Visitor<'de> for DvPortSettingVisitor {
    type Value = Box<dyn DvPortSettingTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvPortSettingTrait JSON object with a _typeName field")
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

impl DvPortSettingTrait for DvPortSetting {
    fn get_blocked(&self) -> &Option<BoolPolicy> { &self.blocked }
    fn get_vm_direct_path_gen_2_allowed(&self) -> &Option<BoolPolicy> { &self.vm_direct_path_gen_2_allowed }
    fn get_in_shaping_policy(&self) -> &Option<DvsTrafficShapingPolicy> { &self.in_shaping_policy }
    fn get_out_shaping_policy(&self) -> &Option<DvsTrafficShapingPolicy> { &self.out_shaping_policy }
    fn get_vendor_specific_config(&self) -> &Option<DvsVendorSpecificConfig> { &self.vendor_specific_config }
    fn get_network_resource_pool_key(&self) -> &Option<StringPolicy> { &self.network_resource_pool_key }
    fn get_filter_policy(&self) -> &Option<DvsFilterPolicy> { &self.filter_policy }
}
impl DvPortSettingTrait for VMwareDvsPortSetting {
    fn get_blocked(&self) -> &Option<BoolPolicy> { &self.blocked }
    fn get_vm_direct_path_gen_2_allowed(&self) -> &Option<BoolPolicy> { &self.vm_direct_path_gen_2_allowed }
    fn get_in_shaping_policy(&self) -> &Option<DvsTrafficShapingPolicy> { &self.in_shaping_policy }
    fn get_out_shaping_policy(&self) -> &Option<DvsTrafficShapingPolicy> { &self.out_shaping_policy }
    fn get_vendor_specific_config(&self) -> &Option<DvsVendorSpecificConfig> { &self.vendor_specific_config }
    fn get_network_resource_pool_key(&self) -> &Option<StringPolicy> { &self.network_resource_pool_key }
    fn get_filter_policy(&self) -> &Option<DvsFilterPolicy> { &self.filter_policy }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvPortSettingTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvPortSetting => Some(from.as_any_ref().downcast_ref::<DvPortSetting>()?),
            StructType::VMwareDvsPortSetting => Some(from.as_any_ref().downcast_ref::<VMwareDvsPortSetting>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvPortSetting => Ok(from.as_any_box().downcast::<DvPortSetting>()?),
            StructType::VMwareDvsPortSetting => Ok(from.as_any_box().downcast::<VMwareDvsPortSetting>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
