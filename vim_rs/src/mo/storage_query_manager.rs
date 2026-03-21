use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object is used to query vCenter Server's storage system
/// entities.
#[derive(Clone)]
pub struct StorageQueryManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl StorageQueryManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Query the set of all hosts which have the specified lun attached.
    /// 
    /// Requires Host.Config.Storage privilege on the hosts which have
    /// the lun in attached state.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### lun_uuid
    /// The UUID of the ScsiLun device.
    ///
    /// ## Returns:
    ///
    /// HostSystem The set of hosts which have the specified lun attached.
    /// No values are returned if there are no hosts with the
    /// specified lun in attached state.
    /// 
    /// Refers instances of *HostSystem*.
    pub async fn query_hosts_with_attached_lun(&self, lun_uuid: &str) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = QueryHostsWithAttachedLunRequestType {lun_uuid, };
        let bytes_opt = self.client.invoke_optional("", "StorageQueryManager", &self.mo_id, "QueryHostsWithAttachedLun", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct QueryHostsWithAttachedLunRequestType<'a> {
    lun_uuid: &'a str,
}

impl<'a> miniserde::Serialize for QueryHostsWithAttachedLunRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryHostsWithAttachedLunRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryHostsWithAttachedLunRequestTypeSer<'b, 'a> {
    data: &'b QueryHostsWithAttachedLunRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryHostsWithAttachedLunRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryHostsWithAttachedLunRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("lunUuid"), &self.data.lun_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
