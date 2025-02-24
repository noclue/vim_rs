use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// IP Route Configuration.
/// 
/// All IPv4 addresses, subnet addresses, and
/// netmasks are specified as strings using dotted decimal notation.
/// For example, "192.0.2.1".
/// IPv6 addresses are 128-bit addresses represented
/// as eight fields of up to four hexadecimal digits. A colon separates each
/// field (:). For example, 2001:DB8:101::230:6eff:fe04:d9ff. The address can
/// also consist of symbol '::' to represent multiple 16-bit groups of
/// contiguous 0's only once in an address as described in RFC 2373.
pub trait HostIpRouteConfigTrait : super::data_object_trait::DataObjectTrait {
    /// The default gateway address.
    fn get_default_gateway(&self) -> &Option<String>;
    /// The gateway device.
    /// 
    /// This applies to service console gateway only, it
    /// is ignored otherwise.
    fn get_gateway_device(&self) -> &Option<String>;
    /// The default ipv6 gateway address
    fn get_ip_v_6_default_gateway(&self) -> &Option<String>;
    /// The ipv6 gateway device.
    /// 
    /// This applies to service console gateway only, it
    fn get_ip_v_6_gateway_device(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn HostIpRouteConfigTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostIpRouteConfigTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostIpRouteConfigVisitor)
            }
        }

struct HostIpRouteConfigVisitor;

impl<'de> de::Visitor<'de> for HostIpRouteConfigVisitor {
    type Value = Box<dyn HostIpRouteConfigTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostIpRouteConfigTrait JSON object with a _typeName field")
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

impl HostIpRouteConfigTrait for HostIpRouteConfig {
    fn get_default_gateway(&self) -> &Option<String> { &self.default_gateway }
    fn get_gateway_device(&self) -> &Option<String> { &self.gateway_device }
    fn get_ip_v_6_default_gateway(&self) -> &Option<String> { &self.ip_v_6_default_gateway }
    fn get_ip_v_6_gateway_device(&self) -> &Option<String> { &self.ip_v_6_gateway_device }
}
impl HostIpRouteConfigTrait for HostIpRouteConfigSpec {
    fn get_default_gateway(&self) -> &Option<String> { &self.default_gateway }
    fn get_gateway_device(&self) -> &Option<String> { &self.gateway_device }
    fn get_ip_v_6_default_gateway(&self) -> &Option<String> { &self.ip_v_6_default_gateway }
    fn get_ip_v_6_gateway_device(&self) -> &Option<String> { &self.ip_v_6_gateway_device }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostIpRouteConfigTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostIpRouteConfig => Some(from.as_any_ref().downcast_ref::<HostIpRouteConfig>()?),
            StructType::HostIpRouteConfigSpec => Some(from.as_any_ref().downcast_ref::<HostIpRouteConfigSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostIpRouteConfig => Ok(from.as_any_box().downcast::<HostIpRouteConfig>()?),
            StructType::HostIpRouteConfigSpec => Ok(from.as_any_box().downcast::<HostIpRouteConfigSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
