use std::sync::Arc;
use crate::core::client::{Client, Result};
/// vSAN stretched Cluster is a specific configuration implemented in
/// environments where disaster/downtime avoidance is a key requirement.
/// 
/// vSAN stretched Clusters with Witness Host refers to a deployment
/// where a user sets up a vSAN cluster with 2 active/active sites with
/// numbers of ESXi hosts between the two sites. The sites are connected via a
/// high bandwidth/low latency link.
/// The third site hosting the vSAN Witness Host is connected to both of
/// the active/active data-sites. This connectivity can be via low bandwidth/high
/// latency links.
/// Each site is configured as a vSAN Fault Domain. The nomenclature used
/// to describe a vSAN stretched Cluster configuration is X+Y+Z, where X is
/// the number of ESXi hosts at data site A, Y is the number of ESXi hosts at data
/// site B, and Z is the number of witness hosts at site C. Data sites are where
/// virtual machines are deployed.
/// The maximum configuration is 15+15+1 (31 nodes).  
/// vSAN Remote Office / Branch Office Deployment, aka ROBO cluster,
/// is a specific deployment of vSAN stretched cluster. A two-node
/// vSAN cluster, and a vSphere vSAN witness host appliance as
/// witness, forms a vSAN ROBO cluster. Copies of vSAN objects
/// that make up a virtual machine are typically stored in two physical nodes,
/// if one of the physical nodes is offline, it is still possible for virtual
/// machines to run using the files located on another physical node. In the
/// case where the hosts in a two-node cluster are unable to communicate across
/// the network, the witness serves as a "tie-breaker" to achieve a quorum and
/// enables the cluster to restart virtual machines impacted by an outage.
/// Details to deploy a vSAN stretched cluster, please reference to API
/// *VimClusterVsanVcStretchedClusterSystem.VSANVcConvertToStretchedCluster*,
/// and for ROBO cluster, please assign
/// witness appliance to parameter 'witnessHost', and each physical host stands
/// for a Fault Domain.  
/// VsanVcStretchedClusterSystem is used to configure and manage vSAN
/// stretched cluster. The ManagedEntity can be accessed through MOID of
/// vsan-stretched-cluster-system, through vSAN service at vCenter server
/// side.
#[derive(Clone)]
pub struct VimClusterVsanVcStretchedClusterSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl VimClusterVsanVcStretchedClusterSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Add a host as witness host to vSAN cluster to re-enable vSAN
    /// stretched cluster.
    /// 
    /// It is expected to be used in following scenarios:
    /// 1\. Stretched cluster is disabled by
    /// *VimClusterVsanVcStretchedClusterSystem.VSANVcRemoveWitnessHost*;
    /// 2\. Replace original witness host with a new one, this should happen
    /// when old witness host is out of service, such as host is down or removed
    /// from vCenter inventory;
    /// This function doesn't change existing Fault Domain configuration, and
    /// please be sure the old witness host was gone or out of service, because
    /// replacing witness host will reduce redundancy.
    /// Relative to API
    /// *VimClusterVsanVcStretchedClusterSystem.VSANVcConvertToStretchedCluster*
    /// , it only takes care of
    /// witness host reconfiguration, vSAN cluster must already be configured
    /// for stretched cluster.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target vSAN cluster to add/replace witness host;
    /// 
    /// ***Required privileges:*** Host.Inventory.AddHostToCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### witness_host
    /// The witness host to be added into vSAN cluster.
    /// This host must be connected and managed by the same
    /// vCenter server, and cannot be a part of target
    /// cluster;
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### preferred_fd
    /// The name of preferred Fault Domain;
    ///
    /// ### disk_mapping
    /// The diskMapping to be created on witness host. If
    /// disk claim is configured as auto-mode on witness host,
    /// this parameter is not required.
    ///
    /// ### metadata_mode
    /// True to indicate the host runs as metadata host instead
    /// of normal witness host. This is currently reserved by
    /// VMware internally to represent a different cluster type
    /// other than stretched cluster. Leave this unset for
    /// vSAN stretched cluster.
    ///
    /// ### storage_pool_spec
    /// The specification to add disks to vSAN storage pool.
    /// This parameter cannot be set together with diskMapping
    /// param.
    ///
    /// ## Returns:
    ///
    /// vim.Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: If any host in target cluster is not
    /// connected to vCenter server;
    /// 
    /// ***InvalidArgument***: If below issues exist:
    /// 1\. target cluster doesn't enable
    /// vSAN;
    /// 2\. witness host resides in target
    /// cluster;
    /// 3\. neither IPv4 nor IPv6 is properly
    /// configured for vSAN traffic
    /// on all hosts in target cluster;
    /// 4\. both diskMapping and storagePoolSpec
    /// are specified
    /// 
    /// ***NotEnoughLicenses***: If any host in target cluster doesn't
    /// have vSAN stretched cluster license;
    /// 
    /// ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn vsan_vc_add_witness_host(&self, cluster: &crate::types::structs::ManagedObjectReference, witness_host: &crate::types::structs::ManagedObjectReference, preferred_fd: &str, disk_mapping: Option<&crate::types::structs::VsanHostDiskMapping>, metadata_mode: Option<bool>, storage_pool_spec: Option<&crate::types::structs::VsanAddStoragePoolDiskSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanVcAddWitnessHostRequestType {cluster, witness_host, preferred_fd, disk_mapping, metadata_mode, storage_pool_spec, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/VSANVcAddWitnessHost", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// This API is used to convert a batch of traditional vSAN clusters into
    /// vSAN stretched clusters sharing the same witness host.
    /// 
    /// Relative to API
    /// *VimClusterVsanVcStretchedClusterSystem.VSANVcConvertToStretchedCluster*
    /// , it takes care of witness host reconfiguration, and the stretched cluster
    /// system in each cluster may be configured:
    /// 1. Configure vSAN cluster into two sites through the specified
    ///    vSAN Fault Domain setting;
    /// 2. Setup preferred Fault Domain through the specified
    ///    preferredFault Domain setting;
    ///    
    ///   
    /// Clusters that are already working in stretched mode are not
    /// supported. If the configuration of one or more clusters fails, the
    /// remaining successful operations will not be rolled back because this is
    /// a batch operation for multiple clusters.
    ///
    /// ## Parameters:
    ///
    /// ### config_spec
    /// The mapping between target witness host config and target
    /// clusters's config(configSpec.clusters), to decide how to
    /// configure target clusters to work in stretched mode.
    ///
    /// ## Returns:
    ///
    /// vim.Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If below issues exist:
    /// 1. Target witness doesn't support
    ///    sharing to multiple clusters, but
    ///    does provide more than one cluster;
    /// 2. Target witness and clusters do not
    ///    meet the number limit requirements
    ///    for components;
    /// 3. One or more of target clusters
    ///    doesn't enable vSAN;
    /// 4. witness host resides in one or more
    ///    target clusters;
    /// 5. neither IPv4 nor IPv6 is properly
    ///    configured for vSAN traffic
    ///    on all hosts in target clusters;
    ///    
    /// ***NotEnoughLicenses***: If any host in target clusters doesn't
    /// have vSAN stretched cluster license;
    /// 
    /// ***NotSupported***: If this API is not supported on current platform.
    /// 
    /// ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn vsan_vc_add_witness_host_for_clusters(&self, config_spec: &crate::types::structs::VsanVcStretchedClusterConfigSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanVcAddWitnessHostForClustersRequestType {config_spec, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/VsanVcAddWitnessHostForClusters", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// This API is used to convert a traditional vSAN cluster to
    /// vSAN stretched cluster.
    /// 
    /// It will help to:
    /// 1. Configure vSAN cluster into two sites through
    ///    vSAN Fault Domain setting;
    /// 2. Add witness host and configure unicast communication
    ///    at cluster wide;
    /// 3. Setup preferred Fault Domain;
    /// 4. Create vSAN disk group on witness host if
    ///    auto-claim is not enabled on witness host;
    ///    
    ///   
    /// Relative to API
    /// *VimClusterVsanVcStretchedClusterSystem.VSANVcAddWitnessHost*,
    /// it not only takes care of
    /// setting up witness host, but also help on vSAN sites configuration,
    /// to guarantee the vSAN cluster works in stretched mode.
    /// vSAN cluster already works in stretched mode, is not supported by
    /// this API.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster to be converted; It is expected
    /// to enable vSAN, but not a vSAN stretched
    /// cluster;
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### fault_domain_config
    /// The mapping between Fault Domain and vSAN
    /// hosts reside in target cluster, to decide
    /// how to configure vSAN cluster into two
    /// sites;
    ///
    /// ### witness_host
    /// The witness host to be added into vSAN cluster.
    /// This host must be connected and managed by the same
    /// vCenter server, and cannot be a part of target
    /// cluster;
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### preferred_fd
    /// The name of preferred Fault Domain;
    ///
    /// ### disk_mapping
    /// The diskMapping to be created on witness host. If
    /// disk claim is configured as auto-mode on witness host,
    /// this parameter is not required.
    ///
    /// ### storage_pool_spec
    /// The specification to add disks to vSAN storage pool.
    /// This parameter cannot be set together with diskMapping
    /// param.
    ///
    /// ## Returns:
    ///
    /// vim.Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: If any host in target cluster is not
    /// connected to vCenter server;
    /// 
    /// ***InvalidArgument***: If below issues exist:
    /// 1. target cluster doesn't enable
    ///    vSAN;
    /// 2. witness host resides in target
    ///    cluster;
    /// 3. neither IPv4 nor IPv6
    ///    is properly
    ///    configured for vSAN traffic
    ///    on all hosts in target
    ///    cluster;
    /// 4. target cluster is already a vSAN
    ///    stretched cluster;
    /// 5. both diskMapping and storagePoolSpec
    ///    are specified;
    ///    
    ///    ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn vsan_vc_convert_to_stretched_cluster(&self, cluster: &crate::types::structs::ManagedObjectReference, fault_domain_config: &crate::types::structs::VimClusterVsanStretchedClusterFaultDomainConfig, witness_host: &crate::types::structs::ManagedObjectReference, preferred_fd: &str, disk_mapping: Option<&crate::types::structs::VsanHostDiskMapping>, storage_pool_spec: Option<&crate::types::structs::VsanAddStoragePoolDiskSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanVcConvertToStretchedClusterRequestType {cluster, fault_domain_config, witness_host, preferred_fd, disk_mapping, storage_pool_spec, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/VSANVcConvertToStretchedCluster", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query configuration of preferred Fault Domain of specified cluster.
    /// 
    /// If call this API against vSAN service of version 6.5 and before,
    /// additional privilege Host.Inventory.EditCluster is required, please
    /// be noted.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Target cluster to query.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// VSANPreferredFaultDomainInfo which contains preferred Fault
    /// Domain's user friendly name and UUID. If specified cluster
    /// is not vSAN stretched cluster, both fields will be
    /// unset.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: If any host in target cluster is not
    /// connected to vCenter server;
    /// 
    /// ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn vsan_vc_get_preferred_fault_domain(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<crate::types::structs::VimClusterVsanPreferredFaultDomainInfo>> {
        let input = VsanVcGetPreferredFaultDomainRequestType {cluster, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/VSANVcGetPreferredFaultDomain", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Query witness host configuration of specified cluster.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster to query;
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// VSANWitnessHostInfo\[\] which contains witness host's UUID,
    /// ManagedEntity instance, preferred Fault Domain's user friendly
    /// name and UUID, IP address of unicast agent, and name of the Fault
    /// Domain that witness host resides in. If specified cluster in vSAN
    /// stretched cluster, all fields above will be set to actual setting,
    /// otherwise all fields will be unset.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: If any host in target cluster is not
    /// connected to vCenter server;
    /// 
    /// ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn vsan_vc_get_witness_hosts(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::VimClusterVsanWitnessHostInfo>>> {
        let input = VsanVcGetWitnessHostsRequestType {cluster, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/VSANVcGetWitnessHosts", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Check whether specified host is a witness host.
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The target host to check.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// true is host is a witness host,
    /// false is host is not a witness host.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn vsan_vc_is_witness_host(&self, host: &crate::types::structs::ManagedObjectReference) -> Result<bool> {
        let input = VsanVcIsWitnessHostRequestType {host, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/VSANVcIsWitnessHost", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Return whether the host is a virtual appliance witness host
    /// for stretched cluster
    /// Witness host can be a VM deployed from vSAN witness server OVF, the VM added
    /// to vCenter datacenter as a ESXi host, and has no difference with other
    /// ESXi host in a stretched cluster, but the VM host can only used as witness
    /// host, UI need to know if the witness host is a virtual appliance, using this
    /// API can tell the host is a virtual appliance or not.
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// dictionary of hosts' MO id and if the host is a virtual appliance
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn vsan_is_witness_virtual_appliance(&self, hosts: &[crate::types::structs::ManagedObjectReference]) -> Result<Option<Vec<crate::types::structs::VsanHostVirtualApplianceInfo>>> {
        let input = VsanIsWitnessVirtualApplianceRequestType {hosts, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/VSANIsWitnessVirtualAppliance", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Query whether a given host could be used as shared witness for a group of
    /// specified ROBO clusters.
    /// 
    /// Result data would contain compatibility check result
    /// for both sharedWitnessHost and roboClusters.
    /// For sharedWitnessHost, it will check following items.
    /// 1\) Check whether given host is a witness host.
    /// 2\) Check whether this witness host has shared witness capability.
    /// 3\) Check whether clusters count would exceed per shared witness host's limit.
    /// For roboClusters, it will check following items.
    /// 1\) For a vSAN not enabled cluster, check whether it could be a candidate of
    /// ROBO cluster (cluster which has 2 hosts).
    /// 2\) For a vSAN enabled cluster, check whether it is a ROBO cluster (stretched
    /// cluster which has 2 hosts).
    /// 3\) For a ROBO cluster, check whether its component limit exceeds current
    /// shared witness host's component count limitation for individual cluster.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### shared_witness_host
    /// A host entity which would be used as a shared
    /// witness host.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### robo_clusters
    /// A list of ROBO clusters which would use
    /// sharedWitnessHost as their witness.
    /// 
    /// Refers instances of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// SharedWitnessCompatibilityResult to present compatibility check
    /// result of given shared witness host and robo cluster list.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If any unexpected runtime fault is met.
    /// 
    /// ***NotSupported***: If this API is not supported on current platform.
    pub async fn query_shared_witness_compatibility(&self, shared_witness_host: &crate::types::structs::ManagedObjectReference, robo_clusters: &[crate::types::structs::ManagedObjectReference]) -> Result<crate::types::structs::VsanSharedWitnessCompatibilityResult> {
        let input = QuerySharedWitnessCompatibilityRequestType {shared_witness_host, robo_clusters, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/QuerySharedWitnessCompatibility", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query cluster runtime information for each cluster associated to given
    /// witness host.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### witness_host
    /// Witness host managed entity.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### skip_components_count
    /// Skip setting in the result the number of
    /// components for each cluster on the given
    /// witness host. This can fail if the host is
    /// disconnected, set to true to avoid failure
    /// and retrieve other information in the result.
    ///
    /// ## Returns:
    ///
    /// List of ClusterRuntimeInfo to present each cluster's information.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If any unexpected runtime fault is met.
    /// 
    /// ***NotSupported***: If this API is not supported on current platform.
    pub async fn query_shared_witness_cluster_info(&self, witness_host: &crate::types::structs::ManagedObjectReference, skip_components_count: Option<bool>) -> Result<Option<Vec<crate::types::structs::ClusterRuntimeInfo>>> {
        let input = QuerySharedWitnessClusterInfoRequestType {witness_host, skip_components_count, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/QuerySharedWitnessClusterInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Remove witness host from the vSAN stretched cluster to disable
    /// vSAN stretched cluster.
    /// 
    /// Detailed changes will happen on target
    /// cluster:
    /// 1. vSAN on witness host will be disabled;
    /// 2. Unicast agent setting will be removed from all
    ///    data hosts in specified
    ///    vSAN stretched cluster;
    ///    
    ///   
    /// Original Fault Domains for both two sites will be kept, and vSAN is
    /// still enabled on all data hosts.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster to disable;
    /// 
    /// ***Required privileges:*** Host.Inventory.RemoveHostFromCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### witness_host
    /// Witness host to remove;
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### witness_address
    /// The IP address of witness host used as unicast agent.
    ///
    /// ## Returns:
    ///
    /// vim.Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: If any host in target cluster is not
    /// connected to vCenter server;
    /// 
    /// ***InvalidArgument***: If below issues exist:
    /// 1. target cluster doesn't enable
    ///    vSAN;
    /// 2. witness host is specified
    ///    but doesn't
    ///    match the in-use
    ///    configuration;
    /// 3. target cluster is not vSAN
    ///    stretched cluster;
    ///    
    /// ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn vsan_vc_remove_witness_host(&self, cluster: &crate::types::structs::ManagedObjectReference, witness_host: Option<&crate::types::structs::ManagedObjectReference>, witness_address: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanVcRemoveWitnessHostRequestType {cluster, witness_host, witness_address, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/VSANVcRemoveWitnessHost", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Replace witness host for all specified vSAN stretched clusters.
    /// 
    /// In other word, It is used to configure multiple robo clusters for
    /// a given witness host.
    /// 
    /// Each cluster in config is expected to be used in following scenario:
    /// Replace original witness host with a new one, and then the old witness host
    /// will no longer serve the target cluster. In this scenario, the
    /// preferredFdName and faultDomainConfig are optional parameters. If there is
    /// no specified preferredFdName/faultDomainConfig, target cluster will use the
    /// previous configuration to configure itself.
    /// Relative to API
    /// *VimClusterVsanVcStretchedClusterSystem.VSANVcAddWitnessHost*,
    /// it takes care of witness host reconfiguration, and the stretched cluster
    /// system in each cluster may be reconfigured. Clusters that are working
    /// in normal mode(no-stretched) are not supported.
    /// 
    /// If the configuration of one or more clusters fails, the remaining
    /// successful operations will not be rolled back because this is a batch
    /// operation for multiple clusters.
    ///
    /// ## Parameters:
    ///
    /// ### config_spec
    /// The mapping between target witness host config and
    /// target clusters's config(configSpec.clusters), to decide
    /// how to reconfigure target stretched clusters with new
    /// witness host.
    ///
    /// ## Returns:
    ///
    /// vim.Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If below issues exist:
    /// 1. Target witness doesn't support
    ///    sharing to multiple clusters, but
    ///    does provide more than one cluster;
    /// 2. Target witness and clusters do not
    ///    meet the number limit requirements
    ///    for components;
    /// 3. One or more of target clusters
    ///    doesn't enable vSAN;
    /// 4. witness host resides in one or more
    ///    target clusters;
    /// 5. neither IPv4 nor IPv6 is properly
    ///    configured for vSAN traffic
    ///    on all hosts in target clusters;
    ///    
    /// ***NotEnoughLicenses***: If any host in target clusters doesn't
    /// have vSAN stretched cluster license;
    /// 
    /// ***NotSupported***: If this API is not supported on current platform.
    /// 
    /// ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn vsan_vc_replace_witness_host_for_clusters(&self, config_spec: &crate::types::structs::VsanVcStretchedClusterConfigSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanVcReplaceWitnessHostForClustersRequestType {config_spec, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/VsanVcReplaceWitnessHostForClusters", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query hosts' capabilities of supporting vSAN stretched cluster,
    /// which reside in specified cluster, to decide whether specified cluster
    /// supports vSAN stretched cluster feature.
    /// 
    /// It could be used for
    /// any cluster, but only when all hosts reside in target cluster can
    /// support vSAN stretched cluster, it can be converted to
    /// vSAN stretched cluster.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster to query;
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### verify_all_connected
    /// Whether to ignore disconnected hosts. If it is set
    /// to true, vim.fault.InvalidState will be met if any
    /// host in target cluster is disconnected; if it is set
    /// to false, disconnected hosts will be ignored. Default
    /// value is false. But a cluster with disconnected hosts
    /// cannot be converted to vSAN stretched cluster;
    ///
    /// ## Returns:
    ///
    /// List of VSANStretchedClusterCapability, to present whether each
    /// host can support vSAN stretched cluster.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: If any host in target cluster is not
    /// connected to vCenter server and
    /// verifyAllConnected is set to true;
    /// 
    /// ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn vsan_vc_retrieve_stretched_cluster_vc_capability(&self, cluster: &crate::types::structs::ManagedObjectReference, verify_all_connected: Option<bool>) -> Result<Option<Vec<crate::types::structs::VimClusterVsanStretchedClusterCapability>>> {
        let input = VsanVcRetrieveStretchedClusterVcCapabilityRequestType {cluster, verify_all_connected, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/VSANVcRetrieveStretchedClusterVcCapability", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Set preferred Fault Domain for a vSAN stretched cluster.
    /// 
    /// This API could set/reconfigure preferred Fault Domain setting.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Target vSAN stretched cluster;
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### preferred_fd
    /// The user friendly name of preferred Fault Domain;
    ///
    /// ### witness_host
    /// The witness host to apply preferred Fault Domain setting;
    /// if it is not specified, will take the in-use witness
    /// host configured at cluster side.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// vim.Task
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: If any host in target cluster is not
    /// connected to vCenter server;
    /// 
    /// ***VsanFault***: If any unexpected runtime fault is met.
    pub async fn vsan_vc_set_preferred_fault_domain(&self, cluster: &crate::types::structs::ManagedObjectReference, preferred_fd: &str, witness_host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanVcSetPreferredFaultDomainRequestType {cluster, preferred_fd, witness_host, };
        let path = format!("/vsan/VimClusterVsanVcStretchedClusterSystem/{moId}/VSANVcSetPreferredFaultDomain", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(rename = "VSANVcAddWitnessHostRequestType", tag = "_typeName")]
struct VsanVcAddWitnessHostRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "witnessHost")]
    witness_host: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "preferredFd")]
    preferred_fd: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "diskMapping")]
    disk_mapping: Option<&'a crate::types::structs::VsanHostDiskMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadataMode")]
    metadata_mode: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storagePoolSpec")]
    storage_pool_spec: Option<&'a crate::types::structs::VsanAddStoragePoolDiskSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVcAddWitnessHostForClustersRequestType<'a> {
    #[serde(rename = "configSpec")]
    config_spec: &'a crate::types::structs::VsanVcStretchedClusterConfigSpec,
}
#[derive(serde::Serialize)]
#[serde(rename = "VSANVcConvertToStretchedClusterRequestType", tag = "_typeName")]
struct VsanVcConvertToStretchedClusterRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "faultDomainConfig")]
    fault_domain_config: &'a crate::types::structs::VimClusterVsanStretchedClusterFaultDomainConfig,
    #[serde(rename = "witnessHost")]
    witness_host: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "preferredFd")]
    preferred_fd: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "diskMapping")]
    disk_mapping: Option<&'a crate::types::structs::VsanHostDiskMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storagePoolSpec")]
    storage_pool_spec: Option<&'a crate::types::structs::VsanAddStoragePoolDiskSpec>,
}
#[derive(serde::Serialize)]
#[serde(rename = "VSANVcGetPreferredFaultDomainRequestType", tag = "_typeName")]
struct VsanVcGetPreferredFaultDomainRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(rename = "VSANVcGetWitnessHostsRequestType", tag = "_typeName")]
struct VsanVcGetWitnessHostsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(rename = "VSANVcIsWitnessHostRequestType", tag = "_typeName")]
struct VsanVcIsWitnessHostRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(rename = "VSANIsWitnessVirtualApplianceRequestType", tag = "_typeName")]
struct VsanIsWitnessVirtualApplianceRequestType<'a> {
    hosts: &'a [crate::types::structs::ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QuerySharedWitnessCompatibilityRequestType<'a> {
    #[serde(rename = "sharedWitnessHost")]
    shared_witness_host: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "roboClusters")]
    robo_clusters: &'a [crate::types::structs::ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QuerySharedWitnessClusterInfoRequestType<'a> {
    #[serde(rename = "witnessHost")]
    witness_host: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "skipComponentsCount")]
    skip_components_count: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(rename = "VSANVcRemoveWitnessHostRequestType", tag = "_typeName")]
struct VsanVcRemoveWitnessHostRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "witnessHost")]
    witness_host: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "witnessAddress")]
    witness_address: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVcReplaceWitnessHostForClustersRequestType<'a> {
    #[serde(rename = "configSpec")]
    config_spec: &'a crate::types::structs::VsanVcStretchedClusterConfigSpec,
}
#[derive(serde::Serialize)]
#[serde(rename = "VSANVcRetrieveStretchedClusterVcCapabilityRequestType", tag = "_typeName")]
struct VsanVcRetrieveStretchedClusterVcCapabilityRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "verifyAllConnected")]
    verify_all_connected: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(rename = "VSANVcSetPreferredFaultDomainRequestType", tag = "_typeName")]
struct VsanVcSetPreferredFaultDomainRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "preferredFd")]
    preferred_fd: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "witnessHost")]
    witness_host: Option<&'a crate::types::structs::ManagedObjectReference>,
}
