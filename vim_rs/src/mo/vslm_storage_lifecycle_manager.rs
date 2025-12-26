use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Interface to manage storage inventory in VSLM service.
/// 
/// VSLM maintains inventory of VStorageObjects present on all the datastores
/// connected to VC. If there is a change in datastore membership in VC that has
/// to be updated in VSLM as well.
/// APIs in this class are called as callback functions when Datastore
/// membership in VC changes.
#[derive(Clone)]
pub struct VslmStorageLifecycleManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VslmStorageLifecycleManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Query the mapping of *Datacenter* and *Datastore*
    /// for a specified datastore url.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datastore_url
    /// The datastore URL as specified in
    /// *DatastoreInfo.url*
    ///
    /// ## Returns:
    ///
    /// Returns array of *VslmQueryDatastoreInfoResult*
    /// representing the mapping between datastores and datacenters.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the specified datastoreUrl cannot be found.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_query_datastore_info(&self, datastore_url: &str) -> Result<Option<Vec<crate::types::structs::VslmQueryDatastoreInfoResult>>> {
        let input = VslmQueryDatastoreInfoRequestType {datastore_url, };
        let path = format!("/vslm/VslmStorageLifecycleManager/{moId}/VslmQueryDatastoreInfo", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::VslmQueryDatastoreInfoResult>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Sync the FCD info on the passed in datastore.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datastore_url
    /// The datastore URL as specified in
    /// *DatastoreInfo.url*
    ///
    /// ### full_sync
    /// If this is set to true, all information for this datastore
    /// will be discarded from the catalog and reloaded from the
    /// datastore's catalog
    ///
    /// ### fcd_id
    /// If set, this call blocks until fcdId is persisited into db
    /// if this fcdId is not found in queue, assume persisted and return
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore.
    /// 
    /// ***NotFound***: If matching datastore could not be found for the given
    /// datastoreMoId.
    /// 
    /// ***VslmSyncFault***: If an exception occured during datastore sync.
    /// 
    /// ***VslmFault***: If a VSLM internal server error occurred.
    pub async fn vslm_sync_datastore(&self, datastore_url: &str, full_sync: bool, fcd_id: Option<&crate::types::structs::Id>) -> Result<()> {
        let input = VslmSyncDatastoreRequestType {datastore_url, full_sync, fcd_id, };
        let path = format!("/vslm/VslmStorageLifecycleManager/{moId}/VslmSyncDatastore", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmQueryDatastoreInfoRequestType<'a> {
    #[serde(rename = "datastoreUrl")]
    datastore_url: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VslmSyncDatastoreRequestType<'a> {
    #[serde(rename = "datastoreUrl")]
    datastore_url: &'a str,
    #[serde(rename = "fullSync")]
    full_sync: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fcdId")]
    fcd_id: Option<&'a crate::types::structs::Id>,
}
