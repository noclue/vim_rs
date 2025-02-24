use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base object type for issues that can occur when reassigning the execution
/// host of a virtual machine using migrate or relocate.
/// 
/// These issues are typically
/// used as argument in the MigrationEvent. When a MigrationFault is used as a value
/// in a MigrationEvent, the type of MigrationEvent determines if the issue is a
/// warning or an error (for example, MigrationHostWarningEvent or
/// MigrationHostErrorEvent). When thrown as an exception, the fault is an error.
/// 
/// Issues are categorized as errors or warnings according to the following
/// criteria:
/// 
/// If the virtual machine is powered on:
/// 1. Error for fatal problems with the VMotion interfaces or licensing.
/// 2. Error if VMotion would fail.
/// 3. Error if VMotion would in any way interrupt the continuous and consistent
///    operation of the virtual machine.
/// 4. Warning for potential performance or connectivity problems between the
///    source and destination VMotion interfaces.
/// 5. Warning if the virtual machine's currently disconnected devices may not
///    be connectable after VMotion.
///    
/// If the virtual machine is powered off or suspended:
/// 1. Error if the destination host cannot access all the files that comprise
///    the virtual machine (including virtual disks).
/// 2. Error if aspects of the virtual machine are not supported by the
///    destination host's hardware or software.
/// 3. Warning if problems would occur when powering on or resuming the
///    virtual machine, if the usage/configuration of the destination
///    host were to remain in its current state.
pub trait MigrationFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn MigrationFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn MigrationFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(MigrationFaultVisitor)
            }
        }

struct MigrationFaultVisitor;

