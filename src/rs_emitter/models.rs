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
    emit_common_types(printer)?;
    emit_vim_object(vim_model, printer)?;
    //emit_struct_type_table(vim_model, printer)?;

    emit_enums(vim_model, printer)?;
    emit_structs(vim_model, printer)?;
    emit_value_deserializers(vim_model, printer)?;
    emit_object_deserializers(vim_model, printer)?;
    emit_any_deserialization(printer)?;
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
    emit_trait_deserialization(printer, name, vim_type)
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

fn emit_trait_deserialization(printer: &mut dyn Printer, type_name: &str, vim_type: &Struct) -> Result<()> {
    let trait_name = format!("{}Trait", to_type_name(type_name));
    let type_field_name = vim_type.field_name();
    printer.println(&format!(r#"impl<'de> Deserialize<'de> for Box<dyn {trait_name}> {{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
        deserializer.deserialize_map({trait_name}Visitor)
    }}
}}

struct {trait_name}Visitor;

impl<'de> Visitor<'de> for {trait_name}Visitor {{
    type Value = Box<dyn {trait_name}>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {{
        formatter.write_str("a valid {trait_name} JSON object with a _typeName field")
}}

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {{
        let deserializer = serde::de::value::MapAccessDeserializer::new(&mut map);
        let any: VimAny = serde::de::Deserialize::deserialize(deserializer)?;
        match any {{
            VimAny::Object(obj) => Ok(obj.as_{type_field_name}_box().map_err(de::Error::custom)?),
            VimAny::Value(value) => {{
                Err(de::Error::custom(format!("expected object not wrapped value: {{:?}}", value)))
            }}
        }}
    }}
}}
"#))?;
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


fn emit_value_deserializers(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {

    printer.println("type ValueDeserializer = fn(v: &RawValue) -> Result<VimAny, serde_json::Error>;")?;
    printer.println("static mut VALUE_DESERIALIZER_MAP: Option<std::collections::HashMap<&str, Box<ValueDeserializer>>> = None;")?;
    printer.println("static INITIALIZE_VALUE_DESERIALIZERS: std::sync::Once = std::sync::Once::new();")?;
    
    printer.println("fn get_value_deserializer(type_name: &str) -> Option<&'static Box<ValueDeserializer>> {")?;
    printer.indent();
    printer.println("INITIALIZE_VALUE_DESERIALIZERS.call_once(|| {")?;
    printer.indent();
    printer.println("let mut value_deserializers: HashMap<&str, Box<ValueDeserializer>> = HashMap::new();")?;
    for (type_name, box_type) in &vim_model.any_value_types {
        let value_type = to_rust_type(vim_model, &box_type.property_type)?;
        let discriminator = box_type.discriminator_value.as_ref().unwrap_or(type_name);
        printer.println(&format!(r#"value_deserializers.insert("{discriminator}", Box::new(|v| {{"#))?;
        printer.indent();
        printer.println(&format!("let value: {value_type} = de::Deserialize::deserialize(v)"))?;
        printer.indent();
        printer.println(".map_err(de::Error::custom)?;")?;
        printer.dedent();
        printer.println("return Ok(VimAny::Value(Box::new(value)));")?;
        printer.dedent();
        printer.println("}));")?;

    }
    printer.println("unsafe { VALUE_DESERIALIZER_MAP = Some(value_deserializers); }")?;
    printer.dedent();
    printer.println("});")?;
    printer.println("let map = unsafe { VALUE_DESERIALIZER_MAP.as_ref().unwrap() };")?;
    printer.println("map.get(type_name)")?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn emit_object_deserializers(vim_model: &VimModel, printer: &mut dyn Printer) -> Result<()> {
    printer.println("type AnyDeserializer<'a> = de::value::MapDeserializer<'a, std::vec::IntoIter<(String, &'a RawValue)>, serde_json::Error>;")?;
    printer.println("type ObjectDeserializer = fn(AnyDeserializer) -> Result<VimAny, serde_json::Error>;")?;
    printer.println("static mut OBJECT_DESERIALIZER_MAP: Option<std::collections::HashMap<&str, Box<ObjectDeserializer>>> = None;")?;
    printer.println("static INITIALIZE_OBJECT_DESERIALIZERS: std::sync::Once = std::sync::Once::new();")?;
    printer.println("fn get_object_deserializer(type_name: &str) -> Option<&'static Box<ObjectDeserializer>> {")?;
    printer.indent();
    printer.println("INITIALIZE_OBJECT_DESERIALIZERS.call_once(|| {")?;
    printer.indent();
    printer.println("let mut object_deserializers: HashMap<&str, Box<ObjectDeserializer>> = HashMap::new();")?;

    for discriminator in vim_model.structs.keys() {
        let data_type = to_type_name(discriminator);
        printer.println(&format!(r#"object_deserializers.insert("{discriminator}", Box::new(|ds: AnyDeserializer| -> Result<VimAny, serde_json::Error> {{"#))?;
        printer.indent();
        printer.println(&format!(r#"let obj = {data_type}::deserialize(ds).map_err(de::Error::custom).map(|obj|VimAny::Object(Box::new(obj)))?;"#))?;
        printer.println("Ok(obj)")?;
        printer.dedent();
        printer.println("}));")?;
    };
    printer.println("unsafe { OBJECT_DESERIALIZER_MAP = Some(object_deserializers); }")?;
    printer.dedent();
    printer.println("});")?;
    printer.println("let map = unsafe { OBJECT_DESERIALIZER_MAP.as_ref().unwrap() };")?;
    printer.println("map.get(type_name)")?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn emit_any_deserialization(printer: &mut dyn Printer) -> Result<()> {
    printer.println(r#"enum VimAny {
    Object(Box<dyn VimObject>),
    Value(Box<dyn Any>)
}

impl<'de> Deserialize<'de> for VimAny {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(VimAnyVisitor)
    }
}

struct VimAnyVisitor;

impl<'de> Visitor<'de> for VimAnyVisitor {
    type Value = VimAny;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("vim JSON object with _typeName field discrimnator")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut type_name = None;
        let mut map_data: Vec<(String, &RawValue)> = Vec::new();
        while let Some(key) = map.next_key::<String>()? {
            if key == "_typeName" {
                if type_name.is_some() {
                    return Err(de::Error::duplicate_field("_typeName"));
                }
                let value: String = map.next_value()?;
                type_name = Some(value);
            } else {
                let value: &RawValue = map.next_value()?;
                map_data.push((key, value));
            }
        }
        let type_name: String = type_name.ok_or_else(|| de::Error::missing_field("_typeName"))?;

        // Process value elements
        if map_data.len() == 1 && map_data[0].0 == "_value" {
            let v: &RawValue = map_data.get(0).ok_or_else(|| de::Error::missing_field("_value"))?.1;
            if let Some(value_deserializer) = get_value_deserializer(&type_name) {
                return value_deserializer(v).map_err(de::Error::custom);
            }
        }
        let ds = de::value::MapDeserializer::new(map_data.into_iter());
        let deser = get_object_deserializer(&type_name)
            .ok_or_else(|| de::Error::custom(format!("unknown variant `{type_name}`")))?;
        deser(ds).map_err(de::Error::custom)
    }
}"#)?;
    Ok(())
}
