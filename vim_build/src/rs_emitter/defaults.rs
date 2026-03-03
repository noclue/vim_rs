//! Emits Default trait implementations for structs, enums, and trait objects
//! into a single defaults.rs module (conditionally compiled via feature gate).

use crate::printer::Printer;
use crate::vim_model::{DataType, EmitMode, Field, Model, Struct};

use super::errors::{Error, Result};
use super::names::{parent_field_name, to_enum_variant, to_field_name, to_type_name};
use super::structs::ANY;

pub struct DefaultsEmitter<'a> {
    vim_model: &'a Model,
    printer: &'a mut dyn Printer,
}

impl<'a> DefaultsEmitter<'a> {
    pub fn new(vim_model: &'a Model, printer: &'a mut dyn Printer) -> Self {
        Self { vim_model, printer }
    }

    pub fn emit_all(&mut self) -> Result<()> {
        self.emit_use_statements()?;
        self.emit_enum_defaults()?;
        self.emit_struct_defaults()?;
        self.emit_trait_defaults()?;
        Ok(())
    }

    fn emit_use_statements(&mut self) -> Result<()> {
        self.printer.println("//! Default trait implementations for vim_rs types.")?;
        self.printer.println("//! This module is conditionally compiled when the `defaults` feature is enabled.")?;
        self.printer.newline()?;
        self.printer.println("use super::enums;")?;
        self.printer.println("use super::structs;")?;
        self.printer.println("use super::traits;")?;
        self.printer.newline()?;
        Ok(())
    }

    fn emit_enum_defaults(&mut self) -> Result<()> {
        for (_, vim_enum) in &self.vim_model.enums {
            let enum_name = to_type_name(&vim_enum.name);
            let first_variant = vim_enum
                .variants
                .first()
                .map(|v| to_enum_variant(v))
                .ok_or_else(|| Error::InternalError(format!("Enum {} has no variants", vim_enum.name)))?;

            self.printer.println(&format!(
                "impl Default for enums::{enum_name} {{"
            ))?;
            self.printer.indent();
            self.printer.println("fn default() -> Self {")?;
            self.printer.indent();
            self.printer.println(&format!("Self::{first_variant}"))?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.newline()?;
        }
        Ok(())
    }

    fn emit_struct_defaults(&mut self) -> Result<()> {
        for (_name, vim_type) in &self.vim_model.structs {
            let struct_type = vim_type.borrow();
            if struct_type.name == ANY {
                continue;
            }
            if let EmitMode::Skip(_) = struct_type.emit_mode {
                continue;
            }
            self.emit_struct_default(&struct_type)?;
        }
        Ok(())
    }

    fn emit_struct_default(&mut self, vim_type: &Struct) -> Result<()> {
        let struct_name = to_type_name(&vim_type.name);
        let is_pruned = vim_type.emit_mode == EmitMode::Prune;

        self.printer.println(&format!(
            "impl Default for structs::{struct_name} {{"
        ))?;
        self.printer.indent();
        self.printer.println("fn default() -> Self {")?;
        self.printer.indent();
        self.printer.println(&format!("Self {{"))?;
        self.printer.indent();

        // Parent field (if any) - same logic as emit_struct_all_fields
        if let Some(parent) = vim_type.parent.as_ref() {
            if parent != ANY && self.vim_model.has_any_fields_in_chain(parent)? {
                let parent_field = parent_field_name(parent);
                let parent_type = to_type_name(parent);
                self.printer.println(&format!(
                    "{parent_field}: structs::{parent_type}::default(),"
                ))?;
            }
        }

        // Own fields only
        for (_, field) in &vim_type.fields {
            let field_name = to_field_name(&field.name);
            let default_expr = self.default_expr_for_field(field)?;
            self.printer.println(&format!("{field_name}: {default_expr},"))?;
        }

        // Pruned-specific fields
        if is_pruned {
            self.printer.println("type_: None,")?;
            self.printer.println(
                "extra_fields_: std::collections::HashMap::new(),",
            )?;
        }

        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        Ok(())
    }

    fn default_expr_for_field(&self, field: &Field) -> Result<String> {
        if field.optional {
            return Ok("None".to_string());
        }

        Ok(match &field.vim_type {
            DataType::Boolean => "false".to_string(),
            DataType::String | DataType::DateTime => "String::new()".to_string(),
            DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 => {
                "0".to_string()
            }
            DataType::Float | DataType::Double => "0.0".to_string(),
            DataType::Binary => "Vec::new()".to_string(),
            DataType::Array(_) => "Vec::new()".to_string(),
            DataType::Reference(ref_name) => self.default_expr_for_ref(ref_name)?,
        })
    }

    fn default_expr_for_ref(&self, ref_name: &str) -> Result<String> {
        if ref_name == ANY {
            return Ok("Default::default()".to_string());
        }

        if let Some(struct_type) = self.vim_model.structs.get(ref_name) {
            let s = struct_type.borrow();
            let type_name = to_type_name(ref_name);
            if s.has_children() && s.emit_mode == EmitMode::Emit {
                // Box<dyn Trait> - use Default::default() which we implement for Box<dyn XxxTrait>
                Ok("Default::default()".to_string())
            } else {
                // Concrete struct
                Ok(format!("structs::{type_name}::default()"))
            }
        } else if self.vim_model.enums.contains_key(ref_name) {
            let type_name = to_type_name(ref_name);
            Ok(format!("enums::{type_name}::default()"))
        } else {
            Err(Error::TypeNotFound(ref_name.to_string()))
        }
    }

    fn emit_trait_defaults(&mut self) -> Result<()> {
        for (struct_name, struct_ref) in &self.vim_model.structs {
            let s = struct_ref.borrow();
            if struct_name == ANY || s.children.is_empty() {
                continue;
            }
            if s.emit_mode != EmitMode::Emit {
                continue;
            }

            let trait_name = to_type_name(struct_name);
            let base_struct_name = to_type_name(struct_name);

            self.printer.println(&format!(
                "impl Default for Box<dyn traits::{trait_name}Trait> {{"
            ))?;
            self.printer.indent();
            self.printer.println("fn default() -> Self {")?;
            self.printer.indent();
            self.printer.println(&format!(
                "Box::new(structs::{base_struct_name}::default())"
            ))?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.newline()?;
        }
        Ok(())
    }
}
