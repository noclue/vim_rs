use ratatui::prelude::{Line, StatefulWidget, Style, Stylize, Widget};
use ratatui::layout::{Constraint, Rect};
use ratatui::buffer::Buffer;
use ratatui::widgets::{Block, HighlightSpacing, Table, TableState};
use std::cell::RefCell;
use vim_rs::types::structs::ObjectUpdate;
use vim_rs::types::enums::ObjectUpdateKindEnum;
use indexmap::IndexMap;
use crate::vm::VirtualMachine;

/// A widget that displays a list of virtual machines.
///
/// This is an async widget that fetches the list of VMs from the vCenter API. It contains
/// an inner `Arc<RwLock<VmListState>>` that holds the state of the widget. Cloning the
/// widget will clone the Arc, so you can pass it around to other threads, and this is used to spawn
/// a background task to fetch the VMs.
pub struct VmListWidget {
    state: RefCell<VmListState>,
}

#[derive(Debug, Default)]
struct VmListState {
    vms: IndexMap<String, VirtualMachine>,
    loading_state: LoadingState,
    table_state: TableState,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum LoadingState {
    #[default]
    Idle,
    Error(String),
}

impl VmListWidget {
    pub(crate) fn new() -> Self {
        Self {
            state: RefCell::new(VmListState::default()),
        }
    }

    pub(crate) fn apply_updates(&mut self, updates: Vec<ObjectUpdate>) {
        for update in updates {
            match update.kind {
                ObjectUpdateKindEnum::Enter => {
                    self.add_vm(&update);
                }
                ObjectUpdateKindEnum::Modify => {
                    let state = self.state.get_mut();
                    let Some(ref row) = update.change_set else {
                        continue;
                    };
                    let Some(vm) = state.vms.get_mut(&update.obj.value) else {
                        self.add_vm(&update);
                        continue;
                    };
                    vm.apply_update(row).unwrap();
                }
                ObjectUpdateKindEnum::Leave => {
                    let state = self.state.get_mut();
                    state.vms.shift_remove(&update.obj.value);
                }
                _ => {}
            }
        }

        let vm_count = self.state.borrow().vms.len();
        if vm_count == 0 {
            self.set_loading_state(LoadingState::Error("No VMs found".to_string()));
            return;
        } else {
            let mut state = self.state.borrow_mut();
            match state.table_state.selected() {
                Some(selected) => {
                    if selected >= vm_count {
                        state.table_state.select(Some(vm_count - 1));
                    }
                }
                None => {
                    state.table_state.select(Some(0));
                }
            }
        }
    }

    fn add_vm(&mut self, update: &ObjectUpdate) {
        let vm = match VirtualMachine::try_from(update) {
            Ok(vm) => vm,
            Err(e) => {
                panic!("Error converting to VM: {}", e);
                // self.set_loading_state(LoadingState::Error(e.to_string()));
                // return;
            }
        };
        let state = self.state.get_mut();
        state.vms.insert(vm.id().to_string(), vm);
    }

    pub(crate) fn set_loading_state(&mut self, state: LoadingState) {
        self.state.get_mut().loading_state = state;
    }

    pub(crate) fn scroll_down(&mut self) {
        self.state.get_mut().table_state.scroll_down_by(1);
    }

    pub(crate) fn scroll_up(&mut self) {
        self.state.get_mut().table_state.scroll_up_by(1);
    }
}

impl Widget for &VmListWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.state.borrow_mut();
        // a block with a right aligned title with the loading state on the right
        let loading_state = Line::from(format!("{:?}", state.loading_state)).right_aligned();
        let block = Block::bordered()
            .title("Virtual Machines")
            .title(loading_state)
            .title_bottom("j/k to scroll, q to quit");

        // a table with the list of pull requests
        let rows = state.vms.values();
        let widths = [
            Constraint::Length(10),
            Constraint::Length(3),
            Constraint::Length(3),

            Constraint::Fill(1),
            Constraint::Max(25),
            Constraint::Max(15),

            Constraint::Max(15),
            Constraint::Max(15),
        ];
        let table = Table::new(rows, widths)
            .block(block)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">>")
            .row_highlight_style(Style::new().on_blue());

        StatefulWidget::render(table, area, buf, &mut state.table_state);
    }
}