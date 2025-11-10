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
    let args: Vec<String> = std::env::args().collect();

    // Check for --toc-only flag
    let toc_only = args.iter().any(|arg| arg == "--toc-only");

    if args.len() < 2 || (args.len() == 2 && args[1] == "--toc-only") {
        eprintln!("Usage: {} <input_file.txt> [output_file.md] [--toc-only]", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} ../guides/vsphere.txt", args[0]);
        eprintln!("  {} ../guides/vsphere.txt ../guides/vsphere_clean.md", args[0]);
        eprintln!("  {} ../guides/vsphere.txt ../guides/toc.md --toc-only", args[0]);
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = if args.len() >= 3 && args[2] != "--toc-only" {
        PathBuf::from(&args[2])
    } else {
        // Default: input_clean.md or input_toc.md for TOC-only mode
        let stem = input_path.file_stem().unwrap().to_string_lossy();
        if toc_only {
            input_path.with_file_name(format!("{}_toc.md", stem))
        } else {
            input_path.with_file_name(format!("{}_clean.md", stem))
        }
    };

    println!("Text Processor for vSphere Documentation");
    println!("========================================\n");
    println!("Input:  {}", input_path.display());
    println!("Output: {}\n", output_path.display());

    // Read input file
    let content = fs::read_to_string(&input_path)
        .with_context(|| format!("Failed to read {}", input_path.display()))?;

    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    println!("Read {} lines\n", lines.len());

    if toc_only {
        // TOC-only mode: just parse and output the TOC as markdown
        println!("TOC-only mode: Parsing and exporting TOC...\n");
        let toc_markdown = toc_parser::parse_and_export_toc(&lines)?;
        
        fs::write(&output_path, toc_markdown)
            .with_context(|| format!("Failed to write {}", output_path.display()))?;
        
        println!("✓ TOC export complete!");
        println!("  Output: {}", output_path.display());
    } else {
        // Normal mode: full processing pipeline
        
        // Pass 1: Parse TOC
        println!("Pass 1: Parsing TOC (first 73 pages)...");
        let toc_map = toc_parser::parse_toc(&lines)?;
        println!("  Found {} pages with TOC entries\n", toc_map.len());

        // Pass 2: Mark headings
        println!("Pass 2: Marking headings (exact match)...");
        let lines = heading_marker::mark_headings(&lines, &toc_map)?;
        println!();

        // Pass 3: Remove headers/footers/URLs
        println!("Pass 3: Removing page markers...");
        let lines = cleanup::remove_page_markers(&lines);
        println!("  {} lines remain\n", lines.len());

        // Pass 4: Fix bullets
        println!("Pass 4: Fixing bullet points...");
        let lines = bullet_fixer::fix_bullets(&lines);
        println!("  Converted • to * and – to nested bullets\n");

        // Pass 5: Merge broken lists
        println!("Pass 5: Merging broken list items...");
        let lines = list_merger::merge_lists(&lines);
        println!("  {} lines after merging\n", lines.len());

        // Write output
        let output = lines.join("\n");
        fs::write(&output_path, output)
            .with_context(|| format!("Failed to write {}", output_path.display()))?;

        println!("✓ Processing complete!");
        println!("  Output: {}", output_path.display());
    }

    Ok(())
}
