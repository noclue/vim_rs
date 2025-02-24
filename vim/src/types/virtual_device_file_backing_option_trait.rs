use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The FileBackingOption data class contains file-specific backing options.
pub trait VirtualDeviceFileBackingOptionTrait : super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait {
    /// Valid filename extension for the filename.
    /// 
    /// If no extensions are present, any file extension is acceptable.
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption>;
}
impl<'s> serde::Serialize for dyn VirtualDeviceFileBackingOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceFileBackingOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceFileBackingOptionVisitor)
            }
        }

struct VirtualDeviceFileBackingOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceFileBackingOptionVisitor {
    type Value = Box<dyn VirtualDeviceFileBackingOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceFileBackingOptionTrait JSON object with a _typeName field")
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

impl VirtualDeviceFileBackingOptionTrait for VirtualDeviceFileBackingOption {
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption> { &self.file_name_extensions }
}
impl VirtualDeviceFileBackingOptionTrait for VirtualCdromIsoBackingOption {
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption> { &self.file_name_extensions }
}
impl VirtualDeviceFileBackingOptionTrait for VirtualDiskFlatVer1BackingOption {
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption> { &self.file_name_extensions }
}
impl VirtualDeviceFileBackingOptionTrait for VirtualDiskFlatVer2BackingOption {
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption> { &self.file_name_extensions }
}
impl VirtualDeviceFileBackingOptionTrait for VirtualDiskLocalPMemBackingOption {
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption> { &self.file_name_extensions }
}
impl VirtualDeviceFileBackingOptionTrait for VirtualDiskSeSparseBackingOption {
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption> { &self.file_name_extensions }
}
impl VirtualDeviceFileBackingOptionTrait for VirtualDiskSparseVer1BackingOption {
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption> { &self.file_name_extensions }
}
impl VirtualDeviceFileBackingOptionTrait for VirtualDiskSparseVer2BackingOption {
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption> { &self.file_name_extensions }
}
impl VirtualDeviceFileBackingOptionTrait for VirtualFloppyImageBackingOption {
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption> { &self.file_name_extensions }
}
impl VirtualDeviceFileBackingOptionTrait for VirtualParallelPortFileBackingOption {
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption> { &self.file_name_extensions }
}
impl VirtualDeviceFileBackingOptionTrait for VirtualSerialPortFileBackingOption {
    fn get_file_name_extensions(&self) -> &Option<ChoiceOption> { &self.file_name_extensions }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceFileBackingOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceFileBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceFileBackingOption>()?),
            StructType::VirtualCdromIsoBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromIsoBackingOption>()?),
            StructType::VirtualDiskFlatVer1BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer1BackingOption>()?),
            StructType::VirtualDiskFlatVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer2BackingOption>()?),
            StructType::VirtualDiskLocalPMemBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskLocalPMemBackingOption>()?),
            StructType::VirtualDiskSeSparseBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskSeSparseBackingOption>()?),
            StructType::VirtualDiskSparseVer1BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer1BackingOption>()?),
            StructType::VirtualDiskSparseVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer2BackingOption>()?),
            StructType::VirtualFloppyImageBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualFloppyImageBackingOption>()?),
            StructType::VirtualParallelPortFileBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortFileBackingOption>()?),
            StructType::VirtualSerialPortFileBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortFileBackingOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceFileBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceFileBackingOption>()?),
            StructType::VirtualCdromIsoBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromIsoBackingOption>()?),
            StructType::VirtualDiskFlatVer1BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer1BackingOption>()?),
            StructType::VirtualDiskFlatVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer2BackingOption>()?),
            StructType::VirtualDiskLocalPMemBackingOption => Ok(from.as_any_box().downcast::<VirtualDiskLocalPMemBackingOption>()?),
            StructType::VirtualDiskSeSparseBackingOption => Ok(from.as_any_box().downcast::<VirtualDiskSeSparseBackingOption>()?),
            StructType::VirtualDiskSparseVer1BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer1BackingOption>()?),
            StructType::VirtualDiskSparseVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer2BackingOption>()?),
            StructType::VirtualFloppyImageBackingOption => Ok(from.as_any_box().downcast::<VirtualFloppyImageBackingOption>()?),
            StructType::VirtualParallelPortFileBackingOption => Ok(from.as_any_box().downcast::<VirtualParallelPortFileBackingOption>()?),
            StructType::VirtualSerialPortFileBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortFileBackingOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
