use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *DVSHealthCheckConfig* data object
/// defines vSphere Distributed Switch health check configuration.
pub trait DvsHealthCheckConfigTrait : super::data_object_trait::DataObjectTrait {
    /// True if enable health check.
    fn get_enable(&self) -> Option<bool>;
    /// Interval of health check, in minutes.
    fn get_interval(&self) -> Option<i32>;
}
impl<'s> serde::Serialize for dyn DvsHealthCheckConfigTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsHealthCheckConfigTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsHealthCheckConfigVisitor)
            }
        }

struct DvsHealthCheckConfigVisitor;

impl<'de> de::Visitor<'de> for DvsHealthCheckConfigVisitor {
    type Value = Box<dyn DvsHealthCheckConfigTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsHealthCheckConfigTrait JSON object with a _typeName field")
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

impl DvsHealthCheckConfigTrait for DvsHealthCheckConfig {
    fn get_enable(&self) -> Option<bool> { self.enable }
    fn get_interval(&self) -> Option<i32> { self.interval }
}
impl DvsHealthCheckConfigTrait for VMwareDvsHealthCheckConfig {
    fn get_enable(&self) -> Option<bool> { self.enable }
    fn get_interval(&self) -> Option<i32> { self.interval }
}
impl DvsHealthCheckConfigTrait for VMwareDvsTeamingHealthCheckConfig {
    fn get_enable(&self) -> Option<bool> { self.enable }
    fn get_interval(&self) -> Option<i32> { self.interval }
}
impl DvsHealthCheckConfigTrait for VMwareDvsVlanMtuHealthCheckConfig {
    fn get_enable(&self) -> Option<bool> { self.enable }
    fn get_interval(&self) -> Option<i32> { self.interval }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsHealthCheckConfigTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsHealthCheckConfig => Some(from.as_any_ref().downcast_ref::<DvsHealthCheckConfig>()?),
            StructType::VMwareDvsHealthCheckConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsHealthCheckConfig>()?),
            StructType::VMwareDvsTeamingHealthCheckConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsTeamingHealthCheckConfig>()?),
            StructType::VMwareDvsVlanMtuHealthCheckConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsVlanMtuHealthCheckConfig>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsHealthCheckConfig => Ok(from.as_any_box().downcast::<DvsHealthCheckConfig>()?),
            StructType::VMwareDvsHealthCheckConfig => Ok(from.as_any_box().downcast::<VMwareDvsHealthCheckConfig>()?),
            StructType::VMwareDvsTeamingHealthCheckConfig => Ok(from.as_any_box().downcast::<VMwareDvsTeamingHealthCheckConfig>()?),
            StructType::VMwareDvsVlanMtuHealthCheckConfig => Ok(from.as_any_box().downcast::<VMwareDvsVlanMtuHealthCheckConfig>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
