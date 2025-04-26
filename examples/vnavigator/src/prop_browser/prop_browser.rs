use std::collections::VecDeque;
use std::mem;
use std::sync::Arc;
use indexmap::IndexMap;
use log::{debug, warn};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Scrollbar, ScrollbarOrientation, StatefulWidget};
use serde_json::Value;
use tui_tree_widget::{Tree, TreeItem, TreeState};
use vim_rs::core::client::Client;
use vim_rs::core::pc_cache::Cache;
use vim_rs::core::pc_helpers::Error;
use vim_rs::types::enums::{ObjectUpdateKindEnum, PropertyChangeOpEnum};
use vim_rs::types::structs::{ManagedObjectReference, ObjectUpdate, PropertyChange, PropertySpec};
use super::json_to_tree::get_type_name;
use super::prop_utils::{load_props, map_to_tree, props_to_map, to_json_value};

pub struct PropertyBrowserState {
    /// Client for interacting with the vSphere API.
    client: Arc<Client>,
    /// PropertyCollector filter for the current view
    obj: ManagedObjectReference,
    /// Properties of the current view.
    properties: IndexMap<String, Value>,
    /// Data source for the tree view.
    items: Vec<TreeItem<'static, String>>,
    /// Tree state for managing the current selection and scroll position.
    state: TreeState<String>,
    /// History of previous states for back navigation
    history: VecDeque<HistoryEntry>,
    /// Maximum history entries to keep
    max_history: usize,
}
struct HistoryEntry {
    obj: ManagedObjectReference,
    state: TreeState<String>,
}

impl PropertyBrowserState {
    pub async fn new(client: Arc<Client>, obj: ManagedObjectReference) -> anyhow::Result<Self> {
        let properties = load_props(client.clone(), &obj).await?;
        let properties = props_to_map(&properties)?;
        let items = map_to_tree(&properties);

        let state = Self::clean_state(&properties);

        Ok(Self {
            client,
            obj,
            properties,
            items,
            state,
            history: VecDeque::new(),
            max_history: 10,
        })
    }

    // Navigate back to previous state
    pub async fn back(&mut self) -> anyhow::Result<bool> {
        if let Some(entry) = self.history.pop_back() {
            self.obj = entry.obj;
            self.state = entry.state;
            self.refresh().await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn refresh(&mut self) -> anyhow::Result<bool> {
        // Reload properties for current object
        let properties = load_props(self.client.clone(), &self.obj).await?;
        let properties = props_to_map(&properties)?;
        let items = map_to_tree(&properties);

        // Update properties and items, but keep the state intact
        self.properties = properties;
        self.items = items;

        Ok(true)
    }

    fn clean_state(properties: &IndexMap<String, Value>) -> TreeState<String> {
        let mut state = TreeState::default();
        if let Some(first_key) = properties.keys().next() {
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

    /// Expands the currently selected node in the tree. Returns true if the
    /// node was expanded, false otherwise. Returns errors if loading properties
    /// fails e.g. communication failures or unexpected result.
    pub async fn enter(&mut self) -> anyhow::Result<bool> {
        let Some(Value::Object(props)) = self.get_selected() else {
            return Ok(false);
        };

        let Some(type_name) = get_type_name(&props) else {
            return Ok(false);
        };

        if type_name != "ManagedObjectReference" {
            return Ok(false);
        }

        let (Some(Value::String(motype)), Some(Value::String(value))) =
            (props.get("type"), props.get("value")) else {
            return Ok(false);
        };

        let Ok(motype) = serde_json::from_str(&format!("\"{}\"", motype)) else {
            return Ok(false);
        };

        let mo_ref = ManagedObjectReference {
            r#type: motype,
            value: value.clone(),
        };

        let properties = load_props(self.client.clone(), &mo_ref).await?;
        let properties = props_to_map(&properties)?;

        let old_state = mem::replace(&mut self.state, Self::clean_state(&properties));
        self.add_history(self.obj.clone(), old_state);
        self.items = map_to_tree(&properties);
        self.properties = properties;
        self.obj = mo_ref;

        Ok(true)
    }

    fn add_history(&mut self, selected_object: ManagedObjectReference, tree_state: TreeState<String>) {
        let entry = HistoryEntry {
            obj: selected_object,
            state: tree_state,
        };
        self.history.push_back(entry);
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    pub fn get_selected(&self) -> Option<Value> {
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
    pub fn current_object(&self) -> &ManagedObjectReference {
        &self.obj
    }

    fn apply_update(&mut self, changes: Vec<PropertyChange>) -> anyhow::Result<()> {
        for change in changes {
            let name = change.name;
            match change.op {
                PropertyChangeOpEnum::Assign => {
                    if let Some(value) = change.val {
                        self.properties.insert(name.clone(), to_json_value(&value, &name)?);
                    } else {
                        debug!("PropertyBrowserState: Assign operation with no value for property: {}", name);
                    }
                }
                PropertyChangeOpEnum::IndirectRemove => {
                    self.properties.shift_remove_entry(name.as_str());
                }
                _ => {
                    warn!("PropertyBrowserState: Unsupported property change operation: {:?}", change.op);
                    continue;
                }
            }
        }
        Ok(())
    }

    fn get_object_name(&self) -> Option<String> {
        if let Some(Value::String(name)) = self.properties.get("name") {
            Some(name.clone())
        } else {
            None
        }
    }
}

impl Cache for PropertyBrowserState {
    fn prop_spec(&self) -> vim_rs::core::pc_helpers::Result<PropertySpec> {
        let s: &'static str = From::from(&self.obj.r#type);
        Ok(PropertySpec{
            r#type: s.to_string(),
            all: Some(true),
            path_set: None,
        })
    }

    fn process_update(&mut self, update: Vec<ObjectUpdate>) -> vim_rs::core::pc_helpers::Result<()> {
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
                        self.apply_update(changes).map_err(|e| Error::InternalError(e.to_string()))?;
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
                warn!("PropertyBrowserState: update for different object: {}", update.obj.value);
                // Ignore updates for other objects
                continue;
            }
        };
        Ok(())
    }
}



pub struct PropertyBrowser<'a> {
    title: &'a str,
    highlight_style: Style,
    highlight_symbol: &'a str,
    with_scrollbar: bool,
}

impl<'a> PropertyBrowser<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            highlight_style: Style::new()
                .fg(Color::Black)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            highlight_symbol: "> ",
            with_scrollbar: true,
        }
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    pub fn highlight_symbol(mut self, symbol: &'a str) -> Self {
        self.highlight_symbol = symbol;
        self
    }

    pub fn with_scrollbar(mut self, enable: bool) -> Self {
        self.with_scrollbar = enable;
        self
    }
}

impl StatefulWidget for PropertyBrowser<'_> {
    type State = PropertyBrowserState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let object_type: &'static str = From::from(&state.obj.r#type);
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
            .block(Block::bordered().title(title).title_alignment(Alignment::Center))
            .highlight_style(self.highlight_style)
            .highlight_symbol(self.highlight_symbol);

        if self.with_scrollbar {
            widget = widget.experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(None)
                    .track_symbol(None)
                    .end_symbol(None)
            ));
        }

        widget.render(area, buf, &mut state.state);
    }
}

