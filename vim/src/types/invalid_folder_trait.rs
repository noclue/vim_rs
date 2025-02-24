use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An InvalidFolderFault exception is thrown when a
/// node is moved to an invalid place in the hierarchy.
/// 
/// This can be because it is a child of the current node,
/// or a wrong kind of container.
pub trait InvalidFolderTrait : super::vim_fault_trait::VimFaultTrait {
    /// Reference to invalid target.
    /// 
    /// Refers instance of *ManagedEntity*.
    fn get_target(&self) -> &ManagedObjectReference;
}
impl<'s> serde::Serialize for dyn InvalidFolderTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InvalidFolderTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InvalidFolderVisitor)
            }
        }

struct InvalidFolderVisitor;

impl<'de> de::Visitor<'de> for InvalidFolderVisitor {
    type Value = Box<dyn InvalidFolderTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InvalidFolderTrait JSON object with a _typeName field")
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

impl InvalidFolderTrait for InvalidFolder {
    fn get_target(&self) -> &ManagedObjectReference { &self.target }
}
impl InvalidFolderTrait for VmAlreadyExistsInDatacenter {
    fn get_target(&self) -> &ManagedObjectReference { &self.target }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InvalidFolderTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidFolder => Some(from.as_any_ref().downcast_ref::<InvalidFolder>()?),
            StructType::VmAlreadyExistsInDatacenter => Some(from.as_any_ref().downcast_ref::<VmAlreadyExistsInDatacenter>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidFolder => Ok(from.as_any_box().downcast::<InvalidFolder>()?),
            StructType::VmAlreadyExistsInDatacenter => Ok(from.as_any_box().downcast::<VmAlreadyExistsInDatacenter>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
