use crate::event::EventHandler;
use crate::resource_table::ResourceTableWidget;
use anyhow::{Context, Result};
use app::App;
use std::cell::RefCell;
use std::rc::Rc;
use std::{env, sync::Arc};
use vim_rs::core::client::{Client, ClientBuilder};
use vim_rs::core::pc_cache::{CacheManager, ObjectCache, SharedRefCacheProxy};
use crate::indexed_cache::IndexedCache;

mod app;
mod event;
mod vm;
mod resource_table;
mod search;
mod host;
mod indexed_cache;
mod tabular_data;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    let client = init_vim_client().await?;
    let cache = Rc::new(RefCell::new(ObjectCache::new()));
    let cache_manager = Rc::new(RefCell::new(CacheManager::new(client.clone())?));
    let _vm_cache_filter = cache_manager
        .borrow_mut()
        .add_container_cache(
            Box::new(SharedRefCacheProxy::new(cache.clone())),
            &client.service_content().root_folder,
        )
        .await?;
    let indexed_cache= IndexedCache::new(cache.clone());
    let widget = ResourceTableWidget::new(indexed_cache);
    let monitor = cache_manager.borrow().create_monitor()?;
    let event_handler = EventHandler::new(monitor);
    let terminal = ratatui::init();
    let app_result = App::new(event_handler, cache_manager.clone(), widget)
        .run(terminal)
        .await;
    ratatui::restore();
    cache_manager.borrow_mut().destroy().await?;
    app_result
}

async fn init_vim_client() -> Result<Arc<Client>> {
    let vc_server = env::var("VIM_SERVER").with_context(|| "VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").with_context(|| "VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").with_context(|| "VIM_PASSWORD env var not set")?;

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(APP_NAME, APP_VERSION)
        .build()
        .await?;
    Ok(client)
}
