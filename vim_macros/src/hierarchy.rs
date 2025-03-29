use std::string::ToString;
use phf_macros::phf_map;
use crate::resolver;
use crate::resolver::{FieldProcessingType, HierarchyError, NodeData};

// Define nested PHF maps for class -> field -> NodeData lookups
static CLASS_FIELDS: phf::Map<&'static str, phf::Map<&'static str, NodeData>> = phf_map! {
    "VirtualMachine" => phf_map! {
        "name" => NodeData {
            type_decl: "String",
            type_name: "",
            is_optional: false,
            processing_type: FieldProcessingType::Enum("PrimitiveString"),
            path_segment: "name",
        },
        "runtime" => NodeData {
            type_decl: "vim_rs::types::structs::VirtualMachineRuntimeInfo",
            type_name: "VirtualMachineRuntimeInfo",
            is_optional: false,
            processing_type: FieldProcessingType::Struct,
            path_segment: "runtime",
        },
        "summary" => NodeData {
            type_decl: "vim_rs::types::structs::VirtualMachineSummary",
            type_name: "VirtualMachineSummary",
            is_optional: false,
            processing_type: FieldProcessingType::Struct,
            path_segment: "summary",
        },
        "config" => NodeData {
            type_decl: "vim_rs::types::structs::VirtualMachineConfigInfo",
            type_name: "VirtualMachineConfigInfo",
            is_optional: true,
            processing_type: FieldProcessingType::Struct,
            path_segment: "config",
        },
    },
    "VirtualHardware" => phf_map! {
        "num_cpu" => NodeData {
            type_decl: "i32",
            type_name: "",
            is_optional: false,
            processing_type: FieldProcessingType::Enum("PrimitiveInt"),
            path_segment: "numCpu",
        },
        "memory_mb" => NodeData {
            type_decl: "i32",
            type_name: "",
            is_optional: false,
            processing_type: FieldProcessingType::Enum("PrimitiveInt"),
            path_segment: "memoryMb",
        },
        "device" => NodeData {
            type_decl: "Vec<Box<dyn vim_rs::types::traits::VirtualDeviceTrait>>",
            type_name: "VirtualDevice",
            is_optional: false,
            processing_type: FieldProcessingType::Enum("ArrayOfVirtualDevice"),
            path_segment: "device",
        },
    },
    "VirtualMachineConfigInfo" => phf_map! {
        "ft_info" => NodeData {
            type_decl: "Box<dyn vim_rs::types::traits::FaultToleranceConfigInfoTrait>",
            type_name: "FaultToleranceConfigInfo",
            is_optional: true,
            processing_type: FieldProcessingType::Trait,
            path_segment: "ftInfo",
        },
        "hardware" => NodeData {
            type_decl: "vim_rs::types::structs::VirtualHardware",
            type_name: "VirtualHardware",
            is_optional: false,
            processing_type: FieldProcessingType::Struct,
            path_segment: "hardware",
        },
    },
    "VirtualMachineSummary" => phf_map! {
        "overall_status" => NodeData {
            type_decl: "vim_rs::types::enums::ManagedEntityStatusEnum",
            type_name: "",
            is_optional: false,
            processing_type: FieldProcessingType::Enum("ManagedEntityStatus"),
            path_segment: "overallStatus",
        },
        "guest" => NodeData {
            type_decl: "vim_rs::types::structs::VirtualMachineGuestSummary",
            type_name: "VirtualMachineGuestSummary",
            is_optional: true,
            processing_type: FieldProcessingType::Struct,
            path_segment: "guest",
        },
        "storage" => NodeData {
            type_decl: "vim_rs::types::structs::VirtualMachineStorageSummary",
            type_name: "VirtualMachineStorageSummary",
            is_optional: true,
            processing_type: FieldProcessingType::Struct,
            path_segment: "storage",
        },
        "quick_stats" => NodeData {
            type_decl: "vim_rs::types::structs::VirtualMachineQuickStats",
            type_name: "VirtualMachineQuickStats",
            is_optional: true,
            processing_type: FieldProcessingType::Struct,
            path_segment: "quickStats",
        },
        "runtime" => NodeData {
            type_decl: "vim_rs::types::structs::VirtualMachineRuntimeInfo",
            type_name: "VirtualMachineRuntimeInfo",
            is_optional: true,
            processing_type: FieldProcessingType::Struct,
            path_segment: "runtime",
        },
    },
    "VirtualMachineGuestSummary" => phf_map! {
        "guest_full_name" => NodeData {
            type_decl: "String",
            type_name: "",
            is_optional: true,
            processing_type: FieldProcessingType::Enum("PrimitiveString"),
            path_segment: "guestFullName",
        },
    },
    "VirtualMachineStorageSummary" => phf_map! {
        "committed" => NodeData {
            type_decl: "i64",
            type_name: "",
            is_optional: true,
            processing_type: FieldProcessingType::Enum("PrimitiveLong"),
            path_segment: "committed",
        },
        "uncommitted" => NodeData {
            type_decl: "i64",
            type_name: "",
            is_optional: true,
            processing_type: FieldProcessingType::Enum("PrimitiveLong"),
            path_segment: "uncommitted",
        },
        "unshared" => NodeData {
            type_decl: "i64",
            type_name: "",
            is_optional: true,
            processing_type: FieldProcessingType::Enum("PrimitiveLong"),
            path_segment: "unshared",
        },
        "timestamp" => NodeData {
            type_decl: "String",
            type_name: "",
            is_optional: true,
            processing_type: FieldProcessingType::Enum("PrimitiveString"),
            path_segment: "timestamp",
        },
    },
    "VirtualMachineQuickStats" => phf_map! {
        "overall_cpu_usage" => NodeData {
            type_decl: "i32",
            type_name: "",
            is_optional: true,
            processing_type: FieldProcessingType::Enum("PrimitiveInt"),
            path_segment: "overallCpuUsage",
        },
        "host_memory_usage" => NodeData {
            type_decl: "i32",
            type_name: "",
            is_optional: true,
            processing_type: FieldProcessingType::Enum("PrimitiveInt"),
            path_segment: "hostMemoryUsage",
        },
    },
    "VirtualMachineRuntimeInfo" => phf_map! {
        "power_state" => NodeData {
            type_decl: "vim_rs::types::enums::VirtualMachinePowerStateEnum",
            type_name: "",
            is_optional: false,
            processing_type: FieldProcessingType::Enum("VirtualMachinePowerState"),
            path_segment: "powerState",
        },
    },
};

pub(crate) fn lookup_field_data(class: &str, field: &str) -> resolver::Result<&'static NodeData> {
    CLASS_FIELDS
        .get(class)
        .and_then(|fields| fields.get(field))
        .ok_or_else(|| {
            if CLASS_FIELDS.contains_key(class) {
                HierarchyError::UnknownField(class.to_string(), field.to_string())
            } else {
                HierarchyError::UnsupportedObjectType(class.to_string())
            }
        })
}