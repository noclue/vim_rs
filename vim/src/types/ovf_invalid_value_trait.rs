use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// If an invalid value is found in the Ovf descriptor we throw an OvfInvalidValue exception.
pub trait OvfInvalidValueTrait : super::ovf_attribute_trait::OvfAttributeTrait {
    /// Attribute value
    fn get_value(&self) -> &str;
}
impl<'s> serde::Serialize for dyn OvfInvalidValueTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfInvalidValueTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfInvalidValueVisitor)
            }
        }

struct OvfInvalidValueVisitor;

impl<'de> de::Visitor<'de> for OvfInvalidValueVisitor {
    type Value = Box<dyn OvfInvalidValueTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfInvalidValueTrait JSON object with a _typeName field")
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

impl OvfInvalidValueTrait for OvfInvalidValue {
    fn get_value(&self) -> &str { &self.value }
}
impl OvfInvalidValueTrait for OvfInvalidValueConfiguration {
    fn get_value(&self) -> &str { &self.value }
}
impl OvfInvalidValueTrait for OvfInvalidValueEmpty {
    fn get_value(&self) -> &str { &self.value }
}
impl OvfInvalidValueTrait for OvfInvalidValueFormatMalformed {
    fn get_value(&self) -> &str { &self.value }
}
impl OvfInvalidValueTrait for OvfInvalidValueReference {
    fn get_value(&self) -> &str { &self.value }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfInvalidValueTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfInvalidValue => Some(from.as_any_ref().downcast_ref::<OvfInvalidValue>()?),
            StructType::OvfInvalidValueConfiguration => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueConfiguration>()?),
            StructType::OvfInvalidValueEmpty => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueEmpty>()?),
            StructType::OvfInvalidValueFormatMalformed => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueFormatMalformed>()?),
            StructType::OvfInvalidValueReference => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueReference>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfInvalidValue => Ok(from.as_any_box().downcast::<OvfInvalidValue>()?),
            StructType::OvfInvalidValueConfiguration => Ok(from.as_any_box().downcast::<OvfInvalidValueConfiguration>()?),
            StructType::OvfInvalidValueEmpty => Ok(from.as_any_box().downcast::<OvfInvalidValueEmpty>()?),
            StructType::OvfInvalidValueFormatMalformed => Ok(from.as_any_box().downcast::<OvfInvalidValueFormatMalformed>()?),
            StructType::OvfInvalidValueReference => Ok(from.as_any_box().downcast::<OvfInvalidValueReference>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
