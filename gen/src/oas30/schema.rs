use serde::{Deserialize, Serialize};
use std::fmt::Display;
use indexmap::IndexMap;
use super::*;

// From https://spec.openapis.org/oas/v3.0.3#schema-object
/// The Schema Object allows the definition of input and output data types. These types can be
/// objects, but also primitives and arrays. This object is an extended subset of the JSON Schema
/// Specification Wright Draft 00.
///
/// For more information about the properties, see JSON Schema Core and JSON Schema Validation.
/// Unless stated otherwise, the property definitions follow the JSON Schema.
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Schema {
    /// The value of "title" MUST be a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    // Validation rules from
    // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5
    /// The value of "multipleOf" MUST be a number, strictly greater than 0. A numeric instance is
    /// only valid if division by this keyword's value results in an integer.
    #[serde(rename = "multipleOf", skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    /// The value of "maximum" MUST be a number, representing an upper limit for a numeric instance.
    /// If the instance is a number, then this keyword validates if "exclusiveMaximum" is true and
    /// instance is less than the provided value, or else if the instance is less than or exactly
    /// equal to the provided value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    /// The value of "exclusiveMaximum" MUST be a boolean, representing whether the limit in
    /// "maximum" is exclusive or not.  An undefined value is the same as false.
    ///
    /// If "exclusiveMaximum" is true, then a numeric instance SHOULD NOT be equal to the value
    /// specified in "maximum".  If "exclusiveMaximum" is false (or not specified), then a numeric
    /// instance MAY be equal to the value of "maximum".
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    /// The value of "minimum" MUST be a number, representing a lower limit for a numeric instance.
    ///
    /// If the instance is a number, then this keyword validates if "exclusiveMinimum" is true and
    /// instance is greater than the provided value, or else if the instance is greater than or
    /// exactly equal to the provided value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    /// The value of "exclusiveMinimum" MUST be a boolean, representing whether the limit in
    /// "minimum" is exclusive or not.  An undefined value is the same as false.
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    /// The value of "maxLength" MUST be a non-negative integer. A string instance is valid against
    /// this keyword if its length is less than, or equal to, the value of this keyword.
    ///
    /// The length of a string instance is defined as the number of its characters as defined by RFC
    /// 7159 [RFC7159].
    ///
    /// This keyword can be used to control the maximum length of strings.
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
    /// The value of "minLength" MUST be a non-negative integer. A string instance is valid against
    /// this keyword if its length is greater than, or equal to, the value of this keyword.
    ///
    /// The length of a string instance is defined as the number of its characters as defined by RFC
    /// 7159 [RFC7159].
    ///
    /// "minLength", if absent, may be considered as 0.
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,
    /// This string SHOULD be a valid regular expression, according to the ECMA 262 regular
    /// expression dialect.
    ///
    /// A string instance is considered valid if the regular expression matches the instance
    /// successfully. Recall: regular expressions are not implicitly anchored.
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// The value of "maxItems" MUST be a non-negative integer. An array instance is valid against
    /// "maxItems" if its size is less than, or equal to, the value of this keyword.
    ///
    /// This keyword can be used to control the maximum number of items in an array.
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u64>,
    /// The value of "minItems" MUST be a non-negative integer. An array instance is valid against
    /// "minItems" if its size is greater than, or equal to, the value of this keyword.
    ///
    /// This keyword can be used to control the minimum number of items in an array.
    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u64>,
    /// The value of "uniqueItems" MUST be a boolean. If this keyword has boolean value false, the
    /// instance validates successfully. If it has boolean value true, the instance validates
    /// successfully if all of its elements are unique.
    ///
    /// If not present, this keyword may be considered as being present with boolean value false.
    #[serde(rename = "uniqueItems", skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    /// The value of "maxProperties" MUST be a non-negative integer. An object instance is valid
    /// against "maxProperties" if its number of properties is less than, or equal to, the value of
    /// this keyword.
    ///
    /// This keyword can be used to control the maximum number of properties in an object.
    #[serde(rename = "maxProperties", skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<u64>,
    /// The value of "minProperties" MUST be a non-negative integer. An object instance is valid
    /// against "minProperties" if its number of properties is greater than, or equal to, the value
    /// of this keyword.
    ///
    /// This keyword can be used to control the minimum number of properties in an object.
    #[serde(rename = "minProperties", skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<u64>,
    /// The value of this keyword MUST be an array.  This array MUST have at least one element.
    /// Elements of this array MUST be strings, and MUST be unique.
    ///
    /// An object instance is valid against this keyword if its property set contains all elements
    /// in this keyword's array value.
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    // from
    // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.20
    /// The value of this keyword MUST be an array.  This array SHOULD have at least one element.
    /// Elements in the array SHOULD be unique.
    ///
    /// Elements in the array MAY be of any type, including null.
    ///
    /// An instance validates successfully against this keyword if its value is equal to one of the
    /// elements in this keyword's array value.
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enumeration: Option<Vec<serde_json::Value>>,
    /// The value of "type" MUST be a string, representing the type of the schema.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<SchemaType>,
    /// This keyword's value MUST be an array. This array SHOULD have at least one element.
    ///
    /// Elements of the array MUST be objects. Inline or referenced schema MUST be of a Schema
    /// Object and not a standard JSON Schema.
    ///
    /// An instance validates successfully against this keyword if it validates successfully against
    /// all schemas defined by this keyword's value.
    #[serde(rename = "allOf", skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<RefOr<Schema>>>,
    /// This keyword's value MUST be an array. This array SHOULD have at least one element.
    ///
    /// Elements of the array MUST be objects. Inline or referenced schema MUST be of a Schema
    /// Object and not a standard JSON Schema.
    ///
    /// An instance validates successfully against this keyword if it validates successfully against
    /// exactly one schema defined by this keyword's value.
    #[serde(rename = "oneOf", skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<RefOr<Schema>>>,
    /// This keyword's value MUST be an array. This array SHOULD have at least one element.
    ///
    /// Elements of the array MUST be objects. Inline or referenced schema MUST be of a Schema
    /// Object and not a standard JSON Schema.
    ///
    /// An instance validates successfully against this keyword if it validates successfully against
    /// at least one schema defined by this keyword's value.
    #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<RefOr<Schema>>>,
    /// This keyword's value MUST be an object.  Inline or referenced schema MUST be of a Schema
    /// Object and not a standard JSON Schema.
    ///
    /// An instance is valid against this keyword if it fails to validate successfully against the
    /// schema defined by this keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<RefOr<Schema>>,
    /// The value of "items" MUST be an object, and that object MUST be a valid JSON Schema. This
    /// object defines the schema for items in arrays. `items` MUST be present if the `type` is
    /// `array`.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<RefOr<Schema>>,
    /// The value of "properties" MUST be an object. Each value of this object MUST be an object,
    /// and each object MUST be a valid schema.
    ///
    /// Property definitions MUST be a Schema Object and not a standard JSON Schema (inline or
    /// referenced).
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<IndexMap<String, RefOr<Schema>>>,
    /// The value of "additionalProperties" MUST be a boolean or a schema. If `true` is provided,
    /// then the object can have any property. If `false` is provided, then the object cannot have
    /// any additional properties. If a schema is provided, then the object properties MUST match
    /// the schema.
    ///
    /// Inline or referenced schema MUST be of a Schema Object and not a standard JSON Schema.
    #[serde(rename = "additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<RefOr<Schema>>,
    /// The value of "description" MUST be a string. a description will provide explanation about
    /// the purpose of the instance described by this schema.
    ///
    /// [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// See [Data Type Formats](https://spec.openapis.org/oas/v3.0.3#dataTypeFormat) for further
    /// details. While relying on JSON Schema’s defined formats, the OAS offers a few additional
    /// predefined formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DataFormat>,
    /// The default value represents what would be assumed by the consumer of the input as the value
    /// of the schema if one is not provided. Unlike JSON Schema, the value MUST conform to the
    /// defined type for the Schema Object defined at the same level. For example, if type is
    /// string, then default can be "foo" but cannot be 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    /// A `true`` value adds "null" to the allowed type specified by the type keyword, only if type
    /// is explicitly defined within the same Schema Object. Other Schema Object constraints retain
    /// their defined behavior, and therefore may disallow the use of `null` as a value. A `false`
    /// value leaves the specified or default type unmodified. The default value is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    /// Adds support for polymorphism. The discriminator is an object name that is used to
    /// differentiate between other schemas which may satisfy the payload description. See
    /// [Composition and
    /// Inheritance](https://spec.openapis.org/oas/v3.0.3#composition-and-inheritance-polymorphism)
    /// for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<Discriminator>,
    /// Relevant only for Schema `"properties"` definitions. Declares the property as "read only".
    /// This means that it MAY be sent as part of a response but SHOULD NOT be sent as part of the
    /// request. If the property is marked as readOnly being `true`, then the `writeOnly` property
    /// MUST be `false`. Default value is `false`.
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Relevant only for Schema `"properties"` definitions. Declares the property as "write only".
    /// Therefore, it MAY be sent as part of a request but SHOULD NOT be sent as part of the
    /// response. If the property is marked as `writeOnly` being `true`, then the `readOnly`
    /// property MUST be `false`. Default value is `false`.
    #[serde(rename = "writeOnly", skip_serializing_if = "Option::is_none")]
    pub write_only: Option<bool>,
    /// This may be used only on properties schemas. It has no effect on root schemas. Adds
    /// additional metadata to describe the XML representation of this property.
    #[serde(rename = "xml", skip_serializing_if = "Option::is_none")]
    pub xml: Option<XML>,
    /// Additional external documentation for this schema.
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// A free-form property to include an example of an instance for this schema. To represent
    /// examples that cannot be naturally represented in JSON or YAML, a string value can be used to
    /// contain the example with escaping where necessary.
    #[serde(rename = "example", skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    /// Specifies that a schema is deprecated and SHOULD be transitioned out of usage. Default value
    /// is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// Spec Extensions
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}


