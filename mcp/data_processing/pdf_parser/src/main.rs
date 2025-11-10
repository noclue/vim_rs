use extractous::Extractor;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Configuration for PDF parsing
struct Config {
    input_dir: PathBuf,
    output_dir: PathBuf,
}

impl Config {
    fn new() -> Self {
        // Navigate to mcp/data/guides from mcp/data_processing/pdf_parser
        let mcp_data_guides = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()  // -> mcp/data_processing
            .unwrap()
            .parent()  // -> mcp
            .unwrap()
            .join("data")
            .join("guides");

        Config {
            input_dir: mcp_data_guides.join("pdf"),
            output_dir: mcp_data_guides.join("txt"),
        }
    }

    /// Scan input directory for all PDF files
    fn find_pdf_files(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut pdf_files = Vec::new();

        if !self.input_dir.exists() {
            return Err(format!("Input directory does not exist: {}", self.input_dir.display()).into());
        }

        for entry in fs::read_dir(&self.input_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension.eq_ignore_ascii_case("pdf") {
                        if let Some(filename) = path.file_name() {
                            pdf_files.push(filename.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        pdf_files.sort();
        Ok(pdf_files)
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
    let (mut stream, _metadata) = extractor
        .extract_bytes(&file_content)
        .map_err(|e| format!("Failed to extract text from {}: {}", pdf_path.display(), e))?;

    // Read stream to string
    let mut text = String::new();
    stream.read_to_string(&mut text)
        .map_err(|e| format!("Failed to read extracted text: {}", e))?;
    
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

/// Format duration as human-readable string
fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    if total_secs >= 3600 {
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        format!("{}h {}m {}s", hours, mins, secs)
    } else if total_secs >= 60 {
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{}m {}s", mins, secs)
    } else {
        format!("{}.{:03}s", total_secs, millis)
    }
}

/// Process a single PDF file
fn process_pdf(config: &Config, pdf_filename: &str) -> Result<Duration, Box<dyn Error>> {
    let start_time = Instant::now();
    
    println!("\n{:=<60}", "");
    println!("Processing: {}", pdf_filename);
    println!("{:=<60}\n", "");

    // Construct input path
    let input_path = config.input_dir.join(pdf_filename);

    // Check if file exists
    if !input_path.exists() {
        println!("⚠️  File not found: {}", input_path.display());
        println!("   Please place {} in the mcp/data/guides/pdf/ directory", pdf_filename);
        return Ok(Duration::ZERO); // Skip this file but don't fail the whole process
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

    let elapsed = start_time.elapsed();
    println!("\n✨ Successfully processed {} in {}", pdf_filename, format_duration(elapsed));

    Ok(elapsed)
}

fn main() -> Result<(), Box<dyn Error>> {
    let total_start = Instant::now();
    
    println!("\n🚀 PDF Text Extractor");
    println!("{:=<60}\n", "");

    let config = Config::new();

    println!("📂 Input directory:  {}", config.input_dir.display());
    println!("📂 Output directory: {}", config.output_dir.display());
    println!();

    // Scan for PDF files
    let pdf_files = config.find_pdf_files()?;
    
    if pdf_files.is_empty() {
        println!("⚠️  No PDF files found in {}", config.input_dir.display());
        println!("   Please place PDF files in the mcp/data/guides/pdf/ directory");
        return Ok(());
    }

    println!("📋 Found {} PDF file(s):", pdf_files.len());
    for pdf_file in &pdf_files {
        println!("   • {}", pdf_file);
    }
    println!();

    let mut success_count = 0;
    let mut error_count = 0;
    let mut processing_times: Vec<(String, Duration)> = Vec::new();

    // Process each PDF file
    for pdf_file in &pdf_files {
        match process_pdf(&config, pdf_file) {
            Ok(duration) => {
                success_count += 1;
                processing_times.push((pdf_file.clone(), duration));
            }
            Err(e) => {
                eprintln!("❌ Error processing {}: {}", pdf_file, e);
                error_count += 1;
            }
        }
    }

    let total_elapsed = total_start.elapsed();

    // Summary
    println!("\n{:=<60}", "");
    println!("📊 Summary");
    println!("{:=<60}", "");
    println!("✅ Successful: {}", success_count);
    println!("❌ Errors: {}", error_count);
    println!();
    
    // Per-file timing
    if !processing_times.is_empty() {
        println!("⏱️  Processing Times:");
        for (filename, duration) in &processing_times {
            println!("   • {}: {}", filename, format_duration(*duration));
        }
        println!();
    }
    
    // Total time
    println!("⏱️  Total time: {}", format_duration(total_elapsed));
    println!();

    if error_count > 0 {
        return Err("Some PDFs failed to process".into());
    }

    Ok(())
}
