use super::boxed_types::ValueElements;
use super::vim_object_trait::VimObjectTrait;

/// Base type for all VIM objects. This is used in APIs that can return arbitrary type.
/// Structure data types are under the `Object` variant. Primitives and arrays are under the `Value`
/// variant.
///
/// `miniserde::Deserialize` impl is generated in `deserialize.rs`.
pub enum VimAny {
    Object(Box<dyn VimObjectTrait>),
    Value(ValueElements),
}

impl miniserde::Serialize for VimAny {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        match self {
            VimAny::Object(obj) => obj.begin(),
            VimAny::Value(val) => val.begin(),
        }
    }
}

#[cfg(feature = "defaults")]
impl Default for VimAny {
    fn default() -> Self {
        VimAny::Value(ValueElements::PrimitiveString(String::new()))
    }
}

impl std::fmt::Debug for VimAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VimAny::Object(obj) => write!(f, "VimAny::Object({:?})", obj),
            VimAny::Value(val) => write!(f, "VimAny::Value({:?})", val),
        }
    }
}
