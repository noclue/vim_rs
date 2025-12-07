//! Unit tests for web UI components

#[cfg(all(test, feature = "web-ui"))]
use super::*;

// Tests will be added after form_generator and markdown modules are ported
#[cfg(all(test, feature = "web-ui"))]
mod form_generator_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_string_field() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "User name"
                }
            }
        });
        let required = vec!["name".to_string()];
        let html = form_generator::generate_form_html(&schema, &required);
        
        assert!(html.contains("<input"));
        assert!(html.contains("type=\"text\""));
        assert!(html.contains("name=\"name\""));
        assert!(html.contains("required"));
    }

    #[test]
    fn test_number_field() {
        let schema = json!({
            "type": "object",
            "properties": {
                "age": {
                    "type": "number",
                    "description": "User age"
                }
            }
        });
        let required = vec![];
        let html = form_generator::generate_form_html(&schema, &required);
        
        assert!(html.contains("type=\"number\""));
        assert!(html.contains("name=\"age\""));
        assert!(!html.contains("required"));
    }

    #[test]
    fn test_boolean_field() {
        let schema = json!({
            "type": "object",
            "properties": {
                "enabled": {
                    "type": "boolean",
                    "description": "Is enabled"
                }
            }
        });
        let required = vec![];
        let html = form_generator::generate_form_html(&schema, &required);
        
        assert!(html.contains("type=\"checkbox\""));
        assert!(html.contains("name=\"enabled\""));
    }

    #[test]
    fn test_enum_field() {
        let schema = json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "enum": ["active", "inactive", "pending"]
                }
            }
        });
        let required = vec![];
        let html = form_generator::generate_form_html(&schema, &required);
        
        assert!(html.contains("<select"));
        assert!(html.contains("name=\"status\""));
        assert!(html.contains("active"));
        assert!(html.contains("inactive"));
        assert!(html.contains("pending"));
    }
}

#[cfg(all(test, feature = "web-ui"))]
mod markdown_tests {
    use super::*;

    #[test]
    fn test_heading_conversion() {
        let markdown = "# Heading 1\n## Heading 2";
        let html = markdown::markdown_to_html(markdown);
        
        assert!(html.contains("<h1>"));
        assert!(html.contains("Heading 1"));
        assert!(html.contains("<h2>"));
        assert!(html.contains("Heading 2"));
    }

    #[test]
    fn test_code_block_conversion() {
        let markdown = "```rust\nfn main() {}\n```";
        let html = markdown::markdown_to_html(markdown);
        
        assert!(html.contains("<pre>"));
        assert!(html.contains("<code"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_list_conversion() {
        let markdown = "- Item 1\n- Item 2\n- Item 3";
        let html = markdown::markdown_to_html(markdown);
        
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>"));
        assert!(html.contains("Item 1"));
    }

    #[test]
    fn test_link_conversion() {
        let markdown = "[Example](https://example.com)";
        let html = markdown::markdown_to_html(markdown);
        
        assert!(html.contains("<a href=\"https://example.com\""));
        assert!(html.contains("Example"));
    }

    #[test]
    fn test_inline_code_conversion() {
        let markdown = "Use `code` here";
        let html = markdown::markdown_to_html(markdown);
        
        assert!(html.contains("<code>"));
        assert!(html.contains("code"));
    }
}

