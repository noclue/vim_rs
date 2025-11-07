use crate::json_emitter::common::*;
use crate::vim_model::{Model, EmitMode};
use std::path::Path;
use std::time::Duration;
use chrono::Utc;

pub fn emit_metadata_json(
    model: &Model,
    output_dir: &Path,
    generation_duration: Duration,
    pruned_types: &[&str],
) -> super::Result<()> {
    let output = MetadataOutput {
        generated_at: Utc::now(),
        source: "vim_model processed from OpenAPI specification".to_string(),
        statistics: Statistics {
            managed_objects: model.managed_objects.len(),
            total_methods: model.managed_objects.values()
                .map(|mo| mo.methods.len())
                .sum(),
            data_structures_total: model.structs.len(),
            data_structures_emitted: model.structs.values()
                .filter(|s| matches!(s.borrow().emit_mode, EmitMode::Emit))
                .count(),
            data_structures_pruned: model.structs.values()
                .filter(|s| matches!(s.borrow().emit_mode, EmitMode::Prune))
                .count(),
            data_structures_skipped: model.structs.values()
                .filter(|s| matches!(s.borrow().emit_mode, EmitMode::Skip(_)))
                .count(),
            enumerations: model.enums.len(),
            pruned_types: pruned_types.iter().map(|s| s.to_string()).collect(),
        },
        files_generated: vec![
            "managed_objects.json".to_string(),
            "data_structures.json".to_string(),
            "enumerations.json".to_string(),
        ],
        generation_duration_ms: generation_duration.as_millis(),
    };

    let output_path = output_dir.join("metadata.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    println!("Generated: {}", output_path.display());
    Ok(())
}
