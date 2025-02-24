use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// VirtualDeviceOption.RemoteDeviceBackingOption describes the options
/// for a remote device backing.
/// 
/// The primary difference
/// between a remote device backing and a local device backing is that
/// the VirtualCenter server cannot provide a list of remote host devices
/// available for this virtual device backing.
pub trait VirtualDeviceRemoteDeviceBackingOptionTrait : super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait {
    /// Flag to indicate whether the specific instance of this device can
    /// be auto-detected on the host instead of having to specify a
    /// particular physical device.
    fn get_auto_detect_available(&self) -> &BoolOption;
}
impl<'s> serde::Serialize for dyn VirtualDeviceRemoteDeviceBackingOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceRemoteDeviceBackingOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceRemoteDeviceBackingOptionVisitor)
            }
        }

struct VirtualDeviceRemoteDeviceBackingOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceRemoteDeviceBackingOptionVisitor {
    type Value = Box<dyn VirtualDeviceRemoteDeviceBackingOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceRemoteDeviceBackingOptionTrait JSON object with a _typeName field")
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

impl VirtualDeviceRemoteDeviceBackingOptionTrait for VirtualDeviceRemoteDeviceBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceRemoteDeviceBackingOptionTrait for VirtualCdromRemotePassthroughBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceRemoteDeviceBackingOptionTrait for VirtualFloppyRemoteDeviceBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl VirtualDeviceRemoteDeviceBackingOptionTrait for VirtualUsbRemoteClientBackingOption {
    fn get_auto_detect_available(&self) -> &BoolOption { &self.auto_detect_available }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceRemoteDeviceBackingOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceRemoteDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceRemoteDeviceBackingOption>()?),
            StructType::VirtualCdromRemotePassthroughBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromRemotePassthroughBackingOption>()?),
            StructType::VirtualFloppyRemoteDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualFloppyRemoteDeviceBackingOption>()?),
            StructType::VirtualUsbRemoteClientBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbRemoteClientBackingOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceRemoteDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceRemoteDeviceBackingOption>()?),
            StructType::VirtualCdromRemotePassthroughBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromRemotePassthroughBackingOption>()?),
            StructType::VirtualFloppyRemoteDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualFloppyRemoteDeviceBackingOption>()?),
            StructType::VirtualUsbRemoteClientBackingOption => Ok(from.as_any_box().downcast::<VirtualUsbRemoteClientBackingOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
