use convert_case::{Case, Casing};
use openapiv3::{ReferenceOr, Schema, SchemaKind};
use printer::Printer;
use std::io::Read;
use std::collections::HashMap;

mod check_keyword;

use check_keyword::CheckKeyword;



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
        let mut self_ = APISpec { spec: openapi, children: HashMap::new()};
        self_.children = self_.resolve_schema_hierarchy()?;
        Ok(self_)
    }

    pub fn resolve_discriminator_by_name(&self, schema_name: &str) -> Result<String> {
        let schema_or_ref = self.get_schema(schema_name)?;
        let ReferenceOr::Item(schema) = schema_or_ref else {
            return Err(Error::InvalidReference(format!("{} is not a schema", schema_name)));
        };
        self.resolve_discriminator(schema_name, schema)
    }

    pub fn resolve_discriminator(&self, schema_name: &str, schema: &Schema) -> Result<String> {
        let Some(discriminator) = &schema.schema_data.discriminator else {
            let Some(parent) = get_parent_schema(schema)? else {
                return Err(Error::InvalidApiSpecs(format!("{} does not have a discriminator", schema_name)));
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
    fn resolve_schema_hierarchy(
        &self
    ) -> Result<std::collections::HashMap<String, Vec<String>>> {
        let mut hierarchy = std::collections::HashMap::new();

        for (name, schema) in self.spec.components.as_ref().unwrap().schemas.iter() {
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

fn parse_schema_from_reference(reference: &String) -> Result<&str> {
    if !reference.starts_with("#/components/schemas/") {
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
pub fn print_schema_hierarchy(
    api_spec: &APISpec,
    schema_name: &String,
    printer: &mut dyn Printer,
) -> Result<()> {
    let children = api_spec.get_children(schema_name)?;
    printer.println(format!("{} {}", schema_name, children.len()).as_str())?;
    printer.indent();
    for child in children {
        print_schema_hierarchy(api_spec, child, printer)?;
    }
    printer.dedent();
    Ok(())
}

/// Generate Rust code for given schema.
pub fn generate_rust_code(
    project: &APISpec,
    schema_name: String,
    printer: &mut dyn Printer,
) -> Result<()> {
    let ref_or_schema = project.get_schema(&schema_name)?;
    let ReferenceOr::Item(schema) = ref_or_schema else {
        printer.println(format!("// `{}` schema is reference", schema_name).as_str())?;
        return Ok(());
    };
    let pascal_case = schema_name.to_case(Case::Pascal);
    match &schema.schema_kind {
        SchemaKind::Type(openapiv3::Type::String(_)) => {
            printer.println(format!("pub type {} = String;", pascal_case).as_str())?;
        }
        SchemaKind::Type(openapiv3::Type::Integer(_)) => {
            printer.println(format!("pub type {} = i64;", pascal_case).as_str())?;
        }
        SchemaKind::Type(openapiv3::Type::Number(_)) => {
            printer.println(format!("pub type {} = f64;", pascal_case).as_str())?;
        }
        SchemaKind::Type(openapiv3::Type::Boolean(_)) => {
            printer.println(format!("pub type {} = bool;", pascal_case).as_str())?;
        }
        SchemaKind::Type(openapiv3::Type::Array(array_schema)) => {
            let items = &array_schema.items;
            match items {
                None => {
                    return Err(Error::UnsupportedType(format!(
                        "Schema {} is an array without items: {:?}",
                        schema_name,
                        schema
                    )))
                }
                Some(items) => {
                    let item_type = get_property_type(project, items)?;
                    printer.println(format!("pub type {} = Vec<{}>;", pascal_case, item_type).as_str())?;
                }
            }
        }
        SchemaKind::Type(openapiv3::Type::Object(_)) => {
            emit_struct_type(project, &schema_name, schema, printer)?;
            emit_base_type(project, &schema_name, schema, printer)?;
        }
        SchemaKind::Any(_) => {
            emit_struct_type(project, &schema_name, schema, printer)?;
            emit_base_type(project, &schema_name, schema, printer)?;
        }
        _ => {
            return Err(Error::UnsupportedType(format!(
                "Unsupported schema {} of type: {:?}",
                schema_name,
                schema.schema_kind
            )));
        }
    }
    Ok(())
}


/// Emit Rust enum type with all children schemas and the current schema.
fn emit_base_type(
    project: &APISpec,
    schema_name: &String,
    schema: &Schema,
    printer: &mut dyn Printer,
) -> Result<()> {
    let children = project.get_children(schema_name)?;
    if children.len() == 0 {
        return Ok(());
    }
    let pascal_case = schema_name.to_case(Case::Pascal);
    let discriminator_value = schema_name;
    let discriminator = project.resolve_discriminator(schema_name, schema)?;
    printer.println("#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]")?;
    printer.println(format!("#[serde(tag = \"{}\")]", discriminator).as_str())?;
    printer.println(format!("pub enum Base{} {{", pascal_case).as_str())?;
    printer.indent();
    if pascal_case != *discriminator_value {
        printer.println(format!("#[serde(rename = \"{}\")]", discriminator_value).as_str())?;
    }
    printer.println(format!("{}({}),", pascal_case, pascal_case).as_str())?;

    let mut base_types: Vec<&String> = Vec::new();
    for child in children {
        if project.get_children(child)?.len() > 0 {
            base_types.push(child);
            continue;
        }
        let child_pascal_case = child.to_case(Case::Pascal);
        if child_pascal_case != *child{
            printer.println(format!("#[serde(rename = \"{}\")]", child).as_str())?;
        }
        printer.println(format!("{}({}),", child_pascal_case, get_type_name(project, child)?).as_str())?;
    }
    for child in base_types {
        printer.println("#[serde(untagged)]")?;
        let child_pascal_case = child.to_case(Case::Pascal);
        if child_pascal_case != *child{
            printer.println(format!("#[serde(rename = \"{}\")]", child).as_str())?;
        }
        printer.println(format!("{}({}),", child_pascal_case, get_type_name(project, child)?).as_str())?;
    }
    printer.dedent();
    printer.println("}")?;
    Ok(())
}


fn emit_struct_type(
    project: &APISpec,
    schema_name: &String,
    schema: &Schema,
    printer: &mut dyn Printer,
) -> Result<()> {
    printer.println("#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]")?;
    printer.println(format!("pub struct {} {{", schema_name.to_case(Case::Pascal)).as_str())?;
    printer.indent();
    let super_ = get_parent_schema(schema)?;
    if let Some(super_) = super_ {
        printer.println(format!("#[serde(flatten)]").as_str())?;
        printer.println(format!("pub {}_: {},", super_.to_case(Case::Snake), super_.to_case(Case::Pascal)).as_str())?;
    }
    let discriminator: String = project.resolve_discriminator(schema_name, schema).unwrap_or("".to_string());
    emit_props(schema, &discriminator, project, printer)?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn emit_props(schema: &Schema, discriminator: &str, project: &APISpec, printer: &mut dyn Printer) -> Result<()> {
    let res = match &schema.schema_kind {
        SchemaKind::Any(any_schema) => {
            emit_props2(
                project,
                &any_schema.properties,
                &any_schema.required,
                discriminator,
                printer,
            )?;
        }
        SchemaKind::Type(openapiv3::Type::Object(object_schema)) => {
            emit_props2(
                project,
                &object_schema.properties,
                &object_schema.required,
                discriminator,
                printer,
            )?;
        }
        _ => {
            return Err(Error::UnsupportedType(format!(
                "No support for anonymous Object schema: {:?}",
                schema
            )));
        }
    };
    Ok(res)
}

fn emit_props2(
    project: &APISpec,
    properties: &indexmap::IndexMap<String, ReferenceOr<Box<Schema>>>,
    required: &Vec<String>,
    discriminator: &str,
    printer: &mut dyn printer::Printer,
) -> Result<()> {
    for (name, property) in properties.iter() {
        if name == discriminator {
            continue; // skip discriminator as Serde will handle it
        }
        let mut property_type = get_property_type(project, property)?;
        let field_name = name.to_case(Case::Snake);
        let required = required.contains(&name);
        if !required {
            property_type = format!("Option<{}>", property_type);
        }
        printer.println(format!("#[serde(rename = \"{}\")]", name).as_str())?;
        printer.println(format!("pub {}: {},", field_name.into_safe(), property_type).as_str())?;
    }
    Ok(())
}

/// Get the Rust name for an openAPI schema. If the schema has children, return Base{PascalCase}.
/// If the schema does not have children, return PascalCase.
fn get_type_name(project: &APISpec, schema_name: &str) -> Result<String> {
    let pascal_case = schema_name.to_case(Case::Pascal);
    let children = project.get_children(schema_name)?;
    if children.len() == 0 {
        return Ok(pascal_case);
    }
    Ok(format!("Base{}", pascal_case))
}


fn get_property_type(project: &APISpec, property: &ReferenceOr<Box<Schema>>) -> Result<String> {
    match property {
        ReferenceOr::Item(schema) => get_schema_type(project, schema.as_ref()),
        ReferenceOr::Reference { reference } => {
            let schema_name = parse_schema_from_reference(reference)?;
            let type_name = get_type_name(project, schema_name)?;
            Ok(format!("Box<{}>",type_name))
        }
    }
}

fn get_schema_type(project: &APISpec, property: &Schema) -> Result<String> {
    let mut _str = String::new(); // hold the string until it is returned
    let res = match &property.schema_kind {
        SchemaKind::Type(openapiv3::Type::String(_)) => "String",
        SchemaKind::Type(openapiv3::Type::Integer(_)) => "i64",
        SchemaKind::Type(openapiv3::Type::Number(_)) => "f64",
        SchemaKind::Type(openapiv3::Type::Boolean(_)) => "bool",
        SchemaKind::Type(openapiv3::Type::Array(array_schema)) => {
            let items = &array_schema.items;
            match items {
                None => {
                    return Err(Error::UnsupportedType(format!(
                        "Array without items: {:?}",
                        property
                    )))
                }
                Some(items) => {
                    _str = format!("Vec<{}>", get_property_type(project, items)?);
                    _str.as_str()
                }
            }
        }
        SchemaKind::Type(openapiv3::Type::Object(_)) => {
            return Err(Error::UnsupportedType(format!(
                "Object schema: {:?}",
                property
            )))
        }
        _ => {
            return Err(Error::UnsupportedType(format!(
                "Unsupported schema: {:?}",
                property
            )))
        }
    };
    Ok(res.to_string())
}



#[cfg(test)]
mod tests {
    use super::*;
    use printer::StdoutPrinter;

    fn load_openapi() -> APISpec {
        APISpec::new("data/vi_json_openapi_specification_v8_0_2_0.json")
            .expect("Could not load openapi")
    }

    #[test]
    fn test_print_schema() {
        let project = load_openapi();
        let mut printer = StdoutPrinter::new(None, None);
        generate_rust_code(&project, "AbdicateDomOwnershipRequestType".to_string(), &mut printer)
            .expect("Could not generate rust code");
    }

    /// Test writing all schemas to a file.
    #[test]
    fn test_write_all_schemas() {
        let project = load_openapi();
        let file = std::fs::File::create("output.rs").expect("Could not create file");
        let mut printer = printer::FilePrinter::new(file, None, None);
        let comps = project.spec.components.as_ref().unwrap();
        for (name, _) in comps.schemas.iter() {
            generate_rust_code(&project, name.to_string(), &mut printer)
                .expect("Could not generate rust code");
        }
    }

    #[test]
    fn print_schema_hierarchy_test() {
        let project = load_openapi();
        print_schema_hierarchy(&project, &"VirtualDevice".to_string(), &mut StdoutPrinter::new(None, None))
            .expect("Could not print schema hierarchy");
    }

    #[test]
    fn count_schema() {
        let openapi = load_openapi().spec;
        let mut count = 0;
        for (_, _) in openapi.components.unwrap().schemas.iter() {
            count += 1;
        }

        assert_eq!(count, 7182);
    }
}
