/// Merge broken list items across lines

pub fn merge_lists(lines: &[String]) -> Vec<String> {
    let mut output = Vec::new();
    let mut current_item = String::new();
    let mut in_list = false;

    for line in lines {
        let trimmed_start = line.trim_start();

        if trimmed_start.starts_with("* ") {
            // New list item
            if !current_item.is_empty() {
                output.push(current_item.clone());
            }
            current_item = line.clone();
            in_list = true;
        } else if in_list && !line.trim().is_empty() && !trimmed_start.starts_with('#') {
            // Continuation line (not blank, not heading)
            current_item.push(' ');
            current_item.push_str(line.trim());
        } else {
            // Not in list or blank line
            if !current_item.is_empty() {
                output.push(current_item.clone());
                current_item.clear();
            }
            in_list = false;
            output.push(line.clone());
        }
    }

    // Don't forget last item
    if !current_item.is_empty() {
        output.push(current_item);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_broken_list() {
        let input = vec![
            "* Item one".to_string(),
            "continuation".to_string(),
            "* Item two".to_string(),
        ];

        let output = merge_lists(&input);

        assert_eq!(
            output,
            vec![
                "* Item one continuation".to_string(),
                "* Item two".to_string(),
            ]
        );
    }

    #[test]
    fn test_nested_list() {
        let input = vec![
            "* Item one".to_string(),
            "    * Sub item".to_string(),
            "continuation of sub".to_string(),
            "* Item two".to_string(),
        ];

        let output = merge_lists(&input);

        assert_eq!(
            output,
            vec![
                "* Item one".to_string(),
                "    * Sub item continuation of sub".to_string(),
                "* Item two".to_string(),
            ]
        );
    }
}
