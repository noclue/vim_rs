use crate::json_emitter::common::*;
use crate::json_emitter::signature_generator;
use crate::vim_model::{Model, Method, DataType};
use crate::rs_emitter::names::to_fn_name;
use std::path::Path;
use chrono::Utc;

pub fn emit_managed_objects_json(
    model: &Model,
    output_dir: &Path,
) -> super::Result<()> {
    let mut managed_objects = Vec::new();

    for (mo_name, mo) in &model.managed_objects {
        if mo.methods.is_empty() {
            continue; // Skip MOs without methods
        }

        let mut methods = Vec::new();
        for method in &mo.methods {
            let signature = signature_generator::generate_method_signature(
                method, mo_name, model
            );

            let related_types = extract_related_types(method);

            methods.push(MethodEntry {
                name: method.name.clone(),
                rust_name: to_fn_name(&method.name),
                signature,
                description: method.description.clone(),  // Raw markdown - no parsing!
                related_types,
            });
        }

        managed_objects.push(ManagedObjectEntry {
            name: mo_name.clone(),
            rust_module: format!("vim_rs::mo::{}", mo_name),
            rust_struct: mo_name.clone(),
            description: mo.description.clone(),
            methods,
        });
    }

    let output = ManagedObjectsOutput {
        generated_at: Utc::now(),
        source: "vim_model processed from OpenAPI specification".to_string(),
        managed_objects,
    };

    let output_path = output_dir.join("managed_objects.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    println!("Generated: {}", output_path.display());
    Ok(())
}

fn extract_related_types(method: &Method) -> Vec<String> {
    let mut types = Vec::new();

    // Add input types
    if let Some(DataType::Reference(type_name)) = &method.input {
        types.push(type_name.clone());
    }

    // Add output types
    if let Some(DataType::Reference(type_name)) = &method.output {
        types.push(type_name.clone());
    }

    types.sort();
    types.dedup();
    types
}
