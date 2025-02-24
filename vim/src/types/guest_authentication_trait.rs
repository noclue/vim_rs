use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// GuestAuthentication is an abstract base class for authentication
/// in the guest.
pub trait GuestAuthenticationTrait : super::data_object_trait::DataObjectTrait {
    /// This is set to true if the client wants an interactive session
    /// in the guest.
    /// 
    /// Setting this is supported only for *NamePasswordAuthentication*.
    fn get_interactive_session(&self) -> bool;
}
impl<'s> serde::Serialize for dyn GuestAuthenticationTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn GuestAuthenticationTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(GuestAuthenticationVisitor)
            }
        }

struct GuestAuthenticationVisitor;

impl<'de> de::Visitor<'de> for GuestAuthenticationVisitor {
    type Value = Box<dyn GuestAuthenticationTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GuestAuthenticationTrait JSON object with a _typeName field")
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

impl GuestAuthenticationTrait for GuestAuthentication {
    fn get_interactive_session(&self) -> bool { self.interactive_session }
}
impl GuestAuthenticationTrait for NamePasswordAuthentication {
    fn get_interactive_session(&self) -> bool { self.interactive_session }
}
impl GuestAuthenticationTrait for SamlTokenAuthentication {
    fn get_interactive_session(&self) -> bool { self.interactive_session }
}
impl GuestAuthenticationTrait for SspiAuthentication {
    fn get_interactive_session(&self) -> bool { self.interactive_session }
}
impl GuestAuthenticationTrait for TicketedSessionAuthentication {
    fn get_interactive_session(&self) -> bool { self.interactive_session }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn GuestAuthenticationTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestAuthentication => Some(from.as_any_ref().downcast_ref::<GuestAuthentication>()?),
            StructType::NamePasswordAuthentication => Some(from.as_any_ref().downcast_ref::<NamePasswordAuthentication>()?),
            StructType::SamlTokenAuthentication => Some(from.as_any_ref().downcast_ref::<SamlTokenAuthentication>()?),
            StructType::SspiAuthentication => Some(from.as_any_ref().downcast_ref::<SspiAuthentication>()?),
            StructType::TicketedSessionAuthentication => Some(from.as_any_ref().downcast_ref::<TicketedSessionAuthentication>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestAuthentication => Ok(from.as_any_box().downcast::<GuestAuthentication>()?),
            StructType::NamePasswordAuthentication => Ok(from.as_any_box().downcast::<NamePasswordAuthentication>()?),
            StructType::SamlTokenAuthentication => Ok(from.as_any_box().downcast::<SamlTokenAuthentication>()?),
            StructType::SspiAuthentication => Ok(from.as_any_box().downcast::<SspiAuthentication>()?),
            StructType::TicketedSessionAuthentication => Ok(from.as_any_box().downcast::<TicketedSessionAuthentication>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
