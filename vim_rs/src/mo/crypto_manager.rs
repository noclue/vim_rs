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
        self.client.invoke_void("", "CryptoManager", &self.mo_id, "AddKey", Some(&input)).await
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
        let bytes_opt = self.client.invoke_optional("", "CryptoManager", &self.mo_id, "AddKeys", Some(&input)).await?;
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
        let bytes_opt = self.client.invoke_optional("", "CryptoManager", &self.mo_id, "ListKeys", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        self.client.invoke_void("", "CryptoManager", &self.mo_id, "RemoveKey", Some(&input)).await
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
        let bytes_opt = self.client.invoke_optional("", "CryptoManager", &self.mo_id, "RemoveKeys", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Indicate if the encryption feature is enabled.
    pub async fn enabled(&self) -> Result<bool> {
        let bytes_opt = self.client.fetch_property_raw("", "CryptoManager", &self.mo_id, "enabled").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property enabled was empty".to_string()))?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
