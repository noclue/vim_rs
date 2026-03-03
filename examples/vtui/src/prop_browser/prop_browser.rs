use super::json_to_tree::{get_type_name, property_to_tree_item};
use super::prop_utils::to_json_value;
use std::fs::File;

use chrono::{Local, SecondsFormat};
use indexmap::IndexMap;
use log::{debug, warn};
use miniserde::json::{Object, Value};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, ScrollbarOrientation, StatefulWidget};
use std::io::Write;
use std::mem;
use std::path::PathBuf;
use tui_tree_widget::{Scrollbar, Tree, TreeItem, TreeState};
use vim_rs::core::pc_cache::Cache;
use vim_rs::core::pc_helpers::Error;
use vim_rs::types::enums::{ObjectUpdateKindEnum, PropertyChangeOpEnum};
use vim_rs::types::structs::{ManagedObjectReference, ObjectUpdate, PropertyChange, PropertySpec};

pub struct PropertyBrowserState {
    /// PropertyCollector filter for the current view
    obj: ManagedObjectReference,
    /// Properties of the current view.
    properties: IndexMap<String, Value>,
    /// Data source for the tree view.
    items: Vec<TreeItem<'static, String>>,
    /// Tree state for managing the current selection and scroll position.
    state: TreeState<String>,
}

impl PropertyBrowserState {
    pub async fn new(
        obj: ManagedObjectReference,
        tree_state: Option<TreeState<String>>,
    ) -> anyhow::Result<Self> {
        let res = Self {
            obj,
            properties: IndexMap::new(),
            items: Vec::new(),
            state: tree_state.unwrap_or_else(|| TreeState::default()),
        };
        Ok(res)
    }

    pub fn set_obj(
        &mut self,
        obj: ManagedObjectReference,
        new_tree_state: Option<TreeState<String>>,
    ) -> anyhow::Result<TreeState<String>> {
        self.obj = obj;
        self.items = Vec::new();
        self.properties = IndexMap::new();
        Ok(self.replace_tree_state(new_tree_state))
    }

    pub fn replace_tree_state(
        &mut self,
        new_tree_state: Option<TreeState<String>>,
    ) -> TreeState<String> {
        let tree_state = new_tree_state.unwrap_or_else(|| TreeState::default());
        mem::replace(&mut self.state, tree_state)
    }

    fn clean_state(&self) -> TreeState<String> {
        let mut state = TreeState::default();
        if let Some(first_key) = self.properties.keys().next() {
            state.select(vec![first_key.clone()]);
        }
        state
    }

    pub fn up(&mut self) {
        self.state.key_up();
    }

    pub fn down(&mut self) {
        self.state.key_down();
    }

    pub fn left(&mut self) {
        self.state.key_left();
    }

    pub fn right(&mut self) {
        self.state.key_right();
    }

    /// Get the ManagedObjectReference of the selected object in the tree if an object is selected.
    pub fn get_selected_object(&self) -> Option<ManagedObjectReference> {
        let Some(Value::Object(props)) = self.get_selected_node() else {
            return None;
        };

        let Some(type_name) = get_type_name(&props) else {
            return None;
        };

        if type_name != "ManagedObjectReference" {
            return None;
        }

        let (Some(Value::String(motype)), Some(Value::String(value))) =
            (props.get("type"), props.get("value"))
        else {
            return None;
        };

        let Ok(motype) = miniserde::json::from_str(&format!("\"{}\"", motype)) else {
            warn!(
                "PropertyBrowserState: Failed to parse type name: {}",
                motype
            );
            return None;
        };

        Some(ManagedObjectReference {
            r#type: motype,
            value: value.clone(),
        })
    }

