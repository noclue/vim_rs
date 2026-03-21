use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object provides extended APIs to enrich
/// *HostVsanSystem*'s functionality, operating at a single
/// ESXi host level.
/// 
/// *HostVsanSystem* is still valid and provided
/// by vSphere Management SDK. *HostVsanSystem* and VsanSystemEx
/// should be used in combination for different operations to vSAN.
/// It can be accessed with MOID of 'vsanSystemEx' through vSAN service at
/// ESXi host side.
/// It can be also accessed with MOID of 'vsanSystemEx-&lt;hostId&gt;' through vSAN
/// service at vCenter server side, where &lt;hostId&gt; is the ID at the end of
/// the host's MOID: 'host-&lt;hostId&gt;'. For example, if a host's MOID is
/// 'host-5', then 'vsanSystemEx-5' should be used to access this host's
/// VsanSystemEx MO.
#[derive(Clone)]
pub struct VsanSystemEx {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanSystemEx {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Abort outstanding secure wipe disk request.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### disks
    /// -
    ///
    /// ## Returns:
    ///
    /// *VsanHostAbortWipeDiskStatus*\[\] which contain each disk
    /// abort wipe return status.
    pub async fn vsan_host_abort_wipe_disk(&self, disks: &[String]) -> Result<Vec<crate::types::structs::VsanHostAbortWipeDiskStatus>> {
        let input = VsanHostAbortWipeDiskRequestType {disks, };
        let bytes = self.client.invoke("vsan", "VsanSystemEx", &self.mo_id, "VsanHostAbortWipeDisk", Some(&input)).await?;
        let result: Vec<crate::types::structs::VsanHostAbortWipeDiskStatus> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query system information of ESXi such as name, type, version, build type
    /// and build number.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// VsanAboutInfoEx
    pub async fn vsan_get_about_info_ex(&self) -> Result<crate::types::structs::VsanHostAboutInfoEx> {
        let bytes = self.client.invoke("vsan", "VsanSystemEx", &self.mo_id, "VsanGetAboutInfoEx", None).await?;
        let result: crate::types::structs::VsanHostAboutInfoEx = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Get vSAN runtime stats of vSAN host.
    /// 
    /// This API is used to retrieve host's vSAN runtime stats. It bases on
    /// specified stats type list, to retrieve expected vSAN runtime stats.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### stats
    /// List of vSAN runtime stats type. Supported vSAN runtime stats
    /// types are declared in *VsanHostStatsType_enum*.
    /// If this parameter is omitted, all supported runtime stats will
    /// be collected and returned.
    ///
    /// ### cluster_uuid
    /// The cluster uuid used for vSAN runtime stats of
    /// configGeneration in *VsanHostStatsType_enum*.
    ///
    /// ## Returns:
    ///
    /// vim.vsan.host.RuntimeStats
    pub async fn vsan_host_get_runtime_stats(&self, stats: Option<&[String]>, cluster_uuid: Option<&str>) -> Result<crate::types::structs::VsanHostRuntimeStats> {
        let input = VsanHostGetRuntimeStatsRequestType {stats, cluster_uuid, };
        let bytes = self.client.invoke("vsan", "VsanSystemEx", &self.mo_id, "VsanHostGetRuntimeStats", Some(&input)).await?;
        let result: crate::types::structs::VsanHostRuntimeStats = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query vSAN DRS stats of this host and all vSAN VMs registered
    /// on this host.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### host_uuids
    /// In regular vSAN cluster, it indicates sorted
    /// list of vSAN member host Uuid. It is used to
    /// determine the full set of vSAN member hosts
    /// and the sequence to organize vSAN VM stats.
    /// In stretched compute only cluster, it is an array
    /// filled by '0' and '1', to indicate the host
    /// membership of configured site fault domains,
    /// this is to fill in the gap of absence of
    /// vSAN host fault domain on vSAN enabled node.
    ///
    /// ### vms
    /// List of VM instance UUID. If specified, only stats of
    /// given VMs will be returned, otherwise all vSAN VMs
    /// registered on this host will be involved if this
    /// parameter is omitted. By default template VM will be
    /// ignored unless it is specified by this paramter.
    ///
    /// ### host_index
    /// Only applicable to stretched compute only node,
    /// to indicate the index of the target host in
    /// parameter 'hostUuids'.
    ///
    /// ## Returns:
    ///
    /// *VsanHostDrsStats*
    pub async fn vsan_query_host_drs_stats(&self, host_uuids: Option<&[String]>, vms: Option<&[String]>, host_index: Option<i32>) -> Result<crate::types::structs::VsanHostDrsStats> {
        let input = VsanQueryHostDrsStatsRequestType {host_uuids, vms, host_index, };
        let bytes = self.client.invoke("vsan", "VsanSystemEx", &self.mo_id, "VsanQueryHostDrsStats", Some(&input)).await?;
        let result: crate::types::structs::VsanHostDrsStats = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query this host's current runtime status for the VSAN service.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster_uuids
    /// A list of vSAN cluster uuids. If this parameter is
    /// provided, we will only return the runtime status of recognized clusters;
    /// And if it is omitted, we will return that of all clusters associated to
    /// this host.
    ///
    /// ## Returns:
    ///
    /// an array of ClusterStatus which represent host's cluster status
    /// information for the vSAN service.
    pub async fn vsan_query_host_status_ex(&self, cluster_uuids: Option<&[String]>) -> Result<Option<Vec<crate::types::structs::VsanHostClusterStatus>>> {
        let input = VsanQueryHostStatusExRequestType {cluster_uuids, };
        let bytes_opt = self.client.invoke_optional("vsan", "VsanSystemEx", &self.mo_id, "VsanQueryHostStatusEx", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Query information about vSAN objects that are currently syncing data.
    /// 
    /// It
    /// will retrieve information about syncing objects based on query options.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### uuids
    /// Restricts the list of object identities to query.
    ///
    /// ### start
    /// Indicates the number of objects to be skipped when returns
    /// query results. A value less than 0 is illegal. Default: 0.
    ///
    /// ### limit
    /// Restricts the max number of objects to return. Valid value
    /// is from 0 to 200. A value less than 0 is illegal, 0 is valid
    /// only when "includeSummary" is true. Default: 100.
    ///
    /// ### include_summary
    /// Indicates whether to return summary data in query
    /// result. Default: True.
    ///
    /// ## Returns:
    ///
    /// vim.vsan.host.VsanSyncingObjectQueryResult.
    pub async fn vsan_query_syncing_vsan_objects(&self, uuids: Option<&[String]>, start: Option<i32>, limit: Option<i32>, include_summary: Option<bool>) -> Result<crate::types::structs::VsanHostVsanObjectSyncQueryResult> {
        let input = VsanQuerySyncingVsanObjectsRequestType {uuids, start, limit, include_summary, };
        let bytes = self.client.invoke("vsan", "VsanSystemEx", &self.mo_id, "VsanQuerySyncingVsanObjects", Some(&input)).await?;
        let result: crate::types::structs::VsanHostVsanObjectSyncQueryResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query what it takes for an entity(disk group or host) to be evacuated in
    /// various modes.
    /// 
    /// Only account for capacity usage when it determines if an
    /// entity can be evacuated in a given mode. Note that this means all other
    /// considerations such as fault domain, number of spindles or read cache are
    /// ignored and the API will only verify if the cluster has enough capacity to
    /// accommodate the affected objects in a specified mode.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### evac_entity_uuid
    /// The entity UUID which will be evacuated. It can be
    /// an UUID of disk, disk group or host.
    ///
    /// ## Returns:
    ///
    /// WhatIfEvacResult which indicate whether the evacuation can be done
    /// in current cluster state and specified evacuation mode. If so,
    /// this API returns what objects would lose liveness or compliance.
    /// Or on the other hand, if the given entity cannot be evacuated, API
    /// returns many extra resources needed to satisfy this operation.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_what_if_evacuation_result(&self, evac_entity_uuid: &str) -> Result<crate::types::structs::VsanWhatIfEvacResult> {
        let input = VsanQueryWhatIfEvacuationResultRequestType {evac_entity_uuid, };
        let bytes = self.client.invoke("vsan", "VsanSystemEx", &self.mo_id, "VsanQueryWhatIfEvacuationResult", Some(&input)).await?;
        let result: crate::types::structs::VsanWhatIfEvacResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query disk level information for securely wipe disk.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### disks
    /// -
    ///
    /// ## Returns:
    ///
    /// *VsanHostWipeDiskStatus*\[\] which contain each disk
    /// wipe status.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_query_wipe_disk(&self, disks: &[String]) -> Result<Vec<crate::types::structs::VsanHostWipeDiskStatus>> {
        let input = VsanHostQueryWipeDiskRequestType {disks, };
        let bytes = self.client.invoke("vsan", "VsanSystemEx", &self.mo_id, "VsanHostQueryWipeDisk", Some(&input)).await?;
        let result: Vec<crate::types::structs::VsanHostWipeDiskStatus> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Unmount vSAN diskgroup, which stops using specified vSAN diskgroup
    /// for any vSAN I/O without destroying it.
    /// 
    /// This API extended and deprecated
    /// *HostVsanSystem.UnmountDiskMapping_Task*, added vSAN data migration
    /// support, to provide data protection during the process. Without data
    /// migration, for vSAN data which is not configured with redundancy
    /// (policy hostFailureToTolerate=0), or in incompliant state
    /// (data redundancy reduced when policy hostFailureToTolerate&gt;0),
    /// its accessibility could be broken after diskgroup unmounted.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### mappings
    /// List of diskgroups to be unmounted.
    ///
    /// ### maintenance_spec
    /// Any additional actions to move data out of the
    /// diskgroup before unmounting it. See
    /// *HostMaintenanceSpec*. If unspecified,
    /// there is no action taken to move data from the
    /// diskgroup.
    ///
    /// ### timeout
    /// Time to wait for the task to complete in seconds. If the
    /// value is less than or equal to zero, there is no timeout.
    /// The operation fails with a Timedout exception if it timed out.
    ///
    /// ### evac_reason
    /// The reason to evacuate diskgroup.
    /// Allowed values can be found in
    /// *VsanDiskEvacReason_enum*
    /// 
    /// ***Since:*** 8.0.0.4
    ///
    /// ## Returns:
    ///
    /// *Task* to track the diskrgoup unmounting process.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_unmount_disk_mapping_ex(&self, mappings: &[crate::types::structs::VsanHostDiskMapping], maintenance_spec: Option<&crate::types::structs::HostMaintenanceSpec>, timeout: Option<i32>, evac_reason: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanUnmountDiskMappingExRequestType {mappings, maintenance_spec, timeout, evac_reason, };
        let bytes = self.client.invoke("vsan", "VsanSystemEx", &self.mo_id, "VsanUnmountDiskMappingEx", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Initiate task for securely wipe disk.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### disks
    /// -
    ///
    /// ## Returns:
    ///
    /// a *Task* to track secure wipe progress, and the task is
    /// not cancelable.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: if any one of the disks is
    /// ineligible for wipe.
    /// Upon successful completion of the returned *Task*, its
    /// \* *TaskInfo.result* field will be populated with
    /// \* string which is disk canonical name list.
    /// Upon failure of the returned *Task*, its
    /// \* *TaskInfo.error* field will be populated with
    /// \* *MethodFault* for specific disk fault information.
    pub async fn vsan_host_wipe_disk(&self, disks: &[String]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanHostWipeDiskRequestType {disks, };
        let bytes = self.client.invoke("vsan", "VsanSystemEx", &self.mo_id, "VsanHostWipeDisk", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct VsanHostAbortWipeDiskRequestType<'a> {
    disks: &'a [String],
}

impl<'a> miniserde::Serialize for VsanHostAbortWipeDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostAbortWipeDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostAbortWipeDiskRequestTypeSer<'b, 'a> {
    data: &'b VsanHostAbortWipeDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostAbortWipeDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostAbortWipeDiskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("disks"), &self.data.disks as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanHostGetRuntimeStatsRequestType<'a> {
    stats: Option<&'a [String]>,
    cluster_uuid: Option<&'a str>,
}

impl<'a> miniserde::Serialize for VsanHostGetRuntimeStatsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostGetRuntimeStatsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostGetRuntimeStatsRequestTypeSer<'b, 'a> {
    data: &'b VsanHostGetRuntimeStatsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostGetRuntimeStatsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostGetRuntimeStatsRequestType")),
                1 => {
                    let Some(ref val) = self.data.stats else { continue; };
                    return Some((std::borrow::Cow::Borrowed("stats"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.cluster_uuid else { continue; };
                    return Some((std::borrow::Cow::Borrowed("clusterUuid"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQueryHostDrsStatsRequestType<'a> {
    host_uuids: Option<&'a [String]>,
    vms: Option<&'a [String]>,
    host_index: Option<i32>,
}

impl<'a> miniserde::Serialize for VsanQueryHostDrsStatsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryHostDrsStatsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryHostDrsStatsRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryHostDrsStatsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryHostDrsStatsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryHostDrsStatsRequestType")),
                1 => {
                    let Some(ref val) = self.data.host_uuids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hostUuids"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.vms else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vms"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.host_index else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hostIndex"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQueryHostStatusExRequestType<'a> {
    cluster_uuids: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for VsanQueryHostStatusExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryHostStatusExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryHostStatusExRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryHostStatusExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryHostStatusExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryHostStatusExRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster_uuids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("clusterUuids"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQuerySyncingVsanObjectsRequestType<'a> {
    uuids: Option<&'a [String]>,
    start: Option<i32>,
    limit: Option<i32>,
    include_summary: Option<bool>,
}

impl<'a> miniserde::Serialize for VsanQuerySyncingVsanObjectsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQuerySyncingVsanObjectsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQuerySyncingVsanObjectsRequestTypeSer<'b, 'a> {
    data: &'b VsanQuerySyncingVsanObjectsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQuerySyncingVsanObjectsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQuerySyncingVsanObjectsRequestType")),
                1 => {
                    let Some(ref val) = self.data.uuids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("uuids"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.start else { continue; };
                    return Some((std::borrow::Cow::Borrowed("start"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.limit else { continue; };
                    return Some((std::borrow::Cow::Borrowed("limit"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.include_summary else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeSummary"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQueryWhatIfEvacuationResultRequestType<'a> {
    evac_entity_uuid: &'a str,
}

impl<'a> miniserde::Serialize for VsanQueryWhatIfEvacuationResultRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryWhatIfEvacuationResultRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryWhatIfEvacuationResultRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryWhatIfEvacuationResultRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryWhatIfEvacuationResultRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryWhatIfEvacuationResultRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("evacEntityUuid"), &self.data.evac_entity_uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanHostQueryWipeDiskRequestType<'a> {
    disks: &'a [String],
}

impl<'a> miniserde::Serialize for VsanHostQueryWipeDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostQueryWipeDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostQueryWipeDiskRequestTypeSer<'b, 'a> {
    data: &'b VsanHostQueryWipeDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostQueryWipeDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostQueryWipeDiskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("disks"), &self.data.disks as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanUnmountDiskMappingExRequestType<'a> {
    mappings: &'a [crate::types::structs::VsanHostDiskMapping],
    maintenance_spec: Option<&'a crate::types::structs::HostMaintenanceSpec>,
    timeout: Option<i32>,
    evac_reason: Option<&'a str>,
}

impl<'a> miniserde::Serialize for VsanUnmountDiskMappingExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanUnmountDiskMappingExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanUnmountDiskMappingExRequestTypeSer<'b, 'a> {
    data: &'b VsanUnmountDiskMappingExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanUnmountDiskMappingExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanUnmountDiskMappingExRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("mappings"), &self.data.mappings as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.maintenance_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("maintenanceSpec"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.timeout else { continue; };
                    return Some((std::borrow::Cow::Borrowed("timeout"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.evac_reason else { continue; };
                    return Some((std::borrow::Cow::Borrowed("evacReason"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanHostWipeDiskRequestType<'a> {
    disks: &'a [String],
}

impl<'a> miniserde::Serialize for VsanHostWipeDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostWipeDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostWipeDiskRequestTypeSer<'b, 'a> {
    data: &'b VsanHostWipeDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostWipeDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostWipeDiskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("disks"), &self.data.disks as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
