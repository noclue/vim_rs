use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event records a permission operation.
pub trait PermissionEventTrait : super::authorization_event_trait::AuthorizationEventTrait {
    /// The entity to which the permission applied.
    fn get_entity(&self) -> &ManagedEntityEventArgument;
    /// The user name or group to which the permission was granted.
    fn get_principal(&self) -> &str;
    /// Whether or not the principal was a group.
    fn get_group(&self) -> bool;
}
impl<'s> serde::Serialize for dyn PermissionEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn PermissionEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(PermissionEventVisitor)
            }
        }

struct PermissionEventVisitor;

impl<'de> de::Visitor<'de> for PermissionEventVisitor {
    type Value = Box<dyn PermissionEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid PermissionEventTrait JSON object with a _typeName field")
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

impl PermissionEventTrait for PermissionEvent {
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
    fn get_principal(&self) -> &str { &self.principal }
    fn get_group(&self) -> bool { self.group }
}
impl PermissionEventTrait for PermissionAddedEvent {
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
    fn get_principal(&self) -> &str { &self.principal }
    fn get_group(&self) -> bool { self.group }
}
impl PermissionEventTrait for PermissionRemovedEvent {
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
    fn get_principal(&self) -> &str { &self.principal }
    fn get_group(&self) -> bool { self.group }
}
impl PermissionEventTrait for PermissionUpdatedEvent {
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
    fn get_principal(&self) -> &str { &self.principal }
    fn get_group(&self) -> bool { self.group }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn PermissionEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::PermissionEvent => Some(from.as_any_ref().downcast_ref::<PermissionEvent>()?),
            StructType::PermissionAddedEvent => Some(from.as_any_ref().downcast_ref::<PermissionAddedEvent>()?),
            StructType::PermissionRemovedEvent => Some(from.as_any_ref().downcast_ref::<PermissionRemovedEvent>()?),
            StructType::PermissionUpdatedEvent => Some(from.as_any_ref().downcast_ref::<PermissionUpdatedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::PermissionEvent => Ok(from.as_any_box().downcast::<PermissionEvent>()?),
            StructType::PermissionAddedEvent => Ok(from.as_any_box().downcast::<PermissionAddedEvent>()?),
            StructType::PermissionRemovedEvent => Ok(from.as_any_box().downcast::<PermissionRemovedEvent>()?),
            StructType::PermissionUpdatedEvent => Ok(from.as_any_box().downcast::<PermissionUpdatedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
