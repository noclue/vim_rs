use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A common base class for errors that can happen during Import and
/// that is not due to an invalid package (OvfInvalidPackage).
/// 
/// These
/// are typically issued as warnings.
pub trait OvfImportTrait : super::ovf_fault_trait::OvfFaultTrait {
}
impl<'s> serde::Serialize for dyn OvfImportTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfImportTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfImportVisitor)
            }
        }

struct OvfImportVisitor;

impl<'de> de::Visitor<'de> for OvfImportVisitor {
    type Value = Box<dyn OvfImportTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfImportTrait JSON object with a _typeName field")
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

impl OvfImportTrait for OvfImport {
}
impl OvfImportTrait for OvfCpuCompatibility {
}
impl OvfImportTrait for OvfCpuCompatibilityCheckNotSupported {
}
impl OvfImportTrait for OvfHardwareCheck {
}
impl OvfImportTrait for OvfImportFailed {
}
impl OvfImportTrait for OvfMappedOsId {
}
impl OvfImportTrait for OvfMissingHardware {
}
impl OvfImportTrait for OvfNetworkMappingNotSupported {
}
impl OvfImportTrait for OvfUnsupportedDiskProvisioning {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfImportTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfImport => Some(from.as_any_ref().downcast_ref::<OvfImport>()?),
            StructType::OvfCpuCompatibility => Some(from.as_any_ref().downcast_ref::<OvfCpuCompatibility>()?),
            StructType::OvfCpuCompatibilityCheckNotSupported => Some(from.as_any_ref().downcast_ref::<OvfCpuCompatibilityCheckNotSupported>()?),
            StructType::OvfHardwareCheck => Some(from.as_any_ref().downcast_ref::<OvfHardwareCheck>()?),
            StructType::OvfImportFailed => Some(from.as_any_ref().downcast_ref::<OvfImportFailed>()?),
            StructType::OvfMappedOsId => Some(from.as_any_ref().downcast_ref::<OvfMappedOsId>()?),
            StructType::OvfMissingHardware => Some(from.as_any_ref().downcast_ref::<OvfMissingHardware>()?),
            StructType::OvfNetworkMappingNotSupported => Some(from.as_any_ref().downcast_ref::<OvfNetworkMappingNotSupported>()?),
            StructType::OvfUnsupportedDiskProvisioning => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDiskProvisioning>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfImport => Ok(from.as_any_box().downcast::<OvfImport>()?),
            StructType::OvfCpuCompatibility => Ok(from.as_any_box().downcast::<OvfCpuCompatibility>()?),
            StructType::OvfCpuCompatibilityCheckNotSupported => Ok(from.as_any_box().downcast::<OvfCpuCompatibilityCheckNotSupported>()?),
            StructType::OvfHardwareCheck => Ok(from.as_any_box().downcast::<OvfHardwareCheck>()?),
            StructType::OvfImportFailed => Ok(from.as_any_box().downcast::<OvfImportFailed>()?),
            StructType::OvfMappedOsId => Ok(from.as_any_box().downcast::<OvfMappedOsId>()?),
            StructType::OvfMissingHardware => Ok(from.as_any_box().downcast::<OvfMissingHardware>()?),
            StructType::OvfNetworkMappingNotSupported => Ok(from.as_any_box().downcast::<OvfNetworkMappingNotSupported>()?),
            StructType::OvfUnsupportedDiskProvisioning => Ok(from.as_any_box().downcast::<OvfUnsupportedDiskProvisioning>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
