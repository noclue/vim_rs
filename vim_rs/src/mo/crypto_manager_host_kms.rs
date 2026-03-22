use std::sync::Arc;
use crate::core::client::{VimClient, Result};
#[derive(Clone)]
pub struct CryptoManagerHostKms {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl CryptoManagerHostKms {
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
        self.client.invoke_void("", "CryptoManagerHostKMS", &self.mo_id, "AddKey", Some(&input)).await
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
        let bytes_opt = self.client.invoke_optional("", "CryptoManagerHostKMS", &self.mo_id, "AddKeys", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn change_key_task(&self, new_key: &crate::types::structs::CryptoKeyPlain) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ChangeKeyRequestType {new_key, };
        let bytes = self.client.invoke("", "CryptoManagerHostKMS", &self.mo_id, "ChangeKey_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "CryptoManagerHostKMS", &self.mo_id, "CryptoManagerHostDisable", None).await
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
    pub async fn crypto_manager_host_enable(&self, initial_key: &crate::types::structs::CryptoKeyPlain) -> Result<()> {
        let input = CryptoManagerHostEnableRequestType {initial_key, };
        self.client.invoke_void("", "CryptoManagerHostKMS", &self.mo_id, "CryptoManagerHostEnable", Some(&input)).await
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
    pub async fn get_crypto_key_status(&self, keys: Option<&[crate::types::structs::CryptoKeyId]>) -> Result<Option<Vec<crate::types::structs::CryptoManagerHostKeyStatus>>> {
        let input = GetCryptoKeyStatusRequestType {keys, };
        let bytes_opt = self.client.invoke_optional("", "CryptoManagerHostKMS", &self.mo_id, "GetCryptoKeyStatus", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("", "CryptoManagerHostKMS", &self.mo_id, "ListKeys", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        self.client.invoke_void("", "CryptoManagerHostKMS", &self.mo_id, "CryptoManagerHostPrepare", None).await
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
        self.client.invoke_void("", "CryptoManagerHostKMS", &self.mo_id, "RemoveKey", Some(&input)).await
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
        let bytes_opt = self.client.invoke_optional("", "CryptoManagerHostKMS", &self.mo_id, "RemoveKeys", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Indicate if the encryption feature is enabled.
    pub async fn enabled(&self) -> Result<bool> {
        let pv_opt = self.client.fetch_property_raw("", "CryptoManagerHostKMS", &self.mo_id, "enabled").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property enabled was empty".to_string()))?;
        let result: bool = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
}
struct AddKeyRequestType<'a> {
    key: &'a crate::types::structs::CryptoKeyPlain,
}

impl<'a> miniserde::Serialize for AddKeyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddKeyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddKeyRequestTypeSer<'b, 'a> {
    data: &'b AddKeyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddKeyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddKeyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AddKeysRequestType<'a> {
    keys: Option<&'a [crate::types::structs::CryptoKeyPlain]>,
}

impl<'a> miniserde::Serialize for AddKeysRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddKeysRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddKeysRequestTypeSer<'b, 'a> {
    data: &'b AddKeysRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddKeysRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddKeysRequestType")),
                1 => {
                    let Some(ref val) = self.data.keys else { continue; };
                    return Some((std::borrow::Cow::Borrowed("keys"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ChangeKeyRequestType<'a> {
    new_key: &'a crate::types::structs::CryptoKeyPlain,
}

impl<'a> miniserde::Serialize for ChangeKeyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ChangeKeyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ChangeKeyRequestTypeSer<'b, 'a> {
    data: &'b ChangeKeyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ChangeKeyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ChangeKeyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("newKey"), &self.data.new_key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CryptoManagerHostEnableRequestType<'a> {
    initial_key: &'a crate::types::structs::CryptoKeyPlain,
}

impl<'a> miniserde::Serialize for CryptoManagerHostEnableRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CryptoManagerHostEnableRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CryptoManagerHostEnableRequestTypeSer<'b, 'a> {
    data: &'b CryptoManagerHostEnableRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CryptoManagerHostEnableRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CryptoManagerHostEnableRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("initialKey"), &self.data.initial_key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct GetCryptoKeyStatusRequestType<'a> {
    keys: Option<&'a [crate::types::structs::CryptoKeyId]>,
}

impl<'a> miniserde::Serialize for GetCryptoKeyStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GetCryptoKeyStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GetCryptoKeyStatusRequestTypeSer<'b, 'a> {
    data: &'b GetCryptoKeyStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GetCryptoKeyStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GetCryptoKeyStatusRequestType")),
                1 => {
                    let Some(ref val) = self.data.keys else { continue; };
                    return Some((std::borrow::Cow::Borrowed("keys"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ListKeysRequestType {
    limit: Option<i32>,
}

impl miniserde::Serialize for ListKeysRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListKeysRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListKeysRequestTypeSer<'b> {
    data: &'b ListKeysRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for ListKeysRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListKeysRequestType")),
                1 => {
                    let Some(ref val) = self.data.limit else { continue; };
                    return Some((std::borrow::Cow::Borrowed("limit"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveKeyRequestType<'a> {
    key: &'a crate::types::structs::CryptoKeyId,
    force: bool,
}

impl<'a> miniserde::Serialize for RemoveKeyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveKeyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveKeyRequestTypeSer<'b, 'a> {
    data: &'b RemoveKeyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveKeyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveKeyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("force"), &self.data.force as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveKeysRequestType<'a> {
    keys: Option<&'a [crate::types::structs::CryptoKeyId]>,
    force: bool,
}

impl<'a> miniserde::Serialize for RemoveKeysRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveKeysRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveKeysRequestTypeSer<'b, 'a> {
    data: &'b RemoveKeysRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveKeysRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveKeysRequestType")),
                1 => {
                    let Some(ref val) = self.data.keys else { continue; };
                    return Some((std::borrow::Cow::Borrowed("keys"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("force"), &self.data.force as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
