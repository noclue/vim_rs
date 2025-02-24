use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base exception class for VSAN-specific faults raised for host
/// or cluster operations.
/// 
/// See also *HostVsanSystem*, *ComputeResource.ReconfigureComputeResource_Task*.
pub trait VsanFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn VsanFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VsanFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VsanFaultVisitor)
            }
        }

struct VsanFaultVisitor;

impl<'de> de::Visitor<'de> for VsanFaultVisitor {
    type Value = Box<dyn VsanFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VsanFaultTrait JSON object with a _typeName field")
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

impl VsanFaultTrait for VsanFault {
}
impl VsanFaultTrait for CannotChangeVsanClusterUuid {
}
impl VsanFaultTrait for CannotChangeVsanNodeUuid {
}
impl VsanFaultTrait for CannotMoveVsanEnabledHost {
}
impl VsanFaultTrait for DestinationVsanDisabled {
}
impl VsanFaultTrait for VsanClusterUuidMismatch {
}
impl VsanFaultTrait for CannotReconfigureVsanWhenHaEnabled {
}
impl VsanFaultTrait for DuplicateVsanNetworkInterface {
}
impl VsanFaultTrait for VsanDiskFault {
}
impl VsanFaultTrait for DiskHasPartitions {
}
impl VsanFaultTrait for DiskIsLastRemainingNonSsd {
}
impl VsanFaultTrait for DiskIsNonLocal {
}
impl VsanFaultTrait for DiskIsUsb {
}
impl VsanFaultTrait for DiskTooSmall {
}
impl VsanFaultTrait for DuplicateDisks {
}
impl VsanFaultTrait for InsufficientDisks {
}
impl VsanFaultTrait for VsanIncompatibleDiskMapping {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VsanFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VsanFault => Some(from.as_any_ref().downcast_ref::<VsanFault>()?),
            StructType::CannotChangeVsanClusterUuid => Some(from.as_any_ref().downcast_ref::<CannotChangeVsanClusterUuid>()?),
            StructType::CannotChangeVsanNodeUuid => Some(from.as_any_ref().downcast_ref::<CannotChangeVsanNodeUuid>()?),
            StructType::CannotMoveVsanEnabledHost => Some(from.as_any_ref().downcast_ref::<CannotMoveVsanEnabledHost>()?),
            StructType::DestinationVsanDisabled => Some(from.as_any_ref().downcast_ref::<DestinationVsanDisabled>()?),
            StructType::VsanClusterUuidMismatch => Some(from.as_any_ref().downcast_ref::<VsanClusterUuidMismatch>()?),
            StructType::CannotReconfigureVsanWhenHaEnabled => Some(from.as_any_ref().downcast_ref::<CannotReconfigureVsanWhenHaEnabled>()?),
            StructType::DuplicateVsanNetworkInterface => Some(from.as_any_ref().downcast_ref::<DuplicateVsanNetworkInterface>()?),
            StructType::VsanDiskFault => Some(from.as_any_ref().downcast_ref::<VsanDiskFault>()?),
            StructType::DiskHasPartitions => Some(from.as_any_ref().downcast_ref::<DiskHasPartitions>()?),
            StructType::DiskIsLastRemainingNonSsd => Some(from.as_any_ref().downcast_ref::<DiskIsLastRemainingNonSsd>()?),
            StructType::DiskIsNonLocal => Some(from.as_any_ref().downcast_ref::<DiskIsNonLocal>()?),
            StructType::DiskIsUsb => Some(from.as_any_ref().downcast_ref::<DiskIsUsb>()?),
            StructType::DiskTooSmall => Some(from.as_any_ref().downcast_ref::<DiskTooSmall>()?),
            StructType::DuplicateDisks => Some(from.as_any_ref().downcast_ref::<DuplicateDisks>()?),
            StructType::InsufficientDisks => Some(from.as_any_ref().downcast_ref::<InsufficientDisks>()?),
            StructType::VsanIncompatibleDiskMapping => Some(from.as_any_ref().downcast_ref::<VsanIncompatibleDiskMapping>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VsanFault => Ok(from.as_any_box().downcast::<VsanFault>()?),
            StructType::CannotChangeVsanClusterUuid => Ok(from.as_any_box().downcast::<CannotChangeVsanClusterUuid>()?),
            StructType::CannotChangeVsanNodeUuid => Ok(from.as_any_box().downcast::<CannotChangeVsanNodeUuid>()?),
            StructType::CannotMoveVsanEnabledHost => Ok(from.as_any_box().downcast::<CannotMoveVsanEnabledHost>()?),
            StructType::DestinationVsanDisabled => Ok(from.as_any_box().downcast::<DestinationVsanDisabled>()?),
            StructType::VsanClusterUuidMismatch => Ok(from.as_any_box().downcast::<VsanClusterUuidMismatch>()?),
            StructType::CannotReconfigureVsanWhenHaEnabled => Ok(from.as_any_box().downcast::<CannotReconfigureVsanWhenHaEnabled>()?),
            StructType::DuplicateVsanNetworkInterface => Ok(from.as_any_box().downcast::<DuplicateVsanNetworkInterface>()?),
            StructType::VsanDiskFault => Ok(from.as_any_box().downcast::<VsanDiskFault>()?),
            StructType::DiskHasPartitions => Ok(from.as_any_box().downcast::<DiskHasPartitions>()?),
            StructType::DiskIsLastRemainingNonSsd => Ok(from.as_any_box().downcast::<DiskIsLastRemainingNonSsd>()?),
            StructType::DiskIsNonLocal => Ok(from.as_any_box().downcast::<DiskIsNonLocal>()?),
            StructType::DiskIsUsb => Ok(from.as_any_box().downcast::<DiskIsUsb>()?),
            StructType::DiskTooSmall => Ok(from.as_any_box().downcast::<DiskTooSmall>()?),
            StructType::DuplicateDisks => Ok(from.as_any_box().downcast::<DuplicateDisks>()?),
            StructType::InsufficientDisks => Ok(from.as_any_box().downcast::<InsufficientDisks>()?),
            StructType::VsanIncompatibleDiskMapping => Ok(from.as_any_box().downcast::<VsanIncompatibleDiskMapping>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
