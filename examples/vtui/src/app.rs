use crate::event::{AppEvent, Event, EventHandler};
use crate::resource_table::ResourceTableWidget;
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Line, Stylize};
use ratatui::{DefaultTerminal, Frame};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use ratatui::widgets::{Row, TableState};
use vim_rs::core::client::Client;
use vim_rs::core::pc_cache::{CacheManager, Cacheable, ObjectCache, SharedRefCacheProxy};
use vim_rs::core::pc_helpers::BoxableError;
use vim_rs::types::structs::ManagedObjectReference;
use crate::datastore::DatastoreDetails;
use crate::host::Host;
use crate::indexed_cache::IndexedCache;
use crate::search::SearchState;
use crate::tabular_data::{TableDataSource, TabularData};
use crate::vm::VmData;
use crate::resource_type::{ResourceSelectionState, ResourceType};

pub struct App {
    should_quit: bool,
    cache_mgr: Rc<RefCell<CacheManager>>,
    client: Arc<Client>,
    resources: Box<dyn TableDataSource>,
    filter: ManagedObjectReference,
    events: EventHandler,
    search_state: SearchState,
    table_state: TableState,
    resource_selection_state: ResourceSelectionState,
}

const ASCII_ART: &str = r#"     ╭───────╮
 ╭─╮╭┴┬─╮ ╭──╯   ▐█▌
 \ \/ / │ │╔═╗╔═╗╭─╮
  \  /  │ │║ ╚╝ ║│ │
   ╰╯   ╰─╯╚════╝╰─╯"#;

const HELP_HINTS: &[&str] = &[
    "'q' - quit",
    "'/' - search",
    "'r' - select resource",
    "0..9 - sort by column",
    "↑/↓ - scroll up/down",
];

async fn init_data<T: TabularData + Cacheable + 'static>(
    cache_mgr: Rc<RefCell<CacheManager>>,
    container: &ManagedObjectReference,
) -> anyhow::Result<(Box<dyn TableDataSource>, ManagedObjectReference)>
where
    <T as TryFrom<vim_rs::types::structs::ObjectUpdate>>::Error: BoxableError,
    for<'a> Row<'static>: From<&'a T>
{
    let cache = Rc::new(RefCell::new(ObjectCache::<T>::new()));
    let filter = cache_mgr
        .borrow_mut()
        .add_container_cache(
            Box::new(SharedRefCacheProxy::new(cache.clone())),
            container
        )
        .await?;
    let indexed_cache= IndexedCache::new(cache.clone());
    Ok((Box::new(indexed_cache), filter))
}


