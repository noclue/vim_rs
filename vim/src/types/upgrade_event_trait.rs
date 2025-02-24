use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These event types represent events converted from VirtualCenter 1.x.
/// 
/// All upgraded events are converted to string values.
pub trait UpgradeEventTrait : super::event_trait::EventTrait {
    /// The formatted message from the upgrade.
    fn get_message(&self) -> &str;
}
impl<'s> serde::Serialize for dyn UpgradeEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn UpgradeEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(UpgradeEventVisitor)
            }
        }

struct UpgradeEventVisitor;

impl<'de> de::Visitor<'de> for UpgradeEventVisitor {
    type Value = Box<dyn UpgradeEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid UpgradeEventTrait JSON object with a _typeName field")
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

impl UpgradeEventTrait for UpgradeEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl UpgradeEventTrait for ErrorUpgradeEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl UpgradeEventTrait for InfoUpgradeEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl UpgradeEventTrait for UserUpgradeEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl UpgradeEventTrait for WarningUpgradeEvent {
    fn get_message(&self) -> &str { &self.message }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn UpgradeEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::UpgradeEvent => Some(from.as_any_ref().downcast_ref::<UpgradeEvent>()?),
            StructType::ErrorUpgradeEvent => Some(from.as_any_ref().downcast_ref::<ErrorUpgradeEvent>()?),
            StructType::InfoUpgradeEvent => Some(from.as_any_ref().downcast_ref::<InfoUpgradeEvent>()?),
            StructType::UserUpgradeEvent => Some(from.as_any_ref().downcast_ref::<UserUpgradeEvent>()?),
            StructType::WarningUpgradeEvent => Some(from.as_any_ref().downcast_ref::<WarningUpgradeEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::UpgradeEvent => Ok(from.as_any_box().downcast::<UpgradeEvent>()?),
            StructType::ErrorUpgradeEvent => Ok(from.as_any_box().downcast::<ErrorUpgradeEvent>()?),
            StructType::InfoUpgradeEvent => Ok(from.as_any_box().downcast::<InfoUpgradeEvent>()?),
            StructType::UserUpgradeEvent => Ok(from.as_any_box().downcast::<UserUpgradeEvent>()?),
            StructType::WarningUpgradeEvent => Ok(from.as_any_box().downcast::<WarningUpgradeEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
