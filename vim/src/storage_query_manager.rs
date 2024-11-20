use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::ManagedObjectReference;
/// This managed object is used to query vCenter Server's storage system
/// entities.
pub struct StorageQueryManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl StorageQueryManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
    pub async fn query_hosts_with_attached_lun(&self, lun_uuid: &str) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = QueryHostsWithAttachedLunRequestType {lun_uuid, };
        let path = format!("/StorageQueryManager/{moId}/QueryHostsWithAttachedLun", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryHostsWithAttachedLunRequestType<'a> {
    #[serde(rename = "lunUuid")]
    lun_uuid: &'a str,
}
