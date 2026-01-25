use convert_case::{Case, Casing};
use crate::printer::Printer;
use crate::rs_emitter;
use crate::vim_model::{Model};

/// Generates enum with all the struct type names. The names are converted to RUST enum naming convention.
pub fn generate_struct_enum(
    vim_model: &Model,
    printer: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    printer.println("use serde::de;")?;
    printer.newline()?;
    
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
        "#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]",
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
        printer.println(&format!("{},", rust_type_name))?;
    }
    printer.dedent();
    printer.println("}")?;
    printer.newline()?;
    
    generate_phf_map(vim_model, printer)?;
    printer.newline()?;
    
    generate_struct_type_impl(vim_model, printer)?;
    printer.newline()?;
    
    generate_serialize_impl(printer)?;
    printer.newline()?;
    
    generate_deserialize_impl(printer)?;
    printer.newline()?;
    
    generate_display_impl(printer)?;
    printer.newline()?;
    
    generate_debug_impl(printer)?;
    printer.newline()?;
    
    generate_from_impl(printer)?;
    printer.newline()?;
    
    generate_child_of_impl(vim_model, printer)?;
    Ok(())
}


fn generate_phf_map(
    vim_model: &Model,
    printer: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    let mut map_builder = phf_codegen::Map::new();
    
    for (_, data_type) in &vim_model.structs {
        let struct_ref = data_type.borrow();
        let rust_type_name = struct_ref.rust_name();
        if rust_type_name == rs_emitter::structs::ANY {
            continue;
        }
        let vim_name = struct_ref.name.clone();
        map_builder.entry(vim_name, &format!("StructType::{}", rust_type_name));
    }
    
    printer.println(&format!(
        "static STRUCT_TYPE_MAP: phf::Map<&'static str, StructType> = {};",
        map_builder.build()
    ))?;
    Ok(())
}

fn generate_struct_type_impl(
    vim_model: &Model,
    printer: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    printer.println("impl StructType {")?;
    printer.indent();
    
    // Generate as_str() method
    printer.println("pub fn as_str(self) -> &'static str {")?;
    printer.indent();
    printer.println("match self {")?;
    printer.indent();
    for (_, data_type) in &vim_model.structs {
        let struct_ref = data_type.borrow();
        let rust_type_name = struct_ref.rust_name();
        if rust_type_name == rs_emitter::structs::ANY {
            continue;
        }
        let vim_name = &struct_ref.name;
        printer.println(&format!("StructType::{} => \"{}\",", rust_type_name, vim_name))?;
    }
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    printer.newline()?;
    
    // Generate from_str() method
    printer.println("pub fn from_str(s: &str) -> Option<StructType> {")?;
    printer.indent();
    printer.println("STRUCT_TYPE_MAP.get(s).copied()")?;
    printer.dedent();
    printer.println("}")?;
    
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn generate_serialize_impl(
    printer: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    printer.println("impl serde::Serialize for StructType {")?;
    printer.indent();
    printer.println("fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>")?;
    printer.indent();
    printer.println("where")?;
    printer.indent();
    printer.println("S: serde::Serializer,")?;
    printer.dedent();
    printer.dedent();
    printer.println("{")?;
    printer.indent();
    printer.println("serializer.serialize_str(self.as_str())")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn generate_deserialize_impl(
    printer: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    printer.println("impl<'de> de::Deserialize<'de> for StructType {")?;
    printer.indent();
    printer.println("fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {")?;
    printer.indent();
    printer.println("let s = String::deserialize(deserializer)?;")?;
    printer.println("StructType::from_str(&s).ok_or_else(|| de::Error::custom(\"Invalid struct type name\"))")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn generate_display_impl(
    printer: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    printer.println("impl std::fmt::Display for StructType {")?;
    printer.indent();
    printer.println("fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")?;
    printer.indent();
    printer.println("write!(f, \"{}\", self.as_str())")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn generate_debug_impl(
    printer: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    printer.println("impl std::fmt::Debug for StructType {")?;
    printer.indent();
    printer.println("fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")?;
    printer.indent();
    printer.println("write!(f, \"{}\", self.as_str())")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn generate_from_impl(
    printer: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    printer.println("impl From<StructType> for &'static str {")?;
    printer.indent();
    printer.println("fn from(value: StructType) -> Self {")?;
    printer.indent();
    printer.println("value.as_str()")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

pub fn generate_child_of_impl(
    vim_model: &Model,
    prn: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    prn.println("impl StructType {")?;
    prn.indent();
    prn.println("pub fn child_of(self, other: StructType) -> bool {")?;
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
        prn.println(&format!("StructType::{parent} => self <= StructType::{last_child},"))?
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
