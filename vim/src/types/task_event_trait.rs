use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event records the creation of a Task.
/// 
/// Note that the embedded TaskInfo object is a _snapshot_ of the
/// Task state at the time of its creation, so its state will always be
/// "queued". To find the current status of the task, query for the
/// current state of the Task using the eventChainId in the embedded
/// TaskInfo object.
pub trait TaskEventTrait : super::event_trait::EventTrait {
    /// The information about the task.
    fn get_info(&self) -> &TaskInfo;
}
impl<'s> serde::Serialize for dyn TaskEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn TaskEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(TaskEventVisitor)
            }
        }

struct TaskEventVisitor;

impl<'de> de::Visitor<'de> for TaskEventVisitor {
    type Value = Box<dyn TaskEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid TaskEventTrait JSON object with a _typeName field")
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

impl TaskEventTrait for TaskEvent {
    fn get_info(&self) -> &TaskInfo { &self.info }
}
impl TaskEventTrait for TaskTimeoutEvent {
    fn get_info(&self) -> &TaskInfo { &self.info }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn TaskEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::TaskEvent => Some(from.as_any_ref().downcast_ref::<TaskEvent>()?),
            StructType::TaskTimeoutEvent => Some(from.as_any_ref().downcast_ref::<TaskTimeoutEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::TaskEvent => Ok(from.as_any_box().downcast::<TaskEvent>()?),
            StructType::TaskTimeoutEvent => Ok(from.as_any_box().downcast::<TaskTimeoutEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
