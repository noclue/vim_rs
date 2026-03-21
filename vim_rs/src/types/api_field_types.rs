//! Field type metadata for schema-guided deserialization (XML feature).

#![cfg(feature = "xml")]

use super::struct_enum::StructType;

/// Primitive and composite field type descriptors for typed deserialization.
/// Used to guide XML (and potentially other) deserializers that lack
/// self-describing type information in the wire format.
///
/// All references are `'static` since they point into generated constant data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiFieldType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    /// String, DateTime, and enum types
    Str,
    /// Base64-encoded binary
    Binary,
    /// Nested struct; polymorphism via `_typeName` / `xsi:type`
    Object(StructType),
    /// Element type (static ref to avoid alloc)
    Array(&'static ApiFieldType),
    /// VimAny — polymorphic, resolved by xsi:type
    Any,
}

/// Per-type metadata: own fields + optional parent link for chain lookup.
#[derive(Clone, Copy, Debug)]
pub struct ApiTypeInfo {
    pub parent: Option<StructType>,
    pub fields: &'static [(&'static str, ApiFieldType)],
}
