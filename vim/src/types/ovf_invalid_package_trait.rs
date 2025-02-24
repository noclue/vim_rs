use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base fault class for all Invalid OVF package faults.
pub trait OvfInvalidPackageTrait : super::ovf_fault_trait::OvfFaultTrait {
    /// XML OVF descriptor line numbers
    fn get_line_number(&self) -> i32;
}
impl<'s> serde::Serialize for dyn OvfInvalidPackageTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfInvalidPackageTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfInvalidPackageVisitor)
            }
        }

struct OvfInvalidPackageVisitor;

impl<'de> de::Visitor<'de> for OvfInvalidPackageVisitor {
    type Value = Box<dyn OvfInvalidPackageTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfInvalidPackageTrait JSON object with a _typeName field")
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

impl OvfInvalidPackageTrait for OvfInvalidPackage {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfAttribute {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfInvalidValue {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfInvalidValueConfiguration {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfInvalidValueEmpty {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfInvalidValueFormatMalformed {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfInvalidValueReference {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfMissingAttribute {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfConstraint {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfDiskOrderConstraint {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfHostResourceConstraint {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfElement {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfDuplicateElement {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfDuplicatedElementBoundary {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfElementInvalidValue {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfMissingElement {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfMissingElementNormalBoundary {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfUnexpectedElement {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfWrongElement {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfProperty {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfPropertyNetwork {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfPropertyQualifier {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfPropertyQualifierDuplicate {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfPropertyQualifierIgnored {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfPropertyType {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfPropertyValue {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfWrongNamespace {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl OvfInvalidPackageTrait for OvfXmlFormat {
    fn get_line_number(&self) -> i32 { self.line_number }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfInvalidPackageTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfInvalidPackage => Some(from.as_any_ref().downcast_ref::<OvfInvalidPackage>()?),
            StructType::OvfAttribute => Some(from.as_any_ref().downcast_ref::<OvfAttribute>()?),
            StructType::OvfInvalidValue => Some(from.as_any_ref().downcast_ref::<OvfInvalidValue>()?),
            StructType::OvfInvalidValueConfiguration => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueConfiguration>()?),
            StructType::OvfInvalidValueEmpty => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueEmpty>()?),
            StructType::OvfInvalidValueFormatMalformed => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueFormatMalformed>()?),
            StructType::OvfInvalidValueReference => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueReference>()?),
            StructType::OvfMissingAttribute => Some(from.as_any_ref().downcast_ref::<OvfMissingAttribute>()?),
            StructType::OvfConstraint => Some(from.as_any_ref().downcast_ref::<OvfConstraint>()?),
            StructType::OvfDiskOrderConstraint => Some(from.as_any_ref().downcast_ref::<OvfDiskOrderConstraint>()?),
            StructType::OvfHostResourceConstraint => Some(from.as_any_ref().downcast_ref::<OvfHostResourceConstraint>()?),
            StructType::OvfElement => Some(from.as_any_ref().downcast_ref::<OvfElement>()?),
            StructType::OvfDuplicateElement => Some(from.as_any_ref().downcast_ref::<OvfDuplicateElement>()?),
            StructType::OvfDuplicatedElementBoundary => Some(from.as_any_ref().downcast_ref::<OvfDuplicatedElementBoundary>()?),
            StructType::OvfElementInvalidValue => Some(from.as_any_ref().downcast_ref::<OvfElementInvalidValue>()?),
            StructType::OvfMissingElement => Some(from.as_any_ref().downcast_ref::<OvfMissingElement>()?),
            StructType::OvfMissingElementNormalBoundary => Some(from.as_any_ref().downcast_ref::<OvfMissingElementNormalBoundary>()?),
            StructType::OvfUnexpectedElement => Some(from.as_any_ref().downcast_ref::<OvfUnexpectedElement>()?),
            StructType::OvfWrongElement => Some(from.as_any_ref().downcast_ref::<OvfWrongElement>()?),
            StructType::OvfProperty => Some(from.as_any_ref().downcast_ref::<OvfProperty>()?),
            StructType::OvfPropertyNetwork => Some(from.as_any_ref().downcast_ref::<OvfPropertyNetwork>()?),
            StructType::OvfPropertyQualifier => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifier>()?),
            StructType::OvfPropertyQualifierDuplicate => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifierDuplicate>()?),
            StructType::OvfPropertyQualifierIgnored => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifierIgnored>()?),
            StructType::OvfPropertyType => Some(from.as_any_ref().downcast_ref::<OvfPropertyType>()?),
            StructType::OvfPropertyValue => Some(from.as_any_ref().downcast_ref::<OvfPropertyValue>()?),
            StructType::OvfWrongNamespace => Some(from.as_any_ref().downcast_ref::<OvfWrongNamespace>()?),
            StructType::OvfXmlFormat => Some(from.as_any_ref().downcast_ref::<OvfXmlFormat>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfInvalidPackage => Ok(from.as_any_box().downcast::<OvfInvalidPackage>()?),
            StructType::OvfAttribute => Ok(from.as_any_box().downcast::<OvfAttribute>()?),
            StructType::OvfInvalidValue => Ok(from.as_any_box().downcast::<OvfInvalidValue>()?),
            StructType::OvfInvalidValueConfiguration => Ok(from.as_any_box().downcast::<OvfInvalidValueConfiguration>()?),
            StructType::OvfInvalidValueEmpty => Ok(from.as_any_box().downcast::<OvfInvalidValueEmpty>()?),
            StructType::OvfInvalidValueFormatMalformed => Ok(from.as_any_box().downcast::<OvfInvalidValueFormatMalformed>()?),
            StructType::OvfInvalidValueReference => Ok(from.as_any_box().downcast::<OvfInvalidValueReference>()?),
            StructType::OvfMissingAttribute => Ok(from.as_any_box().downcast::<OvfMissingAttribute>()?),
            StructType::OvfConstraint => Ok(from.as_any_box().downcast::<OvfConstraint>()?),
            StructType::OvfDiskOrderConstraint => Ok(from.as_any_box().downcast::<OvfDiskOrderConstraint>()?),
            StructType::OvfHostResourceConstraint => Ok(from.as_any_box().downcast::<OvfHostResourceConstraint>()?),
            StructType::OvfElement => Ok(from.as_any_box().downcast::<OvfElement>()?),
            StructType::OvfDuplicateElement => Ok(from.as_any_box().downcast::<OvfDuplicateElement>()?),
            StructType::OvfDuplicatedElementBoundary => Ok(from.as_any_box().downcast::<OvfDuplicatedElementBoundary>()?),
            StructType::OvfElementInvalidValue => Ok(from.as_any_box().downcast::<OvfElementInvalidValue>()?),
            StructType::OvfMissingElement => Ok(from.as_any_box().downcast::<OvfMissingElement>()?),
            StructType::OvfMissingElementNormalBoundary => Ok(from.as_any_box().downcast::<OvfMissingElementNormalBoundary>()?),
            StructType::OvfUnexpectedElement => Ok(from.as_any_box().downcast::<OvfUnexpectedElement>()?),
            StructType::OvfWrongElement => Ok(from.as_any_box().downcast::<OvfWrongElement>()?),
            StructType::OvfProperty => Ok(from.as_any_box().downcast::<OvfProperty>()?),
            StructType::OvfPropertyNetwork => Ok(from.as_any_box().downcast::<OvfPropertyNetwork>()?),
            StructType::OvfPropertyQualifier => Ok(from.as_any_box().downcast::<OvfPropertyQualifier>()?),
            StructType::OvfPropertyQualifierDuplicate => Ok(from.as_any_box().downcast::<OvfPropertyQualifierDuplicate>()?),
            StructType::OvfPropertyQualifierIgnored => Ok(from.as_any_box().downcast::<OvfPropertyQualifierIgnored>()?),
            StructType::OvfPropertyType => Ok(from.as_any_box().downcast::<OvfPropertyType>()?),
            StructType::OvfPropertyValue => Ok(from.as_any_box().downcast::<OvfPropertyValue>()?),
            StructType::OvfWrongNamespace => Ok(from.as_any_box().downcast::<OvfWrongNamespace>()?),
            StructType::OvfXmlFormat => Ok(from.as_any_box().downcast::<OvfXmlFormat>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
