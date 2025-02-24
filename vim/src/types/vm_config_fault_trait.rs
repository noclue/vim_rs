use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base for configuration / environment issues that can be thrown when powering on or
/// changing the configuration of a virtual machine.
/// 
/// Subclasses of this fault is also
/// used as recent why a migration can fail.
pub trait VmConfigFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn VmConfigFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmConfigFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmConfigFaultVisitor)
            }
        }

struct VmConfigFaultVisitor;

impl<'de> de::Visitor<'de> for VmConfigFaultVisitor {
    type Value = Box<dyn VmConfigFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmConfigFaultTrait JSON object with a _typeName field")
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

impl VmConfigFaultTrait for VmConfigFault {
}
impl VmConfigFaultTrait for CannotAccessVmComponent {
}
impl VmConfigFaultTrait for CannotAccessVmConfig {
}
impl VmConfigFaultTrait for CannotAccessVmDevice {
}
impl VmConfigFaultTrait for CannotAccessNetwork {
}
impl VmConfigFaultTrait for DestinationSwitchFull {
}
impl VmConfigFaultTrait for LegacyNetworkInterfaceInUse {
}
impl VmConfigFaultTrait for VmOnConflictDvPort {
}
impl VmConfigFaultTrait for VmOnVirtualIntranet {
}
impl VmConfigFaultTrait for CannotAccessVmDisk {
}
impl VmConfigFaultTrait for RdmPointsToInaccessibleDisk {
}
impl VmConfigFaultTrait for CannotDisableSnapshot {
}
impl VmConfigFaultTrait for CannotUseNetwork {
}
impl VmConfigFaultTrait for CpuHotPlugNotSupported {
}
impl VmConfigFaultTrait for DeltaDiskFormatNotSupported {
}
impl VmConfigFaultTrait for EightHostLimitViolated {
}
impl VmConfigFaultTrait for FaultToleranceCannotEditMem {
}
impl VmConfigFaultTrait for GenericVmConfigFault {
}
impl VmConfigFaultTrait for InvalidFormat {
}
impl VmConfigFaultTrait for InvalidDiskFormat {
}
impl VmConfigFaultTrait for InvalidSnapshotFormat {
}
impl VmConfigFaultTrait for InvalidVmConfig {
}
impl VmConfigFaultTrait for InvalidDeviceSpec {
}
impl VmConfigFaultTrait for DeviceHotPlugNotSupported {
}
impl VmConfigFaultTrait for DeviceNotFound {
}
impl VmConfigFaultTrait for DeviceUnsupportedForVmPlatform {
}
impl VmConfigFaultTrait for DeviceUnsupportedForVmVersion {
}
impl VmConfigFaultTrait for DisallowedDiskModeChange {
}
impl VmConfigFaultTrait for InvalidController {
}
impl VmConfigFaultTrait for InvalidDeviceBacking {
}
impl VmConfigFaultTrait for InvalidDeviceOperation {
}
impl VmConfigFaultTrait for MissingController {
}
impl VmConfigFaultTrait for SwapPlacementOverrideNotSupported {
}
impl VmConfigFaultTrait for TooManyDevices {
}
impl VmConfigFaultTrait for UnsupportedGuest {
}
impl VmConfigFaultTrait for VmWwnConflict {
}
impl VmConfigFaultTrait for LargeRdmNotSupportedOnDatastore {
}
impl VmConfigFaultTrait for MemoryHotPlugNotSupported {
}
impl VmConfigFaultTrait for NoCompatibleHardAffinityHost {
}
impl VmConfigFaultTrait for NoCompatibleSoftAffinityHost {
}
impl VmConfigFaultTrait for NumVirtualCpusIncompatible {
}
impl VmConfigFaultTrait for OvfConsumerValidationFault {
}
impl VmConfigFaultTrait for QuarantineModeFault {
}
impl VmConfigFaultTrait for RdmNotSupportedOnDatastore {
}
impl VmConfigFaultTrait for RuleViolation {
}
impl VmConfigFaultTrait for SoftRuleVioCorrectionDisallowed {
}
impl VmConfigFaultTrait for SoftRuleVioCorrectionImpact {
}
impl VmConfigFaultTrait for UnsupportedDatastore {
}
impl VmConfigFaultTrait for MemoryFileFormatNotSupportedByDatastore {
}
impl VmConfigFaultTrait for UnSupportedDatastoreForVFlash {
}
impl VmConfigFaultTrait for UnsupportedVmxLocation {
}
impl VmConfigFaultTrait for VAppNotRunning {
}
impl VmConfigFaultTrait for VAppPropertyFault {
}
impl VmConfigFaultTrait for InvalidNetworkInType {
}
impl VmConfigFaultTrait for InvalidPropertyType {
}
impl VmConfigFaultTrait for InvalidPropertyValue {
}
impl VmConfigFaultTrait for UnconfiguredPropertyValue {
}
impl VmConfigFaultTrait for MissingIpPool {
}
impl VmConfigFaultTrait for MissingNetworkIpConfig {
}
impl VmConfigFaultTrait for NoAvailableIp {
}
impl VmConfigFaultTrait for NoVcManagedIpConfigured {
}
impl VmConfigFaultTrait for NotUserConfigurableProperty {
}
impl VmConfigFaultTrait for VFlashCacheHotConfigNotSupported {
}
impl VmConfigFaultTrait for VFlashModuleNotSupported {
}
impl VmConfigFaultTrait for VirtualHardwareCompatibilityIssue {
}
impl VmConfigFaultTrait for CpuIncompatible {
}
impl VmConfigFaultTrait for CpuCompatibilityUnknown {
}
impl VmConfigFaultTrait for CpuIncompatible1Ecx {
}
impl VmConfigFaultTrait for CpuIncompatible81Edx {
}
impl VmConfigFaultTrait for FaultToleranceCpuIncompatible {
}
impl VmConfigFaultTrait for DeviceNotSupported {
}
impl VmConfigFaultTrait for DeviceBackingNotSupported {
}
impl VmConfigFaultTrait for DvPortNotSupported {
}
impl VmConfigFaultTrait for UnusedVirtualDiskBlocksNotScrubbed {
}
impl VmConfigFaultTrait for VirtualDiskBlocksNotFullyProvisioned {
}
impl VmConfigFaultTrait for DeviceControllerNotSupported {
}
impl VmConfigFaultTrait for DigestNotSupported {
}
impl VmConfigFaultTrait for FileBackedPortNotSupported {
}
impl VmConfigFaultTrait for MultiWriterNotSupported {
}
impl VmConfigFaultTrait for NonPersistentDisksNotSupported {
}
impl VmConfigFaultTrait for RdmNotSupported {
}
impl VmConfigFaultTrait for PhysCompatRdmNotSupported {
}
impl VmConfigFaultTrait for RawDiskNotSupported {
}
impl VmConfigFaultTrait for RemoteDeviceNotSupported {
}
impl VmConfigFaultTrait for SharedBusControllerNotSupported {
}
impl VmConfigFaultTrait for VmiNotSupported {
}
impl VmConfigFaultTrait for VirtualDiskModeNotSupported {
}
impl VmConfigFaultTrait for VirtualEthernetCardNotSupported {
}
impl VmConfigFaultTrait for DiskNotSupported {
}
impl VmConfigFaultTrait for IdeDiskNotSupported {
}
impl VmConfigFaultTrait for DrsVmotionIncompatibleFault {
}
impl VmConfigFaultTrait for FeatureRequirementsNotMet {
}
impl VmConfigFaultTrait for MemorySizeNotRecommended {
}
impl VmConfigFaultTrait for MemorySizeNotSupported {
}
impl VmConfigFaultTrait for MemorySizeNotSupportedByDatastore {
}
impl VmConfigFaultTrait for NotEnoughCpus {
}
impl VmConfigFaultTrait for NotEnoughLogicalCpus {
}
impl VmConfigFaultTrait for NumVirtualCoresPerSocketNotSupported {
}
impl VmConfigFaultTrait for NumVirtualCpusNotSupported {
}
impl VmConfigFaultTrait for StorageVmotionIncompatible {
}
impl VmConfigFaultTrait for VirtualHardwareVersionNotSupported {
}
impl VmConfigFaultTrait for WakeOnLanNotSupported {
}
impl VmConfigFaultTrait for VmConfigIncompatibleForFaultTolerance {
}
impl VmConfigFaultTrait for VmConfigIncompatibleForRecordReplay {
}
impl VmConfigFaultTrait for VmHostAffinityRuleViolation {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmConfigFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmConfigFault => Some(from.as_any_ref().downcast_ref::<VmConfigFault>()?),
            StructType::CannotAccessVmComponent => Some(from.as_any_ref().downcast_ref::<CannotAccessVmComponent>()?),
            StructType::CannotAccessVmConfig => Some(from.as_any_ref().downcast_ref::<CannotAccessVmConfig>()?),
            StructType::CannotAccessVmDevice => Some(from.as_any_ref().downcast_ref::<CannotAccessVmDevice>()?),
            StructType::CannotAccessNetwork => Some(from.as_any_ref().downcast_ref::<CannotAccessNetwork>()?),
            StructType::DestinationSwitchFull => Some(from.as_any_ref().downcast_ref::<DestinationSwitchFull>()?),
            StructType::LegacyNetworkInterfaceInUse => Some(from.as_any_ref().downcast_ref::<LegacyNetworkInterfaceInUse>()?),
            StructType::VmOnConflictDvPort => Some(from.as_any_ref().downcast_ref::<VmOnConflictDvPort>()?),
            StructType::VmOnVirtualIntranet => Some(from.as_any_ref().downcast_ref::<VmOnVirtualIntranet>()?),
            StructType::CannotAccessVmDisk => Some(from.as_any_ref().downcast_ref::<CannotAccessVmDisk>()?),
            StructType::RdmPointsToInaccessibleDisk => Some(from.as_any_ref().downcast_ref::<RdmPointsToInaccessibleDisk>()?),
            StructType::CannotDisableSnapshot => Some(from.as_any_ref().downcast_ref::<CannotDisableSnapshot>()?),
            StructType::CannotUseNetwork => Some(from.as_any_ref().downcast_ref::<CannotUseNetwork>()?),
            StructType::CpuHotPlugNotSupported => Some(from.as_any_ref().downcast_ref::<CpuHotPlugNotSupported>()?),
            StructType::DeltaDiskFormatNotSupported => Some(from.as_any_ref().downcast_ref::<DeltaDiskFormatNotSupported>()?),
            StructType::EightHostLimitViolated => Some(from.as_any_ref().downcast_ref::<EightHostLimitViolated>()?),
            StructType::FaultToleranceCannotEditMem => Some(from.as_any_ref().downcast_ref::<FaultToleranceCannotEditMem>()?),
            StructType::GenericVmConfigFault => Some(from.as_any_ref().downcast_ref::<GenericVmConfigFault>()?),
            StructType::InvalidFormat => Some(from.as_any_ref().downcast_ref::<InvalidFormat>()?),
            StructType::InvalidDiskFormat => Some(from.as_any_ref().downcast_ref::<InvalidDiskFormat>()?),
            StructType::InvalidSnapshotFormat => Some(from.as_any_ref().downcast_ref::<InvalidSnapshotFormat>()?),
            StructType::InvalidVmConfig => Some(from.as_any_ref().downcast_ref::<InvalidVmConfig>()?),
            StructType::InvalidDeviceSpec => Some(from.as_any_ref().downcast_ref::<InvalidDeviceSpec>()?),
            StructType::DeviceHotPlugNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceHotPlugNotSupported>()?),
            StructType::DeviceNotFound => Some(from.as_any_ref().downcast_ref::<DeviceNotFound>()?),
            StructType::DeviceUnsupportedForVmPlatform => Some(from.as_any_ref().downcast_ref::<DeviceUnsupportedForVmPlatform>()?),
            StructType::DeviceUnsupportedForVmVersion => Some(from.as_any_ref().downcast_ref::<DeviceUnsupportedForVmVersion>()?),
            StructType::DisallowedDiskModeChange => Some(from.as_any_ref().downcast_ref::<DisallowedDiskModeChange>()?),
            StructType::InvalidController => Some(from.as_any_ref().downcast_ref::<InvalidController>()?),
            StructType::InvalidDeviceBacking => Some(from.as_any_ref().downcast_ref::<InvalidDeviceBacking>()?),
            StructType::InvalidDeviceOperation => Some(from.as_any_ref().downcast_ref::<InvalidDeviceOperation>()?),
            StructType::MissingController => Some(from.as_any_ref().downcast_ref::<MissingController>()?),
            StructType::SwapPlacementOverrideNotSupported => Some(from.as_any_ref().downcast_ref::<SwapPlacementOverrideNotSupported>()?),
            StructType::TooManyDevices => Some(from.as_any_ref().downcast_ref::<TooManyDevices>()?),
            StructType::UnsupportedGuest => Some(from.as_any_ref().downcast_ref::<UnsupportedGuest>()?),
            StructType::VmWwnConflict => Some(from.as_any_ref().downcast_ref::<VmWwnConflict>()?),
            StructType::LargeRdmNotSupportedOnDatastore => Some(from.as_any_ref().downcast_ref::<LargeRdmNotSupportedOnDatastore>()?),
            StructType::MemoryHotPlugNotSupported => Some(from.as_any_ref().downcast_ref::<MemoryHotPlugNotSupported>()?),
            StructType::NoCompatibleHardAffinityHost => Some(from.as_any_ref().downcast_ref::<NoCompatibleHardAffinityHost>()?),
            StructType::NoCompatibleSoftAffinityHost => Some(from.as_any_ref().downcast_ref::<NoCompatibleSoftAffinityHost>()?),
            StructType::NumVirtualCpusIncompatible => Some(from.as_any_ref().downcast_ref::<NumVirtualCpusIncompatible>()?),
            StructType::OvfConsumerValidationFault => Some(from.as_any_ref().downcast_ref::<OvfConsumerValidationFault>()?),
            StructType::QuarantineModeFault => Some(from.as_any_ref().downcast_ref::<QuarantineModeFault>()?),
            StructType::RdmNotSupportedOnDatastore => Some(from.as_any_ref().downcast_ref::<RdmNotSupportedOnDatastore>()?),
            StructType::RuleViolation => Some(from.as_any_ref().downcast_ref::<RuleViolation>()?),
            StructType::SoftRuleVioCorrectionDisallowed => Some(from.as_any_ref().downcast_ref::<SoftRuleVioCorrectionDisallowed>()?),
            StructType::SoftRuleVioCorrectionImpact => Some(from.as_any_ref().downcast_ref::<SoftRuleVioCorrectionImpact>()?),
            StructType::UnsupportedDatastore => Some(from.as_any_ref().downcast_ref::<UnsupportedDatastore>()?),
            StructType::MemoryFileFormatNotSupportedByDatastore => Some(from.as_any_ref().downcast_ref::<MemoryFileFormatNotSupportedByDatastore>()?),
            StructType::UnSupportedDatastoreForVFlash => Some(from.as_any_ref().downcast_ref::<UnSupportedDatastoreForVFlash>()?),
            StructType::UnsupportedVmxLocation => Some(from.as_any_ref().downcast_ref::<UnsupportedVmxLocation>()?),
            StructType::VAppNotRunning => Some(from.as_any_ref().downcast_ref::<VAppNotRunning>()?),
            StructType::VAppPropertyFault => Some(from.as_any_ref().downcast_ref::<VAppPropertyFault>()?),
            StructType::InvalidNetworkInType => Some(from.as_any_ref().downcast_ref::<InvalidNetworkInType>()?),
            StructType::InvalidPropertyType => Some(from.as_any_ref().downcast_ref::<InvalidPropertyType>()?),
            StructType::InvalidPropertyValue => Some(from.as_any_ref().downcast_ref::<InvalidPropertyValue>()?),
            StructType::UnconfiguredPropertyValue => Some(from.as_any_ref().downcast_ref::<UnconfiguredPropertyValue>()?),
            StructType::MissingIpPool => Some(from.as_any_ref().downcast_ref::<MissingIpPool>()?),
            StructType::MissingNetworkIpConfig => Some(from.as_any_ref().downcast_ref::<MissingNetworkIpConfig>()?),
            StructType::NoAvailableIp => Some(from.as_any_ref().downcast_ref::<NoAvailableIp>()?),
            StructType::NoVcManagedIpConfigured => Some(from.as_any_ref().downcast_ref::<NoVcManagedIpConfigured>()?),
            StructType::NotUserConfigurableProperty => Some(from.as_any_ref().downcast_ref::<NotUserConfigurableProperty>()?),
            StructType::VFlashCacheHotConfigNotSupported => Some(from.as_any_ref().downcast_ref::<VFlashCacheHotConfigNotSupported>()?),
            StructType::VFlashModuleNotSupported => Some(from.as_any_ref().downcast_ref::<VFlashModuleNotSupported>()?),
            StructType::VirtualHardwareCompatibilityIssue => Some(from.as_any_ref().downcast_ref::<VirtualHardwareCompatibilityIssue>()?),
            StructType::CpuIncompatible => Some(from.as_any_ref().downcast_ref::<CpuIncompatible>()?),
            StructType::CpuCompatibilityUnknown => Some(from.as_any_ref().downcast_ref::<CpuCompatibilityUnknown>()?),
            StructType::CpuIncompatible1Ecx => Some(from.as_any_ref().downcast_ref::<CpuIncompatible1Ecx>()?),
            StructType::CpuIncompatible81Edx => Some(from.as_any_ref().downcast_ref::<CpuIncompatible81Edx>()?),
            StructType::FaultToleranceCpuIncompatible => Some(from.as_any_ref().downcast_ref::<FaultToleranceCpuIncompatible>()?),
            StructType::DeviceNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceNotSupported>()?),
            StructType::DeviceBackingNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceBackingNotSupported>()?),
            StructType::DvPortNotSupported => Some(from.as_any_ref().downcast_ref::<DvPortNotSupported>()?),
            StructType::UnusedVirtualDiskBlocksNotScrubbed => Some(from.as_any_ref().downcast_ref::<UnusedVirtualDiskBlocksNotScrubbed>()?),
            StructType::VirtualDiskBlocksNotFullyProvisioned => Some(from.as_any_ref().downcast_ref::<VirtualDiskBlocksNotFullyProvisioned>()?),
            StructType::DeviceControllerNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceControllerNotSupported>()?),
            StructType::DigestNotSupported => Some(from.as_any_ref().downcast_ref::<DigestNotSupported>()?),
            StructType::FileBackedPortNotSupported => Some(from.as_any_ref().downcast_ref::<FileBackedPortNotSupported>()?),
            StructType::MultiWriterNotSupported => Some(from.as_any_ref().downcast_ref::<MultiWriterNotSupported>()?),
            StructType::NonPersistentDisksNotSupported => Some(from.as_any_ref().downcast_ref::<NonPersistentDisksNotSupported>()?),
            StructType::RdmNotSupported => Some(from.as_any_ref().downcast_ref::<RdmNotSupported>()?),
            StructType::PhysCompatRdmNotSupported => Some(from.as_any_ref().downcast_ref::<PhysCompatRdmNotSupported>()?),
            StructType::RawDiskNotSupported => Some(from.as_any_ref().downcast_ref::<RawDiskNotSupported>()?),
            StructType::RemoteDeviceNotSupported => Some(from.as_any_ref().downcast_ref::<RemoteDeviceNotSupported>()?),
            StructType::SharedBusControllerNotSupported => Some(from.as_any_ref().downcast_ref::<SharedBusControllerNotSupported>()?),
            StructType::VmiNotSupported => Some(from.as_any_ref().downcast_ref::<VmiNotSupported>()?),
            StructType::VirtualDiskModeNotSupported => Some(from.as_any_ref().downcast_ref::<VirtualDiskModeNotSupported>()?),
            StructType::VirtualEthernetCardNotSupported => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardNotSupported>()?),
            StructType::DiskNotSupported => Some(from.as_any_ref().downcast_ref::<DiskNotSupported>()?),
            StructType::IdeDiskNotSupported => Some(from.as_any_ref().downcast_ref::<IdeDiskNotSupported>()?),
            StructType::DrsVmotionIncompatibleFault => Some(from.as_any_ref().downcast_ref::<DrsVmotionIncompatibleFault>()?),
            StructType::FeatureRequirementsNotMet => Some(from.as_any_ref().downcast_ref::<FeatureRequirementsNotMet>()?),
            StructType::MemorySizeNotRecommended => Some(from.as_any_ref().downcast_ref::<MemorySizeNotRecommended>()?),
            StructType::MemorySizeNotSupported => Some(from.as_any_ref().downcast_ref::<MemorySizeNotSupported>()?),
            StructType::MemorySizeNotSupportedByDatastore => Some(from.as_any_ref().downcast_ref::<MemorySizeNotSupportedByDatastore>()?),
            StructType::NotEnoughCpus => Some(from.as_any_ref().downcast_ref::<NotEnoughCpus>()?),
            StructType::NotEnoughLogicalCpus => Some(from.as_any_ref().downcast_ref::<NotEnoughLogicalCpus>()?),
            StructType::NumVirtualCoresPerSocketNotSupported => Some(from.as_any_ref().downcast_ref::<NumVirtualCoresPerSocketNotSupported>()?),
            StructType::NumVirtualCpusNotSupported => Some(from.as_any_ref().downcast_ref::<NumVirtualCpusNotSupported>()?),
            StructType::StorageVmotionIncompatible => Some(from.as_any_ref().downcast_ref::<StorageVmotionIncompatible>()?),
            StructType::VirtualHardwareVersionNotSupported => Some(from.as_any_ref().downcast_ref::<VirtualHardwareVersionNotSupported>()?),
            StructType::WakeOnLanNotSupported => Some(from.as_any_ref().downcast_ref::<WakeOnLanNotSupported>()?),
            StructType::VmConfigIncompatibleForFaultTolerance => Some(from.as_any_ref().downcast_ref::<VmConfigIncompatibleForFaultTolerance>()?),
            StructType::VmConfigIncompatibleForRecordReplay => Some(from.as_any_ref().downcast_ref::<VmConfigIncompatibleForRecordReplay>()?),
            StructType::VmHostAffinityRuleViolation => Some(from.as_any_ref().downcast_ref::<VmHostAffinityRuleViolation>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmConfigFault => Ok(from.as_any_box().downcast::<VmConfigFault>()?),
            StructType::CannotAccessVmComponent => Ok(from.as_any_box().downcast::<CannotAccessVmComponent>()?),
            StructType::CannotAccessVmConfig => Ok(from.as_any_box().downcast::<CannotAccessVmConfig>()?),
            StructType::CannotAccessVmDevice => Ok(from.as_any_box().downcast::<CannotAccessVmDevice>()?),
            StructType::CannotAccessNetwork => Ok(from.as_any_box().downcast::<CannotAccessNetwork>()?),
            StructType::DestinationSwitchFull => Ok(from.as_any_box().downcast::<DestinationSwitchFull>()?),
            StructType::LegacyNetworkInterfaceInUse => Ok(from.as_any_box().downcast::<LegacyNetworkInterfaceInUse>()?),
            StructType::VmOnConflictDvPort => Ok(from.as_any_box().downcast::<VmOnConflictDvPort>()?),
            StructType::VmOnVirtualIntranet => Ok(from.as_any_box().downcast::<VmOnVirtualIntranet>()?),
            StructType::CannotAccessVmDisk => Ok(from.as_any_box().downcast::<CannotAccessVmDisk>()?),
            StructType::RdmPointsToInaccessibleDisk => Ok(from.as_any_box().downcast::<RdmPointsToInaccessibleDisk>()?),
            StructType::CannotDisableSnapshot => Ok(from.as_any_box().downcast::<CannotDisableSnapshot>()?),
            StructType::CannotUseNetwork => Ok(from.as_any_box().downcast::<CannotUseNetwork>()?),
            StructType::CpuHotPlugNotSupported => Ok(from.as_any_box().downcast::<CpuHotPlugNotSupported>()?),
            StructType::DeltaDiskFormatNotSupported => Ok(from.as_any_box().downcast::<DeltaDiskFormatNotSupported>()?),
            StructType::EightHostLimitViolated => Ok(from.as_any_box().downcast::<EightHostLimitViolated>()?),
            StructType::FaultToleranceCannotEditMem => Ok(from.as_any_box().downcast::<FaultToleranceCannotEditMem>()?),
            StructType::GenericVmConfigFault => Ok(from.as_any_box().downcast::<GenericVmConfigFault>()?),
            StructType::InvalidFormat => Ok(from.as_any_box().downcast::<InvalidFormat>()?),
            StructType::InvalidDiskFormat => Ok(from.as_any_box().downcast::<InvalidDiskFormat>()?),
            StructType::InvalidSnapshotFormat => Ok(from.as_any_box().downcast::<InvalidSnapshotFormat>()?),
            StructType::InvalidVmConfig => Ok(from.as_any_box().downcast::<InvalidVmConfig>()?),
            StructType::InvalidDeviceSpec => Ok(from.as_any_box().downcast::<InvalidDeviceSpec>()?),
            StructType::DeviceHotPlugNotSupported => Ok(from.as_any_box().downcast::<DeviceHotPlugNotSupported>()?),
            StructType::DeviceNotFound => Ok(from.as_any_box().downcast::<DeviceNotFound>()?),
            StructType::DeviceUnsupportedForVmPlatform => Ok(from.as_any_box().downcast::<DeviceUnsupportedForVmPlatform>()?),
            StructType::DeviceUnsupportedForVmVersion => Ok(from.as_any_box().downcast::<DeviceUnsupportedForVmVersion>()?),
            StructType::DisallowedDiskModeChange => Ok(from.as_any_box().downcast::<DisallowedDiskModeChange>()?),
            StructType::InvalidController => Ok(from.as_any_box().downcast::<InvalidController>()?),
            StructType::InvalidDeviceBacking => Ok(from.as_any_box().downcast::<InvalidDeviceBacking>()?),
            StructType::InvalidDeviceOperation => Ok(from.as_any_box().downcast::<InvalidDeviceOperation>()?),
            StructType::MissingController => Ok(from.as_any_box().downcast::<MissingController>()?),
            StructType::SwapPlacementOverrideNotSupported => Ok(from.as_any_box().downcast::<SwapPlacementOverrideNotSupported>()?),
            StructType::TooManyDevices => Ok(from.as_any_box().downcast::<TooManyDevices>()?),
            StructType::UnsupportedGuest => Ok(from.as_any_box().downcast::<UnsupportedGuest>()?),
            StructType::VmWwnConflict => Ok(from.as_any_box().downcast::<VmWwnConflict>()?),
            StructType::LargeRdmNotSupportedOnDatastore => Ok(from.as_any_box().downcast::<LargeRdmNotSupportedOnDatastore>()?),
            StructType::MemoryHotPlugNotSupported => Ok(from.as_any_box().downcast::<MemoryHotPlugNotSupported>()?),
            StructType::NoCompatibleHardAffinityHost => Ok(from.as_any_box().downcast::<NoCompatibleHardAffinityHost>()?),
            StructType::NoCompatibleSoftAffinityHost => Ok(from.as_any_box().downcast::<NoCompatibleSoftAffinityHost>()?),
            StructType::NumVirtualCpusIncompatible => Ok(from.as_any_box().downcast::<NumVirtualCpusIncompatible>()?),
            StructType::OvfConsumerValidationFault => Ok(from.as_any_box().downcast::<OvfConsumerValidationFault>()?),
            StructType::QuarantineModeFault => Ok(from.as_any_box().downcast::<QuarantineModeFault>()?),
            StructType::RdmNotSupportedOnDatastore => Ok(from.as_any_box().downcast::<RdmNotSupportedOnDatastore>()?),
            StructType::RuleViolation => Ok(from.as_any_box().downcast::<RuleViolation>()?),
            StructType::SoftRuleVioCorrectionDisallowed => Ok(from.as_any_box().downcast::<SoftRuleVioCorrectionDisallowed>()?),
            StructType::SoftRuleVioCorrectionImpact => Ok(from.as_any_box().downcast::<SoftRuleVioCorrectionImpact>()?),
            StructType::UnsupportedDatastore => Ok(from.as_any_box().downcast::<UnsupportedDatastore>()?),
            StructType::MemoryFileFormatNotSupportedByDatastore => Ok(from.as_any_box().downcast::<MemoryFileFormatNotSupportedByDatastore>()?),
            StructType::UnSupportedDatastoreForVFlash => Ok(from.as_any_box().downcast::<UnSupportedDatastoreForVFlash>()?),
            StructType::UnsupportedVmxLocation => Ok(from.as_any_box().downcast::<UnsupportedVmxLocation>()?),
            StructType::VAppNotRunning => Ok(from.as_any_box().downcast::<VAppNotRunning>()?),
            StructType::VAppPropertyFault => Ok(from.as_any_box().downcast::<VAppPropertyFault>()?),
            StructType::InvalidNetworkInType => Ok(from.as_any_box().downcast::<InvalidNetworkInType>()?),
            StructType::InvalidPropertyType => Ok(from.as_any_box().downcast::<InvalidPropertyType>()?),
            StructType::InvalidPropertyValue => Ok(from.as_any_box().downcast::<InvalidPropertyValue>()?),
            StructType::UnconfiguredPropertyValue => Ok(from.as_any_box().downcast::<UnconfiguredPropertyValue>()?),
            StructType::MissingIpPool => Ok(from.as_any_box().downcast::<MissingIpPool>()?),
            StructType::MissingNetworkIpConfig => Ok(from.as_any_box().downcast::<MissingNetworkIpConfig>()?),
            StructType::NoAvailableIp => Ok(from.as_any_box().downcast::<NoAvailableIp>()?),
            StructType::NoVcManagedIpConfigured => Ok(from.as_any_box().downcast::<NoVcManagedIpConfigured>()?),
            StructType::NotUserConfigurableProperty => Ok(from.as_any_box().downcast::<NotUserConfigurableProperty>()?),
            StructType::VFlashCacheHotConfigNotSupported => Ok(from.as_any_box().downcast::<VFlashCacheHotConfigNotSupported>()?),
            StructType::VFlashModuleNotSupported => Ok(from.as_any_box().downcast::<VFlashModuleNotSupported>()?),
            StructType::VirtualHardwareCompatibilityIssue => Ok(from.as_any_box().downcast::<VirtualHardwareCompatibilityIssue>()?),
            StructType::CpuIncompatible => Ok(from.as_any_box().downcast::<CpuIncompatible>()?),
            StructType::CpuCompatibilityUnknown => Ok(from.as_any_box().downcast::<CpuCompatibilityUnknown>()?),
            StructType::CpuIncompatible1Ecx => Ok(from.as_any_box().downcast::<CpuIncompatible1Ecx>()?),
            StructType::CpuIncompatible81Edx => Ok(from.as_any_box().downcast::<CpuIncompatible81Edx>()?),
            StructType::FaultToleranceCpuIncompatible => Ok(from.as_any_box().downcast::<FaultToleranceCpuIncompatible>()?),
            StructType::DeviceNotSupported => Ok(from.as_any_box().downcast::<DeviceNotSupported>()?),
            StructType::DeviceBackingNotSupported => Ok(from.as_any_box().downcast::<DeviceBackingNotSupported>()?),
            StructType::DvPortNotSupported => Ok(from.as_any_box().downcast::<DvPortNotSupported>()?),
            StructType::UnusedVirtualDiskBlocksNotScrubbed => Ok(from.as_any_box().downcast::<UnusedVirtualDiskBlocksNotScrubbed>()?),
            StructType::VirtualDiskBlocksNotFullyProvisioned => Ok(from.as_any_box().downcast::<VirtualDiskBlocksNotFullyProvisioned>()?),
            StructType::DeviceControllerNotSupported => Ok(from.as_any_box().downcast::<DeviceControllerNotSupported>()?),
            StructType::DigestNotSupported => Ok(from.as_any_box().downcast::<DigestNotSupported>()?),
            StructType::FileBackedPortNotSupported => Ok(from.as_any_box().downcast::<FileBackedPortNotSupported>()?),
            StructType::MultiWriterNotSupported => Ok(from.as_any_box().downcast::<MultiWriterNotSupported>()?),
            StructType::NonPersistentDisksNotSupported => Ok(from.as_any_box().downcast::<NonPersistentDisksNotSupported>()?),
            StructType::RdmNotSupported => Ok(from.as_any_box().downcast::<RdmNotSupported>()?),
            StructType::PhysCompatRdmNotSupported => Ok(from.as_any_box().downcast::<PhysCompatRdmNotSupported>()?),
            StructType::RawDiskNotSupported => Ok(from.as_any_box().downcast::<RawDiskNotSupported>()?),
            StructType::RemoteDeviceNotSupported => Ok(from.as_any_box().downcast::<RemoteDeviceNotSupported>()?),
            StructType::SharedBusControllerNotSupported => Ok(from.as_any_box().downcast::<SharedBusControllerNotSupported>()?),
            StructType::VmiNotSupported => Ok(from.as_any_box().downcast::<VmiNotSupported>()?),
            StructType::VirtualDiskModeNotSupported => Ok(from.as_any_box().downcast::<VirtualDiskModeNotSupported>()?),
            StructType::VirtualEthernetCardNotSupported => Ok(from.as_any_box().downcast::<VirtualEthernetCardNotSupported>()?),
            StructType::DiskNotSupported => Ok(from.as_any_box().downcast::<DiskNotSupported>()?),
            StructType::IdeDiskNotSupported => Ok(from.as_any_box().downcast::<IdeDiskNotSupported>()?),
            StructType::DrsVmotionIncompatibleFault => Ok(from.as_any_box().downcast::<DrsVmotionIncompatibleFault>()?),
            StructType::FeatureRequirementsNotMet => Ok(from.as_any_box().downcast::<FeatureRequirementsNotMet>()?),
            StructType::MemorySizeNotRecommended => Ok(from.as_any_box().downcast::<MemorySizeNotRecommended>()?),
            StructType::MemorySizeNotSupported => Ok(from.as_any_box().downcast::<MemorySizeNotSupported>()?),
            StructType::MemorySizeNotSupportedByDatastore => Ok(from.as_any_box().downcast::<MemorySizeNotSupportedByDatastore>()?),
            StructType::NotEnoughCpus => Ok(from.as_any_box().downcast::<NotEnoughCpus>()?),
            StructType::NotEnoughLogicalCpus => Ok(from.as_any_box().downcast::<NotEnoughLogicalCpus>()?),
            StructType::NumVirtualCoresPerSocketNotSupported => Ok(from.as_any_box().downcast::<NumVirtualCoresPerSocketNotSupported>()?),
            StructType::NumVirtualCpusNotSupported => Ok(from.as_any_box().downcast::<NumVirtualCpusNotSupported>()?),
            StructType::StorageVmotionIncompatible => Ok(from.as_any_box().downcast::<StorageVmotionIncompatible>()?),
            StructType::VirtualHardwareVersionNotSupported => Ok(from.as_any_box().downcast::<VirtualHardwareVersionNotSupported>()?),
            StructType::WakeOnLanNotSupported => Ok(from.as_any_box().downcast::<WakeOnLanNotSupported>()?),
            StructType::VmConfigIncompatibleForFaultTolerance => Ok(from.as_any_box().downcast::<VmConfigIncompatibleForFaultTolerance>()?),
            StructType::VmConfigIncompatibleForRecordReplay => Ok(from.as_any_box().downcast::<VmConfigIncompatibleForRecordReplay>()?),
            StructType::VmHostAffinityRuleViolation => Ok(from.as_any_box().downcast::<VmHostAffinityRuleViolation>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
