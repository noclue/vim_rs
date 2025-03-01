// Generator for Rust data models from vim

use std::borrow::Borrow;

use crate::vim_model::Model;

use super::super::printer::Printer;

use super::common::emit_description;
use super::names::*;
use super::super::vim_model::*;
use super::errors::{Result, Error};

pub(crate) const ANY: &str = "Any";

pub struct TypesEmitter<'a> {
    vim_model: &'a Model,
    printer: &'a mut dyn Printer,
    tdf: TypeDefResolver<'a>,
}


impl<'a> TypesEmitter<'a> {
    pub fn new(vim_model: &'a Model, printer: &'a mut dyn Printer) -> Self {
        TypesEmitter { vim_model, printer, tdf: TypeDefResolver::new(vim_model) }
    }

    pub fn emit_data_types(&mut self) -> Result<()> {
        self.emit_use_statements()?;
        self.emit_structs()?;

        Ok(())
    }
    fn emit_use_statements(&mut self) -> Result<()> {
        self.printer.println("use super::vim_any::VimAny;")?;
        self.printer.println("use serde::ser::SerializeStruct;")?;
        self.printer.println("use serde::de;")?;
        self.printer.println("use std::fmt::Formatter;")?;
        self.printer.newline()?;
        Ok(())
    }

    fn emit_structs(&mut self) -> Result<()> {
        for (name, vim_type_cell) in &self.vim_model.structs {
            let struct_type = vim_type_cell.borrow();
            if struct_type.name == "Any" {
                continue;
            }
            self.emit_struct_type(name, &struct_type)?;
            self.emit_serialize(&struct_type)?;
            self.emit_deserialize(&struct_type)?;
        }
        Ok(())
    }

