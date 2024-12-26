use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::GuestAuthenticationTrait;
use crate::types::structs::ManagedObjectReference;
/// AuthManager is the managed object that provides APIs
/// to manipulate the guest operating authentication.
pub struct GuestAuthManager {
    client: Arc<Client>,
    mo_id: String,
}
impl GuestAuthManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    pub async fn acquire_credentials_in_guest(&self, vm: &ManagedObjectReference, requested_auth: &dyn GuestAuthenticationTrait, session_id: Option<i64>) -> Result<Box<dyn GuestAuthenticationTrait>> {
        let input = AcquireCredentialsInGuestRequestType {vm, requested_auth, session_id, };
        let path = format!("/GuestAuthManager/{moId}/AcquireCredentialsInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn release_credentials_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn GuestAuthenticationTrait) -> Result<()> {
        let input = ReleaseCredentialsInGuestRequestType {vm, auth, };
        let path = format!("/GuestAuthManager/{moId}/ReleaseCredentialsInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn validate_credentials_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn GuestAuthenticationTrait) -> Result<()> {
        let input = ValidateCredentialsInGuestRequestType {vm, auth, };
        let path = format!("/GuestAuthManager/{moId}/ValidateCredentialsInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AcquireCredentialsInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    #[serde(rename = "requestedAuth")]
    requested_auth: &'a dyn GuestAuthenticationTrait,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sessionID")]
    session_id: Option<i64>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReleaseCredentialsInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn GuestAuthenticationTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ValidateCredentialsInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn GuestAuthenticationTrait,
}
