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
use tracing::{error, info};

// Import data model from the library
use vim_mcp_server::model::ApiData;

// ============================================================================
// MCP Server
// ============================================================================

/// McpServer - A Model Context Protocol server
#[derive(Clone, Debug)]
pub struct McpServer {
    tool_router: ToolRouter<Self>,
    api_data: Arc<ApiData>,
}

/// Input parameters for the hello tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct HelloInput {
    /// Name to greet
    #[schemars(description = "The name of a person to greet")]
    name: String,
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

fn default_limit() -> usize {
    10
}

#[tool_router]
impl McpServer {
    fn new() -> Result<Self> {
        // Try to load API data from the data directory
        let data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
        let api_data = ApiData::load_from_dir(&data_dir)?;

        Ok(Self {
            tool_router: Self::tool_router(),
            api_data: Arc::new(api_data),
        })
    }

    /// A simple hello world tool that greets the user
    #[tool(description = "A simple hello world tool that greets the user")]
    async fn hello(&self, params: Parameters<HelloInput>) -> Result<CallToolResult, McpError> {
        let greeting = format!("Hello, {}! Welcome to vim_rs MCP server.", params.0.name);
        Ok(CallToolResult::success(vec![Content::text(greeting)]))
    }

    /// Search for vSphere API methods by name or description
    #[tool(description = "Search for vSphere API methods by name or description")]
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

    /// Search for vSphere API data structures by name or description
    #[tool(description = "Search for vSphere API data structures by name or description")]
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

    /// Search for vSphere API enumerations by name or description
    #[tool(description = "Search for vSphere API enumerations by name or description")]
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
                title: Some("vSphere API MCP Server".into()),
                version: env!("CARGO_PKG_VERSION").to_owned(),
                icons: None,
                website_url: None,
            },
            instructions: Some("Search and explore the vSphere API using search_methods, search_types, and search_enums tools.".to_string()),
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
    let server = McpServer::new()?;

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
        debug!("Router: {:?}", router);
        assert!(router.has_route("hello"));
        assert!(router.has_route("search_methods"));
        assert!(router.has_route("search_types"));
        assert!(router.has_route("search_enums"));

        // Note: Can't test McpServer::new() here without data files
        // let mcp_server = McpServer::new()?;
        // let tool_router = &mcp_server.tool_router;
        // assert!(tool_router.has_route("hello"));

        Ok(())
    }
}