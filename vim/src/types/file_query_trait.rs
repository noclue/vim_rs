use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The data object type that describes the base query specification.
/// 
/// Contains query
/// filters and details that apply to every file. Querying only file details generally
/// does not require opening files and so is an efficient query. Derived types add
/// query parameters specific to the type of file.
pub trait FileQueryTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn FileQueryTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn FileQueryTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(FileQueryVisitor)
            }
        }

struct FileQueryVisitor;

impl<'de> de::Visitor<'de> for FileQueryVisitor {
    type Value = Box<dyn FileQueryTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid FileQueryTrait JSON object with a _typeName field")
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

impl FileQueryTrait for FileQuery {
}
impl FileQueryTrait for FloppyImageFileQuery {
}
impl FileQueryTrait for FolderFileQuery {
}
impl FileQueryTrait for IsoImageFileQuery {
}
impl FileQueryTrait for VmConfigFileQuery {
}
impl FileQueryTrait for TemplateConfigFileQuery {
}
impl FileQueryTrait for VmDiskFileQuery {
}
impl FileQueryTrait for VmLogFileQuery {
}
impl FileQueryTrait for VmNvramFileQuery {
}
impl FileQueryTrait for VmSnapshotFileQuery {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn FileQueryTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::FileQuery => Some(from.as_any_ref().downcast_ref::<FileQuery>()?),
            StructType::FloppyImageFileQuery => Some(from.as_any_ref().downcast_ref::<FloppyImageFileQuery>()?),
            StructType::FolderFileQuery => Some(from.as_any_ref().downcast_ref::<FolderFileQuery>()?),
            StructType::IsoImageFileQuery => Some(from.as_any_ref().downcast_ref::<IsoImageFileQuery>()?),
            StructType::VmConfigFileQuery => Some(from.as_any_ref().downcast_ref::<VmConfigFileQuery>()?),
            StructType::TemplateConfigFileQuery => Some(from.as_any_ref().downcast_ref::<TemplateConfigFileQuery>()?),
            StructType::VmDiskFileQuery => Some(from.as_any_ref().downcast_ref::<VmDiskFileQuery>()?),
            StructType::VmLogFileQuery => Some(from.as_any_ref().downcast_ref::<VmLogFileQuery>()?),
            StructType::VmNvramFileQuery => Some(from.as_any_ref().downcast_ref::<VmNvramFileQuery>()?),
            StructType::VmSnapshotFileQuery => Some(from.as_any_ref().downcast_ref::<VmSnapshotFileQuery>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::FileQuery => Ok(from.as_any_box().downcast::<FileQuery>()?),
            StructType::FloppyImageFileQuery => Ok(from.as_any_box().downcast::<FloppyImageFileQuery>()?),
            StructType::FolderFileQuery => Ok(from.as_any_box().downcast::<FolderFileQuery>()?),
            StructType::IsoImageFileQuery => Ok(from.as_any_box().downcast::<IsoImageFileQuery>()?),
            StructType::VmConfigFileQuery => Ok(from.as_any_box().downcast::<VmConfigFileQuery>()?),
            StructType::TemplateConfigFileQuery => Ok(from.as_any_box().downcast::<TemplateConfigFileQuery>()?),
            StructType::VmDiskFileQuery => Ok(from.as_any_box().downcast::<VmDiskFileQuery>()?),
            StructType::VmLogFileQuery => Ok(from.as_any_box().downcast::<VmLogFileQuery>()?),
            StructType::VmNvramFileQuery => Ok(from.as_any_box().downcast::<VmNvramFileQuery>()?),
            StructType::VmSnapshotFileQuery => Ok(from.as_any_box().downcast::<VmSnapshotFileQuery>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
