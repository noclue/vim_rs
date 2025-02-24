use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The common base type for all file-related exceptions.
pub trait FileFaultTrait : super::vim_fault_trait::VimFaultTrait {
    /// The file in question.
    fn get_file(&self) -> &str;
}
impl<'s> serde::Serialize for dyn FileFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn FileFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(FileFaultVisitor)
            }
        }

struct FileFaultVisitor;

impl<'de> de::Visitor<'de> for FileFaultVisitor {
    type Value = Box<dyn FileFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid FileFaultTrait JSON object with a _typeName field")
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

impl FileFaultTrait for FileFault {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for CannotAccessFile {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for CannotCreateFile {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for CannotDeleteFile {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for DirectoryNotEmpty {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for FileAlreadyExists {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for FileLocked {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for FileNameTooLong {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for FileNotFound {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for FileNotWritable {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for FileTooLarge {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for IncorrectFileType {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for NetworkCopyFault {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for NoDiskSpace {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for NotADirectory {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for NotAFile {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for TooManyConcurrentNativeClones {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for TooManyNativeCloneLevels {
    fn get_file(&self) -> &str { &self.file }
}
impl FileFaultTrait for TooManyNativeClonesOnFile {
    fn get_file(&self) -> &str { &self.file }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn FileFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::FileFault => Some(from.as_any_ref().downcast_ref::<FileFault>()?),
            StructType::CannotAccessFile => Some(from.as_any_ref().downcast_ref::<CannotAccessFile>()?),
            StructType::CannotCreateFile => Some(from.as_any_ref().downcast_ref::<CannotCreateFile>()?),
            StructType::CannotDeleteFile => Some(from.as_any_ref().downcast_ref::<CannotDeleteFile>()?),
            StructType::DirectoryNotEmpty => Some(from.as_any_ref().downcast_ref::<DirectoryNotEmpty>()?),
            StructType::FileAlreadyExists => Some(from.as_any_ref().downcast_ref::<FileAlreadyExists>()?),
            StructType::FileLocked => Some(from.as_any_ref().downcast_ref::<FileLocked>()?),
            StructType::FileNameTooLong => Some(from.as_any_ref().downcast_ref::<FileNameTooLong>()?),
            StructType::FileNotFound => Some(from.as_any_ref().downcast_ref::<FileNotFound>()?),
            StructType::FileNotWritable => Some(from.as_any_ref().downcast_ref::<FileNotWritable>()?),
            StructType::FileTooLarge => Some(from.as_any_ref().downcast_ref::<FileTooLarge>()?),
            StructType::IncorrectFileType => Some(from.as_any_ref().downcast_ref::<IncorrectFileType>()?),
            StructType::NetworkCopyFault => Some(from.as_any_ref().downcast_ref::<NetworkCopyFault>()?),
            StructType::NoDiskSpace => Some(from.as_any_ref().downcast_ref::<NoDiskSpace>()?),
            StructType::NotADirectory => Some(from.as_any_ref().downcast_ref::<NotADirectory>()?),
            StructType::NotAFile => Some(from.as_any_ref().downcast_ref::<NotAFile>()?),
            StructType::TooManyConcurrentNativeClones => Some(from.as_any_ref().downcast_ref::<TooManyConcurrentNativeClones>()?),
            StructType::TooManyNativeCloneLevels => Some(from.as_any_ref().downcast_ref::<TooManyNativeCloneLevels>()?),
            StructType::TooManyNativeClonesOnFile => Some(from.as_any_ref().downcast_ref::<TooManyNativeClonesOnFile>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::FileFault => Ok(from.as_any_box().downcast::<FileFault>()?),
            StructType::CannotAccessFile => Ok(from.as_any_box().downcast::<CannotAccessFile>()?),
            StructType::CannotCreateFile => Ok(from.as_any_box().downcast::<CannotCreateFile>()?),
            StructType::CannotDeleteFile => Ok(from.as_any_box().downcast::<CannotDeleteFile>()?),
            StructType::DirectoryNotEmpty => Ok(from.as_any_box().downcast::<DirectoryNotEmpty>()?),
            StructType::FileAlreadyExists => Ok(from.as_any_box().downcast::<FileAlreadyExists>()?),
            StructType::FileLocked => Ok(from.as_any_box().downcast::<FileLocked>()?),
            StructType::FileNameTooLong => Ok(from.as_any_box().downcast::<FileNameTooLong>()?),
            StructType::FileNotFound => Ok(from.as_any_box().downcast::<FileNotFound>()?),
            StructType::FileNotWritable => Ok(from.as_any_box().downcast::<FileNotWritable>()?),
            StructType::FileTooLarge => Ok(from.as_any_box().downcast::<FileTooLarge>()?),
            StructType::IncorrectFileType => Ok(from.as_any_box().downcast::<IncorrectFileType>()?),
            StructType::NetworkCopyFault => Ok(from.as_any_box().downcast::<NetworkCopyFault>()?),
            StructType::NoDiskSpace => Ok(from.as_any_box().downcast::<NoDiskSpace>()?),
            StructType::NotADirectory => Ok(from.as_any_box().downcast::<NotADirectory>()?),
            StructType::NotAFile => Ok(from.as_any_box().downcast::<NotAFile>()?),
            StructType::TooManyConcurrentNativeClones => Ok(from.as_any_box().downcast::<TooManyConcurrentNativeClones>()?),
            StructType::TooManyNativeCloneLevels => Ok(from.as_any_box().downcast::<TooManyNativeCloneLevels>()?),
            StructType::TooManyNativeClonesOnFile => Ok(from.as_any_box().downcast::<TooManyNativeClonesOnFile>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
