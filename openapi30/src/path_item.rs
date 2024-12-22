use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use super::*;

// From https://spec.openapis.org/oas/v3.0.3#path-item-object
/// Describes the operations available on a single path. A Path Item may be empty, due to ACL
/// constraints. The path itself is still exposed to the documentation viewer but they will not know
/// which operations and parameters are available.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PathItem {
    /// Allows for an external definition of this path item. The referenced structure MUST be in the
    /// format of a Path Item Object. In case a Path Item Object field appears both in the defined
    /// object and the referenced object, the behavior is undefined.
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    /// An optional, string summary, intended to apply to all operations in this path.
    pub summary: Option<String>,
    /// An optional, string description, intended to apply to all operations in this path.
    /// CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// A definition of a GET operation on this path.
    pub get: Option<Operation>,
    /// A definition of a PUT operation on this path.
    pub put: Option<Operation>,
    /// A definition of a POST operation on this path.
    pub post: Option<Operation>,
    /// A definition of a DELETE operation on this path.
    pub delete: Option<Operation>,
    /// A definition of a OPTIONS operation on this path.
    pub options: Option<Operation>,
    /// A definition of a HEAD operation on this path.
    pub head: Option<Operation>,
    /// A definition of a PATCH operation on this path.
    pub patch: Option<Operation>,
    /// A definition of a TRACE operation on this path.
    pub trace: Option<Operation>,
    /// An alternative server array to service all operations in this path.
    pub servers: Option<Vec<Server>>,
    /// A list of parameters that are applicable for all the operations described under this path.
    /// These parameters can be overridden at the operation level, but cannot be removed there. The
    /// list MUST NOT include duplicated parameters. A unique parameter is defined by a combination
    /// of a name and location. The list can use the Reference Object to link to parameters that are
    /// defined at the OpenAPI Object's components/parameters.
    pub parameters: Option<Vec<Parameter>>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}