use crate::printer::Printer;
use crate::rs_emitter::common::emit_description;
use crate::rs_emitter::errors::{Error, Result};
use crate::rs_emitter::{get_by_ref, getter_name, to_field_name, to_type_name, TypeDefResolver};
use crate::vim_model::{EmitMode, Field, Model, Struct};
use std::ops::Deref;
pub struct TraitEmitter<'a> {
    type_name: String,
    model: &'a Model,
    printer: &'a mut dyn Printer,
    tdf: TypeDefResolver<'a>,
}

impl<'a> TraitEmitter<'a> {
    pub fn new(type_name: String, model: &'a Model, printer: &'a mut dyn Printer) -> Self {
        Self {
            type_name,
            model,
            printer,
            tdf: TypeDefResolver::new(model),
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
        self.emit_serialize()?;
        self.emit_trait_deserialization()?;
        self.generate_implementations(type_ref)?;
        self.generate_cast_from_trait()?;

        Ok(())
    }

    pub fn emit_imports(printer: &mut dyn Printer) -> super::Result<()> {
        printer.println("use super::vim_object_trait::VimObjectTrait;")?;
        printer.println("use super::dyn_serialize;")?;
        printer.println("use super::convert::CastFrom;")?;
        printer.println("use super::struct_enum::StructType;")?;
        printer.println("use super::structs::*;")?;
        printer.println("use serde::de;")?;
        printer.println("use super::vim_any::VimAny;")?;

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
            emit_description(this.printer, doc_string)
        }?;
        self.printer.println(&format!(
            "pub trait {}Trait : super::traits::{}Trait {{",
            struct_name, base_trait
        ))?;
        self.printer.indent();
        for (prop_name, property) in &vim_type.fields {
            self.emit_trait_field(prop_name, property)?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_serialize(&mut self) -> Result<()> {
        let struct_name = to_type_name(&self.type_name);
        self.printer.println(&format!(
            r#"impl<'s> serde::Serialize for dyn {struct_name}Trait + 's {{
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {{
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }}
        }}"#
        ))?;
        Ok(())
    }

    fn emit_trait_deserialization(&mut self) -> Result<()> {
        let struct_name = to_type_name(&self.type_name);
        self.printer.println(&format!(r#"impl<'de> serde::Deserialize<'de> for Box<dyn {struct_name}Trait> {{
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
                deserializer.deserialize_map({struct_name}Visitor)
            }}
        }}"#))?;
        self.printer.newline()?;
        self.printer
            .println(&format!("struct {struct_name}Visitor;"))?;
        self.printer.newline()?;
        self.printer.println(&format!(
            "impl<'de> de::Visitor<'de> for {struct_name}Visitor {{"
        ))?;
        self.printer.indent();
        self.printer
            .println(&format!("type Value = Box<dyn {struct_name}Trait>;"))?;
        self.printer.newline()?;
        self.printer.println(
            "fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {",
        )?;
        self.printer.indent();
        self.printer.println(&format!(r#"formatter.write_str("a valid {struct_name}Trait JSON object with a _typeName field")"#))?;
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
        self.printer
            .println("let deserializer = de::value::MapAccessDeserializer::new(&mut map);")?;
        self.printer
            .println("let any: VimAny = de::Deserialize::deserialize(deserializer)?;")?;
        self.printer.println("match any {")?;
        self.printer.indent();
        self.printer
            .println("VimAny::Object(obj) => Ok(CastFrom::from_box(obj)")?;
        self.printer.indent();
        self.printer.println(
            ".map_err(|_| de::Error::custom(\"Internal error converting to trait type\"))?),",
        )?;
        self.printer.dedent();
        self.printer
            .println("VimAny::Value(value) => Err(de::Error::custom(format!(")?;
        self.printer.indent();
        self.printer
            .println(r#""expected object not wrapped value: {:?}","#)?;
        self.printer.println("value))),")?;
        self.printer.dedent();
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
        for (prop_name, property) in &trait_type.fields {
            self.emit_field_getter(prop_name, property)?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_field_getter(&mut self, prop_name: &str, property: &Field) -> Result<()> {
        let getter_name = getter_name(prop_name);
        let mut field_access = format!("self.{}", to_field_name(prop_name));
        let field_type = self.getter_return_type(property)?;
        if get_by_ref(&property.vim_type) {
            field_access = format!("&{field_access}");
        }
        self.printer.println(&format!(
            "fn {getter_name}(&self) -> {field_type} {{ {field_access} }}"
        ))?;
        Ok(())
    }

    fn emit_trait_field(&mut self, prop_name: &str, property: &Field) -> Result<()> {
        {
            let this = &mut *self;
            let doc_string: &Option<String> = &property.description;
            emit_description(this.printer, doc_string)
        }?;
        let field_name = getter_name(prop_name);
        let field_type = self.getter_return_type(property)?;
        self.printer
            .println(&format!("fn {field_name}(&self) -> {field_type};"))?;
        Ok(())
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

    fn getter_return_type(&mut self, property: &Field) -> Result<String> {
        let mut field_type = self.tdf.field_type(property)?;
        if crate::rs_emitter::structs::get_by_ref(&property.vim_type) {
            field_type = format!("&{field_type}");
        }
        if "&String" == field_type {
            field_type = "&str".to_string();
        }
        Ok(field_type)
    }
}
