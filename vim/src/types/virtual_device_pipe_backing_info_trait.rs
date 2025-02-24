use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The <code>*VirtualDevicePipeBackingInfo*</code> data object type
/// defines information for using a named pipe as backing for a device
/// in a virtual machine.
pub trait VirtualDevicePipeBackingInfoTrait : super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait {
    /// Pipe name for the host pipe associated with this backing.
    fn get_pipe_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn VirtualDevicePipeBackingInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDevicePipeBackingInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDevicePipeBackingInfoVisitor)
            }
        }

struct VirtualDevicePipeBackingInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualDevicePipeBackingInfoVisitor {
    type Value = Box<dyn VirtualDevicePipeBackingInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDevicePipeBackingInfoTrait JSON object with a _typeName field")
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

impl VirtualDevicePipeBackingInfoTrait for VirtualDevicePipeBackingInfo {
    fn get_pipe_name(&self) -> &str { &self.pipe_name }
}
impl VirtualDevicePipeBackingInfoTrait for VirtualSerialPortPipeBackingInfo {
    fn get_pipe_name(&self) -> &str { &self.pipe_name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDevicePipeBackingInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDevicePipeBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDevicePipeBackingInfo>()?),
            StructType::VirtualSerialPortPipeBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortPipeBackingInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDevicePipeBackingInfo => Ok(from.as_any_box().downcast::<VirtualDevicePipeBackingInfo>()?),
            StructType::VirtualSerialPortPipeBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortPipeBackingInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
