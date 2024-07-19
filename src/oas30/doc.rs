use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#example-object
/// Each Example Object provides an example of an instance for an schema.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Example {
    /// Short description for the example.
    pub summary: Option<String>,
    /// Long description for the example. CommonMark syntax MAY be used for rich text
    /// representation.
    pub description: Option<String>,
    /// Embedded literal example. The value field and externalValue field are mutually exclusive. To
    /// represent examples of media types that cannot naturally represented in JSON or YAML, use a
    /// string value to contain the example, escaping where necessary.
    pub value: Option<serde_json::Value>,
    /// A URL that points to the literal example. This provides the capability to reference examples
    /// that cannot easily be included in JSON or YAML documents. The value field and externalValue
    /// field are mutually exclusive.
    #[serde(rename = "externalValue")]
    pub external_value: Option<String>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: HashMap<String, serde_json::Value>,
}
