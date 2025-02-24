use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A base fault for element exceptions in the Ovf XML descriptor.
pub trait OvfElementTrait : super::ovf_invalid_package_trait::OvfInvalidPackageTrait {
    /// The name of the element
    fn get_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn OvfElementTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfElementTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfElementVisitor)
            }
        }

struct OvfElementVisitor;

impl<'de> de::Visitor<'de> for OvfElementVisitor {
    type Value = Box<dyn OvfElementTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfElementTrait JSON object with a _typeName field")
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

impl OvfElementTrait for OvfElement {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfElementTrait for OvfDuplicateElement {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfElementTrait for OvfDuplicatedElementBoundary {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfElementTrait for OvfElementInvalidValue {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfElementTrait for OvfMissingElement {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfElementTrait for OvfMissingElementNormalBoundary {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfElementTrait for OvfUnexpectedElement {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfElementTrait for OvfWrongElement {
    fn get_name(&self) -> &str { &self.name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfElementTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfElement => Some(from.as_any_ref().downcast_ref::<OvfElement>()?),
            StructType::OvfDuplicateElement => Some(from.as_any_ref().downcast_ref::<OvfDuplicateElement>()?),
            StructType::OvfDuplicatedElementBoundary => Some(from.as_any_ref().downcast_ref::<OvfDuplicatedElementBoundary>()?),
            StructType::OvfElementInvalidValue => Some(from.as_any_ref().downcast_ref::<OvfElementInvalidValue>()?),
            StructType::OvfMissingElement => Some(from.as_any_ref().downcast_ref::<OvfMissingElement>()?),
            StructType::OvfMissingElementNormalBoundary => Some(from.as_any_ref().downcast_ref::<OvfMissingElementNormalBoundary>()?),
            StructType::OvfUnexpectedElement => Some(from.as_any_ref().downcast_ref::<OvfUnexpectedElement>()?),
            StructType::OvfWrongElement => Some(from.as_any_ref().downcast_ref::<OvfWrongElement>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfElement => Ok(from.as_any_box().downcast::<OvfElement>()?),
            StructType::OvfDuplicateElement => Ok(from.as_any_box().downcast::<OvfDuplicateElement>()?),
            StructType::OvfDuplicatedElementBoundary => Ok(from.as_any_box().downcast::<OvfDuplicatedElementBoundary>()?),
            StructType::OvfElementInvalidValue => Ok(from.as_any_box().downcast::<OvfElementInvalidValue>()?),
            StructType::OvfMissingElement => Ok(from.as_any_box().downcast::<OvfMissingElement>()?),
            StructType::OvfMissingElementNormalBoundary => Ok(from.as_any_box().downcast::<OvfMissingElementNormalBoundary>()?),
            StructType::OvfUnexpectedElement => Ok(from.as_any_box().downcast::<OvfUnexpectedElement>()?),
            StructType::OvfWrongElement => Ok(from.as_any_box().downcast::<OvfWrongElement>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
