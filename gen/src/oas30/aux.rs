// Auxiliary types supporting the OpenAPI Rust representation

use std::fmt::Debug;

use super::schema::SchemaType;


#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("Schema of type {0:?} has invalid field: {1}")]
    SchemaInvalidField(SchemaType,String),
    #[error("Enum value at index {index} has value of unexpected type: {value:?}")]
    SchemaInvalidEnumValue {
        index: usize,
        value: String,
    }
}

pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}