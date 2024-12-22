use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use super::*;

// From https://spec.openapis.org/oas/v3.0.3#operation-object
/// Describes a single API operation on a path.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Operation {
    /// A list of tags for API documentation control. Tags can be used for logical grouping of
    /// operations by resources or any other qualifier.
    pub tags: Option<Vec<String>>,
    /// A short summary of what the operation does.
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior. CommonMark syntax MAY be used for rich text
    /// representation.
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Unique string used to identify the operation. The id MUST be unique among all operations
    /// described in the API. The operationId value is case-sensitive. Tools and libraries MAY use
    /// the operationId to uniquely identify an operation, therefore, it is RECOMMENDED to follow
    /// common programming naming conventions.
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    /// A list of parameters that are applicable for this operation. If a parameter is already
    /// defined at the Path Item, the new definition will override it, but can never remove it. The
    /// list MUST NOT include duplicated parameters. A unique parameter is defined by a combination
    /// of a name and location. The list can use the Reference Object to link to parameters that are
    /// defined at the OpenAPI Object's components/parameters.
    pub parameters: Option<Vec<RefOr<Parameter>>>,
    /// The request body applicable for this operation. The requestBody is only supported in HTTP
    /// methods where the HTTP 1.1 specification RFC7231 has explicitly defined semantics for
    /// request bodies. In other cases where the HTTP spec is vague, requestBody SHALL be ignored by
    /// consumers.
    #[serde(rename = "requestBody")]
    pub request_body: Option<RefOr<RequestBody>>,
    /// The list of possible responses as they are returned from executing this operation.
    pub responses: Responses,
    /// A map of possible out-of band callbacks related to the parent operation. Each value in the
    /// map is a Path Item Object that describes a set of requests that may be initiated by the API
    /// provider and the expected responses. The key value used to identify the callback object is
    /// an expression, evaluated at runtime, that identifies a URL to use for the callback
    /// operation.
    pub callbacks: Option<IndexMap<String, RefOr<Callback>>>,
    /// Declares this operation to be deprecated. Consumers SHOULD refrain from usage of the
    /// declared operation. Default value is false.
    pub deprecated: Option<bool>,
    /// A declaration of which security mechanisms can be used for this operation. The list of
    /// values includes alternative security requirement objects that can be used. Only one of the
    /// security requirement objects need to be satisfied to authorize a request. This definition
    /// overrides the security defined at the OpenAPI Object. To remove a security declaration from
    /// the operation, an empty array can be used.
    pub security: Option<Vec<SecurityRequirement>>,
    /// An alternative server array to service this operation. If an alternative server object is
    /// specified at the Path Item Object or Root level, it will be overridden by this value.
    pub servers: Option<Vec<Server>>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}


// From https://spec.openapis.org/oas/v3.0.3#request-body-object Describes a single request body.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RequestBody {
    /// A brief description of the request body. This could contain examples of use. CommonMark
    /// syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// The content of the request body. The key is a media type or media type range and the value
    /// describes it. For requests that match multiple keys, only the most specific key is
    /// applicable. e.g. text/plain overrides text/*
    pub content: IndexMap<String, MediaType>,
    /// Determines if the request body is required in the request. Defaults to false.
    pub required: Option<bool>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}



// See https://spec.openapis.org/oas/v3.0.3#responses-object 
/// A container for the expected responses of an operation. The container maps a HTTP response code
/// to the expected response. The documentation is not necessarily expected to cover all possible
/// HTTP response codes because they may not be known in advance. However, documentation is expected
/// to cover a successful operation response and any known errors. The default MAY be used as a
/// default response object for all HTTP codes that are not covered individually by the
/// specification. The Responses Object MUST contain at least one response code, and it SHOULD be
/// the response for a successful operation call.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Responses {
    /// The documentation of responses other than the ones declared for specific HTTP response
    /// codes. Use this field to cover undeclared responses. A Reference Object can link to a
    /// response that the OpenAPI Object's components/responses section defines.
    #[serde(rename = "default")]
    pub default_response: Option<RefOr<Response>>,
    /// Any HTTP status code can be used as the property name, but only one property per code, to
    /// describe the expected response for that HTTP status code. A Reference Object can link to a
    /// response that is defined in the OpenAPI Object's components/responses section. This field
    /// MUST be enclosed in quotation marks (for example, "200") for compatibility between JSON and
    /// YAML. To define a range of response codes, this field MAY contain the uppercase wildcard
    /// character X. For example, "2XX" represents all response codes between [200-299]. Only the
    /// following range definitions are allowed: 1XX, 2XX, 3XX, 4XX, and 5XX. If a response is
    /// defined using an explicit code, the explicit code definition takes precedence over the range
    /// definition for that code.
    #[serde(flatten)]
    pub responses: IndexMap<String, RefOr<Response>>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#response-object 
/// Describes a single response from an API Operation, including design-time, static links to
/// operations based on the response.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Response {
    /// A short description of the response. CommonMark syntax MAY be used for rich text
    /// representation.
    pub description: String,
    /// Maps a header name to its definition. RFC7230 states header names are case-insensitive. If a
    /// response header is defined with the name "Content-Type", it SHALL be ignored.
    pub headers: Option<IndexMap<String, RefOr<Header>>>,
    /// A map containing descriptions of potential response payloads. The key is a media type or
    /// media type range and the value describes it. For responses that match multiple keys, only
    /// the most specific key is applicable. e.g. text/plain overrides text/*
    pub content: Option<IndexMap<String, MediaType>>,
    /// A map of operations links that can be followed from the response. The key of the map is a
    /// short name for the link, following the naming constraints of the names for Component
    /// Objects.
    pub links: Option<IndexMap<String, RefOr<Link>>>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}
