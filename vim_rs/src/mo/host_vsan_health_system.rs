use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostVsanHealthSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostClomdLiveness", None).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostCleanupVmdkLoadTest", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostCreateVmHealthTest", Some(&input)).await?;
        let result: crate::types::structs::VsanHostCreateVmHealthTestResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanFlashScsiControllerFirmware_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanGetHclInfo", Some(&input)).await?;
        let result: crate::types::structs::VsanHostHclInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanGetNetworkDiagnosticsHealthInfo", None).await?;
        let result: crate::types::structs::VsanNetworkDiagnosticsHealthInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanGetProactiveRebalanceInfo", None).await?;
        let result: crate::types::structs::VsanProactiveRebalanceInfoEx = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostPrepareVmdkLoadTest", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes_opt = self.client.invoke_optional("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQueryAdvCfg", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQueryCheckLimits", Some(&input)).await?;
        let result: crate::types::structs::VsanLimitHealthResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQueryEncryptionHealthSummary", None).await?;
        let result: crate::types::structs::VsanEncryptionHealthSummary = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQueryFileServiceHealthSummary", None).await?;
        let result: crate::types::structs::VsanFileServiceHealthSummary = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanQueryHostEMMState", None).await?;
        let result: crate::types::structs::VsanHostEmmSummary = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQueryHostInfoByUuids", Some(&input)).await?;
        let result: Vec<crate::types::structs::VsanQueryResultHostInfo> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQueryObjectHealthSummary", Some(&input)).await?;
        let result: crate::types::structs::VsanObjectOverallHealth = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQueryPhysicalDiskHealthSummary", None).await?;
        let result: crate::types::structs::VsanPhysicalDiskHealthSummary = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQueryRunIperfClient", Some(&input)).await?;
        let result: crate::types::structs::VsanNetworkLoadTestResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQueryRunIperfServer", Some(&input)).await?;
        let result: crate::types::structs::VsanNetworkLoadTestResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQuerySmartStats", Some(&input)).await?;
        let result: crate::types::structs::VsanSmartStatsHostSummary = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQueryVerifyNetworkSettings", Some(&input)).await?;
        let result: crate::types::structs::VsanNetworkHealthResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostQueryHealthSystemVersion", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostRepairImmediateObjects", Some(&input)).await?;
        let result: crate::types::structs::VsanRepairObjectsResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanHostRunVmdkLoadTest", Some(&input)).await?;
        let result: Vec<crate::types::structs::VsanVmdkLoadTestResult> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanStartProactiveRebalance", Some(&input)).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanStopProactiveRebalance", None).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "HostVsanHealthSystem", &self.mo_id, "VsanWaitForVsanHealthGenerationIdChange", Some(&input)).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct VsanHostCleanupVmdkLoadTestRequestType<'a> {
    runname: &'a str,
    specs: Option<&'a [crate::types::structs::VsanVmdkLoadTestSpec]>,
}

