use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These are datastore events.
pub trait DatastoreEventTrait : super::event_trait::EventTrait {
    /// The associated datastore.
    fn get_datastore(&self) -> &Option<DatastoreEventArgument>;
}
impl<'s> serde::Serialize for dyn DatastoreEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DatastoreEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DatastoreEventVisitor)
            }
        }

struct DatastoreEventVisitor;

impl<'de> de::Visitor<'de> for DatastoreEventVisitor {
    type Value = Box<dyn DatastoreEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DatastoreEventTrait JSON object with a _typeName field")
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

impl DatastoreEventTrait for DatastoreEvent {
    fn get_datastore(&self) -> &Option<DatastoreEventArgument> { &self.datastore }
}
impl DatastoreEventTrait for DatastoreCapacityIncreasedEvent {
    fn get_datastore(&self) -> &Option<DatastoreEventArgument> { &self.datastore }
}
impl DatastoreEventTrait for DatastoreDestroyedEvent {
    fn get_datastore(&self) -> &Option<DatastoreEventArgument> { &self.datastore }
}
impl DatastoreEventTrait for DatastoreDuplicatedEvent {
    fn get_datastore(&self) -> &Option<DatastoreEventArgument> { &self.datastore }
}
impl DatastoreEventTrait for DatastoreFileEvent {
    fn get_datastore(&self) -> &Option<DatastoreEventArgument> { &self.datastore }
}
impl DatastoreEventTrait for DatastoreFileCopiedEvent {
    fn get_datastore(&self) -> &Option<DatastoreEventArgument> { &self.datastore }
}
impl DatastoreEventTrait for DatastoreFileDeletedEvent {
    fn get_datastore(&self) -> &Option<DatastoreEventArgument> { &self.datastore }
}
impl DatastoreEventTrait for DatastoreFileMovedEvent {
    fn get_datastore(&self) -> &Option<DatastoreEventArgument> { &self.datastore }
}
impl DatastoreEventTrait for DatastoreIormReconfiguredEvent {
    fn get_datastore(&self) -> &Option<DatastoreEventArgument> { &self.datastore }
}
impl DatastoreEventTrait for DatastoreRenamedEvent {
    fn get_datastore(&self) -> &Option<DatastoreEventArgument> { &self.datastore }
}
impl DatastoreEventTrait for NonViWorkloadDetectedOnDatastoreEvent {
    fn get_datastore(&self) -> &Option<DatastoreEventArgument> { &self.datastore }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DatastoreEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DatastoreEvent => Some(from.as_any_ref().downcast_ref::<DatastoreEvent>()?),
            StructType::DatastoreCapacityIncreasedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreCapacityIncreasedEvent>()?),
            StructType::DatastoreDestroyedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreDestroyedEvent>()?),
            StructType::DatastoreDuplicatedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreDuplicatedEvent>()?),
            StructType::DatastoreFileEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileEvent>()?),
            StructType::DatastoreFileCopiedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileCopiedEvent>()?),
            StructType::DatastoreFileDeletedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileDeletedEvent>()?),
            StructType::DatastoreFileMovedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileMovedEvent>()?),
            StructType::DatastoreIormReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DatastoreIormReconfiguredEvent>()?),
            StructType::DatastoreRenamedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreRenamedEvent>()?),
            StructType::NonViWorkloadDetectedOnDatastoreEvent => Some(from.as_any_ref().downcast_ref::<NonViWorkloadDetectedOnDatastoreEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DatastoreEvent => Ok(from.as_any_box().downcast::<DatastoreEvent>()?),
            StructType::DatastoreCapacityIncreasedEvent => Ok(from.as_any_box().downcast::<DatastoreCapacityIncreasedEvent>()?),
            StructType::DatastoreDestroyedEvent => Ok(from.as_any_box().downcast::<DatastoreDestroyedEvent>()?),
            StructType::DatastoreDuplicatedEvent => Ok(from.as_any_box().downcast::<DatastoreDuplicatedEvent>()?),
            StructType::DatastoreFileEvent => Ok(from.as_any_box().downcast::<DatastoreFileEvent>()?),
            StructType::DatastoreFileCopiedEvent => Ok(from.as_any_box().downcast::<DatastoreFileCopiedEvent>()?),
            StructType::DatastoreFileDeletedEvent => Ok(from.as_any_box().downcast::<DatastoreFileDeletedEvent>()?),
            StructType::DatastoreFileMovedEvent => Ok(from.as_any_box().downcast::<DatastoreFileMovedEvent>()?),
            StructType::DatastoreIormReconfiguredEvent => Ok(from.as_any_box().downcast::<DatastoreIormReconfiguredEvent>()?),
            StructType::DatastoreRenamedEvent => Ok(from.as_any_box().downcast::<DatastoreRenamedEvent>()?),
            StructType::NonViWorkloadDetectedOnDatastoreEvent => Ok(from.as_any_box().downcast::<NonViWorkloadDetectedOnDatastoreEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
