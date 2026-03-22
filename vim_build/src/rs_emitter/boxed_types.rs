use crate::printer::Printer;
use crate::rs_emitter::common::emit_description;
use crate::rs_emitter::Result;
use crate::rs_emitter::{to_type_name, TypeDefResolver};
use crate::vim_model::{DataType, Model};

pub struct BoxedTypesEmitter<'a> {
    vim_model: &'a Model,
    printer: &'a mut dyn Printer,
    tdf: TypeDefResolver<'a>,
}

impl<'a> BoxedTypesEmitter<'a> {
    pub fn new(vim_model: &'a Model, printer: &'a mut dyn Printer) -> BoxedTypesEmitter<'a> {
        BoxedTypesEmitter {
            vim_model,
            printer,
            tdf: TypeDefResolver::new(vim_model),
        }
    }

    /// Emit boxed value types from Vim like ArrayOfInt, ArrayOfString, Boolean etc.
    pub fn emit_boxed_types(&mut self) -> Result<()> {
        self.emit_imports()?;
        self.emit_enum()?;
        self.emit_as_str()?;
        self.emit_into_any()?;
        self.emit_serialize()?;
        // Note: Deserialize for ValueElements is now generated in deserialize.rs
        Ok(())
    }
    fn emit_enum(&mut self) -> Result<()> {
        self.printer.println("#[derive(Debug)]")?;
        self.printer.println("pub enum ValueElements {")?;
        self.printer.indent();
        for (_, box_type) in &self.vim_model.any_value_types {
            {
                let this = &mut *self;
                let doc_string: &Option<String> = &box_type.description;
                emit_description(this.printer, doc_string)
            }?;
            let type_name = to_type_name(&box_type.name);
            let rust_type = self.tdf.to_rust_field_type(&box_type.property_type)?;
            self.printer
                .println(&format!("{type_name}({rust_type}),"))?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_imports(&mut self) -> Result<()> {
        self.printer.println("use miniserde::ser::Fragment;")?;
        self.printer.println("use std::borrow::Cow;")?;
        self.printer.newline()?;
        Ok(())
    }

    fn emit_as_str(&mut self) -> Result<()> {
        self.printer.println("impl ValueElements {")?;
        self.printer.indent();
        self.printer.println("/// Returns the VIM API type name as it appears in the OpenAPI specification.")?;
        self.printer.println("pub fn as_str(&self) -> &'static str {")?;
        self.printer.indent();
        self.printer.println("match self {")?;
        self.printer.indent();
        for (_, box_type) in &self.vim_model.any_value_types {
            let type_name = to_type_name(&box_type.name);
            let str_name = box_type
                .discriminator_value
                .as_ref()
                .unwrap_or(&box_type.name);
            self.printer
                .println(&format!("ValueElements::{type_name}(_) => \"{str_name}\","))?;
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

    /// Move the inner value into a type-erased box for `std::any::Any` downcasting.
    fn emit_into_any(&mut self) -> Result<()> {
        self.printer.println("impl ValueElements {")?;
        self.printer.indent();
        self.printer.println("/// Moves the wrapped value into a `Box<dyn std::any::Any>` for downcasting.")?;
        self.printer.println("pub fn into_any(self) -> Box<dyn std::any::Any> {")?;
        self.printer.indent();
        self.printer.println("match self {")?;
        self.printer.indent();
        for (_, box_type) in &self.vim_model.any_value_types {
            let type_name = to_type_name(&box_type.name);
            self.printer.println(&format!(
                "ValueElements::{type_name}(v) => Box::new(v),"
            ))?;
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

    fn emit_serialize(&mut self) -> Result<()> {
        // impl miniserde::Serialize for ValueElements
        self.printer
            .println("impl miniserde::Serialize for ValueElements {")?;
        self.printer.indent();
        self.printer
            .println("fn begin(&self) -> Fragment<'_> {")?;
        self.printer.indent();
        self.printer
            .println("Fragment::Map(Box::new(ValueElementsSerializer::new(self)))")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // ValueElementsSerializer struct
        self.printer.println("struct ValueElementsSerializer<'a> {")?;
        self.printer.indent();
        self.printer.println("data: &'a ValueElements,")?;
        self.printer.println("seq: usize,")?;
        self.printer.println("type_name: &'static str,")?;
        // Pre-compute base64 for binary variants
        let has_binary = self.vim_model.any_value_types.values().any(|bt| bt.property_type == DataType::Binary);
        if has_binary {
            self.printer.println("b64_cache: Option<String>,")?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        self.printer.println("impl<'a> ValueElementsSerializer<'a> {")?;
        self.printer.indent();
        self.printer.println("fn new(data: &'a ValueElements) -> Self {")?;
        self.printer.indent();
        if has_binary {
            self.printer.println("let b64_cache = match data {")?;
            self.printer.indent();
            for (_, box_type) in &self.vim_model.any_value_types {
                if box_type.property_type == DataType::Binary {
                    let variant = to_type_name(&box_type.name);
                    self.printer.println(&format!(
                        "ValueElements::{variant}(value) => Some(base64::display::Base64Display::new(value, &base64::engine::general_purpose::STANDARD).to_string()),"
                    ))?;
                }
            }
            self.printer.println("_ => None,")?;
            self.printer.dedent();
            self.printer.println("};")?;
            self.printer.println("Self { data, seq: 0, type_name: data.as_str(), b64_cache }")?;
        } else {
            self.printer.println("Self { data, seq: 0, type_name: data.as_str() }")?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // impl Map for ValueElementsSerializer
        self.printer.println("impl<'a> miniserde::ser::Map for ValueElementsSerializer<'a> {")?;
        self.printer.indent();
        self.printer.println("fn next(&mut self) -> Option<(Cow<'_, str>, &dyn miniserde::Serialize)> {")?;
        self.printer.indent();
        self.printer.println("let seq = self.seq;")?;
        self.printer.println("self.seq += 1;")?;
        self.printer.println("match seq {")?;
        self.printer.indent();
        // seq 0: _typeName
        self.printer.println("0 => Some((Cow::Borrowed(\"_typeName\"), &self.type_name)),")?;
        // seq 1: _value
        self.printer.println("1 => {")?;
        self.printer.indent();
        self.printer.println("match self.data {")?;
        self.printer.indent();
        for (_, box_type) in &self.vim_model.any_value_types {
            let variant = to_type_name(&box_type.name);
            if box_type.property_type == DataType::Binary {
                self.printer.println(&format!(
                    "ValueElements::{variant}(_) => Some((Cow::Borrowed(\"_value\"), self.b64_cache.as_ref().unwrap() as &dyn miniserde::Serialize)),"
                ))?;
            } else {
                self.printer.println(&format!(
                    "ValueElements::{variant}(value) => Some((Cow::Borrowed(\"_value\"), value as &dyn miniserde::Serialize)),"
                ))?;
            }
        }
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.println("_ => None,")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;

        Ok(())
    }
}
