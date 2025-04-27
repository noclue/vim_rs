use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
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
    ) -> anyhow::Result<Self> {
        let root_folder = client.service_content().root_folder.clone();
        let (resources, filter) = data_loaders::load_from_container::<VmData>(cache_mgr.clone(), &root_folder).await?;

        Ok(Self {
            cache_mgr,
            client,
            resources,
            filter,
            table_state: TableState::default(),
            parent: None,
        })
    }

    pub fn set_filter(&mut self, filter: Option<String>) {
        self.resources.set_filter(filter)
    }

    pub fn invalidate(&mut self) {
        self.resources.invalidate();
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
            KeyCode::Esc => self.set_filter(None),
            _ => {
                return Ok(false);
            }
        }
        Ok(true)
    }



    async fn expand_collection(&mut self, resource_type: ResourceType) -> anyhow::Result<()> {
        // Read the id of the currently selected resource
        let Some(selected) = self.table_state.selected() else {
            return Ok(());
        };
        let Some((selected_id, selected_name)) = self.resources.item_at_index(selected) else {
            return Ok(());
        };

        let (resources, filter) = match resource_type {
            ResourceType::VirtualMachine => {
                match selected_id.r#type {
                    MoTypesEnum::HostSystem | MoTypesEnum::Datastore |
                    MoTypesEnum::Network | MoTypesEnum::DistributedVirtualPortgroup |
                    MoTypesEnum::OpaqueNetwork => {
                        data_loaders::load_from_property::<VmData>(self.cache_mgr.clone(), &selected_id, "vm").await?
                    }
                    MoTypesEnum::ClusterComputeResource => {
                        data_loaders::load_from_container::<VmData>(self.cache_mgr.clone(), &selected_id).await?
                    }
                    _ => {
                        return Ok(()); // Do nothing we cannot expand VMs of a VM
                    }
                }
            }
            ResourceType::Host => {
                match selected_id.r#type {
                    MoTypesEnum::ClusterComputeResource |
                    MoTypesEnum::Network | MoTypesEnum::DistributedVirtualPortgroup |
                    MoTypesEnum::OpaqueNetwork => {
                        data_loaders::load_from_property::<Host>(self.cache_mgr.clone(), &selected_id, "host").await?
                    }
                    MoTypesEnum::Datastore => {
                        let hosts = get_datastore_hosts(self.client.clone(), &selected_id).await?;
                        data_loaders::load_from_list::<Host>(self.cache_mgr.clone(), &hosts).await?
                    }
                    _ => {
                        return Ok(()); // Do nothing we cannot expand Hosts of a Host or VM
                    }
                }
            }
            ResourceType::Datastore => {
                match selected_id.r#type {
                    MoTypesEnum::ClusterComputeResource | MoTypesEnum::HostSystem => {
                        data_loaders::load_from_property::<DatastoreDetails>(self.cache_mgr.clone(), &selected_id, "datastore").await?
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
                match selected_id.r#type {
                    MoTypesEnum::ClusterComputeResource | MoTypesEnum::HostSystem => {
                        data_loaders::load_from_property::<NetworkDetails>(self.cache_mgr.clone(), &selected_id, "network").await?
                    }
                    _ => {
                        return Ok(());
                    }
                }
            }
            ResourceType::Task => {
                ensure_task_descriptions_initialized(self.client.clone()).await?;
                match selected_id.r#type {
                    MoTypesEnum::ClusterComputeResource | MoTypesEnum::HostSystem |
                    MoTypesEnum::VirtualMachine | MoTypesEnum::Datastore |
                    MoTypesEnum::Network | MoTypesEnum::DistributedVirtualPortgroup |
                    MoTypesEnum::OpaqueNetwork => {
                        data_loaders::load_from_property::<TaskInfo>(self.cache_mgr.clone(), &selected_id, "recentTask").await?
                    }
                    _ => {
                        return Ok(());
                    }
                }
            }
        };
        self.apply_new_table_source(resources, filter).await?;
        self.parent = Some((selected_id, selected_name));
        Ok(())
    }

    pub async fn load_resource_type(&mut self, resource_type: ResourceType) -> anyhow::Result<()> {
        let parent = self.client.service_content().root_folder.clone();
        self.parent = None;

        let (resources, filter) = match resource_type {
            ResourceType::VirtualMachine => {
                data_loaders::load_from_container::<VmData>(self.cache_mgr.clone(), &parent).await?
            }
            ResourceType::Host => {
                data_loaders::load_from_container::<Host>(self.cache_mgr.clone(), &parent).await?
            }
            ResourceType::Datastore => {
                data_loaders::load_from_container::<DatastoreDetails>(self.cache_mgr.clone(), &parent).await?
            }
            ResourceType::Cluster => {
                data_loaders::load_from_container::<ClusterDetails>(self.cache_mgr.clone(), &parent).await?
            }
            ResourceType::Network => {
                data_loaders::load_from_container::<NetworkDetails>(self.cache_mgr.clone(), &parent).await?
            }
            ResourceType::Task => {
                let task_manager = self.client.service_content().task_manager.as_ref();
                let Some(task_manager) = task_manager else {
                    return Ok(());
                };
                // Initialize task descriptions
                ensure_task_descriptions_initialized(self.client.clone()).await?;
                data_loaders::load_from_property::<TaskInfo>(self.cache_mgr.clone(), task_manager, "recentTask").await?
            }
        };
        self.apply_new_table_source(resources, filter).await
    }

    async fn apply_new_table_source(&mut self, resources: Box<dyn TableDataSource>, filter: ManagedObjectReference) -> anyhow::Result<()> {
        self.cache_mgr.borrow_mut().remove_cache(&self.filter).await?;
        self.table_state = TableState::default();
        self.resources = resources;
        self.filter = filter;
        Ok(())
    }

}