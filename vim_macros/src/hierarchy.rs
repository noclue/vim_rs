use std::string::ToString;
use thiserror::Error;

// Add not found error, no properties under node error
#[derive(Error, Debug, PartialEq, Eq)]
pub enum HierarchyError {
    #[error("No properties found under node `{0}`")]
    NoSubPropertiesAvailable(String),
    #[error("Invalid Path: `{0}`")]
    InvalidPath(String),
    #[error("Unsupported Object Type: `{0}`")]
    UnsupportedObjectType(String),
    #[error("No field `{0}` in object `{1}`")]
    UnknownField(String, String),
    #[error("Internal error")]
    InternalError,
}

pub type Result<T> = std::result::Result<T, HierarchyError>;

/// The field processing type that determines what code will be generated for unmarshalling the field
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldProcessingType {
    /// ValueElements enum member includes primitive and array data types
    Enum(&'static str),
    /// Leaf struct data type
    Struct,
    // Polymorphic data type
    Trait,
}

#[derive(Debug, Clone)]
pub struct FieldData {
    /// The Rust data type of the field
    pub data_type: String,
    /// Whether the field is optional
    pub is_optional: bool,
    /// The field processing type
    pub processing_type: FieldProcessingType,
    /// VIM path of the field. This is the field path in VIM syntax as opposed to Rust syntax
    pub vim_path: String,
}



// Default field data to be used as placeholder in case of errors
pub fn get_default_field_data() -> FieldData {
    FieldData {
        data_type: "String".to_string(),
        is_optional: false,
        processing_type: FieldProcessingType::Enum("PrimitiveString"),
        vim_path: "name".to_string(),
    }
}

#[derive(Clone, Debug)]
struct NodeData {
    type_decl: &'static str,
    is_optional: bool,
    processing_type: FieldProcessingType,
    path_segment: &'static str,
}


pub fn resolve_path(managed_object: &str, path: &str) -> Result<FieldData> {
    let tail = path.split('.').collect::<Vec<&str>>();
    let mut obj: &str = managed_object;
    let mut optional = false;
    let mut path = Vec::new();
    let mut field_data = &NodeData {
        type_decl: "",
        is_optional: false,
        processing_type: FieldProcessingType::Struct,
        path_segment: "",
    };

    for segment in tail.iter() {
        if matches!(field_data.processing_type, FieldProcessingType::Enum(_)) {
            return Err(HierarchyError::NoSubPropertiesAvailable(path.join(".").to_string()));
        }
        field_data = lookup_field_data(obj, segment)?;
        obj = field_data.type_decl;
        optional = optional | field_data.is_optional;
        path.push(field_data.path_segment);
    }


    Ok(FieldData {
        data_type: add_optional(obj, optional),
        is_optional: optional,
        processing_type: field_data.processing_type.clone(),
        vim_path: path.join("."),
    })

}

fn add_optional(type_decl: &str, is_optional: bool) -> String {
    if is_optional {
        format!("Option<{}>", type_decl)
    } else {
        type_decl.to_string()
    }
}


fn lookup_field_data(class: &str, field: &str) -> Result<&'static NodeData> {
    match class {
        "VirtualMachine" => {
            match field {
                "name" => Ok(&NodeData {
                    type_decl: "String",
                    is_optional: false,
                    processing_type: FieldProcessingType::Enum("PrimitiveString"),
                    path_segment: "name",
                }),
                "runtime" => Ok(&NodeData {
                    type_decl: "VirtualMachineRuntimeInfo",
                    is_optional: false,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "runtime",
                }),
                "summary" => Ok(&NodeData {
                    type_decl: "VirtualMachineSummary",
                    is_optional: false,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "summary",
                }),
                _ => Err(HierarchyError::UnknownField(class.to_string(), field.to_string())),
            }
        },
        "VirtualMachineSummary" => {
            match field {
                "overall_status" => Ok(&NodeData {
                    type_decl: "ManagedEntityStatusEnum",
                    is_optional: false,
                    processing_type: FieldProcessingType::Enum("ManagedEntityStatusEnum"),
                    path_segment: "overallStatus",
                }),
                "guest" => Ok(&NodeData {
                    type_decl: "VirtualMachineGuestSummary",
                    is_optional: true,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "guest",
                }),
                "storage" => Ok(&NodeData {
                    type_decl: "VirtualMachineStorageSummary",
                    is_optional: true,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "storage",
                }),
                "quick_stats" => Ok(&NodeData {
                    type_decl: "VirtualMachineQuickStats",
                    is_optional: true,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "quickStats",
                }),
                "runtime" => Ok(&NodeData {
                    type_decl: "VirtualMachineRuntimeInfo",
                    is_optional: true,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "runtime",
                }),
                _ => Err(HierarchyError::UnknownField(class.to_string(), field.to_string())),
            }
        },
        "VirtualMachineGuestSummary" => {
            match field {
                "guest_full_name" => Ok(&NodeData {
                    type_decl: "String",
                    is_optional: true,
                    processing_type: FieldProcessingType::Enum("PrimitiveString"),
                    path_segment: "guestFullName",
                }),
                _ => Err(HierarchyError::UnknownField(class.to_string(), field.to_string())),
            }
        },
        "VirtualMachineStorageSummary" => {
            match field {
                "committed" => Ok(&NodeData {
                    type_decl: "i64",
                    is_optional: true,
                    processing_type: FieldProcessingType::Enum("PrimitiveLong"),
                    path_segment: "committed",
                }),
                "uncommitted" => Ok(&NodeData {
                    type_decl: "i64",
                    is_optional: true,
                    processing_type: FieldProcessingType::Enum("PrimitiveLong"),
                    path_segment: "uncommitted",
                }),
                "unshared" => Ok(&NodeData {
                    type_decl: "i64",
                    is_optional: true,
                    processing_type: FieldProcessingType::Enum("PrimitiveLong"),
                    path_segment: "unshared",
                }),
                "timestamp" => Ok(&NodeData {
                    type_decl: "String",
                    is_optional: true,
                    processing_type: FieldProcessingType::Enum("PrimitiveString"),
                    path_segment: "timestamp",
                }),
                _ => Err(HierarchyError::UnknownField(class.to_string(), field.to_string())),
            }
        },
        "VirtualMachineQuickStats" => {
            match field {
                "overall_cpu_usage" => Ok(&NodeData {
                    type_decl: "i32",
                    is_optional: true,
                    processing_type: FieldProcessingType::Enum("PrimitiveInt"),
                    path_segment: "overallCpuUsage",
                }),
                "host_memory_usage" => Ok(&NodeData {
                    type_decl: "i32",
                    is_optional: true,
                    processing_type: FieldProcessingType::Enum("PrimitiveInt"),
                    path_segment: "hostMemoryUsage",
                }),
                _ => Err(HierarchyError::UnknownField(class.to_string(), field.to_string())),
            }
        },
        "VirtualMachineRuntimeInfo" => {
            match field {
                "power_state" => Ok(&NodeData {
                    type_decl: "VirtualMachinePowerStateEnum",
                    is_optional: false,
                    processing_type: FieldProcessingType::Enum("VirtualMachinePowerStateEnum"),
                    path_segment: "powerState",
                }),
                _ => Err(HierarchyError::UnknownField(class.to_string(), field.to_string())),
            }
        },
        _ => Err(HierarchyError::UnsupportedObjectType(class.to_string())),
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_path() {
        let path = "summary.guest.guest_full_name";
        let result = resolve_path("VirtualMachine", path).unwrap();
        assert_eq!(result.data_type, "Option<String>".to_string());
        assert_eq!(result.vim_path, "summary.guest.guestFullName".to_string());
        assert_eq!(result.is_optional, true);
        assert_eq!(result.processing_type, FieldProcessingType::Enum("PrimitiveString"));
    }

    #[test]
    fn test_resolve_path_invalid() {
        let path = "summary.guest.guest_full_name.test";
        let result = resolve_path("VirtualMachine", path);
        assert_eq!(result.unwrap_err(), HierarchyError::NoSubPropertiesAvailable("summary.guest.guestFullName".to_string()));
    }

    #[test]
    fn test_resolve_path_unknown_field() {
        let path = "summary.guest.guest_name";
        let result = resolve_path("VirtualMachine", path);
        assert_eq!(result.unwrap_err(), HierarchyError::UnknownField("VirtualMachineGuestSummary".to_string(), "guest_name".to_string()));
    }

    #[test]
    fn test_resolve_path_unknown_object() {
        let path = "summary.guest.guest_full_name";
        let result = resolve_path("VirtualShip", path);
        assert_eq!(result.unwrap_err(), HierarchyError::UnsupportedObjectType("VirtualShip".to_string()));
    }

}