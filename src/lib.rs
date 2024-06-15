use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind};
use std::io::Read;
use convert_case::{Case, Casing};

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
}

// Result is a type alias for handling errors.
pub type Result<T> = std::result::Result<T, Error>;

/// A project manages the stats of an OpenAPI specification.
/// It is created from a file and additional files may be
/// loaded to resolve references.
pub struct Project {
    spec: openapiv3::OpenAPI,
    // TODO: Support multiple specs and cross file references.
    //specs: HashMap<String, openapiv3::OpenAPI>,
}

impl Project {
    /// Create a new project from a file.
    pub fn new(file: &str) -> Result<Project> {
        let openapi = load_openapi(file)?;
        Result::Ok(Project { spec: openapi })
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
        match (self
            .spec
            .components
            .as_ref()
            .unwrap()
            .schemas
            .get(schema_name))
        {
            Some(schema) => Ok(schema),
            None => Err(Error::NotFound(schema_name.to_string())),
        }
    }

    /// Get ReferenceOr<Schema> and return the Schema if it is a Schema. If it is a Reference, resolve it recursively and return the referenced Schema.
    /// If the reference is not found return NotFound error. If there is cycle in the reference, return Cycle error.
    pub fn resolve_schema<'a>(&'a self, item: &'a ReferenceOr<Schema>) -> Result<&'a Schema> {
        self.resolve_internal(item, vec![])
    }

    pub fn resolve_boxed_schema<'a>(&'a self, item: &'a ReferenceOr<Box<Schema>>) -> Result<&'a Schema> {
        match item {
            ReferenceOr::Item(schema) => Ok(schema),
            ReferenceOr::Reference { reference } => {
                self.resolve_reference(reference, vec![])
            }
        }
    }

