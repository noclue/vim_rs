use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The DeviceBackingOption data class contains device-specific backing options.
pub trait VirtualDeviceDeviceBackingOptionTrait : super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait {
    /// Flag to indicate whether the specific instance of this device can
    /// be auto-detected on the host instead of having to specify a
    /// particular physical device.
    fn get_auto_detect_available(&self) -> &BoolOption;
}
impl<'s> serde::Serialize for dyn VirtualDeviceDeviceBackingOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceDeviceBackingOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceDeviceBackingOptionVisitor)
            }
        }

struct VirtualDeviceDeviceBackingOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceDeviceBackingOptionVisitor {
    type Value = Box<dyn VirtualDeviceDeviceBackingOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceDeviceBackingOptionTrait JSON object with a _typeName field")
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

impl VirtualDeviceDeviceBackingOptionTrait for VirtualDeviceDeviceBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualCdromAtapiBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualCdromPassthroughBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualCdromRemoteAtapiBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualDiskRawDiskMappingVer1BackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualDiskRawDiskVer2BackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualDiskPartitionedRawDiskVer2BackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualEthernetCardLegacyNetworkBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualEthernetCardNetworkBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualFloppyDeviceBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualPciPassthroughDeviceBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualPciPassthroughDynamicBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualParallelPortDeviceBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualPointingDeviceBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualScsiPassthroughDeviceBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualSerialPortDeviceBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualSoundCardDeviceBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualUsbRemoteHostBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceDeviceBackingOptionTrait for VirtualUsbusbBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceDeviceBackingOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceDeviceBackingOption>()?),
            StructType::VirtualCdromAtapiBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromAtapiBackingOption>()?),
            StructType::VirtualCdromPassthroughBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromPassthroughBackingOption>()?),
            StructType::VirtualCdromRemoteAtapiBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromRemoteAtapiBackingOption>()?),
            StructType::VirtualDiskRawDiskMappingVer1BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskRawDiskMappingVer1BackingOption>()?),
            StructType::VirtualDiskRawDiskVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskRawDiskVer2BackingOption>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskPartitionedRawDiskVer2BackingOption>()?),
            StructType::VirtualEthernetCardLegacyNetworkBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardLegacyNetworkBackingOption>()?),
            StructType::VirtualEthernetCardNetworkBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardNetworkBackingOption>()?),
            StructType::VirtualFloppyDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualFloppyDeviceBackingOption>()?),
            StructType::VirtualPciPassthroughDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDeviceBackingOption>()?),
            StructType::VirtualPciPassthroughDynamicBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDynamicBackingOption>()?),
            StructType::VirtualParallelPortDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortDeviceBackingOption>()?),
            StructType::VirtualPointingDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPointingDeviceBackingOption>()?),
            StructType::VirtualScsiPassthroughDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualScsiPassthroughDeviceBackingOption>()?),
            StructType::VirtualSerialPortDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortDeviceBackingOption>()?),
            StructType::VirtualSoundCardDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSoundCardDeviceBackingOption>()?),
            StructType::VirtualUsbRemoteHostBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbRemoteHostBackingOption>()?),
            StructType::VirtualUsbusbBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbusbBackingOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceDeviceBackingOption>()?),
            StructType::VirtualCdromAtapiBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromAtapiBackingOption>()?),
            StructType::VirtualCdromPassthroughBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromPassthroughBackingOption>()?),
            StructType::VirtualCdromRemoteAtapiBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromRemoteAtapiBackingOption>()?),
            StructType::VirtualDiskRawDiskMappingVer1BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskRawDiskMappingVer1BackingOption>()?),
            StructType::VirtualDiskRawDiskVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskRawDiskVer2BackingOption>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskPartitionedRawDiskVer2BackingOption>()?),
            StructType::VirtualEthernetCardLegacyNetworkBackingOption => Ok(from.as_any_box().downcast::<VirtualEthernetCardLegacyNetworkBackingOption>()?),
            StructType::VirtualEthernetCardNetworkBackingOption => Ok(from.as_any_box().downcast::<VirtualEthernetCardNetworkBackingOption>()?),
            StructType::VirtualFloppyDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualFloppyDeviceBackingOption>()?),
            StructType::VirtualPciPassthroughDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDeviceBackingOption>()?),
            StructType::VirtualPciPassthroughDynamicBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDynamicBackingOption>()?),
            StructType::VirtualParallelPortDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualParallelPortDeviceBackingOption>()?),
            StructType::VirtualPointingDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualPointingDeviceBackingOption>()?),
            StructType::VirtualScsiPassthroughDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualScsiPassthroughDeviceBackingOption>()?),
            StructType::VirtualSerialPortDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortDeviceBackingOption>()?),
            StructType::VirtualSoundCardDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualSoundCardDeviceBackingOption>()?),
            StructType::VirtualUsbRemoteHostBackingOption => Ok(from.as_any_box().downcast::<VirtualUsbRemoteHostBackingOption>()?),
            StructType::VirtualUsbusbBackingOption => Ok(from.as_any_box().downcast::<VirtualUsbusbBackingOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
