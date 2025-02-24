use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Superclass for all faults that can be thrown during the callback to an OVF
/// consumer.
/// 
/// The *MethodFault.faultCause* gives details about what went
/// wrong.
pub trait OvfConsumerCallbackFaultTrait : super::ovf_fault_trait::OvfFaultTrait {
    /// The OVF consumer's extension key.
    fn get_extension_key(&self) -> &str;
    /// The OVF consumer's extension name.
    fn get_extension_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn OvfConsumerCallbackFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfConsumerCallbackFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfConsumerCallbackFaultVisitor)
            }
        }

struct OvfConsumerCallbackFaultVisitor;

impl<'de> de::Visitor<'de> for OvfConsumerCallbackFaultVisitor {
    type Value = Box<dyn OvfConsumerCallbackFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfConsumerCallbackFaultTrait JSON object with a _typeName field")
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

impl OvfConsumerCallbackFaultTrait for OvfConsumerCallbackFault {
    fn get_extension_key(&self) -> &str { &self.extension_key }
    fn get_extension_name(&self) -> &str { &self.extension_name }
}
impl OvfConsumerCallbackFaultTrait for OvfConsumerCommunicationError {
    fn get_extension_key(&self) -> &str { &self.extension_key }
    fn get_extension_name(&self) -> &str { &self.extension_name }
}
impl OvfConsumerCallbackFaultTrait for OvfConsumerFault {
    fn get_extension_key(&self) -> &str { &self.extension_key }
    fn get_extension_name(&self) -> &str { &self.extension_name }
}
impl OvfConsumerCallbackFaultTrait for OvfConsumerInvalidSection {
    fn get_extension_key(&self) -> &str { &self.extension_key }
    fn get_extension_name(&self) -> &str { &self.extension_name }
}
impl OvfConsumerCallbackFaultTrait for OvfConsumerUndeclaredSection {
    fn get_extension_key(&self) -> &str { &self.extension_key }
    fn get_extension_name(&self) -> &str { &self.extension_name }
}
impl OvfConsumerCallbackFaultTrait for OvfConsumerUndefinedPrefix {
    fn get_extension_key(&self) -> &str { &self.extension_key }
    fn get_extension_name(&self) -> &str { &self.extension_name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfConsumerCallbackFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfConsumerCallbackFault => Some(from.as_any_ref().downcast_ref::<OvfConsumerCallbackFault>()?),
            StructType::OvfConsumerCommunicationError => Some(from.as_any_ref().downcast_ref::<OvfConsumerCommunicationError>()?),
            StructType::OvfConsumerFault => Some(from.as_any_ref().downcast_ref::<OvfConsumerFault>()?),
            StructType::OvfConsumerInvalidSection => Some(from.as_any_ref().downcast_ref::<OvfConsumerInvalidSection>()?),
            StructType::OvfConsumerUndeclaredSection => Some(from.as_any_ref().downcast_ref::<OvfConsumerUndeclaredSection>()?),
            StructType::OvfConsumerUndefinedPrefix => Some(from.as_any_ref().downcast_ref::<OvfConsumerUndefinedPrefix>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfConsumerCallbackFault => Ok(from.as_any_box().downcast::<OvfConsumerCallbackFault>()?),
            StructType::OvfConsumerCommunicationError => Ok(from.as_any_box().downcast::<OvfConsumerCommunicationError>()?),
            StructType::OvfConsumerFault => Ok(from.as_any_box().downcast::<OvfConsumerFault>()?),
            StructType::OvfConsumerInvalidSection => Ok(from.as_any_box().downcast::<OvfConsumerInvalidSection>()?),
            StructType::OvfConsumerUndeclaredSection => Ok(from.as_any_box().downcast::<OvfConsumerUndeclaredSection>()?),
            StructType::OvfConsumerUndefinedPrefix => Ok(from.as_any_box().downcast::<OvfConsumerUndefinedPrefix>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