impl<'de> de::Visitor<'de> for MigrationFaultVisitor {
    type Value = Box<dyn MigrationFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid MigrationFaultTrait JSON object with a _typeName field")
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

impl MigrationFaultTrait for MigrationFault {
}
impl MigrationFaultTrait for AffinityConfigured {
}
impl MigrationFaultTrait for CannotModifyConfigCpuRequirements {
}
impl MigrationFaultTrait for CannotMoveVmWithDeltaDisk {
}
impl MigrationFaultTrait for CannotMoveVmWithNativeDeltaDisk {
}
impl MigrationFaultTrait for CloneFromSnapshotNotSupported {
}
impl MigrationFaultTrait for DatacenterMismatch {
}
impl MigrationFaultTrait for DisallowedMigrationDeviceAttached {
}
impl MigrationFaultTrait for DiskMoveTypeNotSupported {
}
impl MigrationFaultTrait for FaultToleranceAntiAffinityViolated {
}
impl MigrationFaultTrait for FaultToleranceNeedsThickDisk {
}
impl MigrationFaultTrait for FaultToleranceNotSameBuild {
}
impl MigrationFaultTrait for HaErrorsAtDest {
}
impl MigrationFaultTrait for IncompatibleDefaultDevice {
}
impl MigrationFaultTrait for LargeRdmConversionNotSupported {
}
impl MigrationFaultTrait for MaintenanceModeFileMove {
}
impl MigrationFaultTrait for MigrationDisabled {
}
impl MigrationFaultTrait for MigrationFeatureNotSupported {
}
impl MigrationFaultTrait for FullStorageVMotionNotSupported {
}
impl MigrationFaultTrait for IndependentDiskVMotionNotSupported {
}
impl MigrationFaultTrait for NonHomeRdmvMotionNotSupported {
}
impl MigrationFaultTrait for StorageVMotionNotSupported {
}
impl MigrationFaultTrait for UnsharedSwapVMotionNotSupported {
}
impl MigrationFaultTrait for VMotionAcrossNetworkNotSupported {
}
impl MigrationFaultTrait for MigrationNotReady {
}
impl MigrationFaultTrait for MismatchedNetworkPolicies {
}
impl MigrationFaultTrait for MismatchedVMotionNetworkNames {
}
impl MigrationFaultTrait for NetworksMayNotBeTheSame {
}
impl MigrationFaultTrait for NoGuestHeartbeat {
}
impl MigrationFaultTrait for RdmConversionNotSupported {
}
impl MigrationFaultTrait for RdmNotPreserved {
}
impl MigrationFaultTrait for ReadOnlyDisksWithLegacyDestination {
}
impl MigrationFaultTrait for SnapshotCopyNotSupported {
}
impl MigrationFaultTrait for HotSnapshotMoveNotSupported {
}
impl MigrationFaultTrait for SnapshotCloneNotSupported {
}
impl MigrationFaultTrait for SnapshotMoveFromNonHomeNotSupported {
}
impl MigrationFaultTrait for SnapshotMoveNotSupported {
}
impl MigrationFaultTrait for SnapshotMoveToNonHomeNotSupported {
}
impl MigrationFaultTrait for SnapshotRevertIssue {
}
impl MigrationFaultTrait for SuspendedRelocateNotSupported {
}
impl MigrationFaultTrait for TooManyDisksOnLegacyHost {
}
impl MigrationFaultTrait for ToolsInstallationInProgress {
}
impl MigrationFaultTrait for UncommittedUndoableDisk {
}
impl MigrationFaultTrait for VMotionInterfaceIssue {
}
impl MigrationFaultTrait for VMotionLinkCapacityLow {
}
impl MigrationFaultTrait for VMotionLinkDown {
}
impl MigrationFaultTrait for VMotionNotConfigured {
}
impl MigrationFaultTrait for VMotionNotLicensed {
}
impl MigrationFaultTrait for VMotionNotSupported {
}
impl MigrationFaultTrait for VMotionProtocolIncompatible {
}
impl MigrationFaultTrait for WillLoseHaProtection {
}
impl MigrationFaultTrait for WillModifyConfigCpuRequirements {
}
impl MigrationFaultTrait for WillResetSnapshotDirectory {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn MigrationFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::MigrationFault => Some(from.as_any_ref().downcast_ref::<MigrationFault>()?),
            StructType::AffinityConfigured => Some(from.as_any_ref().downcast_ref::<AffinityConfigured>()?),
            StructType::CannotModifyConfigCpuRequirements => Some(from.as_any_ref().downcast_ref::<CannotModifyConfigCpuRequirements>()?),
            StructType::CannotMoveVmWithDeltaDisk => Some(from.as_any_ref().downcast_ref::<CannotMoveVmWithDeltaDisk>()?),
            StructType::CannotMoveVmWithNativeDeltaDisk => Some(from.as_any_ref().downcast_ref::<CannotMoveVmWithNativeDeltaDisk>()?),
            StructType::CloneFromSnapshotNotSupported => Some(from.as_any_ref().downcast_ref::<CloneFromSnapshotNotSupported>()?),
            StructType::DatacenterMismatch => Some(from.as_any_ref().downcast_ref::<DatacenterMismatch>()?),
            StructType::DisallowedMigrationDeviceAttached => Some(from.as_any_ref().downcast_ref::<DisallowedMigrationDeviceAttached>()?),
            StructType::DiskMoveTypeNotSupported => Some(from.as_any_ref().downcast_ref::<DiskMoveTypeNotSupported>()?),
            StructType::FaultToleranceAntiAffinityViolated => Some(from.as_any_ref().downcast_ref::<FaultToleranceAntiAffinityViolated>()?),
            StructType::FaultToleranceNeedsThickDisk => Some(from.as_any_ref().downcast_ref::<FaultToleranceNeedsThickDisk>()?),
            StructType::FaultToleranceNotSameBuild => Some(from.as_any_ref().downcast_ref::<FaultToleranceNotSameBuild>()?),
            StructType::HaErrorsAtDest => Some(from.as_any_ref().downcast_ref::<HaErrorsAtDest>()?),
            StructType::IncompatibleDefaultDevice => Some(from.as_any_ref().downcast_ref::<IncompatibleDefaultDevice>()?),
            StructType::LargeRdmConversionNotSupported => Some(from.as_any_ref().downcast_ref::<LargeRdmConversionNotSupported>()?),
            StructType::MaintenanceModeFileMove => Some(from.as_any_ref().downcast_ref::<MaintenanceModeFileMove>()?),
            StructType::MigrationDisabled => Some(from.as_any_ref().downcast_ref::<MigrationDisabled>()?),
            StructType::MigrationFeatureNotSupported => Some(from.as_any_ref().downcast_ref::<MigrationFeatureNotSupported>()?),
            StructType::FullStorageVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<FullStorageVMotionNotSupported>()?),
            StructType::IndependentDiskVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<IndependentDiskVMotionNotSupported>()?),
            StructType::NonHomeRdmvMotionNotSupported => Some(from.as_any_ref().downcast_ref::<NonHomeRdmvMotionNotSupported>()?),
            StructType::StorageVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<StorageVMotionNotSupported>()?),
            StructType::UnsharedSwapVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<UnsharedSwapVMotionNotSupported>()?),
            StructType::VMotionAcrossNetworkNotSupported => Some(from.as_any_ref().downcast_ref::<VMotionAcrossNetworkNotSupported>()?),
            StructType::MigrationNotReady => Some(from.as_any_ref().downcast_ref::<MigrationNotReady>()?),
            StructType::MismatchedNetworkPolicies => Some(from.as_any_ref().downcast_ref::<MismatchedNetworkPolicies>()?),
            StructType::MismatchedVMotionNetworkNames => Some(from.as_any_ref().downcast_ref::<MismatchedVMotionNetworkNames>()?),
            StructType::NetworksMayNotBeTheSame => Some(from.as_any_ref().downcast_ref::<NetworksMayNotBeTheSame>()?),
            StructType::NoGuestHeartbeat => Some(from.as_any_ref().downcast_ref::<NoGuestHeartbeat>()?),
            StructType::RdmConversionNotSupported => Some(from.as_any_ref().downcast_ref::<RdmConversionNotSupported>()?),
            StructType::RdmNotPreserved => Some(from.as_any_ref().downcast_ref::<RdmNotPreserved>()?),
            StructType::ReadOnlyDisksWithLegacyDestination => Some(from.as_any_ref().downcast_ref::<ReadOnlyDisksWithLegacyDestination>()?),
            StructType::SnapshotCopyNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotCopyNotSupported>()?),
            StructType::HotSnapshotMoveNotSupported => Some(from.as_any_ref().downcast_ref::<HotSnapshotMoveNotSupported>()?),
            StructType::SnapshotCloneNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotCloneNotSupported>()?),
            StructType::SnapshotMoveFromNonHomeNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotMoveFromNonHomeNotSupported>()?),
            StructType::SnapshotMoveNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotMoveNotSupported>()?),
            StructType::SnapshotMoveToNonHomeNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotMoveToNonHomeNotSupported>()?),
            StructType::SnapshotRevertIssue => Some(from.as_any_ref().downcast_ref::<SnapshotRevertIssue>()?),
            StructType::SuspendedRelocateNotSupported => Some(from.as_any_ref().downcast_ref::<SuspendedRelocateNotSupported>()?),
            StructType::TooManyDisksOnLegacyHost => Some(from.as_any_ref().downcast_ref::<TooManyDisksOnLegacyHost>()?),
            StructType::ToolsInstallationInProgress => Some(from.as_any_ref().downcast_ref::<ToolsInstallationInProgress>()?),
            StructType::UncommittedUndoableDisk => Some(from.as_any_ref().downcast_ref::<UncommittedUndoableDisk>()?),
            StructType::VMotionInterfaceIssue => Some(from.as_any_ref().downcast_ref::<VMotionInterfaceIssue>()?),
            StructType::VMotionLinkCapacityLow => Some(from.as_any_ref().downcast_ref::<VMotionLinkCapacityLow>()?),
            StructType::VMotionLinkDown => Some(from.as_any_ref().downcast_ref::<VMotionLinkDown>()?),
            StructType::VMotionNotConfigured => Some(from.as_any_ref().downcast_ref::<VMotionNotConfigured>()?),
            StructType::VMotionNotLicensed => Some(from.as_any_ref().downcast_ref::<VMotionNotLicensed>()?),
            StructType::VMotionNotSupported => Some(from.as_any_ref().downcast_ref::<VMotionNotSupported>()?),
            StructType::VMotionProtocolIncompatible => Some(from.as_any_ref().downcast_ref::<VMotionProtocolIncompatible>()?),
            StructType::WillLoseHaProtection => Some(from.as_any_ref().downcast_ref::<WillLoseHaProtection>()?),
            StructType::WillModifyConfigCpuRequirements => Some(from.as_any_ref().downcast_ref::<WillModifyConfigCpuRequirements>()?),
            StructType::WillResetSnapshotDirectory => Some(from.as_any_ref().downcast_ref::<WillResetSnapshotDirectory>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::MigrationFault => Ok(from.as_any_box().downcast::<MigrationFault>()?),
            StructType::AffinityConfigured => Ok(from.as_any_box().downcast::<AffinityConfigured>()?),
            StructType::CannotModifyConfigCpuRequirements => Ok(from.as_any_box().downcast::<CannotModifyConfigCpuRequirements>()?),
            StructType::CannotMoveVmWithDeltaDisk => Ok(from.as_any_box().downcast::<CannotMoveVmWithDeltaDisk>()?),
            StructType::CannotMoveVmWithNativeDeltaDisk => Ok(from.as_any_box().downcast::<CannotMoveVmWithNativeDeltaDisk>()?),
            StructType::CloneFromSnapshotNotSupported => Ok(from.as_any_box().downcast::<CloneFromSnapshotNotSupported>()?),
            StructType::DatacenterMismatch => Ok(from.as_any_box().downcast::<DatacenterMismatch>()?),
            StructType::DisallowedMigrationDeviceAttached => Ok(from.as_any_box().downcast::<DisallowedMigrationDeviceAttached>()?),
            StructType::DiskMoveTypeNotSupported => Ok(from.as_any_box().downcast::<DiskMoveTypeNotSupported>()?),
            StructType::FaultToleranceAntiAffinityViolated => Ok(from.as_any_box().downcast::<FaultToleranceAntiAffinityViolated>()?),
            StructType::FaultToleranceNeedsThickDisk => Ok(from.as_any_box().downcast::<FaultToleranceNeedsThickDisk>()?),
            StructType::FaultToleranceNotSameBuild => Ok(from.as_any_box().downcast::<FaultToleranceNotSameBuild>()?),
            StructType::HaErrorsAtDest => Ok(from.as_any_box().downcast::<HaErrorsAtDest>()?),
            StructType::IncompatibleDefaultDevice => Ok(from.as_any_box().downcast::<IncompatibleDefaultDevice>()?),
            StructType::LargeRdmConversionNotSupported => Ok(from.as_any_box().downcast::<LargeRdmConversionNotSupported>()?),
            StructType::MaintenanceModeFileMove => Ok(from.as_any_box().downcast::<MaintenanceModeFileMove>()?),
            StructType::MigrationDisabled => Ok(from.as_any_box().downcast::<MigrationDisabled>()?),
            StructType::MigrationFeatureNotSupported => Ok(from.as_any_box().downcast::<MigrationFeatureNotSupported>()?),
            StructType::FullStorageVMotionNotSupported => Ok(from.as_any_box().downcast::<FullStorageVMotionNotSupported>()?),
            StructType::IndependentDiskVMotionNotSupported => Ok(from.as_any_box().downcast::<IndependentDiskVMotionNotSupported>()?),
            StructType::NonHomeRdmvMotionNotSupported => Ok(from.as_any_box().downcast::<NonHomeRdmvMotionNotSupported>()?),
            StructType::StorageVMotionNotSupported => Ok(from.as_any_box().downcast::<StorageVMotionNotSupported>()?),
            StructType::UnsharedSwapVMotionNotSupported => Ok(from.as_any_box().downcast::<UnsharedSwapVMotionNotSupported>()?),
            StructType::VMotionAcrossNetworkNotSupported => Ok(from.as_any_box().downcast::<VMotionAcrossNetworkNotSupported>()?),
            StructType::MigrationNotReady => Ok(from.as_any_box().downcast::<MigrationNotReady>()?),
            StructType::MismatchedNetworkPolicies => Ok(from.as_any_box().downcast::<MismatchedNetworkPolicies>()?),
            StructType::MismatchedVMotionNetworkNames => Ok(from.as_any_box().downcast::<MismatchedVMotionNetworkNames>()?),
            StructType::NetworksMayNotBeTheSame => Ok(from.as_any_box().downcast::<NetworksMayNotBeTheSame>()?),
            StructType::NoGuestHeartbeat => Ok(from.as_any_box().downcast::<NoGuestHeartbeat>()?),
            StructType::RdmConversionNotSupported => Ok(from.as_any_box().downcast::<RdmConversionNotSupported>()?),
            StructType::RdmNotPreserved => Ok(from.as_any_box().downcast::<RdmNotPreserved>()?),
            StructType::ReadOnlyDisksWithLegacyDestination => Ok(from.as_any_box().downcast::<ReadOnlyDisksWithLegacyDestination>()?),
            StructType::SnapshotCopyNotSupported => Ok(from.as_any_box().downcast::<SnapshotCopyNotSupported>()?),
            StructType::HotSnapshotMoveNotSupported => Ok(from.as_any_box().downcast::<HotSnapshotMoveNotSupported>()?),
            StructType::SnapshotCloneNotSupported => Ok(from.as_any_box().downcast::<SnapshotCloneNotSupported>()?),
            StructType::SnapshotMoveFromNonHomeNotSupported => Ok(from.as_any_box().downcast::<SnapshotMoveFromNonHomeNotSupported>()?),
            StructType::SnapshotMoveNotSupported => Ok(from.as_any_box().downcast::<SnapshotMoveNotSupported>()?),
            StructType::SnapshotMoveToNonHomeNotSupported => Ok(from.as_any_box().downcast::<SnapshotMoveToNonHomeNotSupported>()?),
            StructType::SnapshotRevertIssue => Ok(from.as_any_box().downcast::<SnapshotRevertIssue>()?),
            StructType::SuspendedRelocateNotSupported => Ok(from.as_any_box().downcast::<SuspendedRelocateNotSupported>()?),
            StructType::TooManyDisksOnLegacyHost => Ok(from.as_any_box().downcast::<TooManyDisksOnLegacyHost>()?),
            StructType::ToolsInstallationInProgress => Ok(from.as_any_box().downcast::<ToolsInstallationInProgress>()?),
            StructType::UncommittedUndoableDisk => Ok(from.as_any_box().downcast::<UncommittedUndoableDisk>()?),
            StructType::VMotionInterfaceIssue => Ok(from.as_any_box().downcast::<VMotionInterfaceIssue>()?),
            StructType::VMotionLinkCapacityLow => Ok(from.as_any_box().downcast::<VMotionLinkCapacityLow>()?),
            StructType::VMotionLinkDown => Ok(from.as_any_box().downcast::<VMotionLinkDown>()?),
            StructType::VMotionNotConfigured => Ok(from.as_any_box().downcast::<VMotionNotConfigured>()?),
            StructType::VMotionNotLicensed => Ok(from.as_any_box().downcast::<VMotionNotLicensed>()?),
            StructType::VMotionNotSupported => Ok(from.as_any_box().downcast::<VMotionNotSupported>()?),
            StructType::VMotionProtocolIncompatible => Ok(from.as_any_box().downcast::<VMotionProtocolIncompatible>()?),
            StructType::WillLoseHaProtection => Ok(from.as_any_box().downcast::<WillLoseHaProtection>()?),
            StructType::WillModifyConfigCpuRequirements => Ok(from.as_any_box().downcast::<WillModifyConfigCpuRequirements>()?),
            StructType::WillResetSnapshotDirectory => Ok(from.as_any_box().downcast::<WillResetSnapshotDirectory>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
