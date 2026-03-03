use anyhow::Result;
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
use std::sync::{Arc, Mutex};
use tokio::io::{stdin, stdout};
use tracing::{error, info, warn};
use clap::Parser;
#[cfg(not(feature = "embed-model"))]
use fastembed::InitOptions;
#[cfg(feature = "embed-model")]
use fastembed::InitOptionsUserDefined;
use fastembed::TextEmbedding;
use rayon::prelude::*;

// Import data model from the library
use vim_mcp_server::model::{ApiDatabase, ApiItem, load_embedded_database, STARTER_GUIDE};
use vim_mcp_server::property_collector;
#[cfg(not(feature = "embed-model"))]
use vim_mcp_server::EMBEDDING_MODEL;

// Web UI module (optional)
#[cfg(feature = "web-ui")]
mod web_ui;

#[cfg(feature = "cuda")]
use ort::execution_providers::CUDAExecutionProvider;
#[cfg(feature = "coreml")]
use ort::execution_providers::{CoreMLExecutionProvider, coreml::{ModelFormat, ComputeUnits}};

// ============================================================================
// CLI Arguments
// ============================================================================

/// CLI arguments for vim_mcp_server
#[derive(Parser, Debug)]
#[command(name = "vim_mcp_server")]
#[command(about = "vSphere API MCP Server for Rust", long_about = None)]
struct Cli {
    /// Enable web UI mode (disables stdio MCP mode)
    #[arg(short = 'w', long = "web")]
    web: bool,
    
    /// Web server port (default: 8080)
    #[arg(short = 'p', long = "port", default_value = "8080")]
    port: u16,
    
    /// Bind address (default: 127.0.0.1)
    #[arg(short = 'b', long = "bind", default_value = "127.0.0.1")]
    bind: String,
}

// ============================================================================
// MCP Server
// ============================================================================

/// McpServer - A Model Context Protocol server
#[derive(Clone)]
pub struct McpServer {
    tool_router: ToolRouter<Self>,
    /// Unified API database with items and embeddings
    api_db: Arc<ApiDatabase>,
    /// Embedding model for runtime query embedding
    embedding_model: Option<Arc<Mutex<TextEmbedding>>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
enum SearchFilter {
    All,
    ManagedObjects,
    Methods,
    Structures,
    Enums,
    Fields,
    Examples,
    Traits,
}

impl Default for SearchFilter {
    fn default() -> Self {
        Self::All
    }
}

impl SearchFilter {
    fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::ManagedObjects => "managed_object",
            Self::Methods => "method",
            Self::Structures => "structure",
            Self::Enums => "enum",
            Self::Fields => "field",
            Self::Examples => "example",
            Self::Traits => "trait",
        }
    }
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
    #[schemars(description = "Filter results by type")]
    #[serde(default)]
    filter: SearchFilter,
}

fn default_limit() -> usize {
    10
}

/// Input parameters for unified get tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetInput {
    /// Item ID to retrieve
    #[schemars(description = "Item ID to retrieve. Examples: 'VirtualMachine' (managed object), 'VirtualMachine::power_on_vm_task' (method), 'VirtualHardware::device' (field), 'VirtualDevice' (struct), 'ManagedEntityStatus' (enum), 'VirtualDeviceTrait' (trait), 'example::connection_basic'")]
    id: String,
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

/// Input parameters for get_property_path tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetPropertyPathInput {
    /// Managed object type (e.g., "VirtualMachine", "HostSystem", "Folder")
    #[schemars(description = "The managed object type name (e.g., 'VirtualMachine', 'HostSystem', 'Datacenter')")]
    managed_object: String,

    /// Property path in snake_case (e.g., "guest.ip_address" or empty for top-level fields)
    #[schemars(description = "Property path in snake_case format (e.g., 'guest.ip_address', 'config.hardware.device') or empty string to list top-level fields")]
    #[serde(default)]
    property_path: String,
}

