use crate::printer::Printer;
use crate::rs_emitter::common::emit_description;
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
    pub fn emit_boxed_types(&mut self) -> crate::rs_emitter::Result<()> {
        self.emit_enum()?;
        self.emit_serialize()?;
        Ok(())
    }
    fn emit_enum(&mut self) -> crate::rs_emitter::Result<()> {
        self.printer.println("use serde::ser::SerializeStruct;")?;
        self.printer.println("use super::vim_any::VimAny;")?;
        self.printer.println("use super::structs::*;")?;
        self.printer.newline()?;
        self.printer.println("#[derive(Debug, serde::Deserialize)]")?;
        self.printer.println("#[serde(tag = \"_typeName\", content = \"_value\")]")?;
        self.printer.println("pub enum ValueElements {")?;
        self.printer.indent();
        for (_, box_type) in &self.vim_model.any_value_types {
            {
                let this = &mut *self;
                let doc_string: &Option<String> = &box_type.description;
                emit_description(this.printer, doc_string)
            }?;
            let name = box_type.discriminator_value.as_ref().unwrap_or(&box_type.name);
            let type_name = to_type_name(&box_type.name);
            if &type_name != name {
                self.printer.println(&format!("#[serde(rename = \"{}\")]", name))?;
            }
            let rust_type = self.tdf.to_rust_field_type(&box_type.property_type)?;
            self.printer.println(&format!("{type_name}({rust_type}),"))?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }
    
    fn emit_serialize(& mut self) -> crate::rs_emitter::Result<()> {
        self.printer.println("impl serde::Serialize for ValueElements {")?;
        self.printer.indent();
        self.printer.println("fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>")?;
        self.printer.println("where")?;
        self.printer.println("S: serde::Serializer,")?;
        self.printer.println("{")?;
        self.printer.indent();
        self.printer.println("let mut state = serializer.serialize_struct(\"ValueElements\", 2)?;")?;
        self.printer.println("match self {")?;
        self.printer.indent();
        for (_, box_type) in &self.vim_model.any_value_types {
            let type_name = to_type_name(&box_type.name);
            let ser_name = &box_type.discriminator_value.as_ref().unwrap_or(&box_type.name);
            self.printer.println(&format!("ValueElements::{type_name}(value) => {{"))?;
            self.printer.indent();
            self.printer.println(&format!("state.serialize_field(\"_typeName\", \"{ser_name}\")?;"))?;
            let value = match box_type.property_type {
                DataType::Binary => {
                    "&crate::core::helpers::SerializeBinary { value }"
                }
                _ => {
                    "value"
                }
            };
            self.printer.println(&format!("state.serialize_field(\"_value\", {value})?;"))?;
            self.printer.dedent();
            self.printer.println("},")?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.println("state.end()")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }
}