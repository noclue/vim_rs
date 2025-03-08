use super::*;
use crate::vim_model::struct_order::reorder_structs;
use indexmap::IndexMap;
use openapi30::*;
use std::cell::RefCell;
use std::fmt::Debug;
use log::error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("Unsupported schema reference type: {0}")]
    UnsupportedSchemaReferenceType(String),
    #[error("Schema '{path}' validation error: {source}")]
    SchemaValidation {
        path: String,
        source: openapi30::Error,
    },
    #[error("Array index '{index}' processing error: {source}")]
    ArrayProcessing { index: usize, source: Box<Error> },
    #[error("Invalid type: expected '{expected}', got '{actual:?}'")]
    InvalidType { expected: String, actual: String },
    #[error("Invalid reference: {0}")]
    InvalidReference(String),
    #[error("Unsupported type: {0}")]
    UnsupportedType(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Missing data format for type: {0}")]
    MissingFormat(String),
    #[error("{0}::{1} field decoding error: {2}")]
    FieldDecoding(String, String, Box<Error>),
    #[error("Invalid operation: {0} {1}")]
    InvalidOperation(String, String),
    #[error("Internal processing error: {0}")]
    InternalProcessing(String),
}

// Result is a type alias for handling errors.
pub type Result<T> = std::result::Result<T, Error>;

pub fn load_vim_model(model: &OpenAPI, pruned_types: Option<&[&str]>) -> Result<Model> {
    let mut vim_model = Model {
        enums: IndexMap::new(),
        structs: IndexMap::new(),
        request_types: IndexMap::new(),
        any_value_types: IndexMap::new(),
        managed_objects: IndexMap::new(),
    };
    let Some(components) = model.components.as_ref() else {
        return Err(Error::MissingField("#/components".to_string()));
    };
    let Some(schemas) = components.schemas.as_ref() else {
        return Err(Error::MissingField("#/components/schemas".to_string()));
    };

    transform_schemas(schemas, &mut vim_model)?;
    process_discriminator_mappings(schemas, &mut vim_model)?;
    compute_hierarchy(&mut vim_model)?;
    prune_structs(&mut vim_model, pruned_types)?;
    mark_cycles(&mut vim_model)?;
    load_managed_objects(model, &mut vim_model)?;
    transform_paths(model, &mut vim_model)?;
    vim_model.structs = reorder_structs(&mut vim_model.structs)?;
    
    Ok(vim_model)
}

fn prune_structs(vim_model: &mut Model, pruned_types: Option<&[&str]>) -> Result<()> {
    let pruned_types = pruned_types.unwrap_or(&[]);
    for pruned_type in pruned_types {
        let pruned_type = pruned_type.to_string();
        if let Some(vim_type) = vim_model.structs.get_mut(&pruned_type) {
            vim_type.borrow_mut().emit_mode = EmitMode::Prune;
            skip_children(vim_model, &pruned_type, &pruned_type)?;
        } else { 
            error!("Pruned type not found: {}", pruned_type);
        }
    }
    Ok(())
}

fn skip_children(vim_model: &mut Model, parent: &str, pruned: &str) -> Result<()> {
    let parent_struct = vim_model.structs.get(parent);
    let Some(parent_struct) = parent_struct else {
        return Err(Error::InternalProcessing(format!("Parent struct not found: {}", parent)));
    };
    let children = parent_struct.borrow().children.clone();
    for child in children {
        let child = child.to_string();
        if let Some(vim_type) = vim_model.structs.get_mut(&child) {
            vim_type.borrow_mut().emit_mode = EmitMode::Skip(pruned.to_string());
            skip_children(vim_model, &child, &pruned)?;
        }
    }
    Ok(())
}

/// Populate the `children` of structs in the model.
fn compute_hierarchy(vim_model: &mut Model) -> Result<()> {
    for (_, vim_type) in &vim_model.structs {
        let Some(parent) = &vim_type.borrow().parent else {
            continue;
        };
        vim_model
            .structs
            .get(parent)
            .ok_or_else(|| Error::InvalidReference(parent.clone()))?
            .borrow_mut()
            .children
            .push(vim_type.borrow().name.clone());
    }
    Ok(())
}

