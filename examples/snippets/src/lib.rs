//! Shared helpers for the `snippets` example binaries (connection, env loading).

use anyhow::{Context, Result};
use log::debug;
use std::env;
use std::sync::Arc;
use vim_rs::core::client::{Client, ClientBuilder, TransportMode};

/// Connect to vSphere using `VIM_SERVER`, `VIM_USERNAME`, and `VIM_PASSWORD` from the environment.
///
/// Loads a `.env` file from the current working directory when present; otherwise uses process
/// environment variables only.
pub async fn connect(app_name: &str, app_version: &str) -> Result<Arc<Client>> {
    if let Err(e) = dotenvy::dotenv() {
        debug!(
            "Could not load .env file: {}. Using system environment variables.",
            e
        );
    } else {
        debug!("Loaded environment variables from .env file");
    }

    let vc_server = env::var("VIM_SERVER").with_context(|| "VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").with_context(|| "VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").with_context(|| "VIM_PASSWORD env var not set")?;
    let protocol = env::var("VIM_PROTOCOL").unwrap_or("auto".to_string());
    let transport = match protocol.as_str() {
        "auto" => TransportMode::Auto,
        "json" => TransportMode::Json,
        "soap" => TransportMode::Soap,
        _ => return Err(anyhow::anyhow!("Invalid protocol: {}. Valid values are 'auto', 'json', or 'soap'. Default is 'auto'.", protocol)),
    };

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(app_name, app_version)
        .transport(transport)
        .build()
        .await?;
    debug!("Connected to {}", client.service_content().about.full_name);
    Ok(client)
}
