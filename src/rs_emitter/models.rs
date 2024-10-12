// Generator for Rust data models from vim

use std::borrow::Borrow;

use crate::vim_model::VimModel;

use super::super::printer::Printer;

use super::names::*;
use super::super::vim_model::*;

const ANY: &str = "Any";

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
    emit_vim_object(vim_model, printer)?;

    emit_enums(vim_model, printer)?;
    emit_structs(vim_model, printer)?;
    emit_vimany(printer)?;
    emit_boxed_types(vim_model, printer)?;
    Ok(())
}

fn emit_use_statements(printer: &mut dyn Printer) -> Result<()> {
    printer.println("use std::{any, fmt};")?;
    printer.println("use std::collections::HashMap;")?;

    printer.newline()?;
    Ok(())
}


fn emit_vim_object(_: &VimModel, printer: &mut dyn Printer) -> Result<()> {
    printer.println(r#"pub trait VimObjectTrait : std::fmt::Debug {"#)?;
    printer.println("}")?;
    Ok(())
}

fn emit_enums(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {
    for (_, vim_enum) in &vim_model.enums {
        emit_doc(&vim_enum.description, printer)?;

        let enum_name = to_type_name(&vim_enum.name); 

        printer.println("#[derive(Debug)]")?;
        printer.println(&format!("pub enum {} {{", enum_name))?;
        printer.indent();
        for value in &vim_enum.variants {
            let variant = to_enum_variant(&value);
            printer.println(&format!("{},", variant))?;
        }
        // Make enums open i.e. handle unknown values possibly from future API servers
        printer.println("/// This variant handles values not known at compile time.")?;
        printer.println("Other_(String),")?;
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

fn emit_structs(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {
    for (name, vim_type_cell) in &vim_model.structs {
        let struct_type = vim_type_cell.borrow();
        if struct_type.name == "Any" {
            continue;
        }
        emit_struct_type(&vim_model, name, &struct_type, printer)?;
        emit_trait_type(&vim_model, name, &struct_type, printer)?;
        emit_inherited_traits(&vim_model, printer, name)?;
        emit_vim_object_trait(&vim_model, printer, name)?;
    }
    Ok(())
}

fn emit_struct_type(vim_model: &VimModel, name: &str, vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    emit_doc(&vim_type.description, printer)?;
    let struct_name = to_type_name(name);
    printer.println(&format!("#[derive(Debug)]"))?;
    printer.println(&format!("pub struct {struct_name} {{"))?;
    printer.indent();
    emit_struct_all_fields(vim_model, vim_type, printer)?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn emit_struct_all_fields(vim_model: &VimModel, vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    if let Some(parent) = vim_type.parent.as_ref() {
        if parent != "Any" { // WE do not need to emit fields for the Any type
            let parent_model_ref = vim_model.structs.get(parent).ok_or_else(|| Error::TypeNotFound(parent.clone()))?.borrow();
            let parent_model: &Struct = parent_model_ref.borrow();
            emit_struct_all_fields(vim_model, parent_model, printer)?;
        }
    }
    emit_struct_fields(vim_model, vim_type, printer)
}
fn emit_struct_fields(vim_model: &VimModel, vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    if vim_type.properties.is_empty() { return Ok(()); } // skip the comment if there are no fields
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
    if property.optional {
        field_type = format!("Option<{field_type}>", field_type = field_type);
    }
    printer.println(&format!("pub {field_name}: {field_type},"))?;
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
        VimType::DateTime => Ok("String".to_string()),
        VimType::Array(nested_type) => Ok(format!("Vec<{}>", to_rust_type(vim_model, nested_type)?)),
        VimType::Reference(ref_name) => Ok(get_ref_type_declaration(vim_model, ref_name)?),
    }
}

/// Checks if type is to be returned as value copy or reference. Integer and float types are good to
/// copy. Structures, strings and arrays go by immutable reference
fn get_by_ref(vim_type: &VimType) -> bool {
    match &vim_type {
        VimType::Reference(_) => true,
        VimType::Array(_) => true,
        VimType::Binary => true,
        VimType::String => true,
        VimType::DateTime => true, // Uses string
        _ => false,
    }
}

/// Generate the type declaration for a reference field - enum or struct.
/// If enum return just the enum Pascal case name. If structure then return a Box<> 
/// If the structure has children then we need dynamic trait reference.
/// If the Struct has no children then we reference the Struct type.
fn get_ref_type_declaration(vim_model: &VimModel, ref_name: &str) -> Result<String> {
    // If we cannot find the struct this is programatic error
    let rust_name = to_type_name(ref_name);
    if ref_name == "Any" {
        return Ok("VimAny".to_string());
    }
    if let Some(struct_type) = vim_model.structs.get(ref_name) {
        // TODO: Add Box only for structures and when there is an issue. One case is recursive types
        // that cannot be sized and thus a struct cannot be compiled. MethodFault contains
        // Option<MethodFault> and without Box it cannot be compiled. However for very many types
        // there may not be need for a Box. For example when all members are primitive it is safe to
        // not box a struct.
        let struct_ref = struct_type.borrow();
        if struct_ref.has_children() && struct_ref.name != "Any" {
            Ok(format!("Box<dyn {}Trait>", rust_name))
        } else {
            Ok(format!("Box<{}>", rust_name))
        }
    } else if let Some(_) = vim_model.enums.get(ref_name) {
        Ok(rust_name)
    } else {
        Err(Error::TypeNotFound(ref_name.to_string()))
    }
}

// To allow for polymorphic fields every structure type that is extended will have a trait
// alternative implemented that will be passed a dynamic reference. This trait will be implemented
// all of the structure type descendants. The trait will provide access to the struct type fields
// and will extend the VimObjectTrait as to allow casting between traits.
fn emit_trait_type(vim_model: &VimModel, name: &str, vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    if !vim_type.has_children() { return Ok(()); }
    if ANY == vim_type.name { return Ok(()); } // Skip the Any type
    let struct_name = to_type_name(name);
    let Some(ref parent_trait) = vim_type.parent else {
        return Ok(()); // or error?
    };
    let base_trait = to_type_name(if ANY == parent_trait {
        "VimObject"
    } else {
        parent_trait
    });
    emit_doc(&vim_type.description, printer)?;
    printer.println(&format!("pub trait {}Trait : {}Trait {{", struct_name, base_trait))?;
    printer.indent();
    for (prop_name, property) in &vim_type.properties {
        emit_trait_field(vim_model, printer, prop_name, property)?;
    }
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn emit_trait_field(vim_model: &VimModel, printer: &mut dyn Printer, prop_name: &str, property: &Property) -> Result<()> {
    emit_doc(&property.description, printer)?;
    let field_name = getter_name(&prop_name);
    let field_type = getter_return_type(vim_model, property)?;
    printer.println(&format!("fn {field_name}(&self) -> {field_type};"))?;
    Ok(())
}

fn getter_return_type(vim_model: &VimModel, property: &Property) -> Result<String> {
    let mut field_type = to_rust_type(vim_model, &property.vim_type)?;
    if property.optional {
        field_type = format!("Option<{field_type}>");
    }
    if get_by_ref(&property.vim_type) {
        field_type = format!("&{field_type}");
    }
    if "&String" == field_type {
        field_type = "&str".to_string();
    }
    Ok(field_type)
}

fn emit_inherited_traits(vim_model: &VimModel, printer: &mut dyn Printer, type_name: &String) -> Result<()> {
    let struct_name = &to_type_name(&type_name);
    let mut data_type = vim_model.structs.get(type_name).ok_or_else(|| Error::TypeNotFound(type_name.clone()))?.borrow();
    if data_type.has_children() {
        emit_trait_implementation(vim_model, printer, &data_type, struct_name)?;
    }
    let mut parent_opt = data_type.parent.as_ref();
    while let Some(parent_name) = parent_opt {
        if ANY == parent_name { break; }
        data_type = vim_model.structs.get(parent_name).ok_or_else(|| Error::TypeNotFound(parent_name.clone()))?.borrow();
        emit_trait_implementation(vim_model, printer, &data_type, struct_name)?;
        parent_opt = data_type.parent.as_ref();
    }

    Ok(())
}



/// Emits implementation of a structure type trat for a given structure. The trait should belong to
/// the same structure or an ancestor
fn emit_trait_implementation(vim_model: &VimModel, printer: &mut dyn Printer, trait_type: &Struct, struct_name: &String) -> Result<()> {
    let base_name = to_type_name(&trait_type.name);
    printer.println(&format!("impl {}Trait for {} {{", base_name, struct_name))?;
    printer.indent();
    for (prop_name, property) in &trait_type.properties {
        emit_field_getter(vim_model, printer, prop_name, property)?;
    }
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn emit_field_getter(vim_model: &VimModel, printer: &mut dyn Printer, prop_name: &str, property: &Property) -> Result<()> {
    let getter_name = getter_name(&prop_name);
    let mut field_access = format!("self.{}",to_field_name(&prop_name));
    let field_type = getter_return_type(vim_model, property)?;
    if get_by_ref(&property.vim_type) {
        field_access = format!("&{field_access}");
    }
    printer.println(&format!("fn {getter_name}(&self) -> {field_type} {{ {field_access} }}"))?;
    Ok(())
}

fn emit_vim_object_trait(vim_model: &VimModel, printer: &mut dyn Printer, type_name: &String) -> Result<()> {
    let struct_name = &to_type_name(&type_name);
    let data_type = vim_model.structs.get(type_name).ok_or_else(|| Error::TypeNotFound(type_name.clone()))?.borrow();

    // AnyType and the _value types do not need VimObjectTrait impl
    let Some(_) = &data_type.parent else { return Ok(()); };

    printer.println(&format!("impl VimObjectTrait for {} {{ }}", struct_name))?;

    Ok(())
}

fn emit_vimany(printer: &mut dyn Printer) -> Result<()> {
    printer.println("#[derive(Debug)]")?;
    printer.println(r#"pub enum VimAny {
    Object(Box<dyn VimObjectTrait>),
    Value(ValueElements),
}
"#)?;
    Ok(())
}

/// Emit boxed value types from Vim like ArrayOfInt, ArrayOfString, Boolean etc.
fn emit_boxed_types(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {
    printer.println("#[derive(Debug)]")?;
    printer.println("pub enum ValueElements {")?;
    printer.indent();
    for (_, box_type) in &vim_model.any_value_types {
        emit_doc(&box_type.description, printer)?;
        let name = box_type.discriminator_value.as_ref().unwrap_or(&box_type.name);
        let type_name = to_type_name(name);
        let rust_type = to_rust_type(vim_model, &box_type.property_type)?;
        printer.println(&format!("{type_name}({rust_type}),"))?;
    }
    printer.dedent();
    printer.println("}")?;
    Ok(())
}