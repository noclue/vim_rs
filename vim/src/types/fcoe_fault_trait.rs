use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Deprecated as of vSphere API 8.0. Software FCoE not supported.
/// 
/// Base class for faults that can be thrown while invoking FCoE management operations.
pub trait FcoeFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn FcoeFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn FcoeFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(FcoeFaultVisitor)
            }
        }

struct FcoeFaultVisitor;

impl<'de> de::Visitor<'de> for FcoeFaultVisitor {
    type Value = Box<dyn FcoeFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid FcoeFaultTrait JSON object with a _typeName field")
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

impl FcoeFaultTrait for FcoeFault {
}
impl FcoeFaultTrait for FcoeFaultPnicHasNoPortSet {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn FcoeFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::FcoeFault => Some(from.as_any_ref().downcast_ref::<FcoeFault>()?),
            StructType::FcoeFaultPnicHasNoPortSet => Some(from.as_any_ref().downcast_ref::<FcoeFaultPnicHasNoPortSet>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::FcoeFault => Ok(from.as_any_box().downcast::<FcoeFault>()?),
            StructType::FcoeFaultPnicHasNoPortSet => Ok(from.as_any_box().downcast::<FcoeFaultPnicHasNoPortSet>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
