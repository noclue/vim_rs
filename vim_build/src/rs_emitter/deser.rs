use indexmap::IndexMap;

use crate::vim_model::{BoxType, DataType, EmitMode, Model};
use crate::printer::Printer;

use super::errors::Result;
use super::{to_type_name, TypeDefResolver};

pub struct DeserializationGenerator<'a> {
    vim_model: &'a Model,
    printer: &'a mut dyn Printer,
    any_value_types: IndexMap<String, &'a BoxType>,
    tdf: TypeDefResolver<'a>,
}

impl DeserializationGenerator<'_> {
    pub fn new<'a>(
        vim_model: &'a Model,
        printer: &'a mut dyn Printer,
    ) -> DeserializationGenerator<'a> {
        let mut value_types: IndexMap<String, &BoxType> = IndexMap::new();
        for (name, box_type) in &vim_model.any_value_types {
            if name == "Any" {
                continue;
            }
            let key = box_type
                .discriminator_value
                .as_ref()
                .unwrap_or(name)
                .clone();
            value_types.insert(key, box_type);
        }
        DeserializationGenerator {
            vim_model,
            printer,
            any_value_types: value_types,
            tdf: TypeDefResolver::new(vim_model),
        }
    }

    pub fn generate_deserialization(&mut self) -> Result<()> {
        self.emit_imports()?;
        self.emit_make_place()?;
        self.emit_wrap_value_impls()?;
        self.emit_polymorphic_array_cast_fns()?;
        self.emit_vim_object_holder_impls()?;
        self.emit_type_registry()?;
        self.emit_lookup_type()?;
        self.emit_vim_object_holder_builder_map()?;
        self.emit_value_elements_deser()?;
        self.emit_vim_any_deser()?;
        Ok(())
    }

    fn emit_imports(&mut self) -> Result<()> {
        self.printer.println("use super::mini_de_static::{")?;
        self.printer.indent();
        self.printer.println("TypeInfo, DelegatingDeserializer, WrapValue,")?;
        self.printer.println("VimObjectHolder, VimObjectHolderBuilder, VimAnyBuilder,")?;
        self.printer.println("make_deser, from_val, polymorphic_array_cast,")?;
        self.printer.dedent();
        self.printer.println("};")?;
        self.printer.println("use super::mini_helpers::from_value;")?;
        self.printer.println("use super::vim_any::VimAny;")?;
        self.printer.println("use super::boxed_types::ValueElements;")?;
        self.printer.println("use super::struct_enum::StructType;")?;
        self.printer.println("use super::structs::*;")?;
        self.printer.newline()?;
        Ok(())
    }

    fn emit_make_place(&mut self) -> Result<()> {
        self.printer.println("miniserde::make_place!(Place);")?;
        self.printer.newline()?;
        Ok(())
    }

    /// Emit WrapValue implementations for primitive/array types used in ValueElements.
    /// Deduplicate by Rust type to avoid conflicting trait impls.
    fn emit_wrap_value_impls(&mut self) -> Result<()> {
        self.printer
            .println("// WrapValue implementations for ValueElements types")?;

        let mut seen_types = std::collections::HashSet::new();

        for (_discriminator, box_type) in &self.any_value_types {
            let enum_variant = to_type_name(&box_type.name);
            let rust_type = self.tdf.to_rust_field_type(&box_type.property_type)?;

            // Determine if this is a polymorphic array (trait object array).
            // Those need special handling via cast functions, not WrapValue.
            if self.is_polymorphic_array_type(&box_type.property_type) {
                continue; // Handled by emit_polymorphic_array_cast_fns
            }

            // Skip duplicate Rust types (e.g. multiple array discriminators mapping to Vec<MethodFault>)
            if !seen_types.insert(rust_type.clone()) {
                continue;
            }

            self.printer.println(&format!(
                "impl WrapValue for {rust_type} {{"
            ))?;
            self.printer.indent();
            self.printer.println(&format!(
                "fn wrap(self) -> ValueElements {{ ValueElements::{enum_variant}(self) }}"
            ))?;
            self.printer.dedent();
            self.printer.println("}")?;
        }
        self.printer.newline()?;
        Ok(())
    }

    /// Check if a DataType is a polymorphic array (array of trait objects)
    fn is_polymorphic_array_type(&self, dt: &DataType) -> bool {
        if let DataType::Array(inner) = dt {
            if let DataType::Reference(ref_name) = inner.as_ref() {
                // "Any" maps to VimAny, not a trait object array
                if ref_name == "Any" {
                    return false;
                }
                if let Some(s) = self.vim_model.structs.get(ref_name.as_str()) {
                    let s_ref = s.borrow();
                    return s_ref.has_children() && s_ref.emit_mode == EmitMode::Emit;
                }
            }
        }
        false
    }

    /// Get the trait name for a polymorphic struct reference
    fn get_trait_name(&self, ref_name: &str) -> Option<String> {
        // "Any" maps to VimAny, not a trait - skip it
        if ref_name == "Any" {
            return None;
        }
        if let Some(s) = self.vim_model.structs.get(ref_name) {
            let s_ref = s.borrow();
            if s_ref.has_children() && s_ref.emit_mode == EmitMode::Emit {
                return Some(format!("super::traits::{}Trait", to_type_name(ref_name)));
            }
        }
        None
    }

    /// Emit cast functions for polymorphic array types
    fn emit_polymorphic_array_cast_fns(&mut self) -> Result<()> {
        self.printer
            .println("// Polymorphic array cast functions")?;

        for (_discriminator, box_type) in &self.any_value_types {
            if !self.is_polymorphic_array_type(&box_type.property_type) {
                continue;
            }
            let enum_variant = to_type_name(&box_type.name);
            if let DataType::Array(inner) = &box_type.property_type {
                if let DataType::Reference(ref_name) = inner.as_ref() {
                    if let Some(trait_name) = self.get_trait_name(ref_name) {
                        let fn_name = format!("cast_to_{}_array", super::to_field_name(ref_name));
                        self.printer.println(&format!(
                            "fn {fn_name}(h: Vec<VimObjectHolder>) -> miniserde::Result<ValueElements> {{"
                        ))?;
                        self.printer.indent();
                        self.printer.println(&format!(
                            "polymorphic_array_cast::<dyn {trait_name}>(h, ValueElements::{enum_variant})"
                        ))?;
                        self.printer.dedent();
                        self.printer.println("}")?;
                    }
                }
            }
        }
        self.printer.newline()?;
        Ok(())
    }

    fn emit_vim_object_holder_impls(&mut self) -> Result<()> {
        self.printer
            .println("// VimObjectHolder Deserialize/Visitor implementations")?;
        self.printer
            .println("impl miniserde::Deserialize for VimObjectHolder {")?;
        self.printer.indent();
        self.printer.println(
            "fn begin(out: &mut Option<VimObjectHolder>) -> &mut dyn miniserde::de::Visitor {",
        )?;
        self.printer.indent();
        self.printer.println("Place::new(out)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        self.printer
            .println("impl miniserde::de::Visitor for Place<VimObjectHolder> {")?;
        self.printer.indent();
        self.printer.println(
            "fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {",
        )?;
        self.printer.indent();
        self.printer
            .println("Ok(Box::new(VimObjectHolderBuilder::new(&mut self.out)))")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        Ok(())
    }

    /// Build the PHF TYPE_REGISTRY using phf_codegen::Map
    fn emit_type_registry(&mut self) -> Result<()> {
        self.printer
            .println("// PHF Type Registry for O(1) type name lookup")?;

        let mut map_builder = phf_codegen::Map::new();

        // 1. Object types (all structs)
        for (name, struct_cell) in &self.vim_model.structs {
            if name == "Any" {
                continue;
            }
            let struct_ref = struct_cell.borrow();
            let struct_name = to_type_name(name);

            match &struct_ref.emit_mode {
                EmitMode::Emit | EmitMode::Prune => {
                    // Normal struct or pruned type -> direct builder
                    let builder_expr = if struct_ref.emit_mode == EmitMode::Prune {
                        format!(
                            "TypeInfo::Object {{ name: \"{}\", builder_fn: || Box::new({}Fields::new(None)) }}",
                            struct_ref.discriminator(),
                            struct_name
                        )
                    } else {
                        format!(
                            "TypeInfo::Object {{ name: \"{}\", builder_fn: || Box::new({}Fields::new()) }}",
                            struct_ref.discriminator(),
                            struct_name
                        )
                    };
                    map_builder.entry(struct_ref.discriminator(), &builder_expr);
                }
                EmitMode::Skip(parent_type) => {
                    // Skipped struct (pruned descendant) -> use parent's Fields with type_ preset
                    let parent_name = to_type_name(parent_type);
                    let builder_expr = format!(
                        "TypeInfo::Object {{ name: \"{}\", builder_fn: || Box::new({}Fields::new(Some(StructType::{}))) }}",
                        struct_ref.discriminator(),
                        parent_name,
                        struct_name
                    );
                    map_builder.entry(struct_ref.discriminator(), &builder_expr);
                }
            }
        }

        // 2. Value types (primitives and arrays)
        for (discriminator, box_type) in &self.any_value_types {
            if self.is_polymorphic_array_type(&box_type.property_type) {
                // Polymorphic array type - use cast functions
                if let DataType::Array(inner) = &box_type.property_type {
                    if let DataType::Reference(ref_name) = inner.as_ref() {
                        let fn_name =
                            format!("cast_to_{}_array", super::to_field_name(ref_name));
                        let value_expr = format!(
                            "TypeInfo::Value {{ name: \"{discriminator}\", make_deserializer: || Box::new(DelegatingDeserializer::<Vec<VimObjectHolder>>::new({fn_name})), from_value: |v| {fn_name}(from_value(v)?) }}"
                        );
                        map_builder.entry(discriminator.clone(), &value_expr);
                    }
                }
            } else {
                // Simple value type - use make_deser/from_val
                let rust_type = self.tdf.to_rust_field_type(&box_type.property_type)?;
                let value_expr = format!(
                    "TypeInfo::Value {{ name: \"{discriminator}\", make_deserializer: make_deser::<{rust_type}>, from_value: from_val::<{rust_type}> }}"
                );
                map_builder.entry(discriminator.clone(), &value_expr);
            }
        }

        self.printer.println(&format!(
            "static TYPE_REGISTRY: phf::Map<&'static str, TypeInfo> = {};",
            map_builder.build()
        ))?;
        self.printer.newline()?;

        Ok(())
    }

    fn emit_lookup_type(&mut self) -> Result<()> {
        self.printer.println("#[inline]")?;
        self.printer.println(
            "pub fn lookup_type(type_name: &str) -> Option<&'static TypeInfo> {",
        )?;
        self.printer.indent();
        self.printer.println("TYPE_REGISTRY.get(type_name)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        Ok(())
    }

    fn emit_vim_object_holder_builder_map(&mut self) -> Result<()> {
        self.printer
            .println("impl miniserde::de::Map for VimObjectHolderBuilder<'_> {")?;
        self.printer.indent();
        self.printer.println(
            "fn key(&mut self, key: &str) -> miniserde::Result<&mut dyn miniserde::de::Visitor> {",
        )?;
        self.printer.indent();
        self.printer.println("self.core.key(key, lookup_type)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        self.printer
            .println("fn finish(&mut self) -> miniserde::Result<()> {")?;
        self.printer.indent();
        self.printer
            .println("match self.core.finish(lookup_type)? {")?;
        self.printer.indent();
        self.printer.println("VimAny::Object(obj) => {")?;
        self.printer.indent();
        self.printer
            .println("*self.__out = Some(VimObjectHolder { out: Some(obj), default_type_name: self.core.default_type_name });")?;
        self.printer.println("Ok(())")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer
            .println("VimAny::Value(_) => Err(miniserde::Error),")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        Ok(())
    }

    fn emit_value_elements_deser(&mut self) -> Result<()> {
        self.printer
            .println("// ValueElements deserialization (wrapper format)")?;

        // Deserialize impl
        self.printer
            .println("impl miniserde::Deserialize for ValueElements {")?;
        self.printer.indent();
        self.printer.println(
            "fn begin(out: &mut Option<ValueElements>) -> &mut dyn miniserde::de::Visitor {",
        )?;
        self.printer.indent();
        self.printer.println("Place::new(out)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // Visitor impl
        self.printer
            .println("impl miniserde::de::Visitor for Place<ValueElements> {")?;
        self.printer.indent();
        self.printer.println(
            "fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {",
        )?;
        self.printer.indent();
        self.printer
            .println("Ok(Box::new(ValueElementsFields::new(&mut self.out)))")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // ValueElementsFields struct
        self.printer
            .println("struct ValueElementsFields<'a> {")?;
        self.printer.indent();
        self.printer.println("type_name: Option<String>,")?;
        self.printer
            .println("value: Option<miniserde::json::Value>,")?;
        self.printer
            .println("__out: &'a mut Option<ValueElements>,")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        self.printer
            .println("impl<'a> ValueElementsFields<'a> {")?;
        self.printer.indent();
        self.printer
            .println("fn new(out: &'a mut Option<ValueElements>) -> Self {")?;
        self.printer.indent();
        self.printer
            .println("Self { type_name: None, value: None, __out: out }")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // Map impl
        self.printer
            .println("impl miniserde::de::Map for ValueElementsFields<'_> {")?;
        self.printer.indent();
        self.printer.println(
            "fn key(&mut self, key: &str) -> miniserde::Result<&mut dyn miniserde::de::Visitor> {",
        )?;
        self.printer.indent();
        self.printer.println("match key {")?;
        self.printer.indent();
        self.printer.println(
            "\"_typeName\" => Ok(miniserde::Deserialize::begin(&mut self.type_name)),"
        )?;
        self.printer.println(
            "\"_value\" => Ok(miniserde::Deserialize::begin(&mut self.value)),"
        )?;
        self.printer
            .println("_ => Ok(<dyn miniserde::de::Visitor>::ignore()),")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        self.printer
            .println("fn finish(&mut self) -> miniserde::Result<()> {")?;
        self.printer.indent();
        self.printer.println(
            "let type_name = self.type_name.take().ok_or(miniserde::Error)?;",
        )?;
        self.printer
            .println("let value = self.value.take().ok_or(miniserde::Error)?;")?;
        self.printer.println(
            "let type_info = lookup_type(&type_name).filter(|ti| !ti.is_object()).ok_or(miniserde::Error)?;",
        )?;
        self.printer
            .println("let result = type_info.deserialize_from_value(&value)?;")?;
        self.printer.println("*self.__out = Some(result);")?;
        self.printer.println("Ok(())")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        Ok(())
    }

    fn emit_vim_any_deser(&mut self) -> Result<()> {
        self.printer.println("// VimAny deserialization")?;

        // Deserialize
        self.printer
            .println("impl miniserde::Deserialize for VimAny {")?;
        self.printer.indent();
        self.printer.println(
            "fn begin(out: &mut Option<VimAny>) -> &mut dyn miniserde::de::Visitor {",
        )?;
        self.printer.indent();
        self.printer.println("Place::new(out)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // Visitor
        self.printer
            .println("impl miniserde::de::Visitor for Place<VimAny> {")?;
        self.printer.indent();
        self.printer.println(
            "fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {",
        )?;
        self.printer.indent();
        self.printer
            .println("Ok(Box::new(VimAnyBuilder::new(&mut self.out)))")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // VimAnyBuilder Map impl
        self.printer
            .println("impl miniserde::de::Map for VimAnyBuilder<'_> {")?;
        self.printer.indent();
        self.printer.println(
            "fn key(&mut self, key: &str) -> miniserde::Result<&mut dyn miniserde::de::Visitor> {",
        )?;
        self.printer.indent();
        self.printer.println("self.core.key(key, lookup_type)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        self.printer
            .println("fn finish(&mut self) -> miniserde::Result<()> {")?;
        self.printer.indent();
        self.printer
            .println("let result = self.core.finish(lookup_type)?;")?;
        self.printer.println("*self.__out = Some(result);")?;
        self.printer.println("Ok(())")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        Ok(())
    }
}
