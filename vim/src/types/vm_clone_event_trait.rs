use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The is the base event for all clone operations.
pub trait VmCloneEventTrait : super::vm_event_trait::VmEventTrait {
}
impl<'s> serde::Serialize for dyn VmCloneEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmCloneEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmCloneEventVisitor)
            }
        }

struct VmCloneEventVisitor;

impl<'de> de::Visitor<'de> for VmCloneEventVisitor {
    type Value = Box<dyn VmCloneEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmCloneEventTrait JSON object with a _typeName field")
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

impl VmCloneEventTrait for VmCloneEvent {
}
impl VmCloneEventTrait for VmBeingClonedEvent {
}
impl VmCloneEventTrait for VmBeingClonedNoFolderEvent {
}
impl VmCloneEventTrait for VmCloneFailedEvent {
}
impl VmCloneEventTrait for VmClonedEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmCloneEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmCloneEvent => Some(from.as_any_ref().downcast_ref::<VmCloneEvent>()?),
            StructType::VmBeingClonedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingClonedEvent>()?),
            StructType::VmBeingClonedNoFolderEvent => Some(from.as_any_ref().downcast_ref::<VmBeingClonedNoFolderEvent>()?),
            StructType::VmCloneFailedEvent => Some(from.as_any_ref().downcast_ref::<VmCloneFailedEvent>()?),
            StructType::VmClonedEvent => Some(from.as_any_ref().downcast_ref::<VmClonedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmCloneEvent => Ok(from.as_any_box().downcast::<VmCloneEvent>()?),
            StructType::VmBeingClonedEvent => Ok(from.as_any_box().downcast::<VmBeingClonedEvent>()?),
            StructType::VmBeingClonedNoFolderEvent => Ok(from.as_any_box().downcast::<VmBeingClonedNoFolderEvent>()?),
            StructType::VmCloneFailedEvent => Ok(from.as_any_box().downcast::<VmCloneFailedEvent>()?),
            StructType::VmClonedEvent => Ok(from.as_any_box().downcast::<VmClonedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
