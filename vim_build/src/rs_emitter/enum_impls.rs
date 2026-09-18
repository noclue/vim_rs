use convert_case::{Case, Casing};
use crate::printer::Printer;
use crate::vim_model::{Enum, Model};
use super::errors::Result;
use super::{to_enum_variant, to_type_name};

/// Generates manual implementations for enums to avoid heavy macro overhead.
/// Each enum gets:
/// - A PHF map for string-to-variant lookup
/// - Manual Serialize/Deserialize implementations
/// - as_str() and from_str() methods
/// - Display and Debug implementations
pub fn generate_enum_impls(
    vim_model: &Model,
    printer: &mut dyn Printer,
) -> Result<()> {
    // Emit make_place! for the shared Place type used by Deserialize impls
    printer.println("miniserde::make_place!(Place);")?;
    printer.newline()?;
    for (_, vim_enum) in &vim_model.enums {
        generate_single_enum_impl(vim_enum, printer)?;
        printer.newline()?;
    }
    Ok(())
}

fn generate_single_enum_impl(
    vim_enum: &Enum,
    printer: &mut dyn Printer,
) -> Result<()> {
    let enum_name = to_type_name(&vim_enum.name);
    
    // Generate PHF map
    generate_enum_phf_map(vim_enum, &enum_name, printer)?;
    printer.newline()?;
    
    // Generate impl block with as_str() and from_str()
    generate_enum_methods(vim_enum, &enum_name, printer)?;
    printer.newline()?;
    
    // Generate Serialize implementation
    generate_enum_serialize(vim_enum, &enum_name, printer)?;
    printer.newline()?;
    
    // Generate Deserialize implementation
    generate_enum_deserialize(vim_enum, &enum_name, printer)?;
    printer.newline()?;
    
    // Generate Display implementation
    generate_enum_display(&enum_name, printer)?;
    printer.newline()?;
    
    // Generate Debug implementation
    generate_enum_debug(&enum_name, printer)?;
    printer.newline()?;
    
    // Generate From implementation
    generate_enum_from(&enum_name, printer)?;
    printer.newline()?;
    
    // Generate AsRef implementation
    generate_enum_asref(&enum_name, printer)?;
    
    Ok(())
}

fn generate_enum_phf_map(
    vim_enum: &Enum,
    enum_name: &str,
    printer: &mut dyn Printer,
) -> Result<()> {
    let map_name = format!("{}_MAP", enum_name.to_case(Case::UpperSnake));
    let mut entries: Vec<(String, String)> = Vec::new();
    
    for value in &vim_enum.variants {
        let variant = to_enum_variant(value);
        entries.push((value.clone(), format!("{}::{}", enum_name, variant)));
    }

    let mut map_builder = phf_codegen::Map::new();
    for (key, value) in &entries {
        map_builder.entry(key, value);
    }
    
    printer.println(&format!(
        "static {}: phf::Map<&'static str, {}> = {};",
        map_name,
        enum_name,
        map_builder.build()
    ))?;
    
    Ok(())
}

fn generate_enum_methods(
    vim_enum: &Enum,
    enum_name: &str,
    printer: &mut dyn Printer,
) -> Result<()> {
    printer.println(&format!("impl {} {{", enum_name))?;
    printer.indent();
    
    // Generate as_str() method
    printer.println("pub fn as_str(&self) -> &str {")?;
    printer.indent();
    printer.println("match self {")?;
    printer.indent();
    for value in &vim_enum.variants {
        let variant = to_enum_variant(value);
        printer.println(&format!("{}::{} => \"{}\",", enum_name, variant, value))?;
    }
    printer.println(&format!("{}::Other_(s) => s,", enum_name))?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    printer.newline()?;
    
    // Generate from_str() method
    let map_name = format!("{}_MAP", enum_name.to_case(Case::UpperSnake));
    printer.println("pub fn from_str(s: &str) -> Self {")?;
    printer.indent();
    printer.println(&format!(
        "{}.get(s).cloned().unwrap_or_else(|| {}::Other_(s.to_string()))",
        map_name, enum_name
    ))?;
    printer.dedent();
    printer.println("}")?;
    
    printer.dedent();
    printer.println("}")?;
    
    Ok(())
}

fn generate_enum_serialize(
    _vim_enum: &Enum,
    enum_name: &str,
    printer: &mut dyn Printer,
) -> Result<()> {
    printer.println(&format!("impl miniserde::Serialize for {} {{", enum_name))?;
    printer.indent();
    printer.println("fn begin(&self) -> miniserde::ser::Fragment<'_> {")?;
    printer.indent();
    printer.println("miniserde::ser::Fragment::Str(std::borrow::Cow::Borrowed(self.as_str()))")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    
    Ok(())
}

fn generate_enum_deserialize(
    _vim_enum: &Enum,
    enum_name: &str,
    printer: &mut dyn Printer,
) -> Result<()> {
    // impl Deserialize
    printer.println(&format!("impl miniserde::Deserialize for {} {{", enum_name))?;
    printer.indent();
    printer.println("fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {")?;
    printer.indent();
    printer.println("Place::new(out)")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    printer.newline()?;
    
    // impl Visitor for Place<EnumName>
    printer.println(&format!("impl miniserde::de::Visitor for Place<{}> {{", enum_name))?;
    printer.indent();
    printer.println("fn string(&mut self, s: &str) -> miniserde::Result<()> {")?;
    printer.indent();
    printer.println(&format!("self.out = Some({}::from_str(s));", enum_name))?;
    printer.println("Ok(())")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    
    Ok(())
}

fn generate_enum_display(
    enum_name: &str,
    printer: &mut dyn Printer,
) -> Result<()> {
    printer.println(&format!("impl std::fmt::Display for {} {{", enum_name))?;
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

fn generate_enum_debug(
    enum_name: &str,
    printer: &mut dyn Printer,
) -> Result<()> {
    printer.println(&format!("impl std::fmt::Debug for {} {{", enum_name))?;
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

fn generate_enum_from(
    enum_name: &str,
    printer: &mut dyn Printer,
) -> Result<()> {
    printer.println(&format!("impl<'a> From<&'a {}> for &'a str {{", enum_name))?;
    printer.indent();
    printer.println(&format!("fn from(value: &'a {}) -> Self {{", enum_name))?;
    printer.indent();
    printer.println("value.as_str()")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    
    Ok(())
}

fn generate_enum_asref(
    enum_name: &str,
    printer: &mut dyn Printer,
) -> Result<()> {
    printer.println(&format!("impl AsRef<str> for {} {{", enum_name))?;
    printer.indent();
    printer.println("fn as_ref(&self) -> &str {")?;
    printer.indent();
    printer.println("self.as_str()")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    
    Ok(())
}
