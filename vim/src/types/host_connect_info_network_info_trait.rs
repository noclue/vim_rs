use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The base data object type for information about networks on the host.
pub trait HostConnectInfoNetworkInfoTrait : super::data_object_trait::DataObjectTrait {
    /// Basic network information, such as network name.
    /// 
    /// The managed object reference
    /// is not set.
    fn get_summary(&self) -> &Box<dyn super::network_summary_trait::NetworkSummaryTrait>;
}
impl<'s> serde::Serialize for dyn HostConnectInfoNetworkInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostConnectInfoNetworkInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostConnectInfoNetworkInfoVisitor)
            }
        }

struct HostConnectInfoNetworkInfoVisitor;

impl<'de> de::Visitor<'de> for HostConnectInfoNetworkInfoVisitor {
    type Value = Box<dyn HostConnectInfoNetworkInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostConnectInfoNetworkInfoTrait JSON object with a _typeName field")
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

impl HostConnectInfoNetworkInfoTrait for HostConnectInfoNetworkInfo {
    fn get_summary(&self) -> &Box<dyn super::network_summary_trait::NetworkSummaryTrait> { &self.summary }
}
impl HostConnectInfoNetworkInfoTrait for HostNewNetworkConnectInfo {
    fn get_summary(&self) -> &Box<dyn super::network_summary_trait::NetworkSummaryTrait> { &self.summary }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostConnectInfoNetworkInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostConnectInfoNetworkInfo => Some(from.as_any_ref().downcast_ref::<HostConnectInfoNetworkInfo>()?),
            StructType::HostNewNetworkConnectInfo => Some(from.as_any_ref().downcast_ref::<HostNewNetworkConnectInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostConnectInfoNetworkInfo => Ok(from.as_any_box().downcast::<HostConnectInfoNetworkInfo>()?),
            StructType::HostNewNetworkConnectInfo => Ok(from.as_any_box().downcast::<HostNewNetworkConnectInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
