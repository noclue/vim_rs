use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use super::*;



// From https://spec.openapis.org/oas/v3.0.3#media-type-object
/// Each Media Type Object provides schema and examples for the media type identified by its key.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MediaType {
    /// The schema defining the content of the request, response, or parameter.
    pub schema: Option<RefOr<Schema>>,
    /// Example of the media type. The example object SHOULD be in the correct format as specified
    /// by the media type. The example field is mutually exclusive of the examples field.
    /// Furthermore, if referencing a schema which contains an example, the example value SHALL
    /// override the example provided by the schema.
    pub example: Option<serde_json::Value>,
    /// Examples of the media type
    pub examples: Option<IndexMap<String, RefOr<Example>>>,
    /// A map between a property name and its encoding information. The key, being the property
    /// name, MUST exist in the schema as a property. The encoding object SHALL only apply to
    /// requestBody objects when the media type is multipart or application/x-www-form-urlencoded.
    pub encoding: Option<IndexMap<String, Encoding>>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#encoding-object 
// It is only used in MediaType so no separate file is created for it
/// A single encoding definition applied to a single schema property.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Encoding {
    /// The Content-Type for encoding a specific property.
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    /// A map allowing additional information to be provided as headers, for example
    /// Content-Disposition. Content-Type is described separately and SHALL be ignored in this
    /// section. This property SHALL be ignored if the request body media type is not a multipart.
    pub headers: Option<IndexMap<String, RefOr<Header>>>,
    /// Describes how a specific property value will be serialized depending on its type.
    pub style: Option<Style>,
    /// When this is true, property values of type array or object generate separate parameters for
    /// each value of the array, or key-value pair of the map. For other types of properties this
    /// property has no effect. When style is form, the default value is true. For all other styles,
    /// the default value is false.
    pub explode: Option<bool>,
    /// Determines whether the parameter value SHOULD allow reserved characters, as defined by
    /// RFC3986 :/?#[]@!$&'()*+,;= to be included without percent-encoding. The default value is
    /// false.
    #[serde(rename = "allowReserved")]
    pub allow_reserved: Option<bool>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}