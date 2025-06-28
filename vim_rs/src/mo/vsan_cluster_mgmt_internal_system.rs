use std::sync::Arc;
use crate::core::client::{Client, Result};
/// This system provides interfaces to remediate inconsistency of vSAN configurations
/// at both cluster and host level.
/// 
/// It can be accessed through MOID vsan-cluster-mgmt-internal-system at vCenter side.
#[derive(Clone)]
pub struct VsanClusterMgmtInternalSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl VsanClusterMgmtInternalSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Remediate a cluster, to ensure vSAN cluster state matches vpxd cluster state,
    /// and also guarantee vSAN state of all member hosts is updated if required.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vim.ClusterComputeResource to remediate.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// vim.Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: when specified cluster doesn't exist.
    pub async fn vsan_remediate_vsan_cluster(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanRemediateVsanClusterRequestType {cluster, };
        let path = format!("/vsan/VsanClusterMgmtInternalSystem/{moId}/VsanRemediateVsanCluster", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Remediate a single standalone host.
    /// 
    /// Will ensure the vSAN state of the host is
    /// updated if required. If the host was removed from a cluster, the cluster needs
    /// to remediate in order for the vSAN cluster to correctly reflect that the host
    /// is no longer part of the cluster.
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The target vim.HostSystem to remediate.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// vim.Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: when specified host doesn't exist.
    pub async fn vsan_remediate_vsan_host(&self, host: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanRemediateVsanHostRequestType {host, };
        let path = format!("/vsan/VsanClusterMgmtInternalSystem/{moId}/VsanRemediateVsanHost", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanRemediateVsanClusterRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanRemediateVsanHostRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
}
