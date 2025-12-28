use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Singleton Managed Object used to manage cryptographic keys.
#[derive(Clone)]
pub struct CryptoManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl CryptoManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn add_key(&self, key: &crate::types::structs::CryptoKeyPlain) -> Result<()> {
        let input = AddKeyRequestType {key, };
        let path = format!("/CryptoManager/{moId}/AddKey", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn add_keys(&self, keys: Option<&[crate::types::structs::CryptoKeyPlain]>) -> Result<Option<Vec<crate::types::structs::CryptoKeyResult>>> {
        let input = AddKeysRequestType {keys, };
        let path = format!("/CryptoManager/{moId}/AddKeys", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::CryptoKeyResult>>(bytes.as_ref())?)),
            None => Ok(None),
        }
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
    pub async fn list_keys(&self, limit: Option<i32>) -> Result<Option<Vec<crate::types::structs::CryptoKeyId>>> {
        let input = ListKeysRequestType {limit, };
        let path = format!("/CryptoManager/{moId}/ListKeys", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::CryptoKeyId>>(bytes.as_ref())?)),
            None => Ok(None),
        }
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
    pub async fn remove_key(&self, key: &crate::types::structs::CryptoKeyId, force: bool) -> Result<()> {
        let input = RemoveKeyRequestType {key, force, };
        let path = format!("/CryptoManager/{moId}/RemoveKey", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn remove_keys(&self, keys: Option<&[crate::types::structs::CryptoKeyId]>, force: bool) -> Result<Option<Vec<crate::types::structs::CryptoKeyResult>>> {
        let input = RemoveKeysRequestType {keys, force, };
        let path = format!("/CryptoManager/{moId}/RemoveKeys", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::CryptoKeyResult>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Indicate if the encryption feature is enabled.
    pub async fn enabled(&self) -> Result<bool> {
        let path = format!("/CryptoManager/{moId}/enabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: bool = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddKeyRequestType<'a> {
    key: &'a crate::types::structs::CryptoKeyPlain,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddKeysRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    keys: Option<&'a [crate::types::structs::CryptoKeyPlain]>,
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
    key: &'a crate::types::structs::CryptoKeyId,
    force: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveKeysRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    keys: Option<&'a [crate::types::structs::CryptoKeyId]>,
    force: bool,
}
