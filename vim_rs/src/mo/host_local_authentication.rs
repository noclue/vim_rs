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
        let pv_opt = self.client.fetch_property_raw("", "HostLocalAuthentication", &self.mo_id, "info").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property info was empty".to_string()))?;
        let result: Box<dyn crate::types::traits::HostAuthenticationStoreInfoTrait> = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
}
