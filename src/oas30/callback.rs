use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::operation::PathItem;

// From https://spec.openapis.org/oas/v3.0.3#callback-object A map of possible out-of band callbacks
// related to the parent operation. Each value in the map is a Path Item Object that describes a set
// of requests that may be initiated by the API provider and the expected responses. The key value
// used to identify the callback object is an expression, evaluated at runtime, that identifies a
// URL to use for the callback operation.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Callback {
    /// A Path Item Object used to define a callback request and expected responses. A complete
    /// example is available.
    #[serde(flatten)]
    pub callbacks: HashMap<String, PathItem>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: HashMap<String, serde_json::Value>,
}