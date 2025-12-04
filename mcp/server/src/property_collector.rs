use crate::resolver::{resolve_path, FieldData, FieldProcessingType, HierarchyError};
use crate::field_data::get_type_fields;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A managed object type with its name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedObjectType {
    pub name: String,
}

/// Property information for a given path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyPathInfo {
    /// The property path in VIM syntax (e.g., "summary.guest.guestFullName")
    pub vim_path: String,
    /// The Rust type of the property (e.g., "Option<String>")
    pub rust_type: String,
    /// Whether the property is optional
    pub is_optional: bool,
    /// Documentation for this property
    pub documentation: Option<String>,
    /// Child fields if this is a complex type (struct/trait)
    pub child_fields: Option<Vec<ChildField>>,
}

/// A child field of a complex type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildField {
    /// The field name in Rust syntax (snake_case)
    pub field_name: String,
    /// The field name in VIM syntax (camelCase)
    pub vim_name: String,
    /// The Rust type of the field
    pub rust_type: String,
    /// Whether the field is optional
    pub is_optional: bool,
    /// Brief description if available
    pub documentation: Option<String>,
}

/// Get the list of all managed object types
/// Returns a list of managed object types sorted alphabetically
pub fn get_managed_object_types() -> Vec<ManagedObjectType> {
    // These are the managed object types that are supported
    // This list is derived from the INVENTORY_TYPES in vim_build
    let managed_types = vec![
        "ClusterComputeResource",
        "ComputeResource",
        "Datacenter",
        "Datastore",
        "DistributedVirtualPortgroup",
        "DistributedVirtualSwitch",
        "Folder",
        "HostSystem",
        "Network",
        "ResourcePool",
        "StoragePod",
        "Task",
        "VirtualApp",
        "VirtualMachine",
        "VmwareDistributedVirtualSwitch",
    ];

    managed_types
        .into_iter()
        .map(|name| ManagedObjectType {
            name: name.to_string(),
        })
        .collect()
}

/// Get property information for a managed object and optional property path
///
/// # Arguments
/// * `managed_object` - The managed object type (e.g., "VirtualMachine")
/// * `property_path` - Optional property path (e.g., "guest.ip_address" or empty for top-level fields)
///
/// # Returns
/// PropertyInfo with details about the property, including child fields if it's a complex type
pub fn get_property_path(
    managed_object: &str,
    property_path: &str,
) -> Result<PropertyPathInfo, HierarchyError> {
    if property_path.is_empty() {
        // Return top-level fields for the managed object
        get_top_level_fields(managed_object)
    } else {
        // Resolve the property path and return information
        get_property_details(managed_object, property_path)
    }
}

fn get_top_level_fields(managed_object: &str) -> Result<PropertyPathInfo, HierarchyError> {
    let Some(child_fields) = get_child_fields_for_type(managed_object) else {
        return Err(HierarchyError::UnsupportedObjectType(managed_object.to_string()));
    };

    Ok(PropertyPathInfo {
        vim_path: "".to_string(),
        rust_type: managed_object.to_string(),
        is_optional: false,
        documentation: Some(format!("Top-level properties for {}", managed_object)),
        child_fields: Some(child_fields),
    })
}

fn get_property_details(
    managed_object: &str,
    property_path: &str,
) -> Result<PropertyPathInfo, HierarchyError> {
    let field_data: FieldData = resolve_path(managed_object, property_path)?;

    // Check if this property has child fields (i.e., it's a struct or trait)
    let child_fields = match &field_data.processing_type {
        crate::resolver::FieldProcessingType::Struct | crate::resolver::FieldProcessingType::Trait => {
            // Try to get child fields by attempting to resolve sub-properties
            get_child_fields_for_type(&field_data.type_name)
        }
        _ => None,
    };

    Ok(PropertyPathInfo {
        vim_path: field_data.vim_path.clone(),
        rust_type: field_data.data_type.clone(),
        is_optional: field_data.is_optional,
        documentation: field_data.doc.map(|s| s.to_string()),
        child_fields: child_fields,
    })
}

fn get_child_fields_for_type(type_name: &str) -> Option<Vec<ChildField>> {
    let Ok(type_fields) = get_type_fields(type_name) else {
        return None;
    };

    let mut child_fields = Vec::new();
    for (field_name, field_data) in type_fields.entries() {
        child_fields.push(ChildField {
            field_name: field_name.to_string(),
            vim_name: field_data.path_segment.to_string(),
            rust_type: field_data.type_decl.to_string(),
            is_optional: field_data.is_optional,
            documentation: field_data.doc.map(|s| s.to_string()),
        });
    }

    if child_fields.is_empty() {
        None
    } else {
        Some(child_fields)
    }
}

