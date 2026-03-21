use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This system provides interfaces to remediate inconsistency of vSAN configurations
/// at both cluster and host level.
/// 
/// It can be accessed through MOID vsan-cluster-mgmt-internal-system at vCenter side.
#[derive(Clone)]
pub struct VsanClusterMgmtInternalSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanClusterMgmtInternalSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let bytes = self.client.invoke("vsan", "VsanClusterMgmtInternalSystem", &self.mo_id, "VsanRemediateVsanCluster", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "VsanClusterMgmtInternalSystem", &self.mo_id, "VsanRemediateVsanHost", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct VsanRemediateVsanClusterRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanRemediateVsanClusterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanRemediateVsanClusterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanRemediateVsanClusterRequestTypeSer<'b, 'a> {
    data: &'b VsanRemediateVsanClusterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanRemediateVsanClusterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanRemediateVsanClusterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanRemediateVsanHostRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanRemediateVsanHostRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanRemediateVsanHostRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanRemediateVsanHostRequestTypeSer<'b, 'a> {
    data: &'b VsanRemediateVsanHostRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanRemediateVsanHostRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanRemediateVsanHostRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
