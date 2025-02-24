use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Information for file backing of a virtual storage
/// object.
/// 
/// File backing is mainly used for virtual disks.
pub trait BaseConfigInfoFileBackingInfoTrait : super::base_config_info_backing_info_trait::BaseConfigInfoBackingInfoTrait {
    /// Full file path for the host file used in this backing.
    fn get_file_path(&self) -> &str;
    /// Id refers to the backed storage object where the virtual storage object
    /// is backed on.
    fn get_backing_object_id(&self) -> &Option<String>;
    /// The parent of this virtual disk file, if this is a delta disk backing.
    /// 
    /// This will be unset if this is the root disk backing.
    /// 
    /// Note that the type of the backing is consistent throughout the chain;
    /// any new delta disk backing which is added is of the same type as the
    /// original disk. Also note that since the parent backing is not being
    /// written to, it is possible that the parent backing may be shared among
    /// multiple disks.
    /// 
    /// Only raw disk mappings in
    /// *virtual compatibility mode* can have parents.
    fn get_parent(&self) -> &Option<Box<dyn super::base_config_info_file_backing_info_trait::BaseConfigInfoFileBackingInfoTrait>>;
    /// Size allocated by the FS for this file/chain/link/extent only.
    /// 
    /// This property is used only for a delta disk whose
    /// *BaseConfigInfoFileBackingInfo.parent* is set.
    fn get_delta_size_in_mb(&self) -> Option<i64>;
    /// key id used to encrypt the backing disk.
    fn get_key_id(&self) -> &Option<CryptoKeyId>;
}
impl<'s> serde::Serialize for dyn BaseConfigInfoFileBackingInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn BaseConfigInfoFileBackingInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(BaseConfigInfoFileBackingInfoVisitor)
            }
        }

struct BaseConfigInfoFileBackingInfoVisitor;

impl<'de> de::Visitor<'de> for BaseConfigInfoFileBackingInfoVisitor {
    type Value = Box<dyn BaseConfigInfoFileBackingInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid BaseConfigInfoFileBackingInfoTrait JSON object with a _typeName field")
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

impl BaseConfigInfoFileBackingInfoTrait for BaseConfigInfoFileBackingInfo {
    fn get_file_path(&self) -> &str { &self.file_path }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
    fn get_parent(&self) -> &Option<Box<dyn super::base_config_info_file_backing_info_trait::BaseConfigInfoFileBackingInfoTrait>> { &self.parent }
    fn get_delta_size_in_mb(&self) -> Option<i64> { self.delta_size_in_mb }
    fn get_key_id(&self) -> &Option<CryptoKeyId> { &self.key_id }
}
impl BaseConfigInfoFileBackingInfoTrait for BaseConfigInfoDiskFileBackingInfo {
    fn get_file_path(&self) -> &str { &self.file_path }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
    fn get_parent(&self) -> &Option<Box<dyn super::base_config_info_file_backing_info_trait::BaseConfigInfoFileBackingInfoTrait>> { &self.parent }
    fn get_delta_size_in_mb(&self) -> Option<i64> { self.delta_size_in_mb }
    fn get_key_id(&self) -> &Option<CryptoKeyId> { &self.key_id }
}
impl BaseConfigInfoFileBackingInfoTrait for BaseConfigInfoRawDiskMappingBackingInfo {
    fn get_file_path(&self) -> &str { &self.file_path }
    fn get_backing_object_id(&self) -> &Option<String> { &self.backing_object_id }
    fn get_parent(&self) -> &Option<Box<dyn super::base_config_info_file_backing_info_trait::BaseConfigInfoFileBackingInfoTrait>> { &self.parent }
    fn get_delta_size_in_mb(&self) -> Option<i64> { self.delta_size_in_mb }
    fn get_key_id(&self) -> &Option<CryptoKeyId> { &self.key_id }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn BaseConfigInfoFileBackingInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::BaseConfigInfoFileBackingInfo => Some(from.as_any_ref().downcast_ref::<BaseConfigInfoFileBackingInfo>()?),
            StructType::BaseConfigInfoDiskFileBackingInfo => Some(from.as_any_ref().downcast_ref::<BaseConfigInfoDiskFileBackingInfo>()?),
            StructType::BaseConfigInfoRawDiskMappingBackingInfo => Some(from.as_any_ref().downcast_ref::<BaseConfigInfoRawDiskMappingBackingInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::BaseConfigInfoFileBackingInfo => Ok(from.as_any_box().downcast::<BaseConfigInfoFileBackingInfo>()?),
            StructType::BaseConfigInfoDiskFileBackingInfo => Ok(from.as_any_box().downcast::<BaseConfigInfoDiskFileBackingInfo>()?),
            StructType::BaseConfigInfoRawDiskMappingBackingInfo => Ok(from.as_any_box().downcast::<BaseConfigInfoRawDiskMappingBackingInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
