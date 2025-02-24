use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for events related to datastore file and directory
/// operations.
/// 
/// Property _datastore_ inherited from DatastoreEvent refers
/// to the destination datastore in case there is more than datastore
/// involved in the operation.
pub trait DatastoreFileEventTrait : super::datastore_event_trait::DatastoreEventTrait {
    /// Datastore path of the target file or directory.
    fn get_target_file(&self) -> &str;
    /// Identifier of the initiator of the file operation.
    fn get_source_of_operation(&self) -> &Option<String>;
    /// Indicator whether the datastore file operation succeeded.
    fn get_succeeded(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn DatastoreFileEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DatastoreFileEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DatastoreFileEventVisitor)
            }
        }

struct DatastoreFileEventVisitor;

impl<'de> de::Visitor<'de> for DatastoreFileEventVisitor {
    type Value = Box<dyn DatastoreFileEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DatastoreFileEventTrait JSON object with a _typeName field")
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

impl DatastoreFileEventTrait for DatastoreFileEvent {
    fn get_target_file(&self) -> &str { &self.target_file }
    fn get_source_of_operation(&self) -> &Option<String> { &self.source_of_operation }
    fn get_succeeded(&self) -> Option<bool> { self.succeeded }
}
impl DatastoreFileEventTrait for DatastoreFileCopiedEvent {
    fn get_target_file(&self) -> &str { &self.target_file }
    fn get_source_of_operation(&self) -> &Option<String> { &self.source_of_operation }
    fn get_succeeded(&self) -> Option<bool> { self.succeeded }
}
impl DatastoreFileEventTrait for DatastoreFileDeletedEvent {
    fn get_target_file(&self) -> &str { &self.target_file }
    fn get_source_of_operation(&self) -> &Option<String> { &self.source_of_operation }
    fn get_succeeded(&self) -> Option<bool> { self.succeeded }
}
impl DatastoreFileEventTrait for DatastoreFileMovedEvent {
    fn get_target_file(&self) -> &str { &self.target_file }
    fn get_source_of_operation(&self) -> &Option<String> { &self.source_of_operation }
    fn get_succeeded(&self) -> Option<bool> { self.succeeded }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DatastoreFileEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DatastoreFileEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileEvent>()?),
            StructType::DatastoreFileCopiedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileCopiedEvent>()?),
            StructType::DatastoreFileDeletedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileDeletedEvent>()?),
            StructType::DatastoreFileMovedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileMovedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DatastoreFileEvent => Ok(from.as_any_box().downcast::<DatastoreFileEvent>()?),
            StructType::DatastoreFileCopiedEvent => Ok(from.as_any_box().downcast::<DatastoreFileCopiedEvent>()?),
            StructType::DatastoreFileDeletedEvent => Ok(from.as_any_box().downcast::<DatastoreFileDeletedEvent>()?),
            StructType::DatastoreFileMovedEvent => Ok(from.as_any_box().downcast::<DatastoreFileMovedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