fn process_discriminator_mappings(
    schemas: &IndexMap<String, RefOr<Schema>>,
    vim_model: &mut Model,
) -> Result<()> {
    let any = schemas
        .get(&"Any".to_string())
        .ok_or_else(|| Error::InvalidReference("Any".to_string()))?;
    let RefOr::Val(ref schema) = any else {
        return Err(Error::InvalidReference("Any".to_string()));
    };
    let discriminator =
        schema.as_ref().discriminator.as_ref().ok_or_else(|| {
            Error::MissingField("#/components/schemas/Any/discriminator".to_string())
        })?;
    let Some(mapping) = discriminator.mapping.as_ref() else {
        return Err(Error::MissingField(
            "#/components/schemas/Any/discriminator/mapping".to_string(),
        ));
    };
    for (alias, type_ref) in mapping {
        let type_name = reference_to_schema_name(type_ref)?.to_string();
        if let Some(vim_type) = vim_model.any_value_types.get_mut(&type_name) {
            vim_type.discriminator_value = Some(alias.to_string());
            continue;
        }
        if let Some(vim_type) = vim_model.structs.get_mut(&type_name) {
            vim_type.borrow_mut().discriminator_value = Some(alias.to_string());
            continue;
        }
        if let Some(vim_type) = vim_model.enums.get_mut(&type_name) {
            vim_type.discriminator_value = Some(alias.to_string());
            continue;
        }
        return Err(Error::InvalidReference(type_name));
    }
    Ok(())
}

fn transform_schemas(
    schemas: &IndexMap<String, RefOr<Schema>>,
    vim_model: &mut Model,
) -> Result<()> {
    for (schema_name, ref_or_schema) in schemas {
        let RefOr::Val(ref schema) = ref_or_schema else {
            return Err(Error::UnsupportedSchemaReferenceType(
                schema_name.to_string(),
            ));
        };
        let schema = &schema.as_ref();
        schema.validate().map_err(|e| Error::SchemaValidation {
            path: schema_name.to_string(),
            source: e,
        })?;

        match schema {
            Schema {
                // Enum arm
                schema_type: Some(SchemaType::String),
                enumeration: Some(values),
                ..
            } => {
                let enumeration = Enum {
                    name: schema_name.to_string(),
                    description: schema.description.clone(),
                    variants: as_string_values(values)?,
                    discriminator_value: None,
                };
                vim_model.enums.insert(schema_name.to_string(), enumeration);
            }
            Schema {
                // Any _value arm
                schema_type: Some(SchemaType::Object),
                properties: Some(props),
                ..
            } if get_parent_schema(schema) == Some("Any".to_string())
                && props.len() == 1
                && props.contains_key(&"_value".to_string()) =>
            {
                vim_model.any_value_types.insert(
                    schema_name.to_string(),
                    BoxType {
                        name: schema_name.to_string(),
                        description: schema.description.clone(),
                        property_type: DataType::try_from(
                            props.get(&"_value".to_string()).unwrap(),
                        )?,
                        discriminator_value: None,
                    },
                );
            }
            Schema {
                // Struct arm
                schema_type: Some(SchemaType::Object),
                ..
            } => {
                let structure = build_struct_type(schema_name, schema)?;
                if structure.parent.is_none() && structure.name.ends_with("RequestType") {
                    vim_model
                        .request_types
                        .insert(schema_name.to_string(), RefCell::new(structure));
                } else {
                    vim_model
                        .structs
                        .insert(schema_name.to_string(), RefCell::new(structure));
                }
            }
            _ => {
                dbg!("unhandled schema: {:?}", schema_name);
                return Err(Error::UnsupportedType(schema_name.to_string()));
            }
        }
    }
    Ok(())
}

static EMPTY_VEC: Vec<String> = Vec::new();

fn build_struct_type(schema_name: &str, schema: &Schema) -> Result<Struct> {
    let properties = convert_properties(schema_name, schema)?;
    Ok(Struct {
        name: schema_name.to_string(),
        description: schema.description.clone(),
        fields: properties,
        parent: get_parent_schema(schema),
        discriminator_value: None,
        children: vec![],
        last_child: "".to_string(),
        emit_mode: EmitMode::Emit,
    })
}

