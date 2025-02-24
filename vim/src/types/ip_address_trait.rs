use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This is the abstract base class for IP address.
pub trait IpAddressTrait : super::negatable_expression_trait::NegatableExpressionTrait {
}
impl<'s> serde::Serialize for dyn IpAddressTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn IpAddressTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(IpAddressVisitor)
            }
        }

struct IpAddressVisitor;

impl<'de> de::Visitor<'de> for IpAddressVisitor {
    type Value = Box<dyn IpAddressTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid IpAddressTrait JSON object with a _typeName field")
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

impl IpAddressTrait for IpAddress {
}
impl IpAddressTrait for IpRange {
}
impl IpAddressTrait for SingleIp {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn IpAddressTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::IpAddress => Some(from.as_any_ref().downcast_ref::<IpAddress>()?),
            StructType::IpRange => Some(from.as_any_ref().downcast_ref::<IpRange>()?),
            StructType::SingleIp => Some(from.as_any_ref().downcast_ref::<SingleIp>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::IpAddress => Ok(from.as_any_box().downcast::<IpAddress>()?),
            StructType::IpRange => Ok(from.as_any_box().downcast::<IpRange>()?),
            StructType::SingleIp => Ok(from.as_any_box().downcast::<SingleIp>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
