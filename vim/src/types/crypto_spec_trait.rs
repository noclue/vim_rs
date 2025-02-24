use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type encapsulates virtual machine or disk encryption
/// settings.
pub trait CryptoSpecTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn CryptoSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CryptoSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CryptoSpecVisitor)
            }
        }

struct CryptoSpecVisitor;

impl<'de> de::Visitor<'de> for CryptoSpecVisitor {
    type Value = Box<dyn CryptoSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CryptoSpecTrait JSON object with a _typeName field")
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

impl CryptoSpecTrait for CryptoSpec {
}
impl CryptoSpecTrait for CryptoSpecDecrypt {
}
impl CryptoSpecTrait for CryptoSpecDeepRecrypt {
}
impl CryptoSpecTrait for CryptoSpecEncrypt {
}
impl CryptoSpecTrait for CryptoSpecNoOp {
}
impl CryptoSpecTrait for CryptoSpecRegister {
}
impl CryptoSpecTrait for CryptoSpecShallowRecrypt {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CryptoSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CryptoSpec => Some(from.as_any_ref().downcast_ref::<CryptoSpec>()?),
            StructType::CryptoSpecDecrypt => Some(from.as_any_ref().downcast_ref::<CryptoSpecDecrypt>()?),
            StructType::CryptoSpecDeepRecrypt => Some(from.as_any_ref().downcast_ref::<CryptoSpecDeepRecrypt>()?),
            StructType::CryptoSpecEncrypt => Some(from.as_any_ref().downcast_ref::<CryptoSpecEncrypt>()?),
            StructType::CryptoSpecNoOp => Some(from.as_any_ref().downcast_ref::<CryptoSpecNoOp>()?),
            StructType::CryptoSpecRegister => Some(from.as_any_ref().downcast_ref::<CryptoSpecRegister>()?),
            StructType::CryptoSpecShallowRecrypt => Some(from.as_any_ref().downcast_ref::<CryptoSpecShallowRecrypt>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CryptoSpec => Ok(from.as_any_box().downcast::<CryptoSpec>()?),
            StructType::CryptoSpecDecrypt => Ok(from.as_any_box().downcast::<CryptoSpecDecrypt>()?),
            StructType::CryptoSpecDeepRecrypt => Ok(from.as_any_box().downcast::<CryptoSpecDeepRecrypt>()?),
            StructType::CryptoSpecEncrypt => Ok(from.as_any_box().downcast::<CryptoSpecEncrypt>()?),
            StructType::CryptoSpecNoOp => Ok(from.as_any_box().downcast::<CryptoSpecNoOp>()?),
            StructType::CryptoSpecRegister => Ok(from.as_any_box().downcast::<CryptoSpecRegister>()?),
            StructType::CryptoSpecShallowRecrypt => Ok(from.as_any_box().downcast::<CryptoSpecShallowRecrypt>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
