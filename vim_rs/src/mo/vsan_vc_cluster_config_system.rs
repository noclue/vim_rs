use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type provides a comprehensive way to manage vSAN
/// cluster configuration in below areas:
/// - Enable or disable vSAN
/// - Enable or disable Autoclaim mode for disk group
/// - Enable or disable data efficiency feature
/// - Configure vSAN iSCSI target feature
/// - Manage disk groups
/// - Manage fault domains
/// - Retrieve vSAN generic configuration
/// - Retrieve data efficiency configuration
///   
/// In this class, ReconfigureEx enhances the legacy API
/// *ComputeResource.ReconfigureComputeResource_Task*.
/// 
/// Legacy API
/// *ComputeResource.ReconfigureComputeResource_Task* is deprecated.  
/// The ManagedEntity can be accessed with MOID of 'vsan-cluster-config-system',
/// through vSAN service at vCenter server side.
#[derive(Clone)]
pub struct VsanVcClusterConfigSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanVcClusterConfigSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Get total raw capacity in bytes for all disks claimed in a vSAN cluster.
    /// 
    /// In the case of ESA, that includes all disks in the storage pool. In the case
    /// of OSA, that includes the capacity tier disks but not the cache disks as the
    /// latter does not contribute to capacity. The same value is reported to the
    /// licensing service for capacity entitlement purposes. Note that it differs
    /// from vSAN datastore capacity in two ways. First, it is based on raw capacity
    /// reported by the disk vendor, which includes any and all overhead reserved
    /// internally by vSAN. Secondly, it reflects the cluster configuration, which
    /// is not affected by the runtime state of the cluster, such as disk health,
    /// host health, and network partition.
    /// The claimed capacity is initialized in an async process after enabling vSAN,
    /// so it is possible that the claimed capacity value is not ready yet when
    /// vSAN is still in initialization phase. In this case, API caller can retry in
    /// a short period of time.
    /// 
    /// ***Since:*** 8.0.0.4
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vCenter cluster.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The total claimed disk capacity in raw bytes.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the capacity value has not been initialized.
    /// 
    /// ***NotSupported***: if the cluster has not enabled vSAN.
    pub async fn vsan_cluster_get_claimed_capacity(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<i64> {
        let input = VsanClusterGetClaimedCapacityRequestType {cluster, };
        let path = format!("/vsan/VsanVcClusterConfigSystem/{moId}/VsanClusterGetClaimedCapacity", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: i64 = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Get configInfoEx for a vSAN cluster  
    /// We can get vSAN configuration information through this method.
    /// 
    /// Currently users can only get generic cluster configuration and data
    /// efficiency configuration through this API.
    ///   
    /// This API cannot retrieve fault domain configuration and disk group
    /// configuration. Disk group configuration can be retrieved through
    /// *VimClusterVsanVcDiskManagementSystem.QueryDiskMappings*.
    /// Fault domain configuration
    /// can be retrieved through *HostVsanSystem.config*.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vCenter cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The configuration spec for the vCenter cluster
    /// ConfigInfoEx inherits from *VsanClusterConfigInfo*.
    /// It appends data efficiency configuration.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_cluster_get_config(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::VsanConfigInfoEx> {
        let input = VsanClusterGetConfigRequestType {cluster, };
        let path = format!("/vsan/VsanVcClusterConfigSystem/{moId}/VsanClusterGetConfig", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VsanConfigInfoEx = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Get vSAN runtime stats of all hosts reside in specified cluster.
    /// 
    /// This API is used to retrieve vSAN runtime stats from all hosts reside in
    /// specified vSAN cluster. It bases on specified stats type list, to retrieve
    /// expected vSAN runtime stats. If host is out of reach, such as disconnected
    /// from vCenter, its stats will be ignored and unset.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vCenter cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### stats
    /// List of vSAN runtime stats type. Supported vSAN runtime stats
    /// types are declared in *VsanHostStatsType_enum*.
    /// If this parameter is omitted, all supported runtime stats will
    /// be collected and returned.
    ///
    /// ## Returns:
    ///
    /// vim.vsan.RuntimeStatsHostMap\[\] List of vSAN runtime stats, each item
    /// contains owner host and its stats. If
    /// specified cluster is empty, or vSAN is
    /// disabled on it, an empty list will be
    /// returned.
    pub async fn vsan_cluster_get_runtime_stats(&self, cluster: &crate::types::structs::ManagedObjectReference, stats: Option<&[String]>) -> Result<Option<Vec<crate::types::structs::VsanRuntimeStatsHostMap>>> {
        let input = VsanClusterGetRuntimeStatsRequestType {cluster, stats, };
        let path = format!("/vsan/VsanVcClusterConfigSystem/{moId}/VsanClusterGetRuntimeStats", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VsanRuntimeStatsHostMap>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// To support integration between vSAN and DRS, we are expected to report runtime
    /// stats of vSAN member hosts and VMs locate in specified vSAN cluster from storage
    /// perspective.
    /// 
    /// Per request from DRS service, below stats are requested:
    /// - 1\. capacity contributed to vSAN datastore per host;
    /// - 2\. capacity usage to vSAN datastore per host;
    /// - 3\. total size per vSAN VM;
    /// - 4\. valid data percentage on every host per vSAN VM;
    ///   
    /// This API is used to retrieve runtime stats requested by DRS.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vSAN cluster;
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### vms
    /// Specified VM list of which runtime stats should be reported. If omitted
    /// return stats of all vSAN VMs locate in specified vSAN cluster;
    /// 
    /// Refers instances of *VirtualMachine*.
    ///
    /// ## Returns:
    ///
    /// List of *VsanHostDrsStats*, in which stats should be parsed by
    /// the order of returned DrsStats.
    pub async fn vsan_query_cluster_drs_stats(&self, cluster: &crate::types::structs::ManagedObjectReference, vms: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<Option<Vec<crate::types::structs::VsanHostDrsStats>>> {
        let input = VsanQueryClusterDrsStatsRequestType {cluster, vms, };
        let path = format!("/vsan/VsanVcClusterConfigSystem/{moId}/VsanQueryClusterDrsStats", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VsanHostDrsStats>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Reconfigure a vSAN cluster.
    /// 
    /// This method is used to set vSAN specific configuration, the method can be used
    /// to modify all vSAN related configuration. From vSAN 6.2, this
    /// API is the replacement of *ComputeResource.ReconfigureComputeResource_Task* in order
    /// to support more configurations such as deduplication and compression, fault
    /// domain, disk mapping configuration in the cluster. The legacy API
    /// *ComputeResource.ReconfigureComputeResource_Task* is deprecated because new API
    /// provide everything that legacy API has.
    /// To modify generic vSAN cluster setting such as vSAN enablement and
    /// autoclaim enablement, *VimVsanReconfigSpec.vsanClusterConfig* needs to
    /// be setup.
    /// To modify vSAN cluster specific features such as deduplication and
    /// compression enablement, *VimVsanReconfigSpec.dataEfficiencyConfig*
    /// needs to be setup. Currently for vSAN 6.2, deduplication and
    /// and compression need to be enable/disable together. To enable/disable
    /// deduplication and compression on a vSAN cluster, disk convert process
    /// (*VsanUpgradeSystemEx.PerformVsanUpgradeEx* for description
    /// about disk convert process) will be invoked. This is only supported for
    /// all-flash disk groups.
    /// To modify vSAN cluster fault domain configuration,
    /// *VimVsanReconfigSpec.faultDomainsSpec* needs to be setup.
    /// To modify vSAN cluster disk groups reconfiguration,
    /// *VimVsanReconfigSpec.diskMappingSpec* needs to be setup.
    /// To modify vSAN iSCSI target service configuration,
    /// *VimVsanReconfigSpec.iscsiSpec* needs to be setup.
    /// 
    /// Reconfiguring vSAN cluster requires Host.Inventory.EditCluster privilege on
    /// the cluster, extra privileges may be required depending on what is being
    /// changed:
    /// - Cryptographer.ManageEncryptionPolicy and Cryptographer.ManageKeys if
    ///   changing data encryption configuration.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target VC cluster.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### vsan_reconfig_spec
    /// The configure spec for vSAN cluster.
    ///
    /// ## Returns:
    ///
    /// vim.Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: when turn on data efficiency feature on a cluster
    /// which has hybrid disk group.
    /// 
    /// ***InvalidState***: if the vSAN is not enabled in current
    /// cluster.
    pub async fn vsan_cluster_reconfig(&self, cluster: &crate::types::structs::ManagedObjectReference, vsan_reconfig_spec: &crate::types::structs::VimVsanReconfigSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanClusterReconfigRequestType {cluster, vsan_reconfig_spec, };
        let path = format!("/vsan/VsanVcClusterConfigSystem/{moId}/VsanClusterReconfig", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Regenerate the key(s) used by vSAN encryption for the given cluster.
    /// 
    /// When the key(s) used for vSAN encryption is compromised, this API can
    /// be used to renew the key(s). As described in
    /// *VsanHostEncryptionInfo*, vSAN uses Key Encryption Key
    /// (KEK) to wrap the Data Encryption Key (DEK). Generally a user will only need
    /// to renew the KEK and use new KEK to rewrap the DEK, when KEK is compromised,
    /// or on regular basis. This is referred to as "shallow rekey". If user suspects
    /// that DEK is also compromised, both KEK and DEK will be renewed.
    /// Correspondingly this is called "deep rekey". And as a consequence of DEK
    /// change, the data on vSAN datastore will be re-encrypted with the new
    /// DEK, so this will be a slow process.
    ///   
    /// This API does not support switching to a different KMS cluster. When there is
    /// a need to switch to a different KMS cluster, call
    /// *VsanVcClusterConfigSystem.VsanClusterReconfig* and specify
    /// a different KMS cluster in the configuration
    /// *VsanDataEncryptionConfig.kmsProviderId*.
    /// 
    /// To run either shallow rekey or deep rekey for an encrypted vSAN cluster, both
    /// Cryptographer.ManageKeys privilege and Host.Inventory.EditCluster privilege
    /// are required on the cluster. If it is a shallow rekey, alternatively, caller
    /// with Vsan.Cluster.ShallowRekey privilege on the cluster will also be allowed.
    ///
    /// ## Parameters:
    ///
    /// ### encrypted_cluster
    /// The target VC cluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### deep_rekey
    /// True to perform a deep rekey. Its default value is false when
    /// not provided, which means shallow rekey is performed
    ///
    /// ### allow_reduced_redundancy
    /// This optional parameter is only applicable for
    /// deep rekey when it needs to migrate data across cluster for changing
    /// vSAN disk format. The default value is 'false' if not specified
    /// See *VimVsanReconfigSpec.allowReducedRedundancy*,
    /// *ensureObjectAccessibility*,
    /// and *evacuateAllData*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the vSAN is not enabled or encryption is not
    /// enabled on the cluster.
    pub async fn vsan_encrypted_cluster_rekey_task(&self, encrypted_cluster: &crate::types::structs::ManagedObjectReference, deep_rekey: Option<bool>, allow_reduced_redundancy: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanEncryptedClusterRekeyRequestType {encrypted_cluster, deep_rekey, allow_reduced_redundancy, };
        let path = format!("/vsan/VsanVcClusterConfigSystem/{moId}/VsanEncryptedClusterRekey_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Run checks for lifecycle operations on a given vSAN cluster.
    /// 
    /// The specification indicates the lifecycle operation for which checks will
    /// be performed.
    /// The result contains an overall status of the checks, a list of the checks
    /// run, and vSAN cluster configurations details related to the checks.
    /// The overall status of the checks in the result represents the highest
    /// severity status of all the results of the individual checks. For example,
    /// if all tests pass, the highest severity status is green. If a test results
    /// in yellow status and another in red status, the overall status will be red.
    /// The list of checks contains a description of the check, its result status
    /// and in case of failure, a localized error message.
    /// The vSAN cluster configuration details contains information like witness
    /// host and fault domains information for stretched clusters, or no witness
    /// host information to indicate single-site cluster.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Cluster on which to perform lifecycle checks and return
    /// configuration information.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### vsan_lifecycle_check_spec
    /// Specification for the lifecycle checks.
    ///
    /// ## Returns:
    ///
    /// Lifecycle checks and configuration results.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn run_lifecycle_check(&self, cluster: &crate::types::structs::ManagedObjectReference, vsan_lifecycle_check_spec: &crate::types::structs::VsanVcLifecycleCheckSpec) -> Result<crate::types::structs::VsanVcLifecycleCheckResult> {
        let input = RunLifecycleCheckRequestType {cluster, vsan_lifecycle_check_spec, };
        let path = format!("/vsan/VsanVcClusterConfigSystem/{moId}/RunLifecycleCheck", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VsanVcLifecycleCheckResult = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Validate the vSAN cluster reconfig spec to figure out whether user spec can
    /// be supported by target cluster.
    /// 
    /// This API cannot predict the runtime failure,
    /// but only check the spec based on software capability. If target cluster
    /// cannot support the spec, it returns the detailed config issues.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target VC cluster to apply reconfig spec
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### vsan_reconfig_spec
    /// The configure spec to be validated
    ///
    /// ## Returns:
    ///
    /// A list of different validation results.
    /// The result returned are of two types.
    /// 1\. If the spec enables vSAN ESA, then result for every precheck is returned using
    /// *VsanClusterConfigPrecheckItem*. This includes prechecks of
    /// all possible statuses as in *VsanHealthStatusType_enum*
    /// 2\. If the spec does not enable vSAN ESA, then the result for prechecks are returned
    /// using *ClusterComputeResourceValidationResultBase*. Here, only the
    /// the prechecks which fail the validation are returned. The field
    /// *ClusterComputeResourceValidationResultBase.info* contains the
    /// precheck failure reason.
    pub async fn vsan_validate_config_spec(&self, cluster: &crate::types::structs::ManagedObjectReference, vsan_reconfig_spec: &crate::types::structs::VimVsanReconfigSpec) -> Result<Option<Vec<Box<dyn crate::types::traits::ClusterComputeResourceValidationResultBaseTrait>>>> {
        let input = VsanValidateConfigSpecRequestType {cluster, vsan_reconfig_spec, };
        let path = format!("/vsan/VsanVcClusterConfigSystem/{moId}/VsanValidateConfigSpec", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::ClusterComputeResourceValidationResultBaseTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct VsanClusterGetClaimedCapacityRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanClusterGetClaimedCapacityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterGetClaimedCapacityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterGetClaimedCapacityRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterGetClaimedCapacityRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanClusterGetClaimedCapacityRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterGetClaimedCapacityRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanClusterGetConfigRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanClusterGetConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterGetConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterGetConfigRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterGetConfigRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanClusterGetConfigRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterGetConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanClusterGetRuntimeStatsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    stats: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for VsanClusterGetRuntimeStatsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterGetRuntimeStatsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterGetRuntimeStatsRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterGetRuntimeStatsRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanClusterGetRuntimeStatsRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterGetRuntimeStatsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.stats else { continue; };
                    return Some((std::borrow::Cow::Borrowed("stats"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQueryClusterDrsStatsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    vms: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for VsanQueryClusterDrsStatsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryClusterDrsStatsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryClusterDrsStatsRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryClusterDrsStatsRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanQueryClusterDrsStatsRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryClusterDrsStatsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.vms else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vms"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanClusterReconfigRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    vsan_reconfig_spec: &'a crate::types::structs::VimVsanReconfigSpec,
}

impl<'a> miniserde::Serialize for VsanClusterReconfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterReconfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterReconfigRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterReconfigRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanClusterReconfigRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterReconfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("vsanReconfigSpec"), &self.data.vsan_reconfig_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanEncryptedClusterRekeyRequestType<'a> {
    encrypted_cluster: &'a crate::types::structs::ManagedObjectReference,
    deep_rekey: Option<bool>,
    allow_reduced_redundancy: Option<bool>,
}

impl<'a> miniserde::Serialize for VsanEncryptedClusterRekeyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanEncryptedClusterRekeyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanEncryptedClusterRekeyRequestTypeSer<'b, 'a> {
    data: &'b VsanEncryptedClusterRekeyRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanEncryptedClusterRekeyRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanEncryptedClusterRekeyRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("encryptedCluster"), &self.data.encrypted_cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.deep_rekey else { continue; };
                    return Some((std::borrow::Cow::Borrowed("deepRekey"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.allow_reduced_redundancy else { continue; };
                    return Some((std::borrow::Cow::Borrowed("allowReducedRedundancy"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RunLifecycleCheckRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    vsan_lifecycle_check_spec: &'a crate::types::structs::VsanVcLifecycleCheckSpec,
}

impl<'a> miniserde::Serialize for RunLifecycleCheckRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RunLifecycleCheckRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RunLifecycleCheckRequestTypeSer<'b, 'a> {
    data: &'b RunLifecycleCheckRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RunLifecycleCheckRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RunLifecycleCheckRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("vsanLifecycleCheckSpec"), &self.data.vsan_lifecycle_check_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanValidateConfigSpecRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    vsan_reconfig_spec: &'a crate::types::structs::VimVsanReconfigSpec,
}

impl<'a> miniserde::Serialize for VsanValidateConfigSpecRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanValidateConfigSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanValidateConfigSpecRequestTypeSer<'b, 'a> {
    data: &'b VsanValidateConfigSpecRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanValidateConfigSpecRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanValidateConfigSpecRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("vsanReconfigSpec"), &self.data.vsan_reconfig_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
