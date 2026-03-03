use miniserde::json::{Number, Object, Value};
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

fn number_to_string(n: &Number) -> String {
    match n {
        Number::U64(x) => x.to_string(),
        Number::I64(x) => x.to_string(),
        Number::F64(x) => x.to_string(),
    }
}

/// Convert a JSON property to a TreeItem
pub fn property_to_tree_item(key: String, value: &Value) -> TreeItem<'static, String> {
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

fn value_to_span(value: &Value) -> Span<'static> {
    match value {
        Value::Object(map) => object_to_span(map),
        Value::Array(arr) => Span::styled(format!("[{}]", arr.len()), GROUP),
        Value::String(s) => Span::styled(format!("\"{}\"", s), STRING),
        Value::Null => Span::styled("null", NULL),
        Value::Bool(b) => Span::styled(b.to_string(), BOOL),
        Value::Number(n) => Span::styled(number_to_string(n), NUMBER),
    }
}

fn object_to_span(map: &Object) -> Span<'static> {
    let Some(type_name) = get_type_name(map) else {
        return Span::styled("{...}", GROUP);
    };
    if type_name == "ManagedObjectReference" {
        if let (Some(Value::String(motype)), Some(Value::String(value))) =
            (map.get("type"), map.get("value"))
        {
            return Span::styled(format!("{}: {}", motype, value), MANAGED_OBJECT);
        }
    }
    Span::styled(format!("{{...}}: {}", type_name), GROUP)
}

fn value_children(value: &Value) -> Vec<TreeItem<'static, String>> {
    match value {
        Value::Object(map) => {
            let mut items = Vec::with_capacity(map.len());
            for (key, val) in map.iter() {
                if key == "_typeName" {
                    if let Value::String(s) = val {
                        if s.as_str() == "ManagedObjectReference" {
                            return vec![];
                        }
                    }
                    continue;
                }
                let text = display_line(key.clone(), val);
                let children = value_children(val);
                let item = if children.is_empty() {
                    TreeItem::new_leaf(key.clone(), text)
                } else {
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
                let text = display_line(index_string.clone(), val);
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

pub fn get_type_name(map: &Object) -> Option<String> {
    let value = map.get("_typeName")?;
    match value {
        Value::String(s) => Some(s.clone()),
        _ => None,
    }
}

fn get_key_value(val: &Value) -> Option<String> {
    match val {
        Value::Object(map) => {
            let value = map.get("key")?;
            match value {
                Value::String(s) => Some(s.clone()),
                Value::Number(n) => Some(number_to_string(n)),
                Value::Bool(b) => Some(b.to_string()),
                _ => None,
            }
        }
        _ => None,
    }
}
