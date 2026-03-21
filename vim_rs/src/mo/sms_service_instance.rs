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
        let bytes = self.client.invoke("sms", "SmsServiceInstance", &self.mo_id, "QueryAboutInfo", None).await?;
        let result: crate::types::structs::SmsAboutInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes = self.client.invoke("sms", "SmsServiceInstance", &self.mo_id, "QuerySessionManager", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes = self.client.invoke("sms", "SmsServiceInstance", &self.mo_id, "QueryStorageManager", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
