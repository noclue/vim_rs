use serde::{Deserialize, Serialize};

// From https://spec.openapis.org/oas/v3.0.3#reference-object
/// OpenAPI 3.0 on multiple places uses reference or actual object. This enum represents this
/// construct.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RefOr<T> {
    /// A simple object to allow referencing other components in the specification, internally and
    /// externally.
    ///
    /// The Reference Object is defined by JSON Reference and follows the same structure, behavior and
    /// rules.
    ///
    /// For this specification, reference resolution is accomplished as defined by the JSON Reference
    /// specification and not by the JSON Schema specification.
    ///
    /// This object cannot be extended with additional properties and any properties added SHALL be
    /// ignored.
    Ref{
        /// The reference string.
        #[serde(rename = "$ref")]
        reference: String,
    },
    /// A value
    Val(Box<T>),
}