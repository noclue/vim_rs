use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A PlatformConfigFault is a catch-all fault indicating that some error has
/// occurred regarding the configuration of the host.
/// 
/// Data about the fault is
/// available and will be presented as a platform specific string.
/// 
/// This information carried by this fault cannot be localized. Most likely
/// this information will already have been localized to the locale of the
/// server that generated this fault. Where possible, a more specific fault
/// will be thrown.
pub trait PlatformConfigFaultTrait : super::host_config_fault_trait::HostConfigFaultTrait {
    /// Platform specific text string describing this error.
    fn get_text(&self) -> &str;
}
impl<'s> serde::Serialize for dyn PlatformConfigFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn PlatformConfigFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(PlatformConfigFaultVisitor)
            }
        }

struct PlatformConfigFaultVisitor;

impl<'de> de::Visitor<'de> for PlatformConfigFaultVisitor {
    type Value = Box<dyn PlatformConfigFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid PlatformConfigFaultTrait JSON object with a _typeName field")
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

impl PlatformConfigFaultTrait for PlatformConfigFault {
    fn get_text(&self) -> &str { &self.text }
}
impl PlatformConfigFaultTrait for InvalidBundle {
    fn get_text(&self) -> &str { &self.text }
}
impl PlatformConfigFaultTrait for PatchInstallFailed {
    fn get_text(&self) -> &str { &self.text }
}
impl PlatformConfigFaultTrait for PatchIntegrityError {
    fn get_text(&self) -> &str { &self.text }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn PlatformConfigFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::PlatformConfigFault => Some(from.as_any_ref().downcast_ref::<PlatformConfigFault>()?),
            StructType::InvalidBundle => Some(from.as_any_ref().downcast_ref::<InvalidBundle>()?),
            StructType::PatchInstallFailed => Some(from.as_any_ref().downcast_ref::<PatchInstallFailed>()?),
            StructType::PatchIntegrityError => Some(from.as_any_ref().downcast_ref::<PatchIntegrityError>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::PlatformConfigFault => Ok(from.as_any_box().downcast::<PlatformConfigFault>()?),
            StructType::InvalidBundle => Ok(from.as_any_box().downcast::<InvalidBundle>()?),
            StructType::PatchInstallFailed => Ok(from.as_any_box().downcast::<PatchInstallFailed>()?),
            StructType::PatchIntegrityError => Ok(from.as_any_box().downcast::<PatchIntegrityError>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
