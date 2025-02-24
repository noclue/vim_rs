use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Thrown when virtual machine creation or configuration fails.
/// 
/// This is
/// a base type for all virtual machine configuration errors.
pub trait InvalidVmConfigTrait : super::vm_config_fault_trait::VmConfigFaultTrait {
    /// Path of the property in configSpec that has an invalid value.
    fn get_property(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn InvalidVmConfigTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InvalidVmConfigTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InvalidVmConfigVisitor)
            }
        }

struct InvalidVmConfigVisitor;

impl<'de> de::Visitor<'de> for InvalidVmConfigVisitor {
    type Value = Box<dyn InvalidVmConfigTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InvalidVmConfigTrait JSON object with a _typeName field")
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

impl InvalidVmConfigTrait for InvalidVmConfig {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for InvalidDeviceSpec {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for DeviceHotPlugNotSupported {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for DeviceNotFound {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for DeviceUnsupportedForVmPlatform {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for DeviceUnsupportedForVmVersion {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for DisallowedDiskModeChange {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for InvalidController {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for InvalidDeviceBacking {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for InvalidDeviceOperation {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for MissingController {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for SwapPlacementOverrideNotSupported {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for TooManyDevices {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for UnsupportedGuest {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl InvalidVmConfigTrait for VmWwnConflict {
    fn get_property(&self) -> &Option<String> { &self.property }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InvalidVmConfigTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidVmConfig => Some(from.as_any_ref().downcast_ref::<InvalidVmConfig>()?),
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
            StructType::SwapPlacementOverrideNotSupported => Some(from.as_any_ref().downcast_ref::<SwapPlacementOverrideNotSupported>()?),
            StructType::TooManyDevices => Some(from.as_any_ref().downcast_ref::<TooManyDevices>()?),
            StructType::UnsupportedGuest => Some(from.as_any_ref().downcast_ref::<UnsupportedGuest>()?),
            StructType::VmWwnConflict => Some(from.as_any_ref().downcast_ref::<VmWwnConflict>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidVmConfig => Ok(from.as_any_box().downcast::<InvalidVmConfig>()?),
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
            StructType::SwapPlacementOverrideNotSupported => Ok(from.as_any_box().downcast::<SwapPlacementOverrideNotSupported>()?),
            StructType::TooManyDevices => Ok(from.as_any_box().downcast::<TooManyDevices>()?),
            StructType::UnsupportedGuest => Ok(from.as_any_box().downcast::<UnsupportedGuest>()?),
            StructType::VmWwnConflict => Ok(from.as_any_box().downcast::<VmWwnConflict>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
