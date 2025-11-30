//! API Database types and model for vim_rs MCP server.
//!
//! This crate contains all the data types used by the MCP server and build tools.
//! It does NOT contain the embedded database - that is loaded by the server crate.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use tracing::info;

#[cfg(feature = "json")]
use std::path::Path;
#[cfg(feature = "json")]
use anyhow::Result;
#[cfg(feature = "json")]
use tracing::warn;

use std::collections::HashMap;

// ============================================================================
// Path Types - for navigating to types via the API
// ============================================================================

/// Starting point for a navigation path.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ApiPathOrigin {
    /// Property accessor (GET method) on a managed object.
    /// e.g., `VirtualMachine::config` returns `VirtualMachineConfigInfo`
    PropertyAccessor {
        managed_object: String,
        property_name: String,
    },
    /// Method output (return type).
    /// e.g., `VirtualMachine::reconfigure()` returns `Task`
    MethodOutput {
        managed_object: String,
        method_name: String,
    },
    /// Method input parameter.
    /// e.g., `VirtualMachine::apply_evc_mode_vm_task(mask)` takes `HostFeatureMask[]`
    MethodInput {
        managed_object: String,
        method_name: String,
        parameter_name: String,
    },
}

/// A single navigation step in a path.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ApiPathStep {
    /// Access a field: `.field_name`, `.field_name?`, `.field_name[*]`
    Field {
        field_name: String,
        is_optional: bool,
        is_array: bool,
    },
    /// Downcast to a more specific type.
    /// - If `is_trait_cast` is true: `⇒TypeNameTrait` (uses CastFrom/CastInto)
    /// - If `is_trait_cast` is false: `→TypeName` (uses std::any downcast)
    Downcast {
        to_type: String,
        is_trait_cast: bool,
    },
}

/// Complete path from an origin to a destination type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiTypePath {
    pub origin: ApiPathOrigin,
    pub steps: Vec<ApiPathStep>,
}

impl ApiTypePath {
    /// Render the path as shorthand notation using Rust-style names.
    ///
    /// Notation:
    /// - `TypeName::` - scope resolution after type names
    /// - `.field_name` - field access (snake_case)
    /// - `?` - optional element
    /// - `[*]` - array iteration
    /// - `→TypeName` - downcast to specific type
    /// - `(param_name)` - method input parameter (snake_case)
    ///
    /// Examples:
    /// - `VirtualMachine::config?.hardware.device[*]→VirtualEthernetCard`
    /// - `VirtualMachine::apply_evc_mode_vm_task(mask?)[*]`
    pub fn to_shorthand(&self) -> String {
        let mut result = String::new();

        // Render origin
        match &self.origin {
            ApiPathOrigin::PropertyAccessor {
                managed_object,
                property_name,
            } => {
                result.push_str(managed_object);
                result.push_str("::");
                result.push_str(property_name);
            }
            ApiPathOrigin::MethodOutput {
                managed_object,
                method_name,
            } => {
                result.push_str(managed_object);
                result.push_str("::");
                result.push_str(method_name);
                result.push_str("()");
            }
            ApiPathOrigin::MethodInput {
                managed_object,
                method_name,
                parameter_name,
            } => {
                result.push_str(managed_object);
                result.push_str("::");
                result.push_str(method_name);
                result.push('(');
                result.push_str(parameter_name);
                result.push(')');
            }
        }

        // Render steps
        for step in &self.steps {
            match step {
                ApiPathStep::Field {
                    field_name,
                    is_optional,
                    is_array,
                } => {
                    result.push('.');
                    result.push_str(field_name);
                    if *is_optional {
                        result.push('?');
                    }
                    if *is_array {
                        result.push_str("[*]");
                    }
                }
                ApiPathStep::Downcast {
                    to_type,
                    is_trait_cast,
                } => {
                    if *is_trait_cast {
                        // Trait cast uses double arrow and appends "Trait"
                        result.push_str("⇒");
                        result.push_str(to_type);
                        result.push_str("Trait");
                    } else {
                        // Struct downcast uses single arrow
                        result.push('→');
                        result.push_str(to_type);
                    }
                }
            }
        }

        result
    }

    /// Returns the depth of the path (number of steps).
    pub fn depth(&self) -> usize {
        self.steps.len()
    }
}

// ============================================================================
// Path Selection and Sorting
// ============================================================================

/// Priority order for inventory types when sorting paths.
/// Paths from these managed objects are shown first, in this order.
pub const INVENTORY_TYPE_PRIORITY: &[&str] = &[
    "VirtualMachine",
    "HostSystem",
    "Task",
    "Network",
    "DistributedVirtualPortgroup",
    "VmwareDistributedVirtualSwitch",
    "Folder",
    "Datacenter",
    "ResourcePool",
    "Datastore",
    "StoragePod",
    "ComputeResource",
    "ClusterComputeResource",
    "DistributedVirtualSwitch",
    "VirtualApp",
];

