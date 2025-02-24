use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type contains the basic configuration for
/// a virtual storage object or a virtual storage object snapshot.
pub trait BaseConfigInfoTrait : super::data_object_trait::DataObjectTrait {
    /// ID of this object.
    fn get_id(&self) -> &Id;
    /// Descriptive name of this object.
    fn get_name(&self) -> &str;
    /// The date and time this object was created.
    fn get_create_time(&self) -> &str;
    /// Choice of the deletion behavior of this virtual storage object.
    /// 
    /// If not set, the default value is false.
    fn get_keep_after_delete_vm(&self) -> Option<bool>;
    /// Is virtual storage object relocation disabled.
    /// 
    /// If not set, the default value is false.
    fn get_relocation_disabled(&self) -> Option<bool>;
    /// Is virtual storage object supports native snapshot.
    /// 
    /// If not set, the default value is false.
    fn get_native_snapshot_supported(&self) -> Option<bool>;
    /// If Virtua storage object has changed block tracking enabled.
    /// 
    /// If not set, the default value is false.
    fn get_changed_block_tracking_enabled(&self) -> Option<bool>;
    /// Backing of this object.
    fn get_backing(&self) -> &Box<dyn super::base_config_info_backing_info_trait::BaseConfigInfoBackingInfoTrait>;
    /// Metadata associated with the FCD if available.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    fn get_metadata(&self) -> &Option<Vec<KeyValue>>;
    /// VClock associated with the fcd when the operation completed.
    /// 
    /// The files is unset if the operation is a retrieve.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    fn get_vclock(&self) -> &Option<VslmVClockInfo>;
    /// IDs of the IO Filters associated with the virtual disk.
    /// 
    /// See *IoFilterInfo.id*.
    /// The client cannot modify this information on a virtual machine.
    fn get_iofilter(&self) -> &Option<Vec<String>>;
}
impl<'s> serde::Serialize for dyn BaseConfigInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn BaseConfigInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(BaseConfigInfoVisitor)
            }
        }

struct BaseConfigInfoVisitor;

impl<'de> de::Visitor<'de> for BaseConfigInfoVisitor {
    type Value = Box<dyn BaseConfigInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid BaseConfigInfoTrait JSON object with a _typeName field")
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

impl BaseConfigInfoTrait for BaseConfigInfo {
    fn get_id(&self) -> &Id { &self.id }
    fn get_name(&self) -> &str { &self.name }
    fn get_create_time(&self) -> &str { &self.create_time }
    fn get_keep_after_delete_vm(&self) -> Option<bool> { self.keep_after_delete_vm }
    fn get_relocation_disabled(&self) -> Option<bool> { self.relocation_disabled }
    fn get_native_snapshot_supported(&self) -> Option<bool> { self.native_snapshot_supported }
    fn get_changed_block_tracking_enabled(&self) -> Option<bool> { self.changed_block_tracking_enabled }
    fn get_backing(&self) -> &Box<dyn super::base_config_info_backing_info_trait::BaseConfigInfoBackingInfoTrait> { &self.backing }
    fn get_metadata(&self) -> &Option<Vec<KeyValue>> { &self.metadata }
    fn get_vclock(&self) -> &Option<VslmVClockInfo> { &self.vclock }
    fn get_iofilter(&self) -> &Option<Vec<String>> { &self.iofilter }
}
impl BaseConfigInfoTrait for VStorageObjectConfigInfo {
    fn get_id(&self) -> &Id { &self.id }
    fn get_name(&self) -> &str { &self.name }
    fn get_create_time(&self) -> &str { &self.create_time }
    fn get_keep_after_delete_vm(&self) -> Option<bool> { self.keep_after_delete_vm }
    fn get_relocation_disabled(&self) -> Option<bool> { self.relocation_disabled }
    fn get_native_snapshot_supported(&self) -> Option<bool> { self.native_snapshot_supported }
    fn get_changed_block_tracking_enabled(&self) -> Option<bool> { self.changed_block_tracking_enabled }
    fn get_backing(&self) -> &Box<dyn super::base_config_info_backing_info_trait::BaseConfigInfoBackingInfoTrait> { &self.backing }
    fn get_metadata(&self) -> &Option<Vec<KeyValue>> { &self.metadata }
    fn get_vclock(&self) -> &Option<VslmVClockInfo> { &self.vclock }
    fn get_iofilter(&self) -> &Option<Vec<String>> { &self.iofilter }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn BaseConfigInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::BaseConfigInfo => Some(from.as_any_ref().downcast_ref::<BaseConfigInfo>()?),
            StructType::VStorageObjectConfigInfo => Some(from.as_any_ref().downcast_ref::<VStorageObjectConfigInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::BaseConfigInfo => Ok(from.as_any_box().downcast::<BaseConfigInfo>()?),
            StructType::VStorageObjectConfigInfo => Ok(from.as_any_box().downcast::<VStorageObjectConfigInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
