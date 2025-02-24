use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for specifying MAC addresses.
pub trait MacAddressTrait : super::negatable_expression_trait::NegatableExpressionTrait {
}
impl<'s> serde::Serialize for dyn MacAddressTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn MacAddressTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(MacAddressVisitor)
            }
        }

struct MacAddressVisitor;

impl<'de> de::Visitor<'de> for MacAddressVisitor {
    type Value = Box<dyn MacAddressTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid MacAddressTrait JSON object with a _typeName field")
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

impl MacAddressTrait for MacAddress {
}
impl MacAddressTrait for MacRange {
}
impl MacAddressTrait for SingleMac {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn MacAddressTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::MacAddress => Some(from.as_any_ref().downcast_ref::<MacAddress>()?),
            StructType::MacRange => Some(from.as_any_ref().downcast_ref::<MacRange>()?),
            StructType::SingleMac => Some(from.as_any_ref().downcast_ref::<SingleMac>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::MacAddress => Ok(from.as_any_box().downcast::<MacAddress>()?),
            StructType::MacRange => Ok(from.as_any_box().downcast::<MacRange>()?),
            StructType::SingleMac => Ok(from.as_any_box().downcast::<SingleMac>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
