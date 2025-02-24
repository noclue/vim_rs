use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// DataObject which is a baseclass for other configuration
/// specifications.
pub trait ClusterProfileConfigSpecTrait : super::cluster_profile_create_spec_trait::ClusterProfileCreateSpecTrait {
}
impl<'s> serde::Serialize for dyn ClusterProfileConfigSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterProfileConfigSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterProfileConfigSpecVisitor)
            }
        }

struct ClusterProfileConfigSpecVisitor;

impl<'de> de::Visitor<'de> for ClusterProfileConfigSpecVisitor {
    type Value = Box<dyn ClusterProfileConfigSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterProfileConfigSpecTrait JSON object with a _typeName field")
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

impl ClusterProfileConfigSpecTrait for ClusterProfileConfigSpec {
}
impl ClusterProfileConfigSpecTrait for ClusterProfileCompleteConfigSpec {
}
impl ClusterProfileConfigSpecTrait for ClusterProfileConfigServiceCreateSpec {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterProfileConfigSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterProfileConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterProfileConfigSpec>()?),
            StructType::ClusterProfileCompleteConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterProfileCompleteConfigSpec>()?),
            StructType::ClusterProfileConfigServiceCreateSpec => Some(from.as_any_ref().downcast_ref::<ClusterProfileConfigServiceCreateSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterProfileConfigSpec => Ok(from.as_any_box().downcast::<ClusterProfileConfigSpec>()?),
            StructType::ClusterProfileCompleteConfigSpec => Ok(from.as_any_box().downcast::<ClusterProfileCompleteConfigSpec>()?),
            StructType::ClusterProfileConfigServiceCreateSpec => Ok(from.as_any_box().downcast::<ClusterProfileConfigServiceCreateSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
