use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for all network-attached storage configuration faults.
pub trait NasConfigFaultTrait : super::host_config_fault_trait::HostConfigFaultTrait {
    /// Name of the Nas datastore being configured.
    fn get_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn NasConfigFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NasConfigFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NasConfigFaultVisitor)
            }
        }

struct NasConfigFaultVisitor;

impl<'de> de::Visitor<'de> for NasConfigFaultVisitor {
    type Value = Box<dyn NasConfigFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NasConfigFaultTrait JSON object with a _typeName field")
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

impl NasConfigFaultTrait for NasConfigFault {
    fn get_name(&self) -> &str { &self.name }
}
impl NasConfigFaultTrait for InvalidNasCredentials {
    fn get_name(&self) -> &str { &self.name }
}
impl NasConfigFaultTrait for InvalidNetworkResource {
    fn get_name(&self) -> &str { &self.name }
}
impl NasConfigFaultTrait for NasConnectionLimitReached {
    fn get_name(&self) -> &str { &self.name }
}
impl NasConfigFaultTrait for NasSessionCredentialConflict {
    fn get_name(&self) -> &str { &self.name }
}
impl NasConfigFaultTrait for NasVolumeNotMounted {
    fn get_name(&self) -> &str { &self.name }
}
impl NasConfigFaultTrait for NetworkInaccessible {
    fn get_name(&self) -> &str { &self.name }
}
impl NasConfigFaultTrait for NoPermissionOnNasVolume {
    fn get_name(&self) -> &str { &self.name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NasConfigFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NasConfigFault => Some(from.as_any_ref().downcast_ref::<NasConfigFault>()?),
            StructType::InvalidNasCredentials => Some(from.as_any_ref().downcast_ref::<InvalidNasCredentials>()?),
            StructType::InvalidNetworkResource => Some(from.as_any_ref().downcast_ref::<InvalidNetworkResource>()?),
            StructType::NasConnectionLimitReached => Some(from.as_any_ref().downcast_ref::<NasConnectionLimitReached>()?),
            StructType::NasSessionCredentialConflict => Some(from.as_any_ref().downcast_ref::<NasSessionCredentialConflict>()?),
            StructType::NasVolumeNotMounted => Some(from.as_any_ref().downcast_ref::<NasVolumeNotMounted>()?),
            StructType::NetworkInaccessible => Some(from.as_any_ref().downcast_ref::<NetworkInaccessible>()?),
            StructType::NoPermissionOnNasVolume => Some(from.as_any_ref().downcast_ref::<NoPermissionOnNasVolume>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::NasConfigFault => Ok(from.as_any_box().downcast::<NasConfigFault>()?),
            StructType::InvalidNasCredentials => Ok(from.as_any_box().downcast::<InvalidNasCredentials>()?),
            StructType::InvalidNetworkResource => Ok(from.as_any_box().downcast::<InvalidNetworkResource>()?),
            StructType::NasConnectionLimitReached => Ok(from.as_any_box().downcast::<NasConnectionLimitReached>()?),
            StructType::NasSessionCredentialConflict => Ok(from.as_any_box().downcast::<NasSessionCredentialConflict>()?),
            StructType::NasVolumeNotMounted => Ok(from.as_any_box().downcast::<NasVolumeNotMounted>()?),
            StructType::NetworkInaccessible => Ok(from.as_any_box().downcast::<NetworkInaccessible>()?),
            StructType::NoPermissionOnNasVolume => Ok(from.as_any_box().downcast::<NoPermissionOnNasVolume>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
