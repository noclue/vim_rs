//! Path computation for VIM API model types.
//!
//! This module computes navigation paths from ManagedObject properties and methods
//! to each struct and enum type in the VIM API model. These paths help users understand
//! how to reach and use specific types in the API.
//!
//! The path types (TypePath, PathOrigin, PathStep) are defined in types.rs for
//! easier access by external emitters.

use super::{DataType, EmitMode, HttpMethod, Model, PathOrigin, PathStep, TypePath};
use indexmap::IndexMap;
use std::collections::HashSet;

/// Types with massive inheritance trees that shouldn't have descendant traversal.
/// These are base types with hundreds of descendants where downcast paths aren't useful.
pub const SKIP_DESCENDANTS_TYPES: &[&str] = &[
    "Any",
    "DataObject",
    "MethodFault",
    "Issue",
    "AgencyIssue",
    "AgentIssue",
];

/// Priority order for inventory types when sorting paths.
/// Paths from these managed objects are shown first, in this order.
pub const INVENTORY_TYPE_PRIORITY: &[&str] = &[
    "VirtualMachine",
    "HostSystem",
    "Task",
    "Network",
    "DistributedVirtualPortgroup",
    "VmwareDistributedVirtualSwitch",
    "Folder",
    "Datacenter",
    "ResourcePool",
    "Datastore",
    "StoragePod",
    "ComputeResource",
    "ClusterComputeResource",
    "DistributedVirtualSwitch",
    "VirtualApp",
];

/// Configuration for path computation.
#[derive(Debug, Clone)]
pub struct PathComputeConfig {
    /// Maximum depth of paths to compute (default: 10)
    pub max_depth: usize,
    /// Maximum paths to store per type (default: 50)
    pub max_paths_per_type: usize,
    /// Types to exclude from path computation entirely
    pub exclude_types: Vec<String>,
    /// Types to skip descendant traversal for (in addition to SKIP_DESCENDANTS_TYPES)
    pub skip_descendants_for: Vec<String>,
}

impl Default for PathComputeConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            max_paths_per_type: 10,
            exclude_types: vec![],
            skip_descendants_for: vec![],
        }
    }
}

/// Result type for path computation (internal use)
type PathResult<T> = std::result::Result<T, super::Error>;

