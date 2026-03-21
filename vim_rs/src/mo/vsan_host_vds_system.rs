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
        self.client.invoke_void("vsan", "VsanHostVdsSystem", &self.mo_id, "VsanCompleteMigrateVmsToVds", Some(&input)).await
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
        let bytes = self.client.invoke("vsan", "VsanHostVdsSystem", &self.mo_id, "VsanMigrateVmsToVds", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct VsanCompleteMigrateVmsToVdsRequestType<'a> {
    job_id: &'a str,
    new_state: &'a str,
}

impl<'a> miniserde::Serialize for VsanCompleteMigrateVmsToVdsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanCompleteMigrateVmsToVdsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanCompleteMigrateVmsToVdsRequestTypeSer<'b, 'a> {
    data: &'b VsanCompleteMigrateVmsToVdsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanCompleteMigrateVmsToVdsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanCompleteMigrateVmsToVdsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("jobId"), &self.data.job_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("newState"), &self.data.new_state as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanMigrateVmsToVdsRequestType<'a> {
    vm_config_specs: &'a [crate::types::structs::VsanVmVdsMigrationSpec],
    vds_uuid: &'a str,
    timeout_sec: i64,
    revert: Option<bool>,
}

impl<'a> miniserde::Serialize for VsanMigrateVmsToVdsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanMigrateVmsToVdsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanMigrateVmsToVdsRequestTypeSer<'b, 'a> {
    data: &'b VsanMigrateVmsToVdsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanMigrateVmsToVdsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanMigrateVmsToVdsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vmConfigSpecs"), &self.data.vm_config_specs as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("vdsUuid"), &self.data.vds_uuid as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("timeoutSec"), &self.data.timeout_sec as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.revert else { continue; };
                    return Some((std::borrow::Cow::Borrowed("revert"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
