use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base type for all task reasons.
/// 
/// Task reasons represent the kind of entity responsible for a task's creation.
pub trait TaskReasonTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn TaskReasonTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn TaskReasonTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(TaskReasonVisitor)
            }
        }

struct TaskReasonVisitor;

impl<'de> de::Visitor<'de> for TaskReasonVisitor {
    type Value = Box<dyn TaskReasonTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid TaskReasonTrait JSON object with a _typeName field")
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

impl TaskReasonTrait for TaskReason {
}
impl TaskReasonTrait for TaskReasonAlarm {
}
impl TaskReasonTrait for TaskReasonSchedule {
}
impl TaskReasonTrait for TaskReasonSystem {
}
impl TaskReasonTrait for TaskReasonUser {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn TaskReasonTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::TaskReason => Some(from.as_any_ref().downcast_ref::<TaskReason>()?),
            StructType::TaskReasonAlarm => Some(from.as_any_ref().downcast_ref::<TaskReasonAlarm>()?),
            StructType::TaskReasonSchedule => Some(from.as_any_ref().downcast_ref::<TaskReasonSchedule>()?),
            StructType::TaskReasonSystem => Some(from.as_any_ref().downcast_ref::<TaskReasonSystem>()?),
            StructType::TaskReasonUser => Some(from.as_any_ref().downcast_ref::<TaskReasonUser>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::TaskReason => Ok(from.as_any_box().downcast::<TaskReason>()?),
            StructType::TaskReasonAlarm => Ok(from.as_any_box().downcast::<TaskReasonAlarm>()?),
            StructType::TaskReasonSchedule => Ok(from.as_any_box().downcast::<TaskReasonSchedule>()?),
            StructType::TaskReasonSystem => Ok(from.as_any_box().downcast::<TaskReasonSystem>()?),
            StructType::TaskReasonUser => Ok(from.as_any_box().downcast::<TaskReasonUser>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
