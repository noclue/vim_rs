use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The vSAN Cluster Health System exposes the vSAN cluster health service API
/// and serves as an aggregator to collect the result from each of hosts in the vSAN
/// cluster.
#[derive(Clone)]
pub struct VsanClusterHealthSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanClusterHealthSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let bytes = self.client.invoke("vsan", "VsanClusterHealthSystem", &self.mo_id, "VsanCheckClusterClomdLiveness", Some(&input)).await?;
        let result: crate::types::structs::VsanClusterClomdLivenessResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "VsanClusterHealthSystem", &self.mo_id, "VsanClusterGetHclInfo", Some(&input)).await?;
        let result: crate::types::structs::VsanClusterHclInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes_opt = self.client.invoke_optional("vsan", "VsanClusterHealthSystem", &self.mo_id, "VsanQueryClusterAdvCfgSync", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        let bytes = self.client.invoke("vsan", "VsanClusterHealthSystem", &self.mo_id, "VsanQueryClusterCaptureVsanPcap", Some(&input)).await?;
        let result: crate::types::structs::VsanVsanClusterPcapResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "VsanClusterHealthSystem", &self.mo_id, "VsanQueryClusterCheckLimits", Some(&input)).await?;
        let result: crate::types::structs::VsanClusterLimitHealthResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "VsanClusterHealthSystem", &self.mo_id, "VsanQueryClusterCreateVmHealthTest", Some(&input)).await?;
        let result: crate::types::structs::VsanClusterCreateVmHealthTestResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "VsanClusterHealthSystem", &self.mo_id, "VsanQueryClusterHealthSystemVersions", Some(&input)).await?;
        let result: crate::types::structs::VsanClusterHealthSystemVersionResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "VsanClusterHealthSystem", &self.mo_id, "VsanQueryClusterNetworkPerfTest", Some(&input)).await?;
        let result: crate::types::structs::VsanClusterNetworkLoadTestResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes_opt = self.client.invoke_optional("vsan", "VsanClusterHealthSystem", &self.mo_id, "VsanQueryClusterPhysicalDiskHealthSummary", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        let bytes = self.client.invoke("vsan", "VsanClusterHealthSystem", &self.mo_id, "VsanQueryVerifyClusterNetworkSettings", Some(&input)).await?;
        let result: crate::types::structs::VsanClusterNetworkHealthResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "VsanClusterHealthSystem", &self.mo_id, "VsanRepairClusterImmediateObjects", Some(&input)).await?;
        let result: crate::types::structs::VsanClusterHealthSystemObjectsRepairResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct VsanCheckClusterClomdLivenessRequestType<'a> {
    hosts: &'a [String],
    esx_root_password: &'a str,
}

impl<'a> miniserde::Serialize for VsanCheckClusterClomdLivenessRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanCheckClusterClomdLivenessRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanCheckClusterClomdLivenessRequestTypeSer<'b, 'a> {
    data: &'b VsanCheckClusterClomdLivenessRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanCheckClusterClomdLivenessRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanCheckClusterClomdLivenessRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("esxRootPassword"), &self.data.esx_root_password as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanClusterGetHclInfoRequestType<'a> {
    hosts: &'a [String],
    esx_root_password: &'a str,
}

