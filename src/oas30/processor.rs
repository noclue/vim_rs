
use std::{collections::HashMap, hash::Hash};

use crate::oas30::openapi::{OpenAPI, Schema, SchemaType, RefOr};




pub struct OpenAPIProcessor {
    model: OpenAPI,
    /// The schema registry
    schema_registry: HashMap<String, SchemaDetails>,
}


impl OpenAPIProcessor {
    /// Create a new OpenAPIProcessor
    pub fn new(model: OpenAPI) -> OpenAPIProcessor {
        OpenAPIProcessor {
            model: model,
            schema_registry: HashMap::new(),
        }
    }
}

fn compute_schema_registry(model: &OpenAPI) ->  HashMap<String, SchemaDetails> {
    let Some(schemas) = get_schemas(model) else {

    };
}


/// Checks if a schema is object.
/// A schema is an object if it is of type object
fn is_object(schema: &Schema) -> bool {
    schema.schema_type == Some(SchemaType::Object)
}

/// Checks if a schema is enum.
/// A schema is an enum if it is of type string and has enum values
fn is_enum(schema: &Schema) -> bool {
    if schema.schema_type != Some(SchemaType::String) {
        return false;
    }
    schema.enum_values.is_some()
}

fn get_schemas(model: &OpenAPI) -> Option<&HashMap<String, RefOr<Schema>>> {
    let Some(components) = model.components else {
        return None;
    };
    let Some(ref schemas_map) = components.schemas else {
        return None;
    };
    Some(schemas_map)
}

/// get_parent returns the parent schema if one exists or None. A prent schema is single schema
/// reference in the `all_of` field of schema of type `object`.
fn get_parent_ref(schema: &Schema) -> Option<String> {
    if schema.schema_type != Some(SchemaType::Object) {
        return None;
    }
    let Some(all_of) = &schema.all_of else {
        return None;
    };
    if all_of.len() != 1 {
        return None;
    }
    let RefOr::Ref(reference ) = &all_of[0] else {
        return None;
    };
    return Some(reference.reference);
}

/// Type of vi_json schema. We will want to differentiate the following:
/// - Object: A schema that is type object and extends Any or is Any
/// - Box of Array or Primitive type: A schema that is object, has single member array or primitive typ. It also extends Any and is not referenced by any other schema
/// - Enumeration: A schema that is string and has enum values
/// - RequestType: A schema that is object andd is not descending from Any and is not refered by any other schema
enum SchemaClass {
    Object,
    Box,
    Enumeration,
    RequestType,
}

/// SchemaRegistry data structure to keep basic fact about schemas including their parent, their
/// children and the incomming references.
pub struct SchemaDetails {
    /// The schema name
    schema: String,
    /// The parent schema of the schema
    parent: Option<String>,
    /// The children of the schema
    children: Vec<String>,
    /// The references to the schema
    references: Vec<String>,
}

pub impl SchemaDetails {
    fn new(schema: String) -> SchemaDetails {
        SchemaDetails {
            schema: schema,
            parent: None,
            children: Vec::new(),
            references: Vec::new(),
        }
    }
}

pub struct SchemaRegistry {
    /// The schema details map schema reference to their details
    details: HashMap<String, SchemaDetails>,
}

impl SchemaRegistry {
    /// Create a new schema registry
    pub fn new() -> SchemaRegistry {
        SchemaRegistry {
            details: HashMap::new(),
        }
    }

    /// Add a schema to the registry
    pub fn add_schema(&mut self, schema: Schema, parent: Option<String>) {
        let schema_class = classify_schema(&schema);
        let schema_name = schema.name.clone();
        let schema_details = SchemaDetails {
            schema: schema,
            parent: parent,
            children: Vec::new(),
            references: Vec::new(),
        };
        self.details.insert(schema_name, schema_details);
    }

    /// Add a reference to a schema
    pub fn add_reference(&mut self, schema_name: String, reference: String) {
        let schema_details = self.details.get_mut(&schema_name).unwrap();
        schema_details.references.push(reference);
    }

    /// Add a child to a schema
    pub fn add_child(&mut self, schema_name: String, child: String) {
        let schema_details = self.details.get_mut(&schema_name).unwrap();
        schema_details.children.push(child);
    }


fn classify_schema(schema: &Schema) -> SchemaClass {
    if schema.properties.is_some() {
        SchemaClass::Object
    } else if schema.items.is_some() {
        SchemaClass::Array
    } else {
        SchemaClass::Scalar
    }
}