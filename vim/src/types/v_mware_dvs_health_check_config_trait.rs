use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This class defines health check configuration for
/// VMware vSphere Distributed Switch.
pub trait VMwareDvsHealthCheckConfigTrait : super::dvs_health_check_config_trait::DvsHealthCheckConfigTrait {
}
impl<'s> serde::Serialize for dyn VMwareDvsHealthCheckConfigTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VMwareDvsHealthCheckConfigTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VMwareDvsHealthCheckConfigVisitor)
            }
        }

struct VMwareDvsHealthCheckConfigVisitor;

impl<'de> de::Visitor<'de> for VMwareDvsHealthCheckConfigVisitor {
    type Value = Box<dyn VMwareDvsHealthCheckConfigTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VMwareDvsHealthCheckConfigTrait JSON object with a _typeName field")
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

impl VMwareDvsHealthCheckConfigTrait for VMwareDvsHealthCheckConfig {
}
impl VMwareDvsHealthCheckConfigTrait for VMwareDvsTeamingHealthCheckConfig {
}
impl VMwareDvsHealthCheckConfigTrait for VMwareDvsVlanMtuHealthCheckConfig {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VMwareDvsHealthCheckConfigTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VMwareDvsHealthCheckConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsHealthCheckConfig>()?),
            StructType::VMwareDvsTeamingHealthCheckConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsTeamingHealthCheckConfig>()?),
            StructType::VMwareDvsVlanMtuHealthCheckConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsVlanMtuHealthCheckConfig>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VMwareDvsHealthCheckConfig => Ok(from.as_any_box().downcast::<VMwareDvsHealthCheckConfig>()?),
            StructType::VMwareDvsTeamingHealthCheckConfig => Ok(from.as_any_box().downcast::<VMwareDvsTeamingHealthCheckConfig>()?),
            StructType::VMwareDvsVlanMtuHealthCheckConfig => Ok(from.as_any_box().downcast::<VMwareDvsVlanMtuHealthCheckConfig>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
