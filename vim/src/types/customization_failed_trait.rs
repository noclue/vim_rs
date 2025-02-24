use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The customization sequence in the guest failed.
pub trait CustomizationFailedTrait : super::customization_event_trait::CustomizationEventTrait {
    /// Reason why the customization failed @see CustomizationFailed.ReasonCode .
    fn get_reason(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn CustomizationFailedTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CustomizationFailedTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CustomizationFailedVisitor)
            }
        }

struct CustomizationFailedVisitor;

impl<'de> de::Visitor<'de> for CustomizationFailedVisitor {
    type Value = Box<dyn CustomizationFailedTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CustomizationFailedTrait JSON object with a _typeName field")
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

impl CustomizationFailedTrait for CustomizationFailed {
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl CustomizationFailedTrait for CustomizationLinuxIdentityFailed {
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl CustomizationFailedTrait for CustomizationNetworkSetupFailed {
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl CustomizationFailedTrait for CustomizationSysprepFailed {
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl CustomizationFailedTrait for CustomizationUnknownFailure {
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CustomizationFailedTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationFailed => Some(from.as_any_ref().downcast_ref::<CustomizationFailed>()?),
            StructType::CustomizationLinuxIdentityFailed => Some(from.as_any_ref().downcast_ref::<CustomizationLinuxIdentityFailed>()?),
            StructType::CustomizationNetworkSetupFailed => Some(from.as_any_ref().downcast_ref::<CustomizationNetworkSetupFailed>()?),
            StructType::CustomizationSysprepFailed => Some(from.as_any_ref().downcast_ref::<CustomizationSysprepFailed>()?),
            StructType::CustomizationUnknownFailure => Some(from.as_any_ref().downcast_ref::<CustomizationUnknownFailure>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationFailed => Ok(from.as_any_box().downcast::<CustomizationFailed>()?),
            StructType::CustomizationLinuxIdentityFailed => Ok(from.as_any_box().downcast::<CustomizationLinuxIdentityFailed>()?),
            StructType::CustomizationNetworkSetupFailed => Ok(from.as_any_box().downcast::<CustomizationNetworkSetupFailed>()?),
            StructType::CustomizationSysprepFailed => Ok(from.as_any_box().downcast::<CustomizationSysprepFailed>()?),
            StructType::CustomizationUnknownFailure => Ok(from.as_any_box().downcast::<CustomizationUnknownFailure>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
