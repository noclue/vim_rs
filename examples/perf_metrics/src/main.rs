use std::env;
use vim::mo::PerformanceManager;

use vim::core::client::ClientBuilder;
use log::debug;
use anyhow::{Result, Error, Context};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let vc_server = env::var("VC_SERVER").with_context(||"VC_SERVER env var not set")?;
    let username = env::var("VC_USERNAME").with_context(||"VC_USERNAME env var not set")?;
    let pwd = env::var("VC_PASSWORD").with_context(||"VC_PASSWORD env var not set")?;

    let vim_client = ClientBuilder::new(&vc_server)
        .insecure(true)
        .basic_authn(&username, &pwd)
        .build().await?;
    let Some(perf_manager_moref) = vim_client.service_content().perf_manager.clone() else {
        return Err(Error::msg("Performance manager not found in service content."));
    };
    let perf_manager = PerformanceManager::new(vim_client.clone(), &perf_manager_moref.value.clone());
    perf_manager.perf_counter().await?.iter().for_each(|perf_counter| {
        debug!("perf_counter: {:?}", perf_counter);
    });
    Ok(())
}
