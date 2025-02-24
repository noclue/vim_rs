use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This class defines healthcheck result of a specified Uplink port
/// in vSphere Distributed Switch.
pub trait HostMemberUplinkHealthCheckResultTrait : super::host_member_health_check_result_trait::HostMemberHealthCheckResultTrait {
    /// The uplink port key.
    fn get_uplink_port_key(&self) -> &str;
}
impl<'s> serde::Serialize for dyn HostMemberUplinkHealthCheckResultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostMemberUplinkHealthCheckResultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostMemberUplinkHealthCheckResultVisitor)
            }
        }

struct HostMemberUplinkHealthCheckResultVisitor;

impl<'de> de::Visitor<'de> for HostMemberUplinkHealthCheckResultVisitor {
    type Value = Box<dyn HostMemberUplinkHealthCheckResultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostMemberUplinkHealthCheckResultTrait JSON object with a _typeName field")
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

impl HostMemberUplinkHealthCheckResultTrait for HostMemberUplinkHealthCheckResult {
    fn get_uplink_port_key(&self) -> &str { &self.uplink_port_key }
}
impl HostMemberUplinkHealthCheckResultTrait for VMwareDvsMtuHealthCheckResult {
    fn get_uplink_port_key(&self) -> &str { &self.uplink_port_key }
}
impl HostMemberUplinkHealthCheckResultTrait for VMwareDvsVlanHealthCheckResult {
    fn get_uplink_port_key(&self) -> &str { &self.uplink_port_key }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostMemberUplinkHealthCheckResultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostMemberUplinkHealthCheckResult => Some(from.as_any_ref().downcast_ref::<HostMemberUplinkHealthCheckResult>()?),
            StructType::VMwareDvsMtuHealthCheckResult => Some(from.as_any_ref().downcast_ref::<VMwareDvsMtuHealthCheckResult>()?),
            StructType::VMwareDvsVlanHealthCheckResult => Some(from.as_any_ref().downcast_ref::<VMwareDvsVlanHealthCheckResult>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostMemberUplinkHealthCheckResult => Ok(from.as_any_box().downcast::<HostMemberUplinkHealthCheckResult>()?),
            StructType::VMwareDvsMtuHealthCheckResult => Ok(from.as_any_box().downcast::<VMwareDvsMtuHealthCheckResult>()?),
            StructType::VMwareDvsVlanHealthCheckResult => Ok(from.as_any_box().downcast::<VMwareDvsVlanHealthCheckResult>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
