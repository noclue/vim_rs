use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::VirtualMachineInstantCloneSpec;
use crate::types::ManagedObjectReference;
use crate::types::VirtualMachineCloneSpec;
use crate::types::VirtualMachinePowerStateEnum;
use crate::types::VirtualMachineRelocateSpec;
/// A singleton managed object that can answer questions about
/// the feasibility of certain provisioning operations.
pub struct VirtualMachineProvisioningChecker {
    client: Arc<VimClient>,
    mo_id: String,
}
impl VirtualMachineProvisioningChecker {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Tests the feasibility of a proposed
    /// *VirtualMachine.CloneVM_Task* operation.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The virtual machine we propose to clone.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### folder
    /// The location of the new virtual machine.
    /// 
    /// Refers instance of *Folder*.
    ///
    /// ### name
    /// The name of the new virtual machine.
    ///
    /// ### spec
    /// Specifies how to clone the virtual machine. In cases
    /// where DRS would automatically select a host, all potential
    /// hosts are tested against.
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
    /// ***InvalidArgument***: in the following cases:
    /// - the target host and target pool are not associated with the
    ///   same compute resource
    /// - the target pool represents a cluster without DRS enabled,
    ///   and the host is not specified
    /// - Datastore in a diskLocator entry is not specified
    /// - the specified device ID cannot be found in the virtual machine's current
    ///   configuration
    /// - the object specified in relocate cannot be found
    /// - the target pool is not specified while checking feasibility of
    ///   cloning to a different datacenter or a different vCenter
    ///   service
    /// - the datastore is not specified when testType parameter includes
    ///   datastore tests while checking feasibility of cloning to a
    ///   different datacenter or a different vCenter service
    ///   
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    pub async fn check_clone_task(&self, vm: &ManagedObjectReference, folder: &ManagedObjectReference, name: &str, spec: &VirtualMachineCloneSpec, test_type: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = CheckCloneRequestType {vm, folder, name, spec, test_type, };
        let path = format!("/VirtualMachineProvisioningChecker/{moId}/CheckClone_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Tests the feasibility of a proposed
    /// *VirtualMachine.InstantClone_Task* operation.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The virtual machine we propose to instant clone.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### spec
    /// Specifies how to instant clone the virtual machine.
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
    /// ***InvalidArgument***: in the following cases:
    /// - The destination host does not support Instant Clone.
    /// - The source and destination host are not the same.
    /// - The relocate spec in the Instant Clone spec has
    ///   Datastore set.
    /// - The relocate spec in the Instant Clone spec has host set.
    /// - The Instant clone spec does not have name set.
    /// - The source VM is a template.
    /// - The source VM is not powered on.
    /// - The source VM has PMEM devices/disks configured.
    ///   
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual
    /// machine configuration information is not available.
    pub async fn check_instant_clone_task(&self, vm: &ManagedObjectReference, spec: &VirtualMachineInstantCloneSpec, test_type: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = CheckInstantCloneRequestType {vm, spec, test_type, };
        let path = format!("/VirtualMachineProvisioningChecker/{moId}/CheckInstantClone_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Tests the feasibility of a proposed
    /// *VirtualMachine.MigrateVM_Task* operation.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The virtual machine we propose to migrate.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### host
    /// The target host on which the virtual machines will run. The host
    /// parameter may be left unset if the compute resource associated with
    /// the target pool represents a stand-alone host or a DRS-enabled
    /// cluster. In the former case the stand-alone host is used as the
    /// target host. In the latter case, each connected host in the cluster
    /// that is not in maintenance mode is tested as a target host.
    /// If the virtual machine is a template then either this
    /// parameter or the pool parameter must be set.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### pool
    /// The target resource pool for the virtual machines. If the
    /// pool parameter is left unset, the target pool for each particular
    /// virtual machine's migration will be that virtual machine's current
    /// pool. If the virtual machine is a template then either this
    /// parameter or the host parameter must be set.
    /// The pool parameter must be set for testing the feasibility of
    /// migration to a different datacenter or different vCenter service.
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ### state
    /// The power state that the virtual machines must have. If
    /// this argument is not set, each virtual machine is evaluated
    /// according to its current power state.
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
    /// ***InvalidArgument***: if the target host(s) and target pool for a
    /// migration are not associated with the same compute resource,
    /// or if the host parameter is left unset when the target pool is
    /// associated with a non-DRS cluster.
    /// 
    /// ***InvalidPowerState***: if the state argument is set and at least one
    /// of the specified virtual machines is not in that power state.
    pub async fn check_migrate_task(&self, vm: &ManagedObjectReference, host: Option<&ManagedObjectReference>, pool: Option<&ManagedObjectReference>, state: Option<VirtualMachinePowerStateEnum>, test_type: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = CheckMigrateRequestType {vm, host, pool, state, test_type, };
        let path = format!("/VirtualMachineProvisioningChecker/{moId}/CheckMigrate_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Tests the feasibility of a proposed
    /// *VirtualMachine.RelocateVM_Task* operation.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The virtual machine we propose to relocate.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### spec
    /// The specification of where to relocate the virtual machine.
    /// In cases where DRS would automatically select a host, all potential
    /// hosts are tested against.
    /// The host parameter in the spec may be left unset for checking
    /// feasibility of relocation to a different datacenter or different
    /// vCenter service, if the compute resource associated with the
    /// target pool represents a stand-alone host, the host is tested
    /// against, otherwise each connected host in the cluster that is
    /// not in maintenance mode represented by the target pool is tested
    /// as a target host.
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
    /// ***NotSupported***: if the virtual machine is marked as a template.
    /// 
    /// ***InvalidArgument***: in the following cases:
    /// - the target host and target pool are not associated with the
    ///   same compute resource
    /// - the target pool represents a cluster without DRS enabled,
    ///   and the host is not specified
    /// - Datastore in a diskLocator entry is not specified
    /// - the specified device ID cannot be found in the virtual machine's current
    ///   configuration
    /// - the object specified in relocate cannot be found
    /// - the target pool is not specified while checking feasibility of
    ///   relocation to a different datacenter or different vCenter
    ///   service
    /// - the datastore is not specified when testType parameter includes
    ///   datastore tests while checking feasibility of relocation to a
    ///   different datacenter or a different vCenter service
    ///   
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// host or virtual machine's current state. For example, if the host is in
    /// maintenance mode, or if the virtual machine's configuration information
    /// is not available.
    pub async fn check_relocate_task(&self, vm: &ManagedObjectReference, spec: &VirtualMachineRelocateSpec, test_type: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = CheckRelocateRequestType {vm, spec, test_type, };
        let path = format!("/VirtualMachineProvisioningChecker/{moId}/CheckRelocate_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Investigates the general VMotion compatibility of a set of virtual machines
    /// with a set of hosts.
    /// 
    /// The virtual machine may be in any power state. Hosts
    /// may be in any connection state and also may be in maintenance mode.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The set of virtual machines to analyze for compatibility. All
    /// virtual machines are assumed to be powered-on for the purposes of
    /// this operation.
    /// 
    /// Refers instances of *VirtualMachine*.
    ///
    /// ### host
    /// The set of hosts to analyze for compatibility. All hosts
    /// are assumed to be connected and not in maintenance mode for the
    /// purposes of this operation.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn query_v_motion_compatibility_ex_task(&self, vm: &[ManagedObjectReference], host: &[ManagedObjectReference]) -> Result<ManagedObjectReference> {
        let input = QueryVMotionCompatibilityExRequestType {vm, host, };
        let path = format!("/VirtualMachineProvisioningChecker/{moId}/QueryVMotionCompatibilityEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckCloneRequestType<'a> {
    vm: &'a ManagedObjectReference,
    folder: &'a ManagedObjectReference,
    name: &'a str,
    spec: &'a VirtualMachineCloneSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "testType")]
    test_type: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckInstantCloneRequestType<'a> {
    vm: &'a ManagedObjectReference,
    spec: &'a VirtualMachineInstantCloneSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "testType")]
    test_type: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckMigrateRequestType<'a> {
    vm: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pool: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    state: Option<VirtualMachinePowerStateEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "testType")]
    test_type: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckRelocateRequestType<'a> {
    vm: &'a ManagedObjectReference,
    spec: &'a VirtualMachineRelocateSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "testType")]
    test_type: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVMotionCompatibilityExRequestType<'a> {
    vm: &'a [ManagedObjectReference],
    host: &'a [ManagedObjectReference],
}
