use crate::printer::Printer;
use crate::rs_emitter::common::emit_description_with_paths;
use crate::rs_emitter::errors::{Error, Result};
use crate::rs_emitter::{
    parent_field_name, to_field_name, to_type_name,
};
use crate::vim_model::{EmitMode, Model, Struct};
use std::ops::Deref;
pub struct TraitEmitter<'a> {
    type_name: String,
    model: &'a Model,
    printer: &'a mut dyn Printer,
}

impl<'a> TraitEmitter<'a> {
    pub fn new(type_name: String, model: &'a Model, printer: &'a mut dyn Printer) -> Self {
        Self {
            type_name,
            model,
            printer,
        }
    }

    pub fn emit_trait(&mut self) -> super::Result<()> {
        let vim_type = self
            .model
            .structs
            .get(&self.type_name)
            .ok_or_else(|| Error::TypeNotFound(self.type_name.clone()))?
            .borrow();
        let type_ref = vim_type.deref();

        if !type_ref.has_children() {
            return Err(Error::InternalError(format!(
                "Attempt to generate trait for object with no descendants: {}",
                self.type_name
            )));
        };

        self.emit_trait_type(type_ref)?;
        // No need to emit_serialize() -- miniserde::Serialize is inherited via
        // VimObjectTrait supertrait, so `dyn XxxTrait` auto-implements Serialize.
        self.emit_trait_deserialization()?;
        self.generate_implementations(type_ref)?;
        self.generate_cast_from_trait()?;

        Ok(())
    }

    pub fn emit_imports(printer: &mut dyn Printer) -> super::Result<()> {
        printer.println("use super::vim_object_trait::VimObjectTrait;")?;
        printer.println("use super::convert::CastFrom;")?;
        printer.println("use super::struct_enum::StructType;")?;
        printer.println("use super::structs::*;")?;
        printer.println("use super::mini_de_static::{VimObjectHolder, VimObjectHolderBuilder};")?;
        printer.newline()?;
        printer.println("miniserde::make_place!(Place);")?;

        printer.println("")?;
        Ok(())
    }

    // To allow for polymorphic fields every structure type that has descendants will have a trait
    // alternative that will be passed as dynamic reference. This trait will be implemented for
    // all of the structure type descendants. The trait will provide access to the struct type fields
    // and will extend the VimObjectTrait as to allow up and down casts.
    fn emit_trait_type(&mut self, vim_type: &Struct) -> Result<()> {
        if crate::rs_emitter::structs::ANY == self.type_name {
            return Ok(());
        } // Skip the Any type
        let struct_name = to_type_name(&self.type_name);
        let Some(ref parent_trait) = vim_type.parent else {
            return Ok(()); // or error?
        };
        let base_trait = to_type_name(if crate::rs_emitter::structs::ANY == parent_trait {
            "VimObject"
        } else {
            parent_trait
        });
        {
            let this = &mut *self;
            let doc_string: &Option<String> = &vim_type.description;
            emit_description_with_paths(this.printer, doc_string, &vim_type.paths)
        }?;
        self.printer.println(&format!(
            "pub trait {}Trait : super::traits::{}Trait {{",
            struct_name, base_trait
        ))?;
        self.printer.indent();

        // Only add accessor methods if type has any fields in its chain
        let has_fields = self.model.has_any_fields_in_chain(&self.type_name)?;
        if has_fields {
            // Add parent struct accessor methods
            self.printer.println(&format!(
                "/// Get a reference to the {} parent struct",
                struct_name
            ))?;
            self.printer.println(&format!(
                "fn get_{}(&self) -> &super::structs::{};",
                to_field_name(&self.type_name),
                struct_name
            ))?;
            self.printer.println(&format!(
                "/// Get a mutable reference to the {} parent struct",
                struct_name
            ))?;
            self.printer.println(&format!(
                "fn get_{}_mut(&mut self) -> &mut super::structs::{};",
                to_field_name(&self.type_name),
                struct_name
            ))?;
        }
        self.printer.dedent();
        self.printer.println("}")?;

        // Add Deref implementations for the trait (only if has fields)
        if has_fields {
            self.emit_trait_deref_implementations(vim_type)?;
        }

        Ok(())
    }

