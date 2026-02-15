use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
#[derive(Clone)]
pub struct GuestAliasManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl GuestAliasManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn add_guest_alias(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, username: &str, map_cert: bool, base_64_cert: &str, alias_info: &crate::types::structs::GuestAuthAliasInfo) -> Result<()> {
        let input = AddGuestAliasRequestType {vm, auth, username, map_cert, base_64_cert, alias_info, };
        let path = format!("/GuestAliasManager/{moId}/AddGuestAlias", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn list_guest_aliases(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, username: &str) -> Result<Option<Vec<crate::types::structs::GuestAliases>>> {
        let input = ListGuestAliasesRequestType {vm, auth, username, };
        let path = format!("/GuestAliasManager/{moId}/ListGuestAliases", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::GuestAliases>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn list_guest_mapped_aliases(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait) -> Result<Option<Vec<crate::types::structs::GuestMappedAliases>>> {
        let input = ListGuestMappedAliasesRequestType {vm, auth, };
        let path = format!("/GuestAliasManager/{moId}/ListGuestMappedAliases", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::GuestMappedAliases>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn remove_guest_alias(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, username: &str, base_64_cert: &str, subject: &dyn crate::types::traits::GuestAuthSubjectTrait) -> Result<()> {
        let input = RemoveGuestAliasRequestType {vm, auth, username, base_64_cert, subject, };
        let path = format!("/GuestAliasManager/{moId}/RemoveGuestAlias", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn remove_guest_alias_by_cert(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, username: &str, base_64_cert: &str) -> Result<()> {
        let input = RemoveGuestAliasByCertRequestType {vm, auth, username, base_64_cert, };
        let path = format!("/GuestAliasManager/{moId}/RemoveGuestAliasByCert", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
}
struct AddGuestAliasRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    username: &'a str,
    map_cert: bool,
    base_64_cert: &'a str,
    alias_info: &'a crate::types::structs::GuestAuthAliasInfo,
}

impl<'a> miniserde::Serialize for AddGuestAliasRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddGuestAliasRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddGuestAliasRequestTypeSer<'b, 'a> {
    data: &'b AddGuestAliasRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for AddGuestAliasRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddGuestAliasRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("username"), &self.data.username as &dyn miniserde::Serialize)),
            4 => return Some((std::borrow::Cow::Borrowed("mapCert"), &self.data.map_cert as &dyn miniserde::Serialize)),
            5 => return Some((std::borrow::Cow::Borrowed("base64Cert"), &self.data.base_64_cert as &dyn miniserde::Serialize)),
            6 => return Some((std::borrow::Cow::Borrowed("aliasInfo"), &self.data.alias_info as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ListGuestAliasesRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    username: &'a str,
}

impl<'a> miniserde::Serialize for ListGuestAliasesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListGuestAliasesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListGuestAliasesRequestTypeSer<'b, 'a> {
    data: &'b ListGuestAliasesRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ListGuestAliasesRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListGuestAliasesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("username"), &self.data.username as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ListGuestMappedAliasesRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
}

impl<'a> miniserde::Serialize for ListGuestMappedAliasesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListGuestMappedAliasesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListGuestMappedAliasesRequestTypeSer<'b, 'a> {
    data: &'b ListGuestMappedAliasesRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ListGuestMappedAliasesRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListGuestMappedAliasesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveGuestAliasRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    username: &'a str,
    base_64_cert: &'a str,
    subject: &'a dyn crate::types::traits::GuestAuthSubjectTrait,
}

impl<'a> miniserde::Serialize for RemoveGuestAliasRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveGuestAliasRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveGuestAliasRequestTypeSer<'b, 'a> {
    data: &'b RemoveGuestAliasRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RemoveGuestAliasRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveGuestAliasRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("username"), &self.data.username as &dyn miniserde::Serialize)),
            4 => return Some((std::borrow::Cow::Borrowed("base64Cert"), &self.data.base_64_cert as &dyn miniserde::Serialize)),
            5 => return Some((std::borrow::Cow::Borrowed("subject"), &self.data.subject as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveGuestAliasByCertRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    username: &'a str,
    base_64_cert: &'a str,
}

impl<'a> miniserde::Serialize for RemoveGuestAliasByCertRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveGuestAliasByCertRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveGuestAliasByCertRequestTypeSer<'b, 'a> {
    data: &'b RemoveGuestAliasByCertRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RemoveGuestAliasByCertRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveGuestAliasByCertRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("username"), &self.data.username as &dyn miniserde::Serialize)),
            4 => return Some((std::borrow::Cow::Borrowed("base64Cert"), &self.data.base_64_cert as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