impl Validate for Schema {
    fn validate(&self) -> Result<(), super::aux::Error> {
        let Some(schema_type) = &self.schema_type else {
            return Err(super::aux::Error::MissingField("type".to_string()));
        };
        match schema_type {
            SchemaType::String => self.validate_string(),
            SchemaType::Boolean => self.validate_boolean(),
            SchemaType::Number => self.validate_number(),
            SchemaType::Integer => self.validate_integer(),
            SchemaType::Object => self.validate_object(),
            SchemaType::Array => self.validate_array(),
            _ => Ok(()),
        }
    }
}

impl Schema {
    const TITLE: u64 = 1;
    const MULTIPLE_OF: u64 = 2;
    const MAXIMUM: u64 = 4;
    const EXCLUSIVE_MAXIMUM: u64 = 8;
    const MINIMUM: u64 = 16;
    const EXCLUSIVE_MINIMUM: u64 = 32;
    const MAX_LENGTH: u64 = 64;
    const MIN_LENGTH: u64 = 128;
    const PATTERN: u64 = 256;
    const MAX_ITEMS: u64 = 512;
    const MIN_ITEMS: u64 = 1024;
    const UNIQUE_ITEMS: u64 = 2048;
    const MAX_PROPERTIES: u64 = 4096;
    const MIN_PROPERTIES: u64 = 8192;
    const REQUIRED: u64 = 16384;
    const ENUMERATION: u64 = 32768;
    const SCHEMA_TYPE: u64 = 65536;
    const ALL_OF: u64 = 131072;
    const ONE_OF: u64 = 262144;
    const ANY_OF: u64 = 524288;
    const NOT: u64 = 1048576;
    const ITEMS: u64 = 2097152;
    const PROPERTIES: u64 = 4194304;
    const ADDITIONAL_PROPERTIES: u64 = 8388608;
    const DESCRIPTION: u64 = 16777216;
    const FORMAT: u64 = 33554432;
    const DEFAULT: u64 = 67108864;
    const NULLABLE: u64 = 134217728;
    const DISCRIMINATOR: u64 = 268435456;
    const READ_ONLY: u64 = 536870912;
    const WRITE_ONLY: u64 = 1073741824;
    const XML: u64 = 2147483648;
    const EXTERNAL_DOCS: u64 = 4294967296;
    const EXAMPLE: u64 = 8589934592;
    const DEPRECATED: u64 = 17179869184;
    const EXTENSIONS: u64 = 34359738368;

