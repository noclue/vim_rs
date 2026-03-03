use crate::printer::Printer;
use crate::rs_emitter;
use crate::vim_model::Model;

/// With miniserde, polymorphic serialization is handled by the VimObjectTrait supertrait
/// (VimObjectTrait: miniserde::Serialize). Each concrete type implements Serialize,
/// and Rust's vtable dispatch handles the rest. No explicit match dispatch is needed.
///
/// This file is kept for backwards compatibility with the module structure.
pub fn generate_dyn_serialize(
    _vim_model: &Model,
    printer: &mut dyn Printer,
) -> rs_emitter::errors::Result<()> {
    printer.println(
        "// Polymorphic serialization is handled via VimObjectTrait: miniserde::Serialize supertrait.",
    )?;
    printer.println(
        "// Each concrete type implements miniserde::Serialize and vtable dispatch handles the rest.",
    )?;
    Ok(())
}
