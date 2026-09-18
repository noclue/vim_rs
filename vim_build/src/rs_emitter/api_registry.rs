//! Emits `api_field_registry.rs` — `StructType`-indexed field metadata for XML typed deserialization.

use std::collections::BTreeSet;

use convert_case::{Case, Casing};
use phf_codegen::Map as PhfMapBuilder;
use crate::printer::Printer;
use crate::rs_emitter::errors::Result;
use crate::rs_emitter::structs::ANY;
use crate::vim_model::{DataType, Model, Struct};

/// Generate `vim_rs/src/types/api_field_registry.rs`.
pub fn generate_api_field_registry(vim_model: &Model, printer: &mut dyn Printer) -> Result<()> {
    printer.println("//! API field registry for schema-guided XML deserialization.")?;
    printer.println("//!")?;
    printer.println("//! GENERATED FILE — do not edit.")?;
    printer.newline()?;
    printer.println("#![cfg(feature = \"xml\")]")?;
    printer.newline()?;
    printer.println("use super::api_field_types::{ApiFieldType, ApiTypeInfo};")?;
    printer.println("use super::struct_enum::StructType;")?;
    printer.newline()?;

    // Collect every signature used by any struct field.
    let mut sigs: BTreeSet<String> = BTreeSet::new();
    for (_, data_type) in &vim_model.structs {
        let s = data_type.borrow();
        if s.rust_name() == ANY {
            continue;
        }
        for (_, field) in &s.fields {
            collect_sigs(&field.vim_type, vim_model, &mut sigs);
        }
    }
    // Boxed `Any` descendants (`Primitive*`, `ArrayOf*`, …) need static refs even when they
    // never appear as a struct field type.
    for (_, bt) in &vim_model.any_value_types {
        collect_sigs(&bt.property_type, vim_model, &mut sigs);
    }

    // Emit static ApiFieldType values: inner types before `Array(&...)`.
    let ordered: Vec<String> = topo_sort_sigs(sigs);
    for sig in &ordered {
        emit_static_for_sig(printer, sig, vim_model)?;
    }

    let struct_count = count_struct_variants(vim_model);
    printer.println(&format!("pub const STRUCT_TYPE_COUNT: usize = {struct_count};"))?;
    printer.newline()?;

    printer.println("#[allow(clippy::large_stack_arrays)]")?;
    printer.println("static API_FIELD_TABLE: [ApiTypeInfo; STRUCT_TYPE_COUNT] = [")?;
    printer.indent();

    for (_, data_type) in &vim_model.structs {
        let s = data_type.borrow();
        if s.rust_name() == ANY {
            continue;
        }
        emit_api_type_info(printer, &s, vim_model)?;
    }

    printer.dedent();
    printer.println("];")?;
    printer.newline()?;

    printer.println(
        "/// Look up a field's ApiFieldType by walking the type's inheritance chain.",
    )?;
    printer.println("pub fn lookup_api_field(st: StructType, field_name: &str) -> Option<ApiFieldType> {")?;
    printer.indent();
    printer.println("let mut current = st;")?;
    printer.println("loop {")?;
    printer.indent();
    printer.println("let info = &API_FIELD_TABLE[current as usize];")?;
    printer.println("for &(name, ft) in info.fields {")?;
    printer.indent();
    printer.println("if name == field_name {")?;
    printer.indent();
    printer.println("return Some(ft);")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    printer.println("current = match info.parent {")?;
    printer.indent();
    printer.println("Some(p) => p,")?;
    printer.println("None => return None,")?;
    printer.dedent();
    printer.println("};")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    printer.newline()?;

    printer.println("#[inline]")?;
    printer.println("pub fn resolve_type(type_name: &str) -> Option<StructType> {")?;
    printer.indent();
    printer.println("StructType::from_str(type_name)")?;
    printer.dedent();
    printer.println("}")?;
    printer.newline()?;

    emit_lookup_xml_tables(printer, vim_model)?;

    Ok(())
}