/// Generate a property tree for a managed object type
///
/// Returns a flat list of property paths with their types.
/// Each line shows the full dot-separated path from the root.
/// Optional-ness is propagated: if a parent is optional, all children are shown as Option.
///
/// # Arguments
/// * `managed_object` - The managed object type (e.g., "VirtualMachine")
/// * `start_path` - Optional starting path to show a subtree (e.g., "config.hardware")
/// * `max_depth` - Maximum depth to traverse (1-5)
pub fn get_property_tree(
    managed_object: &str,
    start_path: &str,
    max_depth: usize,
) -> Result<String, HierarchyError> {
    // Clamp depth to 1-5 range
    let max_depth = max_depth.clamp(1, 5);

    // Determine the starting type and whether it's optional
    let (starting_type, base_path, parent_optional) = if start_path.is_empty() {
        (managed_object.to_string(), String::new(), false)
    } else {
        // Resolve the path to get the type at that location
        let field_data = resolve_path(managed_object, start_path)?;

        // Check if the resolved type can be expanded
        match field_data.processing_type {
            FieldProcessingType::Struct | FieldProcessingType::Trait => {
                if field_data.type_name.is_empty() {
                    return Err(HierarchyError::NoSubPropertiesAvailable(start_path.to_string()));
                }
                (
                    field_data.type_name.to_string(),
                    start_path.to_string(),
                    field_data.is_optional,
                )
            }
            _ => {
                return Err(HierarchyError::NoSubPropertiesAvailable(start_path.to_string()));
            }
        }
    };

    let type_fields = get_type_fields(&starting_type)?;

    let mut output = String::new();
    let mut visited = HashSet::new();
    visited.insert(starting_type.clone());

    build_tree_from_type(
        &mut output,
        type_fields,
        0,
        max_depth,
        &mut visited,
        &base_path,
        parent_optional,
    );

    Ok(output)
}

