use anyhow::{Context, Result};
use build_examples::build_examples;
use build_embeddings::build_embeddings;
use build_api_definitions::build_api_definitions;
use vim_mcp_server::model::ApiData;
use std::path::PathBuf;
use std::time::Instant;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    let total_start = Instant::now();

    info!("");
    info!("╔════════════════════════════════════════════════════════════╗");
    info!("║         Data Transformer Orchestrator                      ║");
    info!("╚════════════════════════════════════════════════════════════╝");
    info!("");

    // Calculate workspace root (3 levels up from data_transformer)
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let data_processing_dir = manifest_dir.parent().unwrap();
    let mcp_dir = data_processing_dir.parent().unwrap();
    let workspace_root = mcp_dir.parent().unwrap().to_path_buf();

    info!("Workspace root: {}", workspace_root.display());
    info!("");

    // Define all paths relative to workspace root
    let mcp_data_dir = workspace_root.join("mcp").join("data");
    let examples_dir = workspace_root.join("examples");
    let vim_build_data_dir = workspace_root.join("vim_build/data");

    // Step 1: Build API Definitions
    info!("{:=<70}", "");
    info!("Step 1/3: Build API Definitions");
    info!("{:=<70}", "");
    let step_start = Instant::now();
    match run_build_api_definitions(&mcp_data_dir, &vim_build_data_dir) {
        Ok(_) => {
            info!("✓ Build API Definitions completed in {:?}", step_start.elapsed());
        }
        Err(e) => {
            error!("✗ Build API Definitions failed: {}", e);
            return Err(e).context("Build API Definitions step failed");
        }
    }
    info!("");

    // Step 2: Build Examples
    info!("{:=<70}", "");
    info!("Step 2/3: Build Examples");
    info!("{:=<70}", "");
    let step_start = Instant::now();
    match run_build_examples(&examples_dir, &mcp_data_dir) {
        Ok(_) => {
            info!("✓ Build Examples completed in {:?}", step_start.elapsed());
        }
        Err(e) => {
            error!("✗ Build Examples failed: {}", e);
            return Err(e).context("Build Examples step failed");
        }
    }
    info!("");

    // Step 3: Build Embeddings
    info!("{:=<70}", "");
    info!("Step 3/3: Build Embeddings");
    info!("{:=<70}", "");
    let step_start = Instant::now();
    match run_build_embeddings(&mcp_data_dir).await {
        Ok(_) => {
            info!("✓ Build Embeddings completed in {:?}", step_start.elapsed());
        }
        Err(e) => {
            error!("✗ Build Embeddings failed: {}", e);
            return Err(e).context("Build Embeddings step failed");
        }
    }
    info!("");

    // Final summary
    let total_elapsed = total_start.elapsed();
    info!("{:=<70}", "");
    info!("✨ All steps completed successfully!");
    info!("{:=<70}", "");
    info!("Total execution time: {:?}", total_elapsed);
    info!("");

    Ok(())
}

fn run_build_api_definitions(
    mcp_data_dir: &PathBuf,
    vim_build_data_dir: &PathBuf,
) -> Result<()> {
    let vi_json_spec_path = vim_build_data_dir.join("vi_json_openapi_specification_v9_0_0_0_24798170.json");
    let output_dir = mcp_data_dir.join("api_definitions");

    info!("OpenAPI spec: {}", vi_json_spec_path.display());
    info!("Output directory: {}", output_dir.display());

    build_api_definitions(&vi_json_spec_path, &output_dir)?;
    Ok(())
}

fn run_build_examples(examples_dir: &PathBuf, mcp_data_dir: &PathBuf) -> Result<()> {
    let output_dir = mcp_data_dir.join("api_definitions");

    info!("Examples directory: {}", examples_dir.display());
    info!("Output directory: {}", output_dir.display());

    if !examples_dir.exists() {
        anyhow::bail!("Examples directory not found at {}", examples_dir.display());
    }

    build_examples(examples_dir, &output_dir)?;
    Ok(())
}

async fn run_build_embeddings(mcp_data_dir: &PathBuf) -> Result<()> {
    let embeddings_db_path = mcp_data_dir.join("embeddings.bin");
    let model_cache_dir = mcp_data_dir.join("model_cache");

    info!("Embeddings database: {}", embeddings_db_path.display());
    info!("Model cache directory: {}", model_cache_dir.display());

    // Create cache directory if it doesn't exist
    if !model_cache_dir.exists() {
        std::fs::create_dir_all(&model_cache_dir)
            .context("Failed to create model cache directory")?;
    }

    // Load API data
    let api_data = ApiData::load_from_dir(mcp_data_dir)
        .context("Failed to load API data from JSON files")?;

    build_embeddings(&api_data, &embeddings_db_path, model_cache_dir).await?;
    Ok(())
}
