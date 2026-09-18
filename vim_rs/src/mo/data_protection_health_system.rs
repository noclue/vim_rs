use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object provides access to Data Protection (DP) Health related
/// configuration and query APIs, operating at the vCenter or cluster level.
/// 
/// It can be accessed via the MOID of 'dp-health-system' through vSAN service
/// at vCenter server side.
#[derive(Clone)]
pub struct DataProtectionHealthSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl DataProtectionHealthSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Get the user configured silent data protection health check list of the cluster.
    /// 
    /// This API is only supported on the cluster with data protection enabled.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster which has data protection enabled
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The list of all silent health checks testIds
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: If run directly on an ESX Server host.
    /// 
    /// ***NotFound***: If the cluster is not found.
    /// 
    /// ***VsanFault***: If any runtime error is hit.
    pub async fn vsan_get_dp_cluster_silent_checks(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<String>>> {
        let input = VsanGetDpClusterSilentChecksRequestType {cluster, };
        let bytes_opt = self.client.invoke_optional("vsan", "DataProtectionHealthSystem", &self.mo_id, "VsanGetDpClusterSilentChecks", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Query Data Protection (DP) health data and generate health checks.
    /// 
    /// It's the primary API for fetching latest DP health status.
    /// This API doesn't support host level call and will throw exception in that case.
    /// The DP health summary is pushed from the DP appliance and stored in memory by the vSAN health service.
    /// If the DP appliance is offline or experiences a loss of connectivity,
    /// the vSAN health service will be unable to provide the most current DP health data.
    /// However, if the vSAN health service is restarted, it will first load the
    /// most recent available DP health status from the historical health database.
    /// It will then update the health status once new data is received from the DP appliance,
    /// which typically occurs after a few minutes.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// Returns a health summary data object, containing the current DP health status.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If any runtime error is hit.
    /// 
    /// ***NotSupported***: If the API is called at the host level.
    pub async fn vsan_query_health_summary(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::VsanClusterHealthSummary> {
        let input = VsanQueryHealthSummaryRequestType {cluster, };
        let bytes = self.client.invoke("vsan", "DataProtectionHealthSystem", &self.mo_id, "VsanQueryHealthSummary", Some(&input)).await?;
        let result: crate::types::structs::VsanClusterHealthSummary = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query Data Protection (DP) historical health information based on the query spec.
    /// - Query general DP historical health summary.
    ///   If no groupId and testId specified in the query spec,
    ///   it aggregates all the historical DP health status
    ///   for the given time range. Then returns a general DP health summary
    ///   with only overallHealth, test group IDs, group healths,
    ///   test IDs and test healths.
    /// - Query DP health status history for a certain DP health check.
    ///   If groupId and testId are specified, and start time doesn't
    ///   equal to end time, the result will only contain the historical
    ///   testHealths for the given time range of the target health check.
    /// - Query a snapshot detail for a certain DP health check.
    ///   If groupId and testId are specified, and start time equals to
    ///   end time, the result will contain a full testDetail for the certain
    ///   timestamp of the target DP health check.
    /// - Query overall DP health trend during the given time range.
    ///   If both groupId and testId are specified to "#" in the query spec,
    ///   the result will contain a series of DP health summaries with
    ///   overallHealth, healthStatusCounts and timestamp within the given time range.
    ///   
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The query spec.
    ///
    /// ## Returns:
    ///
    /// DP health summary with historical information.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments.
    /// For example, if the given time range is more than one month,
    /// or in case the end time is before the start time.
    pub async fn vsan_query_historical_health(&self, spec: &crate::types::structs::VsanHistoricalHealthQuerySpec) -> Result<Option<Vec<crate::types::structs::VsanClusterHealthSummary>>> {
        let input = VsanQueryHistoricalHealthRequestType {spec, };
        let bytes_opt = self.client.invoke_optional("vsan", "DataProtectionHealthSystem", &self.mo_id, "VsanQueryHistoricalHealth", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Set silent health check list of the cluster.
    /// 
    /// Specify \[testId/groupId\] for 'addSilentChecks' to add add health
    /// checks to silent list.
    /// Specify \[testId/groupId\] for 'removeSilentChecks' to remove health
    /// checks from silent list.
    /// To restore the whole silent list, set 'removeSilentChecks' = \['all'\].
    /// After the silent health check list is updated, it is recommended to
    /// refresh the vSAN cluster data protection health summary to get the updated result.
    /// The health check items in the silent list should be skipped, and labeled
    /// with 'skipped' status.
    /// Note:
    /// - If groupId is added to silent list, all the checks under this
    ///   group would be silent.
    /// - If groupId is removed from silent list, all the checks under
    ///   this group would back to normal.
    /// - If given invalid testId/groupId, it will throw exception.
    /// - This method will do add first and then remove. So if the given
    ///   health check is in both add and remove list, it would NOT be
    ///   silent in the end.
    /// - This API is only supported for data protection health checks.
    ///   
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target data protection protected cluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### add_silent_checks
    /// The health checks/groups to silent.
    ///
    /// ### remove_silent_checks
    /// The health checks/groups to restore.
    ///
    /// ## Returns:
    ///
    /// True to indicate the value being set successfully.
    /// No return otherwise.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: If run directly on an ESX Server host.
    /// 
    /// ***NotFound***: If the cluster is not found.
    /// 
    /// ***VsanFault***: If any runtime error is hit.
    pub async fn vsan_set_dp_cluster_silent_checks(&self, cluster: &crate::types::structs::ManagedObjectReference, add_silent_checks: Option<&[String]>, remove_silent_checks: Option<&[String]>) -> Result<bool> {
        let input = VsanSetDpClusterSilentChecksRequestType {cluster, add_silent_checks, remove_silent_checks, };
        let bytes = self.client.invoke("vsan", "DataProtectionHealthSystem", &self.mo_id, "VsanSetDpClusterSilentChecks", Some(&input)).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct VsanGetDpClusterSilentChecksRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanGetDpClusterSilentChecksRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanGetDpClusterSilentChecksRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanGetDpClusterSilentChecksRequestTypeSer<'b, 'a> {
    data: &'b VsanGetDpClusterSilentChecksRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanGetDpClusterSilentChecksRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanGetDpClusterSilentChecksRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryHealthSummaryRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanQueryHealthSummaryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryHealthSummaryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryHealthSummaryRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryHealthSummaryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryHealthSummaryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryHealthSummaryRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryHistoricalHealthRequestType<'a> {
    spec: &'a crate::types::structs::VsanHistoricalHealthQuerySpec,
}

impl<'a> miniserde::Serialize for VsanQueryHistoricalHealthRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryHistoricalHealthRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryHistoricalHealthRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryHistoricalHealthRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryHistoricalHealthRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryHistoricalHealthRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanSetDpClusterSilentChecksRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    add_silent_checks: Option<&'a [String]>,
    remove_silent_checks: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for VsanSetDpClusterSilentChecksRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanSetDpClusterSilentChecksRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanSetDpClusterSilentChecksRequestTypeSer<'b, 'a> {
    data: &'b VsanSetDpClusterSilentChecksRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanSetDpClusterSilentChecksRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanSetDpClusterSilentChecksRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.add_silent_checks else { continue; };
                    return Some((std::borrow::Cow::Borrowed("addSilentChecks"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.remove_silent_checks else { continue; };
                    return Some((std::borrow::Cow::Borrowed("removeSilentChecks"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