/// Maximum paths to keep per managed object type
const MAX_PATHS_PER_MANAGED_OBJECT: usize = 5;

/// Get the managed object name from a path origin
fn get_managed_object(origin: &ApiPathOrigin) -> &str {
    match origin {
        ApiPathOrigin::PropertyAccessor { managed_object, .. } => managed_object,
        ApiPathOrigin::MethodOutput { managed_object, .. } => managed_object,
        ApiPathOrigin::MethodInput { managed_object, .. } => managed_object,
    }
}

/// Check if a path origin is a property accessor (vs method)
fn is_property_accessor(origin: &ApiPathOrigin) -> bool {
    matches!(origin, ApiPathOrigin::PropertyAccessor { .. })
}

/// Get the sort key for a path (lower = higher priority).
/// Returns (is_method, inventory_priority, depth) for sorting.
/// Properties always come before methods (highest priority),
/// then inventory type priority, then path depth.
fn path_sort_key(path: &ApiTypePath) -> (bool, usize, usize) {
    let mo = get_managed_object(&path.origin);

    // Properties before methods (false < true in bool ordering) - HIGHEST PRIORITY
    let is_method = !is_property_accessor(&path.origin);

    // Find position in priority list (or max if not found)
    let inventory_priority = INVENTORY_TYPE_PRIORITY
        .iter()
        .position(|&t| t == mo)
        .unwrap_or(usize::MAX);

    // Tertiary sort by path depth (shorter paths first)
    let depth = path.depth();

    (is_method, inventory_priority, depth)
}

/// Sort paths by priority and return a limited selection as borrowed references.
/// Priority: properties before methods, then inventory types, then shorter paths.
/// Also limits paths per managed object to avoid one MO dominating the list.
///
/// # Arguments
/// * `paths` - The paths to select from
/// * `limit` - Maximum number of paths to return
pub fn select_paths_for_display(paths: &[ApiTypePath], limit: usize) -> Vec<&ApiTypePath> {
    if paths.is_empty() {
        return Vec::new();
    }

    // Create sorted indices
    let mut indices: Vec<usize> = (0..paths.len()).collect();
    indices.sort_by_key(|&i| path_sort_key(&paths[i]));

    // Count paths per managed object and filter
    let mut mo_counts: HashMap<&str, usize> = HashMap::new();
    let mut result = Vec::with_capacity(limit.min(paths.len()));

    for idx in indices {
        if result.len() >= limit {
            break;
        }

        let path = &paths[idx];
        let mo = get_managed_object(&path.origin);
        let count = mo_counts.entry(mo).or_insert(0);

        if *count < MAX_PATHS_PER_MANAGED_OBJECT {
            *count += 1;
            result.push(path);
        }
    }

    result
}

// ============================================================================
// Data structures matching the generated JSON files from vim_build
// ============================================================================

// Managed Objects

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodEntry {
    pub name: String,
    pub signature: MethodSignature,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedObjectEntry {
    pub name: String,
    pub rust_module: String,
    pub description: Option<String>,
    pub methods: Vec<MethodEntry>,
}

// Data Structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldEntry {
    pub name: String,
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
    /// Paths from API entry points leading to this struct type.
    #[serde(default)]
    pub paths: Vec<ApiTypePath>,
}

