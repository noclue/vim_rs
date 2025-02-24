use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *SelectionSpec* is the base type for data
/// object types that specify what additional objects to filter.
/// 
/// The base
/// type contains only an optional "name" field, which allows a selection to
/// be named for future reference. More information is available in the
/// subtype.
/// 
/// Named selections support recursive specifications on an object
/// hierarchy. When used by a derived object, the "name" field allows other
/// *SelectionSpec* objects to refer to the object by
/// name. When used as the base type only, the "name" field indicates
/// recursion to the derived object by name.
/// 
/// Names are meaningful only within the same FilterSpec.
pub trait SelectionSpecTrait : super::data_object_trait::DataObjectTrait {
    /// Name of the selection specification.
    fn get_name(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn SelectionSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn SelectionSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(SelectionSpecVisitor)
            }
        }

struct SelectionSpecVisitor;

impl<'de> de::Visitor<'de> for SelectionSpecVisitor {
    type Value = Box<dyn SelectionSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid SelectionSpecTrait JSON object with a _typeName field")
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

impl SelectionSpecTrait for SelectionSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
}
impl SelectionSpecTrait for TraversalSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn SelectionSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::SelectionSpec => Some(from.as_any_ref().downcast_ref::<SelectionSpec>()?),
            StructType::TraversalSpec => Some(from.as_any_ref().downcast_ref::<TraversalSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::SelectionSpec => Ok(from.as_any_box().downcast::<SelectionSpec>()?),
            StructType::TraversalSpec => Ok(from.as_any_box().downcast::<TraversalSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
