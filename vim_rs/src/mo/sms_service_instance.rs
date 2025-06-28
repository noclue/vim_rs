use std::sync::Arc;
use crate::core::client::{Client, Result};
/// Service interface for the Storage Monitoring Service.
#[derive(Clone)]
pub struct SmsServiceInstance {
    client: Arc<Client>,
    mo_id: String,
}
impl SmsServiceInstance {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
        self.client.execute(req).await
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
        self.client.execute(req).await
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
        self.client.execute(req).await
    }
}
