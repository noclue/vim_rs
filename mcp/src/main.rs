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
use tokio::io::{stdin, stdout};
use tracing::{error, info};

/// McpServer - A Model Context Protocol server
#[derive(Clone, Debug)]
pub struct McpServer {
    tool_router: ToolRouter<Self>,
}

/// Input parameters for the hello tool
#[derive(Serialize, Deserialize, JsonSchema)]
struct HelloInput {
    /// Name to greet
    #[schemars(description = "The name of a person to greet")]
    name: String,
}

#[tool_router]
impl McpServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// A simple hello world tool that greets the user
    #[tool(description = "A simple hello world tool that greets the user")]
    async fn hello(&self, params: Parameters<HelloInput>) -> Result<CallToolResult, McpError> {
        let greeting = format!("Hello, {}! Welcome to vim_rs MCP server.", params.0.name);
        Ok(CallToolResult::success(vec![Content::text(greeting)]))
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
                title: Some("Sample MCP server".into()),
                version: env!("CARGO_PKG_VERSION").to_owned(),
                icons: None,
                website_url: None,
            },
            instructions: Some("A simple hello world MCP server that greets the user.".to_string()),
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
    let server = McpServer::new();

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

        let mcp_server = McpServer::new();
        let tool_router = &mcp_server.tool_router;
        assert!(tool_router.has_route("hello"));

        Ok(())
    }
}