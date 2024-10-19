use serde::{Deserialize, Serialize};

// From https://spec.openapis.org/oas/v3.0.3#parameter-location
/// The location of the parameter. Possible values are "query", "header", "path" or "cookie".
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Location {
    /// Query parameters are defined using the query parameters object.
    #[serde(rename = "query")]
    Query,
    /// Header parameters are used to send additional data to the server.
    #[serde(rename = "header")]
    Header,
    /// Path parameters are defined using the path parameters object.
    #[serde(rename = "path")]
    Path,
    /// Cookie parameters provide data to the server via the cookie header.
    #[serde(rename = "cookie")]
    Cookie,
    /// Other type not known by this library
    #[serde(untagged)]
    Other(String),
}