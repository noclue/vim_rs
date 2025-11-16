use vim_mcp_server::{TraitsOutput, TraitEntry, GetterEntry};
use vim_build::vim_model::{Model, EmitMode, Field};
use vim_build::rs_emitter::names::TypeDefResolver;
use std::path::Path;
use chrono::Utc;
use tracing::info;

pub fn emit_traits_json(
    model: &Model,
    output_dir: &Path,
) -> super::Result<()> {
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

        // Build getter methods from fields
        let mut getters = Vec::new();
        for (field_name, field) in &s.fields {
            let getter_name = format!("get_{}", to_field_name(field_name));
            let return_type = match getter_return_type(&tdf, field) {
                Ok(t) => t,
                Err(_) => "UnknownType".to_string(),
            };

            getters.push(GetterEntry {
                name: getter_name,
                return_type,
                description: field.description.clone(),
                field_name: to_field_name(field_name),
            });
        }

        // Collect all implementing types (all descendants)
        let implementing_types = collect_implementing_types(model, name);

        // Collect all descendants recursively
        let all_descendants = collect_all_descendants(model, name);

        // Determine parent trait
        let parent_trait = s.parent.as_ref()
            .filter(|p| *p != "Any")
            .map(|p| format!("{}Trait", to_type_name(p)));

        traits.push(TraitEntry {
            name: name.clone(),
            rust_name: format!("{}Trait", s.rust_name()),
            rust_module: "vim_rs::types::traits".to_string(),
            description: s.description.clone(),
            parent_trait,
            getters,
            implementing_types,
            all_descendants,
        });
    }

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

    if let Ok(children) = model.children(&struct_name.to_string()) {
        for child in children {
            let child_borrow = child.borrow();
            // Only include non-skipped types
            if !child_borrow.emit_mode.is_skip() {
                let child_name = child_borrow.name.clone();
                // Don't include the parent itself
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

fn to_field_name(name: &str) -> String {
    use convert_case::{Case, Casing};
    use check_keyword::CheckKeyword;
    name.to_case(Case::Snake).into_safe()
}

fn to_type_name(name: &str) -> String {
    use convert_case::{Case, Casing};
    use check_keyword::CheckKeyword;
    name.to_case(Case::Pascal).into_safe()
}

fn getter_return_type(tdf: &TypeDefResolver, property: &Field) -> Result<String, Box<dyn std::error::Error>> {
    let mut field_type = tdf.field_type(property)?;

    if get_by_ref(&property.vim_type) {
        field_type = format!("&{field_type}");
    }

    if field_type == "&String" {
        field_type = "&str".to_string();
    }

    Ok(field_type)
}

fn get_by_ref(vim_type: &vim_build::vim_model::DataType) -> bool {
    use vim_build::vim_model::DataType;
    matches!(vim_type, DataType::String | DataType::Binary | DataType::Array(_))
}

