use std::sync::Arc;
use crate::core::client::{Client, Result};
/// This managed object type provides the service interface for obtaining
/// statistical data about various aspects of vSAN performance, as generated
/// and maintained by the vSAN performance service of the cluster.
/// 
/// It also offers
/// methods to enable/disable, configure and perform other maintenance tasks
/// about the vSAN performance service. It is available on both vCenter as well
/// as ESXi under the vSAN extension endpoint. On both systems a singleton object
/// is registered under the Managed Object ID 'vsan-performance-manager'.
///   
/// All the vSAN hosts belongs to one of the following two type in performance service
/// perspective.
///   
/// Stats Master node: see *VsanPerfNodeInformation*
///   
/// Agent node: all other nodes except the master node, which collect its performance
/// statistics when receive the request from master then send it back.
#[derive(Clone)]
pub struct VsanPerformanceManager {
    client: Arc<Client>,
    mo_id: String,
}
impl VsanPerformanceManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Create the vSAN object/directory containing the vSAN Perf Stats DB.
    /// 
    /// Creation of the object also starts the collection of statistics as a side effect,
    /// i.e., it effectively enables the vSAN performance service.
    /// Profile can be 3 formats:
    /// - VirtualMachineEmptyProfileSpec means to use the empty vSAN policy. This is not the
    ///   default policy, but a policy where all fields have default values.
    /// - VirtualMachineDefinedProfileSpec where profileId is set, in which case this profileId
    ///   will be looked up in SPBM for the detailed policy information.
    /// - VirtualMachineDefinedProfileSpec where profileId is an empty string and instead
    ///   the profileData is set for extensionKey 'com.vmware.vim.sps'. In this case the
    ///   objectData field can be either the vSAN expression format, or a SPBM XML string.
    ///   
    ///   
    /// If no profile is supplied, and the call is executed against vCenter, then SPBM will
    /// be consulted for the vSAN datastore's default profile.
    ///   
    /// Profile is ignored if executed against ESXi host.
    /// - If the vSAN object is already exist, return directly.
    /// - If vSAN is disabled, DestinationVsanDisabled exception will be raised.
    /// - If SPBM needs to be contacted, but SPBM is not available, RuntimeFault exception will be raised.
    /// - If the profileId can not be resolved with SPBM, InvalidArgument exception will be raised.
    /// - If objectData was provided but is neither of the two supported formats, InvalidArgument exception
    ///   will be raised.
    /// - If the statsDB object can not be found, FileNotFound exception will be raised.
    /// - If the statsDB object failed to set the policy, e.g. because it is not accessible,
    ///   FileNotWritable exception will be raised.
    /// - If called against VC, but no ESX host could be contacted to perform the operation
    ///   NotFound exception will be raised.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### profile
    /// Profile to be used for the stats object, see above.
    ///
    /// ## Returns:
    ///
    /// mounted path of the vSAN stats object (using "/" as path separator)
    /// i.e. /vmfs/volumes/vsan:525218c52dce3d62-e51a774ec7aef712/
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: if the pre-check tests failed.
    /// 
    /// ***FileAlreadyExists***: if the stats object already exists.
    /// 
    /// ***CannotCreateFile***: if it cannot complete file creation operation.
    /// 
    /// ***NotFound***: if no ESXi host could be contacted to perform the operation
    /// when called against vCenter.
    pub async fn vsan_perf_create_stats_object(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, profile: Option<&dyn crate::types::traits::VirtualMachineProfileSpecTrait>) -> Result<String> {
        let input = VsanPerfCreateStatsObjectRequestType {cluster, profile, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfCreateStatsObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// The asynchronous API of CreateStatsObject.
    /// 
    /// The stats obj is created in
    /// in background, with a task returned. This method is only supported on
    /// vCenter.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### profile
    /// Profile to be used for the stats object, see above
    ///
    /// ## Returns:
    ///
    /// vim task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: if the caller doesn't have the required privilege, or the
    /// cluster has no hosts.
    pub async fn vsan_perf_create_stats_object_task(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, profile: Option<&dyn crate::types::traits::VirtualMachineProfileSpecTrait>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanPerfCreateStatsObjectTaskRequestType {cluster, profile, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfCreateStatsObjectTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Delete vSAN object/directory containing the vSAN Perf Stats DB.
    /// 
    /// This method is only supported on ESXi host.
    /// Note: this will destroy all history and shut down the vSAN performance
    /// service.
    /// If the vSAN object doesn't exist, FileNotWritable exception will be raised.
    /// If vSAN is disabled, DestinationVsanDisabled exception will be raised.
    /// The operation can only be performed by masters, so VsanNodeNotMaster is raised
    /// when the node is not Stats master.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// True on success
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: if the pre-check tests failed, or the host in the states
    /// that do not allow objects deletion (i.e. maintenance mode
    /// with data migration mode: ensure accessibility).
    /// 
    /// ***CannotCreateFile***: if it cannot complete file creation operation.
    /// 
    /// ***NotFound***: if no ESXi host could be contacted to perform the operation
    /// when called against vCenter.
    pub async fn vsan_perf_delete_stats_object(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<bool> {
        let input = VsanPerfDeleteStatsObjectRequestType {cluster, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfDeleteStatsObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// The asynchronous API of DeleteStatsObject.
    /// 
    /// The statistics object is created
    /// in background, with a task returned.
    /// This method is only supported on vCenter.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// vim task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: if the caller doesn't have the required privilege, or the
    /// cluster has no hosts.
    pub async fn vsan_perf_delete_stats_object_task(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanPerfDeleteStatsObjectTaskRequestType {cluster, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfDeleteStatsObjectTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Delete saved time range in performance service.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### name
    /// Delete by the name of *VsanPerfTimeRange*
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_perf_delete_time_range(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, name: &str) -> Result<()> {
        let input = VsanPerfDeleteTimeRangeRequestType {cluster, name, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfDeleteTimeRange", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Get supported aggregated entity types for front end data-driven
    /// reporting of diagnostic exceptions which return aggregated data.
    /// 
    /// This API can be used to build performance graphs of aggregated data in a
    /// dynamic way.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_perf_get_aggregated_entity_types(&self) -> Result<Option<Vec<crate::types::structs::VsanPerfEntityType>>> {
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfGetAggregatedEntityTypes", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
    /// Get supported performance exceptions for front end data-driven
    /// performance exception reporting
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn vsan_perf_get_supported_diagnostic_exceptions(&self) -> Result<Option<Vec<crate::types::structs::VsanPerfDiagnosticException>>> {
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfGetSupportedDiagnosticExceptions", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
    /// This API is used to build performance graphs in a data-driven and dynamic way.
    /// 
    /// Before querying stats, you need to know the entity type for specifying entity ID of the
    /// query spec. If you want to query specific metrics, you need to know what metrics are
    /// supported by that type of entities. And you may want to know how to organize the metrics
    /// into different graphs. The returned list of
    /// *VsanPerfEntityType* data model
    /// tells you all the information you needed for above questions.
    ///   
    /// Each *VsanPerfEntityType* object describes supported
    /// metrics grouped by graphs
    /// for a type of entities like VMs. The name attribute of
    /// *VsanPerfEntityType*
    /// is the entity type ID used as part of the entity ID in the query spec.
    /// See *VsanPerfQuerySpec.entityRefId*.
    ///   
    /// The model of vim.cluster.VsanPerfEntityType defines a list of performance graphs
    /// (*VsanPerfGraph*). And
    /// *VsanPerfGraph* defines a list of metrics
    /// (*VsanPerfMetricId*). This tells you how to organized
    /// different metrics to a graph and supported metrics of a type of entities.
    /// Then front-end/client can compose
    /// query specs using the information from the VsanPerfEntityType list and entity instance
    /// UUIDs to retrieved wanted performance statistics.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn vsan_perf_get_supported_entity_types(&self) -> Result<Option<Vec<crate::types::structs::VsanPerfEntityType>>> {
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfGetSupportedEntityTypes", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
    /// Returns the diagnosis result from the in memory cache for the supplied
    /// task.
    /// 
    /// The task should have been returned by VsanPerfDiagnoseTask. This API
    /// is available only in the vCenter, it is not available at the end-host.
    ///
    /// ## Parameters:
    ///
    /// ### task
    /// Task returned by VsanPerfDiagnoseTask
    /// 
    /// Refers instance of *Task*.
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// A list of performance issues.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: if the caller doesn't have the required privilege, or the
    /// cluster has no hosts.
    /// 
    /// ***NotFound***: If no result is found in the cache for the specified task
    pub async fn get_vsan_perf_diagnosis_result(&self, task: &crate::types::structs::ManagedObjectReference, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::VsanPerfDiagnosticResult>>> {
        let input = GetVsanPerfDiagnosisResultRequestType {task, cluster, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/GetVsanPerfDiagnosisResult", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// This API shall not be used to query the health status for vSAN performance service.
    /// 
    /// Consider this API as deprecated. Use *VsanVcClusterHealthSystem.VsanQueryVcClusterHealthSummary*
    /// instead.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster for which to compute health for.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// vim.cluster.VsanClusterHealthGroup\[\] A list of health groups.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if no ESXi host could be contacted to perform the operation
    /// when called against vCenter.
    pub async fn vsan_perf_query_cluster_health(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Vec<crate::types::structs::VsanClusterHealthGroup>> {
        let input = VsanPerfQueryClusterHealthRequestType {cluster, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfQueryClusterHealth", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query performance service related information about the node(s).
    /// 
    /// Always returns a list, but when run against the host the list is guaranteed
    /// to have length=1. If run against vCenter, information about all hosts in the
    /// cluster is retrieved. If information of one host can not be retrieved, there
    /// are 2 situations:
    /// 1. If the host is connected: it will throw
    ///    "invalid Request", "method fault",
    ///    "vsan fault" or other run time exception message.
    /// 2. If the host is not connected, it will throw
    ///    "host is not in connected status"
    ///    message.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_perf_query_node_information(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::VsanPerfNodeInformation>>> {
        let input = VsanPerfQueryNodeInformationRequestType {cluster, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfQueryNodeInformation", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Query all remote server clusters ever mounted from perf database by the
    /// specified query specification and return their UUIDs.
    /// 
    /// This API is available
    /// on VC and stats master node.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Local vSAN cluster. This parameter will be ignored if the API
    /// is called against host.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### query_spec
    /// Specification for the query operation. If the parameter
    /// is not specified all available remote clusters will be
    /// returned.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If any argument passed to the function is not
    /// specified correctly.
    /// 
    /// ***VsanFault***: If any other unexpected fault is encountered.
    pub async fn query_remote_server_clusters(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, query_spec: Option<&crate::types::structs::VsanRemoteClusterQuerySpec>) -> Result<Vec<String>> {
        let input = QueryRemoteServerClustersRequestType {cluster, query_spec, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/QueryRemoteServerClusters", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Get information about the vSAN object/directory containing the vSAN Perf Stats DB.
    /// 
    /// If the statsDB object can not be found, FileNotFound exception will be raised.
    /// If the statsDB object failed to read the policy, e.g. because it is not accessible,
    /// FileNotWritable exception will be raised.
    /// If called against vCenter, but no ESXi host could be contacted to perform the
    /// operation NotFound exception will be raised.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// Object information structure
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if no ESXi host could be contacted to perform the operation
    /// when called against vCenter.
    pub async fn vsan_perf_query_stats_object_information(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::VsanObjectInformation> {
        let input = VsanPerfQueryStatsObjectInformationRequestType {cluster, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfQueryStatsObjectInformation", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query saved time ranges in performance service.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### query_spec
    /// Specify the name and time boundaries. See details
    /// in *VsanPerfTimeRangeQuerySpec*
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_perf_query_time_ranges(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, query_spec: &crate::types::structs::VsanPerfTimeRangeQuerySpec) -> Result<Option<Vec<crate::types::structs::VsanPerfTimeRange>>> {
        let input = VsanPerfQueryTimeRangesRequestType {cluster, query_spec, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfQueryTimeRanges", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Retrieves the performance metrics for the specified
    /// vSAN entity (or entities) based on the properties specified in
    /// the VsanPerfQuerySpec data object
    /// The supported entity types are listed as bellow.
    /// - 'cluster-domclient'
    /// - 'cluster-domcompmgr'
    /// - 'host-domclient'
    /// - 'host-domcompmgr'
    /// - 'cache-disk'
    /// - 'capacity-disk'
    /// - 'disk-group'
    /// - 'vscsi'
    /// - 'virtual-machine'
    /// - 'virtual-disk'
    /// - 'vsan-host-net'
    /// - 'vsan-vnic-net'
    /// - 'vsan-pnic-net'
    /// - 'lsom-world-cpu'
    /// - 'dom-world-cpu'
    /// - 'host-cpu'
    /// - 'nic-world-cpu'
    /// - 'vsan-cpu'
    /// - 'vsan-memory'
    /// - 'rdt-net'
    ///   
    /// The below entity types are used for vSAN ESA related metrics.
    /// - 'vsan-esa-disk-layer'
    /// - 'vsan-esa-disk-scsifw'
    /// - 'zdom-vtx'
    ///   
    /// The below entity types are used for HCI mesh related metrics.
    /// - 'cluster-remotedomclient'
    /// - 'computeCluster-remotedomclient'
    ///   
    /// The below entity types are used for vSAN direct related metrics.
    /// - 'vsan-direct-cluster'
    /// - 'vsan-direct-host'
    ///   
    /// The below entity types are used for PMem related metrics.
    /// - 'host-pmem'
    /// - 'cluster-pmem'
    ///   
    /// The below entity types are used for vSAN iSCSI service related metrics.
    /// The metrics are only collected when vSAN iSCSI service is enabled.
    /// - 'vsan-iscsi-host'
    /// - 'vsan-iscsi-lun'
    /// - 'vsan-iscsi-target'
    ///   
    /// The below entity type is used for vSAN datastore capacity historical data.
    /// - 'vsan-cluster-capacity'
    ///   
    /// The below entity type is used for vSAN file service related metrics.
    /// The metrics are only collected when vSAN file service is enabled.
    /// - 'vsan-file-service'
    ///   
    /// The below entity type is used for IOInsight related metrics.
    /// The metrics are only collected when the IOInsight instance is running.
    /// - 'ioinsight'
    /// - 'ioinsight-histogram'
    ///   
    /// To identify an entity in vSAN performance query spec, a vSAN performance
    /// entity reference is used.  
    /// An vSAN performance entity is in this format
    /// &lt;entity-type&gt;:&lt;entity-uuid&gt;.  
    /// Below are the examples:
    /// <table cellspacing="0">
    /// <tr>
    /// <th>Entity Type</th>
    /// <th>Entity ID format</th>
    /// <th>Example</th>
    /// <th>Notes</th>
    /// <tr>
    /// <td>cluster-domclient</td>
    /// <td>&lt;cluster-UUID&gt;</td>
    /// <td>'cluster-domclient:52c89b61-f818-e495-af20-816d24c850b8'</td>
    /// <td>The UUID is represented by the associated cluster UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>cluster-domcompmgr</td>
    /// <td>&lt;cluster-UUID&gt;</td>
    /// <td>'cluster-domcompmgr:52c89b61-f818-e495-af20-816d24c850b8'</td>
    /// <td>The UUID is represented by the associated cluster UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>host-domclient</td>
    /// <td>&lt;host-UUID&gt;</td>
    /// <td>'host-domclient:588b2225-c58c-8365-c47b-02001065be12'</td>
    /// <td>The UUID is represented by the associated ESXi host UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>host-domcompmgr</td>
    /// <td>&lt;host-UUID&gt;</td>
    /// <td>'host-domcompmgr:588b2225-c58c-8365-c47b-02001065be12'</td>
    /// <td>The UUID is represented by the associated ESXi host UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>cache-disk</td>
    /// <td> &lt;CacheDisk-UUID&gt;</td>
    /// <td>'cache-disk:55c98c4d-41f0-6ff7-2784-0200103eb5e1'</td>
    /// <td>The UUID is represented by the associated cache disk UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>capacity-disk</td>
    /// <td> &lt;CapacityDisk-UUID&gt;</td>
    /// <td>'capacity-disk:55c98c4d-41f0-6ff7-2784-0200103eb5e1'</td>
    /// <td>The UUID is represented by the associated capacity disk UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>disk-group</td>
    /// <td> &lt;CacheDisk-UUID&gt;</td>
    /// <td>'disk-group:55c98c4d-41f0-6ff7-2784-0200103eb5e1'</td>
    /// <td>The UUID is represented by the associated disk group UUID, which is the same as cache disk UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>vscsi</td>
    /// <td> &lt;VM-instance-UUID&gt;|&lt;VSCSI-name&gt;</td>
    /// <td>'vscsi:55c98c4d-41f0-6ff7-2784-0200103eb5e1|vscsi0:1'</td>
    /// <td>The UUID is represented by the associated VM instance UUID with its VSCSI name. Virtual disk IOPS limit statistics are associated with 'virtual-disk'.</td>
    /// </tr>
    /// <tr>
    /// <td>virtual-machine</td>
    /// <td> &lt;VM-instance-UUID&gt;</td>
    /// <td>'virtual-machine:55c98c4d-41f0-6ff7-2784-0200103eb5e1'</td>
    /// <td>The UUID is represented by the associated VM instance UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>virtual-disk</td>
    /// <td>&lt;VM-dir-uuid&gt;/&lt;VMDK-file-name&gt;</td>
    /// <td>'virtual-disk:a2a04b57-e0e6-502b-e4a0-0200073bd703/iops-160-10.160.109.28-1\_1.vmdk'</td>
    /// <td>The UUID is represented by the VMDK file path, which can be retrieved using the vSphere API.</td>
    /// </tr>
    /// <tr>
    /// <td>vsan-vnic-net</td>
    /// <td> &lt;host-UUID&gt;|&lt;stack-name&gt;|&lt;vnic-name&gt;</td>
    /// <td>'vsan-vnic-net:588b2225-c58c-8365-c47b-02001065be12|defaultTcpipStack|vmknic0'</td>
    /// <td>The UUID is represented by the associated ESXi host UUID with its stack name and vNIC name.</td>
    /// </tr>
    /// <tr>
    /// <td>vsan-pnic-net</td>
    /// <td>&lt;host-UUID&gt;|&lt;pnic-name&gt;</td>
    /// <td>'vsan-pnic-net:588b2225-c58c-8365-c47b-02001065be12|vmnic0'</td>
    /// <td>This UUID is represented by the associated ESXi host UUID with the pNIC name.</td>
    /// </tr>
    /// <tr>
    /// <td>lsom-world-cpu</td>
    /// <td>&lt;host-UUID&gt;|&lt;world-name&gt;|&lt;world-id&gt;</td>
    /// <td>'lsom-world-cpu:5ad47458-3bca-870a-602c-02002c89fe44|VSAN\_0x43050bf3f7f8\_LSOMLLOG|1001393599'</td>
    /// <td>The UUID is represented by the associated ESXi host UUID with the LSOM world name and its world ID.</td>
    /// </tr>
    /// <tr>
    /// <td>dom-world-cpu</td>
    /// <td>&lt;host-UUID&gt;|&lt;world-name&gt;|&lt;world-id&gt;</td>
    /// <td>'dom-world-cpu:5ad47458-3bca-870a-602c-02002c89fe44|VSAN\_0x430bfa348888\_CompServer|1001393015'</td>
    /// <td>The UUID is represented by the associated ESXi host UUID with its DOM world name and world ID.</td>
    /// </tr>
    /// <tr>
    /// <td>host-cpu</td>
    /// <td>&lt;host-UUID&gt;</td>
    /// <td>'host-cpu:5ad47458-3bca-870a-602c-02002c89fe44'</td>
    /// <td>The UUID is represented by the associated ESXi host UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>iscsi-target-alias</td>
    /// <td>&lt;iscsi-target-alias&gt;|&lt;lunid&gt;</td>
    /// <td>'iscsi-target-alias:iscsitargetaliasexample|1'</td>
    /// <td>The UUID is represented by iSCSI target alias with the LUN ID.</td>
    /// </tr>
    /// <tr>
    /// <td>vsan-cluster-capacity</td>
    /// <td>&lt;cluster-UUID&gt;</td>
    /// <td>'vsan-cluster-capacity:52c89b61-f818-e495-af20-816d24c850b8'</td>
    /// <td>The UUID is represented by the associated cluster UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>vsan-file-service</td>
    /// <td>&lt;domain-name&gt;|&lt;share-name&gt;</td>
    /// <td>'vsan-file-service:VSANFS-LOCAL.PRV|genericShare'</td>
    /// <td>The UUID is represented by the domain name and the share name.</td>
    /// </tr>
    /// <tr>
    /// <td>nic-world-cpu</td>
    /// <td>&lt;host-UUID&gt;|&lt;world-name&gt;</td>
    /// <td>'nic-world-cpu:5b9f8fd9-3687-7003-2f0b-02002fc9daae|vmnic0-pollWorld-0'</td>
    /// <td>The UUID is represented by the associated ESXi host UUID with its vNIC/pNIC world name.</td>
    /// </tr>
    /// <tr>
    /// <td>vsan-cpu</td>
    /// <td>&lt;host-UUID&gt;</td>
    /// <td>'vsan-cpu:5afa638a-f98a-c9f5-9f8a-0050569ee233'</td>
    /// <td>The UUID represents the associated ESXi host UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>vsan-memory</td>
    /// <td>&lt;host-UUID&gt;</td>
    /// <td>'vsan-memory:5afa638a-f98a-c9f5-9f8a-0050569ee233'</td>
    /// <td>The UUID represents the associated ESXi host UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>rdt-net</td>
    /// <td>&lt;host-UUID&gt;</td>
    /// <td>'rdt-net:5afa638a-f98a-c9f5-9f8a-0050569ee233'</td>
    /// <td>The UUID represents the associated ESXi host UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>vsan-esa-disk-layer</td>
    /// <td>&lt;host-UUID&gt;</td>
    /// <td>'vsan-esa-disk-layer:5afa638a-f98a-c9f5-9f8a-0050569ee233'</td>
    /// <td>The UUID represents the associated ESXi host UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>vsan-esa-disk-scsifw</td>
    /// <td>&lt;host-UUID&gt;</td>
    /// <td>'vsan-esa-disk-scsifw:5afa638a-f98a-c9f5-9f8a-0050569ee233'</td>
    /// <td>The UUID represents the associated ESXi host UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>zdom-vtx</td>
    /// <td>&lt;host-UUID&gt;</td>
    /// <td>'zdom-vtx:5afa638a-f98a-c9f5-9f8a-0050569ee233'</td>
    /// <td>The UUID represents the associated ESXi host UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>cluster-remotedomclient</td>
    /// <td>&lt;cluster-UUID&gt;</td>
    /// <td>'cluster-remotedomclient:52c89b61-f818-e495-af20-816d24c850b8'</td>
    /// <td>The UUID is represented by the associated cluster UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>computeCluster-remotedomclient</td>
    /// <td>&lt;cluster-UUID&gt;</td>
    /// <td>'computeCluster-remotedomclient:52c89b61-f818-e495-af20-816d24c850b8'</td>
    /// <td>The UUID is represented by the associated cluster UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>vsan-direct-cluster</td>
    /// <td>&lt;cluster-UUID&gt;</td>
    /// <td>'vsan-direct-cluster:52c89b61-f818-e495-af20-816d24c850b8'</td>
    /// <td>The UUID is represented by the associated cluster UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>vsan-direct-host</td>
    /// <td>&lt;host-UUID&gt;</td>
    /// <td>'vsan-direct-host:5afa638a-f98a-c9f5-9f8a-0050569ee233'</td>
    /// <td>The UUID represents the associated ESXi host UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>host-pmem</td>
    /// <td>&lt;host-UUID&gt;</td>
    /// <td>'host-pmem:5afa638a-f98a-c9f5-9f8a-0050569ee233'</td>
    /// <td>The UUID represents the associated ESXi host hardware UUID.</td>
    /// </tr>
    /// <tr>
    /// <td>cluster-pmem</td>
    /// <td>&lt;cluster-MOID&gt;</td>
    /// <td>'cluster-pmem:domain-c21'</td>
    /// <td>The MOID represents the associated managed object ID.</td>
    /// </tr>
    /// <tr>
    /// <td>ioinsight</td>
    /// <td> &lt;VM-instance-UUID&gt;|&lt;VSCSI-name&gt;</td>
    /// <td>'ioinsight:55c98c4d-41f0-6ff7-2784-0200103eb5e1|vscsi0:1'</td>
    /// <td>The UUID is represented by the associated VM instance UUID with its VSCSI name.</td>
    /// </tr>
    /// <tr>
    /// <td>ioinsight-histogram</td>
    /// <td> &lt;VM-instance-UUID&gt;|&lt;VSCSI-name&gt;</td>
    /// <td>'ioinsight-histogram:55c98c4d-41f0-6ff7-2784-0200103eb5e1|vscsi0:1'</td>
    /// <td>The UUID is represented by the associated VM instance UUID with its VSCSI name.</td>
    /// </tr>
    /// </table>
    /// 
    /// **Supported metrics for each entity type:**  
    /// <table cellspacing="0">
    /// <tr><th>Entity Type</th><th>Metrics (Labels)</th></tr>
    /// <tr>
    /// <td nowrap="1">'cluster-domclient'</td>
    /// <td>
    /// 'iopsRead', 'throughputRead', 'latencyAvgRead',
    /// 'iopsWrite', 'throughputWrite', 'latencyAvgWrite',
    /// 'congestion', 'oio'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'cluster-domcompmgr'</td>
    /// <td>
    /// 'iopsRead', 'throughputRead', 'latencyAvgRead',
    /// 'iopsWrite', 'throughputWrite', 'latencyAvgWrite',
    /// 'iopsRecWrite', 'throughputRecWrite', 'latencyAvgRecWrite',
    /// 'congestion', 'oio', 'iopsResyncRead', 'tputResyncRead',
    /// 'latAvgResyncRead'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'host-domclient'</td>
    /// <td>
    /// 'iopsRead', 'throughputRead', 'latencyAvgRead', 'readCount',
    /// 'iopsWrite', 'throughputWrite', 'latencyAvgWrite', 'writeCount',
    /// 'congestion', 'oio', 'clientCacheHits', 'clientCacheHitRate',
    /// 'iopsUnmap', 'throughputUnmap', 'latencyAvgUnmap'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'host-domcompmgr'</td>
    /// <td>
    /// 'iopsRead', 'throughputRead', 'latencyAvgRead', 'readCount',
    /// 'iopsWrite', 'throughputWrite', 'latencyAvgWrite', 'writeCount',
    /// 'iopsRecWrite', 'throughputRecWrite', 'latencyAvgRecWrite',
    /// 'recWriteCount', 'congestion', 'oio', 'iopsResyncRead',
    /// 'tputResyncRead', 'latAvgResyncRead', 'iopsUnmap', 'iopsRecUnmap',
    /// 'throughputUnmap', 'throughputRecUnmap', 'latencyAvgUnmap',
    /// 'latencyAvgRecUnmap'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'cache-disk'</td>
    /// <td>
    /// 'iopsDevRead', 'throughputDevRead', 'latencyDevRead',
    /// 'ioCountDevRead', 'iopsDevWrite', 'throughputDevWrite', 'latencyDevWrite',
    /// 'ioCountDevWrite', 'latencyDevDAvg', 'latencyDevGAvg'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'capacity-disk'</td>
    /// <td>
    /// 'iopsDevRead', 'throughputDevRead', 'latencyDevRead',
    /// 'ioCountDevRead', 'iopsDevWrite', 'throughputDevWrite', 'latencyDevWrite',
    /// 'ioCountDevWrite', 'latencyDevDAvg', 'latencyDevGAvg', 'iopsRead',
    /// 'latencyRead', 'ioCountRead', 'iopsWrite', 'latencyWrite', 'ioCountWrite',
    /// 'deleteCongestion'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'disk-group'</td>
    /// <td>
    /// 'iopsSched', 'latencySched', 'outstandingBytesSched',
    /// 'iopsSchedQueueRec', 'throughputSchedQueueRec','latencySchedQueueRec',
    /// 'iopsSchedQueueVM', 'throughputSchedQueueVM','latencySchedQueueVM',
    /// 'iopsSchedQueueMeta', 'throughputSchedQueueMeta','latencySchedQueueMeta',
    /// 'iopsDelayPctSched', 'latencyDelaySched',
    /// 'rcHitRate', 'wbFreePct', 'warEvictions', 'quotaEvictions',
    /// 'iopsRcRead', 'latencyRcRead', 'ioCountRcRead',
    /// 'iopsWbRead', 'latencyWbRead', 'ioCountWbRead',
    /// 'iopsRcWrite', 'latencyRcWrite', 'ioCountRcWrite',
    /// 'iopsWbWrite', 'latencyWbWrite', 'ioCountWbWrite',
    /// 'ssdBytesDrained', 'zeroBytesDrained',
    /// 'memCongestion', 'slabCongestion', 'ssdCongestion',
    /// 'iopsCongestion', 'logCongestion', 'compCongestion', 'iopsDirectSched',
    /// 'iopsRead', 'throughputRead', 'latencyAvgRead', 'readCount',
    /// 'iopsWrite', 'throughputWrite', 'latencyAvgWrite', 'writeCount',
    /// 'oioWrite', 'oioRecWrite', 'oioWriteSize', 'oioRecWriteSize',
    /// 'rcSize', 'wbSize', 'capacity', 'capacityUsed', 'capacityReserved',
    /// 'throughputSched', 'iopsResyncReadPolicy', 'iopsResyncReadDecom',
    /// 'iopsResyncReadRebalance', 'iopsResyncReadFixComp', 'iopsResyncWritePolicy',
    /// 'iopsResyncWriteDecom', 'iopsResyncWriteRebalance', 'iopsResyncWriteFixComp',
    /// 'tputResyncReadPolicy', 'tputResyncReadDecom', 'tputResyncReadRebalance',
    /// 'tputResyncReadFixComp', 'tputResyncWritePolicy', 'tputResyncWriteDecom',
    /// 'tputResyncWriteRebalance', 'tputResyncWriteFixComp', 'latResyncReadPolicy',
    /// 'latResyncReadDecom', 'latResyncReadRebalance', 'latResyncReadFixComp',
    /// 'latResyncWritePolicy', 'latResyncWriteDecom', 'latResyncWriteRebalance',
    /// 'latResyncWriteFixComp', 'bytesPerSecondBandwidth'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'virtual-machine'</td>
    /// <td>
    /// 'iopsRead', 'throughputRead', 'latencyRead', 'readCount',
    /// 'iopsWrite', 'throughputWrite', 'latencyWrite', 'writeCount'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vscsi'</td>
    /// <td>
    /// 'iopsRead', 'throughputRead', 'latencyRead', 'readCount',
    /// 'iopsWrite', 'throughputWrite', 'latencyWrite', 'writeCount'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'virtual-disk'</td>
    /// <td>
    /// 'iopsLimit', 'NIOPS', 'NIOPSDelayed'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-host-net'</td>
    /// <td>
    /// 'rxThroughput', 'rxPackets', 'rxPacketsLossRate',
    /// 'txThroughput', 'txPackets', 'txPacketsLossRate',
    /// 'portRxDrops', 'portTxDrops',
    /// 'tcpTxRexmitRate', 'tcpRxErrRate'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-vnic-net'</td>
    /// <td>
    /// 'rxThroughput', 'rxPackets', 'rxPacketsLossRate',
    /// 'txThroughput', 'txPackets', 'txPacketsLossRate',
    /// 'portRxDrops', 'portTxDrops'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-pnic-net'</td>
    /// <td>
    /// 'rxThroughput', 'rxPackets', 'rxPacketsLossRate',
    /// 'txThroughput', 'txPackets', 'txPacketsLossRate',
    /// 'portRxDrops', 'portTxDrops', 'pauseCount'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'lsom-world-cpu'</td>
    /// <td>
    /// 'usedPct', 'readyPct'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'dom-world-cpu'</td>
    /// <td>
    /// 'usedPct', 'readyPct'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'host-cpu'</td>
    /// <td>
    /// 'coreUtilPct', 'pcpuUtilPct', 'pcpuUsedPct'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-iscsi-host'</td>
    /// <td>
    /// 'iopsRead', 'iopsWrite', 'iopsTotal',
    /// 'bandwidthRead', 'bandwidthWrite', 'bandwidthTotal',
    /// 'latencyRead', 'latencyWrite', 'latencyTotal', 'queueDepth'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-iscsi-target'</td>
    /// <td>
    /// 'iopsRead', 'iopsWrite', 'iopsTotal',
    /// 'bandwidthRead', 'bandwidthWrite', 'bandwidthTotal',
    /// 'latencyRead', 'latencyWrite', 'latencyTotal', 'queueDepth'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-iscsi-lun'</td>
    /// <td>
    /// 'iopsRead', 'iopsWrite', 'iopsTotal',
    /// 'bandwidthRead', 'bandwidthWrite', 'bandwidthTotal',
    /// 'latencyRead', 'latencyWrite', 'latencyTotal', 'queueDepth'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-cluster-capacity'</td>
    /// <td>
    /// 'total', 'used', 'free', 'savedByDedup', 'dedupRatio'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-file-service'</td>
    /// <td>
    /// 'readRequested', 'readTransferred', 'readOpTotal', 'readLatency',
    /// 'writeRequested', 'writeTransferred', 'writeOpTotal', 'writeLatency'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'nic-world-cpu'</td>
    /// <td>
    /// 'usedPct', 'readyPct'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-cpu'</td>
    /// <td>
    /// 'usedPct', 'readyPct'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-memory'</td>
    /// <td>
    /// 'kernelReservedSize', 'uwReservedSize'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'rdt-net'</td>
    /// <td>
    /// 'checksumMismatchCount'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-esa-disk-layer'</td>
    /// <td>
    /// 'iopsReadCapacity', 'iopsWriteCapacity', 'tputReadCapacity', 'tputReadCapacity',
    /// 'avgLatReadCapacity', 'avgLatReadCapacity'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-esa-disk-scsifw'</td>
    /// <td>
    /// 'iopsDevRead', 'iopsDevWrite', 'latencyDevRead', 'latencyDevWrite',
    /// 'latencyDevGAvg', 'latencyDevDAvg', 'throughputDevRead', 'throughputDevWrite'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'zdom-vtx'</td>
    /// <td>
    /// 'rateTotalCacheMiss', 'rateTotalCacheRef', 'rateTotalLogicalTreeCacheMiss',
    /// 'rateTotalMiddleTreeCacheMiss', 'rateTotalSnapTreeCacheMiss', 'rateTotalBitmapCacheMiss',
    /// 'rateTotalSutCacheMiss', 'rateTxnPrefetchTotalCacheMiss', 'rateTxnPrefetchLogicalTreeCacheMiss',
    /// 'rateTxnPrefetchMiddleTreeCacheMiss', 'rateTxnPrefetchSutCacheMiss', 'rateTxnBankTotalCacheMiss',
    /// 'rateTxnBankLogicalTreeCacheMiss', 'rateTxnBankMiddleTreeCacheMiss', 'rateTxnBankSutCacheMiss',
    /// 'rateTxnUnmapTotalCacheMiss, 'rateTxnUnmapLogicalTreeCacheMiss', 'rateTxnUnmapMiddleTreeCacheMiss',
    /// 'rateTxnUnmapSutCacheMiss', 'rateTxnSegCleaningCtxDataTotalCacheMiss', 'rateTxnSegCleaningCtxDataLogicalTreeCacheMiss',
    /// 'rateTxnSegCleaningCtxDataMiddleTreeCacheMiss', 'rateTxnSegCleaningCtxDataSutCacheMiss',
    /// 'rateTxnlookUpCacheMiss', 'rateTxnlookUpLogicalTreeCacheMiss', 'rateTxnlookUpMiddleTreeCacheMiss',
    /// 'latAvgCacheGet', 'latAvgTotalOpIO', 'latAvgTxnBank', 'latAvgTxnBankTotalIO', 'latAvgTxnUnmap',
    /// 'latAvgTxnUnmapTotalIO', 'cacheMissPerPrefetchTxn', 'cacheMissPerBankFlushTxn', 'cacheMissPerLookupTxn'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'cluster-remotedomclient'</td>
    /// <td>
    /// 'iopsRead', 'iopsWrite', 'throughputRead',
    /// 'throughputWrite', 'latencyAvgRead', 'latencyAvgWrite',
    /// 'congestion', 'oio'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'computeCluster-remotedomclient'</td>
    /// <td>
    /// 'iopsRead', 'iopsWrite', 'throughputRead',
    /// 'throughputWrite', 'latencyAvgRead', 'latencyAvgWrite',
    /// 'congestion', 'oio'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-direct-cluster'</td>
    /// <td>
    /// 'iopsDevRead', 'iopsDevWrite', 'throughputDevRead',
    /// 'throughputDevWrite', 'latencyDevRead', 'latencyDevWrite',
    /// 'oioDevRead', 'oioDevWrite'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'vsan-direct-host'</td>
    /// <td>
    /// 'iopsDevRead', 'iopsDevWrite', 'throughputDevRead',
    /// 'throughputDevWrite', 'latencyDevRead', 'latencyDevWrite',
    /// 'oioDevRead', 'oioDevWrite'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'host-pmem'</td>
    /// <td>
    /// 'bandwidthRead', 'bandwidthWrite', 'bandwidthTotal',
    /// 'latencyRead', 'latencyWrite', 'iopsRead', 'iopsWrite',
    /// 'iopsTotal'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'cluster-pmem'</td>
    /// <td>
    /// 'bandwidthRead', 'bandwidthWrite', 'bandwidthTotal',
    /// 'latencyRead', 'latencyWrite', 'iopsRead', 'iopsWrite',
    /// 'iopsTotal'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'ioinsight'</td>
    /// <td>
    /// 'iopsRead', 'iopsWrite', 'iopsTotal', 'throughputRead', 'throughputWrite',
    /// 'throughputSequential', 'throughputRandom', 'throughputTotal',
    /// 'sequentialReadRatio', 'sequentialWriteRatio', 'sequentialRatio',
    /// 'randomReadRatio', 'randomWriteRatio', 'randomRatio',
    /// 'aligned4kReadRatio', 'aligned4kWriteRatio', 'aligned4kRatio',
    /// 'unaligned4kReadRatio', 'unaligned4kWriteRatio', 'unaligned4kRatio',
    /// 'readRatio', 'writeRatio'
    /// </td>
    /// </tr>
    /// <tr>
    /// <td nowrap="1">'ioinsight-histogram'</td>
    /// <td>
    /// 'iosz0\_4k', 'riosz0\_4k', 'wiosz0\_4k',
    /// 'iosz4k', 'riosz4k', 'wiosz4k',
    /// 'iosz4k\_8k', 'riosz4k\_8k', 'wiosz4k\_8k',
    /// 'iosz8k', 'riosz8k', 'wiosz8k',
    /// 'iosz8k\_16k', 'riosz8k\_16k', 'wiosz8k\_16k',
    /// 'iosz16k', 'riosz16k', 'wiosz16k',
    /// 'iosz16k\_32k', 'riosz16k\_32k', 'wiosz16k\_32k',
    /// 'iosz32k', 'riosz32k', 'wiosz32k',
    /// 'iosz32k\_64k', 'riosz32k\_64k', 'wiosz32k\_64k',
    /// 'iosz64k', 'riosz64k', 'wiosz64k',
    /// 'iosz64k\_128k', 'riosz64k\_128k', 'wiosz64k\_128k',
    /// 'iosz128k', 'riosz128k', 'wiosz128k',
    /// 'iosz128k\_256k', 'riosz128k\_256k', 'wiosz128k\_256k',
    /// 'iosz256k', 'riosz256k', 'wiosz256k',
    /// 'iosz256k\_512k', 'riosz256k\_512k', 'wiosz256k\_512k',
    /// 'iosz512k', 'riosz512k', 'wiosz512k',
    /// 'iosz512k\_1m', 'riosz512k\_1m', 'wiosz512k\_1m',
    /// 'iosz1m', 'riosz1m', 'wiosz1m', 'iosz1\_m', 'riosz1\_m', 'wiosz1\_m',
    /// 'lat0\_1us', 'rlat0\_1us', 'wlat0\_1us',
    /// 'lat1\_10us', 'rlat1\_10us', 'wlat1\_10us',
    /// 'lat10\_100us', 'rlat10\_100us', 'wlat10\_100us',
    /// 'lat100\_500us', 'rlat100\_500us', 'wlat100\_500us',
    /// 'lat500us\_1ms', 'rlat500us\_1ms', 'wlat500us\_1ms',
    /// 'lat1\_5ms', 'rlat1\_5ms', 'wlat1\_5ms',
    /// 'lat5\_10ms', 'rlat5\_10ms', 'wlat5\_10ms',
    /// 'lat10\_25ms', 'rlat10\_25ms', 'wlat10\_25ms',
    /// 'lat25\_50ms', 'rlat25\_50ms', 'wlat25\_50ms',
    /// 'lat50\_100ms', 'rlat50\_100ms', 'wlat50\_100ms',
    /// 'lat100\_ms', 'rlat100\_ms', 'wlat100\_ms'
    /// </td>
    /// </tr>
    /// </table>
    ///
    /// ## Parameters:
    ///
    /// ### query_specs
    /// A array of VsanPerfQuerySpec objects. The VsanPerfQuerySpec object
    /// specifies a reference for an entity, plus optional criteria for filtering
    /// results. Only metrics for the entities that can be resolved are returned in
    /// any result.  
    /// The VsanPerfQuerySpec object in this operation can
    /// query for different metrics. Or, select all types of statistics for a
    /// single entity. See above for supported entity types, metric groups and metrics  
    /// The VsanPerfQuerySpec object supports wildcard query by setting UUID to '\*', it retrieves
    /// all entities based on the specified entity type, startTime, and endTime.  
    /// From version 8.0U2, the VsanPerfQuerySpec object supports multi-entity query for a single
    /// entity type and unified duration by setting queried node id to '&lt;UUID1&gt;,&lt;UUID2&gt;,..',
    /// it will return all relevant entities according to the entity type, startTime and endTime.
    /// The maximum limit of the number of UUIDs is 400.  
    /// **Note**: To avoid bad performance and resource usage issues caused by massive stats data
    /// from a stats query. There are some validation checks:
    /// - In each query, the startTime and endTime must be specified in
    ///   the query spec. And the suggested time span is less than 24 hours. To query stats
    ///   for larger time range, please use paging mechanism. For example, split the time range
    ///   in to smaller ones, and use multiple status queries with smaller time ranges.
    /// - In each query, when there is no wildcard or multi-entity query specified in the parameter
    ///   querySpecs, the number of items within querySpecs should not exceed 100. When the parameter
    ///   querySpecs includes more than 100 items, please use the paging mechanism.
    /// - In each query, if there is wildcard query or multi-entity query, the parameter querySpecs
    ///   can only contain either one wildcard query or one multi-entity query.
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The metric values for the specified entity or entities.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the set of arguments passed to the function is
    /// not specified correctly.
    /// 
    /// ***NotSupported***: if the host queried is not a Stats Daemon master
    /// 
    /// ***NotFound***: if no ESXi host could be contacted to perform the operation
    /// when called against vCenter.
    pub async fn vsan_perf_query_perf(&self, query_specs: &[crate::types::structs::VsanPerfQuerySpec], cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Vec<crate::types::structs::VsanPerfEntityMetricCsv>> {
        let input = VsanPerfQueryPerfRequestType {query_specs, cluster, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfQueryPerf", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// The API is designed to return a list of hotspot entities that are consuming the
    /// most IOPS, throughput or latency according to given start time and end time in
    /// the vSAN cluster.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster, which is ignored if the API is called against
    /// host.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### query_spec
    /// Represent query specification to retrieve the desired top
    /// entities.
    ///
    /// ## Returns:
    ///
    /// A list of hotspot entities with the expected metric values at the given
    /// start time and end time.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the set of arguments passed to the function is
    /// not specified correctly, e.g., numEntities is more than 64.
    /// 
    /// ***Timedout***: if this API is timeout.
    /// 
    /// ***VsanNodeNotMaster***: if this API is invokded against stats agent node.
    /// 
    /// ***NotFound***: if the stats primary node is not found in target cluster.
    /// 
    /// ***NotSupported***: if vSAN is not enabled in target cluster.
    pub async fn query_vsan_perf_hotspot_entities(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, query_spec: &crate::types::structs::VsanPerfHotspotQuerySpec) -> Result<Vec<crate::types::structs::VsanPerfHotspotEntitiesMetrics>> {
        let input = QueryVsanPerfHotspotEntitiesRequestType {cluster, query_spec, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/QueryVsanPerfHotspotEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// The API is designed to return a list of top contributors with either type of
    /// VM or disk group that are consuming the most IOPS, throughput or latency in
    /// the vSAN cluster.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster, which is ignored if the API is called against
    /// host.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### query_spec
    /// Represent query specification to retrieve the desired top
    /// entities.
    ///
    /// ## Returns:
    ///
    /// A list of top entities with the expected metric values at the given
    /// time stamp.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the set of arguments passed to the function is
    /// not specified correctly, e.g., numEntities is above 50.
    /// 
    /// ***VsanNodeNotMaster***: if this API is invokded against stats agent node.
    pub async fn query_vsan_perf_top_entities(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, query_spec: &crate::types::structs::VsanPerfTopQuerySpec) -> Result<Vec<crate::types::structs::VsanPerfEntityMetricCsv>> {
        let input = QueryVsanPerfTopEntitiesRequestType {cluster, query_spec, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/QueryVsanPerfTopEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Save time ranges in performance service.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### time_ranges
    /// *VsanPerfTimeRange* list to be saved.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_perf_save_time_ranges(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, time_ranges: &[crate::types::structs::VsanPerfTimeRange]) -> Result<()> {
        let input = VsanPerfSaveTimeRangesRequestType {cluster, time_ranges, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfSaveTimeRanges", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Set the policy of the vSAN object/directory containing the vSAN Perf Stats DB.
    /// 
    /// The cluster parameter is ignored if called on ESXi.
    /// Profile can be 3 formats:
    /// - 1\. VirtualMachineEmptyProfileSpec means to use the empty vSAN policy. This is not the
    ///   default policy, but a policy where all fields have default values.
    /// - 2\. VirtualMachineDefinedProfileSpec where profileId is set, in which case this
    ///   profileId will be looked up in SPBM for the detailed policy information.
    /// - 3\. VirtualMachineDefinedProfileSpec where profileId is an empty string and instead
    ///   the profileData is set for extensionKey 'com.vmware.vim.sps'. In this case the
    ///   objectData field can be either the vSAN expression format, or a SPBM XML string.
    ///   
    ///   
    /// If no profile is supplied, and the call is executed against vCenter, then SPBM will
    /// be consulted for the vSAN datastore's default profile.
    ///   
    /// When this method returns successfully, the profile has been applied, but vSAN may
    /// still be remediating in order to implement the new policy. The health state of the
    /// object and resync information should be monitored to check on the progress.
    ///   
    /// Only the third option is available when called on ESXi, other formats will raise
    /// InvalidArgument exception.
    ///   
    /// Exception:
    /// - If SPBM needs to be contacted, but SPBM is not available, RuntimeFault exception will
    ///   be raised.
    /// - If the profileId can not be resolved with SPBM, InvalidArgument exception will be raised.
    /// - If objectData was provided but is neither of the two supported formats, InvalidArgument
    ///   exception will be raised.
    /// - If the statsDB object can not be found, FileNotFound exception will be raised.
    /// - If the statsDB object failed to set the policy, e.g. because it is not accessible,
    ///   FileNotWritable exception will be raised.
    /// - If called against vCenter, but no ESXi host could be contacted to perform
    ///   the operation NotFound exception will be raised.
    ///   
    ///   
    /// Python code example:
    ///   
    /// spec = vim.vm.DefinedProfileSpec()
    ///   
    /// VsanPerfSetStatsObjectPolicy(self.clusterRef, spec)
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### profile
    /// See above description for all possible options.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if no ESXi host could be contacted to perform the operation
    /// when called against vCenter.
    /// 
    /// ***VsanFault***: if the caller doesn't have the required privilege
    pub async fn vsan_perf_set_stats_object_policy(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, profile: Option<&dyn crate::types::traits::VirtualMachineProfileSpecTrait>) -> Result<bool> {
        let input = VsanPerfSetStatsObjectPolicyRequestType {cluster, profile, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfSetStatsObjectPolicy", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Toggle vSAN performance service verbose mode.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### verbose_mode
    /// Switch of verbose mode, the type is boolean.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_perf_toggle_verbose_mode(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, verbose_mode: bool) -> Result<()> {
        let input = VsanPerfToggleVerboseModeRequestType {cluster, verbose_mode, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfToggleVerboseMode", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Deprecated as of vSphere API 6.7, please use VsanPerfDiagnoseTask instead.
    /// 
    /// API to do performance diagnosis.
    ///
    /// ## Parameters:
    ///
    /// ### perf_diagnose_query
    /// The query describing details of diagnosis
    /// required, such as the period of diagnosis and the query type.
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The list of performance issues found. Each performance issue is
    /// returned as a VsanPerfDiagnosticResult. The VsanPerfDiagnosticResult
    /// object will contain the entity and the metrics that caused the performance
    /// exception.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_perf_diagnose(&self, perf_diagnose_query: &crate::types::structs::VsanPerfDiagnoseQuerySpec, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::VsanPerfDiagnosticResult>>> {
        let input = VsanPerfDiagnoseRequestType {perf_diagnose_query, cluster, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfDiagnose", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Runs a diagnostic query to determine performance issues in a vSAN
    /// cluster.
    /// 
    /// This API call investigates the state of the vSAN cluster during
    /// the chosen period of time, and returns any issues (list of
    /// VsanPerfDiagnosticResult) that may be limiting the
    /// performance of the vSAN cluster. This API is available from only the vCenter,
    /// it is not available at the end-host. Processing is performed in the
    /// background, and a task is returned. Please wait for the task to finish, and
    /// then call GetVsanPerfDiagnosisResult to retrieve results.
    ///
    /// ## Parameters:
    ///
    /// ### perf_diagnose_query
    /// The query describing details of diagnosis
    /// required, such as the period of diagnosis and the query type.
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// A task doing the asynchronous work.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the set of arguments passed to the function is
    /// not specified correctly.
    /// 
    /// ***NotFound***: if no ESXi host could be contacted to perform the
    /// operation when called against vCenter or if the API was not invoked on
    /// vCenter or if CEIP is not enabled
    /// 
    /// ***VsanFault***: if the caller doesn't have the required privilege, or the
    /// cluster has no hosts.
    pub async fn vsan_perf_diagnose_task(&self, perf_diagnose_query: &crate::types::structs::VsanPerfDiagnoseQuerySpec, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanPerfDiagnoseTaskRequestType {perf_diagnose_query, cluster, };
        let path = format!("/vsan/VsanPerformanceManager/{moId}/VsanPerfDiagnoseTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfCreateStatsObjectRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a dyn crate::types::traits::VirtualMachineProfileSpecTrait>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfCreateStatsObjectTaskRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a dyn crate::types::traits::VirtualMachineProfileSpecTrait>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfDeleteStatsObjectRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfDeleteStatsObjectTaskRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfDeleteTimeRangeRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GetVsanPerfDiagnosisResultRequestType<'a> {
    task: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfQueryClusterHealthRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfQueryNodeInformationRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryRemoteServerClustersRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "querySpec")]
    query_spec: Option<&'a crate::types::structs::VsanRemoteClusterQuerySpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfQueryStatsObjectInformationRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfQueryTimeRangesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(rename = "querySpec")]
    query_spec: &'a crate::types::structs::VsanPerfTimeRangeQuerySpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfQueryPerfRequestType<'a> {
    #[serde(rename = "querySpecs")]
    query_specs: &'a [crate::types::structs::VsanPerfQuerySpec],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVsanPerfHotspotEntitiesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(rename = "querySpec")]
    query_spec: &'a crate::types::structs::VsanPerfHotspotQuerySpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVsanPerfTopEntitiesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(rename = "querySpec")]
    query_spec: &'a crate::types::structs::VsanPerfTopQuerySpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfSaveTimeRangesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(rename = "timeRanges")]
    time_ranges: &'a [crate::types::structs::VsanPerfTimeRange],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfSetStatsObjectPolicyRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a dyn crate::types::traits::VirtualMachineProfileSpecTrait>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfToggleVerboseModeRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(rename = "verboseMode")]
    verbose_mode: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfDiagnoseRequestType<'a> {
    #[serde(rename = "perfDiagnoseQuery")]
    perf_diagnose_query: &'a crate::types::structs::VsanPerfDiagnoseQuerySpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPerfDiagnoseTaskRequestType<'a> {
    #[serde(rename = "perfDiagnoseQuery")]
    perf_diagnose_query: &'a crate::types::structs::VsanPerfDiagnoseQuerySpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}
