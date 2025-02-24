use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A GuestRegistryKeyFault exception is thrown when an operation fails
/// because of some errors in accessing/modifying a guest registry key.
pub trait GuestRegistryKeyFaultTrait : super::guest_registry_fault_trait::GuestRegistryFaultTrait {
    /// The full path to the windows registry key.
    fn get_key_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn GuestRegistryKeyFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn GuestRegistryKeyFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(GuestRegistryKeyFaultVisitor)
            }
        }

struct GuestRegistryKeyFaultVisitor;

impl<'de> de::Visitor<'de> for GuestRegistryKeyFaultVisitor {
    type Value = Box<dyn GuestRegistryKeyFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GuestRegistryKeyFaultTrait JSON object with a _typeName field")
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

impl GuestRegistryKeyFaultTrait for GuestRegistryKeyFault {
    fn get_key_name(&self) -> &str { &self.key_name }
}
impl GuestRegistryKeyFaultTrait for GuestRegistryKeyAlreadyExists {
    fn get_key_name(&self) -> &str { &self.key_name }
}
impl GuestRegistryKeyFaultTrait for GuestRegistryKeyHasSubkeys {
    fn get_key_name(&self) -> &str { &self.key_name }
}
impl GuestRegistryKeyFaultTrait for GuestRegistryKeyInvalid {
    fn get_key_name(&self) -> &str { &self.key_name }
}
impl GuestRegistryKeyFaultTrait for GuestRegistryKeyParentVolatile {
    fn get_key_name(&self) -> &str { &self.key_name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn GuestRegistryKeyFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestRegistryKeyFault => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyFault>()?),
            StructType::GuestRegistryKeyAlreadyExists => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyAlreadyExists>()?),
            StructType::GuestRegistryKeyHasSubkeys => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyHasSubkeys>()?),
            StructType::GuestRegistryKeyInvalid => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyInvalid>()?),
            StructType::GuestRegistryKeyParentVolatile => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyParentVolatile>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestRegistryKeyFault => Ok(from.as_any_box().downcast::<GuestRegistryKeyFault>()?),
            StructType::GuestRegistryKeyAlreadyExists => Ok(from.as_any_box().downcast::<GuestRegistryKeyAlreadyExists>()?),
            StructType::GuestRegistryKeyHasSubkeys => Ok(from.as_any_box().downcast::<GuestRegistryKeyHasSubkeys>()?),
            StructType::GuestRegistryKeyInvalid => Ok(from.as_any_box().downcast::<GuestRegistryKeyInvalid>()?),
            StructType::GuestRegistryKeyParentVolatile => Ok(from.as_any_box().downcast::<GuestRegistryKeyParentVolatile>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
