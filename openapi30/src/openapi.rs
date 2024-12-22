use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use super::*;

// From https://spec.openapis.org/oas/v3.0.3#schema
/// This is the root document object of the OpenAPI document.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OpenAPI {
    /// This string MUST be the semantic version number of the OpenAPI Specification version that
    /// the OpenAPI document uses. The openapi field SHOULD be used by tooling specifications and
    /// clients to interpret the OpenAPI document. This is not related to the API info.version
    /// string.
    pub openapi: String,
    /// Provides metadata about the API. The metadata MAY be used by tooling as required.
    pub info: Info,
    /// An array of Server Objects, which provide connectivity information to a target server. If
    /// the servers property is not provided, or is an empty array, the default value would be a
    /// Server Object with a url value of /.
    pub servers: Option<Vec<Server>>,
    /// The available paths and operations for the API.
    pub paths: IndexMap<String, PathItem>,
    /// An element to hold various schemas for the specification.
    pub components: Option<Components>,
    /// A declaration of which security mechanisms can be used across the API. The list of values
    /// includes alternative security requirement objects that can be used. Only one of the security
    /// requirement objects need to be satisfied to authorize a request. Individual operations can
    /// override this definition.
    pub security: Option<Vec<SecurityRequirement>>,
    /// A list of tags used by the specification with additional metadata. The order of the tags can
    /// be used to reflect on their order by the parsing tools. Not all tags that are used by the
    /// Operation Object must be declared. The tags that are not declared MAY be organized randomly
    /// or based on the tools' logic. Each tag name in the list MUST be unique.
    pub tags: Option<Vec<Tag>>,
    /// Additional external documentation.
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#info-object
/// The object provides metadata about the API. The metadata MAY be used by the clients if needed,
/// and MAY be presented in editing or documentation generation tools for convenience.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Info {
    /// The title of the API.
    pub title: String,
    /// The version of the OpenAPI document (which is distinct from the OpenAPI Specification
    /// version or the API implementation version).
    pub version: String,
    /// A short description of the API. CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API. MUST be in the format of a URL.
    pub terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    pub license: Option<License>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#contact-object
/// Contact information for the exposed API.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    pub name: Option<String>,
    /// The URL pointing to the contact information. MUST be in the format of a URL.
    pub url: Option<String>,
    /// The email address of the contact person/organization. MUST be in the format of an email
    /// address.
    pub email: Option<String>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#license-object
/// License information for the exposed API.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct License {
    /// The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API. MUST be in the format of a URL.
    pub url: Option<String>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

// from https://spec.openapis.org/oas/v3.0.3#components-object
/// Holds a set of reusable objects for different aspects of the OAS. All objects defined within the
/// components object will have no effect on the API unless they are explicitly referenced from
/// properties outside the components object.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Components {
    /// An object to hold reusable Schema Objects.
    pub schemas: Option<IndexMap<String, RefOr<Schema>>>,
    /// An object to hold reusable Response Objects.
    pub responses: Option<IndexMap<String, RefOr<Response>>>,
    /// An object to hold reusable Parameter Objects.
    pub parameters: Option<IndexMap<String, RefOr<Parameter>>>,
    /// An object to hold reusable Example Objects.
    pub examples: Option<IndexMap<String, RefOr<Example>>>,
    /// An object to hold reusable Request Body Objects.
    #[serde(rename = "requestBodies")]
    pub request_bodies: Option<IndexMap<String, RefOr<RequestBody>>>,
    /// An object to hold reusable Header Objects.
    pub headers: Option<IndexMap<String, RefOr<Header>>>,
    /// An object to hold reusable Security Scheme Objects.
    #[serde(rename = "securitySchemes")]
    pub security_schemes: Option<IndexMap<String, RefOr<SecurityScheme>>>,
    /// An object to hold reusable Link Objects.
    pub links: Option<IndexMap<String, RefOr<Link>>>,
    /// An object to hold reusable Callback Objects.
    pub callbacks: Option<IndexMap<String, RefOr<Callback>>>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

// From https://spec.openapis.org/oas/v3.0.3#tag-object
/// Adds metadata to a single tag that is used by the Operation Object. It is not mandatory to have
/// a Tag Object per tag used there.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    /// The name of the tag.
    pub name: String,
    /// A short description for the tag. CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    pub fn load_openapi() {
        let openapi = include_str!("../../gen/data/vi_json_openapi_specification_v8_0_2_0.json");
        let spec: super::OpenAPI = serde_json::from_str(openapi).unwrap();
        let schemas = spec.components.unwrap().schemas.unwrap();
        dbg!(schemas
            .iter()
            .filter(|i| i.0.starts_with("ArrayOf"))
            .count());
    }

    #[test]
    fn test_schema_type_deserialization() {
        let schema_type = json!("integer");
        let schema_type: SchemaType = serde_json::from_value(schema_type).unwrap();
        assert_eq!(schema_type, SchemaType::Integer);
        let schema_type = json!("animal");
        let schema_type: SchemaType = serde_json::from_value(schema_type).unwrap();
        assert_eq!(schema_type, SchemaType::Other("animal".to_string()));
    }

    #[test]
    fn test_ref_or_deserialization() {
        let ref_or = json!({"$ref": "#/components/schemas/Animal"});
        let ref_or: RefOr<Schema> = serde_json::from_value(ref_or).unwrap();
        assert_eq!(ref_or, RefOr::Ref{reference:"#/components/schemas/Animal".to_string(), description: None});
        let ref_or = json!({"type": "string"});
        let ref_or: RefOr<Schema> = serde_json::from_value(ref_or).unwrap();
        assert_eq!(ref_or, RefOr::Val(Box::new(Schema {
            schema_type: Some(SchemaType::String),
            ..Default::default()
        })));
    }

    #[test]
    fn test_schema_validation() {
        let valid_string = "{
            \"type\": \"string\",
            \"title\": \"Test\",
            \"maxLength\": 10,
            \"minLength\": 5,
            \"pattern\": \"^a.*$\",
            \"enum\": [\"a\", \"b\"],
            \"format\": \"date\",
            \"default\": \"a\",
            \"nullable\": true,
            \"example\": \"a\",
            \"deprecated\": true,
            \"extensions\": {\"x-test\": 1}
        }";
        let schema: Schema = serde_json::from_str(valid_string).unwrap();
        assert!(schema.validate().is_ok());
        let invalid_string = "{
            \"type\": \"string\",
            \"title\": \"Test\",
            \"items\": {\"type\": \"string\"}
        }";
        let schema: Schema = serde_json::from_str(invalid_string).unwrap();
        assert_eq!(
            schema.validate(),
            Err(Error::SchemaInvalidField(SchemaType::String, "items".to_string()))
        );
    }
}