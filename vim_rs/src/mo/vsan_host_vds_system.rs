use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// vSAN optimized methods for assisting VDS related operations, especially
/// migrations from VSS to VDS.
/// 
/// In every ESXi host there is a singleton instance of this class
/// with the Managed Object ID of 'ha-vsan-host-vds-system'.
#[derive(Clone)]
pub struct VsanHostVdsSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanHostVdsSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Mark the jobID with "rollback" or "done" flag.
    /// 
    /// If it is "done", the migration
    /// is considered completed. But if it is marked with "rollback", all the change
    /// made by previous call of "VsanMigrateVmsToVds" to the network will be rolled
    /// back.
    /// 
    /// ***Required privileges:*** Network.Assign
    ///
    /// ## Parameters:
    ///
    /// ### job_id
    /// Unique identifier returned by VsanMigrateVmsToVds()
    ///
    /// ### new_state
    /// Indicates if the migration is considered completed or
    /// should be rolled back.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: Task not found
    /// 
    /// ***VsanFault***: Any unexpected runtime error.
    pub async fn vsan_complete_migrate_vms_to_vds(&self, job_id: &str, new_state: &str) -> Result<()> {
        let input = VsanCompleteMigrateVmsToVdsRequestType {job_id, new_state, };
        let path = format!("/vsan/VsanHostVdsSystem/{moId}/VsanCompleteMigrateVmsToVds", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Make all the VMs on the host to adopt the VDS network.
    /// 
    /// ***Required privileges:*** Network.Assign
    ///
    /// ## Parameters:
    ///
    /// ### vm_config_specs
    /// VMs to be migrated via associated specs
    ///
    /// ### vds_uuid
    /// UUID of the VDS that is being migrated to.
    ///
    /// ### timeout_sec
    /// Time in seconds. See above for timeout behavior.
    ///
    /// ### revert
    /// -
    ///
    /// ## Returns:
    ///
    /// A unique identifier referring to this operation
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: Task not found
    /// 
    /// ***VsanFault***: Any unexpected runtime error.
    pub async fn vsan_migrate_vms_to_vds(&self, vm_config_specs: &[crate::types::structs::VsanVmVdsMigrationSpec], vds_uuid: &str, timeout_sec: i64, revert: Option<bool>) -> Result<String> {
        let input = VsanMigrateVmsToVdsRequestType {vm_config_specs, vds_uuid, timeout_sec, revert, };
        let path = format!("/vsan/VsanHostVdsSystem/{moId}/VsanMigrateVmsToVds", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: String = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanCompleteMigrateVmsToVdsRequestType<'a> {
    #[serde(rename = "jobId")]
    job_id: &'a str,
    #[serde(rename = "newState")]
    new_state: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanMigrateVmsToVdsRequestType<'a> {
    #[serde(rename = "vmConfigSpecs")]
    vm_config_specs: &'a [crate::types::structs::VsanVmVdsMigrationSpec],
    #[serde(rename = "vdsUuid")]
    vds_uuid: &'a str,
    #[serde(rename = "timeoutSec")]
    timeout_sec: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    revert: Option<bool>,
}