    fn validate_number(&self) -> Result<(), Error> {
        self.validate_numeric(SchemaType::Number)?;
        if let Some(enum_items) = &self.enumeration {
            self.validate_enum_values(enum_items, serde_json::Value::is_number)?;
        }
        Ok(())
    }
    fn validate_integer(&self) -> Result<(), Error> {
        self.validate_numeric(SchemaType::Integer)?;
        if let Some(enum_items) = &self.enumeration {
            self.validate_enum_values(enum_items, serde_json::Value::is_i64)?;
        }
        Ok(())
    }
    fn validate_numeric(&self, schema_type: SchemaType) -> Result<(), Error> {
        self.validate_allowed(
            schema_type,
            Self::TITLE
                | Self::MULTIPLE_OF
                | Self::MAXIMUM
                | Self::EXCLUSIVE_MAXIMUM
                | Self::MINIMUM
                | Self::EXCLUSIVE_MINIMUM
                | Self::ENUMERATION
                | Self::SCHEMA_TYPE
                | Self::DESCRIPTION
                | Self::FORMAT
                | Self::DEFAULT
                | Self::NULLABLE
                | Self::READ_ONLY
                | Self::WRITE_ONLY
                | Self::XML
                | Self::EXTERNAL_DOCS
                | Self::EXAMPLE
                | Self::DEPRECATED
                | Self::EXTENSIONS,
        )
    }
    fn validate_string(&self) -> Result<(), Error> {
        self.validate_allowed(
            SchemaType::String,
            Self::TITLE
                | Self::MAX_LENGTH
                | Self::MIN_LENGTH
                | Self::PATTERN
                | Self::ENUMERATION
                | Self::SCHEMA_TYPE
                | Self::DESCRIPTION
                | Self::FORMAT
                | Self::DEFAULT
                | Self::NULLABLE
                | Self::READ_ONLY
                | Self::WRITE_ONLY
                | Self::XML
                | Self::EXTERNAL_DOCS
                | Self::EXAMPLE
                | Self::DEPRECATED
                | Self::EXTENSIONS,
        )?;
        if let Some(enum_items) = &self.enumeration {
            self.validate_enum_values(enum_items, serde_json::Value::is_string)?;
        }
        Ok(())
    }
    fn validate_boolean(&self) -> Result<(), Error> {
        self.validate_allowed(
            SchemaType::Boolean,
            Self::TITLE
                | Self::SCHEMA_TYPE
                | Self::DESCRIPTION
                | Self::DEFAULT
                | Self::NULLABLE
                | Self::READ_ONLY
                | Self::WRITE_ONLY
                | Self::XML
                | Self::EXTERNAL_DOCS
                | Self::EXAMPLE
                | Self::DEPRECATED
                | Self::EXTENSIONS,
        )
    }
    fn validate_object(&self) -> Result<(), Error> {
        self.validate_allowed(
            SchemaType::Object,
            Self::TITLE
                | Self::MAX_PROPERTIES
                | Self::MIN_PROPERTIES
                | Self::REQUIRED
                | Self::SCHEMA_TYPE
                | Self::ALL_OF
                | Self::ONE_OF
                | Self::ANY_OF
                | Self::NOT
                | Self::PROPERTIES
                | Self::ADDITIONAL_PROPERTIES
                | Self::DESCRIPTION
                | Self::DEFAULT
                | Self::NULLABLE
                | Self::DISCRIMINATOR
                | Self::READ_ONLY
                | Self::WRITE_ONLY
                | Self::XML
                | Self::EXTERNAL_DOCS
                | Self::EXAMPLE
                | Self::DEPRECATED
                | Self::EXTENSIONS,
        )
    }

