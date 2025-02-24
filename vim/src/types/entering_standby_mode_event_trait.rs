use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event records that a host has begun the process of entering
/// standby mode.
/// 
/// All virtual machine operations
/// are blocked, except the following:
/// - MigrateVM
/// - PowerOffVM
/// - SuspendVM
/// - ShutdownGuest
/// - StandbyGuest
pub trait EnteringStandbyModeEventTrait : super::host_event_trait::HostEventTrait {
}
impl<'s> serde::Serialize for dyn EnteringStandbyModeEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn EnteringStandbyModeEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(EnteringStandbyModeEventVisitor)
            }
        }

struct EnteringStandbyModeEventVisitor;

impl<'de> de::Visitor<'de> for EnteringStandbyModeEventVisitor {
    type Value = Box<dyn EnteringStandbyModeEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid EnteringStandbyModeEventTrait JSON object with a _typeName field")
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

impl EnteringStandbyModeEventTrait for EnteringStandbyModeEvent {
}
impl EnteringStandbyModeEventTrait for DrsEnteringStandbyModeEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn EnteringStandbyModeEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::EnteringStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<EnteringStandbyModeEvent>()?),
            StructType::DrsEnteringStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<DrsEnteringStandbyModeEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::EnteringStandbyModeEvent => Ok(from.as_any_box().downcast::<EnteringStandbyModeEvent>()?),
            StructType::DrsEnteringStandbyModeEvent => Ok(from.as_any_box().downcast::<DrsEnteringStandbyModeEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
