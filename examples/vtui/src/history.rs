use std::collections::VecDeque;
use crate::prop_browser::HistoryRecord as PropertyEntry;
use crate::resource_browser::HistoryRecord as ResourceEntry;

pub enum History {
    Resource(ResourceEntry),
    Property(PropertyEntry),
}

/// HistoryManager is responsible for managing the history of previous states
/// for back navigation in the UI.
pub struct HistoryManager {
    /// History of previous states for back navigation
    history: VecDeque<History>,
    /// Maximum history entries to keep
    max_history: usize,
}

impl HistoryManager {
    /// Creates a new HistoryManager with the specified maximum history size.
    pub fn new(max_history: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    /// Adds a new resource entry to the history.
    pub fn add_resource_entry(&mut self, entry: ResourceEntry) {
        if self.history.len() == self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(History::Resource(entry));
    }

    /// Adds a new property entry to the history.
    pub fn add_property_entry(&mut self, entry: PropertyEntry) {
        if self.history.len() == self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(History::Property(entry));
    }

    /// Returns the last entry in the history.
    pub fn pop(&mut self) -> Option<History> {
        if self.history.is_empty() {
            None
        } else {
            self.history.pop_back()
        }
    }
}