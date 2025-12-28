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
        let path = format!("/StorageQueryManager/{moId}/QueryHostsWithAttachedLun", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::ManagedObjectReference>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryHostsWithAttachedLunRequestType<'a> {
    #[serde(rename = "lunUuid")]
    lun_uuid: &'a str,
}
