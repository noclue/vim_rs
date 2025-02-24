use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// DVPortgroup related events.
pub trait DvPortgroupEventTrait : super::event_trait::EventTrait {
}
impl<'s> serde::Serialize for dyn DvPortgroupEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvPortgroupEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvPortgroupEventVisitor)
            }
        }

struct DvPortgroupEventVisitor;

impl<'de> de::Visitor<'de> for DvPortgroupEventVisitor {
    type Value = Box<dyn DvPortgroupEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvPortgroupEventTrait JSON object with a _typeName field")
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

impl DvPortgroupEventTrait for DvPortgroupEvent {
}
impl DvPortgroupEventTrait for DvPortgroupCreatedEvent {
}
impl DvPortgroupEventTrait for DvPortgroupDestroyedEvent {
}
impl DvPortgroupEventTrait for DvPortgroupReconfiguredEvent {
}
impl DvPortgroupEventTrait for DvPortgroupRenamedEvent {
}
impl DvPortgroupEventTrait for DvpgImportEvent {
}
impl DvPortgroupEventTrait for DvpgRestoreEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvPortgroupEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvPortgroupEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupEvent>()?),
            StructType::DvPortgroupCreatedEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupCreatedEvent>()?),
            StructType::DvPortgroupDestroyedEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupDestroyedEvent>()?),
            StructType::DvPortgroupReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupReconfiguredEvent>()?),
            StructType::DvPortgroupRenamedEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupRenamedEvent>()?),
            StructType::DvpgImportEvent => Some(from.as_any_ref().downcast_ref::<DvpgImportEvent>()?),
            StructType::DvpgRestoreEvent => Some(from.as_any_ref().downcast_ref::<DvpgRestoreEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvPortgroupEvent => Ok(from.as_any_box().downcast::<DvPortgroupEvent>()?),
            StructType::DvPortgroupCreatedEvent => Ok(from.as_any_box().downcast::<DvPortgroupCreatedEvent>()?),
            StructType::DvPortgroupDestroyedEvent => Ok(from.as_any_box().downcast::<DvPortgroupDestroyedEvent>()?),
            StructType::DvPortgroupReconfiguredEvent => Ok(from.as_any_box().downcast::<DvPortgroupReconfiguredEvent>()?),
            StructType::DvPortgroupRenamedEvent => Ok(from.as_any_box().downcast::<DvPortgroupRenamedEvent>()?),
            StructType::DvpgImportEvent => Ok(from.as_any_box().downcast::<DvpgImportEvent>()?),
            StructType::DvpgRestoreEvent => Ok(from.as_any_box().downcast::<DvpgRestoreEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
