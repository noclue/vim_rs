mod glyph_widths;
mod toc_parser;
mod heading_marker;
mod cleanup;
mod bullet_fixer;
mod list_merger;

use anyhow::Result;
use text_processor::process_all_files;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Initialize logging to stderr
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Navigate to mcp/data/guides from mcp/data_processing/text_processor
    let mcp_data_guides = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()  // -> mcp/data_processing
        .unwrap()
        .parent()  // -> mcp
        .unwrap()
        .join("data")
        .join("guides");
    
    let txt_dir = mcp_data_guides.join("txt");
    let md_dir = mcp_data_guides.join("md");

    // Process all txt files from txt_dir to md_dir
    process_all_files(&txt_dir, &md_dir)
}
