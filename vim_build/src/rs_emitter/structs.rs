// Generator for Rust data models from vim

use crate::vim_model::Model;

use super::super::printer::Printer;

use super::super::vim_model::*;
use super::common::{emit_description, emit_description_with_paths};
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
        self.printer.println("use super::struct_enum;")?;
        self.printer
            .println("use super::mini_de_static::{FieldsBuilder, VimObjectHolder};")?;
        self.printer
            .println("use super::mini_helpers::Base64;")?;
        self.printer
            .println("use super::convert::CastFrom;")?;
        self.printer.println("use miniserde::ser::Fragment;")?;
        self.printer.println("use std::borrow::Cow;")?;
        self.printer.newline()?;
        self.printer.println("miniserde::make_place!(Place);")?;
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
            emit_description_with_paths(this.printer, doc_string, &vim_type.paths)
        }?;
        let struct_name = to_type_name(name);
        if name == "ManagedObjectReference" {
            // Add Clone, PartialEq in addition for ManagedObjectReference
            self.printer
                .println("#[derive(Clone, PartialEq, Eq, Hash)]")?;
        }
        self.printer
            .println(&format!("pub struct {struct_name} {{"))?;
        self.printer.indent();
        self.emit_struct_all_fields(vim_type)?;
        if vim_type.emit_mode == EmitMode::Prune {
            self.printer.println(&format!(
                r#"/// Discriminator value. If `None` during serialization "{}" will be used."#,
                vim_type.discriminator()
            ))?;
            self.printer
                .println("pub type_: Option<struct_enum::StructType>,")?;
            self.printer
                .println("/// Extra fields not part of the base type schema")?;
            self.printer.println(
                "pub extra_fields_: std::collections::HashMap<String, miniserde::json::Value>,",
            )?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        self.emit_deref_implementations(vim_type)?;
        Ok(())
    }

    fn emit_deref_implementations(&mut self, vim_type: &Struct) -> Result<()> {
        // Only emit Deref if there's a parent with fields (not a marker trait)
        if let Some(parent) = vim_type.parent.as_ref() {
            if parent != "Any" && self.vim_model.has_any_fields_in_chain(parent)? {
                let struct_name = to_type_name(&vim_type.name);
                let parent_type = to_type_name(parent);
                let parent_field = parent_field_name(parent);

                // Emit Deref implementation
                self.printer
                    .println(&format!("impl std::ops::Deref for {struct_name} {{"))?;
                self.printer.indent();
                self.printer
                    .println(&format!("type Target = super::structs::{parent_type};"))?;
                self.printer.newline()?;
                self.printer.println("fn deref(&self) -> &Self::Target {")?;
                self.printer.indent();
                self.printer.println(&format!("&self.{parent_field}"))?;
                self.printer.dedent();
                self.printer.println("}")?;
                self.printer.dedent();
                self.printer.println("}")?;
                self.printer.newline()?;

                // Emit DerefMut implementation
                self.printer
                    .println(&format!("impl std::ops::DerefMut for {struct_name} {{"))?;
                self.printer.indent();
                self.printer
                    .println("fn deref_mut(&mut self) -> &mut Self::Target {")?;
                self.printer.indent();
                self.printer.println(&format!("&mut self.{parent_field}"))?;
                self.printer.dedent();
                self.printer.println("}")?;
                self.printer.dedent();
                self.printer.println("}")?;
                self.printer.newline()?;
            }
        }
        Ok(())
    }

    fn emit_debug(&mut self, vim_type: &Struct) -> Result<()> {
        let prn = &mut *self.printer;
        let struct_name = to_type_name(&vim_type.name);
        prn.println(&format!("impl std::fmt::Debug for {struct_name} {{"))?;
        prn.indent();
        prn.println("fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")?;
        prn.indent();
        prn.println("let json = miniserde::json::to_string(self);")?;
        prn.println("crate::types::mini_helpers::write_pretty_json(f, &json)")?;
        prn.dedent();
        prn.println("}")?;
        prn.dedent();
        prn.println("}")?;
        prn.newline()?;
        Ok(())
    }

    fn emit_struct_all_fields(&mut self, vim_type: &Struct) -> Result<()> {
        // Emit a single parent field instead of flattening all parent fields
        // Skip parent field if parent has no fields (it's just a marker trait)
        if let Some(parent) = vim_type.parent.as_ref() {
            if parent != "Any" && self.vim_model.has_any_fields_in_chain(parent)? {
                // Emit parent field for composition
                let parent_field = parent_field_name(parent);
                let parent_type = to_type_name(parent);
                self.printer.println(&format!("// Parent field"))?;
                self.printer.println(&format!(
                    "pub {parent_field}: super::structs::{parent_type},"
                ))?;
            }
        }
        // Emit only this type's own fields
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

    /// Build field access path for serialization. With composition, fields from parent types
    /// are accessed through parent fields. E.g., for VirtualVmxnet accessing VirtualDevice.key:
    /// "self.virtual_ethernet_card_.virtual_device_.key"
    /// Skip empty parent types (marker traits) that have no fields.
    /// Build the access path for a field relative to the struct root (no `self.` prefix).
    /// Returns a path like `field_name` for own fields, or
    /// `parent_.field_name` for inherited fields.
    /// The caller prepends `self.data.` as needed.
    fn build_field_access_path(
        &self,
        vim_type: &Struct,
        target_struct_name: &str,
        field_name: &str,
    ) -> Result<String> {
        if vim_type.name == target_struct_name {
            // Field belongs to current struct
            return Ok(field_name.to_string());
        }

        // Need to navigate through parent chain, skipping empty parents
        let mut path = String::new();
        let mut current_type_name = vim_type.name.clone();

        loop {
            let current_struct = self
                .vim_model
                .structs
                .get(&current_type_name)
                .ok_or_else(|| Error::TypeNotFound(current_type_name.clone()))?
                .borrow();

            if let Some(parent) = current_struct.parent.as_ref() {
                if parent == "Any" {
                    break;
                }

                // Only add parent field to path if parent has fields (not a marker trait)
                if self.vim_model.has_any_fields_in_chain(parent)? {
                    let parent_field = parent_field_name(parent);
                    if path.is_empty() {
                        path.push_str(&parent_field);
                    } else {
                        path.push_str(&format!(".{}", parent_field));
                    }
                }

                if parent == target_struct_name {
                    // Found the parent that owns the field
                    if path.is_empty() {
                        path.push_str(field_name);
                    } else {
                        path.push_str(&format!(".{}", field_name));
                    }
                    return Ok(path);
                }

                // Continue up the chain
                current_type_name = parent.clone();
            } else {
                break;
            }
        }

        // Fallback
        Ok(field_name.to_string())
    }

    fn emit_serialize(&mut self, vim_type: &Struct) -> Result<()> {
        let struct_name = to_type_name(&vim_type.name);
        let discriminant = vim_type.discriminator();
        let inheritance_chain = self.vim_model.inheritance_chain(&vim_type.name)?;
        let ser_name = format!("{struct_name}Serializer");
        let is_pruned = vim_type.emit_mode == EmitMode::Prune;

        // Collect all fields with their metadata for both the constructor and next()
        struct FieldInfo {
            serialization_name: String,
            field_access: String,
            optional: bool,
            is_binary: bool,
        }
        let mut fields = Vec::new();
        for struct_type in &inheritance_chain {
            for (_, field) in &struct_type.borrow().fields {
                let field_name = to_field_name(&field.name);
                let field_access = self.build_field_access_path(
                    vim_type,
                    &struct_type.borrow().name,
                    &field_name,
                )?;
                fields.push(FieldInfo {
                    serialization_name: field.name.clone(),
                    field_access,
                    optional: field.optional,
                    is_binary: field.vim_type == DataType::Binary,
                });
            }
        }

        let has_binary = fields.iter().any(|f| f.is_binary);
        let has_optional = fields.iter().any(|f| f.optional);
        // Whether self.data is accessed in next() -- non-binary fields or pruned extra_fields_
        let needs_data = fields.iter().any(|f| !f.is_binary) || is_pruned;
        // Whether the serializer needs a constructor (data access or binary pre-computation)
        let needs_constructor = needs_data || has_binary;

        // 1. impl miniserde::Serialize for StructName
        self.printer.println(&format!(
            "impl miniserde::Serialize for {struct_name} {{"
        ))?;
        self.printer.indent();
        self.printer.println(&format!(
            "fn begin(&self) -> Fragment<'_> {{"
        ))?;
        self.printer.indent();
        if needs_constructor {
            self.printer.println(&format!(
                "Fragment::Map(Box::new({ser_name}::new(self)))"
            ))?;
        } else {
            self.printer.println(&format!(
                "Fragment::Map(Box::new({ser_name} {{ seq: 0 }}))"
            ))?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // 2. Serializer struct
        if needs_data {
            self.printer.println(&format!(
                "struct {ser_name}<'a> {{"
            ))?;
            self.printer.indent();
            self.printer.println(&format!(
                "data: &'a {struct_name},"
            ))?;
        } else {
            self.printer.println(&format!(
                "struct {ser_name} {{"
            ))?;
            self.printer.indent();
        }
        self.printer.println("seq: usize,")?;
        if is_pruned {
            self.printer.println("type_name: &'static str,")?;
            self.printer.println(&format!(
                "extra_iter: Option<std::collections::hash_map::Iter<'a, String, miniserde::json::Value>>,"
            ))?;
        }
        if has_binary {
            // Add pre-computed base64 fields for binary data
            for (i, f) in fields.iter().enumerate() {
                if f.is_binary {
                    if f.optional {
                        self.printer.println(&format!(
                            "b64_{i}: Option<String>,"
                        ))?;
                    } else {
                        self.printer.println(&format!(
                            "b64_{i}: String,"
                        ))?;
                    }
                }
            }
        }
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // 3. Constructor (needed when serializer holds data or has pre-computed fields)
        if needs_constructor {
            if needs_data {
                self.printer.println(&format!(
                    "impl<'a> {ser_name}<'a> {{"
                ))?;
                self.printer.indent();
                self.printer.println(&format!(
                    "fn new(data: &'a {struct_name}) -> Self {{"
                ))?;
            } else {
                self.printer.println(&format!(
                    "impl {ser_name} {{"
                ))?;
                self.printer.indent();
                self.printer.println(&format!(
                    "fn new(data: &{struct_name}) -> Self {{"
                ))?;
            }
            self.printer.indent();

            if is_pruned {
                self.printer.println(&format!(
                    r#"let type_name: &'static str = data.type_.as_ref().map(|t| t.as_str()).unwrap_or("{discriminant}");"#
                ))?;
            }

            // Pre-compute base64 for binary fields
            if has_binary {
                for (i, f) in fields.iter().enumerate() {
                    if f.is_binary {
                        let access = &f.field_access;
                        if f.optional {
                            self.printer.println(&format!(
                                "let b64_{i} = data.{access}.as_ref().map(|data| base64::display::Base64Display::new(data, &base64::engine::general_purpose::STANDARD).to_string());"
                            ))?;
                        } else {
                            self.printer.println(&format!(
                                "let b64_{i} = base64::display::Base64Display::new(&data.{access}, &base64::engine::general_purpose::STANDARD).to_string();"
                            ))?;
                        }
                    }
                }
            }

            self.printer.println("Self {")?;
            self.printer.indent();
            if needs_data {
                self.printer.println("data,")?;
            }
            self.printer.println("seq: 0,")?;
            if is_pruned {
                self.printer.println("type_name,")?;
                self.printer.println("extra_iter: None,")?;
            }
            if has_binary {
                for (i, f) in fields.iter().enumerate() {
                    if f.is_binary {
                        self.printer.println(&format!("b64_{i},"))?;
                    }
                }
            }
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.newline()?;
        }

        // 4. impl Map for Serializer
        if needs_data {
            self.printer.println(&format!(
                "impl<'a> miniserde::ser::Map for {ser_name}<'a> {{"
            ))?;
        } else {
            self.printer.println(&format!(
                "impl miniserde::ser::Map for {ser_name} {{"
            ))?;
        }
        self.printer.indent();
        self.printer.println(
            "fn next(&mut self) -> Option<(Cow<'_, str>, &dyn miniserde::Serialize)> {"
        )?;
        self.printer.indent();

        // Use loop for optional field skipping
        if has_optional || is_pruned {
            self.printer.println("loop {")?;
            self.printer.indent();
        }

        // For pruned types, drain extra_iter first
        if is_pruned {
            self.printer.println("if let Some(iter) = &mut self.extra_iter {")?;
            self.printer.indent();
            self.printer.println("if let Some((key, value)) = iter.next() {")?;
            self.printer.indent();
            self.printer.println(
                "return Some((Cow::Borrowed(key.as_str()), value as &dyn miniserde::Serialize));"
            )?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.println("self.extra_iter = None;")?;
            self.printer.println("return None;")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.newline()?;
        }

        self.printer.println("let seq = self.seq;")?;
        self.printer.println("self.seq += 1;")?;
        self.printer.println("match seq {")?;
        self.printer.indent();

        // seq 0: _typeName
        if is_pruned {
            self.printer.println(
                "0 => return Some((Cow::Borrowed(\"_typeName\"), &self.type_name as &dyn miniserde::Serialize)),"
            )?;
        } else {
            self.printer.println(&format!(
                "0 => return Some((Cow::Borrowed(\"_typeName\"), &\"{discriminant}\")),",
            ))?;
        }

        // Remaining fields
        for (i, f) in fields.iter().enumerate() {
            let seq_num = i + 1;
            let ser_name_str = &f.serialization_name;
            let field_access = &f.field_access;

            if f.optional {
                self.printer.println(&format!("{seq_num} => {{"))?;
                self.printer.indent();
                if f.is_binary {
                    self.printer.println(&format!(
                        "let Some(ref b64) = self.b64_{i} else {{ continue; }};"
                    ))?;
                    self.printer.println(&format!(
                        "return Some((Cow::Borrowed(\"{ser_name_str}\"), b64 as &dyn miniserde::Serialize));"
                    ))?;
                } else {
                    self.printer.println(&format!(
                        "let Some(ref val) = self.data.{field_access} else {{ continue; }};"
                    ))?;
                    self.printer.println(&format!(
                        "return Some((Cow::Borrowed(\"{ser_name_str}\"), val as &dyn miniserde::Serialize));"
                    ))?;
                }
                self.printer.dedent();
                self.printer.println("}")?;
            } else if f.is_binary {
                self.printer.println(&format!(
                    "{seq_num} => return Some((Cow::Borrowed(\"{ser_name_str}\"), &self.b64_{i} as &dyn miniserde::Serialize)),"
                ))?;
            } else {
                self.printer.println(&format!(
                    "{seq_num} => return Some((Cow::Borrowed(\"{ser_name_str}\"), &self.data.{field_access} as &dyn miniserde::Serialize)),"
                ))?;
            }
        }

        // After all fields
        let after_fields_seq = fields.len() + 1;
        if is_pruned {
            self.printer.println(&format!("{after_fields_seq} => {{"))?;
            self.printer.indent();
            self.printer.println("if !self.data.extra_fields_.is_empty() {")?;
            self.printer.indent();
            self.printer.println("self.extra_iter = Some(self.data.extra_fields_.iter());")?;
            self.printer.println("continue;")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.println("return None;")?;
            self.printer.dedent();
            self.printer.println("}")?;
        }

        self.printer.println("_ => return None,")?;
        self.printer.dedent();
        self.printer.println("}")?;

        if has_optional || is_pruned {
            self.printer.dedent();
            self.printer.println("}")?;
        }

        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        Ok(())
    }

    fn emit_deserialize(&mut self, vim_type: &Struct) -> Result<()> {
        let struct_name = to_type_name(&vim_type.name);
        let fields_name = format!("{struct_name}Fields");
        let inheritance_chain = self.vim_model.inheritance_chain(&vim_type.name)?;
        let is_pruned = vim_type.emit_mode == EmitMode::Prune;

        // Collect field info for deserialization
        enum DeserFieldKind {
            Normal,
            Binary,
            TraitObject { trait_name: String, base_type_name: String },
        }
        struct DeserField {
            field_name: String,
            ser_name: String,
            field_type: String,
            optional: bool,
            kind: DeserFieldKind,
        }
        let mut deser_fields = Vec::new();
        for struct_type in &inheritance_chain {
            for (_, field) in &struct_type.borrow().fields {
                let field_name = to_field_name(&field.name);
                let field_type = self.tdf.field_type(field)?;
                let kind = if field.vim_type == DataType::Binary {
                    DeserFieldKind::Binary
                } else if let DataType::Reference(ref_name) = &field.vim_type {
                    // Check if the referenced struct has children and is not pruned.
                    // "Any" maps to VimAny (an enum), not a trait object.
                    if ref_name == "Any" {
                        DeserFieldKind::Normal
                    } else if let Some(s) = self.vim_model.structs.get(ref_name.as_str()) {
                        let s_ref = s.borrow();
                        if s_ref.has_children() && s_ref.emit_mode == EmitMode::Emit {
                            DeserFieldKind::TraitObject {
                                trait_name: format!(
                                    "super::traits::{}Trait",
                                    to_type_name(ref_name)
                                ),
                                base_type_name: ref_name.clone(),
                            }
                        } else {
                            DeserFieldKind::Normal
                        }
                    } else {
                        DeserFieldKind::Normal
                    }
                } else {
                    DeserFieldKind::Normal
                };
                deser_fields.push(DeserField {
                    field_name,
                    ser_name: field.name.clone(),
                    field_type,
                    optional: field.optional,
                    kind,
                });
            }
        }

        // 1. impl Deserialize for StructName
        self.printer.println(&format!(
            "impl miniserde::Deserialize for {struct_name} {{"
        ))?;
        self.printer.indent();
        self.printer.println(&format!(
            "fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {{"
        ))?;
        self.printer.indent();
        self.printer.println("Place::new(out)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // 2. impl Visitor for Place<StructName>
        self.printer.println(&format!(
            "impl miniserde::de::Visitor for Place<{struct_name}> {{"
        ))?;
        self.printer.indent();
        self.printer.println(
            "fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {"
        )?;
        self.printer.indent();
        self.printer.println(&format!(
            "Ok(Box::new({fields_name}::with_output(&mut self.out)))"
        ))?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // 3. Fields struct
        self.printer.println(&format!(
            "pub struct {fields_name}<'a> {{"
        ))?;
        self.printer.indent();
        for (i, f) in deser_fields.iter().enumerate() {
            let deser_type = match &f.kind {
                DeserFieldKind::Binary => "Option<Base64>".to_string(),
                DeserFieldKind::TraitObject { .. } => "Option<VimObjectHolder>".to_string(),
                DeserFieldKind::Normal => {
                    if f.optional {
                        f.field_type.clone()  // Already Option<T>
                    } else {
                        format!("Option<{}>", f.field_type) // Wrap in Option for accumulation
                    }
                }
            };
            self.printer.println(&format!(
                "f{i}: {deser_type}, // {}", f.field_name
            ))?;
        }
        if is_pruned {
            self.printer.println("type_: Option<struct_enum::StructType>,")?;
            self.printer.println("type_name: Option<String>,")?;
            self.printer.println("extra_fields_: std::collections::HashMap<String, miniserde::json::Value>,")?;
            self.printer.println("current_extra_key: Option<String>,")?;
            self.printer.println("current_extra_value: Option<miniserde::json::Value>,")?;
            self.printer.println("#[cfg(feature = \"xml\")]")?;
            self.printer.println("resolved_type: Option<struct_enum::StructType>,")?;
            self.printer.println("#[cfg(feature = \"xml\")]")?;
            self.printer
                .println("api_extra_visitor: super::api_typed_visitor::ApiTypedValueVisitor,")?;
        }
        self.printer.println(&format!(
            "__out: Option<&'a mut Option<{struct_name}>>,"
        ))?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // 4. Constructors
        self.printer.println(&format!(
            "impl {fields_name}<'_> {{"
        ))?;
        self.printer.indent();

        // new() for polymorphic use
        if is_pruned {
            self.printer.println(&format!(
                "pub fn new(type_: Option<struct_enum::StructType>) -> {fields_name}<'static> {{"
            ))?;
        } else {
            self.printer.println(&format!(
                "pub fn new() -> {fields_name}<'static> {{"
            ))?;
        }
        self.printer.indent();
        self.printer.println(&format!("{fields_name} {{"))?;
        self.printer.indent();
        for (i, f) in deser_fields.iter().enumerate() {
            match &f.kind {
                DeserFieldKind::TraitObject { base_type_name, .. } => {
                    self.printer.println(&format!(
                        "f{i}: Some(VimObjectHolder {{ out: None, default_type_name: Some(\"{base_type_name}\") }}),"
                    ))?;
                }
                _ => {
                    self.printer.println(&format!("f{i}: None,"))?;
                }
            }
        }
        if is_pruned {
            self.printer.println("type_,")?;
            self.printer.println("type_name: None,")?;
            self.printer.println("extra_fields_: std::collections::HashMap::new(),")?;
            self.printer.println("current_extra_key: None,")?;
            self.printer.println("current_extra_value: None,")?;
            self.printer.println("#[cfg(feature = \"xml\")]")?;
            self.printer.println("resolved_type: type_,")?;
            self.printer.println("#[cfg(feature = \"xml\")]")?;
            self.printer.println(
                "api_extra_visitor: super::api_typed_visitor::ApiTypedValueVisitor::new(),",
            )?;
        }
        self.printer.println("__out: None,")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;

        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // with_output() for standalone use
        self.printer.println(&format!(
            "impl<'a> {fields_name}<'a> {{"
        ))?;
        self.printer.indent();
        self.printer.println(&format!(
            "fn with_output(out: &'a mut Option<{struct_name}>) -> Self {{"
        ))?;
        self.printer.indent();
        self.printer.println(&format!("{fields_name} {{"))?;
        self.printer.indent();
        for (i, f) in deser_fields.iter().enumerate() {
            match &f.kind {
                DeserFieldKind::TraitObject { base_type_name, .. } => {
                    self.printer.println(&format!(
                        "f{i}: Some(VimObjectHolder {{ out: None, default_type_name: Some(\"{base_type_name}\") }}),"
                    ))?;
                }
                // _ if f.is_array => {
                //     self.printer.println(&format!("f{i}: Some(Vec::new()),"))?;
                // }
                _ => {
                    self.printer.println(&format!("f{i}: None,"))?;
                }
            }
        }
        if is_pruned {
            self.printer.println("type_: None,")?;
            self.printer.println("type_name: None,")?;
            self.printer.println("extra_fields_: std::collections::HashMap::new(),")?;
            self.printer.println("current_extra_key: None,")?;
            self.printer.println("current_extra_value: None,")?;
            self.printer.println("#[cfg(feature = \"xml\")]")?;
            self.printer.println("resolved_type: None,")?;
            self.printer.println("#[cfg(feature = \"xml\")]")?;
            self.printer.println(
                "api_extra_visitor: super::api_typed_visitor::ApiTypedValueVisitor::new(),",
            )?;
        }
        self.printer.println("__out: Some(out),")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // shift_extra() for pruned types
        if is_pruned {
            self.printer.println("fn shift_extra(&mut self) {")?;
            self.printer.indent();
            self.printer.println("#[cfg(feature = \"xml\")]")?;
            self.printer.println("{")?;
            self.printer.indent();
            self.printer.println("if self.resolved_type.is_none() {")?;
            self.printer.indent();
            self.printer.println("if let Some(tn) = self.type_name.as_deref() {")?;
            self.printer.indent();
            self.printer.println("self.resolved_type = struct_enum::StructType::from_str(tn);")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.println("#[cfg(feature = \"xml\")]")?;
            self.printer.println("let value = self.current_extra_value.take().or_else(|| self.api_extra_visitor.take_value());")?;
            self.printer.println("#[cfg(not(feature = \"xml\"))]")?;
            self.printer.println("let value = self.current_extra_value.take();")?;
            self.printer.println("if let (Some(k), Some(v)) = (self.current_extra_key.take(), value) {")?;
            self.printer.indent();
            self.printer.println("self.extra_fields_.insert(k, v);")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.newline()?;
        }

        // build() method
        self.printer.println(&format!(
            "fn build(&mut self) -> miniserde::Result<{struct_name}> {{"
        ))?;
        self.printer.indent();

        if is_pruned {
            self.printer.println("self.shift_extra();")?;
        }

        // Extract fields from Options, handle trait casts, binary unwrap
        for (i, f) in deser_fields.iter().enumerate() {
            match &f.kind {
                DeserFieldKind::Binary => {
                    if f.optional {
                        self.printer.println(&format!(
                            "let {} = self.f{i}.take().map(|b| b.0);",
                            f.field_name
                        ))?;
                    } else {
                        self.printer.println(&format!(
                            "let {} = self.f{i}.take().ok_or(miniserde::Error)?.0;",
                            f.field_name
                        ))?;
                    }
                }
                DeserFieldKind::TraitObject { trait_name, .. } => {
                    if f.optional {
                        self.printer.println(&format!(
                            "let {}: {} = if let Some(holder) = self.f{i}.take() {{",
                            f.field_name, f.field_type
                        ))?;
                        self.printer.indent();
                        self.printer.println("if let Some(vim_obj) = holder.out {")?;
                        self.printer.indent();
                        self.printer.println(&format!(
                            "Some(<dyn {trait_name}>::from_box(vim_obj).map_err(|_| miniserde::Error)?)"
                        ))?;
                        self.printer.dedent();
                        self.printer.println("} else { None }")?;
                        self.printer.dedent();
                        self.printer.println("} else { None };")?;
                    } else {
                        self.printer.println(&format!(
                            "let holder = self.f{i}.take().ok_or(miniserde::Error)?;"
                        ))?;
                        self.printer.println(&format!(
                            "let {}: {} = <dyn {trait_name}>::from_box(holder.out.ok_or(miniserde::Error)?).map_err(|_| miniserde::Error)?;",
                            f.field_name, f.field_type
                        ))?;
                    }
                }
                DeserFieldKind::Normal => {
                    if f.optional {
                        self.printer.println(&format!(
                            "let {} = self.f{i}.take();",
                            f.field_name
                        ))?;
                    } else {
                        self.printer.println(&format!(
                            "let {} = self.f{i}.take().ok_or(miniserde::Error)?;",
                            f.field_name
                        ))?;
                    }
                }
            }
        }

        // Handle pruned type_ resolution
        if is_pruned {
            self.printer.println("let type_ = self.type_.take().or_else(|| {")?;
            self.printer.indent();
            self.printer.println("self.type_name.as_deref().and_then(|tn| {")?;
            self.printer.indent();
            self.printer.println("let st = struct_enum::StructType::from_str(tn);")?;
            self.printer.println(&format!(
                "if st == Some(struct_enum::StructType::{struct_name}) {{ None }} else {{ st }}"
            ))?;
            self.printer.dedent();
            self.printer.println("})")?;
            self.printer.dedent();
            self.printer.println("});")?;
        }

        // Build nested struct construction (same logic as before)
        let mut field_idx = 0;

        for (idx, struct_type_ref) in inheritance_chain.iter().enumerate() {
            let struct_ref = (**struct_type_ref).borrow();
            let current_struct_name = to_type_name(&struct_ref.name);
            let is_last = idx == inheritance_chain.len() - 1;

            if !is_last && !self.vim_model.has_any_fields_in_chain(&struct_ref.name)? {
                continue;
            }

            if is_last {
                self.printer
                    .println(&format!("Ok({current_struct_name} {{"))?;
            } else {
                self.printer.println(&format!(
                    "let {}_temp = {current_struct_name} {{",
                    to_field_name(&struct_ref.name)
                ))?;
            }
            self.printer.indent();

            // Parent field reference
            if let Some(parent_name) = &struct_ref.parent {
                if parent_name != "Any" && !is_last {
                    let mut ancestor_name = parent_name.clone();
                    loop {
                        if self.vim_model.has_any_fields_in_chain(&ancestor_name)? {
                            let parent_field = parent_field_name(&ancestor_name);
                            let parent_temp_var = to_field_name(&ancestor_name);
                            self.printer
                                .println(&format!("{parent_field}: {parent_temp_var}_temp,"))?;
                            break;
                        }
                        if let Some(next_parent) = &self.vim_model.structs.get(&ancestor_name) {
                            if let Some(next) = &next_parent.borrow().parent {
                                if next == "Any" { break; }
                                ancestor_name = next.clone();
                            } else { break; }
                        } else { break; }
                    }
                }
            }

            // Assign fields
            for (_, _field) in &struct_ref.fields {
                let f = &deser_fields[field_idx];
                self.printer
                    .println(&format!("{}: {},", f.field_name, f.field_name))?;
                field_idx += 1;
            }

            if is_last {
                // Add parent field
                if let Some(parent_name) = &vim_type.parent {
                    if parent_name != "Any" {
                        let mut ancestor_name = parent_name.clone();
                        loop {
                            if self.vim_model.has_any_fields_in_chain(&ancestor_name)? {
                                let parent_field = parent_field_name(&ancestor_name);
                                let parent_temp_var = to_field_name(&ancestor_name);
                                self.printer
                                    .println(&format!("{parent_field}: {parent_temp_var}_temp,"))?;
                                break;
                            }
                            if let Some(next_parent) = &self.vim_model.structs.get(&ancestor_name) {
                                if let Some(next) = &next_parent.borrow().parent {
                                    if next == "Any" { break; }
                                    ancestor_name = next.clone();
                                } else { break; }
                            } else { break; }
                        }
                    }
                }
                if is_pruned {
                    self.printer.println("type_,")?;
                    self.printer.println("extra_fields_: std::mem::take(&mut self.extra_fields_),")?;
                }
                self.printer.dedent();
                self.printer.println("})")?;
            } else {
                self.printer.dedent();
                self.printer.println("};")?;
                self.printer.newline()?;
            }
        }

        self.printer.dedent();
        self.printer.println("}")?; // end build()

        self.printer.dedent();
        self.printer.println("}")?; // end impl<'a> Fields<'a>
        self.printer.newline()?;

        // 5. impl Map for Fields
        self.printer.println(&format!(
            "impl miniserde::de::Map for {fields_name}<'_> {{"
        ))?;
        self.printer.indent();

        // key() method
        self.printer.println(
            "fn key(&mut self, key: &str) -> miniserde::Result<&mut dyn miniserde::de::Visitor> {"
        )?;
        self.printer.indent();
        if is_pruned {
            self.printer.println("self.shift_extra();")?;
        }
        self.printer.println("match key {")?;
        self.printer.indent();

        if is_pruned {
            self.printer.println(
                "\"_typeName\" => Ok(miniserde::Deserialize::begin(&mut self.type_name)),"
            )?;
        } else {
            self.printer.println(
                "\"_typeName\" => Ok(<dyn miniserde::de::Visitor>::ignore()),"
            )?;
        }

        let is_mor = vim_type.name == "ManagedObjectReference";
        for (i, f) in deser_fields.iter().enumerate() {
            let ser_name = &f.ser_name;
            let key_pattern = if is_mor {
                match ser_name.as_str() {
                    "type" => r##""type" | "@type""##.to_string(),
                    "value" => r##""value" | "#text""##.to_string(),
                    _ => format!("\"{ser_name}\""),
                }
            } else {
                format!("\"{ser_name}\"")
            };
            match &f.kind {
                DeserFieldKind::Binary | DeserFieldKind::TraitObject { .. } | DeserFieldKind::Normal => {
                    self.printer.println(&format!(
                        "{key_pattern} => Ok(miniserde::Deserialize::begin(&mut self.f{i})),"
                    ))?;
                }
            }
        }

        if is_pruned {
            self.printer.println("_ => {")?;
            self.printer.indent();
            self.printer.println("self.current_extra_key = Some(key.to_owned());")?;
            self.printer.println("#[cfg(feature = \"xml\")]")?;
            self.printer.println("{")?;
            self.printer.indent();
            self.printer.println("let st = self.resolved_type.or(self.type_);")?;
            self.printer.println("if let Some(st) = st {")?;
            self.printer.indent();
            self.printer.println("if let Some(ft) = super::api_field_registry::lookup_api_field(st, key) {")?;
            self.printer.indent();
            self.printer.println("self.api_extra_visitor.reset(ft);")?;
            self.printer.println("return Ok(&mut self.api_extra_visitor);")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.dedent();
            self.printer.println("}")?;
            self.printer.println("Ok(miniserde::Deserialize::begin(&mut self.current_extra_value))")?;
            self.printer.dedent();
            self.printer.println("}")?;
        } else {
            self.printer.println(
                "_ => Ok(<dyn miniserde::de::Visitor>::ignore()),"
            )?;
        }

        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;

        // finish() method
        self.printer.println(
            "fn finish(&mut self) -> miniserde::Result<()> {"
        )?;
        self.printer.indent();
        self.printer.println("let result = self.build()?;")?;
        self.printer.println("if let Some(out) = self.__out.take() {")?;
        self.printer.indent();
        self.printer.println("*out = Some(result);")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.println("Ok(())")?;
        self.printer.dedent();
        self.printer.println("}")?;

        self.printer.dedent();
        self.printer.println("}")?; // end impl Map
        self.printer.newline()?;

        // 6. impl FieldsBuilder for Fields<'static>
        self.printer.println(&format!(
            "impl FieldsBuilder for {fields_name}<'static> {{"
        ))?;
        self.printer.indent();
        self.printer.println(
            "fn build_boxed(&mut self) -> miniserde::Result<Box<dyn super::vim_object_trait::VimObjectTrait>> {"
        )?;
        self.printer.indent();
        self.printer.println("Ok(Box::new(self.build()?))")?;
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
