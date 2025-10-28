use super::super::types::vim_any::VimAny;
use crate::types::structs::{ManagedObjectReference, ObjectSpec, PropertySpec, TraversalSpec};

/// Trait for errors that can be properly boxed and sent across threads
pub trait BoxableError: std::error::Error + Send + Sync + 'static {}

// Blanket implementation for all types that satisfy the requirements
impl<E: std::error::Error + Send + Sync + 'static> BoxableError for E {}

// Re-export the unified error types for backwards compatibility within this module
pub use super::error::{Error, Result};

/// Get the type name from a VimAny value. This is used for error reporting.
pub fn type_name(value: &VimAny) -> String {
    match value {
        VimAny::Value(value) => {
            let type_name: &'static str = value.into();
            type_name.to_string()
        }
        VimAny::Object(obj) => {
            let type_name: &'static str = obj.data_type().into();
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
    let r#type = view_moref.r#type.clone();
    vec![ObjectSpec {
        data_object_: DataObject {},
        obj: view_moref,
        skip: Some(false),
        select_set: Some(vec![Box::new(TraversalSpec {
            selection_spec_: SelectionSpec {
                data_object_: DataObject {},
                name: Some("traverseEntities".to_string()),
            },
            r#type: StaticStr::from(r#type).to_string(),
            path: "view".to_string(),
            skip: Some(false),
            select_set: None,
        })]),
    }]
}
