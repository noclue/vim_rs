// Generator for Rust data models from vim

use std::borrow::Borrow;
use crate::vim_model::Model;

use super::super::printer::Printer;

use super::super::vim_model::*;
use super::common::emit_description;
use super::errors::{Error, Result};
use super::names::*;

pub(crate) const ANY: &str = "Any";

pub struct TypesEmitter<'a> {
    vim_model: &'a Model,
    printer: &'a mut dyn Printer,
    tdf: TypeDefResolver<'a>,
}

impl<'a> TypesEmitter<'a> {
    pub fn new(vim_model: &'a Model, printer: &'a mut dyn Printer) -> Self {
        TypesEmitter {
            vim_model,
            printer,
            tdf: TypeDefResolver::new(vim_model),
        }
    }

    pub fn emit_data_types(&mut self) -> Result<()> {
        self.emit_use_statements()?;
        self.emit_structs()?;

        Ok(())
    }
    fn emit_use_statements(&mut self) -> Result<()> {
        self.printer.println("use super::vim_any::VimAny;")?;
        self.printer.println("use super::struct_enum;")?;
        self.printer.println("use serde::ser::SerializeMap;")?;
        self.printer.println("use serde::de;")?;
        self.printer.println("use std::fmt::Formatter;")?;
        self.printer.newline()?;
        Ok(())
    }

