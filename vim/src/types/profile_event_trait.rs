use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event records a Profile specific event.
pub trait ProfileEventTrait : super::event_trait::EventTrait {
    /// Link to the profile to which this event applies
    fn get_profile(&self) -> &ProfileEventArgument;
}
impl<'s> serde::Serialize for dyn ProfileEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ProfileEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ProfileEventVisitor)
            }
        }

struct ProfileEventVisitor;

impl<'de> de::Visitor<'de> for ProfileEventVisitor {
    type Value = Box<dyn ProfileEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ProfileEventTrait JSON object with a _typeName field")
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

impl ProfileEventTrait for ProfileEvent {
    fn get_profile(&self) -> &ProfileEventArgument { &self.profile }
}
impl ProfileEventTrait for ProfileAssociatedEvent {
    fn get_profile(&self) -> &ProfileEventArgument { &self.profile }
}
impl ProfileEventTrait for ProfileChangedEvent {
    fn get_profile(&self) -> &ProfileEventArgument { &self.profile }
}
impl ProfileEventTrait for ProfileCreatedEvent {
    fn get_profile(&self) -> &ProfileEventArgument { &self.profile }
}
impl ProfileEventTrait for ProfileDissociatedEvent {
    fn get_profile(&self) -> &ProfileEventArgument { &self.profile }
}
impl ProfileEventTrait for ProfileReferenceHostChangedEvent {
    fn get_profile(&self) -> &ProfileEventArgument { &self.profile }
}
impl ProfileEventTrait for ProfileRemovedEvent {
    fn get_profile(&self) -> &ProfileEventArgument { &self.profile }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ProfileEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileEvent => Some(from.as_any_ref().downcast_ref::<ProfileEvent>()?),
            StructType::ProfileAssociatedEvent => Some(from.as_any_ref().downcast_ref::<ProfileAssociatedEvent>()?),
            StructType::ProfileChangedEvent => Some(from.as_any_ref().downcast_ref::<ProfileChangedEvent>()?),
            StructType::ProfileCreatedEvent => Some(from.as_any_ref().downcast_ref::<ProfileCreatedEvent>()?),
            StructType::ProfileDissociatedEvent => Some(from.as_any_ref().downcast_ref::<ProfileDissociatedEvent>()?),
            StructType::ProfileReferenceHostChangedEvent => Some(from.as_any_ref().downcast_ref::<ProfileReferenceHostChangedEvent>()?),
            StructType::ProfileRemovedEvent => Some(from.as_any_ref().downcast_ref::<ProfileRemovedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileEvent => Ok(from.as_any_box().downcast::<ProfileEvent>()?),
            StructType::ProfileAssociatedEvent => Ok(from.as_any_box().downcast::<ProfileAssociatedEvent>()?),
            StructType::ProfileChangedEvent => Ok(from.as_any_box().downcast::<ProfileChangedEvent>()?),
            StructType::ProfileCreatedEvent => Ok(from.as_any_box().downcast::<ProfileCreatedEvent>()?),
            StructType::ProfileDissociatedEvent => Ok(from.as_any_box().downcast::<ProfileDissociatedEvent>()?),
            StructType::ProfileReferenceHostChangedEvent => Ok(from.as_any_box().downcast::<ProfileReferenceHostChangedEvent>()?),
            StructType::ProfileRemovedEvent => Ok(from.as_any_box().downcast::<ProfileRemovedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
