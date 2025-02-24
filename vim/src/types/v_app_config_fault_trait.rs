use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base for configuration / environment issues that can be thrown when powering on or
/// changing the configuration of a vApp.
pub trait VAppConfigFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn VAppConfigFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VAppConfigFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VAppConfigFaultVisitor)
            }
        }

struct VAppConfigFaultVisitor;

impl<'de> de::Visitor<'de> for VAppConfigFaultVisitor {
    type Value = Box<dyn VAppConfigFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VAppConfigFaultTrait JSON object with a _typeName field")
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

impl VAppConfigFaultTrait for VAppConfigFault {
}
impl VAppConfigFaultTrait for MissingPowerOffConfiguration {
}
impl VAppConfigFaultTrait for MissingPowerOnConfiguration {
}
impl VAppConfigFaultTrait for NoVmInVApp {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VAppConfigFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VAppConfigFault => Some(from.as_any_ref().downcast_ref::<VAppConfigFault>()?),
            StructType::MissingPowerOffConfiguration => Some(from.as_any_ref().downcast_ref::<MissingPowerOffConfiguration>()?),
            StructType::MissingPowerOnConfiguration => Some(from.as_any_ref().downcast_ref::<MissingPowerOnConfiguration>()?),
            StructType::NoVmInVApp => Some(from.as_any_ref().downcast_ref::<NoVmInVApp>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VAppConfigFault => Ok(from.as_any_box().downcast::<VAppConfigFault>()?),
            StructType::MissingPowerOffConfiguration => Ok(from.as_any_box().downcast::<MissingPowerOffConfiguration>()?),
            StructType::MissingPowerOnConfiguration => Ok(from.as_any_box().downcast::<MissingPowerOnConfiguration>()?),
            StructType::NoVmInVApp => Ok(from.as_any_box().downcast::<NoVmInVApp>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
