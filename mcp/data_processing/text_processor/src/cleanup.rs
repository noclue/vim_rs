/// Remove page headers, footers, and URLs

use regex::Regex;

#[derive(Debug, PartialEq)]
enum State {
    Normal,
    SawFooter,
    SawBlankAfterFooter,
}

pub fn remove_page_markers(lines: &[String]) -> Vec<String> {
    let footer_pattern = Regex::new(r"^VMware by Broadcom\s+\d{1,4}$").unwrap();

    let mut state = State::Normal;
    let mut output = Vec::new();

    for line in lines {
        match state {
            State::Normal => {
                if footer_pattern.is_match(line) {
                    // Skip footer
                    state = State::SawFooter;
                } else {
                    output.push(line.clone());
                }
            }

            State::SawFooter => {
                if line.trim().is_empty() {
                    // Skip blank after footer
                    state = State::SawBlankAfterFooter;
                } else if is_url(line) {
                    // Skip URL, stay in SawFooter
                } else if is_header(line) {
                    // Skip header, return to normal
                    state = State::Normal;
                } else {
                    // Unexpected content, keep it
                    output.push(line.clone());
                    state = State::Normal;
                }
            }

            State::SawBlankAfterFooter => {
                if line.trim().is_empty() {
                    // Skip multiple blanks
                } else if is_url(line) {
                    // Skip URL
                } else if is_header(line) {
                    // Skip header, return to normal
                    state = State::Normal;
                } else {
                    // Content resumes
                    output.push(line.clone());
                    state = State::Normal;
                }
            }
        }
    }

    output
}

fn is_header(line: &str) -> bool {
    line == " VMware vSphere 9.0" || line == " VMware Cloud Foundation 9.0"
}

fn is_url(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("http://") || trimmed.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_page_markers() {
        let input = vec![
            "some content".to_string(),
            "".to_string(),
            "VMware by Broadcom  77".to_string(),
            "".to_string(),
            "https://techdocs.broadcom.com/...".to_string(),
            "".to_string(),
            " VMware vSphere 9.0".to_string(),
            "".to_string(),
            "next content".to_string(),
        ];

        let output = remove_page_markers(&input);

        assert_eq!(output, vec![
            "some content".to_string(),
            "".to_string(),
            "next content".to_string(),
        ]);
    }
}
