use crate::{printer::Printer, vim_model::Model};

use super::common::emit_description_with_paths;
use super::errors::Result;
use super::{to_enum_variant, to_type_name};

pub fn emit_enums(vim_model: &Model, printer: &mut dyn Printer) -> Result<()> {
    for (_, vim_enum) in &vim_model.enums {
        {
            let doc_string: &Option<String> = &vim_enum.description;
            emit_description_with_paths(printer, doc_string, &vim_enum.paths)
        }?;

        let enum_name = to_type_name(&vim_enum.name);
        printer.println("#[derive(Clone, PartialEq, Eq, Hash)]")?;
        printer.println(&format!("pub enum {} {{", enum_name))?;
        printer.indent();
        for value in &vim_enum.variants {
            let variant = to_enum_variant(value);
            printer.println(&format!("{},", variant))?;
        }
        // Make enums open i.e. handle unknown values possibly from future API servers
        printer.println("/// This variant handles values not known at compile time.")?;
        printer.println("Other_(String),")?;
        printer.dedent();
        printer.println("}")?;
        printer.newline()?;
    }
    Ok(())
}
