use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::VchaClusterHealth;
/// FailoverClusterManager provides operations to manage a vCenter
/// High Availability Cluster (VCHA Cluster).
/// 
/// A VCHA Cluster consists of three VMs. One is the Active vCenter VM that
/// serves client requests. Second is the Passive VM that is identical to the
/// Active vCenter VM in terms of resources and capabilities. Passive VM
/// constantly receives updates from Active VM and takes over the role of
/// Active vCenter VM in the event of failover. Third is the Witness VM that
/// acts as a quorum VM in a VCHA Cluster. Sole purpose of Witness VM is to
/// avoid classic split-brain problem in a VCHA Cluster.
/// A VCHA Cluster has following states -
/// 1\. Healthy - All three nodes in a VCHA Cluster are healthy and connected.
/// State replication between Active and Passive node is working and both
/// nodes are in-sync.
/// 2\. Degraded - A VCHA Cluster is said to be in degraded state when it has
/// lost one of the three nodes. Node loss can be due to various reasons and
/// as a result the lost node is not visible to other two nodes. If an Active
/// node is lost, Passive node will take the role of Active node. If Passive or
/// Witness node is lost, Active node will continue to serve requests. A VCHA
/// Cluster can also be in degraded state if state replication fails between
/// Active and Passive nodes.
/// 3\. Isolated - All three nodes are isolated from each other. If this happens
/// while VCHA Cluster is in enabled mode, Active node stops serving client
/// requests. If nodes are isolated in a disabled VCHA Cluster mode, Active
/// node continues to serve client requests.
/// A VCHA Cluster has following modes -
/// 1\. Enabled - State replication between Active and Passive nodes is
/// enabled and automatic failover happens if Active node fails while the
/// VCHA Cluster is in a healthy state.
/// 2\. Disabled - All three nodes are part of VCHA Cluster but state
/// replication and automatic failover is disabled.
/// 3\. Maintenance - All three nodes are part of VCHA Cluster but automatic
/// failover is disabled while state replication continues. Active node continues
/// to serve client requests even if Passive and Witness nodes are lost.
pub struct FailoverClusterManager {
    client: Arc<Client>,
    mo_id: String,
}
impl FailoverClusterManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Returns last known health of the VCHA Cluster.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn get_vcha_cluster_health(&self) -> Result<VchaClusterHealth> {
        let path = format!("/FailoverClusterManager/{moId}/GetVchaClusterHealth", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Returns current mode of a VCHA Cluster.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn get_cluster_mode(&self) -> Result<String> {
        let path = format!("/FailoverClusterManager/{moId}/getClusterMode", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Allows a caller to initiate a failover from Active vCenter Server node
    /// to the Passive node.
    /// 
    /// By default it is a forced failover. The planned
    /// flag can be used to initiate it as a planned failover.
    /// For forced failover, Active node immediately initiates a failover. This
    /// may result into a data loss after failover.
    /// For planned failover, Active node flushes all the state to the
    /// Passive node, waits for the flush to complete before causing a failover.
    /// After the failover, Passive node starts without any data loss.
    /// A failover is allowed only in the following cases -
    /// 1\. Cluster's mode is enabled and all cluster members are present.
    /// 2\. Cluster's mode is maintenance and all cluster members are present.
    /// API throws an exception in all other cases.
    /// 
    /// ***Required privileges:*** Global.VCServer
    ///
    /// ## Parameters:
    ///
    /// ### planned
    /// \- if false, a failover is initiated immediate and may
    /// result in data loss.
    /// if true, a failover is initated after the Active node
    /// flushes its state to Passive and there is no data loss.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn initiate_failover_task(&self, planned: bool) -> Result<ManagedObjectReference> {
        let input = InitiateFailoverRequestType {planned, };
        let path = format!("/FailoverClusterManager/{moId}/initiateFailover_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// setClusterMode method allows caller to manipulate the mode of a
    /// VCHA Cluster
    /// Following mode transitions are allowed -
    /// enabled -&gt; disabled - Allowed only in healthy and degraded states.
    /// 
    /// enabled -&gt; maintenance - Allowed only in healthy state.
    /// disabled -&gt; enabled - Allowed only in healthy state.
    /// maintenance -&gt; enabled - Allowed only in healthy state with all nodes
    /// are running the same version.
    /// maintenance -&gt; disabled - Allowed only in healthy state with all nodes
    /// are running the same version.
    /// All other transitions are not allowed.
    /// VCHA Cluster configuration remains intact in any of the cluster modes.
    /// 
    /// ***Required privileges:*** Global.VCServer
    ///
    /// ## Parameters:
    ///
    /// ### mode
    /// -
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the progress of the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn set_cluster_mode_task(&self, mode: &str) -> Result<ManagedObjectReference> {
        let input = SetClusterModeRequestType {mode, };
        let path = format!("/FailoverClusterManager/{moId}/setClusterMode_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// A list of method names that must not be called and will throw
    /// a fault due to some other method running that the disabled method
    /// can cause side-effects for.
    /// 
    /// This list may include the following methods:
    /// - *FailoverClusterManager.setClusterMode_Task*
    /// - *FailoverClusterManager.getClusterMode*
    /// - *FailoverClusterManager.initiateFailover_Task*
    /// - *FailoverClusterManager.GetVchaClusterHealth*
    ///   
    /// GetClusterHealth will also be disabled if Deploy is in progress.
    /// As with other disabled methods there will be no property updates
    /// on this property when called with non-zero property collector versions.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn disabled_cluster_method(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/FailoverClusterManager/{moId}/disabledClusterMethod", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(rename = "initiateFailoverRequestType", tag = "_typeName")]
struct InitiateFailoverRequestType {
    planned: bool,
}
#[derive(serde::Serialize)]
#[serde(rename = "setClusterModeRequestType", tag = "_typeName")]
struct SetClusterModeRequestType<'a> {
    mode: &'a str,
}
