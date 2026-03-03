mod common;
mod signature_generator;
mod managed_objects;
mod data_structures;
mod enumerations;
mod metadata;
mod traits;

use vim_build::vim_model::Model;
use api_database::{
    ManagedObjectEntry, StructureEntry, EnumerationEntry, TraitEntry
};
use std::path::Path;
use std::time::Instant;
use tracing::info;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Container for all API definitions built in memory.
/// This is used by data_transformer to construct ApiData without intermediate JSON files.
#[derive(Debug, Clone)]
pub struct ApiDefinitions {
    pub managed_objects: Vec<ManagedObjectEntry>,
    pub structures: Vec<StructureEntry>,
    pub enumerations: Vec<EnumerationEntry>,
    pub traits: Vec<TraitEntry>,
}

/// Build all API definitions in memory (no file I/O).
/// This is the primary function for the unified binary pipeline.
pub fn build_api_data(model: &Model) -> Result<ApiDefinitions> {
    let start = Instant::now();

    let managed_objects = managed_objects::build_managed_objects(model);
    let structures = data_structures::build_structures(model);
    let enumerations = enumerations::build_enumerations(model);
    let traits = traits::build_traits(model);

    info!(
        "Built API definitions in {:?}: {} managed objects, {} structures, {} enumerations, {} traits",
        start.elapsed(),
        managed_objects.len(),
        structures.len(),
        enumerations.len(),
        traits.len()
    );

    Ok(ApiDefinitions {
        managed_objects,
        structures,
        enumerations,
        traits,
    })
}

/// Generate all MCP JSON files (for debugging/optional JSON export).
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
    traits::emit_traits_json(model, output_dir)?;
    metadata::emit_metadata_json(model, output_dir, start.elapsed(), pruned_types)?;

    Ok(())
}

