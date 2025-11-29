use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::collections::HashMap;
use indexmap::IndexMap;
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
pub struct ManagedObjectsOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub managed_objects: Vec<ManagedObjectEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataStructuresOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub structures: Vec<StructureEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumerationsOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub enumerations: Vec<EnumerationEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitsOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub traits: Vec<TraitEntry>,
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
    pub chunk_index: usize,
    pub chunk_count: usize,
}

/// Holds all loaded API data
#[derive(Debug, Clone)]
pub struct ApiData {
    // Primary indexes
    pub managed_objects: IndexMap<String, ManagedObjectEntry>,  // keyed by rust_struct
    pub data_structures: IndexMap<String, StructureEntry>,      // keyed by rust_name
    pub enumerations: IndexMap<String, EnumerationEntry>,       // keyed by rust_name
    pub traits: IndexMap<String, TraitEntry>,                    // keyed by rust_name
    pub examples: IndexMap<String, CodeExample>,                 // keyed by name
    pub guides: IndexMap<String, GuideChunk>,                    // keyed by chunk_id
    
    // Secondary indexes for dual-key lookups
    pub managed_objects_by_name: HashMap<String, String>,       // name -> rust_struct
    pub data_structures_by_name: HashMap<String, String>,       // name -> rust_name
    pub enumerations_by_name: HashMap<String, String>,          // name -> rust_name
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
        let traits_path = api_definitions_dir.join("traits.json");
        let examples_path = api_definitions_dir.join("examples.json");

        // Load managed objects and build indexes
        let (managed_objects, managed_objects_by_name) = if managed_objects_path.exists() {
            let content = std::fs::read_to_string(&managed_objects_path)?;
            let output: ManagedObjectsOutput = serde_json::from_str(&content)?;
            let mut map = IndexMap::new();
            let mut name_index = HashMap::new();
            for entry in output.managed_objects {
                let rust_struct = entry.rust_struct.clone();
                let name = entry.name.clone();
                map.insert(rust_struct.clone(), entry);
                name_index.insert(name, rust_struct);
            }
            (map, name_index)
        } else {
            warn!("managed_objects.json not found, using empty map");
            (IndexMap::new(), HashMap::new())
        };

        // Load data structures and build indexes
        let (data_structures, data_structures_by_name) = if data_structures_path.exists() {
            let content = std::fs::read_to_string(&data_structures_path)?;
            let output: DataStructuresOutput = serde_json::from_str(&content)?;
            let mut map = IndexMap::new();
            let mut name_index = HashMap::new();
            for entry in output.structures {
                let rust_name = entry.rust_name.clone();
                let name = entry.name.clone();
                map.insert(rust_name.clone(), entry);
                name_index.insert(name, rust_name);
            }
            (map, name_index)
        } else {
            warn!("data_structures.json not found, using empty map");
            (IndexMap::new(), HashMap::new())
        };

        // Load enumerations and build indexes
        let (enumerations, enumerations_by_name) = if enumerations_path.exists() {
            let content = std::fs::read_to_string(&enumerations_path)?;
            let output: EnumerationsOutput = serde_json::from_str(&content)?;
            let mut map = IndexMap::new();
            let mut name_index = HashMap::new();
            for entry in output.enumerations {
                let rust_name = entry.rust_name.clone();
                let name = entry.name.clone();
                map.insert(rust_name.clone(), entry);
                name_index.insert(name, rust_name);
            }
            (map, name_index)
        } else {
            warn!("enumerations.json not found, using empty map");
            (IndexMap::new(), HashMap::new())
        };

        // Load traits (no secondary index needed)
        let traits = if traits_path.exists() {
            let content = std::fs::read_to_string(&traits_path)?;
            let output: TraitsOutput = serde_json::from_str(&content)?;
            let mut map = IndexMap::new();
            for entry in output.traits {
                map.insert(entry.rust_name.clone(), entry);
            }
            map
        } else {
            warn!("traits.json not found, using empty map");
            IndexMap::new()
        };

        // Load examples
        let examples = if examples_path.exists() {
            let content = std::fs::read_to_string(&examples_path)?;
            let output: ExamplesOutput = serde_json::from_str(&content)?;
            let mut map = IndexMap::new();
            for entry in output.examples {
                map.insert(entry.name.clone(), entry);
            }
            map
        } else {
            warn!("examples.json not found, using empty map");
            IndexMap::new()
        };

        // Load guides
        let mut guides = IndexMap::new();

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
                        for chunk in chunks {
                            guides.insert(chunk.chunk_id.clone(), chunk);
                        }
                    }
                }
            }
        } else {
            warn!("guides directory not found, using empty map");
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
            managed_objects_by_name,
            data_structures_by_name,
            enumerations_by_name,
        })
    }

    pub fn get_managed_object(&self, rust_struct: &str) -> Option<&ManagedObjectEntry> {
        self.managed_objects.get(rust_struct)
            .or_else(|| self.managed_objects_by_name.get(rust_struct)
                .and_then(|name| self.managed_objects.get(name)))
    }

    pub fn get_data_structure(&self, rust_name: &str) -> Option<&StructureEntry> {
        self.data_structures.get(rust_name)
            .or_else(|| self.data_structures_by_name.get(rust_name)
                .and_then(|name| self.data_structures.get(name)))
    }

    pub fn get_enumeration(&self, rust_name: &str) -> Option<&EnumerationEntry> {
        self.enumerations.get(rust_name)
            .or_else(|| self.enumerations_by_name.get(rust_name)
               .and_then(|name| self.enumerations.get(name)))
    }

    pub fn get_trait(&self, rust_name: &str) -> Option<&TraitEntry> {
        self.traits.get(rust_name)
    }

    pub fn get_example(&self, name: &str) -> Option<&CodeExample> {
        self.examples.get(name)
    }

    pub fn get_guide(&self, chunk_id: &str) -> Option<&GuideChunk> {
        self.guides.get(chunk_id)
    }

    pub fn get_method(&self, managed_object: &str, method_name: &str) -> Option<&MethodEntry> {
        self.get_managed_object(managed_object)
            .and_then(|mo| mo.methods.iter().find(|m| m.rust_name == method_name || m.name == method_name))
    }

    pub fn get_field(&self, data_structure: &str, field_name: &str) -> Option<&FieldEntry> {
        self.get_data_structure(data_structure)
            .and_then(|s| s.fields.iter().find(|f| f.rust_name == field_name || f.name == field_name))
    }

}

// ============================================================================
// Embedding Data Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRecord {
    pub text: String,
    pub item_type: String,
    pub object_name: String,
    pub item_name: String,
    pub rust_name: String,
    pub rust_module: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingDatabase {
    pub records: Vec<EmbeddingRecord>,
    pub vectors: Vec<Vec<f32>>,
}