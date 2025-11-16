use anyhow::Result;
use std::path::Path;
use std::time::Instant;
use tracing::info;
use build_api_definitions::build_api_definitions;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(std::io::stderr)
        .init();

    // Determine paths relative to the workspace root
    // build_api_definitions is at mcp/data_processing/build_api_definitions/
    // So we need to go up 3 levels to get to workspace root
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()  // data_processing
        .unwrap()
        .parent()  // mcp
        .unwrap()
        .parent()  // workspace root
        .unwrap();

    let vi_json_spec_path = workspace_root
        .join("vim_build/data/vi_json_openapi_specification_v9_0_0_0_24798170.json");
    
    let mcp_output_dir = workspace_root.join("mcp/data/api_definitions");

    info!("Loading OpenAPI specification from: {}", vi_json_spec_path.display());
    info!("Output directory: {}", mcp_output_dir.display());

    let start = Instant::now();
    
    build_api_definitions(&vi_json_spec_path, &mcp_output_dir)?;
    
    info!("Total time: {:?}", start.elapsed());

    Ok(())
}

