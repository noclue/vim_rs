use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An InvalidRequest fault is thrown in response to a malformed
/// request to the server that fails in the transport layer, e.g.,
/// the SOAP XML request was invalid.
/// 
/// Sub-types of this fault,
/// provides more specific transport errors, such as a using a
/// reference to an unknown managed object type or method.
pub trait InvalidRequestTrait : super::runtime_fault_trait::RuntimeFaultTrait {
}
impl<'s> serde::Serialize for dyn InvalidRequestTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InvalidRequestTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InvalidRequestVisitor)
            }
        }

struct InvalidRequestVisitor;

impl<'de> de::Visitor<'de> for InvalidRequestVisitor {
    type Value = Box<dyn InvalidRequestTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InvalidRequestTrait JSON object with a _typeName field")
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

impl InvalidRequestTrait for InvalidRequest {
}
impl InvalidRequestTrait for InvalidType {
}
impl InvalidRequestTrait for MethodNotFound {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InvalidRequestTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidRequest => Some(from.as_any_ref().downcast_ref::<InvalidRequest>()?),
            StructType::InvalidType => Some(from.as_any_ref().downcast_ref::<InvalidType>()?),
            StructType::MethodNotFound => Some(from.as_any_ref().downcast_ref::<MethodNotFound>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidRequest => Ok(from.as_any_box().downcast::<InvalidRequest>()?),
            StructType::InvalidType => Ok(from.as_any_box().downcast::<InvalidType>()?),
            StructType::MethodNotFound => Ok(from.as_any_box().downcast::<MethodNotFound>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
