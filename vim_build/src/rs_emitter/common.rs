use super::errors::Result;
use crate::printer::Printer;

pub fn emit_description(printer: &mut dyn Printer, doc_string: &Option<String>) -> Result<()> {
    if let Some(doc) = doc_string {
        for line in doc.trim().split('\n') {
            printer.println(&format!("/// {}", line))?;
        }
    }
    Ok(())
}