/// Compute paths to all struct and enum types in the model.
///
/// This function traverses the API model starting from ManagedObject properties
/// and methods, computing navigation paths to each reachable type.
pub fn compute_paths(model: &mut Model, config: &PathComputeConfig) -> PathResult<()> {
    // Collect paths for structs
    let mut struct_paths: IndexMap<String, Vec<TypePath>> = IndexMap::new();
    // Collect paths for enums
    let mut enum_paths: IndexMap<String, Vec<TypePath>> = IndexMap::new();

    // Initialize empty path vectors for all types
    for name in model.structs.keys() {
        struct_paths.insert(name.clone(), Vec::new());
    }
    for name in model.enums.keys() {
        enum_paths.insert(name.clone(), Vec::new());
    }

    // Process each managed object
    for (mo_name, mo) in &model.managed_objects {
        for method in &mo.methods {
            match method.http_method {
                HttpMethod::Get => {
                    // Property accessor - output type
                    if let Some(ref output_type) = method.output {
                        let origin = PathOrigin::PropertyAccessor {
                            managed_object: mo_name.clone(),
                            property_name: method.name.clone(),
                        };
                        let is_optional = method.optional_response;
                        traverse_type(
                            model,
                            config,
                            output_type,
                            origin,
                            Vec::new(),
                            is_optional,
                            &mut struct_paths,
                            &mut enum_paths,
                            &mut HashSet::new(),
                            0,
                        );
                    }
                }
                HttpMethod::Post => {
                    // Method - process both input and output types
                    
                    // Output type
                    if let Some(ref output_type) = method.output {
                        let origin = PathOrigin::MethodOutput {
                            managed_object: mo_name.clone(),
                            method_name: method.name.clone(),
                        };
                        traverse_type(
                            model,
                            config,
                            output_type,
                            origin,
                            Vec::new(),
                            method.optional_response,
                            &mut struct_paths,
                            &mut enum_paths,
                            &mut HashSet::new(),
                            0,
                        );
                    }

                    // Input type - look up the request type to find parameters
                    if let Some(ref input_type) = method.input {
                        if let DataType::Reference(ref request_type_name) = input_type {
                            // Get the request type struct to enumerate its fields as parameters
                            if let Some(request_struct) = model.request_types.get(request_type_name)
                            {
                                let request_struct = request_struct.borrow();
                                for (param_name, field) in &request_struct.fields {
                                    let origin = PathOrigin::MethodInput {
                                        managed_object: mo_name.clone(),
                                        method_name: method.name.clone(),
                                        parameter_name: param_name.clone(),
                                    };
                                    traverse_type(
                                        model,
                                        config,
                                        &field.vim_type,
                                        origin,
                                        Vec::new(),
                                        field.optional,
                                        &mut struct_paths,
                                        &mut enum_paths,
                                        &mut HashSet::new(),
                                        0,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Store computed paths on types (emitters will sort and limit as needed)
    for (name, paths) in struct_paths {
        if let Some(struct_cell) = model.structs.get(&name) {
            let mut struct_ref = struct_cell.borrow_mut();
            struct_ref.paths = paths;
        }
    }

    for (name, paths) in enum_paths {
        if let Some(enum_ref) = model.enums.get_mut(&name) {
            enum_ref.paths = paths;
        }
    }

    Ok(())
}

/// Collect all descendants of a type (not just direct children).
/// This prevents chained downcasts like `→Parent→Child` and instead gives `→Child` directly.
fn collect_all_descendants(model: &Model, type_name: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut queue = Vec::new();

    // Start with direct children
    if let Some(struct_cell) = model.structs.get(type_name) {
        let struct_ref = struct_cell.borrow();
        queue.extend(struct_ref.children.clone());
    }

    // BFS to collect all descendants
    while let Some(current) = queue.pop() {
        result.push(current.clone());
        if let Some(struct_cell) = model.structs.get(&current) {
            let struct_ref = struct_cell.borrow();
            queue.extend(struct_ref.children.clone());
        }
    }

    result
}

/// Traverse a type and its fields to collect paths.
#[allow(clippy::too_many_arguments)]
fn traverse_type(
    model: &Model,
    config: &PathComputeConfig,
    data_type: &DataType,
    origin: PathOrigin,
    current_steps: Vec<PathStep>,
    is_origin_optional: bool,
    struct_paths: &mut IndexMap<String, Vec<TypePath>>,
    enum_paths: &mut IndexMap<String, Vec<TypePath>>,
    visited: &mut HashSet<String>,
    depth: usize,
) {
    // Check depth limit
    if depth >= config.max_depth {
        return;
    }

    match data_type {
        DataType::Array(inner_type) => {
            // For arrays at the origin level, we add an implicit array marker
            // The actual field step will be added when we recurse
            traverse_type(
                model,
                config,
                inner_type,
                origin,
                current_steps,
                is_origin_optional,
                struct_paths,
                enum_paths,
                visited,
                depth,
            );
        }
        DataType::Reference(type_name) => {
            // Check exclusion list
            if config.exclude_types.contains(type_name) {
                return;
            }

            // Check for cycles
            if visited.contains(type_name) {
                return;
            }

            // Is this a struct or enum?
            if let Some(paths) = struct_paths.get_mut(type_name) {
                // Use higher internal limit during collection (will sort & truncate later)
                // This ensures high-priority paths aren't skipped by early termination
                let collection_limit = config.max_paths_per_type * 10;
                if paths.len() >= collection_limit {
                    return;
                }

                // Record path to this type
                let path = TypePath {
                    origin: origin.clone(),
                    steps: current_steps.clone(),
                };
                paths.push(path);

                // Continue traversing into the struct's fields
                visited.insert(type_name.clone());

                if let Some(struct_cell) = model.structs.get(type_name) {
                    let struct_ref = struct_cell.borrow();

                    // Skip if this type is not emitted
                    if struct_ref.emit_mode.is_skip() {
                        visited.remove(type_name);
                        return;
                    }

                    // If this struct has children (is polymorphic), also create paths to descendants
                    // Skip types with massive inheritance trees (Any, DataObject, MethodFault, etc.)
                    let skip_descendants = SKIP_DESCENDANTS_TYPES.contains(&type_name.as_str())
                        || config.skip_descendants_for.iter().any(|t| t == type_name);

                    if !skip_descendants
                        && struct_ref.has_children()
                        && struct_ref.emit_mode == EmitMode::Emit
                    {
                        // Collect ALL descendants (not just direct children) to avoid chained downcasts
                        let all_descendants = collect_all_descendants(model, type_name);

                        for descendant_name in all_descendants {
                            // Skip if descendant is not emitted, and check if it has children (trait cast)
                            let descendant_has_children =
                                if let Some(desc_cell) = model.structs.get(&descendant_name) {
                                    let desc_ref = desc_cell.borrow();
                                    if desc_ref.emit_mode.is_skip() {
                                        continue;
                                    }
                                    desc_ref.has_children()
                                } else {
                                    false
                                };

                            let mut desc_steps = current_steps.clone();
                            desc_steps.push(PathStep::Downcast {
                                to_type: descendant_name.clone(),
                                is_trait_cast: descendant_has_children,
                            });

                            // Record path to this descendant
                            if let Some(paths) = struct_paths.get_mut(&descendant_name) {
                                let collection_limit = config.max_paths_per_type * 10;
                                if paths.len() < collection_limit {
                                    let path = TypePath {
                                        origin: origin.clone(),
                                        steps: desc_steps.clone(),
                                    };
                                    paths.push(path);
                                }
                            }

                            // Also traverse fields of the descendant (without adding another downcast)
                            if let Some(desc_cell) = model.structs.get(&descendant_name) {
                                let desc_ref = desc_cell.borrow();
                                if !visited.contains(&descendant_name) {
                                    visited.insert(descendant_name.clone());
                                    for (field_name, field) in &desc_ref.fields {
                                        let mut field_steps = desc_steps.clone();
                                        let is_array = matches!(field.vim_type, DataType::Array(_));
                                        field_steps.push(PathStep::Field {
                                            field_name: field_name.clone(),
                                            is_optional: field.optional,
                                            is_boxed: field.require_box,
                                            is_array,
                                        });

                                        let inner_type = match &field.vim_type {
                                            DataType::Array(inner) => inner.as_ref(),
                                            _ => &field.vim_type,
                                        };

                                        traverse_type(
                                            model,
                                            config,
                                            inner_type,
                                            origin.clone(),
                                            field_steps,
                                            is_origin_optional,
                                            struct_paths,
                                            enum_paths,
                                            visited,
                                            depth + 1,
                                        );
                                    }
                                    visited.remove(&descendant_name);
                                }
                            }
                        }
                    }

                    // Traverse fields
                    for (field_name, field) in &struct_ref.fields {
                        let mut field_steps = current_steps.clone();
                        let is_array = matches!(field.vim_type, DataType::Array(_));
                        field_steps.push(PathStep::Field {
                            field_name: field_name.clone(),
                            is_optional: field.optional,
                            is_boxed: field.require_box,
                            is_array,
                        });

                        // Get the inner type for arrays
                        let inner_type = match &field.vim_type {
                            DataType::Array(inner) => inner.as_ref(),
                            _ => &field.vim_type,
                        };

                        traverse_type(
                            model,
                            config,
                            inner_type,
                            origin.clone(),
                            field_steps,
                            is_origin_optional,
                            struct_paths,
                            enum_paths,
                            visited,
                            depth + 1,
                        );
                    }
                }

                visited.remove(type_name);
            } else if let Some(paths) = enum_paths.get_mut(type_name) {
                // Use higher internal limit during collection (will sort & truncate later)
                let collection_limit = config.max_paths_per_type * 10;
                if paths.len() >= collection_limit {
                    return;
                }
                // Enums are terminal - just record the path
                let path = TypePath {
                    origin: origin.clone(),
                    steps: current_steps.clone(),
                };
                paths.push(path);
            }
        }
        // Primitive types don't get paths
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_accessor_shorthand() {
        let path = TypePath {
            origin: PathOrigin::PropertyAccessor {
                managed_object: "VirtualMachine".to_string(),
                property_name: "config".to_string(),
            },
            steps: vec![],
        };
        assert_eq!(path.to_shorthand(), "VirtualMachine::config");
    }

    #[test]
    fn test_nested_field_shorthand() {
        let path = TypePath {
            origin: PathOrigin::PropertyAccessor {
                managed_object: "VirtualMachine".to_string(),
                property_name: "config".to_string(),
            },
            steps: vec![PathStep::Field {
                field_name: "hardware".to_string(),
                is_optional: false,
                is_boxed: false,
                is_array: false,
            }],
        };
        assert_eq!(path.to_shorthand(), "VirtualMachine::config.hardware");
    }

    #[test]
    fn test_camel_case_to_snake_case() {
        let path = TypePath {
            origin: PathOrigin::PropertyAccessor {
                managed_object: "VirtualMachine".to_string(),
                property_name: "configInfo".to_string(),
            },
            steps: vec![PathStep::Field {
                field_name: "cpuAllocation".to_string(),
                is_optional: false,
                is_boxed: false,
                is_array: false,
            }],
        };
        // camelCase converted to snake_case
        assert_eq!(path.to_shorthand(), "VirtualMachine::config_info.cpu_allocation");
    }

    #[test]
    fn test_optional_field_shorthand() {
        let path = TypePath {
            origin: PathOrigin::PropertyAccessor {
                managed_object: "VirtualMachine".to_string(),
                property_name: "config".to_string(),
            },
            steps: vec![PathStep::Field {
                field_name: "hardware".to_string(),
                is_optional: true,
                is_boxed: false,
                is_array: false,
            }],
        };
        assert_eq!(path.to_shorthand(), "VirtualMachine::config.hardware?");
    }

    #[test]
    fn test_array_field_shorthand() {
        let path = TypePath {
            origin: PathOrigin::PropertyAccessor {
                managed_object: "VirtualMachine".to_string(),
                property_name: "config".to_string(),
            },
            steps: vec![
                PathStep::Field {
                    field_name: "hardware".to_string(),
                    is_optional: false,
                    is_boxed: false,
                    is_array: false,
                },
                PathStep::Field {
                    field_name: "device".to_string(),
                    is_optional: false,
                    is_boxed: false,
                    is_array: true,
                },
            ],
        };
        assert_eq!(
            path.to_shorthand(),
            "VirtualMachine::config.hardware.device[*]"
        );
    }

    #[test]
    fn test_struct_downcast_shorthand() {
        // Struct downcast (target has no children) uses →
        let path = TypePath {
            origin: PathOrigin::PropertyAccessor {
                managed_object: "VirtualMachine".to_string(),
                property_name: "config".to_string(),
            },
            steps: vec![
                PathStep::Field {
                    field_name: "hardware".to_string(),
                    is_optional: false,
                    is_boxed: false,
                    is_array: false,
                },
                PathStep::Field {
                    field_name: "device".to_string(),
                    is_optional: false,
                    is_boxed: false,
                    is_array: true,
                },
                PathStep::Downcast {
                    to_type: "VirtualEthernetCard".to_string(),
                    is_trait_cast: false,
                },
            ],
        };
        assert_eq!(
            path.to_shorthand(),
            "VirtualMachine::config.hardware.device[*]→VirtualEthernetCard"
        );
    }

    #[test]
    fn test_trait_downcast_shorthand() {
        // Trait cast (target has children) uses ⇒ and appends Trait
        let path = TypePath {
            origin: PathOrigin::PropertyAccessor {
                managed_object: "VirtualMachine".to_string(),
                property_name: "config".to_string(),
            },
            steps: vec![
                PathStep::Field {
                    field_name: "hardware".to_string(),
                    is_optional: false,
                    is_boxed: false,
                    is_array: false,
                },
                PathStep::Field {
                    field_name: "device".to_string(),
                    is_optional: false,
                    is_boxed: false,
                    is_array: true,
                },
                PathStep::Downcast {
                    to_type: "VirtualDevice".to_string(),
                    is_trait_cast: true,
                },
            ],
        };
        assert_eq!(
            path.to_shorthand(),
            "VirtualMachine::config.hardware.device[*]⇒VirtualDeviceTrait"
        );
    }

    #[test]
    fn test_method_input_shorthand() {
        let path = TypePath {
            origin: PathOrigin::MethodInput {
                managed_object: "VirtualMachine".to_string(),
                method_name: "applyEvcModeVmTask".to_string(),
                parameter_name: "featureMask".to_string(),
            },
            steps: vec![],
        };
        // Method and param names converted to snake_case
        assert_eq!(
            path.to_shorthand(),
            "VirtualMachine::apply_evc_mode_vm_task(feature_mask)"
        );
    }

    #[test]
    fn test_method_output_shorthand() {
        let path = TypePath {
            origin: PathOrigin::MethodOutput {
                managed_object: "VirtualMachine".to_string(),
                method_name: "reconfigureVm".to_string(),
            },
            steps: vec![],
        };
        // Method name converted to snake_case
        assert_eq!(path.to_shorthand(), "VirtualMachine::reconfigure_vm()");
    }

    #[test]
    fn test_property_collector_path() {
        let path = TypePath {
            origin: PathOrigin::PropertyAccessor {
                managed_object: "VirtualMachine".to_string(),
                property_name: "config".to_string(),
            },
            steps: vec![
                PathStep::Field {
                    field_name: "hardware".to_string(),
                    is_optional: false,
                    is_boxed: false,
                    is_array: false,
                },
                PathStep::Field {
                    field_name: "device".to_string(),
                    is_optional: false,
                    is_boxed: false,
                    is_array: true,
                },
                PathStep::Downcast {
                    to_type: "VirtualEthernetCard".to_string(),
                    is_trait_cast: false,
                },
            ],
        };
        assert_eq!(
            path.property_collector_path(),
            Some("config.hardware.device".to_string())
        );
    }

    #[test]
    fn test_property_collector_path_snake_case() {
        let path = TypePath {
            origin: PathOrigin::PropertyAccessor {
                managed_object: "VirtualMachine".to_string(),
                property_name: "configInfo".to_string(),
            },
            steps: vec![PathStep::Field {
                field_name: "cpuAllocation".to_string(),
                is_optional: false,
                is_boxed: false,
                is_array: false,
            }],
        };
        // PropertyCollector paths also use snake_case
        assert_eq!(
            path.property_collector_path(),
            Some("config_info.cpu_allocation".to_string())
        );
    }

    #[test]
    fn test_property_collector_path_method_returns_none() {
        let path = TypePath {
            origin: PathOrigin::MethodInput {
                managed_object: "VirtualMachine".to_string(),
                method_name: "reconfigure".to_string(),
                parameter_name: "spec".to_string(),
            },
            steps: vec![],
        };
        assert_eq!(path.property_collector_path(), None);
    }

    #[test]
    fn test_path_depth() {
        let path = TypePath {
            origin: PathOrigin::PropertyAccessor {
                managed_object: "VirtualMachine".to_string(),
                property_name: "config".to_string(),
            },
            steps: vec![
                PathStep::Field {
                    field_name: "hardware".to_string(),
                    is_optional: false,
                    is_boxed: false,
                    is_array: false,
                },
                PathStep::Field {
                    field_name: "device".to_string(),
                    is_optional: false,
                    is_boxed: false,
                    is_array: true,
                },
            ],
        };
        assert_eq!(path.depth(), 2);
    }
}