// Enumerations

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantEntry {
    pub name: String,
    pub description: Option<String>,
    pub discriminator_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumerationEntry {
    pub name: String,
    pub rust_module: String,
    pub description: Option<String>,
    pub variants: Vec<VariantEntry>,
}

// Traits

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

// Code Examples

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub name: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub source_code: String,
    pub file_path: String,
    pub dependencies: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamplesOutput {
    pub examples: Vec<CodeExample>,
}

// ============================================================================
// ApiData - Unified index for all items
// ============================================================================

/// Holds all loaded API data - unified index for all items
#[derive(Debug, Clone)]
pub struct ApiData {
    pub items: IndexMap<String, ApiItemEntry>,
}

impl ApiData {
    /// Load all JSON data files from the specified directory.
    /// This is a legacy method for backward compatibility.
    #[cfg(feature = "json")]
    pub fn load_from_dir(data_dir: &Path) -> Result<Self> {
        let api_definitions_dir = data_dir.join("api_definitions");
        let mut items = IndexMap::new();

        // Load managed objects and methods
        let managed_objects_path = api_definitions_dir.join("managed_objects.json");
        if managed_objects_path.exists() {
            let content = std::fs::read_to_string(&managed_objects_path)?;
            let output: ManagedObjectsOutput = serde_json::from_str(&content)?;
            for mo in output.managed_objects {
                for method in &mo.methods {
                    let id = format!("{}::{}", mo.rust_struct, method.rust_name);
                    items.insert(id.clone(), ApiItemEntry::Method(MethodItemData {
                        id,
                        managed_object: mo.rust_struct.clone(),
                        method: method.clone(),
                    }));
                }
                items.insert(mo.rust_struct.clone(), ApiItemEntry::ManagedObject(mo));
            }
        } else {
            warn!("managed_objects.json not found");
        }

        // Load data structures and fields
        let data_structures_path = api_definitions_dir.join("data_structures.json");
        if data_structures_path.exists() {
            let content = std::fs::read_to_string(&data_structures_path)?;
            let output: DataStructuresOutput = serde_json::from_str(&content)?;
            for structure in output.structures {
                for field in &structure.fields {
                    let id = format!("{}::{}", structure.rust_name, field.rust_name);
                    items.insert(id.clone(), ApiItemEntry::Field(FieldItemData {
                        id,
                        struct_name: structure.rust_name.clone(),
                        struct_description: structure.description.clone(),
                        field: field.clone(),
                        paths: structure.paths.clone(),
                    }));
                }
                items.insert(structure.rust_name.clone(), ApiItemEntry::Structure(structure));
            }
        } else {
            warn!("data_structures.json not found");
        }

        // Load enumerations
        let enumerations_path = api_definitions_dir.join("enumerations.json");
        if enumerations_path.exists() {
            let content = std::fs::read_to_string(&enumerations_path)?;
            let output: EnumerationsOutput = serde_json::from_str(&content)?;
            for e in output.enumerations {
                items.insert(e.rust_name.clone(), ApiItemEntry::Enumeration(e));
            }
        } else {
            warn!("enumerations.json not found");
        }

        // Load traits
        let traits_path = api_definitions_dir.join("traits.json");
        if traits_path.exists() {
            let content = std::fs::read_to_string(&traits_path)?;
            let output: TraitsOutput = serde_json::from_str(&content)?;
            for t in output.traits {
                items.insert(t.rust_name.clone(), ApiItemEntry::Trait(t));
            }
        } else {
            warn!("traits.json not found");
        }

        // Load examples
        let examples_path = api_definitions_dir.join("examples.json");
        if examples_path.exists() {
            let content = std::fs::read_to_string(&examples_path)?;
            let output: ExamplesOutput = serde_json::from_str(&content)?;
            for ex in output.examples {
                let id = format!("example::{}", ex.name);
                items.insert(id, ApiItemEntry::Example(ex));
            }
        } else {
            warn!("examples.json not found");
        }

        let api_data = ApiData { items };

        info!(
            "Loaded {} items: {} managed objects, {} methods, {} structures, {} fields, {} enums, {} traits, {} examples",
            api_data.items.len(),
            api_data.count_by_type("managed_object"),
            api_data.count_by_type("method"),
            api_data.count_by_type("structure"),
            api_data.count_by_type("field"),
            api_data.count_by_type("enum"),
            api_data.count_by_type("trait"),
            api_data.count_by_type("example"),
        );

        Ok(api_data)
    }

    /// Get any item by its unified ID
    pub fn get(&self, id: &str) -> Option<&ApiItemEntry> {
        self.items.get(id)
    }

    /// Count items by type (for logging/stats)
    pub fn count_by_type(&self, item_type: &str) -> usize {
        self.items.values().filter(|i| i.item_type() == item_type).count()
    }

    /// Construct ApiData from in-memory parts (without reading JSON files).
    /// This is used by data_transformer to build the unified database.
    pub fn from_parts(
        managed_objects: Vec<ManagedObjectEntry>,
        structures: Vec<StructureEntry>,
        enumerations: Vec<EnumerationEntry>,
        traits: Vec<TraitEntry>,
        examples: Vec<CodeExample>,
    ) -> Self {
        let mut items = IndexMap::new();

        // Add managed objects and their methods
        for mo in managed_objects {
            for method in &mo.methods {
                let id = format!("{}::{}", mo.name, method.name);
                items.insert(id.clone(), ApiItemEntry::Method(MethodItemData {
                    id,
                    managed_object: mo.name.clone(),
                    method: method.clone(),
                }));
            }
            items.insert(mo.name.clone(), ApiItemEntry::ManagedObject(mo));
        }

        // Add structures and their fields
        for structure in structures {
            for field in &structure.fields {
                let id = format!("{}::{}", structure.name, field.name);
                items.insert(id.clone(), ApiItemEntry::Field(FieldItemData {
                    id,
                    struct_name: structure.name.clone(),
                    struct_description: structure.description.clone(),
                    field: field.clone(),
                    paths: structure.paths.clone(),
                }));
            }
            items.insert(structure.name.clone(), ApiItemEntry::Structure(structure));
        }

        // Add enumerations
        for e in enumerations {
            items.insert(e.name.clone(), ApiItemEntry::Enumeration(e));
        }

        // Add traits
        for t in traits {
            items.insert(t.name.clone(), ApiItemEntry::Trait(t));
        }

        // Add examples
        for ex in examples {
            let id = format!("example::{}", ex.name);
            items.insert(id, ApiItemEntry::Example(ex));
        }

        let api_data = ApiData { items };

        info!(
            "Built {} items from parts: {} managed objects, {} methods, {} structures, {} fields, {} enums, {} traits, {} examples",
            api_data.items.len(),
            api_data.count_by_type("managed_object"),
            api_data.count_by_type("method"),
            api_data.count_by_type("structure"),
            api_data.count_by_type("field"),
            api_data.count_by_type("enum"),
            api_data.count_by_type("trait"),
            api_data.count_by_type("example"),
        );

        api_data
    }
}

// ============================================================================
// ApiItem Trait and Unified Item System
// ============================================================================

/// Common trait for all searchable/retrievable API items
pub trait ApiItem: Send + Sync {
    /// Unique identifier for lookup
    fn id(&self) -> &str;
    
    /// Item type for filtering
    fn item_type(&self) -> &'static str;
    
    /// Text optimized for semantic search embeddings
    fn embedding_text(&self) -> String;
    
    /// Brief markdown summary for search results
    fn search_summary(&self) -> String;
    
    /// Full detailed markdown document
    fn detailed_document(&self) -> String;
}

/// Helper struct for method items (denormalized from ManagedObjectEntry)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodItemData {
    pub id: String,
    pub managed_object: String,
    pub method: MethodEntry,
}

