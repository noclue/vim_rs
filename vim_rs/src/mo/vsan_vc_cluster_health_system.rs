use std::sync::Arc;
use crate::core::client::{Client, Result};
/// This managed object provides access to vSAN Health related configuration
/// and query APIs, operating at a vSAN cluster level.
/// 
/// It can be accessed
/// through MOID of 'vsan-cluster-health-system', through vSAN service at
/// vCenter server side.
#[derive(Clone)]
pub struct VsanVcClusterHealthSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl VsanVcClusterHealthSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Collect vSAN and vCenter support bundle and upload to VMware for the given
    /// SR (Service Request).
    /// 
    /// The collected support bundles including vSAN health
    /// and tracing log bundle, all of ESXi host support bundle and VC support bundle
    /// The upload process will be in three stages separately with the order of
    /// bundle size to avoid possible time out, which are vCenter bundles, small high
    /// importance ESXi/vSAN manifests, vSAN traces and lower priority ESXi
    /// manifests. The network proxy for uploading is also supported by calling
    /// SetVsanClusterTelemetryConfig() in advance.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vCenter cluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### sr_number
    /// The Customer Service Request number ID
    ///
    /// ## Returns:
    ///
    /// task vCenter Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***VsanFault***: 
    /// 
    /// ***NotSupported***:
    pub async fn vsan_attach_vsan_support_bundle_to_sr(&self, cluster: &crate::types::structs::ManagedObjectReference, sr_number: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanAttachVsanSupportBundleToSrRequestType {cluster, sr_number, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanAttachVsanSupportBundleToSr", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Deprecated as of vSphere API 8.0.
    /// 
    /// Download and install the vendor tool required for the target cluster.
    /// 
    /// API iterate thru all the update items for the given cluster and download
    /// the all the vendor tools required by the cluster. Vendor tool update item
    /// is specified by vibSpecType as 'tool'. Download of vendor tool will be
    /// skipped if tool is already present on vCenter storage. Check
    /// VsanDownloadHclFile\_Task for the details about download. After download
    /// vendor tools will be installed on the hosts referenced by the vibSpec of
    /// vendor tools.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    pub async fn vsan_download_and_install_vendor_tool_task(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanDownloadAndInstallVendorToolRequestType {cluster, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanDownloadAndInstallVendorTool_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Deprecated as of vSphere API 8.0.
    /// 
    /// Download driver/firmware from the URL extracted from HCL database by
    /// 'id' and 'md5sum'.
    /// 
    /// The content will be save on the vCenter storage with
    /// the auto-generated filename. The checksum of content will be checked
    /// after content is saved on the disk.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### sha_1_sums
    /// -
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***VsanFault***: 
    /// 
    /// ***NotSupported***:
    pub async fn vsan_download_hcl_file_task(&self, sha_1_sums: &[String]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanDownloadHclFileRequestType {sha_1_sums, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanDownloadHclFile_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Get the HCL driver/firmware constraints for PCIe devices used by vSAN in the
    /// cluster.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vSAN cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### release
    /// The target vSphere release
    ///
    /// ## Returns:
    ///
    /// The HCL driver constraints for all the PCIe device used by vSAN, and
    /// current firmware versions of that device in the cluster.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***VsanFault***: If unexpected error happened during the query, such as the
    /// cluster HCL info is unable to be retrieved, etc.
    /// 
    /// ***NotSupported***:
    pub async fn vsan_get_hcl_constraints(&self, cluster: &crate::types::structs::ManagedObjectReference, release: &str) -> Result<crate::types::structs::VsanHclReleaseConstraint> {
        let input = VsanGetHclConstraintsRequestType {cluster, release, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanGetHclConstraints", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Fetch HCL information including the HCL DB status in general and all devices
    /// used by vSAN on all hosts in the cluster(if any).
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vCenter cluster. Set to NULL if only intent to get
    /// the HCL DB status in general
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### include_hosts_result
    /// True to include the HCL information for
    /// each of host in the cluster.
    ///
    /// ### include_vendor_info
    /// Include the vendor tool information in the result
    ///
    /// ### esx_release
    /// The ESXi release version which HCL is computed against
    ///
    /// ### query_spec
    /// -
    ///
    /// ## Returns:
    ///
    /// The HCL information for the cluster
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vc_cluster_get_hcl_info(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, include_hosts_result: Option<bool>, include_vendor_info: Option<bool>, esx_release: Option<&str>, query_spec: Option<&crate::types::structs::VsanHclQuerySpec>) -> Result<crate::types::structs::VsanClusterHclInfo> {
        let input = VsanVcClusterGetHclInfoRequestType {cluster, include_hosts_result, include_vendor_info, esx_release, query_spec, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanVcClusterGetHclInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Get the vSAN recommendation of ESXi releases to update for a cluster, and
    /// the associated HCL device constraints to comply for those releases.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vSAN cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### minor
    /// A list of minor update releases. E.g., \['ESXi 6.7 U2'\]
    ///
    /// ### major
    /// A list of major update releases. E.g., \['ESXi 6.8', 'ESXi 7.0'\]
    ///
    /// ## Returns:
    ///
    /// A list of VsanHclReleaseConstraint consists of: Hcl Constraints for
    /// one recommended minor release(if applicable) and Hcl Constraints for
    /// one recommended major release(if applicable). If none of the minor or
    /// major releases are prefered by vSAN, an empty list will be returned.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***VsanFault***: If unexpected error happened during the query, such as
    /// the releases to query is not recognized or supported by
    /// vSAN, or vSAN is not able to retrieve the HCL info from
    /// the cluster, etc.
    /// 
    /// ***NotSupported***:
    pub async fn vsan_get_release_recommendation(&self, cluster: &crate::types::structs::ManagedObjectReference, minor: &[String], major: &[String]) -> Result<Option<Vec<crate::types::structs::VsanHclReleaseConstraint>>> {
        let input = VsanGetReleaseRecommendationRequestType {cluster, minor, major, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanGetReleaseRecommendation", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Get the vSAN HCL constraints for disk drives.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### release
    /// The target vSphere release. If not provided, HCL constraints
    /// for all the supported releases will be returned.
    ///
    /// ### disk_models
    /// The disk models to query. If provided, the HCL constraints
    /// with matching disk model info as in the query options will
    /// be returned.
    ///
    /// ## Returns:
    ///
    /// The HCL constraints for all the disk drives.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***VsanFault***: If unexpected error happened during the query, such as the
    /// cluster HCL info is unable to be retrieved, etc.
    /// 
    /// ***NotSupported***:
    pub async fn vsan_get_disk_hcl_constraints(&self, release: Option<&str>, disk_models: Option<&[crate::types::structs::VsanDiskModelInfo]>) -> Result<Option<Vec<crate::types::structs::VsanHclDiskConstraint>>> {
        let input = VsanGetDiskHclConstraintsRequestType {release, disk_models, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanGetDiskHclConstraints", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Fetch HCL information for all vSAN ESA eligible disks of the target
    /// object which can be a vSAN cluster or a list of hosts.
    /// 
    /// Check
    /// *VsanHclQuerySpec* for more details.
    /// Note that it only returns disk related HCL information, the fields of other
    /// hardware components like CPU, physical NIC will be unset.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### query_spec
    /// Provide the capability to customize the query
    ///
    /// ## Returns:
    ///
    /// The HCL information for the cluster
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: If host could not be contacted to perform the operation.
    /// 
    /// ***VsanFault***: If unexpected error happened during the query, such as the
    /// cluster HCL info is unable to be retrieved, etc.
    /// 
    /// ***NotSupported***: The API on host level is not supported.
    pub async fn vsan_get_hcl_info_for_eligible_disks(&self, query_spec: &crate::types::structs::VsanHclQuerySpec) -> Result<crate::types::structs::VsanClusterHclInfo> {
        let input = VsanGetHclInfoForEligibleDisksRequestType {query_spec, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanGetHclInfoForEligibleDisks", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Get the user configured silent health check list of the cluster.
    /// 
    /// This API will not return the system configured silent health check
    /// items. vSAN will automatically configure the following health check
    /// items in the following conditions:
    /// - HCL DB auto update in the air gap case.
    ///   
    /// This API only supported on vCenter.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target VC cluster
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
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***NotSupported***:
    pub async fn vsan_health_get_vsan_cluster_silent_checks(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<String>>> {
        let input = VsanHealthGetVsanClusterSilentChecksRequestType {cluster, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthGetVsanClusterSilentChecks", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Check whether the proactive rebalance is running or not against the
    /// target cluster or the hosts
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
    /// ### target_hosts
    /// Leave this unset
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// True if any of host has running rebalance process
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_health_is_rebalance_running(&self, cluster: &crate::types::structs::ManagedObjectReference, target_hosts: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<bool> {
        let input = VsanHealthIsRebalanceRunningRequestType {cluster, target_hosts, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthIsRebalanceRunning", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Deprecated as of vSphere API 8.0.
    /// 
    /// Purge driver/firmware that was previously put on vCenter (either
    /// via download from URL or upload to vCenter).
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### sha_1_sums
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***VsanFault***: 
    /// 
    /// ***NotSupported***:
    pub async fn vsan_purge_hcl_files(&self, sha_1_sums: &[String]) -> Result<()> {
        let input = VsanPurgeHclFilesRequestType {sha_1_sums, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanPurgeHclFiles", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Query basic info of all supported health checks
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// a list of health check info constructed by testId, testName, groupId and groupName
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_all_supported_health_checks(&self) -> Result<Vec<crate::types::structs::VsanClusterHealthCheckInfo>> {
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryAllSupportedHealthChecks", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Query the historical attach to SR operation result for the vCenter cluster.
    /// 
    /// The historical data is only saved into memory and will disappear after the
    /// service is restart or shut down.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vCenter cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### count
    /// Collect the last test data for the given number at most.
    /// Default to get the latest one historical data.
    ///
    /// ### task_id
    /// The task Id which run the attach to SR operation.Default is None
    /// The count parameter will be ignored when the taskId is not None
    ///
    /// ## Returns:
    ///
    /// The attach to SR operation results
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_attach_to_sr_history(&self, cluster: &crate::types::structs::ManagedObjectReference, count: Option<i32>, task_id: Option<&str>) -> Result<Option<Vec<crate::types::structs::VsanAttachToSrOperation>>> {
        let input = VsanQueryAttachToSrHistoryRequestType {cluster, count, task_id, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryAttachToSrHistory", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Query the historical create VM test result for the vCenter cluster.
    /// 
    /// The
    /// historical data is only saved into memory and will disappear after the
    /// service is restart or shut down.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vCenter cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### count
    /// Collect the last test data for the given number at most.
    /// Default to get the latest one historical data
    ///
    /// ### datastore
    /// The datastore where the create vm test has been run.
    /// It could be the local datastore or the remote datastore
    /// that is mounted to the cluster. By default it will run
    /// with local vSAN datastore.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// The VM creation test results.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If cluster's hosts could not be contacted to
    /// perform the operation.
    pub async fn vsan_query_vc_cluster_create_vm_health_history_test(&self, cluster: &crate::types::structs::ManagedObjectReference, count: Option<i32>, datastore: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::VsanClusterCreateVmHealthTestResult>>> {
        let input = VsanQueryVcClusterCreateVmHealthHistoryTestRequestType {cluster, count, datastore, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryVcClusterCreateVmHealthHistoryTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Executes VM creation test and checks if a tiny VM can be created on each of
    /// host of the vSAN cluster
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster
    /// 
    /// ***Required privileges:*** VirtualMachine.Inventory.Create VirtualMachine.Inventory.Delete
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### timeout
    /// The timeout in seconds for the VM creation test.
    /// The suggested value could be 2 mintues
    ///
    /// ### datastore
    /// The datastore where the VM creation test will be run. It
    /// could be the local datastore or the remote datastore that
    /// is mounted to the cluster. By default it will run with
    /// local vSAN datastore.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster VM creation test result
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If cluster's hosts could not be contacted to
    /// perform the operation.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments.
    /// 
    /// ***NotSupported***: Exception for hostlevel API call for API
    /// with remote datastore in hostlevel call.
    pub async fn vsan_query_vc_cluster_create_vm_health_test(&self, cluster: &crate::types::structs::ManagedObjectReference, timeout: i32, datastore: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::VsanClusterCreateVmHealthTestResult> {
        let input = VsanQueryVcClusterCreateVmHealthTestRequestType {cluster, timeout, datastore, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryVcClusterCreateVmHealthTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Perform a cluster wide health check across all types of health checks.
    /// 
    /// It's the
    /// primary API for fetching vSAN health status.
    /// This API runs a wide variety of health checks in many different categories against
    /// the cluster and returns both API-friendly and UI-friendly data. The 'fields' parameter
    /// should be used to return what the client is really interested in to limit the big result
    /// size for performance consideration. The API can either perform a health check against
    /// the up-to-date state of the hosts in the cluster and takes several seconds to return or
    /// it can retrieve the latest cached result to return in second. It's encouraged to fetch
    /// the cached data to get the quick response unless an update is required.
    /// Note: when this API is called directly from an ESXi host, following test groups are
    /// not supported:
    /// groupId = com.vmware.vsan.health.test.hcl
    /// 
    /// Querying vSAN cluster requires System.Read privilege on the cluster. If the API
    /// is called to have pre-flight health check on a bunch of hosts, System.Read privilege
    /// on all these hosts is required.
    /// 
    /// See also *VsanHealthPerspective_enum*, *VsanHealthPerspective90_enum*.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### vm_create_timeout
    /// -
    ///
    /// ### obj_uuids
    /// -
    ///
    /// ### include_obj_uuids
    /// Whether to include detailed per-object health in the
    /// result. Default to True. Set to false if this value
    /// is not of importance to save computing resources.
    ///
    /// ### fields
    /// If set, only the properties listed in the array are returned in the
    /// result data object. Normally the output of this API can be quite large
    /// so this allows clients which are only interested in a subset to save
    /// network bandwidth and deserialization effort. The possible value is the
    /// field from the VsanClusterHealthSummary class like 'objectHealth',
    /// 'networkHealth' etc
    /// If unset, following properities are included in the result.
    /// 'clusterStatus',
    /// 'timestamp',
    /// 'clusterVersions',
    /// 'objectHealth',
    /// 'vmHealth',
    /// 'networkHealth',
    /// 'limitHealth',
    /// 'advCfgSync',
    /// 'createVmHealth',
    /// 'physicalDisksHealth',
    /// 'hclInfo',
    /// 'groups',
    /// 'overallHealth',
    /// 'overallHealthDescription',
    /// 'clomdLiveness',
    /// 'diskBalance'
    ///
    /// ### fetch_from_cache
    /// True to return the result from cache directly instead
    /// of running the full health check. The cache will be
    /// updated and keep the latest vSAN health summary check
    /// result either triggering from user on-demand request or
    /// the periodical vSAN health check for triggering health
    /// event/alarm. Default is False.
    ///
    /// ### perspective
    /// The total amount of health checks is split into multiple different
    /// perspectives. This parameter controls which health checks are
    /// performed/returned. All supported values are defined
    /// in below enumerations:
    /// *VsanHealthPerspective_enum*,
    /// *VsanHealthPerspective90_enum*.
    ///
    /// ### hosts
    /// Include the individual host(s) into the health check, with
    /// other hosts specified with the cluster parameter.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ### spec
    /// Health summary query specification *VsanClusterHealthQuerySpec*.
    ///
    /// ## Returns:
    ///
    /// Returns a health summary data object.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If the perspective validation failed, or
    /// the API is queried from vSAN witness node, or
    /// any unexpected runtime error.
    pub async fn vsan_query_vc_cluster_health_summary(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, vm_create_timeout: Option<i32>, obj_uuids: Option<&[String]>, include_obj_uuids: Option<bool>, fields: Option<&[String]>, fetch_from_cache: Option<bool>, perspective: Option<&str>, hosts: Option<&[crate::types::structs::ManagedObjectReference]>, spec: Option<&crate::types::structs::VsanClusterHealthQuerySpec>) -> Result<crate::types::structs::VsanClusterHealthSummary> {
        let input = VsanQueryVcClusterHealthSummaryRequestType {cluster, vm_create_timeout, obj_uuids, include_obj_uuids, fields, fetch_from_cache, perspective, hosts, spec, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryVcClusterHealthSummary", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// An asynchronous API to perform all of cluster wide health checks including
    /// the online health checks if CEIP is enabled and returns the task.
    /// 
    /// After the
    /// task is finished, all of health check result will be kept in the cache and
    /// can be fetched from the synchronous API
    /// *VsanVcClusterHealthSystem.VsanQueryVcClusterHealthSummary* by
    /// setting the parameter fetchFromCache as True, the perspective as 'defaultView'
    /// or unset and the parameter hosts should keep the same while leaving the other
    /// parameters as unset.
    /// If a new task is triggered while there's running task for same cluster,
    /// the new triggered task will be pending and return immediatelly once the
    /// running task is completed. The new triggered task will share the same
    /// health summary result as the running one.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### hosts
    /// Include the individual host(s) into the health check, with
    /// other hosts specified with the cluster parameter
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ### include_data_protection_health
    /// This parameter is not used,
    /// and no data protection health will be collected.
    ///
    /// ### include_online_health
    /// True to trigger vSAN online health.
    /// The default is True.
    ///
    /// ## Returns:
    ///
    /// task VC Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_vc_cluster_health_summary_task(&self, cluster: &crate::types::structs::ManagedObjectReference, hosts: Option<&[crate::types::structs::ManagedObjectReference]>, include_data_protection_health: Option<bool>, include_online_health: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanQueryVcClusterHealthSummaryTaskRequestType {cluster, hosts, include_data_protection_health, include_online_health, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryVcClusterHealthSummaryTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// It queries vSAN cluster historical health information based
    /// on the query spec.
    /// - Query general historical health summary.
    ///   If no groupId and testId specified in the query spec,
    ///   it aggregates all the historical vSAN cluster health status
    ///   for the given time range. Then returns a general health summary
    ///   with only overallHealth, test group IDs, group healths,
    ///   test IDs and test healths.
    /// - Query health status history for a certain health check.
    ///   If groupId and testId are specified, and start time doesn't
    ///   equal to end time, the result will only contain the historical
    ///   testHealths for the given time range of the target health check.
    /// - Query a snapshot detail for a certain health check.
    ///   If groupId and testId are specified, and start time equals to
    ///   end time, the result will contain a full testDetail for the certain
    ///   timestamp of the target health check.
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
    /// vSAN cluster health summary with historical infomation.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments.
    /// For example, if given time range
    /// is more than one month. Or end time is
    /// smaller than the start time.
    pub async fn vsan_query_cluster_historical_health(&self, spec: &crate::types::structs::VsanHistoricalHealthQuerySpec) -> Result<Option<Vec<crate::types::structs::VsanClusterHealthSummary>>> {
        let input = VsanQueryClusterHistoricalHealthRequestType {spec, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryClusterHistoricalHealth", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Query the historical network performance test result for the vCenter cluster.
    /// 
    /// The historical data is only saved into memory and will disappear after the
    /// service is restart or shut down.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vCenter cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### count
    /// Collect the last test data for the given number at most.
    /// Default to get the latest one historical data.
    ///
    /// ### spec
    /// The additional query spec for query network performance health
    /// test history.
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster network performance test results.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If host could not be contacted to perform
    /// the operation.
    pub async fn vsan_query_vc_cluster_network_perf_history_test(&self, cluster: &crate::types::structs::ManagedObjectReference, count: Option<i32>, spec: Option<&crate::types::structs::VsanClusterNetworkPerfTaskSpec>) -> Result<Option<Vec<crate::types::structs::VsanClusterNetworkLoadTestResult>>> {
        let input = VsanQueryVcClusterNetworkPerfHistoryTestRequestType {cluster, count, spec, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryVcClusterNetworkPerfHistoryTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Asynchronous implementation of network performance test API
    /// *VsanVcClusterHealthSystem.VsanQueryVcClusterNetworkPerfTest*
    /// This returns task to monitor progress on success result is cached which can
    /// be retrieved using API using API
    /// *VsanVcClusterHealthSystem.VsanQueryVcClusterNetworkPerfHistoryTest*
    /// with the same datastore in query spec.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### spec
    /// The additional query spec for network performance health
    ///
    /// ## Returns:
    ///
    /// The VC task.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If cluster's hosts could not be contacted to
    /// perform the operation.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments.
    pub async fn vsan_query_vc_cluster_network_perf_task(&self, cluster: &crate::types::structs::ManagedObjectReference, spec: Option<&crate::types::structs::VsanClusterNetworkPerfTaskSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanQueryVcClusterNetworkPerfTaskRequestType {cluster, spec, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryVcClusterNetworkPerfTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query the vSAN network performance and checks if it meets the bandwidth
    /// requirements.
    /// 
    /// For multicast performance, the test is done by picking one
    /// host as the sender, and having all other hosts be receivers. For unicast
    /// performance, the test is done by having all hosts form a circular list,
    /// and every host in the list will be sending packets to the next host,
    /// while receiving packets from the previous host at the same time.
    /// For multicast test, the status in test result will be:
    /// - red if the speed is less than 20 MB/s
    /// - yellow if the speed is between 20MB/s and 50MB/s
    /// - green if the speed is larger than 50MB/s
    ///   
    /// For unicast test, the status in test result will be:
    /// - red if the speed is 0 Mb/s
    /// - yellow if the speed is less than 850 Mb/s
    /// - green if the speed is larger than or equal to 850 Mb/s
    ///   
    /// Note: Calling this API requires vCenter and all the connected hosts to
    /// support the same network performance testing capability(multicast or
    /// unicast). As vSAN is moving from multicast to unicast, only the unicast
    /// test method is supported since vSphere 6.7 Update 1 release. This API
    /// will return "Not Supported" in case the test method is not currently
    /// supported by this cluster. User could query VsanGetCapabilities() API
    /// to check whether the cluster supports the unicast network performance
    /// test. The test is supported if the 'netperftest' capability can be found
    /// on the vCenter and all the hosts in this cluster.
    /// **This API is intended for network bandwidth diagnostics in the environment
    /// with no production workloads. It is not designed for network bandwidth
    /// monitoring with production workloads as it might impact the actual network
    /// utilization during the test.**
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### multicast
    /// True to test vSAN multicast network performance.
    /// False to test vSAN unicast network performance.
    ///
    /// ### duration_sec
    /// The duration of the Network Performance test. Default is
    /// 15 seconds if not set.
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster network performance test result
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the host can not be contacted to perform
    /// the operation.
    /// 
    /// ***NotSupported***: If run multicast in vSphere 6.7 above
    /// version.
    pub async fn vsan_query_vc_cluster_network_perf_test(&self, cluster: &crate::types::structs::ManagedObjectReference, multicast: bool, duration_sec: Option<i32>) -> Result<crate::types::structs::VsanClusterNetworkLoadTestResult> {
        let input = VsanQueryVcClusterNetworkPerfTestRequestType {cluster, multicast, duration_sec, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryVcClusterNetworkPerfTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Deprecated as of vSphere API 6.7.
    /// 
    /// Query the historical vSAN storage test result for the vCenter cluster.
    /// 
    /// The historical data is only saved into memory and will disappear after the
    /// service is restart or shut down.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vCenter cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### count
    /// Collect the last test data for the given number at most.
    /// Default to get the latest one historical data.
    ///
    /// ### task_id
    /// The task Id which run the VMDK performance test. Default is None
    /// The count parameter will be ignored when the taskId is not None
    ///
    /// ## Returns:
    ///
    /// The vSAN storage load test results.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_vc_cluster_vmdk_load_history_test(&self, cluster: &crate::types::structs::ManagedObjectReference, count: Option<i32>, task_id: Option<&str>) -> Result<Option<Vec<crate::types::structs::VsanClusterVmdkLoadTestResult>>> {
        let input = VsanQueryVcClusterVmdkLoadHistoryTestRequestType {cluster, count, task_id, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryVcClusterVmdkLoadHistoryTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Deprecated as of vSphere API 6.7.
    /// 
    /// Query the pre-defined workload types for the VMDK performance test.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// The vSAN cluster VMDK workload type
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_vc_cluster_vmdk_workload_types(&self) -> Result<Vec<crate::types::structs::VsanStorageWorkloadType>> {
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryVcClusterVmdkWorkloadTypes", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Query the vSAN file service health on all the hosts in the specified
    /// cluster.
    /// 
    /// Use filter fields to determine whether to include infra health
    /// or container health. It will include infra health at least.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### include_file_server_health
    /// Whether to retrieve file server health,
    /// default value is True.
    ///
    /// ### include_file_share_health
    /// Whether to retrieve file share health,
    /// default value is True.
    ///
    /// ## Returns:
    ///
    /// The file service health summary. Return None if the file service is
    /// not enabled.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the cluster is not found.
    pub async fn vsan_cluster_query_file_service_health_summary(&self, cluster: &crate::types::structs::ManagedObjectReference, include_file_server_health: Option<bool>, include_file_share_health: Option<bool>) -> Result<Option<crate::types::structs::VsanClusterFileServiceHealthSummary>> {
        let input = VsanClusterQueryFileServiceHealthSummaryRequestType {cluster, include_file_server_health, include_file_share_health, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanClusterQueryFileServiceHealthSummary", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Query vSAN physical disks S.M.A.R.T.
    /// 
    /// stats across all hosts
    /// in the target cluster.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The vSAN physical disks S.M.A.R.T. stats
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_vc_cluster_smart_stats_summary(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Vec<crate::types::structs::VsanSmartStatsHostSummary>> {
        let input = VsanQueryVcClusterSmartStatsSummaryRequestType {cluster, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryVcClusterSmartStatsSummary", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Querying version information of vSAN health service installed on all the hosts
    /// in the cluster requested and vCenter Server.
    /// 
    /// It will verify the consistency of the version numbers.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The vSAN health service version
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vc_cluster_query_verify_health_system_versions(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::VsanClusterHealthSystemVersionResult> {
        let input = VsanVcClusterQueryVerifyHealthSystemVersionsRequestType {cluster, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanVcClusterQueryVerifyHealthSystemVersions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Get the periodical vSAN health check interval (in minutes) for the cluster.
    /// 
    /// The periodical vSAN health check will perform a full vSAN health
    /// check at the fix interval which can be used in refreshing the cache as well as
    /// triggering the health event and alarm as the supplement of host event based auto
    /// cache refresh mechanism
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
    /// The periodical health check interval in minutes. Return 0 indicates
    /// the scheduled health check is disabled
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_health_query_vsan_cluster_health_check_interval(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<i32> {
        let input = VsanHealthQueryVsanClusterHealthCheckIntervalRequestType {cluster, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthQueryVsanClusterHealthCheckInterval", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Get the vSAN cluster health system configuration for the cluster
    /// Note when the API is called directly from an ESXi host, the configuration
    /// options may differ from that of from the vCenter.
    /// 
    /// It returns only ones that
    /// may be meaningful to the ESXi host and the health service itself.
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
    /// The vSAN cluster health service configuration
    pub async fn vsan_health_query_vsan_cluster_health_config(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::VsanClusterHealthConfigs> {
        let input = VsanHealthQueryVsanClusterHealthConfigRequestType {cluster, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthQueryVsanClusterHealthConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query the extended attributes for vSAN cluster objects.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### uuids
    /// List of object UUIDs.
    ///
    /// ## Returns:
    ///
    /// a list of vSAN cluster object extended attributes of given objects
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_vc_cluster_obj_ext_attrs(&self, cluster: &crate::types::structs::ManagedObjectReference, uuids: &[String]) -> Result<Option<Vec<crate::types::structs::VsanClusterObjectExtAttrs>>> {
        let input = VsanQueryVcClusterObjExtAttrsRequestType {cluster, uuids, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanQueryVcClusterObjExtAttrs", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Get the vSAN proxy configuration.
    /// 
    /// Return the proxy configuration for vSAN. The method gets the proxy
    /// configured via UI settings or vsan config file. If no proxy is configured,
    /// the api tries to check if any system level proxy is set.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// The vSAN proxy configuration
    pub async fn vsan_health_query_vsan_proxy_config(&self) -> Result<crate::types::structs::VsanClusterTelemetryProxyConfig> {
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthQueryVsanProxyConfig", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Proactive rebalance the vSAN objects on the cluster hosts based
    /// on the vSAN disks usage when the disks are in imbalance status.
    /// 
    /// The typical use case will be adding new host/disk into the vSAN
    /// cluster. Users should be aware that rebalancing causes additional
    /// background IO requires for data movements
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vCenter cluster
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_hosts
    /// Leave this unset
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// task vCenter Task to track the rebalance process on the hosts. The task
    /// will be completed if all of hosts complete the rebalance
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_rebalance_cluster(&self, cluster: &crate::types::structs::ManagedObjectReference, target_hosts: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanRebalanceClusterRequestType {cluster, target_hosts, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanRebalanceCluster", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Repair the absent or degraded vSAN object immediately under
    /// the cluster.
    /// 
    /// The task triggered by this API will be finished after
    /// putting all of the required objects into repairing queue, but it doesn't
    /// mean the repair process is done or successful. The completing time for
    /// repairing all of objects is unpredictable and depends on vSAN
    /// backend.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The VC cluster.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### uuids
    /// The object UUIDs need to be repaired.
    /// Unset to fix all of objects under the cluster
    ///
    /// ## Returns:
    ///
    /// task VC Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_health_repair_cluster_objects_immediate(&self, cluster: &crate::types::structs::ManagedObjectReference, uuids: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanHealthRepairClusterObjectsImmediateRequestType {cluster, uuids, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthRepairClusterObjectsImmediate", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Deprecated as of vSphere API 6.7.
    /// 
    /// Run VMDK load test for the given duration second with the IO workload test
    /// parameter specifying in spec.
    /// 
    /// The test includes three steps. Firstly,
    /// it will create VMDKs on each of cluster host, and then run the IO work test,
    /// at last, it will clean up the all of created VMDKs on each of host.Each of step
    /// can be run separately by specifying the parameter action. The default action
    /// "fullrun" will run all of three steps together
    /// Note: if this API is called directly on ESXi host, for any element of specs'
    /// vmdkCreateSpec.profile should be either unset or empty as SPBM is not available
    /// on ESXi host for profile conversion.
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
    /// ### runname
    /// The name for this test.
    ///
    /// ### duration_sec
    /// The duration time for each of VMDK load test.
    ///
    /// ### specs
    /// -
    ///
    /// ### action
    /// The possible actions are "prepare", "run" and "cleanup"
    /// and "fullrun". Default is "fullrun".
    ///
    /// ## Returns:
    ///
    /// task VC Task.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vc_cluster_run_vmdk_load_test(&self, cluster: &crate::types::structs::ManagedObjectReference, runname: &str, duration_sec: Option<i32>, specs: Option<&[crate::types::structs::VsanVmdkLoadTestSpec]>, action: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanVcClusterRunVmdkLoadTestRequestType {cluster, runname, duration_sec, specs, action, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanVcClusterRunVmdkLoadTest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Collecting vSAN telemetry for the given vCenter cluster and send to VMware
    /// phonehome server.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vCenter cluster for collecting vSAN telemetry data
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***VsanFault***: 
    /// 
    /// ***NotSupported***:
    pub async fn vsan_health_send_vsan_telemetry(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<()> {
        let input = VsanHealthSendVsanTelemetryRequestType {cluster, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthSendVsanTelemetry", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Set the vSAN health log level which will be taken effect immediately.
    /// 
    /// This setting will be refreshed and reverted to default INFO log level after
    /// vCenter system or health service is restarted. Setting the default log level
    /// as INFO in production because higher levels lead to too much log volume
    /// (reduced performance and too quick log rotation), while lower levels will not
    /// have enough information for troubleshooting by VMware support.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### level
    /// The log level to set. Unset will set the log level to the
    /// default log level as INFO. See all log levels through
    /// *VsanHealthLogLevelEnum_enum*
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_health_set_log_level(&self, level: Option<&str>) -> Result<()> {
        let input = VsanHealthSetLogLevelRequestType {level, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthSetLogLevel", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Set the periodical vSAN health check interval (in minutes) for the cluster.
    /// 
    /// The periodical vSAN health check will perform a full vSAN health
    /// check at the fix interval which can be used in refreshing the cache as well as
    /// triggering the health event and alarm as the supplement of host event based auto
    /// cache refresh mechanism.
    /// Smaller value will perform the vSAN health more frequently to keep the cache
    /// to be more up-to-date but will increase the system overhead and vice versa.
    /// The default value is 1 hour.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vCenter cluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### vsan_cluster_health_check_interval
    /// The vSAN cluster periodical health check
    /// interval in minutes. The value 0 will disable the
    /// periodical health check
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***VsanFault***: 
    /// 
    /// ***NotSupported***:
    pub async fn vsan_health_set_vsan_cluster_health_check_interval(&self, cluster: &crate::types::structs::ManagedObjectReference, vsan_cluster_health_check_interval: i32) -> Result<()> {
        let input = VsanHealthSetVsanClusterHealthCheckIntervalRequestType {cluster, vsan_cluster_health_check_interval, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthSetVsanClusterHealthCheckInterval", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Set silent health check list of the cluster.
    /// 
    /// Specify \[testId/groupId\] for 'addSilentChecks' to add add health
    /// checks to silent list.
    /// Specify \[testId/groupId\] for 'removeSilentChecks' to remove health
    /// checks from silent list.
    /// To restore the whole silent list, set 'removeSilentChecks' = \['all'\].
    /// After the silent health check list is updated, it is recommended to
    /// refresh the vSAN cluster health summary to get the updated result.
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
    /// - This API only supported on vCenter.
    ///   
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vCenter cluster
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
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***NotSupported***: 
    /// 
    /// ***VsanFault***:
    pub async fn vsan_health_set_vsan_cluster_silent_checks(&self, cluster: &crate::types::structs::ManagedObjectReference, add_silent_checks: Option<&[String]>, remove_silent_checks: Option<&[String]>) -> Result<bool> {
        let input = VsanHealthSetVsanClusterSilentChecksRequestType {cluster, add_silent_checks, remove_silent_checks, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthSetVsanClusterSilentChecks", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Set the vSAN clsuter CEIP (Customer Experience Improvement Program)
    /// configuration.
    /// 
    /// The configuration includes the interval time for the periodical CEIP telemetry data
    /// collector and the network proxy for sending vSAN telemetry data as well as
    /// HCL updating and uploading vSAN support bundle. Notice this proxy will be shared
    /// across all of vCenter clusters.
    /// The default interval is 1 week.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vCenter cluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### vsan_cluster_health_config
    /// The vSAN cluster CEIP configuration
    pub async fn vsan_health_set_vsan_cluster_telemetry_config(&self, cluster: &crate::types::structs::ManagedObjectReference, vsan_cluster_health_config: &crate::types::structs::VsanClusterHealthConfigs) -> Result<()> {
        let input = VsanHealthSetVsanClusterTelemetryConfigRequestType {cluster, vsan_cluster_health_config, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthSetVsanClusterTelemetryConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// This API can help to build up the mapping from the hardware infomation to
    /// the vSAN VCG entry.
    /// 
    /// The hardware information in the spec can be fetched by
    /// *VsanVcClusterHealthSystem.VsanVcClusterGetHclInfo*.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Refer to *VsanHwToVcgInfoMappingSpec*
    ///
    /// ## Returns:
    ///
    /// True to indicate the VCG product IDs are set successfully.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***VsanFault***: If an unexpected error happened during the process.
    pub async fn set_vsan_vcg_mapping_for_hw_devices(&self, spec: &crate::types::structs::VsanHwToVcgInfoMappingSpec) -> Result<bool> {
        let input = SetVsanVcgMappingForHwDevicesRequestType {spec, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/SetVsanVcgMappingForHwDevices", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Stop proactive rebalance the vSAN objects on the cluster hosts
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vCenter cluster
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_hosts
    /// Leave this unset
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// task vCenter Task to track the rebalance stop process. The task will
    /// be completed if all of hosts have stopped the rebalance
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_stop_rebalance_cluster(&self, cluster: &crate::types::structs::ManagedObjectReference, target_hosts: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanStopRebalanceClusterRequestType {cluster, target_hosts, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanStopRebalanceCluster", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Test the vSAN proxy configuration, which is used for downloading,
    /// HCL DB, sending vSAN telemetry data and uploading support bundle.
    /// 
    /// Return true if the proxy works fine. This API will take several of seconds
    /// to return if given wrong proxy.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### proxy_config
    /// The proxy configuration
    ///
    /// ## Returns:
    ///
    /// True indicates the proxy test is successful
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***VsanFault***: 
    /// 
    /// ***NotSupported***:
    pub async fn vsan_health_test_vsan_cluster_telemetry_proxy(&self, proxy_config: &crate::types::structs::VsanClusterTelemetryProxyConfig) -> Result<bool> {
        let input = VsanHealthTestVsanClusterTelemetryProxyRequestType {proxy_config, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthTestVsanClusterTelemetryProxy", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Update the cluster datastore default policy recommendation for this cluster.
    /// 
    /// Setting this property will result into a task which updates the SPBM policy.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The VC cluster.
    /// 
    /// ***Required privileges:*** Global.Settings
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// task VC Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: when specified cluster does not exist
    /// 
    /// ***VsanFault***: If unexpected error happened during update
    /// policy, such as spbm not available
    pub async fn vsan_health_update_default_ds_policy_recommendation(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanHealthUpdateDefaultDsPolicyRecommendationRequestType {cluster, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanHealthUpdateDefaultDSPolicyRecommendation", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Downloads the HCL database from the VMware official website
    /// http://partnerweb.vmware.com/service/vsan/all.json.
    /// 
    /// The DB will replace any existing DB, even if the existing DB
    /// may appear newer. Caller can supply a custom URL. If not provided,
    /// DB is downloaded from its standard location on vmware.com.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### url
    /// The URL for downloading the HCL DB. Unset to use the official
    /// standard location on vmware.com
    ///
    /// ## Returns:
    ///
    /// Always return True, or throw exception. False is never returned.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***VsanFault***: 
    /// 
    /// ***NotSupported***:
    pub async fn vsan_vc_update_hcl_db_from_web(&self, url: Option<&str>) -> Result<bool> {
        let input = VsanVcUpdateHclDbFromWebRequestType {url, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanVcUpdateHclDbFromWeb", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Upload a DB file in JSON format.
    /// 
    /// The DB will replace any existing DB, even
    /// if the existing DB may appear newer.
    /// The API at host level is for internal use.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### db
    /// The new DB, in base64 encoded, gzipped JSON format
    ///
    /// ## Returns:
    ///
    /// Always return True, or throw exception. False is never returned.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: 
    /// 
    /// ***VsanFault***:
    pub async fn vsan_vc_upload_hcl_db(&self, db: &str) -> Result<bool> {
        let input = VsanVcUploadHclDbRequestType {db, };
        let path = format!("/vsan/VsanVcClusterHealthSystem/{moId}/VsanVcUploadHclDb", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanAttachVsanSupportBundleToSrRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "srNumber")]
    sr_number: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanDownloadAndInstallVendorToolRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanDownloadHclFileRequestType<'a> {
    #[serde(rename = "sha1sums")]
    sha_1_sums: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanGetHclConstraintsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    release: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVcClusterGetHclInfoRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeHostsResult")]
    include_hosts_result: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeVendorInfo")]
    include_vendor_info: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "esxRelease")]
    esx_release: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "querySpec")]
    query_spec: Option<&'a crate::types::structs::VsanHclQuerySpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanGetReleaseRecommendationRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    minor: &'a [String],
    major: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanGetDiskHclConstraintsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    release: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "diskModels")]
    disk_models: Option<&'a [crate::types::structs::VsanDiskModelInfo]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanGetHclInfoForEligibleDisksRequestType<'a> {
    #[serde(rename = "querySpec")]
    query_spec: &'a crate::types::structs::VsanHclQuerySpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHealthGetVsanClusterSilentChecksRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHealthIsRebalanceRunningRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetHosts")]
    target_hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPurgeHclFilesRequestType<'a> {
    #[serde(rename = "sha1sums")]
    sha_1_sums: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryAttachToSrHistoryRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "taskId")]
    task_id: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryVcClusterCreateVmHealthHistoryTestRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datastore: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryVcClusterCreateVmHealthTestRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    timeout: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datastore: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryVcClusterHealthSummaryRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vmCreateTimeout")]
    vm_create_timeout: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objUuids")]
    obj_uuids: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeObjUuids")]
    include_obj_uuids: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    fields: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fetchFromCache")]
    fetch_from_cache: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    perspective: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a crate::types::structs::VsanClusterHealthQuerySpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryVcClusterHealthSummaryTaskRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeDataProtectionHealth")]
    include_data_protection_health: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeOnlineHealth")]
    include_online_health: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryClusterHistoricalHealthRequestType<'a> {
    spec: &'a crate::types::structs::VsanHistoricalHealthQuerySpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryVcClusterNetworkPerfHistoryTestRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a crate::types::structs::VsanClusterNetworkPerfTaskSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryVcClusterNetworkPerfTaskRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a crate::types::structs::VsanClusterNetworkPerfTaskSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryVcClusterNetworkPerfTestRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    multicast: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "durationSec")]
    duration_sec: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryVcClusterVmdkLoadHistoryTestRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "taskId")]
    task_id: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanClusterQueryFileServiceHealthSummaryRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeFileServerHealth")]
    include_file_server_health: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeFileShareHealth")]
    include_file_share_health: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryVcClusterSmartStatsSummaryRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVcClusterQueryVerifyHealthSystemVersionsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHealthQueryVsanClusterHealthCheckIntervalRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHealthQueryVsanClusterHealthConfigRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryVcClusterObjExtAttrsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    uuids: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanRebalanceClusterRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetHosts")]
    target_hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHealthRepairClusterObjectsImmediateRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    uuids: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVcClusterRunVmdkLoadTestRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    runname: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "durationSec")]
    duration_sec: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    specs: Option<&'a [crate::types::structs::VsanVmdkLoadTestSpec]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    action: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHealthSendVsanTelemetryRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHealthSetLogLevelRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    level: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHealthSetVsanClusterHealthCheckIntervalRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "vsanClusterHealthCheckInterval")]
    vsan_cluster_health_check_interval: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHealthSetVsanClusterSilentChecksRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "addSilentChecks")]
    add_silent_checks: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "removeSilentChecks")]
    remove_silent_checks: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHealthSetVsanClusterTelemetryConfigRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "vsanClusterHealthConfig")]
    vsan_cluster_health_config: &'a crate::types::structs::VsanClusterHealthConfigs,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetVsanVcgMappingForHwDevicesRequestType<'a> {
    spec: &'a crate::types::structs::VsanHwToVcgInfoMappingSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanStopRebalanceClusterRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetHosts")]
    target_hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanHealthTestVsanClusterTelemetryProxyRequestType<'a> {
    #[serde(rename = "proxyConfig")]
    proxy_config: &'a crate::types::structs::VsanClusterTelemetryProxyConfig,
}
#[derive(serde::Serialize)]
#[serde(rename = "VsanHealthUpdateDefaultDSPolicyRecommendationRequestType", tag = "_typeName")]
struct VsanHealthUpdateDefaultDsPolicyRecommendationRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVcUpdateHclDbFromWebRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVcUploadHclDbRequestType<'a> {
    db: &'a str,
}
