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
            );
            chunks.push(chunk);
        } else {
            // Split oversized section on paragraph boundaries
            let sub_chunks = split_on_paragraphs(&section.content);
            for (idx, sub_content) in sub_chunks.iter().enumerate() {
                // Skip empty sub-chunks
                let sub_word_count = count_words(sub_content);
                if sub_word_count == 0 {
                    continue;
                }

                let sub_section = if sub_chunks.len() > 1 {
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
    let mut in_list = false;
    let mut in_important_note = false;

    // Hard limit: if chunk exceeds this, force split even in lists
    const HARD_LIMIT: usize = MAX_WORDS + (MAX_WORDS / 2); // 1200 words

    for line in lines {
        let trimmed = line.trim();

        // Detect list start
        if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
            in_list = true;
        }

        // Detect Important/Note boxes
        if trimmed == "Important:" || trimmed == "Note:" {
            in_important_note = true;
        }

        // Detect end of Important/Note (blank line after content)
        if in_important_note && trimmed.is_empty() && !current_chunk.is_empty() {
            in_important_note = false;
        }

        // Detect paragraph boundary (blank line when not in list/important/note)
        let is_paragraph_boundary = trimmed.is_empty() && !in_list && !in_important_note;

        // Count words in this line
        let line_words = count_words(line);

        // Split if:
        // 1. At paragraph boundary and would exceed MAX_WORDS
        // 2. Current chunk exceeds HARD_LIMIT (force split even in lists)
        let should_split = if is_paragraph_boundary && current_words > 0 && current_words + line_words > MAX_WORDS {
            true
        } else if current_words > HARD_LIMIT {
            // Hard limit exceeded - force split at next safe point
            // Safe points: any line break (even in lists)
            true
        } else {
            false
        };

        if should_split {
            // Save current chunk
            chunks.push(current_chunk.join("\n"));
            current_chunk.clear();
            current_words = 0;
            in_list = false;
            in_important_note = false;

            // If this line is not blank, start new chunk with it
            if !trimmed.is_empty() {
                current_chunk.push(line.clone());
                current_words += line_words;
            }
            continue;
        }

        // Add line to current chunk
        current_chunk.push(line.clone());
        current_words += line_words;

        // If blank line and we were in a list, list has ended
        if trimmed.is_empty() && in_list {
            in_list = false;
        }
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
}
