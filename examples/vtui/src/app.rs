use crate::event::{AppEvent, Event, EventHandler};
use crate::resource_table::ResourceTableWidget;
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Line, Stylize};
use ratatui::{DefaultTerminal, Frame};
use std::cell::RefCell;
use std::rc::Rc;
use ratatui::widgets::TableState;
use vim_rs::core::pc_cache::CacheManager;
use crate::indexed_cache::IndexedCache;
use crate::search::SearchState;
use crate::tabular_data::TableDataSource;
use crate::vm::VmData;

pub struct App {
    should_quit: bool,
    cache_mgr: Rc<RefCell<CacheManager>>,
    vms: IndexedCache<VmData>,
    events: EventHandler,
    search_state: SearchState,
    table_state: TableState,
}

impl App {
    pub fn new(
        event_handler: EventHandler,
        cache_mgr: Rc<RefCell<CacheManager>>,
        vms: IndexedCache<VmData>,
    ) -> Self {
        Self {
            should_quit: false,
            cache_mgr,
            vms,
            events: event_handler,
            search_state: SearchState::new(),
            table_state: TableState::default(),
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            match self.events.next().await? {
                Event::Crossterm(event) => self.handle_terminal_event(&event),
                Event::App(app_event) => self.handle_app_event(app_event).await?,
            }
        }
        Ok(())
    }

    async fn handle_app_event(&mut self, event: AppEvent) -> anyhow::Result<()> {
        match event {
            AppEvent::PropertyCollector(update) => {
                self.cache_mgr.borrow_mut().apply_updates(update)?;
                self.vms.invalidate();
            }
            AppEvent::ErrorMessage(_msg) => {
                todo!()
            }
            AppEvent::Quit => {
                self.events.shutdown().await?;
                self.should_quit = true
            }
            AppEvent::Up => self.table_state.scroll_up_by(1),
            AppEvent::Down => self.table_state.scroll_down_by(1),
            AppEvent::ToggleSearch => self.search_state.activate(),
            AppEvent::ClearSearch => self.vms.set_filter(None),
            AppEvent::SearchCharacter(c) => self.search_state.input(c),
            AppEvent::SearchBackspace => self.search_state.delete(),
            AppEvent::SearchConfirm => {
                if let Some(filter) = self.search_state.deactivate() {
                    self.vms.set_filter(Some(filter));
                }
            }
            AppEvent::SearchCancel => {
                self.search_state.cancel();
            }
            AppEvent::SortColumn(column) => {
                self.vms.set_sort_column(Some(column));
            }
        }
        Ok(())
    }
    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title_area, body_area] = vertical.areas(frame.area());
        let title = Line::from("VIM-RS Ratatui example").centered().bold();
        frame.render_widget(title, title_area);

        let table = ResourceTableWidget::new(&mut self.vms);

        frame.render_stateful_widget(table, body_area, &mut self.table_state);

        // Draw search popup if active
        if self.search_state.is_active() {
            let popup_area = ratatui::layout::Rect {
                x: frame.area().width / 4,
                y: frame.area().height / 2 - 1,
                width: frame.area().width / 2,
                height: 3,
            };

            let block = ratatui::widgets::Block::default()
                .title("Search")
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));

            let input_text = ratatui::widgets::Paragraph::new(self.search_state.get_input())
                .block(block)
                .style(ratatui::style::Style::default());

            frame.render_widget(ratatui::widgets::Clear, popup_area);
            frame.render_widget(input_text, popup_area);
        }
    }

    fn handle_terminal_event(&mut self, event: &CrosstermEvent) {
        if let CrosstermEvent::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                if self.search_state.is_active() {
                    match key.code {
                        KeyCode::Enter => self.events.send(AppEvent::SearchConfirm),
                        KeyCode::Esc => self.events.send(AppEvent::SearchCancel),
                        KeyCode::Backspace => self.events.send(AppEvent::SearchBackspace),
                        KeyCode::Char(c) => self.events.send(AppEvent::SearchCharacter(c)),
                        _ => {}                    }

                } else {
                    match key.code {
                        KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                        KeyCode::Char('c') if key.modifiers == crossterm::event::KeyModifiers::CONTROL => {
                            self.events.send(AppEvent::Quit)
                        }
                        KeyCode::Esc => self.events.send(AppEvent::ClearSearch),
                        KeyCode::Char('j') | KeyCode::Down => self.events.send(AppEvent::Down),
                        KeyCode::Char('k') | KeyCode::Up => self.events.send(AppEvent::Up),
                        KeyCode::Char('/') => self.events.send(AppEvent::ToggleSearch),
                        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                            // Convert char to column index (0-based)
                            let column_idx = c.to_digit(10).unwrap() as usize - 1;
                            self.events.send(AppEvent::SortColumn(column_idx));
                        }
                        KeyCode::Char('0') => {
                            self.vms.set_sort_column(None);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
