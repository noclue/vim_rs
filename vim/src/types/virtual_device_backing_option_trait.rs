use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *VirtualDeviceBackingOption* data class
/// defines options for device-specific virtual backing objects.
pub trait VirtualDeviceBackingOptionTrait : super::data_object_trait::DataObjectTrait {
    /// The name of the class the client should use to instantiate backing
    /// for the virtual device.
    fn get_type(&self) -> &str;
}
impl<'s> serde::Serialize for dyn VirtualDeviceBackingOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceBackingOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceBackingOptionVisitor)
            }
        }

struct VirtualDeviceBackingOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceBackingOptionVisitor {
    type Value = Box<dyn VirtualDeviceBackingOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceBackingOptionTrait JSON object with a _typeName field")
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

impl VirtualDeviceBackingOptionTrait for VirtualDeviceBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDeviceDeviceBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualCdromAtapiBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualCdromPassthroughBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualCdromRemoteAtapiBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDiskRawDiskMappingVer1BackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDiskRawDiskVer2BackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDiskPartitionedRawDiskVer2BackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualEthernetCardLegacyNetworkBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualEthernetCardNetworkBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualFloppyDeviceBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualPciPassthroughDeviceBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualPciPassthroughDynamicBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualParallelPortDeviceBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualPointingDeviceBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualScsiPassthroughDeviceBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualSerialPortDeviceBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualSoundCardDeviceBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualUsbRemoteHostBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualUsbusbBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDeviceFileBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualCdromIsoBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDiskFlatVer1BackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDiskFlatVer2BackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDiskLocalPMemBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDiskSeSparseBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDiskSparseVer1BackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDiskSparseVer2BackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualFloppyImageBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualParallelPortFileBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualSerialPortFileBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDevicePipeBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualSerialPortPipeBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDeviceRemoteDeviceBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualCdromRemotePassthroughBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualFloppyRemoteDeviceBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualUsbRemoteClientBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualDeviceUriBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualSerialPortUriBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualEthernetCardDvPortBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualEthernetCardOpaqueNetworkBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualPciPassthroughDvxBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualPciPassthroughPluginBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualPciPassthroughVmiopBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualPrecisionClockSystemClockBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualSerialPortThinPrintBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl VirtualDeviceBackingOptionTrait for VirtualSriovEthernetCardSriovBackingOption {
    fn get_type(&self) -> &str { &self.r#type }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceBackingOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceBackingOption>()?),
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
            StructType::VirtualDeviceFileBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceFileBackingOption>()?),
            StructType::VirtualCdromIsoBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromIsoBackingOption>()?),
            StructType::VirtualDiskFlatVer1BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer1BackingOption>()?),
            StructType::VirtualDiskFlatVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer2BackingOption>()?),
            StructType::VirtualDiskLocalPMemBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskLocalPMemBackingOption>()?),
            StructType::VirtualDiskSeSparseBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskSeSparseBackingOption>()?),
            StructType::VirtualDiskSparseVer1BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer1BackingOption>()?),
            StructType::VirtualDiskSparseVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer2BackingOption>()?),
            StructType::VirtualFloppyImageBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualFloppyImageBackingOption>()?),
            StructType::VirtualParallelPortFileBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortFileBackingOption>()?),
            StructType::VirtualSerialPortFileBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortFileBackingOption>()?),
            StructType::VirtualDevicePipeBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDevicePipeBackingOption>()?),
            StructType::VirtualSerialPortPipeBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortPipeBackingOption>()?),
            StructType::VirtualDeviceRemoteDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceRemoteDeviceBackingOption>()?),
            StructType::VirtualCdromRemotePassthroughBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromRemotePassthroughBackingOption>()?),
            StructType::VirtualFloppyRemoteDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualFloppyRemoteDeviceBackingOption>()?),
            StructType::VirtualUsbRemoteClientBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbRemoteClientBackingOption>()?),
            StructType::VirtualDeviceUriBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceUriBackingOption>()?),
            StructType::VirtualSerialPortUriBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortUriBackingOption>()?),
            StructType::VirtualEthernetCardDvPortBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardDvPortBackingOption>()?),
            StructType::VirtualEthernetCardOpaqueNetworkBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardOpaqueNetworkBackingOption>()?),
            StructType::VirtualPciPassthroughDvxBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDvxBackingOption>()?),
            StructType::VirtualPciPassthroughPluginBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughPluginBackingOption>()?),
            StructType::VirtualPciPassthroughVmiopBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughVmiopBackingOption>()?),
            StructType::VirtualPrecisionClockSystemClockBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPrecisionClockSystemClockBackingOption>()?),
            StructType::VirtualSerialPortThinPrintBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortThinPrintBackingOption>()?),
            StructType::VirtualSriovEthernetCardSriovBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSriovEthernetCardSriovBackingOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceBackingOption>()?),
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
            StructType::VirtualDeviceFileBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceFileBackingOption>()?),
            StructType::VirtualCdromIsoBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromIsoBackingOption>()?),
            StructType::VirtualDiskFlatVer1BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer1BackingOption>()?),
            StructType::VirtualDiskFlatVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer2BackingOption>()?),
            StructType::VirtualDiskLocalPMemBackingOption => Ok(from.as_any_box().downcast::<VirtualDiskLocalPMemBackingOption>()?),
            StructType::VirtualDiskSeSparseBackingOption => Ok(from.as_any_box().downcast::<VirtualDiskSeSparseBackingOption>()?),
            StructType::VirtualDiskSparseVer1BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer1BackingOption>()?),
            StructType::VirtualDiskSparseVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer2BackingOption>()?),
            StructType::VirtualFloppyImageBackingOption => Ok(from.as_any_box().downcast::<VirtualFloppyImageBackingOption>()?),
            StructType::VirtualParallelPortFileBackingOption => Ok(from.as_any_box().downcast::<VirtualParallelPortFileBackingOption>()?),
            StructType::VirtualSerialPortFileBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortFileBackingOption>()?),
            StructType::VirtualDevicePipeBackingOption => Ok(from.as_any_box().downcast::<VirtualDevicePipeBackingOption>()?),
            StructType::VirtualSerialPortPipeBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortPipeBackingOption>()?),
            StructType::VirtualDeviceRemoteDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceRemoteDeviceBackingOption>()?),
            StructType::VirtualCdromRemotePassthroughBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromRemotePassthroughBackingOption>()?),
            StructType::VirtualFloppyRemoteDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualFloppyRemoteDeviceBackingOption>()?),
            StructType::VirtualUsbRemoteClientBackingOption => Ok(from.as_any_box().downcast::<VirtualUsbRemoteClientBackingOption>()?),
            StructType::VirtualDeviceUriBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceUriBackingOption>()?),
            StructType::VirtualSerialPortUriBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortUriBackingOption>()?),
            StructType::VirtualEthernetCardDvPortBackingOption => Ok(from.as_any_box().downcast::<VirtualEthernetCardDvPortBackingOption>()?),
            StructType::VirtualEthernetCardOpaqueNetworkBackingOption => Ok(from.as_any_box().downcast::<VirtualEthernetCardOpaqueNetworkBackingOption>()?),
            StructType::VirtualPciPassthroughDvxBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDvxBackingOption>()?),
            StructType::VirtualPciPassthroughPluginBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughPluginBackingOption>()?),
            StructType::VirtualPciPassthroughVmiopBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughVmiopBackingOption>()?),
            StructType::VirtualPrecisionClockSystemClockBackingOption => Ok(from.as_any_box().downcast::<VirtualPrecisionClockSystemClockBackingOption>()?),
            StructType::VirtualSerialPortThinPrintBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortThinPrintBackingOption>()?),
            StructType::VirtualSriovEthernetCardSriovBackingOption => Ok(from.as_any_box().downcast::<VirtualSriovEthernetCardSriovBackingOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
