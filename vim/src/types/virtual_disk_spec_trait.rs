use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Specification used to create or clone a virtual disk
pub trait VirtualDiskSpecTrait : super::data_object_trait::DataObjectTrait {
    /// The type of the new virtual disk.
    /// 
    /// See also *VirtualDiskType_enum*.
    fn get_disk_type(&self) -> &str;
    /// The type of the virtual disk adapter for the new virtual disk.
    /// 
    /// See also *VirtualDiskAdapterType_enum*.
    fn get_adapter_type(&self) -> &str;
}
impl<'s> serde::Serialize for dyn VirtualDiskSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDiskSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDiskSpecVisitor)
            }
        }

struct VirtualDiskSpecVisitor;

impl<'de> de::Visitor<'de> for VirtualDiskSpecVisitor {
    type Value = Box<dyn VirtualDiskSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDiskSpecTrait JSON object with a _typeName field")
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

impl VirtualDiskSpecTrait for VirtualDiskSpec {
    fn get_disk_type(&self) -> &str { &self.disk_type }
    fn get_adapter_type(&self) -> &str { &self.adapter_type }
}
impl VirtualDiskSpecTrait for DeviceBackedVirtualDiskSpec {
    fn get_disk_type(&self) -> &str { &self.disk_type }
    fn get_adapter_type(&self) -> &str { &self.adapter_type }
}
impl VirtualDiskSpecTrait for FileBackedVirtualDiskSpec {
    fn get_disk_type(&self) -> &str { &self.disk_type }
    fn get_adapter_type(&self) -> &str { &self.adapter_type }
}
impl VirtualDiskSpecTrait for SeSparseVirtualDiskSpec {
    fn get_disk_type(&self) -> &str { &self.disk_type }
    fn get_adapter_type(&self) -> &str { &self.adapter_type }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDiskSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDiskSpec => Some(from.as_any_ref().downcast_ref::<VirtualDiskSpec>()?),
            StructType::DeviceBackedVirtualDiskSpec => Some(from.as_any_ref().downcast_ref::<DeviceBackedVirtualDiskSpec>()?),
            StructType::FileBackedVirtualDiskSpec => Some(from.as_any_ref().downcast_ref::<FileBackedVirtualDiskSpec>()?),
            StructType::SeSparseVirtualDiskSpec => Some(from.as_any_ref().downcast_ref::<SeSparseVirtualDiskSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDiskSpec => Ok(from.as_any_box().downcast::<VirtualDiskSpec>()?),
            StructType::DeviceBackedVirtualDiskSpec => Ok(from.as_any_box().downcast::<DeviceBackedVirtualDiskSpec>()?),
            StructType::FileBackedVirtualDiskSpec => Ok(from.as_any_box().downcast::<FileBackedVirtualDiskSpec>()?),
            StructType::SeSparseVirtualDiskSpec => Ok(from.as_any_box().downcast::<SeSparseVirtualDiskSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
