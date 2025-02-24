use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The TaskInProgress data object type represents a fault when an operation tries
/// to access an entity that already has another (long) operation in progress.
pub trait TaskInProgressTrait : super::vim_fault_trait::VimFaultTrait {
    /// The task already in progress when the operation was attempted.
    /// 
    /// Refers instance of *Task*.
    fn get_task(&self) -> &ManagedObjectReference;
}
impl<'s> serde::Serialize for dyn TaskInProgressTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn TaskInProgressTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(TaskInProgressVisitor)
            }
        }

struct TaskInProgressVisitor;

impl<'de> de::Visitor<'de> for TaskInProgressVisitor {
    type Value = Box<dyn TaskInProgressTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid TaskInProgressTrait JSON object with a _typeName field")
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

impl TaskInProgressTrait for TaskInProgress {
    fn get_task(&self) -> &ManagedObjectReference { &self.task }
}
impl TaskInProgressTrait for VAppTaskInProgress {
    fn get_task(&self) -> &ManagedObjectReference { &self.task }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn TaskInProgressTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::TaskInProgress => Some(from.as_any_ref().downcast_ref::<TaskInProgress>()?),
            StructType::VAppTaskInProgress => Some(from.as_any_ref().downcast_ref::<VAppTaskInProgress>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::TaskInProgress => Ok(from.as_any_box().downcast::<TaskInProgress>()?),
            StructType::VAppTaskInProgress => Ok(from.as_any_box().downcast::<VAppTaskInProgress>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
