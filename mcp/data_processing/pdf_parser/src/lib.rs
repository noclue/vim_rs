use extractous::Extractor;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tracing::{info, warn, error};

/// Configuration for PDF parsing
pub struct Config {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl Config {
    /// Scan input directory for all PDF files
    pub fn find_pdf_files(&self) -> Result<Vec<String>, Box<dyn Error>> {
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
pub fn extract_pdf_text(pdf_path: &Path) -> Result<String, Box<dyn Error>> {
    info!("📄 Processing: {}", pdf_path.display());

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
    
    info!("✅ Extracted {} characters", text.len());
    Ok(text)
}

/// Save extracted text to output file
pub fn save_text(text: &str, output_path: &Path) -> Result<(), Box<dyn Error>> {
    info!("💾 Saving to: {}", output_path.display());

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory {}: {}", parent.display(), e))?;
    }

    // Write the text file
    fs::write(output_path, text)
        .map_err(|e| format!("Failed to write output file {}: {}", output_path.display(), e))?;

    info!("✅ Saved successfully");
    Ok(())
}

/// Format duration as human-readable string
pub fn format_duration(duration: Duration) -> String {
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
pub fn process_pdf(config: &Config, pdf_filename: &str) -> Result<Duration, Box<dyn Error>> {
    let start_time = Instant::now();
    
    info!("");
    info!("{:=<60}", "");
    info!("Processing: {}", pdf_filename);
    info!("{:=<60}", "");
    info!("");

    // Construct input path
    let input_path = config.input_dir.join(pdf_filename);

    // Check if file exists
    if !input_path.exists() {
        warn!("⚠️  File not found: {}", input_path.display());
        warn!("   Please place {} in the mcp/data/guides/pdf/ directory", pdf_filename);
        return Ok(Duration::ZERO); // Skip this file but don't fail the whole process
    }

    // Get file size for reporting
    let file_size = fs::metadata(&input_path)?.len();
    info!("📊 File size: {:.2} MB", file_size as f64 / 1_048_576.0);

    // Extract text
    let text = extract_pdf_text(&input_path)?;

    // Create output filename (replace .pdf with .txt)
    let output_filename = pdf_filename.replace(".pdf", ".txt");
    let output_path = config.output_dir.join(&output_filename);

    // Save extracted text
    save_text(&text, &output_path)?;

    let elapsed = start_time.elapsed();
    info!("");
    info!("✨ Successfully processed {} in {}", pdf_filename, format_duration(elapsed));

    Ok(elapsed)
}

/// Process all PDF files in the configured directory
pub fn process_pdfs(config: &Config, total_start: Instant) -> Result<(), Box<dyn Error>> {
    // Scan for PDF files
    let pdf_files = config.find_pdf_files()?;
    
    if pdf_files.is_empty() {
        warn!("⚠️  No PDF files found in {}", config.input_dir.display());
        warn!("   Please place PDF files in the mcp/data/guides/pdf/ directory");
        return Ok(());
    }

    info!("📋 Found {} PDF file(s):", pdf_files.len());
    for pdf_file in &pdf_files {
        info!("   • {}", pdf_file);
    }
    info!("");

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
                error!("❌ Error processing {}: {}", pdf_file, e);
                error_count += 1;
            }
        }
    }

    let total_elapsed = total_start.elapsed();

    // Summary
    info!("");
    info!("{:=<60}", "");
    info!("📊 Summary");
    info!("{:=<60}", "");
    info!("✅ Successful: {}", success_count);
    info!("❌ Errors: {}", error_count);
    info!("");
    
    // Per-file timing
    if !processing_times.is_empty() {
        info!("⏱️  Processing Times:");
        for (filename, duration) in &processing_times {
            info!("   • {}: {}", filename, format_duration(*duration));
        }
        info!("");
    }
    
    // Total time
    info!("⏱️  Total time: {}", format_duration(total_elapsed));
    info!("");

    if error_count > 0 {
        return Err("Some PDFs failed to process".into());
    }

    Ok(())
}

