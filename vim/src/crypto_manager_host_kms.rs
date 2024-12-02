use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::CryptoKeyId;
use crate::types::CryptoKeyPlain;
use crate::types::CryptoKeyResult;
use crate::types::CryptoManagerHostKeyStatus;
use crate::types::ManagedObjectReference;
pub struct CryptoManagerHostKms {
    client: Arc<VimClient>,
    mo_id: String,
}
impl CryptoManagerHostKms {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
        let path = format!("/CryptoManagerHostKMS/{moId}/AddKey", moId = &self.mo_id);
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
        let path = format!("/CryptoManagerHostKMS/{moId}/AddKeys", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Change the key used for core dump encryption
    /// Note: *CryptoManagerHost.CryptoManagerHostEnable* must be called first
    /// If successful, a "best effort" will be made to "in place" shallow recrypt
    /// any core dumps found in /var/core to use the new key.
    /// 
    /// ***Required privileges:*** Cryptographer.RegisterHost
    ///
    /// ## Parameters:
    ///
    /// ### new_key
    /// The key that replaces the existing core dump encryption key
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is not in
    /// *safe* state
    pub async fn change_key_task(&self, new_key: &CryptoKeyPlain) -> Result<ManagedObjectReference> {
        let input = ChangeKeyRequestType {new_key, };
        let path = format!("/CryptoManagerHostKMS/{moId}/ChangeKey_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Disable encryption on host, if host was in crypto safe mode, put it in
    /// pendingIncapable state and host will be crypto incapable after a reboot
    /// Note: A reboot is expected from user after successfully invoking this API
    /// Note: Do not call this API if the host is in vSAN encrypted cluster
    /// 
    /// ***Required privileges:*** Cryptographer.RegisterHost
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is already crypto disabled.
    pub async fn crypto_manager_host_disable(&self) -> Result<()> {
        let path = format!("/CryptoManagerHostKMS/{moId}/CryptoManagerHostDisable", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Begin core dump encryption by specifying the encryption key and put
    /// the host in *safe* state
    /// Note: *CryptoManagerHost.CryptoManagerHostPrepare* must be called first
    /// 
    /// ***Required privileges:*** Cryptographer.RegisterHost
    ///
    /// ## Parameters:
    ///
    /// ### initial_key
    /// The key to be used for core dump encryption
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is in
    /// *incapable* state
    /// 
    /// ***AlreadyExists***: if the host is in
    /// *safe* state and initialKey differs
    /// from the existing core dump
    /// encryption key
    pub async fn crypto_manager_host_enable(&self, initial_key: &CryptoKeyPlain) -> Result<()> {
        let input = CryptoManagerHostEnableRequestType {initial_key, };
        let path = format!("/CryptoManagerHostKMS/{moId}/CryptoManagerHostEnable", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Get the key status on the host.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### keys
    /// \[in\] Cryptographic keys to query status.
    ///
    /// ## Returns:
    ///
    /// the key status.
    pub async fn get_crypto_key_status(&self, keys: Option<&[CryptoKeyId]>) -> Result<Option<Vec<CryptoManagerHostKeyStatus>>> {
        let input = GetCryptoKeyStatusRequestType {keys, };
        let path = format!("/CryptoManagerHostKMS/{moId}/GetCryptoKeyStatus", moId = &self.mo_id);
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
        let path = format!("/CryptoManagerHostKMS/{moId}/ListKeys", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Prime the host to receive sensitive information and put the host
    /// in *prepared* state
    /// 
    /// ***Required privileges:*** Cryptographer.RegisterHost
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is not in
    /// *incapable* state
    pub async fn crypto_manager_host_prepare(&self) -> Result<()> {
        let path = format!("/CryptoManagerHostKMS/{moId}/CryptoManagerHostPrepare", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
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
        let path = format!("/CryptoManagerHostKMS/{moId}/RemoveKey", moId = &self.mo_id);
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
        let path = format!("/CryptoManagerHostKMS/{moId}/RemoveKeys", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Indicate if the encryption feature is enabled.
    pub async fn enabled(&self) -> Result<bool> {
        let path = format!("/CryptoManagerHostKMS/{moId}/enabled", moId = &self.mo_id);
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
struct ChangeKeyRequestType<'a> {
    #[serde(rename = "newKey")]
    new_key: &'a CryptoKeyPlain,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CryptoManagerHostEnableRequestType<'a> {
    #[serde(rename = "initialKey")]
    initial_key: &'a CryptoKeyPlain,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GetCryptoKeyStatusRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    keys: Option<&'a [CryptoKeyId]>,
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
