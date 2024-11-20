use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::ManagedObjectReference;
use crate::types::VirtualMachineConfigSpec;
/// A singleton managed object that can answer questions about compatibility
/// of a virtual machine with a host.
pub struct VirtualMachineCompatibilityChecker {
    client: Arc<VimClient>,
    mo_id: String,
}
impl VirtualMachineCompatibilityChecker {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
    pub async fn check_compatibility_task(&self, vm: &ManagedObjectReference, host: Option<&ManagedObjectReference>, pool: Option<&ManagedObjectReference>, test_type: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = CheckCompatibilityRequestType {vm, host, pool, test_type, };
        let path = format!("/VirtualMachineCompatibilityChecker/{moId}/CheckCompatibility_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn check_power_on_task(&self, vm: &ManagedObjectReference, host: Option<&ManagedObjectReference>, pool: Option<&ManagedObjectReference>, test_type: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = CheckPowerOnRequestType {vm, host, pool, test_type, };
        let path = format!("/VirtualMachineCompatibilityChecker/{moId}/CheckPowerOn_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn check_vm_config_task(&self, spec: &VirtualMachineConfigSpec, vm: Option<&ManagedObjectReference>, host: Option<&ManagedObjectReference>, pool: Option<&ManagedObjectReference>, test_type: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = CheckVmConfigRequestType {spec, vm, host, pool, test_type, };
        let path = format!("/VirtualMachineCompatibilityChecker/{moId}/CheckVmConfig_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckCompatibilityRequestType<'a> {
    vm: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pool: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "testType")]
    test_type: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckPowerOnRequestType<'a> {
    vm: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pool: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "testType")]
    test_type: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckVmConfigRequestType<'a> {
    spec: &'a VirtualMachineConfigSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    vm: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pool: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "testType")]
    test_type: Option<&'a [String]>,
}
