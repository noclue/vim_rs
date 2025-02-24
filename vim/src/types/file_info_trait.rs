use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type contains rudimentary information about a file in a
/// datastore.
/// 
/// The information here is not meant to cover all information in
/// traditional file systems, but rather to provide sufficient information for files
/// that are associated with virtual machines. Derived types describe the known file
/// types for a datastore.
pub trait FileInfoTrait : super::data_object_trait::DataObjectTrait {
    /// The path relative to the folder path in the search results.
    fn get_path(&self) -> &str;
    /// User friendly name.
    fn get_friendly_name(&self) -> &Option<String>;
    /// The size of the file in bytes.
    fn get_file_size(&self) -> Option<i64>;
    /// The last date and time the file was modified.
    fn get_modification(&self) -> &Option<String>;
    /// The user name of the owner of the file.
    fn get_owner(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn FileInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn FileInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(FileInfoVisitor)
            }
        }

struct FileInfoVisitor;

impl<'de> de::Visitor<'de> for FileInfoVisitor {
    type Value = Box<dyn FileInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid FileInfoTrait JSON object with a _typeName field")
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

impl FileInfoTrait for FileInfo {
    fn get_path(&self) -> &str { &self.path }
    fn get_friendly_name(&self) -> &Option<String> { &self.friendly_name }
    fn get_file_size(&self) -> Option<i64> { self.file_size }
    fn get_modification(&self) -> &Option<String> { &self.modification }
    fn get_owner(&self) -> &Option<String> { &self.owner }
}
impl FileInfoTrait for FloppyImageFileInfo {
    fn get_path(&self) -> &str { &self.path }
    fn get_friendly_name(&self) -> &Option<String> { &self.friendly_name }
    fn get_file_size(&self) -> Option<i64> { self.file_size }
    fn get_modification(&self) -> &Option<String> { &self.modification }
    fn get_owner(&self) -> &Option<String> { &self.owner }
}
impl FileInfoTrait for FolderFileInfo {
    fn get_path(&self) -> &str { &self.path }
    fn get_friendly_name(&self) -> &Option<String> { &self.friendly_name }
    fn get_file_size(&self) -> Option<i64> { self.file_size }
    fn get_modification(&self) -> &Option<String> { &self.modification }
    fn get_owner(&self) -> &Option<String> { &self.owner }
}
impl FileInfoTrait for IsoImageFileInfo {
    fn get_path(&self) -> &str { &self.path }
    fn get_friendly_name(&self) -> &Option<String> { &self.friendly_name }
    fn get_file_size(&self) -> Option<i64> { self.file_size }
    fn get_modification(&self) -> &Option<String> { &self.modification }
    fn get_owner(&self) -> &Option<String> { &self.owner }
}
impl FileInfoTrait for VmConfigFileInfo {
    fn get_path(&self) -> &str { &self.path }
    fn get_friendly_name(&self) -> &Option<String> { &self.friendly_name }
    fn get_file_size(&self) -> Option<i64> { self.file_size }
    fn get_modification(&self) -> &Option<String> { &self.modification }
    fn get_owner(&self) -> &Option<String> { &self.owner }
}
impl FileInfoTrait for TemplateConfigFileInfo {
    fn get_path(&self) -> &str { &self.path }
    fn get_friendly_name(&self) -> &Option<String> { &self.friendly_name }
    fn get_file_size(&self) -> Option<i64> { self.file_size }
    fn get_modification(&self) -> &Option<String> { &self.modification }
    fn get_owner(&self) -> &Option<String> { &self.owner }
}
impl FileInfoTrait for VmDiskFileInfo {
    fn get_path(&self) -> &str { &self.path }
    fn get_friendly_name(&self) -> &Option<String> { &self.friendly_name }
    fn get_file_size(&self) -> Option<i64> { self.file_size }
    fn get_modification(&self) -> &Option<String> { &self.modification }
    fn get_owner(&self) -> &Option<String> { &self.owner }
}
impl FileInfoTrait for VmLogFileInfo {
    fn get_path(&self) -> &str { &self.path }
    fn get_friendly_name(&self) -> &Option<String> { &self.friendly_name }
    fn get_file_size(&self) -> Option<i64> { self.file_size }
    fn get_modification(&self) -> &Option<String> { &self.modification }
    fn get_owner(&self) -> &Option<String> { &self.owner }
}
impl FileInfoTrait for VmNvramFileInfo {
    fn get_path(&self) -> &str { &self.path }
    fn get_friendly_name(&self) -> &Option<String> { &self.friendly_name }
    fn get_file_size(&self) -> Option<i64> { self.file_size }
    fn get_modification(&self) -> &Option<String> { &self.modification }
    fn get_owner(&self) -> &Option<String> { &self.owner }
}
impl FileInfoTrait for VmSnapshotFileInfo {
    fn get_path(&self) -> &str { &self.path }
    fn get_friendly_name(&self) -> &Option<String> { &self.friendly_name }
    fn get_file_size(&self) -> Option<i64> { self.file_size }
    fn get_modification(&self) -> &Option<String> { &self.modification }
    fn get_owner(&self) -> &Option<String> { &self.owner }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn FileInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::FileInfo => Some(from.as_any_ref().downcast_ref::<FileInfo>()?),
            StructType::FloppyImageFileInfo => Some(from.as_any_ref().downcast_ref::<FloppyImageFileInfo>()?),
            StructType::FolderFileInfo => Some(from.as_any_ref().downcast_ref::<FolderFileInfo>()?),
            StructType::IsoImageFileInfo => Some(from.as_any_ref().downcast_ref::<IsoImageFileInfo>()?),
            StructType::VmConfigFileInfo => Some(from.as_any_ref().downcast_ref::<VmConfigFileInfo>()?),
            StructType::TemplateConfigFileInfo => Some(from.as_any_ref().downcast_ref::<TemplateConfigFileInfo>()?),
            StructType::VmDiskFileInfo => Some(from.as_any_ref().downcast_ref::<VmDiskFileInfo>()?),
            StructType::VmLogFileInfo => Some(from.as_any_ref().downcast_ref::<VmLogFileInfo>()?),
            StructType::VmNvramFileInfo => Some(from.as_any_ref().downcast_ref::<VmNvramFileInfo>()?),
            StructType::VmSnapshotFileInfo => Some(from.as_any_ref().downcast_ref::<VmSnapshotFileInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::FileInfo => Ok(from.as_any_box().downcast::<FileInfo>()?),
            StructType::FloppyImageFileInfo => Ok(from.as_any_box().downcast::<FloppyImageFileInfo>()?),
            StructType::FolderFileInfo => Ok(from.as_any_box().downcast::<FolderFileInfo>()?),
            StructType::IsoImageFileInfo => Ok(from.as_any_box().downcast::<IsoImageFileInfo>()?),
            StructType::VmConfigFileInfo => Ok(from.as_any_box().downcast::<VmConfigFileInfo>()?),
            StructType::TemplateConfigFileInfo => Ok(from.as_any_box().downcast::<TemplateConfigFileInfo>()?),
            StructType::VmDiskFileInfo => Ok(from.as_any_box().downcast::<VmDiskFileInfo>()?),
            StructType::VmLogFileInfo => Ok(from.as_any_box().downcast::<VmLogFileInfo>()?),
            StructType::VmNvramFileInfo => Ok(from.as_any_box().downcast::<VmNvramFileInfo>()?),
            StructType::VmSnapshotFileInfo => Ok(from.as_any_box().downcast::<VmSnapshotFileInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
