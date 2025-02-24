use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for DAS data for high availability service for a cluster.
pub trait ClusterDasDataTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn ClusterDasDataTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterDasDataTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterDasDataVisitor)
            }
        }

struct ClusterDasDataVisitor;

impl<'de> de::Visitor<'de> for ClusterDasDataVisitor {
    type Value = Box<dyn ClusterDasDataTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterDasDataTrait JSON object with a _typeName field")
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

impl ClusterDasDataTrait for ClusterDasData {
}
impl ClusterDasDataTrait for ClusterDasDataSummary {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterDasDataTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterDasData => Some(from.as_any_ref().downcast_ref::<ClusterDasData>()?),
            StructType::ClusterDasDataSummary => Some(from.as_any_ref().downcast_ref::<ClusterDasDataSummary>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterDasData => Ok(from.as_any_box().downcast::<ClusterDasData>()?),
            StructType::ClusterDasDataSummary => Ok(from.as_any_box().downcast::<ClusterDasDataSummary>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
