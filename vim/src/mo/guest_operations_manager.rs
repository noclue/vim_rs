use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
/// GuestOperationsManager is the managed object that provides APIs
/// to manipulate the guest operating system files and process.
/// 
/// Each class of APIs is separated into its own manager.
/// Only one guest operation is allowed at a time per virtual machine.
pub struct GuestOperationsManager {
    client: Arc<Client>,
    mo_id: String,
}
impl GuestOperationsManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// A managed object that provides methods to support single sign-on
    /// in the guest operating system.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// Refers instance of *GuestAliasManager*.
    pub async fn alias_manager(&self) -> Result<ManagedObjectReference> {
        let path = format!("/GuestOperationsManager/{moId}/aliasManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// A singleton managed object that provides methods for guest authentication
    /// operations.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// Refers instance of *GuestAuthManager*.
    pub async fn auth_manager(&self) -> Result<ManagedObjectReference> {
        let path = format!("/GuestOperationsManager/{moId}/authManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// A singleton managed object that provides methods for guest file
    /// operations.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// Refers instance of *GuestFileManager*.
    pub async fn file_manager(&self) -> Result<ManagedObjectReference> {
        let path = format!("/GuestOperationsManager/{moId}/fileManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// A singleton managed object that provides methods for guest windows registry
    /// operations.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// Refers instance of *GuestWindowsRegistryManager*.
    pub async fn guest_windows_registry_manager(&self) -> Result<ManagedObjectReference> {
        let path = format!("/GuestOperationsManager/{moId}/guestWindowsRegistryManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// A singleton managed object that provides methods for guest process
    /// operations.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// Refers instance of *GuestProcessManager*.
    pub async fn process_manager(&self) -> Result<ManagedObjectReference> {
        let path = format!("/GuestOperationsManager/{moId}/processManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
