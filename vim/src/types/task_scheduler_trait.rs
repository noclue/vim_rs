use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *TaskScheduler* data object is the base type for the scheduler objects.
/// 
/// The hierarchy of scheduler objects is as follows:
/// 
///         TaskScheduler
///             *AfterStartupTaskScheduler*
///             *OnceTaskScheduler*
///             *RecurrentTaskScheduler*
///                 *HourlyTaskScheduler*
///                     *DailyTaskScheduler*
///                         *WeeklyTaskScheduler*
///                         *MonthlyTaskScheduler*
///                             *MonthlyByDayTaskScheduler*
///                             *MonthlyByWeekdayTaskScheduler*
/// 
/// Use a scheduler object to set the time(s) for task execution.
/// You can use two scheduling modes - single execution or
/// recurring execution:
/// - Use the *AfterStartupTaskScheduler* or the *OnceTaskScheduler*
///   to schedule a single instance of task execution.
/// - Use one of the recurrent task schedulers to schedule
///   hourly, daily, weekly, or monthly task execution.
///   
/// After you have established the task timing, use the scheduler
/// object for the *ScheduledTaskSpec*
/// *ScheduledTaskSpec.scheduler* property value.
pub trait TaskSchedulerTrait : super::data_object_trait::DataObjectTrait {
    /// The time that the schedule for the task takes effect.
    /// 
    /// Task activation is distinct from task execution.
    /// When you activate a task, its schedule starts,
    /// and when the next execution time occurs, the task will run.
    /// If you do not set activeTime, the activation time defaults to
    /// the time that you create the scheduled task.
    fn get_active_time(&self) -> &Option<String>;
    /// The time the schedule for the task expires.
    /// 
    /// If you do not set expireTime, the schedule does not expire.
    fn get_expire_time(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn TaskSchedulerTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn TaskSchedulerTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(TaskSchedulerVisitor)
            }
        }

struct TaskSchedulerVisitor;

impl<'de> de::Visitor<'de> for TaskSchedulerVisitor {
    type Value = Box<dyn TaskSchedulerTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid TaskSchedulerTrait JSON object with a _typeName field")
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

impl TaskSchedulerTrait for TaskScheduler {
    fn get_active_time(&self) -> &Option<String> { &self.active_time }
    fn get_expire_time(&self) -> &Option<String> { &self.expire_time }
}
impl TaskSchedulerTrait for AfterStartupTaskScheduler {
    fn get_active_time(&self) -> &Option<String> { &self.active_time }
    fn get_expire_time(&self) -> &Option<String> { &self.expire_time }
}
impl TaskSchedulerTrait for OnceTaskScheduler {
    fn get_active_time(&self) -> &Option<String> { &self.active_time }
    fn get_expire_time(&self) -> &Option<String> { &self.expire_time }
}
impl TaskSchedulerTrait for RecurrentTaskScheduler {
    fn get_active_time(&self) -> &Option<String> { &self.active_time }
    fn get_expire_time(&self) -> &Option<String> { &self.expire_time }
}
impl TaskSchedulerTrait for HourlyTaskScheduler {
    fn get_active_time(&self) -> &Option<String> { &self.active_time }
    fn get_expire_time(&self) -> &Option<String> { &self.expire_time }
}
impl TaskSchedulerTrait for DailyTaskScheduler {
    fn get_active_time(&self) -> &Option<String> { &self.active_time }
    fn get_expire_time(&self) -> &Option<String> { &self.expire_time }
}
impl TaskSchedulerTrait for MonthlyTaskScheduler {
    fn get_active_time(&self) -> &Option<String> { &self.active_time }
    fn get_expire_time(&self) -> &Option<String> { &self.expire_time }
}
impl TaskSchedulerTrait for MonthlyByDayTaskScheduler {
    fn get_active_time(&self) -> &Option<String> { &self.active_time }
    fn get_expire_time(&self) -> &Option<String> { &self.expire_time }
}
impl TaskSchedulerTrait for MonthlyByWeekdayTaskScheduler {
    fn get_active_time(&self) -> &Option<String> { &self.active_time }
    fn get_expire_time(&self) -> &Option<String> { &self.expire_time }
}
impl TaskSchedulerTrait for WeeklyTaskScheduler {
    fn get_active_time(&self) -> &Option<String> { &self.active_time }
    fn get_expire_time(&self) -> &Option<String> { &self.expire_time }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn TaskSchedulerTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::TaskScheduler => Some(from.as_any_ref().downcast_ref::<TaskScheduler>()?),
            StructType::AfterStartupTaskScheduler => Some(from.as_any_ref().downcast_ref::<AfterStartupTaskScheduler>()?),
            StructType::OnceTaskScheduler => Some(from.as_any_ref().downcast_ref::<OnceTaskScheduler>()?),
            StructType::RecurrentTaskScheduler => Some(from.as_any_ref().downcast_ref::<RecurrentTaskScheduler>()?),
            StructType::HourlyTaskScheduler => Some(from.as_any_ref().downcast_ref::<HourlyTaskScheduler>()?),
            StructType::DailyTaskScheduler => Some(from.as_any_ref().downcast_ref::<DailyTaskScheduler>()?),
            StructType::MonthlyTaskScheduler => Some(from.as_any_ref().downcast_ref::<MonthlyTaskScheduler>()?),
            StructType::MonthlyByDayTaskScheduler => Some(from.as_any_ref().downcast_ref::<MonthlyByDayTaskScheduler>()?),
            StructType::MonthlyByWeekdayTaskScheduler => Some(from.as_any_ref().downcast_ref::<MonthlyByWeekdayTaskScheduler>()?),
            StructType::WeeklyTaskScheduler => Some(from.as_any_ref().downcast_ref::<WeeklyTaskScheduler>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::TaskScheduler => Ok(from.as_any_box().downcast::<TaskScheduler>()?),
            StructType::AfterStartupTaskScheduler => Ok(from.as_any_box().downcast::<AfterStartupTaskScheduler>()?),
            StructType::OnceTaskScheduler => Ok(from.as_any_box().downcast::<OnceTaskScheduler>()?),
            StructType::RecurrentTaskScheduler => Ok(from.as_any_box().downcast::<RecurrentTaskScheduler>()?),
            StructType::HourlyTaskScheduler => Ok(from.as_any_box().downcast::<HourlyTaskScheduler>()?),
            StructType::DailyTaskScheduler => Ok(from.as_any_box().downcast::<DailyTaskScheduler>()?),
            StructType::MonthlyTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyTaskScheduler>()?),
            StructType::MonthlyByDayTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyByDayTaskScheduler>()?),
            StructType::MonthlyByWeekdayTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyByWeekdayTaskScheduler>()?),
            StructType::WeeklyTaskScheduler => Ok(from.as_any_box().downcast::<WeeklyTaskScheduler>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
