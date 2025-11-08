/// Fix bullet points - convert to markdown

pub fn fix_bullets(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .map(|line| {
            let trimmed_start = line.trim_start();

            if trimmed_start.starts_with("• ") {
                // Level 1 bullet: replace at start of line only
                let indent = line.len() - trimmed_start.len();
                format!(
                    "{}* {}",
                    " ".repeat(indent),
                    trimmed_start.strip_prefix("• ").unwrap()
                )
            } else if trimmed_start.starts_with("– ") {
                // Level 2 bullet: replace and add 4-space indent
                let indent = line.len() - trimmed_start.len();
                format!(
                    "{}    * {}",
                    " ".repeat(indent),
                    trimmed_start.strip_prefix("– ").unwrap()
                )
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
    fn test_level1_bullet() {
        let input = vec!["• Item one".to_string()];
        let output = fix_bullets(&input);
        assert_eq!(output, vec!["* Item one".to_string()]);
    }

    #[test]
    fn test_level2_bullet() {
        let input = vec!["– Sub item".to_string()];
        let output = fix_bullets(&input);
        assert_eq!(output, vec!["    * Sub item".to_string()]);
    }

    #[test]
    fn test_emdash_in_text() {
        let input = vec!["* local – The first local disk".to_string()];
        let output = fix_bullets(&input);
        // Should not change em-dash inside text
        assert_eq!(output, vec!["* local – The first local disk".to_string()]);
    }
}
