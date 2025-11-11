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
use std::sync::Arc;
use tokio::io::{stdin, stdout};
use tracing::{error, info, warn};

// Import data model from the library
use vim_mcp_server::model::ApiData;
use vim_mcp_server::property_collector;

// Conditional imports for embeddings feature
#[cfg(feature = "embeddings")]
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
#[cfg(feature = "embeddings")]
use lancedb::{Connection, query::{ExecutableQuery, QueryBase}};
#[cfg(feature = "embeddings")]
use std::sync::Mutex;

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
    embeddings_db: Option<Arc<Connection>>,
}

/// Input parameters for search tools
#[derive(Serialize, Deserialize, JsonSchema)]
struct SearchInput {
    /// Search query
    #[schemars(description = "The search query to find matching items")]
    query: String,

    /// Maximum number of results to return
    #[schemars(description = "Maximum number of results to return (default: 10)")]
    #[serde(default = "default_limit")]
    limit: usize,
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
    #[schemars(description = "Filter results by type: 'all', 'methods', 'types', 'enums', or 'examples' (default: 'all')")]
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

/// Input parameters for search_examples tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct SearchExamplesInput {
    /// Search query for examples
    #[schemars(description = "Search query - can be a category (connection, property_collector, events, etc.) or keyword")]
    query: String,
}

/// Input parameters for list_examples tool (empty struct for consistency)
#[derive(Serialize, Deserialize, JsonSchema)]
struct ListExamplesInput {
    // Empty - list_examples takes no parameters but needs object schema
}

/// Input parameters for get_guide tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetGuideInput {
    /// Chunk ID of the guide section to retrieve
    #[schemars(description = "The chunk_id of the guide section (e.g., 'installing-esx-understanding-auto-deploy')")]
    chunk_id: String,
}

/// Input parameters for search_guides tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct SearchGuidesInput {
    /// Search query for guides
    #[schemars(description = "Search query - keywords from topic, heading, or content (e.g., 'auto deploy', 'host profiles', 'dpu')")]
    query: String,
}