    fn validate_array(&self) -> Result<(), Error> {
        self.validate_allowed(
            SchemaType::Array,
            Self::TITLE
                | Self::MAX_ITEMS
                | Self::MIN_ITEMS
                | Self::UNIQUE_ITEMS
                | Self::ITEMS
                | Self::SCHEMA_TYPE
                | Self::DESCRIPTION
                | Self::DEFAULT
                | Self::NULLABLE
                | Self::READ_ONLY
                | Self::WRITE_ONLY
                | Self::XML
                | Self::EXTERNAL_DOCS
                | Self::EXAMPLE
                | Self::DEPRECATED
                | Self::EXTENSIONS,
        )
    }

    fn validate_enum_values<F>(&self, items: &Vec<serde_json::Value>, check: F) -> Result<(), Error> 
    where
        F: Fn(&serde_json::Value) -> bool
    {
        for (i, item) in items.iter().enumerate() {
            if !check(item) {
                return Err(Error::SchemaInvalidEnumValue{index:i, value: item.to_string()});
            }
        }
        Ok(())
    }
    fn validate_allowed(&self, schema_type: SchemaType, allowed: u64) -> Result<(), Error> {
        if allowed & Self::TITLE == 0 && self.title.is_some() {
            return Err(Error::SchemaInvalidField(schema_type, "title".to_string()));
        }
        if allowed & Self::MULTIPLE_OF == 0 && self.multiple_of.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "multipleOf".to_string(),
            ));
        }
        if allowed & Self::MAXIMUM == 0 && self.maximum.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "maximum".to_string(),
            ));
        }
        if allowed & Self::EXCLUSIVE_MAXIMUM == 0 && self.exclusive_maximum.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "exclusiveMaximum".to_string(),
            ));
        }
        if allowed & Self::MINIMUM == 0 && self.minimum.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "minimum".to_string(),
            ));
        }
        if allowed & Self::EXCLUSIVE_MINIMUM == 0 && self.exclusive_minimum.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "exclusiveMinimum".to_string(),
            ));
        }
        if allowed & Self::MAX_LENGTH == 0 && self.max_length.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "maxLength".to_string(),
            ));
        }
        if allowed & Self::MIN_LENGTH == 0 && self.min_length.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "minLength".to_string(),
            ));
        }
        if allowed & Self::PATTERN == 0 && self.pattern.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "pattern".to_string(),
            ));
        }
        if allowed & Self::MAX_ITEMS == 0 && self.max_items.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "maxItems".to_string(),
            ));
        }
        if allowed & Self::MIN_ITEMS == 0 && self.min_items.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "minItems".to_string(),
            ));
        }
        if allowed & Self::UNIQUE_ITEMS == 0 && self.unique_items.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "uniqueItems".to_string(),
            ));
        }
        if allowed & Self::MAX_PROPERTIES == 0 && self.max_properties.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "maxProperties".to_string(),
            ));
        }
        if allowed & Self::MIN_PROPERTIES == 0 && self.min_properties.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "minProperties".to_string(),
            ));
        }
        if allowed & Self::REQUIRED == 0 && self.required.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "required".to_string(),
            ));
        }
        if allowed & Self::ENUMERATION == 0 && self.enumeration.is_some() {
            return Err(Error::SchemaInvalidField(schema_type, "enum".to_string()));
        }
        if allowed & Self::SCHEMA_TYPE == 0 && self.schema_type.is_some() {
            return Err(Error::SchemaInvalidField(schema_type, "type".to_string()));
        }
        if allowed & Self::ALL_OF == 0 && self.all_of.is_some() {
            return Err(Error::SchemaInvalidField(schema_type, "allOf".to_string()));
        }
        if allowed & Self::ONE_OF == 0 && self.one_of.is_some() {
            return Err(Error::SchemaInvalidField(schema_type, "oneOf".to_string()));
        }
        if allowed & Self::ANY_OF == 0 && self.any_of.is_some() {
            return Err(Error::SchemaInvalidField(schema_type, "anyOf".to_string()));
        }
        if allowed & Self::NOT == 0 && self.not.is_some() {
            return Err(Error::SchemaInvalidField(schema_type, "not".to_string()));
        }
        if allowed & Self::ITEMS == 0 && self.items.is_some() {
            return Err(Error::SchemaInvalidField(schema_type, "items".to_string()));
        }
        if allowed & Self::PROPERTIES == 0 && self.properties.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "properties".to_string(),
            ));
        }
        if allowed & Self::ADDITIONAL_PROPERTIES == 0 && self.additional_properties.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "additionalProperties".to_string(),
            ));
        }
        if allowed & Self::DESCRIPTION == 0 && self.description.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "description".to_string(),
            ));
        }
        if allowed & Self::FORMAT == 0 && self.format.is_some() {
            return Err(Error::SchemaInvalidField(schema_type, "format".to_string()));
        }
        if allowed & Self::DEFAULT == 0 && self.default.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "default".to_string(),
            ));
        }
        if allowed & Self::NULLABLE == 0 && self.nullable.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "nullable".to_string(),
            ));
        }
        if allowed & Self::DISCRIMINATOR == 0 && self.discriminator.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "discriminator".to_string(),
            ));
        }
        if allowed & Self::READ_ONLY == 0 && self.read_only.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "readOnly".to_string(),
            ));
        }
        if allowed & Self::WRITE_ONLY == 0 && self.write_only.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "writeOnly".to_string(),
            ));
        }
        if allowed & Self::XML == 0 && self.xml.is_some() {
            return Err(Error::SchemaInvalidField(schema_type, "xml".to_string()));
        }
        if allowed & Self::EXTERNAL_DOCS == 0 && self.external_docs.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "externalDocs".to_string(),
            ));
        }
        if allowed & Self::EXAMPLE == 0 && self.example.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "example".to_string(),
            ));
        }
        if allowed & Self::DEPRECATED == 0 && self.deprecated.is_some() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "deprecated".to_string(),
            ));
        }
        if allowed & Self::EXTENSIONS == 0 && !self.extensions.is_empty() {
            return Err(Error::SchemaInvalidField(
                schema_type,
                "extensions".to_string(),
            ));
        }

        Ok(())
    }
}

