use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base for customization events.
pub trait CustomizationEventTrait : super::vm_event_trait::VmEventTrait {
    /// The location of the in-guest customization log which will contain
    /// details of the customization operation.
    fn get_log_location(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn CustomizationEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CustomizationEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CustomizationEventVisitor)
            }
        }

struct CustomizationEventVisitor;

impl<'de> de::Visitor<'de> for CustomizationEventVisitor {
    type Value = Box<dyn CustomizationEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CustomizationEventTrait JSON object with a _typeName field")
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

impl CustomizationEventTrait for CustomizationEvent {
    fn get_log_location(&self) -> &Option<String> { &self.log_location }
}
impl CustomizationEventTrait for CustomizationFailed {
    fn get_log_location(&self) -> &Option<String> { &self.log_location }
}
impl CustomizationEventTrait for CustomizationLinuxIdentityFailed {
    fn get_log_location(&self) -> &Option<String> { &self.log_location }
}
impl CustomizationEventTrait for CustomizationNetworkSetupFailed {
    fn get_log_location(&self) -> &Option<String> { &self.log_location }
}
impl CustomizationEventTrait for CustomizationSysprepFailed {
    fn get_log_location(&self) -> &Option<String> { &self.log_location }
}
impl CustomizationEventTrait for CustomizationUnknownFailure {
    fn get_log_location(&self) -> &Option<String> { &self.log_location }
}
impl CustomizationEventTrait for CustomizationStartedEvent {
    fn get_log_location(&self) -> &Option<String> { &self.log_location }
}
impl CustomizationEventTrait for CustomizationSucceeded {
    fn get_log_location(&self) -> &Option<String> { &self.log_location }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CustomizationEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationEvent => Some(from.as_any_ref().downcast_ref::<CustomizationEvent>()?),
            StructType::CustomizationFailed => Some(from.as_any_ref().downcast_ref::<CustomizationFailed>()?),
            StructType::CustomizationLinuxIdentityFailed => Some(from.as_any_ref().downcast_ref::<CustomizationLinuxIdentityFailed>()?),
            StructType::CustomizationNetworkSetupFailed => Some(from.as_any_ref().downcast_ref::<CustomizationNetworkSetupFailed>()?),
            StructType::CustomizationSysprepFailed => Some(from.as_any_ref().downcast_ref::<CustomizationSysprepFailed>()?),
            StructType::CustomizationUnknownFailure => Some(from.as_any_ref().downcast_ref::<CustomizationUnknownFailure>()?),
            StructType::CustomizationStartedEvent => Some(from.as_any_ref().downcast_ref::<CustomizationStartedEvent>()?),
            StructType::CustomizationSucceeded => Some(from.as_any_ref().downcast_ref::<CustomizationSucceeded>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationEvent => Ok(from.as_any_box().downcast::<CustomizationEvent>()?),
            StructType::CustomizationFailed => Ok(from.as_any_box().downcast::<CustomizationFailed>()?),
            StructType::CustomizationLinuxIdentityFailed => Ok(from.as_any_box().downcast::<CustomizationLinuxIdentityFailed>()?),
            StructType::CustomizationNetworkSetupFailed => Ok(from.as_any_box().downcast::<CustomizationNetworkSetupFailed>()?),
            StructType::CustomizationSysprepFailed => Ok(from.as_any_box().downcast::<CustomizationSysprepFailed>()?),
            StructType::CustomizationUnknownFailure => Ok(from.as_any_box().downcast::<CustomizationUnknownFailure>()?),
            StructType::CustomizationStartedEvent => Ok(from.as_any_box().downcast::<CustomizationStartedEvent>()?),
            StructType::CustomizationSucceeded => Ok(from.as_any_box().downcast::<CustomizationSucceeded>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
