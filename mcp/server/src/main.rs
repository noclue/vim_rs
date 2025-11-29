use anyhow::{Result, Context};
use rmcp::{
    ErrorData as McpError, handler::server::{
        ServerHandler,
        tool::{ToolRouter},
        wrapper::Parameters,
    },
    model::*,
    service::ServiceExt,
    tool,
    tool_handler,
    tool_router
};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::sync::Arc;
use tokio::io::{stdin, stdout};
use tracing::{error, info, warn};

// Import data model from the library
use vim_mcp_server::model::ApiData;
use vim_mcp_server::property_collector;
use vim_mcp_server::model::EmbeddingDatabase;

// Conditional imports for embeddings feature
#[cfg(feature = "embeddings")]
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
#[cfg(feature = "embeddings")]
use rayon::prelude::*;
#[cfg(feature = "embeddings")]
use std::sync::Mutex;
#[cfg(feature = "embeddings")]
use std::fs::File;
#[cfg(feature = "embeddings")]
use std::io::BufReader;

// Conditional imports for CUDA GPU acceleration
#[cfg(feature = "cuda")]
use ort::execution_providers::CUDAExecutionProvider;

// ============================================================================
// MCP Server
// ============================================================================

/// McpServer - A Model Context Protocol server
#[derive(Clone)]
pub struct McpServer {
    tool_router: ToolRouter<Self>,
    api_data: Arc<ApiData>,
    #[cfg(feature = "embeddings")]
    embedding_model: Option<Arc<Mutex<TextEmbedding>>>,
    #[cfg(feature = "embeddings")]
    embeddings_db: Option<Arc<EmbeddingDatabase>>,
}

/// Input parameters for semantic search tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct SemanticSearchInput {
    /// Natural language search query
    #[schemars(description = "Natural language query to find relevant API items")]
    query: String,

    /// Maximum number of results to return
    #[schemars(description = "Maximum number of results to return (default: 10)")]
    #[serde(default = "default_limit")]
    limit: usize,

    /// Filter by item type
    #[schemars(description = "Filter results by type: 'all', 'managed_objects', 'methods', 'structures', 'enums', 'fields', 'examples', or 'guides' (default: 'all')")]
    #[serde(default = "default_filter")]
    filter: String,
}

fn default_limit() -> usize {
    10
}

fn default_filter() -> String {
    "all".to_string()
}

/// Input parameters for get_example tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetExampleInput {
    /// Example name
    #[schemars(description = "The name of the example to retrieve (e.g., 'connection_basic', 'property_collector_macro')")]
    name: String,
}

/// Input parameters for get_guide tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetGuideInput {
    /// Chunk ID of the guide section to retrieve
    #[schemars(description = "The chunk_id of the guide section (e.g., 'installing-esx-understanding-auto-deploy')")]
    chunk_id: String,
}

/// Input parameters for get_workflow_guide tool (empty struct for consistency)
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetStarterGuideInput {
    // Empty - get_starter_guide takes no parameters but needs object schema
}

/// Input parameters for list_managed_object_types tool (empty struct for consistency)
#[derive(Serialize, Deserialize, JsonSchema)]
struct ListManagedObjectTypesInput {
    // Empty - list_managed_object_types takes no parameters but needs object schema
}

/// Input parameters for get_property_info tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetPropertyInfoInput {
    /// Managed object type (e.g., "VirtualMachine", "HostSystem", "Folder")
    #[schemars(description = "The managed object type name (e.g., 'VirtualMachine', 'HostSystem', 'Datacenter')")]
    managed_object: String,

    /// Property path in snake_case (e.g., "guest.ip_address" or empty for top-level fields)
    #[schemars(description = "Property path in snake_case format (e.g., 'guest.ip_address', 'config.hardware.device') or empty string to list top-level fields")]
    #[serde(default)]
    property_path: String,
}

/// Input parameters for get_type tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetTypeInput {
    /// Type name (e.g., "VirtualDevice", "VirtualEthernetCard", "VirtualDeviceTrait")
    #[schemars(description = "The type name to query. Can be a managed object (e.g., 'VirtualMachine'), struct (e.g., 'VirtualDevice'), trait (e.g., 'VirtualDeviceTrait'), or enum (e.g., 'ManagedEntityStatusEnum')")]
    type_name: String,
}

/// Input parameters for get_method tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetMethodInput {
    /// Managed object name (e.g., "VirtualMachine", "HostSystem", "Datastore")
    #[schemars(description = "The managed object name (e.g., 'VirtualMachine', 'HostSystem', 'Datastore')")]
    managed_object: String,
    
    /// Method name (e.g., "power_on_vm_task", "enter_maintenance_mode_task")
    #[schemars(description = "The method name to query (e.g., 'power_on_vm_task', 'enter_maintenance_mode_task')")]
    method_name: String,
}

/// Input parameters for get_field tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetFieldInput {
    /// The owner type name (e.g., "VirtualHardware")
    #[schemars(description = "The structure type that owns this field")]
    owner_type: String,
    
    /// The field name (e.g., "device", "mac_address")
    #[schemars(description = "The field name in snake_case (e.g., 'device', 'mac_address')")]
    field_name: String,
}