impl<'a> miniserde::Serialize for VsanHostCleanupVmdkLoadTestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostCleanupVmdkLoadTestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostCleanupVmdkLoadTestRequestTypeSer<'b, 'a> {
    data: &'b VsanHostCleanupVmdkLoadTestRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostCleanupVmdkLoadTestRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostCleanupVmdkLoadTestRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("runname"), &self.data.runname as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.specs else { continue; };
                    return Some((std::borrow::Cow::Borrowed("specs"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanHostCreateVmHealthTestRequestType {
    timeout: i32,
}

impl miniserde::Serialize for VsanHostCreateVmHealthTestRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostCreateVmHealthTestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostCreateVmHealthTestRequestTypeSer<'b> {
    data: &'b VsanHostCreateVmHealthTestRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for VsanHostCreateVmHealthTestRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostCreateVmHealthTestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("timeout"), &self.data.timeout as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanFlashScsiControllerFirmwareRequestType<'a> {
    spec: &'a crate::types::structs::VsanHclFirmwareUpdateSpec,
}

impl<'a> miniserde::Serialize for VsanFlashScsiControllerFirmwareRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanFlashScsiControllerFirmwareRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanFlashScsiControllerFirmwareRequestTypeSer<'b, 'a> {
    data: &'b VsanFlashScsiControllerFirmwareRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanFlashScsiControllerFirmwareRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanFlashScsiControllerFirmwareRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanGetHclInfoRequestType {
    include_vendor_info: Option<bool>,
    vsan_esa_eligible_disks_only: Option<bool>,
}

impl miniserde::Serialize for VsanGetHclInfoRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanGetHclInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanGetHclInfoRequestTypeSer<'b> {
    data: &'b VsanGetHclInfoRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for VsanGetHclInfoRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanGetHclInfoRequestType")),
                1 => {
                    let Some(ref val) = self.data.include_vendor_info else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeVendorInfo"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.vsan_esa_eligible_disks_only else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vsanEsaEligibleDisksOnly"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanHostPrepareVmdkLoadTestRequestType<'a> {
    runname: &'a str,
    specs: &'a [crate::types::structs::VsanVmdkLoadTestSpec],
}

impl<'a> miniserde::Serialize for VsanHostPrepareVmdkLoadTestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostPrepareVmdkLoadTestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostPrepareVmdkLoadTestRequestTypeSer<'b, 'a> {
    data: &'b VsanHostPrepareVmdkLoadTestRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostPrepareVmdkLoadTestRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostPrepareVmdkLoadTestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("runname"), &self.data.runname as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("specs"), &self.data.specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanHostQueryAdvCfgRequestType<'a> {
    options: &'a [String],
    include_all_adv_options: Option<bool>,
    non_default_only: Option<bool>,
}

impl<'a> miniserde::Serialize for VsanHostQueryAdvCfgRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostQueryAdvCfgRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostQueryAdvCfgRequestTypeSer<'b, 'a> {
    data: &'b VsanHostQueryAdvCfgRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostQueryAdvCfgRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostQueryAdvCfgRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("options"), &self.data.options as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.include_all_adv_options else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeAllAdvOptions"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.non_default_only else { continue; };
                    return Some((std::borrow::Cow::Borrowed("nonDefaultOnly"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanHostQueryCheckLimitsRequestType<'a> {
    spec: Option<&'a crate::types::structs::VsanHostQueryCheckLimitsSpec>,
}

impl<'a> miniserde::Serialize for VsanHostQueryCheckLimitsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostQueryCheckLimitsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostQueryCheckLimitsRequestTypeSer<'b, 'a> {
    data: &'b VsanHostQueryCheckLimitsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostQueryCheckLimitsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostQueryCheckLimitsRequestType")),
                1 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanHostQueryHostInfoByUuidsRequestType<'a> {
    uuids: &'a [String],
}

impl<'a> miniserde::Serialize for VsanHostQueryHostInfoByUuidsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostQueryHostInfoByUuidsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostQueryHostInfoByUuidsRequestTypeSer<'b, 'a> {
    data: &'b VsanHostQueryHostInfoByUuidsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostQueryHostInfoByUuidsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostQueryHostInfoByUuidsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("uuids"), &self.data.uuids as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanHostQueryObjectHealthSummaryRequestType<'a> {
    obj_uuids: Option<&'a [String]>,
    include_obj_uuids: Option<bool>,
    local_host_only: Option<bool>,
    include_non_compliance_obj_detail: Option<bool>,
    spec: Option<&'a crate::types::structs::VsanHealthQuerySpec>,
}

impl<'a> miniserde::Serialize for VsanHostQueryObjectHealthSummaryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostQueryObjectHealthSummaryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostQueryObjectHealthSummaryRequestTypeSer<'b, 'a> {
    data: &'b VsanHostQueryObjectHealthSummaryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostQueryObjectHealthSummaryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostQueryObjectHealthSummaryRequestType")),
                1 => {
                    let Some(ref val) = self.data.obj_uuids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("objUuids"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.include_obj_uuids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeObjUuids"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.local_host_only else { continue; };
                    return Some((std::borrow::Cow::Borrowed("localHostOnly"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.include_non_compliance_obj_detail else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeNonComplianceObjDetail"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanHostQueryRunIperfClientRequestType<'a> {
    multicast: bool,
    server_ip: &'a str,
    duration_sec: Option<i32>,
    spec: Option<&'a crate::types::structs::VsanIperfClientSpec>,
}

impl<'a> miniserde::Serialize for VsanHostQueryRunIperfClientRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostQueryRunIperfClientRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostQueryRunIperfClientRequestTypeSer<'b, 'a> {
    data: &'b VsanHostQueryRunIperfClientRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostQueryRunIperfClientRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostQueryRunIperfClientRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("multicast"), &self.data.multicast as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("serverIp"), &self.data.server_ip as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.duration_sec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("durationSec"), val as &dyn miniserde::Serialize));
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
struct VsanHostQueryRunIperfServerRequestType<'a> {
    multicast: bool,
    server_ip: Option<&'a str>,
    duration_sec: Option<i32>,
}

impl<'a> miniserde::Serialize for VsanHostQueryRunIperfServerRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostQueryRunIperfServerRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostQueryRunIperfServerRequestTypeSer<'b, 'a> {
    data: &'b VsanHostQueryRunIperfServerRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostQueryRunIperfServerRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostQueryRunIperfServerRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("multicast"), &self.data.multicast as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.server_ip else { continue; };
                    return Some((std::borrow::Cow::Borrowed("serverIp"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.duration_sec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("durationSec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanHostQuerySmartStatsRequestType<'a> {
    disks: Option<&'a [String]>,
    include_all_disks: Option<bool>,
}

impl<'a> miniserde::Serialize for VsanHostQuerySmartStatsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostQuerySmartStatsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostQuerySmartStatsRequestTypeSer<'b, 'a> {
    data: &'b VsanHostQuerySmartStatsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostQuerySmartStatsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostQuerySmartStatsRequestType")),
                1 => {
                    let Some(ref val) = self.data.disks else { continue; };
                    return Some((std::borrow::Cow::Borrowed("disks"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.include_all_disks else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeAllDisks"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanHostQueryVerifyNetworkSettingsRequestType<'a> {
    peers: Option<&'a [String]>,
    robo_stretched_cluster_witnesses: Option<&'a [String]>,
    v_motion_peers: Option<&'a [String]>,
    spec: Option<&'a crate::types::structs::VsanHealthQuerySpec>,
}

impl<'a> miniserde::Serialize for VsanHostQueryVerifyNetworkSettingsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostQueryVerifyNetworkSettingsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostQueryVerifyNetworkSettingsRequestTypeSer<'b, 'a> {
    data: &'b VsanHostQueryVerifyNetworkSettingsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostQueryVerifyNetworkSettingsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostQueryVerifyNetworkSettingsRequestType")),
                1 => {
                    let Some(ref val) = self.data.peers else { continue; };
                    return Some((std::borrow::Cow::Borrowed("peers"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.robo_stretched_cluster_witnesses else { continue; };
                    return Some((std::borrow::Cow::Borrowed("ROBOStretchedClusterWitnesses"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.v_motion_peers else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vMotionPeers"), val as &dyn miniserde::Serialize));
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
struct VsanHostQueryHealthSystemVersionRequestType {
    display_version: Option<bool>,
}

impl miniserde::Serialize for VsanHostQueryHealthSystemVersionRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostQueryHealthSystemVersionRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostQueryHealthSystemVersionRequestTypeSer<'b> {
    data: &'b VsanHostQueryHealthSystemVersionRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for VsanHostQueryHealthSystemVersionRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostQueryHealthSystemVersionRequestType")),
                1 => {
                    let Some(ref val) = self.data.display_version else { continue; };
                    return Some((std::borrow::Cow::Borrowed("displayVersion"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanHostRepairImmediateObjectsRequestType<'a> {
    uuids: Option<&'a [String]>,
    repair_type: Option<&'a str>,
}

impl<'a> miniserde::Serialize for VsanHostRepairImmediateObjectsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostRepairImmediateObjectsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostRepairImmediateObjectsRequestTypeSer<'b, 'a> {
    data: &'b VsanHostRepairImmediateObjectsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostRepairImmediateObjectsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostRepairImmediateObjectsRequestType")),
                1 => {
                    let Some(ref val) = self.data.uuids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("uuids"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.repair_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("repairType"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanHostRunVmdkLoadTestRequestType<'a> {
    runname: &'a str,
    duration_sec: i32,
    specs: &'a [crate::types::structs::VsanVmdkLoadTestSpec],
}

impl<'a> miniserde::Serialize for VsanHostRunVmdkLoadTestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostRunVmdkLoadTestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostRunVmdkLoadTestRequestTypeSer<'b, 'a> {
    data: &'b VsanHostRunVmdkLoadTestRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostRunVmdkLoadTestRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostRunVmdkLoadTestRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("runname"), &self.data.runname as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("durationSec"), &self.data.duration_sec as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("specs"), &self.data.specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanStartProactiveRebalanceRequestType {
    time_span: Option<i32>,
    variance_threshold: Option<f32>,
    time_threshold: Option<i32>,
    rate_threshold: Option<i32>,
}

impl miniserde::Serialize for VsanStartProactiveRebalanceRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanStartProactiveRebalanceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanStartProactiveRebalanceRequestTypeSer<'b> {
    data: &'b VsanStartProactiveRebalanceRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for VsanStartProactiveRebalanceRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanStartProactiveRebalanceRequestType")),
                1 => {
                    let Some(ref val) = self.data.time_span else { continue; };
                    return Some((std::borrow::Cow::Borrowed("timeSpan"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.variance_threshold else { continue; };
                    return Some((std::borrow::Cow::Borrowed("varianceThreshold"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.time_threshold else { continue; };
                    return Some((std::borrow::Cow::Borrowed("timeThreshold"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.rate_threshold else { continue; };
                    return Some((std::borrow::Cow::Borrowed("rateThreshold"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanWaitForVsanHealthGenerationIdChangeRequestType {
    timeout: i32,
}

impl miniserde::Serialize for VsanWaitForVsanHealthGenerationIdChangeRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanWaitForVsanHealthGenerationIdChangeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanWaitForVsanHealthGenerationIdChangeRequestTypeSer<'b> {
    data: &'b VsanWaitForVsanHealthGenerationIdChangeRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for VsanWaitForVsanHealthGenerationIdChangeRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanWaitForVsanHealthGenerationIdChangeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("timeout"), &self.data.timeout as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