impl App {
    pub async fn new(
        events: EventHandler,
        cache_mgr: Rc<RefCell<CacheManager>>,
        client: Arc<Client>,
    ) -> anyhow::Result<Self> {
        let root_folder = client.service_content().root_folder.clone();
        let (resources, filter) = init_data::<VmData>(cache_mgr.clone(), &root_folder).await?;
        Ok(Self {
            should_quit: false,
            cache_mgr,
            client,
            resources,
            filter,
            events,
            search_state: SearchState::new(),
            table_state: TableState::default(),
            resource_selection_state: ResourceSelectionState::new(),
        })
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
                self.resources.invalidate();
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
            AppEvent::ClearSearch => self.resources.set_filter(None),
            AppEvent::SearchCharacter(c) => self.search_state.input(c),
            AppEvent::SearchBackspace => self.search_state.delete(),
            AppEvent::SearchConfirm => {
                if let Some(filter) = self.search_state.deactivate() {
                    self.resources.set_filter(Some(filter));
                }
            }
            AppEvent::SearchCancel => {
                self.search_state.cancel();
            }
            AppEvent::SortColumn(column) => {
                self.resources.set_sort_column(Some(column));
            }
            AppEvent::ToggleResourceSelection => self.resource_selection_state.activate(),
            AppEvent::ResourceSelectionUp => self.resource_selection_state.move_up(),
            AppEvent::ResourceSelectionDown => self.resource_selection_state.move_down(),
            AppEvent::ResourceSelectionCancel => self.resource_selection_state.cancel(),
            AppEvent::ResourceSelectionConfirm => {
                if let Some(resource_type) = self.resource_selection_state.select() {
                    self.events.send(AppEvent::ResourceSelected(resource_type));
                }
            },
            AppEvent::ResourceSelected(resource_type) => {
                self.load_resource_type(resource_type).await?;
            },
        }
        Ok(())
    }

    async fn load_resource_type(&mut self, resource_type: ResourceType) -> anyhow::Result<()> {
        let root_folder = self.client.service_content().root_folder.clone();
        let (resources, filter) = match resource_type{
            ResourceType::VirtualMachine => {
                init_data::<VmData>(self.cache_mgr.clone(), &root_folder).await?
            }
            ResourceType::Host => {
                init_data::<Host>(self.cache_mgr.clone(), &root_folder).await?
            }
            ResourceType::Datastore => {
                init_data::<DatastoreDetails>(self.cache_mgr.clone(), &root_folder).await?
            }
        };
        self.cache_mgr.borrow_mut().remove_cache(&self.filter).await?;
        self.table_state = TableState::default();
        self.resources = resources;
        self.filter = filter;
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

        let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Length(21)]);
        let [left_area, right_area] = horizontal.areas(top_area);

        // Split the left area into two columns for statuses and help hints
        let left_columns = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [status_area, help_area] = left_columns.areas(left_area);

        // Render statuses
        let status_lines: Vec<Line> = self.build_status_lines();
        let status_paragraph = ratatui::widgets::Paragraph::new(status_lines)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Green));
        frame.render_widget(status_paragraph, status_area);

        // Render help hints
        let help_lines: Vec<Line> = HELP_HINTS.iter().map(|&h| Line::from(h).right_aligned()).collect();
        let help_paragraph = ratatui::widgets::Paragraph::new(help_lines)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
        frame.render_widget(help_paragraph, help_area);

        // Render ASCII art logo
        let logo_paragraph = ratatui::widgets::Paragraph::new(ASCII_ART)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
            .alignment(ratatui::layout::Alignment::Left);
        frame.render_widget(logo_paragraph, right_area);

        let table = ResourceTableWidget::new(self.resources.as_mut());

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
        if self.resource_selection_state.is_active() {
            let height = (self.resource_selection_state.options.len() as u16) + 2; // +2 for borders
            let popup_area = ratatui::layout::Rect {
                x: frame.area().width / 4,
                y: frame.area().height / 2 - height / 2,
                width: frame.area().width / 2,
                height,
            };

            let items: Vec<ratatui::widgets::ListItem> = self.resource_selection_state.options
                .iter()
                .map(|option| ratatui::widgets::ListItem::new(option.to_string()))
                .collect();

            let list = ratatui::widgets::List::new(items)
                .block(ratatui::widgets::Block::default()
                    .title("Select Resource Type")
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan)))
                .highlight_style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::REVERSED))
                .highlight_symbol("> ");

            let mut list_state = ratatui::widgets::ListState::default();
            list_state.select(Some(self.resource_selection_state.selected_index));

            frame.render_widget(ratatui::widgets::Clear, popup_area);
            frame.render_stateful_widget(list, popup_area, &mut list_state);
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

                } else if self.resource_selection_state.is_active() {
                    match key.code {
                        KeyCode::Enter => self.events.send(AppEvent::ResourceSelectionConfirm),
                        KeyCode::Esc => self.events.send(AppEvent::ResourceSelectionCancel),
                        KeyCode::Char('k') | KeyCode::Up => self.events.send(AppEvent::ResourceSelectionUp),
                        KeyCode::Char('j') | KeyCode::Down => self.events.send(AppEvent::ResourceSelectionDown),
                        _ => {}
                    }
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
                        KeyCode::Char('r') => self.events.send(AppEvent::ToggleResourceSelection),
                        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                            // Convert char to column index (0-based)
                            let column_idx = c.to_digit(10).unwrap() as usize - 1;
                            self.events.send(AppEvent::SortColumn(column_idx));
                        }
                        KeyCode::Char('0') => {
                            self.resources.set_sort_column(None);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
