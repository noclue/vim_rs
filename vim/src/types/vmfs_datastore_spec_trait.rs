use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for VMFS datastore addition specification.
/// 
/// Used as a generic
/// way to point to one of the creation specifications that can be used to
/// apply a specification to effect the creation or extension of a VMFS
/// datastore.
pub trait VmfsDatastoreSpecTrait : super::data_object_trait::DataObjectTrait {
    /// The UUID of the SCSI disk on which the VMFS datastore is located.
    /// 
    /// See also *HostScsiDisk*, *ScsiLun.uuid*.
    fn get_disk_uuid(&self) -> &str;
}
impl<'s> serde::Serialize for dyn VmfsDatastoreSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmfsDatastoreSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmfsDatastoreSpecVisitor)
            }
        }

struct VmfsDatastoreSpecVisitor;

impl<'de> de::Visitor<'de> for VmfsDatastoreSpecVisitor {
    type Value = Box<dyn VmfsDatastoreSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmfsDatastoreSpecTrait JSON object with a _typeName field")
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

impl VmfsDatastoreSpecTrait for VmfsDatastoreSpec {
    fn get_disk_uuid(&self) -> &str { &self.disk_uuid }
}
impl VmfsDatastoreSpecTrait for VmfsDatastoreCreateSpec {
    fn get_disk_uuid(&self) -> &str { &self.disk_uuid }
}
impl VmfsDatastoreSpecTrait for VmfsDatastoreExpandSpec {
    fn get_disk_uuid(&self) -> &str { &self.disk_uuid }
}
impl VmfsDatastoreSpecTrait for VmfsDatastoreExtendSpec {
    fn get_disk_uuid(&self) -> &str { &self.disk_uuid }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmfsDatastoreSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmfsDatastoreSpec => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreSpec>()?),
            StructType::VmfsDatastoreCreateSpec => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreCreateSpec>()?),
            StructType::VmfsDatastoreExpandSpec => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreExpandSpec>()?),
            StructType::VmfsDatastoreExtendSpec => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreExtendSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmfsDatastoreSpec => Ok(from.as_any_box().downcast::<VmfsDatastoreSpec>()?),
            StructType::VmfsDatastoreCreateSpec => Ok(from.as_any_box().downcast::<VmfsDatastoreCreateSpec>()?),
            StructType::VmfsDatastoreExpandSpec => Ok(from.as_any_box().downcast::<VmfsDatastoreExpandSpec>()?),
            StructType::VmfsDatastoreExtendSpec => Ok(from.as_any_box().downcast::<VmfsDatastoreExtendSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
