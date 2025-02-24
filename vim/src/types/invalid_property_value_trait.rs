use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The value of the property is not valid given the type of the property.
pub trait InvalidPropertyValueTrait : super::v_app_property_fault_trait::VAppPropertyFaultTrait {
}
impl<'s> serde::Serialize for dyn InvalidPropertyValueTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InvalidPropertyValueTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InvalidPropertyValueVisitor)
            }
        }

struct InvalidPropertyValueVisitor;

impl<'de> de::Visitor<'de> for InvalidPropertyValueVisitor {
    type Value = Box<dyn InvalidPropertyValueTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InvalidPropertyValueTrait JSON object with a _typeName field")
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

impl InvalidPropertyValueTrait for InvalidPropertyValue {
}
impl InvalidPropertyValueTrait for UnconfiguredPropertyValue {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InvalidPropertyValueTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidPropertyValue => Some(from.as_any_ref().downcast_ref::<InvalidPropertyValue>()?),
            StructType::UnconfiguredPropertyValue => Some(from.as_any_ref().downcast_ref::<UnconfiguredPropertyValue>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidPropertyValue => Ok(from.as_any_box().downcast::<InvalidPropertyValue>()?),
            StructType::UnconfiguredPropertyValue => Ok(from.as_any_box().downcast::<UnconfiguredPropertyValue>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
