use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These events indicate authorization events.
pub trait AuthorizationEventTrait : super::event_trait::EventTrait {
}
impl<'s> serde::Serialize for dyn AuthorizationEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn AuthorizationEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(AuthorizationEventVisitor)
            }
        }

struct AuthorizationEventVisitor;

impl<'de> de::Visitor<'de> for AuthorizationEventVisitor {
    type Value = Box<dyn AuthorizationEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid AuthorizationEventTrait JSON object with a _typeName field")
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

impl AuthorizationEventTrait for AuthorizationEvent {
}
impl AuthorizationEventTrait for PermissionEvent {
}
impl AuthorizationEventTrait for PermissionAddedEvent {
}
impl AuthorizationEventTrait for PermissionRemovedEvent {
}
impl AuthorizationEventTrait for PermissionUpdatedEvent {
}
impl AuthorizationEventTrait for RoleEvent {
}
impl AuthorizationEventTrait for RoleAddedEvent {
}
impl AuthorizationEventTrait for RoleRemovedEvent {
}
impl AuthorizationEventTrait for RoleUpdatedEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn AuthorizationEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::AuthorizationEvent => Some(from.as_any_ref().downcast_ref::<AuthorizationEvent>()?),
            StructType::PermissionEvent => Some(from.as_any_ref().downcast_ref::<PermissionEvent>()?),
            StructType::PermissionAddedEvent => Some(from.as_any_ref().downcast_ref::<PermissionAddedEvent>()?),
            StructType::PermissionRemovedEvent => Some(from.as_any_ref().downcast_ref::<PermissionRemovedEvent>()?),
            StructType::PermissionUpdatedEvent => Some(from.as_any_ref().downcast_ref::<PermissionUpdatedEvent>()?),
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
            StructType::AuthorizationEvent => Ok(from.as_any_box().downcast::<AuthorizationEvent>()?),
            StructType::PermissionEvent => Ok(from.as_any_box().downcast::<PermissionEvent>()?),
            StructType::PermissionAddedEvent => Ok(from.as_any_box().downcast::<PermissionAddedEvent>()?),
            StructType::PermissionRemovedEvent => Ok(from.as_any_box().downcast::<PermissionRemovedEvent>()?),
            StructType::PermissionUpdatedEvent => Ok(from.as_any_box().downcast::<PermissionUpdatedEvent>()?),
            StructType::RoleEvent => Ok(from.as_any_box().downcast::<RoleEvent>()?),
            StructType::RoleAddedEvent => Ok(from.as_any_box().downcast::<RoleAddedEvent>()?),
            StructType::RoleRemovedEvent => Ok(from.as_any_box().downcast::<RoleRemovedEvent>()?),
            StructType::RoleUpdatedEvent => Ok(from.as_any_box().downcast::<RoleUpdatedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
