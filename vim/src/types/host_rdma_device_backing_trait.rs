use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object represents the physical
/// backing of an RDMA device.
pub trait HostRdmaDeviceBackingTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn HostRdmaDeviceBackingTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostRdmaDeviceBackingTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostRdmaDeviceBackingVisitor)
            }
        }

struct HostRdmaDeviceBackingVisitor;

impl<'de> de::Visitor<'de> for HostRdmaDeviceBackingVisitor {
    type Value = Box<dyn HostRdmaDeviceBackingTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostRdmaDeviceBackingTrait JSON object with a _typeName field")
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

impl HostRdmaDeviceBackingTrait for HostRdmaDeviceBacking {
}
impl HostRdmaDeviceBackingTrait for HostRdmaDevicePnicBacking {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostRdmaDeviceBackingTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostRdmaDeviceBacking => Some(from.as_any_ref().downcast_ref::<HostRdmaDeviceBacking>()?),
            StructType::HostRdmaDevicePnicBacking => Some(from.as_any_ref().downcast_ref::<HostRdmaDevicePnicBacking>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostRdmaDeviceBacking => Ok(from.as_any_box().downcast::<HostRdmaDeviceBacking>()?),
            StructType::HostRdmaDevicePnicBacking => Ok(from.as_any_box().downcast::<HostRdmaDevicePnicBacking>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