/// Recursively build flat property paths output
///
/// # Arguments
/// * `output` - The string to append output to
/// * `fields` - The fields at the current level
/// * `depth` - Current recursion depth (0-based)
/// * `max_depth` - Maximum depth to traverse
/// * `visited` - Set of visited type names (for cycle detection)
/// * `current_path` - The accumulated path so far (e.g., "config.hardware")
/// * `parent_optional` - Whether any ancestor in the path is optional
fn build_tree_from_type(
    output: &mut String,
    fields: &phf::Map<&'static str, crate::resolver::NodeData>,
    depth: usize,
    max_depth: usize,
    visited: &mut HashSet<String>,
    current_path: &str,
    parent_optional: bool,
) {
    // Collect and sort fields alphabetically for consistent output
    let mut field_entries: Vec<_> = fields.entries().collect();
    field_entries.sort_by_key(|(name, _)| *name);

    for (field_name, node_data) in field_entries.iter() {
        // Build the full path for this field
        let full_path = if current_path.is_empty() {
            field_name.to_string()
        } else {
            format!("{}.{}", current_path, field_name)
        };

        // Effective optional: true if this field is optional OR any parent was optional
        let effective_optional = parent_optional || node_data.is_optional;

        // Format the type, wrapping in Option if effectively optional
        let type_str = if effective_optional {
            format!("Option<{}>", node_data.type_decl)
        } else {
            node_data.type_decl.to_string()
        };

        output.push_str(&format!("{}: {}\n", full_path, type_str));

        // Recursively expand struct/trait types if within depth limit
        if depth + 1 < max_depth {
            let should_expand = matches!(
                node_data.processing_type,
                FieldProcessingType::Struct | FieldProcessingType::Trait
            );

            if should_expand && !node_data.type_name.is_empty() {
                // Check for cycles - only expand if not already in this branch
                if !visited.contains(node_data.type_name) {
                    if let Ok(child_fields) = get_type_fields(node_data.type_name) {
                        visited.insert(node_data.type_name.to_string());

                        build_tree_from_type(
                            output,
                            child_fields,
                            depth + 1,
                            max_depth,
                            visited,
                            &full_path,
                            effective_optional,
                        );

                        visited.remove(node_data.type_name);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_paths_format() {
        // Test that output contains full dot-separated paths, not indented tree
        let result = get_property_tree("VirtualMachine", "", 2).unwrap();

        // Should contain flat paths like "name: ..." not "├─name: ..."
        assert!(
            !result.contains("├─"),
            "Output should not contain box-drawing characters"
        );
        assert!(
            !result.contains("└─"),
            "Output should not contain box-drawing characters"
        );

        // Should contain property paths in format "path: Type"
        assert!(
            result.contains("name: "),
            "Output should contain 'name' property"
        );

        // With depth 2, nested paths should use dots
        // Check for a nested path pattern (at least one dot in path)
        let has_nested_path = result.lines().any(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            parts.first().map(|p| p.contains('.')).unwrap_or(false)
        });
        assert!(has_nested_path, "Output should contain nested paths with dots");
    }

    #[test]
    fn test_depth_limit_one() {
        // With depth=1, should only show immediate children (no nested paths)
        let result = get_property_tree("VirtualMachine", "", 1).unwrap();

        // No line should contain a dot in the path portion (before the colon)
        for line in result.lines() {
            let path = line.split(':').next().unwrap_or("");
            assert!(
                !path.contains('.'),
                "With depth=1, paths should not contain dots: {}",
                line
            );
        }
    }

    #[test]
    fn test_depth_limit_two() {
        // With depth=2, should show one level of nesting (paths like "config.xxx")
        let result_depth_1 = get_property_tree("VirtualMachine", "", 1).unwrap();
        let result_depth_2 = get_property_tree("VirtualMachine", "", 2).unwrap();

        // Depth 2 should have more lines than depth 1
        let lines_1: Vec<_> = result_depth_1.lines().collect();
        let lines_2: Vec<_> = result_depth_2.lines().collect();

        assert!(
            lines_2.len() > lines_1.len(),
            "Depth 2 ({} lines) should have more output than depth 1 ({} lines)",
            lines_2.len(),
            lines_1.len()
        );

        // Depth 2 should have nested paths (with dots)
        let has_nested = result_depth_2.lines().any(|line| {
            line.split(':').next().unwrap_or("").contains('.')
        });
        assert!(has_nested, "Depth 2 should contain nested paths");
    }

    #[test]
    fn test_depth_clamped_to_valid_range() {
        // Depth 0 should be clamped to 1
        let result_0 = get_property_tree("VirtualMachine", "", 0).unwrap();
        let result_1 = get_property_tree("VirtualMachine", "", 1).unwrap();
        assert_eq!(result_0, result_1, "Depth 0 should be clamped to 1");

        // Depth 10 should be clamped to 5
        let result_10 = get_property_tree("VirtualMachine", "", 10).unwrap();
        let result_5 = get_property_tree("VirtualMachine", "", 5).unwrap();
        assert_eq!(result_10, result_5, "Depth 10 should be clamped to 5");
    }

    #[test]
    fn test_optional_propagation() {
        // VirtualMachine.guest is optional
        // VirtualMachine.guest.guest_state is mandatory within GuestInfo
        // But since guest is optional, guest.guest_state should show as Option

        // First verify that 'guest' itself is optional at depth 1
        let result_depth_1 = get_property_tree("VirtualMachine", "", 1).unwrap();
        let guest_line = result_depth_1
            .lines()
            .find(|line| line.starts_with("guest:"))
            .expect("Should have 'guest' property");
        assert!(
            guest_line.contains("Option<"),
            "'guest' should be optional: {}",
            guest_line
        );

        // Now check that nested fields under guest also show as Option
        let result_depth_2 = get_property_tree("VirtualMachine", "", 2).unwrap();
        let guest_state_line = result_depth_2
            .lines()
            .find(|line| line.starts_with("guest.guest_state:"));

        if let Some(line) = guest_state_line {
            // guest_state should be wrapped in Option because guest is optional
            assert!(
                line.contains("Option<"),
                "guest.guest_state should be Option because parent 'guest' is optional: {}",
                line
            );
        }
    }

    #[test]
    fn test_start_path_subtree() {
        // When starting from a subtree, paths should be relative to that point
        let result = get_property_tree("VirtualMachine", "summary", 1).unwrap();

        // Paths should start with "summary." prefix
        for line in result.lines() {
            let path = line.split(':').next().unwrap_or("");
            assert!(
                path.starts_with("summary."),
                "With start_path='summary', paths should start with 'summary.': {}",
                line
            );
        }
    }

    #[test]
    fn test_start_path_propagates_optional() {
        // If we start from an optional path, all children should be optional
        // VirtualMachine.guest is optional

        let result = get_property_tree("VirtualMachine", "guest", 1).unwrap();

        // All paths should show as Option since 'guest' is optional
        for line in result.lines() {
            assert!(
                line.contains("Option<"),
                "All children of optional 'guest' should be Option: {}",
                line
            );
        }
    }

    #[test]
    fn test_managed_object_types_list() {
        let types = get_managed_object_types();

        // Should contain key types
        let type_names: Vec<_> = types.iter().map(|t| t.name.as_str()).collect();
        assert!(type_names.contains(&"VirtualMachine"));
        assert!(type_names.contains(&"HostSystem"));
        assert!(type_names.contains(&"Datacenter"));
        assert!(type_names.contains(&"Folder"));

        // Should be non-empty
        assert!(!types.is_empty(), "Should have managed object types");
    }

    #[test]
    fn test_invalid_managed_object() {
        let result = get_property_tree("NotARealType", "", 1);
        assert!(result.is_err(), "Should error for invalid managed object type");
    }

    #[test]
    fn test_invalid_start_path() {
        let result = get_property_tree("VirtualMachine", "not.a.real.path", 1);
        assert!(result.is_err(), "Should error for invalid property path");
    }
}
