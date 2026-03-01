use api_database::*;
use vim_build::vim_model::{Model, EmitMode, DataType, Struct, TypePath, PathOrigin, PathStep};
use vim_build::rs_emitter::names::{TypeDefResolver, parent_field_name, to_type_name as rust_type_name};
use std::path::Path;
use chrono::Utc;
use tracing::info;
use check_keyword::CheckKeyword;
use convert_case::{Case, Casing};

/// Convert a vim_build TypePath to an api_database ApiTypePath.
fn convert_type_path(path: &TypePath) -> ApiTypePath {
    let origin = match &path.origin {
        PathOrigin::PropertyAccessor { managed_object, property_name } => {
            ApiPathOrigin::PropertyAccessor {
                managed_object: managed_object.clone(),
                property_name: to_snake_case(property_name),
            }
        }
        PathOrigin::MethodOutput { managed_object, method_name } => {
            ApiPathOrigin::MethodOutput {
                managed_object: managed_object.clone(),
                method_name: to_snake_case(method_name),
            }
        }
        PathOrigin::MethodInput { managed_object, method_name, parameter_name } => {
            ApiPathOrigin::MethodInput {
                managed_object: managed_object.clone(),
                method_name: to_snake_case(method_name),
                parameter_name: to_snake_case(parameter_name),
            }
        }
    };

    let steps = path.steps.iter().map(|step| {
        match step {
            PathStep::Field { field_name, is_optional, is_array, .. } => {
                ApiPathStep::Field {
                    field_name: to_snake_case(field_name),
                    is_optional: *is_optional,
                    is_array: *is_array,
                }
            }
            PathStep::Downcast { to_type, is_trait_cast } => {
                ApiPathStep::Downcast {
                    to_type: to_type.clone(),
                    is_trait_cast: *is_trait_cast,
                }
            }
        }
    }).collect();

    ApiTypePath { origin, steps }
}

