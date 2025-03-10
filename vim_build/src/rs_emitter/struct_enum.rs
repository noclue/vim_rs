use convert_case::{Case, Casing};
use crate::printer::Printer;
use crate::rs_emitter;
use crate::vim_model::{Model};

/// Generates enum with all the struct type names. The names are converted to RUST enum naming convention.
pub fn generate_struct_enum(
    vim_model: &Model,
    printer: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    printer.println("/// List of all VIM structure types used in serialization and type casts.")?;
    printer.println("///")?;
    printer.println("/// Values are sorted such that a parent type and all its children always form a contiguous sequence.")?;
    printer.println("///")?;
    printer.println("/// The enum has several advantages over using Rust [`std::any::TypeId`]:")?;
    printer.println("///")?;
    printer
        .println("/// 1. Enum is available at compile time and can be used in match statements.")?;
    printer.println("/// 1. Values are sorted in way that allows match statements over branch in the hierarchy to be ")?;
    printer.println("/// implemented using jump tables i.e. O(1) complexity.")?;
    printer.println("/// 1. Parent child relationship can be checked with range check.")?;
    printer.println("/// 1. Values are 32-bit integers that can be efficiently compared.")?;
    printer.println(
        "#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, serde::Serialize, serde::Deserialize, strum_macros::IntoStaticStr)]",
    )?;
    printer.println("#[repr(u32)]")?;
    printer.println("pub enum StructType {")?;
    printer.indent();
    for (_, data_type) in &vim_model.structs {
        let struct_ref = data_type.borrow();
        let rust_type_name = struct_ref.rust_name();
        if rust_type_name == rs_emitter::structs::ANY {
            continue;
        }
        if rust_type_name != struct_ref.name {
            printer.println(&format!("#[serde(rename = \"{}\")]", struct_ref.name))?;
            printer.println(&format!("#[strum(serialize = \"{}\")]", struct_ref.name))?;
        }
        printer.println(&format!("{},", rust_type_name))?;
    }
    // Make enums open i.e. handle unknown values possibly from future API servers
    printer.println("/// This variant handles values not known at compile time.")?;
    printer.println("#[serde(untagged)]")?;
    printer.println("#[strum(serialize = \"__OTHER__\")]")?;
    printer.println("Other_(String),")?;
    printer.dedent();
    printer.println("}")?;
    generate_child_of_impl(vim_model, printer)?;
    Ok(())
}


pub fn generate_child_of_impl(
    vim_model: &Model,
    prn: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    prn.println("impl StructType {")?;
    prn.indent();
    prn.println("pub fn child_of(&self, other: &StructType) -> bool {")?;
    prn.indent();
    prn.println("if self < other {")?;
    prn.indent();
    prn.println("false // Not equals and not a child - false")?;
    prn.dedent();
    prn.println("} else if other == self {")?;
    prn.indent();
    prn.println("true // Equals - true")?;
    prn.dedent();
    prn.println("} else {")?;
    prn.indent();
    prn.println("match other {")?;
    prn.indent();
    for (_, data_type) in &vim_model.structs {
        let data_type = data_type.borrow();
        if data_type.name == rs_emitter::structs::ANY {
            continue;
        }
        // The if statement below may be slowing things down as it makes the match statement sparse
        // and possibly making the match slower. Of course, it makes the table much smaller i.e. 350
        // entries instead of 3500.
        if !data_type.has_children() {
            continue;
        }
        let parent = &data_type.name.to_case(Case::Pascal);
        let last_child = &data_type.last_child.to_case(Case::Pascal);
        prn.println(&format!("StructType::{parent} => *self <= StructType::{last_child},"))?
    }
    prn.println("_ => false // Others")?;
    prn.dedent();
    prn.println("}")?;
    prn.dedent();
    prn.println("}")?;
    prn.dedent();
    prn.println("}")?;
    prn.dedent();
    prn.println("}")?;
    Ok(())
}
