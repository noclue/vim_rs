use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These are custom field events.
pub trait CustomFieldEventTrait : super::event_trait::EventTrait {
}
impl<'s> serde::Serialize for dyn CustomFieldEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CustomFieldEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CustomFieldEventVisitor)
            }
        }

struct CustomFieldEventVisitor;

impl<'de> de::Visitor<'de> for CustomFieldEventVisitor {
    type Value = Box<dyn CustomFieldEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CustomFieldEventTrait JSON object with a _typeName field")
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

impl CustomFieldEventTrait for CustomFieldEvent {
}
impl CustomFieldEventTrait for CustomFieldDefEvent {
}
impl CustomFieldEventTrait for CustomFieldDefAddedEvent {
}
impl CustomFieldEventTrait for CustomFieldDefRemovedEvent {
}
impl CustomFieldEventTrait for CustomFieldDefRenamedEvent {
}
impl CustomFieldEventTrait for CustomFieldValueChangedEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CustomFieldEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomFieldEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldEvent>()?),
            StructType::CustomFieldDefEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefEvent>()?),
            StructType::CustomFieldDefAddedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefAddedEvent>()?),
            StructType::CustomFieldDefRemovedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefRemovedEvent>()?),
            StructType::CustomFieldDefRenamedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefRenamedEvent>()?),
            StructType::CustomFieldValueChangedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldValueChangedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomFieldEvent => Ok(from.as_any_box().downcast::<CustomFieldEvent>()?),
            StructType::CustomFieldDefEvent => Ok(from.as_any_box().downcast::<CustomFieldDefEvent>()?),
            StructType::CustomFieldDefAddedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefAddedEvent>()?),
            StructType::CustomFieldDefRemovedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefRemovedEvent>()?),
            StructType::CustomFieldDefRenamedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefRenamedEvent>()?),
            StructType::CustomFieldValueChangedEvent => Ok(from.as_any_box().downcast::<CustomFieldValueChangedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
