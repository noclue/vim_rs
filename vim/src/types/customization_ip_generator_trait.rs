use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base type for the various IP specification possibilities.
pub trait CustomizationIpGeneratorTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn CustomizationIpGeneratorTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CustomizationIpGeneratorTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CustomizationIpGeneratorVisitor)
            }
        }

struct CustomizationIpGeneratorVisitor;

impl<'de> de::Visitor<'de> for CustomizationIpGeneratorVisitor {
    type Value = Box<dyn CustomizationIpGeneratorTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CustomizationIpGeneratorTrait JSON object with a _typeName field")
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

impl CustomizationIpGeneratorTrait for CustomizationIpGenerator {
}
impl CustomizationIpGeneratorTrait for CustomizationCustomIpGenerator {
}
impl CustomizationIpGeneratorTrait for CustomizationDhcpIpGenerator {
}
impl CustomizationIpGeneratorTrait for CustomizationFixedIp {
}
impl CustomizationIpGeneratorTrait for CustomizationUnknownIpGenerator {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CustomizationIpGeneratorTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationIpGenerator => Some(from.as_any_ref().downcast_ref::<CustomizationIpGenerator>()?),
            StructType::CustomizationCustomIpGenerator => Some(from.as_any_ref().downcast_ref::<CustomizationCustomIpGenerator>()?),
            StructType::CustomizationDhcpIpGenerator => Some(from.as_any_ref().downcast_ref::<CustomizationDhcpIpGenerator>()?),
            StructType::CustomizationFixedIp => Some(from.as_any_ref().downcast_ref::<CustomizationFixedIp>()?),
            StructType::CustomizationUnknownIpGenerator => Some(from.as_any_ref().downcast_ref::<CustomizationUnknownIpGenerator>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationIpGenerator => Ok(from.as_any_box().downcast::<CustomizationIpGenerator>()?),
            StructType::CustomizationCustomIpGenerator => Ok(from.as_any_box().downcast::<CustomizationCustomIpGenerator>()?),
            StructType::CustomizationDhcpIpGenerator => Ok(from.as_any_box().downcast::<CustomizationDhcpIpGenerator>()?),
            StructType::CustomizationFixedIp => Ok(from.as_any_box().downcast::<CustomizationFixedIp>()?),
            StructType::CustomizationUnknownIpGenerator => Ok(from.as_any_box().downcast::<CustomizationUnknownIpGenerator>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
