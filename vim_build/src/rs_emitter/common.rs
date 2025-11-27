use super::errors::Result;
use crate::printer::Printer;
use crate::vim_model::{PathOrigin, TypePath, INVENTORY_TYPE_PRIORITY};
use std::collections::HashMap;

pub fn emit_description(printer: &mut dyn Printer, doc_string: &Option<String>) -> Result<()> {
    if let Some(doc) = doc_string {
        for line in doc.trim().split('\n') {
            printer.println(&format!("/// {}", line))?;
        }
    }
    Ok(())
}

/// Maximum paths to keep per managed object type
const MAX_PATHS_PER_MANAGED_OBJECT: usize = 5;

/// Maximum total paths to display
const MAX_PATHS_TOTAL: usize = 10;

/// Get the managed object name from a path origin
fn get_managed_object(origin: &PathOrigin) -> &str {
    match origin {
        PathOrigin::PropertyAccessor { managed_object, .. } => managed_object,
        PathOrigin::MethodOutput { managed_object, .. } => managed_object,
        PathOrigin::MethodInput { managed_object, .. } => managed_object,
    }
}

/// Check if a path origin is a property accessor (vs method)
fn is_property_accessor(origin: &PathOrigin) -> bool {
    matches!(origin, PathOrigin::PropertyAccessor { .. })
}

/// Get the sort key for a path (lower = higher priority).
/// Returns (is_method, inventory_priority, depth) for sorting.
/// Properties always come before methods (highest priority),
/// then inventory type priority, then path depth.
fn path_sort_key(path: &TypePath) -> (bool, usize, usize) {
    let mo = get_managed_object(&path.origin);

    // Properties before methods (false < true in bool ordering) - HIGHEST PRIORITY
    let is_method = !is_property_accessor(&path.origin);

    // Find position in priority list (or max if not found)
    let inventory_priority = INVENTORY_TYPE_PRIORITY
        .iter()
        .position(|&t| t == mo)
        .unwrap_or(usize::MAX);

    // Tertiary sort by path depth (shorter paths first)
    let depth = path.depth();

    (is_method, inventory_priority, depth)
}

/// Sort paths by priority and return a limited selection as borrowed references.
/// Priority: properties before methods, then inventory types, then shorter paths.
/// Also limits paths per managed object to avoid one MO dominating the list.
pub fn select_paths_for_display(paths: &[TypePath]) -> Vec<&TypePath> {
    if paths.is_empty() {
        return Vec::new();
    }

    // Create sorted indices
    let mut indices: Vec<usize> = (0..paths.len()).collect();
    indices.sort_by_key(|&i| path_sort_key(&paths[i]));

    // Count paths per managed object and filter
    let mut mo_counts: HashMap<&str, usize> = HashMap::new();
    let mut result = Vec::with_capacity(MAX_PATHS_TOTAL.min(paths.len()));

    for idx in indices {
        if result.len() >= MAX_PATHS_TOTAL {
            break;
        }

        let path = &paths[idx];
        let mo = get_managed_object(&path.origin);
        let count = mo_counts.entry(mo).or_insert(0);

        if *count < MAX_PATHS_PER_MANAGED_OBJECT {
            *count += 1;
            result.push(path);
        }
    }

    result
}

/// Emit a doc string with paths appended as a bullet list.
/// Paths are sorted and filtered for display.
pub fn emit_description_with_paths(
    printer: &mut dyn Printer,
    doc_string: &Option<String>,
    paths: &[TypePath],
) -> Result<()> {
    // Emit the main description
    if let Some(doc) = doc_string {
        for line in doc.trim().split('\n') {
            printer.println(&format!("/// {}", line))?;
        }
    }

    // Sort and select paths for display
    let total_path_count = paths.len();
    let selected_paths = select_paths_for_display(paths);

    // Emit paths if any exist
    if !selected_paths.is_empty() {
        // Add a blank doc line if we had a description
        if doc_string.is_some() {
            printer.println("///")?;
        }
        printer.println("/// ### How to access")?;
        for path in &selected_paths {
            printer.println(&format!("/// - `{}`", path.to_shorthand()))?;
        }
        // Show count if list was truncated
        if selected_paths.len() < total_path_count {
            printer.println(&format!(
                "/// \n/// *({} of {} paths)*",
                selected_paths.len(),
                total_path_count
            ))?;
        }
    }

    Ok(())
}
