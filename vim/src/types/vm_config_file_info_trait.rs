use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type describes a virtual machine configuration file.
pub trait VmConfigFileInfoTrait : super::file_info_trait::FileInfoTrait {
    fn get_config_version(&self) -> Option<i32>;
    /// The encryption information of the virtual machine
    /// configuration file.
    /// 
    /// If encryption was selected in VmConfigFileQueryFlags then this
    /// field is always set. Inspect the VmConfigEncryptionInfo to
    /// determine if the virtual machine configuration file is encrypted.
    fn get_encryption(&self) -> &Option<VmConfigFileEncryptionInfo>;
}
impl<'s> serde::Serialize for dyn VmConfigFileInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmConfigFileInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmConfigFileInfoVisitor)
            }
        }

struct VmConfigFileInfoVisitor;

impl<'de> de::Visitor<'de> for VmConfigFileInfoVisitor {
    type Value = Box<dyn VmConfigFileInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmConfigFileInfoTrait JSON object with a _typeName field")
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

impl VmConfigFileInfoTrait for VmConfigFileInfo {
    fn get_config_version(&self) -> Option<i32> { self.config_version }
    fn get_encryption(&self) -> &Option<VmConfigFileEncryptionInfo> { &self.encryption }
}
impl VmConfigFileInfoTrait for TemplateConfigFileInfo {
    fn get_config_version(&self) -> Option<i32> { self.config_version }
    fn get_encryption(&self) -> &Option<VmConfigFileEncryptionInfo> { &self.encryption }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmConfigFileInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmConfigFileInfo => Some(from.as_any_ref().downcast_ref::<VmConfigFileInfo>()?),
            StructType::TemplateConfigFileInfo => Some(from.as_any_ref().downcast_ref::<TemplateConfigFileInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmConfigFileInfo => Ok(from.as_any_box().downcast::<VmConfigFileInfo>()?),
            StructType::TemplateConfigFileInfo => Ok(from.as_any_box().downcast::<TemplateConfigFileInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
