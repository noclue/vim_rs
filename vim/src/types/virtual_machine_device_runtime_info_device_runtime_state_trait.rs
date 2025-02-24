use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Runtime state of a device.
/// 
/// Subclassed for information that is specific
/// to certain device types.
pub trait VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateVisitor)
            }
        }

struct VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateVisitor;

impl<'de> de::Visitor<'de> for VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateVisitor {
    type Value = Box<dyn VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateTrait JSON object with a _typeName field")
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

impl VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateTrait for VirtualMachineDeviceRuntimeInfoDeviceRuntimeState {
}
impl VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateTrait for VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineDeviceRuntimeInfoDeviceRuntimeState => Some(from.as_any_ref().downcast_ref::<VirtualMachineDeviceRuntimeInfoDeviceRuntimeState>()?),
            StructType::VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState => Some(from.as_any_ref().downcast_ref::<VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineDeviceRuntimeInfoDeviceRuntimeState => Ok(from.as_any_box().downcast::<VirtualMachineDeviceRuntimeInfoDeviceRuntimeState>()?),
            StructType::VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState => Ok(from.as_any_box().downcast::<VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
