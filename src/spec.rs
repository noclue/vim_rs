use openapiv3::{ReferenceOr, Schema, SchemaKind};
use std::collections::HashMap;
use std::io::Read;

use crate::printer;

// Error is an error type for Project.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot read file")]
    FileIOError(#[from] std::io::Error),
    #[error("Cannot parse JSON")]
    JSONError(#[from] serde_json::Error),
    #[error("Reference `{}` cannot be resolved", .0)]
    NotFound(String),
    #[error("Cycle detected in reference `{}`", .0)]
    Cycle(String),
    #[error("Invalid reference format `{}`", .0)]
    InvalidReference(String),
    #[error("Unsupported type `{}`", .0)]
    UnsupportedType(String),
    #[error("Invalid schema `{}`", .0)]
    InvalidApiSpecs(String),
    #[error("Printer error: {0}")]
    WriteError(#[from] printer::Error),
}

// Result is a type alias for handling errors.
pub type Result<T> = std::result::Result<T, Error>;

/// An APISpec manages a compilation uint of OpenAPI specifications. At present
/// one is supported. It is created from a file and additional files may be
/// loaded to resolve references.
pub struct APISpec {
    spec: openapiv3::OpenAPI,
    children: HashMap<String, Vec<String>>,
}

impl APISpec {
    /// Create a new project from a file.
    pub fn new(file: &str) -> Result<APISpec> {
        let openapi = load_openapi(file)?;
        let mut self_ = APISpec {
            spec: openapi,
            children: HashMap::new(),
        };
        self_.children = self_.resolve_schema_hierarchy()?;
        Ok(self_)
    }

    pub fn get_components(&self) -> Result<&openapiv3::Components> {
        self.spec
            .components
            .as_ref()
            .ok_or(Error::InvalidApiSpecs("Components not found".to_string()))
    }

    pub fn resolve_discriminator_by_name(&self, schema_name: &str) -> Result<String> {
        let schema_or_ref = self.get_schema(schema_name)?;
        let ReferenceOr::Item(schema) = schema_or_ref else {
            return Err(Error::InvalidReference(format!(
                "{} is not a schema",
                schema_name
            )));
        };
        self.resolve_discriminator(schema_name, schema)
    }

    pub fn resolve_discriminator(&self, schema_name: &str, schema: &Schema) -> Result<String> {
        let Some(discriminator) = &schema.schema_data.discriminator else {
            let Some(parent) = get_parent_schema(schema)? else {
                return Err(Error::InvalidApiSpecs(format!(
                    "{} does not have a discriminator",
                    schema_name
                )));
            };
            return self.resolve_discriminator_by_name(&parent);
        };
        Ok(discriminator.property_name.clone())
    }

    /// Resolve schema by schema name.
    /// Get Schema by schema name. If the schema is not found return NotFound error.
    /// If there is cycle in the reference, return Cycle error.
    /// If the schema is a Reference, resolve it recursively and return the referenced Schema.
    /// If the schema is a Schema, return the Schema.
    /// If the schema is not found return NotFound error.
    pub fn resolve_schema_by_name(&self, schema_name: &str) -> Result<&Schema> {
        let schema = self.get_schema(schema_name)?;
        self.resolve_schema(schema)
    }

    /// Get Schema by schema name. If the schema is not found return NotFound error.
    /// This method does not resolve the schema if it is a Reference.
    pub fn get_schema(&self, schema_name: &str) -> Result<&ReferenceOr<Schema>> {
        let Some(components) = self.spec.components.as_ref() else {
            return Err(Error::InvalidApiSpecs("Components not found".to_string()));
        };
        match components.schemas.get(schema_name) {
            Some(schema) => Ok(schema),
            None => Err(Error::NotFound(schema_name.to_string())),
        }
    }

    /// Get ReferenceOr<Schema> and return the Schema if it is a Schema.
    /// If it is a Reference, resolve it recursively and return the referenced Schema.
    /// If the reference is not found return NotFound error.
    /// If there is cycle in the reference, return Cycle error.
    pub fn resolve_schema<'a>(&'a self, item: &'a ReferenceOr<Schema>) -> Result<&'a Schema> {
        self.resolve_internal(item, &mut vec![])
    }

    pub fn resolve_boxed_schema<'a>(
        &'a self,
        item: &'a ReferenceOr<Box<Schema>>,
    ) -> Result<&'a Schema> {
        match item {
            ReferenceOr::Item(schema) => Ok(schema),
            ReferenceOr::Reference { reference } => self.resolve_reference(reference, &mut vec![]),
        }
    }

    fn resolve_internal<'a>(
        &'a self,
        item: &'a ReferenceOr<Schema>,
        stack: &mut Vec<String>,
    ) -> Result<&'a Schema> {
        match item {
            ReferenceOr::Item(schema) => Ok(schema),
            ReferenceOr::Reference { reference } => self.resolve_reference(reference, stack),
        }
    }

    fn resolve_reference<'a>(
        &'a self,
        reference: &String,
        stack: &mut Vec<String>,
    ) -> Result<&Schema> {
        let schema_name = parse_schema_from_reference(reference)?;
        if schema_name.len() == reference.len() {
            return Err(Error::InvalidReference(reference.to_string()));
        }
        if stack.contains(&schema_name.to_string()) {
            return Err(Error::Cycle(format!(
                "{}, {}",
                stack.join(", "),
                reference.to_string()
            )));
        }
        stack.push(schema_name.to_string());
        let ref_or_schema = self.get_schema(schema_name)?;
        let schema = self.resolve_internal(ref_or_schema, stack)?;
        Ok(schema)
    }

    /// Get child schemas of the given schema.
    /// If the schema has child schemas, return the child schemas.
    /// If the schema does not have child schemas, return an empty vector.
    /// If the schema is not found, return NotFound error.
    pub fn get_children(&self, schema_name: &str) -> Result<&Vec<String>> {
        match self.children.get(schema_name) {
            Some(children) => Ok(children),
            None => Err(Error::NotFound(schema_name.to_string())),
        }
    }

    /// Resolve hierarchy of schemas. Returns a Map of schema names to their children schema names.
    fn resolve_schema_hierarchy(&self) -> Result<std::collections::HashMap<String, Vec<String>>> {
        let mut hierarchy = std::collections::HashMap::new();

        for (name, schema) in self.get_components()?.schemas.iter() {
            let schema = self.resolve_schema(schema)?;
            let parent = get_parent_schema(schema)?;
            if let Some(parent_name) = parent {
                let child_hierarchy = hierarchy.entry(parent_name).or_insert(vec![]);
                child_hierarchy.push(name.to_string());
            }
            hierarchy.entry(name.to_string()).or_insert(vec![]);
        }
        Ok(hierarchy)
    }
}

/// Load an OpenAPI specification from a file.
fn load_openapi(file: &str) -> Result<openapiv3::OpenAPI> {
    let mut file = std::fs::File::open(file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let openapi: openapiv3::OpenAPI = serde_json::from_str(&data)?;
    Result::Ok(openapi)
}

/// Get parent schema name Some(String) from the "allOf" field if there is exactly one reference value.
/// If there are multiple values, no value or Schema value return None.
pub fn get_parent_schema(schema: &Schema) -> Result<Option<String>> {
    let all_of = match &schema.schema_kind {
        SchemaKind::Any(any_schema) => &any_schema.all_of,
        SchemaKind::AllOf { all_of } => all_of,
        _ => return Ok(None),
    };
    if all_of.len() == 1 {
        if let ReferenceOr::Reference { reference } = &all_of[0] {
            let schema_name = parse_schema_from_reference(reference)?;
            return Ok(Some(schema_name.to_string()));
        }
    }
    Ok(None)
}

pub fn parse_schema_from_reference(reference: &String) -> Result<&str> {
    if !reference.starts_with("#/components/schemas/") {
        return Err(Error::InvalidReference(reference.to_string()));
    }
    let schema_name = reference.trim_start_matches("#/components/schemas/");
    Ok(schema_name)
}
