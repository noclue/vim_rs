use anyhow::Result;
use rmcp::{
    server::Server,
    types::{ServerInfo, Tool, ToolInputSchema, Resource},
};
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to stderr
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting vim_rs MCP server");

    // Create server with metadata
    let server = Server::new(ServerInfo {
        name: "vim-mcp-server".to_string(),
        version: "0.1.0".to_string(),
    });

    // Register tools
    server.add_tool(Tool {
        name: "hello".to_string(),
        description: Some("A simple hello world tool that greets the user".to_string()),
        input_schema: ToolInputSchema {
            schema_type: "object".to_string(),
            properties: Some({
                let mut props = HashMap::new();
                props.insert(
                    "name".to_string(),
                    json!({
                        "type": "string",
                        "description": "Name to greet"
                    }),
                );
                props
            }),
            required: Some(vec!["name".to_string()]),
        },
    });

    server.add_tool(Tool {
        name: "stats".to_string(),
        description: Some("Get statistics about the vSphere API".to_string()),
        input_schema: ToolInputSchema {
            schema_type: "object".to_string(),
            properties: None,
            required: None,
        },
    });

    // Register tool handler
    server.set_tool_handler(|tool_name: String, arguments: Value| async move {
        match tool_name.as_str() {
            "hello" => {
                let name = arguments["name"].as_str().unwrap_or("World");
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": format!("Hello, {}! Welcome to vim_rs MCP server.", name)
                    }]
                }))
            }
            "stats" => Ok(json!({
                "content": [{
                    "type": "text",
                    "text": "vSphere API Statistics:\n- Managed Objects: 184\n- Total Methods: 2,195\n- Data Structures: 3,890\n- Enumerations: 623\n\nGenerated in 4.75 seconds!"
                }]
            })),
            _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_name)),
        }
    });

    // Register resources
    server.add_resource(Resource {
        uri: "vim://metadata".to_string(),
        name: "vSphere API Metadata".to_string(),
        description: Some("Metadata about the generated vSphere API data".to_string()),
        mime_type: Some("application/json".to_string()),
    });

    // Register resource handler
    server.set_resource_handler(|uri: String| async move {
        match uri.as_str() {
            "vim://metadata" => Ok(json!({
                "contents": [{
                    "uri": "vim://metadata",
                    "mimeType": "application/json",
                    "text": r#"{
  "managed_objects": 184,
  "total_methods": 2195,
  "data_structures_total": 3890,
  "enumerations": 623,
  "generation_duration_ms": 4750
}"#
                }]
            })),
            _ => Err(anyhow::anyhow!("Unknown resource: {}", uri)),
        }
    });

    // Run the server (handles stdio communication)
    info!("MCP server ready");
    server.run().await?;

    Ok(())
}
