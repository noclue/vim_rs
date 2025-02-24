use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An attempt is being made to move or copy a virtual machine's disk that has
/// associated snapshots, and preserving the snapshots is not supported
/// because of some aspect of the virtual machine configuration, virtual
/// machine power state, or the requested disk placement.
/// 
/// This is an error
/// for move operations (where the source is deleted after the copy) and a
/// warning for clones (where the source is preserved).
pub trait SnapshotCopyNotSupportedTrait : super::migration_fault_trait::MigrationFaultTrait {
}
impl<'s> serde::Serialize for dyn SnapshotCopyNotSupportedTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn SnapshotCopyNotSupportedTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(SnapshotCopyNotSupportedVisitor)
            }
        }

struct SnapshotCopyNotSupportedVisitor;

impl<'de> de::Visitor<'de> for SnapshotCopyNotSupportedVisitor {
    type Value = Box<dyn SnapshotCopyNotSupportedTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid SnapshotCopyNotSupportedTrait JSON object with a _typeName field")
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

impl SnapshotCopyNotSupportedTrait for SnapshotCopyNotSupported {
}
impl SnapshotCopyNotSupportedTrait for HotSnapshotMoveNotSupported {
}
impl SnapshotCopyNotSupportedTrait for SnapshotCloneNotSupported {
}
impl SnapshotCopyNotSupportedTrait for SnapshotMoveFromNonHomeNotSupported {
}
impl SnapshotCopyNotSupportedTrait for SnapshotMoveNotSupported {
}
impl SnapshotCopyNotSupportedTrait for SnapshotMoveToNonHomeNotSupported {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn SnapshotCopyNotSupportedTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::SnapshotCopyNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotCopyNotSupported>()?),
            StructType::HotSnapshotMoveNotSupported => Some(from.as_any_ref().downcast_ref::<HotSnapshotMoveNotSupported>()?),
            StructType::SnapshotCloneNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotCloneNotSupported>()?),
            StructType::SnapshotMoveFromNonHomeNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotMoveFromNonHomeNotSupported>()?),
            StructType::SnapshotMoveNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotMoveNotSupported>()?),
            StructType::SnapshotMoveToNonHomeNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotMoveToNonHomeNotSupported>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::SnapshotCopyNotSupported => Ok(from.as_any_box().downcast::<SnapshotCopyNotSupported>()?),
            StructType::HotSnapshotMoveNotSupported => Ok(from.as_any_box().downcast::<HotSnapshotMoveNotSupported>()?),
            StructType::SnapshotCloneNotSupported => Ok(from.as_any_box().downcast::<SnapshotCloneNotSupported>()?),
            StructType::SnapshotMoveFromNonHomeNotSupported => Ok(from.as_any_box().downcast::<SnapshotMoveFromNonHomeNotSupported>()?),
            StructType::SnapshotMoveNotSupported => Ok(from.as_any_box().downcast::<SnapshotMoveNotSupported>()?),
            StructType::SnapshotMoveToNonHomeNotSupported => Ok(from.as_any_box().downcast::<SnapshotMoveToNonHomeNotSupported>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
