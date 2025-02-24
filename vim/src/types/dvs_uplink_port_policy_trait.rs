use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The base class for uplink port policy.
pub trait DvsUplinkPortPolicyTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn DvsUplinkPortPolicyTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsUplinkPortPolicyTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsUplinkPortPolicyVisitor)
            }
        }

struct DvsUplinkPortPolicyVisitor;

impl<'de> de::Visitor<'de> for DvsUplinkPortPolicyVisitor {
    type Value = Box<dyn DvsUplinkPortPolicyTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsUplinkPortPolicyTrait JSON object with a _typeName field")
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

impl DvsUplinkPortPolicyTrait for DvsUplinkPortPolicy {
}
impl DvsUplinkPortPolicyTrait for DvsNameArrayUplinkPortPolicy {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsUplinkPortPolicyTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsUplinkPortPolicy => Some(from.as_any_ref().downcast_ref::<DvsUplinkPortPolicy>()?),
            StructType::DvsNameArrayUplinkPortPolicy => Some(from.as_any_ref().downcast_ref::<DvsNameArrayUplinkPortPolicy>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsUplinkPortPolicy => Ok(from.as_any_box().downcast::<DvsUplinkPortPolicy>()?),
            StructType::DvsNameArrayUplinkPortPolicy => Ok(from.as_any_box().downcast::<DvsNameArrayUplinkPortPolicy>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
