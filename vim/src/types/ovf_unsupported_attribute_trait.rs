use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// If the Ovf descriptor have an unsupported attribute.
pub trait OvfUnsupportedAttributeTrait : super::ovf_unsupported_package_trait::OvfUnsupportedPackageTrait {
    /// The name of the element with the unsupported attribute
    fn get_element_name(&self) -> &str;
    /// The name of the unsupported attribute
    fn get_attribute_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn OvfUnsupportedAttributeTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfUnsupportedAttributeTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfUnsupportedAttributeVisitor)
            }
        }

struct OvfUnsupportedAttributeVisitor;

impl<'de> de::Visitor<'de> for OvfUnsupportedAttributeVisitor {
    type Value = Box<dyn OvfUnsupportedAttributeTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfUnsupportedAttributeTrait JSON object with a _typeName field")
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

impl OvfUnsupportedAttributeTrait for OvfUnsupportedAttribute {
    fn get_element_name(&self) -> &str { &self.element_name }
    fn get_attribute_name(&self) -> &str { &self.attribute_name }
}
impl OvfUnsupportedAttributeTrait for OvfUnsupportedAttributeValue {
    fn get_element_name(&self) -> &str { &self.element_name }
    fn get_attribute_name(&self) -> &str { &self.attribute_name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfUnsupportedAttributeTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfUnsupportedAttribute => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedAttribute>()?),
            StructType::OvfUnsupportedAttributeValue => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedAttributeValue>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfUnsupportedAttribute => Ok(from.as_any_box().downcast::<OvfUnsupportedAttribute>()?),
            StructType::OvfUnsupportedAttributeValue => Ok(from.as_any_box().downcast::<OvfUnsupportedAttributeValue>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
