use api_database::*;
use vim_build::vim_model::Model;
use vim_build::rs_emitter::names::to_enum_variant;
use std::path::Path;
use chrono::Utc;
use tracing::info;
use check_keyword::CheckKeyword;
use convert_case::{Case, Casing};


/// Build enumerations in memory (no file I/O).
pub fn build_enumerations(model: &Model) -> Vec<EnumerationEntry> {
    let mut enumerations = Vec::new();

    for (name, enum_def) in &model.enums {
        let rust_name = name.trim_end_matches("_enum").to_case(Case::Pascal).into_safe() + "Enum";


        let mut variants: Vec<VariantEntry> = enum_def.variants.iter().map(|v| {
            VariantEntry {
                name: to_enum_variant(v),
                description: None,  // Can enhance later
                discriminator_value: enum_def.discriminator_value.as_ref()
                    .unwrap_or(v)
                    .clone(),
            }
        }).collect();

        // Add the catch-all Other_ variant present on all vim_rs enums
        variants.push(VariantEntry {
            name: "Other_".to_string(),
            description: Some("Catch-all for values not known at compile time. Holds the raw string value.".to_string()),
            discriminator_value: "(runtime value)".to_string(),
        });

        enumerations.push(EnumerationEntry {
            name: rust_name,
            rust_module: "vim_rs::types::enums".to_string(),
            description: enum_def.description.clone(),
            variants,
        });
    }

    enumerations
}

/// Write enumerations to JSON file (for debugging).
pub fn emit_enumerations_json(
    model: &Model,
    output_dir: &Path,
) -> super::Result<()> {
    let enumerations = build_enumerations(model);

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

