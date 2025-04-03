use std::cell::RefCell;
use std::rc::Rc;
use ratatui::layout::Constraint;
use vim_rs::core::pc_cache::{Cacheable, ObjectCache};
use vim_rs::core::pc_helpers::BoxableError;
use crate::tabular_data::{TableDataSource, TabularData};
use ratatui::widgets::Row;
pub struct IndexedCache<T>
where
    T: Cacheable + TabularData,
    T::Error: BoxableError,
    for<'a> Row<'static>: From<&'a T>
{
    cache: Rc<RefCell<ObjectCache<T>>>,
    indices: Option<Vec<usize>>,  // Filtered/sorted indices into original cache
    filter: Option<String>,       // Current filter criteria
    sort_column: Option<usize>,   // Current sort column
    sort_descending: bool,        // Sort direction
}

impl<T> IndexedCache<T>
where
    T: Cacheable + TabularData,
    T::Error: BoxableError,
    for<'a> Row<'static>: From<&'a T>
{
    pub fn new(cache: Rc<RefCell<ObjectCache<T>>>) -> Self {
        IndexedCache {
            cache,
            indices: None,
            filter: None,
            sort_column: None,
            sort_descending: false,
        }
    }
    fn ensure_indices_updated(&mut self) {
        if self.indices.is_none() {
            self.update_indices();
        }
    }

    fn update_indices(&mut self) {
        // Update the indices based on the current filter and sort criteria
        let cache = self.cache.borrow();
        let mut indices: Vec<usize> = (0..cache.len()).collect();

        if let Some(ref filter) = self.filter {
            indices.retain(|&i| cache[i].matches_filter(filter));
        }

        if let Some(column) = self.sort_column {
            let cmp = T::sort_by_column(column, self.sort_descending);
            if let Some(mut cmp) = cmp {
                indices.sort_by(|&a, &b| cmp(&cache[a], &cache[b]));
            }
        }

        self.indices = Some(indices);
    }
}


impl<T> TableDataSource for IndexedCache<T>
where
    T: Cacheable + TabularData,
    T::Error: BoxableError,
    for<'a> Row<'static>: From<&'a T>
{
    fn get_title(&self) -> &'static str {
        T::get_title()
    }
    fn set_filter(&mut self, filter: Option<String>) {
        self.filter = filter;
        self.invalidate();
    }

    fn get_filter(&self) -> Option<String> {
        self.filter.clone()
    }

    fn set_sort_column(&mut self, column: Option<usize>) {
        // If the column is not sortable, do nothing
        if let Some(sort_column) = column {
            if !T::sortable_columns().contains(&sort_column) {
                return;
            }
        }
        if self.sort_column != column {
            self.sort_descending = false;
        } else {
            self.sort_descending = !self.sort_descending;
        }
        self.sort_column = column;
        self.invalidate();
    }

    fn get_sort_setting(&self) -> Option<(usize, bool)> {
        match self.sort_column {
            Some(column) => Some((column, self.sort_descending)),
            None => None,
        }
    }
    fn iter<'a>(&'a mut self) -> Box<dyn Iterator<Item=Row<'static>> + 'a> {
        self.ensure_indices_updated();
        let Some(indices) = &self.indices else {
            panic!("Internal error: No indices found after ensuring indices updated");
        };

        Box::new(indices.iter()
            .map(|idx| {
                let cache = self.cache.borrow();
                let item = &cache[*idx];
                Row::from(item)
            }))
    }

    fn is_empty(&mut self) -> bool {
        self.ensure_indices_updated();
        let Some(indices) = &self.indices else {
            return true;
        };
        indices.is_empty()
    }
    fn column_sizes(&self) -> Vec<Constraint> {
        T::column_sizes()
    }
    fn header_row(&self) -> Vec<&'static str> {
        T::header_row()
    }
    fn invalidate(&mut self) {
        // Invalidate the cache to force update indices
        self.indices = None;
    }

}
