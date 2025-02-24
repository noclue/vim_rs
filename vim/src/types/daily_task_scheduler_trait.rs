use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *DailyTaskScheduler* data object sets the time for daily
/// task execution.
/// 
/// You set the hour and the inherited minute
/// property to complete the schedule. By default, the scheduled task
/// will run once every day at the specified hour and minute.
/// 
/// If you set the interval to a value greater than 1, the task will
/// execute at the specified daily interval. (For example, an interval
/// of 2 will cause the task to execute at the specified hour and minute
/// every 2 days.)
pub trait DailyTaskSchedulerTrait : super::hourly_task_scheduler_trait::HourlyTaskSchedulerTrait {
    /// The hour at which the *RecurrentTaskScheduler* runs the task.
    /// 
    /// Use UTC (Coordinated Universal Time) values in the range
    /// 0 to 23, where 0 = 12:00 a.m. (UTC) and 12 = 12:00 p.m. (UTC).
    /// 
    /// For vCenter 2.x and prior releases, use the server's local time.
    /// For example, use Eastern Standard Time (EST) or Pacific Daylight Time (PDT),
    /// rather than UTC.
    fn get_hour(&self) -> i32;
}
impl<'s> serde::Serialize for dyn DailyTaskSchedulerTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DailyTaskSchedulerTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DailyTaskSchedulerVisitor)
            }
        }

struct DailyTaskSchedulerVisitor;

impl<'de> de::Visitor<'de> for DailyTaskSchedulerVisitor {
    type Value = Box<dyn DailyTaskSchedulerTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DailyTaskSchedulerTrait JSON object with a _typeName field")
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

impl DailyTaskSchedulerTrait for DailyTaskScheduler {
    fn get_hour(&self) -> i32 { self.hour }
}
impl DailyTaskSchedulerTrait for MonthlyTaskScheduler {
    fn get_hour(&self) -> i32 { self.hour }
}
impl DailyTaskSchedulerTrait for MonthlyByDayTaskScheduler {
    fn get_hour(&self) -> i32 { self.hour }
}
impl DailyTaskSchedulerTrait for MonthlyByWeekdayTaskScheduler {
    fn get_hour(&self) -> i32 { self.hour }
}
impl DailyTaskSchedulerTrait for WeeklyTaskScheduler {
    fn get_hour(&self) -> i32 { self.hour }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DailyTaskSchedulerTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
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
            StructType::DailyTaskScheduler => Ok(from.as_any_box().downcast::<DailyTaskScheduler>()?),
            StructType::MonthlyTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyTaskScheduler>()?),
            StructType::MonthlyByDayTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyByDayTaskScheduler>()?),
            StructType::MonthlyByWeekdayTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyByWeekdayTaskScheduler>()?),
            StructType::WeeklyTaskScheduler => Ok(from.as_any_box().downcast::<WeeklyTaskScheduler>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
