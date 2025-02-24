use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Thrown when a server logon fails due to a bad user name or
/// password.
pub trait InvalidLoginTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn InvalidLoginTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InvalidLoginTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InvalidLoginVisitor)
            }
        }

struct InvalidLoginVisitor;

impl<'de> de::Visitor<'de> for InvalidLoginVisitor {
    type Value = Box<dyn InvalidLoginTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InvalidLoginTrait JSON object with a _typeName field")
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

impl InvalidLoginTrait for InvalidLogin {
}
impl InvalidLoginTrait for InvalidClientCertificate {
}
impl InvalidLoginTrait for PasswordExpired {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InvalidLoginTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidLogin => Some(from.as_any_ref().downcast_ref::<InvalidLogin>()?),
            StructType::InvalidClientCertificate => Some(from.as_any_ref().downcast_ref::<InvalidClientCertificate>()?),
            StructType::PasswordExpired => Some(from.as_any_ref().downcast_ref::<PasswordExpired>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidLogin => Ok(from.as_any_box().downcast::<InvalidLogin>()?),
            StructType::InvalidClientCertificate => Ok(from.as_any_box().downcast::<InvalidClientCertificate>()?),
            StructType::PasswordExpired => Ok(from.as_any_box().downcast::<PasswordExpired>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
