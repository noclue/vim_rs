use anyhow::{Context, Result};
use build_api_definitions::build_api_data;
use build_examples::collect_examples;
use build_embeddings::generate_embeddings;
use api_database::{ApiData, ApiDatabase};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::time::Instant;
use tracing::{info, error};
use clap::Parser;

/// Data Transformer - Builds the unified API database for vim_rs MCP server
#[derive(Parser, Debug)]
#[command(name = "data-transformer")]
#[command(about = "Builds the unified API database for vim_rs MCP server")]
struct Args {
    /// Also emit JSON files for debugging (in addition to binary)
    #[arg(long)]
    emit_json: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();
    let total_start = Instant::now();

    info!("");
    info!("╔════════════════════════════════════════════════════════════╗");
    info!("║         Data Transformer - Unified Binary Builder          ║");
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
    let vim_build_data_dir = workspace_root.join("vim_build").join("data");
    let vi_json_spec_path = vim_build_data_dir.join("vi_json_openapi_specification_v9_0_0_0_24798170.json");
    let model_cache_dir = mcp_data_dir.join("model_cache");
    let output_path = mcp_data_dir.join("api_database.bin");

    // Ensure directories exist
    std::fs::create_dir_all(&mcp_data_dir)?;
    std::fs::create_dir_all(&model_cache_dir)?;

    // =========================================================================
    // Step 1: Build API Definitions (in-memory)
    // =========================================================================
    info!("{:=<70}", "");
    info!("Step 1/4: Build API Definitions");
    info!("{:=<70}", "");
    let step_start = Instant::now();
    
    info!("OpenAPI spec: {}", vi_json_spec_path.display());
    
    let api_defs = build_api_data(&vi_json_spec_path)
        .context("Failed to build API definitions")?;
    
    info!("✓ Built {} managed objects, {} structures, {} enumerations, {} traits in {:?}",
        api_defs.managed_objects.len(),
        api_defs.structures.len(),
        api_defs.enumerations.len(),
        api_defs.traits.len(),
        step_start.elapsed()
    );
    info!("");

    // =========================================================================
    // Step 2: Collect Examples (in-memory)
    // =========================================================================
    info!("{:=<70}", "");
    info!("Step 2/4: Collect Examples");
    info!("{:=<70}", "");
    let step_start = Instant::now();

    info!("Examples directory: {}", examples_dir.display());

    if !examples_dir.exists() {
        error!("Examples directory not found at {}", examples_dir.display());
        anyhow::bail!("Examples directory not found");
    }

    let examples = collect_examples(&examples_dir)
        .context("Failed to collect examples")?;
    
    info!("✓ Collected {} examples in {:?}", examples.len(), step_start.elapsed());
    info!("");

    // =========================================================================
    // Step 3: Construct ApiData from parts
    // =========================================================================
    info!("{:=<70}", "");
    info!("Step 3/4: Construct ApiData");
    info!("{:=<70}", "");
    let step_start = Instant::now();

    let api_data = ApiData::from_parts(
        api_defs.managed_objects,
        api_defs.structures,
        api_defs.enumerations,
        api_defs.traits,
        examples,
    );

    info!("✓ Constructed ApiData with {} items in {:?}", 
        api_data.items.len(), 
        step_start.elapsed()
    );
    info!("");

    // =========================================================================
    // Step 4: Generate Embeddings
    // =========================================================================
    info!("{:=<70}", "");
    info!("Step 4/4: Generate Embeddings");
    info!("{:=<70}", "");
    let step_start = Instant::now();

    info!("Model cache: {}", model_cache_dir.display());

    let embeddings = generate_embeddings(&api_data, &model_cache_dir)
        .context("Failed to generate embeddings")?;

    info!("✓ Generated {} embeddings in {:?}", embeddings.len(), step_start.elapsed());
    info!("");

    // =========================================================================
    // Write unified binary database
    // =========================================================================
    info!("{:=<70}", "");
    info!("Writing Unified Database");
    info!("{:=<70}", "");

    let database = ApiDatabase {
        items: api_data.items,
        embeddings: Some(embeddings),
    };

    info!("Output: {}", output_path.display());

    let file = File::create(&output_path)
        .context("Failed to create output file")?;
    let writer = BufWriter::new(file);

    bincode::serialize_into(writer, &database)
        .context("Failed to serialize database")?;

    let file_size = std::fs::metadata(&output_path)?.len();
    info!("✓ Wrote {} items with embeddings ({:.2} MB)",
        database.items.len(),
        file_size as f64 / (1024.0 * 1024.0)
    );
    info!("");

    // =========================================================================
    // Optional: Emit JSON files for debugging
    // =========================================================================
    if args.emit_json {
        info!("{:=<70}", "");
        info!("Emitting JSON Files (--emit-json)");
        info!("{:=<70}", "");

        let json_output_dir = mcp_data_dir.join("api_definitions");
        std::fs::create_dir_all(&json_output_dir)?;

        // Use the legacy JSON emit functions
        build_api_definitions::build_api_definitions(&vi_json_spec_path, &json_output_dir)?;
        build_examples::build_examples(&examples_dir, &json_output_dir)?;

        info!("✓ Wrote JSON files to {}", json_output_dir.display());
        info!("");
    }

    // =========================================================================
    // Summary
    // =========================================================================
    let total_elapsed = total_start.elapsed();
    info!("{:=<70}", "");
    info!("✨ Build completed successfully!");
    info!("{:=<70}", "");
    info!("Output: {}", output_path.display());
    info!("Size: {:.2} MB", file_size as f64 / (1024.0 * 1024.0));
    info!("Items: {}", database.items.len());
    info!("Total time: {:?}", total_elapsed);
    info!("");

    Ok(())
}