fn convert_properties(schema_name: &str, schema: &Schema) -> Result<IndexMap<String, Field>> {
    let mut result = IndexMap::new();
    let Some(ref properties) = schema.properties else {
        return Ok(result);
    };
    let required = schema.required.as_ref().unwrap_or(&EMPTY_VEC);
    for (property_name, ref_or_property) in properties {
        let description = match ref_or_property {
            RefOr::Val(property) => property.description.as_ref(),
            RefOr::Ref { description, .. } => description.as_ref(),
        };
        let property = Field {
            name: property_name.clone(),
            vim_type: DataType::try_from(ref_or_property).map_err(|op| {
                Error::FieldDecoding(
                    schema_name.to_string(),
                    property_name.to_string(),
                    Box::new(op),
                )
            })?,
            optional: !required.contains(&property_name.to_string()),
            description: description.cloned(),
            require_box: false,
        };
        result.insert(property_name.to_string(), property);
    }
    Ok(result)
}

fn as_string_values(values: &[serde_json::Value]) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    for (i, value) in values.iter().enumerate() {
        let value = as_string(value).map_err(|e| Error::ArrayProcessing {
            index: i,
            source: Box::new(e),
        })?;
        result.push(value);
    }
    Ok(result)
}

fn as_string(value: &serde_json::Value) -> Result<String> {
    let value = value.as_str().ok_or(Error::InvalidType {
        expected: "string".to_string(),
        actual: value.to_string(),
    })?;
    Ok(value.to_string())
}

fn get_parent_schema(schema: &Schema) -> Option<String> {
    let all_of = schema.all_of.as_ref()?;
    if all_of.len() == 1 {
        if let RefOr::Ref { ref reference, .. } = &all_of[0] {
            return Some(trim_schema_prefix(reference).to_string());
        }
    }
    None
}

pub fn reference_to_schema_name(reference: &String) -> Result<&str> {
    if !reference.starts_with("#/components/schemas/") {
        return Err(Error::InvalidReference(reference.to_string()));
    }
    let schema_name = trim_schema_prefix(reference);
    Ok(schema_name)
}

fn trim_schema_prefix(reference: &str) -> &str {
    reference.trim_start_matches("#/components/schemas/")
}

fn load_managed_objects(model: &OpenAPI, vim_model: &mut Model) -> Result<()> {
    let Some(tags) = model.tags.as_ref() else {
        return Ok(());
    };
    for tag in tags {
        let tag_name = &tag.name;
        let tag_description = &tag.description;
        let managed_object = ManagedObject {
            name: tag_name.clone(),
            description: tag_description.clone(),
            methods: vec![],
        };
        vim_model
            .managed_objects
            .insert(tag_name.clone(), managed_object);
    }
    Ok(())
}

fn transform_paths(model: &OpenAPI, vim_model: &mut Model) -> Result<()> {
    for (path, path_item) in &model.paths {
        if let Some(operation) = &path_item.get {
            add_method(
                operation,
                path,
                vim_model,
                property_name(operation, path)?,
                HttpMethod::Get,
                None,
            )?;
        }
        if let Some(operation) = &path_item.post {
            add_method(
                operation,
                path,
                vim_model,
                method_name(operation, path)?,
                HttpMethod::Post,
                get_request_type(operation, path)?,
            )?;
        }
    }
    Ok(())
}

fn add_method(
    operation: &Operation,
    path: &String,
    vim_model: &mut Model,
    name: &str,
    method: HttpMethod,
    request_type: Option<DataType>,
) -> Result<()> {
    let tag = get_tag(operation, path)?;
    let managed_object = ensure_managed_object(vim_model, tag);
    let (return_type, optional) = get_success_return_type(operation, path)?;
    managed_object.methods.push(Method {
        name: name.to_string(),
        description: operation.description.clone(),
        path: path.to_string(),
        http_method: method,
        input: request_type,
        output: return_type,
        optional_response: optional,
        output_description: get_response_description(operation, path)?,
        error_description: get_error_description(operation, path)?,
    });
    Ok(())
}

fn get_response_description(operation: &Operation, path: &String) -> Result<Option<String>> {
    let responses = &operation.responses;
    for (status_code, response) in &responses.responses {
        if status_code.starts_with("2") {
            let RefOr::Val(ref response) = response else {
                return Err(Error::InvalidOperation(
                    operation.operation_id.as_ref().unwrap_or(path).clone(),
                    "expected response".to_string(),
                ));
            };
            return Ok(Some(response.description.clone()));
        }
    }
    Ok(None)
}

fn get_error_description(operation: &Operation, path: &String) -> Result<Option<String>> {
    let responses = &operation.responses;
    for (status_code, response) in &responses.responses {
        if status_code.starts_with("5") {
            let RefOr::Val(ref response) = response else {
                return Err(Error::InvalidOperation(
                    operation.operation_id.as_ref().unwrap_or(path).clone(),
                    "expected response".to_string(),
                ));
            };
            return Ok(Some(response.description.clone()));
        }
    }
    Ok(None)
}

