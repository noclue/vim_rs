use std::cmp::Ordering;
use ratatui::layout::Constraint;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Cell, Row};
use vim_macros::vim_updatable;
use vim_rs::types::enums::HostSystemConnectionStateEnum;
use crate::formatting::{status_color, ID_COLUMN_WIDTH, STATUS, STATUS_COLUMN_WIDTH};
use crate::resource_type::ResourceType;
use crate::tabular_data::TabularData;
vim_updatable!(
    struct Host: HostSystem {
        overall_status = "summary.overall_status",
        connection_state = "runtime.connection_state",
        name = "name",
        version = "config.product.version",
        cpu_usage = "summary.quick_stats.overall_cpu_usage",
        memory_usage = "summary.quick_stats.overall_memory_usage",
        uptime = "summary.quick_stats.uptime",
        vms = "vm.length",
        networks = "network.length",
        datastores = "datastore.length",
    }
);


impl From<&Host> for Row<'_> {
    fn from(host: &Host) -> Self {
        let color = status_color(&host.overall_status);
        let version = if let Some(version) = host.version.as_ref() {
            Cell::from(format!("{version}"))
        } else {
            Cell::default()
        };
        let host_cpu = if let Some(host_cpu) = host.cpu_usage {
            Cell::from(format!("{:.2} MHz", host_cpu as f32))
        } else {
            Cell::default()
        };
        let memory_usage = if let Some(memory_usage) = host.memory_usage {
            if memory_usage > 1024 {
                Cell::from(format!("{:.2} GiB", memory_usage as f32 / 1024.0))
            } else {
                Cell::from(format!("{:.2} MiB", memory_usage as f32))
            }
        } else {
            Cell::default()
        };
        // connected, not_responding, disconnected
        let (symbol, conn_color) = match host.connection_state {
            HostSystemConnectionStateEnum::Connected => ("✓", ratatui::style::Color::Green),
            HostSystemConnectionStateEnum::NotResponding => ("!", ratatui::style::Color::Yellow),
            HostSystemConnectionStateEnum::Disconnected => ("✗", ratatui::style::Color::Red),
            _ => ("?", ratatui::style::Color::Gray),
        };

        let vms = if let Some(vms) = host.vms {
            Cell::from(vms.to_string())
        } else {
            Cell::default()
        };
        let networks = if let Some(networks) = host.networks {
            Cell::from(networks.to_string())
        } else {
            Cell::default()
        };
        let datastores = if let Some(datastores) = host.datastores {
            Cell::from(datastores.to_string())
        } else {
            Cell::default()
        };

        Row::new(vec![
            Cell::from(host.id.value.clone()),
            Cell::from(Span::from(STATUS).style(color)),
            Cell::from(Span::styled(symbol, Style::new().fg(conn_color))),
            Cell::from(Span::from(host.name.clone())),
            version,
            host_cpu,
            memory_usage,
            vms,
            networks,
            datastores,
        ])
    }
}


impl TabularData for Host {
    fn get_title() -> &'static str {
        "Hosts"
    }
    fn column_sizes() -> Vec<Constraint> {
        vec![
            Constraint::Length(ID_COLUMN_WIDTH),
            Constraint::Length(STATUS_COLUMN_WIDTH),
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Max(15),
            Constraint::Max(12),
            Constraint::Max(12),
            Constraint::Max(8),
            Constraint::Max(8),
            Constraint::Max(8),
        ]
    }

    fn header_row() -> Vec<&'static str> {
        vec![
            "ID ",
            "S ",
            "C ",
            "Name ",
            "Version ",
            "CPU ",
            "Memory ",
            "VMs ",
            "Net ",
            "DS ",
        ]
    }

    fn sortable_columns() -> Vec<usize> {
        // ID, Name, Version, CPU and Memory are sortable
        vec![0, 3, 4, 5, 6, 7, 8]
    }

    fn sort_by_column(column_idx: usize, descending: bool) -> Option<Box<dyn FnMut(&Self, &Self) -> Ordering>> {
        let mut f: Box<dyn FnMut(&Self, &Self) -> Ordering>  = match column_idx {
            0 => Box::new(|a, b| a.id.value.cmp(&b.id.value)),
            3 => Box::new(|a, b| a.name.cmp(&b.name)),
            4 => Box::new(|a, b| a.version.cmp(&b.version)),
            5 => Box::new(|a, b| a.cpu_usage.cmp(&b.cpu_usage)),
            6 => Box::new(|a, b| a.memory_usage.cmp(&b.memory_usage)),
            7 => Box::new(|a, b| a.vms.cmp(&b.vms)),
            8 => Box::new(|a, b| a.networks.cmp(&b.networks)),
            _ => return None,
        };
        if descending {
            Some(Box::new(move |a: &Self, b: &Self| f(b, a)))
        } else {
            Some(f)
        }
    }

    fn matches_filter(&self, filter: &str) -> bool {
        // Check if the filter matches any of the ID, Name, OS fields
        let filter = filter.to_lowercase();
        self.id.value.to_lowercase().contains(&filter)
            || self.name.to_lowercase().contains(&filter)
            || self.version.as_ref().unwrap_or(&"".to_string()).to_lowercase().contains(&filter)
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn resource_type() -> ResourceType {
        ResourceType::Host
    }

}