use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Specification of the backing of a virtual
/// storage object.
pub trait VslmCreateSpecBackingSpecTrait : super::data_object_trait::DataObjectTrait {
    /// The datastore managed object where this backing is located.
    /// 
    /// Refers instance of *Datastore*.
    fn get_datastore(&self) -> &ManagedObjectReference;
    /// Relative location in the specified datastore where disk needs to be
    /// created.
    /// 
    /// If not specified disk gets created at the defualt
    /// VStorageObject location on the specified datastore.
    fn get_path(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn VslmCreateSpecBackingSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VslmCreateSpecBackingSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VslmCreateSpecBackingSpecVisitor)
            }
        }

struct VslmCreateSpecBackingSpecVisitor;

impl<'de> de::Visitor<'de> for VslmCreateSpecBackingSpecVisitor {
    type Value = Box<dyn VslmCreateSpecBackingSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VslmCreateSpecBackingSpecTrait JSON object with a _typeName field")
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

impl VslmCreateSpecBackingSpecTrait for VslmCreateSpecBackingSpec {
    fn get_datastore(&self) -> &ManagedObjectReference { &self.datastore }
    fn get_path(&self) -> &Option<String> { &self.path }
}
impl VslmCreateSpecBackingSpecTrait for VslmCreateSpecDiskFileBackingSpec {
    fn get_datastore(&self) -> &ManagedObjectReference { &self.datastore }
    fn get_path(&self) -> &Option<String> { &self.path }
}
impl VslmCreateSpecBackingSpecTrait for VslmCreateSpecRawDiskMappingBackingSpec {
    fn get_datastore(&self) -> &ManagedObjectReference { &self.datastore }
    fn get_path(&self) -> &Option<String> { &self.path }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VslmCreateSpecBackingSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VslmCreateSpecBackingSpec => Some(from.as_any_ref().downcast_ref::<VslmCreateSpecBackingSpec>()?),
            StructType::VslmCreateSpecDiskFileBackingSpec => Some(from.as_any_ref().downcast_ref::<VslmCreateSpecDiskFileBackingSpec>()?),
            StructType::VslmCreateSpecRawDiskMappingBackingSpec => Some(from.as_any_ref().downcast_ref::<VslmCreateSpecRawDiskMappingBackingSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VslmCreateSpecBackingSpec => Ok(from.as_any_box().downcast::<VslmCreateSpecBackingSpec>()?),
            StructType::VslmCreateSpecDiskFileBackingSpec => Ok(from.as_any_box().downcast::<VslmCreateSpecDiskFileBackingSpec>()?),
            StructType::VslmCreateSpecRawDiskMappingBackingSpec => Ok(from.as_any_box().downcast::<VslmCreateSpecRawDiskMappingBackingSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
