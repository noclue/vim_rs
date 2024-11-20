use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::GuestAliases;
use crate::types::GuestMappedAliases;
use crate::types::GuestAuthAliasInfo;
use crate::types::GuestAuthenticationTrait;
use crate::types::GuestAuthSubjectTrait;
use crate::types::ManagedObjectReference;
/// The GuestAliasManager supports single sign-on for virtual machine access
/// to perform guest operations.
/// 
/// The GuestAliasManager provides methods
/// to create and access aliases.
/// 
/// A guest alias defines an association between a guest user account on a virtual
/// machine
/// and an external vSphere user account. The vSphere account is represented by
/// credentials consisting of an X.509 certificate and a subject name.
/// The certificate and subject name
/// are encoded
/// in SAML tokens that are provided by the VMware SSO Server. The SAML tokens
/// are attached
/// to guest operation requests. If the credentials in a SAML token match an
/// alias that is defined
/// for a virtual machine, the ESXi Server guest components grant
/// access for execution of the
/// guest operation
/// in the context of the user account on the virtual machine.
/// 
/// To create a guest alias, use the AddGuestAlias method. AddGuestAlias
/// establishes
/// the association between a guest user account, certificate, and SAML token subject.
/// - The username parameter identifies the guest account.
/// - The base64Cert parameter specifies the X.509 certificate.
/// - The aliasInfo parameter identifies the SAML token subject
///   (*GuestAuthAliasInfo*.
///   *GuestAuthAliasInfo.subject*.
///   *GuestAuthNamedSubject.name*).
///   
/// If there are no aliases defined for a virtual machine, the ESXi Server
/// will perform
/// standard authentication using the credentials associated with a guest
/// operation request.
/// If one or more aliases are defined for a virtual machine, any guest
/// operation request
/// that uses SAML token authentication SAMLTokenAuthentication must specify
/// a token
/// that corresponds to one of the defined aliases.
/// 
/// After defining one or more guest aliases, you can specify
/// *SAMLTokenAuthentication* for the
/// **auth** parameter to guest operation methods:
/// - *GuestProcessManager* methods
/// - *GuestFileManager* methods
/// - *GuestWindowsRegistryManager* methods
///   
/// For information about obtaining a SAML token from a VMware SSO Server,
/// see _VMware Single Sign-On Programming Guide_.
/// 
/// You can define multiple aliases for a guest account. You can also
/// map the credentials
/// to an alias by setting **mapCert** to "true" in the call to the
/// AddGuestAlias method.
/// When an alias has a mapped credential, requests using that alias do not
/// need to identify the guest account.
pub struct GuestAliasManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl GuestAliasManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Defines an alias for a guest account in a virtual machine.
    /// 
    /// After the alias is defined, the ESXi Server will use the alias
    /// to authenticate guest operations requests.
    /// 
    /// This will add the given VMware SSO Server's certificate and a
    /// subject to the alias store of the
    /// specified user in the guest.
    /// 
    /// In order to add an alias to the guest, you must supply
    /// an existing valid credential. This can be any instance
    /// of *GuestAuthentication*, but must be valid for the
    /// specified guest username.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.ModifyAliases
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data for this operation. See
    /// *GuestAuthentication*. These credentials must satisfy
    /// authentication requirements
    /// for a guest account on the specified virtual machine.
    ///
    /// ### username
    /// Username for the guest account on the virtual machine.
    ///
    /// ### map_cert
    /// Indicates whether the certificate associated with the
    /// alias should be mapped. If an alias certificate is mapped,
    /// guest operation requests that use that alias do not have
    /// to specify the guest account username in the
    /// *SAMLTokenAuthentication* object. If mapCert is
    /// false, the request must specify the username.
    ///
    /// ### base_64_cert
    /// X.509 certificate from the VMware SSO Server,
    /// in base64 encoded DER format. The ESXi
    /// Server uses this certificate to authenticate guest
    /// operation requests.
    ///
    /// ### alias_info
    /// Specifies the subject name for authentication.
    /// The subject name (when present) corresponds to
    /// the value of the Subject element
    /// in SAML tokens. The ESXi Server uses the subject
    /// name to authenticate guest operation requests.
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
    /// ***GuestPermissionDenied***: if there are insufficient permissions in
    /// the guest OS.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    /// 
    /// ***InvalidArgument***: if the operation fails because
    /// the certificate is invalid.
    /// 
    /// ***GuestMultipleMappings***: if the operation fails because
    /// mapCert is set and the certificate already
    /// exists in the mapping file for a
    /// different user.
    pub async fn add_guest_alias(&self, vm: &ManagedObjectReference, auth: &dyn GuestAuthenticationTrait, username: &str, map_cert: bool, base_64_cert: &str, alias_info: &GuestAuthAliasInfo) -> Result<()> {
        let input = AddGuestAliasRequestType {vm, auth, username, map_cert, base_64_cert, alias_info, };
        let path = format!("/GuestAliasManager/{moId}/AddGuestAlias", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Lists the
    /// *GuestAliases*
    /// for a specified user in the guest that can be used
    /// for authentication of guest operations.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.QueryAliases
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data for this operation. See
    /// *GuestAuthentication*. These credentials must satisfy
    /// authentication requirements
    /// for a guest account on the specified virtual machine.
    ///
    /// ### username
    /// The guest user whose Alias store is being queried.
    ///
    /// ## Errors:
    ///
    /// ***GuestOperationsFault***: if there is an error processing a guest
    /// operation.
    /// 
    /// ***GuestOperationsUnavailable***: if the agent for guest operations
    /// is not running.
    /// 
    /// ***InvalidPowerState***: if the VM is not powered on.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***GuestPermissionDenied***: if there are insufficient permissions in
    /// the guest OS.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn list_guest_aliases(&self, vm: &ManagedObjectReference, auth: &dyn GuestAuthenticationTrait, username: &str) -> Result<Option<Vec<GuestAliases>>> {
        let input = ListGuestAliasesRequestType {vm, auth, username, };
        let path = format!("/GuestAliasManager/{moId}/ListGuestAliases", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Lists the
    /// *GuestMappedAliases*
    /// in the guest that can be used for
    /// authentication of guest operations.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.QueryAliases
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data for this operation. See
    /// *GuestAuthentication*. These credentials must satisfy
    /// authentication requirements
    /// for a guest account on the specified virtual machine.
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
    /// ***GuestPermissionDenied***: if there are insufficient permissions in
    /// the guest OS.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn list_guest_mapped_aliases(&self, vm: &ManagedObjectReference, auth: &dyn GuestAuthenticationTrait) -> Result<Option<Vec<GuestMappedAliases>>> {
        let input = ListGuestMappedAliasesRequestType {vm, auth, };
        let path = format!("/GuestAliasManager/{moId}/ListGuestMappedAliases", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Removes an alias from the guest so it can no longer be
    /// used for
    /// authentication of guest operations.
    /// 
    /// It will also be removed
    /// from the mapped credentials.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.ModifyAliases
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data for this operation. See
    /// *GuestAuthentication*. These credentials must satisfy
    /// authentication requirements
    /// for a guest account on the specified virtual machine.
    ///
    /// ### username
    /// Username for the guest account on the virtual machine.
    ///
    /// ### base_64_cert
    /// The X.509 certificate associated with the alias to be
    /// removed, in base64 encoded DER format.
    ///
    /// ### subject
    /// The subject of the alias.
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
    /// ***GuestPermissionDenied***: if there are insufficient permissions in
    /// the guest OS.
    /// 
    /// ***InvalidArgument***: if the operation fails because
    /// the certificate is invalid.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn remove_guest_alias(&self, vm: &ManagedObjectReference, auth: &dyn GuestAuthenticationTrait, username: &str, base_64_cert: &str, subject: &dyn GuestAuthSubjectTrait) -> Result<()> {
        let input = RemoveGuestAliasRequestType {vm, auth, username, base_64_cert, subject, };
        let path = format!("/GuestAliasManager/{moId}/RemoveGuestAlias", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Removes a VMware SSO Server's certificate and all
    /// associated aliases from the guest so it
    /// can no longer be used for
    /// authentication of guest operations.
    /// 
    /// It will also be removed
    /// from the global certificate-to-user mapping file in the guest.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.ModifyAliases
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data for this operation. See
    /// *GuestAuthentication*. These credentials must satisfy
    /// authentication requirements
    /// for a guest account on the specified virtual machine.
    ///
    /// ### username
    /// Username for the guest account on the virtual machine.
    ///
    /// ### base_64_cert
    /// The X.509 certificate to be removed, in base64
    /// encoded DER format.
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
    /// ***GuestPermissionDenied***: if there are insufficient permissions in
    /// the guest OS.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***InvalidArgument***: if the operation fails because
    /// the certificate is invalid.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn remove_guest_alias_by_cert(&self, vm: &ManagedObjectReference, auth: &dyn GuestAuthenticationTrait, username: &str, base_64_cert: &str) -> Result<()> {
        let input = RemoveGuestAliasByCertRequestType {vm, auth, username, base_64_cert, };
        let path = format!("/GuestAliasManager/{moId}/RemoveGuestAliasByCert", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddGuestAliasRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn GuestAuthenticationTrait,
    username: &'a str,
    #[serde(rename = "mapCert")]
    map_cert: bool,
    #[serde(rename = "base64Cert")]
    base_64_cert: &'a str,
    #[serde(rename = "aliasInfo")]
    alias_info: &'a GuestAuthAliasInfo,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListGuestAliasesRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn GuestAuthenticationTrait,
    username: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListGuestMappedAliasesRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn GuestAuthenticationTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveGuestAliasRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn GuestAuthenticationTrait,
    username: &'a str,
    #[serde(rename = "base64Cert")]
    base_64_cert: &'a str,
    subject: &'a dyn GuestAuthSubjectTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveGuestAliasByCertRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn GuestAuthenticationTrait,
    username: &'a str,
    #[serde(rename = "base64Cert")]
    base_64_cert: &'a str,
}