    fn resolve_internal<'a>(
        &'a self,
        item: &'a ReferenceOr<Schema>,
        mut stack: Vec<String>,
    ) -> Result<&'a Schema> {
        match item {
            ReferenceOr::Item(schema) => Ok(schema),
            ReferenceOr::Reference { reference } => {
                self.resolve_reference(reference, stack)
            }
        }
    }

    fn resolve_reference<'a>(&'a self, reference: &String, mut stack: Vec<String>) -> std::prelude::v1::Result<&Schema, Error> {
        let schema_name = reference.trim_start_matches("#/components/schemas/");
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
    
    /// Get parent schema name Some(String) from the "allOf" field if there is exactly one reference value.
    /// If there are multiple values, no value or Schema value return None.
    pub fn get_parent_schema(&self, schema: &Schema) -> Result<Option<String>> {
        if let SchemaKind::Any(any_scehma) = &schema.schema_kind {
            let all_of = &any_scehma.all_of;
            if all_of.len() == 1 {
                if let ReferenceOr::Reference { reference } = &all_of[0] {
                    let schema_name = reference.trim_start_matches("#/components/schemas/");
                    return Ok(Some(schema_name.to_string()));
                }
            }
        }
        Result::Ok(None)
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

/// Print hierarchy of schemas. Prints ASCII art hirarchy of schemas starting from the given schema.
/// If the schema has child schemas, they will be printed on new line with increased indentation.
/// If the child schema has child schemas, they will be printed on new line with increased indentation.
pub fn print_schema_hierarchy(project: &Project, schema: String, indent: usize) -> Result<()> {
    let h = resolve_schema_hierarchy(project)?;
    print_schema_hierarchy_internal(&h, &schema, indent)
}

fn print_schema_hierarchy_internal(
    hierarchy: &std::collections::HashMap<String, Vec<String>>,
    schema: &String,
    indent: usize,
) -> Result<()> {
    match hierarchy.get(schema) {
        Some(children) => {
            println!(
                "{:indent$}{} {}",
                "",
                schema,
                children.len(),
                indent = indent
            );
            for child in children {
                print_schema_hierarchy_internal(hierarchy, child, indent + 2)?;
            }
        }
        None => {
            return Err(Error::NotFound(schema.to_string()));
        }
    }
    Ok(())
}

/// Generate Rust code for given schema.
/// The generated code is a struct definition for the schema.
/// The struct name is the schema name.
/// The struct fields are the properties of the schema.
/// The struct fields are named after the properties of the schema.
/// The struct fields are typed according to the type of the properties of the schema.
/// Returns a string of Rust code.
/// The generated code is formatted using prettyplease.
pub fn generate_rust_code(project: &Project, schemaName: String) -> Result<String> {
    let refOrschema = project.get_schema(&schemaName)?;
    match refOrschema {
        ReferenceOr::Item(schema) => generate_rust_code_for_schema(project, schemaName, schema),
        ReferenceOr::Reference { reference } => {
            Ok(format!("\n// `{schemaName}` schema is reference to `{reference}`").into())
        }
    }
}

fn generate_rust_code_for_schema(
    project: &Project,
    schemaName: String,
    schema: &Schema,
) -> Result<String> {
    let mut code = String::new();
    code = format!("#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]\n");
    code.push_str(format!("pub struct {} {{\n", schemaName).as_str());
    match &schema.schema_kind {
        SchemaKind::Any(anySchema) => {
            let properties = &anySchema.properties;
            render_properties(&mut code, project, properties)?;
        }
        SchemaKind::Type(openapiv3::Type::Object(object_schema)) => {}
        _ => {
            let kind = &schema.schema_kind;
            code.push_str(
                format!("    // TODO: Implement support for {schemaName} schema: {kind:?}\n")
                    .as_str(),
            );
        }
    }
    code.push_str("}\n");
    Ok(code)
}

fn render_properties(
    code: &mut String,
    project: &Project,
    properties: &indexmap::map::IndexMap<String, ReferenceOr<Box<Schema>>>,
) -> Result<()> {
    for (name, property) in properties.iter() {
        let property = project.resolve_boxed_schema(property)?;
        let property_type = get_property_type(project, property);
        let field_name = name.to_case(Case::Snake); 
        code.push_str(format!("    #[serde(rename = \"{}\")]\n", name).as_str());
        code.push_str(format!("    pub {}: {},\n", field_name, property_type).as_str());
    }
    Ok(())
}

fn get_property_type(project: &Project, property: &Schema) -> String {
    match &property.schema_kind {
        SchemaKind::Type(openapiv3::Type::String(_)) => "String".to_string(),
        SchemaKind::Type(openapiv3::Type::Integer(_)) => "i64".to_string(),
        SchemaKind::Type(openapiv3::Type::Number(_)) => "f64".to_string(),
        SchemaKind::Type(openapiv3::Type::Boolean(_)) => "bool".to_string(),
        // SchemaKind::Type(openapiv3::Type::Array(array_schema)) => {
        //     let items = project.resolve_boxed_schema(&array_schema.items)?;
        //     format!("Vec<{}>", get_property_type(project, items))
        // },
        // SchemaKind::Type(openapiv3::Type::Object(object_schema)) => {
        //     let properties = object_schema.properties;
        //     format!("{}{}", "{", "}")
        // },
        _ => "Any".to_string(),
    }
}

/// Resolve hierarchy of schemas. Returns a Map of schema names to their children schema names.
fn resolve_schema_hierarchy(
    project: &Project,
) -> Result<std::collections::HashMap<String, Vec<String>>> {
    let mut hierarchy = std::collections::HashMap::new();

    for (name, schema) in project.spec.components.as_ref().unwrap().schemas.iter() {
        let schema = project.resolve_schema(schema)?;
        let parent = project.get_parent_schema(schema)?;
        if let Some(parent_name) = parent {
            let child_hierarchy = hierarchy.entry(parent_name).or_insert(vec![]);
            child_hierarchy.push(name.to_string());
        }
        hierarchy.entry(name.to_string()).or_insert(vec![]);
    }
    Ok(hierarchy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::io::Read;

    fn load_openapi() -> openapiv3::OpenAPI {
        let mut file = std::fs::File::open("data/vi_json_openapi_specification_v8_0_2_0.json")
            .expect("unable to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("unable to read file");
        let openapi: openapiv3::OpenAPI =
            serde_json::from_str(&data).expect("Could not deserialize input");
        openapi
    }

    #[test]
    fn test_print_schema() {
        let openapi = load_openapi();
        let project = Project { spec: openapi };
        let code = generate_rust_code(&project, "VirtualEthernetCard".to_string())
            .expect("Could not generate rust code");
        println!("{}", code);
    }

    #[test]
    fn print_schema_hierarchy_test() {
        let openapi = load_openapi();
        let project = Project { spec: openapi };
        print_schema_hierarchy(&project, "VirtualDevice".to_string(), 0)
            .expect("Could not print schema hierarchy");
    }

    #[test]
    fn count_schema() {
        let openapi = load_openapi();
        let mut count = 0;
        for (_, schema) in openapi.components.unwrap().schemas.iter() {
            count += 1;
        }

        assert_eq!(count, 7182);
    }
}
