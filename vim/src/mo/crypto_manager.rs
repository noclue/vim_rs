use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::CryptoKeyId;
use crate::types::structs::CryptoKeyPlain;
use crate::types::structs::CryptoKeyResult;
/// Singleton Managed Object used to manage cryptographic keys.
pub struct CryptoManager {
    client: Arc<Client>,
    mo_id: String,
}
impl CryptoManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Add an existing key.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// \[in\] The cryptographic key to add.
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: in case the key is already in the key cache
    /// 
    /// ***InvalidState***: in case the host is not Crypto Safe
    /// 
    /// ***InvalidArgument***: in case the keyID is duplicated or key properties
    /// are incorrect.
    pub async fn add_key(&self, key: &CryptoKeyPlain) -> Result<()> {
        let input = AddKeyRequestType {key, };
        let path = format!("/CryptoManager/{moId}/AddKey", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Add multiple existing keys.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### keys
    /// \[in\] List of cryptographic keys to add.
    ///
    /// ## Returns:
    ///
    /// the result for each key operation.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: in case the host is not Crypto Safe
    pub async fn add_keys(&self, keys: Option<&[CryptoKeyPlain]>) -> Result<Option<Vec<CryptoKeyResult>>> {
        let input = AddKeysRequestType {keys, };
        let path = format!("/CryptoManager/{moId}/AddKeys", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// List keys.
    /// 
    /// \* When executed against the host, lists all the keys
    /// added to the host's key cache by *CryptoManager.AddKey*/*CryptoManager.AddKeys*.
    /// \* When executed against the VC, lists all the keys used by
    /// the correctly registered VMs, and the host key.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### limit
    /// \[in\] maximum keys to return.
    ///
    /// ## Returns:
    ///
    /// List of known keys.
    pub async fn list_keys(&self, limit: Option<i32>) -> Result<Option<Vec<CryptoKeyId>>> {
        let input = ListKeysRequestType {limit, };
        let path = format!("/CryptoManager/{moId}/ListKeys", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Remove a key (only the UUID is needed to remove).
    /// 
    /// If "force" is set, removal will happen even if the key is in use.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// \[in\] The key to remove.
    ///
    /// ### force
    /// \[in\] Remove the key even if in use or not existent.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: in case the keyID is not found
    /// and "force" is false.
    /// 
    /// ***ResourceInUse***: if the key is used to encrypt any object
    /// and "force" is false.
    pub async fn remove_key(&self, key: &CryptoKeyId, force: bool) -> Result<()> {
        let input = RemoveKeyRequestType {key, force, };
        let path = format!("/CryptoManager/{moId}/RemoveKey", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Remove multiple keys (only the UUID is needed to remove).
    /// 
    /// If "force" is set, removal will happen even if they are in use.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### keys
    /// \[in\] List of keys to remove.
    ///
    /// ### force
    /// \[in\] Remove the key even if in use. Always successful.
    pub async fn remove_keys(&self, keys: Option<&[CryptoKeyId]>, force: bool) -> Result<Option<Vec<CryptoKeyResult>>> {
        let input = RemoveKeysRequestType {keys, force, };
        let path = format!("/CryptoManager/{moId}/RemoveKeys", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Indicate if the encryption feature is enabled.
    pub async fn enabled(&self) -> Result<bool> {
        let path = format!("/CryptoManager/{moId}/enabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddKeyRequestType<'a> {
    key: &'a CryptoKeyPlain,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddKeysRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    keys: Option<&'a [CryptoKeyPlain]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListKeysRequestType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveKeyRequestType<'a> {
    key: &'a CryptoKeyId,
    force: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveKeysRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    keys: Option<&'a [CryptoKeyId]>,
    force: bool,
}
