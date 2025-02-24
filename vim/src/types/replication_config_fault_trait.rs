use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base type for Replication-related configuration errors.
pub trait ReplicationConfigFaultTrait : super::replication_fault_trait::ReplicationFaultTrait {
}
impl<'s> serde::Serialize for dyn ReplicationConfigFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ReplicationConfigFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ReplicationConfigFaultVisitor)
            }
        }

struct ReplicationConfigFaultVisitor;

impl<'de> de::Visitor<'de> for ReplicationConfigFaultVisitor {
    type Value = Box<dyn ReplicationConfigFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ReplicationConfigFaultTrait JSON object with a _typeName field")
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

impl ReplicationConfigFaultTrait for ReplicationConfigFault {
}
impl ReplicationConfigFaultTrait for ReplicationDiskConfigFault {
}
impl ReplicationConfigFaultTrait for ReplicationVmConfigFault {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ReplicationConfigFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ReplicationConfigFault => Some(from.as_any_ref().downcast_ref::<ReplicationConfigFault>()?),
            StructType::ReplicationDiskConfigFault => Some(from.as_any_ref().downcast_ref::<ReplicationDiskConfigFault>()?),
            StructType::ReplicationVmConfigFault => Some(from.as_any_ref().downcast_ref::<ReplicationVmConfigFault>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ReplicationConfigFault => Ok(from.as_any_box().downcast::<ReplicationConfigFault>()?),
            StructType::ReplicationDiskConfigFault => Ok(from.as_any_box().downcast::<ReplicationDiskConfigFault>()?),
            StructType::ReplicationVmConfigFault => Ok(from.as_any_box().downcast::<ReplicationVmConfigFault>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
