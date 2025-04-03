use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{StatefulWidget, Style, Stylize, Widget};
use ratatui::widgets::{Block, HighlightSpacing, Row, Cell, Table, TableState};
use std::cell::RefCell;
use std::ops::DerefMut;
use ratatui::text::Span;
use crate::tabular_data::TableDataSource;

/// A widget that displays a list of virtual machines.

pub struct ResourceTableWidget<T>
where
    T: TableDataSource
{
    resources: RefCell<T>,
    table_state: RefCell<TableState>,
    filter: Option<String>,
}

impl<T> ResourceTableWidget<T>
where
    T: TableDataSource
{
    pub(crate) fn set_sort_column(&self, sort_column: Option<usize>) {
        self.resources.borrow_mut().set_sort_column(sort_column);
    }
}

impl<T> ResourceTableWidget<T>
where
    T: TableDataSource
{
    pub(crate) fn new(resources: T) -> Self {
        Self {
                resources: RefCell::new(resources),
                table_state: RefCell::new(TableState::default()),
                filter: None,
        }
    }

    pub fn set_filter(&mut self, filter: Option<String>) {
        self.filter = filter;
        self.resources.borrow_mut().set_filter(self.filter.clone());
    }

    pub(crate) fn scroll_down(&mut self) {
        self.table_state.borrow_mut().scroll_down_by(1);
    }

    pub(crate) fn scroll_up(&mut self) {
        self.table_state.borrow_mut().scroll_up_by(1);
    }


    pub fn invalidate(&mut self) {
        self.resources.borrow_mut().invalidate();
    }
}

impl<T: TableDataSource> Widget for &ResourceTableWidget<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let hint = if self.filter.is_some() {
            vec![
                Span::from("j/k to scroll, q to quit, / to search, "),
                Span::styled("Esc clear filter", Style::default().fg(ratatui::style::Color::LightCyan)),
            ]
        } else {
            vec![Span::from("j/k to scroll, q to quit, / to search")]
        };

        let filter = if let Some(filter) = &self.filter {
            Span::styled(format!("filter: {}", filter), Style::default().fg(ratatui::style::Color::Magenta))
        } else {
            Span::styled("all", Style::default().fg(ratatui::style::Color::Magenta))
        };

        let title = vec![
            Span::from("Virtual Machines("),
            filter,
            Span::from(")"),
        ];
        let block = Block::bordered()
            .title(title)
            .title_bottom(hint);


        let mut resources = self.resources.borrow_mut();

        let sort_setting = resources.get_sort_setting();
        let header_row = resources.header_row();
        let mut header = Vec::with_capacity(header_row.len());
        for i in 0..header_row.len() {
            if let Some(sort_setting) = sort_setting {
                if i == sort_setting.0 {
                    let symbol = if sort_setting.1 { "▼" } else { "▲" };
                    header.push(Cell::from(format!("{} {}", header_row[i], symbol)));
                } else {
                    header.push(Cell::from(header_row[i]));
                }
            } else {
                header.push(Cell::from(header_row[i]));
            }
        }
        let header = Row::new(header);

        let widths = resources.column_sizes();


        let mut table_state = self.table_state.borrow_mut();

        if table_state.selected().is_none() && !resources.is_empty(){
            table_state.select(Some(0));
        }

        // a table with the list of pull requests
        let rows = resources.iter();

        let table = Table::new(rows, widths)
            .block(block)
            .header(header)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">>")
            .row_highlight_style(Style::new().on_blue());

        StatefulWidget::render(table, area, buf, table_state.deref_mut());
    }
}
