use ratatui::widgets::Row;
use ratatui::layout::Constraint;
use vim_rs::types::structs::ManagedObjectReference;
use crate::resource_type::ResourceType;

/// Trait for objects that can be displayed in a table. This exposes the static information
/// about the table, such as the title, column sizes, and header row. It also provides
/// methods for sorting and filtering the data.
pub trait TabularData {
    // Get the title of the table
    fn get_title() -> &'static str;
    // Column constraints for the table
    fn column_sizes() -> Vec<Constraint>;

    // Header row with column titles
    fn header_row() -> Vec<&'static str>;

    // Which columns support sorting (by index)
    fn sortable_columns() -> Vec<usize>;

    // Get sorting function for a specific column
    fn sort_by_column(column_idx: usize, descending: bool) -> Option<Box<dyn FnMut(&Self, &Self) -> std::cmp::Ordering>>;

    // Whether this item matches the given filter string
    fn matches_filter(&self, filter: &str) -> bool;

    // Get a display name for the item e.g. "admin_cluster"
    fn name(&self) -> String;

    fn resource_type() -> ResourceType;
}

/// Trait for data sources that can be displayed in a table.
pub trait TableDataSource {
    fn get_title(&self) -> &'static str;
    fn set_filter(&mut self, filter: Option<String>);
    fn get_filter(&self) -> Option<String>;
    fn set_sort_column(&mut self, column: Option<usize>);

    fn get_sort_setting(&self) -> Option<(usize, bool)>;
    fn iter<'a>(&'a mut self) -> Box<dyn Iterator<Item=Row<'static>> + 'a>;
    fn is_empty(&mut self) -> bool;
    fn len(&mut self) -> usize;
    fn total_count(&self) -> usize;
    fn column_sizes(&self) -> Vec<Constraint>;
    fn header_row(&self) -> Vec<&'static str>;
    fn invalidate(&mut self);
    // Get the ID and Name of the object at the given index
    fn item_at_index(&mut self, index: usize) -> Option<(ManagedObjectReference, String)>;
    fn resource_type(&self) -> ResourceType;
}