use vim_mcp_server::*;
use vim_build::vim_model::Model;
use vim_build::rs_emitter::names::to_enum_variant;
use std::path::Path;
use chrono::Utc;
use tracing::info;

pub fn emit_enumerations_json(
    model: &Model,
    output_dir: &Path,
) -> super::Result<()> {
    let mut enumerations = Vec::new();

    for (name, enum_def) in &model.enums {
        let rust_name = name.trim_end_matches("_enum").to_string() + "Enum";

        let variants = enum_def.variants.iter().map(|v| {
            VariantEntry {
                name: v.clone(),
                rust_name: to_enum_variant(v),
                description: None,  // Can enhance later
                discriminator_value: enum_def.discriminator_value.as_ref()
                    .unwrap_or(v)
                    .clone(),
            }
        }).collect();

        enumerations.push(EnumerationEntry {
            name: name.clone(),
            rust_name: rust_name,
            rust_module: "vim_rs::types::enums".to_string(),
            description: enum_def.description.clone(),
            variants,
        });
    }

    let output = EnumerationsOutput {
        generated_at: Utc::now(),
        source: "vim_model processed from OpenAPI specification".to_string(),
        enumerations,
    };

    let output_path = output_dir.join("enumerations.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    info!("Generated: {}", output_path.display());
    Ok(())
}

