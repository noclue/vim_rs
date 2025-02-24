use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event is the base class for all the template upgrade events.
pub trait TemplateUpgradeEventTrait : super::event_trait::EventTrait {
    fn get_legacy_template(&self) -> &str;
}
impl<'s> serde::Serialize for dyn TemplateUpgradeEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn TemplateUpgradeEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(TemplateUpgradeEventVisitor)
            }
        }

struct TemplateUpgradeEventVisitor;

impl<'de> de::Visitor<'de> for TemplateUpgradeEventVisitor {
    type Value = Box<dyn TemplateUpgradeEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid TemplateUpgradeEventTrait JSON object with a _typeName field")
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

impl TemplateUpgradeEventTrait for TemplateUpgradeEvent {
    fn get_legacy_template(&self) -> &str { &self.legacy_template }
}
impl TemplateUpgradeEventTrait for TemplateBeingUpgradedEvent {
    fn get_legacy_template(&self) -> &str { &self.legacy_template }
}
impl TemplateUpgradeEventTrait for TemplateUpgradeFailedEvent {
    fn get_legacy_template(&self) -> &str { &self.legacy_template }
}
impl TemplateUpgradeEventTrait for TemplateUpgradedEvent {
    fn get_legacy_template(&self) -> &str { &self.legacy_template }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn TemplateUpgradeEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::TemplateUpgradeEvent => Some(from.as_any_ref().downcast_ref::<TemplateUpgradeEvent>()?),
            StructType::TemplateBeingUpgradedEvent => Some(from.as_any_ref().downcast_ref::<TemplateBeingUpgradedEvent>()?),
            StructType::TemplateUpgradeFailedEvent => Some(from.as_any_ref().downcast_ref::<TemplateUpgradeFailedEvent>()?),
            StructType::TemplateUpgradedEvent => Some(from.as_any_ref().downcast_ref::<TemplateUpgradedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::TemplateUpgradeEvent => Ok(from.as_any_box().downcast::<TemplateUpgradeEvent>()?),
            StructType::TemplateBeingUpgradedEvent => Ok(from.as_any_box().downcast::<TemplateBeingUpgradedEvent>()?),
            StructType::TemplateUpgradeFailedEvent => Ok(from.as_any_box().downcast::<TemplateUpgradeFailedEvent>()?),
            StructType::TemplateUpgradedEvent => Ok(from.as_any_box().downcast::<TemplateUpgradedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
