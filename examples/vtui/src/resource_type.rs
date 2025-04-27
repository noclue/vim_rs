use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, ListState};
use crate::event::{AppEvent, EventHandler};

// Add at the top of app.rs
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    VirtualMachine,
    Host,
    Cluster,
    Datastore,
    Network,
    // Folder,
    // ResourcePool,
    Task,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::VirtualMachine => write!(f, "Virtual Machine"),
            ResourceType::Host => write!(f, "Host"),
            ResourceType::Cluster => write!(f, "Cluster"),
            ResourceType::Datastore => write!(f, "Datastore"),
            ResourceType::Network => write!(f, "Network"),
            // ResourceType::Folder => write!(f, "Folder"),
            // ResourceType::ResourcePool => write!(f, "Resource Pool"),
            ResourceType::Task => write!(f, "Task"),
        }
    }
}

pub struct ResourceSelectionState {
    active: bool,
    pub(crate) options: Vec<ResourceType>,
    pub(crate) selected_index: usize,
}

impl ResourceSelectionState {
    pub fn new() -> Self {
        Self {
            active: false,
            options: vec![
                ResourceType::VirtualMachine,
                ResourceType::Host,
                ResourceType::Cluster,
                ResourceType::Datastore,
                ResourceType::Network,
                // ResourceType::Folder,
                // ResourceType::ResourcePool,
                ResourceType::Task,
            ],
            selected_index: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let height = (self.options.len() as u16) + 2; // +2 for borders
        let popup_area = ratatui::layout::Rect {
            x: frame.area().width / 4,
            y: frame.area().height / 2 - height / 2,
            width: frame.area().width / 2,
            height,
        };

        let items: Vec<ratatui::widgets::ListItem> = self.options
            .iter()
            .map(|option| ratatui::widgets::ListItem::new(option.to_string()))
            .collect();

        let list = ratatui::widgets::List::new(items)
            .block(Block::default()
                .title("Select Resource Type")
                .style(Style::default().bg(Color::Blue))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow)))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> ");

        let mut list_state = ListState::default();
        list_state.select(Some(self.selected_index));

        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(list, popup_area, &mut list_state);
    }

    pub fn handle_key(&mut self, key_event: &KeyEvent, events: &mut EventHandler) -> bool {
        match key_event.code {
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            KeyCode::Esc => self.cancel(),
            KeyCode::Enter => {
                if let Some(selected) = self.select() {
                    events.send(AppEvent::ResourceSelected(selected));
                }
            }
            _ => {
                return false;
            }
        }
        true
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.selected_index = 0;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn cancel(&mut self) {
        self.active = false;
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.options.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select(&mut self) -> Option<ResourceType> {
        let selected = self.options.get(self.selected_index).cloned();
        self.active = false;
        selected
    }
}