use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An InaccessibleDatastore exception is thrown if the datastore corresponding to the
/// given datastore path isn't currently accessible.
pub trait InaccessibleDatastoreTrait : super::invalid_datastore_trait::InvalidDatastoreTrait {
    fn get_detail(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn InaccessibleDatastoreTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InaccessibleDatastoreTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InaccessibleDatastoreVisitor)
            }
        }

struct InaccessibleDatastoreVisitor;

impl<'de> de::Visitor<'de> for InaccessibleDatastoreVisitor {
    type Value = Box<dyn InaccessibleDatastoreTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InaccessibleDatastoreTrait JSON object with a _typeName field")
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

impl InaccessibleDatastoreTrait for InaccessibleDatastore {
    fn get_detail(&self) -> &Option<String> { &self.detail }
}
impl InaccessibleDatastoreTrait for InaccessibleFtMetadataDatastore {
    fn get_detail(&self) -> &Option<String> { &self.detail }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InaccessibleDatastoreTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InaccessibleDatastore => Some(from.as_any_ref().downcast_ref::<InaccessibleDatastore>()?),
            StructType::InaccessibleFtMetadataDatastore => Some(from.as_any_ref().downcast_ref::<InaccessibleFtMetadataDatastore>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InaccessibleDatastore => Ok(from.as_any_box().downcast::<InaccessibleDatastore>()?),
            StructType::InaccessibleFtMetadataDatastore => Ok(from.as_any_box().downcast::<InaccessibleFtMetadataDatastore>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
