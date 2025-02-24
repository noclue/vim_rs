use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// *HostProfileConfigSpec* is the base data object
/// for all *HostProfile* configuration specifications.
pub trait HostProfileConfigSpecTrait : super::profile_create_spec_trait::ProfileCreateSpecTrait {
}
impl<'s> serde::Serialize for dyn HostProfileConfigSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostProfileConfigSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostProfileConfigSpecVisitor)
            }
        }

struct HostProfileConfigSpecVisitor;

impl<'de> de::Visitor<'de> for HostProfileConfigSpecVisitor {
    type Value = Box<dyn HostProfileConfigSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostProfileConfigSpecTrait JSON object with a _typeName field")
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

impl HostProfileConfigSpecTrait for HostProfileConfigSpec {
}
impl HostProfileConfigSpecTrait for HostProfileCompleteConfigSpec {
}
impl HostProfileConfigSpecTrait for HostProfileHostBasedConfigSpec {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostProfileConfigSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostProfileConfigSpec => Some(from.as_any_ref().downcast_ref::<HostProfileConfigSpec>()?),
            StructType::HostProfileCompleteConfigSpec => Some(from.as_any_ref().downcast_ref::<HostProfileCompleteConfigSpec>()?),
            StructType::HostProfileHostBasedConfigSpec => Some(from.as_any_ref().downcast_ref::<HostProfileHostBasedConfigSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostProfileConfigSpec => Ok(from.as_any_box().downcast::<HostProfileConfigSpec>()?),
            StructType::HostProfileCompleteConfigSpec => Ok(from.as_any_box().downcast::<HostProfileCompleteConfigSpec>()?),
            StructType::HostProfileHostBasedConfigSpec => Ok(from.as_any_box().downcast::<HostProfileHostBasedConfigSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
