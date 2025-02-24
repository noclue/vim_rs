use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *MonthlyTaskScheduler* data object is the base type for
/// the monthly schedulers (*MonthlyByDayTaskScheduler* and
/// *MonthlyByWeekdayTaskScheduler*).
pub trait MonthlyTaskSchedulerTrait : super::daily_task_scheduler_trait::DailyTaskSchedulerTrait {
}
impl<'s> serde::Serialize for dyn MonthlyTaskSchedulerTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn MonthlyTaskSchedulerTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(MonthlyTaskSchedulerVisitor)
            }
        }

struct MonthlyTaskSchedulerVisitor;

impl<'de> de::Visitor<'de> for MonthlyTaskSchedulerVisitor {
    type Value = Box<dyn MonthlyTaskSchedulerTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid MonthlyTaskSchedulerTrait JSON object with a _typeName field")
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

impl MonthlyTaskSchedulerTrait for MonthlyTaskScheduler {
}
impl MonthlyTaskSchedulerTrait for MonthlyByDayTaskScheduler {
}
impl MonthlyTaskSchedulerTrait for MonthlyByWeekdayTaskScheduler {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn MonthlyTaskSchedulerTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::MonthlyTaskScheduler => Some(from.as_any_ref().downcast_ref::<MonthlyTaskScheduler>()?),
            StructType::MonthlyByDayTaskScheduler => Some(from.as_any_ref().downcast_ref::<MonthlyByDayTaskScheduler>()?),
            StructType::MonthlyByWeekdayTaskScheduler => Some(from.as_any_ref().downcast_ref::<MonthlyByWeekdayTaskScheduler>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::MonthlyTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyTaskScheduler>()?),
            StructType::MonthlyByDayTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyByDayTaskScheduler>()?),
            StructType::MonthlyByWeekdayTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyByWeekdayTaskScheduler>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
