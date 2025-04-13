// Add at the top of app.rs
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    VirtualMachine,
    Host,
    Cluster,
    Datastore,
    Network,
    // Folder,
    // ResourcePool,
    Task,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::VirtualMachine => write!(f, "Virtual Machine"),
            ResourceType::Host => write!(f, "Host"),
            ResourceType::Cluster => write!(f, "Cluster"),
            ResourceType::Datastore => write!(f, "Datastore"),
            ResourceType::Network => write!(f, "Network"),
            // ResourceType::Folder => write!(f, "Folder"),
            // ResourceType::ResourcePool => write!(f, "Resource Pool"),
            ResourceType::Task => write!(f, "Task"),
        }
    }
}

pub struct ResourceSelectionState {
    active: bool,
    pub(crate) options: Vec<ResourceType>,
    pub(crate) selected_index: usize,
}

impl ResourceSelectionState {
    pub fn new() -> Self {
        Self {
            active: false,
            options: vec![
                ResourceType::VirtualMachine,
                ResourceType::Host,
                ResourceType::Cluster,
                ResourceType::Datastore,
                ResourceType::Network,
                // ResourceType::Folder,
                // ResourceType::ResourcePool,
                ResourceType::Task,
            ],
            selected_index: 0,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.selected_index = 0;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn cancel(&mut self) {
        self.active = false;
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.options.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select(&mut self) -> Option<ResourceType> {
        let selected = self.options.get(self.selected_index).cloned();
        self.active = false;
        selected
    }
}