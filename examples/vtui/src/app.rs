use crate::event::{AppEvent, Event, EventHandler};
use crate::vm_list::{LoadingState, VmListWidget};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Line, Stylize};
use ratatui::{DefaultTerminal, Frame};
use std::cell::RefCell;
use std::rc::Rc;
use vim_rs::core::pc_helpers::CacheManager;

pub struct App {
    should_quit: bool,
    cache_mgr: Rc<RefCell<CacheManager>>,
    vms: VmListWidget,
    events: EventHandler,
}

impl App {
    pub fn new(
        event_handler: EventHandler,
        cache_mgr: Rc<RefCell<CacheManager>>,
        vms: VmListWidget,
    ) -> Self {
        Self {
            should_quit: false,
            cache_mgr,
            vms,
            events: event_handler,
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
                self.cache_mgr.borrow_mut().apply_updates(update)?
            }
            AppEvent::ErrorMessage(msg) => {
                self.vms.set_loading_state(LoadingState::Error(msg.clone()))
            }
            AppEvent::Quit => {
                self.events.shutdown().await?;
                self.should_quit = true
            }
            AppEvent::Up => self.vms.scroll_up(),
            AppEvent::Down => self.vms.scroll_down(),
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title_area, body_area] = vertical.areas(frame.area());
        let title = Line::from("VIM-RS Ratatui example").centered().bold();
        frame.render_widget(title, title_area);
        frame.render_widget(&self.vms, body_area);
    }

    fn handle_terminal_event(&mut self, event: &CrosstermEvent) {
        if let CrosstermEvent::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.events.send(AppEvent::Quit),
                    KeyCode::Char('j') | KeyCode::Down => self.events.send(AppEvent::Down),
                    KeyCode::Char('k') | KeyCode::Up => self.events.send(AppEvent::Up),
                    _ => {}
                }
            }
        }
    }
}
