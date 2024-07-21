use std::fmt::Debug;
use convert_case::{Case, Casing};
use crate::oas30::*;
use indexmap::IndexMap;
use super::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("Unsupported schema reference type: {0}")]
    UnsupportedSchemaReferenceType(String),
    #[error("Schema '{path}' validation error: {source}")]
    SchemaValidationError{
        path: String,
        source: crate::oas30::Error},
    #[error("Array index '{index}' processing error: {source}")]
    ArrayProcessingError {
        index: usize,
        source: Box<Error>,
    },
    #[error("Invalid type: expected '{expected}', got '{actual:?}'")]
    InvalidType {
        expected: String,
        actual: String,
    },
    #[error("Invalid reference: {0}")]
    InvalidReference(String),
    #[error("Unsupported type: {0}")]
    UnsupportedType(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Missing data format for type: {0}")]
    MissingFormat(String),
}

// Result is a type alias for handling errors.
pub type Result<T> = std::result::Result<T, Error>;


pub fn load_vim_model(model: &OpenAPI) -> Result<VimModel> {
    let mut vim_model = VimModel {
        enums: IndexMap::new(),
        structs: IndexMap::new(),
        request_types: IndexMap::new(),
        box_types_by_parent: IndexMap::new(),
    };
    let Some(ref components) = model.components.as_ref() else {
        return Err(Error::MissingField("#/components".to_string()));
    };
    let Some(schemas) = components.schemas.as_ref() else {
        return Err(Error::MissingField("#/components/schemas".to_string()));
    };

    for (schema_name, ref_or_schema) in schemas {
        let RefOr::Val(ref schema) = ref_or_schema else {
            return Err(Error::UnsupportedSchemaReferenceType(schema_name.to_string()));
        };
        let schema = &schema.as_ref();
        schema.validate().map_err(|e| Error::SchemaValidationError{path: schema_name.to_string(), source: e})?;

        match schema {
            Schema { // Enum arm
                schema_type: Some(SchemaType::String),
                enumeration: Some(values),
                ..
            } => {
                let enumeration = Enum {
                    name: schema_name.to_string(),
                    description: schema.description.clone(),
                    variants: as_string_values(values)?,

                };
                vim_model.enums.insert(schema_name.to_string(), enumeration);
            },
            Schema { // Request Type arm
                schema_type: Some(SchemaType::Object),
                all_of: None,
                ..
            } if get_parent_schema(schema) == None && schema_name != "Any" => {
                vim_model.request_types.insert(schema_name.to_string(), build_request_type(schema_name, schema)?);
            }
            _ => {
                // TODO
            }
        }

    }
    Ok(vim_model)
}


static empty_vec: Vec<String> = Vec::new();
static none_str: String = String::new();

fn build_request_type(schema_name: &str, schema: &Schema) -> Result<RequestType> {
    let mut result = IndexMap::new();
    let Some(ref properties) = schema.properties else {
        return Err(Error::MissingField(format!("#/components/schemas/{schema_name}/properties")));
    };
    let required = schema.required.as_ref().unwrap_or(&empty_vec);
    for (property_name, ref_or_property) in properties {
        let description = match ref_or_property {
            RefOr::Val(property) => property.description.as_ref(),
            RefOr::Ref { .. } => None,
        };
        let property = Property {
            name: property_name.clone(),
            vim_type : VimType::try_from(ref_or_property)?,
            optional: !required.contains(&property_name.to_string()),
            description: description.cloned(),
        };
        result.insert(property_name.to_string(), property);
    }
    Ok(RequestType {
        name: schema_name.to_string(),
        description: schema.description.clone(),
        properties: result,
    })
}


fn as_string_values(values: &Vec<serde_json::Value>) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    for (i, value) in values.iter().enumerate() {
        let value = as_string(value).or_else(|e| Err(Error::ArrayProcessingError { index: i, source: Box::new(e) }))?;
        result.push(value);
    }
    Ok(result)
}

fn as_string(value: &serde_json::Value) -> Result<String> {
    let value = value.as_str().ok_or(Error::InvalidType { expected: "string".to_string(), actual: value.to_string() })?;
    Ok(value.to_string())
}

fn get_parent_schema(schema: &Schema) -> Option<&String> {
    let all_of = schema.all_of.as_ref()?;
    if all_of.len() == 1 {
        if let RefOr::Ref { ref reference } = &all_of[0] {
            return Some(reference)        
        }
    }
    None
}

pub fn reference_to_schema_name(reference: &String) -> Result<&str> {
        if !reference.starts_with("#/components/schemas/") {
        return Err(Error::InvalidReference(reference.to_string()));
    }
    let schema_name = reference.trim_start_matches("#/components/schemas/");
    Ok(schema_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_enum_load() {
        let value = json!({
            "openapi": "3.0.3", 
            "info": {"title": "test", "version": "1.0.0"},
            "paths": {},
            "components": {
                "schemas": {
                    "test": {                        
                        "description": "test",
                        "type": "string", 
                        "enum": ["foo", "bar"]
                    }
                }
            }
        });
        let open_api = serde_json::from_value::<OpenAPI>(value).unwrap();
        let vim_model = load_vim_model(&open_api).unwrap();
        assert_eq!(vim_model.enums.len(), 1);
        let test_enum = vim_model.enums.get("test").unwrap();
        assert_eq!(test_enum.description, Some("test".to_string()));
        assert_eq!(test_enum.variants, vec!["foo".to_string(), "bar".to_string()]);
    }

    #[test]
    fn test_request_type_load() {
        let value = json!({
            "openapi": "3.0.3", 
            "info": {"title": "test", "version": "1.0.0"},
            "paths": {},
            "components": {
                "schemas": {
                    "TestRequestType": {                        
                        "description": "test",
                        "type": "object", 
                        "properties": {
                            "foo": {
                                "description": "test description",
                                "type": "string"
                            },
                            "bar": {
                                "$ref": "#/components/schemas/BaseConfigInfoDiskFileBackingInfoProvisioningType_enum"
                            }
                        },
                        "required": ["foo"]
                    }
                }
            }
        });
        let open_api = serde_json::from_value::<OpenAPI>(value).unwrap();
        let vim_model = load_vim_model(&open_api).unwrap();
        assert_eq!(vim_model.request_types.len(), 1);
        let test_request_type = vim_model.request_types.get("TestRequestType").unwrap();
        assert_eq!(test_request_type.name, "TestRequestType");
        assert_eq!(test_request_type.description, Some("test".to_string()));
        assert_eq!(test_request_type.properties.len(), 2);
        let test_request_type_prop = test_request_type.properties.get("foo").unwrap();
        assert_eq!(test_request_type_prop.name, "foo");
        assert_eq!(test_request_type_prop.vim_type, VimType::String);
        assert_eq!(test_request_type_prop.optional, false);
        assert_eq!(test_request_type_prop.description, Some("test description".to_string()));

        let test_request_type_prop = test_request_type.properties.get("bar").unwrap();
        assert_eq!(test_request_type_prop.name, "bar");
        assert_eq!(test_request_type_prop.vim_type, VimType::Struct("BaseConfigInfoDiskFileBackingInfoProvisioningType_enum".to_string()));
        assert_eq!(test_request_type_prop.optional, true);
        assert_eq!(test_request_type_prop.description, None);
    }
}