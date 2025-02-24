use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This fault is thrown when Distributed Power Management cannot perform
/// a given opeartion because there are insufficient CPU/memory resources
/// on standby hosts (if any) to meet the requirements of the operation.
pub trait InsufficientStandbyResourceTrait : super::insufficient_resources_fault_trait::InsufficientResourcesFaultTrait {
}
impl<'s> serde::Serialize for dyn InsufficientStandbyResourceTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InsufficientStandbyResourceTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InsufficientStandbyResourceVisitor)
            }
        }

struct InsufficientStandbyResourceVisitor;

impl<'de> de::Visitor<'de> for InsufficientStandbyResourceVisitor {
    type Value = Box<dyn InsufficientStandbyResourceTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InsufficientStandbyResourceTrait JSON object with a _typeName field")
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

impl InsufficientStandbyResourceTrait for InsufficientStandbyResource {
}
impl InsufficientStandbyResourceTrait for InsufficientStandbyCpuResource {
}
impl InsufficientStandbyResourceTrait for InsufficientStandbyMemoryResource {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InsufficientStandbyResourceTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InsufficientStandbyResource => Some(from.as_any_ref().downcast_ref::<InsufficientStandbyResource>()?),
            StructType::InsufficientStandbyCpuResource => Some(from.as_any_ref().downcast_ref::<InsufficientStandbyCpuResource>()?),
            StructType::InsufficientStandbyMemoryResource => Some(from.as_any_ref().downcast_ref::<InsufficientStandbyMemoryResource>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InsufficientStandbyResource => Ok(from.as_any_box().downcast::<InsufficientStandbyResource>()?),
            StructType::InsufficientStandbyCpuResource => Ok(from.as_any_box().downcast::<InsufficientStandbyCpuResource>()?),
            StructType::InsufficientStandbyMemoryResource => Ok(from.as_any_box().downcast::<InsufficientStandbyMemoryResource>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
