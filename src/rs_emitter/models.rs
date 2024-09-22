// Generator for Rust data models from vim

use std::borrow::Borrow;
use std::default;

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
    emit_common_types(printer)?;
    emit_vim_object(vim_model, printer)?;
    //emit_struct_type_table(vim_model, printer)?;

    emit_enums(vim_model, printer)?;
    emit_structs(vim_model, printer)?;
    // emit_boxed_types(vim_model, printer)?;
    Ok(())
}

fn emit_use_statements(printer: &mut dyn Printer) -> Result<()> {
    printer.println("use erased_serde::serialize_trait_object;")?;
    printer.println("use std::{any, fmt};")?;
    printer.println("use serde::{Deserialize, Deserializer, Serialize};")?;
    printer.println("use serde::de::{self, MapAccess, Visitor};")?;
    printer.println("use serde_json::value::RawValue;")?;
    printer.println("use std::collections::HashMap;")?;

    printer.newline()?;
    Ok(())
}

fn emit_common_types(printer: &mut dyn Printer) -> Result<()> {
    printer.println(r#"pub trait AsAny: Any {
        fn as_any_ref(&self) -> &dyn Any;
        fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
    }

    impl<T: VimObject + 'static> AsAny for T {
        fn as_any_ref(&self) -> &dyn Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
            self
        }
    }

type _CastResult<T> = Result<T, String>;
    "#)?;
    Ok(())
}



fn emit_vim_object(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {
    printer.println(r#"trait VimObject : AsAny  + erased_serde::Serialize {"#)?;
    printer.indent();
    emit_vim_object_declarations(vim_model, ANY, printer)?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn emit_vim_object_declarations(vim_model: &VimModel, type_name: &str, printer: &mut dyn Printer) -> Result<()> {
    let structure = get_structure(vim_model, type_name)?;
    Ok(for child in &structure.children {
        let child_struct = get_structure(vim_model, child)?;
        if child_struct.has_children() {
            printer.println(&format!("fn is_{}(&self) -> bool {{ false }}", child_struct.field_name()))?;
            printer.println(&format!(r#"fn as_{}_ref(&self) -> CastResult<&dyn {}Trait> {{ Err("Not an {}".to_string()) }}"#, 
                            child_struct.field_name(),
                            child_struct.rust_name(),
                            child_struct.rust_name()))?;
            printer.println(&format!(r#"fn as_{}_box(self: Box<Self>) -> CastResult<Box<dyn {}Trait>> {{ Err("Not an {}".to_string()) }}"#, 
                            child_struct.field_name(),
                            child_struct.rust_name(), 
                            child_struct.rust_name()))?;
            emit_vim_object_declarations(vim_model, child, printer)?;
        }
    })
}

fn emit_enums(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {
    for (_, vim_enum) in &vim_model.enums {
        emit_doc(&vim_enum.description, printer)?;

        let enum_name = to_type_name(&vim_enum.name); 

        printer.println("#[derive(Debug, Serialize, Deserialize)]")?;
        printer.println(&format!("pub enum {} {{", enum_name))?;
        printer.indent();
        for value in &vim_enum.variants {
            let variant = to_enum_variant(&value);
            if value != &variant {
                printer.println(&format!("#[serde(rename = \"{}\")]", value))?;
            }
            printer.println(&format!("{},", variant))?;
        }
        // Make enums open i.e. handle unknown values possibly from future API servers
        printer.println("/// This variant handles values not known at compile time.")?;
        printer.println("#[serde(untagged)]")?;
        printer.println("Other_(String),")?;
        printer.dedent();
        printer.println("}")?;
    }
    Ok(())
}

fn get_structure<'a>(vim_model: &'a VimModel, type_name: &str) -> Result<std::cell::Ref<'a, Struct>> {
    let structure = vim_model.structs.get(type_name).ok_or_else(|| Error::TypeNotFound(type_name.to_string()))?.borrow();
    Ok(structure)
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
            emit_root_type(&vim_model, &struct_type, printer)?;
            continue;
        }
        //if struct_type.parent.is_none() && struct_type.name != "Any" { continue; } // Skip request types
        emit_struct_type(&vim_model, name, &struct_type, printer)?;
        emit_trait_type(&vim_model, name, &struct_type, printer)?;
        emit_inherited_traits(&vim_model, printer, name)?;
        emit_vim_object_trait(&vim_model, printer, name)?;
        //emit_typed_trait(&struct_type, printer)?;
        // TODO emit_default_trait(&struct_type, printer)?;
    }
    Ok(())
}

