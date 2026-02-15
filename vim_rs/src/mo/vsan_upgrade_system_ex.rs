use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// It deprecates vim.VsanUpgradeSystem, to take care of vSAN
/// upgrade process.
/// 
/// It has capability to support additional advanced disk format
/// conversion specification, to help on configuration of
/// latest vSAN advanced features.
/// It supports following vSAN on-disk format versions:
/// - version 1, released by vSAN 1.0, vSphere ESXi 5.5U1;
/// - version 2, released by vSAN 6.0, which supports Virsto file
///   system;
/// - version 3, released by vSAN 6.2, which supports vSAN
///   deduplication and compression.
///   
/// It can be accessed with MOID vsan-upgrade-systemex through vSAN
/// service at vCenter server side.
#[derive(Clone)]
pub struct VsanUpgradeSystemEx {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanUpgradeSystemEx {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Start vSAN disk format conversion on a particular cluster.
    /// 
    /// It deprecates API vim.VsanUpgradeSystem.PerformVsanUpgrade,
    /// besides supporting original basic upgrade options, it also supports
    /// advanced disk format conversion spec, to convert vSAN disk
    /// format to support corresponding vSAN advanced features, such
    /// as vSAN deduplication and compression, etc.
    /// 
    /// In order to perform this on-disk format upgrade, the upgrade process
    /// will perform a rolling evacuation/remove/re-add operation to accomplish
    /// the upgrade. In other words, one disk group at a time, it will evacuate
    /// the data from the disk group, then remove the old format from the now
    /// empty disk group, then reformat the disk group with the new format.
    /// Once all disk groups have been upgraded, and if the performObjectUpgrade
    /// parameter is set, the vSAN object version is also upgraded. Before
    /// the object version is upgraded, it is possible to downgrade the cluster
    /// by passing the downgradeFormat parameter. Once objects are of the new
    /// object version however, downgrade (and thus rollback) are no longer
    /// possible. The new object version is required to allow objects to benefit
    /// from new vSAN features.
    /// 
    /// The upgrade process performs additional "preflight checks" before
    /// proceeding to upgrade the next host. The upgrade process will be halted
    /// if any of those preflight checks fail.
    /// 
    /// If the upgrade process has been halted due to a problem, or even due to
    /// a crash or other failure, it can be re-started at any point in time.
    /// The upgrade will resume where it left off and only do the parts that
    /// are still outstanding. If the upgrade process stopped after removing
    /// vSAN from a disk group, but before re-adding those disks to
    /// vSAN, the upgrade process can recover from that. The preflight
    /// check results indicate such a condition.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster to be upgraded
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### perform_object_upgrade
    /// After all disk groups have been updated, also
    /// upgrade all objects. Once started, rollback
    /// of the on disk format is no longer possible.
    /// Object upgrade unlocks new vSAN features. The
    /// default behavior is upgrading objects is this
    /// parameter is not specified.
    ///
    /// ### downgrade_format
    /// Perform a on-disk format downgrade instead of
    /// upgrade. Only possible if no upgraded objects exist.
    ///
    /// ### allow_reduced_redundancy
    /// Removes the need for one disk group worth of
    /// free space, by allowing reduced redundancy
    /// during disk upgrade.
    ///
    /// ### exclude_hosts
    /// Internal debug option meant for functional testing
    /// of vSAN upgrades. Skips upgrade on certain hosts and
    /// implies performObjectUpgrade being false. Should not
    /// be used by customers.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ### spec
    /// The specification of advanced disk format configuration. The
    /// spec cannot be supported in downgrade process, for now we don't
    /// have any advanced features supported by lower disk format.
    ///
    /// ## Returns:
    ///
    /// vim.Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If a current upgrade or precheck is already
    /// in progress
    pub async fn perform_vsan_upgrade_ex(&self, cluster: &crate::types::structs::ManagedObjectReference, perform_object_upgrade: Option<bool>, downgrade_format: Option<bool>, allow_reduced_redundancy: Option<bool>, exclude_hosts: Option<&[crate::types::structs::ManagedObjectReference]>, spec: Option<&crate::types::structs::VsanDiskFormatConversionSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = PerformVsanUpgradeExRequestType {cluster, perform_object_upgrade, downgrade_format, allow_reduced_redundancy, exclude_hosts, spec, };
        let path = format!("/vsan/VsanUpgradeSystemEx/{moId}/PerformVsanUpgradeEx", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Perform an upgrade preflight check on a cluster asynchronously.
    /// 
    /// Has the same arguments as *VsanUpgradeSystemEx.PerformVsanUpgradePreflightCheckEx* for argument
    /// details.
    /// The checks are performed asynchronously tracked by a task. Upon completion
    /// results can be obtained by calling *VsanUpgradeSystemEx.VsanQueryUpgradeStatusEx*
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster to process conversion.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### downgrade_format
    /// Intend to perform a on-disk format downgrade
    /// instead of upgrade. Adds additional checks.
    ///
    /// ### spec
    /// The specification of advanced disk format configuration.
    /// Adds additional checks. The specification cannot be
    /// supported in downgrade process, for now we don't
    /// have any advanced features supported by lower disk format.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If a current upgrade or precheck is already
    /// in progress
    pub async fn perform_vsan_upgrade_preflight_async_check_task(&self, cluster: &crate::types::structs::ManagedObjectReference, downgrade_format: Option<bool>, spec: Option<&crate::types::structs::VsanDiskFormatConversionSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = PerformVsanUpgradePreflightAsyncCheckRequestType {cluster, downgrade_format, spec, };
        let path = format!("/vsan/VsanUpgradeSystemEx/{moId}/PerformVsanUpgradePreflightAsyncCheck_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Perform an upgrade preflight check on a cluster.
    /// 
    /// It deprecates API vim.VsanUpgradeSystem.PerformVsanUpgradePreflightCheck,
    /// and helps to check following issues which would break vSAN upgrade
    /// process:
    /// 1. Any vSAN host is disconnected from vCenter server
    /// 2. ESXi host before vSphere 6.0 exists in vSAN cluster, of
    ///    which on-disk format cannot be upgraded
    /// 3. Any vSAN host enables auto-mode to claim disk
    /// 4. Network issue exists in vSAN cluster, which leads to communication
    ///    problem among vSAN hosts
    /// 5. Not all vSAN member hosts reside in specified vCenter cluster
    /// 6. Any vSAN host contributes in the other vSAN cluster
    /// 7. There is inaccessible vSAN object in vSAN datastore
    /// 8. Any in-use vSAN disk that are unhealthy, such as physical
    ///    disk lost
    /// 9. Hybrid disk group exists in vSAN cluster, and requests to enable
    ///    Deduplication and Compression
    /// 10. To request enable Deduplication and Compression, but there is ESXi host
    ///     software version is below then ESXi 6.0 Update 2
    /// 11. vSAN downgrade is requested, but vSAN objects have already
    ///     been upgraded to 2.0 or above
    /// 12. Any vSAN object that would present an upgrade issue 
    ///     
    /// Any issue detected by preflight check, requires manual fix by end user,
    /// before trigger vSAN upgrade process through PerformVsanUpgradeEx.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster to be converted.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### downgrade_format
    /// Intend to perform a on-disk format downgrade
    /// instead of upgrade. Adds additional checks.
    ///
    /// ### spec
    /// The specification of advanced disk format configuration.
    /// Adds additional checks. The specification cannot be
    /// supported in downgrade process, for now we don't
    /// have any advanced features supported by lower disk format.
    ///
    /// ## Returns:
    ///
    /// Preflight check result.
    pub async fn perform_vsan_upgrade_preflight_check_ex(&self, cluster: &crate::types::structs::ManagedObjectReference, downgrade_format: Option<bool>, spec: Option<&crate::types::structs::VsanDiskFormatConversionSpec>) -> Result<crate::types::structs::VsanDiskFormatConversionCheckResult> {
        let input = PerformVsanUpgradePreflightCheckExRequestType {cluster, downgrade_format, spec, };
        let path = format!("/vsan/VsanUpgradeSystemEx/{moId}/PerformVsanUpgradePreflightCheckEx", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VsanDiskFormatConversionCheckResult = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Retrieve the latest status of a running, or the previously completed,
    /// upgrade or precheck process.
    /// 
    /// Information about previous upgrade runs are not
    /// always, e.g. when vCenter gets restarted.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster for which to retrieve the upgrade status.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// Status
    pub async fn vsan_query_upgrade_status_ex(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::VsanUpgradeStatusEx> {
        let input = VsanQueryUpgradeStatusExRequestType {cluster, };
        let path = format!("/vsan/VsanUpgradeSystemEx/{moId}/VsanQueryUpgradeStatusEx", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VsanUpgradeStatusEx = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Process a scan and retrieve the highest vSAN disk format
    /// version that given cluster supports, the version is up to version
    /// of ESXi host in specified cluster:
    /// ESX of VSAN2017Q1, support version is 5.
    /// 
    /// ESX of vSphere6.0u2 and vSphere6.5, support version is 4;
    /// ESX of vSphere6.0 series before vSphere6.0U2, support version is 2;
    /// ESX of lower version, is not supported;
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster to process scan.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// int To present the highest supported disk format version.
    pub async fn retrieve_supported_vsan_format_version(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<i32> {
        let input = RetrieveSupportedVsanFormatVersionRequestType {cluster, };
        let path = format!("/vsan/VsanUpgradeSystemEx/{moId}/RetrieveSupportedVsanFormatVersion", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: i32 = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct PerformVsanUpgradeExRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    perform_object_upgrade: Option<bool>,
    downgrade_format: Option<bool>,
    allow_reduced_redundancy: Option<bool>,
    exclude_hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    spec: Option<&'a crate::types::structs::VsanDiskFormatConversionSpec>,
}

impl<'a> miniserde::Serialize for PerformVsanUpgradeExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PerformVsanUpgradeExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PerformVsanUpgradeExRequestTypeSer<'b, 'a> {
    data: &'b PerformVsanUpgradeExRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PerformVsanUpgradeExRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PerformVsanUpgradeExRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.perform_object_upgrade else { continue; };
                    return Some((std::borrow::Cow::Borrowed("performObjectUpgrade"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.downgrade_format else { continue; };
                    return Some((std::borrow::Cow::Borrowed("downgradeFormat"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.allow_reduced_redundancy else { continue; };
                    return Some((std::borrow::Cow::Borrowed("allowReducedRedundancy"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.exclude_hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("excludeHosts"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct PerformVsanUpgradePreflightAsyncCheckRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    downgrade_format: Option<bool>,
    spec: Option<&'a crate::types::structs::VsanDiskFormatConversionSpec>,
}

impl<'a> miniserde::Serialize for PerformVsanUpgradePreflightAsyncCheckRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PerformVsanUpgradePreflightAsyncCheckRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PerformVsanUpgradePreflightAsyncCheckRequestTypeSer<'b, 'a> {
    data: &'b PerformVsanUpgradePreflightAsyncCheckRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PerformVsanUpgradePreflightAsyncCheckRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PerformVsanUpgradePreflightAsyncCheckRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.downgrade_format else { continue; };
                    return Some((std::borrow::Cow::Borrowed("downgradeFormat"), val as &dyn miniserde::Serialize));
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
struct PerformVsanUpgradePreflightCheckExRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    downgrade_format: Option<bool>,
    spec: Option<&'a crate::types::structs::VsanDiskFormatConversionSpec>,
}

impl<'a> miniserde::Serialize for PerformVsanUpgradePreflightCheckExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PerformVsanUpgradePreflightCheckExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PerformVsanUpgradePreflightCheckExRequestTypeSer<'b, 'a> {
    data: &'b PerformVsanUpgradePreflightCheckExRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for PerformVsanUpgradePreflightCheckExRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PerformVsanUpgradePreflightCheckExRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.downgrade_format else { continue; };
                    return Some((std::borrow::Cow::Borrowed("downgradeFormat"), val as &dyn miniserde::Serialize));
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
struct VsanQueryUpgradeStatusExRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanQueryUpgradeStatusExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryUpgradeStatusExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryUpgradeStatusExRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryUpgradeStatusExRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanQueryUpgradeStatusExRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryUpgradeStatusExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveSupportedVsanFormatVersionRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for RetrieveSupportedVsanFormatVersionRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveSupportedVsanFormatVersionRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveSupportedVsanFormatVersionRequestTypeSer<'b, 'a> {
    data: &'b RetrieveSupportedVsanFormatVersionRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RetrieveSupportedVsanFormatVersionRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveSupportedVsanFormatVersionRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
