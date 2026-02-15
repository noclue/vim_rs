use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// FileManager is the managed object that provides APIs
/// to manipulate the guest operating system files.
#[derive(Clone)]
pub struct GuestFileManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl GuestFileManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Changes the file attributes of a specified file inside the guest.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual Machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### guest_file_path
    /// The complete path to the file to be copied in
    /// the guest. If the file points to an symbolic link, then the
    /// attributes of the target file are changed.
    ///
    /// ### file_attributes
    /// Specifies the different file attributes of the
    /// guest file to be changed.
    /// See *GuestFileAttributes*.
    /// If any property is not specified, then the specific attribute of
    /// the file will be unchanged.
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
    /// ***GuestPermissionDenied***: if the operation fails because
    /// the guest authentication will not allow the operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***FileFault***: if there is a file error in the guest operating system.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due
    /// to guest agent configuration.
    pub async fn change_file_attributes_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, guest_file_path: &str, file_attributes: &dyn crate::types::traits::GuestFileAttributesTrait) -> Result<()> {
        let input = ChangeFileAttributesInGuestRequestType {vm, auth, guest_file_path, file_attributes, };
        let path = format!("/GuestFileManager/{moId}/ChangeFileAttributesInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Creates a temporary directory.
    /// 
    /// Creates a new unique temporary directory for the user to use as needed.
    /// The user is responsible for removing it when it is no longer needed.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual Machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### prefix
    /// The prefix to be given to the new temporary directory.
    ///
    /// ### suffix
    /// The suffix to be given to the new temporary directory.
    ///
    /// ### directory_path
    /// The complete path to the directory in which to create the
    /// new directory. If unset or an empty string, a guest-specific location
    /// will be used.
    ///
    /// ## Returns:
    ///
    /// The absolute path of the temporary directory that is created.
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
    /// ***GuestPermissionDenied***: if the operation fails because
    /// the guest authentication will not allow the operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***FileFault***: if there is a file error in the guest operating system.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn create_temporary_directory_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, prefix: &str, suffix: &str, directory_path: Option<&str>) -> Result<String> {
        let input = CreateTemporaryDirectoryInGuestRequestType {vm, auth, prefix, suffix, directory_path, };
        let path = format!("/GuestFileManager/{moId}/CreateTemporaryDirectoryInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Creates a temporary file.
    /// 
    /// Creates a new unique temporary file for the user to use as needed.
    /// The user is responsible for removing it when it is no longer needed.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual Machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### prefix
    /// The prefix to be given to the new temporary file.
    ///
    /// ### suffix
    /// The suffix to be given to the new temporary file.
    ///
    /// ### directory_path
    /// The complete path to the directory in which to
    /// create the file.
    /// If unset, or an empty string, a guest-specific location will be used.
    ///
    /// ## Returns:
    ///
    /// The absolute path of the temporary file that is created.
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
    /// ***GuestPermissionDenied***: if the operation fails because
    /// the guest authentication will not allow the operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***FileFault***: if there is a file error in the guest operating system.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn create_temporary_file_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, prefix: &str, suffix: &str, directory_path: Option<&str>) -> Result<String> {
        let input = CreateTemporaryFileInGuestRequestType {vm, auth, prefix, suffix, directory_path, };
        let path = format!("/GuestFileManager/{moId}/CreateTemporaryFileInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deletes a directory in the guest OS.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual Machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### directory_path
    /// The complete path to the directory to be deleted.
    ///
    /// ### recursive
    /// If true, all subdirectories are also deleted.
    /// If false, the directory must be empty for the operation to succeed.
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
    /// ***GuestPermissionDenied***: if the operation fails because
    /// the guest authentication will not allow the operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***NotADirectory***: if the specified object is not a directory.
    /// 
    /// ***FileFault***: if there is a file error in the guest operating system.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn delete_directory_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, directory_path: &str, recursive: bool) -> Result<()> {
        let input = DeleteDirectoryInGuestRequestType {vm, auth, directory_path, recursive, };
        let path = format!("/GuestFileManager/{moId}/DeleteDirectoryInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Deletes a file in the guest OS
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual Machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### file_path
    /// The complete path to the file or symbolic link to be deleted.
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
    /// ***GuestPermissionDenied***: if the operation fails because
    /// the guest authentication will not allow the operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***NotAFile***: if the specified object is not a file.
    /// 
    /// ***FileFault***: if there is a file error in the guest operating system.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn delete_file_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, file_path: &str) -> Result<()> {
        let input = DeleteFileInGuestRequestType {vm, auth, file_path, };
        let path = format!("/GuestFileManager/{moId}/DeleteFileInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Initiates an operation to transfer a file from the guest.
    /// 
    /// Obtains a reference to
    /// *FileTransferInformation* object
    /// for the file transfer operation. The information object contains a URL
    /// to the file inside the guest to be transferred to the client.
    ///   
    /// See *FileTransferInformation* for
    /// information on how to use the information object. If the power state
    /// of the Virtual Machine is changed when the file transfer is in progress,
    /// or the Virtual Machine is migrated,
    /// then the transfer operation is aborted.
    /// 
    /// In order to ensure a secure connection to the host when transferring
    /// a file using HTTPS, the X.509 certificate for the host must be used
    /// to authenticate the remote end of the connection. The certificate of
    /// the host that the virtual machine is running on can be retrieved using
    /// the following fields:
    /// vm (*VirtualMachine*) -&gt; runtime (*VirtualMachineRuntimeInfo*)
    /// \-&gt; host (*HostSystem*) -&gt; config (*HostConfigInfo*)
    /// \-&gt; certificate.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual Machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Query
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data.
    ///
    /// ### guest_file_path
    /// The complete path to the file inside the guest
    /// that has to be transferred to the client. It cannot be a path to
    /// a directory or a symbolic link.
    ///
    /// ## Returns:
    ///
    /// A reference to
    /// *FileTransferInformation*.
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
    /// ***GuestPermissionDenied***: if the operation fails because
    /// the guest authentication will not allow the operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***FileFault***: if there is a file error in the guest operating system.
    /// 
    /// ***GuestComponentsOutOfDate***: If the guest agent is too old to
    /// support the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: If the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: If the operation is not enabled due
    /// to guest agent configuration.
    pub async fn initiate_file_transfer_from_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, guest_file_path: &str) -> Result<crate::types::structs::FileTransferInformation> {
        let input = InitiateFileTransferFromGuestRequestType {vm, auth, guest_file_path, };
        let path = format!("/GuestFileManager/{moId}/InitiateFileTransferFromGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::FileTransferInformation = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Initiates an operation to transfer a file to the guest.
    /// 
    /// Obtains a URL to the file inside the guest to be transferred from the
    /// client. The user should send a HTTP PUT request specifying the file
    /// content in the body of the request. Multiple PUT request cannot be
    /// sent to the URL simultaneously. URL will be invalidated after a
    /// successful PUT request is sent. If the power state of the Virtual
    /// Machine is changed when the file transfer is in progress, or
    /// the Virtual Machine is migrated, then the
    /// transfer operation is aborted.
    /// 
    /// In order to ensure a secure connection to the host when transferring
    /// a file using HTTPS, the X.509 certificate for the host must be used
    /// to authenticate the remote end of the connection. The certificate of
    /// the host that the virtual machine is running on can be retrieved using
    /// the following fields:
    /// vm (*VirtualMachine*) -&gt; runtime (*VirtualMachineRuntimeInfo*)
    /// \-&gt; host (*HostSystem*) -&gt; config (*HostConfigInfo*)
    /// \-&gt; certificate.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual Machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### guest_file_path
    /// The complete destination path in the guest to
    /// transfer the file from the client. It cannot be a path to
    /// a directory or a symbolic link.
    ///
    /// ### file_attributes
    /// File attributes of the file that has to be
    /// created in the guest. See *GuestFileAttributes*.
    /// If any file attribute is not specified, then the default value
    /// of that property will be set for the file.
    ///
    /// ### file_size
    /// Size of the file to transfer to the guest in bytes.
    ///
    /// ### overwrite
    /// If set, the destination file is clobbered.
    ///
    /// ## Returns:
    ///
    /// A URL to which the user has to send a PUT request.
    /// The host part of the URL is returned as '\*' if the hostname to be
    /// used is the name of the server to which the call was made. For
    /// example, if the call is made to esx-svr-1.domain1.com, and the file
    /// can be uploaded to
    /// `https://esx-svr-1.domain1.com/guestFile?id=1&token=1234`,
    /// the URL returned may be
    /// `https://&#42;/guestFile?id=1&token=1234`.
    /// The client replaces the asterisk with the server name on which it
    /// invoked the call.
    ///   
    ///   
    /// The URL is valid only for 10 minutes from the time it is generated.
    /// Also, the URL becomes invalid whenever the virtual machine is powered
    /// off, suspended or unregistered.
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
    /// ***GuestPermissionDenied***: if the operation fails because
    /// the guest authentication will not allow the operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***FileFault***: if there is a file error in the guest operating system.
    /// 
    /// ***GuestComponentsOutOfDate***: If the guest agent is too old to
    /// support the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: If the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: If the operation is not enabled due to
    /// guest agent configuration.
    pub async fn initiate_file_transfer_to_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, guest_file_path: &str, file_attributes: &dyn crate::types::traits::GuestFileAttributesTrait, file_size: i64, overwrite: bool) -> Result<String> {
        let input = InitiateFileTransferToGuestRequestType {vm, auth, guest_file_path, file_attributes, file_size, overwrite, };
        let path = format!("/GuestFileManager/{moId}/InitiateFileTransferToGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Returns information about files or directories in the guest.
    /// 
    /// The results could be extremely large, so to minimize the size
    /// of the return value for cases where a UI only needs to show
    /// the first N results, the answer is batched. Files are returned in
    /// OS-specific (inode) order. If the directory is modified between
    /// queries, missing or duplicate results can occur.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual Machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Query
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### file_path
    /// The complete path to the directory or file to query.
    ///
    /// ### index
    /// Which result to start the list with. The default is 0.
    ///
    /// ### max_results
    /// The maximum number of results to return. The default
    /// is 50.
    ///
    /// ### match_pattern
    /// A filter for the return values.
    /// Match patterns are specified using perl-compatible regular
    /// expressions.
    /// If matchPattern is unset, then the pattern '.\*' is used.
    ///
    /// ## Returns:
    ///
    /// A *GuestListFileInfo*
    /// object containing information for all the matching files
    /// in the filePath and the number of files left to be returned.
    ///
    /// ## Errors:
    ///
    /// ***GuestOperationsFault***: if there is an error processing a guest
    /// operation.
    /// 
    /// ***GuestOperationsUnavailable***: if the VM agent for guest operations
    /// is not running.
    /// 
    /// ***InvalidArgument***: If the matchPattern is an invalid regular
    /// expression.
    /// 
    /// ***InvalidPowerState***: if the VM is not powered on.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***GuestPermissionDenied***: if the operation fails because
    /// the guest authentication will not allow the operation.
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
    pub async fn list_files_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, file_path: &str, index: Option<i32>, max_results: Option<i32>, match_pattern: Option<&str>) -> Result<crate::types::structs::GuestListFileInfo> {
        let input = ListFilesInGuestRequestType {vm, auth, file_path, index, max_results, match_pattern, };
        let path = format!("/GuestFileManager/{moId}/ListFilesInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::GuestListFileInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Creates a directory in the guest OS
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual Machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### directory_path
    /// The complete path to the directory to be created.
    ///
    /// ### create_parent_directories
    /// Whether any parent directories
    /// are to be created.
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
    /// ***GuestPermissionDenied***: if the directory cannot be created because
    /// the guest authentication will not allow the operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***FileAlreadyExists***: if the specified object already exists.
    /// 
    /// ***FileFault***: if there is a file error in the guest operating system.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn make_directory_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, directory_path: &str, create_parent_directories: bool) -> Result<()> {
        let input = MakeDirectoryInGuestRequestType {vm, auth, directory_path, create_parent_directories, };
        let path = format!("/GuestFileManager/{moId}/MakeDirectoryInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Moves or renames a directory in the guest.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual Machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### src_directory_path
    /// The complete path to the directory to be moved.
    ///
    /// ### dst_directory_path
    /// The complete path to the where the directory is
    /// moved or its new name. It cannot be a path to an existing directory
    /// or an existing file.
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
    /// ***GuestPermissionDenied***: if the operation fails because
    /// the guest authentication will not allow the operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***FileFault***: if there is a file error in the guest operating system.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn move_directory_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, src_directory_path: &str, dst_directory_path: &str) -> Result<()> {
        let input = MoveDirectoryInGuestRequestType {vm, auth, src_directory_path, dst_directory_path, };
        let path = format!("/GuestFileManager/{moId}/MoveDirectoryInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Renames a file in the guest.
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Virtual Machine to perform the operation on.
    /// 
    /// ***Required privileges:*** VirtualMachine.GuestOperations.Modify
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### src_file_path
    /// The complete path to the original file or
    /// symbolic link to be moved.
    ///
    /// ### dst_file_path
    /// The complete path to the where the file is renamed.
    /// It cannot be a path to an existing directory.
    ///
    /// ### overwrite
    /// If set, the destination file is clobbered.
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
    /// ***GuestPermissionDenied***: if the operation fails because
    /// the guest authentication will not allow the operation.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***FileFault***: if there is a file error in the guest operating system.
    /// 
    /// ***GuestComponentsOutOfDate***: if the guest agent is too old to support
    /// the operation.
    /// 
    /// ***OperationNotSupportedByGuest***: if the operation is not supported
    /// by the guest OS.
    /// 
    /// ***OperationDisabledByGuest***: if the operation is not enabled due to
    /// guest agent configuration.
    pub async fn move_file_in_guest(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, src_file_path: &str, dst_file_path: &str, overwrite: bool) -> Result<()> {
        let input = MoveFileInGuestRequestType {vm, auth, src_file_path, dst_file_path, overwrite, };
        let path = format!("/GuestFileManager/{moId}/MoveFileInGuest", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
}
struct ChangeFileAttributesInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    guest_file_path: &'a str,
    file_attributes: &'a dyn crate::types::traits::GuestFileAttributesTrait,
}

impl<'a> miniserde::Serialize for ChangeFileAttributesInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ChangeFileAttributesInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ChangeFileAttributesInGuestRequestTypeSer<'b, 'a> {
    data: &'b ChangeFileAttributesInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ChangeFileAttributesInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ChangeFileAttributesInGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("guestFilePath"), &self.data.guest_file_path as &dyn miniserde::Serialize)),
            4 => return Some((std::borrow::Cow::Borrowed("fileAttributes"), &self.data.file_attributes as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateTemporaryDirectoryInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    prefix: &'a str,
    suffix: &'a str,
    directory_path: Option<&'a str>,
}

impl<'a> miniserde::Serialize for CreateTemporaryDirectoryInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateTemporaryDirectoryInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateTemporaryDirectoryInGuestRequestTypeSer<'b, 'a> {
    data: &'b CreateTemporaryDirectoryInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CreateTemporaryDirectoryInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateTemporaryDirectoryInGuestRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("prefix"), &self.data.prefix as &dyn miniserde::Serialize)),
                4 => return Some((std::borrow::Cow::Borrowed("suffix"), &self.data.suffix as &dyn miniserde::Serialize)),
                5 => {
                    let Some(ref val) = self.data.directory_path else { continue; };
                    return Some((std::borrow::Cow::Borrowed("directoryPath"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CreateTemporaryFileInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    prefix: &'a str,
    suffix: &'a str,
    directory_path: Option<&'a str>,
}

impl<'a> miniserde::Serialize for CreateTemporaryFileInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateTemporaryFileInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateTemporaryFileInGuestRequestTypeSer<'b, 'a> {
    data: &'b CreateTemporaryFileInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CreateTemporaryFileInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateTemporaryFileInGuestRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("prefix"), &self.data.prefix as &dyn miniserde::Serialize)),
                4 => return Some((std::borrow::Cow::Borrowed("suffix"), &self.data.suffix as &dyn miniserde::Serialize)),
                5 => {
                    let Some(ref val) = self.data.directory_path else { continue; };
                    return Some((std::borrow::Cow::Borrowed("directoryPath"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct DeleteDirectoryInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    directory_path: &'a str,
    recursive: bool,
}

impl<'a> miniserde::Serialize for DeleteDirectoryInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteDirectoryInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteDirectoryInGuestRequestTypeSer<'b, 'a> {
    data: &'b DeleteDirectoryInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DeleteDirectoryInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteDirectoryInGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("directoryPath"), &self.data.directory_path as &dyn miniserde::Serialize)),
            4 => return Some((std::borrow::Cow::Borrowed("recursive"), &self.data.recursive as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DeleteFileInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    file_path: &'a str,
}

impl<'a> miniserde::Serialize for DeleteFileInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteFileInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteFileInGuestRequestTypeSer<'b, 'a> {
    data: &'b DeleteFileInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DeleteFileInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteFileInGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("filePath"), &self.data.file_path as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct InitiateFileTransferFromGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    guest_file_path: &'a str,
}

impl<'a> miniserde::Serialize for InitiateFileTransferFromGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(InitiateFileTransferFromGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct InitiateFileTransferFromGuestRequestTypeSer<'b, 'a> {
    data: &'b InitiateFileTransferFromGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for InitiateFileTransferFromGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"InitiateFileTransferFromGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("guestFilePath"), &self.data.guest_file_path as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct InitiateFileTransferToGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    guest_file_path: &'a str,
    file_attributes: &'a dyn crate::types::traits::GuestFileAttributesTrait,
    file_size: i64,
    overwrite: bool,
}

impl<'a> miniserde::Serialize for InitiateFileTransferToGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(InitiateFileTransferToGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct InitiateFileTransferToGuestRequestTypeSer<'b, 'a> {
    data: &'b InitiateFileTransferToGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for InitiateFileTransferToGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"InitiateFileTransferToGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("guestFilePath"), &self.data.guest_file_path as &dyn miniserde::Serialize)),
            4 => return Some((std::borrow::Cow::Borrowed("fileAttributes"), &self.data.file_attributes as &dyn miniserde::Serialize)),
            5 => return Some((std::borrow::Cow::Borrowed("fileSize"), &self.data.file_size as &dyn miniserde::Serialize)),
            6 => return Some((std::borrow::Cow::Borrowed("overwrite"), &self.data.overwrite as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ListFilesInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    file_path: &'a str,
    index: Option<i32>,
    max_results: Option<i32>,
    match_pattern: Option<&'a str>,
}

impl<'a> miniserde::Serialize for ListFilesInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListFilesInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListFilesInGuestRequestTypeSer<'b, 'a> {
    data: &'b ListFilesInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ListFilesInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListFilesInGuestRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("filePath"), &self.data.file_path as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.index else { continue; };
                    return Some((std::borrow::Cow::Borrowed("index"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.max_results else { continue; };
                    return Some((std::borrow::Cow::Borrowed("maxResults"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.match_pattern else { continue; };
                    return Some((std::borrow::Cow::Borrowed("matchPattern"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct MakeDirectoryInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    directory_path: &'a str,
    create_parent_directories: bool,
}

impl<'a> miniserde::Serialize for MakeDirectoryInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MakeDirectoryInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MakeDirectoryInGuestRequestTypeSer<'b, 'a> {
    data: &'b MakeDirectoryInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for MakeDirectoryInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MakeDirectoryInGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("directoryPath"), &self.data.directory_path as &dyn miniserde::Serialize)),
            4 => return Some((std::borrow::Cow::Borrowed("createParentDirectories"), &self.data.create_parent_directories as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MoveDirectoryInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    src_directory_path: &'a str,
    dst_directory_path: &'a str,
}

impl<'a> miniserde::Serialize for MoveDirectoryInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MoveDirectoryInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MoveDirectoryInGuestRequestTypeSer<'b, 'a> {
    data: &'b MoveDirectoryInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for MoveDirectoryInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MoveDirectoryInGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("srcDirectoryPath"), &self.data.src_directory_path as &dyn miniserde::Serialize)),
            4 => return Some((std::borrow::Cow::Borrowed("dstDirectoryPath"), &self.data.dst_directory_path as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MoveFileInGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    src_file_path: &'a str,
    dst_file_path: &'a str,
    overwrite: bool,
}

impl<'a> miniserde::Serialize for MoveFileInGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MoveFileInGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MoveFileInGuestRequestTypeSer<'b, 'a> {
    data: &'b MoveFileInGuestRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for MoveFileInGuestRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MoveFileInGuestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("srcFilePath"), &self.data.src_file_path as &dyn miniserde::Serialize)),
            4 => return Some((std::borrow::Cow::Borrowed("dstFilePath"), &self.data.dst_file_path as &dyn miniserde::Serialize)),
            5 => return Some((std::borrow::Cow::Borrowed("overwrite"), &self.data.overwrite as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