fn method_name<'a>(operation: &'a Operation, path: &String) -> Result<&'a str> {
    let name = operation
        .operation_id
        .as_ref()
        .ok_or_else(|| Error::MissingField(format!("{}/operationId", path)))?;
    let name = name
        .split_once("_")
        .ok_or_else(|| {
            Error::InvalidOperation(
                operation.operation_id.as_ref().unwrap_or(path).clone(),
                "expected operationId to be in the format <class>_<method>".to_string(),
            )
        })?
        .1;
    Ok(name)
}

fn get_tag<'a>(operation: &'a Operation, path: &String) -> Result<&'a String> {
    let tags = operation.tags.as_ref().unwrap_or(&EMPTY_VEC);
    if tags.is_empty() || tags.len() > 1 {
        return Err(Error::InvalidOperation(
            operation.operation_id.as_ref().unwrap_or(path).clone(),
            "expected single tag".to_string(),
        ));
    }
    let tag = &tags[0];
    Ok(tag)
}

fn property_name<'a>(operation: &'a Operation, path: &String) -> Result<&'a str> {
    let op_name = operation
        .operation_id
        .as_ref()
        .ok_or_else(|| Error::MissingField(format!("{}/operationId", path)))?;
    let name = op_name
        .split_once("_")
        .ok_or_else(|| {
            Error::InvalidOperation(
                op_name.clone(),
                "expected operationId to be in the format <class>_<method>".to_string(),
            )
        })?
        .1;
    let name = name
        .split_at_checked(3)
        .ok_or_else(|| {
            Error::InvalidOperation(
                op_name.clone(),
                "expected read operationId to start with 'get'".to_string(),
            )
        })?
        .1;
    Ok(name)
}

fn ensure_managed_object<'a>(vim_model: &'a mut Model, tag: &String) -> &'a mut ManagedObject {
    let managed_object_opt = vim_model.managed_objects.get_mut(tag);
    if managed_object_opt.is_none() {
        let managed_object = ManagedObject {
            name: tag.clone(),
            description: None,
            methods: vec![],
        };
        vim_model
            .managed_objects
            .insert(tag.clone(), managed_object);
    };
    vim_model.managed_objects.get_mut(tag).unwrap()
}

fn get_request_type(operation: &Operation, path: &String) -> Result<Option<DataType>> {
    let Some(request_body) = operation.request_body.as_ref() else {
        return Ok(None);
    };
    let RefOr::Val(request_body) = request_body else {
        return Ok(None);
    };
    let content = &request_body.content;
    let content = content.get("application/json").ok_or_else(|| {
        Error::InvalidOperation(
            operation.operation_id.as_ref().unwrap_or(path).clone(),
            "expected request body application/json content".to_string(),
        )
    })?;
    let schema = content.schema.as_ref().ok_or_else(|| {
        Error::InvalidOperation(
            operation.operation_id.as_ref().unwrap_or(path).clone(),
            "expected request body schema".to_string(),
        )
    })?;
    Ok(Some(DataType::try_from(schema)?))
}

