use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *HostAuthenticationStoreInfo* base class defines status information
/// for local and host Active Directory authentication.
pub trait HostAuthenticationStoreInfoTrait : super::data_object_trait::DataObjectTrait {
    /// Indicates whether the authentication store is configured.
    /// - Host Active Directory authentication - <code>enabled</code>
    ///   is <code>True</code> if the host is a member of a domain.
    /// - Local authentication - <code>enabled</code> is always <code>True</code>.
    fn get_enabled(&self) -> bool;
}
impl<'s> serde::Serialize for dyn HostAuthenticationStoreInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostAuthenticationStoreInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostAuthenticationStoreInfoVisitor)
            }
        }

struct HostAuthenticationStoreInfoVisitor;

impl<'de> de::Visitor<'de> for HostAuthenticationStoreInfoVisitor {
    type Value = Box<dyn HostAuthenticationStoreInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostAuthenticationStoreInfoTrait JSON object with a _typeName field")
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

impl HostAuthenticationStoreInfoTrait for HostAuthenticationStoreInfo {
    fn get_enabled(&self) -> bool { self.enabled }
}
impl HostAuthenticationStoreInfoTrait for HostDirectoryStoreInfo {
    fn get_enabled(&self) -> bool { self.enabled }
}
impl HostAuthenticationStoreInfoTrait for HostActiveDirectoryInfo {
    fn get_enabled(&self) -> bool { self.enabled }
}
impl HostAuthenticationStoreInfoTrait for HostLocalAuthenticationInfo {
    fn get_enabled(&self) -> bool { self.enabled }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostAuthenticationStoreInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostAuthenticationStoreInfo => Some(from.as_any_ref().downcast_ref::<HostAuthenticationStoreInfo>()?),
            StructType::HostDirectoryStoreInfo => Some(from.as_any_ref().downcast_ref::<HostDirectoryStoreInfo>()?),
            StructType::HostActiveDirectoryInfo => Some(from.as_any_ref().downcast_ref::<HostActiveDirectoryInfo>()?),
            StructType::HostLocalAuthenticationInfo => Some(from.as_any_ref().downcast_ref::<HostLocalAuthenticationInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostAuthenticationStoreInfo => Ok(from.as_any_box().downcast::<HostAuthenticationStoreInfo>()?),
            StructType::HostDirectoryStoreInfo => Ok(from.as_any_box().downcast::<HostDirectoryStoreInfo>()?),
            StructType::HostActiveDirectoryInfo => Ok(from.as_any_box().downcast::<HostActiveDirectoryInfo>()?),
            StructType::HostLocalAuthenticationInfo => Ok(from.as_any_box().downcast::<HostLocalAuthenticationInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
