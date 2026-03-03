use std::cmp::Ordering;
use ratatui::layout::Constraint;
use ratatui::text::Span;
use ratatui::widgets::{Cell, Row};
use vim_rs::vim_updatable;
use crate::resource_browser::formatting::{status_color, STATUS, STATUS_COLUMN_WIDTH};
use crate::resource_type::ResourceType;
use crate::resource_browser::tabular_data::TabularData;

vim_updatable!(
    struct NetworkDetails: Network {
        overall_status = "overall_status",
        name = "name",
        summary = "summary",
        vms = "vm.length",
        hosts = "host.length",
    }
);

impl From<&NetworkDetails> for Row<'_> {
    fn from(network: &NetworkDetails) -> Self {
        let status_color = status_color(&network.overall_status);
        let vms = if let Some(vms) = network.vms {
            Cell::from(vms.to_string())
        } else {
            Cell::default()
        };
        let hosts = if let Some(hosts) = network.hosts {
            Cell::from(hosts.to_string())
        } else {
            Cell::default()
        };
        let r#type = network.id.r#type.as_str().to_string();
        Row::new(vec![
            Cell::from(network.id.value.clone()),
            Cell::from(Span::from(STATUS).style(status_color)),
            Cell::from(network.name.clone()),
            Cell::from(r#type),
            vms,
            hosts,
        ])
    }
}

impl TabularData for NetworkDetails {
    fn get_title() -> &'static str {
        "Networks"
    }

    fn column_sizes() -> Vec<Constraint> {
        vec![
            Constraint::Length(24), // DPVG name is too long for standard width
            Constraint::Length(STATUS_COLUMN_WIDTH),
            Constraint::Fill(1),
            Constraint::Length(35),
            Constraint::Length(10),
            Constraint::Length(10),
        ]
    }

    fn header_row() -> Vec<&'static str> {
        vec!["ID", "S", "Name", "Type", "VMs", "Hosts"]
    }

    fn sortable_columns() -> Vec<usize> {
        vec![0, 2, 4, 5]
    }

    fn sort_by_column(column_idx: usize, descending: bool) -> Option<Box<dyn FnMut(&Self, &Self) -> Ordering>> {
        let mut f: Box<dyn FnMut(&Self, &Self) -> Ordering>  = match column_idx {
            0 => Box::new(|a, b| a.id.value.cmp(&b.id.value)),
            2 => Box::new(|a, b| a.name.cmp(&b.name)),
            4 => Box::new(|a, b| a.vms.cmp(&b.vms)),
            5 => Box::new(|a, b| a.hosts.cmp(&b.hosts)),
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
        let r#type = self.id.r#type.as_str().to_string();
        self.id.value.to_lowercase().contains(&filter)
            || self.name.to_lowercase().contains(&filter)
            || r#type.contains(&filter)
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn resource_type() -> ResourceType {
        ResourceType::Network
    }

}