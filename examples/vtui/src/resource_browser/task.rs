use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use ratatui::widgets::{Cell, Row};
use vim_macros::vim_updatable;
use vim_rs::types::enums::TaskInfoStateEnum;
use chrono::{DateTime, FixedOffset, TimeDelta};
use ratatui::layout::Constraint;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use vim_rs::core::client::Client;
use vim_rs::mo::TaskManager;
use vim_rs::types::struct_enum::StructType;
use vim_rs::types::structs::{TaskReasonAlarm, TaskReasonSchedule, TaskReasonUser};
use crate::resource_browser::formatting::ID_COLUMN_WIDTH;
use crate::resource_type::ResourceType;
use crate::resource_browser::tabular_data::TabularData;

vim_updatable!(
    struct TaskInfo: Task {
        name = "info.name",
        description_id = "info.description_id",
        entity = "info.entity",
        entity_name = "info.entity_name",
        state = "info.state",
        progress = "info.progress",
        initiated = "info.queue_time",
        completed = "info.complete_time",
        reason = "info.reason",
    }
);

impl From<&TaskInfo> for Row<'_> {
    /// Shows the following for a task:
    /// - Name or "-" if not available
    /// - State that either shows icon Queued, Error, or Completed. If Running, it shows
    ///  the progress percentage as a 10 character ASCII art bar.
    /// - Description or "-" if not available
    /// - Entity ID value or "-" if not available
    /// - Entity Name or "-" if not available
    /// - Initiated time or "-" if not available
    /// - Duration between initiated and completed time or "-" if either is not available
    fn from(task: &TaskInfo) -> Self {
        let description = task_desc(task);
        let entity = task.entity.clone().map(|x| x.value.clone()).unwrap_or("-".to_string());
        let entity_name = task.entity_name.clone().unwrap_or("-".to_string());
        let initiated = format_datetime(&task.initiated);


        Row::new(vec![
            Cell::from(task.id.value.clone()),
            task_status(&task.state, task.progress),
            Cell::from(description),
            Cell::from(entity),
            Cell::from(entity_name),
            initiated,
            task_duration_cell(&task.initiated, &task.completed),
            Cell::from(get_initiator(task))
        ])
    }
}

impl TabularData for TaskInfo {
    fn get_title() -> &'static str {
        "Tasks"
    }

    fn column_sizes() -> Vec<Constraint> {
        vec![
            Constraint::Length(ID_COLUMN_WIDTH),
            Constraint::Length(13),
            Constraint::Fill(1),
            Constraint::Max(16),
            Constraint::Max(20),
            Constraint::Max(16),
            Constraint::Max(10),
            Constraint::Max(33),
        ]
    }

    fn header_row() -> Vec<&'static str> {
        vec![
            "ID ",
            "State ",
            "Description ",
            "Entity ID ",
            "Entity Name ",
            "Initiated ",
            "Duration ",
            "Initiator ",
        ]
    }

    fn sortable_columns() -> Vec<usize> {
        vec![0, 3, 4, 5, 6]
    }

    fn sort_by_column(column_idx: usize, descending: bool) -> Option<Box<dyn FnMut(&Self, &Self) -> Ordering>> {
        let mut f: Box<dyn FnMut(&Self, &Self) -> Ordering>  = match column_idx {
            0 => Box::new(|a, b| a.id.value.cmp(&b.id.value)),
            3 => Box::new(|a, b| a.entity.as_ref().map(|x| &x.value).cmp(&b.entity.as_ref().map(|x| &x.value))),
            4 => Box::new(|a, b| a.entity_name.cmp(&b.entity_name)),
            5 => Box::new(|a, b| a.initiated.cmp(&b.initiated)),
            6 => Box::new(|a, b| task_duration(&a.initiated, &a.completed).cmp(&task_duration(&b.initiated, &b.completed))),
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
            || self.entity.as_ref().map(|x| x.value.to_lowercase()).unwrap_or("".to_string()).contains(&filter)
            || self.entity_name.as_ref().unwrap_or(&"".to_string()).to_lowercase().contains(&filter)
            || get_initiator(self).to_lowercase().contains(&filter)
    }

    fn name(&self) -> String {
        self.name.clone().unwrap_or("-".to_string())
    }

    fn resource_type() -> ResourceType {
        ResourceType::Task
    }
}

fn get_initiator(task: &TaskInfo) -> String {
    let reason = &task.reason;
    match reason.data_type() {
        StructType::TaskReasonUser => {
            let user = reason.as_ref().as_any_ref().downcast_ref::<TaskReasonUser>().unwrap();
            user.user_name.clone()
        }
        StructType::TaskReasonAlarm => {
            let alarm = reason.as_ref().as_any_ref().downcast_ref::<TaskReasonAlarm>(). unwrap();
            format!("Alarm[{} {}]", alarm.alarm_name, alarm.entity_name)
        }
        StructType::TaskReasonSchedule => {
            let schedule = reason.as_ref().as_any_ref().downcast_ref::<TaskReasonSchedule>().unwrap();
            format!("Schedule[{}]", schedule.name)
        }
        StructType::TaskReasonSystem => "<System>".to_string(),
        _ => "<Unknown>".to_string(),
    }
}

