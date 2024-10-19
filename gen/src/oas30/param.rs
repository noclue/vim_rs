use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use super::*;

// From https://spec.openapis.org/oas/v3.0.3#parameter-object
/// Describes a single operation parameter.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    /// The name of the parameter. Parameter names are case sensitive.
    pub name: String,
    /// The location of the parameter. Possible values are "query", "header", "path" or "cookie".
    #[serde(rename = "in")]
    pub location: Location,
    /// A brief description of the parameter. This could contain examples of use. CommonMark syntax
    /// MAY be used for rich text representation.
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory. If the parameter location is "path", this
    /// property is REQUIRED and its value MUST be true. Otherwise, the property MAY be included and
    /// its default value is false.
    pub required: Option<bool>,
    /// Specifies that a parameter is deprecated and SHOULD be transitioned out of usage.
    pub deprecated: Option<bool>,
    /// Sets the ability to pass empty-valued parameters. This is valid only for query parameters
    /// and allows sending a parameter with an empty value.
    pub allow_empty_value: Option<bool>,
    /// Describes how the parameter value will be serialized depending on the type of the parameter
    /// value. Default values (based on value of in): for query - form; for path - simple; for
    /// header - simple; for cookie - form.
    #[serde(rename = "style")]
    pub serialization_style: Option<Style>,
    /// When this is true, parameter values of type array or object generate separate parameters for
    /// each value of the array or key-value pair of the map. For other types of parameters this
    /// property has no effect. When style is form, the default value is true. For all other styles,
    /// the default value is false.
    #[serde(rename = "explode")]
    pub explode_values: Option<bool>,
    /// Determines whether the parameter value SHOULD allow reserved characters, as defined by
    /// RFC3986 :/?#[]@!$&'()*+,;= to be included without percent-encoding. The default value is
    /// false.
    #[serde(rename = "allowReserved")]
    pub allow_reserved: Option<bool>,
    /// The schema defining the type used for the parameter.
    pub schema: Option<RefOr<Schema>>,
    /// Example of the parameter's potential value. The example SHOULD match the specified schema
    /// and encoding properties if present. The example field is mutually exclusive of the examples
    /// field. Furthermore, if referencing a schema which contains an example, the example value
    /// SHALL override the example provided by the schema.
    pub example: Option<serde_json::Value>,
    /// Examples of the parameter
    pub examples: Option<IndexMap<String, RefOr<Example>>>,
    /// A map containing the representations for the parameter. The key is the media type and the
    /// value describes it. The map MUST only contain one entry. This property is REQUIRED when the
    /// requestBody property is in use and the media type of the request is not
    /// application/x-www-form-urlencoded. The map key MUST be the media type and the value MUST be
    /// a MediaType Object.
    pub content: Option<IndexMap<String, MediaType>>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}



