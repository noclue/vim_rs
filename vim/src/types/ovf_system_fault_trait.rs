use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A common base class to host all the OVF subsystems's system faults.
/// 
/// This is a class of fault that can be thrown because of
/// some api changes, new hardware that are not supported by
/// the host.
pub trait OvfSystemFaultTrait : super::ovf_fault_trait::OvfFaultTrait {
}
impl<'s> serde::Serialize for dyn OvfSystemFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfSystemFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfSystemFaultVisitor)
            }
        }

struct OvfSystemFaultVisitor;

impl<'de> de::Visitor<'de> for OvfSystemFaultVisitor {
    type Value = Box<dyn OvfSystemFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfSystemFaultTrait JSON object with a _typeName field")
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

impl OvfSystemFaultTrait for OvfSystemFault {
}
impl OvfSystemFaultTrait for OvfDiskMappingNotFound {
}
impl OvfSystemFaultTrait for OvfHostValueNotParsed {
}
impl OvfSystemFaultTrait for OvfInternalError {
}
impl OvfSystemFaultTrait for OvfToXmlUnsupportedElement {
}
impl OvfSystemFaultTrait for OvfUnknownDevice {
}
impl OvfSystemFaultTrait for OvfUnknownEntity {
}
impl OvfSystemFaultTrait for OvfUnsupportedDeviceBackingInfo {
}
impl OvfSystemFaultTrait for OvfUnsupportedDeviceBackingOption {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfSystemFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfSystemFault => Some(from.as_any_ref().downcast_ref::<OvfSystemFault>()?),
            StructType::OvfDiskMappingNotFound => Some(from.as_any_ref().downcast_ref::<OvfDiskMappingNotFound>()?),
            StructType::OvfHostValueNotParsed => Some(from.as_any_ref().downcast_ref::<OvfHostValueNotParsed>()?),
            StructType::OvfInternalError => Some(from.as_any_ref().downcast_ref::<OvfInternalError>()?),
            StructType::OvfToXmlUnsupportedElement => Some(from.as_any_ref().downcast_ref::<OvfToXmlUnsupportedElement>()?),
            StructType::OvfUnknownDevice => Some(from.as_any_ref().downcast_ref::<OvfUnknownDevice>()?),
            StructType::OvfUnknownEntity => Some(from.as_any_ref().downcast_ref::<OvfUnknownEntity>()?),
            StructType::OvfUnsupportedDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDeviceBackingInfo>()?),
            StructType::OvfUnsupportedDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDeviceBackingOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfSystemFault => Ok(from.as_any_box().downcast::<OvfSystemFault>()?),
            StructType::OvfDiskMappingNotFound => Ok(from.as_any_box().downcast::<OvfDiskMappingNotFound>()?),
            StructType::OvfHostValueNotParsed => Ok(from.as_any_box().downcast::<OvfHostValueNotParsed>()?),
            StructType::OvfInternalError => Ok(from.as_any_box().downcast::<OvfInternalError>()?),
            StructType::OvfToXmlUnsupportedElement => Ok(from.as_any_box().downcast::<OvfToXmlUnsupportedElement>()?),
            StructType::OvfUnknownDevice => Ok(from.as_any_box().downcast::<OvfUnknownDevice>()?),
            StructType::OvfUnknownEntity => Ok(from.as_any_box().downcast::<OvfUnknownEntity>()?),
            StructType::OvfUnsupportedDeviceBackingInfo => Ok(from.as_any_box().downcast::<OvfUnsupportedDeviceBackingInfo>()?),
            StructType::OvfUnsupportedDeviceBackingOption => Ok(from.as_any_box().downcast::<OvfUnsupportedDeviceBackingOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
