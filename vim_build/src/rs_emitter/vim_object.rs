use crate::printer::Printer;
use crate::rs_emitter;
use crate::vim_model::Model;

pub fn generate_vim_object_trait(vim_model: & Model, printer: & mut dyn Printer) -> rs_emitter::errors::Result<()> {
    printer.println("use super::as_any::AsAny;")?;
    printer.println("use super::dyn_serialize;")?;
    printer.println("use super::struct_enum::StructType;")?;
    printer.println("use super::structs::*;")?;
    printer.println("")?;
    printer.println("/// Base trait of all VIM (Virtual Infrastructure Management) objects.")?;
    printer.println("/// This trait is used to obtain the actual data type of object even")?;
    printer.println("/// when used through a trait reference. The other use of this trait is")?;
    printer.println("/// to upcast a trait reference to a VimObjectTrait reference needed by")?;
    printer.println("/// common library functionality.")?;
    printer.println("pub trait VimObjectTrait: AsAny + std::fmt::Debug {")?;
    printer.indent();
    printer.println("fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait;")?;
    printer.println("fn data_type(&self) -> StructType;")?;
    printer.dedent();
    printer.println("}")?;
    printer.println("")?;
    printer.println("impl serde::Serialize for dyn VimObjectTrait {")?;
    printer.indent();
    printer.println("fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>")?;
    printer.println("where")?;
    printer.indent();
    printer.println("S: serde::Serializer,")?;
    printer.dedent();
    printer.println("{")?;
    printer.indent();
    printer.println("dyn_serialize::serialize_polymorphic(self, serializer)")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    printer.println("")?;
    for (_, data_type) in &vim_model.structs {
        let struct_name =  data_type.borrow().rust_name();
        if struct_name == "Any" {
            continue;
        }
        printer.println(&format!("impl VimObjectTrait for {struct_name} {{"))?;
        printer.indent();
        printer.println("fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {")?;
        printer.indent();
        printer.println("self")?;
        printer.dedent();
        printer.println("}")?;
        printer.println("")?;
        printer.println("fn data_type(&self) -> StructType {")?;
        printer.indent();
        printer.println(&format!("StructType::{struct_name}"))?;
        printer.dedent();
        printer.println("}")?;
        printer.dedent();
        printer.println("}")?;
        printer.println("")?;
    }
    Ok(())
}