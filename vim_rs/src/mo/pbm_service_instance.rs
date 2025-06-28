use std::sync::Arc;
use crate::core::client::{Client, Result};
/// The *PbmServiceInstance* managed object is the root object of the
/// Storage Policy service.
/// 
/// After you connect to the Storage Policy Server,
/// you create a reference to the *PbmServiceInstance*, and use
/// that reference to retrieve the *PbmServiceInstanceContent*
/// data object. The *PbmServiceInstanceContent* object provides
/// access to the Storage Policy managed objects.
#[derive(Clone)]
pub struct PbmServiceInstance {
    client: Arc<Client>,
    mo_id: String,
}
impl PbmServiceInstance {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Retrieves the properties of the Storage Policy service instance.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// Service instance properties that provide access to
    /// Storage Policy managed objects.
    pub async fn pbm_retrieve_service_content(&self) -> Result<crate::types::structs::PbmServiceInstanceContent> {
        let path = format!("/pbm/PbmServiceInstance/{moId}/PbmRetrieveServiceContent", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Contains references to Storage Policy managed objects.
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn content(&self) -> Result<crate::types::structs::PbmServiceInstanceContent> {
        let path = format!("/pbm/PbmServiceInstance/{moId}/content", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
}
