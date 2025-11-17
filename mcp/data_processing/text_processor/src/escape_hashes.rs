/// Escape lines starting with # to prevent them from being interpreted as markdown headings
///
/// This is necessary because the source text file extracted from PDF may contain lines
/// starting with # that are not actual headings. We escape these before marking the real
/// headings from the TOC.

pub fn escape_existing_hashes(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .map(|line| {
            if line.starts_with('#') {
                // Escape the # by prefixing with backslash
                format!("\\{}", line)
            } else {
                line.clone()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_hashes() {
        let input = vec![
            "Normal line".to_string(),
            "#1 Some item".to_string(),
            "# Another hash line".to_string(),
            "  # Indented hash".to_string(),
            "No hash here".to_string(),
        ];

        let output = escape_existing_hashes(&input);

        assert_eq!(
            output,
            vec![
                "Normal line".to_string(),
                "\\#1 Some item".to_string(),
                "\\# Another hash line".to_string(),
                "  # Indented hash".to_string(),  // Not at start, not escaped
                "No hash here".to_string(),
            ]
        );
    }

    #[test]
    fn test_multiple_hashes() {
        let input = vec![
            "## Double hash".to_string(),
            "### Triple hash".to_string(),
        ];

        let output = escape_existing_hashes(&input);

        assert_eq!(
            output,
            vec![
                "\\## Double hash".to_string(),
                "\\### Triple hash".to_string(),
            ]
        );
    }
}