#[tool_router]
impl McpServer {
    async fn new() -> Result<Self> {
        // Try to load API data from the data directory - navigate to mcp/data/
        let mcp_data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("data");
        // ApiData::load_from_dir expects the parent data directory, not api_definitions
        // because it looks for data_dir/guides/ subdirectory
        info!("Loading API data from {}", mcp_data_dir.display());
        info!("Looking for guides in: {}", mcp_data_dir.join("guides").display());
        let api_data = ApiData::load_from_dir(&mcp_data_dir)?;
        info!("Loaded {} guide chunks into memory", api_data.guides.len());
        if api_data.guides.is_empty() {
            warn!("⚠️  WARNING: No guide chunks loaded! Check that guides are in {}", mcp_data_dir.join("guides").display());
        } else {
            // Log a sample guide to verify they're loading correctly
            if let Some(first_guide) = api_data.guides.values().next() {
                info!("Sample guide loaded: {} > {} > {} (chunk_id: {})", 
                    first_guide.heading_h1, 
                    first_guide.heading_h2, 
                    first_guide.heading_h3,
                    first_guide.chunk_id);
            }
        }

        #[cfg(feature = "embeddings")]
        let (embedding_model, embeddings_db) = {
            let embeddings_path = mcp_data_dir.join("embeddings.bin");
            let model_cache_dir = mcp_data_dir.join("model_cache");

            // Create cache directory if it doesn't exist
            if !model_cache_dir.exists() {
                std::fs::create_dir_all(&model_cache_dir)?;
            }

            if embeddings_path.exists() {
                info!("Loading embeddings from {}", embeddings_path.display());
                info!("Using model cache directory: {}", model_cache_dir.display());

                // Load embedding model with persistent cache
                // Configure execution providers: CUDA if available, fallback to CPU
                #[cfg(feature = "cuda")]
                let init_options = {
                    info!("CUDA feature enabled - attempting GPU acceleration");
                    InitOptions::new(EmbeddingModel::AllMiniLML6V2)
                        .with_cache_dir(model_cache_dir)
                        .with_show_download_progress(false)
                        .with_execution_providers(vec![
                            CUDAExecutionProvider::default().build()
                        ])
                };

                #[cfg(not(feature = "cuda"))]
                let init_options = InitOptions::new(EmbeddingModel::AllMiniLML6V2)
                    .with_cache_dir(model_cache_dir)
                    .with_show_download_progress(false);

                match TextEmbedding::try_new(init_options) {
                    Ok(model) => {
                        info!("Embedding model loaded successfully");

                        // Load database from binary file
                        let file = File::open(&embeddings_path).context("Failed to open embeddings file");
                        match file {
                            Ok(f) => {
                                let reader = BufReader::new(f);
                                match bincode::deserialize_from(reader) {
                                    Ok(db) => {
                                        info!("Loaded embeddings database");
                                        (Some(Arc::new(Mutex::new(model))), Some(Arc::new(db)))
                                    },
                                    Err(e) => {
                                        warn!("Failed to deserialize embeddings: {}", e);
                                        (Some(Arc::new(Mutex::new(model))), None)
                                    }
                                }
                            }
                            Err(e) => {
                                warn!("Failed to open embeddings file: {}", e);
                                (Some(Arc::new(Mutex::new(model))), None)
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to load embedding model: {}", e);
                        (None, None)
                    }
                }
            } else {
                info!("Embeddings database not found, semantic search will be unavailable");
                (None, None)
            }
        };

        Ok(Self {
            tool_router: Self::tool_router(),
            api_data: Arc::new(api_data),
            #[cfg(feature = "embeddings")]
            embedding_model,
            #[cfg(feature = "embeddings")]
            embeddings_db,
        })
    }

    /// Get a specific code example by name
    #[tool(description = "Get a specific vim_rs code example by name. Returns the complete source code, description, and Cargo.toml dependencies. Use list_examples to see all available examples.")]
    async fn get_example(&self, params: Parameters<GetExampleInput>) -> Result<CallToolResult, McpError> {
        let name = &params.0.name;

        let example = self.api_data.examples.get(name);

        if let Some(ex) = example {
            let output = format!(
                "# {}\n\n**Category:** {}\n\n## Description\n\n{}\n\n## Source Code\n\n```rust\n{}\n```\n\n## Dependencies (Cargo.toml)\n\n```toml\n{}\n```\n\n**File:** `examples/{}`\n",
                ex.title,
                ex.category,
                ex.description,
                ex.source_code,
                ex.dependencies,
                ex.file_path
            );
            Ok(CallToolResult::success(vec![Content::text(output)]))
        } else {
            let message = format!("Example '{}' not found. Use list_examples to see all available examples.", name);
            Ok(CallToolResult::success(vec![Content::text(message)]))
        }
    }

    /// Get a specific guide section by chunk ID
    #[tool(description = "Get a specific vSphere/VCF guide section by chunk_id. Returns complete content from admin documentation guides. Use semantic_search with filter='guides' to find relevant sections first.")]
    async fn get_guide(&self, params: Parameters<GetGuideInput>) -> Result<CallToolResult, McpError> {
        let chunk_id = &params.0.chunk_id;

        let guide = self.api_data.guides.get(chunk_id);

        if let Some(g) = guide {
            let topics = if g.topics.is_empty() {
                "None".to_string()
            } else {
                g.topics.join(", ")
            };

            let sub_section = g.sub_section.as_ref()
                .map(|s| format!(" - {}", s))
                .unwrap_or_default();

            let mut output = format!(
                "# {} > {} > {}{}\n\n**Source:** {}\n\n**Topics:** {}\n\n**Word Count:** {}\n\n## Content\n\n{}\n",
                g.heading_h1,
                g.heading_h2,
                g.heading_h3,
                sub_section,
                g.source_file,
                topics,
                g.word_count,
                g.content
            );
            if g.chunk_count > 1 {
                output.push_str(&format!("This is part {} of {} parts.", g.chunk_index , g.chunk_count));
            }
            Ok(CallToolResult::success(vec![Content::text(output)]))
        } else {
            let message = format!(
                "Guide section '{}' not found. Use semantic_search with filter='guides' to find relevant guide sections.",
                chunk_id
            );
            Ok(CallToolResult::success(vec![Content::text(message)]))
        }
    }

    /// Get comprehensive vim_rs starter guide
    #[tool(description = "CALL THIS FIRST! Returns the complete vim_rs starter guide with connection patterns, property collector usage, code snippets, and best practices. Essential for writing correct vim_rs code on the first try.")]
    async fn get_starter_guide(&self, _params: Parameters<GetStarterGuideInput>) -> Result<CallToolResult, McpError> {
        // Load the starter guide from the server guides directory
        let guide_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("guides")
            .join("VIM_RS_STARTER_GUIDE.md");

        let content = if guide_path.exists() {
            match std::fs::read_to_string(&guide_path) {
                Ok(content) => content,
                Err(e) => {
                    return Ok(CallToolResult::success(vec![Content::text(format!(
                        "Error reading starter guide: {}. Use search_examples or get_example to find usage patterns.",
                        e
                    ))]));
                }
            }
        } else {
            return Ok(CallToolResult::success(vec![Content::text(format!(
                "Error: Starter guide file not found. Use search_examples or get_example to find usage patterns.",
            ))]));
        };

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    /// List all supported managed object types
    #[tool(description = "Returns a list of all vSphere property collector root managed object types. These types can be used with get_property_info to explore their properties. Returns the top-level types like VirtualMachine, HostSystem, Datacenter, etc.")]
    async fn list_property_collector_root_types(&self, _params: Parameters<ListManagedObjectTypesInput>) -> Result<CallToolResult, McpError> {
        let types = property_collector::get_managed_object_types();

        let mut output = String::from("# Supported Managed Object Types\n\n");
        output.push_str(&format!("Total: {} types\n\n", types.len()));
        output.push_str("Use these types with `get_property_info` to explore their properties.\n\n");

        for (idx, mo_type) in types.iter().enumerate() {
            output.push_str(&format!("{}. **{}**\n", idx + 1, mo_type.name));
        }

        output.push_str("\n## Example Usage\n\n");
        output.push_str("```\n");
        output.push_str("get_property_info(managed_object=\"VirtualMachine\", property_path=\"\")\n");
        output.push_str("get_property_info(managed_object=\"VirtualMachine\", property_path=\"guest.ip_address\")\n");
        output.push_str("```\n");

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    /// Get property information for a managed object and property path
    #[tool(description = "Get detailed information about a property path for a managed object. Provide the managed object type (e.g., 'VirtualMachine') and an optional property path in snake_case (e.g., 'guest.ip_address'). If property_path is empty, returns all top-level properties. Returns the VIM path, Rust type, optionality, documentation, and child fields if it's a complex type.")]
    async fn get_property_info(&self, params: Parameters<GetPropertyInfoInput>) -> Result<CallToolResult, McpError> {
        let managed_object = &params.0.managed_object;
        let property_path = &params.0.property_path;

        let info = match property_collector::get_property_info(managed_object, property_path) {
            Ok(info) => info,
            Err(e) => {
                return Ok(CallToolResult::success(vec![Content::text(format!(
                    "Error getting property info for {}.{}:\n\n{}\n\n\
                    Use `list_managed_object_types` to see all supported types.",
                    managed_object,
                    property_path,
                    e
                ))]));
            }
        };
        let mut output = String::new();

        if property_path.is_empty() {
            output.push_str(&format!("# Top-Level Properties for {}\n\n", managed_object));
        } else {
            output.push_str(&format!("# Property: {}.{}\n\n", managed_object, property_path));
        }

        output.push_str(&format!("**VIM Path:** `{}`\n", info.vim_path));
        output.push_str(&format!("**Rust Type:** `{}`\n", info.rust_type));
        output.push_str(&format!("**Optional:** {}\n\n", info.is_optional));

        if let Some(doc) = &info.documentation {
            output.push_str("## Documentation\n\n");
            output.push_str(doc);
            output.push_str("\n\n");
        }

        if let Some(children) = &info.child_fields {
            output.push_str("## Child Properties\n\n");
            output.push_str(&format!("Found {} child properties:\n\n", children.len()));

            for child in children {
                output.push_str(&format!("### `{}`\n", child.field_name));
                output.push_str(&format!("- **VIM Name:** `{}`\n", child.vim_name));
                output.push_str(&format!("- **Rust Type:** `{}`\n", child.rust_type));
                output.push_str(&format!("- **Optional:** {}\n", child.is_optional));

                if let Some(doc) = &child.documentation {
                    // Truncate doc to first 200 chars for child fields
                    let doc_preview = if doc.len() > 200 {
                        format!("{}...", &doc[..200])
                    } else {
                        doc.clone()
                    };
                    output.push_str(&format!("- **Description:** {}\n", doc_preview));
                }
                output.push_str("\n");
            }

            output.push_str("\n## Example Usage in vim_retrievable!\n\n");
            output.push_str("```rust\n");
            output.push_str("vim_retrievable!(\n");
            output.push_str(&format!("    struct My{}: {} {{\n", managed_object, managed_object));
            for child in children.iter().take(5) {
                let full_path = if property_path.is_empty() {
                    child.field_name.clone()
                } else {
                    format!("{}.{}", property_path, child.field_name)
                };
                output.push_str(&format!("        {} = \"{}\",\n", child.field_name, full_path));
            }
            output.push_str("    }\n");
            output.push_str(");\n");
            output.push_str("```\n");
        }

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    /// Get comprehensive type information for structs, traits, or enums
    #[tool(description = "Get comprehensive information about any vim_rs type including managed objects (VirtualMachine, Datastore), structs (VirtualDevice), traits (VirtualDeviceTrait), and enums (ManagedEntityStatus). Essential for understanding data types, type hierarchies and polymorphism.")]
    async fn get_type(&self, params: Parameters<GetTypeInput>) -> Result<CallToolResult, McpError> {
        let type_name = &params.0.type_name;
        let mut output = String::new();

        // Look up trait
        if let Some(trait_info) = self.api_data.traits.get(type_name) {
            output.push_str(&format!("# Trait: {}\n\n", trait_info.rust_name));
            output.push_str(&format!("**Module:** `{}`\n", trait_info.rust_module));
            output.push_str(&format!("**Original Type:** `{}`\n\n", trait_info.name));

            if let Some(desc) = &trait_info.description {
                output.push_str("## Description\n\n");
                output.push_str(desc);
                output.push_str("\n\n");
            }

            if let Some(parent) = &trait_info.parent_trait {
                output.push_str(&format!("**Parent Trait:** `{}`\n\n", parent));
            }

            // Getter methods
            if !trait_info.getters.is_empty() {
                output.push_str("## Getter Methods\n\n");
                for getter in &trait_info.getters {
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
                    output.push_str("\n");
                }
            }

            // Implementing types
            if !trait_info.implementing_types.is_empty() {
                output.push_str(&format!("## Implementing Types ({} types)\n\n", trait_info.implementing_types.len()));
                for impl_type in trait_info.implementing_types.iter().take(20) {
                    output.push_str(&format!("- `{}`\n", impl_type));
                }
                if trait_info.implementing_types.len() > 20 {
                    output.push_str(&format!("\n... and {} more\n", trait_info.implementing_types.len() - 20));
                }
                output.push_str("\n");
            }

            // Usage example
            output.push_str("## Usage Example\n\n");
            output.push_str("```rust\n");
            output.push_str("use vim_rs::types::convert::CastInto;\n");
            output.push_str(&format!("use vim_rs::types::traits::{};\n\n", trait_info.rust_name));
            output.push_str("// Cast from parent trait to this trait\n");
            if let Some(parent) = &trait_info.parent_trait {
                output.push_str(&format!("let device: &dyn {} = /* ... */;\n", parent));
                output.push_str(&format!("if let Some(specialized): Option<&dyn {}> = device.as_ref().into_ref() {{\n", trait_info.rust_name));
            } else {
                output.push_str(&format!("let device: Box<dyn {}> = /* ... */;\n", trait_info.rust_name));
                output.push_str("if let Some(specialized) = device.as_ref() {\n");
            }
            if !trait_info.getters.is_empty() {
                let getter = &trait_info.getters[0];
                output.push_str(&format!("    let value = specialized.{}();\n", getter.name));
            }
            output.push_str("}\n");
            output.push_str("```\n");

            return Ok(CallToolResult::success(vec![Content::text(output)]));
        }

        // Check if it's a managed object (try rust_struct first, then name)
        let mo_info = self.api_data.managed_objects.get(type_name)
            .or_else(|| {
                self.api_data.managed_objects_by_name.get(type_name)
                    .and_then(|rust_struct| self.api_data.managed_objects.get(rust_struct))
            });
        
        if let Some(mo_info) = mo_info {
            output.push_str(&format!("# Managed Object: {}\n\n", mo_info.rust_struct));
            output.push_str(&format!("**Module:** `{}`\n", mo_info.rust_module));
            output.push_str(&format!("**VIM Type:** `{}`\n\n", mo_info.name));

            if let Some(desc) = &mo_info.description {
                output.push_str("## Description\n\n");
                output.push_str(desc);
                output.push_str("\n\n");
            }

            // Methods - just list names, use get_method tool for details
            if !mo_info.methods.is_empty() {
                output.push_str(&format!("## Methods ({} methods)\n\n", mo_info.methods.len()));
                output.push_str("Use the `get_method` tool to view detailed information about a specific method.\n\n");
                for method in &mo_info.methods {
                    output.push_str(&format!("`{}`, ", method.rust_name));
                }
                output.push_str("\n");
            }

            return Ok(CallToolResult::success(vec![Content::text(output)]));
        }

        // Check if it's a struct (try rust_name first, then name)
        let struct_info = self.api_data.data_structures.get(type_name)
            .or_else(|| {
                self.api_data.data_structures_by_name.get(type_name)
                    .and_then(|rust_name| self.api_data.data_structures.get(rust_name))
            });
        
        if let Some(struct_info) = struct_info {
            output.push_str(&format!("# Struct: {}\n\n", struct_info.rust_name));
            output.push_str(&format!("**Module:** `{}`\n", struct_info.rust_module));
            output.push_str(&format!("**Emit Mode:** {}\n\n", struct_info.emit_mode));

            if let Some(desc) = &struct_info.description {
                output.push_str("## Description\n\n");
                output.push_str(desc);
                output.push_str("\n\n");
            }

            // Inheritance chain
            if !struct_info.inheritance_chain.is_empty() {
                output.push_str("## Inheritance Chain\n\n");
                output.push_str(&struct_info.inheritance_chain.join(" → "));
                output.push_str("\n\n");
            }

            // Implemented traits
            if !struct_info.implements_traits.is_empty() {
                output.push_str("## Implemented Traits\n\n");
                for trait_name in &struct_info.implements_traits {
                    output.push_str(&format!("- `{}`\n", trait_name));
                }
                output.push_str("\n");
            }

            // Direct children
            if !struct_info.children.is_empty() {
                output.push_str(&format!("## Direct Children ({} types)\n\n", struct_info.children.len()));
                for child in struct_info.children.iter().take(10) {
                    output.push_str(&format!("- `{}`\n", child));
                }
                if struct_info.children.len() > 10 {
                    output.push_str(&format!("\n... and {} more\n", struct_info.children.len() - 10));
                }
                output.push_str("\n");
            }

            // All descendants (recursive)
            if !struct_info.all_descendants.is_empty() {
                output.push_str(&format!("## All Descendants ({} types total)\n\n", struct_info.all_descendants.len()));
                for desc in struct_info.all_descendants.iter().take(15) {
                    output.push_str(&format!("- `{}`\n", desc));
                }
                if struct_info.all_descendants.len() > 15 {
                    output.push_str(&format!("\n... and {} more\n", struct_info.all_descendants.len() - 15));
                }
                output.push_str("\n");
            }

            // Fields
            if !struct_info.fields.is_empty() {
                output.push_str(&format!("## Fields ({} fields)\n\n", struct_info.fields.len()));
                for field in &struct_info.fields {
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
                    output.push_str("\n");
                }
            }

            return Ok(CallToolResult::success(vec![Content::text(output)]));
        }

        // Check if it's an enum (try rust_name first, then name)
        let enum_info = self.api_data.enumerations.get(type_name)
            .or_else(|| {
                self.api_data.enumerations_by_name.get(type_name)
                    .and_then(|rust_name| self.api_data.enumerations.get(rust_name))
            });
        
        if let Some(enum_info) = enum_info {
            output.push_str(&format!("# Enum: {}\n\n", enum_info.rust_name));
            output.push_str(&format!("**Module:** `{}`\n\n", enum_info.rust_module));

            if let Some(desc) = &enum_info.description {
                output.push_str("## Description\n\n");
                output.push_str(desc);
                output.push_str("\n\n");
            }

            output.push_str(&format!("## Variants ({} variants)\n\n", enum_info.variants.len()));
            for variant in &enum_info.variants {
                output.push_str(&format!("### `{}`\n", variant.rust_name));
                output.push_str(&format!("- **VIM Value:** `{}`\n", variant.discriminator_value));
                if let Some(desc) = &variant.description {
                    output.push_str(&format!("- **Description:** {}\n", desc));
                }
                output.push_str("\n");
            }

            output.push_str("## Usage Example\n\n");
            output.push_str("```rust\n");
            output.push_str(&format!("use vim_rs::types::enums::{};\n\n", enum_info.rust_name));
            output.push_str(&format!("match value {{\n"));
            for variant in enum_info.variants.iter().take(3) {
                output.push_str(&format!("    {}::{} => {{ /* ... */ }}\n", enum_info.rust_name, variant.rust_name));
            }
            if enum_info.variants.len() > 3 {
                output.push_str("    // ...\n");
            }
            output.push_str("}\n");
            output.push_str("```\n");

            return Ok(CallToolResult::success(vec![Content::text(output)]));
        }

        // Type not found
        let error_msg = format!(
            "Type '{}' not found.\n\n\
            This tool supports:\n\
            - Managed Objects: e.g., 'VirtualMachine', 'Datastore', 'HostSystem'\n\
            - Structs: e.g., 'VirtualDevice', 'VirtualEthernetCard', 'VirtualMachineConfigInfo'\n\
            - Traits: e.g., 'VirtualDeviceTrait', 'VirtualEthernetCardTrait'\n\
            - Enums: e.g., 'ManagedEntityStatus', 'VirtualMachinePowerState'\n\n\
            Use the `search` tool to find type names.",
            type_name
        );
        Ok(CallToolResult::success(vec![Content::text(error_msg)]))
    }

    /// Get detailed information about a specific method on a managed object
    #[tool(description = "Get comprehensive information about a specific method on a managed object. Returns signature, parameters, return type, description, and related types. Use this after get_type_info to explore individual methods in detail.")]
    async fn get_method(&self, params: Parameters<GetMethodInput>) -> Result<CallToolResult, McpError> {
        let managed_object = &params.0.managed_object;
        let method_name = &params.0.method_name;
        let mut output = String::new();

        // Find the managed object (try rust_struct first, then name)
        let mo_info = self.api_data.managed_objects.get(managed_object)
            .or_else(|| {
                self.api_data.managed_objects_by_name.get(managed_object)
                    .and_then(|rust_struct| self.api_data.managed_objects.get(rust_struct))
            });

        let mo_info = match mo_info {
            Some(mo) => mo,
            None => {
                let error_msg = format!(
                    "Managed object '{}' not found.\n\n\
                    Use the `get_type_info` tool to find valid managed object names.",
                    managed_object
                );
                return Ok(CallToolResult::success(vec![Content::text(error_msg)]));
            }
        };

        // Find the method
        let method = mo_info.methods.iter().find(|m|
            m.name == *method_name || m.rust_name == *method_name
        );

        let method = match method {
            Some(m) => m,
            None => {
                let error_msg = format!(
                    "Method '{}' not found on managed object '{}'.\n\n\
                    Available methods:\n{}\n\n\
                    Use the `get_type_info` tool with type_name='{}' to see all available methods.",
                    method_name,
                    mo_info.rust_struct,
                    mo_info.methods.iter()
                        .take(10)
                        .map(|m| format!("- {}", m.rust_name))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    mo_info.rust_struct
                );
                return Ok(CallToolResult::success(vec![Content::text(error_msg)]));
            }
        };

        // Build detailed method information
        output.push_str(&format!("# {}::{}\n\n", mo_info.rust_struct, method.rust_name));
        output.push_str(&format!("**VIM Method:** `{}`\n\n", method.name));

        // Signature
        output.push_str("## Signature\n\n");
        output.push_str("```rust\n");
        output.push_str(&method.signature.full);
        output.push_str("\n```\n\n");

        // Description
        if let Some(desc) = &method.description {
            output.push_str("## Description\n\n");
            output.push_str(desc);
            output.push_str("\n\n");
        }

        // Parameters
        if !method.signature.parameters.is_empty() {
            output.push_str("## Parameters\n\n");
            for param in &method.signature.parameters {
                output.push_str(&format!("### `{}: {}`\n\n", param.name, param.rust_type));
                output.push_str(&format!("**Required:** {}\n\n", if param.required { "Yes" } else { "No" }));
                if let Some(param_desc) = &param.description {
                    output.push_str(param_desc);
                    output.push_str("\n\n");
                }
            }
        }

        // Return type
        output.push_str("## Return Type\n\n");
        output.push_str(&format!("`{}`\n\n", method.signature.return_type));

        // Async
        if method.signature.is_async {
            output.push_str("**Async:** Yes - This method returns a Future and should be awaited.\n\n");
        }

        // Related types
        if !method.related_types.is_empty() {
            output.push_str("## Related Types\n\n");
            for related_type in &method.related_types {
                output.push_str(&format!("- `{}`\n", related_type));
            }
            output.push_str("\n");
        }

        // Usage example
        output.push_str("## Usage Example\n\n");
        output.push_str("```rust\n");
        output.push_str(&format!("use vim_rs::types::{};\n\n", mo_info.rust_module));
        output.push_str(&format!("let obj: {} = /* ... */;\n", mo_info.rust_struct));
        
        // Build parameter list
        let param_list = if method.signature.parameters.is_empty() {
            String::new()
        } else {
            method.signature.parameters.iter()
                .map(|p| {
                    if p.rust_type.starts_with("Option<") {
                        format!("{}: None", p.name)
                    } else {
                        format!("{}: /* {} */", p.name, p.rust_type)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        
        if method.signature.is_async {
            output.push_str(&format!("let result = obj.{}({}).await?;\n", method.rust_name, param_list));
        } else {
            output.push_str(&format!("let result = obj.{}({})?;\n", method.rust_name, param_list));
        }
        output.push_str("```\n");

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    /// Get detailed field information including type, documentation, and ALL field paths from managed objects.
    #[tool(description = "Get comprehensive field information including type, documentation, and ALL field paths from managed objects. Use this to discover how to access deeply nested fields in vim_retrievable! macros.")]
    async fn get_field(&self, params: Parameters<GetFieldInput>) -> Result<CallToolResult, McpError> {
        let owner_type = &params.0.owner_type;
        let field_name = &params.0.field_name;
        
        // Find field in data_structures
        if let Some(field) = self.api_data.get_field(owner_type, field_name) {
        
            let mut output = String::new();
            output.push_str(&format!("# Field: {}.{}\n\n", owner_type, field_name));
            output.push_str(&format!("**VIM Name:** `{}`\n", field.name));
            output.push_str(&format!("**Rust Type:** `{}`\n", field.rust_type));
            output.push_str(&format!("**Required:** {}\n\n", field.required));
            
            if field.is_array {
                output.push_str("**Array:** Yes\n\n");
            }
            
            if let Some(doc) = &field.description {
                output.push_str("## Documentation\n\n");
                output.push_str(doc);
                output.push_str("\n\n");
            }
            
            // Referenced type docs
            let struct_info = self.api_data.get_data_structure(owner_type);
            if let Some(struct_info) = struct_info {
                if let Some(ref type_doc) = struct_info.description {
                    output.push_str(&format!("## Part of Structure: {}\n\n", struct_info.rust_name));
                    output.push_str(type_doc);
                    output.push_str("\n\n");
                    
                    if !struct_info.all_descendants.is_empty() {
                        output.push_str(&format!(
                            "*Polymorphic type with {} descendants. Use CastInto trait for downcasting.*\n\n",
                            struct_info.all_descendants.len()
                        ));
                    }
                }
            }
            Ok(CallToolResult::success(vec![Content::text(output)]))
        }
        else {
            return Ok(CallToolResult::success(vec![Content::text(format!(
                "Field '{}.{}' not found. Use search with filter='fields'.",
                owner_type, field_name
            ))]));
        }
    }

    /// Semantic search using natural language queries (requires embeddings)
    #[cfg(feature = "embeddings")]
    #[tool(description = "Search vSphere documentation using natural language queries. Returns Rust managed objects, methods, structures, enums, examples, and guides based on meaning, not just keywords. Use filter='guides' to search only documentation guides.")]
    async fn search(&self, params: Parameters<SemanticSearchInput>) -> Result<CallToolResult, McpError> {
        // Check if embeddings are available
        if self.embedding_model.is_none() || self.embeddings_db.is_none() {
            let message = "Semantic search is not available. Embeddings database not found.".to_string();
            return Ok(CallToolResult::success(vec![Content::text(message)]));
        }

        let embedding_model = self.embedding_model.as_ref().unwrap();
        let embeddings_db = self.embeddings_db.as_ref().unwrap();

        // Generate embedding for query
        let query_embedding = {
            let mut model = embedding_model.lock().unwrap();
            match model.embed(vec![params.0.query.clone()], None) {
                Ok(mut embeddings) => {
                    if embeddings.is_empty() {
                        return Err(McpError::internal_error("Failed to generate query embedding".to_string(), None));
                    }
                    embeddings.remove(0)
                }
                Err(e) => {
                    return Err(McpError::internal_error(format!("Failed to generate query embedding: {}", e), None));
                }
            }
        };

        // Perform parallel search using Rayon
        let mut scores: Vec<(usize, f32)> = embeddings_db.vectors.par_iter()
            .enumerate()
            .map(|(idx, vec)| {
                // Dot product (vectors are normalized)
                let score: f32 = vec.iter().zip(&query_embedding).map(|(a, b)| a * b).sum();
                (idx, score)
            })
            .collect();

        // Sort by score descending
        scores.par_sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Filter and take top N
        let limit = params.0.limit;
        let filter = &params.0.filter;
        
        let mut formatted_results = Vec::new();
        let mut count = 0;

        for (idx, _score) in scores {
            if count >= limit {
                break;
            }

            let record = &embeddings_db.records[idx];
            
            // Apply filter
            if filter != "all" {
                let filter_type = match filter.as_str() {
                    "methods" => "method",
                    "structures" => "structure",
                    "enums" => "enum",
                    "examples" => "example",
                    "guides" => "guide",
                    "managed_objects" => "managed_object",
                    "fields" => "field",
                    _ => "all"
                };
                
                if record.item_type != filter_type {
                    continue;
                }
            }

            // Format result based on item type
            let details = match record.item_type.as_str() {
                "managed_object" => {                    
                    if let Some(managed_object) = self.api_data.get_managed_object(&record.object_name) {
                        Some(format!(
                            "## {}\n\n**Rust Struct:** `{}`\n\n**Rust Module:** `{}`\n\n**Description:**\n{}\n\n---\n",
                            managed_object.name,
                            managed_object.rust_struct,
                            managed_object.rust_module,
                            managed_object.description.as_deref().unwrap_or("No description")
                        ))
                    } else {
                        None
                    }
                }
                "method" => {
                    // Find the method
                    let mut result = None;
                    if let Some(mo) = self.api_data.get_managed_object(&record.object_name) {
                        if let Some(method) = mo.methods.iter().find(|m| m.name == record.item_name) {
                            let desc = method.description.as_deref().unwrap_or("No description");
                            result = Some(format!(
                                "## {}.{}\n\n**Rust:** `{}`\n\n**Signature:**\n```rust\n{}\n```\n\n**Description:**\n{}\n\n**Related Types:** {}\n\n---\n",
                                mo.rust_struct,
                                method.name,
                                method.rust_name,
                                method.signature.full,
                                desc,
                                method.related_types.join(", ")
                            ));
                        }
                    }
                    result
                }
                "structure" => {
                    if let Some(structure) = self.api_data.get_data_structure(&record.item_name) {
                        let desc = structure.description.as_deref().unwrap_or("No description");
                        let parent_info = structure.parent.as_ref()
                            .map(|p| format!(" (extends {})", p))
                            .unwrap_or_default();

                        let mut field_list = String::new();
                        for field in structure.fields.iter().take(10) {
                            let req = if field.required { "required" } else { "optional" };
                            field_list.push_str(&format!("  - `{}`: {} ({})\n", field.name, field.vim_type, req));
                        }
                        if structure.fields.len() > 10 {
                            field_list.push_str(&format!("  ... and {} more fields\n", structure.fields.len() - 10));
                        }

                        Some(format!(
                            "## {}{}\n\n**Rust:** `{}`\n\n**Description:**\n{}\n\n**Fields:**\n{}\n**Related Types:** {}\n\n---\n",
                            structure.name,
                            parent_info,
                            structure.rust_name,
                            desc,
                            field_list,
                            structure.related_types.join(", ")
                        ))
                    } else {
                        None
                    }
                }
                "enum" => {
                    let enumeration = self.api_data.enumerations_by_name.get(&record.item_name)
                        .and_then(|rust_name| self.api_data.enumerations.get(rust_name));
                    
                    if let Some(enumeration) = enumeration {
                        let desc = enumeration.description.as_deref().unwrap_or("No description");

                        let mut variant_list = String::new();
                        for variant in &enumeration.variants {
                            let variant_desc = variant.description.as_deref().unwrap_or("");
                            variant_list.push_str(&format!("  - `{}`: {}\n", variant.name, variant_desc));
                        }

                        Some(format!(
                            "## {}\n\n**Rust:** `{}`\n\n**Description:**\n{}\n\n**Variants:**\n{}\n---\n",
                            enumeration.name,
                            enumeration.rust_name,
                            desc,
                            variant_list
                        ))
                    } else {
                        None
                    }
                }
                "example" => {
                    if let Some(example) = self.api_data.examples.get(&record.item_name) {
                        Some(format!(
                            "## {} (Example)\n\n**Category:** {}\n\n**Description:**\n{}\n\n**Usage:**\n```\nget_example(\"{}\")\n```\n\n---\n",
                            example.title,
                            example.category,
                            example.description.lines().take(3).collect::<Vec<_>>().join(" "),
                            example.name
                        ))
                    } else {
                        None
                    }
                }
                "guide" => {
                    if let Some(guide) = self.api_data.guides.get(&record.item_name) {
                        let topics = if guide.topics.is_empty() {
                            "None".to_string()
                        } else {
                            guide.topics.join(", ")
                        };

                        let sub_section = guide.sub_section.as_ref()
                            .map(|s| format!(" - {}", s))
                            .unwrap_or_default();

                        // Truncate content to ~500 chars for preview
                        let content_preview = if guide.content.len() > 500 {
                            format!("{}...", &guide.content[..500])
                        } else {
                            guide.content.clone()
                        };

                        Some(format!(
                            "## {} > {} > {}{}\n\n**Source:** {}\n\n**Topics:** {}\n\n**Content:**\n{}\n\n**Usage:**\n```\nget_guide(\"{}\")\n```\n\n---\n",
                            guide.heading_h1,
                            guide.heading_h2,
                            guide.heading_h3,
                            sub_section,
                            guide.source_file,
                            topics,
                            content_preview,
                            guide.chunk_id
                        ))
                    } else {
                        None
                    }
                }
                "field" => {
                    // Find the field - match on BOTH structure name and field name
                    let field_result = self.api_data.data_structures.get(&record.object_name)
                        .and_then(|s| {
                            s.fields.iter()
                                .find(|f| f.rust_name == record.item_name)
                                .map(|f| (s, f))
                        });
                    
                    if let Some((structure, field)) = field_result {
                        
                        Some(format!(
                            "## {} (Field in {})\n\n**Type:** `{}`\n\n{}\n```\nget_field(\"{}\", \"{}\")\n```\n\n---\n",
                            field.rust_name,
                            structure.rust_name,
                            field.rust_type,
                            field.description.as_deref().unwrap_or("No description"),
                            structure.rust_name,
                            field.rust_name
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            };

            if let Some(detail) = details {
                formatted_results.push(detail);
                count += 1;
            }
        }

        if formatted_results.is_empty() {
            let message = format!("No results found for query: '{}'", params.0.query);
            Ok(CallToolResult::success(vec![Content::text(message)]))
        } else {
            let filter_info = if params.0.filter != "all" {
                format!(" (filtered by: {})", params.0.filter)
            } else {
                String::new()
            };
            let message = format!(
                "Found {} semantic match(es) for '{}'{filter_info}:\n\n{}",
                formatted_results.len(),
                params.0.query,
                formatted_results.join("\n")
            );
            Ok(CallToolResult::success(vec![Content::text(message)]))
        }
    }
}

#[tool_handler]
impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation{
                name: env!("CARGO_CRATE_NAME").to_owned(),
                title: Some("vSphere API MCP Server for Rust".into()),
                version: env!("CARGO_PKG_VERSION").to_owned(),
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "🎯 START HERE: Call get_workflow_guide() FIRST to learn the correct vim_rs patterns!\n\n\
                This MCP server provides comprehensive vSphere API documentation for Rust development using vim_rs.\n\n\
                THREE-TIER KNOWLEDGE SYSTEM:\n\
                1. API Reference: search_methods, search_types, search_enums → WHAT exists\n\
                2. Code Examples: list_examples, get_example, search_examples → HOW to code it\n\
                3. Admin Guides: search_guides, get_guide, semantic_search(filter='guides') → WHEN/WHY/GOTCHAS\n\n\
                🆕 PROPERTY COLLECTOR HELPERS:\n\
                - list_managed_object_types → List all supported managed object types (VirtualMachine, HostSystem, etc.)\n\
                - get_property_info → Explore property paths and their types (e.g., VirtualMachine.guest.ip_address)\n\
                Use these to discover valid property paths for vim_retrievable! macro.\n\n\
                ⚠️ CRITICAL FOR POLYMORPHIC TYPES:\n\
                vim_rs uses TRAITS for polymorphic types, NOT enums!\n\
                - VirtualDevice is `Box<dyn VirtualDeviceTrait>`, not an enum\n\
                - Use CastInto trait: `device.as_ref().into_ref()` to cast between traits\n\
                - Import `vim_rs::types::convert::CastInto` when working with polymorphic types\n\n\
                🚨 MOST COMMON MISTAKE - Getting MAC addresses:\n\
                DON'T downcast to every NIC type (VirtualE1000, VirtualE1000E, etc.)\n\
                DO cast to VirtualEthernetCardTrait once:\n\
                  let Some(eth): Option<&dyn VirtualEthernetCardTrait> = device.as_ref().into_ref() else { continue };\n\
                  eth.get_mac_address()  // Works for ALL NIC types!\n\
                See workflow guide for complete example.\n\n\
                CRITICAL: Always use ClientBuilder and vim_retrievable! macro (see workflow guide).\n\
                Never manually construct PropertyCollector specs or fetch objects one-by-one.\n\n\
                For semantic/natural language queries: semantic_search (requires embeddings).\n\n\
                ⚠️ IMPORTANT: This server covers vim_rs (Rust) only, not Python/Go/Java/PowerCLI bindings."
                .to_string()
            ),
            ..Default::default()
        }
    }

}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to stderr
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Starting MCP server");

    // Create the server instance
    let server = McpServer::new().await?;

    // Serve using stdio transport
    info!("MCP server ready");
    let service = server.serve((stdin(), stdout()))
        .await
        .inspect_err(|e| error!("Error serving server: {}", e))?;

    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_server() -> Result<()> {
        let router = McpServer::tool_router();
        assert!(router.has_route("search_methods"));
        assert!(router.has_route("search_types"));
        assert!(router.has_route("search_enums"));

        #[cfg(feature = "embeddings")]
        assert!(router.has_route("semantic_search"));

        // Note: Can't test McpServer::new() here without data files
        // let mcp_server = McpServer::new()?;
        // let tool_router = &mcp_server.tool_router;
        // assert!(tool_router.has_route("hello"));

        Ok(())
    }
}
