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
        let path = format!("/pbm/PbmServiceInstance/{moId}/PbmRetrieveServiceContent", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::PbmServiceInstanceContent = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Contains references to Storage Policy managed objects.
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn content(&self) -> Result<crate::types::structs::PbmServiceInstanceContent> {
        let path = format!("/pbm/PbmServiceInstance/{moId}/content", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::PbmServiceInstanceContent = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
