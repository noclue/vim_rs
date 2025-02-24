use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Detailed information about a datastore.
/// 
/// This is a base type for derived types
/// that have more specific details about a datastore.
/// 
/// See also *HostVmfsVolume*, *HostNasVolume*, *HostLocalFileSystemVolume*.
pub trait DatastoreInfoTrait : super::data_object_trait::DataObjectTrait {
    /// The name of the datastore.
    fn get_name(&self) -> &str;
    /// The unique locator for the datastore.
    fn get_url(&self) -> &str;
    /// Free space of this datastore, in bytes.
    /// 
    /// The server periodically updates this
    /// value. It can be explicitly refreshed with the Refresh operation.
    fn get_free_space(&self) -> i64;
    /// The maximum size of a file that can reside on this file system volume.
    fn get_max_file_size(&self) -> i64;
    /// The maximum capacity of a virtual disk which can be created on this volume.
    fn get_max_virtual_disk_capacity(&self) -> Option<i64>;
    /// The maximum size of a snapshot or a swap file that can reside on this file system volume.
    fn get_max_memory_file_size(&self) -> i64;
    /// Time when the free-space and capacity values in *DatastoreInfo* and
    /// *DatastoreSummary* were updated.
    fn get_timestamp(&self) -> &Option<String>;
    /// The unique container ID of the datastore, if applicable.
    fn get_container_id(&self) -> &Option<String>;
    /// vSAN datastore container that this datastore is alias of.
    /// 
    /// If this
    /// field is unset then this datastore is not alias of any other vSAN
    /// datastore.
    /// See *DatastoreInfo.containerId*.
    fn get_alias_of(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn DatastoreInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DatastoreInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DatastoreInfoVisitor)
            }
        }

struct DatastoreInfoVisitor;

