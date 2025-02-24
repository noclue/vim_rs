use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event records a custom field definition event.
pub trait CustomFieldDefEventTrait : super::custom_field_event_trait::CustomFieldEventTrait {
    /// The unique identifier of the custom field definition.
    fn get_field_key(&self) -> i32;
    /// The name of the custom field.
    fn get_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn CustomFieldDefEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CustomFieldDefEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CustomFieldDefEventVisitor)
            }
        }

struct CustomFieldDefEventVisitor;

impl<'de> de::Visitor<'de> for CustomFieldDefEventVisitor {
    type Value = Box<dyn CustomFieldDefEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CustomFieldDefEventTrait JSON object with a _typeName field")
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

impl CustomFieldDefEventTrait for CustomFieldDefEvent {
    fn get_field_key(&self) -> i32 { self.field_key }
    fn get_name(&self) -> &str { &self.name }
}
impl CustomFieldDefEventTrait for CustomFieldDefAddedEvent {
    fn get_field_key(&self) -> i32 { self.field_key }
    fn get_name(&self) -> &str { &self.name }
}
impl CustomFieldDefEventTrait for CustomFieldDefRemovedEvent {
    fn get_field_key(&self) -> i32 { self.field_key }
    fn get_name(&self) -> &str { &self.name }
}
impl CustomFieldDefEventTrait for CustomFieldDefRenamedEvent {
    fn get_field_key(&self) -> i32 { self.field_key }
    fn get_name(&self) -> &str { &self.name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CustomFieldDefEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomFieldDefEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefEvent>()?),
            StructType::CustomFieldDefAddedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefAddedEvent>()?),
            StructType::CustomFieldDefRemovedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefRemovedEvent>()?),
            StructType::CustomFieldDefRenamedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefRenamedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomFieldDefEvent => Ok(from.as_any_box().downcast::<CustomFieldDefEvent>()?),
            StructType::CustomFieldDefAddedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefAddedEvent>()?),
            StructType::CustomFieldDefRemovedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefRemovedEvent>()?),
            StructType::CustomFieldDefRenamedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefRenamedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
