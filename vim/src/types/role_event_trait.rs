use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event records a role operation.
pub trait RoleEventTrait : super::authorization_event_trait::AuthorizationEventTrait {
    /// The associated role.
    fn get_role(&self) -> &RoleEventArgument;
}
impl<'s> serde::Serialize for dyn RoleEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn RoleEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(RoleEventVisitor)
            }
        }

struct RoleEventVisitor;

impl<'de> de::Visitor<'de> for RoleEventVisitor {
    type Value = Box<dyn RoleEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid RoleEventTrait JSON object with a _typeName field")
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

impl RoleEventTrait for RoleEvent {
    fn get_role(&self) -> &RoleEventArgument { &self.role }
}
impl RoleEventTrait for RoleAddedEvent {
    fn get_role(&self) -> &RoleEventArgument { &self.role }
}
impl RoleEventTrait for RoleRemovedEvent {
    fn get_role(&self) -> &RoleEventArgument { &self.role }
}
impl RoleEventTrait for RoleUpdatedEvent {
    fn get_role(&self) -> &RoleEventArgument { &self.role }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn RoleEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::RoleEvent => Some(from.as_any_ref().downcast_ref::<RoleEvent>()?),
            StructType::RoleAddedEvent => Some(from.as_any_ref().downcast_ref::<RoleAddedEvent>()?),
            StructType::RoleRemovedEvent => Some(from.as_any_ref().downcast_ref::<RoleRemovedEvent>()?),
            StructType::RoleUpdatedEvent => Some(from.as_any_ref().downcast_ref::<RoleUpdatedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::RoleEvent => Ok(from.as_any_box().downcast::<RoleEvent>()?),
            StructType::RoleAddedEvent => Ok(from.as_any_box().downcast::<RoleAddedEvent>()?),
            StructType::RoleRemovedEvent => Ok(from.as_any_box().downcast::<RoleRemovedEvent>()?),
            StructType::RoleUpdatedEvent => Ok(from.as_any_box().downcast::<RoleUpdatedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
