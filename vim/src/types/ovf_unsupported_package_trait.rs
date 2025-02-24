use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A common base class to host all the Ovf Lib Unsupported Package faults
pub trait OvfUnsupportedPackageTrait : super::ovf_fault_trait::OvfFaultTrait {
    /// OVF descriptor linenumber
    fn get_line_number(&self) -> Option<i32>;
}
impl<'s> serde::Serialize for dyn OvfUnsupportedPackageTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfUnsupportedPackageTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfUnsupportedPackageVisitor)
            }
        }

struct OvfUnsupportedPackageVisitor;

impl<'de> de::Visitor<'de> for OvfUnsupportedPackageVisitor {
    type Value = Box<dyn OvfUnsupportedPackageTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfUnsupportedPackageTrait JSON object with a _typeName field")
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

impl OvfUnsupportedPackageTrait for OvfUnsupportedPackage {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl OvfUnsupportedPackageTrait for OvfInvalidVmName {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl OvfUnsupportedPackageTrait for OvfNoHostNic {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl OvfUnsupportedPackageTrait for OvfNoSupportedHardwareFamily {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl OvfUnsupportedPackageTrait for OvfUnsupportedAttribute {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl OvfUnsupportedPackageTrait for OvfUnsupportedAttributeValue {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl OvfUnsupportedPackageTrait for OvfUnsupportedElement {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl OvfUnsupportedPackageTrait for OvfNoSpaceOnController {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl OvfUnsupportedPackageTrait for OvfUnsupportedElementValue {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl OvfUnsupportedPackageTrait for OvfUnsupportedSection {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl OvfUnsupportedPackageTrait for OvfUnsupportedSubType {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl OvfUnsupportedPackageTrait for OvfUnsupportedType {
    fn get_line_number(&self) -> Option<i32> { self.line_number }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfUnsupportedPackageTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfUnsupportedPackage => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedPackage>()?),
            StructType::OvfInvalidVmName => Some(from.as_any_ref().downcast_ref::<OvfInvalidVmName>()?),
            StructType::OvfNoHostNic => Some(from.as_any_ref().downcast_ref::<OvfNoHostNic>()?),
            StructType::OvfNoSupportedHardwareFamily => Some(from.as_any_ref().downcast_ref::<OvfNoSupportedHardwareFamily>()?),
            StructType::OvfUnsupportedAttribute => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedAttribute>()?),
            StructType::OvfUnsupportedAttributeValue => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedAttributeValue>()?),
            StructType::OvfUnsupportedElement => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedElement>()?),
            StructType::OvfNoSpaceOnController => Some(from.as_any_ref().downcast_ref::<OvfNoSpaceOnController>()?),
            StructType::OvfUnsupportedElementValue => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedElementValue>()?),
            StructType::OvfUnsupportedSection => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedSection>()?),
            StructType::OvfUnsupportedSubType => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedSubType>()?),
            StructType::OvfUnsupportedType => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedType>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfUnsupportedPackage => Ok(from.as_any_box().downcast::<OvfUnsupportedPackage>()?),
            StructType::OvfInvalidVmName => Ok(from.as_any_box().downcast::<OvfInvalidVmName>()?),
            StructType::OvfNoHostNic => Ok(from.as_any_box().downcast::<OvfNoHostNic>()?),
            StructType::OvfNoSupportedHardwareFamily => Ok(from.as_any_box().downcast::<OvfNoSupportedHardwareFamily>()?),
            StructType::OvfUnsupportedAttribute => Ok(from.as_any_box().downcast::<OvfUnsupportedAttribute>()?),
            StructType::OvfUnsupportedAttributeValue => Ok(from.as_any_box().downcast::<OvfUnsupportedAttributeValue>()?),
            StructType::OvfUnsupportedElement => Ok(from.as_any_box().downcast::<OvfUnsupportedElement>()?),
            StructType::OvfNoSpaceOnController => Ok(from.as_any_box().downcast::<OvfNoSpaceOnController>()?),
            StructType::OvfUnsupportedElementValue => Ok(from.as_any_box().downcast::<OvfUnsupportedElementValue>()?),
            StructType::OvfUnsupportedSection => Ok(from.as_any_box().downcast::<OvfUnsupportedSection>()?),
            StructType::OvfUnsupportedSubType => Ok(from.as_any_box().downcast::<OvfUnsupportedSubType>()?),
            StructType::OvfUnsupportedType => Ok(from.as_any_box().downcast::<OvfUnsupportedType>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
