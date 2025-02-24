use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// DataTransportConnectionInfo contains common information about data transport
/// connections on a host.
/// 
/// ***Since:*** vSphere API Release 7.0.3.0
pub trait HostDataTransportConnectionInfoTrait : super::data_object_trait::DataObjectTrait {
    /// Static memory consumption by a connection in bytes like buffer sizes, heap sizes, etc.
    fn get_static_memory_consumed(&self) -> i64;
}
impl<'s> serde::Serialize for dyn HostDataTransportConnectionInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostDataTransportConnectionInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostDataTransportConnectionInfoVisitor)
            }
        }

struct HostDataTransportConnectionInfoVisitor;

impl<'de> de::Visitor<'de> for HostDataTransportConnectionInfoVisitor {
    type Value = Box<dyn HostDataTransportConnectionInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostDataTransportConnectionInfoTrait JSON object with a _typeName field")
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

impl HostDataTransportConnectionInfoTrait for HostDataTransportConnectionInfo {
    fn get_static_memory_consumed(&self) -> i64 { self.static_memory_consumed }
}
impl HostDataTransportConnectionInfoTrait for HostNfcConnectionInfo {
    fn get_static_memory_consumed(&self) -> i64 { self.static_memory_consumed }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostDataTransportConnectionInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostDataTransportConnectionInfo => Some(from.as_any_ref().downcast_ref::<HostDataTransportConnectionInfo>()?),
            StructType::HostNfcConnectionInfo => Some(from.as_any_ref().downcast_ref::<HostNfcConnectionInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostDataTransportConnectionInfo => Ok(from.as_any_box().downcast::<HostDataTransportConnectionInfo>()?),
            StructType::HostNfcConnectionInfo => Ok(from.as_any_box().downcast::<HostNfcConnectionInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