/// Input parameters for get_property_tree tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetPropertyTreeInput {
    /// Managed object type (e.g., "VirtualMachine", "HostSystem", "Folder")
    #[schemars(description = "The managed object type name (e.g., 'VirtualMachine', 'HostSystem', 'Folder')")]
    managed_object: String,

    /// Optional starting path to show a subtree (e.g., "config.hardware")
    #[schemars(description = "Optional property path in snake_case to start the tree from (e.g., 'config.hardware'). If empty, shows from root.")]
    #[serde(default)]
    start_path: String,

    /// Maximum depth to traverse (1-5, default 5)
    #[schemars(description = "Maximum depth to traverse (1-5). Use lower values to reduce output size. Default is 5.")]
    #[serde(default = "default_depth")]
    depth: u8,

    /// Optional regex pattern to filter property paths
    #[schemars(description = "Optional regex pattern to filter property paths. Only paths matching the pattern are included (e.g., 'mac|address' to find MAC-related fields).")]
    #[serde(default)]
    filter: String,
}

fn default_depth() -> u8 {
    5
}

#[tool_router]
impl McpServer {
    async fn new() -> Result<Self> {
        // Load unified API database from embedded binary
        info!("Loading embedded API database...");
        let api_db = load_embedded_database()?;
        
        info!(
            "Loaded {} items: {} managed objects, {} methods, {} structures, {} fields, {} enums, {} traits, {} examples",
            api_db.items.len(),
            api_db.count_by_type("managed_object"),
            api_db.count_by_type("method"),
            api_db.count_by_type("structure"),
            api_db.count_by_type("field"),
            api_db.count_by_type("enum"),
            api_db.count_by_type("trait"),
            api_db.count_by_type("example"),
        );

        if api_db.has_embeddings() {
            info!("Embeddings available: {} vectors", 
                api_db.embeddings.as_ref().map(|e| e.len()).unwrap_or(0));
        } else {
            warn!("No embeddings in database, semantic search will be unavailable");
        }

        // Initialize embedding model for runtime query embedding
        let embedding_model = {
            // When embed-model feature is enabled, use the compiled-in model
            #[cfg(feature = "embed-model")]
            let result = {
                info!("Loading embedded BGE-small-en-v1.5 model...");
                let user_model = vim_mcp_server::embedded_model::create_embedded_model();
                
                #[cfg(any(feature = "cuda", feature = "coreml"))]
                let init_options = {
                    let mut providers = Vec::new();

                    #[cfg(feature = "cuda")]
                    {
                        info!("CUDA feature enabled - attempting GPU acceleration");
                        providers.push(CUDAExecutionProvider::default().build());
                    }

                    #[cfg(feature = "coreml")]
                    {
                        info!("CoreML feature enabled - using Apple Neural Engine acceleration");
                        providers.push(
                            CoreMLExecutionProvider::default()
                                .with_model_format(ModelFormat::MLProgram)
                                .with_static_input_shapes(true)
                                .with_compute_units(ComputeUnits::CPUAndGPU)
                                .build()
                        );
                    }

                    InitOptionsUserDefined::new()
                        .with_execution_providers(providers)
                };
                
                #[cfg(not(any(feature = "cuda", feature = "coreml")))]
                let init_options = InitOptionsUserDefined::new();
                
                TextEmbedding::try_new_from_user_defined(user_model, init_options)
            };
            
            // When embed-model is disabled, load from cache directory
            #[cfg(not(feature = "embed-model"))]
            let result = {
                let mcp_data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join("data");
                let model_cache_dir = mcp_data_dir.join("model_cache");

                // Create cache directory if it doesn't exist
                if !model_cache_dir.exists() {
                    std::fs::create_dir_all(&model_cache_dir)?;
                }

                info!("Loading embedding model from cache: {}", model_cache_dir.display());

                #[cfg(any(feature = "cuda", feature = "coreml"))]
                let init_options = {
                    let mut providers = Vec::new();

                    #[cfg(feature = "cuda")]
                    {
                        info!("CUDA feature enabled - attempting GPU acceleration");
                        providers.push(CUDAExecutionProvider::default().build());
                    }

                    #[cfg(feature = "coreml")]
                    {
                        info!("CoreML feature enabled - using Apple Neural Engine acceleration");
                        let coreml_cache = model_cache_dir.join("coreml_cache");
                        std::fs::create_dir_all(&coreml_cache).ok();
                        providers.push(
                            CoreMLExecutionProvider::default()
                                .with_model_format(ModelFormat::MLProgram)
                                .with_static_input_shapes(true)
                                .with_compute_units(ComputeUnits::CPUAndGPU)
                                .with_model_cache_dir(coreml_cache.display().to_string())
                                .build()
                        );
                    }

                    InitOptions::new(EMBEDDING_MODEL)
                        .with_cache_dir(model_cache_dir)
                        .with_show_download_progress(false)
                        .with_execution_providers(providers)
                };

                #[cfg(not(any(feature = "cuda", feature = "coreml")))]
                let init_options = InitOptions::new(EMBEDDING_MODEL)
                    .with_cache_dir(model_cache_dir)
                    .with_show_download_progress(false);

                TextEmbedding::try_new(init_options)
            };
            
            match result {
                Ok(model) => {
                    info!("Embedding model loaded successfully");
                    Some(Arc::new(Mutex::new(model)))
                }
                Err(e) => {
                    warn!("Failed to load embedding model: {}", e);
                    None
                }
            }
        };

        Ok(Self {
            tool_router: Self::tool_router(),
            api_db: Arc::new(api_db),
            embedding_model,
        })
    }

