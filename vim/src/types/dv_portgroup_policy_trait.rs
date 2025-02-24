use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The DistributedVirtualPortgroup policies.
/// 
/// This field is not applicable
/// when queried directly against an ESX host.
pub trait DvPortgroupPolicyTrait : super::data_object_trait::DataObjectTrait {
    /// Allow the *DVPortSetting.blocked* setting
    /// of an individual port to override the setting in
    /// *DVPortgroupConfigInfo.defaultPortConfig* of
    /// a portgroup.
    fn get_block_override_allowed(&self) -> bool;
    /// Allow the *DVPortSetting.inShapingPolicy* or
    /// *DVPortSetting.outShapingPolicy* settings
    /// of an individual port to override the setting in
    /// *DVPortgroupConfigInfo.defaultPortConfig* of
    /// a portgroup.
    fn get_shaping_override_allowed(&self) -> bool;
    /// Allow the *DVPortSetting.vendorSpecificConfig*
    /// setting of an individual port to override the setting in
    /// *DVPortgroupConfigInfo.defaultPortConfig* of
    /// a portgroup.
    fn get_vendor_config_override_allowed(&self) -> bool;
    /// Allow a live port to be moved in and out of the portgroup.
    fn get_live_port_moving_allowed(&self) -> bool;
    /// If true, reset the port network setting back to the portgroup setting
    /// (thus removing the per-port setting) when the port is disconnected from
    /// the connectee.
    fn get_port_config_reset_at_disconnect(&self) -> bool;
    /// Allow the setting of
    /// *DVPortSetting.networkResourcePoolKey* of an
    /// individual port to override the setting in
    /// *DVPortgroupConfigInfo.defaultPortConfig*
    /// of a portgroup.
    fn get_network_resource_pool_override_allowed(&self) -> Option<bool>;
    /// Allow the setting of
    /// *DVPortSetting.filterPolicy*,
    /// for an individual port to override the setting in
    /// *DVPortgroupConfigInfo.defaultPortConfig* of
    /// a portgroup.
    fn get_traffic_filter_override_allowed(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn DvPortgroupPolicyTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvPortgroupPolicyTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvPortgroupPolicyVisitor)
            }
        }

struct DvPortgroupPolicyVisitor;

impl<'de> de::Visitor<'de> for DvPortgroupPolicyVisitor {
    type Value = Box<dyn DvPortgroupPolicyTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvPortgroupPolicyTrait JSON object with a _typeName field")
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

impl DvPortgroupPolicyTrait for DvPortgroupPolicy {
    fn get_block_override_allowed(&self) -> bool { self.block_override_allowed }
    fn get_shaping_override_allowed(&self) -> bool { self.shaping_override_allowed }
    fn get_vendor_config_override_allowed(&self) -> bool { self.vendor_config_override_allowed }
    fn get_live_port_moving_allowed(&self) -> bool { self.live_port_moving_allowed }
    fn get_port_config_reset_at_disconnect(&self) -> bool { self.port_config_reset_at_disconnect }
    fn get_network_resource_pool_override_allowed(&self) -> Option<bool> { self.network_resource_pool_override_allowed }
    fn get_traffic_filter_override_allowed(&self) -> Option<bool> { self.traffic_filter_override_allowed }
}
impl DvPortgroupPolicyTrait for VMwareDvsPortgroupPolicy {
    fn get_block_override_allowed(&self) -> bool { self.block_override_allowed }
    fn get_shaping_override_allowed(&self) -> bool { self.shaping_override_allowed }
    fn get_vendor_config_override_allowed(&self) -> bool { self.vendor_config_override_allowed }
    fn get_live_port_moving_allowed(&self) -> bool { self.live_port_moving_allowed }
    fn get_port_config_reset_at_disconnect(&self) -> bool { self.port_config_reset_at_disconnect }
    fn get_network_resource_pool_override_allowed(&self) -> Option<bool> { self.network_resource_pool_override_allowed }
    fn get_traffic_filter_override_allowed(&self) -> Option<bool> { self.traffic_filter_override_allowed }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvPortgroupPolicyTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvPortgroupPolicy => Some(from.as_any_ref().downcast_ref::<DvPortgroupPolicy>()?),
            StructType::VMwareDvsPortgroupPolicy => Some(from.as_any_ref().downcast_ref::<VMwareDvsPortgroupPolicy>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvPortgroupPolicy => Ok(from.as_any_box().downcast::<DvPortgroupPolicy>()?),
            StructType::VMwareDvsPortgroupPolicy => Ok(from.as_any_box().downcast::<VMwareDvsPortgroupPolicy>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