/// Input parameters for get_workflow_guide tool (empty struct for consistency)
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetWorkflowGuideInput {
    // Empty - get_workflow_guide takes no parameters but needs object schema
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
        // Try to load API data from the data directory - navigate to mcp/data/
        let mcp_data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("data");
        let api_definitions_dir = mcp_data_dir.join("api_definitions");
        let api_data = ApiData::load_from_dir(&api_definitions_dir)?;

        #[cfg(feature = "embeddings")]
        let (embedding_model, embeddings_db) = {
            let embeddings_db_path = mcp_data_dir.join("embeddings.lancedb");
            let model_cache_dir = mcp_data_dir.join("model_cache");

            // Create cache directory if it doesn't exist
            if !model_cache_dir.exists() {
                std::fs::create_dir_all(&model_cache_dir)?;
            }

            if embeddings_db_path.exists() {
                info!("Loading embeddings from {}", embeddings_db_path.display());
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

                        // Connect to LanceDB (now just await it)
                        match lancedb::connect(&embeddings_db_path.to_string_lossy())
                            .execute()
                            .await
                        {
                            Ok(db) => {
                                info!("Connected to embeddings database");
                                (Some(Arc::new(Mutex::new(model))), Some(Arc::new(db)))
                            }
                            Err(e) => {
                                warn!("Failed to connect to embeddings database: {}", e);
                                (None, None)
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

    /// Search for vSphere API methods by name or description (Rust bindings only)
    #[tool(description = "Search for vSphere API methods by name or description. Returns Rust method signatures and usage from the vim_rs crate.")]
    async fn search_methods(&self, params: Parameters<SearchInput>) -> Result<CallToolResult, McpError> {
        let query = params.0.query.to_lowercase();
        let limit = params.0.limit;
        let mut results = Vec::new();
        let mut count = 0;

        for mo in &self.api_data.managed_objects {
            for method in &mo.methods {
                if count >= limit {
                    break;
                }

                let name_match = method.name.to_lowercase().contains(&query);
                let desc_match = method.description.as_ref()
                    .map(|d| d.to_lowercase().contains(&query))
                    .unwrap_or(false);

                if name_match || desc_match {
                    let desc = method.description.as_deref().unwrap_or("No description");
                    let result = format!(
                        "## {}.{}\n\n**Rust:** `{}`\n\n**Signature:**\n```rust\n{}\n```\n\n**Description:**\n{}\n\n**Related Types:** {}\n\n---\n",
                        mo.name,
                        method.name,
                        method.rust_name,
                        method.signature.full,
                        desc,
                        method.related_types.join(", ")
                    );
                    results.push(result);
                    count += 1;
                }
            }
            if count >= limit {
                break;
            }
        }

        if results.is_empty() {
            let message = format!("No methods found matching '{}'", params.0.query);
            Ok(CallToolResult::success(vec![Content::text(message)]))
        } else {
            let message = format!("Found {} method(s) matching '{}':\n\n{}",
                results.len(), params.0.query, results.join("\n"));
            Ok(CallToolResult::success(vec![Content::text(message)]))
        }
    }

    /// Search for vSphere API data structures by name or description (Rust types only)
    #[tool(description = "Search for vSphere API data structures by name or description. Returns Rust struct definitions and field information from the vim_rs crate.")]
    async fn search_types(&self, params: Parameters<SearchInput>) -> Result<CallToolResult, McpError> {
        let query = params.0.query.to_lowercase();
        let limit = params.0.limit;
        let mut results = Vec::new();

        for structure in &self.api_data.data_structures {
            if results.len() >= limit {
                break;
            }

            let name_match = structure.name.to_lowercase().contains(&query);
            let desc_match = structure.description.as_ref()
                .map(|d| d.to_lowercase().contains(&query))
                .unwrap_or(false);

            if name_match || desc_match {
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

                let result = format!(
                    "## {}{}\n\n**Rust:** `{}`\n\n**Description:**\n{}\n\n**Fields:**\n{}\n**Related Types:** {}\n\n---\n",
                    structure.name,
                    parent_info,
                    structure.rust_name,
                    desc,
                    field_list,
                    structure.related_types.join(", ")
                );
                results.push(result);
            }
        }

        if results.is_empty() {
            let message = format!("No data structures found matching '{}'", params.0.query);
            Ok(CallToolResult::success(vec![Content::text(message)]))
        } else {
            let message = format!("Found {} data structure(s) matching '{}':\n\n{}",
                results.len(), params.0.query, results.join("\n"));
            Ok(CallToolResult::success(vec![Content::text(message)]))
        }
    }

    /// Search for vSphere API enumerations by name or description (Rust enums only)
    #[tool(description = "Search for vSphere API enumerations by name or description. Returns Rust enum definitions and variant information from the vim_rs crate.")]
    async fn search_enums(&self, params: Parameters<SearchInput>) -> Result<CallToolResult, McpError> {
        let query = params.0.query.to_lowercase();
        let limit = params.0.limit;
        let mut results = Vec::new();

        for enumeration in &self.api_data.enumerations {
            if results.len() >= limit {
                break;
            }

            let name_match = enumeration.name.to_lowercase().contains(&query);
            let desc_match = enumeration.description.as_ref()
                .map(|d| d.to_lowercase().contains(&query))
                .unwrap_or(false);

            if name_match || desc_match {
                let desc = enumeration.description.as_deref().unwrap_or("No description");

                let mut variant_list = String::new();
                for variant in &enumeration.variants {
                    let variant_desc = variant.description.as_deref().unwrap_or("");
                    variant_list.push_str(&format!("  - `{}`: {}\n", variant.name, variant_desc));
                }

                let result = format!(
                    "## {}\n\n**Rust:** `{}`\n\n**Description:**\n{}\n\n**Variants:**\n{}\n---\n",
                    enumeration.name,
                    enumeration.rust_name,
                    desc,
                    variant_list
                );
                results.push(result);
            }
        }

        if results.is_empty() {
            let message = format!("No enumerations found matching '{}'", params.0.query);
            Ok(CallToolResult::success(vec![Content::text(message)]))
        } else {
            let message = format!("Found {} enumeration(s) matching '{}':\n\n{}",
                results.len(), params.0.query, results.join("\n"));
            Ok(CallToolResult::success(vec![Content::text(message)]))
        }
    }

    /// List all available code examples with categories
    #[tool(description = "List all available vim_rs code examples organized by category. Use this to discover examples for connection, property collector, macros, events, and more.")]
    async fn list_examples(&self, _params: Parameters<ListExamplesInput>) -> Result<CallToolResult, McpError> {
        let mut by_category: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

        for example in &self.api_data.examples {
            by_category
                .entry(example.category.clone())
                .or_insert_with(Vec::new)
                .push(format!("- **{}** - {}", example.name, example.title));
        }

        let mut categories: Vec<_> = by_category.keys().cloned().collect();
        categories.sort();

        let mut output = String::from("# vim_rs Code Examples\n\n");
        output.push_str(&format!("Total examples: {}\n\n", self.api_data.examples.len()));

        for category in categories {
            let examples = by_category.get(&category).unwrap();
            output.push_str(&format!("## {} ({} examples)\n", category, examples.len()));
            for example in examples {
                output.push_str(&format!("{}\n", example));
            }
            output.push('\n');
        }

        output.push_str("Use `get_example` with the example name to see the full code.\n");

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    /// Get a specific code example by name
    #[tool(description = "Get a specific vim_rs code example by name. Returns the complete source code, description, and Cargo.toml dependencies. Use list_examples to see all available examples.")]
    async fn get_example(&self, params: Parameters<GetExampleInput>) -> Result<CallToolResult, McpError> {
        let name = &params.0.name;

        let example = self.api_data.examples.iter()
            .find(|e| e.name == *name);

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

    /// Search code examples by category or keyword
    #[tool(description = "Search vim_rs code examples by category (connection, property_collector, macro_usage, events, performance, general) or keyword in title/description.")]
    async fn search_examples(&self, params: Parameters<SearchExamplesInput>) -> Result<CallToolResult, McpError> {
        let query = params.0.query.to_lowercase();
        let mut results = Vec::new();

        for example in &self.api_data.examples {
            let category_match = example.category.to_lowercase().contains(&query);
            let title_match = example.title.to_lowercase().contains(&query);
            let desc_match = example.description.to_lowercase().contains(&query);
            let name_match = example.name.to_lowercase().contains(&query);

            if category_match || title_match || desc_match || name_match {
                results.push(format!(
                    "**{}** ({})\n{}\nUse: `get_example(\"{}\")`\n",
                    example.title,
                    example.category,
                    example.description.lines().next().unwrap_or(""),
                    example.name
                ));
            }
        }

        if results.is_empty() {
            let message = format!(
                "No examples found matching '{}'. Available categories: connection, property_collector, macro_usage, events, performance, general",
                query
            );
            Ok(CallToolResult::success(vec![Content::text(message)]))
        } else {
            let message = format!(
                "Found {} example(s) matching '{}':\n\n{}",
                results.len(),
                query,
                results.join("\n")
            );
            Ok(CallToolResult::success(vec![Content::text(message)]))
        }
    }

    /// Get a specific guide section by chunk ID
    #[tool(description = "Get a specific vSphere/VCF guide section by chunk_id. Returns complete content from admin documentation guides. Use semantic_search with filter='guides' to find relevant sections first.")]
    async fn get_guide(&self, params: Parameters<GetGuideInput>) -> Result<CallToolResult, McpError> {
        let chunk_id = &params.0.chunk_id;

        let guide = self.api_data.guides.iter()
            .find(|g| g.chunk_id == *chunk_id);

        if let Some(g) = guide {
            let topics = if g.topics.is_empty() {
                "None".to_string()
            } else {
                g.topics.join(", ")
            };

            let sub_section = g.sub_section.as_ref()
                .map(|s| format!(" - {}", s))
                .unwrap_or_default();

            let output = format!(
                "# {} > {}{}\n\n**Source:** {}\n\n**Topics:** {}\n\n**Word Count:** {}\n\n## Content\n\n{}\n",
                g.heading_h2,
                g.heading_h3,
                sub_section,
                g.source_file,
                topics,
                g.word_count,
                g.content
            );
            Ok(CallToolResult::success(vec![Content::text(output)]))
        } else {
            let message = format!(
                "Guide section '{}' not found. Use semantic_search with filter='guides' to find relevant guide sections.",
                chunk_id
            );
            Ok(CallToolResult::success(vec![Content::text(message)]))
        }
    }

    /// Search guide sections by keyword
    #[tool(description = "Search vSphere/VCF documentation guides by keyword in headings, topics, or content. Returns matching guide sections from admin documentation.")]
    async fn search_guides(&self, params: Parameters<SearchGuidesInput>) -> Result<CallToolResult, McpError> {
        let query = params.0.query.to_lowercase();
        let mut results = Vec::new();

        for guide in &self.api_data.guides {
            let h2_match = guide.heading_h2.to_lowercase().contains(&query);
            let h3_match = guide.heading_h3.to_lowercase().contains(&query);
            let topics_match = guide.topics.iter().any(|t| t.to_lowercase().contains(&query));
            let content_match = guide.content.to_lowercase().contains(&query);

            if h2_match || h3_match || topics_match || content_match {
                let sub_section = guide.sub_section.as_ref()
                    .map(|s| format!(" - {}", s))
                    .unwrap_or_default();

                let content_preview = if guide.content.len() > 150 {
                    format!("{}...", &guide.content[..150])
                } else {
                    guide.content.clone()
                };

                results.push(format!(
                    "**{} > {}{}**\n{}\nUse: `get_guide(\"{}\")`\n",
                    guide.heading_h2,
                    guide.heading_h3,
                    sub_section,
                    content_preview,
                    guide.chunk_id
                ));
            }
        }

        if results.is_empty() {
            let message = format!(
                "No guide sections found matching '{}'. Try semantic_search with filter='guides' for better results.",
                query
            );
            Ok(CallToolResult::success(vec![Content::text(message)]))
        } else {
            let message = format!(
                "Found {} guide section(s) matching '{}':\n\n{}",
                results.len(),
                query,
                results.join("\n")
            );
            Ok(CallToolResult::success(vec![Content::text(message)]))
        }
    }

    /// Get comprehensive vim_rs workflow guide
    #[tool(description = "CALL THIS FIRST! Returns the complete vim_rs workflow guide with connection patterns, property collector usage, code snippets, and best practices. Essential for writing correct vim_rs code on the first try.")]
    async fn get_workflow_guide(&self, _params: Parameters<GetWorkflowGuideInput>) -> Result<CallToolResult, McpError> {
        // Load the workflow guide from the server guides directory
        let guide_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("guides")
            .join("VIM_RS_WORKFLOW_GUIDE.md");

        let content = if guide_path.exists() {
            match std::fs::read_to_string(&guide_path) {
                Ok(content) => content,
                Err(e) => {
                    return Ok(CallToolResult::success(vec![Content::text(format!(
                        "Error reading workflow guide: {}. Use search_examples or get_example to find usage patterns.",
                        e
                    ))]));
                }
            }
        } else {
            // Fallback minimal guide if file not found
            r#"# vim_rs Quick Start

## Connection Pattern (ALWAYS USE THIS)
```rust
use anyhow::{Context, Result};
use std::sync::Arc;
use vim_rs::{Client, ClientBuilder};

pub async fn connect(app_name: &str, app_version: &str) -> Result<Arc<Client>> {
    let vc_server = std::env::var("VIM_SERVER").context("VIM_SERVER env var not set")?;
    let username = std::env::var("VIM_USERNAME").context("VIM_USERNAME env var not set")?;
    let pwd = std::env::var("VIM_PASSWORD").context("VIM_PASSWORD env var not set")?;

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(app_name, app_version)
        .build()
        .await?;

    Ok(client)
}
```

## Data Retrieval Pattern (ALWAYS USE THIS)
```rust
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;

vim_retrievable!(
    struct Host: HostSystem {
        name = "name",
        power_state = "runtime.powerState",
    }
);

let retriever = ObjectRetriever::new(client.clone())?;
let hosts: Vec<Host> = retriever
    .retrieve_objects_from_container(&client.service_content().root_folder)
    .await?;
```

For complete guide, use: list_examples, get_example, search_examples
"#.to_string()
        };

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    /// List all supported managed object types
    #[tool(description = "Returns a list of all vSphere managed object types supported by the property collector system. These types can be used with get_property_info to explore their properties. Returns the top-level types like VirtualMachine, HostSystem, Datacenter, etc.")]
    async fn list_managed_object_types(&self, _params: Parameters<ListManagedObjectTypesInput>) -> Result<CallToolResult, McpError> {
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

        match property_collector::get_property_info(managed_object, property_path) {
            Ok(info) => {
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
            Err(e) => {
                let error_msg = format!(
                    "Error getting property info for {}.{}:\n\n{}\n\n\
                    Use `list_managed_object_types` to see all supported types.",
                    managed_object,
                    property_path,
                    e
                );
                Ok(CallToolResult::success(vec![Content::text(error_msg)]))
            }
        }
    }

    /// Semantic search using natural language queries (requires embeddings)
    #[cfg(feature = "embeddings")]
    #[tool(description = "Semantic search for vSphere API using natural language queries. Returns Rust methods, types, enums, code examples, and admin guide sections from vim_rs based on meaning, not just keywords. Use filter='guides' to search only documentation guides.")]
    async fn semantic_search(&self, params: Parameters<SemanticSearchInput>) -> Result<CallToolResult, McpError> {
        // Check if embeddings are available
        if self.embedding_model.is_none() || self.embeddings_db.is_none() {
            let message = "Semantic search is not available. Embeddings database not found. Please run build-embeddings first or use text search tools (search_methods, search_types, search_enums, search_guides).".to_string();
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

        // Query the database
        let table = match embeddings_db.open_table("vim_api").execute().await {
            Ok(table) => table,
            Err(e) => {
                return Err(McpError::internal_error(format!("Failed to open embeddings table: {}", e), None));
            }
        };

        let mut query = table
            .vector_search(query_embedding)
            .map_err(|e| McpError::internal_error(format!("Vector search failed: {}", e), None))?
            .limit(params.0.limit);

        // Apply filter if specified
        if params.0.filter != "all" {
            let filter_type = match params.0.filter.as_str() {
                "methods" => "method",
                "types" => "structure",
                "enums" => "enum",
                "examples" => "example",
                "guides" => "guide",
                _ => {
                    return Err(McpError::invalid_params(
                        format!("Invalid filter value: '{}'. Must be 'all', 'methods', 'types', 'enums', 'examples', or 'guides'", params.0.filter),
                        None
                    ));
                }
            };
            query = query.only_if(format!("item_type = '{}'", filter_type));
        }

        let results = match query.execute().await {
            Ok(batches) => batches,
            Err(e) => {
                return Err(McpError::internal_error(format!("Failed to execute search: {}", e), None));
            }
        };

        // Format results
        use arrow_array::cast::AsArray;
        use futures::TryStreamExt;

        let mut formatted_results = Vec::new();

        // Collect batches from stream
        let batches: Vec<_> = results.try_collect().await
            .map_err(|e| McpError::internal_error(format!("Failed to collect results: {}", e), None))?;

        for batch in batches {
            let item_type_array = batch.column_by_name("item_type").unwrap().as_string::<i32>();
            let item_name_array = batch.column_by_name("item_name").unwrap().as_string::<i32>();
            let object_name_array = batch.column_by_name("object_name").unwrap().as_string::<i32>();
            let rust_name_array = batch.column_by_name("rust_name").unwrap().as_string::<i32>();
            let text_array = batch.column_by_name("text").unwrap().as_string::<i32>();

            for i in 0..batch.num_rows() {
                let item_type = item_type_array.value(i);
                let item_name = item_name_array.value(i);
                let _object_name = object_name_array.value(i);
                let _rust_name = rust_name_array.value(i);
                let _text = text_array.value(i);

                // Find full details from api_data
                let details = match item_type {
                    "method" => {
                        // Find the method
                        let mut result = None;
                        for mo in &self.api_data.managed_objects {
                            if let Some(method) = mo.methods.iter().find(|m| m.name == item_name) {
                                let desc = method.description.as_deref().unwrap_or("No description");
                                result = Some(format!(
                                    "## {}.{}\n\n**Rust:** `{}`\n\n**Signature:**\n```rust\n{}\n```\n\n**Description:**\n{}\n\n**Related Types:** {}\n\n---\n",
                                    mo.name,
                                    method.name,
                                    method.rust_name,
                                    method.signature.full,
                                    desc,
                                    method.related_types.join(", ")
                                ));
                                break;
                            }
                        }
                        result
                    }
                    "structure" => {
                        if let Some(structure) = self.api_data.data_structures.iter().find(|s| s.name == item_name) {
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
                        if let Some(enumeration) = self.api_data.enumerations.iter().find(|e| e.name == item_name) {
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
                        if let Some(example) = self.api_data.examples.iter().find(|e| e.name == item_name) {
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
                        if let Some(guide) = self.api_data.guides.iter().find(|g| g.chunk_id == item_name) {
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
                                "## {} > {}{}\n\n**Source:** {}\n\n**Topics:** {}\n\n**Content:**\n{}\n\n**Usage:**\n```\nget_guide(\"{}\")\n```\n\n---\n",
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
                    _ => None,
                };

                if let Some(detail) = details {
                    formatted_results.push(detail);
                }
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
                - Import `vim_rs::types::convert::CastInto` when working with polymorphic types\n\
                - See workflow guide Step 2.5 for complete examples\n\n\
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
        .with_max_level(tracing::Level::TRACE)
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
    use tracing::debug;

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