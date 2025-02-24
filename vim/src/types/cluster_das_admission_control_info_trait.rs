use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for admission control related information of a vSphere HA cluster.
pub trait ClusterDasAdmissionControlInfoTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn ClusterDasAdmissionControlInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterDasAdmissionControlInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterDasAdmissionControlInfoVisitor)
            }
        }

struct ClusterDasAdmissionControlInfoVisitor;

impl<'de> de::Visitor<'de> for ClusterDasAdmissionControlInfoVisitor {
    type Value = Box<dyn ClusterDasAdmissionControlInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterDasAdmissionControlInfoTrait JSON object with a _typeName field")
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

impl ClusterDasAdmissionControlInfoTrait for ClusterDasAdmissionControlInfo {
}
impl ClusterDasAdmissionControlInfoTrait for ClusterFailoverHostAdmissionControlInfo {
}
impl ClusterDasAdmissionControlInfoTrait for ClusterFailoverLevelAdmissionControlInfo {
}
impl ClusterDasAdmissionControlInfoTrait for ClusterFailoverResourcesAdmissionControlInfo {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterDasAdmissionControlInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterDasAdmissionControlInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasAdmissionControlInfo>()?),
            StructType::ClusterFailoverHostAdmissionControlInfo => Some(from.as_any_ref().downcast_ref::<ClusterFailoverHostAdmissionControlInfo>()?),
            StructType::ClusterFailoverLevelAdmissionControlInfo => Some(from.as_any_ref().downcast_ref::<ClusterFailoverLevelAdmissionControlInfo>()?),
            StructType::ClusterFailoverResourcesAdmissionControlInfo => Some(from.as_any_ref().downcast_ref::<ClusterFailoverResourcesAdmissionControlInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterDasAdmissionControlInfo => Ok(from.as_any_box().downcast::<ClusterDasAdmissionControlInfo>()?),
            StructType::ClusterFailoverHostAdmissionControlInfo => Ok(from.as_any_box().downcast::<ClusterFailoverHostAdmissionControlInfo>()?),
            StructType::ClusterFailoverLevelAdmissionControlInfo => Ok(from.as_any_box().downcast::<ClusterFailoverLevelAdmissionControlInfo>()?),
            StructType::ClusterFailoverResourcesAdmissionControlInfo => Ok(from.as_any_box().downcast::<ClusterFailoverResourcesAdmissionControlInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
