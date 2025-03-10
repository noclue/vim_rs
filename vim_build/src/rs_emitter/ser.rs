use crate::printer::Printer;
use crate::rs_emitter;
use crate::vim_model::{EmitMode, Model};

pub fn generate_serialize_polymorphic_enum(
    vim_model: &Model,
    printer: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    printer.println("use serde::Serialize;")?;
    printer.println("use super::struct_enum::StructType;")?;
    printer.println("use super::vim_object_trait::VimObjectTrait;")?;
    printer.println("use super::structs::*;")?;
    printer.println("")?;
    printer.println("/// Serialize a polymorphic VimObjectTrait into serde::Serializer")?;
    printer.println("pub fn serialize_polymorphic<S>(p: &dyn VimObjectTrait, serializer: S) -> Result<S::Ok, S::Error>")?;
    printer.println("where")?;
    printer.indent();
    printer.println("S: serde::Serializer,")?;
    printer.dedent();
    printer.println("{")?;
    printer.indent();
    printer.println("let data_type = p.data_type();")?;
    printer.println("match data_type {")?;
    printer.indent();
    for (_, data_type) in &vim_model.structs {
        let struct_name = data_type.borrow().rust_name();
        if struct_name == "Any" {
            continue;
        }
        if matches!(data_type.borrow().emit_mode, EmitMode::Skip(_)) {
            continue;
        }

        printer.println(&format!(
            "StructType::{struct_name} => {struct_name}::serialize("
        ))?;
        printer.indent();
        printer.println(&format!(
            "p.as_any_ref().downcast_ref::<{struct_name}>().unwrap(),"
        ))?;
        printer.println("serializer,")?;
        printer.dedent();
        printer.println("),")?;
    }
    printer.println("_ => Err(serde::ser::Error::custom(\"Unknown VIM data type\")),")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;

    Ok(())
}
