use std::{
    env,
    sync::Arc,
};
use std::cell::RefCell;
use std::rc::Rc;
use anyhow::{Context, Result};
use vim_rs::core::client::{Client, ClientBuilder};
use app::App;
use crate::event::EventHandler;
use vim_rs::core::pc_helpers::{Monitor, CacheManager, ObjectCache, SharedRefCacheProxy};
use crate::vm_list::VmListWidget;

mod vm;
mod event;
mod vm_list;
mod app;
mod vm_disp;


const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    let client = init_vim_client().await?;
    let cache = Rc::new(RefCell::new(ObjectCache::new()));
    let cache_manager = Rc::new(RefCell::new(CacheManager::new(client.clone())?));
    let _vm_cache_filter = cache_manager.borrow_mut().add_container_cache(
        Box::new(SharedRefCacheProxy::new(cache.clone())),
        &client.service_content().root_folder,
    ).await?;
    let widget = VmListWidget::new(cache.clone());
    let monitor = Monitor::new(
        client.clone())?;
    let event_handler = EventHandler::new(monitor);
    let terminal = ratatui::init();
    let app_result = App::new(event_handler, cache_manager.clone(), widget).run(terminal).await;
    ratatui::restore();
    cache_manager.borrow_mut().destroy().await?;
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