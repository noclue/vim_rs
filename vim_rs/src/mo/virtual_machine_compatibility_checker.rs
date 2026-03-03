use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// A singleton managed object that can answer questions about compatibility
/// of a virtual machine with a host.
#[derive(Clone)]
pub struct VirtualMachineCompatibilityChecker {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VirtualMachineCompatibilityChecker {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Tests whether or not a virtual machine could be placed on
    /// the given host in the given resource pool.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The virtual machine we'd like to place.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### host
    /// The host we would like the virtual machine
    /// to execute on. The host parameter may be left unset if the compute
    /// resource associated with the pool represents a stand-alone host
    /// or a DRS-enabled cluster. In the former case the stand-alone host is
    /// used. In the latter case, each connected host in the cluster
    /// that is not in maintenance mode is tested. If the virtual machine is a
    /// template then either this parameter or the pool parameter must be set.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### pool
    /// The resource pool we would like the virtual machine
    /// to reside in. If the pool parameter is left unset, then the virtual
    /// machine's current pool is assumed. If the virtual machine is a template
    /// then either this parameter or the host parameter must be set.
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ### test_type
    /// The set of tests to run. If this argument is not set, all
    /// tests will be run. See *CheckTestType_enum* for possible values.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// host or virtual machine's current state. For example, if the host
    /// is in maintenance mode or if the virtual machine's configuration
    /// information is not available.
    /// 
    /// ***InvalidArgument***: if the desired host and pool are not associated
    /// with the same compute resource, the host parameter is left unset
    /// when the specified pool is associated with a non-DRS cluster, or
    /// if the specified vm does not exist.
    /// 
    /// ***DatacenterMismatch***: if the provided host and pool do not belong
    /// to the same datacenter.
    pub async fn check_compatibility_task(&self, vm: &crate::types::structs::ManagedObjectReference, host: Option<&crate::types::structs::ManagedObjectReference>, pool: Option<&crate::types::structs::ManagedObjectReference>, test_type: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CheckCompatibilityRequestType {vm, host, pool, test_type, };
        let path = format!("/VirtualMachineCompatibilityChecker/{moId}/CheckCompatibility_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Tests whether the provided virtual machine can be powered on
    /// on the given host and/or resource pool.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The virtual machine to power on.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### host
    /// The host on which we want to power on the virtual machine.
    /// The host parameter may be left unset if the compute
    /// resource associated with the pool represents a stand-alone host
    /// or a DRS-enabled cluster. In the former case the stand-alone host
    /// is used. In the latter case, each connected host in the cluster
    /// that is not in maintenance mode is tested. Either this parameter
    /// or the pool parameter must be set.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### pool
    /// The resource pool we would like the virtual machine to run
    /// in. If the pool parameter is left unset, we use the host's
    /// root resource pool.
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ### test_type
    /// The set of tests to run. If this argument is not set,
    /// all tests will be run. See *CheckTestType_enum* for possible values.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the desired host and pool are not associated
    /// with the same compute resource, the host parameter is left unset
    /// when the specified pool is associated with a non-DRS cluster, or
    /// if the provided vm does not exist.
    /// 
    /// ***DatacenterMismatch***: if the provided host and pool do not belong
    /// to the same datacenter.
    pub async fn check_power_on_task(&self, vm: &crate::types::structs::ManagedObjectReference, host: Option<&crate::types::structs::ManagedObjectReference>, pool: Option<&crate::types::structs::ManagedObjectReference>, test_type: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CheckPowerOnRequestType {vm, host, pool, test_type, };
        let path = format!("/VirtualMachineCompatibilityChecker/{moId}/CheckPowerOn_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Tests whether the provided virtual machine specification can be applied
    /// on the given host and resource pool.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The specification of the virtual machine to create.
    ///
    /// ### vm
    /// The existing virtual machine to apply the spec to.
    /// If this is not provided, the spec is assumed to be for the creation
    /// of a new virtual machine.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### host
    /// The host we would like the virtual machine
    /// to execute on. The host parameter may be left unset if the compute
    /// resource associated with the pool represents a stand-alone host
    /// or a DRS-enabled cluster. In the former case the stand-alone host
    /// is used. In the latter case, each connected host in the cluster
    /// that is not in maintenance mode is tested. If the virtual machine
    /// is a template, then either this parameter or the pool parameter
    /// must be set.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### pool
    /// The resource pool we would like the virtual machine
    /// to reside in. If the pool parameter is left unset, then we use the
    /// host's root resource pool.
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ### test_type
    /// The set of tests to run. If this argument is not set, all
    /// tests will be run. See *CheckTestType_enum* for possible values.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the desired host and pool are not associated
    /// with the same compute resource, the host parameter is left unset
    /// when the specified pool is associated with a non-DRS cluster, or
    /// if the provided vm does not exist.
    /// 
    /// ***DatacenterMismatch***: if the provided host and pool do not belong
    /// to the same datacenter.
    pub async fn check_vm_config_task(&self, spec: &crate::types::structs::VirtualMachineConfigSpec, vm: Option<&crate::types::structs::ManagedObjectReference>, host: Option<&crate::types::structs::ManagedObjectReference>, pool: Option<&crate::types::structs::ManagedObjectReference>, test_type: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CheckVmConfigRequestType {spec, vm, host, pool, test_type, };
        let path = format!("/VirtualMachineCompatibilityChecker/{moId}/CheckVmConfig_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct CheckCompatibilityRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
    pool: Option<&'a crate::types::structs::ManagedObjectReference>,
    test_type: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for CheckCompatibilityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CheckCompatibilityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CheckCompatibilityRequestTypeSer<'b, 'a> {
    data: &'b CheckCompatibilityRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CheckCompatibilityRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CheckCompatibilityRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.pool else { continue; };
                    return Some((std::borrow::Cow::Borrowed("pool"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.test_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("testType"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CheckPowerOnRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
    pool: Option<&'a crate::types::structs::ManagedObjectReference>,
    test_type: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for CheckPowerOnRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CheckPowerOnRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CheckPowerOnRequestTypeSer<'b, 'a> {
    data: &'b CheckPowerOnRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CheckPowerOnRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CheckPowerOnRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.pool else { continue; };
                    return Some((std::borrow::Cow::Borrowed("pool"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.test_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("testType"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CheckVmConfigRequestType<'a> {
    spec: &'a crate::types::structs::VirtualMachineConfigSpec,
    vm: Option<&'a crate::types::structs::ManagedObjectReference>,
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
    pool: Option<&'a crate::types::structs::ManagedObjectReference>,
    test_type: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for CheckVmConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CheckVmConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CheckVmConfigRequestTypeSer<'b, 'a> {
    data: &'b CheckVmConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CheckVmConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CheckVmConfigRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.vm else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vm"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.pool else { continue; };
                    return Some((std::borrow::Cow::Borrowed("pool"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.test_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("testType"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
