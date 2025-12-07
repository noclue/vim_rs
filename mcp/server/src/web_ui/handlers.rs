//! HTTP request handlers for the web UI

#[cfg(feature = "web-ui")]
use actix_web::{web, HttpResponse, Result as ActixResult};
#[cfg(feature = "web-ui")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "web-ui")]
use serde_json::Value;
#[cfg(feature = "web-ui")]
use std::collections::HashMap;
#[cfg(feature = "web-ui")]
use std::sync::Arc;
#[cfg(feature = "web-ui")]
use crate::McpServer;
#[cfg(feature = "web-ui")]
use super::{assets, form_generator, markdown, templates};

#[cfg(feature = "web-ui")]
#[derive(Serialize)]
struct ToolInfo {
    name: String,
    description: String,
    input_schema: Value,
}

#[cfg(feature = "web-ui")]
#[derive(Deserialize)]
pub(crate) struct InvokeRequest {
    tool_name: String,
    parameters: HashMap<String, String>,
}

#[cfg(feature = "web-ui")]
#[derive(Serialize)]
struct InvokeResponse {
    success: bool,
    tool_name: String,
    parameters: Value,
    output_html: String,
    error: Option<String>,
}

/// Home page - displays tool selector
#[cfg(feature = "web-ui")]
pub async fn index(server: web::Data<Arc<McpServer>>) -> ActixResult<HttpResponse> {
    // Get list of tools directly from the server
    let tools = server.list_tools();
    
    let tool_infos: Vec<ToolInfo> = tools
        .iter()
        .map(|t| ToolInfo {
            name: t.name.to_string(),
            description: t.description.as_ref().map(|d| d.to_string()).unwrap_or_default(),
            input_schema: Value::Object((*t.input_schema).clone()),
        })
        .collect();
    
    // Serialize tools to JSON string for JavaScript
    let tools_json = serde_json::to_string(&tool_infos)
        .unwrap_or_else(|_| "[]".to_string());
    
    let mut tera = tera::Tera::default();
    tera.add_raw_template("index.html", templates::INDEX_HTML)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Template error: {}", e)))?;
    
    let mut context = tera::Context::new();
    context.insert("tools", &tool_infos);
    context.insert("tools_json", &tools_json);
    context.insert("has_tools", &true);
    
    let html = tera
        .render("index.html", &context)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Template error: {}", e)))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

/// Get tool details (for dynamic form generation)
#[cfg(feature = "web-ui")]
pub async fn get_tool(
    server: web::Data<Arc<McpServer>>,
    tool_name: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let tools = server.list_tools();
    
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
        
        let schema_value = Value::Object((*tool.input_schema).clone());
        let form_html = form_generator::generate_form_html(&schema_value, &required);
        
        let mut tera = tera::Tera::default();
        tera.add_raw_template("tool_form.html", templates::TOOL_FORM_HTML)
            .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Template error: {}", e)))?;
        
        let mut context = tera::Context::new();
        context.insert("tool_name", &tool.name);
        context.insert("tool_description", &tool.description.clone().unwrap_or_default());
        context.insert("form_html", &form_html);
        
        let html = tera
            .render("tool_form.html", &context)
            .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Template error: {}", e)))?;
        
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    } else {
        Ok(HttpResponse::NotFound().body("Tool not found"))
    }
}

/// Invoke a tool
#[cfg(feature = "web-ui")]
pub async fn invoke_tool(
    server: web::Data<Arc<McpServer>>,
    req: web::Json<InvokeRequest>,
) -> ActixResult<HttpResponse> {
    let tools = server.list_tools();
    let tool = tools.iter().find(|t| t.name == req.tool_name);
    
    // Convert parameters to JSON
    let mut arguments = serde_json::Map::new();
    
    if let Some(tool) = tool {
        let properties = tool
            .input_schema
            .get("properties")
            .and_then(|v| v.as_object());
        
        if let Some(props) = properties {
            for (key, value) in &req.parameters {
                if value.is_empty() {
                    // Skip empty values
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
    
    // Call the tool directly (in-process)
    match server.call_tool_direct(&req.tool_name, arguments_value.clone()).await {
        Ok(result) => {
            // Extract text content and convert markdown to HTML
            let mut output_text = String::new();
            
            for content in &result.content {
                // Content is Annotated<RawContent>, extract text if it's a text content
                if let Some(text_content) = content.as_text() {
                    output_text.push_str(&text_content.text);
                    output_text.push('\n');
                }
            }
            
            let output_html = markdown::markdown_to_html(&output_text);
            
            let is_error = result.is_error.unwrap_or(false);
            
            Ok(HttpResponse::Ok().json(InvokeResponse {
                success: !is_error,
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

/// Serve the embedded CSS file
#[cfg(feature = "web-ui")]
pub async fn serve_css() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/css")
        .body(assets::STYLE_CSS))
}

#[cfg(all(test, feature = "web-ui"))]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_serve_css() {
        let resp = serve_css().await.unwrap();
        assert_eq!(resp.status(), 200);
        assert!(resp.headers().get("content-type").unwrap().to_str().unwrap().contains("text/css"));
    }
}

