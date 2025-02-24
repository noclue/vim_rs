use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Parameters for scheduled task creation.
pub trait ScheduledTaskSpecTrait : super::data_object_trait::DataObjectTrait {
    /// Name of the scheduled task.
    fn get_name(&self) -> &str;
    /// Description of the scheduled task.
    fn get_description(&self) -> &str;
    /// Flag to indicate whether the scheduled task is enabled or disabled.
    fn get_enabled(&self) -> bool;
    /// The time scheduler that determines when the scheduled task runs.
    fn get_scheduler(&self) -> &Box<dyn super::task_scheduler_trait::TaskSchedulerTrait>;
    /// The action of the scheduled task, to be done when the scheduled task runs.
    fn get_action(&self) -> &Box<dyn super::action_trait::ActionTrait>;
    /// The email notification.
    /// 
    /// If not set, this property is set to empty string, indicating no notification.
    fn get_notification(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn ScheduledTaskSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ScheduledTaskSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ScheduledTaskSpecVisitor)
            }
        }

struct ScheduledTaskSpecVisitor;

impl<'de> de::Visitor<'de> for ScheduledTaskSpecVisitor {
    type Value = Box<dyn ScheduledTaskSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ScheduledTaskSpecTrait JSON object with a _typeName field")
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

impl ScheduledTaskSpecTrait for ScheduledTaskSpec {
    fn get_name(&self) -> &str { &self.name }
    fn get_description(&self) -> &str { &self.description }
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_scheduler(&self) -> &Box<dyn super::task_scheduler_trait::TaskSchedulerTrait> { &self.scheduler }
    fn get_action(&self) -> &Box<dyn super::action_trait::ActionTrait> { &self.action }
    fn get_notification(&self) -> &Option<String> { &self.notification }
}
impl ScheduledTaskSpecTrait for ScheduledTaskInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_description(&self) -> &str { &self.description }
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_scheduler(&self) -> &Box<dyn super::task_scheduler_trait::TaskSchedulerTrait> { &self.scheduler }
    fn get_action(&self) -> &Box<dyn super::action_trait::ActionTrait> { &self.action }
    fn get_notification(&self) -> &Option<String> { &self.notification }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ScheduledTaskSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ScheduledTaskSpec => Some(from.as_any_ref().downcast_ref::<ScheduledTaskSpec>()?),
            StructType::ScheduledTaskInfo => Some(from.as_any_ref().downcast_ref::<ScheduledTaskInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ScheduledTaskSpec => Ok(from.as_any_box().downcast::<ScheduledTaskSpec>()?),
            StructType::ScheduledTaskInfo => Ok(from.as_any_box().downcast::<ScheduledTaskInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
