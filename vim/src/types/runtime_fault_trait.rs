use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The base data object type for all runtime faults that can be
/// thrown by a method.
pub trait RuntimeFaultTrait : super::method_fault_trait::MethodFaultTrait {
}
impl<'s> serde::Serialize for dyn RuntimeFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn RuntimeFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(RuntimeFaultVisitor)
            }
        }

struct RuntimeFaultVisitor;

impl<'de> de::Visitor<'de> for RuntimeFaultVisitor {
    type Value = Box<dyn RuntimeFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid RuntimeFaultTrait JSON object with a _typeName field")
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

impl RuntimeFaultTrait for RuntimeFault {
}
impl RuntimeFaultTrait for CannotDisableDrsOnClustersWithVApps {
}
impl RuntimeFaultTrait for ConflictingDatastoreFound {
}
impl RuntimeFaultTrait for DatabaseError {
}
impl RuntimeFaultTrait for DisallowedChangeByService {
}
impl RuntimeFaultTrait for DisallowedOperationOnFailoverHost {
}
impl RuntimeFaultTrait for FailToLockFaultToleranceVMs {
}
impl RuntimeFaultTrait for InvalidProfileReferenceHost {
}
impl RuntimeFaultTrait for InvalidScheduledTask {
}
impl RuntimeFaultTrait for LicenseAssignmentFailed {
}
impl RuntimeFaultTrait for MethodAlreadyDisabledFault {
}
impl RuntimeFaultTrait for MethodDisabled {
}
impl RuntimeFaultTrait for OperationDisallowedOnHost {
}
impl RuntimeFaultTrait for RestrictedByAdministrator {
}
impl RuntimeFaultTrait for ThirdPartyLicenseAssignmentFailed {
}
impl RuntimeFaultTrait for VAppOperationInProgress {
}
impl RuntimeFaultTrait for HostCommunication {
}
impl RuntimeFaultTrait for HostNotConnected {
}
impl RuntimeFaultTrait for HostNotReachable {
}
impl RuntimeFaultTrait for InvalidArgument {
}
impl RuntimeFaultTrait for IncompatibleSetting {
}
impl RuntimeFaultTrait for InvalidDasConfigArgument {
}
impl RuntimeFaultTrait for InvalidDasRestartPriorityForFtVm {
}
impl RuntimeFaultTrait for InvalidDrsBehaviorForFtVm {
}
impl RuntimeFaultTrait for InvalidIndexArgument {
}
impl RuntimeFaultTrait for InvalidRequest {
}
impl RuntimeFaultTrait for InvalidType {
}
impl RuntimeFaultTrait for MethodNotFound {
}
impl RuntimeFaultTrait for ManagedObjectNotFound {
}
impl RuntimeFaultTrait for NotEnoughLicenses {
}
impl RuntimeFaultTrait for ExpiredFeatureLicense {
}
impl RuntimeFaultTrait for ExpiredAddonLicense {
}
impl RuntimeFaultTrait for ExpiredEditionLicense {
}
impl RuntimeFaultTrait for FailToEnableSpbm {
}
impl RuntimeFaultTrait for HostInventoryFull {
}
impl RuntimeFaultTrait for InUseFeatureManipulationDisallowed {
}
impl RuntimeFaultTrait for IncorrectHostInformation {
}
impl RuntimeFaultTrait for InvalidEditionLicense {
}
impl RuntimeFaultTrait for InventoryHasStandardAloneHosts {
}
impl RuntimeFaultTrait for LicenseDowngradeDisallowed {
}
impl RuntimeFaultTrait for LicenseExpired {
}
impl RuntimeFaultTrait for LicenseKeyEntityMismatch {
}
impl RuntimeFaultTrait for LicenseRestricted {
}
impl RuntimeFaultTrait for LicenseSourceUnavailable {
}
impl RuntimeFaultTrait for NoLicenseServerConfigured {
}
impl RuntimeFaultTrait for VmLimitLicense {
}
impl RuntimeFaultTrait for VramLimitLicense {
}
impl RuntimeFaultTrait for NotImplemented {
}
impl RuntimeFaultTrait for NotSupported {
}
impl RuntimeFaultTrait for HostAccessRestrictedToManagementServer {
}
impl RuntimeFaultTrait for RequestCanceled {
}
impl RuntimeFaultTrait for SecurityError {
}
impl RuntimeFaultTrait for NoPermission {
}
impl RuntimeFaultTrait for NotAuthenticated {
}
impl RuntimeFaultTrait for RestrictedVersion {
}
impl RuntimeFaultTrait for SolutionUserRequired {
}
impl RuntimeFaultTrait for SystemError {
}
impl RuntimeFaultTrait for UnexpectedFault {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn RuntimeFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::RuntimeFault => Some(from.as_any_ref().downcast_ref::<RuntimeFault>()?),
            StructType::CannotDisableDrsOnClustersWithVApps => Some(from.as_any_ref().downcast_ref::<CannotDisableDrsOnClustersWithVApps>()?),
            StructType::ConflictingDatastoreFound => Some(from.as_any_ref().downcast_ref::<ConflictingDatastoreFound>()?),
            StructType::DatabaseError => Some(from.as_any_ref().downcast_ref::<DatabaseError>()?),
            StructType::DisallowedChangeByService => Some(from.as_any_ref().downcast_ref::<DisallowedChangeByService>()?),
            StructType::DisallowedOperationOnFailoverHost => Some(from.as_any_ref().downcast_ref::<DisallowedOperationOnFailoverHost>()?),
            StructType::FailToLockFaultToleranceVMs => Some(from.as_any_ref().downcast_ref::<FailToLockFaultToleranceVMs>()?),
            StructType::InvalidProfileReferenceHost => Some(from.as_any_ref().downcast_ref::<InvalidProfileReferenceHost>()?),
            StructType::InvalidScheduledTask => Some(from.as_any_ref().downcast_ref::<InvalidScheduledTask>()?),
            StructType::LicenseAssignmentFailed => Some(from.as_any_ref().downcast_ref::<LicenseAssignmentFailed>()?),
            StructType::MethodAlreadyDisabledFault => Some(from.as_any_ref().downcast_ref::<MethodAlreadyDisabledFault>()?),
            StructType::MethodDisabled => Some(from.as_any_ref().downcast_ref::<MethodDisabled>()?),
            StructType::OperationDisallowedOnHost => Some(from.as_any_ref().downcast_ref::<OperationDisallowedOnHost>()?),
            StructType::RestrictedByAdministrator => Some(from.as_any_ref().downcast_ref::<RestrictedByAdministrator>()?),
            StructType::ThirdPartyLicenseAssignmentFailed => Some(from.as_any_ref().downcast_ref::<ThirdPartyLicenseAssignmentFailed>()?),
            StructType::VAppOperationInProgress => Some(from.as_any_ref().downcast_ref::<VAppOperationInProgress>()?),
            StructType::HostCommunication => Some(from.as_any_ref().downcast_ref::<HostCommunication>()?),
            StructType::HostNotConnected => Some(from.as_any_ref().downcast_ref::<HostNotConnected>()?),
            StructType::HostNotReachable => Some(from.as_any_ref().downcast_ref::<HostNotReachable>()?),
            StructType::InvalidArgument => Some(from.as_any_ref().downcast_ref::<InvalidArgument>()?),
            StructType::IncompatibleSetting => Some(from.as_any_ref().downcast_ref::<IncompatibleSetting>()?),
            StructType::InvalidDasConfigArgument => Some(from.as_any_ref().downcast_ref::<InvalidDasConfigArgument>()?),
            StructType::InvalidDasRestartPriorityForFtVm => Some(from.as_any_ref().downcast_ref::<InvalidDasRestartPriorityForFtVm>()?),
            StructType::InvalidDrsBehaviorForFtVm => Some(from.as_any_ref().downcast_ref::<InvalidDrsBehaviorForFtVm>()?),
            StructType::InvalidIndexArgument => Some(from.as_any_ref().downcast_ref::<InvalidIndexArgument>()?),
            StructType::InvalidRequest => Some(from.as_any_ref().downcast_ref::<InvalidRequest>()?),
            StructType::InvalidType => Some(from.as_any_ref().downcast_ref::<InvalidType>()?),
            StructType::MethodNotFound => Some(from.as_any_ref().downcast_ref::<MethodNotFound>()?),
            StructType::ManagedObjectNotFound => Some(from.as_any_ref().downcast_ref::<ManagedObjectNotFound>()?),
            StructType::NotEnoughLicenses => Some(from.as_any_ref().downcast_ref::<NotEnoughLicenses>()?),
            StructType::ExpiredFeatureLicense => Some(from.as_any_ref().downcast_ref::<ExpiredFeatureLicense>()?),
            StructType::ExpiredAddonLicense => Some(from.as_any_ref().downcast_ref::<ExpiredAddonLicense>()?),
            StructType::ExpiredEditionLicense => Some(from.as_any_ref().downcast_ref::<ExpiredEditionLicense>()?),
            StructType::FailToEnableSpbm => Some(from.as_any_ref().downcast_ref::<FailToEnableSpbm>()?),
            StructType::HostInventoryFull => Some(from.as_any_ref().downcast_ref::<HostInventoryFull>()?),
            StructType::InUseFeatureManipulationDisallowed => Some(from.as_any_ref().downcast_ref::<InUseFeatureManipulationDisallowed>()?),
            StructType::IncorrectHostInformation => Some(from.as_any_ref().downcast_ref::<IncorrectHostInformation>()?),
            StructType::InvalidEditionLicense => Some(from.as_any_ref().downcast_ref::<InvalidEditionLicense>()?),
            StructType::InventoryHasStandardAloneHosts => Some(from.as_any_ref().downcast_ref::<InventoryHasStandardAloneHosts>()?),
            StructType::LicenseDowngradeDisallowed => Some(from.as_any_ref().downcast_ref::<LicenseDowngradeDisallowed>()?),
            StructType::LicenseExpired => Some(from.as_any_ref().downcast_ref::<LicenseExpired>()?),
            StructType::LicenseKeyEntityMismatch => Some(from.as_any_ref().downcast_ref::<LicenseKeyEntityMismatch>()?),
            StructType::LicenseRestricted => Some(from.as_any_ref().downcast_ref::<LicenseRestricted>()?),
            StructType::LicenseSourceUnavailable => Some(from.as_any_ref().downcast_ref::<LicenseSourceUnavailable>()?),
            StructType::NoLicenseServerConfigured => Some(from.as_any_ref().downcast_ref::<NoLicenseServerConfigured>()?),
            StructType::VmLimitLicense => Some(from.as_any_ref().downcast_ref::<VmLimitLicense>()?),
            StructType::VramLimitLicense => Some(from.as_any_ref().downcast_ref::<VramLimitLicense>()?),
            StructType::NotImplemented => Some(from.as_any_ref().downcast_ref::<NotImplemented>()?),
            StructType::NotSupported => Some(from.as_any_ref().downcast_ref::<NotSupported>()?),
            StructType::HostAccessRestrictedToManagementServer => Some(from.as_any_ref().downcast_ref::<HostAccessRestrictedToManagementServer>()?),
            StructType::RequestCanceled => Some(from.as_any_ref().downcast_ref::<RequestCanceled>()?),
            StructType::SecurityError => Some(from.as_any_ref().downcast_ref::<SecurityError>()?),
            StructType::NoPermission => Some(from.as_any_ref().downcast_ref::<NoPermission>()?),
            StructType::NotAuthenticated => Some(from.as_any_ref().downcast_ref::<NotAuthenticated>()?),
            StructType::RestrictedVersion => Some(from.as_any_ref().downcast_ref::<RestrictedVersion>()?),
            StructType::SolutionUserRequired => Some(from.as_any_ref().downcast_ref::<SolutionUserRequired>()?),
            StructType::SystemError => Some(from.as_any_ref().downcast_ref::<SystemError>()?),
            StructType::UnexpectedFault => Some(from.as_any_ref().downcast_ref::<UnexpectedFault>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::RuntimeFault => Ok(from.as_any_box().downcast::<RuntimeFault>()?),
            StructType::CannotDisableDrsOnClustersWithVApps => Ok(from.as_any_box().downcast::<CannotDisableDrsOnClustersWithVApps>()?),
            StructType::ConflictingDatastoreFound => Ok(from.as_any_box().downcast::<ConflictingDatastoreFound>()?),
            StructType::DatabaseError => Ok(from.as_any_box().downcast::<DatabaseError>()?),
            StructType::DisallowedChangeByService => Ok(from.as_any_box().downcast::<DisallowedChangeByService>()?),
            StructType::DisallowedOperationOnFailoverHost => Ok(from.as_any_box().downcast::<DisallowedOperationOnFailoverHost>()?),
            StructType::FailToLockFaultToleranceVMs => Ok(from.as_any_box().downcast::<FailToLockFaultToleranceVMs>()?),
            StructType::InvalidProfileReferenceHost => Ok(from.as_any_box().downcast::<InvalidProfileReferenceHost>()?),
            StructType::InvalidScheduledTask => Ok(from.as_any_box().downcast::<InvalidScheduledTask>()?),
            StructType::LicenseAssignmentFailed => Ok(from.as_any_box().downcast::<LicenseAssignmentFailed>()?),
            StructType::MethodAlreadyDisabledFault => Ok(from.as_any_box().downcast::<MethodAlreadyDisabledFault>()?),
            StructType::MethodDisabled => Ok(from.as_any_box().downcast::<MethodDisabled>()?),
            StructType::OperationDisallowedOnHost => Ok(from.as_any_box().downcast::<OperationDisallowedOnHost>()?),
            StructType::RestrictedByAdministrator => Ok(from.as_any_box().downcast::<RestrictedByAdministrator>()?),
            StructType::ThirdPartyLicenseAssignmentFailed => Ok(from.as_any_box().downcast::<ThirdPartyLicenseAssignmentFailed>()?),
            StructType::VAppOperationInProgress => Ok(from.as_any_box().downcast::<VAppOperationInProgress>()?),
            StructType::HostCommunication => Ok(from.as_any_box().downcast::<HostCommunication>()?),
            StructType::HostNotConnected => Ok(from.as_any_box().downcast::<HostNotConnected>()?),
            StructType::HostNotReachable => Ok(from.as_any_box().downcast::<HostNotReachable>()?),
            StructType::InvalidArgument => Ok(from.as_any_box().downcast::<InvalidArgument>()?),
            StructType::IncompatibleSetting => Ok(from.as_any_box().downcast::<IncompatibleSetting>()?),
            StructType::InvalidDasConfigArgument => Ok(from.as_any_box().downcast::<InvalidDasConfigArgument>()?),
            StructType::InvalidDasRestartPriorityForFtVm => Ok(from.as_any_box().downcast::<InvalidDasRestartPriorityForFtVm>()?),
            StructType::InvalidDrsBehaviorForFtVm => Ok(from.as_any_box().downcast::<InvalidDrsBehaviorForFtVm>()?),
            StructType::InvalidIndexArgument => Ok(from.as_any_box().downcast::<InvalidIndexArgument>()?),
            StructType::InvalidRequest => Ok(from.as_any_box().downcast::<InvalidRequest>()?),
            StructType::InvalidType => Ok(from.as_any_box().downcast::<InvalidType>()?),
            StructType::MethodNotFound => Ok(from.as_any_box().downcast::<MethodNotFound>()?),
            StructType::ManagedObjectNotFound => Ok(from.as_any_box().downcast::<ManagedObjectNotFound>()?),
            StructType::NotEnoughLicenses => Ok(from.as_any_box().downcast::<NotEnoughLicenses>()?),
            StructType::ExpiredFeatureLicense => Ok(from.as_any_box().downcast::<ExpiredFeatureLicense>()?),
            StructType::ExpiredAddonLicense => Ok(from.as_any_box().downcast::<ExpiredAddonLicense>()?),
            StructType::ExpiredEditionLicense => Ok(from.as_any_box().downcast::<ExpiredEditionLicense>()?),
            StructType::FailToEnableSpbm => Ok(from.as_any_box().downcast::<FailToEnableSpbm>()?),
            StructType::HostInventoryFull => Ok(from.as_any_box().downcast::<HostInventoryFull>()?),
            StructType::InUseFeatureManipulationDisallowed => Ok(from.as_any_box().downcast::<InUseFeatureManipulationDisallowed>()?),
            StructType::IncorrectHostInformation => Ok(from.as_any_box().downcast::<IncorrectHostInformation>()?),
            StructType::InvalidEditionLicense => Ok(from.as_any_box().downcast::<InvalidEditionLicense>()?),
            StructType::InventoryHasStandardAloneHosts => Ok(from.as_any_box().downcast::<InventoryHasStandardAloneHosts>()?),
            StructType::LicenseDowngradeDisallowed => Ok(from.as_any_box().downcast::<LicenseDowngradeDisallowed>()?),
            StructType::LicenseExpired => Ok(from.as_any_box().downcast::<LicenseExpired>()?),
            StructType::LicenseKeyEntityMismatch => Ok(from.as_any_box().downcast::<LicenseKeyEntityMismatch>()?),
            StructType::LicenseRestricted => Ok(from.as_any_box().downcast::<LicenseRestricted>()?),
            StructType::LicenseSourceUnavailable => Ok(from.as_any_box().downcast::<LicenseSourceUnavailable>()?),
            StructType::NoLicenseServerConfigured => Ok(from.as_any_box().downcast::<NoLicenseServerConfigured>()?),
            StructType::VmLimitLicense => Ok(from.as_any_box().downcast::<VmLimitLicense>()?),
            StructType::VramLimitLicense => Ok(from.as_any_box().downcast::<VramLimitLicense>()?),
            StructType::NotImplemented => Ok(from.as_any_box().downcast::<NotImplemented>()?),
            StructType::NotSupported => Ok(from.as_any_box().downcast::<NotSupported>()?),
            StructType::HostAccessRestrictedToManagementServer => Ok(from.as_any_box().downcast::<HostAccessRestrictedToManagementServer>()?),
            StructType::RequestCanceled => Ok(from.as_any_box().downcast::<RequestCanceled>()?),
            StructType::SecurityError => Ok(from.as_any_box().downcast::<SecurityError>()?),
            StructType::NoPermission => Ok(from.as_any_box().downcast::<NoPermission>()?),
            StructType::NotAuthenticated => Ok(from.as_any_box().downcast::<NotAuthenticated>()?),
            StructType::RestrictedVersion => Ok(from.as_any_box().downcast::<RestrictedVersion>()?),
            StructType::SolutionUserRequired => Ok(from.as_any_box().downcast::<SolutionUserRequired>()?),
            StructType::SystemError => Ok(from.as_any_box().downcast::<SystemError>()?),
            StructType::UnexpectedFault => Ok(from.as_any_box().downcast::<UnexpectedFault>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
