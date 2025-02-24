use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A GuestRegistryValueFault exception is thrown when an operation fails
/// because of some errors in accessing/modifying a guest registry value.
pub trait GuestRegistryValueFaultTrait : super::guest_registry_fault_trait::GuestRegistryFaultTrait {
    /// The full path to the windows registry key containing the value.
    fn get_key_name(&self) -> &str;
    /// The name of the value.
    fn get_value_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn GuestRegistryValueFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn GuestRegistryValueFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(GuestRegistryValueFaultVisitor)
            }
        }

struct GuestRegistryValueFaultVisitor;

impl<'de> de::Visitor<'de> for GuestRegistryValueFaultVisitor {
    type Value = Box<dyn GuestRegistryValueFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GuestRegistryValueFaultTrait JSON object with a _typeName field")
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

impl GuestRegistryValueFaultTrait for GuestRegistryValueFault {
    fn get_key_name(&self) -> &str { &self.key_name }
    fn get_value_name(&self) -> &str { &self.value_name }
}
impl GuestRegistryValueFaultTrait for GuestRegistryValueNotFound {
    fn get_key_name(&self) -> &str { &self.key_name }
    fn get_value_name(&self) -> &str { &self.value_name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn GuestRegistryValueFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestRegistryValueFault => Some(from.as_any_ref().downcast_ref::<GuestRegistryValueFault>()?),
            StructType::GuestRegistryValueNotFound => Some(from.as_any_ref().downcast_ref::<GuestRegistryValueNotFound>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::GuestRegistryValueFault => Ok(from.as_any_box().downcast::<GuestRegistryValueFault>()?),
            StructType::GuestRegistryValueNotFound => Ok(from.as_any_box().downcast::<GuestRegistryValueNotFound>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