/// Helper struct for field items (denormalized from StructureEntry)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldItemData {
    pub id: String,
    pub struct_name: String,
    pub struct_description: Option<String>,
    pub field: FieldEntry,
    /// Paths from API entry points to the parent struct (for navigating to this field).
    #[serde(default)]
    pub paths: Vec<ApiTypePath>,
}

/// Unified enum holding all API item types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiItemEntry {
    ManagedObject(ManagedObjectEntry),
    Method(MethodItemData),
    Structure(StructureEntry),
    Field(FieldItemData),
    Enumeration(EnumerationEntry),
    Trait(TraitEntry),
    Example(CodeExample),
}

impl ApiItem for ApiItemEntry {
    fn id(&self) -> &str {
        match self {
            ApiItemEntry::ManagedObject(mo) => &mo.name,
            ApiItemEntry::Method(m) => &m.id,
            ApiItemEntry::Structure(s) => &s.name,
            ApiItemEntry::Field(f) => &f.id,
            ApiItemEntry::Enumeration(e) => &e.name,
            ApiItemEntry::Trait(t) => &t.name,
            ApiItemEntry::Example(ex) => &ex.name,
        }
    }

    fn item_type(&self) -> &'static str {
        match self {
            ApiItemEntry::ManagedObject(_) => "managed_object",
            ApiItemEntry::Method(_) => "method",
            ApiItemEntry::Structure(_) => "structure",
            ApiItemEntry::Field(_) => "field",
            ApiItemEntry::Enumeration(_) => "enum",
            ApiItemEntry::Trait(_) => "trait",
            ApiItemEntry::Example(_) => "example",
        }
    }

    fn embedding_text(&self) -> String {
        match self {
            ApiItemEntry::ManagedObject(mo) => {
                format!(
                    "{} - {}",
                    mo.name,
                    mo.description.as_deref().unwrap_or("No description")
                )
            }
            ApiItemEntry::Method(m) => {
                format!(
                    "{}.{} - {}",
                    m.managed_object,
                    m.method.name,
                    m.method.description.as_deref().unwrap_or("No description")
                )
            }
            ApiItemEntry::Structure(s) => {
                format!(
                    "{} - {}",
                    s.name,
                    s.description.as_deref().unwrap_or("No description")
                )
            }
            ApiItemEntry::Field(f) => {
                let mut text_parts = vec![
                    format!("{} field in {}", f.field.name, f.struct_name),
                    format!("Type: {}", f.field.rust_type),
                ];
                if let Some(ref desc) = f.field.description {
                    text_parts.push(prepare_text(desc));
                }
                if let Some(ref struct_desc) = f.struct_description {
                    text_parts.push(prepare_text(struct_desc));
                }
                text_parts.join(". ")
            }
            ApiItemEntry::Enumeration(e) => {
                let variant_names: Vec<&str> = e.variants.iter()
                    .map(|v| v.name.as_str())
                    .collect();
                format!(
                    "{} - {}. Variants: {}",
                    e.name,
                    prepare_text(e.description.as_deref().unwrap_or("No description")),
                    variant_names.join(", ")
                )
            }
            ApiItemEntry::Trait(t) => {
                format!(
                    "{} - {}",
                    t.name,
                    t.description.as_deref().unwrap_or("No description")
                )
            }
            ApiItemEntry::Example(ex) => {
                format!(
                    "{} - {} (Category: {}). {}",
                    ex.name,
                    ex.title,
                    ex.category,
                    ex.description
                )
            }
        }
    }

    fn search_summary(&self) -> String {
        match self {
            ApiItemEntry::ManagedObject(mo) => {
                format!(
                    "## {}\n\n**ID:** `{}`\n\n**Module:** `{}`\n\n{}\n\n---\n",
                    mo.name,
                    mo.name,
                    mo.rust_module,
                    mo.description.as_deref().unwrap_or("No description")
                )
            }
            ApiItemEntry::Method(m) => {
                let desc = m.method.description.as_deref().unwrap_or("No description");
                format!(
                    "## {}.{}\n\n**ID:** `{}`\n\n**Rust:** `{}`\n\n**Signature:**\n```rust\n{}\n```\n\n{}\n\n---\n",
                    m.managed_object,
                    m.method.name,
                    m.id,
                    m.method.name,
                    m.method.signature.full,
                    desc
                )
            }
            ApiItemEntry::Structure(s) => {
                let desc = s.description.as_deref().unwrap_or("No description");
                let parent_info = s.parent.as_ref()
                    .map(|p| format!(" (extends {})", p))
                    .unwrap_or_default();
                
                // Add top 3 paths
                let paths_section = if !s.paths.is_empty() {
                    let selected = select_paths_for_display(&s.paths, 3);
                    if !selected.is_empty() {
                        let paths_str: Vec<String> = selected.iter()
                            .map(|p| format!("- `{}`", p.to_shorthand()))
                            .collect();
                        format!("\n**Paths:**\n{}\n", paths_str.join("\n"))
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                format!(
                    "## {}{}\n\n**ID:** `{}`\n\n{}{}\n\n---\n",
                    s.name,
                    parent_info,
                    s.name,
                    desc,
                    paths_section
                )
            }
            ApiItemEntry::Field(f) => {
                // Add top 3 parent struct paths
                let paths_section = if !f.paths.is_empty() {
                    let selected = select_paths_for_display(&f.paths, 3);
                    if !selected.is_empty() {
                        let paths_str: Vec<String> = selected.iter()
                            .map(|p| format!("- `{}`", p.to_shorthand()))
                            .collect();
                        format!("\n**Paths to {}:**\n{}\n", f.struct_name, paths_str.join("\n"))
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                format!(
                    "## {}.{}\n\n**ID:** `{}`\n\n**Type:** `{}`\n\n{}{}\n\n---\n",
                    f.struct_name,
                    f.field.name,
                    f.id,
                    f.field.rust_type,
                    f.field.description.as_deref().unwrap_or("No description"),
                    paths_section
                )
            }
            ApiItemEntry::Enumeration(e) => {
                let desc = e.description.as_deref().unwrap_or("No description");
                let variants_preview: Vec<&str> = e.variants.iter()
                    .take(5)
                    .map(|v| v.name.as_str())
                    .collect();
                let variants_str = if e.variants.len() > 5 {
                    format!("{}, ... ({} more)", variants_preview.join(", "), e.variants.len() - 5)
                } else {
                    variants_preview.join(", ")
                };
                format!(
                    "## {}\n\n**ID:** `{}`\n\n**Variants:** {}\n\n---\n",
                    e.name,
                    desc,
                    variants_str
                )
            }
            ApiItemEntry::Trait(t) => {
                let desc = t.description.as_deref().unwrap_or("No description");
                format!(
                    "## {} (Trait)\n\n**ID:** `{}`\n\n**Module:** `{}`\n\n{}\n\n---\n",
                    t.name,
                    t.name,
                    t.rust_module,
                    desc
                )
            }
            ApiItemEntry::Example(ex) => {
                format!(
                    "## {} (Example)\n\n**ID:** `example::{}`\n\n**Category:** {}\n\n{}\n\n---\n",
                    ex.title,
                    ex.name,
                    ex.category,
                    ex.description.lines().take(3).collect::<Vec<_>>().join(" ")
                )
            }
        }
    }

    fn detailed_document(&self) -> String {
        match self {
            ApiItemEntry::ManagedObject(mo) => format_managed_object_doc(mo),
            ApiItemEntry::Method(m) => format_method_doc(m),
            ApiItemEntry::Structure(s) => format_structure_doc(s),
            ApiItemEntry::Field(f) => format_field_doc(f),
            ApiItemEntry::Enumeration(e) => format_enumeration_doc(e),
            ApiItemEntry::Trait(t) => format_trait_doc(t),
            ApiItemEntry::Example(ex) => format_example_doc(ex),
        }
    }
}

// ============================================================================
// Unified API Database (Single Binary Format)
// ============================================================================

/// Unified database containing all API data and embeddings.
/// This is the single binary format that gets embedded in the server executable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDatabase {
    /// All API items indexed by their unique ID
    pub items: IndexMap<String, ApiItemEntry>,
    /// Embedding vectors aligned with items (embeddings[i] corresponds to items[i])
    pub embeddings: Option<Vec<Vec<f32>>>,
}

impl ApiDatabase {
    /// Get any item by its unified ID
    pub fn get(&self, id: &str) -> Option<&ApiItemEntry> {
        self.items.get(id)
    }

    /// Count items by type (for logging/stats)
    pub fn count_by_type(&self, item_type: &str) -> usize {
        self.items.values().filter(|i| i.item_type() == item_type).count()
    }

    /// Get the embedding vector for an item by its index in the items map.
    pub fn get_embedding(&self, index: usize) -> Option<&Vec<f32>> {
        self.embeddings.as_ref().and_then(|e| e.get(index))
    }

    /// Check if embeddings are available
    pub fn has_embeddings(&self) -> bool {
        self.embeddings.is_some()
    }
}

// ============================================================================
// Helper functions for document formatting
// ============================================================================

fn prepare_text(text: &str) -> String {
    let mut cleaned_desc = text.replace('\n', " ").split_whitespace().collect::<Vec<_>>().join(" ");
    if cleaned_desc.len() > 300 {
        let mut start_pos = 300;
        while start_pos < cleaned_desc.len() && !cleaned_desc.is_char_boundary(start_pos) {
            start_pos -= 1;
        }
        let truncate_pos = cleaned_desc[start_pos..]
            .char_indices()
            .find(|(_, ch)| ch.is_whitespace())
            .map(|(idx, _)| start_pos + idx)
            .unwrap_or(cleaned_desc.len());
        cleaned_desc.truncate(truncate_pos);
        cleaned_desc = cleaned_desc.trim_end().to_string();
    }
    cleaned_desc
}

fn format_managed_object_doc(mo: &ManagedObjectEntry) -> String {
    let mut output = format!("# Managed Object: {}\n\n", mo.name);
    output.push_str(&format!("**Module:** `{}`\n", mo.rust_module));
    output.push_str(&format!("**VIM Type:** `{}`\n\n", mo.name));

    if let Some(desc) = &mo.description {
        output.push_str("## Description\n\n");
        output.push_str(desc);
        output.push_str("\n\n");
    }

    if !mo.methods.is_empty() {
        output.push_str(&format!("## Methods ({} methods)\n\n", mo.methods.len()));
        output.push_str("Use `get` with method ID to view detailed information.\n\n");
        for method in &mo.methods {
            output.push_str(&format!("- `{}::{}`\n", mo.name, method.name));
        }
        output.push('\n');
    }

    output
}

fn format_method_doc(m: &MethodItemData) -> String {
    let mut output = format!("# {}::{}\n\n", m.managed_object, m.method.name);
    output.push_str(&format!("**VIM Method:** `{}`\n\n", m.method.name));

    output.push_str("## Signature\n\n```rust\n");
    output.push_str(&m.method.signature.full);
    output.push_str("\n```\n\n");

    if let Some(desc) = &m.method.description {
        output.push_str("## Description\n\n");
        output.push_str(desc);
        output.push_str("\n\n");
    }

    if !m.method.signature.parameters.is_empty() {
        output.push_str("## Parameters\n\n");
        for param in &m.method.signature.parameters {
            output.push_str(&format!("### `{}: {}`\n\n", param.name, param.rust_type));
            output.push_str(&format!("**Required:** {}\n\n", if param.required { "Yes" } else { "No" }));
            if let Some(param_desc) = &param.description {
                output.push_str(param_desc);
                output.push_str("\n\n");
            }
        }
    }

    output.push_str("## Return Type\n\n");
    output.push_str(&format!("`{}`\n\n", m.method.signature.return_type));

    output
}

fn format_structure_doc(s: &StructureEntry) -> String {
    let mut output = format!("# Struct: {}\n\n", s.name);
    output.push_str(&format!("**Module:** `{}`\n", s.rust_module));
    output.push_str(&format!("**Emit Mode:** {}\n\n", s.emit_mode));

    if let Some(desc) = &s.description {
        output.push_str("## Description\n\n");
        output.push_str(desc);
        output.push_str("\n\n");
    }

    // Show top 10 paths for how to access this structure
    if !s.paths.is_empty() {
        let selected_paths = select_paths_for_display(&s.paths, 10);
        if !selected_paths.is_empty() {
            output.push_str("## How to Access\n\n");
            for path in &selected_paths {
                output.push_str(&format!("- `{}`\n", path.to_shorthand()));
            }
            if selected_paths.len() < s.paths.len() {
                output.push_str(&format!(
                    "\n*({} of {} paths)*\n",
                    selected_paths.len(),
                    s.paths.len()
                ));
            }
            output.push('\n');
        }
    }

    if !s.inheritance_chain.is_empty() {
        output.push_str("## Inheritance Chain\n\n");
        output.push_str(&s.inheritance_chain.join(" → "));
        output.push_str("\n\n");
    }

    if !s.implements_traits.is_empty() {
        output.push_str("## Implemented Traits\n\n");
        for trait_name in &s.implements_traits {
            output.push_str(&format!("- `{}`\n", trait_name));
        }
        output.push('\n');
    }

    if !s.children.is_empty() {
        output.push_str(&format!("## Direct Children ({} types)\n\n", s.children.len()));
        for child in s.children.iter().take(10) {
            output.push_str(&format!("- `{}`\n", child));
        }
        if s.children.len() > 10 {
            output.push_str(&format!("\n... and {} more\n", s.children.len() - 10));
        }
        output.push('\n');
    }

    if !s.all_descendants.is_empty() {
        output.push_str(&format!("## All Descendants ({} types total)\n\n", s.all_descendants.len()));
        for desc in s.all_descendants.iter().take(15) {
            output.push_str(&format!("- `{}`\n", desc));
        }
        if s.all_descendants.len() > 15 {
            output.push_str(&format!("\n... and {} more\n", s.all_descendants.len() - 15));
        }
        output.push('\n');
    }

    if !s.fields.is_empty() {
        output.push_str(&format!("## Fields ({} fields)\n\n", s.fields.len()));
        for field in &s.fields {
            output.push_str(&format!("### `{}: {}`\n", field.name, field.rust_type));
            output.push_str(&format!("- **Required:** {}\n", field.required));
            if field.is_array {
                output.push_str("- **Array:** Yes\n");
            }
            if field.is_trait {
                if let Some(trait_name) = &field.trait_name {
                    output.push_str(&format!("- **Trait:** `{}`\n", trait_name));
                }
            }
            if let Some(desc) = &field.description {
                let doc_preview = if desc.len() > 200 {
                    format!("{}...", &desc[..200])
                } else {
                    desc.clone()
                };
                output.push_str(&format!("- **Description:** {}\n", doc_preview));
            }
            output.push('\n');
        }
    }

    output
}

fn format_field_doc(f: &FieldItemData) -> String {
    let mut output = format!("# Field: {}.{}\n\n", f.struct_name, f.field.name);
    output.push_str(&format!("**Rust Type:** `{}`\n", f.field.rust_type));

    if let Some(doc) = &f.field.description {
        output.push_str("## Documentation\n\n");
        output.push_str(doc);
        output.push_str("\n\n");
    }

    // Show top 10 paths for how to access the parent structure (and thus this field)
    if !f.paths.is_empty() {
        let selected_paths = select_paths_for_display(&f.paths, 10);
        if !selected_paths.is_empty() {
            output.push_str("## How to Access\n\n");
            output.push_str(&format!("Access `{}` via:\n\n", f.struct_name));
            for path in &selected_paths {
                output.push_str(&format!("- `{}`\n", path.to_shorthand()));
            }
            if selected_paths.len() < f.paths.len() {
                output.push_str(&format!(
                    "\n*({} of {} paths)*\n",
                    selected_paths.len(),
                    f.paths.len()
                ));
            }
            output.push('\n');
        }
    }

    if let Some(struct_desc) = &f.struct_description {
        output.push_str(&format!("## Part of Structure: {}\n\n", f.struct_name));
        output.push_str(struct_desc);
        output.push_str("\n\n");
    }

    output
}

fn format_enumeration_doc(e: &EnumerationEntry) -> String {
    let mut output = format!("# Enum: {}\n\n", e.name);
    output.push_str(&format!("**Module:** `{}`\n\n", e.rust_module));

    if let Some(desc) = &e.description {
        output.push_str("## Description\n\n");
        output.push_str(desc);
        output.push_str("\n\n");
    }

    output.push_str(&format!("## Variants ({} variants)\n\n", e.variants.len()));
    for variant in &e.variants {
        output.push_str(&format!("### `{}`\n", variant.name));
        output.push_str(&format!("- **VIM Value:** `{}`\n", variant.discriminator_value));
        if let Some(desc) = &variant.description {
            output.push_str(&format!("- **Description:** {}\n", desc));
        }
        output.push('\n');
    }

    output.push_str("## Usage Example\n\n```rust\n");
    output.push_str(&format!("use vim_rs::types::enums::{};\n\n", e.name));
    output.push_str("match value {\n");
    for variant in e.variants.iter().take(3) {
        output.push_str(&format!("    {}::{} => {{ /* ... */ }}\n", e.name, variant.name));
    }
    if e.variants.len() > 3 {
        output.push_str("    // ...\n");
    }
    output.push_str("}\n```\n");

    output
}

fn format_trait_doc(t: &TraitEntry) -> String {
    let mut output = format!("# Trait: {}\n\n", t.name);
    output.push_str(&format!("**Module:** `{}`\n", t.rust_module));
    output.push_str(&format!("**Original Type:** `{}`\n\n", t.name));

    if let Some(desc) = &t.description {
        output.push_str("## Description\n\n");
        output.push_str(desc);
        output.push_str("\n\n");
    }

    if let Some(parent) = &t.parent_trait {
        output.push_str(&format!("**Parent Trait:** `{}`\n\n", parent));
    }

    if !t.getters.is_empty() {
        output.push_str("## Getter Methods\n\n");
        for getter in &t.getters {
            output.push_str(&format!("### `{}() -> {}`\n", getter.name, getter.return_type));
            output.push_str(&format!("- **Field:** `{}`\n", getter.field_name));
            if let Some(desc) = &getter.description {
                let doc_preview = if desc.len() > 200 {
                    format!("{}...", &desc[..200])
                } else {
                    desc.clone()
                };
                output.push_str(&format!("- **Description:** {}\n", doc_preview));
            }
            output.push('\n');
        }
    }

    if !t.implementing_types.is_empty() {
        output.push_str(&format!("## Implementing Types ({} types)\n\n", t.implementing_types.len()));
        for impl_type in t.implementing_types.iter().take(20) {
            output.push_str(&format!("- `{}`\n", impl_type));
        }
        if t.implementing_types.len() > 20 {
            output.push_str(&format!("\n... and {} more\n", t.implementing_types.len() - 20));
        }
        output.push('\n');
    }

    output.push_str("## Usage Example\n\n```rust\n");
    output.push_str("use vim_rs::types::convert::CastInto;\n");
    output.push_str(&format!("use vim_rs::types::traits::{};\n\n", t.name));
    if let Some(parent) = &t.parent_trait {
        output.push_str(&format!("let device: &dyn {} = /* ... */;\n", parent));
        output.push_str(&format!("if let Some(specialized): Option<&dyn {}> = device.as_ref().into_ref() {{\n", t.name));
    } else {
        output.push_str(&format!("let device: Box<dyn {}> = /* ... */;\n", t.name));
        output.push_str("if let Some(specialized) = device.as_ref() {\n");
    }
    if !t.getters.is_empty() {
        let getter = &t.getters[0];
        output.push_str(&format!("    let value = specialized.{}();\n", getter.name));
    }
    output.push_str("}\n```\n");

    output
}

fn format_example_doc(ex: &CodeExample) -> String {
    format!(
        "# {}\n\n**Category:** {}\n\n## Description\n\n{}\n\n## Source Code\n\n```rust\n{}\n```\n\n## Dependencies (Cargo.toml)\n\n```toml\n{}\n```\n\n**File:** `examples/{}`\n",
        ex.title,
        ex.category,
        ex.description,
        ex.source_code,
        ex.dependencies,
        ex.file_path
    )
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_managed_object() -> ManagedObjectEntry {
        ManagedObjectEntry {
            name: "VirtualMachine".to_string(),
            rust_module: "mo".to_string(),
            description: Some("A virtual machine managed object".to_string()),
            methods: vec![
                MethodEntry {
                    name: "power_on_vm_task".to_string(),
                    signature: MethodSignature {
                        full: "async fn power_on_vm_task(&self, host: Option<&HostSystem>) -> Result<Task>".to_string(),
                        parameters: vec![],
                        return_type: "Result<Task>".to_string(),
                    },
                    description: Some("Powers on the virtual machine".to_string()),
                },
            ],
        }
    }

    #[test]
    fn test_managed_object_id() {
        let mo = create_test_managed_object();
        let entry = ApiItemEntry::ManagedObject(mo);
        assert_eq!(entry.id(), "VirtualMachine");
        assert_eq!(entry.item_type(), "managed_object");
    }

    #[test]
    fn test_embedding_text_not_empty() {
        let mo = create_test_managed_object();
        let entry = ApiItemEntry::ManagedObject(mo);
        let text = entry.embedding_text();
        assert!(!text.is_empty());
        assert!(text.contains("VirtualMachine"));
    }
}

