use serde_json::Value;
use ratatui::prelude::{Color, Line, Span, Style};
use tui_tree_widget::TreeItem;

// Styles for different elements in the tree
const KEYS: Style = Style::new().fg(Color::Gray);
const GROUP: Style = Style::new().fg(Color::White);
const STRING: Style = Style::new().fg(Color::LightGreen);
const NUMBER: Style = Style::new().fg(Color::LightBlue);
const BOOL: Style = Style::new().fg(Color::LightMagenta);
const MANAGED_OBJECT: Style = Style::new().fg(Color::LightCyan);
const NULL: Style = GROUP;



/// Convert a JSON property to a TreeItem
pub fn property_to_tree_item(
    key: String,
    value: &Value,
) -> TreeItem<'static, String> {
    let text = display_line(key.clone(), value);
    let children = value_children(value);
    if children.is_empty() {
        TreeItem::new_leaf(key, text)
    } else {
        TreeItem::new(key, text, children).expect("Failed to create tree item; check for duplicate keys/indices")
    }
}




fn display_line(key: String, value: &Value) -> Line<'static> {
    Line::from(vec![
        Span::styled(key, KEYS),
        Span::from(": "),
        value_to_span(value),
    ])
}

// Helper function to get a short string representation for values in the tree labels
fn value_to_span(value: &Value) -> Span<'static> {

    match value {
        Value::Object(map) => object_to_span(map),
        Value::Array(arr) => Span::styled(format!("[{}]", arr.len()),GROUP),
        Value::String(s) => Span::styled(format!("\"{}\"", s), STRING), // Add quotes for strings
        Value::Null => Span::styled("null",NULL),
        Value::Bool(b) => Span::styled(b.to_string(), BOOL),
        Value::Number(n) => Span::styled(n.to_string(),NUMBER),
    }
}

fn object_to_span(map: &serde_json::Map<String, Value>) -> Span<'static> {
    let Some(type_name) = get_type_name(map) else {
        return Span::styled("{...}", GROUP);
    };
    if type_name == "ManagedObjectReference" {
        if let (Some(Value::String(motype)), Some(Value::String(value))) = (map.get("type"), map.get("value")) {
            return Span::styled(format!("{}: {}", motype, value), MANAGED_OBJECT);
        }
    }
    Span::styled(format!("{{...}}: {}", type_name), GROUP)
}

fn value_children(value: &Value) -> Vec<TreeItem<'static, String>> {
    match value {
        Value::Object(map) => {
            let mut items = Vec::with_capacity(map.len());
            for (key, val) in map {
                if key == "_typeName" {
                    if let Value::String(s) = val {
                        if s == "ManagedObjectReference" {
                            return vec![]; // We do not want to expand ManagedObjectReferences
                        }
                    }
                    continue; // Skip the _typeName field
                }
                // Create spans with owned Strings to ensure 'static lifetime
                let text = display_line(key.clone(), val);

                // Recursively process child values
                let children = value_children(val);

                let item = if children.is_empty() {
                    // Leaf node if the value is primitive or an empty collection
                    TreeItem::new_leaf(key.clone(), text)
                } else {
                    // Inner node if the value is an object or array with children
                    TreeItem::new(key.clone(), text, children)
                        .expect("Failed to create tree item; check for duplicate keys/indices")
                };
                items.push(item);
            }
            items
        }
        Value::Array(arr) => {
            let mut items = Vec::with_capacity(arr.len());
            for (index, val) in arr.iter().enumerate() {
                let index_string = get_key_value(val).unwrap_or_else(|| index.to_string());
                // Create spans with owned Strings
                // let index_span = Span::styled(index_string.clone(), KEYS);
                // let text = Line::from(vec![index_span,Span::from(": "), value_to_span(val)]);
                let text = display_line(index_string.clone(), val);

                // Recursively process child values
                let children = value_children(val);

                let item = if children.is_empty() {
                    TreeItem::new_leaf(index.to_string(), text)
                } else {
                    TreeItem::new(index.to_string(), text, children)
                        .expect("Failed to create tree item; check for duplicate keys/indices")
                };
                items.push(item);
            }
            items
        }
        _ => vec![],
    }
}



pub fn get_type_name(map: &serde_json::Map<String, Value>) -> Option<String> {
    let Some(value) = map.get("_typeName") else {
        return None;
    };
    match value {
        Value::String(s) => Some(s.clone()),
        _ => None,
    }
}

fn get_key_value(val: &Value) -> Option<String> {
    match val {
        Value::Object(map) => {
            let Some(value) = map.get("key") else {
                return None;
            };
            match value {
                Value::String(s) => Some(s.clone()),
                Value::Number(n) => Some(n.to_string()),
                Value::Bool(b) => Some(b.to_string()),
                _ => None,
            }
        }
        _ => None,
    }
}