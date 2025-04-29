use crate::event::{AppEvent, Event, EventHandler};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::{DefaultTerminal, Frame};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use ratatui::style::Stylize;
use ratatui::text::Line;
use vim_rs::core::client::Client;
use vim_rs::core::pc_cache::CacheManager;
use crate::body_pane::BodyPane;
use crate::hints;
use crate::hints::HELP_HINTS;
use crate::prop_browser::PropertyBrowserManager;
use crate::resource_browser::ResourceManager;
use crate::search::SearchState;
use crate::resource_type::{ResourceSelectionState, ResourceType};

/// Main application object.
pub struct App {
    /// Flag to indicate if the application should quit.
    should_quit: bool,
    /// Cache manager for managing object caches.
    cache_mgr: Rc<RefCell<CacheManager>>,
    /// Client for interacting with the vSphere API.
    client: Arc<Client>,
    /// Body pane.
    body_pane: BodyPane,
    /// Event dispatcher for processing events.
    events: EventHandler,
    /// Search state for managing the search input and filter.
    search_state: SearchState,
    /// State for managing resource selection.
    resource_selection_state: ResourceSelectionState,
}

const ASCII_ART: &str = r#"     ╭───────╮
 ╭─╮╭┴┬─╮ ╭──╯   ▐█▌
 \ \/ / │ │╔═╗╔═╗╭─╮
  \  /  │ │║ ╚╝ ║│ │
   ╰╯   ╰─╯╚════╝╰─╯"#;


impl App {
    pub async fn new(
        events: EventHandler,
        cache_mgr: Rc<RefCell<CacheManager>>,
        client: Arc<Client>,
    ) -> anyhow::Result<Self> {
        // Create a new ResourceManager instance
        let resource_mgr = ResourceManager::new(client.clone(),
                                                cache_mgr.clone(),
                                                ResourceType::VirtualMachine).await?;
        Ok(Self {
            should_quit: false,
            cache_mgr,
            client,
            body_pane: BodyPane::ResourceBrowser(resource_mgr),
            events,
            search_state: SearchState::new(),
            resource_selection_state: ResourceSelectionState::new(),
        })
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            match self.events.next().await? {
                Event::Crossterm(event) => self.handle_terminal_event(&event).await?,
                Event::App(app_event) => self.handle_app_event(app_event).await?,
            }
        }
        Ok(())
    }

    async fn handle_app_event(&mut self, event: AppEvent) -> anyhow::Result<()> {
        match event {
            AppEvent::PropertyCollector(update) => {
                self.cache_mgr.borrow_mut().apply_updates(update)?;
                if let BodyPane::ResourceBrowser(ref mut resource_mgr) = self.body_pane {
                    resource_mgr.invalidate();
                }
            }
            AppEvent::ErrorMessage(_msg) => {
                todo!()
            }
            AppEvent::Quit => {
                self.events.shutdown().await?;
                self.should_quit = true
            }
            AppEvent::OpenSearch => self.search_state.activate(),
            AppEvent::SearchCompleted(filter) => {
                if let BodyPane::ResourceBrowser(ref mut resource_mgr) = self.body_pane {
                    resource_mgr.set_filter(Some(filter));
                }
            }
            AppEvent::OpenResourceSelection => self.resource_selection_state.activate(),
            AppEvent::ResourceSelected(resource_type) => {
                match self.body_pane {
                    BodyPane::ResourceBrowser(ref mut resource_mgr) => {
                        resource_mgr.load_resource_type(resource_type).await?;
                    },
                    BodyPane::PropertyBrowser(_) => {
                        let res_mgr = ResourceManager::new(self.client.clone(), self.cache_mgr.clone(), resource_type).await?;
                        self.body_pane = BodyPane::ResourceBrowser(res_mgr);
                    }
                }
                if let BodyPane::ResourceBrowser(ref mut resource_mgr) = self.body_pane {
                    resource_mgr.load_resource_type(resource_type).await?;
                }
            },
            AppEvent::LoadProperties(moref) => {
                self.body_pane = BodyPane::PropertyBrowser(PropertyBrowserManager::new(self.cache_mgr.clone(), moref).await?)
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

        if let BodyPane::ResourceBrowser(ref resource_mgr) = self.body_pane {
            let res_type = resource_mgr.resource_type();
            self.render_header(frame, top_area, res_type);
        }

        self.body_pane.render(frame, body_area);
        
        // Draw search popup if active
        if self.search_state.is_active() {
            self.search_state.render(frame);
        }
        if self.resource_selection_state.is_active() {
            self.resource_selection_state.render(frame);
        }

    }

    fn render_header(&mut self, frame: &mut Frame, top_area: Rect, resource_type: ResourceType) {
        let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Length(16), Constraint::Length(16), Constraint::Length(21)]);
        let [status_area, expand_area, help_area, right_area] = horizontal.areas(top_area);

        // Split the left area into two columns for statuses and help hints

        // Render statuses
        let status_lines: Vec<Line> = self.build_status_lines();
        let status_paragraph = ratatui::widgets::Paragraph::new(status_lines)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Green));
        frame.render_widget(status_paragraph, status_area);

        // Render expand hints
        let expand_lines = hints::decorate_hints(hints::get_expand_hint(resource_type));
        let expand_paragraph = ratatui::widgets::Paragraph::new(expand_lines)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
        frame.render_widget(expand_paragraph, expand_area);

        // Render help hints
        let help_lines = hints::decorate_hints(HELP_HINTS);
        let help_paragraph = ratatui::widgets::Paragraph::new(help_lines)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
        frame.render_widget(help_paragraph, help_area);

        // Render ASCII art logo
        let logo_paragraph = ratatui::widgets::Paragraph::new(ASCII_ART)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
            .alignment(ratatui::layout::Alignment::Left);
        frame.render_widget(logo_paragraph, right_area);
    }

    async fn handle_terminal_event(&mut self, event: &CrosstermEvent) -> anyhow::Result<()>{
        if let CrosstermEvent::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                if matches!(key.code, KeyCode::Char('c') if key.modifiers == crossterm::event::KeyModifiers::CONTROL)  {
                    self.events.send(AppEvent::Quit);
                    return Ok(());
                }

                if self.search_state.is_active() && self.search_state.handle_key(key, &mut self.events) {
                    return Ok(());
                }

                if self.resource_selection_state.is_active() && self.resource_selection_state.handle_key(key, &mut self.events) {
                    return Ok(());
                }

                if self.body_pane.handle_key(key, &mut self.events).await? {
                    return Ok(());
                }

                match key.code {
                    KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                    KeyCode::Char('r') => self.events.send(AppEvent::OpenResourceSelection),
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
