//! Declared root/API shape for XML deserialization (`feature = "xml"`).
//!
//! [`DataTypeAware::data_type`] supplies the [`ApiFieldType`] that matches the VIM
//! wire contract for this Rust type. Implementations are generated for API structs
//! and enums; see `data_type_aware_impl.rs` (generated).

#![cfg(feature = "xml")]

use super::api_field_types::ApiFieldType;
use super::mini_helpers::Base64;

/// Associates a Rust binding type with its VIM [`ApiFieldType`] for schema-guided XML.
///
/// Required at XML unmarshalling roots (`from_xml`, `unmarshal` with SOAP) so the
/// parser does not infer shape from the wire alone.
pub trait DataTypeAware {
    fn data_type() -> ApiFieldType;
}

macro_rules! impl_int_data_type {
    ($($t:ty => $v:ident),+ $(,)?) => {
        $(
            impl DataTypeAware for $t {
                fn data_type() -> ApiFieldType {
                    ApiFieldType::$v
                }
            }
        )+
    };
}

impl_int_data_type! {
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
}

impl DataTypeAware for bool {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Bool
    }
}

impl DataTypeAware for String {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for f32 {
    fn data_type() -> ApiFieldType {
        ApiFieldType::F32
    }
}

impl DataTypeAware for f64 {
    fn data_type() -> ApiFieldType {
        ApiFieldType::F64
    }
}

impl DataTypeAware for Base64 {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Binary
    }
}

impl DataTypeAware for Vec<u8> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Binary
    }
}
