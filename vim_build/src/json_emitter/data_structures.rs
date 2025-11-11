use crate::json_emitter::common::*;
use crate::vim_model::{Model, EmitMode, DataType, Struct};
use crate::rs_emitter::names::TypeDefResolver;
use std::path::Path;
use chrono::Utc;

fn format_vim_type(dt: &DataType) -> String {
    match dt {
        DataType::Boolean => "Boolean".to_string(),
        DataType::String => "String".to_string(),
        DataType::Int8 => "Int8".to_string(),
        DataType::Int16 => "Int16".to_string(),
        DataType::Int32 => "Int32".to_string(),
        DataType::Int64 => "Int64".to_string(),
        DataType::Float => "Float".to_string(),
        DataType::Double => "Double".to_string(),
        DataType::DateTime => "DateTime".to_string(),
        DataType::Binary => "Binary".to_string(),
        DataType::Array(inner) => format!("Array<{}>", format_vim_type(inner)),
        DataType::Reference(name) => name.clone(),
    }
}

pub fn emit_data_structures_json(
    model: &Model,
    output_dir: &Path,
) -> super::Result<()> {
    let tdf = TypeDefResolver::new_with_root_package(model, "vim_rs::types".to_string());
    let mut structures = Vec::new();

    for (name, struct_ref) in &model.structs {
        let s = struct_ref.borrow();

        let emit_mode_str = match &s.emit_mode {
            EmitMode::Emit => "Emit",
            EmitMode::Prune => "Prune",
            EmitMode::Skip(_) => "Skip",
        };

        let skip_reason = match &s.emit_mode {
            EmitMode::Skip(parent) => Some(format!("Descendant of pruned type {}", parent)),
            _ => None,
        };

        let mut fields = Vec::new();
        for (field_name, field) in &s.fields {
            let rust_type = match tdf.field_type(field) {
                Ok(t) => t,
                Err(_) => "UnknownType".to_string(),
            };

            let is_array = matches!(&field.vim_type, DataType::Array(_));

            // Check if type is a trait (has children)
            let (is_trait, trait_name) = if let DataType::Reference(ref_name) = &field.vim_type {
                if let Some(ref_struct) = model.structs.get(ref_name) {
                    let rs = ref_struct.borrow();
                    let has_children = !rs.children.is_empty() && matches!(rs.emit_mode, EmitMode::Emit);
                    if has_children {
                        (true, Some(format!("{}Trait", rs.rust_name())))
                    } else {
                        (false, None)
                    }
                } else {
                    (false, None)
                }
            } else {
                (false, None)
            };

            fields.push(FieldEntry {
                name: field_name.clone(),
                rust_name: field.rust_name(),
                rust_type,
                vim_type: format_vim_type(&field.vim_type),
                required: !field.optional,
                description: field.description.clone(),
                is_array,
                is_boxed: field.require_box,
                is_trait,
                trait_name,
            });
        }

        // Extract related types from fields
        let related_types = extract_related_types(&s);

        // Build inheritance chain
        let inheritance_chain = build_inheritance_chain(model, name);

        // Collect implemented traits (traits from all ancestors that have children)
        let implements_traits = collect_implemented_traits(model, name);

        // Collect all descendants recursively
        let all_descendants = collect_all_descendants(model, name);

        structures.push(StructureEntry {
            name: name.clone(),
            rust_name: s.rust_name(),
            rust_module: "vim_rs::types::structs".to_string(),
            description: s.description.clone(),
            parent: s.parent.clone(),
            children: s.children.clone(),
            emit_mode: emit_mode_str.to_string(),
            skip_reason,
            fields,
            related_types,
            inheritance_chain,
            implements_traits,
            all_descendants,
        });
    }

    let output = DataStructuresOutput {
        generated_at: Utc::now(),
        source: "vim_model processed from OpenAPI specification".to_string(),
        structures,
    };

    let output_path = output_dir.join("data_structures.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    println!("Generated: {}", output_path.display());
    Ok(())
}

fn extract_related_types(s: &Struct) -> Vec<String> {
    let mut types = Vec::new();

    // Add parent
    if let Some(parent) = &s.parent {
        types.push(parent.clone());
    }

    // Add types from fields
    for field in s.fields.values() {
        if let DataType::Reference(type_name) = &field.vim_type {
            types.push(type_name.clone());
        } else if let DataType::Array(box_type) = &field.vim_type {
            if let DataType::Reference(type_name) = box_type.as_ref() {
                types.push(type_name.clone());
            }
        }
    }

    types.sort();
    types.dedup();
    types
}

fn build_inheritance_chain(model: &Model, name: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut current = name.to_string();

    chain.push(current.clone());

    while let Some(struct_ref) = model.structs.get(&current) {
        let s = struct_ref.borrow();
        if let Some(parent) = &s.parent {
            chain.insert(0, parent.clone());
            current = parent.clone();
        } else {
            break;
        }
    }

    chain
}

fn collect_implemented_traits(model: &Model, name: &str) -> Vec<String> {
    let mut traits = Vec::new();
    let mut current = name.to_string();

    // Walk up the inheritance chain and collect traits from ancestors that have children
    while let Some(struct_ref) = model.structs.get(&current) {
        let s = struct_ref.borrow();

        // If this type has children and is emitted, it has a trait
        if s.has_children() && matches!(s.emit_mode, EmitMode::Emit) && current != "Any" {
            traits.push(format!("{}Trait", s.rust_name()));
        }

        // Move to parent
        if let Some(parent) = &s.parent {
            if parent == "Any" {
                break;
            }
            current = parent.clone();
        } else {
            break;
        }
    }

    traits.reverse(); // Put most general trait first
    traits
}

fn collect_all_descendants(model: &Model, name: &str) -> Vec<String> {
    let mut descendants = Vec::new();

    if let Ok(children) = model.children(&name.to_string()) {
        for child in children {
            let child_borrow = child.borrow();
            // Only include non-skipped types
            if !child_borrow.emit_mode.is_skip() {
                let child_name = child_borrow.name.clone();
                // Don't include the parent itself
                if child_name != name {
                    descendants.push(child_name);
                }
            }
        }
    }

    descendants.sort();
    descendants.dedup();
    descendants
}
