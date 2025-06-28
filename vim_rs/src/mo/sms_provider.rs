use std::sync::Arc;
use crate::core::client::{Client, Result};
/// Provider interface for Storage Monitoring Service (SMS).
#[derive(Clone)]
pub struct SmsProvider {
    client: Arc<Client>,
    mo_id: String,
}
impl SmsProvider {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Get provider information.
    /// 
    /// ***Required privileges:*** StorageViews.View
    pub async fn query_provider_info(&self) -> Result<Box<dyn crate::types::traits::SmsProviderInfoTrait>> {
        let path = format!("/sms/SmsProvider/{moId}/QueryProviderInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
}
