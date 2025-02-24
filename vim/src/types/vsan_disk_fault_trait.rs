use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base exception class for VSAN disk-related faults.
pub trait VsanDiskFaultTrait : super::vsan_fault_trait::VsanFaultTrait {
    /// The canonical name for the disk at hand, if applicable.
    /// 
    /// See also *ScsiLun.canonicalName*.
    fn get_device(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn VsanDiskFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VsanDiskFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VsanDiskFaultVisitor)
            }
        }

struct VsanDiskFaultVisitor;

impl<'de> de::Visitor<'de> for VsanDiskFaultVisitor {
    type Value = Box<dyn VsanDiskFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VsanDiskFaultTrait JSON object with a _typeName field")
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

impl VsanDiskFaultTrait for VsanDiskFault {
    fn get_device(&self) -> &Option<String> { &self.device }
}
impl VsanDiskFaultTrait for DiskHasPartitions {
    fn get_device(&self) -> &Option<String> { &self.device }
}
impl VsanDiskFaultTrait for DiskIsLastRemainingNonSsd {
    fn get_device(&self) -> &Option<String> { &self.device }
}
impl VsanDiskFaultTrait for DiskIsNonLocal {
    fn get_device(&self) -> &Option<String> { &self.device }
}
impl VsanDiskFaultTrait for DiskIsUsb {
    fn get_device(&self) -> &Option<String> { &self.device }
}
impl VsanDiskFaultTrait for DiskTooSmall {
    fn get_device(&self) -> &Option<String> { &self.device }
}
impl VsanDiskFaultTrait for DuplicateDisks {
    fn get_device(&self) -> &Option<String> { &self.device }
}
impl VsanDiskFaultTrait for InsufficientDisks {
    fn get_device(&self) -> &Option<String> { &self.device }
}
impl VsanDiskFaultTrait for VsanIncompatibleDiskMapping {
    fn get_device(&self) -> &Option<String> { &self.device }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VsanDiskFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
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