fn format_datetime<'a, 'b>(datetime: &'a String) -> Cell<'b> {
    let Ok(datetime) = DateTime::parse_from_rfc3339(datetime) else {
        return Cell::default();
    };

    let local_datetime = datetime.with_timezone(&chrono::Local);
    let formatted_datetime = local_datetime.format("%b %d %H:%M:%S").to_string();
    Cell::from(formatted_datetime)
}

fn task_duration_cell<'a, 'b, 'c>(initiated: &'a String, completed: &'b Option<String>) -> Cell<'c> {
    match task_duration(initiated, completed) {
        Some(delta) => {
            let formatted_delta = format_timedelta(delta);
            Cell::from(formatted_delta)
        }
        None => Cell::default(),
    }
}

fn task_duration(initiated: &String, completed: &Option<String>) -> Option<TimeDelta> {
    let Ok(initiated) = DateTime::parse_from_rfc3339(initiated) else {
        return None;
    };
    let completed = completed.clone().map(|x| {
        match DateTime::parse_from_rfc3339(&x) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }).unwrap_or_else(|| {
        // Return the now instant if completed is None
        let now : DateTime<FixedOffset> = chrono::Utc::now().fixed_offset();
        Some(now)
    });
    let Some(completed) = completed else {
        return None;
    };
    Some(completed - initiated)
}

fn format_timedelta(delta: chrono::TimeDelta) -> String {
    let days = delta.num_days();
    let hours = delta.num_hours() % 24;
    let minutes = delta.num_minutes() % 60;
    let seconds = delta.num_seconds() % 60;

    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}



/// Returns a cell with the task status.
/// queued -              "⟳ Queued "
/// error -               "✖ Error "
/// completed -           "✔ Completed "
/// running no progress - "Running..."
/// running progress -    "[█████░░░░░]" - indicating 10% increments between 0 and 100
fn task_status<'a, 'b>(state: &'a TaskInfoStateEnum, progress: Option<i32>) -> Cell<'b> {
    match state {
        TaskInfoStateEnum::Queued => labeled_icon("⟳", " Queued ", Color::Yellow),
        TaskInfoStateEnum::Error => labeled_icon("✖", " Error ", Color::Red),
        TaskInfoStateEnum::Success => labeled_icon("✔", " Completed ", Color::Green),
        TaskInfoStateEnum::Running => {
            if let Some(progress) = progress {
                let progress = progress / 10;
                let bar = Line::from(vec![
                    Span::styled("[", Style::default().fg(Color::Gray)),
                    Span::from("█".repeat(progress as usize)),
                    Span::styled("░".repeat(10 - progress as usize), Style::default().fg(Color::Gray)),
                    Span::styled("]", Style::default().fg(Color::Gray)),
                ]);
                Cell::from(bar)
            } else {
                Cell::from("Running...")
            }
        }
        _ => Cell::default(),
    }
}

fn labeled_icon<'a>(icon: &'a str, text: &'a str, color: Color) -> Cell<'a> {
    Cell::from(Line::from(vec![
        Span::styled(icon, Style::default().fg(color)),
        Span::styled(text, Style::default().fg(Color::Gray))]))
}

fn task_desc(task: &TaskInfo) -> String {
    let Some(description) = get_task_description(&task.description_id) else {
        // Decode the name
        if let Some(ref name) = task.name {
            let name = name.trim_end_matches("_Task");
            if name == "Destroy" || name == "Remove" {
                if let Some(ref entity) = task.entity {
                    let s: &str = entity.clone().r#type.into();
                    return format!("{}.{}", name, s)
                }
            }
            if !name.is_empty() {
                return name.to_string();
            }
        }
        return task.description_id.clone();
    };
    description
}

// Global static for storing task descriptions
static TASK_DESCRIPTIONS: OnceLock<HashMap<String, String>> = OnceLock::new();

/// Function to ensure task descriptions are initialized. The VIM API uses a predefined set of task
/// descriptions. To display tasks we need to ensure those descriptions are first cached into a 
/// globally accessible maps
pub async fn ensure_task_descriptions_initialized(client: Arc<Client>) -> anyhow::Result<()> {
    if TASK_DESCRIPTIONS.get().is_some() {
        return Ok(());
    }
    let task_manager = client.service_content().task_manager.as_ref();
    let Some(task_manager) = task_manager else {
        return Ok(());
    };
    // Initialize task descriptions
    let task_manager = &TaskManager::new(client.clone(),&task_manager.value);

    let descriptions = task_manager.description().await?;
    // Transform the response into a HashMap
    let methods = descriptions.method_info;
    let description_map = methods.iter()
        .map(|desc| (desc.get_key().to_string(), desc.get_label().to_string()))
        .collect::<HashMap<String, String>>();

    // Set the global map (will only work once)
    TASK_DESCRIPTIONS.get_or_init(|| description_map);
    Ok(())
}

// Function to look up a description
fn get_task_description(id: &str) -> Option<String> {
    TASK_DESCRIPTIONS.get()
        .map(|map| map.get(id).cloned())
        .unwrap_or(None)
}