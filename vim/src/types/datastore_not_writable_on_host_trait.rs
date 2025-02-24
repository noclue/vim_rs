use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This exception is thrown if a datastore is not
/// writable on the target host.
pub trait DatastoreNotWritableOnHostTrait : super::invalid_datastore_trait::InvalidDatastoreTrait {
    /// The target host on which the datastore is not writable.
    /// 
    /// Refers instance of *HostSystem*.
    fn get_host(&self) -> &ManagedObjectReference;
}
impl<'s> serde::Serialize for dyn DatastoreNotWritableOnHostTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DatastoreNotWritableOnHostTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DatastoreNotWritableOnHostVisitor)
            }
        }

struct DatastoreNotWritableOnHostVisitor;

impl<'de> de::Visitor<'de> for DatastoreNotWritableOnHostVisitor {
    type Value = Box<dyn DatastoreNotWritableOnHostTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DatastoreNotWritableOnHostTrait JSON object with a _typeName field")
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

impl DatastoreNotWritableOnHostTrait for DatastoreNotWritableOnHost {
    fn get_host(&self) -> &ManagedObjectReference { &self.host }
}
impl DatastoreNotWritableOnHostTrait for SwapDatastoreNotWritableOnHost {
    fn get_host(&self) -> &ManagedObjectReference { &self.host }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DatastoreNotWritableOnHostTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DatastoreNotWritableOnHost => Some(from.as_any_ref().downcast_ref::<DatastoreNotWritableOnHost>()?),
            StructType::SwapDatastoreNotWritableOnHost => Some(from.as_any_ref().downcast_ref::<SwapDatastoreNotWritableOnHost>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DatastoreNotWritableOnHost => Ok(from.as_any_box().downcast::<DatastoreNotWritableOnHost>()?),
            StructType::SwapDatastoreNotWritableOnHost => Ok(from.as_any_box().downcast::<SwapDatastoreNotWritableOnHost>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
