use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{info, warn};

// ============================================================================
// Data structures matching the generated JSON files from vim_build
// ============================================================================

// Managed Objects - matches vim_build/src/json_emitter/common.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub rust_type: String,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodSignature {
    pub full: String,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: String,
    pub is_async: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodEntry {
    pub name: String,
    pub rust_name: String,
    pub signature: MethodSignature,
    pub description: Option<String>,
    pub related_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedObjectEntry {
    pub name: String,
    pub rust_module: String,
    pub rust_struct: String,
    pub description: Option<String>,
    pub methods: Vec<MethodEntry>,
}

// Data Structures - matches vim_build/src/json_emitter/common.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldEntry {
    pub name: String,
    pub rust_name: String,
    pub rust_type: String,
    pub vim_type: String,
    pub required: bool,
    pub description: Option<String>,
    pub is_array: bool,
    pub is_boxed: bool,
    pub is_trait: bool,
    pub trait_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureEntry {
    pub name: String,
    pub rust_name: String,
    pub rust_module: String,
    pub description: Option<String>,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub emit_mode: String,
    pub skip_reason: Option<String>,
    pub fields: Vec<FieldEntry>,
    pub related_types: Vec<String>,
    pub inheritance_chain: Vec<String>,
}

// Enumerations - matches vim_build/src/json_emitter/common.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantEntry {
    pub name: String,
    pub rust_name: String,
    pub description: Option<String>,
    pub discriminator_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumerationEntry {
    pub name: String,
    pub rust_name: String,
    pub rust_module: String,
    pub description: Option<String>,
    pub variants: Vec<VariantEntry>,
}

// Wrapper structures with metadata (what's actually in the JSON files)

#[derive(Debug, Serialize, Deserialize)]
struct ManagedObjectsOutput {
    managed_objects: Vec<ManagedObjectEntry>,
    // Ignoring generated_at and source for now
}

#[derive(Debug, Serialize, Deserialize)]
struct DataStructuresOutput {
    structures: Vec<StructureEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EnumerationsOutput {
    enumerations: Vec<EnumerationEntry>,
}

/// Holds all loaded API data
#[derive(Debug, Clone)]
pub struct ApiData {
    pub managed_objects: Vec<ManagedObjectEntry>,
    pub data_structures: Vec<StructureEntry>,
    pub enumerations: Vec<EnumerationEntry>,
}

impl ApiData {
    /// Load all JSON data files from the specified directory
    pub fn load_from_dir(data_dir: &Path) -> Result<Self> {
        let managed_objects_path = data_dir.join("managed_objects.json");
        let data_structures_path = data_dir.join("data_structures.json");
        let enumerations_path = data_dir.join("enumerations.json");

        let managed_objects = if managed_objects_path.exists() {
            let content = std::fs::read_to_string(&managed_objects_path)?;
            let output: ManagedObjectsOutput = serde_json::from_str(&content)?;
            output.managed_objects
        } else {
            warn!("managed_objects.json not found, using empty list");
            Vec::new()
        };

        let data_structures = if data_structures_path.exists() {
            let content = std::fs::read_to_string(&data_structures_path)?;
            let output: DataStructuresOutput = serde_json::from_str(&content)?;
            output.structures
        } else {
            warn!("data_structures.json not found, using empty list");
            Vec::new()
        };

        let enumerations = if enumerations_path.exists() {
            let content = std::fs::read_to_string(&enumerations_path)?;
            let output: EnumerationsOutput = serde_json::from_str(&content)?;
            output.enumerations
        } else {
            warn!("enumerations.json not found, using empty list");
            Vec::new()
        };

        info!(
            "Loaded {} managed objects, {} data structures, {} enumerations",
            managed_objects.len(),
            data_structures.len(),
            enumerations.len()
        );

        Ok(ApiData {
            managed_objects,
            data_structures,
            enumerations,
        })
    }
}
