use crate::printer::Printer;
use super::errors::{Result};



pub fn emit_doc(printer:&mut dyn Printer, doc_string: &Option<String>) -> Result<()> {
    if let Some(doc) = doc_string {
        for line in doc.trim().split('\n') {
            printer.println(&format!("/// {}", line))?;
        }
    }
    Ok(())
}