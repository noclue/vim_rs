use std::env;
use std::sync::Arc;
use vim_macros::vim_retrievable;
use anyhow::{Context, Result};
use vim_rs::core::client::{Client, ClientBuilder};
use log::debug;
use vim_rs::core::pc_helpers::ObjectRetriever;


vim_retrievable!(
    struct Host: HostSystem {
        name = "name",
        power_state = "runtime.power_state",
        connected = "runtime.connection_state",
        cpu_usage = "summary.quick_stats.overall_cpu_usage",
        memory_usage = "summary.quick_stats.overall_memory_usage",
        uptime = "summary.quick_stats.uptime",
    }
);

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let client = connect().await?;
    let retriever = ObjectRetriever::new(client.clone())?;
    let hosts: Vec<Host> = retriever.retrieve_objects_from_container(&client.service_content().root_folder).await?;

    for host in hosts {
        println!("Host: {:?}", host)
    }

    Ok(())
}

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

async fn connect() -> Result<Arc<Client>> {
    let vc_server = env::var("VIM_SERVER").with_context(|| "VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").with_context(|| "VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").with_context(|| "VIM_PASSWORD env var not set")?;

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(APP_NAME, APP_VERSION)
        .build().await?;
    debug!("Connected to {}", client.service_content().about.full_name);
    Ok(client)
}




