use super::super::types::vim_any::VimAny;
use thiserror::Error;

/// Error type for Unmarshalling PropertyCollector data into a Rust struct. This is used whenever
/// the returned data does not match the expected type.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid data type for property {property}. Expected `{expected}` got '{got}'")]
    InvalidPropertyType{property: String, expected: String, got: String},
    #[error("Received None for required field '{0}'")]
    NoneValueForRequiredField(String),
    #[error("No data found in ObjectUpdate/ObjectContent")]
    NoDataFound,
    #[error("Unexpected property path = `{0}`")]
    UnexpectedPropertyPath(String),
}

pub type Result<T> = std::result::Result<T, Error>;


/// Get the type name from a VimAny value. This is used for error reporting.
pub fn type_name(value :&VimAny) -> String {
    match value {
        VimAny::Value(value) => {
            let type_name : &'static str = value.into();
            type_name.to_string()
        },
        VimAny::Object(obj) => {
            let type_name : &'static str = obj.data_type().into();
            type_name.to_string()
        }
    }
}