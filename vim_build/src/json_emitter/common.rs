// vim_build/src/json_emitter/common.rs
// v1: Simplified schema - no parsing

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

// Managed Objects Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagedObjectsOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub managed_objects: Vec<ManagedObjectEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagedObjectEntry {
    pub name: String,
    pub rust_module: String,
    pub rust_struct: String,
    pub description: Option<String>,
    pub methods: Vec<MethodEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MethodEntry {
    pub name: String,
    pub rust_name: String,
    pub signature: MethodSignature,
    pub description: Option<String>,  // Raw markdown, no parsing
    pub related_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MethodSignature {
    pub full: String,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: String,
    pub is_async: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub rust_type: String,
    pub required: bool,
    pub description: Option<String>,
}

// Data Structures Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct DataStructuresOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub structures: Vec<StructureEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

// Enumerations Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumerationsOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub enumerations: Vec<EnumerationEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumerationEntry {
    pub name: String,
    pub rust_name: String,
    pub rust_module: String,
    pub description: Option<String>,
    pub variants: Vec<VariantEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariantEntry {
    pub name: String,
    pub rust_name: String,
    pub description: Option<String>,
    pub discriminator_value: String,
}

// Metadata Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub statistics: Statistics,
    pub files_generated: Vec<String>,
    pub generation_duration_ms: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    pub managed_objects: usize,
    pub total_methods: usize,
    pub data_structures_total: usize,
    pub data_structures_emitted: usize,
    pub data_structures_pruned: usize,
    pub data_structures_skipped: usize,
    pub enumerations: usize,
    pub pruned_types: Vec<String>,
}
