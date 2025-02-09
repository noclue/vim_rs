use std::{env, sync::Arc};
use vim::mo::{PerformanceManager, PropertyCollector, PropertyFilter, ServiceInstance, SessionManager, View, ViewManager};
use vim::types::structs::{CastInto, ServiceContent, ValueElements, VimAny, VirtualEthernetCardTrait}; 
use vim::types::enums::{self, MoTypesEnum};
use vim::types::structs;

use vim::core::client::Client;
use tokio;
use log::{debug, error, info, trace};
use env_logger;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("MethodFault: {0:?}")]
    MethodFault(Box<dyn vim::types::structs::MethodFaultTrait>),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("VimClient error: {0}")]
    VimClientError(#[from] vim::core::client::Error),
    #[error("Error: {0}")]
    Error(String),
}

type Result<T> = std::result::Result<T, Error>;

async fn create_client(vc_server: String, username: String, pwd: String) -> Result<(Arc<Client>, ServiceContent)> {
    let http_client = reqwest::ClientBuilder::new()
    .danger_accept_invalid_certs(true)
    .danger_accept_invalid_hostnames(true)
    .build()?;

    let vim_client = Client::new(http_client, &vc_server, None);

    let service_instance = ServiceInstance::new(vim_client.clone(), "ServiceInstance");

    let content = service_instance.content().await?;
    debug!("ServiceInstance::content obtained from vCenter {}",
            content.about.full_name);

    let Some(ref session_mgr_moref) = content.session_manager else {
        return Err(Error::Error("No session manager found".to_string()));
    };

    let sm = SessionManager::new(vim_client.clone(), &session_mgr_moref.value.clone());
    let us = sm.login(&username, &pwd, Some("en")).await?;

    info!("Session created for: {:?}", us.user_name);
    Ok((vim_client, content))
}



#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting up!");

    let vc_server = env::var("VC_SERVER").map_err(|_| Error::Error(String::from("VC_SERVER env var not set")))?;
    let username = env::var("VC_USERNAME").map_err(|_| Error::Error(String::from("VC_USERNAME env var not set")))?;
    let pwd = env::var("VC_PASSWORD").map_err(|_| Error::Error(String::from("VC_PASSWORD env var not set")))?;

    let (vim_client, content) = create_client(vc_server, username, pwd).await?;

    let Some(perf_manager_moref) = content.perf_manager else {
        return Err(Error::Error(String::from("No performance manager found")));
    };

    // let perf_manager = PerformanceManager::new(vim_client.clone(), &perf_manager_moref.value.clone());

    // perf_manager.


    Ok(())
    
}
