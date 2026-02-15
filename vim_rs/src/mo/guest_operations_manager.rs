use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// GuestOperationsManager is the managed object that provides APIs
/// to manipulate the guest operating system files and process.
/// 
/// Each class of APIs is separated into its own manager.
/// Only one guest operation is allowed at a time per virtual machine.
#[derive(Clone)]
pub struct GuestOperationsManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl GuestOperationsManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn alias_manager(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let path = format!("/GuestOperationsManager/{moId}/aliasManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::ManagedObjectReference>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// A singleton managed object that provides methods for guest authentication
    /// operations.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// Refers instance of *GuestAuthManager*.
    pub async fn auth_manager(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let path = format!("/GuestOperationsManager/{moId}/authManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::ManagedObjectReference>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// A singleton managed object that provides methods for guest file
    /// operations.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// Refers instance of *GuestFileManager*.
    pub async fn file_manager(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let path = format!("/GuestOperationsManager/{moId}/fileManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::ManagedObjectReference>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// A singleton managed object that provides methods for guest windows registry
    /// operations.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// Refers instance of *GuestWindowsRegistryManager*.
    pub async fn guest_windows_registry_manager(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let path = format!("/GuestOperationsManager/{moId}/guestWindowsRegistryManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::ManagedObjectReference>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// A singleton managed object that provides methods for guest process
    /// operations.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Returns:
    ///
    /// Refers instance of *GuestProcessManager*.
    pub async fn process_manager(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let path = format!("/GuestOperationsManager/{moId}/processManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::ManagedObjectReference>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
