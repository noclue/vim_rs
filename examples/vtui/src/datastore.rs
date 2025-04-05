use std::cmp::Ordering;
use ratatui::layout::Constraint;
use ratatui::prelude::{Color, Span, Style};
use ratatui::widgets::{Cell, Row};
use vim_macros::vim_updatable;
use crate::formatting;
use crate::formatting::{status_color, STATUS};
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
        Row::new(vec![
            Cell::from(datastore.id.value.clone()),
            Cell::from(Span::from(STATUS).style(status_color)),
            Cell::from(accessible),
            Cell::from(Span::from(datastore.name.clone())),
            Cell::from(datastore.fs_type.clone()),
            shared,
            capacity,
            free_space,
        ])
    }
}

impl TabularData for DatastoreDetails {
    fn get_title() -> &'static str {
        "Datastores"
    }

    fn column_sizes() -> Vec<Constraint> {
        vec![
            Constraint::Length(20),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Max(15),
            Constraint::Length(4),
            Constraint::Max(12),
            Constraint::Max(12),
        ]
    }

    fn header_row() -> Vec<&'static str> {
        vec![
            "ID",
            "Sta",
            "Ac",
            "Name",
            "FS Type",
            "Shr",
            "Capacity",
            "Free",
        ]
    }

    fn sortable_columns() -> Vec<usize> {
        vec![0, 3, 4, 6, 7]
    }

    fn sort_by_column(column_idx: usize, descending: bool) -> Option<Box<dyn FnMut(&Self, &Self) -> Ordering>> {
        let mut f: Box<dyn FnMut(&Self, &Self) -> Ordering>  = match column_idx {
            0 => Box::new(|a, b| a.id.value.cmp(&b.id.value)),
            3 => Box::new(|a, b| a.name.cmp(&b.name)),
            4 => Box::new(|a, b| a.fs_type.cmp(&b.fs_type)),
            6 => Box::new(|a, b| a.capacity.cmp(&b.capacity)),
            7 => Box::new(|a, b| a.free_space.cmp(&b.free_space)),
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
            || self.fs_type.to_lowercase().contains(&filter)    }
}