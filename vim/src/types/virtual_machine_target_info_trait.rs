use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The TargetInfo specified a value that can be used in the device backings to
/// connect the virtual machine to a physical (or logical) host device.
pub trait VirtualMachineTargetInfoTrait : super::data_object_trait::DataObjectTrait {
    /// The identification of the endpoint on the host.
    /// 
    /// The format of this depends
    /// on the kind of virtual device this endpoints is used for. For example,
    /// for a VirtualEthernetCard this would be a networkname, and for a VirtualCDROM
    /// it would be a device name.
    fn get_name(&self) -> &str;
    /// List of configurations that this device is available for.
    /// 
    /// This is only filled
    /// out if more than one configuration is requested.
    fn get_configuration_tag(&self) -> &Option<Vec<String>>;
}
impl<'s> serde::Serialize for dyn VirtualMachineTargetInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualMachineTargetInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualMachineTargetInfoVisitor)
            }
        }

struct VirtualMachineTargetInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualMachineTargetInfoVisitor {
    type Value = Box<dyn VirtualMachineTargetInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualMachineTargetInfoTrait JSON object with a _typeName field")
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

impl VirtualMachineTargetInfoTrait for VirtualMachineTargetInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineCdromInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineDatastoreInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineDiskDeviceInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineIdeDiskDeviceInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineScsiDiskDeviceInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineDynamicPassthroughInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineFloppyInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineNetworkInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for OpaqueNetworkTargetInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineParallelInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachinePciPassthroughInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineSriovInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachinePciSharedGpuPassthroughInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachinePrecisionClockInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineScsiPassthroughInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineSerialInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineSgxTargetInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineSoundInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineUsbInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineVFlashModuleInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineVMotionStunTimeInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineVendorDeviceGroupInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineVgpuDeviceInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl VirtualMachineTargetInfoTrait for VirtualMachineVgpuProfileInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_configuration_tag(&self) -> &Option<Vec<String>> { &self.configuration_tag }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualMachineTargetInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineTargetInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineTargetInfo>()?),
            StructType::VirtualMachineCdromInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineCdromInfo>()?),
            StructType::VirtualMachineDatastoreInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineDatastoreInfo>()?),
            StructType::VirtualMachineDiskDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineDiskDeviceInfo>()?),
            StructType::VirtualMachineIdeDiskDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineIdeDiskDeviceInfo>()?),
            StructType::VirtualMachineScsiDiskDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineScsiDiskDeviceInfo>()?),
            StructType::VirtualMachineDynamicPassthroughInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineDynamicPassthroughInfo>()?),
            StructType::VirtualMachineFloppyInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineFloppyInfo>()?),
            StructType::VirtualMachineNetworkInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineNetworkInfo>()?),
            StructType::OpaqueNetworkTargetInfo => Some(from.as_any_ref().downcast_ref::<OpaqueNetworkTargetInfo>()?),
            StructType::VirtualMachineParallelInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineParallelInfo>()?),
            StructType::VirtualMachinePciPassthroughInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachinePciPassthroughInfo>()?),
            StructType::VirtualMachineSriovInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSriovInfo>()?),
            StructType::VirtualMachinePciSharedGpuPassthroughInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachinePciSharedGpuPassthroughInfo>()?),
            StructType::VirtualMachinePrecisionClockInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachinePrecisionClockInfo>()?),
            StructType::VirtualMachineScsiPassthroughInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineScsiPassthroughInfo>()?),
            StructType::VirtualMachineSerialInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSerialInfo>()?),
            StructType::VirtualMachineSgxTargetInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSgxTargetInfo>()?),
            StructType::VirtualMachineSoundInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSoundInfo>()?),
            StructType::VirtualMachineUsbInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineUsbInfo>()?),
            StructType::VirtualMachineVFlashModuleInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVFlashModuleInfo>()?),
            StructType::VirtualMachineVMotionStunTimeInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVMotionStunTimeInfo>()?),
            StructType::VirtualMachineVendorDeviceGroupInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVendorDeviceGroupInfo>()?),
            StructType::VirtualMachineVgpuDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVgpuDeviceInfo>()?),
            StructType::VirtualMachineVgpuProfileInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVgpuProfileInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineTargetInfo => Ok(from.as_any_box().downcast::<VirtualMachineTargetInfo>()?),
            StructType::VirtualMachineCdromInfo => Ok(from.as_any_box().downcast::<VirtualMachineCdromInfo>()?),
            StructType::VirtualMachineDatastoreInfo => Ok(from.as_any_box().downcast::<VirtualMachineDatastoreInfo>()?),
            StructType::VirtualMachineDiskDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineDiskDeviceInfo>()?),
            StructType::VirtualMachineIdeDiskDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineIdeDiskDeviceInfo>()?),
            StructType::VirtualMachineScsiDiskDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineScsiDiskDeviceInfo>()?),
            StructType::VirtualMachineDynamicPassthroughInfo => Ok(from.as_any_box().downcast::<VirtualMachineDynamicPassthroughInfo>()?),
            StructType::VirtualMachineFloppyInfo => Ok(from.as_any_box().downcast::<VirtualMachineFloppyInfo>()?),
            StructType::VirtualMachineNetworkInfo => Ok(from.as_any_box().downcast::<VirtualMachineNetworkInfo>()?),
            StructType::OpaqueNetworkTargetInfo => Ok(from.as_any_box().downcast::<OpaqueNetworkTargetInfo>()?),
            StructType::VirtualMachineParallelInfo => Ok(from.as_any_box().downcast::<VirtualMachineParallelInfo>()?),
            StructType::VirtualMachinePciPassthroughInfo => Ok(from.as_any_box().downcast::<VirtualMachinePciPassthroughInfo>()?),
            StructType::VirtualMachineSriovInfo => Ok(from.as_any_box().downcast::<VirtualMachineSriovInfo>()?),
            StructType::VirtualMachinePciSharedGpuPassthroughInfo => Ok(from.as_any_box().downcast::<VirtualMachinePciSharedGpuPassthroughInfo>()?),
            StructType::VirtualMachinePrecisionClockInfo => Ok(from.as_any_box().downcast::<VirtualMachinePrecisionClockInfo>()?),
            StructType::VirtualMachineScsiPassthroughInfo => Ok(from.as_any_box().downcast::<VirtualMachineScsiPassthroughInfo>()?),
            StructType::VirtualMachineSerialInfo => Ok(from.as_any_box().downcast::<VirtualMachineSerialInfo>()?),
            StructType::VirtualMachineSgxTargetInfo => Ok(from.as_any_box().downcast::<VirtualMachineSgxTargetInfo>()?),
            StructType::VirtualMachineSoundInfo => Ok(from.as_any_box().downcast::<VirtualMachineSoundInfo>()?),
            StructType::VirtualMachineUsbInfo => Ok(from.as_any_box().downcast::<VirtualMachineUsbInfo>()?),
            StructType::VirtualMachineVFlashModuleInfo => Ok(from.as_any_box().downcast::<VirtualMachineVFlashModuleInfo>()?),
            StructType::VirtualMachineVMotionStunTimeInfo => Ok(from.as_any_box().downcast::<VirtualMachineVMotionStunTimeInfo>()?),
            StructType::VirtualMachineVendorDeviceGroupInfo => Ok(from.as_any_box().downcast::<VirtualMachineVendorDeviceGroupInfo>()?),
            StructType::VirtualMachineVgpuDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineVgpuDeviceInfo>()?),
            StructType::VirtualMachineVgpuProfileInfo => Ok(from.as_any_box().downcast::<VirtualMachineVgpuProfileInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
