use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *RecurrentTaskScheduler* data object is the base type for
/// the hierarchy that includes hourly, daily, weekly, and monthly task schedulers.
pub trait RecurrentTaskSchedulerTrait : super::task_scheduler_trait::TaskSchedulerTrait {
    /// How often to run the scheduled task.
    /// 
    /// The value must be greater than
    /// or equal to 1 and less than 1000. The default value is 1.
    /// The interval acts as a multiplier for the unit of time associated
    /// with a particular scheduler (hours, days, weeks, or months).
    /// For example, setting the *HourlyTaskScheduler* interval
    /// to 4 causes the task to run every 4 hours.
    fn get_interval(&self) -> i32;
}
impl<'s> serde::Serialize for dyn RecurrentTaskSchedulerTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn RecurrentTaskSchedulerTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(RecurrentTaskSchedulerVisitor)
            }
        }

struct RecurrentTaskSchedulerVisitor;

impl<'de> de::Visitor<'de> for RecurrentTaskSchedulerVisitor {
    type Value = Box<dyn RecurrentTaskSchedulerTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid RecurrentTaskSchedulerTrait JSON object with a _typeName field")
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

impl RecurrentTaskSchedulerTrait for RecurrentTaskScheduler {
    fn get_interval(&self) -> i32 { self.interval }
}
impl RecurrentTaskSchedulerTrait for HourlyTaskScheduler {
    fn get_interval(&self) -> i32 { self.interval }
}
impl RecurrentTaskSchedulerTrait for DailyTaskScheduler {
    fn get_interval(&self) -> i32 { self.interval }
}
impl RecurrentTaskSchedulerTrait for MonthlyTaskScheduler {
    fn get_interval(&self) -> i32 { self.interval }
}
impl RecurrentTaskSchedulerTrait for MonthlyByDayTaskScheduler {
    fn get_interval(&self) -> i32 { self.interval }
}
impl RecurrentTaskSchedulerTrait for MonthlyByWeekdayTaskScheduler {
    fn get_interval(&self) -> i32 { self.interval }
}
impl RecurrentTaskSchedulerTrait for WeeklyTaskScheduler {
    fn get_interval(&self) -> i32 { self.interval }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn RecurrentTaskSchedulerTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
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
