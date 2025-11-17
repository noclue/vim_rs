use crate::markdown_parser::Section;
use crate::topic_extractor;
use crate::GuideChunk;
use anyhow::Result;

const MAX_WORDS: usize = 800;

/// Create chunks from parsed sections
pub fn create_chunks(sections: &[Section], source_file: &str) -> Result<Vec<GuideChunk>> {
    let mut chunks = Vec::new();

    for section in sections {
        let content_text = section.content.join("\n");
        let word_count = count_words(&content_text);

        // Skip empty sections
        if word_count == 0 {
            continue;
        }

        // If section fits within max, create single chunk
        if word_count <= MAX_WORDS {
            let chunk = create_chunk(
                &section.h1,
                &section.h2,
                &section.h3,
                None,
                &content_text,
                source_file,
                &chunks,
                1,  // chunk_index
                1,  // chunk_count
            );
            chunks.push(chunk);
        } else {
            // Split oversized section on paragraph boundaries
            let sub_chunks = split_on_paragraphs(&section.content);
            
            // Filter out empty sub-chunks and collect valid content
            let valid_sub_chunks: Vec<&String> = sub_chunks.iter()
                .filter(|sc| count_words(sc) > 0)
                .collect();
            
            let total_chunks = valid_sub_chunks.len();
            
            for (idx, sub_content) in valid_sub_chunks.iter().enumerate() {
                let sub_section = if total_chunks > 1 {
                    Some(format!("Part {}", idx + 1))
                } else {
                    None
                };

                let chunk = create_chunk(
                    &section.h1,
                    &section.h2,
                    &section.h3,
                    sub_section,
                    sub_content,
                    source_file,
                    &chunks,
                    idx + 1,        // chunk_index (1-based)
                    total_chunks,   // chunk_count
                );
                chunks.push(chunk);
            }
        }
    }

    Ok(chunks)
}

/// Split content on paragraph boundaries while respecting MAX_WORDS
fn split_on_paragraphs(lines: &[String]) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current_chunk = Vec::new();
    let mut current_words = 0;

    // Hard limit: if chunk exceeds this, force split at any line boundary
    const HARD_LIMIT: usize = MAX_WORDS + (MAX_WORDS / 2); // 1200 words

    for line in lines {
        let line_words = count_words(line);
        let is_blank = line.trim().is_empty();

        // Check if we should split:
        // 1. At blank line when exceeding MAX_WORDS
        // 2. At any line boundary if exceeding HARD_LIMIT
        let should_split = if current_words > HARD_LIMIT {
            // Hard limit exceeded - split at any line boundary
            !current_chunk.is_empty()
        } else if is_blank && current_words > MAX_WORDS {
            // At paragraph boundary and exceeding MAX_WORDS
            true
        } else {
            false
        };

        if should_split {
            // Save current chunk
            chunks.push(current_chunk.join("\n"));
            current_chunk.clear();
            current_words = 0;
        }

        // Add line to current chunk
        current_chunk.push(line.clone());
        current_words += line_words;
    }

    // Don't forget the last chunk
    if !current_chunk.is_empty() {
        chunks.push(current_chunk.join("\n"));
    }

    chunks
}

/// Create a single chunk with metadata
fn create_chunk(
    h1: &str,
    h2: &str,
    h3: &str,
    sub_section: Option<String>,
    content: &str,
    source_file: &str,
    existing_chunks: &[GuideChunk],
    chunk_index: usize,
    chunk_count: usize,
) -> GuideChunk {
    let word_count = count_words(content);
    let topics = topic_extractor::extract_topics(Some(h1), h2, h3, content);
    let chunk_id = generate_chunk_id(h1, h2, h3, existing_chunks);

    GuideChunk {
        heading_h1: h1.to_string(),
        heading_h2: h2.to_string(),
        heading_h3: h3.to_string(),
        sub_section,
        content: content.to_string(),
        word_count,
        source_file: source_file.to_string(),
        chunk_id,
        topics,
        chunk_index,
        chunk_count,
    }
}

