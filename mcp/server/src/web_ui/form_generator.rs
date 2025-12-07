//! Generate HTML form inputs from JSON Schema

#[cfg(feature = "web-ui")]
use serde_json::Value;
#[cfg(feature = "web-ui")]
use std::collections::HashSet;

/// Generate HTML form inputs from JSON Schema
#[cfg(feature = "web-ui")]
pub fn generate_form_html(schema: &Value, required_fields: &[String]) -> String {
    let required_set: HashSet<&str> = required_fields.iter().map(|s| s.as_str()).collect();
    
    let properties = match schema.get("properties") {
        Some(Value::Object(props)) => props,
        _ => return String::from("<p>No parameters required</p>"),
    };
    
    // Get definitions for resolving $ref
    let definitions = schema.get("definitions").and_then(|v| v.as_object());

    let mut html = String::new();
    
    for (field_name, field_schema) in properties {
        html.push_str(&generate_field_html(
            field_name,
            field_schema,
            required_set.contains(field_name.as_str()),
            0,
            definitions,
        ));
    }
    
    if html.is_empty() {
        html = String::from("<p>No parameters required</p>");
    }
    
    html
}

#[cfg(feature = "web-ui")]
fn generate_field_html(name: &str, schema: &Value, required: bool, depth: usize, definitions: Option<&serde_json::Map<String, Value>>) -> String {
    let mut resolved_schema = schema;
    
    // Check for $ref and resolve it if possible
    if let Some(ref_path) = schema.get("$ref").and_then(|v| v.as_str()) {
        if let Some(def_name) = ref_path.strip_prefix("#/definitions/") {
            if let Some(defs) = definitions {
                if let Some(def_schema) = defs.get(def_name) {
                    resolved_schema = def_schema;
                }
            }
        }
    }
    
    let field_type = resolved_schema
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("string");
    
    // Prefer description from original schema, fallback to resolved schema
    let description = schema
        .get("description")
        .or_else(|| resolved_schema.get("description"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    
    let required_marker = if required { " *" } else { "" };
    let required_attr = if required { " required" } else { "" };
    
    let indent = "  ".repeat(depth);
    
    match field_type {
        "string" => {
            // Check for enum values
            if let Some(enum_values) = resolved_schema.get("enum") {
                if let Some(values) = enum_values.as_array() {
                    return format!(
                        r#"{indent}<div class="form-group">
{indent}  <label for="{name}">{name}{required_marker}</label>
{indent}  <select id="{name}" name="{name}" class="form-control"{required_attr}>
{options}
{indent}  </select>
{indent}  {help}
{indent}</div>
"#,
                        indent = indent,
                        name = name,
                        required_marker = required_marker,
                        required_attr = required_attr,
                        options = values
                            .iter()
                            .filter_map(|v| v.as_str())
                            .map(|v| format!(r#"{indent}    <option value="{v}"{selected}>{v}</option>"#, 
                                indent = indent, 
                                v = v,
                                selected = if v == "all" { " selected" } else { "" }
                            ))
                            .collect::<Vec<_>>()
                            .join("\n"),
                        help = if !description.is_empty() {
                            format!(r#"<small class="form-text">{}</small>"#, html_escape(description))
                        } else {
                            String::new()
                        }
                    );
                }
            }
            
            // Regular string input
            format!(
                r#"{indent}<div class="form-group">
{indent}  <label for="{name}">{name}{required_marker}</label>
{indent}  <input type="text" id="{name}" name="{name}" class="form-control"{required_attr}>
{indent}  {help}
{indent}</div>
"#,
                indent = indent,
                name = name,
                required_marker = required_marker,
                required_attr = required_attr,
                help = if !description.is_empty() {
                    format!(r#"<small class="form-text">{}</small>"#, html_escape(description))
                } else {
                    String::new()
                }
            )
        }
        "number" | "integer" => format!(
            r#"{indent}<div class="form-group">
{indent}  <label for="{name}">{name}{required_marker}</label>
{indent}  <input type="number" id="{name}" name="{name}" class="form-control"{required_attr}>
{indent}  {help}
{indent}</div>
"#,
            indent = indent,
            name = name,
            required_marker = required_marker,
            required_attr = required_attr,
            help = if !description.is_empty() {
                format!(r#"<small class="form-text">{}</small>"#, html_escape(description))
            } else {
                String::new()
            }
        ),
        "boolean" => format!(
            r#"{indent}<div class="form-group form-check">
{indent}  <input type="checkbox" id="{name}" name="{name}" class="form-check-input" value="true">
{indent}  <label for="{name}" class="form-check-label">{name}{required_marker}</label>
{indent}  {help}
{indent}</div>
"#,
            indent = indent,
            name = name,
            required_marker = required_marker,
            help = if !description.is_empty() {
                format!(r#"<small class="form-text">{}</small>"#, html_escape(description))
            } else {
                String::new()
            }
        ),
        "array" => {
            format!(
                r#"{indent}<div class="form-group">
{indent}  <label for="{name}">{name}{required_marker}</label>
{indent}  <textarea id="{name}" name="{name}" class="form-control" rows="3" placeholder="Enter JSON array, e.g., [&quot;item1&quot;, &quot;item2&quot;]"{required_attr}></textarea>
{indent}  {help}
{indent}</div>
"#,
                indent = indent,
                name = name,
                required_marker = required_marker,
                required_attr = required_attr,
                help = if !description.is_empty() {
                    format!(r#"<small class="form-text">{}</small>"#, html_escape(description))
                } else {
                    String::from(r#"<small class="form-text">Enter as JSON array</small>"#)
                }
            )
        }
        "object" => {
            let mut inner_html = String::new();
            
            if let Some(Value::Object(props)) = resolved_schema.get("properties") {
                let inner_required = resolved_schema
                    .get("required")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(String::from)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                
                let inner_required_set: HashSet<&str> =
                    inner_required.iter().map(|s| s.as_str()).collect();
                
                for (field_name, field_schema) in props {
                    inner_html.push_str(&generate_field_html(
                        field_name,
                        field_schema,
                        inner_required_set.contains(field_name.as_str()),
                        depth + 1,
                        definitions,
                    ));
                }
            }
            
            format!(
                r#"{indent}<fieldset class="form-group">
{indent}  <legend>{name}{required_marker}</legend>
{indent}  {help}
{inner_html}{indent}</fieldset>
"#,
                indent = indent,
                name = name,
                required_marker = required_marker,
                help = if !description.is_empty() {
                    format!(r#"<small class="form-text">{}</small>"#, html_escape(description))
                } else {
                    String::new()
                },
                inner_html = inner_html
            )
        }
        _ => format!(
            r#"{indent}<div class="form-group">
{indent}  <label for="{name}">{name}{required_marker}</label>
{indent}  <input type="text" id="{name}" name="{name}" class="form-control"{required_attr}>
{indent}  {help}
{indent}</div>
"#,
            indent = indent,
            name = name,
            required_marker = required_marker,
            required_attr = required_attr,
            help = if !description.is_empty() {
                format!(r#"<small class="form-text">{}</small>"#, html_escape(description))
            } else {
                String::new()
            }
        ),
    }
}

#[cfg(feature = "web-ui")]
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(all(test, feature = "web-ui"))]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generate_string_field() {
        let schema = json!({
            "type": "string",
            "description": "A test field"
        });
        let html = generate_field_html("test_field", &schema, true, 0, None);
        assert!(html.contains("test_field *"));
        assert!(html.contains("required"));
        assert!(html.contains("A test field"));
    }

    #[test]
    fn test_generate_number_field() {
        let schema = json!({
            "type": "number",
            "description": "A number field"
        });
        let html = generate_field_html("num_field", &schema, false, 0, None);
        assert!(html.contains("num_field"));
        assert!(!html.contains("required"));
        assert!(html.contains("type=\"number\""));
    }
}

