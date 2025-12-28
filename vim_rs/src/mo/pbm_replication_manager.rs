use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *PbmReplicationManager* provides methods dealing with replication aspects of virtual
/// machine and virtual disk requirement profiles.
#[derive(Clone)]
pub struct PbmReplicationManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl PbmReplicationManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Returns identifiers for replication groups associated with virtual machines, virtual
    /// disks or virtual machines and all their disks.
    /// 
    /// If the query is performed for a virtual machine
    /// and all it's disks *virtualMachineAndDisks*, an entry per
    /// disk and one for the virtual machine config will be returned.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### entities
    /// Array of server object references. Valid types are
    /// *virtualMachine*,
    /// *virtualMachineAndDisks*,
    /// *virtualDiskId*,
    /// *virtualDiskUUID*
    ///
    /// ## Returns:
    ///
    /// Array of query result objects. Each *PbmQueryReplicationGroupResult*
    /// object identifies a virtual machine, or virtual disk and contains the replication group id
    /// associated with that entity, if any. It also describes the fault, if there is an error associated
    /// with one of the entities.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if <code>entities</code> is null or empty.
    /// 
    /// ***PbmFault***: If there is an internal service error.
    pub async fn pbm_query_replication_groups(&self, entities: Option<&[crate::types::structs::PbmServerObjectRef]>) -> Result<Option<Vec<crate::types::structs::PbmQueryReplicationGroupResult>>> {
        let input = PbmQueryReplicationGroupsRequestType {entities, };
        let path = format!("/pbm/PbmReplicationManager/{moId}/PbmQueryReplicationGroups", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmQueryReplicationGroupResult>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQueryReplicationGroupsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entities: Option<&'a [crate::types::structs::PbmServerObjectRef]>,
}
