use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Section {
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

    let mut current_h2 = String::new();
    let mut current_h3 = String::new();
    let mut current_content = Vec::new();

    // Sections to skip entirely
    let skip_sections = vec![
        "Related Links",
        "Table of Contents",
        "Legal Notice",
        "Copyright",
    ];

    let mut skip_until_next_heading = false;

    for line in lines {
        let trimmed = line.trim();

        // Check for H2
        if trimmed.starts_with("## ") {
            // Save previous section if exists
            if !current_h3.is_empty() && !current_content.is_empty() && !skip_until_next_heading {
                sections.push(Section {
                    h2: current_h2.clone(),
                    h3: current_h3.clone(),
                    content: current_content.clone(),
                });
            }

            // Start new H2
            current_h2 = trimmed.strip_prefix("## ").unwrap().to_string();
            current_h3.clear();
            current_content.clear();
            skip_until_next_heading = false;
        }
        // Check for H3
        else if trimmed.starts_with("### ") {
            // Save previous section if exists
            if !current_h3.is_empty() && !current_content.is_empty() && !skip_until_next_heading {
                sections.push(Section {
                    h2: current_h2.clone(),
                    h3: current_h3.clone(),
                    content: current_content.clone(),
                });
            }

            // Start new H3
            current_h3 = trimmed.strip_prefix("### ").unwrap().to_string();
            current_content.clear();

            // Check if this section should be skipped
            skip_until_next_heading = skip_sections.iter().any(|skip| current_h3.contains(skip));
        }
        // Regular content line
        else if !current_h3.is_empty() && !skip_until_next_heading {
            current_content.push(line);
        }
    }

    // Don't forget the last section
    if !current_h3.is_empty() && !current_content.is_empty() && !skip_until_next_heading {
        sections.push(Section {
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
        let markdown = r#"## Installing ESX
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

        assert_eq!(sections.len(), 3);

        assert_eq!(sections[0].h2, "Installing ESX");
        assert_eq!(sections[0].h3, "Understanding Auto Deploy");
        assert!(sections[0].content.join("\n").contains("understanding section"));

        assert_eq!(sections[1].h2, "Installing ESX");
        assert_eq!(sections[1].h3, "Prerequisites");

        assert_eq!(sections[2].h2, "Configuring vSphere");
        assert_eq!(sections[2].h3, "Basic Configuration");
    }

    #[test]
    fn test_skip_related_links() {
        let markdown = r#"## Some Topic

### Actual Content
This should be included.

### Related Links
https://example.com
This should be skipped.

### More Content
This should be included again.
"#;

        let sections = parse_markdown(markdown).unwrap();

        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].h3, "Actual Content");
        assert_eq!(sections[1].h3, "More Content");
    }
}