fn get_success_return_type(
    operation: &Operation,
    path: &String,
) -> Result<(Option<DataType>, bool)> {
    let responses = &operation.responses;
    for (status_code, response) in &responses.responses {
        if status_code.starts_with("2") {
            let RefOr::Val(ref response) = response else {
                return Err(Error::InvalidOperation(
                    operation.operation_id.as_ref().unwrap_or(path).clone(),
                    "expected response".to_string(),
                ));
            };
            let Some(content) = response.content.as_ref() else {
                return Ok((None, false)); // No content type, no return type
            };
            let content = content.get("application/json").ok_or_else(|| {
                Error::InvalidOperation(
                    operation.operation_id.as_ref().unwrap_or(path).clone(),
                    "expected response application/json content".to_string(),
                )
            })?;
            let schema = content.schema.as_ref().ok_or_else(|| {
                Error::InvalidOperation(
                    operation.operation_id.as_ref().unwrap_or(path).clone(),
                    "expected success response schema".to_string(),
                )
            })?;
            let nullable = match schema {
                RefOr::Val(schema) => schema.nullable.unwrap_or(false),
                RefOr::Ref {
                    reference: _,
                    description: _,
                    nullable,
                } => nullable.unwrap_or(false),
            };
            return Ok((Some(DataType::try_from(schema)?), nullable));
        }
    }
    Err(Error::InvalidOperation(
        operation.operation_id.as_ref().unwrap_or(path).clone(),
        "expected 2xx response".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Read;

    #[test]
    fn test_enum_load() {
        let value = json!({
            "openapi": "3.0.3",
            "info": {"title": "test", "version": "1.0.0"},
            "paths": {},
            "tags": [],
            "components": {
                "schemas": {
                    "Any": {
                        "type": "object",
                        "description": "The base of all data types. Not to be used directly on the wire.\n",
                        "properties": {
                          "_typeName": {
                            "description": "The type discriminator. Refers to the name of a valid data object type.\n",
                            "type": "string"
                          }
                        },
                        "required": [
                          "_typeName"
                        ],
                        "discriminator": {
                          "propertyName": "_typeName",
                          "mapping": {}
                        }
                    },
                    "test": {
                        "description": "test",
                        "type": "string",
                        "enum": ["foo", "bar"]
                    }
                }
            }
        });
        let open_api = serde_json::from_value::<OpenAPI>(value).unwrap();
        let vim_model = load_vim_model(&open_api, None).unwrap();
        assert_eq!(vim_model.enums.len(), 1);
        let test_enum = vim_model.enums.get("test").unwrap();
        assert_eq!(test_enum.description, Some("test".to_string()));
        assert_eq!(
            test_enum.variants,
            vec!["foo".to_string(), "bar".to_string()]
        );
    }

    #[test]
    fn test_request_type_load() {
        let value = json!({
            "openapi": "3.0.3",
            "info": {"title": "test", "version": "1.0.0"},
            "paths": {},
            "tags": [],
            "components": {
                "schemas": {
                    "Any": {
                        "type": "object",
                        "description": "The base of all data types. Not to be used directly on the wire.\n",
                        "properties": {
                          "_typeName": {
                            "description": "The type discriminator. Refers to the name of a valid data object type.\n",
                            "type": "string"
                          }
                        },
                        "required": [
                          "_typeName"
                        ],
                        "discriminator": {
                          "propertyName": "_typeName",
                          "mapping": {}
                        }
                    },
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
        let vim_model = load_vim_model(&open_api, None).unwrap();
        assert_eq!(vim_model.structs.len(), 2);
        let test_request_type = vim_model.structs.get("TestRequestType").unwrap().borrow();
        assert_eq!(test_request_type.name, "TestRequestType");
        assert_eq!(test_request_type.description, Some("test".to_string()));
        assert_eq!(test_request_type.fields.len(), 2);
        assert_eq!(test_request_type.parent, None);
        let test_request_type_prop = test_request_type.fields.get("foo").unwrap();
        assert_eq!(test_request_type_prop.name, "foo");
        assert_eq!(test_request_type_prop.vim_type, DataType::String);
        assert!(!test_request_type_prop.optional);
        assert_eq!(
            test_request_type_prop.description,
            Some("test description".to_string())
        );

        let test_request_type_prop = test_request_type.fields.get("bar").unwrap();
        assert_eq!(test_request_type_prop.name, "bar");
        assert_eq!(
            test_request_type_prop.vim_type,
            DataType::Reference(
                "BaseConfigInfoDiskFileBackingInfoProvisioningType_enum".to_string()
            )
        );
        assert!(test_request_type_prop.optional);
        assert_eq!(test_request_type_prop.description, None);
    }

    #[test]
    fn test_struct_load() {
        let value = json!({
            "openapi": "3.0.3",
            "info": {"title": "test", "version": "1.0.0"},
            "paths": {},
            "tags": [],
            "components": {
                "schemas": {
                    "Any": {
                        "type": "object",
                        "description": "The base of all data types. Not to be used directly on the wire.\n",
                        "properties": {
                            "_typeName": {
                            "description": "The type discriminator. Refers to the name of a valid data object type.\n",
                            "type": "string"
                            }
                        },
                        "required": [
                            "_typeName"
                        ],
                        "discriminator": {
                            "propertyName": "_typeName",
                            "mapping": {}
                        }
                    },
                    "MethodFault": {
                        "type": "object",
                        "description": "The base data object type for all the object model faults\nthat an application might handle.\n",
                        "properties": {
                            "faultCause": {
                            "description": "Fault which is the cause of this fault.\n",
                            "$ref": "#/components/schemas/MethodFault"
                            },
                            "faultMessage": {
                            "description": "Message which has details about the error\nMessage can also contain a key to message catalog which\ncan be used to generate better localized messages.\n",
                            "type": "array",
                            "items": {
                                "$ref": "#/components/schemas/LocalizableMessage"
                            }
                            }
                        },
                        "allOf": [
                            {
                            "$ref": "#/components/schemas/Any"
                            }
                        ]
                    },
                }
            }
        });
        let open_api = serde_json::from_value::<OpenAPI>(value).unwrap();
        let vim_model = load_vim_model(&open_api, None).unwrap();
        assert_eq!(vim_model.structs.len(), 2);
        let test_struct = vim_model.structs.get("MethodFault").unwrap().borrow();
        assert_eq!(test_struct.name, "MethodFault");
        assert_eq!(test_struct.description, Some("The base data object type for all the object model faults\nthat an application might handle.\n".to_string()));
        assert_eq!(test_struct.fields.len(), 2);
        assert_eq!(test_struct.parent, Some("Any".to_string()));
        let test_request_type_prop = test_struct.fields.get("faultCause").unwrap();
        assert_eq!(test_request_type_prop.name, "faultCause");
        assert_eq!(
            test_request_type_prop.vim_type,
            DataType::Reference("MethodFault".to_string())
        );
        assert!(test_request_type_prop.optional);

        let any = vim_model.structs.get("Any").unwrap().borrow();
        assert_eq!(any.children, vec!["MethodFault".to_string()]);
    }

    #[test]
    fn test_box_load() {
        let value = json!({
            "openapi": "3.0.3",
            "info": {"title": "test", "version": "1.0.0"},
            "paths": {},
            "components": {
                "schemas": {
                    "Any": {
                        "type": "object",
                        "description": "The base of all data types. Not to be used directly on the wire.\n",
                        "properties": {
                          "_typeName": {
                            "description": "The type discriminator. Refers to the name of a valid data object type.\n",
                            "type": "string"
                          }
                        },
                        "required": [
                          "_typeName"
                        ],
                        "discriminator": {
                          "propertyName": "_typeName",
                          "mapping": {
                            "boolean": "#/components/schemas/PrimitiveBoolean",
                          }
                        }
                    },
                    "PrimitiveBoolean": {
                        "type": "object",
                        "description": "A boxed Boolean primitive. To be used in *Any* placeholders.\n",
                        "properties": {
                          "_value": {
                            "type": "boolean"
                          }
                        },
                        "required": [
                          "_value"
                        ],
                        "allOf": [
                          {
                            "$ref": "#/components/schemas/Any"
                          }
                        ]
                    },
                }
            }
        });
        let open_api = serde_json::from_value::<OpenAPI>(value).unwrap();
        let vim_model = load_vim_model(&open_api, None).unwrap();
        assert_eq!(vim_model.any_value_types.len(), 1);
        let boxed_type = vim_model
            .any_value_types
            .get(&"PrimitiveBoolean".to_string())
            .unwrap();
        assert_eq!(boxed_type.name, "PrimitiveBoolean".to_string());
        assert_eq!(
            boxed_type.description,
            Some("A boxed Boolean primitive. To be used in *Any* placeholders.\n".to_string())
        );
        assert_eq!(boxed_type.property_type, DataType::Boolean);
        assert_eq!(boxed_type.discriminator_value, Some("boolean".to_string()));
    }

    fn load_openapi() -> OpenAPI {
        let mut file =
            std::fs::File::open("data/vi_json_openapi_specification_v8_0_2_0.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let openapi: OpenAPI = serde_json::from_str(&data).unwrap();
        openapi
    }

    #[test]
    fn test_load_vim_model() {
        let model = load_openapi();
        let vim_model = load_vim_model(&model, None).unwrap();
        assert_eq!(vim_model.any_value_types.len(), 3071);
        assert_eq!(vim_model.enums.len(), 414);
        assert_eq!(
            vim_model.structs.len() + vim_model.request_types.len(),
            3697
        );
        assert_eq!(vim_model.managed_objects.len(), 139);
        assert!(vim_model.structs.contains_key("VirtualE1000"));
        assert!(vim_model
            .request_types
            .contains_key("RetrievePropertiesRequestType"));
        assert!(vim_model.managed_objects.contains_key("VirtualMachine"));
        assert_eq!(
            vim_model
                .managed_objects
                .get("VirtualMachine")
                .unwrap()
                .methods
                .len(),
            101
        );
    }
}
