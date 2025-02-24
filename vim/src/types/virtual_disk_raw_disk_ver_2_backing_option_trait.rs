use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualDiskOption.RawDiskVer2BackingOption object type
/// contains the available options when backing a virtual disk
/// using a host device on VMware Server.
pub trait VirtualDiskRawDiskVer2BackingOptionTrait : super::virtual_device_device_backing_option_trait::VirtualDeviceDeviceBackingOptionTrait {
    /// Valid extensions for the filename of the raw disk descriptor
    /// file.
    fn get_descriptor_file_name_extensions(&self) -> &ChoiceOption;
    /// Flag to indicate whether this backing supports disk UUID property.
    fn get_uuid(&self) -> bool;
}
impl<'s> serde::Serialize for dyn VirtualDiskRawDiskVer2BackingOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDiskRawDiskVer2BackingOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDiskRawDiskVer2BackingOptionVisitor)
            }
        }

struct VirtualDiskRawDiskVer2BackingOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualDiskRawDiskVer2BackingOptionVisitor {
    type Value = Box<dyn VirtualDiskRawDiskVer2BackingOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDiskRawDiskVer2BackingOptionTrait JSON object with a _typeName field")
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

impl VirtualDiskRawDiskVer2BackingOptionTrait for VirtualDiskRawDiskVer2BackingOption {
    fn get_descriptor_file_name_extensions(&self) -> &ChoiceOption { &self.descriptor_file_name_extensions }
    fn get_uuid(&self) -> bool { self.uuid }
}
impl VirtualDiskRawDiskVer2BackingOptionTrait for VirtualDiskPartitionedRawDiskVer2BackingOption {
    fn get_descriptor_file_name_extensions(&self) -> &ChoiceOption { &self.descriptor_file_name_extensions }
    fn get_uuid(&self) -> bool { self.uuid }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDiskRawDiskVer2BackingOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDiskRawDiskVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskRawDiskVer2BackingOption>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskPartitionedRawDiskVer2BackingOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDiskRawDiskVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskRawDiskVer2BackingOption>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskPartitionedRawDiskVer2BackingOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
