use ratatui::widgets::Row;
use ratatui::layout::Constraint;

pub trait TabularData {
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
}


pub trait TableDataSource {
    fn get_title(&self) -> &'static str;
    fn set_filter(&mut self, filter: Option<String>);
    fn get_filter(&self) -> Option<String>;
    fn set_sort_column(&mut self, column: Option<usize>);

    fn get_sort_setting(&self) -> Option<(usize, bool)>;
    fn iter<'a>(&'a mut self) -> Box<dyn Iterator<Item=Row<'static>> + 'a>;
    fn is_empty(&mut self) -> bool;
    fn column_sizes(&self) -> Vec<Constraint>;
    fn header_row(&self) -> Vec<&'static str>;
    fn invalidate(&mut self);
}