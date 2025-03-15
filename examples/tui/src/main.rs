use std::{
    env,
    sync::Arc
    ,
};
use anyhow::{Context, Result};
use vim_rs::core::client::{Client, ClientBuilder};
use app::App;
use vm::VirtualMachine;
use crate::event::EventHandler;
use crate::monitor::Monitor;

mod vm;
mod event;
mod monitor;
mod vm_list;
mod app;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    let client = init_vim_client().await?;
    let monitor = Monitor::new(
        client.clone(),
        client.service_content().root_folder.clone(),
        VirtualMachine::prop_spec())
        .await?;
    let event_handler = EventHandler::new(monitor);
    let terminal = ratatui::init();
    let app_result = App::new(event_handler).run(terminal).await;
    ratatui::restore();
    app_result
}

async fn init_vim_client() -> Result<Arc<Client>> {
    let vc_server = env::var("VIM_SERVER").with_context(||"VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").with_context(||"VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").with_context(||"VIM_PASSWORD env var not set")?;

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(APP_NAME, APP_VERSION)
        .build().await?;
    Ok(client)
}



