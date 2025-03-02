use std::env;
use vim::mo::PerformanceManager;

use vim::core::client::ClientBuilder;
use tokio;
use log::{debug, info};
use env_logger;
use utils::{Result, Error};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting up!");

    let vc_server = env::var("VC_SERVER").map_err(|_| Error::Error(String::from("VC_SERVER env var not set")))?;
    let username = env::var("VC_USERNAME").map_err(|_| Error::Error(String::from("VC_USERNAME env var not set")))?;
    let pwd = env::var("VC_PASSWORD").map_err(|_| Error::Error(String::from("VC_PASSWORD env var not set")))?;

    let vim_client = ClientBuilder::new(&vc_server)
        .insecure(true)
        .basic_authn(&username, &pwd)
        .build().await?;


    let Some(perf_manager_moref) = vim_client.service_content().perf_manager.clone() else {
        return Err(Error::Error(String::from("No performance manager found")));
    };

    let perf_manager = PerformanceManager::new(vim_client.clone(), &perf_manager_moref.value.clone());

    perf_manager.perf_counter().await?.iter().for_each(|perf_counter| {
        debug!("perf_counter: {:?}", perf_counter);
    });


    Ok(())
    
}
