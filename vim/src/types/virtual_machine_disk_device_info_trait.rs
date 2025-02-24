use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The DiskDeviceInfo class contains basic information about a specific disk hardware
/// device.
pub trait VirtualMachineDiskDeviceInfoTrait : super::virtual_machine_target_info_trait::VirtualMachineTargetInfoTrait {
    /// Size of disk
    fn get_capacity(&self) -> Option<i64>;
    /// List of known virtual machines using this physical disk as a backing
    /// 
    /// Refers instances of *VirtualMachine*.
    fn get_vm(&self) -> &Option<Vec<ManagedObjectReference>>;
}
impl<'s> serde::Serialize for dyn VirtualMachineDiskDeviceInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualMachineDiskDeviceInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualMachineDiskDeviceInfoVisitor)
            }
        }

struct VirtualMachineDiskDeviceInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualMachineDiskDeviceInfoVisitor {
    type Value = Box<dyn VirtualMachineDiskDeviceInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualMachineDiskDeviceInfoTrait JSON object with a _typeName field")
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

impl VirtualMachineDiskDeviceInfoTrait for VirtualMachineDiskDeviceInfo {
    fn get_capacity(&self) -> Option<i64> { self.capacity }
    fn get_vm(&self) -> &Option<Vec<ManagedObjectReference>> { &self.vm }
}
impl VirtualMachineDiskDeviceInfoTrait for VirtualMachineIdeDiskDeviceInfo {
    fn get_capacity(&self) -> Option<i64> { self.capacity }
    fn get_vm(&self) -> &Option<Vec<ManagedObjectReference>> { &self.vm }
}
impl VirtualMachineDiskDeviceInfoTrait for VirtualMachineScsiDiskDeviceInfo {
    fn get_capacity(&self) -> Option<i64> { self.capacity }
    fn get_vm(&self) -> &Option<Vec<ManagedObjectReference>> { &self.vm }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualMachineDiskDeviceInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineDiskDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineDiskDeviceInfo>()?),
            StructType::VirtualMachineIdeDiskDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineIdeDiskDeviceInfo>()?),
            StructType::VirtualMachineScsiDiskDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineScsiDiskDeviceInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineDiskDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineDiskDeviceInfo>()?),
            StructType::VirtualMachineIdeDiskDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineIdeDiskDeviceInfo>()?),
            StructType::VirtualMachineScsiDiskDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineScsiDiskDeviceInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
