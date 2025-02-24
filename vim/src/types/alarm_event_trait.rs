use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event is an alarm events.
pub trait AlarmEventTrait : super::event_trait::EventTrait {
    /// The associated alarm object.
    fn get_alarm(&self) -> &AlarmEventArgument;
}
impl<'s> serde::Serialize for dyn AlarmEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn AlarmEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(AlarmEventVisitor)
            }
        }

struct AlarmEventVisitor;

impl<'de> de::Visitor<'de> for AlarmEventVisitor {
    type Value = Box<dyn AlarmEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid AlarmEventTrait JSON object with a _typeName field")
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

impl AlarmEventTrait for AlarmEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmAcknowledgedEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmActionTriggeredEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmClearedEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmCreatedEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmEmailCompletedEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmEmailFailedEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmReconfiguredEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmRemovedEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmScriptCompleteEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmScriptFailedEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmSnmpCompletedEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmSnmpFailedEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl AlarmEventTrait for AlarmStatusChangedEvent {
    fn get_alarm(&self) -> &AlarmEventArgument { &self.alarm }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn AlarmEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::AlarmEvent => Some(from.as_any_ref().downcast_ref::<AlarmEvent>()?),
            StructType::AlarmAcknowledgedEvent => Some(from.as_any_ref().downcast_ref::<AlarmAcknowledgedEvent>()?),
            StructType::AlarmActionTriggeredEvent => Some(from.as_any_ref().downcast_ref::<AlarmActionTriggeredEvent>()?),
            StructType::AlarmClearedEvent => Some(from.as_any_ref().downcast_ref::<AlarmClearedEvent>()?),
            StructType::AlarmCreatedEvent => Some(from.as_any_ref().downcast_ref::<AlarmCreatedEvent>()?),
            StructType::AlarmEmailCompletedEvent => Some(from.as_any_ref().downcast_ref::<AlarmEmailCompletedEvent>()?),
            StructType::AlarmEmailFailedEvent => Some(from.as_any_ref().downcast_ref::<AlarmEmailFailedEvent>()?),
            StructType::AlarmReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<AlarmReconfiguredEvent>()?),
            StructType::AlarmRemovedEvent => Some(from.as_any_ref().downcast_ref::<AlarmRemovedEvent>()?),
            StructType::AlarmScriptCompleteEvent => Some(from.as_any_ref().downcast_ref::<AlarmScriptCompleteEvent>()?),
            StructType::AlarmScriptFailedEvent => Some(from.as_any_ref().downcast_ref::<AlarmScriptFailedEvent>()?),
            StructType::AlarmSnmpCompletedEvent => Some(from.as_any_ref().downcast_ref::<AlarmSnmpCompletedEvent>()?),
            StructType::AlarmSnmpFailedEvent => Some(from.as_any_ref().downcast_ref::<AlarmSnmpFailedEvent>()?),
            StructType::AlarmStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<AlarmStatusChangedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::AlarmEvent => Ok(from.as_any_box().downcast::<AlarmEvent>()?),
            StructType::AlarmAcknowledgedEvent => Ok(from.as_any_box().downcast::<AlarmAcknowledgedEvent>()?),
            StructType::AlarmActionTriggeredEvent => Ok(from.as_any_box().downcast::<AlarmActionTriggeredEvent>()?),
            StructType::AlarmClearedEvent => Ok(from.as_any_box().downcast::<AlarmClearedEvent>()?),
            StructType::AlarmCreatedEvent => Ok(from.as_any_box().downcast::<AlarmCreatedEvent>()?),
            StructType::AlarmEmailCompletedEvent => Ok(from.as_any_box().downcast::<AlarmEmailCompletedEvent>()?),
            StructType::AlarmEmailFailedEvent => Ok(from.as_any_box().downcast::<AlarmEmailFailedEvent>()?),
            StructType::AlarmReconfiguredEvent => Ok(from.as_any_box().downcast::<AlarmReconfiguredEvent>()?),
            StructType::AlarmRemovedEvent => Ok(from.as_any_box().downcast::<AlarmRemovedEvent>()?),
            StructType::AlarmScriptCompleteEvent => Ok(from.as_any_box().downcast::<AlarmScriptCompleteEvent>()?),
            StructType::AlarmScriptFailedEvent => Ok(from.as_any_box().downcast::<AlarmScriptFailedEvent>()?),
            StructType::AlarmSnmpCompletedEvent => Ok(from.as_any_box().downcast::<AlarmSnmpCompletedEvent>()?),
            StructType::AlarmSnmpFailedEvent => Ok(from.as_any_box().downcast::<AlarmSnmpFailedEvent>()?),
            StructType::AlarmStatusChangedEvent => Ok(from.as_any_box().downcast::<AlarmStatusChangedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
