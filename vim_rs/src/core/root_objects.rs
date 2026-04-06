use crate::core::client::VimClientHandle;
use crate::mo::{CnsVolumeManager, EsxAgentManager, PbmServiceInstance, ServiceInstance, SmsServiceInstance, VimClusterVsanVcDiskManagementSystem, VimClusterVsanVcStretchedClusterSystem, VsanCapabilitySystem, VsanClusterPowerSystem, VsanDiagnosticsSystem, VsanFileServiceSystem, VsanIoInsightManager, VsanIscsiTargetSystem, VsanMassCollector, VsanObjectSystem, VsanPerformanceManager, VsanPhoneHomeSystem, VsanRemoteDatastoreSystem, VsanResourceCheckSystem, VsanSpaceReportSystem, VsanUpgradeSystemEx, VsanVcClusterConfigSystem, VsanVcClusterHealthSystem, VsanVcsaDeployerSystem, VsanVdsSystem, VsanVumSystem, VslmServiceInstance};

/// RootObjects provides access to the root objects of the Virtual Infrastructure JSON API.
pub struct RootObjects {
    client: VimClientHandle,
}

/// The RootObjects struct provides methods to access various root objects in the vSphere environment.
impl RootObjects {
    /// Creates a new instance of RootObjects with the provided [`VimClient`](crate::core::client::VimClient) handle.
    pub fn new(client: VimClientHandle) -> Self {
        Self { client }
    }

    /// Returns the [`VimClient`](crate::core::client::VimClient) handle used by this RootObjects instance.
    pub fn client(&self) -> VimClientHandle {
        self.client.clone()
    }

    /// Returns an instance of VIM ServiceInstance. This is the root of the main virtualization API.
    pub fn vim_service_instance(&self) -> ServiceInstance {
        ServiceInstance::new(self.client.clone(), "ServiceInstance")
    }

    /// Returns an instance of PbmServiceInstance, which is the root object for the vSphere Storage
    /// Policy-Based Management (PBM) API.
    pub fn pbm_service_instance(&self) -> PbmServiceInstance {
        PbmServiceInstance::new(self.client.clone(), "ServiceInstance")
    }

    /// Returns an instance of SmsServiceInstance, which is the root object for the vSphere Storage
    /// Management API.
    pub fn sms_service_instance(&self) -> SmsServiceInstance {
        SmsServiceInstance::new(self.client.clone(), "ServiceInstance")
    }

    /// Root object of the vSphere Storage Lifecycle Manager (VSLM) API.
    pub fn vslm_service_instance(&self) -> VslmServiceInstance {
        VslmServiceInstance::new(self.client.clone(), "ServiceInstance")
    }

    /// Returns an instance of EsxAgentManager.
    pub fn esx_agent_manager(&self) -> EsxAgentManager {
        EsxAgentManager::new(self.client.clone(), "EsxAgentManager")
    }

    /// Returns an instance of VsanObjectCatalog, which provides access to well-known vSAN managed objects.
    pub fn vsan_object_catalog(&self) -> VsanObjectCatalog {
        VsanObjectCatalog::new(self.client.clone())
    }

}

/// The VsanObjectCatalog struct provides methods to access the well vSAN managed objects on vCenter.
pub struct VsanObjectCatalog {
    client: VimClientHandle,
}

impl VsanObjectCatalog {
    pub fn new(client: VimClientHandle) -> Self {
        Self { client }
    }

    /// Returns the [`VimClient`](crate::core::client::VimClient) handle used by this VsanObjectCatalog instance.
    pub fn client(&self) -> VimClientHandle {
        self.client.clone()
    }

    /// Returns the vSAN disk management system for managing disk groups including all-flash disk
    /// support and detailed group information.
    pub fn disk_management_system(&self) -> VimClusterVsanVcDiskManagementSystem {
        VimClusterVsanVcDiskManagementSystem::new(self.client.clone(),
                                                  "vsan-disk-management-system")
    }

