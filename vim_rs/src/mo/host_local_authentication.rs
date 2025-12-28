use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *HostLocalAuthentication* managed object represents
/// local authentication for user accounts on an ESX host.
#[derive(Clone)]
pub struct HostLocalAuthentication {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostLocalAuthentication {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Information about the authentication store.
    pub async fn info(&self) -> Result<Box<dyn crate::types::traits::HostAuthenticationStoreInfoTrait>> {
        let path = format!("/HostLocalAuthentication/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: Box<dyn crate::types::traits::HostAuthenticationStoreInfoTrait> = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
}
