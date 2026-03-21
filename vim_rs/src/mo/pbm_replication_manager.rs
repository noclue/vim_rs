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
        let bytes_opt = self.client.invoke_optional("pbm", "PbmReplicationManager", &self.mo_id, "PbmQueryReplicationGroups", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct PbmQueryReplicationGroupsRequestType<'a> {
    entities: Option<&'a [crate::types::structs::PbmServerObjectRef]>,
}

impl<'a> miniserde::Serialize for PbmQueryReplicationGroupsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PbmQueryReplicationGroupsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PbmQueryReplicationGroupsRequestTypeSer<'b, 'a> {
    data: &'b PbmQueryReplicationGroupsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for PbmQueryReplicationGroupsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PbmQueryReplicationGroupsRequestType")),
                1 => {
                    let Some(ref val) = self.data.entities else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entities"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
