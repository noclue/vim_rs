use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::server::Server;



// From https://spec.openapis.org/oas/v3.0.3#link-object
/// The Link object represents a possible design-time link for a response. The presence of a link
/// does not guarantee the caller's ability to successfully invoke it, rather it provides a known
/// relationship and traversal mechanism between responses and other operations.
///
/// Unlike dynamic links (i.e. links provided in the response payload), the OAS linking mechanism
/// does not require link information in the runtime response.
///
/// For computing links, and providing instructions to execute them, a runtime expression is used
/// for accessing values in an operation and using them as parameters while invoking the linked
/// operation.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Link {
    /// A relative or absolute URI reference to an OAS operation. This field is mutually exclusive
    /// of the operationId field, and MUST point to an Operation Object. Relative operationRef
    /// values MAY be used to locate an existing Operation Object in the OpenAPI definition. Ignored
    /// if the operationId field is specified.
    #[serde(rename = "operationRef")]
    pub operation_ref: Option<String>,
    /// The name of an existing, resolvable OAS operation, as defined with a unique operationId.
    /// This field is mutually exclusive of the operationRef field.
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    /// A map representing parameters to pass to an operation as specified with operationId or
    /// identified via operationRef. The key is the parameter name to be used, whereas the value can
    /// be a constant or an expression to be evaluated and passed to the linked operation. The
    /// parameter name can be qualified using the parameter location [{in}.]{name} for operations
    /// that use parameters.
    pub parameters: Option<HashMap<String, serde_json::Value>>,
    /// A literal value or {expression} to use as a request body when calling the target operation.
    #[serde(rename = "requestBody")]
    pub request_body: Option<serde_json::Value>,
    /// A description of the link. CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// A server object to be used by the target operation.
    pub server: Option<Server>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: HashMap<String, serde_json::Value>,
}