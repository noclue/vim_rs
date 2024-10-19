use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// From https://spec.openapis.org/oas/v3.0.3#server-object
/// An object representing a Server.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Server {
    /// A URL to the target host. This URL supports Server Variables and MAY be relative, to
    /// indicate that the host location is relative to the location where the OpenAPI document is
    /// being served. Variable substitutions will be made when a variable is named in {brackets}.
    pub url: String,
    /// An optional string describing the host designated by the URL. CommonMark syntax MAY be used
    /// for rich text representation.
    pub description: Option<String>,
    /// A map between a variable name and its value. The value is used for substitution in the
    /// server's URL template.
    pub variables: Option<HashMap<String, ServerVariable>>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#server-variable-object
/// An object representing a Server Variable for server URL template substitution.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ServerVariable {
    /// An enumeration of string values to be used if the substitution options are from a limited
    /// set. The array SHOULD NOT be empty.
    #[serde(rename = "enum")]
    pub enumeration: Option<Vec<String>>,
    /// The default value to use for substitution, which SHALL be sent if an alternate value is not
    /// supplied. Note this behavior is different than the Schema Object's treatment of default
    /// values, because in those cases parameter values are optional. If the enum is defined, the
    /// value SHOULD exist in the enum's values.
    pub default: String,
    /// An optional description for the server variable. CommonMark syntax MAY be used for rich text
    /// representation.
    pub description: Option<String>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: HashMap<String, serde_json::Value>,
}