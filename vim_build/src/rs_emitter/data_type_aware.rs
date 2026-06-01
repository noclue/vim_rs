//! Emits `vim_rs/src/types/data_type_aware_impl.rs` — `DataTypeAware` impls for all structs and enums.

use crate::printer::Printer;
use crate::rs_emitter::to_type_name;
use crate::rs_emitter::structs::ANY;
use crate::vim_model::{EmitMode, Model};

use super::errors::Result;

pub fn generate_data_type_aware_impls(model: &Model, printer: &mut dyn Printer) -> Result<()> {
    printer.println("//! GENERATED — do not edit. See `vim_build`.")?;
    printer.println("#![cfg(feature = \"xml\")]")?;
    printer.newline()?;
    printer.println("use super::api_field_types::ApiFieldType;")?;
    printer.println("use super::data_type_aware::DataTypeAware;")?;
    printer.println("use super::struct_enum::StructType;")?;
    printer.println("use super::enums::*;")?;
    printer.println("use super::structs::*;")?;
    printer.newline()?;

    for (_, data_type) in &model.structs {
        let struct_ref = data_type.borrow();
        if struct_ref.name == "Any" {
            continue;
        }
        if let EmitMode::Skip(_) = struct_ref.emit_mode {
            continue;
        }
        let rust_type_name = struct_ref.rust_name();
        if rust_type_name == ANY {
            continue;
        }
        let type_name = to_type_name(&struct_ref.name);
        printer.println(&format!(
            "impl DataTypeAware for {} {{",
            type_name
        ))?;
        printer.indent();
        printer.println("fn data_type() -> ApiFieldType {")?;
        printer.indent();
        printer.println(&format!(
            "ApiFieldType::Object(StructType::{})",
            rust_type_name
        ))?;
        printer.dedent();
        printer.println("}")?;
        printer.dedent();
        printer.println("}")?;
        printer.newline()?;
    }

    for (_, vim_enum) in &model.enums {
        let enum_name = to_type_name(&vim_enum.name);
        printer.println(&format!("impl DataTypeAware for {} {{", enum_name))?;
        printer.indent();
        printer.println("fn data_type() -> ApiFieldType {")?;
        printer.indent();
        printer.println("ApiFieldType::Str")?;
        printer.dedent();
        printer.println("}")?;
        printer.dedent();
        printer.println("}")?;
        printer.newline()?;
    }

    // Polymorphic SOAP/XML roots use `Box<dyn …Trait>` the same way as concrete structs.
    // Match `emit_inheritable_traits`: only types that actually emitted a `*Trait` in `traits.rs`.
    for (_, data_type) in &model.structs {
        let struct_ref = data_type.borrow();
        if struct_ref.name == "Any" {
            continue;
        }
        if struct_ref.emit_mode != EmitMode::Emit {
            continue;
        }
        if struct_ref.children.is_empty() {
            continue;
        }
        let rust_type_name = struct_ref.rust_name();
        let type_name = to_type_name(&struct_ref.name);
        printer.println(&format!(
            "impl DataTypeAware for Box<dyn super::traits::{type_name}Trait> {{"
        ))?;
        printer.indent();
        printer.println("fn data_type() -> ApiFieldType {")?;
        printer.indent();
        printer.println(&format!(
            "ApiFieldType::Object(StructType::{})",
            rust_type_name
        ))?;
        printer.dedent();
        printer.println("}")?;
        printer.dedent();
        printer.println("}")?;
        printer.newline()?;
    }

    Ok(())
}
