use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for specifying Ports.
/// 
/// Objects of the base class represent any port (single/range/list).
pub trait DvsIpPortTrait : super::negatable_expression_trait::NegatableExpressionTrait {
}
impl<'s> serde::Serialize for dyn DvsIpPortTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsIpPortTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsIpPortVisitor)
            }
        }

struct DvsIpPortVisitor;

impl<'de> de::Visitor<'de> for DvsIpPortVisitor {
    type Value = Box<dyn DvsIpPortTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsIpPortTrait JSON object with a _typeName field")
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

impl DvsIpPortTrait for DvsIpPort {
}
impl DvsIpPortTrait for DvsIpPortRange {
}
impl DvsIpPortTrait for DvsSingleIpPort {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsIpPortTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsIpPort => Some(from.as_any_ref().downcast_ref::<DvsIpPort>()?),
            StructType::DvsIpPortRange => Some(from.as_any_ref().downcast_ref::<DvsIpPortRange>()?),
            StructType::DvsSingleIpPort => Some(from.as_any_ref().downcast_ref::<DvsSingleIpPort>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsIpPort => Ok(from.as_any_box().downcast::<DvsIpPort>()?),
            StructType::DvsIpPortRange => Ok(from.as_any_box().downcast::<DvsIpPortRange>()?),
            StructType::DvsSingleIpPort => Ok(from.as_any_box().downcast::<DvsSingleIpPort>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
