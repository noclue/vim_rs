use std::sync::Arc;
use crate::core::client::{Client, Result};
/// The VsanCapabilitySystem exposes interfaces to retrieve the supported
/// capabilities on the current system.
/// 
/// The Managed Entity can be accessed
/// through MOID of vsan-vc-capability-system at vCenter server side or
/// vsan-capability-system at ESXi server side.
#[derive(Clone)]
pub struct VsanCapabilitySystem {
    client: Arc<Client>,
    mo_id: String,
}
impl VsanCapabilitySystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Retrieves the supported capabilities on the current system.
    /// 
    /// The calculation
    /// is based on the available APIs and registered managed objects. This way the
    /// client can be sure if a certain feature is supported on the system or not.
    /// For scenarios like disconnected hosts, older version hosts than ESXi 6.0 U1,
    /// or unable to retrieve capabilities due to other reasons, along with empty
    /// capability set, status of target managed object at the retrieving time is
    /// also returned to explain the reason. For a host whose capabilities are
    /// calculated according to host version, a 'calculated' capability status is
    /// returned. For other cases, the statuses field in
    /// *VsanCapability*
    /// is omitted.
    ///
    /// ## Parameters:
    ///
    /// ### targets
    /// An optional list of targeted managed objects. The supported
    /// targets are HostSystem and ClusterComputeResource instances. If a HostSystem
    /// is given, the result contains information about the capabilities of this
    /// certain host. If a cluster is passed, the result contains information about
    /// the capabilities of all hosts that reside in the cluster and the capabilities
    /// of the vCenter. If the targets parameter is empty, the result contains only
    /// the capabilities of the current vCenter.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// A list of vim.cluster.VsanCapability.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_get_capabilities(&self, targets: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<Vec<crate::types::structs::VsanCapability>> {
        let input = VsanGetCapabilitiesRequestType {targets, };
        let path = format!("/vsan/VsanCapabilitySystem/{moId}/VsanGetCapabilities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanGetCapabilitiesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    targets: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}
