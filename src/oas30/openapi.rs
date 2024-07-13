use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// From https://spec.openapis.org/oas/v3.0.3#schema
/// This is the root document object of the OpenAPI document.
#[derive(Debug, Serialize, Deserialize)]
struct OpenAPI {
    /// This string MUST be the semantic version number of the OpenAPI Specification version that
    /// the OpenAPI document uses. The openapi field SHOULD be used by tooling specifications and
    /// clients to interpret the OpenAPI document. This is not related to the API info.version
    /// string.
    openapi: String,
    /// Provides metadata about the API. The metadata MAY be used by tooling as required.
    info: Info,
    /// An array of Server Objects, which provide connectivity information to a target server. If
    /// the servers property is not provided, or is an empty array, the default value would be a
    /// Server Object with a url value of /.
    servers: Option<Vec<Server>>,
    /// The available paths and operations for the API.
    paths: HashMap<String, PathItem>,
    /// An element to hold various schemas for the specification.
    components: Option<Components>,
    /// A declaration of which security mechanisms can be used across the API. The list of values
    /// includes alternative security requirement objects that can be used. Only one of the security
    /// requirement objects need to be satisfied to authorize a request. Individual operations can
    /// override this definition.
    security: Option<Vec<SecurityRequirement>>,
    /// A list of tags used by the specification with additional metadata. The order of the tags can
    /// be used to reflect on their order by the parsing tools. Not all tags that are used by the
    /// Operation Object must be declared. The tags that are not declared MAY be organized randomly
    /// or based on the tools' logic. Each tag name in the list MUST be unique.
    tags: Option<Vec<Tag>>,
    /// Additional external documentation.
    #[serde(rename = "externalDocs")]
    external_docs: Option<ExternalDocumentation>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#info-object 
/// The object provides metadata about the API. The metadata MAY be used by the clients if needed,
/// and MAY be presented in editing or documentation generation tools for convenience.
#[derive(Debug, Serialize, Deserialize)]
struct Info {
    /// The title of the API.
    title: String,
    /// The version of the OpenAPI document (which is distinct from the OpenAPI Specification
    /// version or the API implementation version).
    version: String,
    /// A short description of the API. CommonMark syntax MAY be used for rich text representation.
    description: Option<String>,
    /// A URL to the Terms of Service for the API. MUST be in the format of a URL.
    terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    contact: Option<Contact>,
    /// The license information for the exposed API.
    license: Option<License>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#contact-object
/// Contact information for the exposed API.
#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    /// The identifying name of the contact person/organization.
    name: Option<String>,
    /// The URL pointing to the contact information. MUST be in the format of a URL.
    url: Option<String>,
    /// The email address of the contact person/organization. MUST be in the format of an email
    /// address.
    email: Option<String>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#license-object
/// License information for the exposed API.
#[derive(Debug, Serialize, Deserialize)]
struct License {
    /// The license name used for the API.
    name: String,
    /// A URL to the license used for the API. MUST be in the format of a URL.
    url: Option<String>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#server-object 
/// An object representing a Server.
#[derive(Debug, Serialize, Deserialize)]
struct Server {
    /// A URL to the target host. This URL supports Server Variables and MAY be relative, to
    /// indicate that the host location is relative to the location where the OpenAPI document is
    /// being served. Variable substitutions will be made when a variable is named in {brackets}.
    url: String,
    /// An optional string describing the host designated by the URL. CommonMark syntax MAY be used
    /// for rich text representation.
    description: Option<String>,
    /// A map between a variable name and its value. The value is used for substitution in the
    /// server's URL template.
    variables: Option<HashMap<String, ServerVariable>>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#server-variable-object 
/// An object representing a Server Variable for server URL template substitution.
#[derive(Debug, Serialize, Deserialize)]
struct ServerVariable {
    /// An enumeration of string values to be used if the substitution options are from a limited
    /// set. The array SHOULD NOT be empty.
    #[serde(rename = "enum")]
    enumeration: Option<Vec<String>>,
    /// The default value to use for substitution, which SHALL be sent if an alternate value is not
    /// supplied. Note this behavior is different than the Schema Object's treatment of default
    /// values, because in those cases parameter values are optional. If the enum is defined, the
    /// value SHOULD exist in the enum's values.
    default: String,
    /// An optional description for the server variable. CommonMark syntax MAY be used for rich text
    /// representation.
    description: Option<String>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// from https://spec.openapis.org/oas/v3.0.3#components-object
/// Holds a set of reusable objects for different aspects of the OAS. All objects defined within the
/// components object will have no effect on the API unless they are explicitly referenced from
/// properties outside the components object.
#[derive(Debug, Serialize, Deserialize)]
struct Components {
    /// An object to hold reusable Schema Objects.
    schemas: Option<HashMap<String, RefOr<Schema>>>,
    /// An object to hold reusable Response Objects.
    responses: Option<HashMap<String, RefOr<Response>>>,
    /// An object to hold reusable Parameter Objects.
    parameters: Option<HashMap<String, RefOr<Parameter>>>,
    /// An object to hold reusable Example Objects.
    examples: Option<HashMap<String, RefOr<Example>>>,
    /// An object to hold reusable Request Body Objects.
    #[serde(rename = "requestBodies")]
    request_bodies: Option<HashMap<String, RefOr<RequestBody>>>,
    /// An object to hold reusable Header Objects.
    headers: Option<HashMap<String, RefOr<Header>>>,
    /// An object to hold reusable Security Scheme Objects.
    #[serde(rename = "securitySchemes")]
    security_schemes: Option<HashMap<String, RefOr<SecurityScheme>>>,
    /// An object to hold reusable Link Objects.
    links: Option<HashMap<String, RefOr<Link>>>,
    /// An object to hold reusable Callback Objects.
    callbacks: Option<HashMap<String, RefOr<Callback>>>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#path-item-object 
/// Describes the operations available on a single path. A Path Item may be empty, due to ACL
/// constraints. The path itself is still exposed to the documentation viewer but they will not know
/// which operations and parameters are available.
#[derive(Debug, Serialize, Deserialize)]
struct PathItem {
    /// Allows for an external definition of this path item. The referenced structure MUST be in the
    /// format of a Path Item Object. In case a Path Item Object field appears both in the defined
    /// object and the referenced object, the behavior is undefined.
    #[serde(rename = "$ref")]
    reference: Option<String>,
    /// An optional, string summary, intended to apply to all operations in this path.
    summary: Option<String>,
    /// An optional, string description, intended to apply to all operations in this path.
    /// CommonMark syntax MAY be used for rich text representation.
    description: Option<String>,
    /// A definition of a GET operation on this path.
    get: Option<Operation>,
    /// A definition of a PUT operation on this path.
    put: Option<Operation>,
    /// A definition of a POST operation on this path.
    post: Option<Operation>,
    /// A definition of a DELETE operation on this path.
    delete: Option<Operation>,
    /// A definition of a OPTIONS operation on this path.
    options: Option<Operation>,
    /// A definition of a HEAD operation on this path.
    head: Option<Operation>,
    /// A definition of a PATCH operation on this path.
    patch: Option<Operation>,
    /// A definition of a TRACE operation on this path.
    trace: Option<Operation>,
    /// An alternative server array to service all operations in this path.
    servers: Option<Vec<Server>>,
    /// A list of parameters that are applicable for all the operations described under this path.
    /// These parameters can be overridden at the operation level, but cannot be removed there. The
    /// list MUST NOT include duplicated parameters. A unique parameter is defined by a combination
    /// of a name and location. The list can use the Reference Object to link to parameters that are
    /// defined at the OpenAPI Object's components/parameters.
    parameters: Option<Vec<Parameter>>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#operation-object 
/// Describes a single API operation on a path.
#[derive(Debug, Serialize, Deserialize)]
struct Operation {
    /// A list of tags for API documentation control. Tags can be used for logical grouping of
    /// operations by resources or any other qualifier.
    tags: Option<Vec<String>>,
    /// A short summary of what the operation does.
    summary: Option<String>,
    /// A verbose explanation of the operation behavior. CommonMark syntax MAY be used for rich text
    /// representation.
    description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(rename = "externalDocs")]
    external_docs: Option<ExternalDocumentation>,
    /// Unique string used to identify the operation. The id MUST be unique among all operations
    /// described in the API. The operationId value is case-sensitive. Tools and libraries MAY use
    /// the operationId to uniquely identify an operation, therefore, it is RECOMMENDED to follow
    /// common programming naming conventions.
    #[serde(rename = "operationId")]
    operation_id: Option<String>,
    /// A list of parameters that are applicable for this operation. If a parameter is already
    /// defined at the Path Item, the new definition will override it, but can never remove it. The
    /// list MUST NOT include duplicated parameters. A unique parameter is defined by a combination
    /// of a name and location. The list can use the Reference Object to link to parameters that are
    /// defined at the OpenAPI Object's components/parameters.
    parameters: Option<Vec<RefOr<Parameter>>>,
    /// The request body applicable for this operation. The requestBody is only supported in HTTP
    /// methods where the HTTP 1.1 specification RFC7231 has explicitly defined semantics for
    /// request bodies. In other cases where the HTTP spec is vague, requestBody SHALL be ignored by
    /// consumers.
    #[serde(rename = "requestBody")]
    request_body: Option<RefOr<RequestBody>>,
    /// The list of possible responses as they are returned from executing this operation.
    responses: Responses,
    /// A map of possible out-of band callbacks related to the parent operation. Each value in the
    /// map is a Path Item Object that describes a set of requests that may be initiated by the API
    /// provider and the expected responses. The key value used to identify the callback object is
    /// an expression, evaluated at runtime, that identifies a URL to use for the callback
    /// operation.
    callbacks: Option<HashMap<String, RefOr<Callback>>>,
    /// Declares this operation to be deprecated. Consumers SHOULD refrain from usage of the
    /// declared operation. Default value is false.
    deprecated: Option<bool>,
    /// A declaration of which security mechanisms can be used for this operation. The list of
    /// values includes alternative security requirement objects that can be used. Only one of the
    /// security requirement objects need to be satisfied to authorize a request. This definition
    /// overrides the security defined at the OpenAPI Object. To remove a security declaration from
    /// the operation, an empty array can be used.
    security: Option<Vec<SecurityRequirement>>,
    /// An alternative server array to service this operation. If an alternative server object is
    /// specified at the Path Item Object or Root level, it will be overridden by this value.
    servers: Option<Vec<Server>>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#external-documentation-object 
/// Allows referencing an external resource for extended documentation.
#[derive(Debug, Serialize, Deserialize)]
struct ExternalDocumentation {
    /// A short description of the target documentation. CommonMark syntax MAY be used for rich text
    /// representation.
    description: Option<String>,
    /// The URL for the target documentation. Value MUST be in the format of a URL.
    url: String,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#parameter-object 
/// Describes a single operation parameter.
#[derive(Debug, Serialize, Deserialize)]
struct Parameter {
    /// The name of the parameter. Parameter names are case sensitive.
    name: String,
    /// The location of the parameter. Possible values are "query", "header", "path" or "cookie".
    #[serde(rename = "in")]
    location: ParameterLocation,
    /// A brief description of the parameter. This could contain examples of use. CommonMark syntax
    /// MAY be used for rich text representation.
    description: Option<String>,
    /// Determines whether this parameter is mandatory. If the parameter location is "path", this
    /// property is REQUIRED and its value MUST be true. Otherwise, the property MAY be included and
    /// its default value is false.
    required: Option<bool>,
    /// Specifies that a parameter is deprecated and SHOULD be transitioned out of usage.
    deprecated: Option<bool>,
    /// Sets the ability to pass empty-valued parameters. This is valid only for query parameters
    /// and allows sending a parameter with an empty value.
    allow_empty_value: Option<bool>,
    /// Describes how the parameter value will be serialized depending on the type of the parameter
    /// value. Default values (based on value of in): for query - form; for path - simple; for
    /// header - simple; for cookie - form.
    #[serde(rename = "style")]
    serialization_style: Option<StyleValues>,
    /// When this is true, parameter values of type array or object generate separate parameters for
    /// each value of the array or key-value pair of the map. For other types of parameters this
    /// property has no effect. When style is form, the default value is true. For all other styles,
    /// the default value is false.
    #[serde(rename = "explode")]
    explode_values: Option<bool>,
    /// Determines whether the parameter value SHOULD allow reserved characters, as defined by
    /// RFC3986 :/?#[]@!$&'()*+,;= to be included without percent-encoding. The default value is
    /// false.
    #[serde(rename = "allowReserved")]
    allow_reserved: Option<bool>,
    /// The schema defining the type used for the parameter.
    schema: Option<RefOr<Schema>>,
    /// Example of the parameter's potential value. The example SHOULD match the specified schema
    /// and encoding properties if present. The example field is mutually exclusive of the examples
    /// field. Furthermore, if referencing a schema which contains an example, the example value
    /// SHALL override the example provided by the schema.
    example: Option<serde_json::Value>,
    /// Examples of the parameter
    examples: Option<HashMap<String, RefOr<Example>>>,
    /// A map containing the representations for the parameter. The key is the media type and the
    /// value describes it. The map MUST only contain one entry. This property is REQUIRED when the
    /// requestBody property is in use and the media type of the request is not
    /// application/x-www-form-urlencoded. The map key MUST be the media type and the value MUST be
    /// a MediaType Object.
    content: Option<HashMap<String, MediaType>>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#parameter-location 
/// The location of the parameter. Possible values are "query", "header", "path" or "cookie".
#[derive(Debug, Serialize, Deserialize)]
enum ParameterLocation {
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

// From https://spec.openapis.org/oas/v3.0.3#style-values 
/// Describes how the parameter value will be serialized depending on the type of the parameter
/// value.
#[derive(Debug, Serialize, Deserialize)]
enum StyleValues {
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

// From https://spec.openapis.org/oas/v3.0.3#request-body-object Describes a single request body.
#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
    /// A brief description of the request body. This could contain examples of use. CommonMark
    /// syntax MAY be used for rich text representation.
    description: Option<String>,
    /// The content of the request body. The key is a media type or media type range and the value
    /// describes it. For requests that match multiple keys, only the most specific key is
    /// applicable. e.g. text/plain overrides text/*
    content: HashMap<String, MediaType>,
    /// Determines if the request body is required in the request. Defaults to false.
    required: Option<bool>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#media-type-object Each Media Type Object provides
// schema and examples for the media type identified by its key.
#[derive(Debug, Serialize, Deserialize)]
struct MediaType {
    /// The schema defining the content of the request, response, or parameter.
    schema: Option<RefOr<Schema>>,
    /// Example of the media type. The example object SHOULD be in the correct format as specified
    /// by the media type. The example field is mutually exclusive of the examples field.
    /// Furthermore, if referencing a schema which contains an example, the example value SHALL
    /// override the example provided by the schema.
    example: Option<serde_json::Value>,
    /// Examples of the media type
    examples: Option<HashMap<String, RefOr<Example>>>,
    /// A map between a property name and its encoding information. The key, being the property
    /// name, MUST exist in the schema as a property. The encoding object SHALL only apply to
    /// requestBody objects when the media type is multipart or application/x-www-form-urlencoded.
    encoding: Option<HashMap<String, Encoding>>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#encoding-object A single encoding definition applied to
// a single schema property.
#[derive(Debug, Serialize, Deserialize)]
struct Encoding {
    /// The Content-Type for encoding a specific property.
    #[serde(rename = "contentType")]
    content_type: Option<String>,
    /// A map allowing additional information to be provided as headers, for example
    /// Content-Disposition. Content-Type is described separately and SHALL be ignored in this
    /// section. This property SHALL be ignored if the request body media type is not a multipart.
    headers: Option<HashMap<String, RefOr<Header>>>,
    /// Describes how a specific property value will be serialized depending on its type.
    style: Option<StyleValues>,
    /// When this is true, property values of type array or object generate separate parameters for
    /// each value of the array, or key-value pair of the map. For other types of properties this
    /// property has no effect. When style is form, the default value is true. For all other styles,
    /// the default value is false.
    explode: Option<bool>,
    /// Determines whether the parameter value SHOULD allow reserved characters, as defined by
    /// RFC3986 :/?#[]@!$&'()*+,;= to be included without percent-encoding. The default value is
    /// false.
    #[serde(rename = "allowReserved")]
    allow_reserved: Option<bool>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

/// See https://spec.openapis.org/oas/v3.0.3#responses-object A container for the expected responses
/// of an operation. The container maps a HTTP response code to the expected response. The
/// documentation is not necessarily expected to cover all possible HTTP response codes because they
/// may not be known in advance. However, documentation is expected to cover a successful operation
/// response and any known errors. The default MAY be used as a default response object for all HTTP
/// codes that are not covered individually by the specification. The Responses Object MUST contain
/// at least one response code, and it SHOULD be the response for a successful operation call.
#[derive(Debug, Serialize, Deserialize)]
struct Responses {
    /// The documentation of responses other than the ones declared for specific HTTP response
    /// codes. Use this field to cover undeclared responses. A Reference Object can link to a
    /// response that the OpenAPI Object's components/responses section defines.
    #[serde(rename = "default")]
    default_response: Option<RefOr<Response>>,
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
    responses: HashMap<String, RefOr<Response>>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#response-object Describes a single response from an API
// Operation, including design-time, static links to operations based on the response.
#[derive(Debug, Serialize, Deserialize)]
struct Response {
    /// A short description of the response. CommonMark syntax MAY be used for rich text
    /// representation.
    description: String,
    /// Maps a header name to its definition. RFC7230 states header names are case-insensitive. If a
    /// response header is defined with the name "Content-Type", it SHALL be ignored.
    headers: Option<HashMap<String, RefOr<Header>>>,
    /// A map containing descriptions of potential response payloads. The key is a media type or
    /// media type range and the value describes it. For responses that match multiple keys, only
    /// the most specific key is applicable. e.g. text/plain overrides text/*
    content: Option<HashMap<String, MediaType>>,
    /// A map of operations links that can be followed from the response. The key of the map is a
    /// short name for the link, following the naming constraints of the names for Component
    /// Objects.
    links: Option<HashMap<String, RefOr<Link>>>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#callback-object A map of possible out-of band callbacks
// related to the parent operation. Each value in the map is a Path Item Object that describes a set
// of requests that may be initiated by the API provider and the expected responses. The key value
// used to identify the callback object is an expression, evaluated at runtime, that identifies a
// URL to use for the callback operation.
#[derive(Debug, Serialize, Deserialize)]
struct Callback {
    /// A Path Item Object used to define a callback request and expected responses. A complete
    /// example is available.
    #[serde(flatten)]
    callbacks: HashMap<String, PathItem>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#example-object
/// Each Example Object provides an example of an instance for an schema.
#[derive(Debug, Serialize, Deserialize)]
struct Example {
    /// Short description for the example.
    summary: Option<String>,
    /// Long description for the example. CommonMark syntax MAY be used for rich text
    /// representation.
    description: Option<String>,
    /// Embedded literal example. The value field and externalValue field are mutually exclusive. To
    /// represent examples of media types that cannot naturally represented in JSON or YAML, use a
    /// string value to contain the example, escaping where necessary.
    value: Option<serde_json::Value>,
    /// A URL that points to the literal example. This provides the capability to reference examples
    /// that cannot easily be included in JSON or YAML documents. The value field and externalValue
    /// field are mutually exclusive.
    #[serde(rename = "externalValue")]
    external_value: Option<String>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

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
#[derive(Debug, Serialize, Deserialize)]
struct Link {
    /// A relative or absolute URI reference to an OAS operation. This field is mutually exclusive
    /// of the operationId field, and MUST point to an Operation Object. Relative operationRef
    /// values MAY be used to locate an existing Operation Object in the OpenAPI definition. Ignored
    /// if the operationId field is specified.
    #[serde(rename = "operationRef")]
    operation_ref: Option<String>,
    /// The name of an existing, resolvable OAS operation, as defined with a unique operationId.
    /// This field is mutually exclusive of the operationRef field.
    #[serde(rename = "operationId")]
    operation_id: Option<String>,
    /// A map representing parameters to pass to an operation as specified with operationId or
    /// identified via operationRef. The key is the parameter name to be used, whereas the value can
    /// be a constant or an expression to be evaluated and passed to the linked operation. The
    /// parameter name can be qualified using the parameter location [{in}.]{name} for operations
    /// that use parameters.
    parameters: Option<HashMap<String, serde_json::Value>>,
    /// A literal value or {expression} to use as a request body when calling the target operation.
    #[serde(rename = "requestBody")]
    request_body: Option<serde_json::Value>,
    /// A description of the link. CommonMark syntax MAY be used for rich text representation.
    description: Option<String>,
    /// A server object to be used by the target operation.
    server: Option<Server>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#header-object
/// Describes a single header parameter. A Header Object allows the definition of a header for a
/// response.
#[derive(Debug, Serialize, Deserialize)]
struct Header {
    /// A brief description of the parameter. This could contain examples of use. CommonMark syntax
    /// MAY be used for rich text representation.
    description: Option<String>,
    /// Determines whether this parameter is mandatory. If the parameter location is "path", this
    /// property is REQUIRED and its value MUST be true. Otherwise, the property MAY be included and
    /// its default value is false.
    required: Option<bool>,
    /// Specifies that a parameter is deprecated and SHOULD be transitioned out of usage.
    deprecated: Option<bool>,
    /// Specifies that a parameter is deprecated and SHOULD be transitioned out of usage.
    allow_empty_value: Option<bool>,
    /// Describes how the parameter value will be serialized depending on the type of the parameter
    /// value. Default values (based on value of in): for query - form; for path - simple; for
    /// header - simple; for cookie - form.
    #[serde(rename = "style")]
    serialization_style: Option<StyleValues>,
    /// When this is true, parameter values of type array or object generate separate parameters for
    /// each value of the array or key-value pair of the map. For other types of parameters this
    /// property has no effect. When style is form, the default value is true. For all other styles,
    /// the default value is false.
    #[serde(rename = "explode")]
    explode_values: Option<bool>,
    /// Determines whether the parameter value SHOULD allow reserved characters, as defined by
    /// RFC3986 :/?#[]@!$&'()*+,;= to be included without percent-encoding. The default value is
    /// false.
    #[serde(rename = "allowReserved")]
    allow_reserved: Option<bool>,
    /// The schema defining the type used for the parameter.
    schema: Option<RefOr<Schema>>,
    /// Example of the parameter's potential value. The example SHOULD match the specified schema
    /// and encoding properties if present. The example field is mutually exclusive of the examples
    /// field. Furthermore, if referencing a schema which contains an example, the example value
    /// SHALL override the example provided by the schema.
    example: Option<serde_json::Value>,
    /// Examples of the parameter
    examples: Option<HashMap<String, RefOr<Example>>>,
    /// A map containing the representations for the parameter. The key is the media type and the
    /// value describes it. The map MUST only contain one entry. This property is REQUIRED when the
    /// requestBody property is in use and the media type of the request is not
    /// application/x-www-form-urlencoded. The map key MUST be the media type and the value MUST be
    /// a MediaType Object.
    content: Option<HashMap<String, MediaType>>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#tag-object
/// Adds metadata to a single tag that is used by the Operation Object. It is not mandatory to have
/// a Tag Object per tag used there.
#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    /// The name of the tag.
    name: String,
    /// A short description for the tag. CommonMark syntax MAY be used for rich text representation.
    description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(rename = "externalDocs")]
    external_docs: Option<ExternalDocumentation>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#reference-object
/// A simple object to allow referencing other components in the specification, internally and
/// externally.
///
/// The Reference Object is defined by JSON Reference and follows the same structure, behavior and
/// rules.
///
/// For this specification, reference resolution is accomplished as defined by the JSON Reference
/// specification and not by the JSON Schema specification.
///
/// This object cannot be extended with additional properties and any properties added SHALL be
/// ignored.
#[derive(Debug, Serialize, Deserialize)]
struct Reference {
    /// The reference string.
    #[serde(rename = "$ref")]
    reference: String,
}

/// A simple utility enum to allow for either a direct value or a reference to a value.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum RefOr<T> {
    /// A value
    Val(Box<T>),
    /// A reference to a value
    Ref(Reference),
}

// From https://spec.openapis.org/oas/v3.0.3#schema-object
/// The Schema Object allows the definition of input and output data types. These types can be
/// objects, but also primitives and arrays. This object is an extended subset of the JSON Schema
/// Specification Wright Draft 00.
///
/// For more information about the properties, see JSON Schema Core and JSON Schema Validation.
/// Unless stated otherwise, the property definitions follow the JSON Schema.
#[derive(Debug, Serialize, Deserialize)]
struct Schema {
    /// The value of "title" MUST be a string.
    title: Option<String>,
    // Validation rules from
    // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5
    /// The value of "multipleOf" MUST be a number, strictly greater than 0. A numeric instance is
    /// only valid if division by this keyword's value results in an integer.
    #[serde(rename = "multipleOf")]
    multiple_of: Option<f64>,
    /// The value of "maximum" MUST be a number, representing an upper limit for a numeric instance.
    /// If the instance is a number, then this keyword validates if "exclusiveMaximum" is true and
    /// instance is less than the provided value, or else if the instance is less than or exactly
    /// equal to the provided value.
    maximum: Option<f64>,
    /// The value of "exclusiveMaximum" MUST be a boolean, representing whether the limit in
    /// "maximum" is exclusive or not.  An undefined value is the same as false.
    ///
    /// If "exclusiveMaximum" is true, then a numeric instance SHOULD NOT be equal to the value
    /// specified in "maximum".  If "exclusiveMaximum" is false (or not specified), then a numeric
    /// instance MAY be equal to the value of "maximum".
    #[serde(rename = "exclusiveMaximum")]
    exclusive_maximum: Option<bool>,
    /// The value of "minimum" MUST be a number, representing a lower limit for a numeric instance.
    ///
    /// If the instance is a number, then this keyword validates if "exclusiveMinimum" is true and
    /// instance is greater than the provided value, or else if the instance is greater than or
    /// exactly equal to the provided value.
    minimum: Option<f64>,
    /// The value of "exclusiveMinimum" MUST be a boolean, representing whether the limit in
    /// "minimum" is exclusive or not.  An undefined value is the same as false.
    #[serde(rename = "exclusiveMinimum")]
    exclusive_minimum: Option<bool>,
    /// The value of "maxLength" MUST be a non-negative integer. A string instance is valid against
    /// this keyword if its length is less than, or equal to, the value of this keyword.
    ///
    /// The length of a string instance is defined as the number of its characters as defined by RFC
    /// 7159 [RFC7159].
    ///
    /// This keyword can be used to control the maximum length of strings.
    #[serde(rename = "maxLength")]
    max_length: Option<u64>,
    /// The value of "minLength" MUST be a non-negative integer. A string instance is valid against
    /// this keyword if its length is greater than, or equal to, the value of this keyword.
    ///
    /// The length of a string instance is defined as the number of its characters as defined by RFC
    /// 7159 [RFC7159].
    ///
    /// "minLength", if absent, may be considered as 0.
    #[serde(rename = "minLength")]
    min_length: Option<u64>,
    /// This string SHOULD be a valid regular expression, according to the ECMA 262 regular
    /// expression dialect.
    ///
    /// A string instance is considered valid if the regular expression matches the instance
    /// successfully. Recall: regular expressions are not implicitly anchored.
    #[serde(rename = "pattern")]
    pattern: Option<String>,
    /// The value of "maxItems" MUST be a non-negative integer. An array instance is valid against
    /// "maxItems" if its size is less than, or equal to, the value of this keyword.
    ///
    /// This keyword can be used to control the maximum number of items in an array.
    #[serde(rename = "maxItems")]
    max_items: Option<u64>,
    /// The value of "minItems" MUST be a non-negative integer. An array instance is valid against
    /// "minItems" if its size is greater than, or equal to, the value of this keyword.
    ///
    /// This keyword can be used to control the minimum number of items in an array.
    #[serde(rename = "minItems")]
    min_items: Option<u64>,
    /// The value of "uniqueItems" MUST be a boolean. If this keyword has boolean value false, the
    /// instance validates successfully. If it has boolean value true, the instance validates
    /// successfully if all of its elements are unique.
    ///
    /// If not present, this keyword may be considered as being present with boolean value false.
    #[serde(rename = "uniqueItems")]
    unique_items: Option<bool>,
    /// The value of "maxProperties" MUST be a non-negative integer. An object instance is valid
    /// against "maxProperties" if its number of properties is less than, or equal to, the value of
    /// this keyword.
    ///
    /// This keyword can be used to control the maximum number of properties in an object.
    #[serde(rename = "maxProperties")]
    max_properties: Option<u64>,
    /// The value of "minProperties" MUST be a non-negative integer. An object instance is valid
    /// against "minProperties" if its number of properties is greater than, or equal to, the value
    /// of this keyword.
    ///
    /// This keyword can be used to control the minimum number of properties in an object.
    #[serde(rename = "minProperties")]
    min_properties: Option<u64>,
    /// The value of this keyword MUST be an array.  This array MUST have at least one element.
    /// Elements of this array MUST be strings, and MUST be unique.
    ///
    /// An object instance is valid against this keyword if its property set contains all elements
    /// in this keyword's array value.
    #[serde(rename = "required")]
    required: Option<Vec<String>>,
    // from
    // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.20
    /// The value of this keyword MUST be an array.  This array SHOULD have at least one element.
    /// Elements in the array SHOULD be unique.
    ///
    /// Elements in the array MAY be of any type, including null.
    ///
    /// An instance validates successfully against this keyword if its value is equal to one of the
    /// elements in this keyword's array value.
    #[serde(rename = "enum")]
    enumeration: Option<Vec<serde_json::Value>>,
    /// The value of "type" MUST be a string, representing the type of the schema.
    #[serde(rename = "type")]
    schema_type: Option<SchemaType>,
    /// This keyword's value MUST be an array. This array SHOULD have at least one element.
    /// 
    /// Elements of the array MUST be objects. Inline or referenced schema MUST be of a Schema
    /// Object and not a standard JSON Schema.
    /// 
    /// An instance validates successfully against this keyword if it validates successfully against
    /// all schemas defined by this keyword's value.
    #[serde(rename = "allOf")]
    all_of: Option<Vec<RefOr<Schema>>>,
    /// This keyword's value MUST be an array. This array SHOULD have at least one element.
    /// 
    /// Elements of the array MUST be objects. Inline or referenced schema MUST be of a Schema
    /// Object and not a standard JSON Schema.
    /// 
    /// An instance validates successfully against this keyword if it validates successfully against
    /// exactly one schema defined by this keyword's value.
    #[serde(rename = "oneOf")]
    one_of: Option<Vec<RefOr<Schema>>>,
    /// This keyword's value MUST be an array. This array SHOULD have at least one element.
    /// 
    /// Elements of the array MUST be objects. Inline or referenced schema MUST be of a Schema
    /// Object and not a standard JSON Schema.
    /// 
    /// An instance validates successfully against this keyword if it validates successfully against
    /// at least one schema defined by this keyword's value.
    #[serde(rename = "anyOf")]
    any_of: Option<Vec<RefOr<Schema>>>,
    /// This keyword's value MUST be an object.  Inline or referenced schema MUST be of a Schema
    /// Object and not a standard JSON Schema.
    /// 
    /// An instance is valid against this keyword if it fails to validate successfully against the
    /// schema defined by this keyword.
    not: Option<RefOr<Schema>>,
    /// The value of "items" MUST be an object, and that object MUST be a valid JSON Schema. This
    /// object defines the schema for items in arrays. `items` MUST be present if the `type` is
    /// `array`.
    #[serde(rename = "items")]
    items: Option<RefOr<Schema>>,
    /// The value of "properties" MUST be an object. Each value of this object MUST be an object, 
    /// and each object MUST be a valid schema.
    /// 
    /// Property definitions MUST be a Schema Object and not a standard JSON Schema (inline or
    /// referenced).
    #[serde(rename = "properties")]
    properties: Option<HashMap<String, RefOr<Schema>>>,
    /// The value of "additionalProperties" MUST be a boolean or a schema. If `true` is provided,
    /// then the object can have any property. If `false` is provided, then the object cannot have
    /// any additional properties. If a schema is provided, then the object properties MUST match
    /// the schema.
    /// 
    /// Inline or referenced schema MUST be of a Schema Object and not a standard JSON Schema.
    #[serde(rename = "additionalProperties")]
    additional_properties: Option<RefOr<Schema>>,
    /// The value of "description" MUST be a string. a description will provide explanation about
    /// the purpose of the instance described by this schema.
    /// 
    /// [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
    description: Option<String>,
    /// See [Data Type Formats](https://spec.openapis.org/oas/v3.0.3#dataTypeFormat) for further
    /// details. While relying on JSON Schema’s defined formats, the OAS offers a few additional
    /// predefined formats.
    format: Option<DataFormat>,
    /// The default value represents what would be assumed by the consumer of the input as the value
    /// of the schema if one is not provided. Unlike JSON Schema, the value MUST conform to the
    /// defined type for the Schema Object defined at the same level. For example, if type is
    /// string, then default can be "foo" but cannot be 1.
    default: Option<serde_json::Value>,
    /// A `true`` value adds "null" to the allowed type specified by the type keyword, only if type
    /// is explicitly defined within the same Schema Object. Other Schema Object constraints retain
    /// their defined behavior, and therefore may disallow the use of `null` as a value. A `false`
    /// value leaves the specified or default type unmodified. The default value is `false`.
    nullable: Option<bool>,
    /// Adds support for polymorphism. The discriminator is an object name that is used to
    /// differentiate between other schemas which may satisfy the payload description. See
    /// [Composition and
    /// Inheritance](https://spec.openapis.org/oas/v3.0.3#composition-and-inheritance-polymorphism)
    /// for more details.
    discriminator: Option<Discriminator>,
    /// Relevant only for Schema `"properties"` definitions. Declares the property as "read only".
    /// This means that it MAY be sent as part of a response but SHOULD NOT be sent as part of the
    /// request. If the property is marked as readOnly being `true`, then the `writeOnly` property
    /// MUST be `false`. Default value is `false`.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// Relevant only for Schema `"properties"` definitions. Declares the property as "write only".
    /// Therefore, it MAY be sent as part of a request but SHOULD NOT be sent as part of the
    /// response. If the property is marked as `writeOnly` being `true`, then the `readOnly`
    /// property MUST be `false`. Default value is `false`.
    #[serde(rename = "writeOnly")]
    write_only: Option<bool>,
    /// This may be used only on properties schemas. It has no effect on root schemas. Adds
    /// additional metadata to describe the XML representation of this property.
    #[serde(rename = "xml")]
    xml: Option<XML>,
    /// Additional external documentation for this schema.
    #[serde(rename = "externalDocs")]
    external_docs: Option<ExternalDocumentation>,
    /// A free-form property to include an example of an instance for this schema. To represent
    /// examples that cannot be naturally represented in JSON or YAML, a string value can be used to
    /// contain the example with escaping where necessary.
    #[serde(rename = "example")]
    example: Option<serde_json::Value>,
    /// Specifies that a schema is deprecated and SHOULD be transitioned out of usage. Default value
    /// is false.
    deprecated: Option<bool>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#data-types TODO: I am unsure where all schema types are
// defined. Is there "any" type? I found `null`` is not supported and instead `nullable` should be
// used in https://spec.openapis.org/oas/v3.0.3#data-types There is list of 6 types excluding
// `integer` in JSON schema
// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-00#section-4.2 However the OAS
// spec insists on using `integer` as additional type it seems
// https://spec.openapis.org/oas/v3.0.3#data-types
/// The value of "type" MUST be a string, representing the type of the schema.
#[derive(Debug, Serialize, Deserialize)]
enum SchemaType {
    /// Value MUST be a number without fractional part
    #[serde(rename = "integer")]
    Integer,
    /// Value MUST be a number.
    #[serde(rename = "number")]
    Number,
    /// Value MUST be a string.
    #[serde(rename = "string")]
    String,
    /// Value MUST be a boolean.
    #[serde(rename = "boolean")]
    Boolean,
    /// Value MUST be an object.
    #[serde(rename = "object")]
    Object,
    /// Value MUST be an array.
    #[serde(rename = "array")]
    Array,
    #[serde(untagged)]
    Other(String),
}

// From https://spec.openapis.org/oas/v3.0.3#data-types
/// Primitives have an optional modifier property: format. OAS uses several known formats to define
/// in fine detail the data type being used. However, to support documentation needs, the format
/// property is an open string-valued property, and can have any value. Formats such as "email",
/// "uuid", and so on, MAY be used even though undefined by this specification. Types that are not
/// accompanied by a format property follow the type definition in the JSON Schema. Tools that do
/// not recognize a specific format MAY default back to the type alone, as if the format is not
/// specified.
#[derive(Debug, Serialize, Deserialize)]
enum DataFormat {
    /// signed 32 bits
    #[serde(rename = "int32")]
    Int32,
    /// signed 64 bits
    #[serde(rename = "int64")]
    Int64,
    /// float 32 bits
    #[serde(rename = "float")]
    Float,
    /// double 64 bits
    #[serde(rename = "double")]
    Double,
    /// base64 encoded characters
    #[serde(rename = "byte")]
    Byte,
    /// any sequence of octets
    #[serde(rename = "binary")]
    Binary,
    /// `full-date` defined by [RFC3339](https://tools.ietf.org/html/rfc3339#section-5.6)
    #[serde(rename = "date")]
    Date,
    /// `date-time` defined by [RFC3339](https://tools.ietf.org/html/rfc3339#section-5.6)
    #[serde(rename = "date-time")]
    DateTime,
    /// Hint to UIs to obscure input
    #[serde(rename = "password")]
    Password,
    /// Other type not known by this library. Some common examples are `email`, `uuid`, etc.
    #[serde(untagged)]
    Other(String),
}

// From https://spec.openapis.org/oas/v3.0.3#discriminator-object
/// When request bodies or response payloads may be one of a number of different schemas, a
/// discriminator object can be used to aid in serialization, deserialization, and validation. The
/// discriminator is a specific object in a schema which is used to inform the consumer of the
/// document of an alternative schema based on the value associated with it.
///
/// When using the discriminator, inline schemas will not be considered.
#[derive(Debug, Serialize, Deserialize)]
struct Discriminator {
    /// The name of the property in the payload that will hold the discriminator value.
    #[serde(rename = "propertyName")]
    property_name: String,
    /// An object to hold mappings between payload values and schema names or references.
    mapping: Option<HashMap<String, String>>,
}

// From https://spec.openapis.org/oas/v3.0.3#xml-object
/// A metadata object that allows for more fine-tuned XML model definitions.
#[derive(Debug, Serialize, Deserialize)]
struct XML {
    /// Replaces the name of the element/attribute used for the described schema property. When
    /// defined within items, it will affect the name of the individual XML elements within the
    /// list. When defined alongside type being array (outside the items), it will affect the
    /// wrapping element around the array.
    #[serde(rename = "name")]
    name: Option<String>,
    /// The URI of the namespace definition. Value MUST be in the form of an absolute URI.
    #[serde(rename = "namespace")]
    namespace: Option<String>,
    /// The prefix to be used for the name.
    #[serde(rename = "prefix")]
    prefix: Option<String>,
    /// Declares whether the property definition translates to an attribute instead of an element.
    /// Default value is false.
    #[serde(rename = "attribute")]
    attribute: Option<bool>,
    /// MAY be used only for an array definition. Signifies whether the array is wrapped (for
    /// example, `<books><book/><book/></books>`) or unwrapped (`<book/><book/>`). Default value is
    /// false. The definition takes effect only when defined alongside type being array (outside the
    /// items).
    #[serde(rename = "wrapped")]
    wrapped: Option<bool>,
}

// From https://spec.openapis.org/oas/v3.0.3#security-scheme-object
/// Defines a security scheme that can be used by the operations. Supported schemes are HTTP
/// authentication, an API key (either as a header or as a query parameter), OAuth2's common flows
/// (implicit, client credentials, password and authorization code) as defined in
/// [RFC6749](https://tools.ietf.org/html/rfc6749), and [OpenID Connect
/// Discovery](https://tools.ietf.org/html/draft-ietf-oauth-discovery-06).
#[derive(Debug, Serialize, Deserialize)]
struct SecurityScheme {
    /// The type of the security scheme. Valid values are "apiKey", "http", "oauth2",
    /// "openIdConnect".
    #[serde(rename = "type")]
    scheme_type: SecuritySchemeType,
    /// A short description for security scheme. CommonMark syntax MAY be used for rich text
    /// representation.
    description: Option<String>,
    /// The name of the header, query or cookie parameter to be used.
    #[serde(rename = "name")]
    name: Option<String>,
    /// The location of the API key. Valid values are "query" or "header" or "cookie".
    #[serde(rename = "in")]
    location: Option<ParameterLocation>,
    /// The name of the HTTP Authorization scheme to be used in the Authorization header as defined
    /// in RFC7235.
    #[serde(rename = "scheme")]
    scheme: Option<String>,
    /// A hint to the client to identify how the bearer token is formatted. Bearer tokens are
    /// usually generated by an authorization server, so this information is primarily for
    /// documentation purposes.
    #[serde(rename = "bearerFormat")]
    bearer_format: Option<String>,
    /// An object containing configuration information for the flow types supported.
    flows: Option<OAuthFlows>,
    /// The OpenID Connect URL to discover OAuth2 configuration values. This MUST be in the form of
    /// a URL.
    #[serde(rename = "openIdConnectUrl")]
    open_id_connect_url: Option<String>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#security-scheme-type
/// The type of the security scheme. Valid values are "apiKey", "http", "oauth2", "openIdConnect".
#[derive(Debug, Serialize, Deserialize)]
enum SecuritySchemeType {
    /// API key
    #[serde(rename = "apiKey")]
    ApiKey,
    /// HTTP
    #[serde(rename = "http")]
    Http,
    /// OAuth2
    #[serde(rename = "oauth2")]
    OAuth2,
    /// OpenID Connect
    #[serde(rename = "openIdConnect")]
    OpenIdConnect,
    /// Other type not known by this library
    #[serde(untagged)]
    Other(String),
}

// From https://spec.openapis.org/oas/v3.0.3#oauth-flows-object
/// Allows configuration of the supported OAuth Flows.
#[derive(Debug, Serialize, Deserialize)]
struct OAuthFlows {
    /// Configuration for the OAuth Implicit flow
    #[serde(rename = "implicit")]
    implicit: Option<OAuthFlow>,
    /// Configuration for the OAuth Resource Owner Password flow
    #[serde(rename = "password")]
    password: Option<OAuthFlow>,
    /// Configuration for the OAuth Client Credentials flow. Previously called application in
    /// OpenAPI 2.0.
    #[serde(rename = "clientCredentials")]
    client_credentials: Option<OAuthFlow>,
    /// Configuration for the OAuth Authorization Code flow. Previously called accessCode in OpenAPI
    /// 2.0.
    #[serde(rename = "authorizationCode")]
    authorization_code: Option<OAuthFlow>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#oauth-flow-object
/// Configuration details for a supported OAuth Flow
#[derive(Debug, Serialize, Deserialize)]
struct OAuthFlow {
    /// The authorization URL to be used for this flow. This MUST be in the form of a URL. This is
    /// required for "implicit" and "authorizationCode" flows.
    #[serde(rename = "authorizationUrl")]
    authorization_url: Option<String>,
    /// The token URL to be used for this flow. This MUST be in the form of a URL. This is required
    /// for "password", "clientCredentials" and "authorizationCode" flows.
    #[serde(rename = "tokenUrl")]
    token_url: Option<String>,
    /// The URL to be used for obtaining refresh tokens. This MUST be in the form of a URL.
    #[serde(rename = "refreshUrl")]
    refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme. A map between the scope name and a
    /// short description for it. The map MAY be empty.
    scopes: Option<HashMap<String, String>>,
    /// Spec Extensions
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#security-requirement-object
/// Lists the required security schemes to execute this operation. The name used for each property
/// MUST correspond to a security scheme declared in the Security Schemes under the Components
/// Object.
///
/// Security Requirement Objects that contain multiple schemes require that all schemes MUST be
/// satisfied for a request to be authorized. This enables support for scenarios where multiple
/// query parameters or HTTP headers are required to convey security information.
///
/// When a list of Security Requirement Objects is defined on the OpenAPI Object or Operation
/// Object, only one of Security Requirement Objects in the list needs to be satisfied to authorize
/// the request.
#[derive(Debug, Serialize, Deserialize)]
struct SecurityRequirement {
    /// Each name MUST correspond to a security scheme which is declared in the Security Schemes
    /// under the Components Object.
    #[serde(flatten)]
    requirements: HashMap<String, Vec<String>>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn load_openapi() {
        let openapi = include_str!("../../data/vi_json_openapi_specification_v8_0_2_0.json");
        let spec: super::OpenAPI = serde_json::from_str(openapi).unwrap();
        let schemas = spec.components.unwrap().schemas.unwrap();
        dbg!(schemas.iter().filter(|i| i.0.starts_with("ArrayOf")).count());
    }
}