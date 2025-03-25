use std::collections::VecDeque;
use thiserror::Error;

// Add not found error, no properties under node error
#[derive(Error, Debug, PartialEq, Eq)]
pub enum HierarchyError {
    #[error("No properties found under node `{0}`")]
    NoSubPropertiesAvailable(String),
    #[error("Invalid Path: `{0}`")]
    InvalidPath(String),
    #[error("Unsupported Managed Object Type: `{0}`")]
    UnsupportedManagedObject(String),
    #[error("Unsupported Data Object Type: `{0}`")]
    UnsupportedDataObject(String),
    #[error("No field `{0}` in object `{1}`")]
    UnknownField(String, String),
    #[error("Internal error")]
    InternalError,
}

/// The field processing type that determines what code will be generated for unmarshalling the field
enum FieldProcessingType {
    /// ValueElements enum member includes primitive and array data types
    Enum,
    /// Leaf struct data type
    Struct,
    // Polymorphic data type
    Trait,
}
struct FieldData {
    /// The Rust data type of the field
    data_type: String,
    /// Whether the field is optional
    is_optional: bool,
    /// The name of the field. In case of ValueElements enum, this is the enum field name
    field_name: String,
    /// The field processing type
    processing_type: FieldProcessingType,
    /// VIM path of the field. Thisis the field path in VIM syntax as opposed to Rust syntax
    vim_path: String,
}

pub type Result<T> = std::result::Result<T, HierarchyError>;
pub fn resolve_path(managed_object: &str, path: &str) -> Result<FieldData> {
    let mut tail = path.split('.').collect::<VecDeque<&str>>();
    let Some(head) = tail.pop_front() else { return Err(HierarchyError::InvalidPath(path.to_string())); };

    match managed_object {
        "VirtualMachine" => {
            match head {
                "name" => add_primitive("String", "name", &mut tail, false),
                "runtime" => add_struct("VirtualMachineRuntimeInfo", &mut tail, false),
                "summary" => add_struct("VirtualMachineSummary", &mut tail, false),
                _ => {
                    Err(HierarchyError::UnknownField(head.to_string(), "VirtualMachine".to_string()))
                }
            }
        }
        _ => {
            Err(HierarchyError::UnsupportedManagedObject(managed_object.to_string()))
        },
    }

}

fn resolve_do_path(data_type: &str, tail: &mut VecDeque<&str>, is_optional: bool) -> Result<FieldData> {
    let Some(head) = tail.pop_front() else {
        return Err(HierarchyError::InternalError)
    };
    match data_type {
        "VirtualMachineSummary" => {
            match head {
                "overallStatus" => add_enum("ManagedEntityStatusEnum", "overallStatus", tail, is_optional),
                "guest" => add_struct("VirtualMachineGuestSummary", tail, true),
                "storage" => add_struct("VirtualMachineStorageSummary", tail, true),
                "quickStats" => add_struct("VirtualMachineQuickStats", tail, is_optional),
                "runtime" => add_struct("VirtualMachineRuntimeInfo", tail, is_optional),
                _ => Err(HierarchyError::UnknownField(head.to_string(), "VirtualMachineSummary".to_string())),
            }
        },
        "VirtualMachineGuestSummary" => {
            match head {
                "guestFullName" => add_primitive("String", "guestFullName", tail, true),
                _ => Err(HierarchyError::UnknownField(head.to_string(), "VirtualMachineGuestSummary".to_string())),
            }
        },
        "VirtualMachineStorageSummary" => {
            match head {
                "committed" => add_primitive("i64", "committed", tail, is_optional),
                "uncommitted" => add_primitive("i64", "uncommitted", tail, is_optional),
                "unshared" => add_primitive("i64", "unshared", tail, is_optional),
                "timestamp" => add_primitive("String", "timestamp", tail, is_optional),
                _ => Err(HierarchyError::UnknownField(head.to_string(), "VirtualMachineStorageSummary".to_string())),
            }
        },
        "VirtualMachineQuickStats" => {
            match head {
                "overallCpuUsage" => add_primitive("i32", "overallCpuUsage", tail, true),
                "hostMemoryUsage" => add_primitive("i32", "hostMemoryUsage", tail, true),
                _ => Err(HierarchyError::UnknownField(head.to_string(), "VirtualMachineQuickStats".to_string())),
            }
        },
        "VirtualMachineRuntimeInfo" => {
            match head {
                "powerState" => add_enum("VirtualMachinePowerStateEnum", "powerState", tail, is_optional),
                _ => Err(HierarchyError::UnknownField(head.to_string(), "VirtualMachineRuntimeInfo".to_string())),
            }
        },
        _ => Err(HierarchyError::UnsupportedDataObject(data_type.to_string())),
    }
}

