use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base object type for optional operations supported by the customization process.
pub trait CustomizationOptionsTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn CustomizationOptionsTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CustomizationOptionsTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CustomizationOptionsVisitor)
            }
        }

struct CustomizationOptionsVisitor;

impl<'de> de::Visitor<'de> for CustomizationOptionsVisitor {
    type Value = Box<dyn CustomizationOptionsTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CustomizationOptionsTrait JSON object with a _typeName field")
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

impl CustomizationOptionsTrait for CustomizationOptions {
}
impl CustomizationOptionsTrait for CustomizationLinuxOptions {
}
impl CustomizationOptionsTrait for CustomizationWinOptions {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CustomizationOptionsTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationOptions => Some(from.as_any_ref().downcast_ref::<CustomizationOptions>()?),
            StructType::CustomizationLinuxOptions => Some(from.as_any_ref().downcast_ref::<CustomizationLinuxOptions>()?),
            StructType::CustomizationWinOptions => Some(from.as_any_ref().downcast_ref::<CustomizationWinOptions>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationOptions => Ok(from.as_any_box().downcast::<CustomizationOptions>()?),
            StructType::CustomizationLinuxOptions => Ok(from.as_any_box().downcast::<CustomizationLinuxOptions>()?),
            StructType::CustomizationWinOptions => Ok(from.as_any_box().downcast::<CustomizationWinOptions>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
