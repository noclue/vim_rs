use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These are events used to describe migration warning and errors
pub trait MigrationEventTrait : super::vm_event_trait::VmEventTrait {
    /// The fault that describes the migration issue.
    /// 
    /// This is typically either a
    /// MigrationFault or a VmConfigFault.
    fn get_fault(&self) -> &Box<dyn super::method_fault_trait::MethodFaultTrait>;
}
impl<'s> serde::Serialize for dyn MigrationEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn MigrationEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(MigrationEventVisitor)
            }
        }

struct MigrationEventVisitor;

impl<'de> de::Visitor<'de> for MigrationEventVisitor {
    type Value = Box<dyn MigrationEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid MigrationEventTrait JSON object with a _typeName field")
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

impl MigrationEventTrait for MigrationEvent {
    fn get_fault(&self) -> &Box<dyn super::method_fault_trait::MethodFaultTrait> { &self.fault }
}
impl MigrationEventTrait for MigrationErrorEvent {
    fn get_fault(&self) -> &Box<dyn super::method_fault_trait::MethodFaultTrait> { &self.fault }
}
impl MigrationEventTrait for MigrationHostErrorEvent {
    fn get_fault(&self) -> &Box<dyn super::method_fault_trait::MethodFaultTrait> { &self.fault }
}
impl MigrationEventTrait for MigrationHostWarningEvent {
    fn get_fault(&self) -> &Box<dyn super::method_fault_trait::MethodFaultTrait> { &self.fault }
}
impl MigrationEventTrait for MigrationResourceErrorEvent {
    fn get_fault(&self) -> &Box<dyn super::method_fault_trait::MethodFaultTrait> { &self.fault }
}
impl MigrationEventTrait for MigrationResourceWarningEvent {
    fn get_fault(&self) -> &Box<dyn super::method_fault_trait::MethodFaultTrait> { &self.fault }
}
impl MigrationEventTrait for MigrationWarningEvent {
    fn get_fault(&self) -> &Box<dyn super::method_fault_trait::MethodFaultTrait> { &self.fault }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn MigrationEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::MigrationEvent => Some(from.as_any_ref().downcast_ref::<MigrationEvent>()?),
            StructType::MigrationErrorEvent => Some(from.as_any_ref().downcast_ref::<MigrationErrorEvent>()?),
            StructType::MigrationHostErrorEvent => Some(from.as_any_ref().downcast_ref::<MigrationHostErrorEvent>()?),
            StructType::MigrationHostWarningEvent => Some(from.as_any_ref().downcast_ref::<MigrationHostWarningEvent>()?),
            StructType::MigrationResourceErrorEvent => Some(from.as_any_ref().downcast_ref::<MigrationResourceErrorEvent>()?),
            StructType::MigrationResourceWarningEvent => Some(from.as_any_ref().downcast_ref::<MigrationResourceWarningEvent>()?),
            StructType::MigrationWarningEvent => Some(from.as_any_ref().downcast_ref::<MigrationWarningEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::MigrationEvent => Ok(from.as_any_box().downcast::<MigrationEvent>()?),
            StructType::MigrationErrorEvent => Ok(from.as_any_box().downcast::<MigrationErrorEvent>()?),
            StructType::MigrationHostErrorEvent => Ok(from.as_any_box().downcast::<MigrationHostErrorEvent>()?),
            StructType::MigrationHostWarningEvent => Ok(from.as_any_box().downcast::<MigrationHostWarningEvent>()?),
            StructType::MigrationResourceErrorEvent => Ok(from.as_any_box().downcast::<MigrationResourceErrorEvent>()?),
            StructType::MigrationResourceWarningEvent => Ok(from.as_any_box().downcast::<MigrationResourceWarningEvent>()?),
            StructType::MigrationWarningEvent => Ok(from.as_any_box().downcast::<MigrationWarningEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
