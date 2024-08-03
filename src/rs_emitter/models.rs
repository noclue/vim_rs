// Generator for Rust data models from vim

use std::borrow::Borrow;

use crate::vim_model::VimModel;

use super::super::printer::Printer;

use super::names::*;
use super::super::vim_model::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Type not found error: {0}")]
    TypeNotFound(String),
    #[error("Printer error: {0}")]
    PrinterError(#[from] super::super::printer::Error),
}

// Result is a type alias for handling errors.
pub type Result<T> = std::result::Result<T, Error>;


pub fn emit_data_types(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {
    emit_use_statements(printer)?;
    emit_struct_type_table(vim_model, printer)?;
    emit_enums(vim_model, printer)?;
    emit_structs(vim_model, printer)?;
    // emit_boxed_types(vim_model, printer)?;
    Ok(())
}


fn emit_use_statements(printer: &mut dyn Printer) -> Result<()> {
    printer.println("use std::{any, fmt::Debug};")?;
    printer.println("use serde::{Deserialize, Serialize};")?;

    printer.newline()?;
    Ok(())
}

fn emit_enums(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {
    for (_, vim_enum) in &vim_model.enums {
        emit_doc(&vim_enum.description, printer)?;

        let enum_name = to_type_name(&vim_enum.name); 

        printer.println("#[derive(Debug, PartialEq, Serialize, Deserialize)]")?;
        printer.println(&format!("pub enum {} {{", enum_name))?;
        printer.indent();
        for value in &vim_enum.variants {
            let variant = to_enum_variant(&value);
            if value != &variant {
                printer.println(&format!("#[serde(rename = \"{}\")]", value))?;
            }
            printer.println(&format!("{},", variant))?;
        }
        printer.dedent();
        printer.println("}")?;
    }
    Ok(())
}

fn emit_doc(doc_string: &Option<String>, printer: &mut dyn Printer) -> Result<()> {
    Ok(if let Some(doc) = doc_string {
        for line in doc.trim().split('\n') {
            printer.println(&format!("/// {}", line))?;
        }
    })
}

fn emit_struct_type_table(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {    
    printer.println("static mut TYPE_MAP: Option<std::collections::HashMap<TypeId, Vec<TypeId>>> = None;")?;
    printer.println("static START: std::sync::Once = std::sync::Once::new();")?;

    printer.newline()?;
    
    printer.println("fn get_hierarchy(t: &any::TypeId) -> &'static Vec<any::TypeId> {")?;
    printer.indent();
    printer.println("START.call_once(|| {")?;
    printer.indent();
    printer.println("let mut map = std::collections::HashMap::new();")?;
    for (name, vim_type_cell) in &vim_model.structs {
        let mut vim_type = vim_type_cell.borrow();
        if vim_type.parent.is_none() && vim_type.name != "Any" { continue; }
        printer.print_indent()?;
        let type_name = to_type_name(name);
        printer.print(format!("map.insert(any::TypeId::of::<{type_name}>(), vec![any::TypeId::of::<{type_name}>()").as_str())?;
        let mut parent_opt = vim_type.parent.as_ref();
        while let Some(parent_name) = parent_opt {
            let type_name = to_type_name(parent_name);
            printer.print(format!(", any::TypeId::of::<{}>", type_name).as_str())?;
            let Some(parent_cell) = vim_model.structs.get(parent_name) else { 
                return Err(Error::TypeNotFound(parent_name.to_string()));
            };
            vim_type = parent_cell.borrow(); // We need this reference alive for te loop to turn
            parent_opt = vim_type.parent.as_ref();
        }
        printer.print("]);")?;
        printer.newline()?;
    }
    printer.println("unsafe { TYPE_MAP = Some(map); }")?;
    printer.dedent();
    printer.println("});")?;
    printer.println("let map = unsafe { TYPE_MAP.as_ref().unwrap() };")?;
    printer.println("let Some(types) = map.get(t) else { panic!(\"Type Id {:?} not found in the type map\", t); };")?;
    printer.println("types")?;
    printer.dedent();
    printer.println("}")?;
    
    Ok(())
}

fn emit_structs(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {
    for (name, vim_type_cell) in &vim_model.structs {
        let struct_type = vim_type_cell.borrow();
        if struct_type.parent.is_none() && struct_type.name != "Any" { continue; }
        emit_doc(&struct_type.description, printer)?;
        emit_struct_type(&vim_model, name, &struct_type, printer)?;
        emit_base_trait(&vim_model, name, &struct_type, printer)?;
    }
    Ok(())
}


fn emit_struct_type(vim_model: &VimModel, name: &str, vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    let struct_name = to_type_name(name);
    printer.println(&format!("#[derive(Debug, PartialEq, Serialize, Deserialize)]"))?;
    printer.println(&format!("pub struct {struct_name} {{"))?;
    printer.indent();
    emit_struct_all_fields(vim_model, vim_type, printer)?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn emit_struct_all_fields(vim_model: &VimModel, vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    if let Some(parent) = vim_type.parent.as_ref() {
        let parent_model_ref = vim_model.structs.get(parent).ok_or_else(|| Error::TypeNotFound(parent.clone()))?.borrow();
        let parent_model: &Struct = parent_model_ref.borrow();
        emit_struct_all_fields(vim_model, parent_model, printer)?;
    }
    emit_struct_fields(vim_model, vim_type, printer)
}
fn emit_struct_fields(vim_model: &VimModel, vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    if vim_type.properties.is_empty() { return Ok(()); }
    printer.println(&format!("// Fields of {}", vim_type.name))?;
    for (prop_name, property) in &vim_type.properties {
        emit_struct_field(vim_model, prop_name, property, printer)?;
    }
    Ok(())
}

fn emit_struct_field(vim_model: &VimModel, prop_name: &str, property: &Property, printer: &mut dyn Printer) -> Result<()> {
    emit_doc(&property.description, printer)?;
    let field_name = to_field_name(&prop_name);
    let mut field_type = to_rust_type(vim_model, &property.vim_type)?;
    if &field_name != prop_name {
        printer.println(&format!("#[serde(rename = \"{}\")]", prop_name))?;
    }
    if property.optional {
        field_type = format!("pub {field_name}: Option<{field_type}>,", field_name = field_name, field_type = field_type);
    }
    printer.println(&format!("{field_name}: {field_type},"))?;
    Ok(())
}

fn to_rust_type(vim_model: &VimModel, vim_type: &VimType) -> Result<String> {
    match &vim_type {
        VimType::Boolean => Ok("bool".to_string()),
        VimType::String => Ok("String".to_string()),
        VimType::Binary => Ok("Vec<u8>".to_string()),
        VimType::Int8 => Ok("i8".to_string()),
        VimType::Int16 => Ok("i16".to_string()),
        VimType::Int32 => Ok("i32".to_string()),
        VimType::Int64 => Ok("i64".to_string()),
        VimType::Float => Ok("f32".to_string()),
        VimType::Double => Ok("f64".to_string()),
        VimType::DateTime => Ok("chrono::DateTime<chrono::Utc>".to_string()),
        VimType::Array(nested_type) => Ok(format!("Vec<{}>", to_rust_type(vim_model, nested_type)?)),
        // TODO: Add Box only for traits and when there is a loop (e.g. MethodDault contains
        // Option<MethodFault> and without Box it cannot be compiled)
        VimType::Reference(ref_name) => Ok(format!("Box<{}>", to_struct_type(vim_model, ref_name)?)),
    }
}

/// Generate struct typed field. If the structure has children then we need to reference the "Base*" interface.
/// If the Struct has no children then we need to reference the Struct type.
fn to_struct_type(vim_model: &VimModel, ref_name: &str) -> Result<String> {
    // If we cannot find the struct thisis programatic error
    let rust_name = to_type_name(ref_name);
    if let Some(struct_type) = vim_model.structs.get(ref_name) {
        if struct_type.borrow().children.is_empty() {
            Ok(ref_name.to_string())
        } else {
            Ok(format!("dyn Base{}", ref_name))
        }            
    } else if let Some(_) = vim_model.enums.get(ref_name) {
        Ok(rust_name)
    } else {
        Err(Error::TypeNotFound(ref_name.to_string()))
    }
}

const DISCRIMINATOR: &str = "_typeName";

fn emit_base_trait(vim_model: &VimModel, name: &str, vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    if vim_type.children.is_empty() { return Ok(()); }
    let struct_name = to_type_name(name);
    let operation_name = to_fn_name(name);
    printer.println(&format!("#[typetag::serde(tag = \"{}\")]", DISCRIMINATOR))?;
    printer.println(&format!("pub trait Base{} {{", struct_name))?;
    printer.indent();
    printer.println(&format!("fn {}(&self) -> &{};", operation_name, struct_name))?;
    printer.dedent();
    printer.println("}")?;
    // Emit implementation for the base trait returning `self` tagged with `#[typetag::serde]`
    printer.println(&format!("#[typetag::serde]"))?;
    printer.println(&format!("impl Base{} for {} {{", struct_name, struct_name))?;
    printer.indent();
    printer.println(&format!("fn {}(&self) -> &{} {{ self }}", operation_name, struct_name))?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}