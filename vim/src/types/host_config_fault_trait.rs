use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for all Host configuration related faults
pub trait HostConfigFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn HostConfigFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostConfigFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostConfigFaultVisitor)
            }
        }

struct HostConfigFaultVisitor;

impl<'de> de::Visitor<'de> for HostConfigFaultVisitor {
    type Value = Box<dyn HostConfigFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostConfigFaultTrait JSON object with a _typeName field")
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

impl HostConfigFaultTrait for HostConfigFault {
}
impl HostConfigFaultTrait for AdminDisabled {
}
impl HostConfigFaultTrait for AdminNotDisabled {
}
impl HostConfigFaultTrait for BlockedByFirewall {
}
impl HostConfigFaultTrait for ClockSkew {
}
impl HostConfigFaultTrait for DisableAdminNotSupported {
}
impl HostConfigFaultTrait for HostConfigFailed {
}
impl HostConfigFaultTrait for HostInDomain {
}
impl HostConfigFaultTrait for InvalidHostName {
}
impl HostConfigFaultTrait for NasConfigFault {
}
impl HostConfigFaultTrait for InvalidNasCredentials {
}
impl HostConfigFaultTrait for InvalidNetworkResource {
}
impl HostConfigFaultTrait for NasConnectionLimitReached {
}
impl HostConfigFaultTrait for NasSessionCredentialConflict {
}
impl HostConfigFaultTrait for NasVolumeNotMounted {
}
impl HostConfigFaultTrait for NetworkInaccessible {
}
impl HostConfigFaultTrait for NoPermissionOnNasVolume {
}
impl HostConfigFaultTrait for NoGateway {
}
impl HostConfigFaultTrait for NoVirtualNic {
}
impl HostConfigFaultTrait for PlatformConfigFault {
}
impl HostConfigFaultTrait for InvalidBundle {
}
impl HostConfigFaultTrait for PatchInstallFailed {
}
impl HostConfigFaultTrait for PatchIntegrityError {
}
impl HostConfigFaultTrait for VmfsMountFault {
}
impl HostConfigFaultTrait for VmfsAlreadyMounted {
}
impl HostConfigFaultTrait for VmfsAmbiguousMount {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostConfigFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostConfigFault => Some(from.as_any_ref().downcast_ref::<HostConfigFault>()?),
            StructType::AdminDisabled => Some(from.as_any_ref().downcast_ref::<AdminDisabled>()?),
            StructType::AdminNotDisabled => Some(from.as_any_ref().downcast_ref::<AdminNotDisabled>()?),
            StructType::BlockedByFirewall => Some(from.as_any_ref().downcast_ref::<BlockedByFirewall>()?),
            StructType::ClockSkew => Some(from.as_any_ref().downcast_ref::<ClockSkew>()?),
            StructType::DisableAdminNotSupported => Some(from.as_any_ref().downcast_ref::<DisableAdminNotSupported>()?),
            StructType::HostConfigFailed => Some(from.as_any_ref().downcast_ref::<HostConfigFailed>()?),
            StructType::HostInDomain => Some(from.as_any_ref().downcast_ref::<HostInDomain>()?),
            StructType::InvalidHostName => Some(from.as_any_ref().downcast_ref::<InvalidHostName>()?),
            StructType::NasConfigFault => Some(from.as_any_ref().downcast_ref::<NasConfigFault>()?),
            StructType::InvalidNasCredentials => Some(from.as_any_ref().downcast_ref::<InvalidNasCredentials>()?),
            StructType::InvalidNetworkResource => Some(from.as_any_ref().downcast_ref::<InvalidNetworkResource>()?),
            StructType::NasConnectionLimitReached => Some(from.as_any_ref().downcast_ref::<NasConnectionLimitReached>()?),
            StructType::NasSessionCredentialConflict => Some(from.as_any_ref().downcast_ref::<NasSessionCredentialConflict>()?),
            StructType::NasVolumeNotMounted => Some(from.as_any_ref().downcast_ref::<NasVolumeNotMounted>()?),
            StructType::NetworkInaccessible => Some(from.as_any_ref().downcast_ref::<NetworkInaccessible>()?),
            StructType::NoPermissionOnNasVolume => Some(from.as_any_ref().downcast_ref::<NoPermissionOnNasVolume>()?),
            StructType::NoGateway => Some(from.as_any_ref().downcast_ref::<NoGateway>()?),
            StructType::NoVirtualNic => Some(from.as_any_ref().downcast_ref::<NoVirtualNic>()?),
            StructType::PlatformConfigFault => Some(from.as_any_ref().downcast_ref::<PlatformConfigFault>()?),
            StructType::InvalidBundle => Some(from.as_any_ref().downcast_ref::<InvalidBundle>()?),
            StructType::PatchInstallFailed => Some(from.as_any_ref().downcast_ref::<PatchInstallFailed>()?),
            StructType::PatchIntegrityError => Some(from.as_any_ref().downcast_ref::<PatchIntegrityError>()?),
            StructType::VmfsMountFault => Some(from.as_any_ref().downcast_ref::<VmfsMountFault>()?),
            StructType::VmfsAlreadyMounted => Some(from.as_any_ref().downcast_ref::<VmfsAlreadyMounted>()?),
            StructType::VmfsAmbiguousMount => Some(from.as_any_ref().downcast_ref::<VmfsAmbiguousMount>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostConfigFault => Ok(from.as_any_box().downcast::<HostConfigFault>()?),
            StructType::AdminDisabled => Ok(from.as_any_box().downcast::<AdminDisabled>()?),
            StructType::AdminNotDisabled => Ok(from.as_any_box().downcast::<AdminNotDisabled>()?),
            StructType::BlockedByFirewall => Ok(from.as_any_box().downcast::<BlockedByFirewall>()?),
            StructType::ClockSkew => Ok(from.as_any_box().downcast::<ClockSkew>()?),
            StructType::DisableAdminNotSupported => Ok(from.as_any_box().downcast::<DisableAdminNotSupported>()?),
            StructType::HostConfigFailed => Ok(from.as_any_box().downcast::<HostConfigFailed>()?),
            StructType::HostInDomain => Ok(from.as_any_box().downcast::<HostInDomain>()?),
            StructType::InvalidHostName => Ok(from.as_any_box().downcast::<InvalidHostName>()?),
            StructType::NasConfigFault => Ok(from.as_any_box().downcast::<NasConfigFault>()?),
            StructType::InvalidNasCredentials => Ok(from.as_any_box().downcast::<InvalidNasCredentials>()?),
            StructType::InvalidNetworkResource => Ok(from.as_any_box().downcast::<InvalidNetworkResource>()?),
            StructType::NasConnectionLimitReached => Ok(from.as_any_box().downcast::<NasConnectionLimitReached>()?),
            StructType::NasSessionCredentialConflict => Ok(from.as_any_box().downcast::<NasSessionCredentialConflict>()?),
            StructType::NasVolumeNotMounted => Ok(from.as_any_box().downcast::<NasVolumeNotMounted>()?),
            StructType::NetworkInaccessible => Ok(from.as_any_box().downcast::<NetworkInaccessible>()?),
            StructType::NoPermissionOnNasVolume => Ok(from.as_any_box().downcast::<NoPermissionOnNasVolume>()?),
            StructType::NoGateway => Ok(from.as_any_box().downcast::<NoGateway>()?),
            StructType::NoVirtualNic => Ok(from.as_any_box().downcast::<NoVirtualNic>()?),
            StructType::PlatformConfigFault => Ok(from.as_any_box().downcast::<PlatformConfigFault>()?),
            StructType::InvalidBundle => Ok(from.as_any_box().downcast::<InvalidBundle>()?),
            StructType::PatchInstallFailed => Ok(from.as_any_box().downcast::<PatchInstallFailed>()?),
            StructType::PatchIntegrityError => Ok(from.as_any_box().downcast::<PatchIntegrityError>()?),
            StructType::VmfsMountFault => Ok(from.as_any_box().downcast::<VmfsMountFault>()?),
            StructType::VmfsAlreadyMounted => Ok(from.as_any_box().downcast::<VmfsAlreadyMounted>()?),
            StructType::VmfsAmbiguousMount => Ok(from.as_any_box().downcast::<VmfsAmbiguousMount>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
