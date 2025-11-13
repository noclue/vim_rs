use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Section {
    /// H1 heading (e.g., "Installing ESX Using vSphere Auto Deploy")
    pub h1: String,

    /// H2 heading (e.g., "Installing ESX Using vSphere Auto Deploy")
    pub h2: String,

    /// H3 heading (e.g., "Understanding vSphere Auto Deploy")
    pub h3: String,

    /// Content lines (everything between this H3 and the next H2/H3)
    pub content: Vec<String>,
}

/// Parse markdown to identify H2/H3 boundaries and extract sections
pub fn parse_markdown(content: &str) -> Result<Vec<Section>> {
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut sections = Vec::new();

    let mut current_h1 = String::new();
    let mut current_h2 = String::new();
    let mut current_h3 = String::new();
    let mut current_content = Vec::new();

    // H1 headings to skip entirely
    let skip_sections = vec![
        "Release Notes",
        "Documentation Legal Notice",
    ];

    let mut skip_until_next_h1 = true;

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("# ") {
            // Save previous section before starting new H1
            if !current_content.is_empty() && !skip_until_next_h1 {
                sections.push(Section {
                    h1: current_h1.clone(),
                    h2: current_h2.clone(),
                    h3: current_h3.clone(),
                    content: current_content.clone(),
                });
            }
            
            current_h1 = trimmed.strip_prefix("# ").unwrap().to_string();
            current_h2.clear();
            current_h3.clear();
            current_content.clear();

            // Check if this H1 should be skipped
            let should_skip = skip_sections.iter().any(|skip| current_h1.to_lowercase().contains(&skip.to_lowercase()));
            skip_until_next_h1 = should_skip;
            continue;
        }

        // Skip content until we find a non-skipped H1 heading
        if skip_until_next_h1 {
            continue;
        }

        // Check for H2
        if trimmed.starts_with("## ") {
            // Save previous section if exists (could be content under H2 without H3)
            if !current_content.is_empty() {
                sections.push(Section {
                    h1: current_h1.clone(),
                    h2: current_h2.clone(),
                    h3: current_h3.clone(),
                    content: current_content.clone(),
                });
            }

            // Start new H2
            current_h2 = trimmed.strip_prefix("## ").unwrap().to_string();
            current_h3.clear();
            current_content.clear();
        }
        // Check for H3
        else if trimmed.starts_with("### ") {
            // Save previous section if exists (could be content between H2 and H3)
            if !current_content.is_empty() {
                sections.push(Section {
                    h1: current_h1.clone(),
                    h2: current_h2.clone(),
                    h3: current_h3.clone(),
                    content: current_content.clone(),
                });
            }

            // Start new H3
            current_h3 = trimmed.strip_prefix("### ").unwrap().to_string();
            current_content.clear();

        }
        // Regular content line
        else {
            current_content.push(line);
        }
    }

    // Don't forget the last section
    if !current_content.is_empty() && !skip_until_next_h1 {
        sections.push(Section {
            h1: current_h1.clone(),
            h2: current_h2.clone(),
            h3: current_h3.clone(),
            content: current_content.clone(),
        });
    }

    Ok(sections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_structure() {
        let markdown = r#"# Main Guide Title
## Installing ESX
Some intro text.

### Understanding Auto Deploy
This is the understanding section.
It has multiple lines.

### Prerequisites
- Item 1
- Item 2

## Configuring vSphere

### Basic Configuration
Config details here.
"#;

        let sections = parse_markdown(markdown).unwrap();

        assert_eq!(sections.len(), 5);

        // First section: content between H2 and first H3
        assert_eq!(sections[0].h1, "Main Guide Title");
        assert_eq!(sections[0].h2, "Installing ESX");
        assert_eq!(sections[0].h3, "");
        assert!(sections[0].content.join("\n").contains("Some intro text"));

        // Second section: first H3
        assert_eq!(sections[1].h1, "Main Guide Title");
        assert_eq!(sections[1].h2, "Installing ESX");
        assert_eq!(sections[1].h3, "Understanding Auto Deploy");
        assert!(sections[1].content.join("\n").contains("understanding section"));

        // Third section: second H3
        assert_eq!(sections[2].h1, "Main Guide Title");
        assert_eq!(sections[2].h2, "Installing ESX");
        assert_eq!(sections[2].h3, "Prerequisites");

        // Fourth section: blank line between H2 and H3 (will be filtered out by chunker)
        assert_eq!(sections[3].h1, "Main Guide Title");
        assert_eq!(sections[3].h2, "Configuring vSphere");
        assert_eq!(sections[3].h3, "");

        // Fifth section: H3 under new H2
        assert_eq!(sections[4].h1, "Main Guide Title");
        assert_eq!(sections[4].h2, "Configuring vSphere");
        assert_eq!(sections[4].h3, "Basic Configuration");
    }

    #[test]
    fn test_preserve_content_between_h2_and_h3() {
        let markdown = r#"# Guide Title
## Section A
Content before H3.
More content here.

### Subsection A1
Content in subsection.
"#;

        let sections = parse_markdown(markdown).unwrap();

        assert_eq!(sections.len(), 2);
        
        // First section should have content between H2 and H3
        assert_eq!(sections[0].h1, "Guide Title");
        assert_eq!(sections[0].h2, "Section A");
        assert_eq!(sections[0].h3, "");
        assert!(sections[0].content.join("\n").contains("Content before H3"));
        
        // Second section should have H3 content
        assert_eq!(sections[1].h2, "Section A");
        assert_eq!(sections[1].h3, "Subsection A1");
        assert!(sections[1].content.join("\n").contains("Content in subsection"));
    }

    #[test]
    fn test_content_directly_under_h2() {
        let markdown = r#"# Guide Title
## Section Without H3
This content is directly under H2.
No H3 heading here.
"#;

        let sections = parse_markdown(markdown).unwrap();

        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].h1, "Guide Title");
        assert_eq!(sections[0].h2, "Section Without H3");
        assert_eq!(sections[0].h3, "");
        assert!(sections[0].content.join("\n").contains("directly under H2"));
    }

    #[test]
    fn test_skip_sections() {
        let markdown = r#"# Release Notes
Some release notes content.
This should be skipped.

# Actual Guide Title
## Real Content
This should be included.
"#;

        let sections = parse_markdown(markdown).unwrap();

        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].h1, "Actual Guide Title");
        assert_eq!(sections[0].h2, "Real Content");
        assert!(sections[0].content.join("\n").contains("should be included"));
        assert!(!sections[0].content.join("\n").contains("release notes"));
    }

    #[test]
    fn test_skip_related_links() {
        let markdown = r#"# Guide Title
## Some Topic

### Actual Content
This should be included.

### Related Links
https://example.com
This should be included (we don't skip H3 sections).

### More Content
This should be included again.
"#;

        let sections = parse_markdown(markdown).unwrap();

        // We preserve all sections, including blank lines and Related Links
        assert_eq!(sections.len(), 4);
        assert_eq!(sections[0].h1, "Guide Title");
        assert_eq!(sections[0].h2, "Some Topic");
        assert_eq!(sections[0].h3, "");
        assert_eq!(sections[1].h1, "Guide Title");
        assert_eq!(sections[1].h3, "Actual Content");
        assert_eq!(sections[2].h1, "Guide Title");
        assert_eq!(sections[2].h3, "Related Links");
        assert_eq!(sections[3].h1, "Guide Title");
        assert_eq!(sections[3].h3, "More Content");
    }
}
