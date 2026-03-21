use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *HostAuthenticationStore* base class represents both local user
/// and host Active Directory authentication for an ESX host.
/// - Local user authentication is always enabled. The vSphere API
///   does not support local user configuration for a host.
/// - Active Directory authentication for ESX hosts relies on
///   an established Active Directory account that
///   has the authority to add the host to a domain.
#[derive(Clone)]
pub struct HostAuthenticationStore {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostAuthenticationStore {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Information about the authentication store.
    pub async fn info(&self) -> Result<Box<dyn crate::types::traits::HostAuthenticationStoreInfoTrait>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostAuthenticationStore", &self.mo_id, "info").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property info was empty".to_string()))?;
        let result: Box<dyn crate::types::traits::HostAuthenticationStoreInfoTrait> = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
