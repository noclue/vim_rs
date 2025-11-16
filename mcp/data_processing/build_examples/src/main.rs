use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::info;
use walkdir::WalkDir;
use build_examples::build_examples;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting example indexing for vim_rs MCP server");

    // Navigate to the examples directory
    let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("examples");

    if !examples_dir.exists() {
        anyhow::bail!("Examples directory not found at {}", examples_dir.display());
    }

    // Write to JSON - output to mcp/data/api_definitions/examples.json
    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("data")
        .join("api_definitions");


    build_examples(&examples_dir, &output_dir)?;

    Ok(())
}
