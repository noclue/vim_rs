use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type defines the action initiated by a scheduled task or alarm.
/// 
/// This is an abstract type.
/// A client creates a scheduled task or an alarm each of which triggers
/// an action, defined by a subclass of this type.
pub trait ActionTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn ActionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ActionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ActionVisitor)
            }
        }

struct ActionVisitor;

impl<'de> de::Visitor<'de> for ActionVisitor {
    type Value = Box<dyn ActionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ActionTrait JSON object with a _typeName field")
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

impl ActionTrait for Action {
}
impl ActionTrait for CreateTaskAction {
}
impl ActionTrait for MethodAction {
}
impl ActionTrait for RunScriptAction {
}
impl ActionTrait for SendEmailAction {
}
impl ActionTrait for SendSnmpAction {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ActionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::Action => Some(from.as_any_ref().downcast_ref::<Action>()?),
            StructType::CreateTaskAction => Some(from.as_any_ref().downcast_ref::<CreateTaskAction>()?),
            StructType::MethodAction => Some(from.as_any_ref().downcast_ref::<MethodAction>()?),
            StructType::RunScriptAction => Some(from.as_any_ref().downcast_ref::<RunScriptAction>()?),
            StructType::SendEmailAction => Some(from.as_any_ref().downcast_ref::<SendEmailAction>()?),
            StructType::SendSnmpAction => Some(from.as_any_ref().downcast_ref::<SendSnmpAction>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::Action => Ok(from.as_any_box().downcast::<Action>()?),
            StructType::CreateTaskAction => Ok(from.as_any_box().downcast::<CreateTaskAction>()?),
            StructType::MethodAction => Ok(from.as_any_box().downcast::<MethodAction>()?),
            StructType::RunScriptAction => Ok(from.as_any_box().downcast::<RunScriptAction>()?),
            StructType::SendEmailAction => Ok(from.as_any_box().downcast::<SendEmailAction>()?),
            StructType::SendSnmpAction => Ok(from.as_any_box().downcast::<SendSnmpAction>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
