use std::cmp::Ordering;
use ratatui::layout::Constraint;
use ratatui::prelude::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Cell, Row};
use vim_macros::vim_updatable;
use crate::formatting::{status_color, STATUS};
use crate::tabular_data::TabularData;

vim_updatable!(
    struct ClusterDetails: ClusterComputeResource {
        name = "name",
        overall_status = "overall_status",
        available_cpu = "summary_ex.effective_cpu",
        available_memory = "summary_ex.effective_memory",
        number_of_hosts = "summary_ex.num_hosts",
        drs = "configuration.drs_config.enabled",
        ha = "configuration.das_config.enabled",
        cpu_evc = "summary_ex.current_evc_mode_key",
        gpu_evc = "summary_ex.current_evc_graphics_mode_key",
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
        let cpu_evc = if let Some(cpu_evc) = &cluster.cpu_evc {
            Cell::from(format!("{cpu_evc}"))
        } else {
            Cell::from(Span::styled("-", Style::default().fg(Color::Gray)))
        };
        let gpu_evc = if let Some(gpu_evc) = &cluster.gpu_evc {
            Cell::from(format!("{gpu_evc}"))
        } else {
            Cell::from(Span::styled("-", Style::default().fg(Color::Gray)))
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
            cpu_evc,
            gpu_evc,
        ])
    }
}

impl TabularData for ClusterDetails {
    fn get_title() -> &'static str {
        "Clusters"
    }

    fn column_sizes() -> Vec<Constraint> {
        vec![
            Constraint::Length(10), // ID
            Constraint::Length(4), // status
            Constraint::Fill(1), // name
            Constraint::Length(8), // number of hosts
            Constraint::Length(12), // available cpu
            Constraint::Length(12), // available memory
            Constraint::Length(4), // drs
            Constraint::Length(4), // ha
            Constraint::Max(20), // cpu evc
            Constraint::Max(20), // gpu evc
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
            "CPU EVC",
            "GPU EVC",
        ]
    }

    fn sortable_columns() -> Vec<usize> {
        vec![0, 2, 3, 4, 5]
    }

    fn sort_by_column(column_idx: usize, descending: bool) -> Option<Box<dyn FnMut(&Self, &Self) -> Ordering>> {
        let mut f: Box<dyn FnMut(&Self, &Self) -> Ordering> = match column_idx {
            0 => Box::new(|a, b| a.id.value.cmp(&b.id.value)),
            2 => Box::new(|a, b| a.name.cmp(&b.name)),
            3 => Box::new(|a, b| a.number_of_hosts.cmp(&b.number_of_hosts)),
            4 => Box::new(|a, b| a.available_cpu.cmp(&b.available_cpu)),
            5 => Box::new(|a, b| a.available_memory.cmp(&b.available_memory)),
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
}