/// XSD / SOAP primitive names and `Any` wrapper types (`ArrayOf*`, `Primitive*`) for `xsi:type`.
fn emit_lookup_xml_tables(printer: &mut dyn Printer, model: &Model) -> Result<()> {
    printer.println("/// XSD primitive local names (`string`, `int`, …) mapped to [`ApiFieldType`].")?;
    printer.println("fn lookup_xml_primitive(type_name: &str) -> Option<ApiFieldType> {")?;
    printer.indent();
    printer.println("match type_name {")?;
    printer.indent();
    // Canonical XSD names (local part only — prefixes stripped before lookup).
    let mut prim: Vec<(&str, &str)> = vec![
        ("string", "str"),
        ("boolean", "bool"),
        ("byte", "i8"),
        ("short", "i16"),
        ("int", "i32"),
        ("long", "i64"),
        ("unsignedByte", "i16"),
        ("unsignedShort", "i32"),
        ("unsignedInt", "i64"),
        ("unsignedLong", "i64"),
        ("float", "f32"),
        ("double", "f64"),
        ("decimal", "str"),
        ("dateTime", "str"),
        ("base64Binary", "binary"),
        ("anyType", "any"),
    ];
    prim.sort_by(|a, b| a.0.cmp(b.0));
    for (xml_name, sig) in prim {
        let sym = static_name_for_sig(sig);
        printer.println(&format!("\"{xml_name}\" => Some({sym}),"))?;
    }
    printer.println("_ => None,")?;
    printer.dedent();
    printer.println("}")?;
    printer.dedent();
    printer.println("}")?;
    printer.newline()?;

    // Stable iteration for deterministic codegen.
    let mut boxed_keys: Vec<String> = model.any_value_types.keys().cloned().collect();
    boxed_keys.sort();

    let mut wrapper_entries: Vec<(String, String)> = Vec::new();
    for k in &boxed_keys {
        let bt = model.any_value_types.get(k).expect("boxed key");
        let sym = data_type_to_expr(&bt.property_type, model)?;
        wrapper_entries.push((k.clone(), sym));
    }

    let mut wrapper_map = PhfMapBuilder::new();
    for (key, value) in &wrapper_entries {
        wrapper_map.entry(key, value);
    }

    printer.println(
        "/// `Primitive*` / `ArrayOf*` boxed value types from the schema (`Any` descendants). O(1) lookup.",
    )?;
    printer.println(&format!(
        "static ANY_VALUE_WRAPPER_MAP: phf::Map<&'static str, ApiFieldType> = {};",
        wrapper_map.build()
    ))?;
    printer.newline()?;

    printer.println("#[inline]")?;
    printer.println("fn lookup_any_value_wrapper(type_name: &str) -> Option<ApiFieldType> {")?;
    printer.indent();
    printer.println("ANY_VALUE_WRAPPER_MAP.get(type_name).copied()")?;
    printer.dedent();
    printer.println("}")?;
    printer.newline()?;

    printer.println(
        "/// Resolve `xsi:type` local names (after stripping namespace prefixes) to an [`ApiFieldType`].",
    )?;
    printer.println(
        "/// Resolution order: [`StructType`] first, then boxed `Any` value types (PHF), then XSD primitives.",
    )?;
    printer.println("pub fn lookup_xml_type(type_name: &str) -> Option<ApiFieldType> {")?;
    printer.indent();
    printer.println("resolve_type(type_name)")?;
    printer.println("    .map(ApiFieldType::Object)")?;
    printer.println("    .or_else(|| lookup_any_value_wrapper(type_name))")?;
    printer.println("    .or_else(|| lookup_xml_primitive(type_name))")?;
    printer.dedent();
    printer.println("}")?;

    Ok(())
}

fn count_struct_variants(vim_model: &Model) -> usize {
    vim_model
        .structs
        .iter()
        .filter(|(_, dt)| dt.borrow().rust_name() != ANY)
        .count()
}

fn collect_sigs(dt: &DataType, model: &Model, out: &mut BTreeSet<String>) {
    out.insert(data_type_sig(dt, model));
    if let DataType::Array(inner) = dt {
        collect_sigs(inner, model, out);
    }
}

