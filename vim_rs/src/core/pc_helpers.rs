use log::error;
use super::super::types::vim_any::VimAny;
use super::client;
use thiserror::Error;
use crate::types::enums::MoTypesEnum;
use crate::types::structs::{ManagedObjectReference, ObjectSpec, PropertySpec, TraversalSpec};

/// Trait for errors that can be properly boxed and sent across threads
pub trait BoxableError: std::error::Error + Send + Sync + 'static {}

// Blanket implementation for all types that satisfy the requirements
impl<E: std::error::Error + Send + Sync + 'static> BoxableError for E {}

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
    #[error("Internal Error: {0}")]
    InternalError(String),
    #[error("Remote call failure: {0:?}")]
    RemoteCommunicationError(#[from] client::Error),
    #[error("Unexpected property path = `{0}`")]
    UnexpectedPropertyPath(String),
    #[error("Generic Erorr '{0}'")]
    GenericError(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Lock poisoned: {0}")]
    PoisonError(String),
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

/// A trait for objects that can be queried using the PropertyCollector utilities. These objects
/// provide a `PropertySpec` for the object type.
pub trait Queriable {
    /// The property spec for this object type.
    fn prop_spec() -> PropertySpec;
}


type StaticStr = &'static str;

/// Create an ObjectSpec for a view. This is used to specify objects to be monitored from a view.
pub(crate) fn obj_spec_for_view(view_moref: ManagedObjectReference) -> Vec<ObjectSpec> {
    vec![ObjectSpec {
        obj: view_moref,
        skip: Some(false),
        select_set: Some(vec![Box::new(TraversalSpec {
            name: Some("traverseEntities".to_string()),
            r#type: StaticStr::from(MoTypesEnum::ContainerView).to_string(),
            path: "view".to_string(),
            skip: Some(false),
            select_set: None,
        })]),
    }]
}

