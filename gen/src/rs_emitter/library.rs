use crate::printer::Printer;
use super::errors::Result;


pub fn emit_library(modules: &Vec<String>, printer: &mut dyn Printer) -> Result<()> {
    for module in modules {
        printer.println(&format!("pub mod {};", module))?;
    }
    printer.println("")?;
    // Add public re-exports
    for module in modules {
        printer.println(&format!("pub use {}::*;", module))?;
    }
    Ok(())
}