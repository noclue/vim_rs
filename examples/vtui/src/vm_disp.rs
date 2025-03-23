use ratatui::widgets::{Cell, Row};
use vim_rs::types::enums::{ManagedEntityStatusEnum, VirtualMachinePowerStateEnum};
use ratatui::prelude::{Color, Span, Style, Stylize};
use crate::vm::VirtualMachine;

const STATUS: &str = "● ";
const POWER_ON: &str = "● ";
// U+25CF
const POWER_OFF: &str = "○ ";
// U+25CB
const SUSPENDED: &str = "◐ ";

impl From<&VirtualMachine> for Row<'_> {
    fn from(vm: &VirtualMachine) -> Self {
        let color = match vm.status {
            ManagedEntityStatusEnum::Green => Style::new().fg(ratatui::style::Color::Green),
            ManagedEntityStatusEnum::Yellow => Style::new().fg(ratatui::style::Color::Yellow),
            ManagedEntityStatusEnum::Red => Style::new().fg(ratatui::style::Color::Red),
            ManagedEntityStatusEnum::Gray => Style::new().fg(ratatui::style::Color::Gray),
            _ => Style::default(),
        };
        let power_state = match vm.power_state {
            VirtualMachinePowerStateEnum::PoweredOn => Span::styled(POWER_ON, Style::default().fg(Color::Green)),
            VirtualMachinePowerStateEnum::PoweredOff => Span::styled(POWER_OFF, Style::default().fg(Color::Red)),
            VirtualMachinePowerStateEnum::Suspended => Span::styled(SUSPENDED, Style::default().fg(Color::Yellow)),
            _ => Span::from("?").gray(),
        };
        let used_space = if let Some(ref storage) = vm.storage {
            Cell::from(format!("{:.2} GB", storage.committed as f64 / 1024.0 / 1024.0 / 1024.0))
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
                Cell::from(format!("{:.2} GB", host_memory as f32 / 1024.0))
            } else {
                Cell::from(format!("{:.2} MB", host_memory as f32))
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