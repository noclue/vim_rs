use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base type for storing values.
pub trait CustomFieldValueTrait : super::data_object_trait::DataObjectTrait {
    /// The ID of the field to which this value belongs.
    fn get_key(&self) -> i32;
}
impl<'s> serde::Serialize for dyn CustomFieldValueTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CustomFieldValueTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CustomFieldValueVisitor)
            }
        }

struct CustomFieldValueVisitor;

impl<'de> de::Visitor<'de> for CustomFieldValueVisitor {
    type Value = Box<dyn CustomFieldValueTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CustomFieldValueTrait JSON object with a _typeName field")
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

impl CustomFieldValueTrait for CustomFieldValue {
    fn get_key(&self) -> i32 { self.key }
}
impl CustomFieldValueTrait for CustomFieldStringValue {
    fn get_key(&self) -> i32 { self.key }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CustomFieldValueTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomFieldValue => Some(from.as_any_ref().downcast_ref::<CustomFieldValue>()?),
            StructType::CustomFieldStringValue => Some(from.as_any_ref().downcast_ref::<CustomFieldStringValue>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomFieldValue => Ok(from.as_any_box().downcast::<CustomFieldValue>()?),
            StructType::CustomFieldStringValue => Ok(from.as_any_box().downcast::<CustomFieldStringValue>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
