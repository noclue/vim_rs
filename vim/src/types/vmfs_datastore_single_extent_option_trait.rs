use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Datastore addition policy to use a single extent on the disk for a VMFS
/// datastore.
/// 
/// A single extent implies that one disk partition will be
/// created on the disk for creating or increasing the capacity of a VMFS datastore.
pub trait VmfsDatastoreSingleExtentOptionTrait : super::vmfs_datastore_base_option_trait::VmfsDatastoreBaseOptionTrait {
    /// The block range to be used as an extent in a VMFS datastore.
    fn get_vmfs_extent(&self) -> &HostDiskPartitionBlockRange;
}
impl<'s> serde::Serialize for dyn VmfsDatastoreSingleExtentOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmfsDatastoreSingleExtentOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmfsDatastoreSingleExtentOptionVisitor)
            }
        }

struct VmfsDatastoreSingleExtentOptionVisitor;

impl<'de> de::Visitor<'de> for VmfsDatastoreSingleExtentOptionVisitor {
    type Value = Box<dyn VmfsDatastoreSingleExtentOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmfsDatastoreSingleExtentOptionTrait JSON object with a _typeName field")
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

impl VmfsDatastoreSingleExtentOptionTrait for VmfsDatastoreSingleExtentOption {
    fn get_vmfs_extent(&self) -> &HostDiskPartitionBlockRange { &self.vmfs_extent }
}
impl VmfsDatastoreSingleExtentOptionTrait for VmfsDatastoreAllExtentOption {
    fn get_vmfs_extent(&self) -> &HostDiskPartitionBlockRange { &self.vmfs_extent }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmfsDatastoreSingleExtentOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmfsDatastoreSingleExtentOption => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreSingleExtentOption>()?),
            StructType::VmfsDatastoreAllExtentOption => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreAllExtentOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmfsDatastoreSingleExtentOption => Ok(from.as_any_box().downcast::<VmfsDatastoreSingleExtentOption>()?),
            StructType::VmfsDatastoreAllExtentOption => Ok(from.as_any_box().downcast::<VmfsDatastoreAllExtentOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