    fn emit_struct_type(&mut self, name: &str, vim_type: &Struct) -> Result<()> {
        {
            let this = &mut *self;
            let doc_string: &Option<String> = &vim_type.description;
            emit_description(this.printer, doc_string)
        }?;
        let struct_name = to_type_name(name);
        //let discriminator = vim_type.discriminator_value.clone().unwrap_or(name.to_string());
        if name == "ManagedObjectReference" {
            // Add Clone, PartialEq in addition for ManagedObjectReference
            self.printer.println("#[derive(Debug, Clone, PartialEq)]")?;
        } else {
            self.printer.println("#[derive(Debug)]")?;
        }
        // if struct_name == discriminator {
        //     self.printer.println(r#"#[serde(tag="_typeName")]"#)?;
        // } else {
        //     self.printer.println(&format!(r#"#[serde(rename = "{discriminator}", tag = "_typeName")]"#))?;
        // }
        self.printer.println(&format!("pub struct {struct_name} {{"))?;
        self.printer.indent();
        self.emit_struct_all_fields(vim_type)?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_struct_all_fields(&mut self, vim_type: &Struct) -> Result<()> {
        if let Some(parent) = vim_type.parent.as_ref() {
            if parent != "Any" { // WE do not need to emit fields for the Any type
                let parent_model_ref = self.vim_model.structs.get(parent).ok_or_else(|| Error::TypeNotFound(parent.clone()))?.borrow();
                let parent_model: &Struct = parent_model_ref.borrow();
                self.emit_struct_all_fields(parent_model)?;
            }
        }
        self.emit_struct_fields(vim_type)
    }
    fn emit_struct_fields(&mut self, vim_type: &Struct) -> Result<()> {
        if vim_type.fields.is_empty() { return Ok(()); } // skip the comment if there are no fields
        self.printer.println(&format!("// Fields of {}", vim_type.name))?;
        for (_, property) in &vim_type.fields {
            self.emit_struct_field(property)?;
        }
        Ok(())
    }

    fn emit_struct_field(&mut self, field: &Field) -> Result<()> {
        {
            let this = &mut *self;
            let doc_string: &Option<String> = &field.description;
            emit_description(this.printer, doc_string)
        }?;
        let field_name = to_field_name(&field.name);
        let field_type = self.tdf.field_type(field)?;
        // if field.optional {
        //     self.printer.println(&format!("#[serde(default, skip_serializing_if = \"Option::is_none\")]"))?;
        // }
        // if field_name != field.name {
        //     self.printer.println(&format!(r#"#[serde(rename = "{}")]"#, field.name))?;
        // }
        // if field.vim_type == DataType::Binary {
        //     if field.optional {
        //         self.printer.println(r#"#[serde(with = "crate::core::base64::option")]"#)?;
        //     } else {
        //         self.printer.println(r#"#[serde(with = "crate::core::base64::vec")]"#)?;
        //     }
        // }
        self.printer.println(&format!("pub {field_name}: {field_type},"))?;
        Ok(())
    }

    // Emit serde::Serialize for the struct. It should look roughly as follows. In essence iterate
    // over the fields and serialize each field. If the field is optional then skip serializing if
    // the field is None.
    // impl serde::Serialize for Cat {
    //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    //     where
    //         S: serde::Serializer,
    //     {
    //         let mut state = serializer.serialize_struct("Cat", 4)?;
    //         state.serialize_field("_typeName", "Cat")?;
    //         state.serialize_field("name", &self.name)?;
    //         state.serialize_field("tricolor", &self.tricolor)?;
    //         if let Some(friend) = &self.friend {
    //             state.serialize_field("friend", friend)?;
    //         } else {
    //             state.skip_field("friend")?;
    //         };
    //         state.end()
    //     }
    // }
    fn emit_serialize(&mut self, vim_type: &Struct) -> Result<()> {
        let struct_name = to_type_name(&vim_type.name);
        let mut field_count = 1;
        let inheritance_chain = self.vim_model.inheritance_chain(&vim_type.name)?;
        for struct_type in &inheritance_chain {
            field_count = field_count + (*struct_type).borrow().fields.len();
        }
        self.printer.println(&format!("impl serde::Serialize for {struct_name} {{"))?;
        self.printer.indent();
        self.printer.println("fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>")?;
        self.printer.println("where")?;
        self.printer.indent();
        self.printer.println("S: serde::Serializer,")?;
        self.printer.dedent();
        self.printer.println("{")?;
        self.printer.indent();
        self.printer.println(&format!("let mut state = serializer.serialize_struct(\"{struct_name}\", {field_count})?;"))?;
        self.printer.println(&format!("state.serialize_field(\"_typeName\", \"{struct_name}\")?;"))?;
        for struct_type in inheritance_chain {
            for (_, field) in &struct_type.borrow().fields {
                let field_name = to_field_name(&field.name);
                let serialization_name = &field.name;
                if !field.optional {
                    let field_value = if field.vim_type == DataType::Binary {
                        format!("&crate::core::helpers::SerializeBinary {{ value: &self.{field_name} }}")
                    } else {
                        format!("&self.{field_name}")
                    };
                    self.printer.println(&format!("state.serialize_field(\"{serialization_name}\", {field_value})?;"))?;
                } else {
                    let field_value = if field.vim_type == DataType::Binary {
                        "&crate::core::helpers::SerializeBinary { value: field_value }"
                    } else {
                        "field_value"
                    };
                    self.printer.println(&format!("if let Some(field_value) = &self.{field_name} {{"))?;
                    self.printer.indent();
                    self.printer.println(&format!("state.serialize_field(\"{serialization_name}\", {field_value})?;"))?;
                    self.printer.dedent();
                    self.printer.println("} else {")?;
                    self.printer.indent();
                    self.printer.println(&format!("state.skip_field(\"{serialization_name}\")?;"))?;
                    self.printer.dedent();
                    self.printer.println("}")?;
                }
            }
        }
        self.printer.println("state.end()")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    // Emit serde::Deserialize for the struct. In essence:
    // - Provide Deserialize for the struct e.g `impl<'de> de::Deserialize<'de> for Cat`
    // - Provide Visitor for the struct like `__CatVisitor` with visit_map method
    // - Declare Option placeholder for each field using sequential names like `field0`, `field1`
    // - Deserialize the fields of the struct using a match statement in a loop
    // - Move the deserialized fields into new struct instance while checking for missing required fields
    // - Return the new struct instance
    // Use the DeserializeBinary helper for binary fields
    // Example:
    // impl<'de> de::Deserialize<'de> for Cat {
    //     fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    //         deserializer.deserialize_map(__CatVisitor)
    //     }
    // }
    // 
    // struct __CatVisitor;
    // 
    // impl<'de> de::Visitor<'de> for __CatVisitor {
    //     type Value = Cat;
    // 
    //     fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
    //         formatter.write_str("A Cat!")
    //     }
    // 
    //     fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    //     where
    //         A: MapAccess<'de>,
    //     {
    //         let mut field1: Option<String> = None;
    //         let mut field2: Option<bool> = None;
    //         let mut field3: Option<Box<dyn AnimalTrait>> = None;
    //         let mut field4: Option<Vec<u8>> = None;
    //         while let Some(key) = map.next_key::<String>()? {
    //             match key.as_str() {
    //                 "_typeName" => {
    //                     let discriminator: String = map.next_value()?;
    //                     if discriminator != "Cat" {
    //                         return Err(de::Error::custom(format!(
    //                             "Expected Cat, got {}",
    //                             discriminator
    //                         )));
    //                     }
    //                 }
    //                 "name" => {
    //                     field1 = Some(map.next_value()?);
    //                 }
    //                 "tricolor" => {
    //                     field2 = Some(map.next_value()?);
    //                 }
    //                 "friend" => {
    //                     field3 = Some(map.next_value()?);
    //                 }
    //                 "data" => {
    //                     field4 = Some(map.next_value::<DeserializeBinary>()?.value);
    //                 }
    //                 _ => {
    //                     let _: serde_json::Value = map.next_value()?;
    //                 }
    //             }
    //         }
    //         Ok(Cat {
    //             name: field1.ok_or(de::Error::missing_field("name"))?,
    //             tricolor: field2.ok_or(de::Error::missing_field("tricolor"))?,
    //             friend: field3,
    //             data: field4,
    //         })
    //     }
    // }
    fn emit_deserialize(&mut self, vim_type: &Struct) -> Result<()> {
        let struct_name = to_type_name(&vim_type.name);
        let type_name = vim_type.discriminator_value.clone().unwrap_or(vim_type.name.clone());
        let inheritance_chain = self.vim_model.inheritance_chain(&vim_type.name)?;

        self.printer.println(&format!("impl<'de> de::Deserialize<'de> for {struct_name} {{"))?;
        self.printer.indent();
        self.printer.println(&format!("fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{"))?;
        self.printer.indent();
        self.printer.println(&format!("deserializer.deserialize_map(__{struct_name}Visitor)"))?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        self.printer.println(&format!("struct __{struct_name}Visitor;"))?;
        self.printer.newline()?;
        self.printer.println(&format!("impl<'de> de::Visitor<'de> for __{struct_name}Visitor {{"))?;
        self.printer.indent();
        self.printer.println(&format!("type Value = {struct_name};", struct_name = struct_name))?;
        self.printer.newline()?;
        self.printer.println("fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {")?;
        self.printer.indent();
        self.printer.println(&format!(r#"formatter.write_str("A {type_name} JSON.")"#))?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        self.printer.println("fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>")?;
        self.printer.println("where")?;
        self.printer.indent();
        self.printer.println("A: de::MapAccess<'de>,")?;
        self.printer.dedent();
        self.printer.println("{")?;
        self.printer.indent();
        let mut field_count = 1;
        for struct_type in &inheritance_chain {
            for (_, field) in &(*struct_type).borrow().fields {
                let field_name = to_field_name(&field.name);
                let field_type = self.tdf.field_type(field)?;
                let field_type = if !field.optional {
                    format!("Option<{}>", field_type)
                } else {
                    field_type
                };
                self.printer.println(&format!("let mut field{field_count}: {field_type} = None; // {field_name}"))?;
                field_count += 1;
            }
        }
        self.printer.newline()?;
        self.printer.println("while let Some(key) = map.next_key::<String>()? {")?;
        self.printer.indent();
        self.printer.println("match key.as_str() {")?;
        self.printer.indent();
        self.printer.println(r#""_typeName" => {"#)?;
        self.printer.indent();
        self.printer.println("let discriminator: String = map.next_value()?;")?;
        self.printer.println(&format!(r#"if discriminator != "{type_name}" {{"#))?;
        self.printer.indent();
        self.printer.println(&format!(r#"return Err(de::Error::custom(format!("Expected {type_name}, got {{discriminator}}")));"#))?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("},")?;
        field_count = 1;
        for struct_type in &inheritance_chain {
            for (_, field) in &(*struct_type).borrow().fields {
                let ser_name = &field.name;
                self.printer.println(&format!(r#""{ser_name}" => {{"#))?;
                self.printer.indent();
                if field.vim_type == DataType::Binary {
                    self.printer.println(&format!("field{field_count} = Some(map.next_value::<crate::core::helpers::DeserializeBinary>()?.value);"))?;
                } else {
                    self.printer.println(&format!("field{field_count} = Some(map.next_value()?);"))?;
                }
                self.printer.dedent();
                self.printer.println("},")?;
                field_count += 1;
            }
        }
        self.printer.println(r#"_ => { let _: serde_json::Value = map.next_value()?; }"#)?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        self.printer.println(&format!("Ok({struct_name} {{"))?;
        self.printer.indent();
        field_count = 1;
        for struct_type in &inheritance_chain {
            for (_, field) in &(*struct_type).borrow().fields {
                let field_name = to_field_name(&field.name);
                let field_value = if !field.optional {
                    format!("field{field_count}.ok_or(de::Error::missing_field(\"{field_name}\"))?")
                } else {
                    format!("field{field_count}")
                };
                self.printer.println(&format!("{field_name}: {field_value},"))?;
                field_count += 1;
            }
        }
        self.printer.dedent();
        self.printer.println("})")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        Ok(())
    }
}


/// Checks if type is to be returned as value copy or reference. Integer and float types are good to
/// copy. Structures, strings and arrays go by immutable reference
pub fn get_by_ref(vim_type: &DataType) -> bool {
    match &vim_type {
        DataType::Reference(_) => true,
        DataType::Array(_) => true,
        DataType::Binary => true,
        DataType::String => true,
        DataType::DateTime => true, // Uses string
        _ => false,
    }
}




