use std::string::ToString;
use crate::resolver;
use crate::resolver::{FieldProcessingType, HierarchyError, NodeData};

pub(crate) fn lookup_field_data(class: &str, field: &str) -> resolver::Result<&'static NodeData> {
    match class {
        "VirtualMachine" => {
            match field {
                "name" => Ok(&NodeData {
                    type_decl: "String",
                    type_name: "",
                    is_optional: false,
                    processing_type: FieldProcessingType::Enum("PrimitiveString"),
                    path_segment: "name",
                }),
                "runtime" => Ok(&NodeData {
                    type_decl: "vim_rs::types::structs::VirtualMachineRuntimeInfo",
                    type_name: "VirtualMachineRuntimeInfo",
                    is_optional: false,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "runtime",
                }),
                "summary" => Ok(&NodeData {
                    type_decl: "vim_rs::types::structs::VirtualMachineSummary",
                    type_name: "VirtualMachineSummary",
                    is_optional: false,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "summary",
                }),
                "config" => Ok(&NodeData {
                    type_decl: "vim_rs::types::structs::VirtualMachineConfigInfo",
                    type_name: "VirtualMachineConfigInfo",
                    is_optional: true,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "config",
                }),
                _ => Err(HierarchyError::UnknownField(class.to_string(), field.to_string())),
            }
        },
        "VirtualHardware" => {
            match field {
                "num_cpu" => Ok(&NodeData {
                    type_decl: "i32",
                    type_name: "",
                    is_optional: false,
                    processing_type: FieldProcessingType::Enum("PrimitiveInt"),
                    path_segment: "numCpu",
                }),
                "memory_mb" => Ok(&NodeData {
                    type_decl: "i32",
                    type_name: "",
                    is_optional: false,
                    processing_type: FieldProcessingType::Enum("PrimitiveInt"),
                    path_segment: "memoryMb",
                }),
                "device" => Ok(&NodeData {
                    type_decl: "Vec<Box<dyn vim_rs::types::traits::VirtualDeviceTrait>>",
                    type_name: "VirtualDevice",
                    is_optional: false,
                    processing_type: FieldProcessingType::Enum("ArrayOfVirtualDevice"),
                    path_segment: "device",
                }),
                _ => Err(HierarchyError::UnknownField(class.to_string(), field.to_string())),
            }
        },
        "VirtualMachineConfigInfo" => {
            match field {
                "ft_info" => Ok(&NodeData {
                    type_decl: "Box<dyn vim_rs::types::traits::FaultToleranceConfigInfoTrait>",
                    type_name: "FaultToleranceConfigInfo",
                    is_optional: true,
                    processing_type: FieldProcessingType::Trait,
                    path_segment: "ftInfo",
                }),
                "hardware" => Ok(&NodeData {
                    type_decl: "vim_rs::types::structs::VirtualHardware",
                    type_name: "VirtualHardware",
                    is_optional: false,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "hardware",
                }),
                _ => Err(HierarchyError::UnknownField(class.to_string(), field.to_string())),
            }
        },
        "VirtualMachineSummary" => {
            match field {
                "overall_status" => Ok(&NodeData {
                    type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum",
                    type_name: "",
                    is_optional: false,
                    processing_type: FieldProcessingType::Enum("ManagedEntityStatus"),
                    path_segment: "overallStatus",
                }),
                "guest" => Ok(&NodeData {
                    type_decl: "vim_rs::types::structs::VirtualMachineGuestSummary",
                    type_name: "VirtualMachineGuestSummary",
                    is_optional: true,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "guest",
                }),
                "storage" => Ok(&NodeData {
                    type_decl: "vim_rs::types::structs::VirtualMachineStorageSummary",
                    type_name: "VirtualMachineStorageSummary",
                    is_optional: true,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "storage",
                }),
                "quick_stats" => Ok(&NodeData {
                    type_decl: "vim_rs::types::structs::VirtualMachineQuickStats",
                    type_name: "VirtualMachineQuickStats",
                    is_optional: true,
                    processing_type: FieldProcessingType::Struct,
                    path_segment: "quickStats",
                }),
                "runtime" => Ok(&NodeData {
                    type_decl: "vim_rs::types::structs::VirtualMachineRuntimeInfo",
                    type_name: "VirtualMachineRuntimeInfo",
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
                    type_name: "",
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
                    type_name: "",
                    is_optional: true,
                    processing_type: FieldProcessingType::Enum("PrimitiveLong"),
                    path_segment: "committed",
                }),
                "uncommitted" => Ok(&NodeData {
                    type_decl: "i64",
                    type_name: "",
                    is_optional: true,
                    processing_type: FieldProcessingType::Enum("PrimitiveLong"),
                    path_segment: "uncommitted",
                }),
                "unshared" => Ok(&NodeData {
                    type_decl: "i64",
                    type_name: "",
                    is_optional: true,
                    processing_type: FieldProcessingType::Enum("PrimitiveLong"),
                    path_segment: "unshared",
                }),
                "timestamp" => Ok(&NodeData {
                    type_decl: "String",
                    type_name: "",
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
                    type_name: "",
                    is_optional: true,
                    processing_type: FieldProcessingType::Enum("PrimitiveInt"),
                    path_segment: "overallCpuUsage",
                }),
                "host_memory_usage" => Ok(&NodeData {
                    type_decl: "i32",
                    type_name: "",
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
                    type_decl: "vim_rs::types::enums::VirtualMachinePowerStateEnum",
                    type_name: "",
                    is_optional: false,
                    processing_type: FieldProcessingType::Enum("VirtualMachinePowerState"),
                    path_segment: "powerState",
                }),
                _ => Err(HierarchyError::UnknownField(class.to_string(), field.to_string())),
            }
        },
        _ => Err(HierarchyError::UnsupportedObjectType(class.to_string())),
    }
}




