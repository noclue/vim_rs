use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
#[derive(Clone)]
pub struct HttpNfcLease {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HttpNfcLease {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn http_nfc_lease_abort(&self, fault: Option<&crate::types::structs::MethodFault>) -> Result<()> {
        let input = HttpNfcLeaseAbortRequestType {fault, };
        self.client.invoke_void("", "HttpNfcLease", &self.mo_id, "HttpNfcLeaseAbort", Some(&input)).await
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
        self.client.invoke_void("", "HttpNfcLease", &self.mo_id, "HttpNfcLeaseComplete", None).await
    }
    /// Gets the download manifest for this lease.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn http_nfc_lease_get_manifest(&self) -> Result<Option<Vec<crate::types::structs::HttpNfcLeaseManifestEntry>>> {
        let bytes_opt = self.client.invoke_optional("", "HttpNfcLease", &self.mo_id, "HttpNfcLeaseGetManifest", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn http_nfc_lease_probe_urls(&self, files: Option<&[crate::types::structs::HttpNfcLeaseSourceFile]>, timeout: Option<i32>) -> Result<Option<Vec<crate::types::structs::HttpNfcLeaseProbeResult>>> {
        let input = HttpNfcLeaseProbeUrlsRequestType {files, timeout, };
        let bytes_opt = self.client.invoke_optional("", "HttpNfcLease", &self.mo_id, "HttpNfcLeaseProbeUrls", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        self.client.invoke_void("", "HttpNfcLease", &self.mo_id, "HttpNfcLeaseProgress", Some(&input)).await
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
    pub async fn http_nfc_lease_pull_from_urls_task(&self, files: Option<&[crate::types::structs::HttpNfcLeaseSourceFile]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = HttpNfcLeasePullFromUrlsRequestType {files, };
        let bytes = self.client.invoke("", "HttpNfcLease", &self.mo_id, "HttpNfcLeasePullFromUrls_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn http_nfc_lease_set_manifest_checksum_type(&self, device_urls_to_checksum_types: Option<&[crate::types::structs::KeyValue]>) -> Result<()> {
        let input = HttpNfcLeaseSetManifestChecksumTypeRequestType {device_urls_to_checksum_types, };
        self.client.invoke_void("", "HttpNfcLease", &self.mo_id, "HttpNfcLeaseSetManifestChecksumType", Some(&input)).await
    }
    /// Current supported capabilities by this lease
    /// See *HttpNfcLeaseCapabilities*
    pub async fn capabilities(&self) -> Result<crate::types::structs::HttpNfcLeaseCapabilities> {
        let bytes_opt = self.client.fetch_property_raw("", "HttpNfcLease", &self.mo_id, "capabilities").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property capabilities was empty".to_string()))?;
        let result: crate::types::structs::HttpNfcLeaseCapabilities = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// If the lease is in the error state, this property contains the
    /// error that caused the lease to be aborted.
    pub async fn error(&self) -> Result<Option<crate::types::structs::MethodFault>> {
        let bytes_opt = self.client.fetch_property_raw("", "HttpNfcLease", &self.mo_id, "error").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Provides information on the objects contained in this lease.
    /// 
    /// The
    /// info property is only valid when the lease is in the ready state.
    pub async fn info(&self) -> Result<Option<crate::types::structs::HttpNfcLeaseInfo>> {
        let bytes_opt = self.client.fetch_property_raw("", "HttpNfcLease", &self.mo_id, "info").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Provides progress information (0-100 percent) for the initializing state
    /// of the lease.
    /// 
    /// Clients can use this to track overall progress.
    pub async fn initialize_progress(&self) -> Result<i32> {
        let bytes_opt = self.client.fetch_property_raw("", "HttpNfcLease", &self.mo_id, "initializeProgress").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property initializeProgress was empty".to_string()))?;
        let result: i32 = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Current mode of the lease.
    /// 
    /// See *HttpNfcLeaseMode_enum* for possible values.
    pub async fn mode(&self) -> Result<String> {
        let bytes_opt = self.client.fetch_property_raw("", "HttpNfcLease", &self.mo_id, "mode").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property mode was empty".to_string()))?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// The current state of the lease.
    pub async fn state(&self) -> Result<crate::types::enums::HttpNfcLeaseStateEnum> {
        let bytes_opt = self.client.fetch_property_raw("", "HttpNfcLease", &self.mo_id, "state").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property state was empty".to_string()))?;
        let result: crate::types::enums::HttpNfcLeaseStateEnum = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Provides progress information (0-100 percent) for current transfer.
    /// 
    /// Transfer covers download, upload and pull scenario.
    /// Can be externally updated by progress method.
    pub async fn transfer_progress(&self) -> Result<i32> {
        let bytes_opt = self.client.fetch_property_raw("", "HttpNfcLease", &self.mo_id, "transferProgress").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property transferProgress was empty".to_string()))?;
        let result: i32 = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct HttpNfcLeaseAbortRequestType<'a> {
    fault: Option<&'a crate::types::structs::MethodFault>,
}

impl<'a> miniserde::Serialize for HttpNfcLeaseAbortRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HttpNfcLeaseAbortRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HttpNfcLeaseAbortRequestTypeSer<'b, 'a> {
    data: &'b HttpNfcLeaseAbortRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HttpNfcLeaseAbortRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HttpNfcLeaseAbortRequestType")),
                1 => {
                    let Some(ref val) = self.data.fault else { continue; };
                    return Some((std::borrow::Cow::Borrowed("fault"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct HttpNfcLeaseProbeUrlsRequestType<'a> {
    files: Option<&'a [crate::types::structs::HttpNfcLeaseSourceFile]>,
    timeout: Option<i32>,
}

impl<'a> miniserde::Serialize for HttpNfcLeaseProbeUrlsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HttpNfcLeaseProbeUrlsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HttpNfcLeaseProbeUrlsRequestTypeSer<'b, 'a> {
    data: &'b HttpNfcLeaseProbeUrlsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HttpNfcLeaseProbeUrlsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HttpNfcLeaseProbeUrlsRequestType")),
                1 => {
                    let Some(ref val) = self.data.files else { continue; };
                    return Some((std::borrow::Cow::Borrowed("files"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.timeout else { continue; };
                    return Some((std::borrow::Cow::Borrowed("timeout"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct HttpNfcLeaseProgressRequestType {
    percent: i32,
}

impl miniserde::Serialize for HttpNfcLeaseProgressRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HttpNfcLeaseProgressRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HttpNfcLeaseProgressRequestTypeSer<'b> {
    data: &'b HttpNfcLeaseProgressRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for HttpNfcLeaseProgressRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HttpNfcLeaseProgressRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("percent"), &self.data.percent as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HttpNfcLeasePullFromUrlsRequestType<'a> {
    files: Option<&'a [crate::types::structs::HttpNfcLeaseSourceFile]>,
}

impl<'a> miniserde::Serialize for HttpNfcLeasePullFromUrlsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HttpNfcLeasePullFromUrlsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HttpNfcLeasePullFromUrlsRequestTypeSer<'b, 'a> {
    data: &'b HttpNfcLeasePullFromUrlsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HttpNfcLeasePullFromUrlsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HttpNfcLeasePullFromUrlsRequestType")),
                1 => {
                    let Some(ref val) = self.data.files else { continue; };
                    return Some((std::borrow::Cow::Borrowed("files"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct HttpNfcLeaseSetManifestChecksumTypeRequestType<'a> {
    device_urls_to_checksum_types: Option<&'a [crate::types::structs::KeyValue]>,
}

impl<'a> miniserde::Serialize for HttpNfcLeaseSetManifestChecksumTypeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HttpNfcLeaseSetManifestChecksumTypeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HttpNfcLeaseSetManifestChecksumTypeRequestTypeSer<'b, 'a> {
    data: &'b HttpNfcLeaseSetManifestChecksumTypeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HttpNfcLeaseSetManifestChecksumTypeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HttpNfcLeaseSetManifestChecksumTypeRequestType")),
                1 => {
                    let Some(ref val) = self.data.device_urls_to_checksum_types else { continue; };
                    return Some((std::borrow::Cow::Borrowed("deviceUrlsToChecksumTypes"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
