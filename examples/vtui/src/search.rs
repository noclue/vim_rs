use ratatui::Frame;
use ratatui::layout::Rect;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use crate::event::{AppEvent, EventHandler};

/// Represents the state of the search popup
pub struct SearchState {
    active: bool,
    input: String,
    cursor_position: usize,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            active: false,
            input: String::new(),
            cursor_position: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let popup_area = Rect {
            x: frame.area().width / 4,
            y: frame.area().height / 2 - 1,
            width: frame.area().width / 2,
            height: 3,
        };

        let block = Block::default()
            .title("Search")
            .style(Style::default().bg(Color::Blue))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow));

        let input_text = Paragraph::new(self.get_input())
            .block(block)
            .style(Style::default());

        frame.render_widget(Clear, popup_area);
        frame.render_widget(input_text, popup_area);
    }

    pub fn handle_key(&mut self, key_event: &KeyEvent, events: &mut EventHandler) -> bool {
        match key_event.code {
            KeyCode::Esc => {
                self.cancel();
                true
            }
            KeyCode::Enter => {
                if let Some(filter) = self.deactivate() {
                    events.send(AppEvent::SearchCompleted(filter));
                }
                true
            }
            KeyCode::Backspace => {
                self.delete();
                true
            }
            KeyCode::Char(c) => {
                self.input(c);
                true
            }
            _ => {
                false
            }
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn deactivate(&mut self) -> Option<String> {
        self.active = false;
        if self.input.is_empty() {
            None
        } else {
            Some(self.input.clone())
        }
    }

    pub fn cancel(&mut self) {
        self.active = false;
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn input(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn delete(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }
}