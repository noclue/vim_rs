use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The <code>*VirtualDeviceDeviceBackingInfo*</code> data object type
/// defines information about a host device or resource that backs a device
/// in a virtual machine.
pub trait VirtualDeviceDeviceBackingInfoTrait : super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait {
    /// The name of the device on the host system.
    fn get_device_name(&self) -> &str;
    /// Indicates whether the device should be auto detected
    /// instead of directly specified.
    /// 
    /// If this value is set to TRUE,
    /// deviceName is ignored.
    fn get_use_auto_detect(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn VirtualDeviceDeviceBackingInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceDeviceBackingInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceDeviceBackingInfoVisitor)
            }
        }

struct VirtualDeviceDeviceBackingInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceDeviceBackingInfoVisitor {
    type Value = Box<dyn VirtualDeviceDeviceBackingInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceDeviceBackingInfoTrait JSON object with a _typeName field")
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

impl VirtualDeviceDeviceBackingInfoTrait for VirtualDeviceDeviceBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualCdromAtapiBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualCdromPassthroughBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualDiskRawDiskVer2BackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualDiskPartitionedRawDiskVer2BackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualEthernetCardLegacyNetworkBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualEthernetCardNetworkBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualFloppyDeviceBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualPciPassthroughDeviceBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualPciPassthroughDynamicBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualParallelPortDeviceBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualPointingDeviceDeviceBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualScsiPassthroughDeviceBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualSerialPortDeviceBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualSoundCardDeviceBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualUsbRemoteHostBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceDeviceBackingInfoTrait for VirtualUsbusbBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceDeviceBackingInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceDeviceBackingInfo>()?),
            StructType::VirtualCdromAtapiBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromAtapiBackingInfo>()?),
            StructType::VirtualCdromPassthroughBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromPassthroughBackingInfo>()?),
            StructType::VirtualDiskRawDiskVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskRawDiskVer2BackingInfo>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskPartitionedRawDiskVer2BackingInfo>()?),
            StructType::VirtualEthernetCardLegacyNetworkBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardLegacyNetworkBackingInfo>()?),
            StructType::VirtualEthernetCardNetworkBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardNetworkBackingInfo>()?),
            StructType::VirtualFloppyDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualFloppyDeviceBackingInfo>()?),
            StructType::VirtualPciPassthroughDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDeviceBackingInfo>()?),
            StructType::VirtualPciPassthroughDynamicBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDynamicBackingInfo>()?),
            StructType::VirtualParallelPortDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortDeviceBackingInfo>()?),
            StructType::VirtualPointingDeviceDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPointingDeviceDeviceBackingInfo>()?),
            StructType::VirtualScsiPassthroughDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualScsiPassthroughDeviceBackingInfo>()?),
            StructType::VirtualSerialPortDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortDeviceBackingInfo>()?),
            StructType::VirtualSoundCardDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSoundCardDeviceBackingInfo>()?),
            StructType::VirtualUsbRemoteHostBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualUsbRemoteHostBackingInfo>()?),
            StructType::VirtualUsbusbBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualUsbusbBackingInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceDeviceBackingInfo>()?),
            StructType::VirtualCdromAtapiBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromAtapiBackingInfo>()?),
            StructType::VirtualCdromPassthroughBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromPassthroughBackingInfo>()?),
            StructType::VirtualDiskRawDiskVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskRawDiskVer2BackingInfo>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskPartitionedRawDiskVer2BackingInfo>()?),
            StructType::VirtualEthernetCardLegacyNetworkBackingInfo => Ok(from.as_any_box().downcast::<VirtualEthernetCardLegacyNetworkBackingInfo>()?),
            StructType::VirtualEthernetCardNetworkBackingInfo => Ok(from.as_any_box().downcast::<VirtualEthernetCardNetworkBackingInfo>()?),
            StructType::VirtualFloppyDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualFloppyDeviceBackingInfo>()?),
            StructType::VirtualPciPassthroughDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDeviceBackingInfo>()?),
            StructType::VirtualPciPassthroughDynamicBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDynamicBackingInfo>()?),
            StructType::VirtualParallelPortDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualParallelPortDeviceBackingInfo>()?),
            StructType::VirtualPointingDeviceDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualPointingDeviceDeviceBackingInfo>()?),
            StructType::VirtualScsiPassthroughDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualScsiPassthroughDeviceBackingInfo>()?),
            StructType::VirtualSerialPortDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortDeviceBackingInfo>()?),
            StructType::VirtualSoundCardDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualSoundCardDeviceBackingInfo>()?),
            StructType::VirtualUsbRemoteHostBackingInfo => Ok(from.as_any_box().downcast::<VirtualUsbRemoteHostBackingInfo>()?),
            StructType::VirtualUsbusbBackingInfo => Ok(from.as_any_box().downcast::<VirtualUsbusbBackingInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
