use pdf_parser::{process_pdfs, Config};
use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;
use tracing::info;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging to stderr
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let total_start = Instant::now();
    
    info!("");
    info!("🚀 PDF Text Extractor");
    info!("{:=<60}", "");
    info!("");

    // Navigate to mcp/data/guides from mcp/data_processing/pdf_parser
    let mcp_data_guides = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()  // -> mcp/data_processing
        .unwrap()
        .parent()  // -> mcp
        .unwrap()
        .join("data")
        .join("guides");

    let config = Config {
        input_dir: mcp_data_guides.join("pdf"),
        output_dir: mcp_data_guides.join("txt"),
    };

    info!("📂 Input directory:  {}", config.input_dir.display());
    info!("📂 Output directory: {}", config.output_dir.display());
    info!("");

    process_pdfs(&config, total_start)?;
    Ok(())
}
