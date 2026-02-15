use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// AuthManager is the managed object that provides APIs
/// to manipulate the guest operating authentication.
#[derive(Clone)]
pub struct GuestAuthManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl GuestAuthManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Authenticates in the guest and returns a *GuestAuthentication* object
    /// with the acquired credentials for use in subsequent guest operation calls.
    /// 
    /// This can be used to authenticate inside the guest and obtain a
    /// *GuestAuthentication* object for supported authentication types.
    /// This operation is not needed for Name and Password Authentication. To use
    /// Name and Password Authentication, see *NamePasswordAuthentication*.
    /// For SSPI authentication, requestAuth should be of the type *SSPIAuthentication*.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// MoRef of the VM to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Query
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### requested_auth
    /// The guest authentication data used to acquire credentials.
    /// See *GuestAuthentication*.
    ///
    /// ### session_id
    /// The sessionID number should be provided only when
    /// responding to a server challenge. The sessionID number to be used with
    /// the challenge is found in the
    /// *GuestAuthenticationChallenge* object.
    ///
    /// ## Returns:
    ///
    /// Returns a *GuestAuthentication* object that can be used in
    /// guest operation calls.
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
    /// ***GuestAuthenticationChallenge***: if the credential information
    /// provided requires a challenge to authenticate.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to
    /// support the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due
    /// to guest agent configuration.
    /// 
    /// ***TooManyGuestLogons***: if there are too many concurrent login
    /// sessions active in the guest.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    pub async fn acquire_credentials_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, requested_auth: &dyn crate::types::traits::GuestAuthenticationTrait, session_id: Option<i64>) -> Result<Box<dyn crate::types::traits::GuestAuthenticationTrait>> {
        let input = AcquireCredentialsInGuestRequestType {vm, requested_auth, session_id, };
        let path = format!("/GuestAuthManager/{moId}/AcquireCredentialsInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Box<dyn crate::types::traits::GuestAuthenticationTrait> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Releases session data and resources associated with
    /// a *GuestAuthentication* object returned by *GuestAuthManager.AcquireCredentialsInGuest*.
    /// 
    /// This frees any resources and session data associated with a
    /// *GuestAuthentication* object returned by *GuestAuthManager.AcquireCredentialsInGuest*.
    /// The *GuestAuthentication* object can no longer be used to
    /// authenticate in the guest once released. Currently this operation is only
    /// valid for *TicketedSessionAuthentication* objects.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// MoRef of the VM to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Query
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
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
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to
    /// support the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due
    /// to guest agent configuration.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    pub async fn release_credentials_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait) -> Result<()> {
        let input = ReleaseCredentialsInGuestRequestType {vm, auth, };
        let path = format!("/GuestAuthManager/{moId}/ReleaseCredentialsInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Validates the *GuestAuthentication* credentials.
    /// 
    /// This can be used to check the authentication data, or
    /// validate any authentication that has a timeout is still valid.
    /// If the authentication is not valid, *GuestPermissionDenied*
    /// will be thrown.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// MoRef of the VM to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Query
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
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
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported by
    /// the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    pub async fn validate_credentials_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait) -> Result<()> {
        let input = ValidateCredentialsInGuestRequestType {vm, auth, };
        let path = format!("/GuestAuthManager/{moId}/ValidateCredentialsInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
}
struct AcquireCredentialsInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    requested_auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    session_id: Option<i64>,
}

impl<'a> miniserde::Serialize for AcquireCredentialsInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AcquireCredentialsInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AcquireCredentialsInGuestRequestTypeSer<'b, 'a> {
    data: &'b AcquireCredentialsInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for AcquireCredentialsInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AcquireCredentialsInGuestRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("requestedAuth"), &self.data.requested_auth as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.session_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("sessionID"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ReleaseCredentialsInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
}

impl<'a> miniserde::Serialize for ReleaseCredentialsInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReleaseCredentialsInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReleaseCredentialsInGuestRequestTypeSer<'b, 'a> {
    data: &'b ReleaseCredentialsInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ReleaseCredentialsInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReleaseCredentialsInGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ValidateCredentialsInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
}

impl<'a> miniserde::Serialize for ValidateCredentialsInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ValidateCredentialsInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ValidateCredentialsInGuestRequestTypeSer<'b, 'a> {
    data: &'b ValidateCredentialsInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ValidateCredentialsInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ValidateCredentialsInGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
