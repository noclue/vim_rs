use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// <code>*VirtualDeviceFileBackingInfo*</code> is a data object type
/// for information about file backing for a device in a virtual machine.
pub trait VirtualDeviceFileBackingInfoTrait : super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait {
    /// Filename for the host file used in this backing.
    fn get_file_name(&self) -> &str;
    /// Reference to the datastore managed object where this file is stored.
    /// 
    /// If the file is not located on a datastore, then this reference is null.
    /// This is not used for configuration.
    /// 
    /// Refers instance of *Datastore*.
    fn get_datastore(&self) -> &Option<ManagedObjectReference>;
    /// Backing object's durable and unmutable identifier.
    /// 
    /// Each backing object has a unique identifier which is not settable.
    fn get_backing_object_id(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn VirtualDeviceFileBackingInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceFileBackingInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceFileBackingInfoVisitor)
            }
        }

struct VirtualDeviceFileBackingInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceFileBackingInfoVisitor {
    type Value = Box<dyn VirtualDeviceFileBackingInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceFileBackingInfoTrait JSON object with a _typeName field")
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

impl VirtualDeviceFileBackingInfoTrait for VirtualDeviceFileBackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualCdromIsoBackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualDiskFlatVer1BackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualDiskFlatVer2BackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualDiskLocalPMemBackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualDiskRawDiskMappingVer1BackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualDiskSeSparseBackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualDiskSparseVer1BackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualDiskSparseVer2BackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualFloppyImageBackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualNvdimmBackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualParallelPortFileBackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl VirtualDeviceFileBackingInfoTrait for VirtualSerialPortFileBackingInfo {
    fn get_file_name(&self) -> &str { &self.file_name }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceFileBackingInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceFileBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceFileBackingInfo>()?),
            StructType::VirtualCdromIsoBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromIsoBackingInfo>()?),
            StructType::VirtualDiskFlatVer1BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer1BackingInfo>()?),
            StructType::VirtualDiskFlatVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer2BackingInfo>()?),
            StructType::VirtualDiskLocalPMemBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskLocalPMemBackingInfo>()?),
            StructType::VirtualDiskRawDiskMappingVer1BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskRawDiskMappingVer1BackingInfo>()?),
            StructType::VirtualDiskSeSparseBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskSeSparseBackingInfo>()?),
            StructType::VirtualDiskSparseVer1BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer1BackingInfo>()?),
            StructType::VirtualDiskSparseVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer2BackingInfo>()?),
            StructType::VirtualFloppyImageBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualFloppyImageBackingInfo>()?),
            StructType::VirtualNvdimmBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualNvdimmBackingInfo>()?),
            StructType::VirtualParallelPortFileBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortFileBackingInfo>()?),
            StructType::VirtualSerialPortFileBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortFileBackingInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceFileBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceFileBackingInfo>()?),
            StructType::VirtualCdromIsoBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromIsoBackingInfo>()?),
            StructType::VirtualDiskFlatVer1BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer1BackingInfo>()?),
            StructType::VirtualDiskFlatVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer2BackingInfo>()?),
            StructType::VirtualDiskLocalPMemBackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskLocalPMemBackingInfo>()?),
            StructType::VirtualDiskRawDiskMappingVer1BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskRawDiskMappingVer1BackingInfo>()?),
            StructType::VirtualDiskSeSparseBackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskSeSparseBackingInfo>()?),
            StructType::VirtualDiskSparseVer1BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer1BackingInfo>()?),
            StructType::VirtualDiskSparseVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer2BackingInfo>()?),
            StructType::VirtualFloppyImageBackingInfo => Ok(from.as_any_box().downcast::<VirtualFloppyImageBackingInfo>()?),
            StructType::VirtualNvdimmBackingInfo => Ok(from.as_any_box().downcast::<VirtualNvdimmBackingInfo>()?),
            StructType::VirtualParallelPortFileBackingInfo => Ok(from.as_any_box().downcast::<VirtualParallelPortFileBackingInfo>()?),
            StructType::VirtualSerialPortFileBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortFileBackingInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
