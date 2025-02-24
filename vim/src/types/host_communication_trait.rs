use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A HostCommunication fault is thrown if an error happened
/// while communicating to a host.
/// 
/// This would typically be
/// due to network connections or server failures.
pub trait HostCommunicationTrait : super::runtime_fault_trait::RuntimeFaultTrait {
}
impl<'s> serde::Serialize for dyn HostCommunicationTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostCommunicationTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostCommunicationVisitor)
            }
        }

struct HostCommunicationVisitor;

impl<'de> de::Visitor<'de> for HostCommunicationVisitor {
    type Value = Box<dyn HostCommunicationTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostCommunicationTrait JSON object with a _typeName field")
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

impl HostCommunicationTrait for HostCommunication {
}
impl HostCommunicationTrait for HostNotConnected {
}
impl HostCommunicationTrait for HostNotReachable {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostCommunicationTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostCommunication => Some(from.as_any_ref().downcast_ref::<HostCommunication>()?),
            StructType::HostNotConnected => Some(from.as_any_ref().downcast_ref::<HostNotConnected>()?),
            StructType::HostNotReachable => Some(from.as_any_ref().downcast_ref::<HostNotReachable>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostCommunication => Ok(from.as_any_box().downcast::<HostCommunication>()?),
            StructType::HostNotConnected => Ok(from.as_any_box().downcast::<HostNotConnected>()?),
            StructType::HostNotReachable => Ok(from.as_any_box().downcast::<HostNotReachable>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
