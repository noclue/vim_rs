use std::sync::Arc;
use crate::core::client::{Client, Result};
/// The vSAN Cluster Health System exposes the vSAN cluster health service API
/// and serves as an aggregator to collect the result from each of hosts in the vSAN
/// cluster.
#[derive(Clone)]
pub struct VsanClusterHealthSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl VsanClusterHealthSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Queries the CLOM daemon liveness on all of vSAN hosts
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The vSAN hosts
    ///
    /// ### esx_root_password
    /// The root password for the host. The password should
    /// be the same for all of hosts in the vSAN cluster
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster clomd health result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_check_cluster_clomd_liveness(&self, hosts: &[String], esx_root_password: &str) -> Result<crate::types::structs::VsanClusterClomdLivenessResult> {
        let input = VsanCheckClusterClomdLivenessRequestType {hosts, esx_root_password, };
        let path = format!("/vsan/VsanClusterHealthSystem/{moId}/VsanCheckClusterClomdLiveness", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Fetch HCL information about all devices in use by vSAN on all hosts.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The vSAN hosts
    ///
    /// ### esx_root_password
    /// The root password for the host. The password should
    /// be the same for all of hosts in the vSAN cluster
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster HCL information
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_cluster_get_hcl_info(&self, hosts: &[String], esx_root_password: &str) -> Result<crate::types::structs::VsanClusterHclInfo> {
        let input = VsanClusterGetHclInfoRequestType {hosts, esx_root_password, };
        let path = format!("/vsan/VsanClusterHealthSystem/{moId}/VsanClusterGetHclInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Queries vSAN advanced configuration and checks if all of the hosts in a
    /// vSAN cluster have consistent advanced configuration options
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The vSAN hosts
    ///
    /// ### esx_root_password
    /// The root password for the host. The password should
    /// be the same for all of hosts in the vSAN cluster
    ///
    /// ### options
    /// list of vsi path for the configuration name
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster advanced configuration consistent result.
    /// If empty, indicates all in sync.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_cluster_adv_cfg_sync(&self, hosts: &[String], esx_root_password: &str, options: Option<&[String]>) -> Result<Option<Vec<crate::types::structs::VsanClusterAdvCfgSyncResult>>> {
        let input = VsanQueryClusterAdvCfgSyncRequestType {hosts, esx_root_password, options, };
        let path = format!("/vsan/VsanClusterHealthSystem/{moId}/VsanQueryClusterAdvCfgSync", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Captures the vSAN cluster multicast network trace to ensure that all
    /// of hosts in the cluster can receive multicast packets
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The vSAN hosts
    ///
    /// ### esx_root_password
    /// The root password for the host. The password should
    /// be the same for all of hosts in the vSAN cluster
    ///
    /// ### duration
    /// Duration to watch for packets in second. 1 minute is recommended
    ///
    /// ### vmknic
    /// The map for host and the vmknic using for vSAN
    ///
    /// ### include_raw_pcap
    /// True to include the raw pcap data in the result. Default is False
    ///
    /// ### include_igmp
    /// True to include the IGMP network test data. Default is True
    ///
    /// ### cmmds_msg_type_filter
    /// The filter for vSAN message type. The network message
    /// whose type is not in the filter list will be ignored. All of
    /// the supported message types include "MASTER\_HEARTBEAT",
    /// "MASTER\_UPDATE" and "AGENT\_HEARTBEAT".
    ///
    /// ### cmmds_ports
    /// The vSAN multicast ports. Unset will use the default vSAN
    /// multicast ports.
    ///
    /// ### cluster_uuid
    /// The vSAN cluster UUID.
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster multicast trace result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_cluster_capture_vsan_pcap(&self, hosts: &[String], esx_root_password: &str, duration: i32, vmknic: Option<&[crate::types::structs::VsanClusterHostVmknicMapping]>, include_raw_pcap: Option<bool>, include_igmp: Option<bool>, cmmds_msg_type_filter: Option<&[String]>, cmmds_ports: Option<&[i32]>, cluster_uuid: Option<&str>) -> Result<crate::types::structs::VsanVsanClusterPcapResult> {
        let input = VsanQueryClusterCaptureVsanPcapRequestType {hosts, esx_root_password, duration, vmknic, include_raw_pcap, include_igmp, cmmds_msg_type_filter, cmmds_ports, cluster_uuid, };
        let path = format!("/vsan/VsanClusterHealthSystem/{moId}/VsanQueryClusterCaptureVsanPcap", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Queries the vSAN cluster component limits, disk space and read cache
    /// reservation assuming one host failure
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The vSAN hosts
    ///
    /// ### esx_root_password
    /// The root password for the host. The password should
    /// be the same for all of hosts in the vSAN cluster
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster limit health result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_cluster_check_limits(&self, hosts: &[String], esx_root_password: &str) -> Result<crate::types::structs::VsanClusterLimitHealthResult> {
        let input = VsanQueryClusterCheckLimitsRequestType {hosts, esx_root_password, };
        let path = format!("/vsan/VsanClusterHealthSystem/{moId}/VsanQueryClusterCheckLimits", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Executes VM creation test and checks if a tiny VM can be created on each of
    /// host of the vSAN cluster
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The vSAN hosts
    ///
    /// ### esx_root_password
    /// The root password for the host. The password should
    /// be the same for all of hosts in the vSAN cluster
    ///
    /// ### timeout
    /// The timeout time for the VM creation test
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster VM creation test result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_cluster_create_vm_health_test(&self, hosts: &[String], esx_root_password: &str, timeout: i32) -> Result<crate::types::structs::VsanClusterCreateVmHealthTestResult> {
        let input = VsanQueryClusterCreateVmHealthTestRequestType {hosts, esx_root_password, timeout, };
        let path = format!("/vsan/VsanClusterHealthSystem/{moId}/VsanQueryClusterCreateVmHealthTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Queries vSAN health service version number string for each host and
    /// check if all of them are consistent.
    /// 
    /// Mark issueFound flag as True if they
    /// are not consistent.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The vSAN hosts
    ///
    /// ### esx_root_password
    /// The root password for the host. The password should
    /// be the same for all of hosts in the vSAN cluster
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster health service version result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_cluster_health_system_versions(&self, hosts: &[String], esx_root_password: &str) -> Result<crate::types::structs::VsanClusterHealthSystemVersionResult> {
        let input = VsanQueryClusterHealthSystemVersionsRequestType {hosts, esx_root_password, };
        let path = format!("/vsan/VsanClusterHealthSystem/{moId}/VsanQueryClusterHealthSystemVersions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Queries the vSAN network performance and checks if it meets the bandwidth requirements.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The vSAN hosts
    ///
    /// ### esx_root_password
    /// The root password for the host. The password should
    /// be the same for all of hosts in the vSAN cluster
    ///
    /// ### multicast
    /// True to test vSAN multicast network performance.
    /// False to test vSAN unicast network performance.
    ///
    /// ### duration_sec
    /// The duration time for the Network Performance test. Default is 15
    /// seconds if not set.
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster network performance test result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_cluster_network_perf_test(&self, hosts: &[String], esx_root_password: &str, multicast: bool, duration_sec: Option<i32>) -> Result<crate::types::structs::VsanClusterNetworkLoadTestResult> {
        let input = VsanQueryClusterNetworkPerfTestRequestType {hosts, esx_root_password, multicast, duration_sec, };
        let path = format!("/vsan/VsanClusterHealthSystem/{moId}/VsanQueryClusterNetworkPerfTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Queries the vSAN physical disks health on all of vSAN hosts
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The vSAN hosts
    ///
    /// ### esx_root_password
    /// The root password for the host. The password should
    /// be the same for all of hosts in the vSAN cluster
    ///
    /// ## Returns:
    ///
    /// The vSAN physical disks health result.
    /// If empty, it indicates there is no connected host in the cluster.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_cluster_physical_disk_health_summary(&self, hosts: &[String], esx_root_password: &str) -> Result<Option<Vec<crate::types::structs::VsanPhysicalDiskHealthSummary>>> {
        let input = VsanQueryClusterPhysicalDiskHealthSummaryRequestType {hosts, esx_root_password, };
        let path = format!("/vsan/VsanClusterHealthSystem/{moId}/VsanQueryClusterPhysicalDiskHealthSummary", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Queries the vSAN cluster network setting and connectivity health status
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The vSAN hosts
    ///
    /// ### esx_root_password
    /// The root password for the host. The password should
    /// be the same for all of hosts in the vSAN cluster
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster network health result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_verify_cluster_network_settings(&self, hosts: &[String], esx_root_password: &str) -> Result<crate::types::structs::VsanClusterNetworkHealthResult> {
        let input = VsanQueryVerifyClusterNetworkSettingsRequestType {hosts, esx_root_password, };
        let path = format!("/vsan/VsanClusterHealthSystem/{moId}/VsanQueryVerifyClusterNetworkSettings", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Fix absent and degraded components in object immediately.
    /// 
    /// The result will contain which object have been in queue for
    /// repair or failed to be repaired
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The vSAN hosts
    ///
    /// ### esx_root_password
    /// The root password for the host. The password should
    /// be the same for all of hosts in the vSAN cluster
    ///
    /// ### uuids
    /// The vSAN objects UUID to be repaired. Unset to repair
    /// all of reduced objects in the vSAN cluster
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster object repair result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_repair_cluster_immediate_objects(&self, hosts: &[String], esx_root_password: &str, uuids: Option<&[String]>) -> Result<crate::types::structs::VsanClusterHealthSystemObjectsRepairResult> {
        let input = VsanRepairClusterImmediateObjectsRequestType {hosts, esx_root_password, uuids, };
        let path = format!("/vsan/VsanClusterHealthSystem/{moId}/VsanRepairClusterImmediateObjects", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanCheckClusterClomdLivenessRequestType<'a> {
    hosts: &'a [String],
    #[serde(rename = "esxRootPassword")]
    esx_root_password: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanClusterGetHclInfoRequestType<'a> {
    hosts: &'a [String],
    #[serde(rename = "esxRootPassword")]
    esx_root_password: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryClusterAdvCfgSyncRequestType<'a> {
    hosts: &'a [String],
    #[serde(rename = "esxRootPassword")]
    esx_root_password: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    options: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryClusterCaptureVsanPcapRequestType<'a> {
    hosts: &'a [String],
    #[serde(rename = "esxRootPassword")]
    esx_root_password: &'a str,
    duration: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    vmknic: Option<&'a [crate::types::structs::VsanClusterHostVmknicMapping]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeRawPcap")]
    include_raw_pcap: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeIgmp")]
    include_igmp: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cmmdsMsgTypeFilter")]
    cmmds_msg_type_filter: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cmmdsPorts")]
    cmmds_ports: Option<&'a [i32]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clusterUuid")]
    cluster_uuid: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryClusterCheckLimitsRequestType<'a> {
    hosts: &'a [String],
    #[serde(rename = "esxRootPassword")]
    esx_root_password: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryClusterCreateVmHealthTestRequestType<'a> {
    hosts: &'a [String],
    #[serde(rename = "esxRootPassword")]
    esx_root_password: &'a str,
    timeout: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryClusterHealthSystemVersionsRequestType<'a> {
    hosts: &'a [String],
    #[serde(rename = "esxRootPassword")]
    esx_root_password: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryClusterNetworkPerfTestRequestType<'a> {
    hosts: &'a [String],
    #[serde(rename = "esxRootPassword")]
    esx_root_password: &'a str,
    multicast: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "durationSec")]
    duration_sec: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryClusterPhysicalDiskHealthSummaryRequestType<'a> {
    hosts: &'a [String],
    #[serde(rename = "esxRootPassword")]
    esx_root_password: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryVerifyClusterNetworkSettingsRequestType<'a> {
    hosts: &'a [String],
    #[serde(rename = "esxRootPassword")]
    esx_root_password: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanRepairClusterImmediateObjectsRequestType<'a> {
    hosts: &'a [String],
    #[serde(rename = "esxRootPassword")]
    esx_root_password: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    uuids: Option<&'a [String]>,
}
