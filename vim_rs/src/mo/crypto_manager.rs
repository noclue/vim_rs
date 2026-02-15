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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::CryptoKeyResult>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::CryptoKeyId>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::CryptoKeyResult>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Indicate if the encryption feature is enabled.
    pub async fn enabled(&self) -> Result<bool> {
        let path = format!("/CryptoManager/{moId}/enabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: bool = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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

impl miniserde::ser::Map for AddKeyRequestTypeSer<'_, '_> {
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

impl miniserde::ser::Map for AddKeysRequestTypeSer<'_, '_> {
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

impl miniserde::ser::Map for ListKeysRequestTypeSer<'_> {
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

impl miniserde::ser::Map for RemoveKeyRequestTypeSer<'_, '_> {
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

impl miniserde::ser::Map for RemoveKeysRequestTypeSer<'_, '_> {
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
