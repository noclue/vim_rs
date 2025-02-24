use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Different attributes for a guest file.
/// - Check *GuestPosixFileAttributes*
///   for Posix guest files.
/// - Check *GuestWindowsFileAttributes*
///   for Windows guest files.
pub trait GuestFileAttributesTrait : super::data_object_trait::DataObjectTrait {
    /// The date and time the file was last modified.
    /// 
    /// If this property is not specified when passing a
    /// *GuestFileAttributes* object to
    /// *GuestFileManager.InitiateFileTransferToGuest*,
    /// the default value will be the time when the file is created inside the
    /// guest.
    fn get_modification_time(&self) -> &Option<String>;
    /// The date and time the file was last accessed.
    /// 
    /// If this property is not specified when passing a
    /// *GuestFileAttributes* object to
    /// *GuestFileManager.InitiateFileTransferToGuest*,
    /// the default value will be the time when the file is created inside the
    /// guest.
    fn get_access_time(&self) -> &Option<String>;
    /// The target for the file if it's a symbolic link.
    /// 
    /// This is currently only set for Linux guest operating systems,
    /// but may be supported in the
    /// future on Windows guest operating systems that support symbolic links.
    /// This property gives information about files when returned from
    /// *GuestFileManager.ListFilesInGuest* or
    /// *GuestFileManager.InitiateFileTransferFromGuest*
    /// as part of a *GuestFileAttributes* object.
    /// This property will be ignored when passing a
    /// *GuestFileAttributes* object to
    /// *GuestFileManager.InitiateFileTransferToGuest* or
    /// *GuestFileManager.ChangeFileAttributesInGuest*.
    /// If the file is a symbolic link, then the attributes of the target
    /// are returned, not those of the symbolic link.
    fn get_symlink_target(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn GuestFileAttributesTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn GuestFileAttributesTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(GuestFileAttributesVisitor)
            }
        }

struct GuestFileAttributesVisitor;

impl<'de> de::Visitor<'de> for GuestFileAttributesVisitor {
    type Value = Box<dyn GuestFileAttributesTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GuestFileAttributesTrait JSON object with a _typeName field")
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

impl GuestFileAttributesTrait for GuestFileAttributes {
    fn get_modification_time(&self) -> &Option<String> { &self.modification_time }
    fn get_access_time(&self) -> &Option<String> { &self.access_time }
    fn get_symlink_target(&self) -> &Option<String> { &self.symlink_target }
}
impl GuestFileAttributesTrait for GuestPosixFileAttributes {
    fn get_modification_time(&self) -> &Option<String> { &self.modification_time }
    fn get_access_time(&self) -> &Option<String> { &self.access_time }
    fn get_symlink_target(&self) -> &Option<String> { &self.symlink_target }
}
impl GuestFileAttributesTrait for GuestWindowsFileAttributes {
    fn get_modification_time(&self) -> &Option<String> { &self.modification_time }
    fn get_access_time(&self) -> &Option<String> { &self.access_time }
    fn get_symlink_target(&self) -> &Option<String> { &self.symlink_target }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn GuestFileAttributesTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestFileAttributes => Some(from.as_any_ref().downcast_ref::<GuestFileAttributes>()?),
            StructType::GuestPosixFileAttributes => Some(from.as_any_ref().downcast_ref::<GuestPosixFileAttributes>()?),
            StructType::GuestWindowsFileAttributes => Some(from.as_any_ref().downcast_ref::<GuestWindowsFileAttributes>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestFileAttributes => Ok(from.as_any_box().downcast::<GuestFileAttributes>()?),
            StructType::GuestPosixFileAttributes => Ok(from.as_any_box().downcast::<GuestPosixFileAttributes>()?),
            StructType::GuestWindowsFileAttributes => Ok(from.as_any_box().downcast::<GuestWindowsFileAttributes>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
