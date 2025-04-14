use log::info;
use utils::connect;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    info!("Root folder permissions: {}", client.fetch_property::<serde_json::Value>(client.service_content().root_folder.clone(), "permission").await?);

    Ok(())
}