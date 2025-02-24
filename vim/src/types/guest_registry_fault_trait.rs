use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A GuestRegistryFault exception is thrown when an operation fails
/// because of some errors in accessing/modifying the guest registry.
pub trait GuestRegistryFaultTrait : super::guest_operations_fault_trait::GuestOperationsFaultTrait {
    /// The windows system error number from GetLastError().
    fn get_windows_system_error_code(&self) -> i64;
}
impl<'s> serde::Serialize for dyn GuestRegistryFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn GuestRegistryFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(GuestRegistryFaultVisitor)
            }
        }

struct GuestRegistryFaultVisitor;

impl<'de> de::Visitor<'de> for GuestRegistryFaultVisitor {
    type Value = Box<dyn GuestRegistryFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GuestRegistryFaultTrait JSON object with a _typeName field")
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

impl GuestRegistryFaultTrait for GuestRegistryFault {
    fn get_windows_system_error_code(&self) -> i64 { self.windows_system_error_code }
}
impl GuestRegistryFaultTrait for GuestRegistryKeyFault {
    fn get_windows_system_error_code(&self) -> i64 { self.windows_system_error_code }
}
impl GuestRegistryFaultTrait for GuestRegistryKeyAlreadyExists {
    fn get_windows_system_error_code(&self) -> i64 { self.windows_system_error_code }
}
impl GuestRegistryFaultTrait for GuestRegistryKeyHasSubkeys {
    fn get_windows_system_error_code(&self) -> i64 { self.windows_system_error_code }
}
impl GuestRegistryFaultTrait for GuestRegistryKeyInvalid {
    fn get_windows_system_error_code(&self) -> i64 { self.windows_system_error_code }
}
impl GuestRegistryFaultTrait for GuestRegistryKeyParentVolatile {
    fn get_windows_system_error_code(&self) -> i64 { self.windows_system_error_code }
}
impl GuestRegistryFaultTrait for GuestRegistryValueFault {
    fn get_windows_system_error_code(&self) -> i64 { self.windows_system_error_code }
}
impl GuestRegistryFaultTrait for GuestRegistryValueNotFound {
    fn get_windows_system_error_code(&self) -> i64 { self.windows_system_error_code }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn GuestRegistryFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestRegistryFault => Some(from.as_any_ref().downcast_ref::<GuestRegistryFault>()?),
            StructType::GuestRegistryKeyFault => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyFault>()?),
            StructType::GuestRegistryKeyAlreadyExists => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyAlreadyExists>()?),
            StructType::GuestRegistryKeyHasSubkeys => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyHasSubkeys>()?),
            StructType::GuestRegistryKeyInvalid => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyInvalid>()?),
            StructType::GuestRegistryKeyParentVolatile => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyParentVolatile>()?),
            StructType::GuestRegistryValueFault => Some(from.as_any_ref().downcast_ref::<GuestRegistryValueFault>()?),
            StructType::GuestRegistryValueNotFound => Some(from.as_any_ref().downcast_ref::<GuestRegistryValueNotFound>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestRegistryFault => Ok(from.as_any_box().downcast::<GuestRegistryFault>()?),
            StructType::GuestRegistryKeyFault => Ok(from.as_any_box().downcast::<GuestRegistryKeyFault>()?),
            StructType::GuestRegistryKeyAlreadyExists => Ok(from.as_any_box().downcast::<GuestRegistryKeyAlreadyExists>()?),
            StructType::GuestRegistryKeyHasSubkeys => Ok(from.as_any_box().downcast::<GuestRegistryKeyHasSubkeys>()?),
            StructType::GuestRegistryKeyInvalid => Ok(from.as_any_box().downcast::<GuestRegistryKeyInvalid>()?),
            StructType::GuestRegistryKeyParentVolatile => Ok(from.as_any_box().downcast::<GuestRegistryKeyParentVolatile>()?),
            StructType::GuestRegistryValueFault => Ok(from.as_any_box().downcast::<GuestRegistryValueFault>()?),
            StructType::GuestRegistryValueNotFound => Ok(from.as_any_box().downcast::<GuestRegistryValueNotFound>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
