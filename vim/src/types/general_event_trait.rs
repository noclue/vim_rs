use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These are general events.
pub trait GeneralEventTrait : super::event_trait::EventTrait {
    /// A short form of the message string, not localized.
    fn get_message(&self) -> &str;
}
impl<'s> serde::Serialize for dyn GeneralEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn GeneralEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(GeneralEventVisitor)
            }
        }

struct GeneralEventVisitor;

impl<'de> de::Visitor<'de> for GeneralEventVisitor {
    type Value = Box<dyn GeneralEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GeneralEventTrait JSON object with a _typeName field")
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

impl GeneralEventTrait for GeneralEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl GeneralEventTrait for ExtendedEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl GeneralEventTrait for GeneralHostErrorEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl GeneralEventTrait for GeneralHostInfoEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl GeneralEventTrait for GeneralHostWarningEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl GeneralEventTrait for GeneralUserEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl GeneralEventTrait for GeneralVmErrorEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl GeneralEventTrait for GeneralVmInfoEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl GeneralEventTrait for GeneralVmWarningEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn GeneralEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::GeneralEvent => Some(from.as_any_ref().downcast_ref::<GeneralEvent>()?),
            StructType::ExtendedEvent => Some(from.as_any_ref().downcast_ref::<ExtendedEvent>()?),
            StructType::GeneralHostErrorEvent => Some(from.as_any_ref().downcast_ref::<GeneralHostErrorEvent>()?),
            StructType::GeneralHostInfoEvent => Some(from.as_any_ref().downcast_ref::<GeneralHostInfoEvent>()?),
            StructType::GeneralHostWarningEvent => Some(from.as_any_ref().downcast_ref::<GeneralHostWarningEvent>()?),
            StructType::GeneralUserEvent => Some(from.as_any_ref().downcast_ref::<GeneralUserEvent>()?),
            StructType::GeneralVmErrorEvent => Some(from.as_any_ref().downcast_ref::<GeneralVmErrorEvent>()?),
            StructType::GeneralVmInfoEvent => Some(from.as_any_ref().downcast_ref::<GeneralVmInfoEvent>()?),
            StructType::GeneralVmWarningEvent => Some(from.as_any_ref().downcast_ref::<GeneralVmWarningEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::GeneralEvent => Ok(from.as_any_box().downcast::<GeneralEvent>()?),
            StructType::ExtendedEvent => Ok(from.as_any_box().downcast::<ExtendedEvent>()?),
            StructType::GeneralHostErrorEvent => Ok(from.as_any_box().downcast::<GeneralHostErrorEvent>()?),
            StructType::GeneralHostInfoEvent => Ok(from.as_any_box().downcast::<GeneralHostInfoEvent>()?),
            StructType::GeneralHostWarningEvent => Ok(from.as_any_box().downcast::<GeneralHostWarningEvent>()?),
            StructType::GeneralUserEvent => Ok(from.as_any_box().downcast::<GeneralUserEvent>()?),
            StructType::GeneralVmErrorEvent => Ok(from.as_any_box().downcast::<GeneralVmErrorEvent>()?),
            StructType::GeneralVmInfoEvent => Ok(from.as_any_box().downcast::<GeneralVmInfoEvent>()?),
            StructType::GeneralVmWarningEvent => Ok(from.as_any_box().downcast::<GeneralVmWarningEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
