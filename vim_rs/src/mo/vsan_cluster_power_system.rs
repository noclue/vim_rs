use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanClusterPowerSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ClusterPowerContext = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: bool = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct PerformClusterPowerActionRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::PerformClusterPowerActionSpec,
}

impl<'a> miniserde::Serialize for PerformClusterPowerActionRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PerformClusterPowerActionRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PerformClusterPowerActionRequestTypeSer<'b, 'a> {
    data: &'b PerformClusterPowerActionRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for PerformClusterPowerActionRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PerformClusterPowerActionRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryClusterPowerContextRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryClusterPowerContextRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryClusterPowerContextRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryClusterPowerContextRequestTypeSer<'b, 'a> {
    data: &'b QueryClusterPowerContextRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryClusterPowerContextRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryClusterPowerContextRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateClusterPowerStatusRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    status: &'a str,
}

impl<'a> miniserde::Serialize for UpdateClusterPowerStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateClusterPowerStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateClusterPowerStatusRequestTypeSer<'b, 'a> {
    data: &'b UpdateClusterPowerStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateClusterPowerStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateClusterPowerStatusRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("status"), &self.data.status as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
