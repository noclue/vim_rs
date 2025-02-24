use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// <code>*VirtualDeviceBackingInfo*</code> is a base data object type
/// for information about the backing of a device in a virtual machine.
/// 
/// This base type does not define any properties. It is used as a namespace
/// for general-purpose subtypes. Specific devices are represented by subtypes
/// which define properties for device-specific backing information.
pub trait VirtualDeviceBackingInfoTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn VirtualDeviceBackingInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceBackingInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceBackingInfoVisitor)
            }
        }

struct VirtualDeviceBackingInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceBackingInfoVisitor {
    type Value = Box<dyn VirtualDeviceBackingInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceBackingInfoTrait JSON object with a _typeName field")
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

impl VirtualDeviceBackingInfoTrait for VirtualDeviceBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDeviceDeviceBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualCdromAtapiBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualCdromPassthroughBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDiskRawDiskVer2BackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDiskPartitionedRawDiskVer2BackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualEthernetCardLegacyNetworkBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualEthernetCardNetworkBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualFloppyDeviceBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualPciPassthroughDeviceBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualPciPassthroughDynamicBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualParallelPortDeviceBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualPointingDeviceDeviceBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualScsiPassthroughDeviceBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualSerialPortDeviceBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualSoundCardDeviceBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualUsbRemoteHostBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualUsbusbBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDeviceFileBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualCdromIsoBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDiskFlatVer1BackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDiskFlatVer2BackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDiskLocalPMemBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDiskRawDiskMappingVer1BackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDiskSeSparseBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDiskSparseVer1BackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDiskSparseVer2BackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualFloppyImageBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualNvdimmBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualParallelPortFileBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualSerialPortFileBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDevicePipeBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualSerialPortPipeBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDeviceRemoteDeviceBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualCdromRemoteAtapiBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualCdromRemotePassthroughBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualFloppyRemoteDeviceBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualUsbRemoteClientBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualDeviceUriBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualSerialPortUriBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualEthernetCardDistributedVirtualPortBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualEthernetCardOpaqueNetworkBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualPciPassthroughDvxBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualPciPassthroughPluginBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualPciPassthroughVmiopBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualPrecisionClockSystemClockBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualSerialPortThinPrintBackingInfo {
}
impl VirtualDeviceBackingInfoTrait for VirtualSriovEthernetCardSriovBackingInfo {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceBackingInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceBackingInfo>()?),
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
            StructType::VirtualDeviceFileBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceFileBackingInfo>()?),
            StructType::VirtualCdromIsoBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromIsoBackingInfo>()?),
            StructType::VirtualDiskFlatVer1BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer1BackingInfo>()?),
            StructType::VirtualDiskFlatVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer2BackingInfo>()?),
            StructType::VirtualDiskLocalPMemBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskLocalPMemBackingInfo>()?),
            StructType::VirtualDiskRawDiskMappingVer1BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskRawDiskMappingVer1BackingInfo>()?),
            StructType::VirtualDiskSeSparseBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskSeSparseBackingInfo>()?),
            StructType::VirtualDiskSparseVer1BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer1BackingInfo>()?),
            StructType::VirtualDiskSparseVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer2BackingInfo>()?),
            StructType::VirtualFloppyImageBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualFloppyImageBackingInfo>()?),
            StructType::VirtualNvdimmBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualNvdimmBackingInfo>()?),
            StructType::VirtualParallelPortFileBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortFileBackingInfo>()?),
            StructType::VirtualSerialPortFileBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortFileBackingInfo>()?),
            StructType::VirtualDevicePipeBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDevicePipeBackingInfo>()?),
            StructType::VirtualSerialPortPipeBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortPipeBackingInfo>()?),
            StructType::VirtualDeviceRemoteDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceRemoteDeviceBackingInfo>()?),
            StructType::VirtualCdromRemoteAtapiBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromRemoteAtapiBackingInfo>()?),
            StructType::VirtualCdromRemotePassthroughBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromRemotePassthroughBackingInfo>()?),
            StructType::VirtualFloppyRemoteDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualFloppyRemoteDeviceBackingInfo>()?),
            StructType::VirtualUsbRemoteClientBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualUsbRemoteClientBackingInfo>()?),
            StructType::VirtualDeviceUriBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceUriBackingInfo>()?),
            StructType::VirtualSerialPortUriBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortUriBackingInfo>()?),
            StructType::VirtualEthernetCardDistributedVirtualPortBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardDistributedVirtualPortBackingInfo>()?),
            StructType::VirtualEthernetCardOpaqueNetworkBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardOpaqueNetworkBackingInfo>()?),
            StructType::VirtualPciPassthroughDvxBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDvxBackingInfo>()?),
            StructType::VirtualPciPassthroughPluginBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughPluginBackingInfo>()?),
            StructType::VirtualPciPassthroughVmiopBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughVmiopBackingInfo>()?),
            StructType::VirtualPrecisionClockSystemClockBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPrecisionClockSystemClockBackingInfo>()?),
            StructType::VirtualSerialPortThinPrintBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortThinPrintBackingInfo>()?),
            StructType::VirtualSriovEthernetCardSriovBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSriovEthernetCardSriovBackingInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceBackingInfo>()?),
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
            StructType::VirtualDeviceFileBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceFileBackingInfo>()?),
            StructType::VirtualCdromIsoBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromIsoBackingInfo>()?),
            StructType::VirtualDiskFlatVer1BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer1BackingInfo>()?),
            StructType::VirtualDiskFlatVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer2BackingInfo>()?),
            StructType::VirtualDiskLocalPMemBackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskLocalPMemBackingInfo>()?),
            StructType::VirtualDiskRawDiskMappingVer1BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskRawDiskMappingVer1BackingInfo>()?),
            StructType::VirtualDiskSeSparseBackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskSeSparseBackingInfo>()?),
            StructType::VirtualDiskSparseVer1BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer1BackingInfo>()?),
            StructType::VirtualDiskSparseVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer2BackingInfo>()?),
            StructType::VirtualFloppyImageBackingInfo => Ok(from.as_any_box().downcast::<VirtualFloppyImageBackingInfo>()?),
            StructType::VirtualNvdimmBackingInfo => Ok(from.as_any_box().downcast::<VirtualNvdimmBackingInfo>()?),
            StructType::VirtualParallelPortFileBackingInfo => Ok(from.as_any_box().downcast::<VirtualParallelPortFileBackingInfo>()?),
            StructType::VirtualSerialPortFileBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortFileBackingInfo>()?),
            StructType::VirtualDevicePipeBackingInfo => Ok(from.as_any_box().downcast::<VirtualDevicePipeBackingInfo>()?),
            StructType::VirtualSerialPortPipeBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortPipeBackingInfo>()?),
            StructType::VirtualDeviceRemoteDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceRemoteDeviceBackingInfo>()?),
            StructType::VirtualCdromRemoteAtapiBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromRemoteAtapiBackingInfo>()?),
            StructType::VirtualCdromRemotePassthroughBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromRemotePassthroughBackingInfo>()?),
            StructType::VirtualFloppyRemoteDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualFloppyRemoteDeviceBackingInfo>()?),
            StructType::VirtualUsbRemoteClientBackingInfo => Ok(from.as_any_box().downcast::<VirtualUsbRemoteClientBackingInfo>()?),
            StructType::VirtualDeviceUriBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceUriBackingInfo>()?),
            StructType::VirtualSerialPortUriBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortUriBackingInfo>()?),
            StructType::VirtualEthernetCardDistributedVirtualPortBackingInfo => Ok(from.as_any_box().downcast::<VirtualEthernetCardDistributedVirtualPortBackingInfo>()?),
            StructType::VirtualEthernetCardOpaqueNetworkBackingInfo => Ok(from.as_any_box().downcast::<VirtualEthernetCardOpaqueNetworkBackingInfo>()?),
            StructType::VirtualPciPassthroughDvxBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDvxBackingInfo>()?),
            StructType::VirtualPciPassthroughPluginBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughPluginBackingInfo>()?),
            StructType::VirtualPciPassthroughVmiopBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughVmiopBackingInfo>()?),
            StructType::VirtualPrecisionClockSystemClockBackingInfo => Ok(from.as_any_box().downcast::<VirtualPrecisionClockSystemClockBackingInfo>()?),
            StructType::VirtualSerialPortThinPrintBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortThinPrintBackingInfo>()?),
            StructType::VirtualSriovEthernetCardSriovBackingInfo => Ok(from.as_any_box().downcast::<VirtualSriovEthernetCardSriovBackingInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
