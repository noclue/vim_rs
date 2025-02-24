use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The virtual machine is not supported on the target datastore.
/// 
/// This fault is
/// thrown by provisioning operations when an attempt is made to create a virtual
/// machine on an unsupported datastore (for example, creating a non-legacy
/// virtual machine on a legacy datastore).
pub trait UnsupportedDatastoreTrait : super::vm_config_fault_trait::VmConfigFaultTrait {
    /// The invalid datastore for this virtual machine.
    /// 
    /// Refers instance of *Datastore*.
    fn get_datastore(&self) -> &Option<ManagedObjectReference>;
}
impl<'s> serde::Serialize for dyn UnsupportedDatastoreTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn UnsupportedDatastoreTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(UnsupportedDatastoreVisitor)
            }
        }

struct UnsupportedDatastoreVisitor;

impl<'de> de::Visitor<'de> for UnsupportedDatastoreVisitor {
    type Value = Box<dyn UnsupportedDatastoreTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid UnsupportedDatastoreTrait JSON object with a _typeName field")
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

impl UnsupportedDatastoreTrait for UnsupportedDatastore {
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
}
impl UnsupportedDatastoreTrait for MemoryFileFormatNotSupportedByDatastore {
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
}
impl UnsupportedDatastoreTrait for UnSupportedDatastoreForVFlash {
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn UnsupportedDatastoreTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::UnsupportedDatastore => Some(from.as_any_ref().downcast_ref::<UnsupportedDatastore>()?),
            StructType::MemoryFileFormatNotSupportedByDatastore => Some(from.as_any_ref().downcast_ref::<MemoryFileFormatNotSupportedByDatastore>()?),
            StructType::UnSupportedDatastoreForVFlash => Some(from.as_any_ref().downcast_ref::<UnSupportedDatastoreForVFlash>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::UnsupportedDatastore => Ok(from.as_any_box().downcast::<UnsupportedDatastore>()?),
            StructType::MemoryFileFormatNotSupportedByDatastore => Ok(from.as_any_box().downcast::<MemoryFileFormatNotSupportedByDatastore>()?),
            StructType::UnSupportedDatastoreForVFlash => Ok(from.as_any_box().downcast::<UnSupportedDatastoreForVFlash>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
