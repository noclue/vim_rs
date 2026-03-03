use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The VsanCapabilitySystem exposes interfaces to retrieve the supported
/// capabilities on the current system.
/// 
/// The Managed Entity can be accessed
/// through MOID of vsan-vc-capability-system at vCenter server side or
/// vsan-capability-system at ESXi server side.
#[derive(Clone)]
pub struct VsanCapabilitySystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanCapabilitySystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Vec<crate::types::structs::VsanCapability> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct VsanGetCapabilitiesRequestType<'a> {
    targets: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for VsanGetCapabilitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanGetCapabilitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanGetCapabilitiesRequestTypeSer<'b, 'a> {
    data: &'b VsanGetCapabilitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanGetCapabilitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanGetCapabilitiesRequestType")),
                1 => {
                    let Some(ref val) = self.data.targets else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targets"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
