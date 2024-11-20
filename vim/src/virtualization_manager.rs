use std::sync::Arc;
use crate::vim_client::{VimClient};
/// Deprecated as of VI API 2.5, use the VMware vCenter Converter,
/// an optional software plug-in for vCenter Server for
/// migrating physical and virtual machines to VMware vSphere.
/// 
/// The VirtualizationManager is the interface for discover and consolidate
/// host and services from physical environment to virtualization environment.
pub struct VirtualizationManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl VirtualizationManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
}
