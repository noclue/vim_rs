use serde::{Deserialize, Serialize};

// From https://spec.openapis.org/oas/v3.0.3#style-values
/// Describes how the parameter value will be serialized depending on the type of the parameter
/// value.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StyleValues {
    /// Path-style parameters defined by
    /// [RFC6570](https://tools.ietf.org/html/rfc6570#section-3.2.7)
    #[serde(rename = "matrix")]
    Matrix,
    /// Label style parameters defined by
    /// [RFC6570](https://tools.ietf.org/html/rfc6570#section-3.2.5).
    #[serde(rename = "label")]
    Label,
    /// Form style parameters defined by
    /// [RFC6570](https://tools.ietf.org/html/rfc6570#section-3.2.8). This option replaces
    /// collectionFormat with a csv (when explode is false) or multi (when explode is true) value
    /// from OpenAPI 2.0.
    #[serde(rename = "form")]
    Form,
    /// Simple style parameters defined by
    /// [RFC6570](https://tools.ietf.org/html/rfc6570#section-3.2.2). This option replaces
    /// collectionFormat with a csv value from OpenAPI 2.0.
    #[serde(rename = "simple")]
    Simple,
    /// Space separated array values. This option replaces collectionFormat equal to ssv from
    /// OpenAPI 2.0.
    #[serde(rename = "spaceDelimited")]
    SpaceDelimited,
    /// Pipe separated array values. This option replaces collectionFormat equal to pipes from
    /// OpenAPI 2.0.
    #[serde(rename = "pipeDelimited")]
    PipeDelimited,
    /// Provides a simple way of rendering nested objects using form parameters.
    #[serde(rename = "deepObject")]
    DeepObject,
    #[serde(untagged)]
    Other(String),
}