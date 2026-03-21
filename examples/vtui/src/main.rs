use crate::event::EventHandler;
use anyhow::{Context, Result};
use app::App;
use std::cell::RefCell;
use std::rc::Rc;
use std::env;
use std::fs::File;
use std::path::Path;
use log::{info, LevelFilter};
use simplelog::{Config, WriteLogger};
use vim_rs::core::client::{ClientBuilder, TransportMode, VimClientHandle};
use vim_rs::core::pc_cache::CacheManager;

mod app;
mod event;
mod search;
mod resource_type;
mod hints;
mod resource_browser;
mod prop_browser;
mod body_pane;
mod history;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from a local `.env` file if present.
    // This is optional; real environment variables still take precedence.
    let _ = dotenvy::dotenv();

    setup_logging()?;
    
    info!("Starting vtui application!");
    
    let client = match init_vim_client().await {
        Ok(client) => client,
        Err(err) => {
            print_usage();
            eprintln!("Error initializing client: {}", err);
            return Err(err.into());
        }
    };
    let cache_manager = Rc::new(RefCell::new(CacheManager::new(client.clone())?));
    let monitor = cache_manager.borrow().create_monitor()?;
    let event_handler = EventHandler::new(monitor);
    let terminal = ratatui::init();

    let app_result = App::new(event_handler, cache_manager.clone(), client.clone()).await?
        .run(terminal)
        .await;
    ratatui::restore();
    cache_manager.borrow_mut().destroy().await?;
    app_result
}

async fn init_vim_client() -> Result<VimClientHandle> {
    let vc_server = env::var("VIM_SERVER").with_context(|| "VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").with_context(|| "VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").with_context(|| "VIM_PASSWORD env var not set")?;
    let insecure = env::var("VIM_INSECURE").map(|insecure| insecure != "false").unwrap_or(false);

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(insecure)
        .transport(TransportMode::Auto)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        .build()
        .await?;
    Ok(client)
}

fn print_usage() {
    println!("Usage: vtui");
    println!("Make sure to set the following environment variables:");
    println!("VIM_SERVER: The server address (FQDN or IP) of the vSphere instance");
    println!("VIM_USERNAME: The username to connect to the vSphere instance");
    println!("VIM_PASSWORD: The password to connect to the vSphere instance");
    println!("VIM_INSECURE: Flag to allow insecure connections (default: false)");
    println!("LOG_LEVEL: The log level (trace, debug, info, warn, error off) (default: info). Use 'trace' for wire logging.");
}


fn setup_logging() -> anyhow::Result<()> {
    // Create logs directory if it doesn't exist
    std::fs::create_dir_all("logs")?;

    let log_file_path = Path::new("logs/vtui.log");


    WriteLogger::init(
        log_level(),
        Config::default(),
        File::create(log_file_path)?,
    ).map_err(|e| anyhow::anyhow!("Failed to initialize logger: {}", e))?;

    info!("Logging system initialized");
    Ok(())
}

fn log_level() -> LevelFilter {
    match env::var("LOG_LEVEL").as_deref() {
        Ok("trace") => LevelFilter::Trace,
        Ok("debug") => LevelFilter::Debug,
        Ok("info") => LevelFilter::Info,
        Ok("warn") => LevelFilter::Warn,
        Ok("error") => LevelFilter::Error,
        Ok("off") => LevelFilter::Off,
        _ => LevelFilter::Info, // Default log level
    }
}