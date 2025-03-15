use ratatui::{DefaultTerminal, Frame};
use std::time::Duration;
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyEventKind};
use futures_util::StreamExt;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Line, Stylize};
use crate::event::{AppEvent, Event, EventHandler};
use crate::vm_list::{LoadingState, VmListWidget};

pub struct App {
    should_quit: bool,
    vms: VmListWidget,
    events: EventHandler,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new(event_handler: EventHandler) -> Self {
        Self {
            should_quit: false,
            vms: VmListWidget::new(),
            events: event_handler,
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => self.handle_event(&event),
                Event::App(AppEvent::PropertyCollector(update)) => {
                    self.vms.apply_updates(update)
                },
                Event::App(AppEvent::StatusMessage(msg)) => {
                    self.vms.set_loading_state(LoadingState::Error(msg.clone()));
                }
                Event::App(AppEvent::Quit) => {
                    self.events.shutdown().await?;
                    self.should_quit = true
                },
                Event::App(AppEvent::Up) => self.vms.scroll_up(),
                Event::App(AppEvent::Down) => self.vms.scroll_down(),
            }
            tokio::select! {

                _ = interval.tick() => {  },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }
        Ok(())
    }

    fn tick(&mut self) {
    }

    fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title_area, body_area] = vertical.areas(frame.area());
        let title = Line::from("VIM-RS Ratatui example").centered().bold();
        frame.render_widget(title, title_area);
        frame.render_widget(&self.vms, body_area);
    }

    fn handle_event(&mut self, event: &CrosstermEvent) {
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