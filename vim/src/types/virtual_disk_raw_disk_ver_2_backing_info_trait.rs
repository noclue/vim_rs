use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type contains information about backing a virtual disk by
/// using a host device, as used by VMware Server.
pub trait VirtualDiskRawDiskVer2BackingInfoTrait : super::virtual_device_device_backing_info_trait::VirtualDeviceDeviceBackingInfoTrait {
    /// The name of the raw disk descriptor file.
    fn get_descriptor_file_name(&self) -> &str;
    /// Disk UUID for the virtual disk, if available.
    fn get_uuid(&self) -> &Option<String>;
    /// The change ID of the virtual disk for the corresponding
    /// snapshot or virtual machine.
    /// 
    /// This can be used to track
    /// incremental changes to a virtual disk. See
    /// *VirtualMachine.QueryChangedDiskAreas*.
    fn get_change_id(&self) -> &Option<String>;
    /// The sharing mode of the virtual disk.
    /// 
    /// See *VirtualDiskSharing_enum*. The default value is
    /// no sharing.
    fn get_sharing(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn VirtualDiskRawDiskVer2BackingInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDiskRawDiskVer2BackingInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDiskRawDiskVer2BackingInfoVisitor)
            }
        }

struct VirtualDiskRawDiskVer2BackingInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualDiskRawDiskVer2BackingInfoVisitor {
    type Value = Box<dyn VirtualDiskRawDiskVer2BackingInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDiskRawDiskVer2BackingInfoTrait JSON object with a _typeName field")
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

impl VirtualDiskRawDiskVer2BackingInfoTrait for VirtualDiskRawDiskVer2BackingInfo {
    fn get_descriptor_file_name(&self) -> &str { &self.descriptor_file_name }
    fn get_uuid(&self) -> &Option<String> { &self.uuid }
    fn get_change_id(&self) -> &Option<String> { &self.change_id }
    fn get_sharing(&self) -> &Option<String> { &self.sharing }
}
impl VirtualDiskRawDiskVer2BackingInfoTrait for VirtualDiskPartitionedRawDiskVer2BackingInfo {
    fn get_descriptor_file_name(&self) -> &str { &self.descriptor_file_name }
    fn get_uuid(&self) -> &Option<String> { &self.uuid }
    fn get_change_id(&self) -> &Option<String> { &self.change_id }
    fn get_sharing(&self) -> &Option<String> { &self.sharing }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDiskRawDiskVer2BackingInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDiskRawDiskVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskRawDiskVer2BackingInfo>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskPartitionedRawDiskVer2BackingInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDiskRawDiskVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskRawDiskVer2BackingInfo>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskPartitionedRawDiskVer2BackingInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