    fn emit_structs(&mut self) -> Result<()> {
        for (name, vim_type) in &self.vim_model.structs {
            let struct_type = vim_type.borrow();
            if struct_type.name == "Any" {
                continue;
            }
            if let EmitMode::Skip(_) = struct_type.emit_mode {
                continue;
            }
            self.emit_struct_type(name, &struct_type)?;
            self.emit_debug(&struct_type)?;
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
        if name == "ManagedObjectReference" {
            // Add Clone, PartialEq in addition for ManagedObjectReference
            self.printer.println("#[derive(Clone, PartialEq, Eq, Hash)]")?;
        }
        self.printer
            .println(&format!("pub struct {struct_name} {{"))?;
        self.printer.indent();
        self.emit_struct_all_fields(vim_type)?;
        if vim_type.emit_mode == EmitMode::Prune {
            self.printer.println(&format!(r#"/// Discriminator value. If `None` during serialization "{}" will be used."#, vim_type.discriminator()))?;
            self.printer.println("pub type_: Option<struct_enum::StructType>,")?;
            self.printer.println("/// Extra fields not part of the base type schema")?;
            self.printer.println("pub extra_fields_: std::collections::HashMap<String, serde_json::Value>,")?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_debug(&mut self, vim_type: &Struct) -> Result<()> {
        let prn = &mut *self.printer;
        let struct_name = to_type_name(&vim_type.name);
        prn.println(&format!("impl std::fmt::Debug for {struct_name} {{"))?;
        prn.indent();
        prn.println("fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")?;
        prn.indent();
        prn.println("let mut writer = crate::core::helpers::FmtWriter { formatter: f };")?;
        prn.println("serde_json::to_writer_pretty(&mut writer, self).map_err(|_| std::fmt::Error)")?;
        prn.dedent();
        prn.println("}")?;
        prn.dedent();
        prn.println("}")?;
        prn.newline()?;
        Ok(())
    }

    fn emit_struct_all_fields(&mut self, vim_type: &Struct) -> Result<()> {
        if let Some(parent) = vim_type.parent.as_ref() {
            if parent != "Any" {
                // WE do not need to emit fields for the Any type
                let parent_model_ref = self
                    .vim_model
                    .structs
                    .get(parent)
                    .ok_or_else(|| Error::TypeNotFound(parent.clone()))?
                    .borrow();
                let parent_model: &Struct = parent_model_ref.borrow();
                self.emit_struct_all_fields(parent_model)?;
            }
        }
        self.emit_struct_fields(vim_type)
    }
    fn emit_struct_fields(&mut self, vim_type: &Struct) -> Result<()> {
        if vim_type.fields.is_empty() {
            return Ok(());
        } // skip the comment if there are no fields
        self.printer
            .println(&format!("// Fields of {}", vim_type.name))?;
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
        self.printer
            .println(&format!("pub {field_name}: {field_type},"))?;
        Ok(())
    }

    fn emit_serialize(&mut self, vim_type: &Struct) -> Result<()> {
        let struct_name = to_type_name(&vim_type.name);
        let discriminant = vim_type.discriminator();
        let inheritance_chain = self.vim_model.inheritance_chain(&vim_type.name)?;
        self.printer
            .println(&format!("impl serde::Serialize for {struct_name} {{"))?;
        self.printer.indent();
        self.printer
            .println("fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>")?;
        self.printer.println("where")?;
        self.printer.indent();
        self.printer.println("S: serde::Serializer,")?;
        self.printer.dedent();
        self.printer.println("{")?;
        self.printer.indent();
        self.printer.println("let mut state = serializer.serialize_map(None)?;")?;
        if vim_type.emit_mode == EmitMode::Prune {
            self.printer.println(&format!(r#"let type_ = self.type_.as_ref().unwrap_or(&struct_enum::StructType::{struct_name});"#))?;
            self.printer.println(r#"state.serialize_entry("_typeName", type_)?;"#)?;
        } else {
            self.printer.println(&format!(
                "state.serialize_entry(\"_typeName\", \"{discriminant}\")?;"
            ))?;
        }
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
                    self.printer.println(&format!(
                        "state.serialize_entry(\"{serialization_name}\", {field_value})?;"
                    ))?;
                } else {
                    let field_value = if field.vim_type == DataType::Binary {
                        "&crate::core::helpers::SerializeBinary { value: field_value }"
                    } else {
                        "field_value"
                    };
                    self.printer
                        .println(&format!("if let Some(field_value) = &self.{field_name} {{"))?;
                    self.printer.indent();
                    self.printer.println(&format!(
                        "state.serialize_entry(\"{serialization_name}\", {field_value})?;"
                    ))?;
                    self.printer.dedent();
                    self.printer.println("}")?;
                }
            }
        }
        if vim_type.emit_mode == EmitMode::Prune {
            self.printer.println("for (key, value) in &self.extra_fields_ {")?;
            self.printer.indent();
            self.printer.println("state.serialize_entry(key, value)?;")?;
            self.printer.dedent();
            self.printer.println("}")?;
        }
        self.printer.println("state.end()")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_deserialize(&mut self, vim_type: &Struct) -> Result<()> {
        let struct_name = to_type_name(&vim_type.name);
        let type_name = vim_type.discriminator();
        let inheritance_chain = self.vim_model.inheritance_chain(&vim_type.name)?;

        self.printer.println(&format!(
            "impl<'de> de::Deserialize<'de> for {struct_name} {{"
        ))?;
        self.printer.indent();
        self.printer.println(
            "fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {",
        )?;
        self.printer.indent();
        if vim_type.emit_mode == EmitMode::Prune {
            self.printer.println(&format!(
                "deserializer.deserialize_map(__{struct_name}Visitor(None))"
            ))?;
        } else {
            self.printer.println(&format!(
                "deserializer.deserialize_map(__{struct_name}Visitor)"
            ))?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        if vim_type.emit_mode == EmitMode::Prune {
            self.printer
                .println(&format!("pub struct __{struct_name}Visitor(pub Option<struct_enum::StructType>);"))?;
        } else {
            self.printer
                .println(&format!("struct __{struct_name}Visitor;"))?;
        }
        self.printer.newline()?;
        self.printer.println(&format!(
            "impl<'de> de::Visitor<'de> for __{struct_name}Visitor {{"
        ))?;
        self.printer.indent();
        self.printer.println(&format!(
            "type Value = {struct_name};",
            struct_name = struct_name
        ))?;
        self.printer.newline()?;
        self.printer
            .println("fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {")?;
        self.printer.indent();
        self.printer
            .println(&format!(r#"formatter.write_str("{type_name} JSON.")"#))?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        self.printer
            .println("fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>")?;
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
                self.printer.println(&format!(
                    "let mut field{field_count}: {field_type} = None; // {field_name}"
                ))?;
                field_count += 1;
            }
        }
        if vim_type.emit_mode == EmitMode::Prune {
            self.printer.println(&format!("let mut type_: Option<struct_enum::StructType> = Some(struct_enum::StructType::{struct_name});"))?;
            self.printer.println("let mut extra_fields_: std::collections::HashMap<String, serde_json::Value> = std::collections::HashMap::new();")?;
        }
        self.printer.newline()?;
        self.printer
            .println("while let Some(key) = map.next_key::<String>()? {")?;
        self.printer.indent();
        self.printer.println("match key.as_str() {")?;
        self.printer.indent();
        self.printer.println(r#""_typeName" => {"#)?;
        self.printer.indent();
        self.printer
            .println("let discriminator: struct_enum::StructType = map.next_value()?;")?;
        self.printer
            .println(&format!(r#"if discriminator != struct_enum::StructType::{struct_name} {{"#))?;
        self.printer.indent();
        if vim_type.emit_mode == EmitMode::Prune {
            self.printer.println("type_ = Some(discriminator);")?;
        } else {
            self.printer.println(&format!(r#"return Err(de::Error::custom(format!("Expected {type_name}, got {{:?}}", discriminator)));"#))?;
        }
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
                    self.printer
                        .println(&format!("field{field_count} = Some(map.next_value()?);"))?;
                }
                self.printer.dedent();
                self.printer.println("},")?;
                field_count += 1;
            }
        }
        if vim_type.emit_mode == EmitMode::Prune {
            self.printer.println(r#"_ => {"#)?;
            self.printer.indent();
            self.printer.println("let value: serde_json::Value = map.next_value()?;")?;
            self.printer.println("extra_fields_.insert(key, value);")?;
            self.printer.dedent();
            self.printer.println("},")?;
        } else {
            self.printer
                .println(r#"_ => { let _: serde_json::Value = map.next_value()?; }"#)?;
        }
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
                self.printer
                    .println(&format!("{field_name}: {field_value},"))?;
                field_count += 1;
            }
        }
        if vim_type.emit_mode == EmitMode::Prune {
            self.printer.println("type_,")?;
            self.printer.println("extra_fields_,")?;
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
