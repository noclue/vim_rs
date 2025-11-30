use api_database::*;
use crate::api_builder::signature_generator;
use vim_build::vim_model::{Model, Method, DataType};
use vim_build::rs_emitter::names::to_fn_name;
use std::path::Path;
use chrono::Utc;
use tracing::info;

/// Build managed objects in memory (no file I/O).
pub fn build_managed_objects(model: &Model) -> Vec<ManagedObjectEntry> {
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


            methods.push(MethodEntry {
                name: method.name.clone(),
                rust_name: to_fn_name(&method.name),
                signature,
                description: method.description.clone(),  // Raw markdown - no parsing!
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

    managed_objects
}

/// Write managed objects to JSON file (for debugging).
pub fn emit_managed_objects_json(
    model: &Model,
    output_dir: &Path,
) -> super::Result<()> {
    let managed_objects = build_managed_objects(model);

    let output = ManagedObjectsOutput {
        generated_at: Utc::now(),
        source: "vim_model processed from OpenAPI specification".to_string(),
        managed_objects,
    };

    let output_path = output_dir.join("managed_objects.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    info!("Generated: {}", output_path.display());
    Ok(())
}

