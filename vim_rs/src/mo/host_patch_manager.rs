use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object is the interface for scanning and patching an ESX
/// server.
/// 
/// VMware publishes updates through its external website. A patch update is
/// synonymous with a bulletin. An update may contain many individual patch
/// binaries, but its installation and uninstallation are atomic.
#[derive(Clone)]
pub struct HostPatchManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostPatchManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of vSphere 8.0u3, and there is no replacement available.
    /// 
    /// Check the list of metadata and returns the dependency, obsolete and conflict information
    /// The operation is cancelable through the returned *Task* object.
    /// 
    /// No integrity checks
    /// are performed on the metadata.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### meta_urls
    /// a list of urls pointing to metadata.zip.
    ///
    /// ### bundle_urls
    /// a list of urls pointing to an "offline" bundle. It is not supported in 5.0 or later.
    ///
    /// ### spec
    /// -
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the
    /// *HostPatchManagerStatus*
    /// upon success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***RequestCanceled***: if the operation is canceled.
    /// 
    /// ***InvalidState***: if the feature cannot be supported on the platform,
    /// potentially because the hardware configuration does not support it.
    /// 
    /// ***TaskInProgress***: if there is already a patch installation in progress.
    /// 
    /// ***PlatformConfigFault***: if any error occurs during the operation.
    /// More detailed information will be returned within the payload of the
    /// exception as xml string.
    pub async fn check_host_patch_task(&self, meta_urls: Option<&[String]>, bundle_urls: Option<&[String]>, spec: Option<&crate::types::structs::HostPatchManagerPatchManagerOperationSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CheckHostPatchRequestType {meta_urls, bundle_urls, spec, };
        let path = format!("/HostPatchManager/{moId}/CheckHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated method is deprecated, use *HostPatchManager.InstallHostPatchV2_Task* instead.
    /// 
    /// Patch the host.
    /// 
    /// The operation is not cancelable. If the
    /// patch installation failed, an atomic rollback of the installation will
    /// be attempted. Manual rollback is required if the atomic rollback
    /// failed, see *PatchInstallFailed* for details.
    /// 
    /// ***Required privileges:*** Host.Config.Patch
    ///
    /// ## Parameters:
    ///
    /// ### repository
    /// Location of the repository that contains the
    /// bulletin depot. The depot must be organized as a flat
    /// collection of bulletins with each one being a folder named
    /// after the bulletin ID. Each folder must contain both
    /// update metadata and required binaries.
    ///
    /// ### update_id
    /// The update to be installed on the host.
    ///
    /// ### force
    /// Specify whether to force reinstall an update.
    /// By default, installing an already-installed update would fail
    /// with the *PatchAlreadyInstalled* fault. If
    /// force is set to true, the update will be forcefully reinstalled,
    /// thus overwriting the already installed update.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***PatchMetadataInvalid***: if the required metadata is invalid - for
    /// example, it is not found in the repository, is corrupted and so
    /// on. Typically a more specific subclass of PatchMetadataInvalid is
    /// thrown.
    /// 
    /// ***PatchBinariesNotFound***: if required update related binaries were not
    /// available.
    /// 
    /// ***PatchNotApplicable***: if the patch is not applicable. Typically a
    /// more specific subclass of PatchNotApplicable is thrown to indicate
    /// a specific problem - for example, PatchSuperseded if the patch is
    /// superseded, MissingDependency if required patch or libraries are not
    /// installed, AlreadyInstalled if the patch is already installed.
    /// 
    /// ***NoDiskSpace***: if the update can not be installed because there is
    /// insufficient disk space for the installation, including temporary
    /// space used for rollback.
    /// 
    /// ***PatchInstallFailed***: if the installation failed,
    /// *PlatformConfigFault.text* has details of the
    /// failure. Automatic rollback might have succeeded or failed.
    /// 
    /// ***RebootRequired***: if the update cannot be installed without
    /// restarting the host. This might occur on account of a prior
    /// update installation which needed to be installed separately
    /// from other updates.
    /// 
    /// ***InvalidState***: if the host is not in maintenance mode but the
    /// patch install requires all virtual machines to be powered off.
    /// 
    /// ***TaskInProgress***: if there is already a patch installation in progress.
    pub async fn install_host_patch_task(&self, repository: &crate::types::structs::HostPatchManagerLocator, update_id: &str, force: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = InstallHostPatchRequestType {repository, update_id, force, };
        let path = format!("/HostPatchManager/{moId}/InstallHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere 8.0u3, and there is no replacement available.
    /// 
    /// Patch the host.
    /// 
    /// The operation is not cancelable. If the
    /// patch installation failed, an atomic rollback of the installation will
    /// be attempted. Manual rollback is required if the atomic rollback
    /// failed, see *PatchInstallFailed* for details.
    /// 
    /// ***Required privileges:*** Host.Config.Patch
    ///
    /// ## Parameters:
    ///
    /// ### meta_urls
    /// A list of urls pointing to metadata.zip.
    ///
    /// ### bundle_urls
    /// a list of urls pointing to an "offline" bundle. It is not supported in 5.0 or later.
    ///
    /// ### vib_urls
    /// The urls of update binary files to be installed.
    ///
    /// ### spec
    /// -
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***RequestCanceled***: if the operation is canceled.
    /// 
    /// ***InvalidState***: if the feature cannot be supported on the platform,
    /// potentially because the hardware configuration does not support it.
    /// 
    /// ***TaskInProgress***: if there is already a patch installation in progress.
    /// 
    /// ***PlatformConfigFault***: if any error occurs during the operation.
    /// More detailed information will be returned within the payload of the
    /// exception as xml string.
    pub async fn install_host_patch_v_2_task(&self, meta_urls: Option<&[String]>, bundle_urls: Option<&[String]>, vib_urls: Option<&[String]>, spec: Option<&crate::types::structs::HostPatchManagerPatchManagerOperationSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = InstallHostPatchV2RequestType {meta_urls, bundle_urls, vib_urls, spec, };
        let path = format!("/HostPatchManager/{moId}/InstallHostPatchV2_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere 8.0u3, and there is no replacement available.
    /// 
    /// Query the host for installed bulletins.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// -
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***RequestCanceled***: if the operation is canceled.
    /// 
    /// ***InvalidState***: if the bulletin ID did not exist.
    /// 
    /// ***TaskInProgress***: if there is already a patch installation in progress.
    /// 
    /// ***PlatformConfigFault***: if any error occurs during the operation.
    /// More detailed information will be returned within the payload of the
    /// exception as xml string.
    pub async fn query_host_patch_task(&self, spec: Option<&crate::types::structs::HostPatchManagerPatchManagerOperationSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = QueryHostPatchRequestType {spec, };
        let path = format!("/HostPatchManager/{moId}/QueryHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of VI API 4.0, use *HostPatchManager.ScanHostPatchV2_Task*.
    /// 
    /// Scan the host for the patch status.
    /// 
    /// The operation is cancelable
    /// through the returned *Task* object. Integrity checks are
    /// performed on the metadata only during the scan operation.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### repository
    /// Location of the repository that contains the
    /// bulletin depot. The depot must be organized as a flat
    /// collection of bulletins with each one being a folder named
    /// after the bulletin ID. Each folder must contain the full
    /// update metadata.
    ///
    /// ### update_id
    /// The updates to scan. Wildcards can be used to specify
    /// the update IDs. The wildcards will be expanded to include all
    /// updates whose IDs match the specified wildcard and whose metadata
    /// is available in the repository. Specifying no update is
    /// equivalent to a wildcard "\*". In this case all updates available
    /// in the repository will be scanned.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the
    /// *HostPatchManagerStatus*
    /// upon success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***RequestCanceled***: if the operation is canceled.
    /// 
    /// ***PatchMetadataInvalid***: if query required metadata is invalid - for
    /// example, it is not found in the repository, is corrupted and
    /// so on. Typically a more specific subclass of PatchMetadataInvalid
    /// is thrown.
    /// 
    /// ***PlatformConfigFault***: if there is any error in the repository access,
    /// metadata download, repository level integrity check, or reading the
    /// metadata. See *PlatformConfigFault.text* for
    /// specific details.
    pub async fn scan_host_patch_task(&self, repository: &crate::types::structs::HostPatchManagerLocator, update_id: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ScanHostPatchRequestType {repository, update_id, };
        let path = format!("/HostPatchManager/{moId}/ScanHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere 8.0u3, and there is no replacement available.
    /// 
    /// Scan the host for the patch status.
    /// 
    /// The operation is cancelable
    /// through the returned *Task* object. Integrity checks are
    /// performed on the metadata only during the scan operation.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### meta_urls
    /// a list of urls pointing to metadata.zip.
    ///
    /// ### bundle_urls
    /// a list of urls pointing to an "offline" bundle. It is not supported in 5.0 or later.
    ///
    /// ### spec
    /// -
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the
    /// *HostPatchManagerStatus*
    /// upon success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***RequestCanceled***: if the operation is canceled.
    /// 
    /// ***InvalidState***: if the feature cannot be supported on the platform,
    /// potentially because the hardware configuration does not support it.
    /// 
    /// ***TaskInProgress***: if there is already a patch installation in progress.
    /// 
    /// ***PlatformConfigFault***: if there is any error in the repository access,
    /// metadata download, repository level integrity check, or reading the
    /// metadata. See *PlatformConfigFault.text* for
    /// specific details.
    pub async fn scan_host_patch_v_2_task(&self, meta_urls: Option<&[String]>, bundle_urls: Option<&[String]>, spec: Option<&crate::types::structs::HostPatchManagerPatchManagerOperationSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ScanHostPatchV2RequestType {meta_urls, bundle_urls, spec, };
        let path = format!("/HostPatchManager/{moId}/ScanHostPatchV2_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere 8.0u3, and there is no replacement available.
    /// 
    /// Stage the vib files to esx local location and possibly do some run time check.
    /// 
    /// ***Required privileges:*** Host.Config.Patch
    ///
    /// ## Parameters:
    ///
    /// ### meta_urls
    /// A list of urls pointing to metadata.zip.
    ///
    /// ### bundle_urls
    /// a list of urls pointing to an "offline" bundle. It is not supported in 5.0 or later.
    ///
    /// ### vib_urls
    /// The urls of update binary files to be staged.
    ///
    /// ### spec
    /// -
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the
    /// *HostPatchManagerStatus*
    /// upon success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***RequestCanceled***: if the operation is canceled.
    /// 
    /// ***InvalidState***: if the feature cannot be supported on the platform,
    /// potentially because the hardware configuration does not support it.
    /// 
    /// ***TaskInProgress***: if there is already a patch installation in progress.
    /// 
    /// ***PlatformConfigFault***: if any error occurs during the operation.
    /// More detailed information will be returned within the payload of the
    /// exception as xml string.
    pub async fn stage_host_patch_task(&self, meta_urls: Option<&[String]>, bundle_urls: Option<&[String]>, vib_urls: Option<&[String]>, spec: Option<&crate::types::structs::HostPatchManagerPatchManagerOperationSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = StageHostPatchRequestType {meta_urls, bundle_urls, vib_urls, spec, };
        let path = format!("/HostPatchManager/{moId}/StageHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere 8.0u3, and there is no replacement available.
    /// 
    /// Uninstall patch from the host.
    /// 
    /// The operation is not cancelable.
    /// 
    /// ***Required privileges:*** Host.Config.Patch
    ///
    /// ## Parameters:
    ///
    /// ### bulletin_ids
    /// A list of bulletin IDs to be removed.
    ///
    /// ### spec
    /// -
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the feature cannot be supported on the platform,
    /// potentially because the hardware configuration does not support it.
    /// 
    /// ***TaskInProgress***: if there is already a patch installation in progress.
    /// 
    /// ***PlatformConfigFault***: if any error occurs during the operation.
    /// More detailed information will be returned within the payload of the
    /// exception as xml string.
    pub async fn uninstall_host_patch_task(&self, bulletin_ids: Option<&[String]>, spec: Option<&crate::types::structs::HostPatchManagerPatchManagerOperationSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UninstallHostPatchRequestType {bulletin_ids, spec, };
        let path = format!("/HostPatchManager/{moId}/UninstallHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct CheckHostPatchRequestType<'a> {
    meta_urls: Option<&'a [String]>,
    bundle_urls: Option<&'a [String]>,
    spec: Option<&'a crate::types::structs::HostPatchManagerPatchManagerOperationSpec>,
}

impl<'a> miniserde::Serialize for CheckHostPatchRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CheckHostPatchRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CheckHostPatchRequestTypeSer<'b, 'a> {
    data: &'b CheckHostPatchRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CheckHostPatchRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CheckHostPatchRequestType")),
                1 => {
                    let Some(ref val) = self.data.meta_urls else { continue; };
                    return Some((std::borrow::Cow::Borrowed("metaUrls"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.bundle_urls else { continue; };
                    return Some((std::borrow::Cow::Borrowed("bundleUrls"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct InstallHostPatchRequestType<'a> {
    repository: &'a crate::types::structs::HostPatchManagerLocator,
    update_id: &'a str,
    force: Option<bool>,
}

impl<'a> miniserde::Serialize for InstallHostPatchRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(InstallHostPatchRequestTypeSer { data: self, seq: 0 }))
    }
}

struct InstallHostPatchRequestTypeSer<'b, 'a> {
    data: &'b InstallHostPatchRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for InstallHostPatchRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"InstallHostPatchRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("repository"), &self.data.repository as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("updateID"), &self.data.update_id as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.force else { continue; };
                    return Some((std::borrow::Cow::Borrowed("force"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct InstallHostPatchV2RequestType<'a> {
    meta_urls: Option<&'a [String]>,
    bundle_urls: Option<&'a [String]>,
    vib_urls: Option<&'a [String]>,
    spec: Option<&'a crate::types::structs::HostPatchManagerPatchManagerOperationSpec>,
}

impl<'a> miniserde::Serialize for InstallHostPatchV2RequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(InstallHostPatchV2RequestTypeSer { data: self, seq: 0 }))
    }
}

struct InstallHostPatchV2RequestTypeSer<'b, 'a> {
    data: &'b InstallHostPatchV2RequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for InstallHostPatchV2RequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"InstallHostPatchV2RequestType")),
                1 => {
                    let Some(ref val) = self.data.meta_urls else { continue; };
                    return Some((std::borrow::Cow::Borrowed("metaUrls"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.bundle_urls else { continue; };
                    return Some((std::borrow::Cow::Borrowed("bundleUrls"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.vib_urls else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vibUrls"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryHostPatchRequestType<'a> {
    spec: Option<&'a crate::types::structs::HostPatchManagerPatchManagerOperationSpec>,
}

impl<'a> miniserde::Serialize for QueryHostPatchRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryHostPatchRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryHostPatchRequestTypeSer<'b, 'a> {
    data: &'b QueryHostPatchRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryHostPatchRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryHostPatchRequestType")),
                1 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ScanHostPatchRequestType<'a> {
    repository: &'a crate::types::structs::HostPatchManagerLocator,
    update_id: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for ScanHostPatchRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ScanHostPatchRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ScanHostPatchRequestTypeSer<'b, 'a> {
    data: &'b ScanHostPatchRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ScanHostPatchRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ScanHostPatchRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("repository"), &self.data.repository as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.update_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("updateID"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ScanHostPatchV2RequestType<'a> {
    meta_urls: Option<&'a [String]>,
    bundle_urls: Option<&'a [String]>,
    spec: Option<&'a crate::types::structs::HostPatchManagerPatchManagerOperationSpec>,
}

impl<'a> miniserde::Serialize for ScanHostPatchV2RequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ScanHostPatchV2RequestTypeSer { data: self, seq: 0 }))
    }
}

struct ScanHostPatchV2RequestTypeSer<'b, 'a> {
    data: &'b ScanHostPatchV2RequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ScanHostPatchV2RequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ScanHostPatchV2RequestType")),
                1 => {
                    let Some(ref val) = self.data.meta_urls else { continue; };
                    return Some((std::borrow::Cow::Borrowed("metaUrls"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.bundle_urls else { continue; };
                    return Some((std::borrow::Cow::Borrowed("bundleUrls"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct StageHostPatchRequestType<'a> {
    meta_urls: Option<&'a [String]>,
    bundle_urls: Option<&'a [String]>,
    vib_urls: Option<&'a [String]>,
    spec: Option<&'a crate::types::structs::HostPatchManagerPatchManagerOperationSpec>,
}

impl<'a> miniserde::Serialize for StageHostPatchRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(StageHostPatchRequestTypeSer { data: self, seq: 0 }))
    }
}

struct StageHostPatchRequestTypeSer<'b, 'a> {
    data: &'b StageHostPatchRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for StageHostPatchRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"StageHostPatchRequestType")),
                1 => {
                    let Some(ref val) = self.data.meta_urls else { continue; };
                    return Some((std::borrow::Cow::Borrowed("metaUrls"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.bundle_urls else { continue; };
                    return Some((std::borrow::Cow::Borrowed("bundleUrls"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.vib_urls else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vibUrls"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UninstallHostPatchRequestType<'a> {
    bulletin_ids: Option<&'a [String]>,
    spec: Option<&'a crate::types::structs::HostPatchManagerPatchManagerOperationSpec>,
}

impl<'a> miniserde::Serialize for UninstallHostPatchRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UninstallHostPatchRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UninstallHostPatchRequestTypeSer<'b, 'a> {
    data: &'b UninstallHostPatchRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UninstallHostPatchRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UninstallHostPatchRequestType")),
                1 => {
                    let Some(ref val) = self.data.bulletin_ids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("bulletinIds"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