    /// Unified get tool for retrieving any API item by ID
    #[tool(description = "Get detailed information about any vim_rs API item by ID. Supports managed objects, methods, structures, fields, enums, traits, examples, and guides. Use search to find IDs.")]
    async fn get(&self, params: Parameters<GetInput>) -> Result<CallToolResult, McpError> {
        let id = &params.0.id;
        
        if let Some(item) = self.api_db.get(id) {
            Ok(CallToolResult::success(vec![Content::text(item.detailed_document())]))
        } else {
            // Try to provide helpful suggestions
            let mut suggestions = Vec::new();
            
            // Check for close matches in items
            for item_id in self.api_db.items.keys().take(1000) {
                if item_id.to_lowercase().contains(&id.to_lowercase()) 
                   || id.to_lowercase().contains(&item_id.to_lowercase()) {
                    suggestions.push(format!("- `{}`", item_id));
                    if suggestions.len() >= 5 {
                        break;
                    }
                }
            }
            
            let suggestions_text = if suggestions.is_empty() {
                String::new()
            } else {
                format!("\n\n**Did you mean:**\n{}", suggestions.join("\n"))
            };
            
            let msg = format!(
                "Item '{}' not found.{}\n\n\
                **ID Format Examples:**\n\
                - Managed Object: `VirtualMachine`\n\
                - Method: `VirtualMachine::power_on_vm_task`\n\
                - Structure: `VirtualDevice`\n\
                - Field: `VirtualHardware::device`\n\
                - Enum: `ManagedEntityStatus`\n\
                - Trait: `VirtualDeviceTrait`\n\
                - Example: `example::connection_basic`\n\n\
                Use `search` to find valid IDs.",
                id,
                suggestions_text
            );
            Ok(CallToolResult::success(vec![Content::text(msg)]))
        }
    }

    /// Get comprehensive vim_rs starter guide
    #[tool(description = "CALL THIS FIRST! Returns the complete vim_rs starter guide with connection patterns, property collector usage, code snippets, and best practices. Essential for writing correct vim_rs code on the first try.")]
    async fn get_starter_guide(&self, _params: Parameters<GetStarterGuideInput>) -> Result<CallToolResult, McpError> {
        // Return the embedded starter guide (compiled into the binary)
        Ok(CallToolResult::success(vec![Content::text(STARTER_GUIDE)]))
    }

