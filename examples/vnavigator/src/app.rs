use crate::event::{AppEvent, Event, EventHandler};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::{DefaultTerminal, Frame};
use std::sync::Arc;
use ratatui::style::Stylize;
use ratatui::text::Line;
use vim_rs::core::client::Client;
use crate::prop_browser::{PropertyBrowser, PropertyBrowserState};

/// Main application object.
pub struct App {
    /// Flag to indicate if the application should quit.
    should_quit: bool,
    /// Event dispatcher for processing events.
    events: EventHandler,

    client: Arc<Client>,

    property_browser: PropertyBrowserState,
}

const ASCII_ART: &str = r#"     ╭───────╮
 ╭─╮╭┴┬─╮ ╭──╯   ▐█▌
 \ \/ / │ │╔═╗╔═╗╭─╮
  \  /  │ │║ ╚╝ ║│ │
   ╰╯   ╰─╯╚════╝╰─╯"#;


impl App {
    pub async fn new(
        events: EventHandler,
        client: Arc<Client>,
    ) -> anyhow::Result<Self> {
        let root_folder = &client.service_content().root_folder;
        let property_browser = PropertyBrowserState::new(client.clone(), root_folder.clone()).await?;

        Ok(Self {
            should_quit: false,
            events,
            client,
            property_browser,
        })
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            match self.events.next().await? {
                Event::Crossterm(event) => self.handle_terminal_event(&event).await,
                Event::App(app_event) => self.handle_app_event(app_event).await?,
            }
        }
        Ok(())
    }

    async fn handle_app_event(&mut self, event: AppEvent) -> anyhow::Result<()> {
        match event {
            AppEvent::Quit => {
                self.events.shutdown().await?;
                self.should_quit = true
            }
        }
        Ok(())
    }

    fn build_status_lines(&self) -> Vec<Line> {
        let mut res = Vec::<Line>::with_capacity(4);

        // Get about information from the service content
        let about = &self.client.service_content().about;

        // 1. vTUI version
        res.push(Line::from(vec![
            "vTUI Version: ".yellow(),
            env!("CARGO_PKG_VERSION").gray()
        ]));

        // 2. vSphere full product name
        res.push(Line::from(vec![
            "vSphere: ".yellow(),
            about.full_name.clone().gray()
        ]));

        // 3. vSphere system UUID
        if let Some(ref uuid) = about.instance_uuid {
            res.push(Line::from(vec![
                "vSphere UUID: ".yellow(),
                uuid.clone().gray()
            ]));
        } else {
            res.push(Line::from(vec![
                "vSphere UUID: ".yellow(),
                "N/A".gray()
            ]));
        }

        // 4. Used API version
        res.push(Line::from(vec![
            "API Version: ".yellow(),
            self.client.api_release().gray()
        ]));

        res


    }

    fn draw(&mut self, frame: &mut Frame) {

        let vertical = Layout::vertical([Constraint::Length(5), Constraint::Fill(1)]);
        let [top_area, body_area] = vertical.areas(frame.area());

        self.render_header(frame, top_area);


        let props = PropertyBrowser::new("Tree Widget");
        frame.render_stateful_widget(props, body_area, &mut self.property_browser);
    }

    fn render_header(&mut self, frame: &mut Frame, top_area: Rect) {
        let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Length(16), Constraint::Length(16), Constraint::Length(21)]);
        let [status_area, _expand_area, _help_area, right_area] = horizontal.areas(top_area);

        // Split the left area into two columns for statuses and help hints

        // Render statuses
        let status_lines: Vec<Line> = self.build_status_lines();
        let status_paragraph = ratatui::widgets::Paragraph::new(status_lines)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Green));
        frame.render_widget(status_paragraph, status_area);

        // Render expand hints
        // let expand_lines = hints::decorate_hints(hints::get_expand_hint(self.resources.resource_type()));
        // let expand_paragraph = ratatui::widgets::Paragraph::new(expand_lines)
        //     .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
        // frame.render_widget(expand_paragraph, expand_area);
        //
        // // Render help hints
        // let help_lines = hints::decorate_hints(HELP_HINTS);
        // let help_paragraph = ratatui::widgets::Paragraph::new(help_lines)
        //     .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
        // frame.render_widget(help_paragraph, help_area);

        // Render ASCII art logo
        let logo_paragraph = ratatui::widgets::Paragraph::new(ASCII_ART)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
            .alignment(ratatui::layout::Alignment::Left);
        frame.render_widget(logo_paragraph, right_area);
    }

    async fn handle_terminal_event(&mut self, event: &CrosstermEvent) {
        if let CrosstermEvent::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                if matches!(key.code, KeyCode::Char('c') if key.modifiers == crossterm::event::KeyModifiers::CONTROL) {
                    self.events.send(AppEvent::Quit)
                } else {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.events.send(AppEvent::Quit),
                        KeyCode::Char('w') | KeyCode::Up => { self.property_browser.up(); },
                        KeyCode::Char('s') | KeyCode::Down => { self.property_browser.down(); },
                        KeyCode::Char('a') | KeyCode::Left => { self.property_browser.left(); },
                        KeyCode::Char('d') | KeyCode::Right => { self.property_browser.right(); },
                        KeyCode::Enter => {
                            let _ = self.property_browser.enter().await;
                        }
                        KeyCode::Backspace => {
                            let _ = self.property_browser.back().await;
                        }
                        KeyCode::F(5) => {
                            let _ = self.property_browser.refresh().await;
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}
