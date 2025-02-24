use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A base clase for faults that are related to connecting or
/// adding a host to the inventory.
pub trait HostConnectFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn HostConnectFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostConnectFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostConnectFaultVisitor)
            }
        }

struct HostConnectFaultVisitor;

impl<'de> de::Visitor<'de> for HostConnectFaultVisitor {
    type Value = Box<dyn HostConnectFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostConnectFaultTrait JSON object with a _typeName field")
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

impl HostConnectFaultTrait for HostConnectFault {
}
impl HostConnectFaultTrait for AgentInstallFailed {
}
impl HostConnectFaultTrait for AlreadyBeingManaged {
}
impl HostConnectFaultTrait for AlreadyConnected {
}
impl HostConnectFaultTrait for CannotAddHostWithFtVmAsStandalone {
}
impl HostConnectFaultTrait for CannotAddHostWithFtVmToDifferentCluster {
}
impl HostConnectFaultTrait for CannotAddHostWithFtVmToNonHaCluster {
}
impl HostConnectFaultTrait for GatewayConnectFault {
}
impl HostConnectFaultTrait for GatewayNotFound {
}
impl HostConnectFaultTrait for GatewayNotReachable {
}
impl HostConnectFaultTrait for GatewayOperationRefused {
}
impl HostConnectFaultTrait for GatewayToHostConnectFault {
}
impl HostConnectFaultTrait for GatewayHostNotReachable {
}
impl HostConnectFaultTrait for GatewayToHostAuthFault {
}
impl HostConnectFaultTrait for GatewayToHostTrustVerifyFault {
}
impl HostConnectFaultTrait for MultipleCertificatesVerifyFault {
}
impl HostConnectFaultTrait for NoHost {
}
impl HostConnectFaultTrait for NoPermissionOnHost {
}
impl HostConnectFaultTrait for NotSupportedHost {
}
impl HostConnectFaultTrait for NonVmwareOuiMacNotSupportedHost {
}
impl HostConnectFaultTrait for NotSupportedHostForVFlash {
}
impl HostConnectFaultTrait for NotSupportedHostForVmcp {
}
impl HostConnectFaultTrait for NotSupportedHostForVmemFile {
}
impl HostConnectFaultTrait for NotSupportedHostForVsan {
}
impl HostConnectFaultTrait for NotSupportedHostInCluster {
}
impl HostConnectFaultTrait for EvcAdmissionFailed {
}
impl HostConnectFaultTrait for EvcAdmissionFailedCpuFeaturesForMode {
}
impl HostConnectFaultTrait for EvcAdmissionFailedCpuModel {
}
impl HostConnectFaultTrait for EvcAdmissionFailedCpuModelForMode {
}
impl HostConnectFaultTrait for EvcAdmissionFailedCpuVendor {
}
impl HostConnectFaultTrait for EvcAdmissionFailedCpuVendorUnknown {
}
impl HostConnectFaultTrait for EvcAdmissionFailedHostDisconnected {
}
impl HostConnectFaultTrait for EvcAdmissionFailedHostSoftware {
}
impl HostConnectFaultTrait for EvcAdmissionFailedHostSoftwareForMode {
}
impl HostConnectFaultTrait for EvcAdmissionFailedVmActive {
}
impl HostConnectFaultTrait for NotSupportedHostInDvs {
}
impl HostConnectFaultTrait for NotSupportedHostInHaCluster {
}
impl HostConnectFaultTrait for ReadHostResourcePoolTreeFailed {
}
impl HostConnectFaultTrait for SslDisabledFault {
}
impl HostConnectFaultTrait for SslVerifyFault {
}
impl HostConnectFaultTrait for TooManyHosts {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostConnectFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostConnectFault => Some(from.as_any_ref().downcast_ref::<HostConnectFault>()?),
            StructType::AgentInstallFailed => Some(from.as_any_ref().downcast_ref::<AgentInstallFailed>()?),
            StructType::AlreadyBeingManaged => Some(from.as_any_ref().downcast_ref::<AlreadyBeingManaged>()?),
            StructType::AlreadyConnected => Some(from.as_any_ref().downcast_ref::<AlreadyConnected>()?),
            StructType::CannotAddHostWithFtVmAsStandalone => Some(from.as_any_ref().downcast_ref::<CannotAddHostWithFtVmAsStandalone>()?),
            StructType::CannotAddHostWithFtVmToDifferentCluster => Some(from.as_any_ref().downcast_ref::<CannotAddHostWithFtVmToDifferentCluster>()?),
            StructType::CannotAddHostWithFtVmToNonHaCluster => Some(from.as_any_ref().downcast_ref::<CannotAddHostWithFtVmToNonHaCluster>()?),
            StructType::GatewayConnectFault => Some(from.as_any_ref().downcast_ref::<GatewayConnectFault>()?),
            StructType::GatewayNotFound => Some(from.as_any_ref().downcast_ref::<GatewayNotFound>()?),
            StructType::GatewayNotReachable => Some(from.as_any_ref().downcast_ref::<GatewayNotReachable>()?),
            StructType::GatewayOperationRefused => Some(from.as_any_ref().downcast_ref::<GatewayOperationRefused>()?),
            StructType::GatewayToHostConnectFault => Some(from.as_any_ref().downcast_ref::<GatewayToHostConnectFault>()?),
            StructType::GatewayHostNotReachable => Some(from.as_any_ref().downcast_ref::<GatewayHostNotReachable>()?),
            StructType::GatewayToHostAuthFault => Some(from.as_any_ref().downcast_ref::<GatewayToHostAuthFault>()?),
            StructType::GatewayToHostTrustVerifyFault => Some(from.as_any_ref().downcast_ref::<GatewayToHostTrustVerifyFault>()?),
            StructType::MultipleCertificatesVerifyFault => Some(from.as_any_ref().downcast_ref::<MultipleCertificatesVerifyFault>()?),
            StructType::NoHost => Some(from.as_any_ref().downcast_ref::<NoHost>()?),
            StructType::NoPermissionOnHost => Some(from.as_any_ref().downcast_ref::<NoPermissionOnHost>()?),
            StructType::NotSupportedHost => Some(from.as_any_ref().downcast_ref::<NotSupportedHost>()?),
            StructType::NonVmwareOuiMacNotSupportedHost => Some(from.as_any_ref().downcast_ref::<NonVmwareOuiMacNotSupportedHost>()?),
            StructType::NotSupportedHostForVFlash => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVFlash>()?),
            StructType::NotSupportedHostForVmcp => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVmcp>()?),
            StructType::NotSupportedHostForVmemFile => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVmemFile>()?),
            StructType::NotSupportedHostForVsan => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVsan>()?),
            StructType::NotSupportedHostInCluster => Some(from.as_any_ref().downcast_ref::<NotSupportedHostInCluster>()?),
            StructType::EvcAdmissionFailed => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailed>()?),
            StructType::EvcAdmissionFailedCpuFeaturesForMode => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuFeaturesForMode>()?),
            StructType::EvcAdmissionFailedCpuModel => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuModel>()?),
            StructType::EvcAdmissionFailedCpuModelForMode => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuModelForMode>()?),
            StructType::EvcAdmissionFailedCpuVendor => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuVendor>()?),
            StructType::EvcAdmissionFailedCpuVendorUnknown => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuVendorUnknown>()?),
            StructType::EvcAdmissionFailedHostDisconnected => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedHostDisconnected>()?),
            StructType::EvcAdmissionFailedHostSoftware => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedHostSoftware>()?),
            StructType::EvcAdmissionFailedHostSoftwareForMode => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedHostSoftwareForMode>()?),
            StructType::EvcAdmissionFailedVmActive => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedVmActive>()?),
            StructType::NotSupportedHostInDvs => Some(from.as_any_ref().downcast_ref::<NotSupportedHostInDvs>()?),
            StructType::NotSupportedHostInHaCluster => Some(from.as_any_ref().downcast_ref::<NotSupportedHostInHaCluster>()?),
            StructType::ReadHostResourcePoolTreeFailed => Some(from.as_any_ref().downcast_ref::<ReadHostResourcePoolTreeFailed>()?),
            StructType::SslDisabledFault => Some(from.as_any_ref().downcast_ref::<SslDisabledFault>()?),
            StructType::SslVerifyFault => Some(from.as_any_ref().downcast_ref::<SslVerifyFault>()?),
            StructType::TooManyHosts => Some(from.as_any_ref().downcast_ref::<TooManyHosts>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostConnectFault => Ok(from.as_any_box().downcast::<HostConnectFault>()?),
            StructType::AgentInstallFailed => Ok(from.as_any_box().downcast::<AgentInstallFailed>()?),
            StructType::AlreadyBeingManaged => Ok(from.as_any_box().downcast::<AlreadyBeingManaged>()?),
            StructType::AlreadyConnected => Ok(from.as_any_box().downcast::<AlreadyConnected>()?),
            StructType::CannotAddHostWithFtVmAsStandalone => Ok(from.as_any_box().downcast::<CannotAddHostWithFtVmAsStandalone>()?),
            StructType::CannotAddHostWithFtVmToDifferentCluster => Ok(from.as_any_box().downcast::<CannotAddHostWithFtVmToDifferentCluster>()?),
            StructType::CannotAddHostWithFtVmToNonHaCluster => Ok(from.as_any_box().downcast::<CannotAddHostWithFtVmToNonHaCluster>()?),
            StructType::GatewayConnectFault => Ok(from.as_any_box().downcast::<GatewayConnectFault>()?),
            StructType::GatewayNotFound => Ok(from.as_any_box().downcast::<GatewayNotFound>()?),
            StructType::GatewayNotReachable => Ok(from.as_any_box().downcast::<GatewayNotReachable>()?),
            StructType::GatewayOperationRefused => Ok(from.as_any_box().downcast::<GatewayOperationRefused>()?),
            StructType::GatewayToHostConnectFault => Ok(from.as_any_box().downcast::<GatewayToHostConnectFault>()?),
            StructType::GatewayHostNotReachable => Ok(from.as_any_box().downcast::<GatewayHostNotReachable>()?),
            StructType::GatewayToHostAuthFault => Ok(from.as_any_box().downcast::<GatewayToHostAuthFault>()?),
            StructType::GatewayToHostTrustVerifyFault => Ok(from.as_any_box().downcast::<GatewayToHostTrustVerifyFault>()?),
            StructType::MultipleCertificatesVerifyFault => Ok(from.as_any_box().downcast::<MultipleCertificatesVerifyFault>()?),
            StructType::NoHost => Ok(from.as_any_box().downcast::<NoHost>()?),
            StructType::NoPermissionOnHost => Ok(from.as_any_box().downcast::<NoPermissionOnHost>()?),
            StructType::NotSupportedHost => Ok(from.as_any_box().downcast::<NotSupportedHost>()?),
            StructType::NonVmwareOuiMacNotSupportedHost => Ok(from.as_any_box().downcast::<NonVmwareOuiMacNotSupportedHost>()?),
            StructType::NotSupportedHostForVFlash => Ok(from.as_any_box().downcast::<NotSupportedHostForVFlash>()?),
            StructType::NotSupportedHostForVmcp => Ok(from.as_any_box().downcast::<NotSupportedHostForVmcp>()?),
            StructType::NotSupportedHostForVmemFile => Ok(from.as_any_box().downcast::<NotSupportedHostForVmemFile>()?),
            StructType::NotSupportedHostForVsan => Ok(from.as_any_box().downcast::<NotSupportedHostForVsan>()?),
            StructType::NotSupportedHostInCluster => Ok(from.as_any_box().downcast::<NotSupportedHostInCluster>()?),
            StructType::EvcAdmissionFailed => Ok(from.as_any_box().downcast::<EvcAdmissionFailed>()?),
            StructType::EvcAdmissionFailedCpuFeaturesForMode => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuFeaturesForMode>()?),
            StructType::EvcAdmissionFailedCpuModel => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuModel>()?),
            StructType::EvcAdmissionFailedCpuModelForMode => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuModelForMode>()?),
            StructType::EvcAdmissionFailedCpuVendor => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuVendor>()?),
            StructType::EvcAdmissionFailedCpuVendorUnknown => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuVendorUnknown>()?),
            StructType::EvcAdmissionFailedHostDisconnected => Ok(from.as_any_box().downcast::<EvcAdmissionFailedHostDisconnected>()?),
            StructType::EvcAdmissionFailedHostSoftware => Ok(from.as_any_box().downcast::<EvcAdmissionFailedHostSoftware>()?),
            StructType::EvcAdmissionFailedHostSoftwareForMode => Ok(from.as_any_box().downcast::<EvcAdmissionFailedHostSoftwareForMode>()?),
            StructType::EvcAdmissionFailedVmActive => Ok(from.as_any_box().downcast::<EvcAdmissionFailedVmActive>()?),
            StructType::NotSupportedHostInDvs => Ok(from.as_any_box().downcast::<NotSupportedHostInDvs>()?),
            StructType::NotSupportedHostInHaCluster => Ok(from.as_any_box().downcast::<NotSupportedHostInHaCluster>()?),
            StructType::ReadHostResourcePoolTreeFailed => Ok(from.as_any_box().downcast::<ReadHostResourcePoolTreeFailed>()?),
            StructType::SslDisabledFault => Ok(from.as_any_box().downcast::<SslDisabledFault>()?),
            StructType::SslVerifyFault => Ok(from.as_any_box().downcast::<SslVerifyFault>()?),
            StructType::TooManyHosts => Ok(from.as_any_box().downcast::<TooManyHosts>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
