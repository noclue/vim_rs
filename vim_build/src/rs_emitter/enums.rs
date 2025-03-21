use crate::{printer::Printer, vim_model::Model};

use super::common::emit_description;
use super::errors::Result;
use super::{to_enum_variant, to_type_name};

pub fn emit_enums(vim_model: &Model, printer: &mut dyn Printer) -> Result<()> {
    for (_, vim_enum) in &vim_model.enums {
        {
            let doc_string: &Option<String> = &vim_enum.description;
            emit_description(printer, doc_string)
        }?;

        let enum_name = to_type_name(&vim_enum.name);
        if vim_enum.name == "MoTypes_enum" {
            // Add clone and partial eq for MoTypes_enum
            printer.println("#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]")?;
        } else {
            printer.println("#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]")?;
        }
        printer.println(&format!("pub enum {} {{", enum_name))?;
        printer.indent();
        for value in &vim_enum.variants {
            let variant = to_enum_variant(value);
            if value != &variant {
                printer.println(&format!("#[serde(rename = \"{}\")]", value))?;
                printer.println(&format!("#[strum(serialize = \"{}\")]", value))?;
            }
            printer.println(&format!("{},", variant))?;
        }
        // Make enums open i.e. handle unknown values possibly from future API servers
        printer.println("/// This variant handles values not known at compile time.")?;
        printer.println("#[serde(untagged)]")?;
        printer.println("#[strum(serialize = \"__OTHER__\")]")?;
        printer.println("Other_(String),")?;
        printer.dedent();
        printer.println("}")?;
    }
    Ok(())
}
