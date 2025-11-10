use std::collections::HashSet;

/// Extract topics/keywords from headings and content
pub fn extract_topics(h2: &str, h3: &str, content: &str) -> Vec<String> {
    let mut topics = HashSet::new();

    // Extract from H2 and H3 headings
    topics.extend(extract_from_heading(h2));
    topics.extend(extract_from_heading(h3));

    // Extract from Important/Note boxes (high-value keywords)
    topics.extend(extract_from_important_notes(content));

    // Convert to sorted vec
    let mut topics_vec: Vec<String> = topics.into_iter().collect();
    topics_vec.sort();

    topics_vec
}

/// Extract meaningful words from heading
fn extract_from_heading(heading: &str) -> Vec<String> {
    // Common words to skip
    let stopwords = [
        "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for",
        "of", "with", "by", "from", "up", "about", "into", "through", "during",
        "including", "using", "how", "what", "when", "where", "which", "who",
        "why", "can", "could", "should", "would", "may", "might", "must",
    ];

    heading
        .split_whitespace()
        .filter(|word| {
            let lower = word.to_lowercase();
            // Keep if: (1) not a stopword, (2) length > 2, (3) alphanumeric
            !stopwords.contains(&lower.as_str())
                && lower.len() > 2
                && lower.chars().any(|c| c.is_alphanumeric())
        })
        .map(|word| {
            // Clean and normalize
            word.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
        })
        .filter(|word| !word.is_empty())
        .collect()
}

/// Extract keywords from Important/Note boxes
fn extract_from_important_notes(content: &str) -> Vec<String> {
    let mut keywords = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut in_important_note = false;
    let mut note_content = String::new();

    for line in lines {
        let trimmed = line.trim();

        if trimmed == "Important:" || trimmed == "Note:" {
            in_important_note = true;
            note_content.clear();
            continue;
        }

        if in_important_note {
            if trimmed.is_empty() && !note_content.is_empty() {
                // End of Important/Note block
                keywords.extend(extract_key_phrases(&note_content));
                in_important_note = false;
                note_content.clear();
            } else {
                note_content.push_str(trimmed);
                note_content.push(' ');
            }
        }
    }

    // Don't forget last note if file ends
    if !note_content.is_empty() {
        keywords.extend(extract_key_phrases(&note_content));
    }

    keywords
}

/// Extract key phrases from Important/Note text
fn extract_key_phrases(text: &str) -> Vec<String> {
    let mut phrases = Vec::new();

    // Look for capitalized terms (likely product names, features)
    let words: Vec<&str> = text.split_whitespace().collect();

    for word in &words {
        let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric());

        // Keep if: starts with capital, length > 2, contains letter
        if cleaned.len() > 2
            && cleaned.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
            && cleaned.chars().any(|c| c.is_alphabetic())
        {
            phrases.push(cleaned.to_lowercase());
        }
    }

    // Look for technical terms (e.g., "vSphere", "Auto Deploy", "DPU")
    // These often have specific patterns
    for window in words.windows(2) {
        let combined = format!("{} {}", window[0], window[1]);
        let cleaned = combined.trim_matches(|c: char| !c.is_alphanumeric() && c != ' ');

        // Multi-word capitalized terms
        if cleaned.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
            && cleaned.len() > 4
        {
            phrases.push(cleaned.to_lowercase());
        }
    }

    phrases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_from_heading() {
        let topics = extract_from_heading("Understanding vSphere Auto Deploy");
        assert!(topics.contains(&"understanding".to_string()));
        assert!(topics.contains(&"vsphere".to_string()));
        assert!(topics.contains(&"auto".to_string()));
        assert!(topics.contains(&"deploy".to_string()));
    }

    #[test]
    fn test_stopwords_filtered() {
        let topics = extract_from_heading("How to Use the Configuration Manager");
        assert!(!topics.contains(&"how".to_string()));
        assert!(!topics.contains(&"to".to_string()));
        assert!(!topics.contains(&"the".to_string()));
        assert!(topics.contains(&"use".to_string()));
        assert!(topics.contains(&"configuration".to_string()));
        assert!(topics.contains(&"manager".to_string()));
    }

    #[test]
    fn test_extract_from_important_notes() {
        let content = r#"Some regular content.

Important:

The vSphere Host Profiles capability is deprecated in ESX 9.0 and will be removed.

More content here.

Note:  You cannot use Auto Deploy on ESX hosts configured with DPUs.

Even more content.
"#;

        let topics = extract_from_important_notes(content);
        assert!(topics.len() > 0);
        // Should extract capitalized terms like "vSphere", "Host", "Profiles", "ESX", "Auto", "Deploy", "DPUs"
    }

    #[test]
    fn test_extract_topics_combined() {
        let h2 = "Installing ESX Using vSphere Auto Deploy";
        let h3 = "Understanding Auto Deploy";
        let content = r#"Auto Deploy provisions ESX hosts with an image profile.

Important:

The vSphere Host Profiles capability is deprecated in ESX 9.0.
"#;

        let topics = extract_topics(h2, h3, content);

        // Should have topics from headings
        assert!(topics.contains(&"installing".to_string()));
        assert!(topics.contains(&"vsphere".to_string()));
        assert!(topics.contains(&"auto".to_string()));
        assert!(topics.contains(&"deploy".to_string()));

        // Should have extracted some terms from Important box
        assert!(topics.len() > 5);
    }
}