/// Convert a camelCase or PascalCase string to snake_case.
fn to_snake_case(s: &str) -> String {
    s.to_case(Case::Snake).into_safe()
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

/// Build structures in memory (no file I/O).
pub fn build_structures(model: &Model) -> Vec<StructureEntry> {
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
        for (_field_name, field) in &s.fields {
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

        // Insert parent field at position 0 if struct has a useful parent (compositional inheritance)
        if let Some(parent_name) = &s.parent {
            if parent_name != "Any" {
                if model.has_useful_parent(name).unwrap_or(false) {
                    let parent_struct_name = rust_type_name(parent_name);
                    let parent_field = parent_field_name(parent_name);
                    let parent_type = format!("vim_rs::types::structs::{}", parent_struct_name);
                    fields.insert(
                        0,
                        FieldEntry {
                            name: parent_field,
                            rust_type: parent_type,
                            vim_type: parent_name.clone(),
                            required: true,
                            description: Some(format!(
                                "Parent struct (compositional inheritance). \
                                 Fields from {} are accessible directly via Deref coercion.",
                                parent_struct_name
                            )),
                            is_array: false,
                            is_boxed: false,
                            is_trait: false,
                            trait_name: None,
                            is_parent_field: true,
                        },
                    );
                }
            }
        }

        // Extract related types from fields
        let related_types = extract_related_types(&s);

        // Build inheritance chain
        let inheritance_chain = build_inheritance_chain(model, name);

        // Collect implemented traits (traits from all ancestors that have children)
        let implements_traits = collect_implemented_traits(model, name);

        // Collect all descendants recursively
        let all_descendants = collect_all_descendants(model, name);

        // Convert paths from vim_build format to api_database format
        let paths: Vec<ApiTypePath> = s.paths.iter().map(convert_type_path).collect();

        // Build inherited field groups (only for Emit mode - Skip types have no Rust struct)
        let inherited_field_groups = build_inherited_field_groups(model, name, &tdf);

        // Build extra_fields and pruned_parent for Skip emit mode
        let (extra_fields, pruned_parent, pruned_parent_fields, pruned_parent_inherited_groups) =
            build_skip_mode_extra(model, name, &tdf);

        structures.push(StructureEntry {
            name: s.rust_name(),
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
            paths,
            inherited_field_groups,
            extra_fields,
            pruned_parent,
            pruned_parent_fields,
            pruned_parent_inherited_groups,
        });
    }

    structures
}

/// Write data structures to JSON file (for debugging).
pub fn emit_data_structures_json(
    model: &Model,
    output_dir: &Path,
) -> super::Result<()> {
    let structures = build_structures(model);

    let output = DataStructuresOutput {
        generated_at: Utc::now(),
        source: "vim_model processed from OpenAPI specification".to_string(),
        structures,
    };

    let output_path = output_dir.join("data_structures.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    info!("Generated: {}", output_path.display());
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

fn build_inherited_field_groups(
    model: &Model,
    name: &str,
    tdf: &TypeDefResolver<'_>,
) -> Vec<InheritedFieldGroup> {
    let mut groups = Vec::new();
    // Skip for Skip emit mode - no Rust struct
    let current_struct = match model.structs.get(name) {
        Some(r) => r.borrow(),
        None => return groups,
    };
    if matches!(current_struct.emit_mode, EmitMode::Skip(_)) {
        return groups;
    }

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
        // Skip ancestors that are pruned or skipped - their fields are in extra_fields on the pruned type
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

fn build_skip_mode_extra(
    model: &Model,
    name: &str,
    tdf: &TypeDefResolver<'_>,
) -> (Vec<ExtraFieldEntry>, Option<String>, Vec<FieldEntry>, Vec<InheritedFieldGroup>) {
    let s = match model.structs.get(name) {
        Some(r) => r.borrow(),
        None => return (Vec::new(), None, Vec::new(), Vec::new()),
    };

    let EmitMode::Skip(pruned_parent_name) = &s.emit_mode else {
        return (Vec::new(), None, Vec::new(), Vec::new());
    };

    let pruned_parent = rust_type_name(pruned_parent_name);
    let mut extra_fields = Vec::new();

    // Collect fields from this Skip type and its ancestors up to (but not including) the pruned parent
    let mut current = name.to_string();
    while current != *pruned_parent_name {
        let struct_ref = match model.structs.get(&current) {
            Some(r) => r,
            None => break,
        };
        let struct_borrow = struct_ref.borrow();
        for (_field_name, field) in &struct_borrow.fields {
            let rust_type = match tdf.field_type(field) {
                Ok(t) => t,
                Err(_) => "UnknownType".to_string(),
            };
            extra_fields.push(ExtraFieldEntry {
                vim_name: field.name.clone(),
                vim_type: format_vim_type(&field.vim_type),
                rust_type,
                description: field.description.clone(),
            });
        }
        current = match &struct_borrow.parent {
            Some(p) if p != "Any" => p.clone(),
            _ => break,
        };
    }

    // Denormalize the pruned parent's own fields and inherited field groups
    let pruned_parent_fields = build_fields_for(model, pruned_parent_name, tdf);
    let pruned_parent_inherited = build_inherited_field_groups(model, pruned_parent_name, tdf);

    (extra_fields, Some(pruned_parent), pruned_parent_fields, pruned_parent_inherited)
}

/// Build the field list for a given struct name (used to denormalize pruned parent fields).
fn build_fields_for(
    model: &Model,
    name: &str,
    tdf: &TypeDefResolver<'_>,
) -> Vec<FieldEntry> {
    let struct_ref = match model.structs.get(name) {
        Some(r) => r,
        None => return Vec::new(),
    };
    let s = struct_ref.borrow();
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

    fields
}

fn collect_all_descendants(model: &Model, name: &str) -> Vec<String> {
    let mut descendants = Vec::new();

    let is_pruned = model
        .structs
        .get(name)
        .map(|s| matches!(s.borrow().emit_mode, EmitMode::Prune))
        .unwrap_or(false);

    if let Ok(children) = model.children(&name.to_string()) {
        for child in children {
            let child_borrow = child.borrow();
            // Include Skip types when parent is Pruned; otherwise exclude them
            if is_pruned || !child_borrow.emit_mode.is_skip() {
                let child_name = child_borrow.name.clone();
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


