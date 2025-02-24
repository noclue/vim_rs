use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This describes the registry value data.
pub trait GuestRegValueDataSpecTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn GuestRegValueDataSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn GuestRegValueDataSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(GuestRegValueDataSpecVisitor)
            }
        }

struct GuestRegValueDataSpecVisitor;

impl<'de> de::Visitor<'de> for GuestRegValueDataSpecVisitor {
    type Value = Box<dyn GuestRegValueDataSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GuestRegValueDataSpecTrait JSON object with a _typeName field")
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

impl GuestRegValueDataSpecTrait for GuestRegValueDataSpec {
}
impl GuestRegValueDataSpecTrait for GuestRegValueBinarySpec {
}
impl GuestRegValueDataSpecTrait for GuestRegValueDwordSpec {
}
impl GuestRegValueDataSpecTrait for GuestRegValueExpandStringSpec {
}
impl GuestRegValueDataSpecTrait for GuestRegValueMultiStringSpec {
}
impl GuestRegValueDataSpecTrait for GuestRegValueQwordSpec {
}
impl GuestRegValueDataSpecTrait for GuestRegValueStringSpec {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn GuestRegValueDataSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestRegValueDataSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueDataSpec>()?),
            StructType::GuestRegValueBinarySpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueBinarySpec>()?),
            StructType::GuestRegValueDwordSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueDwordSpec>()?),
            StructType::GuestRegValueExpandStringSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueExpandStringSpec>()?),
            StructType::GuestRegValueMultiStringSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueMultiStringSpec>()?),
            StructType::GuestRegValueQwordSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueQwordSpec>()?),
            StructType::GuestRegValueStringSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueStringSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestRegValueDataSpec => Ok(from.as_any_box().downcast::<GuestRegValueDataSpec>()?),
            StructType::GuestRegValueBinarySpec => Ok(from.as_any_box().downcast::<GuestRegValueBinarySpec>()?),
            StructType::GuestRegValueDwordSpec => Ok(from.as_any_box().downcast::<GuestRegValueDwordSpec>()?),
            StructType::GuestRegValueExpandStringSpec => Ok(from.as_any_box().downcast::<GuestRegValueExpandStringSpec>()?),
            StructType::GuestRegValueMultiStringSpec => Ok(from.as_any_box().downcast::<GuestRegValueMultiStringSpec>()?),
            StructType::GuestRegValueQwordSpec => Ok(from.as_any_box().downcast::<GuestRegValueQwordSpec>()?),
            StructType::GuestRegValueStringSpec => Ok(from.as_any_box().downcast::<GuestRegValueStringSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
