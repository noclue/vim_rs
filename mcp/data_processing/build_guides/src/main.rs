mod markdown_parser;
mod chunker;
mod topic_extractor;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideChunk {
    /// H2 heading (major topic)
    pub heading_h2: String,

    /// H3 heading (semantic unit)
    pub heading_h3: String,

    /// Sub-section identifier if split (e.g., "Part 1")
    pub sub_section: Option<String>,

    /// Chunk content (200-800 words)
    pub content: String,

    /// Word count
    pub word_count: usize,

    /// Source markdown file
    pub source_file: String,

    /// Unique chunk identifier
    pub chunk_id: String,

    /// Auto-extracted topics/keywords
    pub topics: Vec<String>,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: build_guides <input.md> <output.json>");
        eprintln!();
        eprintln!("Chunks markdown documentation into 200-800 word sections for embedding.");
        eprintln!();
        eprintln!("Expected paths:");
        eprintln!("  Input:  mcp/data/guides/md/<source>.md");
        eprintln!("  Output: mcp/data/guides/<source>.json");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  build_guides \\");
        eprintln!("    mcp/data/guides/md/vmware-vsphere-9-0.md \\");
        eprintln!("    mcp/data/guides/vmware-vsphere-9-0.json");
        eprintln!();
        eprintln!("Note: Output MUST go to mcp/data/guides/ directory");
        eprintln!("      (not api_definitions/) for MCP server to load it.");
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);

    // Validate output path is in guides directory
    if let Some(parent) = output_path.parent() {
        let parent_name = parent.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if parent_name != "guides" {
            eprintln!("WARNING: Output path should be in 'guides' directory!");
            eprintln!("         Current: {}", output_path.display());
            eprintln!("         Expected: .../mcp/data/guides/<filename>.json");
            eprintln!();
            eprintln!("The MCP server loads guide chunks from mcp/data/guides/");
            eprintln!("Files in other directories will not be loaded.");
            eprintln!();
        }
    }

    println!("Reading markdown file: {}", input_path.display());
    let content = fs::read_to_string(&input_path)?;

    println!("Parsing markdown structure...");
    let sections = markdown_parser::parse_markdown(&content)?;

    println!("Found {} H3 sections", sections.len());

    println!("Chunking sections...");
    let source_file = input_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let chunks = chunker::create_chunks(&sections, &source_file)?;

    println!("Created {} chunks", chunks.len());

    // Statistics
    let word_counts: Vec<usize> = chunks.iter().map(|c| c.word_count).collect();
    let min_words = word_counts.iter().min().unwrap_or(&0);
    let max_words = word_counts.iter().max().unwrap_or(&0);
    let avg_words = if !word_counts.is_empty() {
        word_counts.iter().sum::<usize>() / word_counts.len()
    } else {
        0
    };

    println!("Word count stats: min={}, max={}, avg={}", min_words, max_words, avg_words);

    println!("Writing output: {}", output_path.display());
    let json = serde_json::to_string_pretty(&chunks)?;
    fs::write(&output_path, json)?;

    println!("Done!");

    Ok(())
}
