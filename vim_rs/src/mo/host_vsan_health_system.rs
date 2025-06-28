use std::sync::Arc;
use crate::core::client::{Client, Result};
/// This managed object consolidates vSAN Health APIs that operate on a single
/// ESXi host, i.e., the APIs in this Managed Object do not correlate health
/// among multiple nodes in a vSAN cluster.
/// 
/// Typically this level is not very
/// useful for direct user consumption, and the cluster level APIs should be used
/// instead. The cluster level APIs build upon the APIs in this Managed Object.
/// All information retrieved is as seen by this host, which under network
/// partitions is not the global view.  
/// The ManagedEntity can be accessed with MOID of 'ha-vsan-health-system',
/// through vSAN service at at ESXi host side.
#[derive(Clone)]
pub struct HostVsanHealthSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostVsanHealthSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Check CLOMD liveness on vSAN host.
    /// 
    /// It returns True only if CLOMD is alive,
    /// otherwise it throws *VsanFault*. It never returns False.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: when CLOMD is not alive.
    pub async fn vsan_host_clomd_liveness(&self) -> Result<bool> {
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostClomdLiveness", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Deprecated as of vSphere API 6.7.
    /// 
    /// Cleanup the VMDK load test.
    /// 
    /// It will delete the created test VMDKs during this
    /// test.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### runname
    /// The name for this VMDK load test
    ///
    /// ### specs
    /// The VMDK load test spec list each of which includes
    /// the VMDK creation spec and IO load test spec.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_cleanup_vmdk_load_test(&self, runname: &str, specs: Option<&[crate::types::structs::VsanVmdkLoadTestSpec]>) -> Result<String> {
        let input = VsanHostCleanupVmdkLoadTestRequestType {runname, specs, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostCleanupVmdkLoadTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Perform VM creation test on localhost.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### timeout
    /// time out for the creation of VM
    ///
    /// ## Returns:
    ///
    /// summarized creation vm test result on the host
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***:
    pub async fn vsan_host_create_vm_health_test(&self, timeout: i32) -> Result<crate::types::structs::VsanHostCreateVmHealthTestResult> {
        let input = VsanHostCreateVmHealthTestRequestType {timeout, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostCreateVmHealthTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Deprecated as of vSphere API 8.0.
    /// 
    /// Perform a SCSI controller firmware upgrade.
    /// 
    /// Pre-requisites:
    /// - Host must be in maintenance mode to ensure vSAN is in a stable
    ///   quiesced state
    /// - The vendor tools required to perform the firmware upgrade must
    ///   have been installed
    ///   
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Firmware update specification.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_flash_scsi_controller_firmware_task(&self, spec: &crate::types::structs::VsanHclFirmwareUpdateSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanFlashScsiControllerFirmwareRequestType {spec, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanFlashScsiControllerFirmware_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Fetch HCL information about all devices in use by vSAN.
    /// 
    /// Currently covers SCSI controllers.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### include_vendor_info
    /// Include vendor tool controller raw info in the result
    ///
    /// ### vsan_esa_eligible_disks_only
    /// True indicates the query is only for
    /// vSAN ESA eligible disks. Other details of
    /// physical NICs or compute resources will not
    /// be returned.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_get_hcl_info(&self, include_vendor_info: Option<bool>, vsan_esa_eligible_disks_only: Option<bool>) -> Result<crate::types::structs::VsanHostHclInfo> {
        let input = VsanGetHclInfoRequestType {include_vendor_info, vsan_esa_eligible_disks_only, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanGetHclInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query the network related information of the host for health check.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// The network diagnostics information.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_get_network_diagnostics_health_info(&self) -> Result<crate::types::structs::VsanNetworkDiagnosticsHealthInfo> {
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanGetNetworkDiagnosticsHealthInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Retrieve information of proactive rebalance on this host
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// The current proactive rebalance information
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_get_proactive_rebalance_info(&self) -> Result<crate::types::structs::VsanProactiveRebalanceInfoEx> {
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanGetProactiveRebalanceInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Deprecated as of vSphere API 6.7.
    /// 
    /// Prepare the VMDK load test.
    /// 
    /// It will create VMDKs on the host according to the
    /// VMDK creation spec given in the specs parameter.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### runname
    /// The name for this VMDK load test
    ///
    /// ### specs
    /// The VMDK load test spec list each of which includes
    /// the VMDK creation spec and IO load test spec
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_prepare_vmdk_load_test(&self, runname: &str, specs: &[crate::types::structs::VsanVmdkLoadTestSpec]) -> Result<String> {
        let input = VsanHostPrepareVmdkLoadTestRequestType {runname, specs, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostPrepareVmdkLoadTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query advanced configuration on host
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### options
    /// list of path for the configuration name
    /// for example: \[VSAN.VsanSparseCacheThreshold, VSAN.ClomRepairDelay, VSAN.ClomRebalanceThreshold, VSAN.DomLongOpTraceMS...\]
    ///
    /// ### include_all_adv_options
    /// Flag to check for all possible config options.
    /// If set to True the options parameter is ignored.
    ///
    /// ### non_default_only
    /// Flag to return only options with non-default values.
    ///
    /// ## Returns:
    ///
    /// list of key value pair of the querying options &lt;Option: configValue&gt;
    /// for example:&lt;'VSAN.VsanSparseCacheThreshold':1&gt;, &lt;'VSAN.ClomRepairDelay':60&gt;...
    /// Allow to return empty list.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: the path is not found
    pub async fn vsan_host_query_adv_cfg(&self, options: &[String], include_all_adv_options: Option<bool>, non_default_only: Option<bool>) -> Result<Option<Vec<Box<dyn crate::types::traits::OptionValueTrait>>>> {
        let input = VsanHostQueryAdvCfgRequestType {options, include_all_adv_options, non_default_only, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQueryAdvCfg", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Determines limit health, i.e.
    /// 
    /// if any system resources (free disk space, vSAN component
    /// counts, etc.) are exhausted, or would be exhausted after simulated failures. All
    /// information is as seen by this host, which under network partitions is not the global
    /// view.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Please refer to *VsanHostQueryCheckLimitsSpec* for more details.
    ///
    /// ## Returns:
    ///
    /// Summarized limit health as seen by this host
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_query_check_limits(&self, spec: Option<&crate::types::structs::VsanHostQueryCheckLimitsSpec>) -> Result<crate::types::structs::VsanLimitHealthResult> {
        let input = VsanHostQueryCheckLimitsRequestType {spec, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQueryCheckLimits", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query the encryption health summary on the host.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// The encryption health summary result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_query_encryption_health_summary(&self) -> Result<crate::types::structs::VsanEncryptionHealthSummary> {
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQueryEncryptionHealthSummary", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Query the file service health summary on the host.
    /// 
    /// It will include
    /// infra health at least.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// The file service health summary result.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_query_file_service_health_summary(&self) -> Result<crate::types::structs::VsanFileServiceHealthSummary> {
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQueryFileServiceHealthSummary", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Query the host's maintenance mode and vSAN node decommission state.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// the summary including if host is in MM and node decommission state on this host.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_host_emm_state(&self) -> Result<crate::types::structs::VsanHostEmmSummary> {
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanQueryHostEMMState", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Query host info by host uuid.
    /// 
    /// Multiple vSAN Host UUIDs may be passed in.
    /// If a uuid can't be resolved, it is still included in the result set, but without
    /// any additional information given.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### uuids
    /// List of vSAN Host/Node UUIDs
    ///
    /// ## Returns:
    ///
    /// The hosts information for the given vSAN host/node UUIDs
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_query_host_info_by_uuids(&self, uuids: &[String]) -> Result<Vec<crate::types::structs::VsanQueryResultHostInfo>> {
        let input = VsanHostQueryHostInfoByUuidsRequestType {uuids, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQueryHostInfoByUuids", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query the object health status
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### obj_uuids
    /// The DOM object UUID list to be queried.
    /// None to query all of objects
    ///
    /// ### include_obj_uuids
    /// True to include the object UUID list
    /// in the returned list and default is False
    ///
    /// ### local_host_only
    /// True to only query the objects owned by the host
    /// and default is False
    ///
    /// ### include_non_compliance_obj_detail
    /// True to include all of non-compliant objects
    /// detail information. The default is False
    ///
    /// ### spec
    /// The additional query spec for object health.
    /// If the object health version is v2 *VsanObjectHealthVersion_enum*,
    /// it will try to convert v1 to v2 if not all of host can
    /// support the new object health version.
    /// If the object health version is unknown, it will always
    /// try to return the v2 object health version if all hosts support
    /// or return v1 if it doesn't
    ///
    /// ## Returns:
    ///
    /// The object health status query result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_query_object_health_summary(&self, obj_uuids: Option<&[String]>, include_obj_uuids: Option<bool>, local_host_only: Option<bool>, include_non_compliance_obj_detail: Option<bool>, spec: Option<&crate::types::structs::VsanHealthQuerySpec>) -> Result<crate::types::structs::VsanObjectOverallHealth> {
        let input = VsanHostQueryObjectHealthSummaryRequestType {obj_uuids, include_obj_uuids, local_host_only, include_non_compliance_obj_detail, spec, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQueryObjectHealthSummary", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query the physical disks health summary on the host
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// The physical disks health summary result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_query_physical_disk_health_summary(&self) -> Result<crate::types::structs::VsanPhysicalDiskHealthSummary> {
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQueryPhysicalDiskHealthSummary", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Run the network performance test client side program to act as the
    /// sender to send the packet to each of receiver.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### multicast
    /// True to test mutlicast network performance.
    /// False to test unicast network performance.
    ///
    /// ### server_ip
    /// The server IP binding to in the test
    ///
    /// ### duration_sec
    /// The duration of the network performance test.
    /// Default is 15 seconds if not set.
    ///
    /// ### spec
    /// The additional query spec for iperf client.
    ///
    /// ## Returns:
    ///
    /// The network performance test result
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the host can not be contacted to perform
    /// the operation.
    /// 
    /// ***NotSupported***: If run multicast in vSphere 6.7 above
    /// version.
    pub async fn vsan_host_query_run_iperf_client(&self, multicast: bool, server_ip: &str, duration_sec: Option<i32>, spec: Option<&crate::types::structs::VsanIperfClientSpec>) -> Result<crate::types::structs::VsanNetworkLoadTestResult> {
        let input = VsanHostQueryRunIperfClientRequestType {multicast, server_ip, duration_sec, spec, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQueryRunIperfClient", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Run the network performance test server side program to act as the
    /// receiver to receive the packet from sender.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### multicast
    /// True to test mutlicast network performance.
    /// False to test unicast network performance.
    ///
    /// ### server_ip
    /// The server IP binding to in the test
    ///
    /// ### duration_sec
    /// The duration of the network performance test.
    /// Default is 15 seconds if not set.
    ///
    /// ## Returns:
    ///
    /// The network performance test result
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_query_run_iperf_server(&self, multicast: bool, server_ip: Option<&str>, duration_sec: Option<i32>) -> Result<crate::types::structs::VsanNetworkLoadTestResult> {
        let input = VsanHostQueryRunIperfServerRequestType {multicast, server_ip, duration_sec, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQueryRunIperfServer", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query the physical disks S.M.A.R.T.
    /// 
    /// stats on the host
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### disks
    /// -
    ///
    /// ### include_all_disks
    /// -
    ///
    /// ## Returns:
    ///
    /// The physical disks S.M.A.R.T. stats
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_query_smart_stats(&self, disks: Option<&[String]>, include_all_disks: Option<bool>) -> Result<crate::types::structs::VsanSmartStatsHostSummary> {
        let input = VsanHostQuerySmartStatsRequestType {disks, include_all_disks, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQuerySmartStats", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Queries all network settings required to perform a cluster wide network health
    /// check.
    /// 
    /// In addition, for all specified peers connectivity checks are performed,
    /// providing network health from the perspective of this ESXi host.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### peers
    /// List of IP addresses of other hosts in the vSAN cluster.
    /// Used to perform connectivity checks.
    ///
    /// ### robo_stretched_cluster_witnesses
    /// List of ROBO witness IPs.
    /// Used to info host that during the network connectivity check, the
    /// time out threshold should be different.
    ///
    /// ### v_motion_peers
    /// List of IP addresses of vMotion traffic enabled NICs excludes this host.
    /// Used to perform vMotion connectivity checks.
    ///
    /// ### spec
    /// -
    ///
    /// ## Returns:
    ///
    /// Summarized network health related information from the perspective of this host.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_query_verify_network_settings(&self, peers: Option<&[String]>, robo_stretched_cluster_witnesses: Option<&[String]>, v_motion_peers: Option<&[String]>, spec: Option<&crate::types::structs::VsanHealthQuerySpec>) -> Result<crate::types::structs::VsanNetworkHealthResult> {
        let input = VsanHostQueryVerifyNetworkSettingsRequestType {peers, robo_stretched_cluster_witnesses, v_motion_peers, spec, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQueryVerifyNetworkSettings", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Return a string which represents vSAN version number for the querying host.
    /// 
    /// for example: vSphere 6.0u2 host will return '6.2.0.0'
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### display_version
    /// True to return vSAN display version instead of
    /// internal version. Default is False.
    ///
    /// ## Returns:
    ///
    /// The vSAN version.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_query_health_system_version(&self, display_version: Option<bool>) -> Result<String> {
        let input = VsanHostQueryHealthSystemVersionRequestType {display_version, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostQueryHealthSystemVersion", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// When the health check for vSAN object health test detects issues,
    /// this API can be used to repair the objects immediately.
    /// 
    /// the result would return objects in three lists
    /// the queuing repairing list: contains objects which are queuing for repairing
    /// the failed repaired list: contains objects which fails to repaired
    /// the not in queue list: contains objects which are not in repairing queue due
    /// to out of slot even though the objects are in the queuing repairing list, it
    /// does not mean the repair process is done. The completing time for repairing
    /// all of objects is unpredictable and depends on vSAN backend.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### uuids
    /// UUID list of the objects to be fixed
    ///
    /// ### repair_type
    /// Type of repair, can be 'fix-object-immediate' (default) or 'crawler'
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_repair_immediate_objects(&self, uuids: Option<&[String]>, repair_type: Option<&str>) -> Result<crate::types::structs::VsanRepairObjectsResult> {
        let input = VsanHostRepairImmediateObjectsRequestType {uuids, repair_type, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostRepairImmediateObjects", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Deprecated as of vSphere API 6.7.
    /// 
    /// Run the VMDK load test according to the IO load test spec.
    /// 
    /// It cannot be run before the test VMDK is created by invoking
    /// QueryPrepareVmdkLoadTest() API.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### runname
    /// The name for this VMDK load test
    ///
    /// ### duration_sec
    /// The duration time for each of VMDK load test
    ///
    /// ### specs
    /// The VMDK load test spec list each of which includes
    /// the VMDK creation spec and IO load test spec
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_run_vmdk_load_test(&self, runname: &str, duration_sec: i32, specs: &[crate::types::structs::VsanVmdkLoadTestSpec]) -> Result<Vec<crate::types::structs::VsanVmdkLoadTestResult>> {
        let input = VsanHostRunVmdkLoadTestRequestType {runname, duration_sec, specs, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanHostRunVmdkLoadTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Initiate proactive rebalance on target host
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### time_span
    /// Determines how long this proactive rebalance
    /// operation lasts in seconds, default 86400.
    ///
    /// ### variance_threshold
    /// Only if the disk's fullness (defined as
    /// used\_capacity/disk\_capacity) is above mean fullness
    /// and exceeds the lowest-usage disk in the cluster than
    /// this threshold, this disk is qualified for proactive
    /// rebalancing, default 0.3.
    ///
    /// ### time_threshold
    /// Only if the variance threshold has been
    /// continuously exceeded for this amount of time (in sec),
    /// the proactive rebalance operation will be applied to
    /// this disk, default 1800.
    ///
    /// ### rate_threshold
    /// Determines how many bytes CLOMD on this node can
    /// move out per hour (MB/hr) for proactive rebalancing,
    /// default 51200.
    ///
    /// ## Returns:
    ///
    /// True if the proactive rebalance has been triggered successfully but
    /// doesn't mean the proactive rebalance has been finished
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_start_proactive_rebalance(&self, time_span: Option<i32>, variance_threshold: Option<f32>, time_threshold: Option<i32>, rate_threshold: Option<i32>) -> Result<bool> {
        let input = VsanStartProactiveRebalanceRequestType {time_span, variance_threshold, time_threshold, rate_threshold, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanStartProactiveRebalance", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Stop proactive rebalance on target host
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Returns:
    ///
    /// True if triggering stopping proactive rebalance successfully but
    /// doesn't mean the proactive rebalance has been stopped
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_stop_proactive_rebalance(&self) -> Result<bool> {
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanStopProactiveRebalance", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Waiting until the change of current vSAN health generation ID or timed out.
    /// 
    /// The change of generation ID indicates there are potential vSAN health status
    /// changes in the host
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### timeout
    /// The timeout in seconds. The recommended timeout is 10s
    ///
    /// ## Returns:
    ///
    /// True indicates there is generation ID change
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_wait_for_vsan_health_generation_id_change(&self, timeout: i32) -> Result<bool> {
        let input = VsanWaitForVsanHealthGenerationIdChangeRequestType {timeout, };
        let path = format!("/vsan/HostVsanHealthSystem/{moId}/VsanWaitForVsanHealthGenerationIdChange", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostCleanupVmdkLoadTestRequestType<'a> {
    runname: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    specs: Option<&'a [crate::types::structs::VsanVmdkLoadTestSpec]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostCreateVmHealthTestRequestType {
    timeout: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanFlashScsiControllerFirmwareRequestType<'a> {
    spec: &'a crate::types::structs::VsanHclFirmwareUpdateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanGetHclInfoRequestType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeVendorInfo")]
    include_vendor_info: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vsanEsaEligibleDisksOnly")]
    vsan_esa_eligible_disks_only: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostPrepareVmdkLoadTestRequestType<'a> {
    runname: &'a str,
    specs: &'a [crate::types::structs::VsanVmdkLoadTestSpec],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostQueryAdvCfgRequestType<'a> {
    options: &'a [String],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeAllAdvOptions")]
    include_all_adv_options: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nonDefaultOnly")]
    non_default_only: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostQueryCheckLimitsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a crate::types::structs::VsanHostQueryCheckLimitsSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostQueryHostInfoByUuidsRequestType<'a> {
    uuids: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostQueryObjectHealthSummaryRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objUuids")]
    obj_uuids: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeObjUuids")]
    include_obj_uuids: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "localHostOnly")]
    local_host_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeNonComplianceObjDetail")]
    include_non_compliance_obj_detail: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a crate::types::structs::VsanHealthQuerySpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostQueryRunIperfClientRequestType<'a> {
    multicast: bool,
    #[serde(rename = "serverIp")]
    server_ip: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "durationSec")]
    duration_sec: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a crate::types::structs::VsanIperfClientSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostQueryRunIperfServerRequestType<'a> {
    multicast: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serverIp")]
    server_ip: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "durationSec")]
    duration_sec: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostQuerySmartStatsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    disks: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeAllDisks")]
    include_all_disks: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostQueryVerifyNetworkSettingsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    peers: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ROBOStretchedClusterWitnesses")]
    robo_stretched_cluster_witnesses: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vMotionPeers")]
    v_motion_peers: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a crate::types::structs::VsanHealthQuerySpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostQueryHealthSystemVersionRequestType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayVersion")]
    display_version: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostRepairImmediateObjectsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    uuids: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "repairType")]
    repair_type: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHostRunVmdkLoadTestRequestType<'a> {
    runname: &'a str,
    #[serde(rename = "durationSec")]
    duration_sec: i32,
    specs: &'a [crate::types::structs::VsanVmdkLoadTestSpec],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanStartProactiveRebalanceRequestType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeSpan")]
    time_span: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "varianceThreshold")]
    variance_threshold: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeThreshold")]
    time_threshold: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rateThreshold")]
    rate_threshold: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanWaitForVsanHealthGenerationIdChangeRequestType {
    timeout: i32,
}
