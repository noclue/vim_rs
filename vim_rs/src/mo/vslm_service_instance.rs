use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *VslmServiceInstance* managed object is the root object of the
/// vSphere Storage Lifecycle Management(VSLM) service.
/// 
/// After you connect to
/// VSLM Server, you create a reference to the *VslmServiceInstance*, and use
/// that reference to retrieve the *VslmServiceInstanceContent* data
/// object. The *VslmServiceInstanceContent* object provides access to
/// VSLM managed objects.
#[derive(Clone)]
pub struct VslmServiceInstance {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VslmServiceInstance {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Retrieves the properties of the Storage Lifecycle Management service
    /// instance.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// Service instance properties that provide access to
    /// Storage Lifecycle Management managed objects.
    pub async fn retrieve_content(&self) -> Result<crate::types::structs::VslmServiceInstanceContent> {
        let path = format!("/vslm/VslmServiceInstance/{moId}/RetrieveContent", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VslmServiceInstanceContent = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Contains references to Storage Lifecycle Management managed objects.
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn content(&self) -> Result<crate::types::structs::VslmServiceInstanceContent> {
        let path = format!("/vslm/VslmServiceInstance/{moId}/content", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VslmServiceInstanceContent = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