fn data_type_sig(dt: &DataType, model: &Model) -> String {
    match dt {
        DataType::Boolean => "bool".to_string(),
        DataType::String | DataType::DateTime => "str".to_string(),
        DataType::Int8 => "i8".to_string(),
        DataType::Int16 => "i16".to_string(),
        DataType::Int32 => "i32".to_string(),
        DataType::Int64 => "i64".to_string(),
        DataType::Float => "f32".to_string(),
        DataType::Double => "f64".to_string(),
        DataType::Binary => "binary".to_string(),
        DataType::Reference(name) if name == "Any" => "any".to_string(),
        DataType::Reference(name) if model.enums.contains_key(name) => "str".to_string(),
        DataType::Reference(name) if model.structs.contains_key(name) => {
            format!("obj:{name}")
        }
        DataType::Reference(_) => "str".to_string(),
        DataType::Array(inner) => format!("arr:{}", data_type_sig(inner, model)),
    }
}

fn sig_rank(sig: &str) -> usize {
    match sig.strip_prefix("arr:") {
        Some(rest) => 1 + sig_rank(rest),
        None => 0,
    }
}

fn topo_sort_sigs(sigs: BTreeSet<String>) -> Vec<String> {
    let mut v: Vec<String> = sigs.into_iter().collect();
    v.sort_by(|a, b| sig_rank(a).cmp(&sig_rank(b)).then_with(|| a.cmp(b)));
    v
}

fn static_name_for_sig(sig: &str) -> String {
    let normalized: String = sig
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    format!("AFT_{}", normalized.as_str().to_case(Case::UpperSnake))
}

fn emit_static_for_sig(printer: &mut dyn Printer, sig: &str, model: &Model) -> Result<()> {
    let name = static_name_for_sig(sig);
    let rhs = sig_to_api_field_expr(sig, model)?;
    printer.println(&format!("static {name}: ApiFieldType = {rhs};"))?;
    Ok(())
}

fn sig_to_api_field_expr(sig: &str, model: &Model) -> Result<String> {
    let expr = match sig {
        "bool" => "ApiFieldType::Bool".to_string(),
        "str" => "ApiFieldType::Str".to_string(),
        "i8" => "ApiFieldType::I8".to_string(),
        "i16" => "ApiFieldType::I16".to_string(),
        "i32" => "ApiFieldType::I32".to_string(),
        "i64" => "ApiFieldType::I64".to_string(),
        "f32" => "ApiFieldType::F32".to_string(),
        "f64" => "ApiFieldType::F64".to_string(),
        "binary" => "ApiFieldType::Binary".to_string(),
        "any" => "ApiFieldType::Any".to_string(),
        s if s.starts_with("obj:") => {
            let schema_name = &s[4..];
            let st = model
                .structs
                .get(schema_name)
                .unwrap_or_else(|| panic!("registry: missing struct {schema_name}"));
            let rust = st.borrow().rust_name();
            format!("ApiFieldType::Object(StructType::{rust})")
        }
        s if s.starts_with("arr:") => {
            let inner_sig = &s[4..];
            let inner_static = static_name_for_sig(inner_sig);
            format!("ApiFieldType::Array(&{inner_static})")
        }
        _ => panic!("registry: unknown sig {sig}"),
    };
    Ok(expr)
}

fn data_type_to_expr(dt: &DataType, model: &Model) -> Result<String> {
    let sig = data_type_sig(dt, model);
    let name = static_name_for_sig(&sig);
    Ok(name)
}

fn emit_api_type_info(printer: &mut dyn Printer, s: &Struct, model: &Model) -> Result<()> {
    let parent_expr = match &s.parent {
        Some(p) if p != "Any" => {
            let parent = model
                .structs
                .get(p)
                .unwrap_or_else(|| panic!("registry: parent struct {} missing", p));
            let rust = parent.borrow().rust_name();
            format!("Some(StructType::{rust})")
        }
        _ => "None".to_string(),
    };

    printer.println("ApiTypeInfo {")?;
    printer.indent();
    printer.println(&format!("parent: {parent_expr},"))?;
    printer.println("fields: &[")?;
    printer.indent();
    for (_, field) in &s.fields {
        let fname = &field.name;
        let static_ref = data_type_to_expr(&field.vim_type, model)?;
        printer.println(&format!("(\"{fname}\", {static_ref}),"))?;
    }
    printer.dedent();
    printer.println("],")?;
    printer.dedent();
    printer.println("},")?;
    Ok(())
}
