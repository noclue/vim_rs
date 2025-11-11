use crate::resolver::{resolve_path, FieldData, HierarchyError};
use crate::field_data::lookup_field_data;
use serde::{Deserialize, Serialize};

/// A managed object type with its name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedObjectType {
    pub name: String,
}

/// Property information for a given path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyInfo {
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
pub fn get_property_info(
    managed_object: &str,
    property_path: &str,
) -> Result<PropertyInfo, HierarchyError> {
    if property_path.is_empty() {
        // Return top-level fields for the managed object
        get_top_level_fields(managed_object)
    } else {
        // Resolve the property path and return information
        get_property_details(managed_object, property_path)
    }
}

fn get_top_level_fields(managed_object: &str) -> Result<PropertyInfo, HierarchyError> {
    // Get all fields for this managed object by using the lookup_field_data function
    // We need to iterate through the CLASS_FIELDS to find all fields for this object
    // Since we can't directly access the map, we'll try common fields and collect them

    // For now, we'll use a helper approach: try to resolve an empty path which should give us
    // information about the object itself
    let mut child_fields = Vec::new();

    // Common properties that most managed objects have
    let common_props = vec![
        "name", "parent", "config", "summary", "runtime", "guest",
        "network", "datastore", "vm", "host", "resourcePool", "snapshot",
        "storage", "availableField", "value", "alarmActionsEnabled",
        "tag", "customValue", "overallStatus", "configStatus", "configIssue",
        "effectiveRole", "permission", "recentTask", "declaredAlarmState",
        "triggeredAlarmState", "disabledMethod",
    ];

    for prop in common_props {
        if let Ok(node) = lookup_field_data(managed_object, prop) {
            child_fields.push(ChildField {
                field_name: prop.replace(".", "_"),
                vim_name: node.path_segment.to_string(),
                rust_type: node.type_decl.to_string(),
                is_optional: node.is_optional,
                documentation: node.doc.map(|s| s.to_string()),
            });
        }
    }

    if child_fields.is_empty() {
        return Err(HierarchyError::UnsupportedObjectType(managed_object.to_string()));
    }

    Ok(PropertyInfo {
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
) -> Result<PropertyInfo, HierarchyError> {
    let field_data: FieldData = resolve_path(managed_object, property_path)?;

    // Check if this property has child fields (i.e., it's a struct or trait)
    let child_fields = match &field_data.processing_type {
        crate::resolver::FieldProcessingType::Struct | crate::resolver::FieldProcessingType::Trait => {
            // Try to get child fields by attempting to resolve sub-properties
            get_child_fields_for_type(&field_data)
        }
        _ => None,
    };

    Ok(PropertyInfo {
        vim_path: field_data.vim_path.clone(),
        rust_type: field_data.data_type.clone(),
        is_optional: field_data.is_optional,
        documentation: field_data.doc.map(|s| s.to_string()),
        child_fields,
    })
}

fn get_child_fields_for_type(field_data: &FieldData) -> Option<Vec<ChildField>> {
    // Extract the type name from the Rust type
    // Types look like: "Option<vim_rs::types::structs::VirtualMachineGuestSummary>"
    // or "vim_rs::types::structs::VirtualMachineConfigInfo"
    let type_str = &field_data.data_type;

    // Remove Option<> wrapper if present
    let inner_type = if type_str.starts_with("Option<") {
        &type_str[7..type_str.len() - 1]
    } else {
        type_str
    };

    // Extract the struct name from the fully qualified path
    let type_name = inner_type
        .split("::")
        .last()
        .and_then(|s| {
            // Remove any trait wrapper like "Box<dyn ...Trait>>"
            if s.contains("Trait>>") {
                s.split("Trait>>").next()
            } else if s.ends_with(">") {
                // Handle Vec<...> and other generic types
                None
            } else {
                Some(s)
            }
        })?;

    // Try to get fields for this type
    let mut child_fields = Vec::new();

    // Try common field names for structs
    let common_fields = vec![
        "name", "type", "value", "key", "description", "summary",
        "label", "unit_info", "rollup_type", "stats_type",
        "guest_full_name", "guest_id", "ip_address", "host_name",
        "tools_status", "tools_version", "tools_running_status",
        "tools_version_status", "tools_version_status2",
    ];

    for field in common_fields {
        if let Ok(node) = lookup_field_data(type_name, field) {
            child_fields.push(ChildField {
                field_name: field.to_string(),
                vim_name: node.path_segment.to_string(),
                rust_type: node.type_decl.to_string(),
                is_optional: node.is_optional,
                documentation: node.doc.map(|s| s.to_string()),
            });
        }
    }

    if child_fields.is_empty() {
        None
    } else {
        Some(child_fields)
    }
}
