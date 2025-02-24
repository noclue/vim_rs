use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event is the base event for relocate and clone base events.
pub trait VmRelocateSpecEventTrait : super::vm_event_trait::VmEventTrait {
}
impl<'s> serde::Serialize for dyn VmRelocateSpecEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmRelocateSpecEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmRelocateSpecEventVisitor)
            }
        }

struct VmRelocateSpecEventVisitor;

impl<'de> de::Visitor<'de> for VmRelocateSpecEventVisitor {
    type Value = Box<dyn VmRelocateSpecEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmRelocateSpecEventTrait JSON object with a _typeName field")
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

impl VmRelocateSpecEventTrait for VmRelocateSpecEvent {
}
impl VmRelocateSpecEventTrait for VmBeingRelocatedEvent {
}
impl VmRelocateSpecEventTrait for VmRelocateFailedEvent {
}
impl VmRelocateSpecEventTrait for VmRelocatedEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmRelocateSpecEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmRelocateSpecEvent => Some(from.as_any_ref().downcast_ref::<VmRelocateSpecEvent>()?),
            StructType::VmBeingRelocatedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingRelocatedEvent>()?),
            StructType::VmRelocateFailedEvent => Some(from.as_any_ref().downcast_ref::<VmRelocateFailedEvent>()?),
            StructType::VmRelocatedEvent => Some(from.as_any_ref().downcast_ref::<VmRelocatedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmRelocateSpecEvent => Ok(from.as_any_box().downcast::<VmRelocateSpecEvent>()?),
            StructType::VmBeingRelocatedEvent => Ok(from.as_any_box().downcast::<VmBeingRelocatedEvent>()?),
            StructType::VmRelocateFailedEvent => Ok(from.as_any_box().downcast::<VmRelocateFailedEvent>()?),
            StructType::VmRelocatedEvent => Ok(from.as_any_box().downcast::<VmRelocatedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
