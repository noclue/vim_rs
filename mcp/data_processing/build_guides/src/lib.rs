mod markdown_parser;
mod chunker;
mod topic_extractor;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideChunk {
    /// H1 heading (document/chapter title)
    pub heading_h1: String,

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

    /// Index of this chunk within the article (1-based)
    pub chunk_index: usize,

    /// Total number of chunks in the article
    pub chunk_count: usize,
}

/// Process a single markdown file and write the output as JSON
pub fn process_single_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    info!("Reading markdown file: {}", input_path.display());
    let content = fs::read_to_string(&input_path)?;

    info!("Parsing markdown structure...");
    let sections = markdown_parser::parse_markdown(&content)?;

    info!("Found {} sections", sections.len());

    info!("Chunking sections...");
    let source_file = input_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let chunks = chunker::create_chunks(&sections, &source_file)?;

    info!("Created {} chunks", chunks.len());

    // Statistics
    let word_counts: Vec<usize> = chunks.iter().map(|c| c.word_count).collect();
    let min_words = word_counts.iter().min().unwrap_or(&0);
    let max_words = word_counts.iter().max().unwrap_or(&0);
    let avg_words = if !word_counts.is_empty() {
        word_counts.iter().sum::<usize>() / word_counts.len()
    } else {
        0
    };

    info!("Word count stats: min={}, max={}, avg={}", min_words, max_words, avg_words);

    info!("Writing output: {}", output_path.display());
    let json = serde_json::to_string_pretty(&chunks)?;
    fs::write(&output_path, json)?;

    info!("Done!");

    Ok(())
}

