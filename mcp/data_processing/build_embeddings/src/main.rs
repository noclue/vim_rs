use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::info;
use vim_mcp_server::model::ApiData;
use build_embeddings::build_embeddings;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(std::io::stderr)
        .init();

    info!("Starting embedding generation for vim_rs MCP server");

    // Paths - navigate to mcp/data/
    let mcp_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("data");
    let embeddings_db_path = mcp_data_dir.join("embeddings.lancedb");
    let model_cache_dir = mcp_data_dir.join("model_cache");

    // Create cache directory if it doesn't exist
    if !model_cache_dir.exists() {
        std::fs::create_dir_all(&model_cache_dir)
            .context("Failed to create model cache directory")?;
    }

    // ApiData::load_from_dir expects the parent data directory, not api_definitions
    // because it looks for data_dir/guides/ subdirectory
    let api_data = ApiData::load_from_dir(&mcp_data_dir)
        .context("Failed to load API data from JSON files")?;

    build_embeddings(&api_data, &embeddings_db_path, model_cache_dir).await?;

    Ok(())
}