    /// vSAN stretched Cluster is a specific configuration implemented in
    /// environments where disaster/downtime avoidance is a key requirement.
    ///
    /// vSAN stretched Clusters with Witness Host refers to a deployment
    /// where a user sets up a vSAN cluster with 2 active/active sites with
    /// numbers of ESXi hosts between the two sites. The sites are connected via a
    /// high bandwidth/low latency link.
    ///
    /// Third site hosting the vSAN Witness Host is connected to both of the
    /// active/active data-sites. This connectivity can be via low bandwidth/high
    /// latency links.
    pub fn stretched_cluster_system(&self) -> VimClusterVsanVcStretchedClusterSystem {
        VimClusterVsanVcStretchedClusterSystem::new(self.client.clone(),
                                                        "vsan-stretched-cluster-system")
    }

    /// Comprehensive way to manage vSAN cluster configuration in below areas:
    /// - Enable or disable vSAN
    /// - Enable or disable Autoclaim mode for disk group
    /// - Enable or disable data efficiency feature
    /// - Configure vSAN iSCSI target feature
    /// - Manage disk groups
    /// - Manage fault domains
    /// - Retrieve vSAN generic configuration
    /// - Retrieve data efficiency configuration
    pub fn cluster_config_system(&self) -> VsanVcClusterConfigSystem {
        VsanVcClusterConfigSystem::new(self.client.clone(), "vsan-cluster-config-system")
    }

    /// This managed object type provides the service interface for obtaining
    /// statistical data about various aspects of vSAN performance, as generated
    /// and maintained by the vSAN performance service of the cluster.
    ///
    /// It also offers methods to enable/disable, configure and perform other
    /// maintenance tasks about the vSAN performance service.
    pub fn performance_manager(&self) -> VsanPerformanceManager {
        VsanPerformanceManager::new(self.client.clone(), "vsan-performance-manager")
    }

    /// This managed object provides access to vSAN Health related configuration
    /// and query APIs, operating at a vSAN cluster level.
    pub fn cluster_health_system(&self) -> VsanVcClusterHealthSystem {
        VsanVcClusterHealthSystem::new(self.client.clone(), "vsan-cluster-health-system")
    }

    /// Takes care of vSAN upgrade process and deprecates the vim.VsanUpgradeSystem
    ///
    /// Provides capability to support additional advanced disk format
    /// conversion specification, to help on configuration of
    /// latest vSAN advanced features.
    /// It supports following vSAN on-disk format versions:
    /// - version 1, released by vSAN 1.0, vSphere ESXi 5.5U1;
    /// - version 2, released by vSAN 6.0, which supports Virsto file
    ///   system;
    /// - version 3, released by vSAN 6.2, which supports vSAN
    ///   deduplication and compression.
    pub fn upgrade_system_ex(&self) -> VsanUpgradeSystemEx {
        VsanUpgradeSystemEx::new(self.client.clone(), "vsan-upgrade-systemex")
    }


    /// This managed object type provides the service interface to report the
    /// vSAN cluster space usage information including the space overview, the
    /// space usage breakdown to various vSAN object types and the vSAN data
    /// efficiency info after enabling vSAN deduplication.
    pub fn space_report_system(&self) -> VsanSpaceReportSystem {
        VsanSpaceReportSystem::new(self.client.clone(), "vsan-cluster-space-report-system")
    }

    /// This managed object type provides the service interface for setting the
    /// storage policy to one vSAN object, querying the vSAN object
    /// status information, i.e., its storage profile, its health status.
    pub fn cluster_object_system(&self) -> VsanObjectSystem {
        VsanObjectSystem::new(self.client.clone(), "vsan-cluster-object-system")
    }

