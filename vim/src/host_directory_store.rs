use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::HostAuthenticationStoreInfoTrait;
/// *HostDirectoryStore* is a base class for
/// directory-based authentication stores.
pub struct HostDirectoryStore {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HostDirectoryStore {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Information about the authentication store.
    pub async fn info(&self) -> Result<Box<dyn HostAuthenticationStoreInfoTrait>> {
        let path = format!("/HostDirectoryStore/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
