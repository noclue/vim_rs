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
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);

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
