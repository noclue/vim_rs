use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An InvalidDatastore exception is thrown if an operation fails because of a
/// problem with the specified datastore.
/// 
/// Typically, a subclass of this
/// exception is thrown, indicating a problem such as an inaccessible datastore
/// or an invalid datastore path.
pub trait InvalidDatastoreTrait : super::vim_fault_trait::VimFaultTrait {
    /// The datastore that is invalid.
    /// 
    /// Refers instance of *Datastore*.
    fn get_datastore(&self) -> &Option<ManagedObjectReference>;
    /// The name of the datastore that is invalid.
    fn get_name(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn InvalidDatastoreTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InvalidDatastoreTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InvalidDatastoreVisitor)
            }
        }

struct InvalidDatastoreVisitor;

impl<'de> de::Visitor<'de> for InvalidDatastoreVisitor {
    type Value = Box<dyn InvalidDatastoreTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InvalidDatastoreTrait JSON object with a _typeName field")
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

impl InvalidDatastoreTrait for InvalidDatastore {
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_name(&self) -> &Option<String> { &self.name }
}
impl InvalidDatastoreTrait for DatastoreNotWritableOnHost {
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_name(&self) -> &Option<String> { &self.name }
}
impl InvalidDatastoreTrait for SwapDatastoreNotWritableOnHost {
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_name(&self) -> &Option<String> { &self.name }
}
impl InvalidDatastoreTrait for InaccessibleDatastore {
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_name(&self) -> &Option<String> { &self.name }
}
impl InvalidDatastoreTrait for InaccessibleFtMetadataDatastore {
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_name(&self) -> &Option<String> { &self.name }
}
impl InvalidDatastoreTrait for InvalidDatastorePath {
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_name(&self) -> &Option<String> { &self.name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InvalidDatastoreTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidDatastore => Some(from.as_any_ref().downcast_ref::<InvalidDatastore>()?),
            StructType::DatastoreNotWritableOnHost => Some(from.as_any_ref().downcast_ref::<DatastoreNotWritableOnHost>()?),
            StructType::SwapDatastoreNotWritableOnHost => Some(from.as_any_ref().downcast_ref::<SwapDatastoreNotWritableOnHost>()?),
            StructType::InaccessibleDatastore => Some(from.as_any_ref().downcast_ref::<InaccessibleDatastore>()?),
            StructType::InaccessibleFtMetadataDatastore => Some(from.as_any_ref().downcast_ref::<InaccessibleFtMetadataDatastore>()?),
            StructType::InvalidDatastorePath => Some(from.as_any_ref().downcast_ref::<InvalidDatastorePath>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidDatastore => Ok(from.as_any_box().downcast::<InvalidDatastore>()?),
            StructType::DatastoreNotWritableOnHost => Ok(from.as_any_box().downcast::<DatastoreNotWritableOnHost>()?),
            StructType::SwapDatastoreNotWritableOnHost => Ok(from.as_any_box().downcast::<SwapDatastoreNotWritableOnHost>()?),
            StructType::InaccessibleDatastore => Ok(from.as_any_box().downcast::<InaccessibleDatastore>()?),
            StructType::InaccessibleFtMetadataDatastore => Ok(from.as_any_box().downcast::<InaccessibleFtMetadataDatastore>()?),
            StructType::InvalidDatastorePath => Ok(from.as_any_box().downcast::<InvalidDatastorePath>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
