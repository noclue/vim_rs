use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, StatefulWidget, Style, Stylize};
use ratatui::widgets::{Block, HighlightSpacing, Row, Cell, Table, TableState};
use ratatui::text::Span;
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::ManagedObjectReference;
use crate::tabular_data::TableDataSource;




/// A widget that displays a list of virtual machines.
pub struct ResourceTableWidget<'a>

{
    resources: &'a mut dyn TableDataSource,
    parent: &'a Option<(ManagedObjectReference, String)>
}

impl<'a> ResourceTableWidget<'a>
{
    pub(crate) fn new(resources: &'a mut dyn TableDataSource, parent: &'a Option<(ManagedObjectReference, String)>) -> Self {
        Self {
            resources,
            parent
        }
    }
}

impl<'a> StatefulWidget for ResourceTableWidget<'a> {
    type State = TableState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {

        let filter = self.resources.get_filter();
        let hint = if filter.is_some() {
            vec![
                Span::styled("Esc clear filter", Style::default().fg(ratatui::style::Color::LightCyan)),
            ]
        } else {
            vec![Span::default()]
        };

        let mut title_items = Vec::new();
        title_items.push(Span::styled(self.resources.get_title(), Style::default().fg(ratatui::style::Color::White)));
        title_items.push(Span::from(" ("));
        title_items.push(if let Some(filter) = &filter {
            Span::styled(format!("filter: {}", filter), Style::default().fg(ratatui::style::Color::Magenta))
        } else {
            Span::styled("all", Style::default().fg(ratatui::style::Color::Magenta))
        });
        title_items.push(Span::from(")["));

        let len = self.resources.len();
        let total =self.resources.total_count();
        let count = if len != total {
            format!("{} / {}", len, total)
        } else {
            format!("{}", len)
        };
        title_items.push(Span::styled(count, Style::default().fg(ratatui::style::Color::White)));
        title_items.push(Span::from("]"));

        if let Some((id, name)) = self.parent {
            title_items.push(Span::from(" - "));
            title_items.push(Span::styled(object_handle(id, name), Style::default().fg(ratatui::style::Color::Green)));
        }


        let title = Line::from(title_items).alignment(ratatui::layout::Alignment::Center);

        let block = Block::bordered()
            .title(title)
            .title_bottom(hint)
            .border_style(Style::default().fg(ratatui::style::Color::Gray));


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

        let header = Row::new(header).style(Style::default().fg(ratatui::style::Color::Cyan));

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

/// Devises a simple string representation of an object form its type and name.
fn object_handle(id: &ManagedObjectReference, name: &String) -> String {
    let type_str = match id.r#type {
        MoTypesEnum::VirtualMachine => "VM",
        MoTypesEnum::HostSystem => "Host",
        MoTypesEnum::Datastore => "Datastore",
        MoTypesEnum::ClusterComputeResource => "Cluster",
        MoTypesEnum::Network => "Network",
        MoTypesEnum::DistributedVirtualPortgroup => "DVPG",
        MoTypesEnum::OpaqueNetwork => "NSX",
        _ => "Unknown",
    };
    format!("{}: {}", type_str, name)
}