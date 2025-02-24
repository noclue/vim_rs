use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Details of a Trusted Platform Module (TPM) event recording kernel security
/// option passed at boot time and currently in effect.
/// 
/// This event type exists to simplify parsing of the security-related information
/// by internal and third-party solutions. Each boot option may be passed to kernel
/// multiple times and/or in different forms. Replicating the parsing logic of the
/// kernel would be neither convinient, nor secure for the client applications.
/// 
/// Each instance of this event reports details of a single security-related
/// boot option, as set in the kernel.
pub trait HostTpmBootSecurityOptionEventDetailsTrait : super::host_tpm_event_details_trait::HostTpmEventDetailsTrait {
    /// Security-related options string, reflecting the state of an option set
    /// in the kernel.
    /// 
    /// This string is in the form of a KEY=VALUE pair.
    fn get_boot_security_option(&self) -> &str;
}
impl<'s> serde::Serialize for dyn HostTpmBootSecurityOptionEventDetailsTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostTpmBootSecurityOptionEventDetailsTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostTpmBootSecurityOptionEventDetailsVisitor)
            }
        }

struct HostTpmBootSecurityOptionEventDetailsVisitor;

impl<'de> de::Visitor<'de> for HostTpmBootSecurityOptionEventDetailsVisitor {
    type Value = Box<dyn HostTpmBootSecurityOptionEventDetailsTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostTpmBootSecurityOptionEventDetailsTrait JSON object with a _typeName field")
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

impl HostTpmBootSecurityOptionEventDetailsTrait for HostTpmBootSecurityOptionEventDetails {
    fn get_boot_security_option(&self) -> &str { &self.boot_security_option }
}
impl HostTpmBootSecurityOptionEventDetailsTrait for HostTpmNvTagEventDetails {
    fn get_boot_security_option(&self) -> &str { &self.boot_security_option }
}
impl HostTpmBootSecurityOptionEventDetailsTrait for HostTpmSignerEventDetails {
    fn get_boot_security_option(&self) -> &str { &self.boot_security_option }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostTpmBootSecurityOptionEventDetailsTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostTpmBootSecurityOptionEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmBootSecurityOptionEventDetails>()?),
            StructType::HostTpmNvTagEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmNvTagEventDetails>()?),
            StructType::HostTpmSignerEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmSignerEventDetails>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostTpmBootSecurityOptionEventDetails => Ok(from.as_any_box().downcast::<HostTpmBootSecurityOptionEventDetails>()?),
            StructType::HostTpmNvTagEventDetails => Ok(from.as_any_box().downcast::<HostTpmNvTagEventDetails>()?),
            StructType::HostTpmSignerEventDetails => Ok(from.as_any_box().downcast::<HostTpmSignerEventDetails>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
