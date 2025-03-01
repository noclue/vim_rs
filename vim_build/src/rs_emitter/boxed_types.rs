use crate::printer::Printer;
use crate::rs_emitter::common::emit_description;
use crate::rs_emitter::{to_type_name, TypeDefResolver};
use crate::rs_emitter::Result;
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
        self.emit_enum()?;
        self.emit_serialize()?;
        self.emit_deserialize()?;
        Ok(())
    }
    fn emit_enum(&mut self) -> Result<()> {
        self.printer.println("use serde::de;")?;
        self.printer.println("use serde::ser::SerializeStruct;")?;
        self.printer.println("use super::deserialize::get_value_deserializer;")?;
        self.printer.println("use super::vim_any::VimAny;")?;
        self.printer.println("use super::structs::*;")?;
        self.printer.newline()?;
        self.printer.println("#[derive(Debug)]")?;
        //self.printer.println("#[serde(tag = \"_typeName\", content = \"_value\")]")?;
        self.printer.println("pub enum ValueElements {")?;
        self.printer.indent();
        for (_, box_type) in &self.vim_model.any_value_types {
            {
                let this = &mut *self;
                let doc_string: &Option<String> = &box_type.description;
                emit_description(this.printer, doc_string)
            }?;
            //let name = box_type.discriminator_value.as_ref().unwrap_or(&box_type.name);
            let type_name = to_type_name(&box_type.name);
            // if &type_name != name {
            //     self.printer.println(&format!("#[serde(rename = \"{}\")]", name))?;
            // }
            let rust_type = self.tdf.to_rust_field_type(&box_type.property_type)?;
            self.printer.println(&format!("{type_name}({rust_type}),"))?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }
    
    fn emit_serialize(&mut self) -> Result<()> {
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
    
    fn emit_deserialize(&mut self) -> Result<()> {
        self.printer.println("impl<'de> serde::Deserialize<'de> for ValueElements {")?;
        self.printer.indent();
        self.printer.println("fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {")?;
        self.printer.indent();
        self.printer.println("deserializer.deserialize_map(__ValueElementsVisitor)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        self.printer.println("struct __ValueElementsVisitor;")?;
        self.printer.newline()?;
        self.printer.println("impl<'de> serde::de::Visitor<'de> for __ValueElementsVisitor {")?;
        self.printer.indent();
        self.printer.println("type Value = ValueElements;")?;
        self.printer.newline()?;
        self.printer.println("fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {")?;
        self.printer.indent();
        self.printer.println("formatter.write_str(\"A ValueElements!\")")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;
        self.printer.println("fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {")?;
        self.printer.indent();
        self.printer.println("let mut type_name: Option<String> = None;")?;
        self.printer.println("let mut value: Option<&serde_json::value::RawValue> = None;")?;
        self.printer.println("while let Some(key) = map.next_key::<String>()? {")?;
        self.printer.indent();
        self.printer.println("match key.as_str() {")?;
        self.printer.indent();
        self.printer.println("\"_typeName\" => {")?;
        self.printer.indent();
        self.printer.println("if type_name.is_some() {")?;
        self.printer.indent();
        self.printer.println("return Err(de::Error::duplicate_field(\"_typeName\"));")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.println("type_name = Some(map.next_value()?);")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.println("\"_value\" => {")?;
        self.printer.indent();
        self.printer.println("if value.is_some() {")?;
        self.printer.indent();
        self.printer.println("return Err(de::Error::duplicate_field(\"_value\"));")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.println("value = Some(map.next_value()?);")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.println("_ => {")?;
        self.printer.indent();
        self.printer.println("let _: serde_json::Value = map.next_value()?;")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.println("let type_name = type_name.ok_or(de::Error::missing_field(\"_typeName\"))?;")?;
        self.printer.println("let value = value.ok_or(de::Error::missing_field(\"_value\"))?;")?;
        self.printer.println("let Some(dsfunc) = get_value_deserializer(type_name.as_str()) else {")?;
        self.printer.indent();
        self.printer.println("return Err(de::Error::custom(format!(\"Unknown type: {}\", type_name)));")?;
        self.printer.dedent();
        self.printer.println("};")?;
        self.printer.println("dsfunc(value).map_err(de::Error::custom)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        
        
        
        Ok(())
    }
}