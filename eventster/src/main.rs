use std::sync::Arc;
use std::env;
use vim::service_instance::ServiceInstance;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()?;

    let vc_url = env::var("VC_URL")?;
    let vim_client = Arc::new(vim_client::VimClient::new(http_client, vc_url)?);

    let service_instance = ServiceInstance::new(vim_client.clone(), "ServiceInstance");

    let content = service_instance.content().await?;

    let session_mgr_moref = content.session_manager.ok_or(error::Error::SessionManagerNotFound)?;
    Ok(())
}