    /// List all supported managed object types
    #[tool(description = "Returns a list of all vSphere property collector root managed object types. These types can be used with get_property_path to explore their properties. Returns the top-level types like VirtualMachine, HostSystem, Datacenter, etc.")]
    async fn list_property_collector_root_types(&self, _params: Parameters<ListManagedObjectTypesInput>) -> Result<CallToolResult, McpError> {
        let types = property_collector::get_managed_object_types();

        let mut output = String::from("# Supported Managed Object Types\n\n");
        output.push_str(&format!("Total: {} types\n\n", types.len()));
        output.push_str("Use these types with `get_property_path` to explore their properties.\n\n");

        for (idx, mo_type) in types.iter().enumerate() {
            output.push_str(&format!("{}. **{}**\n", idx + 1, mo_type.name));
        }

        output.push_str("\n## Example Usage\n\n");
        output.push_str("```\n");
        output.push_str("get_property_path(managed_object=\"VirtualMachine\", property_path=\"\")\n");
        output.push_str("get_property_path(managed_object=\"VirtualMachine\", property_path=\"guest.ip_address\")\n");
        output.push_str("```\n");

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    /// Get property information for a managed object and property path
    #[tool(description = "Get detailed information about a property path for a managed object. Provide the managed object type (e.g., 'VirtualMachine') and an optional property path in snake_case (e.g., 'guest.ip_address'). If property_path is empty, returns all top-level properties. Returns the VIM path, Rust type, optionality, documentation, and child fields if it's a complex type.")]
    async fn get_property_path(&self, params: Parameters<GetPropertyPathInput>) -> Result<CallToolResult, McpError> {
        let managed_object = &params.0.managed_object;
        let property_path = &params.0.property_path;

        let info = match property_collector::get_property_path(managed_object, property_path) {
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

        output.push_str(&format!("**Rust Type:** `{}`\n", info.rust_type));

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
                output.push_str(&format!("- **Rust Type:** `{}`\n", child.rust_type));

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

    /// Get the complete property tree for a managed object type
    #[tool(description = "Get property paths for a managed object type. Shows flat dot-separated paths (e.g., 'config.hardware.device') with Rust types. Optional-ness propagates from parent to child. Use 'depth' (1-5) to limit output. Use 'filter' (regex) to include only matching paths.")]
    async fn get_property_tree(&self, params: Parameters<GetPropertyTreeInput>) -> Result<CallToolResult, McpError> {
        let managed_object = &params.0.managed_object;
        let start_path = &params.0.start_path;
        let depth = params.0.depth as usize;
        let filter = params.0.filter.trim();
        let filter_regex = if filter.is_empty() {
            None
        } else {
            match regex::Regex::new(filter) {
                Ok(re) => Some(re),
                Err(e) => {
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "Invalid regex filter `{}`: {}\n\n\
                        Provide a valid Rust-flavour regex, or omit the filter to return all paths.",
                        filter, e
                    ))]));
                }
            }
        };

        match property_collector::get_property_tree(managed_object, start_path, depth, filter_regex.as_ref()) {
            Ok(tree) => {
                let title = if start_path.is_empty() {
                    managed_object.to_string()
                } else {
                    format!("{}.{}", managed_object, start_path)
                };
                let output = format!(
                    "# Property Paths: {}\n\n```\n{}\n```\n\n*Depth: {}.*",
                    title,
                    tree.trim_end(),
                    depth.clamp(1, 5)
                );
                Ok(CallToolResult::success(vec![Content::text(output)]))
            }
            Err(e) => {
                Ok(CallToolResult::success(vec![Content::text(format!(
                    "Error getting property tree for {}:\n\n{}\n\n\
                    Use `list_property_collector_root_types` to see all supported types.",
                    managed_object,
                    e
                ))]))
            }
        }
    }

    /// Semantic search using natural language queries
    #[tool(description = "Search vSphere API documentation using natural language queries. Returns Rust managed objects, methods, structures, enums, and examples based on meaning, not just keywords.")]
    async fn search(&self, params: Parameters<SemanticSearchInput>) -> Result<CallToolResult, McpError> {
        // Check if embeddings are available
        let embeddings = match &self.api_db.embeddings {
            Some(e) => e,
            None => {
                let message = "Semantic search is not available. No embeddings in database.".to_string();
                return Ok(CallToolResult::success(vec![Content::text(message)]));
            }
        };

        let embedding_model = match &self.embedding_model {
            Some(m) => m,
            None => {
                let message = "Semantic search is not available. Embedding model not loaded.".to_string();
                return Ok(CallToolResult::success(vec![Content::text(message)]));
            }
        };

        // Generate embedding for query
        let query_embedding = {
            let mut model = embedding_model.lock().unwrap();
            match model.embed(vec![params.0.query.clone()], None) {
                Ok(mut embs) => {
                    if embs.is_empty() {
                        return Err(McpError::internal_error("Failed to generate query embedding".to_string(), None));
                    }
                    embs.remove(0)
                }
                Err(e) => {
                    return Err(McpError::internal_error(format!("Failed to generate query embedding: {}", e), None));
                }
            }
        };

        // Perform parallel search using Rayon
        // Embeddings are aligned with items by index
        let mut scores: Vec<(usize, f32)> = embeddings.par_iter()
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

        // Get items as a vec for indexing (IndexMap preserves insertion order)
        let items: Vec<_> = self.api_db.items.iter().collect();

        for (idx, _score) in scores {
            if count >= limit {
                break;
            }

            // Get the item at this index
            let (_id, item) = &items[idx];
            
            // Apply filter
            if *filter != SearchFilter::All {
                if item.item_type() != filter.as_str() {
                    continue;
                }
            }

            formatted_results.push(item.search_summary());
            count += 1;
        }

        if formatted_results.is_empty() {
            let message = format!("No results found for query: '{}'", params.0.query);
            Ok(CallToolResult::success(vec![Content::text(message)]))
        } else {
            let filter_info = if *filter != SearchFilter::All {
                format!(" (filtered by: {})", filter.as_str())
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

// Additional methods for web UI direct access
impl McpServer {
    /// List all available tools (for web UI)
    pub fn list_tools(&self) -> Vec<Tool> {
        McpServer::tool_router().list_all()
    }
    
    /// Call a tool directly (for web UI, bypassing JSON-RPC)
    pub async fn call_tool_direct(&self, tool_name: &str, arguments: serde_json::Value) -> Result<CallToolResult> {
        match tool_name {
            "get" => {
                let params: GetInput = serde_json::from_value(arguments)?;
                self.get(Parameters(params)).await.map_err(|e| anyhow::anyhow!("{:?}", e))
            }
            "get_starter_guide" => {
                let params: GetStarterGuideInput = serde_json::from_value(arguments.clone()).unwrap_or(GetStarterGuideInput {});
                self.get_starter_guide(Parameters(params)).await.map_err(|e| anyhow::anyhow!("{:?}", e))
            }
            "list_property_collector_root_types" => {
                let params: ListManagedObjectTypesInput = serde_json::from_value(arguments.clone()).unwrap_or(ListManagedObjectTypesInput {});
                self.list_property_collector_root_types(Parameters(params)).await.map_err(|e| anyhow::anyhow!("{:?}", e))
            }
            "get_property_path" => {
                let params: GetPropertyPathInput = serde_json::from_value(arguments)?;
                self.get_property_path(Parameters(params)).await.map_err(|e| anyhow::anyhow!("{:?}", e))
            }
            "get_property_tree" => {
                let params: GetPropertyTreeInput = serde_json::from_value(arguments)?;
                self.get_property_tree(Parameters(params)).await.map_err(|e| anyhow::anyhow!("{:?}", e))
            }
            "search" => {
                let params: SemanticSearchInput = serde_json::from_value(arguments)?;
                self.search(Parameters(params)).await.map_err(|e| anyhow::anyhow!("{:?}", e))
            }
            _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_name)),
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
                "🎯 START HERE: Call get_starter_guide() FIRST to learn the correct vim_rs patterns!\n\n\
                This MCP server provides comprehensive vSphere API documentation for Rust development using vim_rs.\n\n\
                UNIFIED API:\n\
                - search(query) → Find items by natural language query\n\
                - get(id) → Get detailed info for any item by ID\n\n\
                ID FORMATS:\n\
                - Managed Object: 'VirtualMachine'\n\
                - Method: 'VirtualMachine::power_on_vm_task'\n\
                - Structure: 'VirtualDevice'\n\
                - Field: 'VirtualHardware::device'\n\
                - Enum: 'ManagedEntityStatus'\n\
                - Trait: 'VirtualDeviceTrait'\n\
                - Example: 'example::connection_basic'\n\n\
                PATH NOTATION (shown in search results):\n\
                - `::` = property/method on Managed Object, `.` = field access\n\
                - `?` = optional, `[*]` = array, `→` = downcast, `⇒` = trait cast\n\
                - Example: `VirtualMachine::config?.hardware.device[*]→VirtualEthernetCard`\n\n\
                PROPERTY COLLECTOR HELPERS:\n\
                - list_property_collector_root_types → List all supported managed object types\n\
                - get_property_path → Explore property paths and their types\n\n\
                ⚠️ CRITICAL FOR POLYMORPHIC TYPES:\n\
                vim_rs uses TRAITS for polymorphic types, NOT enums!\n\
                - VirtualDevice is `Box<dyn VirtualDeviceTrait>`, not an enum\n\
                - Use CastInto trait: `device.as_ref().into_ref()` to cast between traits\n\
                - Import `vim_rs::types::convert::CastInto` when working with polymorphic types\n\n\
                CRITICAL: Always use ClientBuilder and vim_retrievable! macro (see starter guide).\n\
                Never manually construct PropertyCollector specs or fetch objects one-by-one.\n\n\
                ⚠️ IMPORTANT: This server covers vim_rs (Rust) only, not Python/Go/Java/PowerCLI bindings."
                .to_string()
            ),
            ..Default::default()
        }
    }

}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();
    
    // Initialize logging to stderr
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Create the server instance
    let server = McpServer::new().await?;

    // Choose mode based on CLI flags
    if cli.web {
        #[cfg(feature = "web-ui")]
        {
            info!("Starting MCP server in web UI mode");
            web_ui::start_server(server, &cli.bind, cli.port).await?;
        }
        
        #[cfg(not(feature = "web-ui"))]
        {
            error!("Web UI feature not enabled. Rebuild with --features web-ui");
            return Err(anyhow::anyhow!("Web UI feature not enabled"));
        }
    } else {
        // Serve using stdio transport (default)
        info!("Starting MCP server in stdio mode");
        info!("MCP server ready");
        let service = server.serve((stdin(), stdout()))
            .await
            .inspect_err(|e| error!("Error serving server: {}", e))?;

        service.waiting().await?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_server() -> Result<()> {
        let router = McpServer::tool_router();
        
        // Verify unified get tool exists
        assert!(router.has_route("get"));
        
        // Verify specialized tools exist
        assert!(router.has_route("get_starter_guide"));
        assert!(router.has_route("get_property_path"));
        assert!(router.has_route("get_property_tree"));
        assert!(router.has_route("list_property_collector_root_types"));
        assert!(router.has_route("search"));

        Ok(())
    }
}
