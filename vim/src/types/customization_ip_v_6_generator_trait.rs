use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base type for the various IpV6 specification possibilities
pub trait CustomizationIpV6GeneratorTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn CustomizationIpV6GeneratorTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CustomizationIpV6GeneratorTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CustomizationIpV6GeneratorVisitor)
            }
        }

struct CustomizationIpV6GeneratorVisitor;

impl<'de> de::Visitor<'de> for CustomizationIpV6GeneratorVisitor {
    type Value = Box<dyn CustomizationIpV6GeneratorTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CustomizationIpV6GeneratorTrait JSON object with a _typeName field")
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

impl CustomizationIpV6GeneratorTrait for CustomizationIpV6Generator {
}
impl CustomizationIpV6GeneratorTrait for CustomizationAutoIpV6Generator {
}
impl CustomizationIpV6GeneratorTrait for CustomizationCustomIpV6Generator {
}
impl CustomizationIpV6GeneratorTrait for CustomizationDhcpIpV6Generator {
}
impl CustomizationIpV6GeneratorTrait for CustomizationFixedIpV6 {
}
impl CustomizationIpV6GeneratorTrait for CustomizationStatelessIpV6Generator {
}
impl CustomizationIpV6GeneratorTrait for CustomizationUnknownIpV6Generator {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CustomizationIpV6GeneratorTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationIpV6Generator>()?),
            StructType::CustomizationAutoIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationAutoIpV6Generator>()?),
            StructType::CustomizationCustomIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationCustomIpV6Generator>()?),
            StructType::CustomizationDhcpIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationDhcpIpV6Generator>()?),
            StructType::CustomizationFixedIpV6 => Some(from.as_any_ref().downcast_ref::<CustomizationFixedIpV6>()?),
            StructType::CustomizationStatelessIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationStatelessIpV6Generator>()?),
            StructType::CustomizationUnknownIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationUnknownIpV6Generator>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationIpV6Generator>()?),
            StructType::CustomizationAutoIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationAutoIpV6Generator>()?),
            StructType::CustomizationCustomIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationCustomIpV6Generator>()?),
            StructType::CustomizationDhcpIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationDhcpIpV6Generator>()?),
            StructType::CustomizationFixedIpV6 => Ok(from.as_any_box().downcast::<CustomizationFixedIpV6>()?),
            StructType::CustomizationStatelessIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationStatelessIpV6Generator>()?),
            StructType::CustomizationUnknownIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationUnknownIpV6Generator>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