    fn get_selected_node(&self) -> Option<Value> {
        let selected = self.state.selected();
        if selected.is_empty() {
            return None;
        }

        let properties = &self.properties;
        let first = selected.first()?;
        let Some(mut value) = properties.get(first) else {
            return None;
        };

        for item in selected.iter().skip(1) {
            match value {
                Value::Object(map) => {
                    value = map.get(item)?;
                }
                Value::Array(arr) => {
                    let index: usize = item.parse().ok()?;
                    if index < arr.len() {
                        value = &arr[index];
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }

        Some(value.clone())
    }

    fn apply_update(&mut self, changes: Vec<PropertyChange>) -> anyhow::Result<()> {
        let was_empty = self.items.is_empty();
        for change in changes {
            let name = change.name;
            match change.op {
                PropertyChangeOpEnum::Assign => {
                    if let Some(value) = change.val {
                        let json_value = to_json_value(&value, &name)?;
                        self.update_item(&name, &json_value);
                        self.properties.insert(name.clone(), json_value);
                    } else {
                        debug!(
                            "PropertyBrowserState: Assign operation with no value for property: {}",
                            name
                        );
                    }
                }
                PropertyChangeOpEnum::IndirectRemove => {
                    self.properties.shift_remove_entry(name.as_str());
                    self.remove_item(&name)?;
                }
                _ => {
                    warn!(
                        "PropertyBrowserState: Unsupported property change operation: {:?}",
                        change.op
                    );
                    continue;
                }
            }
        }
        //self.items = map_to_tree(&self.properties);

        if was_empty && !self.items.is_empty() && self.state.selected().is_empty() {
            self.state = self.clean_state();
        }
        Ok(())
    }

    fn remove_item(&mut self, name: &str) -> anyhow::Result<()> {
        if let Some(pos) = self.items.iter().position(|item| item.identifier() == name) {
            self.items.remove(pos);
        } else {
            warn!(
                "PropertyBrowserState::remove_item: Item not found in tree: {}",
                name
            );
        }
        Ok(())
    }

    fn update_item(&mut self, name: &String, value: &Value) {
        let tree_item = property_to_tree_item(name.clone(), value);

        let item_name = name.clone();

        // If item with item.identifier == name already exists, update it else push new item at the end
        if let Some(pos) = self
            .items
            .iter()
            .position(|item| item.identifier() == &item_name)
        {
            self.items[pos] = tree_item;
        } else {
            self.items.push(tree_item);
        }
    }

    fn get_object_name(&self) -> Option<String> {
        if let Some(Value::String(name)) = self.properties.get("name") {
            Some(name.clone())
        } else {
            None
        }
    }

    pub fn dump_to_json(&self) -> anyhow::Result<()> {
        // Convert IndexMap to miniserde Object for serialization
        let obj: Object = self
            .properties
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let json_str = miniserde::json::to_string(&obj);
        let json_content = pretty_print_json(&json_str);

        let filename = self.generate_json_filename()?;
        let path = PathBuf::from(&filename);

        let mut file = File::create(path)?;
        file.write_all(json_content.as_bytes())?;

        Ok(())
    }

    fn generate_json_filename(&self) -> anyhow::Result<String> {
        // Get object name if available
        let name_prefix = self
            .get_object_name()
            .map(|name| {
                // Replace characters that aren't suitable for filenames
                name.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_")
            })
            .unwrap_or_default();

        // Get object type and ID
        let object_type: &str = self.obj.r#type.as_str();
        let object_id = &self.obj.value;

        // Get current datetime in RFC 3339 format
        let timestamp = Local::now()
            .to_rfc3339_opts(SecondsFormat::Secs, true)
            .replace([':', '-'], ""); // Remove characters not suitable for filenames

        // Build filename
        let mut filename = String::new();

        // Add name prefix if available
        if !name_prefix.is_empty() {
            filename.push_str(&name_prefix);
            filename.push('_');
        }

        // Add object type and ID
        filename.push_str(object_type);
        filename.push('_');
        filename.push_str(object_id);
        filename.push('_');

        // Add timestamp and extension
        filename.push_str(&timestamp);
        filename.push_str(".json");

        Ok(filename)
    }
}

impl Cache for PropertyBrowserState {
    fn prop_spec(&self) -> vim_rs::core::pc_helpers::Result<PropertySpec> {
        let s = self.obj.r#type.as_str();
        Ok(PropertySpec {
            r#type: s.to_string(),
            all: Some(true),
            path_set: None,
        })
    }

    fn process_update(
        &mut self,
        update: Vec<ObjectUpdate>,
    ) -> vim_rs::core::pc_helpers::Result<()> {
        if update.is_empty() {
            return Ok(());
        };
        for update in update {
            if update.obj.value == self.obj.value {
                match update.kind {
                    ObjectUpdateKindEnum::Enter | ObjectUpdateKindEnum::Modify => {
                        let Some(changes) = update.change_set else {
                            continue;
                        };
                        debug!("object {:?} update", update.obj);
                        self.apply_update(changes)
                            .map_err(|e| Error::internal(e.to_string()))?;
                        continue;
                    }
                    ObjectUpdateKindEnum::Leave => {
                        debug!("object {:?} left", update.obj);
                        // Clear the state and items
                        self.state = TreeState::default();
                        self.items = Vec::new();
                        self.properties = IndexMap::new();
                        continue;
                    }
                    _ => {
                        // Ignore other update types
                        continue;
                    }
                }
            } else {
                warn!(
                    "PropertyBrowserState: update for different object: {}",
                    update.obj.value
                );
                // Ignore updates for other objects
                continue;
            }
        }
        Ok(())
    }
}

pub struct PropertyBrowser<'a> {
    highlight_style: Style,
    highlight_symbol: &'a str,
    with_scrollbar: bool,
}

impl<'a> PropertyBrowser<'a> {
    pub fn new() -> Self {
        Self {
            highlight_style: Style::new()
                .fg(Color::Black)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            highlight_symbol: "> ",
            with_scrollbar: true,
        }
    }
}

/// Pretty-print compact JSON with indentation (miniserde has no to_string_pretty).
fn pretty_print_json(json: &str) -> String {
    let mut out = String::new();
    let mut indent = 0usize;
    let mut in_string = false;
    let mut escape_next = false;
    let chars: Vec<char> = json.chars().collect();
    let len = chars.len();
    let mut i = 0;

    fn write_indent(s: &mut String, level: usize) {
        for _ in 0..level {
            s.push_str("  ");
        }
    }

    while i < len {
        let ch = chars[i];

        if escape_next {
            out.push(ch);
            escape_next = false;
            i += 1;
            continue;
        }

        if in_string {
            out.push(ch);
            if ch == '\\' {
                escape_next = true;
            } else if ch == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        match ch {
            '"' => {
                in_string = true;
                out.push('"');
            }
            '{' | '[' => {
                indent += 1;
                out.push(ch);
                let next_meaningful = chars[i + 1..].iter().position(|c| !c.is_ascii_whitespace());
                if let Some(pos) = next_meaningful {
                    let next_ch = chars[i + 1 + pos];
                    if next_ch != '}' && next_ch != ']' {
                        out.push('\n');
                        write_indent(&mut out, indent);
                    }
                }
            }
            '}' | ']' => {
                indent = indent.saturating_sub(1);
                out.push('\n');
                write_indent(&mut out, indent);
                out.push(ch);
            }
            ',' => {
                out.push_str(",\n");
                write_indent(&mut out, indent);
            }
            ':' => {
                out.push_str(": ");
            }
            c if c.is_ascii_whitespace() => {}
            _ => {
                out.push(ch);
            }
        }
        i += 1;
    }
    out
}

impl StatefulWidget for PropertyBrowser<'_> {
    type State = PropertyBrowserState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let object_type = state.obj.r#type.as_str();
        let object_id = &state.obj.value;

        let mut spans = Vec::new();

        // Add name if available
        if let Some(name) = state.get_object_name() {
            spans.push(Span::styled(name, Style::default().fg(Color::White)));
            spans.push(Span::raw(" "));
        }

        // Add ID in brackets
        spans.extend_from_slice(&[
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled(object_type, Style::default().fg(Color::Cyan)),
            Span::styled(": ", Style::default().fg(Color::DarkGray)),
            Span::styled(object_id, Style::default().fg(Color::Cyan)),
            Span::styled("]", Style::default().fg(Color::DarkGray)),
        ]);

        let title = Line::from(spans);

        let mut widget = Tree::new(&state.items)
            .expect("all item identifiers are unique")
            .block(
                Block::bordered()
                    .title(title)
                    .title_alignment(Alignment::Center)
                    .title_bottom(
                        Line::styled(
                            "→ - expand, ← - collapse, ↑↓ - scroll",
                            Style::default().fg(Color::Cyan),
                        )
                        .alignment(Alignment::Right),
                    ),
            )
            .highlight_style(self.highlight_style)
            .highlight_symbol(self.highlight_symbol);

        if self.with_scrollbar {
            widget = widget.experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(None)
                    .track_symbol(None)
                    .end_symbol(None),
            ));
        }

        widget.render(area, buf, &mut state.state);
    }
}
