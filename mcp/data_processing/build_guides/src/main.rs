use anyhow::Result;
use build_guides::process_single_file;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Initialize logging to stderr
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::TRACE)
        .init();

    // Navigate to mcp/data/guides from mcp/data_processing/pdf_parser
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()  // -> mcp/data_processing
        .unwrap()
        .parent()  // -> mcp
        .unwrap()
        .join("data");

    let guides_dir = data_dir.join("guides");
    let md_dir = guides_dir.join("md");
    let json_dir = data_dir.join("api_definitions");

    let input_path = md_dir.join("vmware-vsphere-9-0.md");
    let output_path = json_dir.join("vmware-vsphere-9-0_guide.json");

    process_single_file(&input_path, &output_path)?;
    Ok(())
}
