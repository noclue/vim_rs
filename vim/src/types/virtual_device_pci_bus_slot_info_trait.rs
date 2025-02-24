use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The <code>*VirtualDevicePciBusSlotInfo*</code> data object type
/// defines information about a pci bus slot of pci device in a virtual machine.
pub trait VirtualDevicePciBusSlotInfoTrait : super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait {
    /// The pci slot number of the virtual device.
    /// 
    /// The pci slot number assignment should generally be left to the system.
    /// If assigned a value, and the value is invalid or duplicated, it will
    /// automatically be reassigned. This will not cause an error.
    /// 
    /// Generally, the PCI slot numbers should never be specified in an
    /// Reconfigure operation, and only in a CreateVM operation if i) they
    /// are specified for all devices, and ii) the numbers have been
    /// determined by looking at an existing VM configuration of similar
    /// hardware version. In other words, when the virtual hardware configuration
    /// is duplicated.
    fn get_pci_slot_number(&self) -> i32;
}
impl<'s> serde::Serialize for dyn VirtualDevicePciBusSlotInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDevicePciBusSlotInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDevicePciBusSlotInfoVisitor)
            }
        }

struct VirtualDevicePciBusSlotInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualDevicePciBusSlotInfoVisitor {
    type Value = Box<dyn VirtualDevicePciBusSlotInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDevicePciBusSlotInfoTrait JSON object with a _typeName field")
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

impl VirtualDevicePciBusSlotInfoTrait for VirtualDevicePciBusSlotInfo {
    fn get_pci_slot_number(&self) -> i32 { self.pci_slot_number }
}
impl VirtualDevicePciBusSlotInfoTrait for VirtualUsbControllerPciBusSlotInfo {
    fn get_pci_slot_number(&self) -> i32 { self.pci_slot_number }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDevicePciBusSlotInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDevicePciBusSlotInfo => Some(from.as_any_ref().downcast_ref::<VirtualDevicePciBusSlotInfo>()?),
            StructType::VirtualUsbControllerPciBusSlotInfo => Some(from.as_any_ref().downcast_ref::<VirtualUsbControllerPciBusSlotInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDevicePciBusSlotInfo => Ok(from.as_any_box().downcast::<VirtualDevicePciBusSlotInfo>()?),
            StructType::VirtualUsbControllerPciBusSlotInfo => Ok(from.as_any_box().downcast::<VirtualUsbControllerPciBusSlotInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
