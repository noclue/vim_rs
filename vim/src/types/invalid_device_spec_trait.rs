use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An InvalidDeviceSpec exception is thrown if
/// a virtual machine creation or configuration fails because a device
/// specification contains an invalid value.
pub trait InvalidDeviceSpecTrait : super::invalid_vm_config_trait::InvalidVmConfigTrait {
    /// Index of the device in the configuration specification that has the invalid value.
    fn get_device_index(&self) -> i32;
}
impl<'s> serde::Serialize for dyn InvalidDeviceSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InvalidDeviceSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InvalidDeviceSpecVisitor)
            }
        }

struct InvalidDeviceSpecVisitor;

impl<'de> de::Visitor<'de> for InvalidDeviceSpecVisitor {
    type Value = Box<dyn InvalidDeviceSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InvalidDeviceSpecTrait JSON object with a _typeName field")
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

impl InvalidDeviceSpecTrait for InvalidDeviceSpec {
    fn get_device_index(&self) -> i32 { self.device_index }
}
impl InvalidDeviceSpecTrait for DeviceHotPlugNotSupported {
    fn get_device_index(&self) -> i32 { self.device_index }
}
impl InvalidDeviceSpecTrait for DeviceNotFound {
    fn get_device_index(&self) -> i32 { self.device_index }
}
impl InvalidDeviceSpecTrait for DeviceUnsupportedForVmPlatform {
    fn get_device_index(&self) -> i32 { self.device_index }
}
impl InvalidDeviceSpecTrait for DeviceUnsupportedForVmVersion {
    fn get_device_index(&self) -> i32 { self.device_index }
}
impl InvalidDeviceSpecTrait for DisallowedDiskModeChange {
    fn get_device_index(&self) -> i32 { self.device_index }
}
impl InvalidDeviceSpecTrait for InvalidController {
    fn get_device_index(&self) -> i32 { self.device_index }
}
impl InvalidDeviceSpecTrait for InvalidDeviceBacking {
    fn get_device_index(&self) -> i32 { self.device_index }
}
impl InvalidDeviceSpecTrait for InvalidDeviceOperation {
    fn get_device_index(&self) -> i32 { self.device_index }
}
impl InvalidDeviceSpecTrait for MissingController {
    fn get_device_index(&self) -> i32 { self.device_index }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InvalidDeviceSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidDeviceSpec => Some(from.as_any_ref().downcast_ref::<InvalidDeviceSpec>()?),
            StructType::DeviceHotPlugNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceHotPlugNotSupported>()?),
            StructType::DeviceNotFound => Some(from.as_any_ref().downcast_ref::<DeviceNotFound>()?),
            StructType::DeviceUnsupportedForVmPlatform => Some(from.as_any_ref().downcast_ref::<DeviceUnsupportedForVmPlatform>()?),
            StructType::DeviceUnsupportedForVmVersion => Some(from.as_any_ref().downcast_ref::<DeviceUnsupportedForVmVersion>()?),
            StructType::DisallowedDiskModeChange => Some(from.as_any_ref().downcast_ref::<DisallowedDiskModeChange>()?),
            StructType::InvalidController => Some(from.as_any_ref().downcast_ref::<InvalidController>()?),
            StructType::InvalidDeviceBacking => Some(from.as_any_ref().downcast_ref::<InvalidDeviceBacking>()?),
            StructType::InvalidDeviceOperation => Some(from.as_any_ref().downcast_ref::<InvalidDeviceOperation>()?),
            StructType::MissingController => Some(from.as_any_ref().downcast_ref::<MissingController>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidDeviceSpec => Ok(from.as_any_box().downcast::<InvalidDeviceSpec>()?),
            StructType::DeviceHotPlugNotSupported => Ok(from.as_any_box().downcast::<DeviceHotPlugNotSupported>()?),
            StructType::DeviceNotFound => Ok(from.as_any_box().downcast::<DeviceNotFound>()?),
            StructType::DeviceUnsupportedForVmPlatform => Ok(from.as_any_box().downcast::<DeviceUnsupportedForVmPlatform>()?),
            StructType::DeviceUnsupportedForVmVersion => Ok(from.as_any_box().downcast::<DeviceUnsupportedForVmVersion>()?),
            StructType::DisallowedDiskModeChange => Ok(from.as_any_box().downcast::<DisallowedDiskModeChange>()?),
            StructType::InvalidController => Ok(from.as_any_box().downcast::<InvalidController>()?),
            StructType::InvalidDeviceBacking => Ok(from.as_any_box().downcast::<InvalidDeviceBacking>()?),
            StructType::InvalidDeviceOperation => Ok(from.as_any_box().downcast::<InvalidDeviceOperation>()?),
            StructType::MissingController => Ok(from.as_any_box().downcast::<MissingController>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
