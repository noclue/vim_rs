use std::cmp::Ordering;
use ratatui::layout::Constraint;
use vim_macros::vim_updatable;
use ratatui::widgets::{Cell, Row};
use vim_rs::types::enums::{ManagedEntityStatusEnum, VirtualMachinePowerStateEnum};
use ratatui::prelude::{Color, Span, Style, Stylize};
use crate::tabular_data::TabularData;

vim_updatable!(
    struct VmData: VirtualMachine {
        name = "name",
        os = "summary.guest.guest_full_name",
        storage = "summary.storage",
        host_cpu = "summary.quick_stats.overall_cpu_usage",
        host_memory = "summary.quick_stats.host_memory_usage",
        status = "overall_status",
        power_state = "runtime.power_state",
    }
);

pub(crate) const STATUS: &str = "● ";
const POWER_ON: &str = "● ";
// U+25CF
const POWER_OFF: &str = "○ ";
// U+25CB
const SUSPENDED: &str = "◐ ";

impl From<&VmData> for Row<'_> {
    fn from(vm: &VmData) -> Self {
        let color = status_color(&vm.status);
        let power_state = match vm.power_state {
            VirtualMachinePowerStateEnum::PoweredOn => {
                Span::styled(POWER_ON, Style::default().fg(Color::Green))
            }
            VirtualMachinePowerStateEnum::PoweredOff => {
                Span::styled(POWER_OFF, Style::default().fg(Color::Red))
            }
            VirtualMachinePowerStateEnum::Suspended => {
                Span::styled(SUSPENDED, Style::default().fg(Color::Yellow))
            }
            _ => Span::from("?").gray(),
        };
        let used_space = if let Some(ref storage) = vm.storage {
            Cell::from(format!(
                "{:.2} GB",
                storage.committed as f64 / 1024.0 / 1024.0 / 1024.0
            ))
        } else {
            Cell::default()
        };
        let host_cpu = if let Some(host_cpu) = vm.host_cpu {
            Cell::from(format!("{:.2} MHz", host_cpu as f32))
        } else {
            Cell::default()
        };
        let host_memory = if let Some(host_memory) = vm.host_memory {
            if host_memory > 1024 {
                Cell::from(format!("{:.2} GiB", host_memory as f32 / 1024.0))
            } else {
                Cell::from(format!("{:.2} MiB", host_memory as f32))
            }
        } else {
            Cell::default()
        };

        Row::new(vec![
            Cell::from(vm.id.value.clone()),
            Cell::from(Span::from(STATUS).style(color)),
            Cell::from(power_state),
            Cell::from(vm.name.clone()),
            Cell::from(vm.os.clone().unwrap_or("<unknown>".to_string())),
            used_space,
            host_cpu,
            host_memory,
        ])
    }
}

pub(crate) fn status_color(status: &ManagedEntityStatusEnum) -> Style {
    match status {
        ManagedEntityStatusEnum::Green => Style::new().fg(Color::Green),
        ManagedEntityStatusEnum::Yellow => Style::new().fg(Color::Yellow),
        ManagedEntityStatusEnum::Red => Style::new().fg(Color::Red),
        ManagedEntityStatusEnum::Gray => Style::new().fg(Color::Gray),
        _ => Style::default(),
    }
}

impl TabularData for VmData {
    fn column_sizes() -> Vec<Constraint> {
        vec![
            Constraint::Length(10),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Max(15),
            Constraint::Max(12),
            Constraint::Max(12),
            Constraint::Max(12),
        ]
    }

    fn header_row() -> Vec<&'static str> {
        vec![
            "ID ",
            "S ",
            "P ",
            "Name ",
            "OS ",
            "Used Space ",
            "CPU ",
            "Memory ",
        ]
    }

    fn sortable_columns() -> Vec<usize> {
        // ID, Name, OS, Used Space, CPU and Memory are sortable
        vec![0, 3, 4, 5, 6, 7]
    }

    fn sort_by_column(column_idx: usize, descending: bool) -> Option<Box<dyn FnMut(&Self, &Self) -> Ordering>> {
        let mut f: Box<dyn FnMut(&Self, &Self) -> Ordering>  = match column_idx {
            0 => Box::new(|a, b| a.id.value.cmp(&b.id.value)),
            3 => Box::new(|a, b| a.name.cmp(&b.name)),
            4 => Box::new(|a, b| a.os.cmp(&b.os)),
            5 => Box::new(|a, b| a.storage.as_ref().map_or(0, |s| s.committed).cmp(&b.storage.as_ref().map_or(0, |s| s.committed))),
            6 => Box::new(|a, b| a.host_cpu.cmp(&b.host_cpu)),
            7 => Box::new(|a, b| a.host_memory.cmp(&b.host_memory)),
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
            || self.os.as_ref().unwrap_or(&"".to_string()).to_lowercase().contains(&filter)
    }
}