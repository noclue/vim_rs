use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A base object type for a virtual machine name that can be either fixed or
/// auto-generated.
pub trait CustomizationNameTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn CustomizationNameTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CustomizationNameTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CustomizationNameVisitor)
            }
        }

struct CustomizationNameVisitor;

impl<'de> de::Visitor<'de> for CustomizationNameVisitor {
    type Value = Box<dyn CustomizationNameTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CustomizationNameTrait JSON object with a _typeName field")
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

impl CustomizationNameTrait for CustomizationName {
}
impl CustomizationNameTrait for CustomizationCustomName {
}
impl CustomizationNameTrait for CustomizationFixedName {
}
impl CustomizationNameTrait for CustomizationPrefixName {
}
impl CustomizationNameTrait for CustomizationUnknownName {
}
impl CustomizationNameTrait for CustomizationVirtualMachineName {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CustomizationNameTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationName => Some(from.as_any_ref().downcast_ref::<CustomizationName>()?),
            StructType::CustomizationCustomName => Some(from.as_any_ref().downcast_ref::<CustomizationCustomName>()?),
            StructType::CustomizationFixedName => Some(from.as_any_ref().downcast_ref::<CustomizationFixedName>()?),
            StructType::CustomizationPrefixName => Some(from.as_any_ref().downcast_ref::<CustomizationPrefixName>()?),
            StructType::CustomizationUnknownName => Some(from.as_any_ref().downcast_ref::<CustomizationUnknownName>()?),
            StructType::CustomizationVirtualMachineName => Some(from.as_any_ref().downcast_ref::<CustomizationVirtualMachineName>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationName => Ok(from.as_any_box().downcast::<CustomizationName>()?),
            StructType::CustomizationCustomName => Ok(from.as_any_box().downcast::<CustomizationCustomName>()?),
            StructType::CustomizationFixedName => Ok(from.as_any_box().downcast::<CustomizationFixedName>()?),
            StructType::CustomizationPrefixName => Ok(from.as_any_box().downcast::<CustomizationPrefixName>()?),
            StructType::CustomizationUnknownName => Ok(from.as_any_box().downcast::<CustomizationUnknownName>()?),
            StructType::CustomizationVirtualMachineName => Ok(from.as_any_box().downcast::<CustomizationVirtualMachineName>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
