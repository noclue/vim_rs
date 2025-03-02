use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::VsanUpgradeSystemPreflightCheckResult;
use crate::types::structs::VsanUpgradeSystemUpgradeStatus;
/// VSAN Upgrade System.
/// 
/// Used to perform and monitor VSAN on-disk format
/// upgrades.
pub struct VsanUpgradeSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl VsanUpgradeSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Start VSAN on-disk format upgrade process on a particular cluster.
    /// 
    /// In order to perform this on-disk format upgrade, the upgrade process
    /// will perform a rolling evacuation/remove/re-add operation to accomplish
    /// the upgrade. In other words, one disk group at a time, it will evacuate
    /// the data from the disk group, then remove the old format from the now
    /// empty disk group, then reformat the disk group with the new format.
    /// Once all disk groups have been upgraded, and if the performObjectUpgrade
    /// parameter is set, the VSAN object version is also upgraded. Before
    /// the object version is upgraded, it is possible to downgrade the cluster
    /// by passing the downgradeFormat parameter. Once objects are of the new
    /// object version however, downgrade (and thus rollback) are no longer
    /// possible. The new object version is required to allow objects to benefit
    /// from new VSAN features.
    /// This is a long running (hours to days) task. In addition to normal
    /// task progress reporting, use the queryUpgradeStatus() API which allows
    /// to retrieve in-depth status updates from the upgrade process. In there
    /// will be a detailed log of every operation the upgrade process has taken
    /// or issues it encountered. Some are simple log messages, others refer
    /// to operations like evacuating a disk group. For such log entries, the
    /// task object of the evacuation task is provided to allow "sub-task"
    /// tracking.
    /// Before starting, the upgrade process will perform a pre-flight check,
    /// and abort if any of the pre-conditions are not met. See
    /// 
    /// See also *VsanUpgradeSystem.PerformVsanUpgradePreflightCheck*for details on the pre-conditions being checked for.
    /// The upgrade process performs additional "pre-flight checks" before
    /// proceeding to upgrade the next host. The upgrade process will be halted
    /// if any of those pre-flight checks fail.
    /// If the upgrade process has been halted due to a problem, or even due to
    /// a crash or other failure, it can be re-started at any point in time.
    /// The upgrade will resume where it left off and only do the parts that
    /// are still outstanding. If the upgrade process stopped after removing
    /// VSAN from a disk group, but before re-adding those disks to VSAN, the
    /// upgrade process can recover from that. The pre-flight check results
    /// indicate such a condition. The upgrade process will however only re-add
    /// those disks if the restoreBackup parameter is passed in as true.
    /// Privilege "Host.Config.Storage" on all hosts under specified cluster
    /// is required..
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster to be upgraded
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### perform_object_upgrade
    /// After all disk groups have been updated, also
    /// upgrade all objects. Once started, rollback
    /// of the on disk format is no longer possible.
    /// Object upgrade unlocks new VSAN features.
    ///
    /// ### downgrade_format
    /// Perform a on-disk format downgrade instead of
    /// upgrade. Only possible if no upgraded objects exist.
    ///
    /// ### allow_reduced_redundancy
    /// Removes the need for one disk group worth of
    /// free space, by allowing reduced redundancy
    /// during disk upgrade.
    ///
    /// ### exclude_hosts
    /// Internal debug option meant for functional testing
    /// of VSAN upgrades. Skips upgrade on certain hosts and
    /// implies performObjectUpgrade being false. Should not
    /// be used by customers.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn perform_vsan_upgrade_task(&self, cluster: &ManagedObjectReference, perform_object_upgrade: Option<bool>, downgrade_format: Option<bool>, allow_reduced_redundancy: Option<bool>, exclude_hosts: Option<&[ManagedObjectReference]>) -> Result<ManagedObjectReference> {
        let input = PerformVsanUpgradeRequestType {cluster, perform_object_upgrade, downgrade_format, allow_reduced_redundancy, exclude_hosts, };
        let path = format!("/VsanUpgradeSystem/{moId}/PerformVsanUpgrade_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Perform an upgrade pre-flight check on a cluster.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster for which to perform the check.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### downgrade_format
    /// Intend to perform a on-disk format downgrade instead
    /// of upgrade. Adds additional checks.
    ///
    /// ## Returns:
    ///
    /// Pre-flight check result.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn perform_vsan_upgrade_preflight_check(&self, cluster: &ManagedObjectReference, downgrade_format: Option<bool>) -> Result<VsanUpgradeSystemPreflightCheckResult> {
        let input = PerformVsanUpgradePreflightCheckRequestType {cluster, downgrade_format, };
        let path = format!("/VsanUpgradeSystem/{moId}/PerformVsanUpgradePreflightCheck", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Retrieve the latest status of a running, or the previously completed,
    /// upgrade process.
    /// 
    /// Information about previous upgrade runs are not
    /// always, e.g. when VC gets restarted.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster for which to retrieve the upgrade status.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// Status
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn query_vsan_upgrade_status(&self, cluster: &ManagedObjectReference) -> Result<VsanUpgradeSystemUpgradeStatus> {
        let input = QueryVsanUpgradeStatusRequestType {cluster, };
        let path = format!("/VsanUpgradeSystem/{moId}/QueryVsanUpgradeStatus", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PerformVsanUpgradeRequestType<'a> {
    cluster: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "performObjectUpgrade")]
    perform_object_upgrade: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "downgradeFormat")]
    downgrade_format: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowReducedRedundancy")]
    allow_reduced_redundancy: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "excludeHosts")]
    exclude_hosts: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PerformVsanUpgradePreflightCheckRequestType<'a> {
    cluster: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "downgradeFormat")]
    downgrade_format: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVsanUpgradeStatusRequestType<'a> {
    cluster: &'a ManagedObjectReference,
}
