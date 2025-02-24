use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base type for the expressions specifying the conditions that define
/// the status of an alarm.
pub trait AlarmExpressionTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn AlarmExpressionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn AlarmExpressionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(AlarmExpressionVisitor)
            }
        }

struct AlarmExpressionVisitor;

impl<'de> de::Visitor<'de> for AlarmExpressionVisitor {
    type Value = Box<dyn AlarmExpressionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid AlarmExpressionTrait JSON object with a _typeName field")
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

impl AlarmExpressionTrait for AlarmExpression {
}
impl AlarmExpressionTrait for AndAlarmExpression {
}
impl AlarmExpressionTrait for EventAlarmExpression {
}
impl AlarmExpressionTrait for MetricAlarmExpression {
}
impl AlarmExpressionTrait for OrAlarmExpression {
}
impl AlarmExpressionTrait for StateAlarmExpression {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn AlarmExpressionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::AlarmExpression => Some(from.as_any_ref().downcast_ref::<AlarmExpression>()?),
            StructType::AndAlarmExpression => Some(from.as_any_ref().downcast_ref::<AndAlarmExpression>()?),
            StructType::EventAlarmExpression => Some(from.as_any_ref().downcast_ref::<EventAlarmExpression>()?),
            StructType::MetricAlarmExpression => Some(from.as_any_ref().downcast_ref::<MetricAlarmExpression>()?),
            StructType::OrAlarmExpression => Some(from.as_any_ref().downcast_ref::<OrAlarmExpression>()?),
            StructType::StateAlarmExpression => Some(from.as_any_ref().downcast_ref::<StateAlarmExpression>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::AlarmExpression => Ok(from.as_any_box().downcast::<AlarmExpression>()?),
            StructType::AndAlarmExpression => Ok(from.as_any_box().downcast::<AndAlarmExpression>()?),
            StructType::EventAlarmExpression => Ok(from.as_any_box().downcast::<EventAlarmExpression>()?),
            StructType::MetricAlarmExpression => Ok(from.as_any_box().downcast::<MetricAlarmExpression>()?),
            StructType::OrAlarmExpression => Ok(from.as_any_box().downcast::<OrAlarmExpression>()?),
            StructType::StateAlarmExpression => Ok(from.as_any_box().downcast::<StateAlarmExpression>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
