use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// <code>*VirtualDeviceRemoteDeviceBackingInfo*</code> is a data object type
/// for information
/// about a remote device backing used by a device in a virtual machine.
/// 
/// The primary difference between a remote device backing and a
/// local device backing is that the VirtualCenter server cannot provide a list
/// of remote host devices available for this virtual device backing.
pub trait VirtualDeviceRemoteDeviceBackingInfoTrait : super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait {
    /// The name of the device on the remote system.
    fn get_device_name(&self) -> &str;
    /// Indicates whether the device should be auto detected
    /// instead of directly specified.
    /// 
    /// If this value is set to TRUE,
    /// <code>deviceName</code> is ignored.
    fn get_use_auto_detect(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn VirtualDeviceRemoteDeviceBackingInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceRemoteDeviceBackingInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceRemoteDeviceBackingInfoVisitor)
            }
        }

struct VirtualDeviceRemoteDeviceBackingInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceRemoteDeviceBackingInfoVisitor {
    type Value = Box<dyn VirtualDeviceRemoteDeviceBackingInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceRemoteDeviceBackingInfoTrait JSON object with a _typeName field")
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

impl VirtualDeviceRemoteDeviceBackingInfoTrait for VirtualDeviceRemoteDeviceBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceRemoteDeviceBackingInfoTrait for VirtualCdromRemoteAtapiBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceRemoteDeviceBackingInfoTrait for VirtualCdromRemotePassthroughBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceRemoteDeviceBackingInfoTrait for VirtualFloppyRemoteDeviceBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl VirtualDeviceRemoteDeviceBackingInfoTrait for VirtualUsbRemoteClientBackingInfo {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_use_auto_detect(&self) -> Option<bool> { self.use_auto_detect }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceRemoteDeviceBackingInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceRemoteDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceRemoteDeviceBackingInfo>()?),
            StructType::VirtualCdromRemoteAtapiBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromRemoteAtapiBackingInfo>()?),
            StructType::VirtualCdromRemotePassthroughBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromRemotePassthroughBackingInfo>()?),
            StructType::VirtualFloppyRemoteDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualFloppyRemoteDeviceBackingInfo>()?),
            StructType::VirtualUsbRemoteClientBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualUsbRemoteClientBackingInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceRemoteDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceRemoteDeviceBackingInfo>()?),
            StructType::VirtualCdromRemoteAtapiBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromRemoteAtapiBackingInfo>()?),
            StructType::VirtualCdromRemotePassthroughBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromRemotePassthroughBackingInfo>()?),
            StructType::VirtualFloppyRemoteDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualFloppyRemoteDeviceBackingInfo>()?),
            StructType::VirtualUsbRemoteClientBackingInfo => Ok(from.as_any_box().downcast::<VirtualUsbRemoteClientBackingInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
