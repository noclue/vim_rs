use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Bootable device.
pub trait VirtualMachineBootOptionsBootableDeviceTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn VirtualMachineBootOptionsBootableDeviceTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualMachineBootOptionsBootableDeviceTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualMachineBootOptionsBootableDeviceVisitor)
            }
        }

struct VirtualMachineBootOptionsBootableDeviceVisitor;

impl<'de> de::Visitor<'de> for VirtualMachineBootOptionsBootableDeviceVisitor {
    type Value = Box<dyn VirtualMachineBootOptionsBootableDeviceTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualMachineBootOptionsBootableDeviceTrait JSON object with a _typeName field")
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

impl VirtualMachineBootOptionsBootableDeviceTrait for VirtualMachineBootOptionsBootableDevice {
}
impl VirtualMachineBootOptionsBootableDeviceTrait for VirtualMachineBootOptionsBootableCdromDevice {
}
impl VirtualMachineBootOptionsBootableDeviceTrait for VirtualMachineBootOptionsBootableDiskDevice {
}
impl VirtualMachineBootOptionsBootableDeviceTrait for VirtualMachineBootOptionsBootableEthernetDevice {
}
impl VirtualMachineBootOptionsBootableDeviceTrait for VirtualMachineBootOptionsBootableFloppyDevice {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualMachineBootOptionsBootableDeviceTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineBootOptionsBootableDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableDevice>()?),
            StructType::VirtualMachineBootOptionsBootableCdromDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableCdromDevice>()?),
            StructType::VirtualMachineBootOptionsBootableDiskDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableDiskDevice>()?),
            StructType::VirtualMachineBootOptionsBootableEthernetDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableEthernetDevice>()?),
            StructType::VirtualMachineBootOptionsBootableFloppyDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableFloppyDevice>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineBootOptionsBootableDevice => Ok(from.as_any_box().downcast::<VirtualMachineBootOptionsBootableDevice>()?),
            StructType::VirtualMachineBootOptionsBootableCdromDevice => Ok(from.as_any_box().downcast::<VirtualMachineBootOptionsBootableCdromDevice>()?),
            StructType::VirtualMachineBootOptionsBootableDiskDevice => Ok(from.as_any_box().downcast::<VirtualMachineBootOptionsBootableDiskDevice>()?),
            StructType::VirtualMachineBootOptionsBootableEthernetDevice => Ok(from.as_any_box().downcast::<VirtualMachineBootOptionsBootableEthernetDevice>()?),
            StructType::VirtualMachineBootOptionsBootableFloppyDevice => Ok(from.as_any_box().downcast::<VirtualMachineBootOptionsBootableFloppyDevice>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
