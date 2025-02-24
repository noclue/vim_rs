use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event records that a host has begun the process of
/// exiting standby mode.
pub trait ExitingStandbyModeEventTrait : super::host_event_trait::HostEventTrait {
}
impl<'s> serde::Serialize for dyn ExitingStandbyModeEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ExitingStandbyModeEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ExitingStandbyModeEventVisitor)
            }
        }

struct ExitingStandbyModeEventVisitor;

impl<'de> de::Visitor<'de> for ExitingStandbyModeEventVisitor {
    type Value = Box<dyn ExitingStandbyModeEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ExitingStandbyModeEventTrait JSON object with a _typeName field")
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

impl ExitingStandbyModeEventTrait for ExitingStandbyModeEvent {
}
impl ExitingStandbyModeEventTrait for DrsExitingStandbyModeEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ExitingStandbyModeEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ExitingStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<ExitingStandbyModeEvent>()?),
            StructType::DrsExitingStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<DrsExitingStandbyModeEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ExitingStandbyModeEvent => Ok(from.as_any_box().downcast::<ExitingStandbyModeEvent>()?),
            StructType::DrsExitingStandbyModeEvent => Ok(from.as_any_box().downcast::<DrsExitingStandbyModeEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