fn add_primitive(name: &str, field_name: &str, tail: &mut VecDeque<&str>, is_optional: bool) -> Result<FieldData> {
    if tail.is_empty() {
        add_string_opt(name.to_string(), is_optional)
    } else {
        Err(HierarchyError::NoSubPropertiesAvailable(field_name.to_string()))
    }
}

fn add_enum(name: &str, field_name: &str, tail: &mut VecDeque<&str>, vim_path: &mut Vec<&str>, is_optional: bool) -> Result<FieldData> {
    if tail.is_empty() {
        add_string_opt(format!("vim_rs::types::enums::{}", name), vim_path, is_optional, name.to_string(), FieldProcessingType::Enum)
    } else {
        Err(HierarchyError::NoSubPropertiesAvailable(field_name.to_string()))
    }
}

fn add_trait(name: &str, tail: &mut VecDeque<&str>, vim_path: &mut Vec<&str>, is_optional: bool) -> Result<FieldData> {
    if tail.is_empty() {
        add_string_opt(format!("Box<dyn vim_rs::types::traits::{}Trait>", name), vim_path, is_optional)
    } else {
        resolve_do_path(name, tail, is_optional)
    }
}

fn add_struct(name: &str, tail: &mut VecDeque<&str>, vim_path: &mut Vec<&str>, is_optional: bool) -> Result<FieldData> {
    if tail.is_empty() {
        add_string_opt(format!("vim_rs::types::structs::{}", name), vim_path, is_optional, name.to_string(), FieldProcessingType::Struct)
    } else {
        resolve_do_path(name, tail, is_optional)
    }
}

fn add_string_opt(type_expr: String, vim_path: &Vec<&str>, is_optional: bool, type_name: String, proc_type: FieldProcessingType) -> Result<FieldData> {
    Ok(FieldData {
        data_type: if is_optional {
                        format!("Option<{}>", type_expr)
                    } else {
                        type_expr
                    },
        is_optional,
        field_name: type_name,
        processing_type: proc_type,
        vim_path: vim_path.join(".").to_string(),
    })

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_path() {
        let path = "summary.guest.guestFullName";
        let result = resolve_path("VirtualMachine", path);
        assert_eq!(result.unwrap(), "Option<String>".to_string());
    }

    #[test]
    fn test_resolve_path_invalid() {
        let path = "summary.guest.guestFullName.test";
        let result = resolve_path("VirtualMachine", path);
        assert_eq!(result.unwrap_err(), HierarchyError::NoSubPropertiesAvailable("guestFullName".to_string()));
    }

    #[test]
    fn test_resolve_path_unknown_field() {
        let path = "summary.guest.guestFullName";
        let result = resolve_path("VirtualMachine", path);
        assert_eq!(result.unwrap(), "Option<String>".to_string());
    }

    #[test]
    fn test_resolve_path_unknown_object() {
        let path = "summary.guest.guestFullName";
        let result = resolve_path("VirtualMachine", path);
        assert_eq!(result.unwrap(), "Option<String>".to_string());
    }

    #[test]
    fn test_resolve_path_no_sub_properties() {
        let path = "summary.guest.guestFullName";
        let result = resolve_path("VirtualMachine", path);
        assert_eq!(result.unwrap(), "Option<String>".to_string());
    }
}