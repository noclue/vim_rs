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
use fastembed::{InitOptions, TextEmbedding};
use rayon::prelude::*;

// Import data model from the library
use vim_mcp_server::model::{ApiDatabase, ApiItem, load_embedded_database, STARTER_GUIDE};
use vim_mcp_server::property_collector;
use vim_mcp_server::EMBEDDING_MODEL;

// CUDA GPU acceleration (optional)
#[cfg(feature = "cuda")]
use ort::execution_providers::CUDAExecutionProvider;

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
    #[schemars(description = "Filter results by type: 'all', 'managed_objects', 'methods', 'structures', 'enums', 'fields', or 'examples' (default: 'all')")]
    #[serde(default = "default_filter")]
    filter: String,
}

fn default_limit() -> usize {
    10
}

fn default_filter() -> String {
    "all".to_string()
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

            // Configure execution providers: CUDA if available, fallback to CPU
            #[cfg(feature = "cuda")]
            let init_options = {
                info!("CUDA feature enabled - attempting GPU acceleration");
                InitOptions::new(EMBEDDING_MODEL)
                    .with_cache_dir(model_cache_dir)
                    .with_show_download_progress(false)
                    .with_execution_providers(vec![
                        CUDAExecutionProvider::default().build()
                    ])
            };

            #[cfg(not(feature = "cuda"))]
            let init_options = InitOptions::new(EMBEDDING_MODEL)
                .with_cache_dir(model_cache_dir)
                .with_show_download_progress(false);

            match TextEmbedding::try_new(init_options) {
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
            if filter != "all" {
                let filter_type = match filter.as_str() {
                    "methods" => "method",
                    "structures" => "structure",
                    "enums" => "enum",
                    "examples" => "example",
                    "managed_objects" => "managed_object",
                    "fields" => "field",
                    "traits" => "trait",
                    _ => "all"
                };
                
                if item.item_type() != filter_type {
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
                PROPERTY COLLECTOR HELPERS:\n\
                - list_property_collector_root_types → List all supported managed object types\n\
                - get_property_info → Explore property paths and their types\n\n\
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
        
        // Verify unified get tool exists
        assert!(router.has_route("get"));
        
        // Verify specialized tools exist
        assert!(router.has_route("get_starter_guide"));
        assert!(router.has_route("get_property_info"));
        assert!(router.has_route("list_property_collector_root_types"));
        assert!(router.has_route("search"));

        Ok(())
    }
}
