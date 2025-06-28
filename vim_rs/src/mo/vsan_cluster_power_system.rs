use std::sync::Arc;
use crate::core::client::{Client, Result};
/// This managed object type provides the service interface for vsan cluster
/// power action.
/// 
/// i.e. power off a whole cluster, query current cluster power
/// context, power on the cluster, etc. The vSAN cluster power system will be
/// supported in both of VC and ESXi host.
/// When the ManagedEntity is accessed with MOID of 'vsan-cluster-power-system'
/// through vSAN service at vCenter server, it acts as cluster-level APIs.
/// When it accessed with MOID of 'ha-vsan-power-system' through vSAN service
/// at ESXi host side, its scope is only limited to that host.
#[derive(Clone)]
pub struct VsanClusterPowerSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl VsanClusterPowerSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Execute cluster power off or power on action.
    /// 
    /// When it's called from VC, it will acts as cluster level API to perform the
    /// cluster power action.
    /// When it's called from host, if it's an orchestration host, it will act the
    /// similar role of VC to orchestrate the cluster power worflow. Otherwise, it
    /// will execute specific host power actions such as power off according to spec
    /// 
    /// ***Required privileges:*** Host.Config.Power Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster where to take power action.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### spec
    /// Indicate the detailed power action specification.
    ///
    /// ## Returns:
    ///
    /// A task object tracking the power action. In case there is something
    /// wrong, the task would contain the detailed error message and the
    /// error steps.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// power off the cluster without powerOffReason.
    /// 
    /// ***VsanFault***: Exception for generic vSAN related errors, for example,
    /// some hosts are disconnected when starting to power off.
    pub async fn perform_cluster_power_action(&self, cluster: &crate::types::structs::ManagedObjectReference, spec: &crate::types::structs::PerformClusterPowerActionSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = PerformClusterPowerActionRequestType {cluster, spec, };
        let path = format!("/vsan/VsanClusterPowerSystem/{moId}/PerformClusterPowerAction", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query the ClusterPowerContext.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster which to query ClusterPowerContext.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// ClusterPowerContext
    pub async fn query_cluster_power_context(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ClusterPowerContext> {
        let input = QueryClusterPowerContextRequestType {cluster, };
        let path = format!("/vsan/VsanClusterPowerSystem/{moId}/QueryClusterPowerContext", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Update the current cluster power status.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster which to update the power status.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### status
    /// The target status needs to be set.
    ///
    /// ## Returns:
    ///
    /// a boolean indicates success or not.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***VsanFault***: Exception for generic vSAN related errors, for example,
    /// trying to update the power status when there is a running
    /// power action task.
    pub async fn update_cluster_power_status(&self, cluster: &crate::types::structs::ManagedObjectReference, status: &str) -> Result<bool> {
        let input = UpdateClusterPowerStatusRequestType {cluster, status, };
        let path = format!("/vsan/VsanClusterPowerSystem/{moId}/UpdateClusterPowerStatus", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PerformClusterPowerActionRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::PerformClusterPowerActionSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryClusterPowerContextRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateClusterPowerStatusRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    status: &'a str,
}
