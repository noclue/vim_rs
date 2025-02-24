use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *HourlyTaskScheduler* data object sets the time for hourly
/// task execution.
/// 
/// By default, the scheduled task will run once every hour,
/// at the specified minute.
/// 
/// If you set the interval to a value greater than 1, the task will
/// execute at the specified hourly interval. (For example, an interval
/// of 2 will cause the task to execute at the specified minute every 2 hours.)
pub trait HourlyTaskSchedulerTrait : super::recurrent_task_scheduler_trait::RecurrentTaskSchedulerTrait {
    /// The minute at which the *RecurrentTaskScheduler* runs
    /// the task.
    /// 
    /// Specify the minute value as a UTC (Coordinated Universal Time)
    /// value in the range 0 to 59.
    /// 
    /// For vCenter 2.x and prior releases, use the server's local time.
    /// For example, use Australia Northern Territory (UTC +9:30) or Indian (UTC +5:30)
    /// time values, rather than a UTC value.
    fn get_minute(&self) -> i32;
}
impl<'s> serde::Serialize for dyn HourlyTaskSchedulerTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HourlyTaskSchedulerTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HourlyTaskSchedulerVisitor)
            }
        }

struct HourlyTaskSchedulerVisitor;

impl<'de> de::Visitor<'de> for HourlyTaskSchedulerVisitor {
    type Value = Box<dyn HourlyTaskSchedulerTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HourlyTaskSchedulerTrait JSON object with a _typeName field")
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

impl HourlyTaskSchedulerTrait for HourlyTaskScheduler {
    fn get_minute(&self) -> i32 { self.minute }
}
impl HourlyTaskSchedulerTrait for DailyTaskScheduler {
    fn get_minute(&self) -> i32 { self.minute }
}
impl HourlyTaskSchedulerTrait for MonthlyTaskScheduler {
    fn get_minute(&self) -> i32 { self.minute }
}
impl HourlyTaskSchedulerTrait for MonthlyByDayTaskScheduler {
    fn get_minute(&self) -> i32 { self.minute }
}
impl HourlyTaskSchedulerTrait for MonthlyByWeekdayTaskScheduler {
    fn get_minute(&self) -> i32 { self.minute }
}
impl HourlyTaskSchedulerTrait for WeeklyTaskScheduler {
    fn get_minute(&self) -> i32 { self.minute }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HourlyTaskSchedulerTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
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
