use vim_rs::types::enums::{ManagedEntityStatusEnum, MoTypesEnum, PropertyChangeOpEnum, VirtualMachinePowerStateEnum};
use vim_rs::types::structs::{ManagedObjectReference, ObjectContent, ObjectUpdate, PropertyChange, PropertySpec};
use vim_rs::types::vim_any::VimAny;
use vim_rs::types::boxed_types::ValueElements;
use vim_rs::types::convert::CastInto;
use log::error;
use vim_rs::core::pc_helpers;


#[derive(Debug)]
pub struct VirtualMachine {
    pub id: ManagedObjectReference,
    pub name: String, // name
    pub os: Option<String>, // summary.guest.guestFullName
    pub storage: Option<vim_rs::types::structs::VirtualMachineStorageSummary>, // summary.storage
    pub host_cpu: Option<i32>, // summary.quickStats.overallCpuUsage
    pub host_memory: Option<i32>, // summary.quickStats.hostMemoryUsage
    pub status: ManagedEntityStatusEnum, // overallStatus
    pub power_state: VirtualMachinePowerStateEnum, // runtime.powerState
    pub ft_info: Option<Box<dyn vim_rs::types::traits::FaultToleranceConfigInfoTrait>>, // config.ftInfo
    pub devices: Option<Vec<Box<dyn vim_rs::types::traits::VirtualDeviceTrait>>>, // config.hardware.device
}

impl VirtualMachine {
    pub fn prop_spec() -> PropertySpec {
        vim_rs::types::structs::PropertySpec {
            all: Some(false),
            path_set: Some(vec![
                "name".into(),
                "summary.guest.guestFullName".into(),
                "summary.storage".into(),
                "summary.quickStats.overallCpuUsage".into(),
                "summary.quickStats.hostMemoryUsage".into(),
                "overallStatus".into(),
                "runtime.powerState".into(),
                "config.ftInfo".into(),
                "config.hardware.device".into(),
            ]),
            r#type: Into::<&str>::into(MoTypesEnum::VirtualMachine).to_string(),
        }
    }

    pub fn id(&self) -> &ManagedObjectReference {
        &self.id
    }

