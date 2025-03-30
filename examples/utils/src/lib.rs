use anyhow::{Context, Result};
use log::debug;
use std::env;
use std::sync::Arc;
use vim_rs::core::client::{Client, ClientBuilder};

/// Connect to the vSphere server using the credentials provide via environment variables.
pub async fn connect(app_name: &str, app_version: &str) -> Result<Arc<Client>> {
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
