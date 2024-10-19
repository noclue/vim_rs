use indexmap::IndexMap;
use serde::{Deserialize, Serialize};


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
    pub extensions: IndexMap<String, serde_json::Value>,
}