use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Parameters for alarm creation.
pub trait AlarmSpecTrait : super::data_object_trait::DataObjectTrait {
    /// Name of the alarm.
    fn get_name(&self) -> &str;
    /// System name of the alarm.
    /// 
    /// This is set only for predefined Alarms - i.e. Alarms created by the
    /// server or extensions automatically. After creation this value cannot be
    /// modified. User-created Alarms do not have a systemName at all.
    /// 
    /// The purpose of this field is to identify system-created Alarms
    /// reliably, even if they are edited by users.
    /// 
    /// When creating Alarms with systemName, the systemName and the name of the
    /// alarm should be equal.
    /// 
    /// When reconfiguring an Alarm with systemName, the same systemName should
    /// be passed in the new AlarmSpec. Renaming Alarms with systemName is not
    /// allowed, i.e. when reconfiguring, the name passed in the new AlarmSpec
    /// should be equal to either the systemName or its localized version (the
    /// current name in the Alarm's info).
    fn get_system_name(&self) -> &Option<String>;
    /// Description of the alarm.
    fn get_description(&self) -> &str;
    /// Flag to indicate whether or not the alarm is enabled or disabled.
    fn get_enabled(&self) -> bool;
    /// Top-level alarm expression that defines trigger conditions.
    fn get_expression(&self) -> &Box<dyn super::alarm_expression_trait::AlarmExpressionTrait>;
    /// Action to perform when the alarm is triggered.
    fn get_action(&self) -> &Option<Box<dyn super::alarm_action_trait::AlarmActionTrait>>;
    /// Frequency in seconds, which specifies how often appropriate actions
    /// should repeat when an alarm does not change state.
    fn get_action_frequency(&self) -> Option<i32>;
    /// Tolerance and maximum frequency settings.
    fn get_setting(&self) -> &Option<AlarmSetting>;
}
impl<'s> serde::Serialize for dyn AlarmSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn AlarmSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(AlarmSpecVisitor)
            }
        }

struct AlarmSpecVisitor;

impl<'de> de::Visitor<'de> for AlarmSpecVisitor {
    type Value = Box<dyn AlarmSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid AlarmSpecTrait JSON object with a _typeName field")
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

impl AlarmSpecTrait for AlarmSpec {
    fn get_name(&self) -> &str { &self.name }
    fn get_system_name(&self) -> &Option<String> { &self.system_name }
    fn get_description(&self) -> &str { &self.description }
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_expression(&self) -> &Box<dyn super::alarm_expression_trait::AlarmExpressionTrait> { &self.expression }
    fn get_action(&self) -> &Option<Box<dyn super::alarm_action_trait::AlarmActionTrait>> { &self.action }
    fn get_action_frequency(&self) -> Option<i32> { self.action_frequency }
    fn get_setting(&self) -> &Option<AlarmSetting> { &self.setting }
}
impl AlarmSpecTrait for AlarmInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_system_name(&self) -> &Option<String> { &self.system_name }
    fn get_description(&self) -> &str { &self.description }
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_expression(&self) -> &Box<dyn super::alarm_expression_trait::AlarmExpressionTrait> { &self.expression }
    fn get_action(&self) -> &Option<Box<dyn super::alarm_action_trait::AlarmActionTrait>> { &self.action }
    fn get_action_frequency(&self) -> Option<i32> { self.action_frequency }
    fn get_setting(&self) -> &Option<AlarmSetting> { &self.setting }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn AlarmSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::AlarmSpec => Some(from.as_any_ref().downcast_ref::<AlarmSpec>()?),
            StructType::AlarmInfo => Some(from.as_any_ref().downcast_ref::<AlarmInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::AlarmSpec => Ok(from.as_any_box().downcast::<AlarmSpec>()?),
            StructType::AlarmInfo => Ok(from.as_any_box().downcast::<AlarmInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
