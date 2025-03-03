use super::boxed_types::ValueElements;
use super::deserialize::VimAnyVisitor;
use super::vim_object_trait::VimObjectTrait;

/// Base type for all VIM objects. This is used in APIs that can return arbitrary type.
/// Structure data types are under the `Object` variant. Primitives and arrays are under the `Value`
/// variant.
#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum VimAny {
    Object(Box<dyn VimObjectTrait>),
    Value(ValueElements),
    //Unknown(serde_json::Value),
}

impl<'de> serde::Deserialize<'de> for VimAny {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(VimAnyVisitor)
    }
}