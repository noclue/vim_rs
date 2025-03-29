use thiserror::Error;
use crate::field_data::lookup_field_data;


// Add not found error, no properties under node error
#[derive(Error, Debug, PartialEq, Eq)]
pub enum HierarchyError {
    #[error("No properties found under node `{0}`")]
    NoSubPropertiesAvailable(String),
    #[error("Unsupported Object Type: `{0}`")]
    UnsupportedObjectType(String),
    #[error("Type `{0}` has no property field `{1}`")]
    UnknownField(String, String),
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
pub(crate) struct NodeData {
    /// String to emit as the type declaration in the generated code
    pub type_decl: &'static str,
    /// Name of the type used ofr further field look ups. Only relevant for complex types - structs and traits
    pub type_name: &'static str,
    /// Whether the field is optional
    pub is_optional: bool,
    /// The field processing type
    pub processing_type: FieldProcessingType,
    /// The path segment in the VIM model
    pub path_segment: &'static str,
}

pub fn resolve_path(managed_object: &str, path: &str) -> Result<FieldData> {
    let tail = path.split('.').collect::<Vec<&str>>();
    let mut obj: &str = managed_object;
    let mut optional = false;
    let mut path = Vec::new();
    let mut field_data = &NodeData {
        type_decl: "",
        type_name: "",
        is_optional: false,
        processing_type: FieldProcessingType::Struct,
        path_segment: "",
    };

    for segment in tail.iter() {
        if matches!(field_data.processing_type, FieldProcessingType::Enum(_)) {
            return Err(HierarchyError::NoSubPropertiesAvailable(path.join(".").to_string()));
        }
        field_data = lookup_field_data(obj, segment)?;
        obj = field_data.type_name;
        optional = optional | field_data.is_optional;
        path.push(field_data.path_segment);
    }

    Ok(FieldData {
        data_type: add_optional(&field_data.type_decl, optional),
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

#[cfg(test)]
mod tests {
    use crate::resolver::{resolve_path, FieldProcessingType, HierarchyError};

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
    fn test_resolve_path_struct() {
        let path = "summary.guest";
        let result = resolve_path("VirtualMachine", path).unwrap();
        assert_eq!(result.data_type, "Option<vim_rs::types::structs::VirtualMachineGuestSummary>".to_string());
        assert_eq!(result.vim_path, "summary.guest".to_string());
        assert_eq!(result.is_optional, true);
        assert_eq!(result.processing_type, FieldProcessingType::Struct);
    }

    #[test]
    fn test_resolve_path_trait() {
        let path = "config.ft_info";
        let result = resolve_path("VirtualMachine", path).unwrap();
        assert_eq!(result.data_type, "Option<Box<dyn vim_rs::types::traits::FaultToleranceConfigInfoTrait>>".to_string());
        assert_eq!(result.vim_path, "config.ftInfo".to_string());
        assert_eq!(result.is_optional, true);
        assert_eq!(result.processing_type, FieldProcessingType::Trait);
    }

    #[test]
    fn test_resolve_array() {
        let path = "config.hardware.device";
        let result = resolve_path("VirtualMachine", path).unwrap();
        assert_eq!(result.data_type, "Option<Vec<Box<dyn vim_rs::types::traits::VirtualDeviceTrait>>>".to_string());
        assert_eq!(result.vim_path, "config.hardware.device".to_string());
        assert_eq!(result.is_optional, true);
        assert_eq!(result.processing_type, FieldProcessingType::Enum("ArrayOfVirtualDevice"));
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