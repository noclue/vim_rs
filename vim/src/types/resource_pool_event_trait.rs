use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event is the base class for all resource pool events.
pub trait ResourcePoolEventTrait : super::event_trait::EventTrait {
    fn get_resource_pool(&self) -> &ResourcePoolEventArgument;
}
impl<'s> serde::Serialize for dyn ResourcePoolEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ResourcePoolEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ResourcePoolEventVisitor)
            }
        }

struct ResourcePoolEventVisitor;

impl<'de> de::Visitor<'de> for ResourcePoolEventVisitor {
    type Value = Box<dyn ResourcePoolEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ResourcePoolEventTrait JSON object with a _typeName field")
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

impl ResourcePoolEventTrait for ResourcePoolEvent {
    fn get_resource_pool(&self) -> &ResourcePoolEventArgument { &self.resource_pool }
}
impl ResourcePoolEventTrait for ResourcePoolCreatedEvent {
    fn get_resource_pool(&self) -> &ResourcePoolEventArgument { &self.resource_pool }
}
impl ResourcePoolEventTrait for ResourcePoolDestroyedEvent {
    fn get_resource_pool(&self) -> &ResourcePoolEventArgument { &self.resource_pool }
}
impl ResourcePoolEventTrait for ResourcePoolMovedEvent {
    fn get_resource_pool(&self) -> &ResourcePoolEventArgument { &self.resource_pool }
}
impl ResourcePoolEventTrait for ResourcePoolReconfiguredEvent {
    fn get_resource_pool(&self) -> &ResourcePoolEventArgument { &self.resource_pool }
}
impl ResourcePoolEventTrait for ResourceViolatedEvent {
    fn get_resource_pool(&self) -> &ResourcePoolEventArgument { &self.resource_pool }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ResourcePoolEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ResourcePoolEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolEvent>()?),
            StructType::ResourcePoolCreatedEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolCreatedEvent>()?),
            StructType::ResourcePoolDestroyedEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolDestroyedEvent>()?),
            StructType::ResourcePoolMovedEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolMovedEvent>()?),
            StructType::ResourcePoolReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolReconfiguredEvent>()?),
            StructType::ResourceViolatedEvent => Some(from.as_any_ref().downcast_ref::<ResourceViolatedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ResourcePoolEvent => Ok(from.as_any_box().downcast::<ResourcePoolEvent>()?),
            StructType::ResourcePoolCreatedEvent => Ok(from.as_any_box().downcast::<ResourcePoolCreatedEvent>()?),
            StructType::ResourcePoolDestroyedEvent => Ok(from.as_any_box().downcast::<ResourcePoolDestroyedEvent>()?),
            StructType::ResourcePoolMovedEvent => Ok(from.as_any_box().downcast::<ResourcePoolMovedEvent>()?),
            StructType::ResourcePoolReconfiguredEvent => Ok(from.as_any_box().downcast::<ResourcePoolReconfiguredEvent>()?),
            StructType::ResourceViolatedEvent => Ok(from.as_any_box().downcast::<ResourceViolatedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