    pub fn apply_update(&mut self, row: Vec<PropertyChange>) -> pc_helpers::Result<()> {
        for prop in row {
            if matches!(prop.op, PropertyChangeOpEnum::Add | PropertyChangeOpEnum::Remove | PropertyChangeOpEnum::Other_(_)) {
                // It is assumption of the code here that create_filter was called with `partial_updates = false`.
                // This flag value implies only `assign` and `indirectRemove` operations are returned. `add` and
                // `remove` operations are used when `partial_updates = true`. The big problem is that with
                // `partial_updates` server will return paths to sub-properties that we cannot easily resolve in
                // Rust hence the assumption that `partial_updates = false` is made.`Add` and `Remove` operations
                // are not supported in this code.
                error!("Unsupported PropertyChangeOp: {:?} for property {} in  object {:?}", prop.op, prop.name, self.id);
                continue;
            }
            match prop.name.as_str() {
                "name" => {
                    self.name = match prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveString(val))) => val.clone(),
                        None => return Err(pc_helpers::Error::NoneValueForRequiredField("name".to_string())),
                        _ => "<Unknown>".to_string(),
                    };
                }
                "summary.guest.guestFullName" => {
                    self.os = match prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveString(val))) => Some(val.clone()),
                        None => None,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType {property: "summary.guest.guestFullName".to_string(), expected: "String".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "summary.storage" => {
                    self.storage = match prop.val {
                        Some(VimAny::Object(obj)) => {
                            let name: &'static str = obj.data_type().into();
                            match obj.as_any_box().downcast::<vim_rs::types::structs::VirtualMachineStorageSummary>() {
                                Ok(storage) => Some(*storage),
                                Err(_) => return Err(pc_helpers::Error::InvalidPropertyType {property: "summary.storage".to_string(), expected: "VirtualMachineStorageSummary".to_string(), got: name.to_string()}),
                            }
                        },
                        None => None,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType {property: "summary.storage".to_string(), expected: "VirtualMachineStorageSummary".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "summary.quickStats.overallCpuUsage" => {
                    self.host_cpu = match prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveInt(val))) => Some(val),
                        None => None,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType {property: "summary.quickStats.overallCpuUsage".to_string(), expected: "i32".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "summary.quickStats.hostMemoryUsage" => {
                    self.host_memory = match prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveInt(val))) => Some(val),
                        None => None,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType {property: "summary.quickStats.hostMemoryUsage".to_string(), expected: "i32".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "overallStatus" => {
                    self.status = match prop.val {
                        Some(VimAny::Value(ValueElements::ManagedEntityStatus(val))) => val.clone(),
                        None => return Err(pc_helpers::Error::NoneValueForRequiredField("overallStatus".to_string())),
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType {property: "overallStatus".to_string(), expected: "ManagedEntityStatus".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "runtime.powerState" => {
                    self.power_state = match prop.val {
                        Some(VimAny::Value(ValueElements::VirtualMachinePowerState(val))) => val.clone(),
                        None => return Err(pc_helpers::Error::NoneValueForRequiredField("runtime.powerState".to_string())),
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType { property: "runtime.powerState".to_string(), expected: "VirtualMachinePowerState".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "config.ftInfo" => {
                    self.ft_info = match prop.val {
                        Some(VimAny::Object(obj)) => {
                            let name: &'static str = obj.data_type().into();
                            match obj.into_box() {
                                Ok(ft_info) => Some(ft_info),
                                Err(_) => return Err(pc_helpers::Error::InvalidPropertyType {property: "config.ftInfo".to_string(), expected: "FaultToleranceConfigInfo".to_string(), got: name.to_string()}),
                            }
                        },
                        None => None,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType {property: "config.ftInfo".to_string(), expected: "FaultToleranceConfigInfo".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "config.hardware.device" => {
                    self.devices = match prop.val {
                        Some(VimAny::Value(ValueElements::ArrayOfVirtualDevice(vd))) => Some(vd),
                        None => None,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType {property: "config.hardware.device".to_string(), expected: "ArrayOfVirtualDevice".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl TryFrom<ObjectUpdate> for VirtualMachine {
    type Error = pc_helpers::Error;

    fn try_from(row: ObjectUpdate) -> pc_helpers::Result<Self> {
        let id = row.obj;
        let Some(row) = row.change_set else {
            return Err(pc_helpers::Error::NoDataFound);
        };

        let mut field1 = None;
        let mut field2 = None;
        let mut field5 = None;
        let mut field6 = None;
        let mut field7 = None;
        let mut field3 = None;
        let mut field4 = None;
        let mut field8 = None;
        let mut field9 = None;

        for prop in row {
            if matches!(prop.op, PropertyChangeOpEnum::Add | PropertyChangeOpEnum::Remove | PropertyChangeOpEnum::Other_(_)) {
                // It is assumption of the code here that create_filter was called with `partial_updates = false`.
                // This flag value implies only `assign` and `indirectRemove` operations are returned. `add` and
                // `remove` operations are used when `partial_updates = true`. The big problem is that with
                // `partial_updates` server will return paths to sub-properties that we cannot easily resolve in
                // Rust hence the assumption that `partial_updates = false` is made.`Add` and `Remove` operations
                // are not supported in this code.
                error!("Unsupported PropertyChangeOp: {:?} for property {} in  object {:?}", prop.op, prop.name, id);
                continue;
            }
            match prop.name.as_str() {
                "name" => {
                    field1 = match prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveString(val))) => Some(val),
                        None => continue,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType {property: "name".to_string(), expected: "String".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "summary.guest.guestFullName" => {
                    field2 = match prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveString(val))) => Some(val.clone()),
                        None => continue,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType {property: "summary.guest.guestFullName".to_string(), expected: "String".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "summary.storage" => {
                    field5 = match prop.val {
                        Some(VimAny::Object(obj)) => {
                            let name: &'static str = obj.data_type().into();
                            match obj.as_any_box().downcast() {
                                Ok(storage) => Some(*storage),
                                Err(_) => return Err(pc_helpers::Error::InvalidPropertyType {property: "summary.storage".to_string(), expected: "VirtualMachineStorageSummary".to_string(), got: name.to_string()}),
                            }
                        },
                        None => continue,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType {property: "summary.storage".to_string(), expected: "VirtualMachineStorageSummary".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "summary.quickStats.overallCpuUsage" => {
                    field6 = match prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveInt(val))) => Some(val),
                        None => continue,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType { property: "summary.quickStats.overallCpuUsage".to_string(), expected: "i32".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "summary.quickStats.hostMemoryUsage" => {
                    field7 = match prop.val {
                        Some(VimAny::Value(ValueElements::PrimitiveInt(val))) => Some(val),
                        None => continue,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType { property: "summary.quickStats.hostMemoryUsage".to_string(), expected: "i32".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "overallStatus" => {
                    field3 = match prop.val {
                        Some(VimAny::Value(ValueElements::ManagedEntityStatus(val))) => Some(val.clone()),
                        None => continue,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType { property: "overallStatus".to_string(), expected: "ManagedEntityStatus".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "runtime.powerState" => {
                    field4 = match prop.val {
                        Some(VimAny::Value(ValueElements::VirtualMachinePowerState(val))) => Some(val.clone()),
                        None => continue,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType { property: "runtime.powerState".to_string(), expected: "VirtualMachinePowerState".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "config.ftInfo" => {
                    field8 = match prop.val {
                        Some(VimAny::Object(obj)) => {
                            let name: &'static str = obj.data_type().into();
                            match obj.into_box() {
                                Ok(ft_info) => Some(ft_info),
                                Err(_) => return Err(pc_helpers::Error::InvalidPropertyType { property: "config.ftInfo".to_string(), expected: "FaultToleranceConfigInfo".to_string(), got: name.to_string()}),
                            }
                        },
                        None => continue,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType { property: "config.ftInfo".to_string(), expected: "FaultToleranceConfigInfo".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "config.hardware.device" => {
                    field9 = match prop.val {
                        Some(VimAny::Value(ValueElements::ArrayOfVirtualDevice(vd))) => Some(vd),
                        None => continue,
                        Some(ref val) => return Err(pc_helpers::Error::InvalidPropertyType { property: "config.hardware.device".to_string(), expected: "ArrayOfVirtualDevice".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                _ => {}
            }
        }
        Ok(VirtualMachine {
            id,
            name: field1.ok_or(pc_helpers::Error::NoneValueForRequiredField(String::from("name")))?,
            os: field2,
            status: field3.ok_or(pc_helpers::Error::NoneValueForRequiredField(String::from("overallStatus")))?,
            power_state: field4.ok_or(pc_helpers::Error::NoneValueForRequiredField(String::from("runtime.powerState")))?,
            storage: field5,
            host_cpu: field6,
            host_memory: field7,
            ft_info: field8,
            devices: field9,
        })
    }
}

impl TryFrom<ObjectContent> for VirtualMachine {
    type Error = pc_helpers::Error;

    fn try_from(row: ObjectContent) -> pc_helpers::Result<Self> {
        let id = row.obj;
        let Some(row) = row.prop_set else {
            return Err(pc_helpers::Error::NoDataFound);
        };

        let mut field1 = None;
        let mut field2 = None;
        let mut field5 = None;
        let mut field6 = None;
        let mut field7 = None;
        let mut field3 = None;
        let mut field4 = None;
        let mut field8 = None;
        let mut field9 = None;

        for prop in row {
            match prop.name.as_str() {
                "name" => {
                    field1 = match prop.val {
                        VimAny::Value(ValueElements::PrimitiveString(val)) => Some(val),
                        ref val => return Err(pc_helpers::Error::InvalidPropertyType {property: "name".to_string(), expected: "PrimitiveString".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "summary.guest.guestFullName" => {
                    field2 = match prop.val {
                        VimAny::Value(ValueElements::PrimitiveString(val)) => Some(val.clone()),
                        ref val => return Err(pc_helpers::Error::InvalidPropertyType {property: "summary.guest.guestFullName".to_string(), expected: "PrimitiveString".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "summary.storage" => {
                    field5 = match prop.val {
                        VimAny::Object(obj) => {
                            let name: &'static str = obj.data_type().into();
                            match obj.as_any_box().downcast() {
                                Ok(val) => Some(*val),
                                Err(_) => return Err(pc_helpers::Error::InvalidPropertyType {property: "summary.storage".to_string(), expected: "VirtualMachineStorageSummary".to_string(), got: name.to_string()}),
                            }
                        },
                        ref val => return Err(pc_helpers::Error::InvalidPropertyType {property: "summary.storage".to_string(), expected: "VirtualMachineStorageSummary".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "summary.quickStats.overallCpuUsage" => {
                    field6 = match prop.val {
                        VimAny::Value(ValueElements::PrimitiveInt(val)) => Some(val),
                        ref val => return Err(pc_helpers::Error::InvalidPropertyType { property: "summary.quickStats.overallCpuUsage".to_string(), expected: "i32".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "summary.quickStats.hostMemoryUsage" => {
                    field7 = match prop.val {
                        VimAny::Value(ValueElements::PrimitiveInt(val)) => Some(val),
                        ref val => return Err(pc_helpers::Error::InvalidPropertyType { property: "summary.quickStats.hostMemoryUsage".to_string(), expected: "i32".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "overallStatus" => {
                    field3 = match prop.val {
                        VimAny::Value(ValueElements::ManagedEntityStatus(val)) => Some(val.clone()),
                        ref val => return Err(pc_helpers::Error::InvalidPropertyType { property: "overallStatus".to_string(), expected: "ManagedEntityStatus".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "runtime.powerState" => {
                    field4 = match prop.val {
                        VimAny::Value(ValueElements::VirtualMachinePowerState(val)) => Some(val.clone()),
                        ref val => return Err(pc_helpers::Error::InvalidPropertyType { property: "runtime.powerState".to_string(), expected: "VirtualMachinePowerState".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "config.ftInfo" => {
                    field8 = match prop.val {
                        VimAny::Object(obj) => {
                            let name: &'static str = obj.data_type().into();
                            match obj.into_box() {
                                Ok(val) => Some(val),
                                Err(_) => return Err(pc_helpers::Error::InvalidPropertyType { property: "config.ftInfo".to_string(), expected: "FaultToleranceConfigInfo".to_string(), got: name.to_string()}),
                            }
                        },
                        ref val => return Err(pc_helpers::Error::InvalidPropertyType { property: "config.ftInfo".to_string(), expected: "FaultToleranceConfigInfo".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                "config.hardware.device" => {
                    field9 = match prop.val {
                        VimAny::Value(ValueElements::ArrayOfVirtualDevice(vd)) => Some(vd),
                        ref val => return Err(pc_helpers::Error::InvalidPropertyType { property: "config.hardware.device".to_string(), expected: "ArrayOfVirtualDevice".to_string(), got: pc_helpers::type_name(val)}),
                    };
                }
                name => {
                    error!("Unexpected property {} in object {:?}", name, id);
                }
            }
        }
        Ok(VirtualMachine {
            id,
            name: field1.ok_or(pc_helpers::Error::NoneValueForRequiredField(String::from("name")))?,
            os: field2,
            status: field3.ok_or(pc_helpers::Error::NoneValueForRequiredField(String::from("overallStatus")))?,
            power_state: field4.ok_or(pc_helpers::Error::NoneValueForRequiredField(String::from("runtime.powerState")))?,
            storage: field5,
            host_cpu: field6,
            host_memory: field7,
            ft_info: field8,
            devices: field9,
        })
    }
}




