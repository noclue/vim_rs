use serde::{Deserialize, Serialize};

// From https://spec.openapis.org/oas/v3.0.3#reference-object
/// OpenAPI 3.0 on multiple places uses reference or actual object. This enum represents this
/// construct.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RefOr<T> {
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
    Ref{
        /// The reference string.
        #[serde(rename = "$ref")]
        reference: String,
        /// An optional, string description. This is non-standard but used in the VIM API definition.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// A `true`` value adds "null" to the allowed type specified by the type keyword, only if type
        /// is explicitly defined within the same Schema Object. Other Schema Object constraints retain
        /// their defined behavior, and therefore may disallow the use of `null` as a value. A `false`
        /// value leaves the specified or default type unmodified. The default value is `false`.
        /// 
        /// This is non-standard but used in the VIM API definition.
        #[serde(skip_serializing_if = "Option::is_none")]
        nullable: Option<bool>,
    },
    /// A value
    Val(Box<T>),
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use serde_json::json;

    #[test]
    fn test_ref_or_serialization() {
        let ref_or: RefOr<Schema> = RefOr::Ref{reference: "#/components/schemas/Animal".to_string(), description: None, nullable: None};
        assert_eq!(serde_json::to_value(ref_or).unwrap(), json!({"$ref": "#/components/schemas/Animal"}));
    }

    #[test]
    fn test_ref_or_deserialization() {
        let ref_or = json!({"$ref": "#/components/schemas/Animal"});
        let ref_or: RefOr<Schema> = serde_json::from_value(ref_or).unwrap();
        assert_eq!(ref_or, RefOr::Ref{reference: "#/components/schemas/Animal".to_string(), description: None, nullable: None});
    }

    #[test]
    fn test_ref_or_deserialization_with_description() {
        let ref_or = json!({"description": "test", "$ref": "#/components/schemas/Animal", "nullable": true});
        let ref_or: RefOr<Schema> = serde_json::from_value(ref_or).unwrap();
        // `description` is ignored as the `$ref` schema has no support for additional fields
        assert_eq!(ref_or, RefOr::Ref{reference: "#/components/schemas/Animal".to_string(), description: Some("test".to_string()), nullable: Some(true)});

    }
}