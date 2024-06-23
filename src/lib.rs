use convert_case::{Case, Casing};
use openapiv3::{ReferenceOr, Schema, SchemaKind};
use printer::Printer;
use check_keyword::CheckKeyword;
use spec::{APISpec, get_parent_schema, parse_schema_from_reference};
mod printer;
mod spec;

// Error is an error type for Project.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unsupported type `{}`", .0)]
    UnsupportedType(String),
    #[error("Printer error: {0}")]
    WriteError(#[from] printer::Error),
    #[error("Spec error: {0}")]
    SpecError(#[from] spec::Error),
}

// Result is a type alias for handling errors.
pub type Result<T> = std::result::Result<T, Error>;


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
        let comps = project.get_components().unwrap();
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
        let spec = load_openapi();
        let componenets = spec.get_components().unwrap();
        let mut count = 0;
        for (_, _) in componenets.schemas.iter() {
            count += 1;
        }

        assert_eq!(count, 7182);
    }
}
