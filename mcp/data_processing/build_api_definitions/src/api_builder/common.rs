use api_database::{FieldEntry, InheritedFieldGroup};
use vim_build::vim_model::{Model, EmitMode, DataType};
use vim_build::rs_emitter::names::{TypeDefResolver, parent_field_name};

pub fn format_vim_type(dt: &DataType) -> String {
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

pub fn collect_all_descendants(model: &Model, name: &str) -> Vec<String> {
    let mut descendants = Vec::new();

    let is_pruned = model
        .structs
        .get(name)
        .map(|s| matches!(s.borrow().emit_mode, EmitMode::Prune))
        .unwrap_or(false);

    if let Ok(children) = model.children(&name.to_string()) {
        for child in children {
            let child_borrow = child.borrow();
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

/// Walk the parent chain starting from `name` and collect inherited field groups.
///
/// Stops when it reaches `Any`, a missing struct, or a Prune/Skip ancestor.
/// Callers that need to bail out early for Skip-mode structs should do so
/// *before* calling this function (see `data_structures.rs`).
pub fn build_inherited_field_groups(
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
