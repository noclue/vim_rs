use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base type for Replication-related errors.
pub trait ReplicationFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn ReplicationFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ReplicationFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ReplicationFaultVisitor)
            }
        }

struct ReplicationFaultVisitor;

impl<'de> de::Visitor<'de> for ReplicationFaultVisitor {
    type Value = Box<dyn ReplicationFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ReplicationFaultTrait JSON object with a _typeName field")
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

impl ReplicationFaultTrait for ReplicationFault {
}
impl ReplicationFaultTrait for IncompatibleHostForVmReplication {
}
impl ReplicationFaultTrait for ReplicationConfigFault {
}
impl ReplicationFaultTrait for ReplicationDiskConfigFault {
}
impl ReplicationFaultTrait for ReplicationVmConfigFault {
}
impl ReplicationFaultTrait for ReplicationIncompatibleWithFt {
}
impl ReplicationFaultTrait for ReplicationInvalidOptions {
}
impl ReplicationFaultTrait for ReplicationNotSupportedOnHost {
}
impl ReplicationFaultTrait for ReplicationVmFault {
}
impl ReplicationFaultTrait for ReplicationVmInProgressFault {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ReplicationFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ReplicationFault => Some(from.as_any_ref().downcast_ref::<ReplicationFault>()?),
            StructType::IncompatibleHostForVmReplication => Some(from.as_any_ref().downcast_ref::<IncompatibleHostForVmReplication>()?),
            StructType::ReplicationConfigFault => Some(from.as_any_ref().downcast_ref::<ReplicationConfigFault>()?),
            StructType::ReplicationDiskConfigFault => Some(from.as_any_ref().downcast_ref::<ReplicationDiskConfigFault>()?),
            StructType::ReplicationVmConfigFault => Some(from.as_any_ref().downcast_ref::<ReplicationVmConfigFault>()?),
            StructType::ReplicationIncompatibleWithFt => Some(from.as_any_ref().downcast_ref::<ReplicationIncompatibleWithFt>()?),
            StructType::ReplicationInvalidOptions => Some(from.as_any_ref().downcast_ref::<ReplicationInvalidOptions>()?),
            StructType::ReplicationNotSupportedOnHost => Some(from.as_any_ref().downcast_ref::<ReplicationNotSupportedOnHost>()?),
            StructType::ReplicationVmFault => Some(from.as_any_ref().downcast_ref::<ReplicationVmFault>()?),
            StructType::ReplicationVmInProgressFault => Some(from.as_any_ref().downcast_ref::<ReplicationVmInProgressFault>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ReplicationFault => Ok(from.as_any_box().downcast::<ReplicationFault>()?),
            StructType::IncompatibleHostForVmReplication => Ok(from.as_any_box().downcast::<IncompatibleHostForVmReplication>()?),
            StructType::ReplicationConfigFault => Ok(from.as_any_box().downcast::<ReplicationConfigFault>()?),
            StructType::ReplicationDiskConfigFault => Ok(from.as_any_box().downcast::<ReplicationDiskConfigFault>()?),
            StructType::ReplicationVmConfigFault => Ok(from.as_any_box().downcast::<ReplicationVmConfigFault>()?),
            StructType::ReplicationIncompatibleWithFt => Ok(from.as_any_box().downcast::<ReplicationIncompatibleWithFt>()?),
            StructType::ReplicationInvalidOptions => Ok(from.as_any_box().downcast::<ReplicationInvalidOptions>()?),
            StructType::ReplicationNotSupportedOnHost => Ok(from.as_any_box().downcast::<ReplicationNotSupportedOnHost>()?),
            StructType::ReplicationVmFault => Ok(from.as_any_box().downcast::<ReplicationVmFault>()?),
            StructType::ReplicationVmInProgressFault => Ok(from.as_any_box().downcast::<ReplicationVmInProgressFault>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
