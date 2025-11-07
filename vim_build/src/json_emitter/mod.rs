mod common;
mod signature_generator;
mod managed_objects;
mod data_structures;
mod enumerations;
mod metadata;

use crate::vim_model::Model;
use std::path::Path;
use std::time::Instant;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Generate all MCP JSON files
pub fn emit_mcp_data(
    model: &Model,
    output_dir: &Path,
    pruned_types: &[&str],
) -> Result<()> {
    // Create output directory
    std::fs::create_dir_all(output_dir)?;

    let start = Instant::now();

    // Generate all JSON files
    managed_objects::emit_managed_objects_json(model, output_dir)?;
    data_structures::emit_data_structures_json(model, output_dir)?;
    enumerations::emit_enumerations_json(model, output_dir)?;
    metadata::emit_metadata_json(model, output_dir, start.elapsed(), pruned_types)?;

    Ok(())
}
