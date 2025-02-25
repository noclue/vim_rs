use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HttpNfcLeaseCapabilities;
use crate::types::structs::HttpNfcLeaseInfo;
use crate::types::structs::HttpNfcLeaseManifestEntry;
use crate::types::structs::HttpNfcLeaseProbeResult;
use crate::types::structs::HttpNfcLeaseSourceFile;
use crate::types::structs::KeyValue;
use crate::types::structs::ManagedObjectReference;
/// Represents a lease on a *VirtualMachine* or
/// a *VirtualApp*, which can be used to import or export
/// disks for the entity.
/// 
/// While the lease is held, operations
/// that alter the state of the virtual machines covered by the lease
/// are blocked. Examples of blocked operations are PowerOn, Destroy,
/// Migrate, etc.
/// 
/// A lease is in one of four states:
/// <dl>
/// <dt>Initializing</dt>
/// <dd>This is the initial state. The lease remains in this state
/// while the corresponding import/export task is preparing the
/// objects. In an import session, this involves creating
/// inventory objects.</dd>
/// <dt>Ready</dt>
/// <dd>The lease changes to this state once the corresponding
/// import/export task is done preparing the lease. The leased
/// objects are now ready, and the client can use the information
/// provided in the *HttpNfcLease.info* property to determine where to
/// up/download disks. The client must call *HttpNfcLease.HttpNfcLeaseProgress*
/// periodically to keep the lease alive and report progress to
/// the corresponding import/export task. Failure to do so causes
/// the lease to time out and enter the error state.</dd>
/// <dt>Done</dt>
/// <dd>When the client is done transferring disks, it calls
/// *HttpNfcLease.HttpNfcLeaseComplete* to signal the end of the import/export session.
/// This causes the corresponding import/export task to complete
/// successfully.</dd>
/// <dt>Error</dt>
/// <dd>If an error occurs during initialization or the lease times out,
/// it will change to this state. The client can also abort the lease
/// manually by calling *HttpNfcLease.HttpNfcLeaseAbort*. In this state, the *HttpNfcLease.error*
/// property can be read to determine the cause.
/// If the lease belongs to an import session, all objects created
/// during the import are removed when the lease enters this state.</dd>
/// </dl>
/// The import/export task corresponding to the lease continues running while
/// the lease is held.
pub struct HttpNfcLease {
    client: Arc<Client>,
    mo_id: String,
}
impl HttpNfcLease {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Aborts the import/export and releases this lease.
    /// 
    /// Operations on the
    /// objects contained in this lease will no longer be blocked. After
    /// calling this method, this lease will no longer be valid.
    /// 
    /// Clients should call this method if an error occurs while accessing
    /// the disks, or if the operation is cancelled. The client can report
    /// the cause of the abort to other clients listening on the task with
    /// the fault parameter.
    ///
    /// ## Parameters:
    ///
    /// ### fault
    /// \[in\] The fault that caused the abort, if any.
    ///
    /// ## Errors:
    ///
    /// ***Timedout***: if the lease has timed out before this call.
    /// 
    /// ***InvalidState***: if the lease has already been aborted.
    pub async fn http_nfc_lease_abort(&self, fault: Option<&dyn crate::types::traits::MethodFaultTrait>) -> Result<()> {
        let input = HttpNfcLeaseAbortRequestType {fault, };
        let path = format!("/HttpNfcLease/{moId}/HttpNfcLeaseAbort", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Completes the import/export and releases this lease.
    /// 
    /// Operations on
    /// the objects contained in this lease will no longer be blocked. After
    /// calling this method, this lease will no longer be valid.
    /// 
    /// Clients should call this method when they are done accessing the
    /// disks for the *VirtualMachine*s in this lease. The status
    /// of the corresponding task will be set to success.
    ///
    /// ## Errors:
    ///
    /// ***Timedout***: if the lease has timed out before this call.
    /// 
    /// ***InvalidState***: if the lease has already been completed or
    /// aborted.
    pub async fn http_nfc_lease_complete(&self) -> Result<()> {
        let path = format!("/HttpNfcLease/{moId}/HttpNfcLeaseComplete", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Gets the download manifest for this lease.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn http_nfc_lease_get_manifest(&self) -> Result<Option<Vec<HttpNfcLeaseManifestEntry>>> {
        let path = format!("/HttpNfcLease/{moId}/HttpNfcLeaseGetManifest", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Perform a series of validations on the target host to see if
    /// it can succesfully perform PullFromUrls.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    ///
    /// ## Parameters:
    ///
    /// ### files
    /// \[in\] List of remote source file descriptors
    /// There should be the same number of *HttpNfcLeaseSourceFile*
    /// as *HttpNfcLeaseDeviceUrl* provided by this lease.
    ///
    /// ### timeout
    /// \[in\] time in seconds for each url validation.
    /// Maximum timeout is 60.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if no source files are provided.
    /// 
    /// ***InvalidState***: if the lease has already been aborted.
    pub async fn http_nfc_lease_probe_urls(&self, files: Option<&[HttpNfcLeaseSourceFile]>, timeout: Option<i32>) -> Result<Option<Vec<HttpNfcLeaseProbeResult>>> {
        let input = HttpNfcLeaseProbeUrlsRequestType {files, timeout, };
        let path = format!("/HttpNfcLease/{moId}/HttpNfcLeaseProbeUrls", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Sets the disk up/download progress, and renews this lease.
    /// 
    /// A lease
    /// will time out automatically after a while. If the client wishes to
    /// continue using it, for example if it is not done accessing the
    /// disks, this method must be called periodically.
    ///
    /// ## Parameters:
    ///
    /// ### percent
    /// \[in\] Completion status represented as an integer
    /// in the 0-100 range.
    ///
    /// ## Errors:
    ///
    /// ***Timedout***: if the lease has timed out or vSphere has not
    /// detected data transfer progress.
    pub async fn http_nfc_lease_progress(&self, percent: i32) -> Result<()> {
        let input = HttpNfcLeaseProgressRequestType {percent, };
        let path = format!("/HttpNfcLease/{moId}/HttpNfcLeaseProgress", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Upgrades current lease from push to pull mode.
    ///
    /// ## Parameters:
    ///
    /// ### files
    /// \[in\] List of remote source file descriptors
    /// There should be the same number of *HttpNfcLeaseSourceFile*
    /// as *HttpNfcLeaseDeviceUrl* provided by this lease.
    /// Privilege VApp.PullFromUrls is required.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the lease has already been aborted.
    pub async fn http_nfc_lease_pull_from_urls_task(&self, files: Option<&[HttpNfcLeaseSourceFile]>) -> Result<ManagedObjectReference> {
        let input = HttpNfcLeasePullFromUrlsRequestType {files, };
        let path = format!("/HttpNfcLease/{moId}/HttpNfcLeasePullFromUrls_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Sets desired checksum algorithm per each file that will be returned in
    /// ManifestEntry.
    /// 
    /// Should be set before any transfer starts.
    ///
    /// ## Parameters:
    ///
    /// ### device_urls_to_checksum_types
    /// \[in\] Should contain key value pairs:
    /// where key is *HttpNfcLeaseDeviceUrl.key* returned in this lease info and value
    /// is desired algorithm from *HttpNfcLeaseManifestEntryChecksumType_enum*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn http_nfc_lease_set_manifest_checksum_type(&self, device_urls_to_checksum_types: Option<&[KeyValue]>) -> Result<()> {
        let input = HttpNfcLeaseSetManifestChecksumTypeRequestType {device_urls_to_checksum_types, };
        let path = format!("/HttpNfcLease/{moId}/HttpNfcLeaseSetManifestChecksumType", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Current supported capabilities by this lease
    /// See *HttpNfcLeaseCapabilities*
    pub async fn capabilities(&self) -> Result<HttpNfcLeaseCapabilities> {
        let path = format!("/HttpNfcLease/{moId}/capabilities", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// If the lease is in the error state, this property contains the
    /// error that caused the lease to be aborted.
    pub async fn error(&self) -> Result<Option<Box<dyn crate::types::traits::MethodFaultTrait>>> {
        let path = format!("/HttpNfcLease/{moId}/error", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Provides information on the objects contained in this lease.
    /// 
    /// The
    /// info property is only valid when the lease is in the ready state.
    pub async fn info(&self) -> Result<Option<HttpNfcLeaseInfo>> {
        let path = format!("/HttpNfcLease/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Provides progress information (0-100 percent) for the initializing state
    /// of the lease.
    /// 
    /// Clients can use this to track overall progress.
    pub async fn initialize_progress(&self) -> Result<i32> {
        let path = format!("/HttpNfcLease/{moId}/initializeProgress", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Current mode of the lease.
    /// 
    /// See *HttpNfcLeaseMode_enum* for possible values.
    pub async fn mode(&self) -> Result<String> {
        let path = format!("/HttpNfcLease/{moId}/mode", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The current state of the lease.
    pub async fn state(&self) -> Result<crate::types::enums::HttpNfcLeaseStateEnum> {
        let path = format!("/HttpNfcLease/{moId}/state", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Provides progress information (0-100 percent) for current transfer.
    /// 
    /// Transfer covers download, upload and pull scenario.
    /// Can be externally updated by progress method.
    pub async fn transfer_progress(&self) -> Result<i32> {
        let path = format!("/HttpNfcLease/{moId}/transferProgress", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HttpNfcLeaseAbortRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    fault: Option<&'a dyn crate::types::traits::MethodFaultTrait>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HttpNfcLeaseProbeUrlsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    files: Option<&'a [HttpNfcLeaseSourceFile]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timeout: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HttpNfcLeaseProgressRequestType {
    percent: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HttpNfcLeasePullFromUrlsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    files: Option<&'a [HttpNfcLeaseSourceFile]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HttpNfcLeaseSetManifestChecksumTypeRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deviceUrlsToChecksumTypes")]
    device_urls_to_checksum_types: Option<&'a [KeyValue]>,
}