impl<'a> miniserde::Serialize for VsanClusterGetHclInfoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterGetHclInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterGetHclInfoRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterGetHclInfoRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanClusterGetHclInfoRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterGetHclInfoRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("esxRootPassword"), &self.data.esx_root_password as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryClusterAdvCfgSyncRequestType<'a> {
    hosts: &'a [String],
    esx_root_password: &'a str,
    options: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for VsanQueryClusterAdvCfgSyncRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryClusterAdvCfgSyncRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryClusterAdvCfgSyncRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryClusterAdvCfgSyncRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryClusterAdvCfgSyncRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryClusterAdvCfgSyncRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("esxRootPassword"), &self.data.esx_root_password as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.options else { continue; };
                    return Some((std::borrow::Cow::Borrowed("options"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQueryClusterCaptureVsanPcapRequestType<'a> {
    hosts: &'a [String],
    esx_root_password: &'a str,
    duration: i32,
    vmknic: Option<&'a [crate::types::structs::VsanClusterHostVmknicMapping]>,
    include_raw_pcap: Option<bool>,
    include_igmp: Option<bool>,
    cmmds_msg_type_filter: Option<&'a [String]>,
    cmmds_ports: Option<&'a [i32]>,
    cluster_uuid: Option<&'a str>,
}

impl<'a> miniserde::Serialize for VsanQueryClusterCaptureVsanPcapRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryClusterCaptureVsanPcapRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryClusterCaptureVsanPcapRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryClusterCaptureVsanPcapRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryClusterCaptureVsanPcapRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryClusterCaptureVsanPcapRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("esxRootPassword"), &self.data.esx_root_password as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("duration"), &self.data.duration as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.vmknic else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vmknic"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.include_raw_pcap else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeRawPcap"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.include_igmp else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeIgmp"), val as &dyn miniserde::Serialize));
                }
                7 => {
                    let Some(ref val) = self.data.cmmds_msg_type_filter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cmmdsMsgTypeFilter"), val as &dyn miniserde::Serialize));
                }
                8 => {
                    let Some(ref val) = self.data.cmmds_ports else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cmmdsPorts"), val as &dyn miniserde::Serialize));
                }
                9 => {
                    let Some(ref val) = self.data.cluster_uuid else { continue; };
                    return Some((std::borrow::Cow::Borrowed("clusterUuid"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQueryClusterCheckLimitsRequestType<'a> {
    hosts: &'a [String],
    esx_root_password: &'a str,
}

impl<'a> miniserde::Serialize for VsanQueryClusterCheckLimitsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryClusterCheckLimitsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryClusterCheckLimitsRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryClusterCheckLimitsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryClusterCheckLimitsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryClusterCheckLimitsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("esxRootPassword"), &self.data.esx_root_password as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryClusterCreateVmHealthTestRequestType<'a> {
    hosts: &'a [String],
    esx_root_password: &'a str,
    timeout: i32,
}

impl<'a> miniserde::Serialize for VsanQueryClusterCreateVmHealthTestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryClusterCreateVmHealthTestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryClusterCreateVmHealthTestRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryClusterCreateVmHealthTestRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryClusterCreateVmHealthTestRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryClusterCreateVmHealthTestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("esxRootPassword"), &self.data.esx_root_password as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("timeout"), &self.data.timeout as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryClusterHealthSystemVersionsRequestType<'a> {
    hosts: &'a [String],
    esx_root_password: &'a str,
}

impl<'a> miniserde::Serialize for VsanQueryClusterHealthSystemVersionsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryClusterHealthSystemVersionsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryClusterHealthSystemVersionsRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryClusterHealthSystemVersionsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryClusterHealthSystemVersionsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryClusterHealthSystemVersionsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("esxRootPassword"), &self.data.esx_root_password as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryClusterNetworkPerfTestRequestType<'a> {
    hosts: &'a [String],
    esx_root_password: &'a str,
    multicast: bool,
    duration_sec: Option<i32>,
}

impl<'a> miniserde::Serialize for VsanQueryClusterNetworkPerfTestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryClusterNetworkPerfTestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryClusterNetworkPerfTestRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryClusterNetworkPerfTestRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryClusterNetworkPerfTestRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryClusterNetworkPerfTestRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("esxRootPassword"), &self.data.esx_root_password as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("multicast"), &self.data.multicast as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.duration_sec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("durationSec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQueryClusterPhysicalDiskHealthSummaryRequestType<'a> {
    hosts: &'a [String],
    esx_root_password: &'a str,
}

impl<'a> miniserde::Serialize for VsanQueryClusterPhysicalDiskHealthSummaryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryClusterPhysicalDiskHealthSummaryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryClusterPhysicalDiskHealthSummaryRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryClusterPhysicalDiskHealthSummaryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryClusterPhysicalDiskHealthSummaryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryClusterPhysicalDiskHealthSummaryRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("esxRootPassword"), &self.data.esx_root_password as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryVerifyClusterNetworkSettingsRequestType<'a> {
    hosts: &'a [String],
    esx_root_password: &'a str,
}

impl<'a> miniserde::Serialize for VsanQueryVerifyClusterNetworkSettingsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryVerifyClusterNetworkSettingsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryVerifyClusterNetworkSettingsRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryVerifyClusterNetworkSettingsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryVerifyClusterNetworkSettingsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryVerifyClusterNetworkSettingsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("esxRootPassword"), &self.data.esx_root_password as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanRepairClusterImmediateObjectsRequestType<'a> {
    hosts: &'a [String],
    esx_root_password: &'a str,
    uuids: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for VsanRepairClusterImmediateObjectsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanRepairClusterImmediateObjectsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanRepairClusterImmediateObjectsRequestTypeSer<'b, 'a> {
    data: &'b VsanRepairClusterImmediateObjectsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanRepairClusterImmediateObjectsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanRepairClusterImmediateObjectsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("esxRootPassword"), &self.data.esx_root_password as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.uuids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("uuids"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
