use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These events are scheduled task events.
pub trait ScheduledTaskEventTrait : super::event_trait::EventTrait {
    /// The scheduled task object.
    fn get_scheduled_task(&self) -> &ScheduledTaskEventArgument;
    /// The entity on which the scheduled task registered.
    fn get_entity(&self) -> &ManagedEntityEventArgument;
}
impl<'s> serde::Serialize for dyn ScheduledTaskEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ScheduledTaskEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ScheduledTaskEventVisitor)
            }
        }

struct ScheduledTaskEventVisitor;

impl<'de> de::Visitor<'de> for ScheduledTaskEventVisitor {
    type Value = Box<dyn ScheduledTaskEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ScheduledTaskEventTrait JSON object with a _typeName field")
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

impl ScheduledTaskEventTrait for ScheduledTaskEvent {
    fn get_scheduled_task(&self) -> &ScheduledTaskEventArgument { &self.scheduled_task }
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
}
impl ScheduledTaskEventTrait for ScheduledTaskCompletedEvent {
    fn get_scheduled_task(&self) -> &ScheduledTaskEventArgument { &self.scheduled_task }
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
}
impl ScheduledTaskEventTrait for ScheduledTaskCreatedEvent {
    fn get_scheduled_task(&self) -> &ScheduledTaskEventArgument { &self.scheduled_task }
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
}
impl ScheduledTaskEventTrait for ScheduledTaskEmailCompletedEvent {
    fn get_scheduled_task(&self) -> &ScheduledTaskEventArgument { &self.scheduled_task }
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
}
impl ScheduledTaskEventTrait for ScheduledTaskEmailFailedEvent {
    fn get_scheduled_task(&self) -> &ScheduledTaskEventArgument { &self.scheduled_task }
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
}
impl ScheduledTaskEventTrait for ScheduledTaskFailedEvent {
    fn get_scheduled_task(&self) -> &ScheduledTaskEventArgument { &self.scheduled_task }
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
}
impl ScheduledTaskEventTrait for ScheduledTaskReconfiguredEvent {
    fn get_scheduled_task(&self) -> &ScheduledTaskEventArgument { &self.scheduled_task }
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
}
impl ScheduledTaskEventTrait for ScheduledTaskRemovedEvent {
    fn get_scheduled_task(&self) -> &ScheduledTaskEventArgument { &self.scheduled_task }
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
}
impl ScheduledTaskEventTrait for ScheduledTaskStartedEvent {
    fn get_scheduled_task(&self) -> &ScheduledTaskEventArgument { &self.scheduled_task }
    fn get_entity(&self) -> &ManagedEntityEventArgument { &self.entity }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ScheduledTaskEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ScheduledTaskEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskEvent>()?),
            StructType::ScheduledTaskCompletedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskCompletedEvent>()?),
            StructType::ScheduledTaskCreatedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskCreatedEvent>()?),
            StructType::ScheduledTaskEmailCompletedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskEmailCompletedEvent>()?),
            StructType::ScheduledTaskEmailFailedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskEmailFailedEvent>()?),
            StructType::ScheduledTaskFailedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskFailedEvent>()?),
            StructType::ScheduledTaskReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskReconfiguredEvent>()?),
            StructType::ScheduledTaskRemovedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskRemovedEvent>()?),
            StructType::ScheduledTaskStartedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskStartedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ScheduledTaskEvent => Ok(from.as_any_box().downcast::<ScheduledTaskEvent>()?),
            StructType::ScheduledTaskCompletedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskCompletedEvent>()?),
            StructType::ScheduledTaskCreatedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskCreatedEvent>()?),
            StructType::ScheduledTaskEmailCompletedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskEmailCompletedEvent>()?),
            StructType::ScheduledTaskEmailFailedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskEmailFailedEvent>()?),
            StructType::ScheduledTaskFailedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskFailedEvent>()?),
            StructType::ScheduledTaskReconfiguredEvent => Ok(from.as_any_box().downcast::<ScheduledTaskReconfiguredEvent>()?),
            StructType::ScheduledTaskRemovedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskRemovedEvent>()?),
            StructType::ScheduledTaskStartedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskStartedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
