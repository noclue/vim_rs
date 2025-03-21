use std::sync::Arc;
use crate::core::client::{Client, Result};
/// The *HostAuthenticationStore* base class represents both local user
/// and host Active Directory authentication for an ESX host.
/// - Local user authentication is always enabled. The vSphere API
///   does not support local user configuration for a host.
/// - Active Directory authentication for ESX hosts relies on
///   an established Active Directory account that
///   has the authority to add the host to a domain.
pub struct HostAuthenticationStore {
    client: Arc<Client>,
    mo_id: String,
}
impl HostAuthenticationStore {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Information about the authentication store.
    pub async fn info(&self) -> Result<Box<dyn crate::types::traits::HostAuthenticationStoreInfoTrait>> {
        let path = format!("/HostAuthenticationStore/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
}
