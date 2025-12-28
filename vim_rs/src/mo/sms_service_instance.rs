use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Service interface for the Storage Monitoring Service.
#[derive(Clone)]
pub struct SmsServiceInstance {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl SmsServiceInstance {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Retrieves information about the service.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Returns:
    ///
    /// AboutInfo information about the system
    pub async fn query_about_info(&self) -> Result<crate::types::structs::SmsAboutInfo> {
        let path = format!("/sms/SmsServiceInstance/{moId}/QueryAboutInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::SmsAboutInfo = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Retrieves SMS Session Manager managed object.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// A managed object *SmsSessionManager* reference.
    /// 
    /// Refers instance of *SmsSessionManager*.
    pub async fn query_session_manager(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/sms/SmsServiceInstance/{moId}/QuerySessionManager", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Retrieves Storage Manager managed object.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Returns:
    ///
    /// A managed object *SmsStorageManager* reference.
    /// 
    /// Refers instance of *SmsStorageManager*.
    pub async fn query_storage_manager(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/sms/SmsServiceInstance/{moId}/QueryStorageManager", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
}
