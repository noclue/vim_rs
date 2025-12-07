//! Web UI module for testing MCP tools via browser
//!
//! This module provides a web interface for interacting with the MCP server.
//! When enabled with the `--web` flag, it starts an HTTP server that serves
//! a web UI for testing and exploring MCP tools.

#[cfg(feature = "web-ui")]
pub mod handlers;
#[cfg(feature = "web-ui")]
pub mod form_generator;
#[cfg(feature = "web-ui")]
pub mod markdown;
#[cfg(feature = "web-ui")]
pub mod templates;
#[cfg(feature = "web-ui")]
pub mod assets;

#[cfg(all(test, feature = "web-ui"))]
mod tests;

#[cfg(feature = "web-ui")]
use actix_web::{web, App, HttpServer};
#[cfg(feature = "web-ui")]
use anyhow::Result;
#[cfg(feature = "web-ui")]
use std::sync::Arc;
#[cfg(feature = "web-ui")]
use crate::McpServer;

/// Start the web server for the MCP test client UI
#[cfg(feature = "web-ui")]
pub async fn start_server(
    server: McpServer,
    bind_addr: &str,
    port: u16,
) -> Result<()> {
    let server = Arc::new(server);
    let bind = format!("{}:{}", bind_addr, port);
    
    tracing::info!("Starting web UI at http://{}", bind);
    tracing::info!("Open your browser and navigate to http://{}", bind);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/", web::get().to(handlers::index))
            .route("/static/style.css", web::get().to(handlers::serve_css))
            .route("/api/tool/{tool_name}", web::get().to(handlers::get_tool))
            .route("/api/invoke", web::post().to(handlers::invoke_tool))
    })
    .bind(&bind)?
    .run()
    .await?;
    
    Ok(())
}

