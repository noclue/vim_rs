use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{StatefulWidget, Style, Stylize};
use ratatui::widgets::{Block, HighlightSpacing, Row, Cell, Table, TableState};
use ratatui::text::Span;
use crate::tabular_data::TableDataSource;




/// A widget that displays a list of virtual machines.
pub struct ResourceTableWidget<'a>

{
    resources: &'a mut dyn TableDataSource,
}

impl<'a> ResourceTableWidget<'a>
{
    pub(crate) fn new(resources: &'a mut dyn TableDataSource) -> Self {
        Self {
            resources,
        }
    }
}

impl<'a> StatefulWidget for ResourceTableWidget<'a> {
    type State = TableState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {

        let filter = self.resources.get_filter();
        let hint = if filter.is_some() {
            vec![
                Span::from("j/k to scroll, q to quit, / to search, "),
                Span::styled("Esc clear filter", Style::default().fg(ratatui::style::Color::LightCyan)),
            ]
        } else {
            vec![Span::from("j/k to scroll, q to quit, / to search")]
        };

        let filter = if let Some(filter) = &filter {
            Span::styled(format!("filter: {}", filter), Style::default().fg(ratatui::style::Color::Magenta))
        } else {
            Span::styled("all", Style::default().fg(ratatui::style::Color::Magenta))
        };

        let title = vec![
            Span::from(self.resources.get_title()),
            Span::from(" ("),
            filter,
            Span::from(")"),
        ];
        let block = Block::bordered()
            .title(title)
            .title_bottom(hint);


        let sort_setting = self.resources.get_sort_setting();
        let header_row = self.resources.header_row();
        let mut header = Vec::with_capacity(header_row.len());
        for i in 0..header_row.len() {
            if let Some(sort_setting) = sort_setting {
                if i == sort_setting.0 {
                    let arrow_span = if sort_setting.1 {
                        Span::styled("▼", Style::default().fg(ratatui::style::Color::Blue))
                    } else {
                        Span::styled("▲", Style::default().fg(ratatui::style::Color::Green))
                    };
                    header.push(Cell::from(ratatui::text::Line::from(vec![
                        Span::from(header_row[i]),
                        arrow_span
                    ])));
                } else {
                    header.push(Cell::from(header_row[i]));
                }
            } else {
                header.push(Cell::from(header_row[i]));
            }
        }
        let header = Row::new(header);

        let widths = self.resources.column_sizes();

        if state.selected().is_none() && !self.resources.is_empty(){
            state.select(Some(0));
        }

        let rows = self.resources.iter();

        let table = Table::new(rows, widths)
            .block(block)
            .header(header)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol("▶ ")
            .row_highlight_style(Style::new().on_blue());

        StatefulWidget::render(table, area, buf, state);
    }
}
