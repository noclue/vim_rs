use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type describes a request to a service.
/// 
/// It is used as argument to
/// *SessionManager.AcquireGenericServiceTicket*.
/// This is the base class for more specific service request specifications.
/// E.g. for HTTP services the derived class will provide a URL property.
pub trait SessionManagerServiceRequestSpecTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn SessionManagerServiceRequestSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn SessionManagerServiceRequestSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(SessionManagerServiceRequestSpecVisitor)
            }
        }

struct SessionManagerServiceRequestSpecVisitor;

impl<'de> de::Visitor<'de> for SessionManagerServiceRequestSpecVisitor {
    type Value = Box<dyn SessionManagerServiceRequestSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid SessionManagerServiceRequestSpecTrait JSON object with a _typeName field")
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

impl SessionManagerServiceRequestSpecTrait for SessionManagerServiceRequestSpec {
}
impl SessionManagerServiceRequestSpecTrait for SessionManagerHttpServiceRequestSpec {
}
impl SessionManagerServiceRequestSpecTrait for SessionManagerVmomiServiceRequestSpec {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn SessionManagerServiceRequestSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::SessionManagerServiceRequestSpec => Some(from.as_any_ref().downcast_ref::<SessionManagerServiceRequestSpec>()?),
            StructType::SessionManagerHttpServiceRequestSpec => Some(from.as_any_ref().downcast_ref::<SessionManagerHttpServiceRequestSpec>()?),
            StructType::SessionManagerVmomiServiceRequestSpec => Some(from.as_any_ref().downcast_ref::<SessionManagerVmomiServiceRequestSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::SessionManagerServiceRequestSpec => Ok(from.as_any_box().downcast::<SessionManagerServiceRequestSpec>()?),
            StructType::SessionManagerHttpServiceRequestSpec => Ok(from.as_any_box().downcast::<SessionManagerHttpServiceRequestSpec>()?),
            StructType::SessionManagerVmomiServiceRequestSpec => Ok(from.as_any_box().downcast::<SessionManagerVmomiServiceRequestSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
