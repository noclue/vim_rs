// Auxiliary types supporting the OpenAPI Rust representation

use super::openapi::SchemaType;


#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("Schema of type {0:?} has invalid field: {1}")]
    SchemaInvalidField(SchemaType,String),
}

pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}