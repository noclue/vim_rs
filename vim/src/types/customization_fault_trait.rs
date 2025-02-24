use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base for exceptions that can be thrown from the customizer.
pub trait CustomizationFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn CustomizationFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CustomizationFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CustomizationFaultVisitor)
            }
        }

struct CustomizationFaultVisitor;

impl<'de> de::Visitor<'de> for CustomizationFaultVisitor {
    type Value = Box<dyn CustomizationFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CustomizationFaultTrait JSON object with a _typeName field")
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

impl CustomizationFaultTrait for CustomizationFault {
}
impl CustomizationFaultTrait for CannotDecryptPasswords {
}
impl CustomizationFaultTrait for CustomizationPending {
}
impl CustomizationFaultTrait for IpHostnameGeneratorError {
}
impl CustomizationFaultTrait for LinuxVolumeNotClean {
}
impl CustomizationFaultTrait for MissingLinuxCustResources {
}
impl CustomizationFaultTrait for MissingWindowsCustResources {
}
impl CustomizationFaultTrait for MountError {
}
impl CustomizationFaultTrait for NicSettingMismatch {
}
impl CustomizationFaultTrait for NoDisksToCustomize {
}
impl CustomizationFaultTrait for UncustomizableGuest {
}
impl CustomizationFaultTrait for UnexpectedCustomizationFault {
}
impl CustomizationFaultTrait for VolumeEditorError {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CustomizationFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationFault => Some(from.as_any_ref().downcast_ref::<CustomizationFault>()?),
            StructType::CannotDecryptPasswords => Some(from.as_any_ref().downcast_ref::<CannotDecryptPasswords>()?),
            StructType::CustomizationPending => Some(from.as_any_ref().downcast_ref::<CustomizationPending>()?),
            StructType::IpHostnameGeneratorError => Some(from.as_any_ref().downcast_ref::<IpHostnameGeneratorError>()?),
            StructType::LinuxVolumeNotClean => Some(from.as_any_ref().downcast_ref::<LinuxVolumeNotClean>()?),
            StructType::MissingLinuxCustResources => Some(from.as_any_ref().downcast_ref::<MissingLinuxCustResources>()?),
            StructType::MissingWindowsCustResources => Some(from.as_any_ref().downcast_ref::<MissingWindowsCustResources>()?),
            StructType::MountError => Some(from.as_any_ref().downcast_ref::<MountError>()?),
            StructType::NicSettingMismatch => Some(from.as_any_ref().downcast_ref::<NicSettingMismatch>()?),
            StructType::NoDisksToCustomize => Some(from.as_any_ref().downcast_ref::<NoDisksToCustomize>()?),
            StructType::UncustomizableGuest => Some(from.as_any_ref().downcast_ref::<UncustomizableGuest>()?),
            StructType::UnexpectedCustomizationFault => Some(from.as_any_ref().downcast_ref::<UnexpectedCustomizationFault>()?),
            StructType::VolumeEditorError => Some(from.as_any_ref().downcast_ref::<VolumeEditorError>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationFault => Ok(from.as_any_box().downcast::<CustomizationFault>()?),
            StructType::CannotDecryptPasswords => Ok(from.as_any_box().downcast::<CannotDecryptPasswords>()?),
            StructType::CustomizationPending => Ok(from.as_any_box().downcast::<CustomizationPending>()?),
            StructType::IpHostnameGeneratorError => Ok(from.as_any_box().downcast::<IpHostnameGeneratorError>()?),
            StructType::LinuxVolumeNotClean => Ok(from.as_any_box().downcast::<LinuxVolumeNotClean>()?),
            StructType::MissingLinuxCustResources => Ok(from.as_any_box().downcast::<MissingLinuxCustResources>()?),
            StructType::MissingWindowsCustResources => Ok(from.as_any_box().downcast::<MissingWindowsCustResources>()?),
            StructType::MountError => Ok(from.as_any_box().downcast::<MountError>()?),
            StructType::NicSettingMismatch => Ok(from.as_any_box().downcast::<NicSettingMismatch>()?),
            StructType::NoDisksToCustomize => Ok(from.as_any_box().downcast::<NoDisksToCustomize>()?),
            StructType::UncustomizableGuest => Ok(from.as_any_box().downcast::<UncustomizableGuest>()?),
            StructType::UnexpectedCustomizationFault => Ok(from.as_any_box().downcast::<UnexpectedCustomizationFault>()?),
            StructType::VolumeEditorError => Ok(from.as_any_box().downcast::<VolumeEditorError>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