// From https://spec.openapis.org/oas/v3.0.3#data-types TODO: I am unsure where all schema types are
// defined. Is there "any" type? I found `null`` is not supported and instead `nullable` should be
// used in https://spec.openapis.org/oas/v3.0.3#data-types There is list of 6 types excluding
// `integer` in JSON schema
// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-00#section-4.2 However the OAS
// spec insists on using `integer` as additional type it seems
// https://spec.openapis.org/oas/v3.0.3#data-types
/// The value of "type" MUST be a string, representing the type of the schema.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SchemaType {
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

impl Display for SchemaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaType::Integer => write!(f, "integer"),
            SchemaType::Number => write!(f, "number"),
            SchemaType::String => write!(f, "string"),
            SchemaType::Boolean => write!(f, "boolean"),
            SchemaType::Object => write!(f, "object"),
            SchemaType::Array => write!(f, "array"),
            SchemaType::Other(s) => write!(f, "{}", s),
        }
    }
}

// From https://spec.openapis.org/oas/v3.0.3#data-types
/// Primitives have an optional modifier property: format. OAS uses several known formats to define
/// in fine detail the data type being used. However, to support documentation needs, the format
/// property is an open string-valued property, and can have any value. Formats such as "email",
/// "uuid", and so on, MAY be used even though undefined by this specification. Types that are not
/// accompanied by a format property follow the type definition in the JSON Schema. Tools that do
/// not recognize a specific format MAY default back to the type alone, as if the format is not
/// specified.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DataFormat {
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

