use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub input_schema: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolResult {
    pub content: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Content {
    Text { text: String },
    Image { data: String, mime_type: String },
    Resource { resource: Value },
}

#[derive(Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: i64,
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Value>,
}

#[derive(Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

pub struct McpClient {
    process: Option<Child>,
    stdin: Option<ChildStdin>,
    stdout: Option<BufReader<ChildStdout>>,
    request_id: i64,
    tools_cache: Option<Vec<Tool>>,
}

impl McpClient {
    /// Create a new MCP client by spawning the server process
    pub fn new(command_path: &str, args: &[String]) -> Result<Self> {
        // Spawn the MCP server process
        let mut process = Command::new(command_path)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit()) // Keep stderr for debugging
            .spawn()
            .with_context(|| format!("Failed to spawn MCP server: {}", command_path))?;

        // Get stdin/stdout handles
        let stdin = process.stdin.take().context("Failed to get stdin handle")?;
        let stdout = process
            .stdout
            .take()
            .context("Failed to get stdout handle")?;
        let stdout = BufReader::new(stdout);

        let mut client = Self {
            process: Some(process),
            stdin: Some(stdin),
            stdout: Some(stdout),
            request_id: 0,
            tools_cache: None,
        };

        // Initialize the MCP session
        client.initialize()?;

        Ok(client)
    }

    fn initialize(&mut self) -> Result<()> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: self.next_id(),
            method: "initialize".to_string(),
            params: Some(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "mcp_test_client",
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
        };

        self.send_request(&request)?;
        let _response = self.read_response()?;
        
        // Send initialized notification
        let initialized = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: self.next_id(),
            method: "notifications/initialized".to_string(),
            params: None,
        };
        self.send_request(&initialized)?;

        Ok(())
    }

    fn next_id(&mut self) -> i64 {
        self.request_id += 1;
        self.request_id
    }

    fn send_request(&mut self, request: &JsonRpcRequest) -> Result<()> {
        let stdin = self.stdin.as_mut().context("stdin not available")?;
        let json = serde_json::to_string(request)?;
        writeln!(stdin, "{}", json)?;
        stdin.flush()?;
        Ok(())
    }

    fn read_response(&mut self) -> Result<JsonRpcResponse> {
        let stdout = self.stdout.as_mut().context("stdout not available")?;
        let mut line = String::new();
        stdout.read_line(&mut line)?;
        
        let response: JsonRpcResponse = serde_json::from_str(&line)
            .with_context(|| format!("Failed to parse JSON-RPC response: {}", line))?;
        
        if let Some(error) = response.error {
            anyhow::bail!("MCP error: {}", error);
        }
        
        Ok(response)
    }

    /// List all available tools from the server
    pub fn list_tools(&mut self) -> Result<Vec<Tool>> {
        if let Some(ref cached) = self.tools_cache {
            return Ok(cached.clone());
        }

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: self.next_id(),
            method: "tools/list".to_string(),
            params: None,
        };

        self.send_request(&request)?;
        let response = self.read_response()?;

        let result = response.result.context("No result in response")?;
        let tools_obj = result.as_object().context("Result is not an object")?;
        let tools_array = tools_obj
            .get("tools")
            .context("No 'tools' field")?
            .as_array()
            .context("'tools' is not an array")?;

        let tools: Vec<Tool> = serde_json::from_value(Value::Array(tools_array.clone()))?;
        
        self.tools_cache = Some(tools.clone());
        Ok(tools)
    }

    /// Invoke a tool with the given parameters
    pub fn call_tool(&mut self, name: &str, arguments: Value) -> Result<CallToolResult> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: self.next_id(),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": name,
                "arguments": arguments
            })),
        };

        self.send_request(&request)?;
        let response = self.read_response()?;

        let result = response
            .result
            .context("No result in tool call response")?;
        let tool_result: CallToolResult = serde_json::from_value(result)?;

        Ok(tool_result)
    }
}

impl Drop for McpClient {
    fn drop(&mut self) {
        // Kill the subprocess when the client is dropped
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
            let _ = process.wait();
        }
    }
}

/// Thread-safe wrapper for McpClient
pub type SharedMcpClient = std::sync::Arc<Mutex<McpClient>>;
