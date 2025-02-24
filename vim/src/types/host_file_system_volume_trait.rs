use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Detailed information about a file system.
/// 
/// This is a base type for derived types
/// that have more specific details about specific filesystem types.
/// 
/// Typically a FileSystem is exposed as a datatore
/// 
/// See also *DatastoreInfo*, *HostVmfsVolume*, *HostNasVolume*, *HostVffsVolume*, *HostLocalFileSystemVolume*
/// 
/// However, a FileSystemVolume need not be exposed a datastore., *HostVfatVolume*.
pub trait HostFileSystemVolumeTrait : super::data_object_trait::DataObjectTrait {
    /// FileSystemType of this particular file system
    /// See *HostFileSystemVolumeFileSystemType_enum*
    fn get_type(&self) -> &str;
    /// Name of the file system volume.
    fn get_name(&self) -> &str;
    /// The capacity of the file system volume, in bytes.
    fn get_capacity(&self) -> i64;
}
impl<'s> serde::Serialize for dyn HostFileSystemVolumeTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostFileSystemVolumeTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostFileSystemVolumeVisitor)
            }
        }

struct HostFileSystemVolumeVisitor;

impl<'de> de::Visitor<'de> for HostFileSystemVolumeVisitor {
    type Value = Box<dyn HostFileSystemVolumeTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostFileSystemVolumeTrait JSON object with a _typeName field")
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

impl HostFileSystemVolumeTrait for HostFileSystemVolume {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_name(&self) -> &str { &self.name }
    fn get_capacity(&self) -> i64 { self.capacity }
}
impl HostFileSystemVolumeTrait for HostLocalFileSystemVolume {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_name(&self) -> &str { &self.name }
    fn get_capacity(&self) -> i64 { self.capacity }
}
impl HostFileSystemVolumeTrait for HostNasVolume {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_name(&self) -> &str { &self.name }
    fn get_capacity(&self) -> i64 { self.capacity }
}
impl HostFileSystemVolumeTrait for HostPMemVolume {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_name(&self) -> &str { &self.name }
    fn get_capacity(&self) -> i64 { self.capacity }
}
impl HostFileSystemVolumeTrait for HostVfatVolume {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_name(&self) -> &str { &self.name }
    fn get_capacity(&self) -> i64 { self.capacity }
}
impl HostFileSystemVolumeTrait for HostVffsVolume {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_name(&self) -> &str { &self.name }
    fn get_capacity(&self) -> i64 { self.capacity }
}
impl HostFileSystemVolumeTrait for HostVmfsVolume {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_name(&self) -> &str { &self.name }
    fn get_capacity(&self) -> i64 { self.capacity }
}
impl HostFileSystemVolumeTrait for HostVvolVolume {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_name(&self) -> &str { &self.name }
    fn get_capacity(&self) -> i64 { self.capacity }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostFileSystemVolumeTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostFileSystemVolume => Some(from.as_any_ref().downcast_ref::<HostFileSystemVolume>()?),
            StructType::HostLocalFileSystemVolume => Some(from.as_any_ref().downcast_ref::<HostLocalFileSystemVolume>()?),
            StructType::HostNasVolume => Some(from.as_any_ref().downcast_ref::<HostNasVolume>()?),
            StructType::HostPMemVolume => Some(from.as_any_ref().downcast_ref::<HostPMemVolume>()?),
            StructType::HostVfatVolume => Some(from.as_any_ref().downcast_ref::<HostVfatVolume>()?),
            StructType::HostVffsVolume => Some(from.as_any_ref().downcast_ref::<HostVffsVolume>()?),
            StructType::HostVmfsVolume => Some(from.as_any_ref().downcast_ref::<HostVmfsVolume>()?),
            StructType::HostVvolVolume => Some(from.as_any_ref().downcast_ref::<HostVvolVolume>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostFileSystemVolume => Ok(from.as_any_box().downcast::<HostFileSystemVolume>()?),
            StructType::HostLocalFileSystemVolume => Ok(from.as_any_box().downcast::<HostLocalFileSystemVolume>()?),
            StructType::HostNasVolume => Ok(from.as_any_box().downcast::<HostNasVolume>()?),
            StructType::HostPMemVolume => Ok(from.as_any_box().downcast::<HostPMemVolume>()?),
            StructType::HostVfatVolume => Ok(from.as_any_box().downcast::<HostVfatVolume>()?),
            StructType::HostVffsVolume => Ok(from.as_any_box().downcast::<HostVffsVolume>()?),
            StructType::HostVmfsVolume => Ok(from.as_any_box().downcast::<HostVmfsVolume>()?),
            StructType::HostVvolVolume => Ok(from.as_any_box().downcast::<HostVvolVolume>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
