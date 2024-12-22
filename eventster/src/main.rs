use std::env;
use vim::service_instance::ServiceInstance;
use tokio;
use vim::session_manager::SessionManager;
use log::{debug, info};
use env_logger;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("MethodFault: {0:?}")]
    MethodFault(Box<dyn vim::types::MethodFaultTrait>),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("VimClient error: {0}")]
    VimClientError(#[from] vim::vim_client::Error),
    #[error("Error: {0}")]
    Error(String),
}

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Starting up!");
    let http_client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .user_agent(APP_USER_AGENT)
        .build()?;

    let vc_server = env::var("VC_SERVER")?;
    let username = env::var("VC_USERNAME")?;
    let pwd = env::var("VC_PASSWORD")?;
    {
        let vim_client = vim::vim_client::VimClient::new(http_client, &vc_server, None);

        let service_instance = ServiceInstance::new(vim_client.clone(), "ServiceInstance");

        let content = service_instance.content().await?;
        debug!("ServiceInstance::content obtained from vCenter {}",
                content.about.full_name);

        let session_mgr_moref = content.session_manager.ok_or(Error::Error("No session manager found".to_string()))?;

        let sm = SessionManager::new(vim_client.clone(), &session_mgr_moref.value);
        let us = sm.login(&username, &pwd, Some("en")).await?;

        info!("Session created for: {:?}", us.user_name);
    }
    info!("Client Disposed!");

    Ok(())
}
