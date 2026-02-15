use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// WindowsRegistryManager is the managed object that provides APIs
/// to manipulate the Registry in a Windows guest OS.
#[derive(Clone)]
pub struct GuestWindowsRegistryManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl GuestWindowsRegistryManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Create a registry key.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data.
    ///
    /// ### key_name
    /// The path to the registry key to be created.
    ///
    /// ### is_volatile
    /// If true, the key is created in memory and is not
    /// preserved across system reboot. Otherwise, it shall
    /// persist in the file system.
    ///
    /// ### class_type
    /// User defined class type for this key. May be omitted.
    ///
    /// ## Errors:
    ///
    /// ***GuestOperationsFault***: if there is an error processing a guest
    /// operation.
    /// 
    /// ***GuestOperationsUnavailable***: if the VM agent for guest operations
    /// is not running.
    /// 
    /// ***InvalidPowerState***: if the VM is not powered on.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***GuestRegistryKeyInvalid***: if the registry key is not valid. Check
    /// the HKEY Root specified.
    /// 
    /// ***GuestRegistryKeyAlreadyExists***: if the registry key already exists.
    /// 
    /// ***GuestRegistryKeyParentVolatile***: if trying to create a non-volatile
    /// registry subkey under a volatile
    /// registry parent key.
    /// 
    /// ***GuestPermissionDenied***: if the program path cannot be run because
    /// the guest authentication will not allow the
    /// operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported by
    /// the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn create_registry_key_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, key_name: &crate::types::structs::GuestRegKeyNameSpec, is_volatile: bool, class_type: Option<&str>) -> Result<()> {
        let input = CreateRegistryKeyInGuestRequestType {vm, auth, key_name, is_volatile, class_type, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/CreateRegistryKeyInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Delete a registry key.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data.
    ///
    /// ### key_name
    /// The path to the registry key to be deleted.
    ///
    /// ### recursive
    /// If true, the key is deleted along with any subkeys (if
    /// present). Otherwise, it shall only delete the key if it
    /// has no subkeys.
    ///
    /// ## Errors:
    ///
    /// ***GuestOperationsFault***: if there is an error processing a guest
    /// operation.
    /// 
    /// ***GuestOperationsUnavailable***: if the VM agent for guest operations
    /// is not running.
    /// 
    /// ***InvalidPowerState***: if the VM is not powered on.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***GuestRegistryKeyInvalid***: if the registry key is not valid. Check
    /// the HKEY Root specified.
    /// 
    /// ***GuestRegistryKeyHasSubkeys***: if the parameter recursive is false and
    /// the key has subkeys.
    /// 
    /// ***GuestPermissionDenied***: if the program path cannot be run because
    /// the guest authentication will not allow the
    /// operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported by
    /// the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn delete_registry_key_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, key_name: &crate::types::structs::GuestRegKeyNameSpec, recursive: bool) -> Result<()> {
        let input = DeleteRegistryKeyInGuestRequestType {vm, auth, key_name, recursive, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/DeleteRegistryKeyInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Delete a registry value.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data.
    ///
    /// ### value_name
    /// The registry value name to be deleted.
    /// The Value "name" (specified in
    /// *GuestRegValueNameSpec*)
    /// can be empty. If "name" is empty, it deletes the value
    /// for the unnamed or default value of the given key.
    ///
    /// ## Errors:
    ///
    /// ***GuestOperationsFault***: if there is an error processing a guest
    /// operation.
    /// 
    /// ***GuestOperationsUnavailable***: if the VM agent for guest operations
    /// is not running.
    /// 
    /// ***InvalidPowerState***: if the VM is not powered on.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***GuestRegistryKeyInvalid***: if the registry key is not valid. Check
    /// the HKEY Root specified.
    /// 
    /// ***GuestRegistryValueNotFound***: if the registry value was not found.
    /// 
    /// ***GuestPermissionDenied***: if the program path cannot be run because
    /// the guest authentication will not allow the
    /// operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported by
    /// the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn delete_registry_value_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, value_name: &crate::types::structs::GuestRegValueNameSpec) -> Result<()> {
        let input = DeleteRegistryValueInGuestRequestType {vm, auth, value_name, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/DeleteRegistryValueInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// List all registry subkeys for a given registry key.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Query
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data.
    ///
    /// ### key_name
    /// The path to the registry key for which all subkeys are to
    /// be listed.
    ///
    /// ### recursive
    /// If true, all subkeys are listed recursively.
    ///
    /// ### match_pattern
    /// A filter for the key names returned, specified using
    /// perl-compatible regular expressions. If matchPattern
    /// is unset, then the pattern '.\*' is used, which returns
    /// all key names found, otherwise only those key names
    /// that match the input pattern shall be returned.
    ///
    /// ## Returns:
    ///
    /// The list of subkeys is returned in an array of
    /// *GuestRegKeySpec* structures.
    ///
    /// ## Errors:
    ///
    /// ***GuestOperationsFault***: if there is an error processing a guest
    /// operation.
    /// 
    /// ***GuestOperationsUnavailable***: if the VM agent for guest operations
    /// is not running.
    /// 
    /// ***InvalidPowerState***: if the VM is not powered on.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***GuestRegistryKeyInvalid***: if the registry key is not valid. Check
    /// the HKEY Root specified.
    /// 
    /// ***GuestPermissionDenied***: if the program path cannot be run because
    /// the guest authentication will not allow the
    /// operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported by
    /// the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn list_registry_keys_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, key_name: &crate::types::structs::GuestRegKeyNameSpec, recursive: bool, match_pattern: Option<&str>) -> Result<Option<Vec<crate::types::structs::GuestRegKeyRecordSpec>>> {
        let input = ListRegistryKeysInGuestRequestType {vm, auth, key_name, recursive, match_pattern, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/ListRegistryKeysInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::GuestRegKeyRecordSpec>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// List all registry values for a given registry key.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Query
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data.
    ///
    /// ### key_name
    /// The path to the registry key for which all values are to be
    /// listed.
    ///
    /// ### expand_strings
    /// If true, all values that have expandable data such
    /// as environment variable names, shall get expanded in
    /// the result.
    ///
    /// ### match_pattern
    /// A filter for the value names returned, specified using
    /// perl-compatible regular expressions. If matchPattern
    /// is unset, then the pattern '.\*' is used, which returns
    /// all value names found, otherwise only those value
    /// names that match the input pattern shall be returned.
    ///
    /// ## Returns:
    ///
    /// The list of values is returned in an array of
    /// *GuestRegValueSpec* structures.
    ///
    /// ## Errors:
    ///
    /// ***GuestOperationsFault***: if there is an error processing a guest
    /// operation.
    /// 
    /// ***GuestOperationsUnavailable***: if the VM agent for guest operations
    /// is not running.
    /// 
    /// ***InvalidPowerState***: if the VM is not powered on.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***GuestRegistryKeyInvalid***: if the registry key is not valid. Check
    /// the HKEY Root specified.
    /// 
    /// ***GuestPermissionDenied***: if the program path cannot be run because
    /// the guest authentication will not allow the
    /// operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported by
    /// the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn list_registry_values_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, key_name: &crate::types::structs::GuestRegKeyNameSpec, expand_strings: bool, match_pattern: Option<&str>) -> Result<Option<Vec<crate::types::structs::GuestRegValueSpec>>> {
        let input = ListRegistryValuesInGuestRequestType {vm, auth, key_name, expand_strings, match_pattern, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/ListRegistryValuesInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::GuestRegValueSpec>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Set/Create a registry value.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data.
    ///
    /// ### value
    /// The information for the registry value to be set/created.
    /// The Value "name" (specified in
    /// *GuestRegValueNameSpec*)
    /// and the Value "data" (specified in
    /// *GuestRegValueSpec*)
    /// can both be empty. If "name" is empty, it sets the value for
    /// the unnamed or default value of the given key.
    ///
    /// ## Errors:
    ///
    /// ***GuestOperationsFault***: if there is an error processing a guest
    /// operation.
    /// 
    /// ***GuestOperationsUnavailable***: if the VM agent for guest operations
    /// is not running.
    /// 
    /// ***InvalidPowerState***: if the VM is not powered on.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***GuestRegistryKeyInvalid***: if the registry key is not valid. Check
    /// the HKEY Root specified.
    /// 
    /// ***GuestPermissionDenied***: if the program path cannot be run because
    /// the guest authentication will not allow the
    /// operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported by
    /// the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn set_registry_value_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, value: &crate::types::structs::GuestRegValueSpec) -> Result<()> {
        let input = SetRegistryValueInGuestRequestType {vm, auth, value, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/SetRegistryValueInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
}
struct CreateRegistryKeyInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    key_name: &'a crate::types::structs::GuestRegKeyNameSpec,
    is_volatile: bool,
    class_type: Option<&'a str>,
}

impl<'a> miniserde::Serialize for CreateRegistryKeyInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateRegistryKeyInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateRegistryKeyInGuestRequestTypeSer<'b, 'a> {
    data: &'b CreateRegistryKeyInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CreateRegistryKeyInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateRegistryKeyInGuestRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("keyName"), &self.data.key_name as &dyn miniserde::Serialize)),
                4 => return Some((std::borrow::Cow::Borrowed("isVolatile"), &self.data.is_volatile as &dyn miniserde::Serialize)),
                5 => {
                    let Some(ref val) = self.data.class_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("classType"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct DeleteRegistryKeyInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    key_name: &'a crate::types::structs::GuestRegKeyNameSpec,
    recursive: bool,
}

impl<'a> miniserde::Serialize for DeleteRegistryKeyInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteRegistryKeyInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteRegistryKeyInGuestRequestTypeSer<'b, 'a> {
    data: &'b DeleteRegistryKeyInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DeleteRegistryKeyInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteRegistryKeyInGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("keyName"), &self.data.key_name as &dyn miniserde::Serialize)),
            4 => return Some((std::borrow::Cow::Borrowed("recursive"), &self.data.recursive as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DeleteRegistryValueInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    value_name: &'a crate::types::structs::GuestRegValueNameSpec,
}

impl<'a> miniserde::Serialize for DeleteRegistryValueInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteRegistryValueInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteRegistryValueInGuestRequestTypeSer<'b, 'a> {
    data: &'b DeleteRegistryValueInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DeleteRegistryValueInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteRegistryValueInGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("valueName"), &self.data.value_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ListRegistryKeysInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    key_name: &'a crate::types::structs::GuestRegKeyNameSpec,
    recursive: bool,
    match_pattern: Option<&'a str>,
}

impl<'a> miniserde::Serialize for ListRegistryKeysInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListRegistryKeysInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListRegistryKeysInGuestRequestTypeSer<'b, 'a> {
    data: &'b ListRegistryKeysInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ListRegistryKeysInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListRegistryKeysInGuestRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("keyName"), &self.data.key_name as &dyn miniserde::Serialize)),
                4 => return Some((std::borrow::Cow::Borrowed("recursive"), &self.data.recursive as &dyn miniserde::Serialize)),
                5 => {
                    let Some(ref val) = self.data.match_pattern else { continue; };
                    return Some((std::borrow::Cow::Borrowed("matchPattern"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ListRegistryValuesInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    key_name: &'a crate::types::structs::GuestRegKeyNameSpec,
    expand_strings: bool,
    match_pattern: Option<&'a str>,
}

impl<'a> miniserde::Serialize for ListRegistryValuesInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListRegistryValuesInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListRegistryValuesInGuestRequestTypeSer<'b, 'a> {
    data: &'b ListRegistryValuesInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ListRegistryValuesInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListRegistryValuesInGuestRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("keyName"), &self.data.key_name as &dyn miniserde::Serialize)),
                4 => return Some((std::borrow::Cow::Borrowed("expandStrings"), &self.data.expand_strings as &dyn miniserde::Serialize)),
                5 => {
                    let Some(ref val) = self.data.match_pattern else { continue; };
                    return Some((std::borrow::Cow::Borrowed("matchPattern"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct SetRegistryValueInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    value: &'a crate::types::structs::GuestRegValueSpec,
}

impl<'a> miniserde::Serialize for SetRegistryValueInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetRegistryValueInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetRegistryValueInGuestRequestTypeSer<'b, 'a> {
    data: &'b SetRegistryValueInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for SetRegistryValueInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetRegistryValueInGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("value"), &self.data.value as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
