use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostPatchManagerLocator;
use crate::types::structs::HostPatchManagerPatchManagerOperationSpec;
use crate::types::structs::ManagedObjectReference;
/// This managed object is the interface for scanning and patching an ESX
/// server.
/// 
/// VMware publishes updates through its external website. A patch update is
/// synonymous with a bulletin. An update may contain many individual patch
/// binaries, but its installation and uninstallation are atomic.
pub struct HostPatchManager {
    client: Arc<Client>,
    mo_id: String,
}
impl HostPatchManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
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
    pub async fn check_host_patch_task(&self, meta_urls: Option<&[String]>, bundle_urls: Option<&[String]>, spec: Option<&HostPatchManagerPatchManagerOperationSpec>) -> Result<ManagedObjectReference> {
        let input = CheckHostPatchRequestType {meta_urls, bundle_urls, spec, };
        let path = format!("/HostPatchManager/{moId}/CheckHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn install_host_patch_task(&self, repository: &HostPatchManagerLocator, update_id: &str, force: Option<bool>) -> Result<ManagedObjectReference> {
        let input = InstallHostPatchRequestType {repository, update_id, force, };
        let path = format!("/HostPatchManager/{moId}/InstallHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
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
    pub async fn install_host_patch_v_2_task(&self, meta_urls: Option<&[String]>, bundle_urls: Option<&[String]>, vib_urls: Option<&[String]>, spec: Option<&HostPatchManagerPatchManagerOperationSpec>) -> Result<ManagedObjectReference> {
        let input = InstallHostPatchV2RequestType {meta_urls, bundle_urls, vib_urls, spec, };
        let path = format!("/HostPatchManager/{moId}/InstallHostPatchV2_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
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
    pub async fn query_host_patch_task(&self, spec: Option<&HostPatchManagerPatchManagerOperationSpec>) -> Result<ManagedObjectReference> {
        let input = QueryHostPatchRequestType {spec, };
        let path = format!("/HostPatchManager/{moId}/QueryHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn scan_host_patch_task(&self, repository: &HostPatchManagerLocator, update_id: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = ScanHostPatchRequestType {repository, update_id, };
        let path = format!("/HostPatchManager/{moId}/ScanHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
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
    pub async fn scan_host_patch_v_2_task(&self, meta_urls: Option<&[String]>, bundle_urls: Option<&[String]>, spec: Option<&HostPatchManagerPatchManagerOperationSpec>) -> Result<ManagedObjectReference> {
        let input = ScanHostPatchV2RequestType {meta_urls, bundle_urls, spec, };
        let path = format!("/HostPatchManager/{moId}/ScanHostPatchV2_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
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
    pub async fn stage_host_patch_task(&self, meta_urls: Option<&[String]>, bundle_urls: Option<&[String]>, vib_urls: Option<&[String]>, spec: Option<&HostPatchManagerPatchManagerOperationSpec>) -> Result<ManagedObjectReference> {
        let input = StageHostPatchRequestType {meta_urls, bundle_urls, vib_urls, spec, };
        let path = format!("/HostPatchManager/{moId}/StageHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
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
    pub async fn uninstall_host_patch_task(&self, bulletin_ids: Option<&[String]>, spec: Option<&HostPatchManagerPatchManagerOperationSpec>) -> Result<ManagedObjectReference> {
        let input = UninstallHostPatchRequestType {bulletin_ids, spec, };
        let path = format!("/HostPatchManager/{moId}/UninstallHostPatch_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckHostPatchRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metaUrls")]
    meta_urls: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bundleUrls")]
    bundle_urls: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a HostPatchManagerPatchManagerOperationSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct InstallHostPatchRequestType<'a> {
    repository: &'a HostPatchManagerLocator,
    #[serde(rename = "updateID")]
    update_id: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct InstallHostPatchV2RequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metaUrls")]
    meta_urls: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bundleUrls")]
    bundle_urls: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vibUrls")]
    vib_urls: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a HostPatchManagerPatchManagerOperationSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryHostPatchRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a HostPatchManagerPatchManagerOperationSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ScanHostPatchRequestType<'a> {
    repository: &'a HostPatchManagerLocator,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updateID")]
    update_id: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ScanHostPatchV2RequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metaUrls")]
    meta_urls: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bundleUrls")]
    bundle_urls: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a HostPatchManagerPatchManagerOperationSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct StageHostPatchRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metaUrls")]
    meta_urls: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bundleUrls")]
    bundle_urls: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vibUrls")]
    vib_urls: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a HostPatchManagerPatchManagerOperationSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UninstallHostPatchRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bulletinIds")]
    bulletin_ids: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a HostPatchManagerPatchManagerOperationSpec>,
}