fn emit_struct_type(vim_model: &VimModel, name: &str, vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    emit_doc(&vim_type.description, printer)?;
    let struct_name = to_type_name(name);
    if has_binary_fields_in_hierarchy(vim_model, vim_type)? {
        printer.println("#[serde_with::serde_as]")?;
    }
    printer.println(&format!("#[derive(Debug, PartialEq, Serialize, Deserialize)]"))?;
    printer.println(&format!("pub struct {struct_name} {{"))?;
    printer.indent();
    emit_struct_all_fields(vim_model, vim_type, printer)?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn has_binary_fields_in_hierarchy(vim_model: &VimModel, vim_type: &Struct) -> Result<bool> {
    if has_binary_fields(vim_type) { return Ok(true); }
    let Some(parent) = vim_type.parent.as_ref() else {
        return Ok(false)
    };
    let parent_model_ref = vim_model.structs.get(parent).ok_or_else(|| Error::TypeNotFound(parent.clone()))?.borrow();
    let parent_model: &Struct = parent_model_ref.borrow();
    has_binary_fields_in_hierarchy(vim_model, parent_model)
}

fn has_binary_fields(vim_type: &Struct) -> bool {
    vim_type.properties.iter().any(|(_, property)| property.vim_type == VimType::Binary)
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
    if &field_name != prop_name {
        printer.println(&format!("#[serde(rename = \"{}\")]", prop_name))?;
    }
    if property.vim_type == VimType::Binary {
        printer.println("#[serde_as(as = \"serde_with::base64::Base64\")]")?;
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

const DISCRIMINATOR: &str = "_typeName";

// To allow for polymorphic fields every structure type that is extended will have a trait
// alternative implemented that will be passed a dynamic reference. This trait will be implemented
// all of the structure type descendants. The trait will provide access to the struct type fields
// and will extend the VimObject as to allow casting between traits.
fn emit_trait_type(vim_model: &VimModel, name: &str, vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    if !vim_type.has_children() { return Ok(()); }
    if "Any" == vim_type.name { return Ok(()); } // Skip the Any type
    let struct_name = to_type_name(name);
    let Some(ref parent_trait) = vim_type.parent else {
        return Ok(()); // or error?
    };
    let base_trait = if "Any" == parent_trait {
        "VimObject"
    } else {
        parent_trait
    };
    emit_doc(&vim_type.description, printer)?;
    printer.println(&format!("pub trait {}Trait : {}Trait + std::fmt::Debug {{", struct_name, base_trait))?;
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
    let mut field_type = to_rust_type(vim_model, &property.vim_type)?;
    if property.optional {
        field_type = format!("Option<{field_type}>");
    }
    if get_by_ref(&property.vim_type) {
        field_type = format!("&{field_type}");
    }
    printer.println(&format!("fn {field_name}(&self) -> {field_type};"))?;
    Ok(())
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
    let mut field_type = to_rust_type(vim_model, &property.vim_type)?;
    if property.optional {
        field_type = format!("Option<{field_type}>");
    }
    if get_by_ref(&property.vim_type) {
        field_type = format!("&{field_type}");
        field_access = format!("&{field_access}");
    }
    printer.println(&format!("fn {getter_name}(&self) -> {field_type} {{ {field_access} }}"))?;
    Ok(())
}

fn emit_vim_object_trait(vim_model: &VimModel, printer: &mut dyn Printer, type_name: &String) -> Result<()> {
    let struct_name = &to_type_name(&type_name);
    let mut data_type = vim_model.structs.get(type_name).ok_or_else(|| Error::TypeNotFound(type_name.clone()))?.borrow();

    // AnyType and the _value types do not need VimObject impl
    let Some(parent_name) = &data_type.parent else { return Ok(()); };
    if ANY == parent_name && ! data_type.has_children() { return Ok(()); };

    printer.println(&format!("impl VimObject for {} {{", struct_name))?;
    printer.indent();

    if data_type.has_children() {
        emit_vim_object_impl_for_trait(printer, &data_type)?;
    }
    let mut parent_opt = data_type.parent.as_ref();
    while let Some(parent_name) = parent_opt {
        if ANY == parent_name { break; }
        data_type = vim_model.structs.get(parent_name).ok_or_else(|| Error::TypeNotFound(parent_name.clone()))?.borrow();
        emit_vim_object_impl_for_trait(printer, &data_type)?;
        parent_opt = data_type.parent.as_ref();
    }

    printer.dedent();
    printer.println("}")?;

    Ok(())
}

fn emit_vim_object_impl_for_trait(printer: &mut dyn Printer, data_type: &std::cell::Ref<'_, Struct>) -> Result<()> {
    printer.println(&format!("fn is_{}(&self) -> bool {{ true }}", data_type.field_name()))?;
    printer.println(&format!(r#"fn as_{}_ref(&self) -> CastResult<&dyn {}Trait> {{ Ok(Self) }}"#, 
                    data_type.field_name(),
                    data_type.rust_name()))?;
    printer.println(&format!(r#"fn as_{}_box(self: Box<Self>) -> CastResult<Box<dyn {}Trait>> {{ Ok(Self) }}"#, 
                    data_type.field_name(),
                    data_type.rust_name()))?;
    Ok(())
}




// fn emit_parent_trait(vim_model: &VimModel, wsdl_name: &str, type_name: &str, parent_name: &str, parent_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
//     if parent_name == "Any" { return Ok(()); } // Skip the Any type as it is enum
//     if wsdl_name != type_name {
//         printer.println(&format!("#[typetag::serde(name = \"{wsdl_name}\")]"))?;
//     } else {
//         printer.println(&format!("#[typetag::serde]"))?;
//     }
//     printer.println(&format!("impl Base{} for {} {{", to_type_name(parent_name), type_name))?;
//     printer.indent();
//     printer.println(&format!("fn {}(&self) -> &{} {{ crate::typed::cast_as(self).unwrap() }}", to_fn_name(parent_name), to_type_name(parent_name)))?;
//     printer.dedent();
//     printer.println("}")?;
//     if let Some(parent) = parent_type.parent.as_ref() {
//         let parent_model_ref = vim_model.structs.get(parent).ok_or_else(|| Error::TypeNotFound(parent.clone()))?.borrow();
//         let parent_model: &Struct = parent_model_ref.borrow();
//         emit_parent_trait(vim_model, wsdl_name, type_name, parent, parent_model, printer)?;
//     }
//     Ok(())
// }

fn emit_typed_trait(vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    if !needs_typed_trait(vim_type) { return Ok(()); }
    let struct_name = vim_type.rust_name();
    printer.println(&format!("impl crate::typed::Typed for {} {{", struct_name))?;
    printer.indent();
    printer.println(&format!("fn get_types() -> &'static Vec<any::TypeId> {{ get_hierarchy(&any::TypeId::of::<{}>()) }}", struct_name))?;
    printer.println("fn types(&self) -> &Vec<any::TypeId> { &self.types_ }")?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn needs_typed_trait(vim_type: &Struct) -> bool {
    if !vim_type.has_children() { // Childless do not need Typed if they have no parent or their parent is Any
        let Some(ref parent) = vim_type.parent else { return false; }; // No inheritance
        if parent == "Any" { return false; } // Skip types without children whose parent is Any as Any is enum
    } 
    true
}


// TODO Delete. This is not needed anymore
fn emit_types_field(vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    if !needs_typed_trait(vim_type)  { return Ok(()); } // No inheritance emulation
    let struct_name = vim_type.rust_name();
    printer.println(&format!("#[deprecated(note = \"This field is not intended to be used. Please initialize with the default value or using '{}::get_types()'.\")]", struct_name))?;
    printer.println(&format!("#[serde(skip, default = \"{}::get_types\")]", struct_name))?;
    printer.println("types_: &'static Vec<any::TypeId>,")?;
    Ok(())
}

fn emit_root_type(vim_model: &VimModel, vim_type: &Struct, printer: &mut dyn Printer) -> Result<()> {
    emit_doc(&vim_type.description, printer)?;
    printer.println("#[derive(Debug, Serialize, Deserialize)]")?;
    printer.println("#[serde(tag = \"_typeName\")]")?;
    printer.println(&format!("pub enum {} {{", vim_type.rust_name()))?;
    printer.indent();
    for name in &vim_type.children { // Render leaf nodes first expecting them to be tagged
        let child_type = vim_model.structs.get(name).ok_or_else(|| Error::TypeNotFound(name.clone()))?.borrow();
        if !child_type.children.is_empty() { continue; }
        let rust_name = child_type.rust_name();
        printer.println(&format!("{rust_name}({rust_name}),"))?;
    }

    for name in &vim_type.children { // Render non-leaf nodes untagged as we will rely on the Base* types for each of them
        let child_type = vim_model.structs.get(name).ok_or_else(|| Error::TypeNotFound(name.clone()))?.borrow();
        if child_type.children.is_empty() { continue; }
        let rust_name = child_type.rust_name();
        printer.println("#[serde(untagged)]")?;
        printer.println(&format!("{rust_name}(Box<dyn Base{rust_name}>),"))?;
    }
    printer.println("#[serde(untagged)]")?;
    printer.println(&format!("Value(ValueElements),"))?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}


/// Emit any value types from VimModel like ArrayOfInt, ArrayOfString, etc.
fn emit_boxed_types(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {
    printer.println("#[derive(Debug, Serialize, Deserialize)]")?;
    printer.println("#[serde(tag = \"_typeName\", content = \"_value\")]")?;
    printer.println("pub enum ValueElements {")?;
    printer.indent();
    for (_, box_type) in &vim_model.any_value_types {
        emit_doc(&box_type.description, printer)?;
        let type_name = to_type_name(&box_type.name);
        if type_name != box_type.name {
            printer.println(&format!("#[serde(rename = \"{}\")]", box_type.name))?;
        }
        let rust_type = to_rust_type(vim_model, &box_type.property_type)?;
        printer.println(&format!("{type_name}({rust_type}),"))?;
    }
    printer.dedent();
    printer.println("}")?;
    Ok(())
}




fn DELETE_emit_struct_type_table(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {    
    printer.println("static mut TYPE_MAP: Option<std::collections::HashMap<any::TypeId, Vec<any::TypeId>>> = None;")?;
    printer.println("static START: std::sync::Once = std::sync::Once::new();")?;

    printer.newline()?;
    
    printer.println("fn get_hierarchy(t: &any::TypeId) -> &'static Vec<any::TypeId> {")?;
    printer.indent();
    printer.println("START.call_once(|| {")?;
    printer.indent();
    printer.println("let mut map = std::collections::HashMap::new();")?;
    for (name, vim_type_cell) in &vim_model.structs {
        let mut vim_type = vim_type_cell.borrow();
        if vim_type.parent.is_none() { continue; }
        printer.print_indent()?;
        let type_name = to_type_name(name);
        printer.print(format!("map.insert(any::TypeId::of::<{type_name}>(), vec![any::TypeId::of::<{type_name}>()").as_str())?;
        let mut parent_opt = vim_type.parent.as_ref();
        while let Some(parent_name) = parent_opt {
            if parent_name == "Any" { break; } // Skip the Any type
            let type_name = to_type_name(parent_name);
            printer.print(format!(", any::TypeId::of::<{}>()", type_name).as_str())?;
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
