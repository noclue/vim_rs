use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base type for Snapshot-related errors.
pub trait SnapshotFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn SnapshotFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn SnapshotFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(SnapshotFaultVisitor)
            }
        }

struct SnapshotFaultVisitor;

impl<'de> de::Visitor<'de> for SnapshotFaultVisitor {
    type Value = Box<dyn SnapshotFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid SnapshotFaultTrait JSON object with a _typeName field")
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

impl SnapshotFaultTrait for SnapshotFault {
}
impl SnapshotFaultTrait for ApplicationQuiesceFault {
}
impl SnapshotFaultTrait for FilesystemQuiesceFault {
}
impl SnapshotFaultTrait for MemorySnapshotOnIndependentDisk {
}
impl SnapshotFaultTrait for MultipleSnapshotsNotSupported {
}
impl SnapshotFaultTrait for SnapshotDisabled {
}
impl SnapshotFaultTrait for SnapshotIncompatibleDeviceInVm {
}
impl SnapshotFaultTrait for SnapshotLocked {
}
impl SnapshotFaultTrait for SnapshotNoChange {
}
impl SnapshotFaultTrait for TooManySnapshotLevels {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn SnapshotFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::SnapshotFault => Some(from.as_any_ref().downcast_ref::<SnapshotFault>()?),
            StructType::ApplicationQuiesceFault => Some(from.as_any_ref().downcast_ref::<ApplicationQuiesceFault>()?),
            StructType::FilesystemQuiesceFault => Some(from.as_any_ref().downcast_ref::<FilesystemQuiesceFault>()?),
            StructType::MemorySnapshotOnIndependentDisk => Some(from.as_any_ref().downcast_ref::<MemorySnapshotOnIndependentDisk>()?),
            StructType::MultipleSnapshotsNotSupported => Some(from.as_any_ref().downcast_ref::<MultipleSnapshotsNotSupported>()?),
            StructType::SnapshotDisabled => Some(from.as_any_ref().downcast_ref::<SnapshotDisabled>()?),
            StructType::SnapshotIncompatibleDeviceInVm => Some(from.as_any_ref().downcast_ref::<SnapshotIncompatibleDeviceInVm>()?),
            StructType::SnapshotLocked => Some(from.as_any_ref().downcast_ref::<SnapshotLocked>()?),
            StructType::SnapshotNoChange => Some(from.as_any_ref().downcast_ref::<SnapshotNoChange>()?),
            StructType::TooManySnapshotLevels => Some(from.as_any_ref().downcast_ref::<TooManySnapshotLevels>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::SnapshotFault => Ok(from.as_any_box().downcast::<SnapshotFault>()?),
            StructType::ApplicationQuiesceFault => Ok(from.as_any_box().downcast::<ApplicationQuiesceFault>()?),
            StructType::FilesystemQuiesceFault => Ok(from.as_any_box().downcast::<FilesystemQuiesceFault>()?),
            StructType::MemorySnapshotOnIndependentDisk => Ok(from.as_any_box().downcast::<MemorySnapshotOnIndependentDisk>()?),
            StructType::MultipleSnapshotsNotSupported => Ok(from.as_any_box().downcast::<MultipleSnapshotsNotSupported>()?),
            StructType::SnapshotDisabled => Ok(from.as_any_box().downcast::<SnapshotDisabled>()?),
            StructType::SnapshotIncompatibleDeviceInVm => Ok(from.as_any_box().downcast::<SnapshotIncompatibleDeviceInVm>()?),
            StructType::SnapshotLocked => Ok(from.as_any_box().downcast::<SnapshotLocked>()?),
            StructType::SnapshotNoChange => Ok(from.as_any_box().downcast::<SnapshotNoChange>()?),
            StructType::TooManySnapshotLevels => Ok(from.as_any_box().downcast::<TooManySnapshotLevels>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
