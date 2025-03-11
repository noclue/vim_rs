use std::{
    env,
    sync::{Arc, RwLock},
    time::Duration,
};

use anyhow::{Result, Context};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, EventStream, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget},
    DefaultTerminal, Frame,
};
use vim_rs::{
    core::client::{Client, ClientBuilder},
};
use vim_rs::mo::{ContainerView, PropertyCollector, ViewManager};
use vim_rs::types::boxed_types::ValueElements;
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs;
use vim_rs::types::structs::DynamicProperty;
use vim_rs::types::vim_any::VimAny;
use futures_util::stream::StreamExt;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    let client = init_vim_client().await?;
    let terminal = ratatui::init();
    let app_result = App::new(client).run(terminal).await;
    ratatui::restore();
    app_result
}

async fn init_vim_client() -> Result<Arc<Client>> {
    let vc_server = env::var("VIM_SERVER").with_context(||"VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").with_context(||"VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").with_context(||"VIM_PASSWORD env var not set")?;

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(APP_NAME, APP_VERSION)
        .build().await?;
    Ok(client)
}

struct App {
    should_quit: bool,
    vms: VmListWidget,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new(client: Arc<Client>) -> Self {
        Self {
            should_quit: false,
            vms: VmListWidget::new(client),
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.vms.run();

        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| self.draw(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
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

    fn handle_event(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                    KeyCode::Char('j') | KeyCode::Down => self.vms.scroll_down(),
                    KeyCode::Char('k') | KeyCode::Up => self.vms.scroll_up(),
                    _ => {}
                }
            }
        }
    }
}

/// A widget that displays a list of virtual machines.
///
/// This is an async widget that fetches the list of VMs from the vCenter API. It contains
/// an inner `Arc<RwLock<VmListState>>` that holds the state of the widget. Cloning the
/// widget will clone the Arc, so you can pass it around to other threads, and this is used to spawn
/// a background task to fetch the VMs.
#[derive(Clone)]
struct VmListWidget {
    state: Arc<RwLock<VmListState>>,
    client: Arc<Client>,
}

#[derive(Debug, Default)]
struct VmListState {
    vms: Vec<VirtualMachine>,
    loading_state: LoadingState,
    table_state: TableState,
}

#[derive(Debug, Clone)]
struct VirtualMachine {
    id: String,
    name: String,
    os: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum LoadingState {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error(String),
}

impl VmListWidget {
    fn new(client: Arc<Client>) -> Self {
        Self {
            state: Arc::new(RwLock::new(VmListState::default())),
            client,
        }
    }
    /// Start fetching the VMs in the background.
    ///
    /// This method spawns a background task that fetches the VMs from the vCenter API.
    /// The result of the fetch is then converted to view state that is displayed.
    fn run(&self) {
        let this = self.clone(); // clone the widget to pass to the background task
        tokio::spawn(this.fetch_vms());
    }

    async fn fetch_vms(self) {
        self.set_loading_state(LoadingState::Loading);

        let content = self.client.service_content();
        let view_manager = ViewManager::new(self.client.clone(), content.view_manager.clone().unwrap().value.as_str());

        let Ok(view_moref) = view_manager.create_container_view(
            &content.root_folder,
            Some(&[Into::<&str>::into(MoTypesEnum::VirtualMachine).to_string()]),
            true,
        ).await else {
            self.set_loading_state(LoadingState::Error("Failed to create container view.".to_string()));
            return;
        };

        let view = ContainerView::new(self.client.clone(), &view_moref.value);

        let property_collector = PropertyCollector::new(self.client.clone(), &content.property_collector.value);

        let spec_set = vec![structs::PropertyFilterSpec {
            object_set: vec![structs::ObjectSpec {
                obj: view_moref.clone(),
                skip: Some(false),
                select_set: Some(vec![Box::new(structs::TraversalSpec {
                    name: Some("traverseEntities".to_string()),
                    r#type: Into::<&str>::into(MoTypesEnum::ContainerView).to_string(),
                    path: "view".to_string(),
                    skip: Some(false),
                    select_set: None,
                })]),
            }],
            prop_set: vec![structs::PropertySpec {
                all: Some(false),
                path_set: Some(vec!["name".to_string(), "summary.guest.guestFullName".into()]),
                r#type: Into::<&str>::into(MoTypesEnum::VirtualMachine).to_string(),
            }],
            report_missing_objects_in_results: Some(true),
        }];
        let options = structs::RetrieveOptions {
            max_objects: Some(100),
        };
        let Ok(Some(retrieve_result)) = property_collector.retrieve_properties_ex(&spec_set, &options).await else {
            self.set_loading_state(LoadingState::Error("No virtual machines found.".to_string()));
            return;
        };
        if let Some(token) = retrieve_result.token {
            property_collector.cancel_retrieve_properties_ex(&token).await.unwrap();
        }
        view.destroy_view().await.unwrap();

        self.set_loading_state(LoadingState::Loaded);
        let vms = retrieve_result.objects.iter().map(|obj| {
            let Some(propset) = &obj.prop_set else {
                return VirtualMachine {
                    id: obj.obj.value.clone(),
                    name: "<Unknown>".to_string(),
                    os: "<Unknown>".to_string(),
                };
            };
            VirtualMachine {
                id: obj.obj.value.clone(),
                name: get_str_property(propset, "name").unwrap_or_else(|| "<Unknown>".to_string()),
                os: get_str_property(propset, "summary.guest.guestFullName").unwrap_or_else(|| "<Unknown>".to_string()),
            }
        }).collect::<Vec<_>>();
        let mut state = self.state.write().unwrap();
        state.vms.extend(vms);
        if !state.vms.is_empty() {
            state.table_state.select(Some(0));
        }
    }


    fn set_loading_state(&self, state: LoadingState) {
        self.state.write().unwrap().loading_state = state;
    }

    fn scroll_down(&self) {
        self.state.write().unwrap().table_state.scroll_down_by(1);
    }

    fn scroll_up(&self) {
        self.state.write().unwrap().table_state.scroll_up_by(1);
    }
}

impl Widget for &VmListWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.state.write().unwrap();

        // a block with a right aligned title with the loading state on the right
        let loading_state = Line::from(format!("{:?}", state.loading_state)).right_aligned();
        let block = Block::bordered()
            .title("Virtual Machines")
            .title(loading_state)
            .title_bottom("j/k to scroll, q to quit");

        // a table with the list of pull requests
        let rows = state.vms.iter();
        let widths = [
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Max(49),
        ];
        let table = Table::new(rows, widths)
            .block(block)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">>")
            .row_highlight_style(Style::new().on_blue());

        StatefulWidget::render(table, area, buf, &mut state.table_state);
    }
}

fn get_str_property(row: &Vec<DynamicProperty>, property: &str) -> Option<String> {
    for prop in row {
        if prop.name == property {
            let VimAny::Value(ValueElements::PrimitiveString(ref val)) = prop.val else {
                return None;
            };
            return Some(val.clone());
        }
    }
    None
}

impl From<&VirtualMachine> for Row<'_> {
    fn from(pr: &VirtualMachine) -> Self {
        let pr = pr.clone();
        Row::new(vec![pr.id, pr.name, pr.os])
    }
}
