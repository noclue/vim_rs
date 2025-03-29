use ratatui::prelude::{Line, StatefulWidget, Style, Stylize, Widget};
use ratatui::layout::{Constraint, Rect};
use ratatui::buffer::Buffer;
use ratatui::widgets::{Block, Cell, HighlightSpacing, Row, Table, TableState};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use crate::object_cache::ObjectCache;
use crate::vm::VirtualMachine;

/// A widget that displays a list of virtual machines.

pub struct VmListWidget {
    state: VmListState,
}


struct VmListState {
    vms: Rc<RefCell<ObjectCache<VirtualMachine>>>,
    loading_state: LoadingState,
    table_state: RefCell<TableState>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum LoadingState {
    #[default]
    Idle,
    Error(String),
}

impl VmListWidget {
    pub(crate) fn new(cache: Rc<RefCell<ObjectCache<VirtualMachine>>>) -> Self {
        Self {
            state: VmListState{
                vms: cache,
                loading_state: LoadingState::Idle,
                table_state: RefCell::new(TableState::default()),
            },
        }
    }

    pub(crate) fn set_loading_state(&mut self, state: LoadingState) {
        self.state.loading_state = state;
    }

    pub(crate) fn scroll_down(&mut self) {
        self.state.table_state.borrow_mut().scroll_down_by(1);
    }

    pub(crate) fn scroll_up(&mut self) {
        self.state.table_state.borrow_mut().scroll_up_by(1);
    }
}

impl Widget for &VmListWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let state = &self.state;
        // a block with a right aligned title with the loading state on the right
        let loading_state = Line::from(format!("{:?}", state.loading_state)).right_aligned();
        let block = Block::bordered()
            .title("Virtual Machines")
            .title(loading_state)
            .title_bottom("j/k to scroll, q to quit");

        let header = Row::new(vec![
            Cell::from("ID "),
            Cell::from("S "),
            Cell::from("P "),
            Cell::from("Name "),
            Cell::from("OS "),
            Cell::from("Used Space "),
            Cell::from("CPU "),
            Cell::from("Memory "),
        ]);

        // a table with the list of pull requests
        let cache = state.vms.borrow();

        let widths = [
            Constraint::Length(10),
            Constraint::Length(4),
            Constraint::Length(4),

            Constraint::Fill(1),
            Constraint::Max(15),
            Constraint::Max(12),

            Constraint::Max(12),
            Constraint::Max(12),
        ];
        let table = Table::new(cache.iter(), widths)
            .block(block)
            .header(header)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">>")
            .row_highlight_style(Style::new().on_blue());

        StatefulWidget::render(table, area, buf, state.table_state.borrow_mut().deref_mut());
    }
}