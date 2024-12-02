use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::GuestAuthenticationTrait;
use crate::types::GuestProcessInfo;
use crate::types::GuestProgramSpecTrait;
use crate::types::ManagedObjectReference;
/// ProcessManager is the managed object that provides APIs
/// to manipulate the guest operating system processes.
pub struct GuestProcessManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl GuestProcessManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// List the processes running in the guest operating system,
    /// plus those started by *GuestProcessManager.StartProgramInGuest*
    /// that have recently completed.
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
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### pids
    /// If set, only return information about the specified processes.
    /// Otherwise, information about all processes are returned.
    /// If a specified processes does not exist, nothing will
    /// be returned for that process.
    ///
    /// ## Returns:
    ///
    /// The list running processes is returned in an array of
    /// *GuestProcessInfo* structures.
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
    pub async fn list_processes_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn GuestAuthenticationTrait, pids: Option<&[i64]>) -> Result<Option<Vec<GuestProcessInfo>>> {
        let input = ListProcessesInGuestRequestType {vm, auth, pids, };
        let path = format!("/GuestProcessManager/{moId}/ListProcessesInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Reads an environment variable from the guest OS
    /// 
    /// If the authentication uses interactiveSession, then the
    /// environment being read will be that of the user logged into the desktop.
    /// Otherwise it's the environment of the user specified by the auth.
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
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### names
    /// The names of the variables to be read. If not set, then
    /// all the environment variables are returned.
    ///
    /// ## Returns:
    ///
    /// A string array containing the value of the variables,
    /// or all environment variables if nothing is specified.
    /// The format of each string is "name=value".
    /// If any specified environment variable isn't set, then nothing is
    /// returned for that variable.
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
    /// accepted by the guest OS.
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
    pub async fn read_environment_variable_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn GuestAuthenticationTrait, names: Option<&[String]>) -> Result<Option<Vec<String>>> {
        let input = ReadEnvironmentVariableInGuestRequestType {vm, auth, names, };
        let path = format!("/GuestProcessManager/{moId}/ReadEnvironmentVariableInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Starts a program in the guest operating system.
    /// 
    /// A process started this way can have its status queried with
    /// *GuestProcessManager.ListProcessesInGuest*.
    /// When the process completes, its exit code and end time will be
    /// available for 5 minutes after completion.
    /// 
    /// If VMware Tools is restarted, the exit code and end time will not
    /// be available.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Execute
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### spec
    /// The arguments describing the program to be started.
    ///
    /// ## Returns:
    ///
    /// The pid of the program started.
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
    /// ***FileNotFound***: if the program path does not exist.
    /// 
    /// ***FileFault***: if there is a file error in the guest operating system.
    /// 
    /// ***CannotAccessFile***: if the program path cannot be accessed.
    /// 
    /// ***GuestPermissionDenied***: if the program path cannot be run because
    /// the guest authentication will not allow the operation.
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
    pub async fn start_program_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn GuestAuthenticationTrait, spec: &dyn GuestProgramSpecTrait) -> Result<i64> {
        let input = StartProgramInGuestRequestType {vm, auth, spec, };
        let path = format!("/GuestProcessManager/{moId}/StartProgramInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Terminates a process in the guest OS.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Execute
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### pid
    /// Process ID of the process to be terminated
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
    /// ***GuestProcessNotFound***: if the pid does not refer to a valid process.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***GuestPermissionDenied***: if the process cannot be terminated because
    /// the guest authentication will not allow the operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to
    /// support the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due
    /// to guest agent configuration.
    pub async fn terminate_process_in_guest(&self, vm: &ManagedObjectReference, auth: &dyn GuestAuthenticationTrait, pid: i64) -> Result<()> {
        let input = TerminateProcessInGuestRequestType {vm, auth, pid, };
        let path = format!("/GuestProcessManager/{moId}/TerminateProcessInGuest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListProcessesInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn GuestAuthenticationTrait,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pids: Option<&'a [i64]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReadEnvironmentVariableInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn GuestAuthenticationTrait,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    names: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct StartProgramInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn GuestAuthenticationTrait,
    spec: &'a dyn GuestProgramSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct TerminateProcessInGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn GuestAuthenticationTrait,
    pid: i64,
}
