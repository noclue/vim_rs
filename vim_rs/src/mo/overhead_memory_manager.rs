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
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: i64 = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct LookupVmOverheadMemoryRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    host: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for LookupVmOverheadMemoryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(LookupVmOverheadMemoryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct LookupVmOverheadMemoryRequestTypeSer<'b, 'a> {
    data: &'b LookupVmOverheadMemoryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for LookupVmOverheadMemoryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"LookupVmOverheadMemoryRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