impl Display for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataFormat::Int32 => write!(f, "int32"),
            DataFormat::Int64 => write!(f, "int64"),
            DataFormat::Float => write!(f, "float"),
            DataFormat::Double => write!(f, "double"),
            DataFormat::Byte => write!(f, "byte"),
            DataFormat::Binary => write!(f, "binary"),
            DataFormat::Date => write!(f, "date"),
            DataFormat::DateTime => write!(f, "date-time"),
            DataFormat::Password => write!(f, "password"),
            DataFormat::Other(s) => write!(f, "{}", s),
        }
    }
}

// From https://spec.openapis.org/oas/v3.0.3#discriminator-object
/// When request bodies or response payloads may be one of a number of different schemas, a
/// discriminator object can be used to aid in serialization, deserialization, and validation. The
/// discriminator is a specific object in a schema which is used to inform the consumer of the
/// document of an alternative schema based on the value associated with it.
///
/// When using the discriminator, inline schemas will not be considered.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Discriminator {
    /// The name of the property in the payload that will hold the discriminator value.
    #[serde(rename = "propertyName")]
    pub property_name: String,
    /// An object to hold mappings between payload values and schema names or references.
    pub mapping: Option<IndexMap<String, String>>,
}

// From https://spec.openapis.org/oas/v3.0.3#xml-object
/// A metadata object that allows for more fine-tuned XML model definitions.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XML {
    /// Replaces the name of the element/attribute used for the described schema property. When
    /// defined within items, it will affect the name of the individual XML elements within the
    /// list. When defined alongside type being array (outside the items), it will affect the
    /// wrapping element around the array.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The URI of the namespace definition. Value MUST be in the form of an absolute URI.
    #[serde(rename = "namespace")]
    pub namespace: Option<String>,
    /// The prefix to be used for the name.
    #[serde(rename = "prefix")]
    pub prefix: Option<String>,
    /// Declares whether the property definition translates to an attribute instead of an element.
    /// Default value is false.
    #[serde(rename = "attribute")]
    pub attribute: Option<bool>,
    /// MAY be used only for an array definition. Signifies whether the array is wrapped (for
    /// example, `<books><book/><book/></books>`) or unwrapped (`<book/><book/>`). Default value is
    /// false. The definition takes effect only when defined alongside type being array (outside the
    /// items).
    #[serde(rename = "wrapped")]
    pub wrapped: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "type": "string"
        }"#;
        let schema: Schema = serde_json::from_str(json).unwrap();
        assert_eq!(schema.schema_type.unwrap(), SchemaType::String);
    }

    #[test]
    fn test_serialize() {
        let schema = Schema {
            schema_type: Some(SchemaType::String),
            ..Default::default()
        };
        let json = serde_json::to_string(&schema).unwrap();
        assert_eq!(json, r#"{"type":"string"}"#);
    }
}