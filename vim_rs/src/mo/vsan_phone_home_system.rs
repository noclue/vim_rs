use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// VsanPhoneHomeSystem contains a collection of APIs to perform online health
/// checks.
/// 
/// The Managed Entity can be accessed through MOID of
/// vsan-phonehome-system through vSAN service at vCenter server side.
#[derive(Clone)]
pub struct VsanPhoneHomeSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanPhoneHomeSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// QueryVsanCloudHealthStatus returns a VsanCloudHealthStatus object
    /// with various states of the vSAN CEIP Collector Agent.
    /// 
    /// When checking for internet connectivity, a connection to
    /// https://vcsa.vmware.com/ will be attempted with timeout of 10 seconds.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Returns:
    ///
    /// vim.vsan.VsanCloudHealthStatus
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn query_vsan_cloud_health_status(&self) -> Result<Option<crate::types::structs::VsanCloudHealthStatus>> {
        let path = format!("/vsan/VsanPhoneHomeSystem/{moId}/QueryVsanCloudHealthStatus", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::VsanCloudHealthStatus>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// An asynchronous API for performing vSAN online health checks.
    /// 
    /// This API generates a task to collect current vSAN environment settings and
    /// performance data driven by a manifest, and send the collected data to VMware
    /// for data streaming analysis. It then queries the analyzed results back to
    /// help customer have the best vSAN practices.
    /// This task usually takes about 1 minute for a typical 4 nodes vSAN cluster.
    /// However, this task can take more time based on the size of vSAN cluster, the
    /// size of queried data driven by manifest and the customer Internet speed.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vSAN cluster
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// task vCenter Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_perform_online_health_check(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanPerformOnlineHealthCheckRequestType {cluster, };
        let path = format!("/vsan/VsanPhoneHomeSystem/{moId}/VsanPerformOnlineHealthCheck", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// It is a VC level API that iterates over all hosts
    /// in the clusters to get LSOM wbSize through the
    /// host level API.
    /// 
    /// Only supported in vSAN OSA
    /// Example:
    /// {
    /// host-29 :{
    /// "5257a570-922c-9a38-e79a-3ab2871ab0b8": {
    /// "wbSize": 16105168896 },
    /// "521530b5-91cf-0dfe-d469-9cdfc9146b64": {
    /// "wbSize": 0 },
    /// "521446ca-e8bb-e369-912c-abb02b97001f": {
    /// "wbSize": 0 },
    /// },
    /// host-30:{
    /// "5257a570-922c-9a38-e79a-3ab2871ab0b8": {
    /// "wbSize": 16105168896 },
    /// "521530b5-91cf-0dfe-d469-9cdfc9146b64": {
    /// "wbSize": 0 },
    /// "521446ca-e8bb-e369-912c-abb02b97001f": {
    /// "wbSize": 0 }
    /// }
    /// }
    /// 
    /// ***Since:*** 8.0.0.4
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_lso_mwbsize(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<String>> {
        let input = VsanQueryLsoMwbsizeRequestType {cluster, };
        let path = format!("/vsan/VsanPhoneHomeSystem/{moId}/VsanQueryLSOMwbsize", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<String>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// API to Enhance DDH health state monitoring for
    /// VSAN NVMe disks to check the NVMe critical warnings.
    /// 
    /// This API iterates over each host to call the Host
    /// Level API to collect the data
    /// Note: This data is collected for only VSAN NVMe devices
    /// 
    /// ***Since:*** 8.0.0.4
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Target cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// string of json format data, for example:
    /// { "host-29":{
    /// "eui.3c22afd568ada7e3000c29675e8e0701": {
    /// "spare\_space": 0,
    /// "temp\_exceeded\_threshold": 0,
    /// "subsystem\_reliability\_degraded": 0,
    /// "read\_only": 0,
    /// "backup\_failed": 0
    /// },
    /// "eui.1c29f18de63cdc84000c2965dd62c898": {
    /// "spare\_space": 0,
    /// "temp\_exceeded\_threshold": 0,
    /// "subsystem\_reliability\_degraded": 0,
    /// "read\_only": 0,
    /// "backup\_failed": 0
    /// }
    /// }
    /// "host-30":{
    /// "eui.3c22afd568ada7e3000c29675e8e0701": {
    /// "spare\_space": 0,
    /// "temp\_exceeded\_threshold": 0,
    /// "subsystem\_reliability\_degraded": 0,
    /// "read\_only": 0,
    /// "backup\_failed": 0
    /// },
    /// "eui.1c29f18de63cdc84000c2965dd62c898": {
    /// "spare\_space": 0,
    /// "temp\_exceeded\_threshold": 0,
    /// "subsystem\_reliability\_degraded": 0,
    /// "read\_only": 0,
    /// "backup\_failed": 0
    /// }
    /// }
    /// }
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_nvme_critical_warning_stats(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<String>> {
        let input = VsanQueryNvmeCriticalWarningStatsRequestType {cluster, };
        let path = format!("/vsan/VsanPhoneHomeSystem/{moId}/VsanQueryNvmeCriticalWarningStats", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<String>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// API to get vSAN object snapshot information in a vSAN cluster
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Target cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// string of json format data, for example:
    /// {"summary" : {
    /// "TotalObjects" : ,
    /// "TotalSnapshots" :
    /// },
    /// "detail" : {
    /// "snapshot\_count\_0" : count,
    /// "snapshot\_count\_1" : count,
    /// "snapshot\_count\_2" : count
    /// }
    /// }
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the given argument is not passed correctly.
    /// 
    /// ***InvalidType***: Exception for invalid type arguments, for example,
    /// if the given argument is not type of cluster moId.
    /// 
    /// ***ManagedObjectNotFound***: Exception for invalid reference argument,
    /// for example, if the given cluster is not
    /// found.
    pub async fn vsan_query_object_snapshots_info(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<String>> {
        let input = VsanQueryObjectSnapshotsInfoRequestType {cluster, };
        let path = format!("/vsan/VsanPhoneHomeSystem/{moId}/VsanQueryObjectSnapshotsInfo", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<String>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// VC API to iterate over all hots to collect zDOMscrubber stats
    /// from the path "/vmkModules/vsan/zdom/zdomObjects/&lt;ObjectUuid&gt;/zdomPaused"
    /// Note: This data is collected for hosts which are part of VSAN ESA cluster only
    /// 
    /// ***Since:*** 8.0.0.4
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Target cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// string of json format data, for example:
    /// {
    /// "&lt;host1\_name&gt;" : \[{'Object\_uuid': &lt;value&gt;,
    /// 'state': &lt;value&gt;}, {'Object\_uuid': &lt;value&gt;,
    /// 'state': &lt;value&gt;}\],
    /// "&lt;host2\_name&gt;" : \[{'Object\_uuid': &lt;value&gt;,
    /// 'state': &lt;value&gt;}\]
    /// }
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_zdom_scrubber_data(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<String>> {
        let input = VsanQueryZdomScrubberDataRequestType {cluster, };
        let path = format!("/vsan/VsanPhoneHomeSystem/{moId}/VsanQueryZdomScrubberData", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<String>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct VsanPerformOnlineHealthCheckRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanPerformOnlineHealthCheckRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanPerformOnlineHealthCheckRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanPerformOnlineHealthCheckRequestTypeSer<'b, 'a> {
    data: &'b VsanPerformOnlineHealthCheckRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanPerformOnlineHealthCheckRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanPerformOnlineHealthCheckRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryLsoMwbsizeRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanQueryLsoMwbsizeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryLsoMwbsizeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryLsoMwbsizeRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryLsoMwbsizeRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanQueryLsoMwbsizeRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryLSOMwbsizeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryNvmeCriticalWarningStatsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanQueryNvmeCriticalWarningStatsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryNvmeCriticalWarningStatsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryNvmeCriticalWarningStatsRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryNvmeCriticalWarningStatsRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanQueryNvmeCriticalWarningStatsRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryNvmeCriticalWarningStatsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryObjectSnapshotsInfoRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanQueryObjectSnapshotsInfoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryObjectSnapshotsInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryObjectSnapshotsInfoRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryObjectSnapshotsInfoRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanQueryObjectSnapshotsInfoRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryObjectSnapshotsInfoRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryZdomScrubberDataRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanQueryZdomScrubberDataRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryZdomScrubberDataRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryZdomScrubberDataRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryZdomScrubberDataRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanQueryZdomScrubberDataRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryZdomScrubberDataRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