/// Count words in text
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Generate unique chunk ID from headings
fn generate_chunk_id(h1: &str, h2: &str, h3: &str, existing_chunks: &[GuideChunk]) -> String {
    // Convert to kebab-case
    let h1_slug = slugify(h1);
    let h2_slug = slugify(h2);
    let h3_slug = slugify(h3);

    // Build base ID from all headings (h1-h2-h3)
    // If h2 or h3 is empty, omit it from the ID
    let mut parts = vec![h1_slug];
    if !h2.is_empty() {
        parts.push(h2_slug);
    }
    if !h3.is_empty() {
        parts.push(h3_slug);
    }
    let base_id = parts.join("-");

    // Count how many chunks already exist with this base
    let count = existing_chunks
        .iter()
        .filter(|c| c.chunk_id.starts_with(&base_id))
        .count();

    if count == 0 {
        base_id
    } else {
        format!("{}-{:03}", base_id, count + 1)
    }
}

/// Convert text to slug (kebab-case)
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("Hello world"), 2);
        assert_eq!(count_words("  Multiple   spaces  "), 2);
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Understanding vSphere Auto Deploy"), "understanding-vsphere-auto-deploy");
        assert_eq!(slugify("Multiple---Dashes"), "multiple-dashes");
        assert_eq!(slugify("Special!@#Characters"), "specialcharacters");
    }

    #[test]
    fn test_generate_chunk_id() {
        let chunks = vec![];
        let id1 = generate_chunk_id("ESX Installation", "Installing ESX", "Understanding Auto Deploy", &chunks);
        assert_eq!(id1, "esx-installation-installing-esx-understanding-auto-deploy");

        let chunk1 = GuideChunk {
            heading_h1: "ESX Installation".to_string(),
            heading_h2: "Installing ESX".to_string(),
            heading_h3: "Understanding Auto Deploy".to_string(),
            sub_section: None,
            content: "test".to_string(),
            word_count: 1,
            source_file: "test".to_string(),
            chunk_id: id1.clone(),
            topics: vec![],
            chunk_index: 1,
            chunk_count: 1,
        };

        let id2 = generate_chunk_id("ESX Installation", "Installing ESX", "Understanding Auto Deploy", &[chunk1]);
        assert_eq!(id2, "esx-installation-installing-esx-understanding-auto-deploy-002");
        
        // Test with empty H2
        let id3 = generate_chunk_id("ESX Installation", "", "Understanding Auto Deploy", &chunks);
        assert_eq!(id3, "esx-installation-understanding-auto-deploy");
        
        // Test with empty H2 and H3
        let id4 = generate_chunk_id("ESX Installation", "", "", &chunks);
        assert_eq!(id4, "esx-installation");
    }

    #[test]
    fn test_create_chunks_includes_h1() {
        use crate::markdown_parser::Section;

        let sections = vec![
            Section {
                h1: "Main Guide".to_string(),
                h2: "Section A".to_string(),
                h3: "Subsection 1".to_string(),
                content: vec!["Content here.".to_string()],
            },
            Section {
                h1: "Main Guide".to_string(),
                h2: "Section B".to_string(),
                h3: "".to_string(),
                content: vec!["Content under H2 only.".to_string()],
            },
        ];

        let chunks = create_chunks(&sections, "test").unwrap();

        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].heading_h1, "Main Guide");
        assert_eq!(chunks[0].heading_h2, "Section A");
        assert_eq!(chunks[0].heading_h3, "Subsection 1");
        
        assert_eq!(chunks[1].heading_h1, "Main Guide");
        assert_eq!(chunks[1].heading_h2, "Section B");
        assert_eq!(chunks[1].heading_h3, "");
    }

    #[test]
    fn test_split_on_paragraphs_small_content() {
        // Content that fits within MAX_WORDS should not be split
        let lines = vec![
            "This is a paragraph.".to_string(),
            "It has multiple lines.".to_string(),
            "".to_string(),
            "This is another paragraph.".to_string(),
        ];
        
        let chunks = split_on_paragraphs(&lines);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], lines.join("\n"));
    }

    #[test]
    fn test_split_on_paragraphs_at_blank_line() {
        // Create content that exceeds MAX_WORDS with a blank line
        let mut lines = Vec::new();
        
        // First paragraph: ~900 words (exceeds MAX_WORDS of 800)
        for _ in 0..90 {
            lines.push("word word word word word word word word word word".to_string());
        }
        lines.push("".to_string()); // Blank line - should trigger split
        
        // Second paragraph: small
        lines.push("Second paragraph here.".to_string());
        
        let chunks = split_on_paragraphs(&lines);
        assert_eq!(chunks.len(), 2, "Should split at blank line when exceeding MAX_WORDS");
    }

    #[test]
    fn test_split_on_paragraphs_hard_limit() {
        // Create content that exceeds HARD_LIMIT without blank lines
        let mut lines = Vec::new();
        
        // 1300 words without any blank lines (exceeds HARD_LIMIT of 1200)
        for _ in 0..130 {
            lines.push("word word word word word word word word word word".to_string());
        }
        
        let chunks = split_on_paragraphs(&lines);
        assert!(chunks.len() >= 2, "Should force split when exceeding HARD_LIMIT");
        
        // Verify that chunks respect the hard limit
        for chunk in &chunks {
            let word_count = count_words(chunk);
            assert!(word_count <= 1210, "Chunks should not exceed HARD_LIMIT by much");
        }
    }

    #[test]
    fn test_split_on_paragraphs_no_split_below_max() {
        // Content just under MAX_WORDS should not split even at blank lines
        let mut lines = Vec::new();
        
        // ~700 words (below MAX_WORDS of 800)
        for _ in 0..70 {
            lines.push("word word word word word word word word word word".to_string());
        }
        lines.push("".to_string()); // Blank line
        lines.push("More content here.".to_string());
        
        let chunks = split_on_paragraphs(&lines);
        assert_eq!(chunks.len(), 1, "Should not split when below MAX_WORDS");
    }

    #[test]
    fn test_split_on_paragraphs_empty_input() {
        let lines: Vec<String> = vec![];
        let chunks = split_on_paragraphs(&lines);
        assert_eq!(chunks.len(), 0, "Empty input should produce no chunks");
    }

    #[test]
    fn test_split_on_paragraphs_multiple_blank_lines() {
        // Test with multiple blank lines and content that needs splitting
        let mut lines = Vec::new();
        
        // First section: ~850 words
        for _ in 0..85 {
            lines.push("word word word word word word word word word word".to_string());
        }
        lines.push("".to_string());
        lines.push("".to_string()); // Multiple blank lines
        
        // Second section: ~850 words
        for _ in 0..85 {
            lines.push("word word word word word word word word word word".to_string());
        }
        lines.push("".to_string());
        
        lines.push("Final paragraph.".to_string());
        
        let chunks = split_on_paragraphs(&lines);
        assert!(chunks.len() >= 2, "Should split at blank lines when exceeding MAX_WORDS");
    }

    #[test]
    fn test_chunk_index_and_count() {
        use crate::markdown_parser::Section;

        // Test single chunk (no split)
        let sections = vec![
            Section {
                h1: "Guide".to_string(),
                h2: "Small Section".to_string(),
                h3: "".to_string(),
                content: vec!["This is a small section.".to_string()],
            },
        ];
        
        let chunks = create_chunks(&sections, "test").unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].chunk_index, 1, "Single chunk should have index 1");
        assert_eq!(chunks[0].chunk_count, 1, "Single chunk should have count 1");

        // Test split section (multiple chunks)
        let mut large_content = Vec::new();
        // Create content that will split into multiple chunks
        for i in 0..100 {
            large_content.push(format!("Line {} with some words here to increase word count.", i));
        }
        large_content.push("".to_string()); // Blank line
        for i in 100..200 {
            large_content.push(format!("Line {} with some more words here to increase word count.", i));
        }
        
        let sections = vec![
            Section {
                h1: "Guide".to_string(),
                h2: "Large Section".to_string(),
                h3: "".to_string(),
                content: large_content,
            },
        ];
        
        let chunks = create_chunks(&sections, "test").unwrap();
        assert!(chunks.len() > 1, "Large section should split into multiple chunks");
        
        let total = chunks.len();
        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.chunk_index, i + 1, "Chunk index should be 1-based");
            assert_eq!(chunk.chunk_count, total, "All chunks should have same total count");
        }
    }
}