    fn emit_trait_deref_implementations(&mut self, _vim_type: &Struct) -> Result<()> {
        let struct_name = to_type_name(&self.type_name);
        let getter_method = format!("get_{}", to_field_name(&self.type_name));
        let getter_mut_method = format!("get_{}_mut", to_field_name(&self.type_name));

        // Emit Deref for trait object
        self.printer.println(&format!(
            "impl std::ops::Deref for dyn {}Trait {{",
            struct_name
        ))?;
        self.printer.indent();
        self.printer
            .println(&format!("type Target = super::structs::{};", struct_name))?;
        self.printer.newline()?;
        self.printer.println("fn deref(&self) -> &Self::Target {")?;
        self.printer.indent();
        self.printer.println(&format!("self.{}()", getter_method))?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // Emit DerefMut for trait object
        self.printer.println(&format!(
            "impl std::ops::DerefMut for dyn {}Trait {{",
            struct_name
        ))?;
        self.printer.indent();
        self.printer
            .println("fn deref_mut(&mut self) -> &mut Self::Target {")?;
        self.printer.indent();
        self.printer
            .println(&format!("self.{}()", getter_mut_method))?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        Ok(())
    }

    fn emit_serialize(&mut self) -> Result<()> {
        let struct_name = to_type_name(&self.type_name);
        self.printer.println(&format!(
            "impl miniserde::Serialize for dyn {struct_name}Trait + '_ {{"
        ))?;
        self.printer.indent();
        self.printer.println("fn begin(&self) -> miniserde::ser::Fragment<'_> {")?;
        self.printer.indent();
        self.printer.println("self.as_vim_object_ref().begin()")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_trait_deserialization(&mut self) -> Result<()> {
        let struct_name = to_type_name(&self.type_name);

        // impl Deserialize for Box<dyn XxxTrait>
        self.printer.println(&format!(
            "impl miniserde::Deserialize for Box<dyn {struct_name}Trait> {{"
        ))?;
        self.printer.indent();
        self.printer.println(
            "fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {"
        )?;
        self.printer.indent();
        self.printer.println("Place::new(out)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // impl Visitor for Place<Box<dyn XxxTrait>>
        self.printer.println(&format!(
            "impl miniserde::de::Visitor for Place<Box<dyn {struct_name}Trait>> {{"
        ))?;
        self.printer.indent();
        self.printer.println(
            "fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {"
        )?;
        self.printer.indent();
        self.printer.println(&format!(
            "Ok(Box::new({struct_name}TraitBoxBuilder {{"
        ))?;
        self.printer.indent();
        self.printer.println("core: super::mini_de_static::PolyCore::new(),")?;
        self.printer.println("__out: &mut self.out,")?;
        self.printer.dedent();
        self.printer.println("}))")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // TraitBoxBuilder struct -- uses PolyCore directly to avoid self-referential issues
        self.printer.println(&format!(
            "struct {struct_name}TraitBoxBuilder<'a> {{"
        ))?;
        self.printer.indent();
        self.printer.println("core: super::mini_de_static::PolyCore,")?;
        self.printer.println(&format!(
            "__out: &'a mut Option<Box<dyn {struct_name}Trait>>,"
        ))?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // Map impl for TraitBoxBuilder
        self.printer.println(&format!(
            "impl miniserde::de::Map for {struct_name}TraitBoxBuilder<'_> {{"
        ))?;
        self.printer.indent();
        self.printer.println(
            "fn key(&mut self, key: &str) -> miniserde::Result<&mut dyn miniserde::de::Visitor> {",
        )?;
        self.printer.indent();
        self.printer
            .println("self.core.key(key, super::deserialize::lookup_type)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        self.printer
            .println("fn finish(&mut self) -> miniserde::Result<()> {")?;
        self.printer.indent();
        self.printer.println(
            "match self.core.finish(super::deserialize::lookup_type)? {"
        )?;
        self.printer.indent();
        self.printer.println(&format!(
            "super::vim_any::VimAny::Object(obj) => {{"
        ))?;
        self.printer.indent();
        self.printer.println(&format!(
            "*self.__out = Some(<dyn {struct_name}Trait>::from_box(obj).map_err(|_| miniserde::Error)?);"
        ))?;
        self.printer.println("Ok(())")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer
            .println("_ => Err(miniserde::Error),")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        Ok(())
    }

    fn generate_implementations(&mut self, trait_type: &Struct) -> Result<()> {
        for child_type in self.model.children(&self.type_name)? {
            if child_type.borrow().emit_mode.is_skip() {
                continue;
            }
            let struct_name = &child_type.borrow().name;
            self.emit_trait_implementation(trait_type, struct_name)?
        }
        Ok(())
    }

    /// Emits implementation of a structure type trait for a given structure. The trait should belong to
    /// the same structure or an ancestor
    fn emit_trait_implementation(&mut self, trait_type: &Struct, type_name: &str) -> Result<()> {
        let base_name = to_type_name(&trait_type.name);
        let struct_name = &to_type_name(type_name);
        self.printer
            .println(&format!("impl {}Trait for {} {{", base_name, struct_name))?;
        self.printer.indent();

        // Only implement accessor methods if trait type has fields
        let has_fields = self.model.has_any_fields_in_chain(&trait_type.name)?;
        if has_fields {
            // Implement parent accessor methods
            let trait_struct_name = &trait_type.name;
            let getter_method = format!("get_{}", to_field_name(trait_struct_name));
            let getter_mut_method = format!("get_{}_mut", to_field_name(trait_struct_name));

            // Build the access path from type_name to trait_type
            let access_path = self.build_parent_access_path(type_name, trait_struct_name)?;

            self.printer.println(&format!(
                "fn {}(&self) -> &super::structs::{} {{ {} }}",
                getter_method, base_name, access_path
            ))?;

            // For mutable getter, we need to return without the leading & if it's self
            let mut_access = if access_path == "&self" {
                "self".to_string()
            } else {
                access_path.replace("&self", "&mut self")
            };
            self.printer.println(&format!(
                "fn {}(&mut self) -> &mut super::structs::{} {{ {} }}",
                getter_mut_method, base_name, mut_access
            ))?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    /// Build access path from child type to parent type through composition chain
    /// Skip empty parents (marker traits) that have no fields
    fn build_parent_access_path(&self, from_type: &str, to_type: &str) -> Result<String> {
        if from_type == to_type {
            return Ok("&self".to_string());
        }

        // Navigate parent chain from from_type until we find to_type, skipping empty parents
        let mut path = String::from("&self");
        let mut current_type_name = from_type.to_string();

        loop {
            let current_struct = self
                .model
                .structs
                .get(&current_type_name)
                .ok_or_else(|| Error::TypeNotFound(current_type_name.clone()))?
                .borrow();

            if let Some(parent) = current_struct.parent.as_ref() {
                if parent == "Any" {
                    break;
                }

                // Only add parent field to path if parent has fields (not a marker trait)
                if self.model.has_any_fields_in_chain(parent)? {
                    let parent_field = parent_field_name(parent);
                    path.push_str(&format!(".{}", parent_field));
                }

                if parent == to_type {
                    return Ok(path);
                }

                // Continue up the chain
                current_type_name = parent.clone();
            } else {
                break;
            }
        }

        // Could not find parent in chain - might be accessing via Deref
        Ok("&self".to_string())
    }

    fn generate_cast_from_trait(&mut self) -> Result<()> {
        if self.model.structs.get(&self.type_name).is_none() {
            return Err(Error::TypeNotFound(self.type_name.clone()));
        }
        self.printer.println(&format!(
            "impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn {}Trait {{",
            to_type_name(&self.type_name)
        ))?;
        self.printer.indent();
        self.printer
            .println("fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {")?;
        self.printer.indent();
        self.printer.println("let data_type = from.data_type();")?;
        self.printer.println("match data_type {")?;
        self.printer.indent();
        // Generate match arms for each data type. Arms start with the type itself and end with last_child.
        for child_struct in self.model.children(&self.type_name)? {
            if matches!(child_struct.borrow().emit_mode, EmitMode::Skip(_)) {
                continue;
            }
            let type_name = child_struct.borrow().rust_name();
            self.printer.println(&format!("StructType::{type_name} => Some(from.as_any_ref().downcast_ref::<{type_name}>()?),"))?;
        }
        self.printer.println("_ => None,")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.println("")?;
        self.printer.println(
            "fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {",
        )?;
        self.printer.indent();
        self.printer.println("let data_type = from.data_type();")?;
        self.printer.println("match data_type {")?;
        self.printer.indent();
        for child_struct in self.model.children(&self.type_name)? {
            if matches!(child_struct.borrow().emit_mode, EmitMode::Skip(_)) {
                continue;
            }
            let type_name = child_struct.borrow().rust_name();
            self.printer.println(&format!(
                "StructType::{type_name} => Ok(from.as_any_box().downcast::<{type_name}>()?),"
            ))?;
        }
        self.printer.println("_ => Err(from.as_any_box()),")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;

        Ok(())
    }
}
