mod glyph_widths;
mod toc_parser;
mod heading_marker;
mod cleanup;
mod bullet_fixer;
mod list_merger;

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
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

fn process_single_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    println!("Input:  {}", input_path.display());
    println!("Output: {}", output_path.display());

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create output directory {}", parent.display()))?;
    }

    // Read input file
    let content = fs::read_to_string(&input_path)
        .with_context(|| format!("Failed to read {}", input_path.display()))?;

    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    println!("Read {} lines", lines.len());

    // Pass 1: Parse TOC
    println!("  Pass 1: Parsing TOC...");
    let toc_map = toc_parser::parse_toc(&lines)?;

    // Pass 2: Mark headings
    println!("  Pass 2: Marking headings...");
    let lines = heading_marker::mark_headings(&lines, &toc_map)?;

    // Pass 3: Remove headers/footers/URLs
    println!("  Pass 3: Removing page markers...");
    let lines = cleanup::remove_page_markers(&lines);

    // Pass 4: Fix bullets
    println!("  Pass 4: Fixing bullet points...");
    let lines = bullet_fixer::fix_bullets(&lines);

    // Pass 5: Merge broken lists
    println!("  Pass 5: Merging broken list items...");
    let lines = list_merger::merge_lists(&lines);

    // Write output
    let output = lines.join("\n");
    fs::write(&output_path, output)
        .with_context(|| format!("Failed to write {}", output_path.display()))?;

    println!("  ✓ Complete! {} lines written\n", lines.len());

    Ok(())
}

fn process_all_files(txt_dir: &PathBuf, md_dir: &PathBuf) -> Result<()> {
    println!("Text Processor for vSphere Documentation");
    println!("=========================================\n");
    println!("Input:  {}", txt_dir.display());
    println!("Output: {}\n", md_dir.display());

    // Ensure output directory exists
    fs::create_dir_all(md_dir)
        .with_context(|| format!("Failed to create output directory {}", md_dir.display()))?;

    // Find all .txt files
    let mut txt_files = Vec::new();
    for entry in fs::read_dir(txt_dir)
        .with_context(|| format!("Failed to read directory {}", txt_dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("txt") {
                    txt_files.push(path);
                }
            }
        }
    }

    if txt_files.is_empty() {
        println!("⚠️  No .txt files found in {}", txt_dir.display());
        return Ok(());
    }

    txt_files.sort();
    println!("Found {} file(s) to process\n", txt_files.len());

    let mut success_count = 0;
    let mut error_count = 0;

    // Process each file
    for input_path in txt_files {
        let stem = input_path.file_stem().unwrap().to_string_lossy();
        let output_filename = format!("{}.md", stem);
        let output_path = md_dir.join(output_filename);

        println!("{:=<70}", "");
        println!("Processing: {}", input_path.file_name().unwrap().to_string_lossy());
        println!("{:=<70}", "");

        match process_single_file(&input_path, &output_path) {
            Ok(_) => {
                success_count += 1;
            }
            Err(e) => {
                eprintln!("  ❌ Error: {}\n", e);
                error_count += 1;
            }
        }
    }

    // Summary
    println!("{:=<70}", "");
    println!("Summary: {} successful, {} errors", success_count, error_count);
    println!("{:=<70}\n", "");

    if error_count > 0 {
        return Err(anyhow::anyhow!("Some files failed to process"));
    }

    Ok(())
}
