use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The common base type for all guest operations faults.
pub trait GuestOperationsFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn GuestOperationsFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn GuestOperationsFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(GuestOperationsFaultVisitor)
            }
        }

struct GuestOperationsFaultVisitor;

impl<'de> de::Visitor<'de> for GuestOperationsFaultVisitor {
    type Value = Box<dyn GuestOperationsFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GuestOperationsFaultTrait JSON object with a _typeName field")
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

impl GuestOperationsFaultTrait for GuestOperationsFault {
}
impl GuestOperationsFaultTrait for GuestAuthenticationChallenge {
}
impl GuestOperationsFaultTrait for GuestComponentsOutOfDate {
}
impl GuestOperationsFaultTrait for GuestMultipleMappings {
}
impl GuestOperationsFaultTrait for GuestOperationsUnavailable {
}
impl GuestOperationsFaultTrait for GuestPermissionDenied {
}
impl GuestOperationsFaultTrait for GuestProcessNotFound {
}
impl GuestOperationsFaultTrait for GuestRegistryFault {
}
impl GuestOperationsFaultTrait for GuestRegistryKeyFault {
}
impl GuestOperationsFaultTrait for GuestRegistryKeyAlreadyExists {
}
impl GuestOperationsFaultTrait for GuestRegistryKeyHasSubkeys {
}
impl GuestOperationsFaultTrait for GuestRegistryKeyInvalid {
}
impl GuestOperationsFaultTrait for GuestRegistryKeyParentVolatile {
}
impl GuestOperationsFaultTrait for GuestRegistryValueFault {
}
impl GuestOperationsFaultTrait for GuestRegistryValueNotFound {
}
impl GuestOperationsFaultTrait for InvalidGuestLogin {
}
impl GuestOperationsFaultTrait for OperationDisabledByGuest {
}
impl GuestOperationsFaultTrait for OperationNotSupportedByGuest {
}
impl GuestOperationsFaultTrait for TooManyGuestLogons {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn GuestOperationsFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestOperationsFault => Some(from.as_any_ref().downcast_ref::<GuestOperationsFault>()?),
            StructType::GuestAuthenticationChallenge => Some(from.as_any_ref().downcast_ref::<GuestAuthenticationChallenge>()?),
            StructType::GuestComponentsOutOfDate => Some(from.as_any_ref().downcast_ref::<GuestComponentsOutOfDate>()?),
            StructType::GuestMultipleMappings => Some(from.as_any_ref().downcast_ref::<GuestMultipleMappings>()?),
            StructType::GuestOperationsUnavailable => Some(from.as_any_ref().downcast_ref::<GuestOperationsUnavailable>()?),
            StructType::GuestPermissionDenied => Some(from.as_any_ref().downcast_ref::<GuestPermissionDenied>()?),
            StructType::GuestProcessNotFound => Some(from.as_any_ref().downcast_ref::<GuestProcessNotFound>()?),
            StructType::GuestRegistryFault => Some(from.as_any_ref().downcast_ref::<GuestRegistryFault>()?),
            StructType::GuestRegistryKeyFault => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyFault>()?),
            StructType::GuestRegistryKeyAlreadyExists => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyAlreadyExists>()?),
            StructType::GuestRegistryKeyHasSubkeys => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyHasSubkeys>()?),
            StructType::GuestRegistryKeyInvalid => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyInvalid>()?),
            StructType::GuestRegistryKeyParentVolatile => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyParentVolatile>()?),
            StructType::GuestRegistryValueFault => Some(from.as_any_ref().downcast_ref::<GuestRegistryValueFault>()?),
            StructType::GuestRegistryValueNotFound => Some(from.as_any_ref().downcast_ref::<GuestRegistryValueNotFound>()?),
            StructType::InvalidGuestLogin => Some(from.as_any_ref().downcast_ref::<InvalidGuestLogin>()?),
            StructType::OperationDisabledByGuest => Some(from.as_any_ref().downcast_ref::<OperationDisabledByGuest>()?),
            StructType::OperationNotSupportedByGuest => Some(from.as_any_ref().downcast_ref::<OperationNotSupportedByGuest>()?),
            StructType::TooManyGuestLogons => Some(from.as_any_ref().downcast_ref::<TooManyGuestLogons>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestOperationsFault => Ok(from.as_any_box().downcast::<GuestOperationsFault>()?),
            StructType::GuestAuthenticationChallenge => Ok(from.as_any_box().downcast::<GuestAuthenticationChallenge>()?),
            StructType::GuestComponentsOutOfDate => Ok(from.as_any_box().downcast::<GuestComponentsOutOfDate>()?),
            StructType::GuestMultipleMappings => Ok(from.as_any_box().downcast::<GuestMultipleMappings>()?),
            StructType::GuestOperationsUnavailable => Ok(from.as_any_box().downcast::<GuestOperationsUnavailable>()?),
            StructType::GuestPermissionDenied => Ok(from.as_any_box().downcast::<GuestPermissionDenied>()?),
            StructType::GuestProcessNotFound => Ok(from.as_any_box().downcast::<GuestProcessNotFound>()?),
            StructType::GuestRegistryFault => Ok(from.as_any_box().downcast::<GuestRegistryFault>()?),
            StructType::GuestRegistryKeyFault => Ok(from.as_any_box().downcast::<GuestRegistryKeyFault>()?),
            StructType::GuestRegistryKeyAlreadyExists => Ok(from.as_any_box().downcast::<GuestRegistryKeyAlreadyExists>()?),
            StructType::GuestRegistryKeyHasSubkeys => Ok(from.as_any_box().downcast::<GuestRegistryKeyHasSubkeys>()?),
            StructType::GuestRegistryKeyInvalid => Ok(from.as_any_box().downcast::<GuestRegistryKeyInvalid>()?),
            StructType::GuestRegistryKeyParentVolatile => Ok(from.as_any_box().downcast::<GuestRegistryKeyParentVolatile>()?),
            StructType::GuestRegistryValueFault => Ok(from.as_any_box().downcast::<GuestRegistryValueFault>()?),
            StructType::GuestRegistryValueNotFound => Ok(from.as_any_box().downcast::<GuestRegistryValueNotFound>()?),
            StructType::InvalidGuestLogin => Ok(from.as_any_box().downcast::<InvalidGuestLogin>()?),
            StructType::OperationDisabledByGuest => Ok(from.as_any_box().downcast::<OperationDisabledByGuest>()?),
            StructType::OperationNotSupportedByGuest => Ok(from.as_any_box().downcast::<OperationNotSupportedByGuest>()?),
            StructType::TooManyGuestLogons => Ok(from.as_any_box().downcast::<TooManyGuestLogons>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
