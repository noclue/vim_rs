use std::cmp::Ordering;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Cell, Row};
use vim_macros::vim_updatable;
use crate::resource_browser::formatting::{status_color, ID_COLUMN_WIDTH, STATUS, STATUS_COLUMN_WIDTH};
use crate::resource_type::ResourceType;
use crate::resource_browser::tabular_data::TabularData;

vim_updatable!(
    struct ClusterDetails: ClusterComputeResource {
        name = "name",
        overall_status = "overall_status",
        available_cpu = "summary_ex.effective_cpu",
        available_memory = "summary_ex.effective_memory",
        number_of_hosts = "summary_ex.num_hosts",
        drs = "configuration.drs_config.enabled",
        ha = "configuration.das_config.enabled",
        hosts = "host.length",
        networks = "network.length",
        datastores = "datastore.length",
    }
);

impl From<&ClusterDetails> for Row<'_> {
    fn from(cluster: &ClusterDetails) -> Self {
        let status_color = status_color(&cluster.overall_status);
        let cpu = Cell::from(format!("{:.2} GHz", cluster.available_cpu as f32 / 1000.0));
        let memory = if cluster.available_memory > 1024 {
            Cell::from(format!("{:.2} GiB", cluster.available_memory as f32 / 1024.0))
        } else {
            Cell::from(format!("{:.2} MiB", cluster.available_memory as f32))
        };
        let drs = if matches!(cluster.drs, Some(true)) {
            Cell::from(Span::styled("✓", Style::default().fg(Color::Green)))
        } else {
            Cell::from(Span::styled("✗", Style::default().fg(Color::Gray)))
        };
        let ha = if matches!(cluster.ha, Some(true)) {
            Cell::from(Span::styled("✓", Style::default().fg(Color::Green)))
        } else {
            Cell::from(Span::styled("✗", Style::default().fg(Color::Gray)))
        };
        let networks = if let Some(networks) = cluster.networks {
            Cell::from(networks.to_string())
        } else {
            Cell::default()
        };
        let datastores = if let Some(datastores) = cluster.datastores {
            Cell::from(datastores.to_string())
        } else {
            Cell::default()
        };

        Row::new(vec![
            Cell::from(cluster.id.value.clone()),
            Cell::from(Span::from(STATUS).style(status_color)),
            Cell::from(cluster.name.clone()),
            Cell::from(cluster.number_of_hosts.to_string()),
            cpu,
            memory,
            drs,
            ha,
            networks,
            datastores,
        ])
    }
}

impl TabularData for ClusterDetails {
    fn get_title() -> &'static str {
        "Clusters"
    }

    fn column_sizes() -> Vec<Constraint> {
        vec![
            Constraint::Length(ID_COLUMN_WIDTH),
            Constraint::Length(STATUS_COLUMN_WIDTH),
            Constraint::Fill(1), // name
            Constraint::Length(8), // number of hosts
            Constraint::Length(12), // available cpu
            Constraint::Length(12), // available memory
            Constraint::Length(4), // drs
            Constraint::Length(4), // ha
            Constraint::Max(10), // networks
            Constraint::Max(10), // datastores
        ]
    }

    fn header_row() -> Vec<&'static str> {
        vec![
            "ID ",
            "S",
            "Name",
            "Hosts",
            "CPU",
            "Memory",
            "DRS",
            "HA",
            "Networks",
            "Datastores",
        ]
    }

    fn sortable_columns() -> Vec<usize> {
        vec![0, 2, 3, 4, 5, 8, 9]
    }

    fn sort_by_column(column_idx: usize, descending: bool) -> Option<Box<dyn FnMut(&Self, &Self) -> Ordering>> {
        let mut f: Box<dyn FnMut(&Self, &Self) -> Ordering> = match column_idx {
            0 => Box::new(|a, b| a.id.value.cmp(&b.id.value)),
            2 => Box::new(|a, b| a.name.cmp(&b.name)),
            3 => Box::new(|a, b| a.number_of_hosts.cmp(&b.number_of_hosts)),
            4 => Box::new(|a, b| a.available_cpu.cmp(&b.available_cpu)),
            5 => Box::new(|a, b| a.available_memory.cmp(&b.available_memory)),
            8 => Box::new(|a, b| a.networks.cmp(&b.networks)),
            9 => Box::new(|a, b| a.datastores.cmp(&b.datastores)),
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
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn resource_type() -> ResourceType {
        ResourceType::Cluster
    }
}