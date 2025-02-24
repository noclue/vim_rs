use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The host does not support the backings for the disks specified by the virtual
/// machine.
/// 
/// For example, this fault is thrown if a virtual machine is created from
/// a template that specifies backings that the host does not have. Similarly, this fault
/// is thrown if a virtual machine is registered on a host that does not support the
/// specified backings.
pub trait DiskNotSupportedTrait : super::virtual_hardware_compatibility_issue_trait::VirtualHardwareCompatibilityIssueTrait {
    /// The ID of disk that is not supported.
    fn get_disk(&self) -> i32;
}
impl<'s> serde::Serialize for dyn DiskNotSupportedTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DiskNotSupportedTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DiskNotSupportedVisitor)
            }
        }

struct DiskNotSupportedVisitor;

impl<'de> de::Visitor<'de> for DiskNotSupportedVisitor {
    type Value = Box<dyn DiskNotSupportedTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DiskNotSupportedTrait JSON object with a _typeName field")
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

impl DiskNotSupportedTrait for DiskNotSupported {
    fn get_disk(&self) -> i32 { self.disk }
}
impl DiskNotSupportedTrait for IdeDiskNotSupported {
    fn get_disk(&self) -> i32 { self.disk }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DiskNotSupportedTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DiskNotSupported => Some(from.as_any_ref().downcast_ref::<DiskNotSupported>()?),
            StructType::IdeDiskNotSupported => Some(from.as_any_ref().downcast_ref::<IdeDiskNotSupported>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DiskNotSupported => Ok(from.as_any_box().downcast::<DiskNotSupported>()?),
            StructType::IdeDiskNotSupported => Ok(from.as_any_box().downcast::<IdeDiskNotSupported>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
