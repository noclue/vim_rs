use ratatui::widgets::{Cell, Row};
use vim_rs::types::enums::{ManagedEntityStatusEnum, MoTypesEnum, VirtualMachinePowerStateEnum};
use ratatui::prelude::{Color, Span, Style, Stylize};
use vim_rs::types::structs::{ObjectUpdate, PropertyChange, PropertySpec};
use vim_rs::types::vim_any::VimAny;
use vim_rs::types::boxed_types::ValueElements;
use vim_rs::types::structs;
use anyhow::{Context, Result};



#[derive(Debug, Clone)]
pub struct VirtualMachine {
    id: String,
    name: String,
    os: String,
    used_space: Option<i64>, // summary.storage.committed
    host_cpu: Option<i32>, // summary.quickStats.overallCpuUsage
    host_memory: Option<i32>, // summary.quickStats.hostMemoryUsage
    status: Option<ManagedEntityStatusEnum>, // summary.overallStatus
    power_state: Option<VirtualMachinePowerStateEnum>, // summary.runtime.powerState
}

impl VirtualMachine {
    pub fn prop_spec() -> PropertySpec {
        structs::PropertySpec {
            all: Some(false),
            path_set: Some(vec![
                "name".into(),
                "summary.guest.guestFullName".into(),
                "summary.storage.committed".into(),
                "summary.quickStats.overallCpuUsage".into(),
                "summary.quickStats.hostMemoryUsage".into(),
                "overallStatus".into(),
                "runtime.powerState".into(),
            ]),
            r#type: Into::<&str>::into(MoTypesEnum::VirtualMachine).to_string(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn apply_update(&mut self, row: &Vec<PropertyChange>) -> Result<()>{
        for prop in row {
            match prop.name.as_str() {
                "name" => {
                    self.name = match &prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveString(val))) => val.clone(),
                        _ => "<Unknown>".to_string(),
                    };
                }
                "summary.guest.guestFullName" => {
                    self.os = match &prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveString(val))) => val.clone(),
                        _ => "<Unknown>".to_string(),
                    };
                }
                "summary.storage.committed" => {
                    self.used_space = match &prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveLong(val))) => Some(*val),
                        _ => None,
                    };
                }
                "summary.quickStats.overallCpuUsage" => {
                    self.host_cpu = match &prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveInt(val))) => Some(*val),
                        _ => None,
                    };
                }
                "summary.quickStats.hostMemoryUsage" => {
                    self.host_memory = match &prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveInt(val))) => Some(*val),
                        _ => None,
                    };
                }
                "overallStatus" => {
                    self.status = match &prop.val {
                        Some(VimAny::Value(ValueElements::ManagedEntityStatus(val))) => Some(val.clone()),
                        _ => None,
                    };
                }
                "runtime.powerState" => {
                    self.power_state = match &prop.val {
                        Some(VimAny::Value(ValueElements::VirtualMachinePowerState(val))) => Some(val.clone()),
                        _ => None,
                    };
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl TryFrom<&ObjectUpdate> for VirtualMachine {
    type Error = anyhow::Error;

    fn try_from(row: &ObjectUpdate) -> anyhow::Result<Self> {
        let id = row.obj.value.clone();
        let row = row.change_set.as_ref().context("No change set found")?;

        let mut name = "<Unknown>".to_string();
        let mut os = "<Unknown>".to_string();
        let mut used_space = None;
        let mut host_cpu = None;
        let mut host_memory = None;
        let mut status = None;
        let mut power_state = None;

        for prop in row {
            match prop.name.as_str() {
                "name" => {
                    name = match &prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveString(val))) => val.clone(),
                        None => continue,
                        _ => return Err(anyhow::anyhow!("Invalid value type for property 'name'")),
                    };
                }
                "summary.guest.guestFullName" => {
                    os = match &prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveString(val))) => val.clone(),
                        None => continue,
                        _ => return Err(anyhow::anyhow!("Invalid value type for property 'summary.guest.guestFullName'")),
                    };
                }
                "summary.storage.committed" => {
                    used_space = match &prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveLong(val))) => Some(*val),
                        _ => return Err(anyhow::anyhow!("Invalid value type for property 'summary.storage.committed'")),
                    };
                }
                "summary.quickStats.overallCpuUsage" => {
                    host_cpu = match &prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveInt(val))) => Some(*val),
                        _ => return Err(anyhow::anyhow!("Invalid value type for property 'summary.quickStats.overallCpuUsage'")),
                    };
                }
                "summary.quickStats.hostMemoryUsage" => {
                    host_memory = match &prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveInt(val))) => Some(*val),
                        _ => return Err(anyhow::anyhow!("Invalid value type for property 'summary.quickStats.hostMemoryUsage'")),
                    };
                }
                "overallStatus" => {
                    status = match &prop.val {
                        Some(VimAny::Value(ValueElements::ManagedEntityStatus(val))) => Some(val.clone()),
                        _ => return Err(anyhow::anyhow!("Invalid value type for property 'overallStatus'")),
                    };
                }
                "runtime.powerState" => {
                    power_state = match &prop.val {
                        Some(VimAny::Value(ValueElements::VirtualMachinePowerState(val))) => Some(val.clone()),
                        _ => return Err(anyhow::anyhow!("Invalid value type for property 'runtime.powerState'")),
                    };
                }
                _ => {}
            }
        }
        Ok(VirtualMachine {
            id,
            name,
            os,
            status,
            power_state,
            used_space,
            host_cpu,
            host_memory,
        })
    }
}


const STATUS: &str = "● ";
const POWER_ON: &str = "● ";    // U+25CF
const POWER_OFF: &str = "○ ";   // U+25CB
const SUSPENDED: &str = "◐ ";   // U+25D0


impl From<&VirtualMachine> for Row<'_> {
    fn from(vm: &VirtualMachine) -> Self {
        let vm = vm.clone();
        let color = match vm.status {
            Some(ManagedEntityStatusEnum::Green) => Style::new().fg(ratatui::style::Color::Green),
            Some(ManagedEntityStatusEnum::Yellow) => Style::new().fg(ratatui::style::Color::Yellow),
            Some(ManagedEntityStatusEnum::Red) => Style::new().fg(ratatui::style::Color::Red),
            Some(ManagedEntityStatusEnum::Gray) => Style::new().fg(ratatui::style::Color::Gray),
            _ => Style::default(),
        };
        let power_state = match vm.power_state {
            Some(VirtualMachinePowerStateEnum::PoweredOn) => Span::styled(POWER_ON, Style::default().fg(Color::Green)),
            Some(VirtualMachinePowerStateEnum::PoweredOff) => Span::styled(POWER_OFF, Style::default().fg(Color::Red)),
            Some(VirtualMachinePowerStateEnum::Suspended) => Span::styled(SUSPENDED, Style::default().fg(Color::Yellow)),
            _ => Span::from("?").gray(),
        };
        let used_space = if let Some(used_space) = vm.used_space {
            Cell::from(format!("{:.2} GB", used_space as f64 / 1024.0 / 1024.0 / 1024.0))
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
            Cell::from(vm.id),
            Cell::from(Span::from(STATUS).style(color)),
            Cell::from(power_state),

            Cell::from(vm.name),
            Cell::from(vm.os),
            used_space,

            host_cpu,
            host_memory,
        ])
    }
}