mod config;
mod form_generator;
mod markdown;
mod mcp_client;

use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Result as ActixResult};
use config::McpConfig;
use mcp_client::{McpClient, SharedMcpClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tera::Tera;
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    mcp_client: SharedMcpClient,
    tera: Arc<Tera>,
}

#[derive(Serialize)]
struct ToolInfo {
    name: String,
    description: String,
    input_schema: Value,
}

#[derive(Deserialize)]
struct InvokeRequest {
    tool_name: String,
    parameters: HashMap<String, String>,
}

#[derive(Serialize)]
struct InvokeResponse {
    success: bool,
    tool_name: String,
    parameters: Value,
    output_html: String,
    error: Option<String>,
}

/// Home page
async fn index(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let mut context = tera::Context::new();
    
    // Get list of tools
    let mut client = state.mcp_client.lock().await;
    match client.list_tools() {
        Ok(tools) => {
            let tool_infos: Vec<ToolInfo> = tools
                .iter()
                .map(|t| ToolInfo {
                    name: t.name.clone(),
                    description: t.description.clone().unwrap_or_default(),
                    input_schema: t.input_schema.clone(),
                })
                .collect();
            
            // Serialize tools to JSON string for JavaScript
            let tools_json = serde_json::to_string(&tool_infos)
                .unwrap_or_else(|_| "[]".to_string());
            
            context.insert("tools", &tool_infos);
            context.insert("tools_json", &tools_json);
            context.insert("has_tools", &true);
        }
        Err(e) => {
            context.insert("error", &format!("Failed to list tools: {}", e));
            context.insert("has_tools", &false);
            context.insert("tools_json", "[]");
        }
    }
    
    let html = state
        .tera
        .render("index.html", &context)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Template error: {}", e)))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

/// Get tool details (for dynamic form generation)
async fn get_tool(
    state: web::Data<AppState>,
    tool_name: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let mut client = state.mcp_client.lock().await;
    
    match client.list_tools() {
        Ok(tools) => {
            if let Some(tool) = tools.iter().find(|t| t.name == *tool_name) {
                let required = tool
                    .input_schema
                    .get("required")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                
                let form_html =
                    form_generator::generate_form_html(&tool.input_schema, &required);
                
                let mut context = tera::Context::new();
                context.insert("tool_name", &tool.name);
                context.insert("tool_description", &tool.description.clone().unwrap_or_default());
                context.insert("form_html", &form_html);
                
                let html = state
                    .tera
                    .render("tool_form.html", &context)
                    .map_err(|e| {
                        actix_web::error::ErrorInternalServerError(format!("Template error: {}", e))
                    })?;
                
                Ok(HttpResponse::Ok().content_type("text/html").body(html))
            } else {
                Ok(HttpResponse::NotFound().body("Tool not found"))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError()
            .body(format!("Failed to list tools: {}", e))),
    }
}

/// Invoke a tool
async fn invoke_tool(
    state: web::Data<AppState>,
    req: web::Json<InvokeRequest>,
) -> ActixResult<HttpResponse> {
    let mut client = state.mcp_client.lock().await;
    
    // Convert parameters to JSON
    let mut arguments = serde_json::Map::new();
    
    // Get the tool's schema to understand parameter types
    let tools = match client.list_tools() {
        Ok(tools) => tools,
        Err(e) => {
            return Ok(HttpResponse::Ok().json(InvokeResponse {
                success: false,
                tool_name: req.tool_name.clone(),
                parameters: Value::Null,
                output_html: String::new(),
                error: Some(format!("Failed to list tools: {}", e)),
            }));
        }
    };
    
    let tool = tools.iter().find(|t| t.name == req.tool_name);
    
    if let Some(tool) = tool {
        let properties = tool
            .input_schema
            .get("properties")
            .and_then(|v| v.as_object());
        
        if let Some(props) = properties {
            for (key, value) in &req.parameters {
                if value.is_empty() {
                    // Skip empty values unless required
                    continue;
                }
                
                // Get the type from schema
                let param_type = props
                    .get(key)
                    .and_then(|v| v.get("type"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("string");
                
                // Convert based on type
                let json_value = match param_type {
                    "number" | "integer" => {
                        if let Ok(num) = value.parse::<i64>() {
                            Value::Number(num.into())
                        } else if let Ok(num) = value.parse::<f64>() {
                            Value::Number(
                                serde_json::Number::from_f64(num).unwrap_or_else(|| 0.into()),
                            )
                        } else {
                            Value::String(value.clone())
                        }
                    }
                    "boolean" => Value::Bool(value == "true" || value == "on"),
                    "array" | "object" => {
                        // Try to parse as JSON
                        serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.clone()))
                    }
                    _ => Value::String(value.clone()),
                };
                
                arguments.insert(key.clone(), json_value);
            }
        }
    }
    
    let arguments_value = Value::Object(arguments);
    
    // Call the tool
    match client.call_tool(&req.tool_name, arguments_value.clone()) {
        Ok(result) => {
            // Extract text content and convert markdown to HTML
            let mut output_text = String::new();
            
            for content in &result.content {
                match content {
                    mcp_client::Content::Text { text } => {
                        output_text.push_str(text);
                        output_text.push('\n');
                    }
                    mcp_client::Content::Image { .. } => {
                        output_text.push_str("[Image content not displayed]\n");
                    }
                    mcp_client::Content::Resource { .. } => {
                        output_text.push_str("[Resource content not displayed]\n");
                    }
                }
            }
            
            let output_html = markdown::markdown_to_html(&output_text);
            
            Ok(HttpResponse::Ok().json(InvokeResponse {
                success: !result.is_error.unwrap_or(false),
                tool_name: req.tool_name.clone(),
                parameters: arguments_value,
                output_html,
                error: None,
            }))
        }
        Err(e) => Ok(HttpResponse::Ok().json(InvokeResponse {
            success: false,
            tool_name: req.tool_name.clone(),
            parameters: arguments_value,
            output_html: String::new(),
            error: Some(format!("Tool invocation failed: {}", e)),
        })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting MCP Test Client...");
    
    // Load configuration
    let config = McpConfig::load_from_file("mcp.json").expect("Failed to load mcp.json");
    
    // Get the first server from config
    let (server_name, server_config) = config
        .first_server()
        .expect("No MCP servers configured in mcp.json");
    
    println!("Using MCP server: {}", server_name);
    println!("Command: {}", server_config.command);
    
    // Create MCP client (synchronous, not async)
    let mcp_client = McpClient::new(&server_config.command, &server_config.args)
        .expect("Failed to create MCP client");
    
    let mcp_client = Arc::new(Mutex::new(mcp_client));
    
    println!("MCP client connected");
    
    // Initialize Tera templates
    let tera = Tera::new("templates/**/*.html").expect("Failed to initialize Tera templates");
    
    let app_state = AppState {
        mcp_client,
        tera: Arc::new(tera),
    };
    
    let bind_addr = "127.0.0.1:8080";
    println!("Starting web server at http://{}", bind_addr);
    println!("Open your browser and navigate to http://{}", bind_addr);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(fs::Files::new("/static", "static"))
            .route("/", web::get().to(index))
            .route("/api/tool/{tool_name}", web::get().to(get_tool))
            .route("/api/invoke", web::post().to(invoke_tool))
    })
    .bind(bind_addr)?
    .run()
    .await
}

