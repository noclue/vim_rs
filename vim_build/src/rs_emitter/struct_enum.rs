use crate::printer::Printer;
use crate::rs_emitter;
use crate::vim_model::Model;

/// Generates enum with all the struct type names. The names are converted to RUST enum naming convention.
pub fn generate_struct_enum(vim_model: & Model, printer: & mut dyn Printer) -> rs_emitter::errors::Result<()> {
    printer.println("/// List of all VIM structure types used in serialization and type casts.")?;
    printer.println("///")?;
    printer.println("/// Values are sorted such that a parent type and all its children always form a contiguous sequence.")?;
    printer.println("///")?;
    printer.println("/// The enum has several advantages over using Rust [`std::any::TypeId`]:" )?;
    printer.println("///")?;
    printer.println("/// 1. Enum is available at compile time and can be used in match statements.")?;
    printer.println("/// 1. Values are sorted in way that allows match statements over branch in the hierarchy to be ")?;
    printer.println("/// implemented using jump tables i.e. O(1) complexity.")?;
    printer.println("/// 1. Parent child relationship can be checked with range check.")?;
    printer.println("/// 1. Values are 32-bit integers that can be efficiently compared.")?;
    printer.println("#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, strum_macros::IntoStaticStr)]")?;
    printer.println("#[repr(u32)]")?;
    printer.println("pub enum StructType {")?;
    printer.indent();
    for (_, data_type) in &vim_model.structs {
        let struct_ref = data_type.borrow();
        let rust_type_name =  struct_ref.rust_name();
        if rust_type_name == rs_emitter::structs::ANY {
            continue;
        }
        if rust_type_name != struct_ref.name {
            printer.println(&format!("#[strum(serialize = \"{}\")]", struct_ref.name))?;
        }
        printer.println(&format!("{},", rust_type_name))?;
    }
    printer.dedent();
    printer.println("}")?;
    Ok(())
}