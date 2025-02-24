use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base fault for Active Directory related problems.
pub trait ActiveDirectoryFaultTrait : super::vim_fault_trait::VimFaultTrait {
    /// The error code reported by the Active Directory API.
    fn get_error_code(&self) -> Option<i32>;
}
impl<'s> serde::Serialize for dyn ActiveDirectoryFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ActiveDirectoryFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ActiveDirectoryFaultVisitor)
            }
        }

struct ActiveDirectoryFaultVisitor;

impl<'de> de::Visitor<'de> for ActiveDirectoryFaultVisitor {
    type Value = Box<dyn ActiveDirectoryFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ActiveDirectoryFaultTrait JSON object with a _typeName field")
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

impl ActiveDirectoryFaultTrait for ActiveDirectoryFault {
    fn get_error_code(&self) -> Option<i32> { self.error_code }
}
impl ActiveDirectoryFaultTrait for DomainNotFound {
    fn get_error_code(&self) -> Option<i32> { self.error_code }
}
impl ActiveDirectoryFaultTrait for InvalidCamServer {
    fn get_error_code(&self) -> Option<i32> { self.error_code }
}
impl ActiveDirectoryFaultTrait for CamServerRefusedConnection {
    fn get_error_code(&self) -> Option<i32> { self.error_code }
}
impl ActiveDirectoryFaultTrait for InvalidCamCertificate {
    fn get_error_code(&self) -> Option<i32> { self.error_code }
}
impl ActiveDirectoryFaultTrait for NoPermissionOnAd {
    fn get_error_code(&self) -> Option<i32> { self.error_code }
}
impl ActiveDirectoryFaultTrait for NonAdUserRequired {
    fn get_error_code(&self) -> Option<i32> { self.error_code }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ActiveDirectoryFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ActiveDirectoryFault => Some(from.as_any_ref().downcast_ref::<ActiveDirectoryFault>()?),
            StructType::DomainNotFound => Some(from.as_any_ref().downcast_ref::<DomainNotFound>()?),
            StructType::InvalidCamServer => Some(from.as_any_ref().downcast_ref::<InvalidCamServer>()?),
            StructType::CamServerRefusedConnection => Some(from.as_any_ref().downcast_ref::<CamServerRefusedConnection>()?),
            StructType::InvalidCamCertificate => Some(from.as_any_ref().downcast_ref::<InvalidCamCertificate>()?),
            StructType::NoPermissionOnAd => Some(from.as_any_ref().downcast_ref::<NoPermissionOnAd>()?),
            StructType::NonAdUserRequired => Some(from.as_any_ref().downcast_ref::<NonAdUserRequired>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ActiveDirectoryFault => Ok(from.as_any_box().downcast::<ActiveDirectoryFault>()?),
            StructType::DomainNotFound => Ok(from.as_any_box().downcast::<DomainNotFound>()?),
            StructType::InvalidCamServer => Ok(from.as_any_box().downcast::<InvalidCamServer>()?),
            StructType::CamServerRefusedConnection => Ok(from.as_any_box().downcast::<CamServerRefusedConnection>()?),
            StructType::InvalidCamCertificate => Ok(from.as_any_box().downcast::<InvalidCamCertificate>()?),
            StructType::NoPermissionOnAd => Ok(from.as_any_box().downcast::<NoPermissionOnAd>()?),
            StructType::NonAdUserRequired => Ok(from.as_any_box().downcast::<NonAdUserRequired>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
