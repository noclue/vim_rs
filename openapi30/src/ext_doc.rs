use serde::{Deserialize, Serialize};
use indexmap::IndexMap;


// From https://spec.openapis.org/oas/v3.0.3#external-documentation-object
/// Allows referencing an external resource for extended documentation.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExternalDocumentation {
    /// A short description of the target documentation. CommonMark syntax MAY be used for rich text
    /// representation.
    pub description: Option<String>,
    /// The URL for the target documentation. Value MUST be in the format of a URL.
    pub url: String,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
