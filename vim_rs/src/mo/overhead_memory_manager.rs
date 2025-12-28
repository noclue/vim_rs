use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Provide static VM overhead memory values for (vm, host) pairs in
/// Virtual Center.
#[derive(Clone)]
pub struct OverheadMemoryManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl OverheadMemoryManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Return static VM overhead memory value in bytes for a (vm, host) pair from
    /// the overhead memory module (OMM) in Virtual Center.
    /// 
    /// ***Required privileges:*** Global.VCServer
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The Virtual Machine managed object reference.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### host
    /// The Host managed object reference.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Overhead memory value, if found in the OMM.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the overhead memory value is not found in the OMM.
    /// 
    /// ***InvalidType***: If the MoRefs do not point to appropriate type of
    /// inventory objects - VM and Host respectively.
    /// 
    /// ***InvalidArgument***: If any of the MoRefs are NULL.
    /// 
    /// ***ManagedObjectNotFound***: If the inventory objects cannot be found.
    pub async fn lookup_vm_overhead_memory(&self, vm: &crate::types::structs::ManagedObjectReference, host: &crate::types::structs::ManagedObjectReference) -> Result<i64> {
        let input = LookupVmOverheadMemoryRequestType {vm, host, };
        let path = format!("/OverheadMemoryManager/{moId}/LookupVmOverheadMemory", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: i64 = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct LookupVmOverheadMemoryRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    host: &'a crate::types::structs::ManagedObjectReference,
}
