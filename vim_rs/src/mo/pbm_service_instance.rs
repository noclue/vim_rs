use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl PbmServiceInstance {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let bytes = self.client.invoke("pbm", "PbmServiceInstance", &self.mo_id, "PbmRetrieveServiceContent", None).await?;
        let result: crate::types::structs::PbmServiceInstanceContent = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Contains references to Storage Policy managed objects.
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn content(&self) -> Result<crate::types::structs::PbmServiceInstanceContent> {
        let bytes_opt = self.client.fetch_property_raw("pbm", "PbmServiceInstance", &self.mo_id, "content").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property content was empty".to_string()))?;
        let result: crate::types::structs::PbmServiceInstanceContent = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
