use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An OVF descriptor Attribute base class.
pub trait OvfAttributeTrait : super::ovf_invalid_package_trait::OvfInvalidPackageTrait {
    /// Element name where the attribute is defined
    fn get_element_name(&self) -> &str;
    /// Attribute name
    fn get_attribute_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn OvfAttributeTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfAttributeTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfAttributeVisitor)
            }
        }

struct OvfAttributeVisitor;

impl<'de> de::Visitor<'de> for OvfAttributeVisitor {
    type Value = Box<dyn OvfAttributeTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfAttributeTrait JSON object with a _typeName field")
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

impl OvfAttributeTrait for OvfAttribute {
    fn get_element_name(&self) -> &str { &self.element_name }
    fn get_attribute_name(&self) -> &str { &self.attribute_name }
}
impl OvfAttributeTrait for OvfInvalidValue {
    fn get_element_name(&self) -> &str { &self.element_name }
    fn get_attribute_name(&self) -> &str { &self.attribute_name }
}
impl OvfAttributeTrait for OvfInvalidValueConfiguration {
    fn get_element_name(&self) -> &str { &self.element_name }
    fn get_attribute_name(&self) -> &str { &self.attribute_name }
}
impl OvfAttributeTrait for OvfInvalidValueEmpty {
    fn get_element_name(&self) -> &str { &self.element_name }
    fn get_attribute_name(&self) -> &str { &self.attribute_name }
}
impl OvfAttributeTrait for OvfInvalidValueFormatMalformed {
    fn get_element_name(&self) -> &str { &self.element_name }
    fn get_attribute_name(&self) -> &str { &self.attribute_name }
}
impl OvfAttributeTrait for OvfInvalidValueReference {
    fn get_element_name(&self) -> &str { &self.element_name }
    fn get_attribute_name(&self) -> &str { &self.attribute_name }
}
impl OvfAttributeTrait for OvfMissingAttribute {
    fn get_element_name(&self) -> &str { &self.element_name }
    fn get_attribute_name(&self) -> &str { &self.attribute_name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfAttributeTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfAttribute => Some(from.as_any_ref().downcast_ref::<OvfAttribute>()?),
            StructType::OvfInvalidValue => Some(from.as_any_ref().downcast_ref::<OvfInvalidValue>()?),
            StructType::OvfInvalidValueConfiguration => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueConfiguration>()?),
            StructType::OvfInvalidValueEmpty => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueEmpty>()?),
            StructType::OvfInvalidValueFormatMalformed => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueFormatMalformed>()?),
            StructType::OvfInvalidValueReference => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueReference>()?),
            StructType::OvfMissingAttribute => Some(from.as_any_ref().downcast_ref::<OvfMissingAttribute>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfAttribute => Ok(from.as_any_box().downcast::<OvfAttribute>()?),
            StructType::OvfInvalidValue => Ok(from.as_any_box().downcast::<OvfInvalidValue>()?),
            StructType::OvfInvalidValueConfiguration => Ok(from.as_any_box().downcast::<OvfInvalidValueConfiguration>()?),
            StructType::OvfInvalidValueEmpty => Ok(from.as_any_box().downcast::<OvfInvalidValueEmpty>()?),
            StructType::OvfInvalidValueFormatMalformed => Ok(from.as_any_box().downcast::<OvfInvalidValueFormatMalformed>()?),
            StructType::OvfInvalidValueReference => Ok(from.as_any_box().downcast::<OvfInvalidValueReference>()?),
            StructType::OvfMissingAttribute => Ok(from.as_any_box().downcast::<OvfMissingAttribute>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
