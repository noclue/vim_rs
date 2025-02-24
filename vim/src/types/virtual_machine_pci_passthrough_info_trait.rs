use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Description of a generic PCI device that can be attached to a virtual machine.
pub trait VirtualMachinePciPassthroughInfoTrait : super::virtual_machine_target_info_trait::VirtualMachineTargetInfoTrait {
    /// Details of the PCI device, including vendor, class and
    /// device identification information.
    fn get_pci_device(&self) -> &HostPciDevice;
    /// The ID of the system the PCI device is attached to.
    fn get_system_id(&self) -> &str;
}
impl<'s> serde::Serialize for dyn VirtualMachinePciPassthroughInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualMachinePciPassthroughInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualMachinePciPassthroughInfoVisitor)
            }
        }

struct VirtualMachinePciPassthroughInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualMachinePciPassthroughInfoVisitor {
    type Value = Box<dyn VirtualMachinePciPassthroughInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualMachinePciPassthroughInfoTrait JSON object with a _typeName field")
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

impl VirtualMachinePciPassthroughInfoTrait for VirtualMachinePciPassthroughInfo {
    fn get_pci_device(&self) -> &HostPciDevice { &self.pci_device }
    fn get_system_id(&self) -> &str { &self.system_id }
}
impl VirtualMachinePciPassthroughInfoTrait for VirtualMachineSriovInfo {
    fn get_pci_device(&self) -> &HostPciDevice { &self.pci_device }
    fn get_system_id(&self) -> &str { &self.system_id }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualMachinePciPassthroughInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachinePciPassthroughInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachinePciPassthroughInfo>()?),
            StructType::VirtualMachineSriovInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSriovInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachinePciPassthroughInfo => Ok(from.as_any_box().downcast::<VirtualMachinePciPassthroughInfo>()?),
            StructType::VirtualMachineSriovInfo => Ok(from.as_any_box().downcast::<VirtualMachineSriovInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
