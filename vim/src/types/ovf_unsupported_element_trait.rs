use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// If the Ovf descriptor has an unsupported element where it is not allowed.
pub trait OvfUnsupportedElementTrait : super::ovf_unsupported_package_trait::OvfUnsupportedPackageTrait {
    /// The name of the unsupported element
    fn get_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn OvfUnsupportedElementTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfUnsupportedElementTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfUnsupportedElementVisitor)
            }
        }

struct OvfUnsupportedElementVisitor;

impl<'de> de::Visitor<'de> for OvfUnsupportedElementVisitor {
    type Value = Box<dyn OvfUnsupportedElementTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfUnsupportedElementTrait JSON object with a _typeName field")
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

impl OvfUnsupportedElementTrait for OvfUnsupportedElement {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfUnsupportedElementTrait for OvfNoSpaceOnController {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfUnsupportedElementTrait for OvfUnsupportedElementValue {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfUnsupportedElementTrait for OvfUnsupportedSection {
    fn get_name(&self) -> &str { &self.name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfUnsupportedElementTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfUnsupportedElement => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedElement>()?),
            StructType::OvfNoSpaceOnController => Some(from.as_any_ref().downcast_ref::<OvfNoSpaceOnController>()?),
            StructType::OvfUnsupportedElementValue => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedElementValue>()?),
            StructType::OvfUnsupportedSection => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedSection>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfUnsupportedElement => Ok(from.as_any_box().downcast::<OvfUnsupportedElement>()?),
            StructType::OvfNoSpaceOnController => Ok(from.as_any_box().downcast::<OvfNoSpaceOnController>()?),
            StructType::OvfUnsupportedElementValue => Ok(from.as_any_box().downcast::<OvfUnsupportedElementValue>()?),
            StructType::OvfUnsupportedSection => Ok(from.as_any_box().downcast::<OvfUnsupportedSection>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
