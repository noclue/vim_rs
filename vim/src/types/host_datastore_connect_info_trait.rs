use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The base data object type for information about datastores on the host.
pub trait HostDatastoreConnectInfoTrait : super::data_object_trait::DataObjectTrait {
    /// Basic datastore information.
    /// 
    /// The managed object reference is not set.
    fn get_summary(&self) -> &DatastoreSummary;
}
impl<'s> serde::Serialize for dyn HostDatastoreConnectInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostDatastoreConnectInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostDatastoreConnectInfoVisitor)
            }
        }

struct HostDatastoreConnectInfoVisitor;

impl<'de> de::Visitor<'de> for HostDatastoreConnectInfoVisitor {
    type Value = Box<dyn HostDatastoreConnectInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostDatastoreConnectInfoTrait JSON object with a _typeName field")
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

impl HostDatastoreConnectInfoTrait for HostDatastoreConnectInfo {
    fn get_summary(&self) -> &DatastoreSummary { &self.summary }
}
impl HostDatastoreConnectInfoTrait for HostDatastoreExistsConnectInfo {
    fn get_summary(&self) -> &DatastoreSummary { &self.summary }
}
impl HostDatastoreConnectInfoTrait for HostDatastoreNameConflictConnectInfo {
    fn get_summary(&self) -> &DatastoreSummary { &self.summary }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostDatastoreConnectInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostDatastoreConnectInfo => Some(from.as_any_ref().downcast_ref::<HostDatastoreConnectInfo>()?),
            StructType::HostDatastoreExistsConnectInfo => Some(from.as_any_ref().downcast_ref::<HostDatastoreExistsConnectInfo>()?),
            StructType::HostDatastoreNameConflictConnectInfo => Some(from.as_any_ref().downcast_ref::<HostDatastoreNameConflictConnectInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostDatastoreConnectInfo => Ok(from.as_any_box().downcast::<HostDatastoreConnectInfo>()?),
            StructType::HostDatastoreExistsConnectInfo => Ok(from.as_any_box().downcast::<HostDatastoreExistsConnectInfo>()?),
            StructType::HostDatastoreNameConflictConnectInfo => Ok(from.as_any_box().downcast::<HostDatastoreNameConflictConnectInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
