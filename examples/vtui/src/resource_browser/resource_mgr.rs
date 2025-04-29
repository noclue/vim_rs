use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Arc;
use anyhow::anyhow;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::TableState;
use vim_rs::core::client::Client;
use vim_rs::core::pc_cache::CacheManager;
use vim_rs::types::structs::ManagedObjectReference;
use crate::resource_browser::data_loaders;
use crate::resource_browser::tabular_data::TableDataSource;
use crate::resource_browser::vm::VmData;
use crossterm::event::{KeyCode, KeyEvent};
use log::{debug, warn};
use vim_rs::types::enums::MoTypesEnum;
use crate::event::{AppEvent, EventHandler};
use crate::resource_browser::cluster::ClusterDetails;
use crate::resource_browser::datastore::{get_datastore_hosts, DatastoreDetails};
use crate::resource_browser::host::Host;
use crate::resource_browser::network::NetworkDetails;
use crate::resource_browser::resource_table::ResourceTableWidget;
use crate::resource_browser::task::{ensure_task_descriptions_initialized, TaskInfo};
use crate::resource_type::ResourceType;

pub struct ResourceManager {
    /// Cache manager for managing object caches.
    cache_mgr: Rc<RefCell<CacheManager>>,
    /// Client for interacting with the vSphere API.
    client: Arc<Client>,
    /// Data source for the table view.
    resources: Box<dyn TableDataSource>,
    /// PropertyCollector filter for the current view
    filter: ManagedObjectReference,
    /// Ratatui Table state for managing the current selection and scroll position.
    table_state: TableState,
    /// Parent object reference for the current view when expanding a sub collection e.g. VMs in host.
    parent: Option<(ManagedObjectReference, String)>,

    navigation_history: VecDeque<NavigationRecord>,

    max_history_size: usize,

    pending_table_state: Option<(usize, Option<usize>)>, // (offset, selected_index)
}

struct NavigationRecord {
    resource_type: ResourceType,
    parent: Option<(ManagedObjectReference, String)>,
    selected_index: Option<usize>,
    offset: usize,
    search_filter: Option<String>,
    sort: Option<(usize, bool)>,
}

impl NavigationRecord {
    fn from_current_state(resource_mgr: &ResourceManager) -> Self {
        Self {
            resource_type: resource_mgr.resource_type(),
            parent: resource_mgr.parent.clone(),
            selected_index: resource_mgr.table_state.selected(),
            offset: resource_mgr.table_state.offset(),
            search_filter: resource_mgr.resources.get_filter(),
            sort: resource_mgr.resources.get_sort_setting(),
        }
    }
}


impl ResourceManager {
    /// Creates a new ResourceManager instance i.e. table view. It automatically loads virtual
    /// machine table at the start.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the vSphere API client.
    /// * `cache_mgr` - A reference to the cache manager for managing object caches.
    pub async fn new(
        client: Arc<Client>,
        cache_mgr: Rc<RefCell<CacheManager>>,
        resource_type: ResourceType,
    ) -> anyhow::Result<Self> {
        let (resources, filter) = Self::load_from_container(resource_type, cache_mgr.clone(), &client).await?;

        Ok(Self {
            cache_mgr,
            client,
            resources,
            filter,
            table_state: TableState::default(),
            parent: None,
            navigation_history: VecDeque::new(),
            max_history_size: 10,
            pending_table_state: None,
        })
    }

    pub fn set_filter(&mut self, filter: Option<String>) {
        self.resources.set_filter(filter)
    }

    pub fn invalidate(&mut self) {
        self.resources.invalidate();
        // Apply any pending table state after data is loaded
        if let Some((offset, selected)) = self.pending_table_state.take() {
            self.table_state = TableState::default()
                .with_offset(offset)
                .with_selected(selected);
        }
    }

    pub fn resource_type(&self) -> ResourceType {
        self.resources.resource_type()
    }

    pub fn render(&mut self, frame: &mut Frame, body_area: Rect) {
        let table = ResourceTableWidget::new(self.resources.as_mut(), &self.parent);
        frame.render_stateful_widget(table, body_area, &mut self.table_state);

    }

    pub async fn handle_key(&mut self, key: &KeyEvent, events: &mut EventHandler) -> anyhow::Result<bool> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.table_state.scroll_down_by(1),
            KeyCode::Char('k') | KeyCode::Up => self.table_state.scroll_up_by(1),

            KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                // Convert char to column index (0-based)
                let column_idx = c.to_digit(10).unwrap() as usize - 1;
                self.resources.set_sort_column(Some(column_idx));
            }
            KeyCode::Char('0') => self.resources.set_sort_column(None),

            // Add shortcut keys to sub-collections - (n)etwork, (d)atastore, (h)ost, (v)m, (c)luster
            KeyCode::Char('n') => self.expand_collection(ResourceType::Network).await?,
            KeyCode::Char('d') => self.expand_collection(ResourceType::Datastore).await?,
            KeyCode::Char('h') => self.expand_collection(ResourceType::Host).await?,
            KeyCode::Char('v') => self.expand_collection(ResourceType::VirtualMachine).await?,
            //KeyCode::Char('c') => self.events.send(AppEvent::ExpandCollection(ResourceType::Cluster)),
            KeyCode::Char('t') => self.expand_collection(ResourceType::Task).await?,
            KeyCode::Char('/') => events.send(AppEvent::OpenSearch),
            KeyCode::Backspace => self.navigate_back().await?,
            KeyCode::Esc => self.set_filter(None),
            KeyCode::Enter => {
                if let Some((selected_id, _)) = self.selected_item() {
                    events.send(AppEvent::LoadProperties(selected_id))
                }
            },
            _ => {
                return Ok(false);
            }
        }
        Ok(true)
    }
    pub(crate) async fn load_resource_type(&mut self, resource_type: ResourceType) -> anyhow::Result<()> {
        // Save the current navigation state
        self.save_navigation_state();
        self.load_resource_type_int(resource_type).await
    }

    fn save_navigation_state(&mut self) {
        let record = NavigationRecord::from_current_state(self);
        if self.navigation_history.len() == self.max_history_size {
            self.navigation_history.pop_front();
        }
        self.navigation_history.push_back(record);
    }

    async fn expand_collection(&mut self, resource_type: ResourceType) -> anyhow::Result<()> {
        // Save the current navigation state
        self.save_navigation_state();
        // Read the id of the currently selected resource
        let Some((selected_id, selected_name)) = self.selected_item() else {
            return Ok(());
        };

        self.expand_parent_collection(resource_type, &selected_id, selected_name).await
    }

    fn selected_item(&mut self) -> Option<(ManagedObjectReference, String)> {
        let Some(selected) = self.table_state.selected() else {
            return None;
        };
        let Some((selected_id, selected_name)) = self.resources.item_at_index(selected) else {
            return None;
        };
        Some((selected_id, selected_name))
    }

    async fn expand_parent_collection(&mut self, resource_type: ResourceType, parent_id: &ManagedObjectReference, parent_name: String) -> anyhow::Result<()> {
        let (resources, filter) = match resource_type {
            ResourceType::VirtualMachine => {
                match parent_id.r#type {
                    MoTypesEnum::HostSystem | MoTypesEnum::Datastore |
                    MoTypesEnum::Network | MoTypesEnum::DistributedVirtualPortgroup |
                    MoTypesEnum::OpaqueNetwork => {
                        data_loaders::load_from_property::<VmData>(self.cache_mgr.clone(), &parent_id, "vm").await?
                    }
                    MoTypesEnum::ClusterComputeResource => {
                        data_loaders::load_from_container::<VmData>(self.cache_mgr.clone(), &parent_id).await?
                    }
                    _ => {
                        return Ok(()); // Do nothing we cannot expand VMs of a VM
                    }
                }
            }
            ResourceType::Host => {
                match parent_id.r#type {
                    MoTypesEnum::ClusterComputeResource |
                    MoTypesEnum::Network | MoTypesEnum::DistributedVirtualPortgroup |
                    MoTypesEnum::OpaqueNetwork => {
                        data_loaders::load_from_property::<Host>(self.cache_mgr.clone(), &parent_id, "host").await?
                    }
                    MoTypesEnum::Datastore => {
                        let hosts = get_datastore_hosts(self.client.clone(), &parent_id).await?;
                        data_loaders::load_from_list::<Host>(self.cache_mgr.clone(), &hosts).await?
                    }
                    _ => {
                        return Ok(()); // Do nothing we cannot expand Hosts of a Host or VM
                    }
                }
            }
            ResourceType::Datastore => {
                match parent_id.r#type {
                    MoTypesEnum::ClusterComputeResource | MoTypesEnum::HostSystem => {
                        data_loaders::load_from_property::<DatastoreDetails>(self.cache_mgr.clone(), &parent_id, "datastore").await?
                    }
                    _ => {
                        return Ok(());
                    }
                }
            }
            ResourceType::Cluster => {
                return Ok(()); // No sub-collection of clusters for now
            }
            ResourceType::Network => {
                match parent_id.r#type {
                    MoTypesEnum::ClusterComputeResource | MoTypesEnum::HostSystem => {
                        data_loaders::load_from_property::<NetworkDetails>(self.cache_mgr.clone(), &parent_id, "network").await?
                    }
                    _ => {
                        return Ok(());
                    }
                }
            }
            ResourceType::Task => {
                ensure_task_descriptions_initialized(self.client.clone()).await?;
                match parent_id.r#type {
                    MoTypesEnum::ClusterComputeResource | MoTypesEnum::HostSystem |
                    MoTypesEnum::VirtualMachine | MoTypesEnum::Datastore |
                    MoTypesEnum::Network | MoTypesEnum::DistributedVirtualPortgroup |
                    MoTypesEnum::OpaqueNetwork => {
                        data_loaders::load_from_property::<TaskInfo>(self.cache_mgr.clone(), &parent_id, "recentTask").await?
                    }
                    _ => {
                        return Ok(());
                    }
                }
            }
        };
        self.apply_new_table_source(resources, filter).await?;
        self.parent = Some((parent_id.clone(), parent_name));
        Ok(())
    }


    async fn load_resource_type_int(&mut self, resource_type: ResourceType) -> anyhow::Result<()> {
        self.parent = None;
        let cache_mgr = self.cache_mgr.clone();
        let client = &self.client;

        let (resources, filter) = Self::load_from_container(resource_type, cache_mgr, client).await?;
        self.apply_new_table_source(resources, filter).await
    }

    async fn load_from_container(resource_type: ResourceType, cache_mgr: Rc<RefCell<CacheManager>>, client: &Arc<Client>) -> anyhow::Result<(Box<dyn TableDataSource>, ManagedObjectReference)> {
        let parent = client.service_content().root_folder.clone();
        let (resources, filter) = match resource_type {
            ResourceType::VirtualMachine => {
                data_loaders::load_from_container::<VmData>(cache_mgr, &parent).await?
            }
            ResourceType::Host => {
                data_loaders::load_from_container::<Host>(cache_mgr, &parent).await?
            }
            ResourceType::Datastore => {
                data_loaders::load_from_container::<DatastoreDetails>(cache_mgr, &parent).await?
            }
            ResourceType::Cluster => {
                data_loaders::load_from_container::<ClusterDetails>(cache_mgr, &parent).await?
            }
            ResourceType::Network => {
                data_loaders::load_from_container::<NetworkDetails>(cache_mgr, &parent).await?
            }
            ResourceType::Task => {
                let task_manager = client.service_content().task_manager.as_ref();
                let Some(task_manager) = task_manager else {
                    return Err(anyhow!("Task manager not available"));
                };
                // Initialize task descriptions
                ensure_task_descriptions_initialized(client.clone()).await?;
                data_loaders::load_from_property::<TaskInfo>(cache_mgr, task_manager, "recentTask").await?
            }
        };
        Ok((resources, filter))
    }

    async fn apply_new_table_source(&mut self, resources: Box<dyn TableDataSource>, filter: ManagedObjectReference) -> anyhow::Result<()> {
        self.cache_mgr.borrow_mut().remove_cache(&self.filter).await?;
        self.table_state = TableState::default();
        self.resources = resources;
        self.filter = filter;
        Ok(())
    }

    async fn navigate_back(&mut self) -> anyhow::Result<()> {
        let Some(previous_state) = self.navigation_history.pop_back() else {
            return Ok(());
        };
        if let Some(parent) = previous_state.parent {
            self.expand_parent_collection(previous_state.resource_type, &parent.0, parent.1).await?;
        } else {
            self.parent = None;
            self.load_resource_type_int(previous_state.resource_type).await?;
        }

        // Store the table state to be applied after data is loaded
        self.pending_table_state = Some((previous_state.offset, previous_state.selected_index));

        self.resources.set_filter(previous_state.search_filter);
        if let Some((column, descending)) = previous_state.sort {
            self.resources.set_sort_setting(column, descending);
        } else {
            self.resources.set_sort_column(None);
        }
        self.resources.invalidate();

        Ok(())
    }
}

impl Drop for ResourceManager {
    fn drop(&mut self) {
        let cache_mgr = self.cache_mgr.clone();
        let filter = self.filter.clone();
        // Schedule task to remove the cache
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                debug!("Terminating ResourceManager. Releasing filter");
                cache_mgr.borrow_mut().remove_cache(&filter).await.unwrap_or_else(|e| {
                    warn!("Failed to remove ResourceManager filter: {:?}, {}", filter, e);
                });
            });
        });
    }
}