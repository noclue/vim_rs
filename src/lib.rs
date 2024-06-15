use openapiv3::{AnySchema, OpenAPI, ReferenceOr, Schema, SchemaKind};
use printer::{Printer, StdoutPrinter};
use std::{f32::consts::E, io::Read};
use convert_case::{Case, Casing};

mod printer;

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
}

// Result is a type alias for handling errors.
pub type Result<T> = std::result::Result<T, Error>;

/// An APISpec manages a compilation uint of OpenAPI specifications. At present
/// one is supported. It is created from a file and additional files may be
/// loaded to resolve references.
pub struct APISpec {
    spec: openapiv3::OpenAPI,
    // TODO: Support multiple specs and cross file references.
    //specs: HashMap<String, openapiv3::OpenAPI>,
}

impl APISpec {
    /// Create a new project from a file.
    pub fn new(file: &str) -> Result<APISpec> {
        let openapi = load_openapi(file)?;
        Result::Ok(APISpec { spec: openapi })
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

    fn resolve_reference<'a>(&'a self, reference: &String, mut stack: Vec<String>) -> Result<&Schema> {
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

fn parse_schema_from_reference(reference: &String) -> Result<&str> {
    if ! reference.starts_with("#/components/schemas/") {
        return Err(Error::InvalidReference(reference.to_string()));
    }
    let schema_name = reference.trim_start_matches("#/components/schemas/");
    Ok(schema_name)
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
pub fn print_schema_hierarchy(project: &APISpec, schema: String) -> Result<()> {
    let printer = &mut  StdoutPrinter::new(None, None);
    let h = resolve_schema_hierarchy(project)?;
    print_schema_hierarchy_internal(&h, &schema, printer)
}

fn print_schema_hierarchy_internal(
    hierarchy: &std::collections::HashMap<String, Vec<String>>,
    schema: &String,
    printer: &mut dyn Printer,
) -> Result<()> {
    match hierarchy.get(schema) {
        Some(children) => {
            printer.println(format!("{} {}", schema, children.len()).as_str());
            printer.indent();
            for child in children {
                print_schema_hierarchy_internal(hierarchy, child, printer)?;
            }
            printer.dedent();
        }
        None => {
            return Err(Error::NotFound(schema.to_string()));
        }
    }
    Ok(())
}

/// Generate Rust code for given schema.
pub fn generate_rust_code(project: &APISpec, schemaName: String, printer: &mut dyn Printer) -> Result<()> {
    let refOrschema = project.get_schema(&schemaName)?;
    match refOrschema {
        ReferenceOr::Item(schema) => generate_rust_code_for_schema(project, schemaName, schema, printer),
        ReferenceOr::Reference { reference } => {
            // TODO
            printer.print(format!("\n// `{schemaName}` schema is reference to `{reference}`").as_str());
            Ok(())
        }
    }
}

fn generate_rust_code_for_schema(
    project: &APISpec,
    schema_name: String,
    schema: &Schema,
    printer: &mut dyn Printer,
) -> Result<()> {
    printer.println(format!("#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]").as_str());
    printer.println(format!("pub struct {} {{", schema_name).as_str());
    printer.indent();
    match &schema.schema_kind {
        SchemaKind::Any(anySchema) => {
            render_properties(project, anySchema, printer)?;
        }
        SchemaKind::Type(openapiv3::Type::Object(object_schema)) => {}
        _ => {
            printer.println(format!("// TODO: Implement support for {} schema: {:?}", 
                                    schema_name, 
                                    &schema.schema_kind
                                ).as_str());
        }
    }
    printer.dedent();
    printer.println("}");
    Ok(())
}

fn render_properties(
    project: &APISpec,
    schema: &AnySchema,
    printer: &mut dyn printer::Printer,
) -> Result<()> {
    for (name, property) in schema.properties.iter() {
        let mut property_type = get_property_type(project, property)?;
        let field_name = name.to_case(Case::Snake);
        let required = schema.required.contains(&name);
        if !required {
            property_type = format!("Option<{}>", property_type);
        }
        printer.println(format!("#[serde(rename = \"{}\")]", name).as_str());
        printer.println(format!("pub {}: {},", field_name, property_type).as_str());
    }
    Ok(())
}

fn get_property_type(project: &APISpec, property: &ReferenceOr<Box<Schema>>) -> Result<String> {
    match property {
        ReferenceOr::Item(schema) => get_schema_type(project, schema.as_ref()),
        ReferenceOr::Reference { reference } => {
            let schema_name = parse_schema_from_reference(reference)?;
            Ok(schema_name.to_string())
        }
        
    }
}

fn get_schema_type(project: &APISpec, property: &Schema) -> Result<String> {
    let mut str = String::new();
    let mut res = match &property.schema_kind {
        SchemaKind::Type(openapiv3::Type::String(_)) => "String",
        SchemaKind::Type(openapiv3::Type::Integer(_)) => "i64",
        SchemaKind::Type(openapiv3::Type::Number(_)) => "f64",
        SchemaKind::Type(openapiv3::Type::Boolean(_)) => "bool",
        SchemaKind::Type(openapiv3::Type::Array(array_schema)) => {
            let items = &array_schema.items;
            match items {
                None => return Err(Error::UnsupportedType(format!("Array without items: {:?}", property))),
                Some(items) => {
                    str = format!("Vec<{}>", get_property_type(project, items)?);
                    str.as_str()
                }
            }
        },
        SchemaKind::Type(openapiv3::Type::Object(_)) => 
            return Err(Error::UnsupportedType(format!("Object schema: {:?}", property))),
        _ => return Err(Error::UnsupportedType(format!("Unsupported schema: {:?}", property))),
    };
    Ok(res.to_string())
}

/// Resolve hierarchy of schemas. Returns a Map of schema names to their children schema names.
fn resolve_schema_hierarchy(
    project: &APISpec,
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

    fn load_openapi() -> APISpec {
        APISpec::new("data/vi_json_openapi_specification_v8_0_2_0.json")
            .expect("Could not load openapi")
    }

    #[test]
    fn test_print_schema() {
        let project = load_openapi();
        let mut printer = StdoutPrinter::new(None, None);
        generate_rust_code(&project, "ConfigTarget".to_string(), &mut printer)
            .expect("Could not generate rust code");
    }

    #[test]
    fn print_schema_hierarchy_test() {
        let project = load_openapi();
        print_schema_hierarchy(&project, "VirtualDevice".to_string())
            .expect("Could not print schema hierarchy");
    }

    #[test]
    fn count_schema() {
        let openapi = load_openapi().spec;
        let mut count = 0;
        for (_, schema) in openapi.components.unwrap().schemas.iter() {
            count += 1;
        }

        assert_eq!(count, 7182);
    }
}