    /// The VsanIscsiTargetSystem exposes interfaces from vCenter to perform
    /// vSAN iSCSI target service related operations.
    pub fn cluster_iscsi_target_system(&self) -> VsanIscsiTargetSystem {
        VsanIscsiTargetSystem::new(self.client.clone(), "vsan-cluster-iscsi-target-system")
    }

    /// Deployment engine that allows deployment of VCSA (Virtual Center Service
    /// Appliance) onto a vSAN datastore.
    pub fn vcsa_deployer_system(&self) -> VsanVcsaDeployerSystem {
        VsanVcsaDeployerSystem::new(self.client.clone(), "vsan-vcsa-deployer-system")
    }

    /// vSAN optimized methods for performing VDS related operations, especially
    /// migrations from VSS to VDS.
    pub fn vds_system(&self) -> VsanVdsSystem {
        VsanVdsSystem::new(self.client.clone(), "vsan-vds-system")
    }

    /// The VsanCapabilitySystem exposes interfaces to retrieve the supported
    /// capabilities on the current system.
    pub fn capability_system(&self) -> VsanCapabilitySystem {
        VsanCapabilitySystem::new(self.client.clone(), "vsan-vc-capability-system")
    }

    /// VsanMassCollector contains a collection of APIs to query vSAN management API's
    /// and values of managed object properties.
    pub fn mass_collector(&self) -> VsanMassCollector {
        VsanMassCollector::new(self.client.clone(), "vsan-mass-collector")
    }

    /// VsanPhoneHomeSystem contains a collection of APIs to perform online health
    /// checks.
    pub fn phonehome_system(&self) -> VsanPhoneHomeSystem {
        VsanPhoneHomeSystem::new(self.client.clone(), "vsan-phonehome-system")
    }

    /// As vSAN VUM integration feature is mostly an automatic backend
    /// feature, so currently only get and set config methods are available in its
    /// control path.
    pub fn vum_system(&self) -> VsanVumSystem {
        VsanVumSystem::new(self.client.clone(), "vsan-vum-system")
    }

    /// This managed object type provides interfaces to perform resource check
    /// and query for various operations, e.g.
    pub fn cluster_resource_check_system(&self) -> VsanResourceCheckSystem {
        VsanResourceCheckSystem::new(self.client.clone(), "vsan-cluster-resource-check-system")
    }


    /// This is the interface for managing the lifecycle of volumes that are consumed
    /// by containers or pods, in case of Kubernetes.
    pub fn cns_volume_manager(&self) -> CnsVolumeManager {
        CnsVolumeManager::new(self.client.clone(), "cns-volume-manager")
    }

    /// Returns the vSAN file service system for managing file service related configuration and
    /// queries.
    pub fn cluster_file_service_system(&self) -> VsanFileServiceSystem {
        VsanFileServiceSystem::new(self.client.clone(), "vsan-cluster-file-service-system")
    }

    /// Returns the vSAN remote datastore system for managing remote datastores in a vSAN environment.
    pub fn remote_datastore_system(&self) -> VsanRemoteDatastoreSystem {
        VsanRemoteDatastoreSystem::new(self.client.clone(), "vsan-remote-datastore-system")
    }

    /// Returns the vSAN IO insight manager for analyzing storage I/O patterns and performance.
    pub fn io_insight_manager(&self) -> VsanIoInsightManager {
        VsanIoInsightManager::new(self.client.clone(), "vsan-cluster-ioinsight-manager")
    }

    /// Returns the vSAN diagnostics system for troubleshooting and collecting cluster diagnostics.
    pub fn cluster_diagnostics_system(&self) -> VsanDiagnosticsSystem {
        VsanDiagnosticsSystem::new(self.client.clone(), "vsan-cluster-diagnostics-system")
    }

    /// Returns the vSAN cluster power system for managing power policies and energy efficiency.
    pub fn cluster_power_system(&self) -> VsanClusterPowerSystem {
        VsanClusterPowerSystem::new(self.client.clone(), "vsan-cluster-power-system")
    }
}