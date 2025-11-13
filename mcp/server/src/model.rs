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
    pub implements_traits: Vec<String>,
    pub all_descendants: Vec<String>,
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

// Traits - matches vim_build/src/json_emitter/traits.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetterEntry {
    pub name: String,
    pub return_type: String,
    pub description: Option<String>,
    pub field_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitEntry {
    pub name: String,
    pub rust_name: String,
    pub rust_module: String,
    pub description: Option<String>,
    pub parent_trait: Option<String>,
    pub getters: Vec<GetterEntry>,
    pub implementing_types: Vec<String>,
    pub all_descendants: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize)]
struct TraitsOutput {
    traits: Vec<TraitEntry>,
}

// Code Examples - for teaching usage patterns

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub name: String,
    pub title: String,
    pub description: String,
    pub category: String,  // "connection", "property_collector", "macro_usage", etc.
    pub source_code: String,
    pub file_path: String,
    pub dependencies: String,  // Cargo.toml snippet
}

#[derive(Debug, Serialize, Deserialize)]
struct ExamplesOutput {
    examples: Vec<CodeExample>,
}

// Guide chunks - matches build_guides output

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideChunk {
    pub heading_h1: String,
    pub heading_h2: String,
    pub heading_h3: String,
    pub sub_section: Option<String>,
    pub content: String,
    pub word_count: usize,
    pub source_file: String,
    pub chunk_id: String,
    pub topics: Vec<String>,
}

/// Holds all loaded API data
#[derive(Debug, Clone)]
pub struct ApiData {
    pub managed_objects: Vec<ManagedObjectEntry>,
    pub data_structures: Vec<StructureEntry>,
    pub enumerations: Vec<EnumerationEntry>,
    pub traits: Vec<TraitEntry>,
    pub examples: Vec<CodeExample>,
    pub guides: Vec<GuideChunk>,
}

impl ApiData {
    /// Load all JSON data files from the specified directory
    /// Looks for API definition files in data_dir/api_definitions/ if not found in data_dir/
    pub fn load_from_dir(data_dir: &Path) -> Result<Self> {
        // Try api_definitions subdirectory first, then root directory
        let api_definitions_dir = data_dir.join("api_definitions");
        let managed_objects_path = api_definitions_dir.join("managed_objects.json");
        let data_structures_path = api_definitions_dir.join("data_structures.json");
        let enumerations_path = api_definitions_dir.join("enumerations.json");

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

        let traits_path = if api_definitions_dir.join("traits.json").exists() {
            api_definitions_dir.join("traits.json")
        } else {
            data_dir.join("traits.json")
        };
        let traits = if traits_path.exists() {
            let content = std::fs::read_to_string(&traits_path)?;
            let output: TraitsOutput = serde_json::from_str(&content)?;
            output.traits
        } else {
            warn!("traits.json not found, using empty list");
            Vec::new()
        };

        let examples_path = if api_definitions_dir.join("examples.json").exists() {
            api_definitions_dir.join("examples.json")
        } else {
            data_dir.join("examples.json")
        };
        let examples = if examples_path.exists() {
            let content = std::fs::read_to_string(&examples_path)?;
            let output: ExamplesOutput = serde_json::from_str(&content)?;
            output.examples
        } else {
            warn!("examples.json not found, using empty list");
            Vec::new()
        };

        let mut guides = Vec::new();

        if api_definitions_dir.exists() && api_definitions_dir.is_dir() {
            for entry in std::fs::read_dir(&api_definitions_dir)? {
                let entry = entry?;
                let path = entry.path();

                // Only process .json files
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    if file_name.ends_with("_guide.json") {
                        let content = std::fs::read_to_string(&path)?;
                        let chunks: Vec<GuideChunk> = serde_json::from_str(&content)?;
                        info!("Loaded {} chunks from {}", chunks.len(), path.display());
                        guides.extend(chunks);
                    }
                }
            }
        } else {
            warn!("guides directory not found, using empty list");
        }

        info!(
            "Loaded {} managed objects, {} data structures, {} enumerations, {} traits, {} examples, {} guide chunks",
            managed_objects.len(),
            data_structures.len(),
            enumerations.len(),
            traits.len(),
            examples.len(),
            guides.len()
        );

        Ok(ApiData {
            managed_objects,
            data_structures,
            enumerations,
            traits,
            examples,
            guides,
        })
    }
}
