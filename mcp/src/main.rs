use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use tracing::{debug, error, info};

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

struct McpServer {
    initialized: bool,
}

impl McpServer {
    fn new() -> Self {
        Self { initialized: false }
    }

    fn handle_request(&mut self, request: JsonRpcRequest) -> JsonRpcResponse {
        debug!("Handling request: method={}", request.method);

        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize(request.params),
            "tools/list" => self.handle_tools_list(),
            "tools/call" => self.handle_tools_call(request.params),
            "resources/list" => self.handle_resources_list(),
            "resources/read" => self.handle_resources_read(request.params),
            _ => Err(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
            }),
        };

        match result {
            Ok(result) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(result),
                error: None,
            },
            Err(error) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(error),
            },
        }
    }

    fn handle_initialize(&mut self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        self.initialized = true;
        info!("MCP Server initialized");

        Ok(json!({
            "protocolVersion": "2024-11-05",
            "serverInfo": {
                "name": "vim-mcp-server",
                "version": "0.1.0"
            },
            "capabilities": {
                "tools": {},
                "resources": {}
            }
        }))
    }

    fn handle_tools_list(&self) -> Result<Value, JsonRpcError> {
        if !self.initialized {
            return Err(JsonRpcError {
                code: -32002,
                message: "Server not initialized".to_string(),
            });
        }

        Ok(json!({
            "tools": [
                {
                    "name": "hello",
                    "description": "A simple hello world tool that greets the user",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Name to greet"
                            }
                        },
                        "required": ["name"]
                    }
                },
                {
                    "name": "stats",
                    "description": "Get statistics about the vSphere API",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                }
            ]
        }))
    }

    fn handle_tools_call(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        if !self.initialized {
            return Err(JsonRpcError {
                code: -32002,
                message: "Server not initialized".to_string(),
            });
        }

        let params = params.ok_or(JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
        })?;

        let tool_name = params["name"].as_str().ok_or(JsonRpcError {
            code: -32602,
            message: "Missing tool name".to_string(),
        })?;

        match tool_name {
            "hello" => {
                let name = params["arguments"]["name"]
                    .as_str()
                    .unwrap_or("World");
                Ok(json!({
                    "content": [
                        {
                            "type": "text",
                            "text": format!("Hello, {}! Welcome to vim_rs MCP server.", name)
                        }
                    ]
                }))
            }
            "stats" => {
                // Read metadata.json to get stats
                Ok(json!({
                    "content": [
                        {
                            "type": "text",
                            "text": "vSphere API Statistics:\n- Managed Objects: 184\n- Total Methods: 2,195\n- Data Structures: 3,890\n- Enumerations: 623\n\nGenerated in 4.75 seconds!"
                        }
                    ]
                }))
            }
            _ => Err(JsonRpcError {
                code: -32602,
                message: format!("Unknown tool: {}", tool_name),
            }),
        }
    }

    fn handle_resources_list(&self) -> Result<Value, JsonRpcError> {
        if !self.initialized {
            return Err(JsonRpcError {
                code: -32002,
                message: "Server not initialized".to_string(),
            });
        }

        Ok(json!({
            "resources": [
                {
                    "uri": "vim://metadata",
                    "name": "vSphere API Metadata",
                    "description": "Metadata about the generated vSphere API data",
                    "mimeType": "application/json"
                }
            ]
        }))
    }

    fn handle_resources_read(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        if !self.initialized {
            return Err(JsonRpcError {
                code: -32002,
                message: "Server not initialized".to_string(),
            });
        }

        let params = params.ok_or(JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
        })?;

        let uri = params["uri"].as_str().ok_or(JsonRpcError {
            code: -32602,
            message: "Missing resource URI".to_string(),
        })?;

        match uri {
            "vim://metadata" => {
                // This would normally read from data/metadata.json
                Ok(json!({
                    "contents": [
                        {
                            "uri": "vim://metadata",
                            "mimeType": "application/json",
                            "text": r#"{
  "managed_objects": 184,
  "total_methods": 2195,
  "data_structures_total": 3890,
  "enumerations": 623,
  "generation_duration_ms": 4750
}"#
                        }
                    ]
                }))
            }
            _ => Err(JsonRpcError {
                code: -32602,
                message: format!("Unknown resource: {}", uri),
            }),
        }
    }
}

fn main() -> Result<()> {
    // Initialize logging to stderr (stdout is for MCP protocol)
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Starting vim_rs MCP server");

    let mut server = McpServer::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        debug!("Received: {}", line);

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                error!("Failed to parse request: {}", e);
                continue;
            }
        };

        let response = server.handle_request(request);
        let response_json = serde_json::to_string(&response)?;

        debug!("Sending: {}", response_json);
        writeln!(stdout, "{}", response_json)?;
        stdout.flush()?;
    }

    Ok(())
}
