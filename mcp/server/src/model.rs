use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
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

/// Holds all loaded API data - unified index for all items
#[derive(Debug, Clone)]
pub struct ApiData {
    pub items: IndexMap<String, ApiItemEntry>,
}

impl ApiData {
    /// Load all JSON data files from the specified directory
    pub fn load_from_dir(data_dir: &Path) -> Result<Self> {
        let api_definitions_dir = data_dir.join("api_definitions");
        let mut items = IndexMap::new();

        // Load managed objects and methods
        let managed_objects_path = api_definitions_dir.join("managed_objects.json");
        if managed_objects_path.exists() {
            let content = std::fs::read_to_string(&managed_objects_path)?;
            let output: ManagedObjectsOutput = serde_json::from_str(&content)?;
            for mo in output.managed_objects {
                // Add methods first (so they come after their parent in iteration order)
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
                // Add fields
                for field in &structure.fields {
                    let id = format!("{}::{}", structure.rust_name, field.rust_name);
                    items.insert(id.clone(), ApiItemEntry::Field(FieldItemData {
                        id,
                        struct_name: structure.rust_name.clone(),
                        struct_description: structure.description.clone(),
                        field: field.clone(),
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
}

// ============================================================================
// ApiItem Trait and Unified Item System
// ============================================================================

/// Common trait for all searchable/retrievable API items
pub trait ApiItem: Send + Sync {
    /// Unique identifier for lookup
    fn id(&self) -> &str;
    
    /// Item type for filtering: "managed_object", "method", "structure", 
    /// "enum", "trait", "field", "example"
    fn item_type(&self) -> &'static str;
    
    /// Text optimized for semantic search embeddings
    fn embedding_text(&self) -> String;
    
    /// Brief markdown summary for search results (include ID for retrieval)
    fn search_summary(&self) -> String;
    
    /// Full detailed markdown document
    fn detailed_document(&self) -> String;
}

/// Helper struct for method items (denormalized from ManagedObjectEntry)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodItemData {
    pub id: String,              // precomputed: "VirtualMachine::power_on_vm_task"
    pub managed_object: String,  // parent ID: "VirtualMachine"
    pub method: MethodEntry,
}

/// Helper struct for field items (denormalized from StructureEntry)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldItemData {
    pub id: String,                        // precomputed: "VirtualHardware::device"
    pub struct_name: String,               // parent ID: "VirtualHardware"
    pub struct_description: Option<String>, // for context in detailed_document
    pub field: FieldEntry,
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
            ApiItemEntry::ManagedObject(mo) => &mo.rust_struct,
            ApiItemEntry::Method(m) => &m.id,
            ApiItemEntry::Structure(s) => &s.rust_name,
            ApiItemEntry::Field(f) => &f.id,
            ApiItemEntry::Enumeration(e) => &e.rust_name,
            ApiItemEntry::Trait(t) => &t.rust_name,
            ApiItemEntry::Example(ex) => &ex.name, // Note: full ID is "example::{name}" but we store by full ID
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
                    format!("{} field in {}", f.field.rust_name, f.struct_name),
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
                    "## {}\n\n**ID:** `{}`\n\n**Rust Struct:** `{}`\n\n**Module:** `{}`\n\n{}\n\n---\n",
                    mo.name,
                    mo.rust_struct,
                    mo.rust_struct,
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
                    m.method.rust_name,
                    m.method.signature.full,
                    desc
                )
            }
            ApiItemEntry::Structure(s) => {
                let desc = s.description.as_deref().unwrap_or("No description");
                let parent_info = s.parent.as_ref()
                    .map(|p| format!(" (extends {})", p))
                    .unwrap_or_default();
                format!(
                    "## {}{}\n\n**ID:** `{}`\n\n**Rust:** `{}`\n\n{}\n\n---\n",
                    s.name,
                    parent_info,
                    s.rust_name,
                    s.rust_name,
                    desc
                )
            }
            ApiItemEntry::Field(f) => {
                format!(
                    "## {} (Field in {})\n\n**ID:** `{}`\n\n**Type:** `{}`\n\n{}\n\n---\n",
                    f.field.rust_name,
                    f.struct_name,
                    f.id,
                    f.field.rust_type,
                    f.field.description.as_deref().unwrap_or("No description")
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
                    "## {}\n\n**ID:** `{}`\n\n**Rust:** `{}`\n\n{}\n\n**Variants:** {}\n\n---\n",
                    e.name,
                    e.rust_name,
                    e.rust_name,
                    desc,
                    variants_str
                )
            }
            ApiItemEntry::Trait(t) => {
                let desc = t.description.as_deref().unwrap_or("No description");
                format!(
                    "## {} (Trait)\n\n**ID:** `{}`\n\n**Module:** `{}`\n\n{}\n\n---\n",
                    t.rust_name,
                    t.rust_name,
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
            ApiItemEntry::ManagedObject(mo) => {
                let mut output = format!("# Managed Object: {}\n\n", mo.rust_struct);
                output.push_str(&format!("**Module:** `{}`\n", mo.rust_module));
                output.push_str(&format!("**VIM Type:** `{}`\n\n", mo.name));

                if let Some(desc) = &mo.description {
                    output.push_str("## Description\n\n");
                    output.push_str(desc);
                    output.push_str("\n\n");
                }

                if !mo.methods.is_empty() {
                    output.push_str(&format!("## Methods ({} methods)\n\n", mo.methods.len()));
                    output.push_str("Use `get` with method ID (e.g., `VirtualMachine::power_on_vm_task`) to view detailed information.\n\n");
                    for method in &mo.methods {
                        output.push_str(&format!("- `{}::{}`\n", mo.rust_struct, method.rust_name));
                    }
                    output.push('\n');
                }

                output
            }
            ApiItemEntry::Method(m) => {
                let mut output = format!("# {}::{}\n\n", m.managed_object, m.method.rust_name);
                output.push_str(&format!("**VIM Method:** `{}`\n\n", m.method.name));

                output.push_str("## Signature\n\n");
                output.push_str("```rust\n");
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

                if m.method.signature.is_async {
                    output.push_str("**Async:** Yes - This method returns a Future and should be awaited.\n\n");
                }

                if !m.method.related_types.is_empty() {
                    output.push_str("## Related Types\n\n");
                    for related_type in &m.method.related_types {
                        output.push_str(&format!("- `{}`\n", related_type));
                    }
                    output.push('\n');
                }

                output
            }
            ApiItemEntry::Structure(s) => {
                let mut output = format!("# Struct: {}\n\n", s.rust_name);
                output.push_str(&format!("**Module:** `{}`\n", s.rust_module));
                output.push_str(&format!("**Emit Mode:** {}\n\n", s.emit_mode));

                if let Some(desc) = &s.description {
                    output.push_str("## Description\n\n");
                    output.push_str(desc);
                    output.push_str("\n\n");
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
                        output.push_str(&format!("### `{}: {}`\n", field.rust_name, field.rust_type));
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
            ApiItemEntry::Field(f) => {
                let mut output = format!("# Field: {}.{}\n\n", f.struct_name, f.field.rust_name);
                output.push_str(&format!("**VIM Name:** `{}`\n", f.field.name));
                output.push_str(&format!("**Rust Type:** `{}`\n", f.field.rust_type));
                output.push_str(&format!("**Required:** {}\n\n", f.field.required));

                if f.field.is_array {
                    output.push_str("**Array:** Yes\n\n");
                }

                if let Some(doc) = &f.field.description {
                    output.push_str("## Documentation\n\n");
                    output.push_str(doc);
                    output.push_str("\n\n");
                }

                if let Some(struct_desc) = &f.struct_description {
                    output.push_str(&format!("## Part of Structure: {}\n\n", f.struct_name));
                    output.push_str(struct_desc);
                    output.push_str("\n\n");
                }

                output
            }
            ApiItemEntry::Enumeration(e) => {
                let mut output = format!("# Enum: {}\n\n", e.rust_name);
                output.push_str(&format!("**Module:** `{}`\n\n", e.rust_module));

                if let Some(desc) = &e.description {
                    output.push_str("## Description\n\n");
                    output.push_str(desc);
                    output.push_str("\n\n");
                }

                output.push_str(&format!("## Variants ({} variants)\n\n", e.variants.len()));
                for variant in &e.variants {
                    output.push_str(&format!("### `{}`\n", variant.rust_name));
                    output.push_str(&format!("- **VIM Value:** `{}`\n", variant.discriminator_value));
                    if let Some(desc) = &variant.description {
                        output.push_str(&format!("- **Description:** {}\n", desc));
                    }
                    output.push('\n');
                }

                output.push_str("## Usage Example\n\n");
                output.push_str("```rust\n");
                output.push_str(&format!("use vim_rs::types::enums::{};\n\n", e.rust_name));
                output.push_str("match value {\n");
                for variant in e.variants.iter().take(3) {
                    output.push_str(&format!("    {}::{} => {{ /* ... */ }}\n", e.rust_name, variant.rust_name));
                }
                if e.variants.len() > 3 {
                    output.push_str("    // ...\n");
                }
                output.push_str("}\n");
                output.push_str("```\n");

                output
            }
            ApiItemEntry::Trait(t) => {
                let mut output = format!("# Trait: {}\n\n", t.rust_name);
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

                output.push_str("## Usage Example\n\n");
                output.push_str("```rust\n");
                output.push_str("use vim_rs::types::convert::CastInto;\n");
                output.push_str(&format!("use vim_rs::types::traits::{};\n\n", t.rust_name));
                output.push_str("// Cast from parent trait to this trait\n");
                if let Some(parent) = &t.parent_trait {
                    output.push_str(&format!("let device: &dyn {} = /* ... */;\n", parent));
                    output.push_str(&format!("if let Some(specialized): Option<&dyn {}> = device.as_ref().into_ref() {{\n", t.rust_name));
                } else {
                    output.push_str(&format!("let device: Box<dyn {}> = /* ... */;\n", t.rust_name));
                    output.push_str("if let Some(specialized) = device.as_ref() {\n");
                }
                if !t.getters.is_empty() {
                    let getter = &t.getters[0];
                    output.push_str(&format!("    let value = specialized.{}();\n", getter.name));
                }
                output.push_str("}\n");
                output.push_str("```\n");

                output
            }
            ApiItemEntry::Example(ex) => {
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
        }
    }
}

/// Prepare text for embedding by removing extra whitespace and newline and cutting to 300 characters
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

// ============================================================================
// Embedding Data Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRecord {
    pub text: String,
    pub item_type: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingDatabase {
    pub records: Vec<EmbeddingRecord>,
    pub vectors: Vec<Vec<f32>>,
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
            rust_struct: "VirtualMachine".to_string(),
            description: Some("A virtual machine managed object".to_string()),
            methods: vec![
                MethodEntry {
                    name: "PowerOnVM_Task".to_string(),
                    rust_name: "power_on_vm_task".to_string(),
                    signature: MethodSignature {
                        full: "async fn power_on_vm_task(&self, host: Option<&HostSystem>) -> Result<Task>".to_string(),
                        parameters: vec![],
                        return_type: "Result<Task>".to_string(),
                        is_async: true,
                    },
                    description: Some("Powers on the virtual machine".to_string()),
                    related_types: vec!["Task".to_string()],
                },
            ],
        }
    }

    fn create_test_structure() -> StructureEntry {
        StructureEntry {
            name: "VirtualDevice".to_string(),
            rust_name: "VirtualDevice".to_string(),
            rust_module: "structs".to_string(),
            description: Some("Base class for virtual devices".to_string()),
            parent: None,
            children: vec!["VirtualEthernetCard".to_string()],
            emit_mode: "struct".to_string(),
            skip_reason: None,
            fields: vec![
                FieldEntry {
                    name: "key".to_string(),
                    rust_name: "key".to_string(),
                    rust_type: "i32".to_string(),
                    vim_type: "int".to_string(),
                    required: true,
                    description: Some("The unique device key".to_string()),
                    is_array: false,
                    is_boxed: false,
                    is_trait: false,
                    trait_name: None,
                },
            ],
            related_types: vec![],
            inheritance_chain: vec![],
            implements_traits: vec!["VirtualDeviceTrait".to_string()],
            all_descendants: vec!["VirtualEthernetCard".to_string()],
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
    fn test_method_id() {
        let mo = create_test_managed_object();
        let method_data = MethodItemData {
            id: "VirtualMachine::power_on_vm_task".to_string(),
            managed_object: mo.rust_struct.clone(),
            method: mo.methods[0].clone(),
        };
        let entry = ApiItemEntry::Method(method_data);
        assert_eq!(entry.id(), "VirtualMachine::power_on_vm_task");
        assert_eq!(entry.item_type(), "method");
    }

    #[test]
    fn test_structure_id() {
        let s = create_test_structure();
        let entry = ApiItemEntry::Structure(s);
        assert_eq!(entry.id(), "VirtualDevice");
        assert_eq!(entry.item_type(), "structure");
    }

    #[test]
    fn test_field_id() {
        let s = create_test_structure();
        let field_data = FieldItemData {
            id: "VirtualDevice::key".to_string(),
            struct_name: s.rust_name.clone(),
            struct_description: s.description.clone(),
            field: s.fields[0].clone(),
        };
        let entry = ApiItemEntry::Field(field_data);
        assert_eq!(entry.id(), "VirtualDevice::key");
        assert_eq!(entry.item_type(), "field");
    }

    #[test]
    fn test_embedding_text_not_empty() {
        let mo = create_test_managed_object();
        let entry = ApiItemEntry::ManagedObject(mo);
        let text = entry.embedding_text();
        assert!(!text.is_empty());
        assert!(text.contains("VirtualMachine"));
    }

    #[test]
    fn test_search_summary_contains_id() {
        let mo = create_test_managed_object();
        let entry = ApiItemEntry::ManagedObject(mo);
        let summary = entry.search_summary();
        assert!(summary.contains("**ID:**"));
        assert!(summary.contains("VirtualMachine"));
    }

    #[test]
    fn test_detailed_document_is_markdown() {
        let mo = create_test_managed_object();
        let entry = ApiItemEntry::ManagedObject(mo);
        let doc = entry.detailed_document();
        assert!(doc.starts_with("# "));
        assert!(doc.contains("## "));
    }

    #[test]
    fn test_prepare_text_truncation() {
        let long_text = "a ".repeat(200);  // 400 chars
        let result = prepare_text(&long_text);
        assert!(result.len() <= 302);  // 300 + some buffer for word boundary
    }

    #[test]
    fn test_prepare_text_whitespace() {
        let text = "Hello\n  World  \n  Foo";
        let result = prepare_text(text);
        assert_eq!(result, "Hello World Foo");
    }
}