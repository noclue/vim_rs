use anyhow::{Context, Result};
use log::debug;
use std::env;
use std::sync::Arc;
use vim_rs::core::client::{Client, ClientBuilder};

/// Connect to the vSphere server using the credentials provide via environment variables.
///
/// This function will attempt to load environment variables from a `.env` file if present.
/// The `.env` file should be placed in the current working directory where you run the binary.
///
/// If the `.env` file is not found, it will fall back to using system environment variables.
///
/// Required environment variables:
/// - `VIM_SERVER`: vCenter/ESXi server URL
/// - `VIM_USERNAME`: Username for authentication
/// - `VIM_PASSWORD`: Password for authentication
pub async fn connect(app_name: &str, app_version: &str) -> Result<Arc<Client>> {
    // Try to load .env file from current directory, but don't fail if it doesn't exist
    // This allows falling back to system environment variables
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

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(app_name, app_version)
        .build()
        .await?;
    debug!("Connected to {}", client.service_content().about.full_name);
    Ok(client)
}
