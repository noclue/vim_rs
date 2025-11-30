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

/// Maximum depth for property tree traversal
const MAX_TREE_DEPTH: usize = 5;

/// Generate a property tree for a managed object type
///
/// Returns a markdown-formatted tree showing all properties up to 5 levels deep.
/// Uses box drawing characters for clean visual presentation.
///
/// # Arguments
/// * `managed_object` - The managed object type (e.g., "VirtualMachine")
/// * `start_path` - Optional starting path to show a subtree (e.g., "config.hardware")
pub fn get_property_tree(managed_object: &str, start_path: &str) -> Result<String, HierarchyError> {
    // Determine the starting type - either the managed object or resolved from path
    let (starting_type, root_type_str) = if start_path.is_empty() {
        (managed_object.to_string(), managed_object.to_string())
    } else {
        // Resolve the path to get the type at that location
        let field_data = resolve_path(managed_object, start_path)?;
        
        // Check if the resolved type can be expanded
        match field_data.processing_type {
            FieldProcessingType::Struct | FieldProcessingType::Trait => {
                if field_data.type_name.is_empty() {
                    return Err(HierarchyError::NoSubPropertiesAvailable(start_path.to_string()));
                }
                (field_data.type_name.to_string(), field_data.data_type)
            }
            _ => {
                return Err(HierarchyError::NoSubPropertiesAvailable(start_path.to_string()));
            }
        }
    };

    let type_fields = get_type_fields(&starting_type)?;

    let mut output = String::new();
    
    // Show the root type
    output.push_str(&format!("{}\n", root_type_str));
    
    let mut visited = HashSet::new();
    visited.insert(starting_type.clone());

    build_tree_from_type(&mut output, type_fields, 0, &mut visited, "");

    Ok(output)
}

/// Recursively build the tree output with box drawing characters
fn build_tree_from_type(
    output: &mut String,
    fields: &phf::Map<&'static str, crate::resolver::NodeData>,
    depth: usize,
    visited: &mut HashSet<String>,
    prefix: &str,
) {
    // Collect and sort fields alphabetically for consistent output
    let mut field_entries: Vec<_> = fields.entries().collect();
    field_entries.sort_by_key(|(name, _)| *name);

    let total = field_entries.len();

    for (idx, (field_name, node_data)) in field_entries.iter().enumerate() {
        let is_last = idx == total - 1;
        
        // Box drawing characters
        let branch = if is_last { "└─" } else { "├─" };
        let child_prefix = if is_last { "  " } else { "│ " };

        // Format the type, wrapping in Option if needed
        let type_str = if node_data.is_optional {
            format!("Option<{}>", node_data.type_decl)
        } else {
            node_data.type_decl.to_string()
        };

        output.push_str(&format!("{}{}{}: {}\n", prefix, branch, field_name, type_str));

        // Recursively expand struct/trait types if within depth limit
        if depth + 1 < MAX_TREE_DEPTH {
            let should_expand = matches!(
                node_data.processing_type,
                FieldProcessingType::Struct | FieldProcessingType::Trait
            );

            if should_expand && !node_data.type_name.is_empty() {
                // Check for cycles - only expand if not already in this branch
                if !visited.contains(node_data.type_name) {
                    if let Ok(child_fields) = get_type_fields(node_data.type_name) {
                        visited.insert(node_data.type_name.to_string());
                        
                        let new_prefix = format!("{}{}", prefix, child_prefix);
                        build_tree_from_type(output, child_fields, depth + 1, visited, &new_prefix);
                        
                        visited.remove(node_data.type_name);
                    }
                }
            }
        }
    }
}
