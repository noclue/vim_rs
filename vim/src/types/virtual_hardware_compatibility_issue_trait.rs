use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// There is a problem with the compatibility between the intended execution host
/// and the virtual machine.
/// 
/// This may be an error or warning depending on
/// the specific fault subclass.
pub trait VirtualHardwareCompatibilityIssueTrait : super::vm_config_fault_trait::VmConfigFaultTrait {
}
impl<'s> serde::Serialize for dyn VirtualHardwareCompatibilityIssueTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualHardwareCompatibilityIssueTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualHardwareCompatibilityIssueVisitor)
            }
        }

struct VirtualHardwareCompatibilityIssueVisitor;

impl<'de> de::Visitor<'de> for VirtualHardwareCompatibilityIssueVisitor {
    type Value = Box<dyn VirtualHardwareCompatibilityIssueTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualHardwareCompatibilityIssueTrait JSON object with a _typeName field")
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

impl VirtualHardwareCompatibilityIssueTrait for VirtualHardwareCompatibilityIssue {
}
impl VirtualHardwareCompatibilityIssueTrait for CpuIncompatible {
}
impl VirtualHardwareCompatibilityIssueTrait for CpuCompatibilityUnknown {
}
impl VirtualHardwareCompatibilityIssueTrait for CpuIncompatible1Ecx {
}
impl VirtualHardwareCompatibilityIssueTrait for CpuIncompatible81Edx {
}
impl VirtualHardwareCompatibilityIssueTrait for FaultToleranceCpuIncompatible {
}
impl VirtualHardwareCompatibilityIssueTrait for DeviceNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for DeviceBackingNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for DvPortNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for UnusedVirtualDiskBlocksNotScrubbed {
}
impl VirtualHardwareCompatibilityIssueTrait for VirtualDiskBlocksNotFullyProvisioned {
}
impl VirtualHardwareCompatibilityIssueTrait for DeviceControllerNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for DigestNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for FileBackedPortNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for MultiWriterNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for NonPersistentDisksNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for RdmNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for PhysCompatRdmNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for RawDiskNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for RemoteDeviceNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for SharedBusControllerNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for VmiNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for VirtualDiskModeNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for VirtualEthernetCardNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for DiskNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for IdeDiskNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for DrsVmotionIncompatibleFault {
}
impl VirtualHardwareCompatibilityIssueTrait for FeatureRequirementsNotMet {
}
impl VirtualHardwareCompatibilityIssueTrait for MemorySizeNotRecommended {
}
impl VirtualHardwareCompatibilityIssueTrait for MemorySizeNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for MemorySizeNotSupportedByDatastore {
}
impl VirtualHardwareCompatibilityIssueTrait for NotEnoughCpus {
}
impl VirtualHardwareCompatibilityIssueTrait for NotEnoughLogicalCpus {
}
impl VirtualHardwareCompatibilityIssueTrait for NumVirtualCoresPerSocketNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for NumVirtualCpusNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for StorageVmotionIncompatible {
}
impl VirtualHardwareCompatibilityIssueTrait for VirtualHardwareVersionNotSupported {
}
impl VirtualHardwareCompatibilityIssueTrait for WakeOnLanNotSupported {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualHardwareCompatibilityIssueTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
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
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
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
            _ => Err(from.as_any_box()),
        }
    }
}