impl<'de> de::Visitor<'de> for DatastoreInfoVisitor {
    type Value = Box<dyn DatastoreInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DatastoreInfoTrait JSON object with a _typeName field")
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

impl DatastoreInfoTrait for DatastoreInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_url(&self) -> &str { &self.url }
    fn get_free_space(&self) -> i64 { self.free_space }
    fn get_max_file_size(&self) -> i64 { self.max_file_size }
    fn get_max_virtual_disk_capacity(&self) -> Option<i64> { self.max_virtual_disk_capacity }
    fn get_max_memory_file_size(&self) -> i64 { self.max_memory_file_size }
    fn get_timestamp(&self) -> &Option<String> { &self.timestamp }
    fn get_container_id(&self) -> &Option<String> { &self.container_id }
    fn get_alias_of(&self) -> &Option<String> { &self.alias_of }
}
impl DatastoreInfoTrait for LocalDatastoreInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_url(&self) -> &str { &self.url }
    fn get_free_space(&self) -> i64 { self.free_space }
    fn get_max_file_size(&self) -> i64 { self.max_file_size }
    fn get_max_virtual_disk_capacity(&self) -> Option<i64> { self.max_virtual_disk_capacity }
    fn get_max_memory_file_size(&self) -> i64 { self.max_memory_file_size }
    fn get_timestamp(&self) -> &Option<String> { &self.timestamp }
    fn get_container_id(&self) -> &Option<String> { &self.container_id }
    fn get_alias_of(&self) -> &Option<String> { &self.alias_of }
}
impl DatastoreInfoTrait for NasDatastoreInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_url(&self) -> &str { &self.url }
    fn get_free_space(&self) -> i64 { self.free_space }
    fn get_max_file_size(&self) -> i64 { self.max_file_size }
    fn get_max_virtual_disk_capacity(&self) -> Option<i64> { self.max_virtual_disk_capacity }
    fn get_max_memory_file_size(&self) -> i64 { self.max_memory_file_size }
    fn get_timestamp(&self) -> &Option<String> { &self.timestamp }
    fn get_container_id(&self) -> &Option<String> { &self.container_id }
    fn get_alias_of(&self) -> &Option<String> { &self.alias_of }
}
impl DatastoreInfoTrait for PMemDatastoreInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_url(&self) -> &str { &self.url }
    fn get_free_space(&self) -> i64 { self.free_space }
    fn get_max_file_size(&self) -> i64 { self.max_file_size }
    fn get_max_virtual_disk_capacity(&self) -> Option<i64> { self.max_virtual_disk_capacity }
    fn get_max_memory_file_size(&self) -> i64 { self.max_memory_file_size }
    fn get_timestamp(&self) -> &Option<String> { &self.timestamp }
    fn get_container_id(&self) -> &Option<String> { &self.container_id }
    fn get_alias_of(&self) -> &Option<String> { &self.alias_of }
}
impl DatastoreInfoTrait for VmfsDatastoreInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_url(&self) -> &str { &self.url }
    fn get_free_space(&self) -> i64 { self.free_space }
    fn get_max_file_size(&self) -> i64 { self.max_file_size }
    fn get_max_virtual_disk_capacity(&self) -> Option<i64> { self.max_virtual_disk_capacity }
    fn get_max_memory_file_size(&self) -> i64 { self.max_memory_file_size }
    fn get_timestamp(&self) -> &Option<String> { &self.timestamp }
    fn get_container_id(&self) -> &Option<String> { &self.container_id }
    fn get_alias_of(&self) -> &Option<String> { &self.alias_of }
}
impl DatastoreInfoTrait for VsanDatastoreInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_url(&self) -> &str { &self.url }
    fn get_free_space(&self) -> i64 { self.free_space }
    fn get_max_file_size(&self) -> i64 { self.max_file_size }
    fn get_max_virtual_disk_capacity(&self) -> Option<i64> { self.max_virtual_disk_capacity }
    fn get_max_memory_file_size(&self) -> i64 { self.max_memory_file_size }
    fn get_timestamp(&self) -> &Option<String> { &self.timestamp }
    fn get_container_id(&self) -> &Option<String> { &self.container_id }
    fn get_alias_of(&self) -> &Option<String> { &self.alias_of }
}
impl DatastoreInfoTrait for VvolDatastoreInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_url(&self) -> &str { &self.url }
    fn get_free_space(&self) -> i64 { self.free_space }
    fn get_max_file_size(&self) -> i64 { self.max_file_size }
    fn get_max_virtual_disk_capacity(&self) -> Option<i64> { self.max_virtual_disk_capacity }
    fn get_max_memory_file_size(&self) -> i64 { self.max_memory_file_size }
    fn get_timestamp(&self) -> &Option<String> { &self.timestamp }
    fn get_container_id(&self) -> &Option<String> { &self.container_id }
    fn get_alias_of(&self) -> &Option<String> { &self.alias_of }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DatastoreInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DatastoreInfo => Some(from.as_any_ref().downcast_ref::<DatastoreInfo>()?),
            StructType::LocalDatastoreInfo => Some(from.as_any_ref().downcast_ref::<LocalDatastoreInfo>()?),
            StructType::NasDatastoreInfo => Some(from.as_any_ref().downcast_ref::<NasDatastoreInfo>()?),
            StructType::PMemDatastoreInfo => Some(from.as_any_ref().downcast_ref::<PMemDatastoreInfo>()?),
            StructType::VmfsDatastoreInfo => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreInfo>()?),
            StructType::VsanDatastoreInfo => Some(from.as_any_ref().downcast_ref::<VsanDatastoreInfo>()?),
            StructType::VvolDatastoreInfo => Some(from.as_any_ref().downcast_ref::<VvolDatastoreInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DatastoreInfo => Ok(from.as_any_box().downcast::<DatastoreInfo>()?),
            StructType::LocalDatastoreInfo => Ok(from.as_any_box().downcast::<LocalDatastoreInfo>()?),
            StructType::NasDatastoreInfo => Ok(from.as_any_box().downcast::<NasDatastoreInfo>()?),
            StructType::PMemDatastoreInfo => Ok(from.as_any_box().downcast::<PMemDatastoreInfo>()?),
            StructType::VmfsDatastoreInfo => Ok(from.as_any_box().downcast::<VmfsDatastoreInfo>()?),
            StructType::VsanDatastoreInfo => Ok(from.as_any_box().downcast::<VsanDatastoreInfo>()?),
            StructType::VvolDatastoreInfo => Ok(from.as_any_box().downcast::<VvolDatastoreInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
