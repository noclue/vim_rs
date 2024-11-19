use crate::printer::Printer;
use super::errors::Result;


pub fn emit_library(modules: &Vec<String>, printer: &mut dyn Printer) -> Result<()> {
    printer.println("mod base64;")?;
    printer.println("pub mod vim_client;")?;
    printer.println("pub mod types;")?;
    for module in modules {
        printer.println(&format!("pub mod {};", module))?;
    }
    Ok(())
}