use api_database::{TraitsOutput, TraitEntry, TraitDerefTarget, FieldEntry, InheritedFieldGroup};
use vim_build::vim_model::{Model, EmitMode, DataType};
use vim_build::rs_emitter::names::{TypeDefResolver, parent_field_name};
use std::path::Path;
use chrono::Utc;
use tracing::info;

/// Build traits in memory (no file I/O).
pub fn build_traits(model: &Model) -> Vec<TraitEntry> {
    let tdf = TypeDefResolver::new_with_root_package(model, "vim_rs::types".to_string());
    let mut traits = Vec::new();

    // Iterate through all structs that have children (i.e., have traits)
    for (name, struct_ref) in &model.structs {
        let s = struct_ref.borrow();

        // Only emit traits for structs that have children and are emitted
        if !s.has_children() || !matches!(s.emit_mode, EmitMode::Emit) {
            continue;
        }

        // Skip Any type
        if name == "Any" {
            continue;
        }

        // Build deref_target: fields accessible via Deref (no per-field getters exist)
        let mut fields = Vec::new();
        for (_field_name, field) in &s.fields {
            let rust_type = match tdf.field_type(field) {
                Ok(t) => t,
                Err(_) => "UnknownType".to_string(),
            };
            let is_array = matches!(&field.vim_type, DataType::Array(_));
            let (is_trait, trait_name) = if let DataType::Reference(ref_name) = &field.vim_type {
                if let Some(ref_struct) = model.structs.get(ref_name) {
                    let rs = ref_struct.borrow();
                    let has_children =
                        !rs.children.is_empty() && matches!(rs.emit_mode, EmitMode::Emit);
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
                name: field.rust_name(),
                rust_type,
                vim_type: format_vim_type(&field.vim_type),
                required: !field.optional,
                description: field.description.clone(),
                is_array,
                is_boxed: field.require_box,
                is_trait,
                trait_name,
                is_parent_field: false,
            });
        }

        let deref_target = Some(TraitDerefTarget {
            struct_name: s.rust_name(),
            fields,
        });

        // Collect all implementing types (all descendants)
        let implementing_types = collect_implementing_types(model, name);

        // Collect all descendants recursively
        let all_descendants = collect_all_descendants(model, name);

        // Determine parent trait
        let parent_trait = s.parent.as_ref()
            .filter(|p| *p != "Any")
            .map(|p| format!("{}Trait", to_type_name(p)));

        // Build inherited field groups from parent trait chain
        let inherited_field_groups = build_inherited_field_groups(model, name, &tdf);

        traits.push(TraitEntry {
            name: format!("{}Trait", s.rust_name()),
            rust_module: "vim_rs::types::traits".to_string(),
            description: s.description.clone(),
            parent_trait,
            getters: Vec::new(),
            deref_target,
            implementing_types,
            all_descendants,
            inherited_field_groups,
        });
    }

    traits
}

/// Write traits to JSON file (for debugging).
pub fn emit_traits_json(
    model: &Model,
    output_dir: &Path,
) -> super::Result<()> {
    let traits = build_traits(model);

    let output = TraitsOutput {
        generated_at: Utc::now(),
        source: "vim_model processed from OpenAPI specification".to_string(),
        traits,
    };

    let output_path = output_dir.join("traits.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    info!("Generated: {}", output_path.display());
    Ok(())
}

fn build_inherited_field_groups(
    model: &Model,
    name: &str,
    tdf: &TypeDefResolver<'_>,
) -> Vec<InheritedFieldGroup> {
    let mut groups = Vec::new();
    let mut path_prefix = String::new();
    let mut current = name.to_string();

    loop {
        let parent_name = match model.structs.get(&current) {
            Some(r) => r.borrow().parent.clone(),
            None => break,
        };
        let parent_name = match parent_name {
            Some(p) if p != "Any" => p,
            _ => break,
        };
        let parent_struct = match model.structs.get(&parent_name) {
            Some(r) => r.borrow(),
            None => break,
        };
        if matches!(parent_struct.emit_mode, EmitMode::Prune | EmitMode::Skip(_)) {
            break;
        }

        let segment = if path_prefix.is_empty() {
            parent_field_name(&parent_name)
        } else {
            format!("{}.{}", path_prefix, parent_field_name(&parent_name))
        };
        path_prefix = segment.clone();

        let mut ancestor_fields = Vec::new();
        for (_field_name, field) in &parent_struct.fields {
            let rust_type = match tdf.field_type(field) {
                Ok(t) => t,
                Err(_) => "UnknownType".to_string(),
            };
            let is_array = matches!(&field.vim_type, DataType::Array(_));
            let (is_trait, trait_name) = if let DataType::Reference(ref_name) = &field.vim_type {
                if let Some(ref_struct) = model.structs.get(ref_name) {
                    let rs = ref_struct.borrow();
                    let has_children =
                        !rs.children.is_empty() && matches!(rs.emit_mode, EmitMode::Emit);
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

            ancestor_fields.push(FieldEntry {
                name: field.rust_name(),
                rust_type,
                vim_type: format_vim_type(&field.vim_type),
                required: !field.optional,
                description: field.description.clone(),
                is_array,
                is_boxed: field.require_box,
                is_trait,
                trait_name,
                is_parent_field: false,
            });
        }

        if !ancestor_fields.is_empty() {
            groups.push(InheritedFieldGroup {
                source_type: parent_struct.rust_name(),
                path_prefix: path_prefix.clone(),
                fields: ancestor_fields,
            });
        }

        current = parent_name.clone();
    }

    groups
}

fn collect_implementing_types(model: &Model, struct_name: &str) -> Vec<String> {
    let mut types = Vec::new();

    if let Ok(children) = model.children(&struct_name.to_string()) {
        for child in children {
            let child_borrow = child.borrow();
            if matches!(child_borrow.emit_mode, EmitMode::Emit | EmitMode::Prune) {
                types.push(child_borrow.name.clone());
            }
        }
    }

    types.sort();
    types
}

fn collect_all_descendants(model: &Model, struct_name: &str) -> Vec<String> {
    let mut descendants = Vec::new();

    let is_pruned = model
        .structs
        .get(struct_name)
        .map(|s| matches!(s.borrow().emit_mode, EmitMode::Prune))
        .unwrap_or(false);

    if let Ok(children) = model.children(&struct_name.to_string()) {
        for child in children {
            let child_borrow = child.borrow();
            if is_pruned || !child_borrow.emit_mode.is_skip() {
                let child_name = child_borrow.name.clone();
                if child_name != struct_name {
                    descendants.push(child_name);
                }
            }
        }
    }

    descendants.sort();
    descendants.dedup();
    descendants
}

fn to_type_name(name: &str) -> String {
    use convert_case::{Case, Casing};
    use check_keyword::CheckKeyword;
    name.to_case(Case::Pascal).into_safe()
}

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

