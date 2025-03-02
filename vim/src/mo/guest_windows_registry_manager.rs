use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::GuestRegKeyNameSpec;
use crate::types::structs::GuestRegKeyRecordSpec;
use crate::types::structs::GuestRegValueNameSpec;
use crate::types::structs::GuestRegValueSpec;
use crate::types::structs::ManagedObjectReference;
/// WindowsRegistryManager is the managed object that provides APIs
/// to manipulate the Registry in a Windows guest OS.
pub struct GuestWindowsRegistryManager {
    client: Arc<Client>,
    mo_id: String,
}
impl GuestWindowsRegistryManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    pub async fn create_registry_key_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, key_name: &GuestRegKeyNameSpec, is_volatile: bool, class_type: Option<&str>) -> Result<()> {
        let input = CreateRegistryKeyInGuestRequestType {vm, auth, key_name, is_volatile, class_type, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/CreateRegistryKeyInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
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
    pub async fn delete_registry_key_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, key_name: &GuestRegKeyNameSpec, recursive: bool) -> Result<()> {
        let input = DeleteRegistryKeyInGuestRequestType {vm, auth, key_name, recursive, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/DeleteRegistryKeyInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
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
    pub async fn delete_registry_value_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, value_name: &GuestRegValueNameSpec) -> Result<()> {
        let input = DeleteRegistryValueInGuestRequestType {vm, auth, value_name, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/DeleteRegistryValueInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
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
    pub async fn list_registry_keys_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, key_name: &GuestRegKeyNameSpec, recursive: bool, match_pattern: Option<&str>) -> Result<Option<Vec<GuestRegKeyRecordSpec>>> {
        let input = ListRegistryKeysInGuestRequestType {vm, auth, key_name, recursive, match_pattern, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/ListRegistryKeysInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn list_registry_values_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, key_name: &GuestRegKeyNameSpec, expand_strings: bool, match_pattern: Option<&str>) -> Result<Option<Vec<GuestRegValueSpec>>> {
        let input = ListRegistryValuesInGuestRequestType {vm, auth, key_name, expand_strings, match_pattern, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/ListRegistryValuesInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn set_registry_value_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, value: &GuestRegValueSpec) -> Result<()> {
        let input = SetRegistryValueInGuestRequestType {vm, auth, value, };
        let path = format!("/GuestWindowsRegistryManager/{moId}/SetRegistryValueInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateRegistryKeyInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    #[serde(rename = "keyName")]
    key_name: &'a GuestRegKeyNameSpec,
    #[serde(rename = "isVolatile")]
    is_volatile: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "classType")]
    class_type: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteRegistryKeyInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    #[serde(rename = "keyName")]
    key_name: &'a GuestRegKeyNameSpec,
    recursive: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteRegistryValueInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    #[serde(rename = "valueName")]
    value_name: &'a GuestRegValueNameSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListRegistryKeysInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    #[serde(rename = "keyName")]
    key_name: &'a GuestRegKeyNameSpec,
    recursive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "matchPattern")]
    match_pattern: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListRegistryValuesInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    #[serde(rename = "keyName")]
    key_name: &'a GuestRegKeyNameSpec,
    #[serde(rename = "expandStrings")]
    expand_strings: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "matchPattern")]
    match_pattern: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetRegistryValueInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    value: &'a GuestRegValueSpec,
}
