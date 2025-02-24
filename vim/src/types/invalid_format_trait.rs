use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Throws when an invalid format is detected.
/// 
/// For example, when
/// a virtual machine is registered and the system is unable to parse
/// the files as a virtual machine.
pub trait InvalidFormatTrait : super::vm_config_fault_trait::VmConfigFaultTrait {
}
impl<'s> serde::Serialize for dyn InvalidFormatTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InvalidFormatTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InvalidFormatVisitor)
            }
        }

struct InvalidFormatVisitor;

impl<'de> de::Visitor<'de> for InvalidFormatVisitor {
    type Value = Box<dyn InvalidFormatTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InvalidFormatTrait JSON object with a _typeName field")
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

impl InvalidFormatTrait for InvalidFormat {
}
impl InvalidFormatTrait for InvalidDiskFormat {
}
impl InvalidFormatTrait for InvalidSnapshotFormat {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InvalidFormatTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidFormat => Some(from.as_any_ref().downcast_ref::<InvalidFormat>()?),
            StructType::InvalidDiskFormat => Some(from.as_any_ref().downcast_ref::<InvalidDiskFormat>()?),
            StructType::InvalidSnapshotFormat => Some(from.as_any_ref().downcast_ref::<InvalidSnapshotFormat>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidFormat => Ok(from.as_any_box().downcast::<InvalidFormat>()?),
            StructType::InvalidDiskFormat => Ok(from.as_any_box().downcast::<InvalidDiskFormat>()?),
            StructType::InvalidSnapshotFormat => Ok(from.as_any_box().downcast::<InvalidSnapshotFormat>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
