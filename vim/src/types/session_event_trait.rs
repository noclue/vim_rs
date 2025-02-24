use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These are session events.
pub trait SessionEventTrait : super::event_trait::EventTrait {
}
impl<'s> serde::Serialize for dyn SessionEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn SessionEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(SessionEventVisitor)
            }
        }

struct SessionEventVisitor;

impl<'de> de::Visitor<'de> for SessionEventVisitor {
    type Value = Box<dyn SessionEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid SessionEventTrait JSON object with a _typeName field")
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

impl SessionEventTrait for SessionEvent {
}
impl SessionEventTrait for AlreadyAuthenticatedSessionEvent {
}
impl SessionEventTrait for BadUsernameSessionEvent {
}
impl SessionEventTrait for GlobalMessageChangedEvent {
}
impl SessionEventTrait for NoAccessUserEvent {
}
impl SessionEventTrait for ServerStartedSessionEvent {
}
impl SessionEventTrait for SessionTerminatedEvent {
}
impl SessionEventTrait for UserLoginSessionEvent {
}
impl SessionEventTrait for UserLogoutSessionEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn SessionEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::SessionEvent => Some(from.as_any_ref().downcast_ref::<SessionEvent>()?),
            StructType::AlreadyAuthenticatedSessionEvent => Some(from.as_any_ref().downcast_ref::<AlreadyAuthenticatedSessionEvent>()?),
            StructType::BadUsernameSessionEvent => Some(from.as_any_ref().downcast_ref::<BadUsernameSessionEvent>()?),
            StructType::GlobalMessageChangedEvent => Some(from.as_any_ref().downcast_ref::<GlobalMessageChangedEvent>()?),
            StructType::NoAccessUserEvent => Some(from.as_any_ref().downcast_ref::<NoAccessUserEvent>()?),
            StructType::ServerStartedSessionEvent => Some(from.as_any_ref().downcast_ref::<ServerStartedSessionEvent>()?),
            StructType::SessionTerminatedEvent => Some(from.as_any_ref().downcast_ref::<SessionTerminatedEvent>()?),
            StructType::UserLoginSessionEvent => Some(from.as_any_ref().downcast_ref::<UserLoginSessionEvent>()?),
            StructType::UserLogoutSessionEvent => Some(from.as_any_ref().downcast_ref::<UserLogoutSessionEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::SessionEvent => Ok(from.as_any_box().downcast::<SessionEvent>()?),
            StructType::AlreadyAuthenticatedSessionEvent => Ok(from.as_any_box().downcast::<AlreadyAuthenticatedSessionEvent>()?),
            StructType::BadUsernameSessionEvent => Ok(from.as_any_box().downcast::<BadUsernameSessionEvent>()?),
            StructType::GlobalMessageChangedEvent => Ok(from.as_any_box().downcast::<GlobalMessageChangedEvent>()?),
            StructType::NoAccessUserEvent => Ok(from.as_any_box().downcast::<NoAccessUserEvent>()?),
            StructType::ServerStartedSessionEvent => Ok(from.as_any_box().downcast::<ServerStartedSessionEvent>()?),
            StructType::SessionTerminatedEvent => Ok(from.as_any_box().downcast::<SessionTerminatedEvent>()?),
            StructType::UserLoginSessionEvent => Ok(from.as_any_box().downcast::<UserLoginSessionEvent>()?),
            StructType::UserLogoutSessionEvent => Ok(from.as_any_box().downcast::<UserLogoutSessionEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
