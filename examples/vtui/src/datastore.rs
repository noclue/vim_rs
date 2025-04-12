use std::cmp::Ordering;
use std::sync::Arc;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Cell, Row};
use vim_macros::vim_updatable;
use vim_rs::core::client::Client;
use vim_rs::mo::Datastore;
use vim_rs::types::structs::ManagedObjectReference;
use crate::formatting;
use crate::formatting::{status_color, ID_COLUMN_WIDTH, STATUS, STATUS_COLUMN_WIDTH};
use crate::resource_type::ResourceType;
use crate::tabular_data::TabularData;
vim_updatable!(
    struct DatastoreDetails: Datastore {
        overall_status = "overall_status",
        accessible = "summary.accessible",
        name= "name",
        fs_type = "summary.r#type",
        //drive_type = "drive_type",
        shared = "summary.multiple_host_access",
        capacity = "summary.capacity",
        // provisioned =
        free_space = "summary.free_space",
        vms = "vm.length",
        hosts = "host.length",
    }
);

impl From<&DatastoreDetails> for Row<'_> {
    fn from(datastore: &DatastoreDetails) -> Self {
        let status_color = status_color(&datastore.overall_status);
        let accessible = if datastore.accessible {
            Cell::from(Cell::from(Span::styled("✓", Style::default().fg(Color::Green))))
        } else {
            Cell::from(Cell::from(Span::styled("✗", Style::default().fg(Color::Red))))
        };
        let capacity = formatting::format_byte_size(datastore.capacity);
        let free_space = formatting::format_byte_size(datastore.free_space);

        let shared = match datastore.shared {
            Some(true) => Cell::from(Span::styled("↔", Style::default().fg(Color::Blue))),
            _ => Cell::from(Span::styled("⭘", Style::default().fg(Color::Gray)))
        };
        let vms = if let Some(vms) = datastore.vms {
            Cell::from(vms.to_string())
        } else {
            Cell::default()
        };
        let hosts = if let Some(hosts) = datastore.hosts {
            Cell::from(hosts.to_string())
        } else {
            Cell::default()
        };
        Row::new(vec![
            Cell::from(datastore.id.value.clone()),
            Cell::from(Span::from(STATUS).style(status_color)),
            Cell::from(accessible),
            Cell::from(Span::from(datastore.name.clone())),
            Cell::from(datastore.fs_type.clone()),
            shared,
            capacity,
            free_space,
            vms,
            hosts,
        ])
    }
}

impl TabularData for DatastoreDetails {
    fn get_title() -> &'static str {
        "Datastores"
    }

    fn column_sizes() -> Vec<Constraint> {
        vec![
            Constraint::Length(ID_COLUMN_WIDTH),
            Constraint::Length(STATUS_COLUMN_WIDTH),
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Max(15),
            Constraint::Length(4),
            Constraint::Max(12),
            Constraint::Max(12),
            Constraint::Max(8),
            Constraint::Max(8),
        ]
    }

    fn header_row() -> Vec<&'static str> {
        vec![
            "ID",
            "S",
            "A",
            "Name",
            "FS Type",
            "Shr",
            "Capacity",
            "Free",
            "VMs",
            "Hosts",
        ]
    }

    fn sortable_columns() -> Vec<usize> {
        vec![0, 3, 4, 6, 7, 8, 9]
    }

    fn sort_by_column(column_idx: usize, descending: bool) -> Option<Box<dyn FnMut(&Self, &Self) -> Ordering>> {
        let mut f: Box<dyn FnMut(&Self, &Self) -> Ordering>  = match column_idx {
            0 => Box::new(|a, b| a.id.value.cmp(&b.id.value)),
            3 => Box::new(|a, b| a.name.cmp(&b.name)),
            4 => Box::new(|a, b| a.fs_type.cmp(&b.fs_type)),
            6 => Box::new(|a, b| a.capacity.cmp(&b.capacity)),
            7 => Box::new(|a, b| a.free_space.cmp(&b.free_space)),
            8 => Box::new(|a, b| a.vms.cmp(&b.vms)),
            9 => Box::new(|a, b| a.hosts.cmp(&b.hosts)),
            _ => return None,
        };
        if descending {
            Some(Box::new(move |a: &Self, b: &Self| f(b, a)))
        } else {
            Some(f)
        }
    }

    fn matches_filter(&self, filter: &str) -> bool {
        let filter = filter.to_lowercase();
        self.id.value.to_lowercase().contains(&filter)
            || self.name.to_lowercase().contains(&filter)
            || self.fs_type.to_lowercase().contains(&filter)
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn resource_type() -> ResourceType {
        ResourceType::Datastore
    }

}

pub async fn get_datastore_hosts(client: Arc<Client>, datastore: &ManagedObjectReference) -> anyhow::Result<Vec<ManagedObjectReference>> {
    let ds_stor = Datastore::new(client.clone(), &datastore.value.clone());
    let mount_infos = ds_stor.host().await?;
    let Some(mount_infos) = mount_infos else {
        return Ok(Vec::new());
    };

    Ok(mount_infos.iter().map(|info| { info.key.clone() }).collect())
}