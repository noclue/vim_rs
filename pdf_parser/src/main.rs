use extractous::Extractor;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration for PDF parsing
struct Config {
    input_dir: PathBuf,
    output_dir: PathBuf,
    pdf_files: Vec<String>,
}

impl Config {
    fn new() -> Self {
        let base_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

        Config {
            input_dir: base_dir.join("data"),
            output_dir: base_dir.parent().unwrap().join("mcp").join("guides"),
            pdf_files: vec!["vcf.pdf".to_string(), "vsphere.pdf".to_string()],
        }
    }
}

/// Extract text from a PDF file
fn extract_pdf_text(pdf_path: &Path) -> Result<String, Box<dyn Error>> {
    println!("📄 Processing: {}", pdf_path.display());

    // Create extractor instance
    let extractor = Extractor::new();

    // Read the PDF file
    let file_content = fs::read(pdf_path)
        .map_err(|e| format!("Failed to read PDF file {}: {}", pdf_path.display(), e))?;

    // Extract text from the PDF
    let text = extractor
        .extract_bytes(&file_content)
        .map_err(|e| format!("Failed to extract text from {}: {}", pdf_path.display(), e))?;

    println!("✅ Extracted {} characters", text.len());
    Ok(text)
}

/// Save extracted text to output file
fn save_text(text: &str, output_path: &Path) -> Result<(), Box<dyn Error>> {
    println!("💾 Saving to: {}", output_path.display());

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory {}: {}", parent.display(), e))?;
    }

    // Write the text file
    fs::write(output_path, text)
        .map_err(|e| format!("Failed to write output file {}: {}", output_path.display(), e))?;

    println!("✅ Saved successfully");
    Ok(())
}

/// Process a single PDF file
fn process_pdf(config: &Config, pdf_filename: &str) -> Result<(), Box<dyn Error>> {
    println!("\n{'=':<60}", "");
    println!("Processing: {}", pdf_filename);
    println!("{'=':<60}\n", "");

    // Construct input path
    let input_path = config.input_dir.join(pdf_filename);

    // Check if file exists
    if !input_path.exists() {
        println!("⚠️  File not found: {}", input_path.display());
        println!("   Please place {} in the pdf_parser/data/ directory", pdf_filename);
        return Ok(()); // Skip this file but don't fail the whole process
    }

    // Get file size for reporting
    let file_size = fs::metadata(&input_path)?.len();
    println!("📊 File size: {:.2} MB", file_size as f64 / 1_048_576.0);

    // Extract text
    let text = extract_pdf_text(&input_path)?;

    // Create output filename (replace .pdf with .txt)
    let output_filename = pdf_filename.replace(".pdf", ".txt");
    let output_path = config.output_dir.join(&output_filename);

    // Save extracted text
    save_text(&text, &output_path)?;

    println!("\n✨ Successfully processed {}", pdf_filename);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n🚀 PDF Text Extractor");
    println!("{'=':<60}\n", "");

    let config = Config::new();

    println!("📂 Input directory:  {}", config.input_dir.display());
    println!("📂 Output directory: {}", config.output_dir.display());
    println!();

    let mut success_count = 0;
    let mut error_count = 0;

    // Process each PDF file
    for pdf_file in &config.pdf_files {
        match process_pdf(&config, pdf_file) {
            Ok(_) => success_count += 1,
            Err(e) => {
                eprintln!("❌ Error processing {}: {}", pdf_file, e);
                error_count += 1;
            }
        }
    }

    // Summary
    println!("\n{'=':<60}", "");
    println!("📊 Summary");
    println!("{'=':<60}", "");
    println!("✅ Successful: {}", success_count);
    println!("❌ Errors: {}", error_count);
    println!();

    if error_count > 0 {
        return Err("Some PDFs failed to process".into());
    }

    Ok(())
}
