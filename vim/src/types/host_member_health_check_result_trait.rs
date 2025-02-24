use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This class defines healthcheck result of the vSphere Distributed Switch.
pub trait HostMemberHealthCheckResultTrait : super::data_object_trait::DataObjectTrait {
    /// The summary of health check result.
    fn get_summary(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn HostMemberHealthCheckResultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostMemberHealthCheckResultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostMemberHealthCheckResultVisitor)
            }
        }

struct HostMemberHealthCheckResultVisitor;

impl<'de> de::Visitor<'de> for HostMemberHealthCheckResultVisitor {
    type Value = Box<dyn HostMemberHealthCheckResultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostMemberHealthCheckResultTrait JSON object with a _typeName field")
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

impl HostMemberHealthCheckResultTrait for HostMemberHealthCheckResult {
    fn get_summary(&self) -> &Option<String> { &self.summary }
}
impl HostMemberHealthCheckResultTrait for HostMemberUplinkHealthCheckResult {
    fn get_summary(&self) -> &Option<String> { &self.summary }
}
impl HostMemberHealthCheckResultTrait for VMwareDvsMtuHealthCheckResult {
    fn get_summary(&self) -> &Option<String> { &self.summary }
}
impl HostMemberHealthCheckResultTrait for VMwareDvsVlanHealthCheckResult {
    fn get_summary(&self) -> &Option<String> { &self.summary }
}
impl HostMemberHealthCheckResultTrait for VMwareDvsTeamingHealthCheckResult {
    fn get_summary(&self) -> &Option<String> { &self.summary }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostMemberHealthCheckResultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostMemberHealthCheckResult => Some(from.as_any_ref().downcast_ref::<HostMemberHealthCheckResult>()?),
            StructType::HostMemberUplinkHealthCheckResult => Some(from.as_any_ref().downcast_ref::<HostMemberUplinkHealthCheckResult>()?),
            StructType::VMwareDvsMtuHealthCheckResult => Some(from.as_any_ref().downcast_ref::<VMwareDvsMtuHealthCheckResult>()?),
            StructType::VMwareDvsVlanHealthCheckResult => Some(from.as_any_ref().downcast_ref::<VMwareDvsVlanHealthCheckResult>()?),
            StructType::VMwareDvsTeamingHealthCheckResult => Some(from.as_any_ref().downcast_ref::<VMwareDvsTeamingHealthCheckResult>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostMemberHealthCheckResult => Ok(from.as_any_box().downcast::<HostMemberHealthCheckResult>()?),
            StructType::HostMemberUplinkHealthCheckResult => Ok(from.as_any_box().downcast::<HostMemberUplinkHealthCheckResult>()?),
            StructType::VMwareDvsMtuHealthCheckResult => Ok(from.as_any_box().downcast::<VMwareDvsMtuHealthCheckResult>()?),
            StructType::VMwareDvsVlanHealthCheckResult => Ok(from.as_any_box().downcast::<VMwareDvsVlanHealthCheckResult>()?),
            StructType::VMwareDvsTeamingHealthCheckResult => Ok(from.as_any_box().downcast::<VMwareDvsTeamingHealthCheckResult>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
