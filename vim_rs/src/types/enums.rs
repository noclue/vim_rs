/// Possible values:
/// - `Alarm`
/// - `AlarmManager`
/// - `AuthorizationManager`
/// - `CertificateManager`
/// - `ClusterComputeResource`
/// - `ClusterEVCManager`
/// - `ClusterProfile`
/// - `ClusterProfileManager`
/// - `ComputeResource`
/// - `ContainerView`
/// - `CryptoManager`
/// - `CryptoManagerHost`
/// - `CryptoManagerHostKMS`
/// - `CryptoManagerKmip`
/// - `CustomFieldsManager`
/// - `CustomizationSpecManager`
/// - `Datacenter`
/// - `Datastore`
/// - `DatastoreNamespaceManager`
/// - `DiagnosticManager`
/// - `DistributedVirtualPortgroup`
/// - `DistributedVirtualSwitch`
/// - `DistributedVirtualSwitchManager`
/// - `EnvironmentBrowser`
/// - `EventHistoryCollector`
/// - `EventManager`
/// - `ExtensibleManagedObject`
/// - `ExtensionManager`
/// - `FailoverClusterConfigurator`
/// - `FailoverClusterManager`
/// - `FileManager`
/// - `Folder`
/// - `GuestAliasManager`
/// - `GuestAuthManager`
/// - `GuestFileManager`
/// - `GuestOperationsManager`
/// - `GuestProcessManager`
/// - `GuestWindowsRegistryManager`
/// - `HealthUpdateManager`
/// - `HistoryCollector`
/// - `HostAccessManager`
/// - `HostActiveDirectoryAuthentication`
/// - `HostAssignableHardwareManager`
/// - `HostAuthenticationManager`
/// - `HostAuthenticationStore`
/// - `HostAutoStartManager`
/// - `HostBootDeviceSystem`
/// - `HostCacheConfigurationManager`
/// - `HostCertificateManager`
/// - `HostCpuSchedulerSystem`
/// - `HostDatastoreBrowser`
/// - `HostDatastoreSystem`
/// - `HostDateTimeSystem`
/// - `HostDiagnosticSystem`
/// - `HostDirectoryStore`
/// - `HostEsxAgentHostManager`
/// - `HostFirewallSystem`
/// - `HostFirmwareSystem`
/// - `HostGraphicsManager`
/// - `HostHealthStatusSystem`
/// - `HostImageConfigManager`
/// - `HostKernelModuleSystem`
/// - `HostLocalAccountManager`
/// - `HostLocalAuthentication`
/// - `HostMemorySystem`
/// - `HostNetworkSystem`
/// - `HostNvdimmSystem`
/// - `HostPatchManager`
/// - `HostPciPassthruSystem`
/// - `HostPowerSystem`
/// - `HostProfile`
/// - `HostProfileManager`
/// - `HostServiceSystem`
/// - `HostSnmpSystem`
/// - `HostSpecificationManager`
/// - `HostStorageSystem`
/// - `HostSystem`
/// - `HostVFlashManager`
/// - `HostVMotionSystem`
/// - `HostVStorageObjectManager`
/// - `HostVirtualNicManager`
/// - `HostVsanInternalSystem`
/// - `HostVsanSystem`
/// - `HttpNfcLease`
/// - `InventoryView`
/// - `IoFilterManager`
/// - `IpPoolManager`
/// - `IscsiManager`
/// - `LicenseAssignmentManager`
/// - `LicenseManager`
/// - `ListView`
/// - `LocalizationManager`
/// - `ManagedEntity`
/// - `ManagedObject`
/// - `ManagedObjectView`
/// - `MessageBusProxy`
/// - `Network`
/// - `OpaqueNetwork`
/// - `OptionManager`
/// - `OverheadMemoryManager`
/// - `OvfManager`
/// - `PerformanceManager`
/// - `Profile`
/// - `ProfileComplianceManager`
/// - `ProfileManager`
/// - `PropertyCollector`
/// - `PropertyFilter`
/// - `ResourcePlanningManager`
/// - `ResourcePool`
/// - `ScheduledTask`
/// - `ScheduledTaskManager`
/// - `SearchIndex`
/// - `ServiceInstance`
/// - `ServiceManager`
/// - `SessionManager`
/// - `SimpleCommand`
/// - `SiteInfoManager`
/// - `StoragePod`
/// - `StorageQueryManager`
/// - `StorageResourceManager`
/// - `Task`
/// - `TaskHistoryCollector`
/// - `TaskManager`
/// - `TenantTenantManager`
/// - `UserDirectory`
/// - `VStorageObjectManagerBase`
/// - `VcenterVStorageObjectManager`
/// - `View`
/// - `ViewManager`
/// - `VirtualApp`
/// - `VirtualDiskManager`
/// - `VirtualMachine`
/// - `VirtualMachineCompatibilityChecker`
/// - `VirtualMachineGuestCustomizationManager`
/// - `VirtualMachineProvisioningChecker`
/// - `VirtualMachineSnapshot`
/// - `VirtualizationManager`
/// - `VmwareDistributedVirtualSwitch`
/// - `VsanUpgradeSystem`
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum MoTypesEnum {
    Alarm,
    AlarmManager,
    AuthorizationManager,
    CertificateManager,
    ClusterComputeResource,
    #[serde(rename = "ClusterEVCManager")]
    #[strum(serialize = "ClusterEVCManager")]
    ClusterEvcManager,
    ClusterProfile,
    ClusterProfileManager,
    ComputeResource,
    ContainerView,
    CryptoManager,
    CryptoManagerHost,
    #[serde(rename = "CryptoManagerHostKMS")]
    #[strum(serialize = "CryptoManagerHostKMS")]
    CryptoManagerHostKms,
    CryptoManagerKmip,
    CustomFieldsManager,
    CustomizationSpecManager,
    Datacenter,
    Datastore,
    DatastoreNamespaceManager,
    DiagnosticManager,
    DistributedVirtualPortgroup,
    DistributedVirtualSwitch,
    DistributedVirtualSwitchManager,
    EnvironmentBrowser,
    EventHistoryCollector,
    EventManager,
    ExtensibleManagedObject,
    ExtensionManager,
    FailoverClusterConfigurator,
    FailoverClusterManager,
    FileManager,
    Folder,
    GuestAliasManager,
    GuestAuthManager,
    GuestFileManager,
    GuestOperationsManager,
    GuestProcessManager,
    GuestWindowsRegistryManager,
    HealthUpdateManager,
    HistoryCollector,
    HostAccessManager,
    HostActiveDirectoryAuthentication,
    HostAssignableHardwareManager,
    HostAuthenticationManager,
    HostAuthenticationStore,
    HostAutoStartManager,
    HostBootDeviceSystem,
    HostCacheConfigurationManager,
    HostCertificateManager,
    HostCpuSchedulerSystem,
    HostDatastoreBrowser,
    HostDatastoreSystem,
    HostDateTimeSystem,
    HostDiagnosticSystem,
    HostDirectoryStore,
    HostEsxAgentHostManager,
    HostFirewallSystem,
    HostFirmwareSystem,
    HostGraphicsManager,
    HostHealthStatusSystem,
    HostImageConfigManager,
    HostKernelModuleSystem,
    HostLocalAccountManager,
    HostLocalAuthentication,
    HostMemorySystem,
    HostNetworkSystem,
    HostNvdimmSystem,
    HostPatchManager,
    HostPciPassthruSystem,
    HostPowerSystem,
    HostProfile,
    HostProfileManager,
    HostServiceSystem,
    HostSnmpSystem,
    HostSpecificationManager,
    HostStorageSystem,
    HostSystem,
    HostVFlashManager,
    HostVMotionSystem,
    HostVStorageObjectManager,
    HostVirtualNicManager,
    HostVsanInternalSystem,
    HostVsanSystem,
    HttpNfcLease,
    InventoryView,
    IoFilterManager,
    IpPoolManager,
    IscsiManager,
    LicenseAssignmentManager,
    LicenseManager,
    ListView,
    LocalizationManager,
    ManagedEntity,
    ManagedObject,
    ManagedObjectView,
    MessageBusProxy,
    Network,
    OpaqueNetwork,
    OptionManager,
    OverheadMemoryManager,
    OvfManager,
    PerformanceManager,
    Profile,
    ProfileComplianceManager,
    ProfileManager,
    PropertyCollector,
    PropertyFilter,
    ResourcePlanningManager,
    ResourcePool,
    ScheduledTask,
    ScheduledTaskManager,
    SearchIndex,
    ServiceInstance,
    ServiceManager,
    SessionManager,
    SimpleCommand,
    SiteInfoManager,
    StoragePod,
    StorageQueryManager,
    StorageResourceManager,
    Task,
    TaskHistoryCollector,
    TaskManager,
    TenantTenantManager,
    UserDirectory,
    VStorageObjectManagerBase,
    VcenterVStorageObjectManager,
    View,
    ViewManager,
    VirtualApp,
    VirtualDiskManager,
    VirtualMachine,
    VirtualMachineCompatibilityChecker,
    VirtualMachineGuestCustomizationManager,
    VirtualMachineProvisioningChecker,
    VirtualMachineSnapshot,
    VirtualizationManager,
    VmwareDistributedVirtualSwitch,
    VsanUpgradeSystem,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enum representing result of batch-APis.
/// 
/// Possible values:
/// - `success`
/// - `fail`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum BatchResultResultEnum {
    #[serde(rename = "success")]
    #[strum(serialize = "success")]
    Success,
    #[serde(rename = "fail")]
    #[strum(serialize = "fail")]
    Fail,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// HCIWorkflowState identifies the state of the cluser from the perspective of HCI
/// workflow.
/// 
/// The workflow begins with in\_progress mode and can transition
/// to 'done' or 'invalid', both of which are terminal states.
/// 
/// Possible values:
/// - `in_progress`: Indicates cluster is getting configured or will be configured.
/// - `done`: Indicates cluster configuration is complete.
/// - `invalid`: Indicates the workflow was abandoned on the cluster before the
///   configuration could complete.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterComputeResourceHciWorkflowStateEnum {
    #[serde(rename = "in_progress")]
    #[strum(serialize = "in_progress")]
    InProgress,
    #[serde(rename = "done")]
    #[strum(serialize = "done")]
    Done,
    #[serde(rename = "invalid")]
    #[strum(serialize = "invalid")]
    Invalid,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `healthy`: Indicates vCS health status is normal.
/// - `degraded`: Indicates only vCS is unhealthy.
/// - `nonhealthy`: Indicates vCS is unhealthy and other cluster services are impacted.
///   
/// ***Since:*** vSphere API Release 7.0.1.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterComputeResourceVcsHealthStatusEnum {
    #[serde(rename = "healthy")]
    #[strum(serialize = "healthy")]
    Healthy,
    #[serde(rename = "degraded")]
    #[strum(serialize = "degraded")]
    Degraded,
    #[serde(rename = "nonhealthy")]
    #[strum(serialize = "nonhealthy")]
    Nonhealthy,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The SPBM(Storage Policy Based Management) license state for a host
/// 
/// Possible values:
/// - `licensed`: The host is licensed
/// - `unlicensed`: The host is not licensed
/// - `unknown`: The host license information is unknown, this could happen if the
///   host is not in a available state
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ComputeResourceHostSpbmLicenseInfoHostSpbmLicenseStateEnum {
    #[serde(rename = "licensed")]
    #[strum(serialize = "licensed")]
    Licensed,
    #[serde(rename = "unlicensed")]
    #[strum(serialize = "unlicensed")]
    Unlicensed,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Config spec operation type.
/// 
/// Possible values:
/// - `add`: Indicates the addition of an element to the configuration.
/// - `edit`: Indicates the change of an element in the configuration.
/// - `remove`: Indicates the removal of an element in the configuration.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ConfigSpecOperationEnum {
    #[serde(rename = "add")]
    #[strum(serialize = "add")]
    Add,
    #[serde(rename = "edit")]
    #[strum(serialize = "edit")]
    Edit,
    #[serde(rename = "remove")]
    #[strum(serialize = "remove")]
    Remove,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `True`: Is accessible
/// - `False`: Is not accessible
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DatastoreAccessibleEnum {
    True,
    False,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines the current maintenance mode state of the datastore.
/// 
/// Possible values:
/// - `normal`: Default state.
/// - `enteringMaintenance`: Started entering maintenance mode, but not finished.
///   
///   This could happen when waiting for user input or for
///   long-running vmotions to complete.
/// - `inMaintenance`: Successfully entered maintenance mode.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DatastoreSummaryMaintenanceModeStateEnum {
    #[serde(rename = "normal")]
    #[strum(serialize = "normal")]
    Normal,
    #[serde(rename = "enteringMaintenance")]
    #[strum(serialize = "enteringMaintenance")]
    EnteringMaintenance,
    #[serde(rename = "inMaintenance")]
    #[strum(serialize = "inMaintenance")]
    InMaintenance,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Pre-defined constants for possible creators of log files.
/// 
/// Possible values:
/// - `vpxd`: VirtualCenter service
/// - `vpxa`: VirtualCenter agent
/// - `hostd`: Host agent
/// - `serverd`: Host server agent
/// - `install`: Installation
/// - `vpxClient`: Virtual infrastructure client
/// - `recordLog`: System Record Log
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DiagnosticManagerLogCreatorEnum {
    #[serde(rename = "vpxd")]
    #[strum(serialize = "vpxd")]
    Vpxd,
    #[serde(rename = "vpxa")]
    #[strum(serialize = "vpxa")]
    Vpxa,
    #[serde(rename = "hostd")]
    #[strum(serialize = "hostd")]
    Hostd,
    #[serde(rename = "serverd")]
    #[strum(serialize = "serverd")]
    Serverd,
    #[serde(rename = "install")]
    #[strum(serialize = "install")]
    Install,
    #[serde(rename = "vpxClient")]
    #[strum(serialize = "vpxClient")]
    VpxClient,
    #[serde(rename = "recordLog")]
    #[strum(serialize = "recordLog")]
    RecordLog,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Constants for defined formats.
/// 
/// For more information, see the comment for the format property.
/// 
/// Possible values:
/// - `plain`: A standard ASCII-based line-based log file.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DiagnosticManagerLogFormatEnum {
    #[serde(rename = "plain")]
    #[strum(serialize = "plain")]
    Plain,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// List of possible host infrastructure traffic classes
/// 
/// Possible values:
/// - `management`: Management Traffic
/// - `faultTolerance`: Fault Tolerance (FT) Traffic
/// - `vmotion`: vMotion Traffic
/// - `virtualMachine`: Virtual Machine Traffic
/// - `iSCSI`: iSCSI Traffic
/// - `nfs`: NFS Traffic
/// - `hbr`: vSphere Replication (VR) Traffic
/// - `vsan`: vSphere Storage Area Network Traffic
/// - `vdp`: vSphere Data Protection - Backup Traffic
/// - `backupNfc`: vSphere Backup NFC Traffic
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `nvmetcp`: vSphere NVMETCP Traffic
///   
///   ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DistributedVirtualSwitchHostInfrastructureTrafficClassEnum {
    #[serde(rename = "management")]
    #[strum(serialize = "management")]
    Management,
    #[serde(rename = "faultTolerance")]
    #[strum(serialize = "faultTolerance")]
    FaultTolerance,
    #[serde(rename = "vmotion")]
    #[strum(serialize = "vmotion")]
    Vmotion,
    #[serde(rename = "virtualMachine")]
    #[strum(serialize = "virtualMachine")]
    VirtualMachine,
    #[serde(rename = "iSCSI")]
    #[strum(serialize = "iSCSI")]
    IScsi,
    #[serde(rename = "nfs")]
    #[strum(serialize = "nfs")]
    Nfs,
    #[serde(rename = "hbr")]
    #[strum(serialize = "hbr")]
    Hbr,
    #[serde(rename = "vsan")]
    #[strum(serialize = "vsan")]
    Vsan,
    #[serde(rename = "vdp")]
    #[strum(serialize = "vdp")]
    Vdp,
    #[serde(rename = "backupNfc")]
    #[strum(serialize = "backupNfc")]
    BackupNfc,
    #[serde(rename = "nvmetcp")]
    #[strum(serialize = "nvmetcp")]
    Nvmetcp,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Network resource control version types.
/// 
/// Possible values:
/// - `version2`: Network Resource Control API version 2
/// - `version3`: Network Resource Control API version 3
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DistributedVirtualSwitchNetworkResourceControlVersionEnum {
    #[serde(rename = "version2")]
    #[strum(serialize = "version2")]
    Version2,
    #[serde(rename = "version3")]
    #[strum(serialize = "version3")]
    Version3,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// List of possible teaming modes supported by the vNetwork Distributed
/// Switch.
/// 
/// The different policy modes define the way traffic is routed
/// through the different uplink ports in a team.
/// 
/// Possible values:
/// - `loadbalance_ip`: Routing based on IP hash
/// - `loadbalance_srcmac`: Route based on source MAC hash
/// - `loadbalance_srcid`: Route based on the source of the port ID
/// - `failover_explicit`: Use explicit failover order
/// - `loadbalance_loadbased`: Routing based by dynamically balancing traffic through the NICs
///   in a team.
///   
///   This is the recommended teaming policy when the
///   network I/O control feature is enabled for the vNetwork
///   Distributed Switch.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DistributedVirtualSwitchNicTeamingPolicyModeEnum {
    #[serde(rename = "loadbalance_ip")]
    #[strum(serialize = "loadbalance_ip")]
    LoadbalanceIp,
    #[serde(rename = "loadbalance_srcmac")]
    #[strum(serialize = "loadbalance_srcmac")]
    LoadbalanceSrcmac,
    #[serde(rename = "loadbalance_srcid")]
    #[strum(serialize = "loadbalance_srcid")]
    LoadbalanceSrcid,
    #[serde(rename = "failover_explicit")]
    #[strum(serialize = "failover_explicit")]
    FailoverExplicit,
    #[serde(rename = "loadbalance_loadbased")]
    #[strum(serialize = "loadbalance_loadbased")]
    LoadbalanceLoadbased,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The product spec operation types.
/// 
/// Possible values:
/// - `preInstall`: Push the switch's host component of the specified product info to the
///   host members of the switch at a fixed location known by the host.
/// - `upgrade`: Change the switch implementation to use the specified one.
///   
///   If the
///   property values in the specified product info are different from
///   those of the corresponding properties in the switch's product info,
///   a host component preinstall and switch upgrade will be performed.
/// - `notifyAvailableUpgrade`: Set the product information for an available switch upgrade that
///   would be done by the switch implementation.
///   
///   This operation will post
///   a config issue on the switch to signal the availability of an upgrade.
///   This operation is applicable only in the case when switch policy
///   *DVSPolicy.autoUpgradeAllowed*
///   is set to false.
/// - `proceedWithUpgrade`: If productSpec is set to be same as that in the
///   *DvsUpgradeAvailableEvent* configIssue, the switch
///   implementation will proceed with the upgrade.
///   
///   To reject or stop the
///   upgrade, leave the productSpec unset. If productSpec is set but does not
///   match that in *DvsUpgradeAvailableEvent* configIssue,
///   a fault will be raised.
///   This operation is applicable only in the case when switch policy
///   *DVSPolicy.autoUpgradeAllowed*
///   is set to false.
/// - `updateBundleInfo`: Update the bundle URL and ID information.
///   
///   If other properties in
///   the specified product info differ from the
///   corresponding properties of the switch's product info, a fault will
///   be thrown. Updating the bundle ID will result in installing the new host
///   component identified by the bundle ID.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DistributedVirtualSwitchProductSpecOperationTypeEnum {
    #[serde(rename = "preInstall")]
    #[strum(serialize = "preInstall")]
    PreInstall,
    #[serde(rename = "upgrade")]
    #[strum(serialize = "upgrade")]
    Upgrade,
    #[serde(rename = "notifyAvailableUpgrade")]
    #[strum(serialize = "notifyAvailableUpgrade")]
    NotifyAvailableUpgrade,
    #[serde(rename = "proceedWithUpgrade")]
    #[strum(serialize = "proceedWithUpgrade")]
    ProceedWithUpgrade,
    #[serde(rename = "updateBundleInfo")]
    #[strum(serialize = "updateBundleInfo")]
    UpdateBundleInfo,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Correlation state as computed by storageRM
/// module on host.
/// 
/// Possible values:
/// - `Correlated`
/// - `Uncorrelated`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DrsInjectorWorkloadCorrelationStateEnum {
    Correlated,
    Uncorrelated,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `maintenance`: Add host in maintenance mode.
/// - `non_maintenance`: Add host in non-maintenance mode.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum FolderDesiredHostStateEnum {
    #[serde(rename = "maintenance")]
    #[strum(serialize = "maintenance")]
    Maintenance,
    #[serde(rename = "non_maintenance")]
    #[strum(serialize = "non_maintenance")]
    NonMaintenance,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Describes the current state of a replicated *VirtualMachine*
/// 
/// Possible values:
/// - `none`: The *VirtualMachine* has no current replication state.
///   
///   This is a virtual machine that is configured for replication, but is
///   powered off and not undergoing offline replication.
/// - `paused`: The *VirtualMachine* replication is paused.
/// - `syncing`: One or more of the *VirtualMachine* disks is in the
///   process of an initial synchronization with the remote site.
/// - `idle`: The *VirtualMachine* is being replicated but is not
///   currently in the process of having a consistent instance created.
/// - `active`: The *VirtualMachine* is in the process of having
///   a consistent instance created.
/// - `error`: The *VirtualMachine* is unable to replicate due to
///   errors.
///   
///   XXX Currently unused.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ReplicationVmStateEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "paused")]
    #[strum(serialize = "paused")]
    Paused,
    #[serde(rename = "syncing")]
    #[strum(serialize = "syncing")]
    Syncing,
    #[serde(rename = "idle")]
    #[strum(serialize = "idle")]
    Idle,
    #[serde(rename = "active")]
    #[strum(serialize = "active")]
    Active,
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Quiescing is a boolean flag in *ReplicationConfigSpec*
/// and QuiesceModeType describes the supported quiesce mode
/// for *VirtualMachine*.
/// 
/// If application quiescing fails, HBR would attempt
/// filesystem quiescing and if even filesystem quiescing
/// fails, then we would just create a crash consistent
/// instance.
/// 
/// Possible values:
/// - `application`: HBR supports application quescing for this
///   *VirtualMachine*.
/// - `filesystem`: HBR supports filesystem quescing for this
///   *VirtualMachine*.
/// - `none`: HBR does not support quescing for this
///   *VirtualMachine*.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum QuiesceModeEnum {
    #[serde(rename = "application")]
    #[strum(serialize = "application")]
    Application,
    #[serde(rename = "filesystem")]
    #[strum(serialize = "filesystem")]
    Filesystem,
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `Memory`
/// - `Power`
/// - `Fan`
/// - `Network`
/// - `Storage`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HealthUpdateInfoComponentTypeEnum {
    Memory,
    Power,
    Fan,
    Network,
    Storage,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines a host's connection state.
/// 
/// Possible values:
/// - `connected`: Connected to the server.
///   
///   For ESX Server, this is always the setting.
/// - `notResponding`: VirtualCenter is not receiving heartbeats from the server.
///   
///   The state
///   automatically changes to connected once heartbeats are received
///   again. This state is typically used to trigger an alarm on the host.
/// - `disconnected`: The user has explicitly taken the host down.
///   
///   VirtualCenter does not expect to
///   receive heartbeats from the host. The next time a heartbeat is received, the
///   host is moved to the connected state again and an event is logged.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostSystemConnectionStateEnum {
    #[serde(rename = "connected")]
    #[strum(serialize = "connected")]
    Connected,
    #[serde(rename = "notResponding")]
    #[strum(serialize = "notResponding")]
    NotResponding,
    #[serde(rename = "disconnected")]
    #[strum(serialize = "disconnected")]
    Disconnected,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines a host's encryption state
/// 
/// Possible values:
/// - `incapable`: The host is not safe for receiving sensitive material.
/// - `prepared`: The host is prepared for receiving sensitive material
///   but does not have a host key set yet.
/// - `safe`: The host is crypto safe and has a host key set.
/// - `pendingIncapable`: The host is explicitly crypto disabled and pending reboot to be
///   applied.
///   
///   When host is in this state, creating encrypted virtual
///   machines is not allowed, but still need a reboot to totally clean
///   up and enter incapable state.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostCryptoStateEnum {
    #[serde(rename = "incapable")]
    #[strum(serialize = "incapable")]
    Incapable,
    #[serde(rename = "prepared")]
    #[strum(serialize = "prepared")]
    Prepared,
    #[serde(rename = "safe")]
    #[strum(serialize = "safe")]
    Safe,
    #[serde(rename = "pendingIncapable")]
    #[strum(serialize = "pendingIncapable")]
    PendingIncapable,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines a host's power state.
/// 
/// Possible values:
/// - `poweredOn`: The host is powered on.
///   
///   A host that is entering standby mode
///   *entering* is also in this state.
/// - `poweredOff`: The host was specifically powered off by the user through
///   VirtualCenter.
///   
///   This state is not a cetain state, because
///   after VirtualCenter issues the command to power off the host,
///   the host might crash, or kill all the processes but fail to
///   power off.
/// - `standBy`: The host was specifically put in standby mode, either
///   explicitly by the user, or automatically by DPM.
///   
///   This state
///   is not a cetain state, because after VirtualCenter issues the
///   command to put the host in standby state, the host might
///   crash, or kill all the processes but fail to power off. A host
///   that is exiting standby mode *exiting*
///   is also in this state.
/// - `unknown`: If the host is disconnected, or notResponding, we cannot
///   possibly have knowledge of its power state.
///   
///   Hence, the host
///   is marked as unknown.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostSystemPowerStateEnum {
    #[serde(rename = "poweredOn")]
    #[strum(serialize = "poweredOn")]
    PoweredOn,
    #[serde(rename = "poweredOff")]
    #[strum(serialize = "poweredOff")]
    PoweredOff,
    #[serde(rename = "standBy")]
    #[strum(serialize = "standBy")]
    StandBy,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Valid state for host profile remediation.
/// 
/// Possible values:
/// - `remediationReady`: Before precheck remediation and remediation.
/// - `precheckRemediationRunning`: Preecheck remediation is running.
/// - `precheckRemediationComplete`: Preecheck remediation succeeded.
/// - `precheckRemediationFailed`: Preecheck remediation failed.
/// - `remediationRunning`: Remediation is running.
/// - `remediationFailed`: Remediation failed.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostSystemRemediationStateStateEnum {
    #[serde(rename = "remediationReady")]
    #[strum(serialize = "remediationReady")]
    RemediationReady,
    #[serde(rename = "precheckRemediationRunning")]
    #[strum(serialize = "precheckRemediationRunning")]
    PrecheckRemediationRunning,
    #[serde(rename = "precheckRemediationComplete")]
    #[strum(serialize = "precheckRemediationComplete")]
    PrecheckRemediationComplete,
    #[serde(rename = "precheckRemediationFailed")]
    #[strum(serialize = "precheckRemediationFailed")]
    PrecheckRemediationFailed,
    #[serde(rename = "remediationRunning")]
    #[strum(serialize = "remediationRunning")]
    RemediationRunning,
    #[serde(rename = "remediationFailed")]
    #[strum(serialize = "remediationFailed")]
    RemediationFailed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines a host's standby mode.
/// 
/// Possible values:
/// - `entering`: The host is entering standby mode.
/// - `exiting`: The host is exiting standby mode.
/// - `in`: The host is in standby mode.
/// - `none`: The host is not in standy mode, and it is not
///   in the process of entering/exiting standby mode.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostStandbyModeEnum {
    #[serde(rename = "entering")]
    #[strum(serialize = "entering")]
    Entering,
    #[serde(rename = "exiting")]
    #[strum(serialize = "exiting")]
    Exiting,
    #[serde(rename = "in")]
    #[strum(serialize = "in")]
    In,
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// List of supported algorithms for checksum calculation.
/// 
/// Possible values:
/// - `sha1`
/// - `sha256`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HttpNfcLeaseManifestEntryChecksumTypeEnum {
    #[serde(rename = "sha1")]
    #[strum(serialize = "sha1")]
    Sha1,
    #[serde(rename = "sha256")]
    #[strum(serialize = "sha256")]
    Sha256,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// List of supported modes by HttpNfcLease
/// 
/// Possible values:
/// - `pushOrGet`: Client pushes or downloads individual files from/to
///   each host/url provided by this lease in *HttpNfcLease.info*
/// - `pull`: Mode where hosts itself pull files from source URLs.
///   
///   See *HttpNfcLease.HttpNfcLeasePullFromUrls_Task*
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HttpNfcLeaseModeEnum {
    #[serde(rename = "pushOrGet")]
    #[strum(serialize = "pushOrGet")]
    PushOrGet,
    #[serde(rename = "pull")]
    #[strum(serialize = "pull")]
    Pull,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// List of possible states of a lease.
/// 
/// Possible values:
/// - `initializing`: When the lease is being initialized.
/// - `ready`: When the lease is ready and disks may be transferred.
/// - `done`: When the import/export session is completed, and the lease
///   is no longer held.
/// - `error`: When an error has occurred.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HttpNfcLeaseStateEnum {
    #[serde(rename = "initializing")]
    #[strum(serialize = "initializing")]
    Initializing,
    #[serde(rename = "ready")]
    #[strum(serialize = "ready")]
    Ready,
    #[serde(rename = "done")]
    #[strum(serialize = "done")]
    Done,
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines the type of an IO Filter.
/// 
/// Possible values:
/// - `cache`: Cache.
/// - `replication`: Replication.
/// - `encryption`: Encryption.
/// - `compression`: Compression.
/// - `inspection`: Inspection.
/// - `datastoreIoControl`: Datastore I/O Control.
/// - `dataProvider`: Data Provider.
/// - `dataCapture`: Lightweight Data Capture.
///   
///   ***Since:*** vSphere API Release 7.0.2.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum IoFilterTypeEnum {
    #[serde(rename = "cache")]
    #[strum(serialize = "cache")]
    Cache,
    #[serde(rename = "replication")]
    #[strum(serialize = "replication")]
    Replication,
    #[serde(rename = "encryption")]
    #[strum(serialize = "encryption")]
    Encryption,
    #[serde(rename = "compression")]
    #[strum(serialize = "compression")]
    Compression,
    #[serde(rename = "inspection")]
    #[strum(serialize = "inspection")]
    Inspection,
    #[serde(rename = "datastoreIoControl")]
    #[strum(serialize = "datastoreIoControl")]
    DatastoreIoControl,
    #[serde(rename = "dataProvider")]
    #[strum(serialize = "dataProvider")]
    DataProvider,
    #[serde(rename = "dataCapture")]
    #[strum(serialize = "dataCapture")]
    DataCapture,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines the type of operation for an IO Filter.
/// 
/// Possible values:
/// - `install`: Install an IO Filter.
/// - `uninstall`: Uninstall an IO Filter.
/// - `upgrade`: Upgrade an IO Filter.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum IoFilterOperationEnum {
    #[serde(rename = "install")]
    #[strum(serialize = "install")]
    Install,
    #[serde(rename = "uninstall")]
    #[strum(serialize = "uninstall")]
    Uninstall,
    #[serde(rename = "upgrade")]
    #[strum(serialize = "upgrade")]
    Upgrade,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enumeration of the nominal latency-sensitive values which can be
/// used to specify the latency-sensitivity level of the application.
/// 
/// In terms of latency-sensitivity the values relate:
/// high&gt;medium&gt;normal&gt;low.
/// 
/// Possible values:
/// - `low`: The relative latency-sensitivity low value.
/// - `normal`: The relative latency-sensitivity normal value.
///   
///   This is the default latency-sensitivity value.
/// - `medium`: The relative latency-sensitivity medium value.
/// - `high`: The relative latency-sensitivity high value.
/// - `custom`: 
///   
///   Deprecated as of vSphere API Ver 6.0. Value will be ignored and
///   treated as "normal" latency sensitivity.
///   
///   The custom absolute latency-sensitivity specified in
///   *LatencySensitivity.sensitivity* property is used to
///   define the latency-sensitivity.
///   
///   When this value is set to *LatencySensitivity.level* the
///   *LatencySensitivity.sensitivity* property should be
///   set also.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum LatencySensitivitySensitivityLevelEnum {
    #[serde(rename = "low")]
    #[strum(serialize = "low")]
    Low,
    #[serde(rename = "normal")]
    #[strum(serialize = "normal")]
    Normal,
    #[serde(rename = "medium")]
    #[strum(serialize = "medium")]
    Medium,
    #[serde(rename = "high")]
    #[strum(serialize = "high")]
    High,
    #[serde(rename = "custom")]
    #[strum(serialize = "custom")]
    Custom,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Cost units apply to licenses for the purpose of determining
/// how many licenses are needed.
/// 
/// Possible values:
/// - `host`: One license is acquired per host.
/// - `cpuCore`: One license is acquired per CPU core.
/// - `cpuPackage`: One license is acquired per CPU package.
/// - `server`: One license is acquired per server.
/// - `vm`: One license is acquired per virtual machine.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum LicenseFeatureInfoUnitEnum {
    #[serde(rename = "host")]
    #[strum(serialize = "host")]
    Host,
    #[serde(rename = "cpuCore")]
    #[strum(serialize = "cpuCore")]
    CpuCore,
    #[serde(rename = "cpuPackage")]
    #[strum(serialize = "cpuPackage")]
    CpuPackage,
    #[serde(rename = "server")]
    #[strum(serialize = "server")]
    Server,
    #[serde(rename = "vm")]
    #[strum(serialize = "vm")]
    Vm,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Some licenses may only be allowed to load from a specified source.
/// 
/// This enum indicates what restrictions exist for this license if any.
/// 
/// Possible values:
/// - `unrestricted`: The feature does not have a source restriction.
/// - `served`: The feature's license can only be served.
/// - `file`: The feature's license can only come from a file.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum LicenseFeatureInfoSourceRestrictionEnum {
    #[serde(rename = "unrestricted")]
    #[strum(serialize = "unrestricted")]
    Unrestricted,
    #[serde(rename = "served")]
    #[strum(serialize = "served")]
    Served,
    #[serde(rename = "file")]
    #[strum(serialize = "file")]
    File,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Describes the state of the feature.
/// 
/// Possible values:
/// - `enabled`: The current edition license has implicitly enabled this additional feature.
/// - `disabled`: The current edition license does not allow this additional feature.
/// - `optional`: The current edition license allows this additional feature.
///   
///   The
///   *LicenseManager.EnableFeature* and *LicenseManager.DisableFeature* methods can be used to enable or disable
///   this feature.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum LicenseFeatureInfoStateEnum {
    #[serde(rename = "enabled")]
    #[strum(serialize = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "optional")]
    #[strum(serialize = "optional")]
    Optional,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Identifiers of currently supported resources.
/// 
/// Possible values:
/// - `numCpuPackages`: Number of CPU packages on this host.
/// - `numCpuCores`: Number of licensable CPU cores/compute-units on this host.
/// - `memorySize`: Total size of memory installed on this host, measured in kilobytes.
/// - `memoryForVms`: Total size of memory configured for VMs on this host, measured in kilobytes.
/// - `numVmsStarted`: Number of VMs already running on this host.
/// - `numVmsStarting`: Number of VMs that are currently powering-on, immigrating, etc.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostLicensableResourceKeyEnum {
    #[serde(rename = "numCpuPackages")]
    #[strum(serialize = "numCpuPackages")]
    NumCpuPackages,
    #[serde(rename = "numCpuCores")]
    #[strum(serialize = "numCpuCores")]
    NumCpuCores,
    #[serde(rename = "memorySize")]
    #[strum(serialize = "memorySize")]
    MemorySize,
    #[serde(rename = "memoryForVms")]
    #[strum(serialize = "memoryForVms")]
    MemoryForVms,
    #[serde(rename = "numVmsStarted")]
    #[strum(serialize = "numVmsStarted")]
    NumVmsStarted,
    #[serde(rename = "numVmsStarting")]
    #[strum(serialize = "numVmsStarting")]
    NumVmsStarting,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated as of VI API 2.5, use *LicenseManager.QueryLicenseSourceAvailability*
/// to obtain an array of *LicenseAvailabilityInfo* data
/// objects.
/// 
/// Licensed features have unique keys to identify them.
/// 
/// Possible values:
/// - `esxFull`: The edition license for the ESX Server, Standard edition.
///   
///   This is a per
///   CPU package license.
/// - `esxVmtn`: The edition license for the ESX server, VMTN edition.
///   
///   This is a per CPU package
///   license.
/// - `esxExpress`: The edition license for the ESX server, Starter edition.
///   
///   This is a per CPU
///   package license.
/// - `san`: Enable use of SAN.
///   
///   This is a per CPU package license.
/// - `iscsi`: Enable use of iSCSI.
///   
///   This is a per CPU package license.
/// - `nas`: Enable use of NAS.
///   
///   This is a per CPU package license.
/// - `vsmp`: Enable up to 4-way VSMP feature.
///   
///   This is a per CPU package license.
/// - `backup`: Enable ESX Server consolidated backup feature.
///   
///   This is a per CPU package
///   license.
/// - `vc`: The edition license for a VirtualCenter server, full edition.
///   
///   This license
///   is independent of the number of CPU packages for the VirtualCenter host.
/// - `vcExpress`: The edition license for a VirtualCenter server, starter edition.
///   
///   This license
///   limits the number of hosts (esxHost or serverHost) that can be managed by the
///   VirtualCenter product.
/// - `esxHost`: Enable VirtualCenter ESX Server host management functionality.
///   
///   This is a per
///   ESX server CPU package license.
/// - `gsxHost`: Enable VirtualCenter GSX Server host management functionality.
///   
///   This is a per
///   GSX server CPU package license.
/// - `serverHost`: Enable VirtualCenter VMware server host management functionality.
///   
///   This is a per
///   VMware server CPU package license.
/// - `drsPower`: Enable VirtualCenter DRS Power Management Functionality.
///   
///   This is a per CPU package
/// - `vmotion`: Enable VMotion.
///   
///   This is a per ESX server CPU package license.
/// - `drs`: Enable VirtualCenter Distributed Resource Scheduler.
///   
///   This is a per ESX server
///   CPU package license.
/// - `das`: Enable VirtualCenter HA.
///   
///   This is a per ESX server CPU package license.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum LicenseManagerLicenseKeyEnum {
    #[serde(rename = "esxFull")]
    #[strum(serialize = "esxFull")]
    EsxFull,
    #[serde(rename = "esxVmtn")]
    #[strum(serialize = "esxVmtn")]
    EsxVmtn,
    #[serde(rename = "esxExpress")]
    #[strum(serialize = "esxExpress")]
    EsxExpress,
    #[serde(rename = "san")]
    #[strum(serialize = "san")]
    San,
    #[serde(rename = "iscsi")]
    #[strum(serialize = "iscsi")]
    Iscsi,
    #[serde(rename = "nas")]
    #[strum(serialize = "nas")]
    Nas,
    #[serde(rename = "vsmp")]
    #[strum(serialize = "vsmp")]
    Vsmp,
    #[serde(rename = "backup")]
    #[strum(serialize = "backup")]
    Backup,
    #[serde(rename = "vc")]
    #[strum(serialize = "vc")]
    Vc,
    #[serde(rename = "vcExpress")]
    #[strum(serialize = "vcExpress")]
    VcExpress,
    #[serde(rename = "esxHost")]
    #[strum(serialize = "esxHost")]
    EsxHost,
    #[serde(rename = "gsxHost")]
    #[strum(serialize = "gsxHost")]
    GsxHost,
    #[serde(rename = "serverHost")]
    #[strum(serialize = "serverHost")]
    ServerHost,
    #[serde(rename = "drsPower")]
    #[strum(serialize = "drsPower")]
    DrsPower,
    #[serde(rename = "vmotion")]
    #[strum(serialize = "vmotion")]
    Vmotion,
    #[serde(rename = "drs")]
    #[strum(serialize = "drs")]
    Drs,
    #[serde(rename = "das")]
    #[strum(serialize = "das")]
    Das,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated as of vSphere API 4.0, this is not used by the system.
/// 
/// State of licensing subsystem.
/// 
/// Possible values:
/// - `initializing`: Setting or resetting configuration in progress.
/// - `normal`: Running within operating parameters.
/// - `marginal`: License source unavailable, using license cache.
/// - `fault`: Initialization has failed or grace period expired.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum LicenseManagerStateEnum {
    #[serde(rename = "initializing")]
    #[strum(serialize = "initializing")]
    Initializing,
    #[serde(rename = "normal")]
    #[strum(serialize = "normal")]
    Normal,
    #[serde(rename = "marginal")]
    #[strum(serialize = "marginal")]
    Marginal,
    #[serde(rename = "fault")]
    #[strum(serialize = "fault")]
    Fault,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Describes the reservation state of a license.
/// 
/// Possible values:
/// - `notUsed`: This license is currently unused by the system, or the feature does not
///   apply.
///   
///   For example, a DRS license appears as NotUsed if the host is not
///   part of a DRS-enabled cluster.
/// - `noLicense`: This indicates that the license has expired or the system attempted to acquire
///   the license but was not successful in reserving it.
/// - `unlicensedUse`: The LicenseManager failed to acquire a license but the implementation
///   policy allows us to use the licensed feature anyway.
///   
///   This is possible, for
///   example, when a license server becomes unavailable after a license had been
///   successfully reserved from it.
/// - `licensed`: The required number of licenses have been acquired from the license source.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum LicenseReservationInfoStateEnum {
    #[serde(rename = "notUsed")]
    #[strum(serialize = "notUsed")]
    NotUsed,
    #[serde(rename = "noLicense")]
    #[strum(serialize = "noLicense")]
    NoLicense,
    #[serde(rename = "unlicensedUse")]
    #[strum(serialize = "unlicensedUse")]
    UnlicensedUse,
    #[serde(rename = "licensed")]
    #[strum(serialize = "licensed")]
    Licensed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The Status enumeration defines a general "health" value for a managed entity.
/// 
/// Possible values:
/// - `gray`: The status is unknown.
/// - `green`: The entity is OK.
/// - `yellow`: The entity might have a problem.
/// - `red`: The entity definitely has a problem.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ManagedEntityStatusEnum {
    #[serde(rename = "gray")]
    #[strum(serialize = "gray")]
    Gray,
    #[serde(rename = "green")]
    #[strum(serialize = "green")]
    Green,
    #[serde(rename = "yellow")]
    #[strum(serialize = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    #[strum(serialize = "red")]
    Red,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The type of an OST node.
/// 
/// Each OST node corresponds to an element in the OVF descriptor. See *OvfConsumerOstNode*
/// for a description of the different node types.
/// 
/// Possible values:
/// - `envelope`
/// - `virtualSystem`
/// - `virtualSystemCollection`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum OvfConsumerOstNodeTypeEnum {
    #[serde(rename = "envelope")]
    #[strum(serialize = "envelope")]
    Envelope,
    #[serde(rename = "virtualSystem")]
    #[strum(serialize = "virtualSystem")]
    VirtualSystem,
    #[serde(rename = "virtualSystemCollection")]
    #[strum(serialize = "virtualSystemCollection")]
    VirtualSystemCollection,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Types of disk provisioning that can be set for the disk in the deployed OVF
/// package.
/// 
/// Possible values:
/// - `monolithicSparse`: A sparse (allocate on demand) monolithic disk.
///   
///   Disks in this format can
///   be used with other VMware products.
/// - `monolithicFlat`: A preallocated monolithic disk.
///   
///   Disks in this format can be used with
///   other VMware products.
/// - `twoGbMaxExtentSparse`: A sparse (allocate on demand) disk with 2GB maximum extent size.
///   
///   Disks in this format can be used with other VMware products. The 2GB
///   extent size makes these disks easier to burn to dvd or use on
///   filesystems that don't support large files.
/// - `twoGbMaxExtentFlat`: A preallocated disk with 2GB maximum extent size.
///   
///   Disks in this format
///   can be used with other VMware products. The 2GB extent size
///   makes these disks easier to burn to dvd or use on filesystems that
///   don't support large files.
/// - `thin`: Space required for thin-provisioned virtual disk is allocated and
///   zeroed on demand as the space is used.
/// - `thick`: A thick disk has all space allocated at creation time
///   and the space is zeroed on demand as the space is used.
/// - `seSparse`: A sparse (allocate on demand) format with additional space
///   optimizations.
/// - `eagerZeroedThick`: An eager zeroed thick disk has all space allocated and wiped clean
///   of any previous contents on the physical media at creation time.
///   
///   Such disks may take longer time during creation compared to other
///   disk formats.
/// - `sparse`: Depending on the host type, Sparse is mapped to either
///   MonolithicSparse or Thin.
/// - `flat`: Depending on the host type, Flat is mapped to either
///   MonolithicFlat or Thick.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum OvfCreateImportSpecParamsDiskProvisioningTypeEnum {
    #[serde(rename = "monolithicSparse")]
    #[strum(serialize = "monolithicSparse")]
    MonolithicSparse,
    #[serde(rename = "monolithicFlat")]
    #[strum(serialize = "monolithicFlat")]
    MonolithicFlat,
    #[serde(rename = "twoGbMaxExtentSparse")]
    #[strum(serialize = "twoGbMaxExtentSparse")]
    TwoGbMaxExtentSparse,
    #[serde(rename = "twoGbMaxExtentFlat")]
    #[strum(serialize = "twoGbMaxExtentFlat")]
    TwoGbMaxExtentFlat,
    #[serde(rename = "thin")]
    #[strum(serialize = "thin")]
    Thin,
    #[serde(rename = "thick")]
    #[strum(serialize = "thick")]
    Thick,
    #[serde(rename = "seSparse")]
    #[strum(serialize = "seSparse")]
    SeSparse,
    #[serde(rename = "eagerZeroedThick")]
    #[strum(serialize = "eagerZeroedThick")]
    EagerZeroedThick,
    #[serde(rename = "sparse")]
    #[strum(serialize = "sparse")]
    Sparse,
    #[serde(rename = "flat")]
    #[strum(serialize = "flat")]
    Flat,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Indicates how multiple samples of a specific counter type are
/// transformed into a single statistical value.
/// 
/// Possible values:
/// - `average`: The actual value collected or the average of all values collected
///   during the summary period.
/// - `maximum`: The maximum value of the performance counter value over the
///   summarization period.
/// - `minimum`: The minimum value of the performance counter value over the
///   summarization period.
/// - `latest`: The most recent value of the performance counter over the
///   summarization period.
/// - `summation`: The sum of all the values of the performance counter over the
///   summarization period.
/// - `none`: The counter is never rolled up.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum PerfSummaryTypeEnum {
    #[serde(rename = "average")]
    #[strum(serialize = "average")]
    Average,
    #[serde(rename = "maximum")]
    #[strum(serialize = "maximum")]
    Maximum,
    #[serde(rename = "minimum")]
    #[strum(serialize = "minimum")]
    Minimum,
    #[serde(rename = "latest")]
    #[strum(serialize = "latest")]
    Latest,
    #[serde(rename = "summation")]
    #[strum(serialize = "summation")]
    Summation,
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Indicates the type of statistical measurement that a counter&#146;s
/// value represents.
/// 
/// Valid types are &#147;absolute&#148;,
/// &#147;delta&#148;, or &#147;rate&#148;.
/// 
/// Possible values:
/// - `absolute`: Represents an actual value, level, or state of the counter.
///   
///   For
///   example, the &#147;uptime&#148; counter (**system** group)
///   represents the actual number of seconds since startup. The
///   &#147;capacity&#148; counter represents the actual configured size
///   of the specified datastore. In other words, number of samples,
///   samplingPeriod, and intervals have no bearing on an
///   &#147;absolute&#148; counter&#147;s value.
/// - `delta`: Represents an amount of change for the counter during the *PerfInterval.samplingPeriod* as compared to the previous
///   *interval*.
///   
///   The first sampling interval
/// - `rate`: Represents a value that has been normalized over the *PerfInterval.samplingPeriod*, enabling values for the same
///   counter type to be compared, regardless of interval.
///   
///   For example,
///   the number of reads per second.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum PerfStatsTypeEnum {
    #[serde(rename = "absolute")]
    #[strum(serialize = "absolute")]
    Absolute,
    #[serde(rename = "delta")]
    #[strum(serialize = "delta")]
    Delta,
    #[serde(rename = "rate")]
    #[strum(serialize = "rate")]
    Rate,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Indicates the unit of measure represented by a counter or statistical
/// value.
/// 
/// Possible values:
/// - `percent`: Percentage values in units of 1/100th of a percent.
///   
///   For example 100
///   represents 1%.
/// - `kiloBytes`: Kilobytes.
/// - `megaBytes`: Megabytes.
/// - `megaHertz`: Megahertz.
/// - `number`: A quantity of items, for example, the number of CPUs.
/// - `microsecond`: The time in microseconds.
/// - `millisecond`: The time in milliseconds.
/// - `second`: The time in seconds.
/// - `kiloBytesPerSecond`: Kilobytes per second.
/// - `megaBytesPerSecond`: Megabytes per second.
/// - `watt`: Watts
/// - `joule`: Joules
/// - `teraBytes`: Terabytes.
/// - `celsius`: Temperature in celsius.
/// - `nanosecond`: The time in nanoseconds.
///   
///   ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum PerformanceManagerUnitEnum {
    #[serde(rename = "percent")]
    #[strum(serialize = "percent")]
    Percent,
    #[serde(rename = "kiloBytes")]
    #[strum(serialize = "kiloBytes")]
    KiloBytes,
    #[serde(rename = "megaBytes")]
    #[strum(serialize = "megaBytes")]
    MegaBytes,
    #[serde(rename = "megaHertz")]
    #[strum(serialize = "megaHertz")]
    MegaHertz,
    #[serde(rename = "number")]
    #[strum(serialize = "number")]
    Number,
    #[serde(rename = "microsecond")]
    #[strum(serialize = "microsecond")]
    Microsecond,
    #[serde(rename = "millisecond")]
    #[strum(serialize = "millisecond")]
    Millisecond,
    #[serde(rename = "second")]
    #[strum(serialize = "second")]
    Second,
    #[serde(rename = "kiloBytesPerSecond")]
    #[strum(serialize = "kiloBytesPerSecond")]
    KiloBytesPerSecond,
    #[serde(rename = "megaBytesPerSecond")]
    #[strum(serialize = "megaBytesPerSecond")]
    MegaBytesPerSecond,
    #[serde(rename = "watt")]
    #[strum(serialize = "watt")]
    Watt,
    #[serde(rename = "joule")]
    #[strum(serialize = "joule")]
    Joule,
    #[serde(rename = "teraBytes")]
    #[strum(serialize = "teraBytes")]
    TeraBytes,
    #[serde(rename = "celsius")]
    #[strum(serialize = "celsius")]
    Celsius,
    #[serde(rename = "nanosecond")]
    #[strum(serialize = "nanosecond")]
    Nanosecond,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The format in which performance counter data is returned.
/// 
/// Possible values:
/// - `normal`: Counters returned in an array of data objects.
/// - `csv`: Counters returned in comma-separate value (CSV) format.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum PerfFormatEnum {
    #[serde(rename = "normal")]
    #[strum(serialize = "normal")]
    Normal,
    #[serde(rename = "csv")]
    #[strum(serialize = "csv")]
    Csv,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `disabled`: Do not scale shares
/// - `scaleCpuAndMemoryShares`: Scale both CPU and memory shares
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ResourceConfigSpecScaleSharesBehaviorEnum {
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "scaleCpuAndMemoryShares")]
    #[strum(serialize = "scaleCpuAndMemoryShares")]
    ScaleCpuAndMemoryShares,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Types of a host's compatibility with a designated virtual machine
/// that is a candidate for VMotion.
/// 
/// Used with queryVMotionCompatibility
/// both as inputs (to designate which compatibility types to test for)
/// and as outputs (to specify which compatibility types apply for
/// each host).
/// 
/// Possible values:
/// - `cpu`: The host's CPU features are compatible with the
///   the virtual machine's requirements.
/// - `software`: The software platform on the host supports VMotion
///   and is compatible with the virtual machine.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VMotionCompatibilityTypeEnum {
    #[serde(rename = "cpu")]
    #[strum(serialize = "cpu")]
    Cpu,
    #[serde(rename = "software")]
    #[strum(serialize = "software")]
    Software,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated as of vSphere API 4.0, use *CheckTestType_enum* instead.
/// 
/// Types of tests available for validateMigration.
/// 
/// Possible values:
/// - `sourceTests`: Tests that examine only the configuration
///   of the virtual machine and its current host; the destination
///   resource pool and host or cluster are irrelevant.
/// - `compatibilityTests`: Tests that examine both the virtual
///   machine and the destination host or cluster; the destination
///   resource pool is irrelevant.
///   
///   This set excludes tests that fall
///   into the diskAccessibilityTests group.
/// - `diskAccessibilityTests`: Tests that check that the
///   destination host or cluster can see the datastores where the virtual
///   machine's virtual disks are currently located.
///   
///   The destination
///   resource pool is irrelevant. If you are planning to relocate the
///   virtual disks, do not use these tests; instead examine the relevant
///   datastore objects for your planned disk locations to see if they
///   are accessible to the destination host.
/// - `resourceTests`: Tests that check that the destination resource
///   pool can support the virtual machine if it is powered on.
///   
///   The
///   destination host or cluster is relevant because it will affect the
///   amount of overhead memory required to run the virtual machine.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ValidateMigrationTestTypeEnum {
    #[serde(rename = "sourceTests")]
    #[strum(serialize = "sourceTests")]
    SourceTests,
    #[serde(rename = "compatibilityTests")]
    #[strum(serialize = "compatibilityTests")]
    CompatibilityTests,
    #[serde(rename = "diskAccessibilityTests")]
    #[strum(serialize = "diskAccessibilityTests")]
    DiskAccessibilityTests,
    #[serde(rename = "resourceTests")]
    #[strum(serialize = "resourceTests")]
    ResourceTests,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `HttpNfcServiceTicket`: Ticket used for HttpNfc access to a file or disk on a datastore
/// - `HostServiceTicket`: Ticket used for service request on a host
/// - `VcServiceTicket`: Ticket used for service request on a VC
///   
/// ***Since:*** vSphere API Release 7.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum SessionManagerGenericServiceTicketTicketTypeEnum {
    HttpNfcServiceTicket,
    HostServiceTicket,
    VcServiceTicket,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// HTTP request methods.
/// 
/// Possible values:
/// - `httpOptions`
/// - `httpGet`
/// - `httpHead`
/// - `httpPost`
/// - `httpPut`
/// - `httpDelete`
/// - `httpTrace`
/// - `httpConnect`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum SessionManagerHttpServiceRequestSpecMethodEnum {
    #[serde(rename = "httpOptions")]
    #[strum(serialize = "httpOptions")]
    HttpOptions,
    #[serde(rename = "httpGet")]
    #[strum(serialize = "httpGet")]
    HttpGet,
    #[serde(rename = "httpHead")]
    #[strum(serialize = "httpHead")]
    HttpHead,
    #[serde(rename = "httpPost")]
    #[strum(serialize = "httpPost")]
    HttpPost,
    #[serde(rename = "httpPut")]
    #[strum(serialize = "httpPut")]
    HttpPut,
    #[serde(rename = "httpDelete")]
    #[strum(serialize = "httpDelete")]
    HttpDelete,
    #[serde(rename = "httpTrace")]
    #[strum(serialize = "httpTrace")]
    HttpTrace,
    #[serde(rename = "httpConnect")]
    #[strum(serialize = "httpConnect")]
    HttpConnect,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Simplified shares notation.
/// 
/// These designations have different meanings for different resources.
/// 
/// Possible values:
/// - `low`: For CPU: Shares = 500 \* number of virtual CPUs  
///   For Memory: Shares = 5 \* virtual machine memory size in megabytes  
///   For Disk: Shares = 500  
///   For Network: Shares = 0.25 \* *DVSFeatureCapability.networkResourcePoolHighShareValue*
/// - `normal`: For CPU: Shares = 1000 \* number of virtual CPUs  
///   For Memory: Shares = 10 \* virtual machine memory size in megabytes  
///   For Disk: Shares = 1000  
///   For Network: Shares = 0.5 \* *DVSFeatureCapability.networkResourcePoolHighShareValue*
/// - `high`: For CPU: Shares = 2000 \* number of virtual CPUs  
///   For Memory: Shares = 20 \* virtual machine memory size in megabytes  
///   For Disk: Shares = 2000  
///   For Network: Shares = *DVSFeatureCapability.networkResourcePoolHighShareValue*
/// - `custom`: If you specify <code>custom</code> for the *SharesInfo.level* property, when there is resource contention the Server uses the *SharesInfo.shares* value to determine resource allocation.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum SharesLevelEnum {
    #[serde(rename = "low")]
    #[strum(serialize = "low")]
    Low,
    #[serde(rename = "normal")]
    #[strum(serialize = "normal")]
    Normal,
    #[serde(rename = "high")]
    #[strum(serialize = "high")]
    High,
    #[serde(rename = "custom")]
    #[strum(serialize = "custom")]
    Custom,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The encoding of the resultant return data.
/// 
/// This is a hint to the client side
/// to indicate the format of the information being returned.
/// 
/// Possible values:
/// - `CSV`: Comma separated values
/// - `HEX`: Hex encoded binary data
/// - `STRING`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum SimpleCommandEncodingEnum {
    #[serde(rename = "CSV")]
    #[strum(serialize = "CSV")]
    Csv,
    #[serde(rename = "HEX")]
    #[strum(serialize = "HEX")]
    Hex,
    #[serde(rename = "STRING")]
    #[strum(serialize = "STRING")]
    String,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// User specification of congestion threshold mode on a given datastore
/// 
/// For more information, see
/// *StorageIORMInfo.congestionThreshold*
/// 
/// Possible values:
/// - `automatic`: Storagage IO Control will choose appropriate congestion threshold value
///   for that datastore to operate at given percentage of peak throughput.
///   
///   This is the default setting
/// - `manual`: Use user specified Storage IO Control congestion threshold value
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum StorageIormThresholdModeEnum {
    #[serde(rename = "automatic")]
    #[strum(serialize = "automatic")]
    Automatic,
    #[serde(rename = "manual")]
    #[strum(serialize = "manual")]
    Manual,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This option specifies how to select tasks based on child relationships
/// in the inventory hierarchy.
/// 
/// If a managed entity has children, their tasks
/// can be retrieved with this filter option.
/// 
/// Possible values:
/// - `self`: Returns tasks that pertain only to the specified managed entity,
///   and not its children.
/// - `children`: Returns tasks pertaining to child entities only.
///   
///   Excludes
///   tasks pertaining to the specified managed entity itself.
/// - `all`: Returns tasks pertaining either to the specified managed entity
///   or to its child entities.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum TaskFilterSpecRecursionOptionEnum {
    #[serde(rename = "self")]
    #[strum(serialize = "self")]
    Self_,
    #[serde(rename = "children")]
    #[strum(serialize = "children")]
    Children,
    #[serde(rename = "all")]
    #[strum(serialize = "all")]
    All,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This option specifies a time stamp governing the selection of tasks.
/// 
/// Possible values:
/// - `queuedTime`: The time stamp when the task was created and queued.
/// - `startedTime`: The time stamp when the task started.
/// - `completedTime`: The time stamp when the task finished.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum TaskFilterSpecTimeOptionEnum {
    #[serde(rename = "queuedTime")]
    #[strum(serialize = "queuedTime")]
    QueuedTime,
    #[serde(rename = "startedTime")]
    #[strum(serialize = "startedTime")]
    StartedTime,
    #[serde(rename = "completedTime")]
    #[strum(serialize = "completedTime")]
    CompletedTime,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// List of possible states of a task.
/// 
/// Possible values:
/// - `queued`: When there are too many tasks for threads to handle.
/// - `running`: When the busy thread is freed from its current task by
///   finishing the task, it picks a queued task to run.
///   
///   Then the queued tasks are marked as running.
/// - `success`: When a running task has completed.
/// - `error`: When a running task has encountered an error.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum TaskInfoStateEnum {
    #[serde(rename = "queued")]
    #[strum(serialize = "queued")]
    Queued,
    #[serde(rename = "running")]
    #[strum(serialize = "running")]
    Running,
    #[serde(rename = "success")]
    #[strum(serialize = "success")]
    Success,
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The VAppState type defines the set of states a vApp can be
/// in.
/// 
/// The transitory states between started and stopped is modeled explicitly,
/// since the starting or stopping of a vApp is typically a time-consuming
/// process that might take minutes to complete.
/// 
/// Possible values:
/// - `started`: The vApp is currently powered on .
/// - `stopped`: The vApp is currently powered off or suspended.
/// - `starting`: The vApp is in the process of starting.
/// - `stopping`: The vApp is in the process of stopping.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualAppVAppStateEnum {
    #[serde(rename = "started")]
    #[strum(serialize = "started")]
    Started,
    #[serde(rename = "stopped")]
    #[strum(serialize = "stopped")]
    Stopped,
    #[serde(rename = "starting")]
    #[strum(serialize = "starting")]
    Starting,
    #[serde(rename = "stopping")]
    #[strum(serialize = "stopping")]
    Stopping,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The types of virtual disk adapters used by virtual disks
/// 
/// Possible values:
/// - `ide`: Use IDE emulation for the virtual disk
/// - `busLogic`: Use BusLogic emulation for the virtual disk
/// - `lsiLogic`: Use LSILogic emulation for the virtual disk
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDiskAdapterTypeEnum {
    #[serde(rename = "ide")]
    #[strum(serialize = "ide")]
    Ide,
    #[serde(rename = "busLogic")]
    #[strum(serialize = "busLogic")]
    BusLogic,
    #[serde(rename = "lsiLogic")]
    #[strum(serialize = "lsiLogic")]
    LsiLogic,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The types of virtual disks that can be created or cloned.
/// 
/// Possible values:
/// - `preallocated`: A preallocated disk has all space allocated at creation time
///   and the space is zeroed on demand as the space is used.
/// - `thin`: Space required for thin-provisioned virtual disk is allocated and
///   zeroed on demand as the space is used.
/// - `seSparse`: A sparse (allocate on demand) format with additional space
///   optimizations.
/// - `rdm`: Virtual compatibility mode raw disk mapping.
///   
///   An rdm virtual disk
///   grants access to the entire raw disk and the virtual disk can
///   participate in snapshots.
/// - `rdmp`: Physical compatibility mode (pass-through) raw disk mapping.
///   
///   An rdmp
///   virtual disk passes SCSI commands directly to the hardware, but the
///   virtual disk cannot participate in snapshots.
/// - `raw`: Raw device.
/// - `delta`: A redo log disk.
///   
///   This format is only applicable as a destination format
///   in a clone operation, and not usable for disk creation.
/// - `sparse2Gb`: A sparse disk with 2GB maximum extent size.
///   
///   Disks in this format
///   can be used with other VMware products. The 2GB extent size
///   makes these disks easier to burn to dvd or use on filesystems that
///   don't support large files. This format is only applicable as a
///   destination format in a clone operation, and not usable for disk
///   creation.
/// - `thick2Gb`: A thick disk with 2GB maximum extent size.
///   
///   Disks in this format
///   can be used with other VMware products. The 2GB extent size
///   makes these disks easier to burn to dvd or use on filesystems that
///   don't support large files. This format is only applicable as a
///   destination format in a clone operation, and not usable for disk
///   creation.
/// - `eagerZeroedThick`: An eager zeroed thick disk has all space allocated and wiped clean
///   of any previous contents on the physical media at creation time.
///   
///   Such disks may take longer time during creation compared to other
///   disk formats.
/// - `sparseMonolithic`: A sparse monolithic disk.
///   
///   Disks in this format can be used with other
///   VMware products. This format is only applicable as a destination
///   format in a clone operation, and not usable for disk creation.
/// - `flatMonolithic`: A preallocated monolithic disk.
///   
///   Disks in this format can be used with
///   other VMware products. This format is only applicable as a destination
///   format in a clone operation, and not usable for disk creation.
/// - `thick`: 
///   
///   Deprecated as of vSphere API 4.x, use *eagerZeroedThick* instead
///   for clustering application, and *preallocated* for other applications.
///   
///   A thick disk has all space allocated at creation time.
///   
///   This
///   space may contain stale data on the physical media. Thick disks
///   are primarily used for virtual machine clustering, but they are
///   generally insecure and should not be used. Due to better performance
///   and security properties, the use of the 'preallocated' format is
///   preferred over this format.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDiskTypeEnum {
    #[serde(rename = "preallocated")]
    #[strum(serialize = "preallocated")]
    Preallocated,
    #[serde(rename = "thin")]
    #[strum(serialize = "thin")]
    Thin,
    #[serde(rename = "seSparse")]
    #[strum(serialize = "seSparse")]
    SeSparse,
    #[serde(rename = "rdm")]
    #[strum(serialize = "rdm")]
    Rdm,
    #[serde(rename = "rdmp")]
    #[strum(serialize = "rdmp")]
    Rdmp,
    #[serde(rename = "raw")]
    #[strum(serialize = "raw")]
    Raw,
    #[serde(rename = "delta")]
    #[strum(serialize = "delta")]
    Delta,
    #[serde(rename = "sparse2Gb")]
    #[strum(serialize = "sparse2Gb")]
    Sparse2Gb,
    #[serde(rename = "thick2Gb")]
    #[strum(serialize = "thick2Gb")]
    Thick2Gb,
    #[serde(rename = "eagerZeroedThick")]
    #[strum(serialize = "eagerZeroedThick")]
    EagerZeroedThick,
    #[serde(rename = "sparseMonolithic")]
    #[strum(serialize = "sparseMonolithic")]
    SparseMonolithic,
    #[serde(rename = "flatMonolithic")]
    #[strum(serialize = "flatMonolithic")]
    FlatMonolithic,
    #[serde(rename = "thick")]
    #[strum(serialize = "thick")]
    Thick,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Application heartbeat status type.
/// 
/// Possible values:
/// - `appStatusGray`: Heartbeat status is disabled
/// - `appStatusGreen`: Heartbeat status is OK
/// - `appStatusRed`: Heartbeating has stopped
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineAppHeartbeatStatusTypeEnum {
    #[serde(rename = "appStatusGray")]
    #[strum(serialize = "appStatusGray")]
    AppStatusGray,
    #[serde(rename = "appStatusGreen")]
    #[strum(serialize = "appStatusGreen")]
    AppStatusGreen,
    #[serde(rename = "appStatusRed")]
    #[strum(serialize = "appStatusRed")]
    AppStatusRed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The connectivity state of a virtual machine.
/// 
/// When the API is provided directly by
/// a server product, such as ESX Server, then the disconnected state is not
/// possible. However, when accessed through VirtualCenter, the state of a virtual
/// machine is set to disconnected if the hosts that manage the virtual
/// machine becomes unavailable.
/// 
/// Possible values:
/// - `connected`: The server has access to the virtual machine.
/// - `disconnected`: The server is currently disconnected from the virtual machine, since its
///   host is disconnected.
///   
///   See general comment for this enumerated type for more
///   details.
/// - `orphaned`: The virtual machine is no longer registered on the host it is associated
///   with.
///   
///   For example, a virtual machine that is unregistered or deleted
///   directly on a host managed by VirtualCenter shows up in this state.
/// - `inaccessible`: One or more of the virtual machine configuration files are inaccessible.
///   
///   For
///   example, this can be due to transient disk failures. In this case, no
///   configuration can be returned for a virtual machine.
/// - `invalid`: The virtual machine configuration format is invalid.
///   
///   Thus, it is accessible
///   on disk, but corrupted in a way that does not allow the server to read the
///   content. In this case, no configuration can be returned for a virtual
///   machine.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineConnectionStateEnum {
    #[serde(rename = "connected")]
    #[strum(serialize = "connected")]
    Connected,
    #[serde(rename = "disconnected")]
    #[strum(serialize = "disconnected")]
    Disconnected,
    #[serde(rename = "orphaned")]
    #[strum(serialize = "orphaned")]
    Orphaned,
    #[serde(rename = "inaccessible")]
    #[strum(serialize = "inaccessible")]
    Inaccessible,
    #[serde(rename = "invalid")]
    #[strum(serialize = "invalid")]
    Invalid,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The crypto state of a encrypted virtual machine.
/// 
/// Possible values:
/// - `unlocked`: The virtual machine is in unlocked state.
/// - `locked`: The virtual machine is in locked state for the configuration key missing
///   on the ESX host where the VM is registered.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineCryptoStateEnum {
    #[serde(rename = "unlocked")]
    #[strum(serialize = "unlocked")]
    Unlocked,
    #[serde(rename = "locked")]
    #[strum(serialize = "locked")]
    Locked,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The FaultToleranceState type defines a simple set of states for a
/// fault tolerant virtual machine:
/// disabled, starting, and enabled.
/// 
/// Possible values:
/// - `notConfigured`: This state indicates that the virtual machine has not been
///   configured for fault tolerance.
/// - `disabled`: For a virtual machine that is the primary in a fault tolerant group,
///   this state indicates that the virtual machine has at least one
///   registered secondary, but no secondary is enabled.
///   
///   For a virtual machine that is the secondary in a fault tolerant
///   group, this state indicates that the secondary is disabled.
/// - `enabled`: For a virtual machine that is the primary in a fault tolerant group,
///   this state indicates that the virtual machine is not currently
///   powered on, but has at least one enabled secondary
///   For a virtual machine that is the secondary in a fault tolerant
///   group, this state indicates that the secondary is enabled, but is
///   not currently powered on.
/// - `needSecondary`: For a virtual machine that is the primary in a fault tolerant group,
///   this state indicates that the virtual machine is powered on and
///   has at least one enabled secondary, but no secondary is currently
///   active.
///   
///   This state is not valid for a virtual machine that is a secondary
///   in a fault tolerant group.
/// - `starting`: For a virtual machine that is the primary in a fault tolerant group,
///   this state indicates that the virtual machine is powered on and has
///   at least one secondary that is synchronizing its state with the
///   primary.
///   
///   For a virtual machine that is the secondary in a fault tolerant
///   group, this state indicates that the secondary is powered on and is
///   synchronizing its state with the primary virtual machine.
/// - `running`: This state indicates that the virtual machine is running with fault
///   tolerance protection.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineFaultToleranceStateEnum {
    #[serde(rename = "notConfigured")]
    #[strum(serialize = "notConfigured")]
    NotConfigured,
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    #[strum(serialize = "enabled")]
    Enabled,
    #[serde(rename = "needSecondary")]
    #[strum(serialize = "needSecondary")]
    NeedSecondary,
    #[serde(rename = "starting")]
    #[strum(serialize = "starting")]
    Starting,
    #[serde(rename = "running")]
    #[strum(serialize = "running")]
    Running,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The FaultToleranceType defines the type of fault tolerance, if any,
/// the virtual machine is configured for.
/// 
/// Possible values:
/// - `unset`: FT not set
/// - `recordReplay`: Record/replay
/// - `checkpointing`: Checkpointing
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineFaultToleranceTypeEnum {
    #[serde(rename = "unset")]
    #[strum(serialize = "unset")]
    Unset,
    #[serde(rename = "recordReplay")]
    #[strum(serialize = "recordReplay")]
    RecordReplay,
    #[serde(rename = "checkpointing")]
    #[strum(serialize = "checkpointing")]
    Checkpointing,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// MovePriority is an enumeration of values that indicate the priority of the task
/// that moves a virtual machine from one host to another or one storage location
/// to another.
/// 
/// Note this priority can affect both the source and target hosts.
/// 
/// Possible values:
/// - `lowPriority`: The task of moving this virtual machine is low priority.
/// - `highPriority`: The task of moving this virtual machine is high priority.
/// - `defaultPriority`: The task of moving this virtual machine is the default priority.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineMovePriorityEnum {
    #[serde(rename = "lowPriority")]
    #[strum(serialize = "lowPriority")]
    LowPriority,
    #[serde(rename = "highPriority")]
    #[strum(serialize = "highPriority")]
    HighPriority,
    #[serde(rename = "defaultPriority")]
    #[strum(serialize = "defaultPriority")]
    DefaultPriority,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The NeedSecondaryReason type defines all reasons a virtual machine is
/// in the needSecondary Fault Tolerance state following a failure.
/// 
/// Possible values:
/// - `initializing`: Initializing FT
/// - `divergence`: Divergence
/// - `lostConnection`: Lose connection to secondary
/// - `partialHardwareFailure`: Partial hardware failure
/// - `userAction`: Terminated by user
/// - `checkpointError`: Checkpoint error
/// - `other`: All other reasons
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineNeedSecondaryReasonEnum {
    #[serde(rename = "initializing")]
    #[strum(serialize = "initializing")]
    Initializing,
    #[serde(rename = "divergence")]
    #[strum(serialize = "divergence")]
    Divergence,
    #[serde(rename = "lostConnection")]
    #[strum(serialize = "lostConnection")]
    LostConnection,
    #[serde(rename = "partialHardwareFailure")]
    #[strum(serialize = "partialHardwareFailure")]
    PartialHardwareFailure,
    #[serde(rename = "userAction")]
    #[strum(serialize = "userAction")]
    UserAction,
    #[serde(rename = "checkpointError")]
    #[strum(serialize = "checkpointError")]
    CheckpointError,
    #[serde(rename = "other")]
    #[strum(serialize = "other")]
    Other,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The PowerState type defines a simple set of states for a virtual machine:
/// poweredOn, poweredOff, and suspended.
/// 
/// This type does not model substates,
/// such as when a task is running to change the virtual machine state.
/// If the virtual machine is in a state with a task in progress, it
/// transitions to a new state when the task completes. For example, a virtual
/// machine continues to be in the poweredOn state while a suspend task
/// is running, and changes to the suspended state once the task finishes.
/// 
/// As a consequence of this approach, clients interested in monitoring
/// the status of a virtual machine should typically track the
/// *activeTask* data object in addition to the
/// *powerState* object.
/// 
/// Possible values:
/// - `poweredOff`: The virtual machine is currently powered off.
/// - `poweredOn`: The virtual machine is currently powered on.
/// - `suspended`: The virtual machine is currently suspended.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachinePowerStateEnum {
    #[serde(rename = "poweredOff")]
    #[strum(serialize = "poweredOff")]
    PoweredOff,
    #[serde(rename = "poweredOn")]
    #[strum(serialize = "poweredOn")]
    PoweredOn,
    #[serde(rename = "suspended")]
    #[strum(serialize = "suspended")]
    Suspended,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated as of vSphere API 6.0.
/// 
/// The RecordReplayState type defines a simple set of record and replay
/// states for a virtual machine.
/// 
/// Possible values:
/// - `recording`: The virtual machine is recording.
/// - `replaying`: The virtual machine is replaying.
/// - `inactive`: The virtual machine is currently not participating
///   in record or replay.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineRecordReplayStateEnum {
    #[serde(rename = "recording")]
    #[strum(serialize = "recording")]
    Recording,
    #[serde(rename = "replaying")]
    #[strum(serialize = "replaying")]
    Replaying,
    #[serde(rename = "inactive")]
    #[strum(serialize = "inactive")]
    Inactive,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The virtual machine ticket type.
/// 
/// Possible values:
/// - `mks`: 
///   
///   Deprecated as of vSphere API 8.0. Use *webmks* instead.
///   
///   Remote mouse-keyboard-screen ticket.
/// - `device`: 
///   
///   Deprecated as of vSphere 8.0 API. Use *webRemoteDevice*
///   instead.
///   
///   Remote device ticket.
/// - `guestControl`: 
///   
///   Deprecated as of vSphere 6.6.3 API. Use
///   *GuestOperationsManager* instead.
///   
///   Guest operation ticket.
/// - `webmks`: Mouse-keyboard-screen over WebSocket ticket.
///   
///   MKS protocol is VNC (a.k.a. RFB) protocol with
///   VMware extensions; the protocol gracefully degrades
///   to standard VNC if extensions are not available.
///   wss://{Ticket.host}/ticket/{Ticket.ticket}
/// - `guestIntegrity`: Guest Integrity over WebSocket ticket.
///   
///   This ticket grants the client read-only access to guest integrity
///   messages and alerts.
/// - `webRemoteDevice`: Remote device over WebSocket ticket.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineTicketTypeEnum {
    #[serde(rename = "mks")]
    #[strum(serialize = "mks")]
    Mks,
    #[serde(rename = "device")]
    #[strum(serialize = "device")]
    Device,
    #[serde(rename = "guestControl")]
    #[strum(serialize = "guestControl")]
    GuestControl,
    #[serde(rename = "webmks")]
    #[strum(serialize = "webmks")]
    Webmks,
    #[serde(rename = "guestIntegrity")]
    #[strum(serialize = "guestIntegrity")]
    GuestIntegrity,
    #[serde(rename = "webRemoteDevice")]
    #[strum(serialize = "webRemoteDevice")]
    WebRemoteDevice,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Type of disk group operation performed.
/// 
/// Possible values:
/// - `add`: Disk group is being (re-)added.
/// - `remove`: Disk group is being removed.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VsanUpgradeSystemUpgradeHistoryDiskGroupOpTypeEnum {
    #[serde(rename = "add")]
    #[strum(serialize = "add")]
    Add,
    #[serde(rename = "remove")]
    #[strum(serialize = "remove")]
    Remove,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// These constant strings can be used as parameters in user-specified
/// email subject and body templates as well as in scripts.
/// 
/// The action processor
/// in VirtualCenter substitutes the run-time values for the parameters.
/// For example, an email subject provided by the client could be the string:
/// `Alarm - {alarmName} Description:\n{eventDescription}`.
/// Or a script action provided could be: `myScript {alarmName}`.
/// 
/// Possible values:
/// - `targetName`: The name of the entity where the alarm is triggered.
/// - `alarmName`: The name of the triggering alarm.
/// - `oldStatus`: The status prior to the alarm being triggered.
/// - `newStatus`: The status after the alarm is triggered.
/// - `triggeringSummary`: A summary of information involved in triggering the alarm.
/// - `declaringSummary`: A summary of declarations made during the triggering of the alarm.
/// - `eventDescription`: The event description.
/// - `target`: The object of the entity where the alarm is associated.
/// - `alarm`: The object of the triggering alarm.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ActionParameterEnum {
    #[serde(rename = "targetName")]
    #[strum(serialize = "targetName")]
    TargetName,
    #[serde(rename = "alarmName")]
    #[strum(serialize = "alarmName")]
    AlarmName,
    #[serde(rename = "oldStatus")]
    #[strum(serialize = "oldStatus")]
    OldStatus,
    #[serde(rename = "newStatus")]
    #[strum(serialize = "newStatus")]
    NewStatus,
    #[serde(rename = "triggeringSummary")]
    #[strum(serialize = "triggeringSummary")]
    TriggeringSummary,
    #[serde(rename = "declaringSummary")]
    #[strum(serialize = "declaringSummary")]
    DeclaringSummary,
    #[serde(rename = "eventDescription")]
    #[strum(serialize = "eventDescription")]
    EventDescription,
    #[serde(rename = "target")]
    #[strum(serialize = "target")]
    Target,
    #[serde(rename = "alarm")]
    #[strum(serialize = "alarm")]
    Alarm,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Alarm entity type
/// 
/// Possible values:
/// - `entityTypeAll`: Alarms on all entity types.
/// - `entityTypeHost`: Host alarms
/// - `entityTypeVm`: VM alarms
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum AlarmFilterSpecAlarmTypeByEntityEnum {
    #[serde(rename = "entityTypeAll")]
    #[strum(serialize = "entityTypeAll")]
    EntityTypeAll,
    #[serde(rename = "entityTypeHost")]
    #[strum(serialize = "entityTypeHost")]
    EntityTypeHost,
    #[serde(rename = "entityTypeVm")]
    #[strum(serialize = "entityTypeVm")]
    EntityTypeVm,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Alarm triggering type.
/// 
/// The main divisions are event triggered and
/// metric- or state-based alarms.
/// 
/// Possible values:
/// - `triggerTypeAll`: All alarm types.
/// - `triggerTypeEvent`: Event based alarms
/// - `triggerTypeMetric`: Metric or state alarms
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum AlarmFilterSpecAlarmTypeByTriggerEnum {
    #[serde(rename = "triggerTypeAll")]
    #[strum(serialize = "triggerTypeAll")]
    TriggerTypeAll,
    #[serde(rename = "triggerTypeEvent")]
    #[strum(serialize = "triggerTypeEvent")]
    TriggerTypeEvent,
    #[serde(rename = "triggerTypeMetric")]
    #[strum(serialize = "triggerTypeMetric")]
    TriggerTypeMetric,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Basic Comparison operators
/// 
/// Possible values:
/// - `equals`: attribute equals specified value
/// - `notEqualTo`: attribute does not equal specified value
/// - `startsWith`: attribute starts with specified value
/// - `doesNotStartWith`: attribute does not start with specified value
/// - `endsWith`: attribute ends with specified value
/// - `doesNotEndWith`: attribute does not end with specified value
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum EventAlarmExpressionComparisonOperatorEnum {
    #[serde(rename = "equals")]
    #[strum(serialize = "equals")]
    Equals,
    #[serde(rename = "notEqualTo")]
    #[strum(serialize = "notEqualTo")]
    NotEqualTo,
    #[serde(rename = "startsWith")]
    #[strum(serialize = "startsWith")]
    StartsWith,
    #[serde(rename = "doesNotStartWith")]
    #[strum(serialize = "doesNotStartWith")]
    DoesNotStartWith,
    #[serde(rename = "endsWith")]
    #[strum(serialize = "endsWith")]
    EndsWith,
    #[serde(rename = "doesNotEndWith")]
    #[strum(serialize = "doesNotEndWith")]
    DoesNotEndWith,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The operation on the target metric item.
/// 
/// Possible values:
/// - `isAbove`: Test if the target metric item is above the given red or yellow values.
/// - `isBelow`: Test if the target metric item is below the given red or yellow values.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum MetricAlarmOperatorEnum {
    #[serde(rename = "isAbove")]
    #[strum(serialize = "isAbove")]
    IsAbove,
    #[serde(rename = "isBelow")]
    #[strum(serialize = "isBelow")]
    IsBelow,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The operation on the target state.
/// 
/// Possible values:
/// - `isEqual`: Test if the target state matches the given red or yellow states.
/// - `isUnequal`: Test if the target state does not match the given red or yellow states.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum StateAlarmOperatorEnum {
    #[serde(rename = "isEqual")]
    #[strum(serialize = "isEqual")]
    IsEqual,
    #[serde(rename = "isUnequal")]
    #[strum(serialize = "isUnequal")]
    IsUnequal,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Pre-defined constants for possible action types.
/// 
/// Virtual Center
/// uses this information to coordinate with the clients.
/// 
/// Possible values:
/// - `MigrationV1`: Migration action type
/// - `VmPowerV1`: Virtual machine power action type
/// - `HostPowerV1`: Host power action type
/// - `HostMaintenanceV1`: Host entering maintenance mode action type
/// - `StorageMigrationV1`: Storage migration action type
/// - `StoragePlacementV1`: Initial placement action for a virtual machine or a virtual disk
/// - `PlacementV1`: Initial placement action for a virtual machine and its virtual disks
/// - `HostInfraUpdateHaV1`: Host changing infrastructure update ha mode action type.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ActionTypeEnum {
    MigrationV1,
    VmPowerV1,
    HostPowerV1,
    HostMaintenanceV1,
    StorageMigrationV1,
    StoragePlacementV1,
    PlacementV1,
    HostInfraUpdateHaV1,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `onDemand`: Put each host into the crypto safe state automatically when needed.
/// - `forceEnable`: Put each host into the crypto safe state immediately.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterCryptoConfigInfoCryptoModeEnum {
    #[serde(rename = "onDemand")]
    #[strum(serialize = "onDemand")]
    OnDemand,
    #[serde(rename = "forceEnable")]
    #[strum(serialize = "forceEnable")]
    ForceEnable,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The *ClusterDasAamNodeStateDasState_enum* enumerated type defines
/// values for host HA configuration and runtime state properties
/// (*ClusterDasAamNodeState.configState* and
/// *ClusterDasAamNodeState.runtimeState*).
/// 
/// Possible values:
/// - `uninitialized`: HA has never been enabled on the the host.
/// - `initialized`: HA agents have been installed but are not running on the the host.
/// - `configuring`: HA configuration is in progress.
/// - `unconfiguring`: HA configuration is being removed.
/// - `running`: HA agent is running on this host.
/// - `error`: There is an error condition.
///   
///   This can represent a configuration
///   error or a host agent runtime error.
/// - `agentShutdown`: The HA agent has been shut down.
/// - `nodeFailed`: The host is not reachable.
///   
///   This can represent a host failure
///   or an isolated host.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterDasAamNodeStateDasStateEnum {
    #[serde(rename = "uninitialized")]
    #[strum(serialize = "uninitialized")]
    Uninitialized,
    #[serde(rename = "initialized")]
    #[strum(serialize = "initialized")]
    Initialized,
    #[serde(rename = "configuring")]
    #[strum(serialize = "configuring")]
    Configuring,
    #[serde(rename = "unconfiguring")]
    #[strum(serialize = "unconfiguring")]
    Unconfiguring,
    #[serde(rename = "running")]
    #[strum(serialize = "running")]
    Running,
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    #[serde(rename = "agentShutdown")]
    #[strum(serialize = "agentShutdown")]
    AgentShutdown,
    #[serde(rename = "nodeFailed")]
    #[strum(serialize = "nodeFailed")]
    NodeFailed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The policy to determine the candidates from which vCenter Server can
/// choose heartbeat datastores.
/// 
/// Possible values:
/// - `userSelectedDs`: vCenter Server chooses heartbeat datastores from the set specified
///   by the user (see *ClusterDasConfigInfo.heartbeatDatastore*).
///   
///   More specifically,
///   datastores not included in the set will not be chosen. Note that if
///   *ClusterDasConfigInfo.heartbeatDatastore* is empty, datastore heartbeating will
///   be disabled for HA.
/// - `allFeasibleDs`: vCenter Server chooses heartbeat datastores from all the feasible ones,
///   i.e., the datastores that are accessible to more than one host in
///   the cluster.
///   
///   The choice will be made without giving preference to those
///   specified by the user (see *ClusterDasConfigInfo.heartbeatDatastore*).
/// - `allFeasibleDsWithUserPreference`: vCenter Server chooses heartbeat datastores from all the feasible ones
///   while giving preference to those specified by the user (see *ClusterDasConfigInfo.heartbeatDatastore*).
///   
///   More specifically, the datastores not included in *ClusterDasConfigInfo.heartbeatDatastore* will be
///   chosen if and only if the specified ones are not sufficient.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterDasConfigInfoHbDatastoreCandidateEnum {
    #[serde(rename = "userSelectedDs")]
    #[strum(serialize = "userSelectedDs")]
    UserSelectedDs,
    #[serde(rename = "allFeasibleDs")]
    #[strum(serialize = "allFeasibleDs")]
    AllFeasibleDs,
    #[serde(rename = "allFeasibleDsWithUserPreference")]
    #[strum(serialize = "allFeasibleDsWithUserPreference")]
    AllFeasibleDsWithUserPreference,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible states of an HA service.
/// 
/// All services support the
/// disabled and enabled states.
/// 
/// Possible values:
/// - `disabled`: HA service is disabled.
/// - `enabled`: HA service is enabled.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterDasConfigInfoServiceStateEnum {
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    #[strum(serialize = "enabled")]
    Enabled,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The *ClusterDasConfigInfoVmMonitoringState_enum* enum defines values that indicate
/// the state of Virtual Machine Health Monitoring.
/// 
/// Health Monitoring
/// uses the vmTools (guest) and application agent heartbeat modules.
/// You can configure HA to respond to heartbeat failures of either one
/// or both modules. You can also disable the HA response to heartbeat failures.
/// - To set the cluster default for health monitoring, use the
///   ClusterConfigSpecEx.dasConfig.*ClusterDasConfigInfo.vmMonitoring* property.
/// - To set health monitoring for a virtual machine, use the
///   ClusterConfigSpecEx.dasVmConfigSpec.info.dasSettings.*ClusterDasVmSettings.vmToolsMonitoringSettings* property.
/// - To retrieve the current state of health monitoring (cluster setting), use the
///   ClusterConfigInfoEx.dasConfig.*ClusterDasConfigInfo.vmMonitoring*
///   property.
/// - To retrieve the current state of health monitoring for a virtual machine, use the
///   ClusterConfigInfoEx.dasVmConfig\[\].dasSettings.vmToolsMonitoringSettings.*ClusterVmToolsMonitoringSettings.vmMonitoring*
///   property.
///   
/// Possible values:
/// - `vmMonitoringDisabled`: Virtual machine health monitoring is disabled.
///   
///   In this state,
///   HA response to guest and application heartbeat failures are disabled.
/// - `vmMonitoringOnly`: HA response to guest heartbeat failure is enabled.
///   
///   To retrieve the guest heartbeat status, use the
///   *VirtualMachine*.*VirtualMachine.guestHeartbeatStatus*
///   property.
/// - `vmAndAppMonitoring`: HA response to both guest and application heartbeat failure is enabled.
///   - To retrieve the guest heartbeat status, use the
///     *VirtualMachine*.*VirtualMachine.guestHeartbeatStatus*
///     property.
///   - To retrieve the application heartbeat status, use the
///     *GuestInfo*.*GuestInfo.appHeartbeatStatus*
///     property.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterDasConfigInfoVmMonitoringStateEnum {
    #[serde(rename = "vmMonitoringDisabled")]
    #[strum(serialize = "vmMonitoringDisabled")]
    VmMonitoringDisabled,
    #[serde(rename = "vmMonitoringOnly")]
    #[strum(serialize = "vmMonitoringOnly")]
    VmMonitoringOnly,
    #[serde(rename = "vmAndAppMonitoring")]
    #[strum(serialize = "vmAndAppMonitoring")]
    VmAndAppMonitoring,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The *ClusterDasFdmAvailabilityState_enum* enumeration describes the
/// availability states of hosts in a vSphere HA cluster.
/// 
/// In the HA
/// architecture, a agent called the Fault Domain Manager runs on
/// each active host. These agents elect a master and the others become
/// its slaves. The availability state assigned to a given host is
/// determined from information reported by the Fault Domain Manager
/// running on the host, by a Fault Domain Manager that has been elected
/// master, and by vCenter Server. See *ClusterDasFdmHostState*
/// for more information about the vSphere HA architecture.
/// 
/// Possible values:
/// - `uninitialized`: The Fault Domain Manager for the host has not yet been
///   initialized.
///   
///   Hence the host is not part of a vSphere HA
///   fault domain. This state is reported by vCenter Server or
///   by the host itself.
/// - `election`: The Fault Domain Manager on the host has been initialized and
///   the host is either waiting to join the existing master or
///   is participating in an election for a new master.
///   
///   This state
///   is reported by vCenter Server or by the host itself.
/// - `master`: The Fault Domain Manager on the host has been elected a
///   master.
///   
///   This state is reported by the the host itself.
/// - `connectedToMaster`: The normal operating state for a slave host.
///   
///   In this state,
///   the host is exchanging heartbeats with a master over
///   the management network, and is thus connected to it. If
///   there is a management network partition, the slave will be
///   in this state only if it is in the same partition as the master.
///   This state is reported by the master of a slave host.
/// - `networkPartitionedFromMaster`: A slave host is alive and has management network connectivity, but
///   the management network has been partitioned.
///   
///   This state is reported
///   by masters that are in a partition other than the one containing the
///   slave host; the master in the slave's partition will report the slave state
///   as *connectedToMaster*.
/// - `networkIsolated`: A host is alive but is isolated from the management network.
///   
///   See *ClusterDasVmSettingsIsolationResponse_enum* for the criteria
///   used to determine whether a host is isolated.
/// - `hostDown`: The slave host appears to be down.
///   
///   This state is reported by the
///   master of a slave host.
/// - `initializationError`: An error occurred when initilizating the Fault Domain Manager
///   on a host due to a problem with installing the
///   agent or configuring it.
///   
///   This condition can often be cleared by
///   reconfiguring HA for the host. This state is reported by vCenter
///   Server.
/// - `uninitializationError`: An error occurred when unconfiguring the Fault Domain Manager
///   running on a host.
///   
///   In order to clear this condition the host might
///   need to be reconnected to the cluster and reconfigured first.
///   This state is reported by vCenter
///   Server.
/// - `fdmUnreachable`: The Fault Domain Manager (FDM) on the host cannot be reached.
///   
///   This
///   state is reported in two unlikely situations.
///   - First, it is reported by
///     a master if the host responds to ICMP pings sent by the master over the
///     management network but the FDM on the host cannot be reached by the master.
///     This situation will occur if the FDM is unable to run or exit the
///     uninitialized state.
///   - Second, it is reported by vCenter Server if it cannot connect to a
///     master nor the FDM for the host. This situation would occur if all hosts
///     in the cluster failed but vCenter Server is still running. It may also
///     occur if all FDMs are unable to run or exit the uninitialized state.
/// - `retry`: Config/Reconfig/upgrade operation has failed in first attempt and
///   a retry of these operations is scheduled.
///   
///   If any of the retry attempts succeed, the state is set to initialized.
///   If all retry attempts fail, the state is set to initializationError.
///   This state is reported by vCenter.
///   
///   ***Since:*** vSphere API Release 8.0.0.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterDasFdmAvailabilityStateEnum {
    #[serde(rename = "uninitialized")]
    #[strum(serialize = "uninitialized")]
    Uninitialized,
    #[serde(rename = "election")]
    #[strum(serialize = "election")]
    Election,
    #[serde(rename = "master")]
    #[strum(serialize = "master")]
    Master,
    #[serde(rename = "connectedToMaster")]
    #[strum(serialize = "connectedToMaster")]
    ConnectedToMaster,
    #[serde(rename = "networkPartitionedFromMaster")]
    #[strum(serialize = "networkPartitionedFromMaster")]
    NetworkPartitionedFromMaster,
    #[serde(rename = "networkIsolated")]
    #[strum(serialize = "networkIsolated")]
    NetworkIsolated,
    #[serde(rename = "hostDown")]
    #[strum(serialize = "hostDown")]
    HostDown,
    #[serde(rename = "initializationError")]
    #[strum(serialize = "initializationError")]
    InitializationError,
    #[serde(rename = "uninitializationError")]
    #[strum(serialize = "uninitializationError")]
    UninitializationError,
    #[serde(rename = "fdmUnreachable")]
    #[strum(serialize = "fdmUnreachable")]
    FdmUnreachable,
    #[serde(rename = "retry")]
    #[strum(serialize = "retry")]
    Retry,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated as of VI API 2.5, use *ClusterDasVmSettingsRestartPriority_enum*.
/// 
/// The priority of the virtual machine determines the preference
/// given to it if sufficient capacity is not available to power
/// on all failed virtual machines.
/// 
/// For example, high priority
/// virtual machines on a host get preference over low priority
/// virtual machines.
/// 
/// Possible values:
/// - `disabled`: vSphere HA is disabled for this virtual machine.
/// - `low`: Virtual machines with this priority have a lower chance of powering on after a
///   failure if there is insufficient capacity on hosts to meet all virtual machine
///   needs.
/// - `medium`: Virtual machines with this priority have an intermediate chance of powering
///   on after a failure if there is insufficient capacity on hosts to meet all
///   virtual machine needs.
/// - `high`: Virtual machines with this priority have a higher chance of powering on after a
///   failure if there is insufficient capacity on hosts to meet all virtual machine
///   needs.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DasVmPriorityEnum {
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "low")]
    #[strum(serialize = "low")]
    Low,
    #[serde(rename = "medium")]
    #[strum(serialize = "medium")]
    Medium,
    #[serde(rename = "high")]
    #[strum(serialize = "high")]
    High,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The *ClusterDasVmSettingsIsolationResponse_enum* enum defines
/// values that indicate whether or not the virtual machine should be
/// powered off if a host determines that it is isolated from the rest of
/// the cluster.
/// 
/// Host network isolation occurs when a host is still running but it can no
/// longer communicate with other hosts in the cluster and it cannot ping
/// the configured isolation address(es). When the HA agent on a host loses
/// contact with the other hosts, it will ping the isolation addresses. If
/// the pings fail, the host will declare itself isolated.
/// 
/// Once the HA agent declares the host isolated, it will initiate the
/// isolation response workflow after a 30 second delay. You can use the FDM
/// advanced option fdm.isolationPolicyDelaySec to increase the delay. For
/// each virtual machine, the HA agent attempts to determine if a master is
/// responsible for restarting the virtual machine. If it cannot make the
/// determination, or there is a master that is responsible, the agent will
/// apply the configured isolation response. This workflow will continue
/// until the configuration policy, has been applied to all virtual
/// machines, the agent reconnects to another HA agent in the cluster, or
/// the isolation address pings start succeeding. If there is a master agent
/// in the cluster, it will attempt to restart the virtual machines that
/// were powered off during isolation.
/// 
/// By default, the isolated host leaves its virtual machines powered on.
/// You can override the isolation response default with a cluster-wide
/// setting (*ClusterDasConfigInfo.defaultVmSettings*)
/// or a virtual machine setting
/// (*ClusterDasVmSettings.isolationResponse*).
/// - All isolation response values are valid for the
///   *ClusterDasVmSettings.isolationResponse*
///   property specified in a single virtual machine HA configuration.
/// - All values except for <code>clusterIsolationResponse</code> are valid
///   for the cluster-wide default HA configuration for virtual machines
///   (*ClusterDasConfigInfo.defaultVmSettings*).
///   
/// If you ensure that your network infrastructure is sufficiently redundant
/// and that at least one network path is available at all times, host network
/// isolation should be a rare occurrence.
/// 
/// Possible values:
/// - `none`: Do not power off the virtual machine in the event of a host network
///   isolation.
/// - `powerOff`: Power off the virtual machine in the event of a host network
///   isolation.
/// - `shutdown`: Shut down the virtual machine guest operating system in the event of
///   a host network isolation.
///   
///   If the guest operating system fails to
///   shutdown within five minutes, HA will initiate a forced power off.
///   
///   When you use the shutdown isolation response, failover can take
///   longer (compared to the
///   *powerOff*
///   response) because the virtual machine cannot fail over until it is
///   shutdown.
/// - `clusterIsolationResponse`: Use the default isolation response defined for the cluster
///   that contains this virtual machine.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterDasVmSettingsIsolationResponseEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "powerOff")]
    #[strum(serialize = "powerOff")]
    PowerOff,
    #[serde(rename = "shutdown")]
    #[strum(serialize = "shutdown")]
    Shutdown,
    #[serde(rename = "clusterIsolationResponse")]
    #[strum(serialize = "clusterIsolationResponse")]
    ClusterIsolationResponse,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The *ClusterDasVmSettingsRestartPriority_enum* enum defines
/// virtual machine restart priority values to resolve resource contention.
/// 
/// The priority determines the preference that HA gives to a virtual
/// machine if sufficient capacity is not available to power on all failed
/// virtual machines. For example, high priority virtual machines on a host
/// get preference over low priority virtual machines.
/// 
/// All priority values are valid for the restart priority specified in a
/// single virtual machine HA configuration (*ClusterDasVmConfigInfo.dasSettings*).
/// All values except for <code>clusterRestartPriority</code> are valid for
/// the cluster-wide default HA configuration for virtual machines
/// (*ClusterDasConfigInfo.defaultVmSettings*).
/// 
/// Possible values:
/// - `disabled`: vSphere HA is disabled for this virtual machine.
/// - `lowest`: Virtual machines with this priority have the lowest chance of
///   powering on after a failure if there is insufficient capacity on
///   hosts to meet all virtual machine needs.
/// - `low`: Virtual machines with this priority have a lower chance of powering
///   on after a failure if there is insufficient capacity on hosts to meet
///   all virtual machine needs.
/// - `medium`: Virtual machines with this priority have an intermediate chance of
///   powering on after a failure if there is insufficient capacity on
///   hosts to meet all virtual machine needs.
/// - `high`: Virtual machines with this priority have a higher chance of powering
///   on after a failure if there is insufficient capacity on hosts to meet
///   all virtual machine needs.
/// - `highest`: Virtual machines with this priority have the highest chance of
///   powering on after a failure if there is insufficient capacity on
///   hosts to meet all virtual machine needs.
/// - `clusterRestartPriority`: Virtual machines with this priority use the default restart
///   priority defined for the cluster that contains this virtual machine.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterDasVmSettingsRestartPriorityEnum {
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "lowest")]
    #[strum(serialize = "lowest")]
    Lowest,
    #[serde(rename = "low")]
    #[strum(serialize = "low")]
    Low,
    #[serde(rename = "medium")]
    #[strum(serialize = "medium")]
    Medium,
    #[serde(rename = "high")]
    #[strum(serialize = "high")]
    High,
    #[serde(rename = "highest")]
    #[strum(serialize = "highest")]
    Highest,
    #[serde(rename = "clusterRestartPriority")]
    #[strum(serialize = "clusterRestartPriority")]
    ClusterRestartPriority,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `manual`: Specifies that VirtualCenter should generate recommendations
///   for host power operations, but should not execute the
///   recommendations automatically.
/// - `automated`: Specifies that VirtualCenter should generate recommendations
///   for host power operations, and should execute the
///   recommendations automatically.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DpmBehaviorEnum {
    #[serde(rename = "manual")]
    #[strum(serialize = "manual")]
    Manual,
    #[serde(rename = "automated")]
    #[strum(serialize = "automated")]
    Automated,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `manual`: Specifies that VirtualCenter should generate recommendations for
///   virtual machine migration and for placement with a host,
///   but should not implement the recommendations automatically.
/// - `partiallyAutomated`: Specifies that VirtualCenter should generate recommendations for
///   virtual machine migration and for placement with a host,
///   but should automatically implement only the placement at power on.
/// - `fullyAutomated`: Specifies that VirtualCenter should automate both the migration
///   of virtual machines and their placement with a host at power on.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DrsBehaviorEnum {
    #[serde(rename = "manual")]
    #[strum(serialize = "manual")]
    Manual,
    #[serde(rename = "partiallyAutomated")]
    #[strum(serialize = "partiallyAutomated")]
    PartiallyAutomated,
    #[serde(rename = "fullyAutomated")]
    #[strum(serialize = "fullyAutomated")]
    FullyAutomated,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated as of VI API 2.5 use *RecommendationReasonCode_enum*.
/// 
/// List of defined migration reason codes:
/// 
/// Possible values:
/// - `fairnessCpuAvg`: Balance average CPU utilization.
/// - `fairnessMemAvg`: Balance average memory utilization.
/// - `jointAffin`: Fulfill affinity rule.
/// - `antiAffin`: Fulfill anti-affinity rule.
/// - `hostMaint`: Host entering maintenance mode.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DrsRecommendationReasonCodeEnum {
    #[serde(rename = "fairnessCpuAvg")]
    #[strum(serialize = "fairnessCpuAvg")]
    FairnessCpuAvg,
    #[serde(rename = "fairnessMemAvg")]
    #[strum(serialize = "fairnessMemAvg")]
    FairnessMemAvg,
    #[serde(rename = "jointAffin")]
    #[strum(serialize = "jointAffin")]
    JointAffin,
    #[serde(rename = "antiAffin")]
    #[strum(serialize = "antiAffin")]
    AntiAffin,
    #[serde(rename = "hostMaint")]
    #[strum(serialize = "hostMaint")]
    HostMaint,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Describes the operation type of the action.
/// 
/// enterexitQuarantine suggests
/// that the host is only exiting the quarantine state (i.e. not the
/// maintenance mode).
/// 
/// Possible values:
/// - `enterQuarantine`
/// - `exitQuarantine`
/// - `enterMaintenance`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterHostInfraUpdateHaModeActionOperationTypeEnum {
    #[serde(rename = "enterQuarantine")]
    #[strum(serialize = "enterQuarantine")]
    EnterQuarantine,
    #[serde(rename = "exitQuarantine")]
    #[strum(serialize = "exitQuarantine")]
    ExitQuarantine,
    #[serde(rename = "enterMaintenance")]
    #[strum(serialize = "enterMaintenance")]
    EnterMaintenance,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `powerOn`: Power On Operation
/// - `powerOff`: Power Off Operation.
///   
///   Power off operation puts the host in
///   a state that can be woken up remotely.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostPowerOperationTypeEnum {
    #[serde(rename = "powerOn")]
    #[strum(serialize = "powerOn")]
    PowerOn,
    #[serde(rename = "powerOff")]
    #[strum(serialize = "powerOff")]
    PowerOff,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `Manual`: With this behavior configured, the proposed DRS recommendations
///   require manual approval before they are executed.
/// - `Automated`: With this behavior configured, the proposed DRS recommendations are
///   executed immediately.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterInfraUpdateHaConfigInfoBehaviorTypeEnum {
    Manual,
    Automated,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `QuarantineMode`: With this behavior configured, a degraded host will be recommended
///   to be placed in Quarantine Mode.
/// - `MaintenanceMode`: With this behavior configured, a degraded host will be recommended
///   to be placed in Maintenance Mode.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterInfraUpdateHaConfigInfoRemediationTypeEnum {
    QuarantineMode,
    MaintenanceMode,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines the type of placement
/// 
/// Possible values:
/// - `create`: Create a new VM
/// - `reconfigure`: Reconfigure a VM
/// - `relocate`: Relocate a VM
/// - `clone`: Clone a VM
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum PlacementSpecPlacementTypeEnum {
    #[serde(rename = "create")]
    #[strum(serialize = "create")]
    Create,
    #[serde(rename = "reconfigure")]
    #[strum(serialize = "reconfigure")]
    Reconfigure,
    #[serde(rename = "relocate")]
    #[strum(serialize = "relocate")]
    Relocate,
    #[serde(rename = "clone")]
    #[strum(serialize = "clone")]
    Clone,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines the options for a Datacenter::powerOnVm() invocation.
/// 
/// Possible values:
/// - `OverrideAutomationLevel`: Override the DRS automation level.
///   
///   Value type: *DrsBehavior_enum*
///   Default value: current behavior
/// - `ReserveResources`: Reserve resources for the powering-on VMs throughout the
///   power-on session.
///   
///   When this option is set to true, the server
///   will return at most one recommended host per manual VM, and
///   the VM's reservations are held on the recommended host until
///   the VM is actually powered on (either by applying the
///   recommendation or by a power-on request on the VM), or until
///   the recommendation is cancelled, or until the recommendation
///   expires. The expiration time is currently set to 10
///   minutes. This option does not have an effect on automatic VMs
///   since their recommendations are executed immediately. This
///   option is effective on DRS clusters only.
///   Value type: boolean
///   Default value: false
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterPowerOnVmOptionEnum {
    OverrideAutomationLevel,
    ReserveResources,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// List of defined migration reason codes:
/// 
/// Possible values:
/// - `fairnessCpuAvg`: Balance average CPU utilization.
/// - `fairnessMemAvg`: Balance average memory utilization.
/// - `jointAffin`: Fulfill affinity rule.
/// - `antiAffin`: Fulfill anti-affinity rule.
/// - `hostMaint`: Host entering maintenance mode.
/// - `enterStandby`: Host entering standby mode.
/// - `reservationCpu`: balance CPU reservations
/// - `reservationMem`: balance memory reservations
/// - `powerOnVm`: Power on virtual machine
/// - `powerSaving`: Power off host for power savings
/// - `increaseCapacity`: Power on host to increase cluster capacity
/// - `checkResource`: Sanity-check resource pool hierarchy
/// - `unreservedCapacity`: Maintain unreserved capacity
/// - `vmHostHardAffinity`: Fix hard VM/host affinity rule violation
/// - `vmHostSoftAffinity`: Fix soft VM/host affinity rule violation
/// - `balanceDatastoreSpaceUsage`: Balance datastore space usage.
/// - `balanceDatastoreIOLoad`: Balance datastore I/O workload.
/// - `balanceDatastoreIOPSReservation`: Balance datastore IOPS reservation
/// - `datastoreMaint`: Datastore entering maintenance mode.
/// - `virtualDiskJointAffin`: Fix virtual disk affinity rule violation.
/// - `virtualDiskAntiAffin`: Fix virtual disk anti-affinity rule violation.
/// - `datastoreSpaceOutage`: Fix the issue that a datastore run out of space.
/// - `storagePlacement`: Satisfy storage initial placement requests.
/// - `iolbDisabledInternal`: IO load balancing was disabled internally.
/// - `xvmotionPlacement`: Satisfy unified vmotion placement requests.
/// - `networkBandwidthReservation`: Fix network bandwidth reservation violation
/// - `hostInDegradation`: Host is partially degraded.
/// - `hostExitDegradation`: Host is not degraded.
/// - `maxVmsConstraint`: Fix maxVms constraint violation
/// - `ftConstraints`: Fix ft maxVMs and maxVcpus constraint violations
/// - `vmHostAffinityPolicy`: Fix VM/host affinity policy violation
/// - `vmHostAntiAffinityPolicy`: Fix VM/host anti-affinity policy violation
/// - `vmAntiAffinityPolicy`: Fix VM-VM anti-affinity policy violations
/// - `balanceVsanUsage`: ***Since:*** vSphere API Release 7.0.2.0
/// - `ahPlacementOptimization`: Optimize assignable hardware resource orchestration
///   
///   ***Since:*** vSphere API Release 8.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum RecommendationReasonCodeEnum {
    #[serde(rename = "fairnessCpuAvg")]
    #[strum(serialize = "fairnessCpuAvg")]
    FairnessCpuAvg,
    #[serde(rename = "fairnessMemAvg")]
    #[strum(serialize = "fairnessMemAvg")]
    FairnessMemAvg,
    #[serde(rename = "jointAffin")]
    #[strum(serialize = "jointAffin")]
    JointAffin,
    #[serde(rename = "antiAffin")]
    #[strum(serialize = "antiAffin")]
    AntiAffin,
    #[serde(rename = "hostMaint")]
    #[strum(serialize = "hostMaint")]
    HostMaint,
    #[serde(rename = "enterStandby")]
    #[strum(serialize = "enterStandby")]
    EnterStandby,
    #[serde(rename = "reservationCpu")]
    #[strum(serialize = "reservationCpu")]
    ReservationCpu,
    #[serde(rename = "reservationMem")]
    #[strum(serialize = "reservationMem")]
    ReservationMem,
    #[serde(rename = "powerOnVm")]
    #[strum(serialize = "powerOnVm")]
    PowerOnVm,
    #[serde(rename = "powerSaving")]
    #[strum(serialize = "powerSaving")]
    PowerSaving,
    #[serde(rename = "increaseCapacity")]
    #[strum(serialize = "increaseCapacity")]
    IncreaseCapacity,
    #[serde(rename = "checkResource")]
    #[strum(serialize = "checkResource")]
    CheckResource,
    #[serde(rename = "unreservedCapacity")]
    #[strum(serialize = "unreservedCapacity")]
    UnreservedCapacity,
    #[serde(rename = "vmHostHardAffinity")]
    #[strum(serialize = "vmHostHardAffinity")]
    VmHostHardAffinity,
    #[serde(rename = "vmHostSoftAffinity")]
    #[strum(serialize = "vmHostSoftAffinity")]
    VmHostSoftAffinity,
    #[serde(rename = "balanceDatastoreSpaceUsage")]
    #[strum(serialize = "balanceDatastoreSpaceUsage")]
    BalanceDatastoreSpaceUsage,
    #[serde(rename = "balanceDatastoreIOLoad")]
    #[strum(serialize = "balanceDatastoreIOLoad")]
    BalanceDatastoreIoLoad,
    #[serde(rename = "balanceDatastoreIOPSReservation")]
    #[strum(serialize = "balanceDatastoreIOPSReservation")]
    BalanceDatastoreIopsReservation,
    #[serde(rename = "datastoreMaint")]
    #[strum(serialize = "datastoreMaint")]
    DatastoreMaint,
    #[serde(rename = "virtualDiskJointAffin")]
    #[strum(serialize = "virtualDiskJointAffin")]
    VirtualDiskJointAffin,
    #[serde(rename = "virtualDiskAntiAffin")]
    #[strum(serialize = "virtualDiskAntiAffin")]
    VirtualDiskAntiAffin,
    #[serde(rename = "datastoreSpaceOutage")]
    #[strum(serialize = "datastoreSpaceOutage")]
    DatastoreSpaceOutage,
    #[serde(rename = "storagePlacement")]
    #[strum(serialize = "storagePlacement")]
    StoragePlacement,
    #[serde(rename = "iolbDisabledInternal")]
    #[strum(serialize = "iolbDisabledInternal")]
    IolbDisabledInternal,
    #[serde(rename = "xvmotionPlacement")]
    #[strum(serialize = "xvmotionPlacement")]
    XvmotionPlacement,
    #[serde(rename = "networkBandwidthReservation")]
    #[strum(serialize = "networkBandwidthReservation")]
    NetworkBandwidthReservation,
    #[serde(rename = "hostInDegradation")]
    #[strum(serialize = "hostInDegradation")]
    HostInDegradation,
    #[serde(rename = "hostExitDegradation")]
    #[strum(serialize = "hostExitDegradation")]
    HostExitDegradation,
    #[serde(rename = "maxVmsConstraint")]
    #[strum(serialize = "maxVmsConstraint")]
    MaxVmsConstraint,
    #[serde(rename = "ftConstraints")]
    #[strum(serialize = "ftConstraints")]
    FtConstraints,
    #[serde(rename = "vmHostAffinityPolicy")]
    #[strum(serialize = "vmHostAffinityPolicy")]
    VmHostAffinityPolicy,
    #[serde(rename = "vmHostAntiAffinityPolicy")]
    #[strum(serialize = "vmHostAntiAffinityPolicy")]
    VmHostAntiAffinityPolicy,
    #[serde(rename = "vmAntiAffinityPolicy")]
    #[strum(serialize = "vmAntiAffinityPolicy")]
    VmAntiAffinityPolicy,
    #[serde(rename = "balanceVsanUsage")]
    #[strum(serialize = "balanceVsanUsage")]
    BalanceVsanUsage,
    #[serde(rename = "ahPlacementOptimization")]
    #[strum(serialize = "ahPlacementOptimization")]
    AhPlacementOptimization,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Pre-defined constants for possible recommendation types.
/// 
/// Virtual Center
/// uses this information to coordinate with the clients.
/// 
/// Possible values:
/// - `V1`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum RecommendationTypeEnum {
    V1,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `SYSTEM_MANAGED`: System VMs are fully managed by the system.
/// - `ABSENT`: System VMs are absent on the managed entity.
///   
/// ***Since:*** vSphere API Release 8.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterSystemVMsConfigInfoDeploymentModeEnum {
    #[serde(rename = "SYSTEM_MANAGED")]
    #[strum(serialize = "SYSTEM_MANAGED")]
    SystemManaged,
    #[serde(rename = "ABSENT")]
    #[strum(serialize = "ABSENT")]
    Absent,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The VM policy settings that determine the response to
/// storage failures.
/// 
/// Possible values:
/// - `disabled`: VM Component Protection service will not monitor or react to
///   the component failure.
///   
///   This setting does not affect other vSphere
///   HA services such as Host Monitoring or VM Health Monitoring.
/// - `warning`: VM Component Protection service will monitor component failures but
///   will not restart an affected VM.
///   
///   Rather it will notify users about
///   the component failures. This setting does not affect other vSphere HA
///   services such as Host Monitoring or VM Health Monitoring.
/// - `restartConservative`: VM Component Protection service protects VMs conservatively.
///   
///   With this
///   setting, when the service can't determine that capacity is available to
///   restart a VM, it will favor keeping the VM running.
/// - `restartAggressive`: VM Component Protection service protects VMs aggressively.
///   
///   With this setting,
///   the service will terminate an affected VM even if it can't determine that
///   capacity exists to restart the VM.
/// - `clusterDefault`: VM will use the cluster default setting.
///   
///   This option is only meaningful for
///   per-VM settings.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterVmComponentProtectionSettingsStorageVmReactionEnum {
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "warning")]
    #[strum(serialize = "warning")]
    Warning,
    #[serde(rename = "restartConservative")]
    #[strum(serialize = "restartConservative")]
    RestartConservative,
    #[serde(rename = "restartAggressive")]
    #[strum(serialize = "restartAggressive")]
    RestartAggressive,
    #[serde(rename = "clusterDefault")]
    #[strum(serialize = "clusterDefault")]
    ClusterDefault,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// If an APD condition clears after an APD timeout condition has been declared and before
/// VM Component Protection service terminated the VM, the guestOS and application may
/// no longer be operational.
/// 
/// VM Component Protection may be configured to reset the
/// VM (*VirtualMachine.ResetVM_Task*) to restore the service of guest applications.
/// 
/// Possible values:
/// - `none`: VM Component Protection service will not react after APD condition is cleared.
/// - `reset`: VM Component Protection service will reset the VM after APD condition is cleared.
///   
///   Note this only applies if the subject VM is still powered on.
/// - `useClusterDefault`: VM will use the cluster default setting.
///   
///   This option is only meaningful for
///   per-VM settings.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterVmComponentProtectionSettingsVmReactionOnApdClearedEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "reset")]
    #[strum(serialize = "reset")]
    Reset,
    #[serde(rename = "useClusterDefault")]
    #[strum(serialize = "useClusterDefault")]
    UseClusterDefault,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Condition for VM's readiness
/// 
/// Possible values:
/// - `none`: No ready condition specified.
///   
///   In case of vSphere HA, higher restart priority VMs are still
///   placed before lower priority VMs.
/// - `poweredOn`: VM is powered on.
/// - `guestHbStatusGreen`: VM guest operating system is up and responding normally (VM tools
///   heartbeat status is green).
/// - `appHbStatusGreen`: An application running inside the VM is responding normally.
///   
///   To enable Application Monitoring, you must first obtain the
///   appropriate SDK (or be using an application that supports VMware
///   Application Monitoring) and use it to set up customized heartbeats
///   for the applications you want to monitor.
///   See *ClusterDasConfigInfo.vmMonitoring*.
/// - `useClusterDefault`: VM will use the cluster default setting.
///   
///   This option is only
///   meaningful for per-VM settings.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterVmReadinessReadyConditionEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "poweredOn")]
    #[strum(serialize = "poweredOn")]
    PoweredOn,
    #[serde(rename = "guestHbStatusGreen")]
    #[strum(serialize = "guestHbStatusGreen")]
    GuestHbStatusGreen,
    #[serde(rename = "appHbStatusGreen")]
    #[strum(serialize = "appHbStatusGreen")]
    AppHbStatusGreen,
    #[serde(rename = "useClusterDefault")]
    #[strum(serialize = "useClusterDefault")]
    UseClusterDefault,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Network Filter on Failure Type.
/// 
/// It specifies whether all the
/// packets will be allowed or all the packets will be denied when
/// Filter fails to configure.
/// 
/// Possible values:
/// - `failOpen`: Allows all the packets when the Filter fails to configure.
/// - `failClosed`: Denies all the packets when the Filter fails to configure.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DvsFilterOnFailureEnum {
    #[serde(rename = "failOpen")]
    #[strum(serialize = "failOpen")]
    FailOpen,
    #[serde(rename = "failClosed")]
    #[strum(serialize = "failClosed")]
    FailClosed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for
/// *DVPortStatus*.*DVPortStatus.vmDirectPathGen2InactiveReasonNetwork*.
/// 
/// Possible values:
/// - `portNptIncompatibleDvs`: The switch for which this port is defined does not support VMDirectPath Gen 2.
///   
///   See
///   *DVSFeatureCapability*.*DVSFeatureCapability.vmDirectPathGen2Supported*.
/// - `portNptNoCompatibleNics`: None of the physical NICs used as uplinks for this port support
///   VMDirectPath Gen 2.
///   
///   See also *PhysicalNic.vmDirectPathGen2Supported*.
/// - `portNptNoVirtualFunctionsAvailable`: At least some of the physical NICs used as uplinks for this port
///   support VMDirectPath Gen 2, but all available network-passthrough
///   resources are in use by other ports.
/// - `portNptDisabledForPort`: VMDirectPath Gen 2 has been explicitly disabled for this port.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DvPortStatusVmDirectPathGen2InactiveReasonNetworkEnum {
    #[serde(rename = "portNptIncompatibleDvs")]
    #[strum(serialize = "portNptIncompatibleDvs")]
    PortNptIncompatibleDvs,
    #[serde(rename = "portNptNoCompatibleNics")]
    #[strum(serialize = "portNptNoCompatibleNics")]
    PortNptNoCompatibleNics,
    #[serde(rename = "portNptNoVirtualFunctionsAvailable")]
    #[strum(serialize = "portNptNoVirtualFunctionsAvailable")]
    PortNptNoVirtualFunctionsAvailable,
    #[serde(rename = "portNptDisabledForPort")]
    #[strum(serialize = "portNptDisabledForPort")]
    PortNptDisabledForPort,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for
/// *DVPortStatus*.*DVPortStatus.vmDirectPathGen2InactiveReasonOther*.
/// 
/// Possible values:
/// - `portNptIncompatibleHost`: The host for which this port is defined does not support VMDirectPath Gen 2.
///   
///   See *HostCapability*.*HostCapability.vmDirectPathGen2Supported*
/// - `portNptIncompatibleConnectee`: Configuration or state of the port's connectee prevents
///   VMDirectPath Gen 2.
///   
///   See
///   *VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState.vmDirectPathGen2InactiveReasonVm*
///   and/or
///   *VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState.vmDirectPathGen2InactiveReasonExtended*
///   in the appropriate element of the RuntimeInfo.device array of the
///   virtual machine connected to this port.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DvPortStatusVmDirectPathGen2InactiveReasonOtherEnum {
    #[serde(rename = "portNptIncompatibleHost")]
    #[strum(serialize = "portNptIncompatibleHost")]
    PortNptIncompatibleHost,
    #[serde(rename = "portNptIncompatibleConnectee")]
    #[strum(serialize = "portNptIncompatibleConnectee")]
    PortNptIncompatibleConnectee,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The *DistributedVirtualPortgroupBackingType_enum* enum defines
/// the distributed virtual portgroup backing type.
/// 
/// Possible values:
/// - `standard`: The portgroup is created by vCenter.
/// - `nsx`: The portgroup is created by NSX manager.
///   
///   For NSX backing type, We only support ephemeral portgroup type.
///   If *DistributedVirtualPortgroupPortgroupType_enum* is
///   ephemeral, A *DistributedVirtualPort* will be
///   dynamicly created by NSX when the virtual machine is reconfigured
///   to connect to the portgroup.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DistributedVirtualPortgroupBackingTypeEnum {
    #[serde(rename = "standard")]
    #[strum(serialize = "standard")]
    Standard,
    #[serde(rename = "nsx")]
    #[strum(serialize = "nsx")]
    Nsx,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The meta tag names recognizable in the
/// *DVPortgroupConfigInfo.portNameFormat* string.
/// 
/// Possible values:
/// - `dvsName`: This tag will be expanded to the name of the switch.
/// - `portgroupName`: This tag will be expanded to the name of the portgroup.
/// - `portIndex`: This tag will be expanded to the current index of the port.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DistributedVirtualPortgroupMetaTagNameEnum {
    #[serde(rename = "dvsName")]
    #[strum(serialize = "dvsName")]
    DvsName,
    #[serde(rename = "portgroupName")]
    #[strum(serialize = "portgroupName")]
    PortgroupName,
    #[serde(rename = "portIndex")]
    #[strum(serialize = "portIndex")]
    PortIndex,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The *DistributedVirtualPortgroupPortgroupType_enum* enum defines
/// the distributed virtual portgroup types
/// (*DistributedVirtualPortgroup*.*DistributedVirtualPortgroup.config*.*DVPortgroupConfigInfo.type*).
/// 
/// Early binding specifies a static set of ports that are created
/// when you create the distributed virtual portgroup. An ephemeral portgroup uses dynamic
/// ports that are created when you power on a virtual machine.
/// 
/// Possible values:
/// - `earlyBinding`: A free *DistributedVirtualPort* will be selected and assigned to
///   a *VirtualMachine* when the virtual machine is reconfigured to
///   connect to the portgroup.
/// - `lateBinding`: 
///   
///   Deprecated as of vSphere API 5.0.
///   
///   A free *DistributedVirtualPort* will be selected and
///   assigned to a *VirtualMachine* when the virtual machine is
///   powered on.
/// - `ephemeral`: A *DistributedVirtualPort* will be created and assigned to a
///   *VirtualMachine* when the virtual machine is powered on, and will
///   be deleted when the virtual machine is powered off.
///   
///   An ephemeral portgroup has
///   no limit on the number of ports that can be a part of this portgroup.
///   In cases where the vCenter Server is unavailable the host can
///   create conflict ports in this portgroup to be used by a virtual machine
///   at power on.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DistributedVirtualPortgroupPortgroupTypeEnum {
    #[serde(rename = "earlyBinding")]
    #[strum(serialize = "earlyBinding")]
    EarlyBinding,
    #[serde(rename = "lateBinding")]
    #[strum(serialize = "lateBinding")]
    LateBinding,
    #[serde(rename = "ephemeral")]
    #[strum(serialize = "ephemeral")]
    Ephemeral,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The *EntityType_enum* enum identifies
/// the type of entity that was exported
/// (*DistributedVirtualSwitchManager.DVSManagerExportEntity_Task*).
/// 
/// Possible values:
/// - `distributedVirtualSwitch`: Indicates the exported entity is a *DistributedVirtualSwitch*.
/// - `distributedVirtualPortgroup`: Indicates the exported entity is a *DistributedVirtualPortgroup*.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum EntityTypeEnum {
    #[serde(rename = "distributedVirtualSwitch")]
    #[strum(serialize = "distributedVirtualSwitch")]
    DistributedVirtualSwitch,
    #[serde(rename = "distributedVirtualPortgroup")]
    #[strum(serialize = "distributedVirtualPortgroup")]
    DistributedVirtualPortgroup,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The *EntityImportType_enum* enum defines the import type for a
/// *DistributedVirtualSwitchManager*.*DistributedVirtualSwitchManager.DVSManagerImportEntity_Task*
/// operation.
/// 
/// Possible values:
/// - `createEntityWithNewIdentifier`: Create the entity with new identifiers.
///   
///   Specify the
///   *EntityBackupConfig*.*EntityBackupConfig.name* and
///   *EntityBackupConfig*.*EntityBackupConfig.container*
///   properties.
///   
///   The Server ignores any value for the
///   *EntityBackupConfig*.*EntityBackupConfig.key*
///   property.
/// - `createEntityWithOriginalIdentifier`: Recreate the entities with the original identifiers of the entity from which backup was created.
///   
///   The Server throws an exception if an entity with the same identifier already exists.
///   This option will also add the host members to the *DistributedVirtualSwitch* and will
///   try to get the virtual machine networking back with the same *DistributedVirtualPortgroup*.
///   Specify a *Folder* as the
///   *EntityBackupConfig*.*EntityBackupConfig.container*
///   for *EntityBackupConfig*.*EntityBackupConfig.entityType*
///   "distributedVirtualSwitch".
///   
///   The Server ignores any values for the
///   *EntityBackupConfig*.*EntityBackupConfig.key* and
///   *EntityBackupConfig*.*EntityBackupConfig.name*
///   properties.
/// - `applyToEntitySpecified`: Apply the configuration specified in the
///   *EntityBackupConfig*.*EntityBackupConfig.configBlob*
///   property to the entity specified in the
///   *EntityBackupConfig*.*EntityBackupConfig.entityType* and
///   *EntityBackupConfig*.*EntityBackupConfig.key*
///   properties.
///   
///   If you specify
///   *EntityBackupConfig*.*EntityBackupConfig.name*,
///   the Server uses the specified name to rename the entity.
///   
///   The Server ignores any value for the
///   *EntityBackupConfig*.*EntityBackupConfig.container*
///   property.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum EntityImportTypeEnum {
    #[serde(rename = "createEntityWithNewIdentifier")]
    #[strum(serialize = "createEntityWithNewIdentifier")]
    CreateEntityWithNewIdentifier,
    #[serde(rename = "createEntityWithOriginalIdentifier")]
    #[strum(serialize = "createEntityWithOriginalIdentifier")]
    CreateEntityWithOriginalIdentifier,
    #[serde(rename = "applyToEntitySpecified")]
    #[strum(serialize = "applyToEntitySpecified")]
    ApplyToEntitySpecified,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The enum defines the distributed virtual switch mode.
/// 
/// Possible values:
/// - `normal`: traditional package processing mode.
/// - `mux`: ENS mode which skips packet parsing and flow table lookup.
///   
/// ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostDvsConfigSpecSwitchModeEnum {
    #[serde(rename = "normal")]
    #[strum(serialize = "normal")]
    Normal,
    #[serde(rename = "mux")]
    #[strum(serialize = "mux")]
    Mux,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Describes the state of the host proxy switch.
/// 
/// Possible values:
/// - `up`: The host proxy switch is up and running.
/// - `pending`: The host proxy switch is waiting to be initialized.
/// - `outOfSync`: The proxy switch configuration is not the same as the
///   distributed virtual switch configuration in the vCenter Server.
/// - `warning`: The host requires attention.
/// - `disconnected`: The host is disconnected or it is not responding.
/// - `down`: The host proxy is down.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DistributedVirtualSwitchHostMemberHostComponentStateEnum {
    #[serde(rename = "up")]
    #[strum(serialize = "up")]
    Up,
    #[serde(rename = "pending")]
    #[strum(serialize = "pending")]
    Pending,
    #[serde(rename = "outOfSync")]
    #[strum(serialize = "outOfSync")]
    OutOfSync,
    #[serde(rename = "warning")]
    #[strum(serialize = "warning")]
    Warning,
    #[serde(rename = "disconnected")]
    #[strum(serialize = "disconnected")]
    Disconnected,
    #[serde(rename = "down")]
    #[strum(serialize = "down")]
    Down,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Transport zone type.
/// 
/// Possible values:
/// - `vlan`: VLAN based networking
/// - `overlay`: VXLAN based networking
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DistributedVirtualSwitchHostMemberTransportZoneTypeEnum {
    #[serde(rename = "vlan")]
    #[strum(serialize = "vlan")]
    Vlan,
    #[serde(rename = "overlay")]
    #[strum(serialize = "overlay")]
    Overlay,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The connectee types.
/// 
/// Possible values:
/// - `pnic`: The port connects to a Physical NIC.
/// - `vmVnic`: The port connects to a Virtual NIC in a Virtual Machine.
/// - `hostConsoleVnic`: The port connects to a console Virtual NIC on a host.
/// - `hostVmkVnic`: The port connects to a VMkernel Virtual NIC on a host.
/// - `systemCrxVnic`: The port connects to a Virtual NIC in a System CRX VM.
///   
///   ***Since:*** vSphere API Release 8.0.1.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DistributedVirtualSwitchPortConnecteeConnecteeTypeEnum {
    #[serde(rename = "pnic")]
    #[strum(serialize = "pnic")]
    Pnic,
    #[serde(rename = "vmVnic")]
    #[strum(serialize = "vmVnic")]
    VmVnic,
    #[serde(rename = "hostConsoleVnic")]
    #[strum(serialize = "hostConsoleVnic")]
    HostConsoleVnic,
    #[serde(rename = "hostVmkVnic")]
    #[strum(serialize = "hostVmkVnic")]
    HostVmkVnic,
    #[serde(rename = "systemCrxVnic")]
    #[strum(serialize = "systemCrxVnic")]
    SystemCrxVnic,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Network Traffic Rule direction types.
/// 
/// It specifies whether rule
/// needs to be applied for packets which are incoming/outgoing or both.
/// 
/// Possible values:
/// - `incomingPackets`: This specifies that the network rule has to be applied only for
///   incoming packets.
/// - `outgoingPackets`: This specifies that the network rule has to be applied only for
///   outgoing packets.
/// - `both`: This specifies that the network rule has to be applied only for
///   both incoming and outgoing packets.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DvsNetworkRuleDirectionTypeEnum {
    #[serde(rename = "incomingPackets")]
    #[strum(serialize = "incomingPackets")]
    IncomingPackets,
    #[serde(rename = "outgoingPackets")]
    #[strum(serialize = "outgoingPackets")]
    OutgoingPackets,
    #[serde(rename = "both")]
    #[strum(serialize = "both")]
    Both,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Link Aggregation Control Protocol API versions.
/// 
/// Possible values:
/// - `singleLag`: 
///   
///   Deprecated as of vSphere API 7.0u1.
///   
///   One Link Aggregation Control Protocol group in the switch
/// - `multipleLag`: Multiple Link Aggregation Control Protocol in the switch.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VMwareDvsLacpApiVersionEnum {
    #[serde(rename = "singleLag")]
    #[strum(serialize = "singleLag")]
    SingleLag,
    #[serde(rename = "multipleLag")]
    #[strum(serialize = "multipleLag")]
    MultipleLag,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Load balance algorithm in a Link Aggregation Control Protocol group.
/// 
/// Possible values:
/// - `srcMac`: Source MAC address
/// - `destMac`: Destination MAC address
/// - `srcDestMac`: Source and destination MAC address
/// - `destIpVlan`: Destination IP and VLAN
/// - `srcIpVlan`: Source IP and VLAN
/// - `srcDestIpVlan`: Source and destination IP and VLAN
/// - `destTcpUdpPort`: Destination TCP/UDP port number
/// - `srcTcpUdpPort`: Source TCP/UDP port number
/// - `srcDestTcpUdpPort`: Source and destination TCP/UDP port number
/// - `destIpTcpUdpPort`: Destination IP and TCP/UDP port number
/// - `srcIpTcpUdpPort`: Source IP and TCP/UDP port number
/// - `srcDestIpTcpUdpPort`: Source and destination IP and TCP/UDP port number
/// - `destIpTcpUdpPortVlan`: Destination IP, TCP/UDP port number and VLAN
/// - `srcIpTcpUdpPortVlan`: Source IP, TCP/UDP port number and VLAN
/// - `srcDestIpTcpUdpPortVlan`: Source and destination IP,
///   source and destination TCP/UDP port number and VLAN.
/// - `destIp`: Destination IP
/// - `srcIp`: Source IP
/// - `srcDestIp`: Source and Destination IP
/// - `vlan`: VLAN only
/// - `srcPortId`: Source Virtual Port Id
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VMwareDvsLacpLoadBalanceAlgorithmEnum {
    #[serde(rename = "srcMac")]
    #[strum(serialize = "srcMac")]
    SrcMac,
    #[serde(rename = "destMac")]
    #[strum(serialize = "destMac")]
    DestMac,
    #[serde(rename = "srcDestMac")]
    #[strum(serialize = "srcDestMac")]
    SrcDestMac,
    #[serde(rename = "destIpVlan")]
    #[strum(serialize = "destIpVlan")]
    DestIpVlan,
    #[serde(rename = "srcIpVlan")]
    #[strum(serialize = "srcIpVlan")]
    SrcIpVlan,
    #[serde(rename = "srcDestIpVlan")]
    #[strum(serialize = "srcDestIpVlan")]
    SrcDestIpVlan,
    #[serde(rename = "destTcpUdpPort")]
    #[strum(serialize = "destTcpUdpPort")]
    DestTcpUdpPort,
    #[serde(rename = "srcTcpUdpPort")]
    #[strum(serialize = "srcTcpUdpPort")]
    SrcTcpUdpPort,
    #[serde(rename = "srcDestTcpUdpPort")]
    #[strum(serialize = "srcDestTcpUdpPort")]
    SrcDestTcpUdpPort,
    #[serde(rename = "destIpTcpUdpPort")]
    #[strum(serialize = "destIpTcpUdpPort")]
    DestIpTcpUdpPort,
    #[serde(rename = "srcIpTcpUdpPort")]
    #[strum(serialize = "srcIpTcpUdpPort")]
    SrcIpTcpUdpPort,
    #[serde(rename = "srcDestIpTcpUdpPort")]
    #[strum(serialize = "srcDestIpTcpUdpPort")]
    SrcDestIpTcpUdpPort,
    #[serde(rename = "destIpTcpUdpPortVlan")]
    #[strum(serialize = "destIpTcpUdpPortVlan")]
    DestIpTcpUdpPortVlan,
    #[serde(rename = "srcIpTcpUdpPortVlan")]
    #[strum(serialize = "srcIpTcpUdpPortVlan")]
    SrcIpTcpUdpPortVlan,
    #[serde(rename = "srcDestIpTcpUdpPortVlan")]
    #[strum(serialize = "srcDestIpTcpUdpPortVlan")]
    SrcDestIpTcpUdpPortVlan,
    #[serde(rename = "destIp")]
    #[strum(serialize = "destIp")]
    DestIp,
    #[serde(rename = "srcIp")]
    #[strum(serialize = "srcIp")]
    SrcIp,
    #[serde(rename = "srcDestIp")]
    #[strum(serialize = "srcDestIp")]
    SrcDestIp,
    #[serde(rename = "vlan")]
    #[strum(serialize = "vlan")]
    Vlan,
    #[serde(rename = "srcPortId")]
    #[strum(serialize = "srcPortId")]
    SrcPortId,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `allow`
/// - `drop`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DvsMacLimitPolicyTypeEnum {
    #[serde(rename = "allow")]
    #[strum(serialize = "allow")]
    Allow,
    #[serde(rename = "drop")]
    #[strum(serialize = "drop")]
    Drop,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Multicast Filtering mode.
/// 
/// Possible values:
/// - `legacyFiltering`: Legacy filtering mode
/// - `snooping`: IGMP/MLD snooping mode
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VMwareDvsMulticastFilteringModeEnum {
    #[serde(rename = "legacyFiltering")]
    #[strum(serialize = "legacyFiltering")]
    LegacyFiltering,
    #[serde(rename = "snooping")]
    #[strum(serialize = "snooping")]
    Snooping,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The PVLAN port types.
/// 
/// Possible values:
/// - `promiscuous`: The port can communicate with all other ports within the same PVLAN,
///   including the isolated and community ports .
/// - `isolated`: The port can only communicate with the promiscuous ports within the
///   same PVLAN, any other traffics are blocked.
/// - `community`: The ports communicates with other community ports and with
///   promiscuous ports within the same PVLAN.
///   
///   any other traffics are
///   blocked.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VmwareDistributedVirtualSwitchPvlanPortTypeEnum {
    #[serde(rename = "promiscuous")]
    #[strum(serialize = "promiscuous")]
    Promiscuous,
    #[serde(rename = "isolated")]
    #[strum(serialize = "isolated")]
    Isolated,
    #[serde(rename = "community")]
    #[strum(serialize = "community")]
    Community,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The teaming health check match status.
/// 
/// Possible values:
/// - `iphashMatch`: The value of 'loadbalance\_ip' is used in a uplink teaming policy
///   *VmwareUplinkPortTeamingPolicy.policy*
///   in the vSphere Distributed Switch, and the external physical switch
///   has the matching EtherChannel configuration.
/// - `nonIphashMatch`: The value of 'loadbalance\_ip' is not used in a uplink teaming policy
///   *VmwareUplinkPortTeamingPolicy.policy*
///   in the vSphere Distributed Switch, and the external physical switch
///   does not have EtherChannel configuration.
/// - `iphashMismatch`: The value of 'loadbalance\_ip' is used in a uplink teaming policy
///   *VmwareUplinkPortTeamingPolicy.policy*
///   in the vSphere Distributed Switch, but the external physical switch
///   does not have the matching EtherChannel configuration.
/// - `nonIphashMismatch`: The value of 'loadbalance\_ip' is not used in a uplink teaming policy
///   *VmwareUplinkPortTeamingPolicy.policy*
///   in the vSphere Distributed Switch, but the external physical switch
///   has EtherChannel configuration.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VMwareDvsTeamingMatchStatusEnum {
    #[serde(rename = "iphashMatch")]
    #[strum(serialize = "iphashMatch")]
    IphashMatch,
    #[serde(rename = "nonIphashMatch")]
    #[strum(serialize = "nonIphashMatch")]
    NonIphashMatch,
    #[serde(rename = "iphashMismatch")]
    #[strum(serialize = "iphashMismatch")]
    IphashMismatch,
    #[serde(rename = "nonIphashMismatch")]
    #[strum(serialize = "nonIphashMismatch")]
    NonIphashMismatch,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Link Aggregation Control Protocol policy modes.
/// 
/// Possible values:
/// - `active`: Link Aggregation Control Protocol always sends frames along the configured uplinks
/// - `passive`: Link Aggregation Control Protocol acts as "speak when spoken to".
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VMwareUplinkLacpModeEnum {
    #[serde(rename = "active")]
    #[strum(serialize = "active")]
    Active,
    #[serde(rename = "passive")]
    #[strum(serialize = "passive")]
    Passive,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Link Aggregation Control Protocol timeout policy modes.
/// 
/// Possible values:
/// - `fast`: Set long timeout for vmnics in one LACP LAG.
///   
///   Device send fast LACPDUs
/// - `slow`: Set short timeout for vmnics in one LACP LAG.
///   
///   Device send slow LACPDUs
/// 
/// ***Since:*** vSphere API Release 7.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VMwareUplinkLacpTimeoutModeEnum {
    #[serde(rename = "fast")]
    #[strum(serialize = "fast")]
    Fast,
    #[serde(rename = "slow")]
    #[strum(serialize = "slow")]
    Slow,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Distributed Port Mirroring session Encapsulation types.
/// 
/// Possible values:
/// - `gre`: Encapsulate original packets with GRE protocol
/// - `erspan2`: Encapsulate original packets with ERSPAN Type2 protocol
/// - `erspan3`: Encapsulate original packets with ERSPAN Type3 protocol
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VMwareDvsVspanSessionEncapTypeEnum {
    #[serde(rename = "gre")]
    #[strum(serialize = "gre")]
    Gre,
    #[serde(rename = "erspan2")]
    #[strum(serialize = "erspan2")]
    Erspan2,
    #[serde(rename = "erspan3")]
    #[strum(serialize = "erspan3")]
    Erspan3,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Distributed Port Mirroring session types.
/// 
/// Possible values:
/// - `mixedDestMirror`: 
///   
///   Deprecated as of vSphere API 5.1.
///   
///   In mixedDestMirror session, Distributed Ports can be used as source entities,
///   and both Distributed Ports and Uplink Ports Name can be used as destination entities.
/// - `dvPortMirror`: In dvPortMirror session, Distributed Ports can be used as both source
///   and destination entities.
/// - `remoteMirrorSource`: In remoteMirrorSource session, Distributed Ports can be used as source entities,
///   and uplink ports name can be used as destination entities.
/// - `remoteMirrorDest`: In remoteMirrorDest session, vlan Ids can be used as source entities,
///   and Distributed Ports can be used as destination entities.
/// - `encapsulatedRemoteMirrorSource`: In encapsulatedRemoteMirrorSource session, Distributed Ports can be used as source entities,
///   and Ip address can be used as destination entities.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VMwareDvsVspanSessionTypeEnum {
    #[serde(rename = "mixedDestMirror")]
    #[strum(serialize = "mixedDestMirror")]
    MixedDestMirror,
    #[serde(rename = "dvPortMirror")]
    #[strum(serialize = "dvPortMirror")]
    DvPortMirror,
    #[serde(rename = "remoteMirrorSource")]
    #[strum(serialize = "remoteMirrorSource")]
    RemoteMirrorSource,
    #[serde(rename = "remoteMirrorDest")]
    #[strum(serialize = "remoteMirrorDest")]
    RemoteMirrorDest,
    #[serde(rename = "encapsulatedRemoteMirrorSource")]
    #[strum(serialize = "encapsulatedRemoteMirrorSource")]
    EncapsulatedRemoteMirrorSource,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Key management type.
/// 
/// Possible values:
/// - `unknown`
/// - `internal`
/// - `external`
/// 
/// ***Since:*** vSphere API Release 8.0.1.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum CryptoManagerHostKeyManagementTypeEnum {
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "internal")]
    #[strum(serialize = "internal")]
    Internal,
    #[serde(rename = "external")]
    #[strum(serialize = "external")]
    External,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `KeyStateMissingInCache`: Key not found in VC cache and does not specify a provider
/// - `KeyStateClusterInvalid`: Key provider is invalid
/// - `KeyStateClusterUnreachable`: Can not reach the key provider
/// - `KeyStateMissingInKMS`: Key not found in KMS
/// - `KeyStateNotActiveOrEnabled`: Key not active or enabled
/// - `KeyStateManagedByTrustAuthority`: Key is managed by Trust Authority
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum CryptoManagerKmipCryptoKeyStatusKeyUnavailableReasonEnum {
    KeyStateMissingInCache,
    KeyStateClusterInvalid,
    KeyStateClusterUnreachable,
    #[serde(rename = "KeyStateMissingInKMS")]
    #[strum(serialize = "KeyStateMissingInKMS")]
    KeyStateMissingInKms,
    KeyStateNotActiveOrEnabled,
    KeyStateManagedByTrustAuthority,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Key provider management type.
/// 
/// Possible values:
/// - `unknown`
/// - `vCenter`
/// - `trustAuthority`
/// - `nativeProvider`: ***Since:*** vSphere API Release 7.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum KmipClusterInfoKmsManagementTypeEnum {
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "vCenter")]
    #[strum(serialize = "vCenter")]
    VCenter,
    #[serde(rename = "trustAuthority")]
    #[strum(serialize = "trustAuthority")]
    TrustAuthority,
    #[serde(rename = "nativeProvider")]
    #[strum(serialize = "nativeProvider")]
    NativeProvider,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `userDefinedScriptDisabled`: The user defined script is disabled during customization
/// - `customizationDisabled`: The guest customization is disabled by VMware Tools
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `rawDataIsNotSupported`: The cloud-init version is too old to support cloud-init raw data
///   
///   ***Since:*** vSphere API Release 7.0.3.0
/// - `wrongMetadataFormat`: The cloud-init meta data is not valid format
///   
///   ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum CustomizationFailedReasonCodeEnum {
    #[serde(rename = "userDefinedScriptDisabled")]
    #[strum(serialize = "userDefinedScriptDisabled")]
    UserDefinedScriptDisabled,
    #[serde(rename = "customizationDisabled")]
    #[strum(serialize = "customizationDisabled")]
    CustomizationDisabled,
    #[serde(rename = "rawDataIsNotSupported")]
    #[strum(serialize = "rawDataIsNotSupported")]
    RawDataIsNotSupported,
    #[serde(rename = "wrongMetadataFormat")]
    #[strum(serialize = "wrongMetadataFormat")]
    WrongMetadataFormat,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The port blocked/unblocked state.
/// 
/// Possible values:
/// - `unset`: The dvs port is in unset state
/// - `blocked`: The dvs port is in blocked state
/// - `unblocked`: The dvs port is in unblocked state
/// - `unknown`: The dvs port is in unknown state
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DvsEventPortBlockStateEnum {
    #[serde(rename = "unset")]
    #[strum(serialize = "unset")]
    Unset,
    #[serde(rename = "blocked")]
    #[strum(serialize = "blocked")]
    Blocked,
    #[serde(rename = "unblocked")]
    #[strum(serialize = "unblocked")]
    Unblocked,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Severity level constants.
/// 
/// Possible values:
/// - `error`: Something that must be corrected
/// - `warning`: Should be corrected, but the system can continue operating normally
/// - `info`: An informational message
/// - `user`: A user-related message
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum EventEventSeverityEnum {
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    #[serde(rename = "warning")]
    #[strum(serialize = "warning")]
    Warning,
    #[serde(rename = "info")]
    #[strum(serialize = "info")]
    Info,
    #[serde(rename = "user")]
    #[strum(serialize = "user")]
    User,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `info`: Returns informational events.
/// - `warning`: Returns warning events.
/// - `error`: Returns error events.
/// - `user`: Returns events pertaining to users.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum EventCategoryEnum {
    #[serde(rename = "info")]
    #[strum(serialize = "info")]
    Info,
    #[serde(rename = "warning")]
    #[strum(serialize = "warning")]
    Warning,
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    #[serde(rename = "user")]
    #[strum(serialize = "user")]
    User,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This option specifies how to select events based on child relationships
/// in the inventory hierarchy.
/// 
/// If a managed entity has children, their events
/// can be retrieved with this filter option.
/// 
/// Possible values:
/// - `self`: Returns events that pertain only to the specified managed entity,
///   and not its children.
/// - `children`: Returns events pertaining to child entities only.
///   
///   Excludes
///   events pertaining to the specified managed entity itself.
/// - `all`: Returns events pertaining either to the specified managed entity
///   or to its child entities.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum EventFilterSpecRecursionOptionEnum {
    #[serde(rename = "self")]
    #[strum(serialize = "self")]
    Self_,
    #[serde(rename = "children")]
    #[strum(serialize = "children")]
    Children,
    #[serde(rename = "all")]
    #[strum(serialize = "all")]
    All,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `configFailed`: Error while configuring/unconfiguring HA
/// - `timeout`: Timeout while communicating with HA agent
/// - `communicationInitFailed`: HA communication initialization failed
/// - `healthCheckScriptFailed`: Health check script failed
/// - `agentFailed`: HA agent has an error
/// - `agentShutdown`: HA agent was shutdown
/// - `isolationAddressUnpingable`: HA isolation address unpingable
/// - `other`: Other reason
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostDasErrorEventHostDasErrorReasonEnum {
    #[serde(rename = "configFailed")]
    #[strum(serialize = "configFailed")]
    ConfigFailed,
    #[serde(rename = "timeout")]
    #[strum(serialize = "timeout")]
    Timeout,
    #[serde(rename = "communicationInitFailed")]
    #[strum(serialize = "communicationInitFailed")]
    CommunicationInitFailed,
    #[serde(rename = "healthCheckScriptFailed")]
    #[strum(serialize = "healthCheckScriptFailed")]
    HealthCheckScriptFailed,
    #[serde(rename = "agentFailed")]
    #[strum(serialize = "agentFailed")]
    AgentFailed,
    #[serde(rename = "agentShutdown")]
    #[strum(serialize = "agentShutdown")]
    AgentShutdown,
    #[serde(rename = "isolationAddressUnpingable")]
    #[strum(serialize = "isolationAddressUnpingable")]
    IsolationAddressUnpingable,
    #[serde(rename = "other")]
    #[strum(serialize = "other")]
    Other,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `sslThumbprintVerifyFailed`: Failed to verify SSL thumbprint
/// - `licenseExpired`: License expired for the host
/// - `agentUpgrade`: Agent is being upgraded
/// - `userRequest`: User requested disconnect
/// - `insufficientLicenses`: License not available after host upgrade
/// - `agentOutOfDate`: Agent is out of date
/// - `passwordDecryptFailure`: Failed to decrypt password
/// - `unknown`: Unknown reason
/// - `vcVRAMCapacityExceeded`: The vRAM capacity of vCenter will be exceeded
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostDisconnectedEventReasonCodeEnum {
    #[serde(rename = "sslThumbprintVerifyFailed")]
    #[strum(serialize = "sslThumbprintVerifyFailed")]
    SslThumbprintVerifyFailed,
    #[serde(rename = "licenseExpired")]
    #[strum(serialize = "licenseExpired")]
    LicenseExpired,
    #[serde(rename = "agentUpgrade")]
    #[strum(serialize = "agentUpgrade")]
    AgentUpgrade,
    #[serde(rename = "userRequest")]
    #[strum(serialize = "userRequest")]
    UserRequest,
    #[serde(rename = "insufficientLicenses")]
    #[strum(serialize = "insufficientLicenses")]
    InsufficientLicenses,
    #[serde(rename = "agentOutOfDate")]
    #[strum(serialize = "agentOutOfDate")]
    AgentOutOfDate,
    #[serde(rename = "passwordDecryptFailure")]
    #[strum(serialize = "passwordDecryptFailure")]
    PasswordDecryptFailure,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "vcVRAMCapacityExceeded")]
    #[strum(serialize = "vcVRAMCapacityExceeded")]
    VcVramCapacityExceeded,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `vmtoolsHeartbeatFailure`: vmtools heartbeat failure
/// - `appHeartbeatFailure`: application heartbeat failure
/// - `appImmediateResetRequest`: immediate reset request
/// - `vmcpResetApdCleared`: reset issued by VMCP when APD cleared
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VmDasBeingResetEventReasonCodeEnum {
    #[serde(rename = "vmtoolsHeartbeatFailure")]
    #[strum(serialize = "vmtoolsHeartbeatFailure")]
    VmtoolsHeartbeatFailure,
    #[serde(rename = "appHeartbeatFailure")]
    #[strum(serialize = "appHeartbeatFailure")]
    AppHeartbeatFailure,
    #[serde(rename = "appImmediateResetRequest")]
    #[strum(serialize = "appImmediateResetRequest")]
    AppImmediateResetRequest,
    #[serde(rename = "vmcpResetApdCleared")]
    #[strum(serialize = "vmcpResetApdCleared")]
    VmcpResetApdCleared,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The reason for the failure.
/// 
/// Possible values:
/// - `incompatibleHost`: Remote host is incompatible for secondary virtual machine.
///   
///   For instance, the host doesn't have access to the virtual machine's
///   network or datastore.
/// - `loginFailed`: Login to remote host failed.
/// - `registerVmFailed`: Registration of the secondary virtual machine
///   on the remote host failed.
/// - `migrateFailed`: Migration failed.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VmFailedStartingSecondaryEventFailureReasonEnum {
    #[serde(rename = "incompatibleHost")]
    #[strum(serialize = "incompatibleHost")]
    IncompatibleHost,
    #[serde(rename = "loginFailed")]
    #[strum(serialize = "loginFailed")]
    LoginFailed,
    #[serde(rename = "registerVmFailed")]
    #[strum(serialize = "registerVmFailed")]
    RegisterVmFailed,
    #[serde(rename = "migrateFailed")]
    #[strum(serialize = "migrateFailed")]
    MigrateFailed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `shutdown`: The virtual machine was shut down
/// - `poweredOff`: The virtual machine was powered off because shut down failed
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VmShutdownOnIsolationEventOperationEnum {
    #[serde(rename = "shutdown")]
    #[strum(serialize = "shutdown")]
    Shutdown,
    #[serde(rename = "poweredOff")]
    #[strum(serialize = "poweredOff")]
    PoweredOff,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Types of affinities.
/// 
/// Possible values:
/// - `memory`
/// - `cpu`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum AffinityTypeEnum {
    #[serde(rename = "memory")]
    #[strum(serialize = "memory")]
    Memory,
    #[serde(rename = "cpu")]
    #[strum(serialize = "cpu")]
    Cpu,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `NotEnoughSpaceOnDevice`: There is not enough storage space on the host to install the agent.
/// - `PrepareToUpgradeFailed`: Failed to initialize the upgrade directory on the host.
/// - `AgentNotRunning`: The agent was installed but is not running.
/// - `AgentNotReachable`: The agent was installed but did not respond to requests.
/// - `InstallTimedout`: The agent install took too long.
/// - `SignatureVerificationFailed`: The signature verification for the installer failed.
/// - `AgentUploadFailed`: Failed to upload the agent installer.
/// - `AgentUploadTimedout`: The agent upload took too long.
/// - `UnknownInstallerError`: The agent installer failed for an unknown reason.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum AgentInstallFailedReasonEnum {
    NotEnoughSpaceOnDevice,
    PrepareToUpgradeFailed,
    AgentNotRunning,
    AgentNotReachable,
    InstallTimedout,
    SignatureVerificationFailed,
    AgentUploadFailed,
    AgentUploadTimedout,
    UnknownInstallerError,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `APDTimeoutDisabled`: APD timeout has been disabled on one of the host
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum CannotEnableVmcpForClusterReasonEnum {
    #[serde(rename = "APDTimeoutDisabled")]
    #[strum(serialize = "APDTimeoutDisabled")]
    ApdTimeoutDisabled,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `resourcePool`: Move out of the resouce pool
/// - `cluster`: Move out of the cluster
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum CannotMoveFaultToleranceVmMoveTypeEnum {
    #[serde(rename = "resourcePool")]
    #[strum(serialize = "resourcePool")]
    ResourcePool,
    #[serde(rename = "cluster")]
    #[strum(serialize = "cluster")]
    Cluster,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `suspend`: suspend
/// - `powerOff`: power off
/// - `guestShutdown`: guest shutdown
/// - `guestSuspend`: guest suspend
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum CannotPowerOffVmInClusterOperationEnum {
    #[serde(rename = "suspend")]
    #[strum(serialize = "suspend")]
    Suspend,
    #[serde(rename = "powerOff")]
    #[strum(serialize = "powerOff")]
    PowerOff,
    #[serde(rename = "guestShutdown")]
    #[strum(serialize = "guestShutdown")]
    GuestShutdown,
    #[serde(rename = "guestSuspend")]
    #[strum(serialize = "guestSuspend")]
    GuestSuspend,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `NetworkReservationNotSupported`: Network does not support reservation
/// - `MismatchedNetworkPolicies`: Source and destination networks do not have same security policies
/// - `MismatchedDvsVersionOrVendor`: Source and destination DVS do not have same version or vendor
/// - `VMotionToUnsupportedNetworkType`: VMotion to unsupported destination network type
/// - `NetworkUnderMaintenance`: The network is under maintenance
/// - `MismatchedEnsMode`: Source and destination networks do not have same ENS(Enhanced Network Stack) mode
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum CannotUseNetworkReasonEnum {
    NetworkReservationNotSupported,
    MismatchedNetworkPolicies,
    MismatchedDvsVersionOrVendor,
    VMotionToUnsupportedNetworkType,
    NetworkUnderMaintenance,
    MismatchedEnsMode,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `HostNetworkMisconfiguration`: There is a problem with the host network configuration.
/// - `HostMisconfiguration`: There is a problem with the host configuration.
/// - `InsufficientPrivileges`: The privileges were insuffient for the operation.
/// - `NoPrimaryAgentAvailable`: There was no running primary agent available to contact.
///   
///   Check that your other hosts don't have HA errors
/// - `Other`: The HA configuration failed for other reasons.
/// - `NoDatastoresConfigured`: No datastores defined for this host
/// - `CreateConfigVvolFailed`: Failure to create config vvol
/// - `VSanNotSupportedOnHost`: Host in vSAN cluster does not support vSAN.
/// - `DasNetworkMisconfiguration`: There is a problem with the cluster network configuration.
/// - `SetDesiredImageSpecFailed`: Setting desired imageSpec in Personality Manager failed
/// - `ApplyHAVibsOnClusterFailed`: The ApplyHA call to Personality Manager failed
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DasConfigFaultDasConfigFaultReasonEnum {
    HostNetworkMisconfiguration,
    HostMisconfiguration,
    InsufficientPrivileges,
    NoPrimaryAgentAvailable,
    Other,
    NoDatastoresConfigured,
    CreateConfigVvolFailed,
    VSanNotSupportedOnHost,
    DasNetworkMisconfiguration,
    SetDesiredImageSpecFailed,
    #[serde(rename = "ApplyHAVibsOnClusterFailed")]
    #[strum(serialize = "ApplyHAVibsOnClusterFailed")]
    ApplyHaVibsOnClusterFailed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Reasons why a virtual device would not be supported on a host.
/// 
/// Possible values:
/// - `host`: The host does not support this virtual device at all.
/// - `guest`: The device is supported by the host in general, but not for
///   the specific guest OS the virtual machine is using.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DeviceNotSupportedReasonEnum {
    #[serde(rename = "host")]
    #[strum(serialize = "host")]
    Host,
    #[serde(rename = "guest")]
    #[strum(serialize = "guest")]
    Guest,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The disallowed change type.
/// 
/// Possible values:
/// - `hotExtendDisk`: Online extend disk operation.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DisallowedChangeByServiceDisallowedChangeEnum {
    #[serde(rename = "hotExtendDisk")]
    #[strum(serialize = "hotExtendDisk")]
    HotExtendDisk,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// HostSelectionType defines how the host was selected
/// 
/// Possible values:
/// - `user`: The host was specified by the user
/// - `vc`: The host was selected by Virtual Center
/// - `drs`: The host was selected by DRS
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum FtIssuesOnHostHostSelectionTypeEnum {
    #[serde(rename = "user")]
    #[strum(serialize = "user")]
    User,
    #[serde(rename = "vc")]
    #[strum(serialize = "vc")]
    Vc,
    #[serde(rename = "drs")]
    #[strum(serialize = "drs")]
    Drs,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `Datastore`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostHasComponentFailureHostComponentTypeEnum {
    Datastore,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Reasons why fault tolerance is not supported on the host.
/// 
/// Possible values:
/// - `product`: The product does not support fault tolerance.
/// - `processor`: The product supports fault tolerance but the host CPU does not.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostIncompatibleForFaultToleranceReasonEnum {
    #[serde(rename = "product")]
    #[strum(serialize = "product")]
    Product,
    #[serde(rename = "processor")]
    #[strum(serialize = "processor")]
    Processor,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Reasons why record/replay is not supported on a host.
/// 
/// Possible values:
/// - `product`: The product does not support record/replay.
/// - `processor`: The product supports record/replay but the host CPU does not.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostIncompatibleForRecordReplayReasonEnum {
    #[serde(rename = "product")]
    #[strum(serialize = "product")]
    Product,
    #[serde(rename = "processor")]
    #[strum(serialize = "processor")]
    Processor,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `rpo`: Host does not support the RPO configured for VM replication.
/// - `netCompression`: Host does not support network compression configured for VM
///   replication.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum IncompatibleHostForVmReplicationIncompatibleReasonEnum {
    #[serde(rename = "rpo")]
    #[strum(serialize = "rpo")]
    Rpo,
    #[serde(rename = "netCompression")]
    #[strum(serialize = "netCompression")]
    NetCompression,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `admissionControl`: Policies for admission control
/// - `userHeartbeatDs`: User-specified heartbeat datastores
/// - `vmConfig`: VM override
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum InvalidDasConfigArgumentEntryForInvalidArgumentEnum {
    #[serde(rename = "admissionControl")]
    #[strum(serialize = "admissionControl")]
    AdmissionControl,
    #[serde(rename = "userHeartbeatDs")]
    #[strum(serialize = "userHeartbeatDs")]
    UserHeartbeatDs,
    #[serde(rename = "vmConfig")]
    #[strum(serialize = "vmConfig")]
    VmConfig,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `incompatibleVersion`: The associated host and profile version are incompatible.
/// - `missingReferenceHost`: There is no reference host associated with the profile.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum InvalidProfileReferenceHostReasonEnum {
    #[serde(rename = "incompatibleVersion")]
    #[strum(serialize = "incompatibleVersion")]
    IncompatibleVersion,
    #[serde(rename = "missingReferenceHost")]
    #[strum(serialize = "missingReferenceHost")]
    MissingReferenceHost,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `keyEntityMismatch`: The license and the entity to which it is to be assigned are not compatible.
/// - `downgradeDisallowed`: The license downgrade is disallowed because some features are in use.
/// - `inventoryNotManageableByVirtualCenter`: The inventory has hosts which are not manageable by vCenter unless in evaluation.
/// - `hostsUnmanageableByVirtualCenterWithoutLicenseServer`: The inventory has hosts that need the license server to be configured unless vCenter is in evaluation
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum LicenseAssignmentFailedReasonEnum {
    #[serde(rename = "keyEntityMismatch")]
    #[strum(serialize = "keyEntityMismatch")]
    KeyEntityMismatch,
    #[serde(rename = "downgradeDisallowed")]
    #[strum(serialize = "downgradeDisallowed")]
    DowngradeDisallowed,
    #[serde(rename = "inventoryNotManageableByVirtualCenter")]
    #[strum(serialize = "inventoryNotManageableByVirtualCenter")]
    InventoryNotManageableByVirtualCenter,
    #[serde(rename = "hostsUnmanageableByVirtualCenterWithoutLicenseServer")]
    #[strum(serialize = "hostsUnmanageableByVirtualCenterWithoutLicenseServer")]
    HostsUnmanageableByVirtualCenterWithoutLicenseServer,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `virtualVmxnet3`: vmxnet3 virtual Ethernet adapter
/// - `paraVirtualSCSIController`: paravirtualized SCSI controller
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NotSupportedDeviceForFtDeviceTypeEnum {
    #[serde(rename = "virtualVmxnet3")]
    #[strum(serialize = "virtualVmxnet3")]
    VirtualVmxnet3,
    #[serde(rename = "paraVirtualSCSIController")]
    #[strum(serialize = "paraVirtualSCSIController")]
    ParaVirtualScsiController,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Reasons why the number of virtual CPUs is incompatible.
/// 
/// Possible values:
/// - `recordReplay`: 
///   
///   Deprecated as of vSphere API 6.0.
///   
///   The virtual machine needs to support record/replay functionality.
/// - `faultTolerance`: The virtual machine is enabled for fault tolerance.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NumVirtualCpusIncompatibleReasonEnum {
    #[serde(rename = "recordReplay")]
    #[strum(serialize = "recordReplay")]
    RecordReplay,
    #[serde(rename = "faultTolerance")]
    #[strum(serialize = "faultTolerance")]
    FaultTolerance,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `NoCompatibleNonQuarantinedHost`: The cluster does not contain any non-quarantined host satisfying the
///   VM/host affinity rules for the VM.
/// - `CorrectionDisallowed`: The current DRS migration priority setting disallows generating a
///   recommendation to prevent VMs on quarantined hosts.
///   
///   Thus, the
///   violation will not be corrected.
/// - `CorrectionImpact`: DRS has determined that evacuation of VMs from quarantined hosts
///   impacts respecting cluster constraints or performance goals so they
///   are not evacuated.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum QuarantineModeFaultFaultTypeEnum {
    NoCompatibleNonQuarantinedHost,
    CorrectionDisallowed,
    CorrectionImpact,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `diskNotFound`: Could not look up device by key
/// - `diskTypeNotSupported`: Replication not supported for disk type or backend
/// - `invalidDiskKey`: Invalid key value
/// - `invalidDiskReplicationId`: Invalid disk replication ID string
/// - `duplicateDiskReplicationId`: Another disk in the VM has the same replication ID
/// - `invalidPersistentFilePath`: Invalid path (string) for the persistent file
/// - `reconfigureDiskReplicationIdNotAllowed`: Attempting to re-configure the disk's replication ID
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ReplicationDiskConfigFaultReasonForFaultEnum {
    #[serde(rename = "diskNotFound")]
    #[strum(serialize = "diskNotFound")]
    DiskNotFound,
    #[serde(rename = "diskTypeNotSupported")]
    #[strum(serialize = "diskTypeNotSupported")]
    DiskTypeNotSupported,
    #[serde(rename = "invalidDiskKey")]
    #[strum(serialize = "invalidDiskKey")]
    InvalidDiskKey,
    #[serde(rename = "invalidDiskReplicationId")]
    #[strum(serialize = "invalidDiskReplicationId")]
    InvalidDiskReplicationId,
    #[serde(rename = "duplicateDiskReplicationId")]
    #[strum(serialize = "duplicateDiskReplicationId")]
    DuplicateDiskReplicationId,
    #[serde(rename = "invalidPersistentFilePath")]
    #[strum(serialize = "invalidPersistentFilePath")]
    InvalidPersistentFilePath,
    #[serde(rename = "reconfigureDiskReplicationIdNotAllowed")]
    #[strum(serialize = "reconfigureDiskReplicationIdNotAllowed")]
    ReconfigureDiskReplicationIdNotAllowed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `incompatibleHwVersion`: Incompatible VM hardware version
/// - `invalidVmReplicationId`: Invalid VM Replication ID string
/// - `invalidGenerationNumber`: Invalid generation number in VM's configuration
/// - `outOfBoundsRpoValue`: Invalid RPO value (out of bounds)
/// - `invalidDestinationIpAddress`: Invalid destination IP address
/// - `invalidDestinationPort`: Invalid destination port
/// - `invalidExtraVmOptions`: Malformed extra options list
/// - `staleGenerationNumber`: Mis-matching generation number (stale)
/// - `reconfigureVmReplicationIdNotAllowed`: Attempting to re-configure the VM replication ID
/// - `cannotRetrieveVmReplicationConfiguration`: Could not retrieve the VM configuration
/// - `replicationAlreadyEnabled`: Attempting to re-enable replication for the VM
/// - `invalidPriorConfiguration`: The existing replication configuration of the VM is broken
///   (applicable to re-configuration only).
/// - `replicationNotEnabled`: Attempting to re-configure or disable replication for a VM
///   for which replication has not been enabled.
/// - `replicationConfigurationFailed`: Failed to commit the new replication properties for the VM.
/// - `encryptedVm`: VM is encrypted
/// - `invalidThumbprint`: Remote certificate thumbprint is invalid
/// - `incompatibleDevice`: VM hardware contains devices incompatible with replication
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ReplicationVmConfigFaultReasonForFaultEnum {
    #[serde(rename = "incompatibleHwVersion")]
    #[strum(serialize = "incompatibleHwVersion")]
    IncompatibleHwVersion,
    #[serde(rename = "invalidVmReplicationId")]
    #[strum(serialize = "invalidVmReplicationId")]
    InvalidVmReplicationId,
    #[serde(rename = "invalidGenerationNumber")]
    #[strum(serialize = "invalidGenerationNumber")]
    InvalidGenerationNumber,
    #[serde(rename = "outOfBoundsRpoValue")]
    #[strum(serialize = "outOfBoundsRpoValue")]
    OutOfBoundsRpoValue,
    #[serde(rename = "invalidDestinationIpAddress")]
    #[strum(serialize = "invalidDestinationIpAddress")]
    InvalidDestinationIpAddress,
    #[serde(rename = "invalidDestinationPort")]
    #[strum(serialize = "invalidDestinationPort")]
    InvalidDestinationPort,
    #[serde(rename = "invalidExtraVmOptions")]
    #[strum(serialize = "invalidExtraVmOptions")]
    InvalidExtraVmOptions,
    #[serde(rename = "staleGenerationNumber")]
    #[strum(serialize = "staleGenerationNumber")]
    StaleGenerationNumber,
    #[serde(rename = "reconfigureVmReplicationIdNotAllowed")]
    #[strum(serialize = "reconfigureVmReplicationIdNotAllowed")]
    ReconfigureVmReplicationIdNotAllowed,
    #[serde(rename = "cannotRetrieveVmReplicationConfiguration")]
    #[strum(serialize = "cannotRetrieveVmReplicationConfiguration")]
    CannotRetrieveVmReplicationConfiguration,
    #[serde(rename = "replicationAlreadyEnabled")]
    #[strum(serialize = "replicationAlreadyEnabled")]
    ReplicationAlreadyEnabled,
    #[serde(rename = "invalidPriorConfiguration")]
    #[strum(serialize = "invalidPriorConfiguration")]
    InvalidPriorConfiguration,
    #[serde(rename = "replicationNotEnabled")]
    #[strum(serialize = "replicationNotEnabled")]
    ReplicationNotEnabled,
    #[serde(rename = "replicationConfigurationFailed")]
    #[strum(serialize = "replicationConfigurationFailed")]
    ReplicationConfigurationFailed,
    #[serde(rename = "encryptedVm")]
    #[strum(serialize = "encryptedVm")]
    EncryptedVm,
    #[serde(rename = "invalidThumbprint")]
    #[strum(serialize = "invalidThumbprint")]
    InvalidThumbprint,
    #[serde(rename = "incompatibleDevice")]
    #[strum(serialize = "incompatibleDevice")]
    IncompatibleDevice,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `notConfigured`: *VirtualMachine* is not configured for replication
/// - `poweredOff`: *VirtualMachine* is powered off (and is not undergoing
///   offline replication)
/// - `suspended`: *VirtualMachine* is suspended (and is not undergoing
///   offline replication)
/// - `poweredOn`: *VirtualMachine* is powered on
/// - `offlineReplicating`: *VirtualMachine* is in the process of creating an
///   an offline instance.
/// - `invalidState`: *VirtualMachine* is in an invalid state
/// - `invalidInstanceId`: The specified instanceId does not match the *VirtualMachine*
///   instanceId
/// - `closeDiskError`: *VirtualMachine* is in the process of creating an
///   offline instance and we are trying to disable it.
///   
///   The first step is to close the offline disk. If closing disks
///   is not successful, throw this fault.
/// - `groupExist`: *VirtualMachine* is trying to create a group already
///   owned by another VM.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ReplicationVmFaultReasonForFaultEnum {
    #[serde(rename = "notConfigured")]
    #[strum(serialize = "notConfigured")]
    NotConfigured,
    #[serde(rename = "poweredOff")]
    #[strum(serialize = "poweredOff")]
    PoweredOff,
    #[serde(rename = "suspended")]
    #[strum(serialize = "suspended")]
    Suspended,
    #[serde(rename = "poweredOn")]
    #[strum(serialize = "poweredOn")]
    PoweredOn,
    #[serde(rename = "offlineReplicating")]
    #[strum(serialize = "offlineReplicating")]
    OfflineReplicating,
    #[serde(rename = "invalidState")]
    #[strum(serialize = "invalidState")]
    InvalidState,
    #[serde(rename = "invalidInstanceId")]
    #[strum(serialize = "invalidInstanceId")]
    InvalidInstanceId,
    #[serde(rename = "closeDiskError")]
    #[strum(serialize = "closeDiskError")]
    CloseDiskError,
    #[serde(rename = "groupExist")]
    #[strum(serialize = "groupExist")]
    GroupExist,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `fullSync`: Initial synchronization with the remote site
/// - `delta`: Delta updates to generate a consistent instance
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ReplicationVmInProgressFaultActivityEnum {
    #[serde(rename = "fullSync")]
    #[strum(serialize = "fullSync")]
    FullSync,
    #[serde(rename = "delta")]
    #[strum(serialize = "delta")]
    Delta,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `licenseAssignmentFailed`: A general failure has occurred during assigning license to the 3rd party module
/// - `moduleNotInstalled`: The 3rd party module we are trying to license is not installed.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ThirdPartyLicenseAssignmentFailedReasonEnum {
    #[serde(rename = "licenseAssignmentFailed")]
    #[strum(serialize = "licenseAssignmentFailed")]
    LicenseAssignmentFailed,
    #[serde(rename = "moduleNotInstalled")]
    #[strum(serialize = "moduleNotInstalled")]
    ModuleNotInstalled,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `CacheModeNotSupported`
/// - `CacheConsistencyTypeNotSupported`
/// - `CacheBlockSizeNotSupported`
/// - `CacheReservationNotSupported`
/// - `DiskSizeNotSupported`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VFlashModuleNotSupportedReasonEnum {
    CacheModeNotSupported,
    CacheConsistencyTypeNotSupported,
    CacheBlockSizeNotSupported,
    CacheReservationNotSupported,
    DiskSizeNotSupported,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `haNotEnabled`: HA is not enabled on the cluster
/// - `moreThanOneSecondary`: There is already a secondary virtual machine for the primary
///   virtual machine
/// - `recordReplayNotSupported`: 
///   
///   Deprecated as of vSphere API 6.0.
///   
///   The virtual machine does not support record/replay.
///   
///   Vm::Capability.RecordReplaySupported is false.
/// - `replayNotSupported`: 
///   
///   Deprecated as of vSphere API 6.0.
///   
///   It is not possible to turn on Fault Tolerance on this powered-on VM.
///   
///   The support for record/replay should be enabled or Fault Tolerance
///   turned on, when this VM is powered off.
/// - `templateVm`: The virtual machine is a template
/// - `multipleVCPU`: The virtual machine has more than one virtual CPU
/// - `hostInactive`: The host is not active
/// - `ftUnsupportedHardware`: The host ftSupported flag is not set because of hardware issues
/// - `ftUnsupportedProduct`: The host ftSupported flag is not set because of it is a
///   VMware Server 2.0
/// - `missingVMotionNic`: No VMotion license or VMotion nic is not configured on the host
/// - `missingFTLoggingNic`: FT logging nic is not configured on the host
/// - `thinDisk`: The virtual machine has thin provisioned disks
/// - `verifySSLCertificateFlagNotSet`: The "check host certificate" flag is not set
/// - `hasSnapshots`: The virtual machine has one or more snapshots
/// - `noConfig`: No configuration information is available for the virtual machine
/// - `ftSecondaryVm`: The virtual machine is a fault tolerance secondary virtual machine
/// - `hasLocalDisk`: The virtual machine has one or more disks on local datastore
/// - `esxAgentVm`: The virtual machine is an ESX agent VM
/// - `video3dEnabled`: The virtual machine video device has 3D enabled
/// - `hasUnsupportedDisk`: The virtual machine has a virtual disk with unsupported backing type
/// - `insufficientBandwidth`: FT logging nic does not have desired bandwidth
/// - `hasNestedHVConfiguration`: The host does not support fault tolerant VM with nested HV or VBS
///   enabled.
/// - `hasVFlashConfiguration`: The virtual machine has a vFlash memory device or/and disks with
///   vFlash cache configured.
/// - `unsupportedProduct`: VMware product installed on the host does not support
///   fault tolerance
/// - `cpuHvUnsupported`: Host CPU does not support hardware virtualization
/// - `cpuHwmmuUnsupported`: Host CPU does not support hardware MMU virtualization
/// - `cpuHvDisabled`: Host CPU is compatible for replay-based FT, but hardware
///   virtualization has been disabled in the BIOS.
/// - `hasEFIFirmware`: The virtual machine firmware is of type EFI
/// - `tooManyVCPUs`: The host does not support fault tolerance virtual machines
///   with the specified number of virtual CPUs.
/// - `tooMuchMemory`: The host does not support fault tolerance virtual machines
///   with the specified amount of memory.
/// - `unsupportedPMemHAFailOver`: Virtual Machine with Pmem HA Failover is not supported
///   
///   ***Since:*** vSphere API Release 7.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VmFaultToleranceConfigIssueReasonForIssueEnum {
    #[serde(rename = "haNotEnabled")]
    #[strum(serialize = "haNotEnabled")]
    HaNotEnabled,
    #[serde(rename = "moreThanOneSecondary")]
    #[strum(serialize = "moreThanOneSecondary")]
    MoreThanOneSecondary,
    #[serde(rename = "recordReplayNotSupported")]
    #[strum(serialize = "recordReplayNotSupported")]
    RecordReplayNotSupported,
    #[serde(rename = "replayNotSupported")]
    #[strum(serialize = "replayNotSupported")]
    ReplayNotSupported,
    #[serde(rename = "templateVm")]
    #[strum(serialize = "templateVm")]
    TemplateVm,
    #[serde(rename = "multipleVCPU")]
    #[strum(serialize = "multipleVCPU")]
    MultipleVcpu,
    #[serde(rename = "hostInactive")]
    #[strum(serialize = "hostInactive")]
    HostInactive,
    #[serde(rename = "ftUnsupportedHardware")]
    #[strum(serialize = "ftUnsupportedHardware")]
    FtUnsupportedHardware,
    #[serde(rename = "ftUnsupportedProduct")]
    #[strum(serialize = "ftUnsupportedProduct")]
    FtUnsupportedProduct,
    #[serde(rename = "missingVMotionNic")]
    #[strum(serialize = "missingVMotionNic")]
    MissingVMotionNic,
    #[serde(rename = "missingFTLoggingNic")]
    #[strum(serialize = "missingFTLoggingNic")]
    MissingFtLoggingNic,
    #[serde(rename = "thinDisk")]
    #[strum(serialize = "thinDisk")]
    ThinDisk,
    #[serde(rename = "verifySSLCertificateFlagNotSet")]
    #[strum(serialize = "verifySSLCertificateFlagNotSet")]
    VerifySslCertificateFlagNotSet,
    #[serde(rename = "hasSnapshots")]
    #[strum(serialize = "hasSnapshots")]
    HasSnapshots,
    #[serde(rename = "noConfig")]
    #[strum(serialize = "noConfig")]
    NoConfig,
    #[serde(rename = "ftSecondaryVm")]
    #[strum(serialize = "ftSecondaryVm")]
    FtSecondaryVm,
    #[serde(rename = "hasLocalDisk")]
    #[strum(serialize = "hasLocalDisk")]
    HasLocalDisk,
    #[serde(rename = "esxAgentVm")]
    #[strum(serialize = "esxAgentVm")]
    EsxAgentVm,
    #[serde(rename = "video3dEnabled")]
    #[strum(serialize = "video3dEnabled")]
    Video3DEnabled,
    #[serde(rename = "hasUnsupportedDisk")]
    #[strum(serialize = "hasUnsupportedDisk")]
    HasUnsupportedDisk,
    #[serde(rename = "insufficientBandwidth")]
    #[strum(serialize = "insufficientBandwidth")]
    InsufficientBandwidth,
    #[serde(rename = "hasNestedHVConfiguration")]
    #[strum(serialize = "hasNestedHVConfiguration")]
    HasNestedHvConfiguration,
    #[serde(rename = "hasVFlashConfiguration")]
    #[strum(serialize = "hasVFlashConfiguration")]
    HasVFlashConfiguration,
    #[serde(rename = "unsupportedProduct")]
    #[strum(serialize = "unsupportedProduct")]
    UnsupportedProduct,
    #[serde(rename = "cpuHvUnsupported")]
    #[strum(serialize = "cpuHvUnsupported")]
    CpuHvUnsupported,
    #[serde(rename = "cpuHwmmuUnsupported")]
    #[strum(serialize = "cpuHwmmuUnsupported")]
    CpuHwmmuUnsupported,
    #[serde(rename = "cpuHvDisabled")]
    #[strum(serialize = "cpuHvDisabled")]
    CpuHvDisabled,
    #[serde(rename = "hasEFIFirmware")]
    #[strum(serialize = "hasEFIFirmware")]
    HasEfiFirmware,
    #[serde(rename = "tooManyVCPUs")]
    #[strum(serialize = "tooManyVCPUs")]
    TooManyVcpUs,
    #[serde(rename = "tooMuchMemory")]
    #[strum(serialize = "tooMuchMemory")]
    TooMuchMemory,
    #[serde(rename = "unsupportedPMemHAFailOver")]
    #[strum(serialize = "unsupportedPMemHAFailOver")]
    UnsupportedPMemHaFailOver,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `virtualFloppy`: virtual floppy
/// - `virtualCdrom`: virtual Cdrom
/// - `virtualSerialPort`: virtual serial port
/// - `virtualParallelPort`: virtual parallel port
/// - `virtualDisk`: virtual disk
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VmFaultToleranceInvalidFileBackingDeviceTypeEnum {
    #[serde(rename = "virtualFloppy")]
    #[strum(serialize = "virtualFloppy")]
    VirtualFloppy,
    #[serde(rename = "virtualCdrom")]
    #[strum(serialize = "virtualCdrom")]
    VirtualCdrom,
    #[serde(rename = "virtualSerialPort")]
    #[strum(serialize = "virtualSerialPort")]
    VirtualSerialPort,
    #[serde(rename = "virtualParallelPort")]
    #[strum(serialize = "virtualParallelPort")]
    VirtualParallelPort,
    #[serde(rename = "virtualDisk")]
    #[strum(serialize = "virtualDisk")]
    VirtualDisk,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `svmotion`: storage vmotion resolution
/// - `relocate`: relocate resolution
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum WillLoseHaProtectionResolutionEnum {
    #[serde(rename = "svmotion")]
    #[strum(serialize = "svmotion")]
    Svmotion,
    #[serde(rename = "relocate")]
    #[strum(serialize = "relocate")]
    Relocate,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `SHA1`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostActiveDirectoryAuthenticationCertificateDigestEnum {
    #[serde(rename = "SHA1")]
    #[strum(serialize = "SHA1")]
    Sha1,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `unknown`: The Active Directory integration provider does not support
///   domain trust checks.
/// - `ok`: No problems with the domain membership.
/// - `noServers`: The host thinks it's part of a domain,
///   but no domain controllers could be reached to confirm.
/// - `clientTrustBroken`: The client side of the trust relationship is broken.
/// - `serverTrustBroken`: The server side of the trust relationship is broken
///   (or bad machine password).
/// - `inconsistentTrust`: Unexpected domain controller responded.
/// - `otherProblem`: There's some problem with the domain membership.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostActiveDirectoryInfoDomainMembershipStatusEnum {
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "ok")]
    #[strum(serialize = "ok")]
    Ok,
    #[serde(rename = "noServers")]
    #[strum(serialize = "noServers")]
    NoServers,
    #[serde(rename = "clientTrustBroken")]
    #[strum(serialize = "clientTrustBroken")]
    ClientTrustBroken,
    #[serde(rename = "serverTrustBroken")]
    #[strum(serialize = "serverTrustBroken")]
    ServerTrustBroken,
    #[serde(rename = "inconsistentTrust")]
    #[strum(serialize = "inconsistentTrust")]
    InconsistentTrust,
    #[serde(rename = "otherProblem")]
    #[strum(serialize = "otherProblem")]
    OtherProblem,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `none`: No action is taken for this virtual machine.
///   
///   This virtual machine is
///   not a part of the auto-start sequence. This can be used for both auto-start
///   and auto-start settings.
/// - `systemDefault`: The default system action is taken for this virtual machine when it is next in
///   the auto-start order.
///   
///   This can be used for both auto-start and auto-start
///   settings.
/// - `powerOn`: This virtual machine is powered on when it is next in the auto-start order.
/// - `powerOff`: This virtual machine is powered off when it is next in the auto-stop order.
///   
///   This is the default stopAction.
/// - `guestShutdown`: The guest operating system for a virtual machine is shut down when that
///   virtual machine in next in the auto-stop order.
/// - `suspend`: This virtual machine is suspended when it is next in the auto-stop order.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum AutoStartActionEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "systemDefault")]
    #[strum(serialize = "systemDefault")]
    SystemDefault,
    #[serde(rename = "powerOn")]
    #[strum(serialize = "powerOn")]
    PowerOn,
    #[serde(rename = "powerOff")]
    #[strum(serialize = "powerOff")]
    PowerOff,
    #[serde(rename = "guestShutdown")]
    #[strum(serialize = "guestShutdown")]
    GuestShutdown,
    #[serde(rename = "suspend")]
    #[strum(serialize = "suspend")]
    Suspend,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Determines if the virtual machine should start after receiving a heartbeat,
/// ignore heartbeats and start after the startDelay has elapsed, or follow the
/// system default before powering on.
/// 
/// When a virtual machine is next in the start
/// order, the system either waits a specified period of time for a virtual
/// machine to power on or it waits until it receives a successful heartbeat from a
/// powered on virtual machine. By default, this is set to no.
/// 
/// Possible values:
/// - `yes`: The system waits until receiving a heartbeat before powering on the next
///   machine in the order.
/// - `no`: The system does not wait to receive a heartbeat before powering on the next
///   machine in the order.
///   
///   This is the default setting.
/// - `systemDefault`: The system uses the default value to determine whether or not to wait to
///   receive a heartbeat before powering on the next machine in the order.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum AutoStartWaitHeartbeatSettingEnum {
    #[serde(rename = "yes")]
    #[strum(serialize = "yes")]
    Yes,
    #[serde(rename = "no")]
    #[strum(serialize = "no")]
    No,
    #[serde(rename = "systemDefault")]
    #[strum(serialize = "systemDefault")]
    SystemDefault,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enumeration of the supported firmware types.
/// 
/// Possible values:
/// - `BIOS`
/// - `UEFI`
/// 
/// ***Since:*** vSphere API Release 8.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostBiosInfoFirmwareTypeEnum {
    #[serde(rename = "BIOS")]
    #[strum(serialize = "BIOS")]
    Bios,
    #[serde(rename = "UEFI")]
    #[strum(serialize = "UEFI")]
    Uefi,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated as of vSphere API 7.0, use
/// *VmFaultToleranceConfigIssueReasonForIssue_enum*.
/// 
/// Set of possible values for
/// *HostCapability.ftCompatibilityIssues*
/// 
/// Possible values:
/// - `vMotionNotLicensed`: No VMotion license
/// - `missingVMotionNic`: VMotion nic is not configured on the host
/// - `missingFTLoggingNic`: FT logging nic is not configured on the host
/// - `ftNotLicensed`: Host does not have proper FT license
/// - `haAgentIssue`: Host does not have HA agent running properly
/// - `unsupportedProduct`: VMware product installed on the host does not support
///   fault tolerance
/// - `cpuHvUnsupported`: Host CPU does not support hardware virtualization
/// - `cpuHwmmuUnsupported`: Host CPU does not support hardware MMU virtualization
/// - `cpuHvDisabled`: Host CPU is compatible for replay-based FT, but hardware
///   virtualization has been disabled in the BIOS.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostCapabilityFtUnsupportedReasonEnum {
    #[serde(rename = "vMotionNotLicensed")]
    #[strum(serialize = "vMotionNotLicensed")]
    VMotionNotLicensed,
    #[serde(rename = "missingVMotionNic")]
    #[strum(serialize = "missingVMotionNic")]
    MissingVMotionNic,
    #[serde(rename = "missingFTLoggingNic")]
    #[strum(serialize = "missingFTLoggingNic")]
    MissingFtLoggingNic,
    #[serde(rename = "ftNotLicensed")]
    #[strum(serialize = "ftNotLicensed")]
    FtNotLicensed,
    #[serde(rename = "haAgentIssue")]
    #[strum(serialize = "haAgentIssue")]
    HaAgentIssue,
    #[serde(rename = "unsupportedProduct")]
    #[strum(serialize = "unsupportedProduct")]
    UnsupportedProduct,
    #[serde(rename = "cpuHvUnsupported")]
    #[strum(serialize = "cpuHvUnsupported")]
    CpuHvUnsupported,
    #[serde(rename = "cpuHwmmuUnsupported")]
    #[strum(serialize = "cpuHwmmuUnsupported")]
    CpuHwmmuUnsupported,
    #[serde(rename = "cpuHvDisabled")]
    #[strum(serialize = "cpuHvDisabled")]
    CpuHvDisabled,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated as of vSphere API 6.0.
/// 
/// Set of possible values for
/// *HostCapability.replayUnsupportedReason* and
/// *HostCapability.replayCompatibilityIssues*.
/// 
/// Possible values:
/// - `incompatibleProduct`
/// - `incompatibleCpu`
/// - `hvDisabled`
/// - `cpuidLimitSet`
/// - `oldBIOS`
/// - `unknown`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostReplayUnsupportedReasonEnum {
    #[serde(rename = "incompatibleProduct")]
    #[strum(serialize = "incompatibleProduct")]
    IncompatibleProduct,
    #[serde(rename = "incompatibleCpu")]
    #[strum(serialize = "incompatibleCpu")]
    IncompatibleCpu,
    #[serde(rename = "hvDisabled")]
    #[strum(serialize = "hvDisabled")]
    HvDisabled,
    #[serde(rename = "cpuidLimitSet")]
    #[strum(serialize = "cpuidLimitSet")]
    CpuidLimitSet,
    #[serde(rename = "oldBIOS")]
    #[strum(serialize = "oldBIOS")]
    OldBios,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of VMFS unmap API version.
/// 
/// Possible values:
/// - `priority`: only the unmap priority is supported
/// - `fixed`: the unmap bandwidth can be set as a fixed value
/// - `dynamic`: the unmap bandwidth can be set as a range, where the actual
///   bandwidth will be dynamically throttled by the backened
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostCapabilityUnmapMethodSupportedEnum {
    #[serde(rename = "priority")]
    #[strum(serialize = "priority")]
    Priority,
    #[serde(rename = "fixed")]
    #[strum(serialize = "fixed")]
    Fixed,
    #[serde(rename = "dynamic")]
    #[strum(serialize = "dynamic")]
    Dynamic,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for *HostCapability.vmDirectPathGen2UnsupportedReason*.
/// 
/// Possible values:
/// - `hostNptIncompatibleProduct`: The host software does not support VMDirectPath Gen 2.
/// - `hostNptIncompatibleHardware`: The host hardware does not support VMDirectPath Gen 2.
///   
///   Note that
///   this is a general capability for the host and is independent of
///   support by a given physical NIC.
/// - `hostNptDisabled`: The host is configured to disable VMDirectPath Gen 2.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostCapabilityVmDirectPathGen2UnsupportedReasonEnum {
    #[serde(rename = "hostNptIncompatibleProduct")]
    #[strum(serialize = "hostNptIncompatibleProduct")]
    HostNptIncompatibleProduct,
    #[serde(rename = "hostNptIncompatibleHardware")]
    #[strum(serialize = "hostNptIncompatibleHardware")]
    HostNptIncompatibleHardware,
    #[serde(rename = "hostNptDisabled")]
    #[strum(serialize = "hostNptDisabled")]
    HostNptDisabled,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The status of a given certificate as computed per the soft and the hard
/// thresholds in vCenter Server.
/// 
///   
///   
/// There are two different thresholds for the host certificate
/// expirations; a soft threshold (which constitutes of two phases) and a
/// hard threshold.
///   
///   
/// Soft Threshold:
///   
/// Phase One: vCenter Server will publish an event at
/// this time to let the user know about the status, but, no alarms or
/// warnings are raised.
///   
/// Phase Two: During this phase, vCenter Server will publish an event and
/// indicate the certificate status as expiring in the UI.
///   
///   
/// Hard Threshold:
///   
/// vCenter Server will publish an alarm and indicate via the UI that the
/// certificate expiration is imminent.
/// 
/// Possible values:
/// - `unknown`: The certificate status is unknown.
/// - `expired`: The certificate has expired.
/// - `expiring`: The certificate is expiring shortly.
///   
///   (soft threshold - 1)
/// - `expiringShortly`: The certificate is expiring shortly.
///   
///   (soft threshold - 2)
/// - `expirationImminent`: The certificate expiration is imminent.
///   
///   (hard threshold)
/// - `good`: The certificate is good.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostCertificateManagerCertificateInfoCertificateStatusEnum {
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "expired")]
    #[strum(serialize = "expired")]
    Expired,
    #[serde(rename = "expiring")]
    #[strum(serialize = "expiring")]
    Expiring,
    #[serde(rename = "expiringShortly")]
    #[strum(serialize = "expiringShortly")]
    ExpiringShortly,
    #[serde(rename = "expirationImminent")]
    #[strum(serialize = "expirationImminent")]
    ExpirationImminent,
    #[serde(rename = "good")]
    #[strum(serialize = "good")]
    Good,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Certificate type supported by Host
/// 
/// Possible values:
/// - `Machine`: Machine certificate of the Host
/// - `VASAClient`: VASA Client certificate used for communication with VASA Provider
///   
/// ***Since:*** vSphere API Release 8.0.1.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostCertificateManagerCertificateKindEnum {
    Machine,
    #[serde(rename = "VASAClient")]
    #[strum(serialize = "VASAClient")]
    VasaClient,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This is a global mode on a configuration specification indicating
/// whether the structure represents the desired state or the set of
/// operations to apply on the managed object.
/// 
/// Possible values:
/// - `modify`: Indicates that the structure represents the
///   set of operations to apply on the managed object.
/// - `replace`: Indicates that the structure represents the
///   desired state of the managed object.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostConfigChangeModeEnum {
    #[serde(rename = "modify")]
    #[strum(serialize = "modify")]
    Modify,
    #[serde(rename = "replace")]
    #[strum(serialize = "replace")]
    Replace,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This list indicates the operation that should be performed for an
/// entity.
/// 
/// Possible values:
/// - `add`: Indicates the addition of an entity to the configuration.
/// - `remove`: Indicates the removal of an entity from the configuration.
/// - `edit`: Indicates changes on the entity.
///   
///   The entity must exist or a
///   *NotFound* error will be thrown.
/// - `ignore`: Indicates that an entity will be ignored: it won't be added when it
///   doesn't exist, or removed/changed when it exists.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostConfigChangeOperationEnum {
    #[serde(rename = "add")]
    #[strum(serialize = "add")]
    Add,
    #[serde(rename = "remove")]
    #[strum(serialize = "remove")]
    Remove,
    #[serde(rename = "edit")]
    #[strum(serialize = "edit")]
    Edit,
    #[serde(rename = "ignore")]
    #[strum(serialize = "ignore")]
    Ignore,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `unknown`
/// - `intel`
/// - `amd`
/// - `hygon`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostCpuPackageVendorEnum {
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "intel")]
    #[strum(serialize = "intel")]
    Intel,
    #[serde(rename = "amd")]
    #[strum(serialize = "amd")]
    Amd,
    #[serde(rename = "hygon")]
    #[strum(serialize = "hygon")]
    Hygon,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values for Current CPU power management policy
/// 
/// Possible values:
/// - `off`
/// - `staticPolicy`
/// - `dynamicPolicy`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostCpuPowerManagementInfoPolicyTypeEnum {
    #[serde(rename = "off")]
    #[strum(serialize = "off")]
    Off,
    #[serde(rename = "staticPolicy")]
    #[strum(serialize = "staticPolicy")]
    StaticPolicy,
    #[serde(rename = "dynamicPolicy")]
    #[strum(serialize = "dynamicPolicy")]
    DynamicPolicy,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Types of time synchronization protocols.
/// 
/// Possible values:
/// - `ntp`: Network Time Protocol (NTP).
/// - `ptp`: Precision Time Protocol (PTP).
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostDateTimeInfoProtocolEnum {
    #[serde(rename = "ntp")]
    #[strum(serialize = "ntp")]
    Ntp,
    #[serde(rename = "ptp")]
    #[strum(serialize = "ptp")]
    Ptp,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The type of diagnostic partition.
/// 
/// Private diagnostic partition has one
/// slot, so can only be used by one host. Shared diagnostic parititon
/// needs multiple slots so to be usable by multiple hosts.
/// 
/// Possible values:
/// - `singleHost`
/// - `multiHost`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DiagnosticPartitionTypeEnum {
    #[serde(rename = "singleHost")]
    #[strum(serialize = "singleHost")]
    SingleHost,
    #[serde(rename = "multiHost")]
    #[strum(serialize = "multiHost")]
    MultiHost,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Type of partition indicating the type of storage on which the partition
/// resides.
/// 
/// If the diagnostic partition is local only, it will only need
/// one slot. If the diagnostic partition is on shared storage, it could
/// be used by multiple hosts. As a result, it will need multiple slots.
/// 
/// Possible values:
/// - `directAttached`
/// - `networkAttached`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DiagnosticPartitionStorageTypeEnum {
    #[serde(rename = "directAttached")]
    #[strum(serialize = "directAttached")]
    DirectAttached,
    #[serde(rename = "networkAttached")]
    #[strum(serialize = "networkAttached")]
    NetworkAttached,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The set of digest methods that can be used by TPM to calculate the PCR
/// values.
/// 
/// Possible values:
/// - `SHA1`
/// - `MD5`: 
///   
///   Deprecated as of vSphere API 6.7.
///   
///   MD5.
/// - `SHA256`
/// - `SHA384`
/// - `SHA512`
/// - `SM3_256`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostDigestInfoDigestMethodTypeEnum {
    #[serde(rename = "SHA1")]
    #[strum(serialize = "SHA1")]
    Sha1,
    #[serde(rename = "MD5")]
    #[strum(serialize = "MD5")]
    Md5,
    #[serde(rename = "SHA256")]
    #[strum(serialize = "SHA256")]
    Sha256,
    #[serde(rename = "SHA384")]
    #[strum(serialize = "SHA384")]
    Sha384,
    #[serde(rename = "SHA512")]
    #[strum(serialize = "SHA512")]
    Sha512,
    #[serde(rename = "SM3_256")]
    #[strum(serialize = "SM3_256")]
    Sm3256,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This enum specifies the supported digest verification settings.
/// 
/// For NVMe over TCP connections, both header and data digests may be
/// requested during the process of establishing the connection.
/// For details, see:
/// - NVM Express Technical Proposal 8000 - NVMe/TCP Transport,
///   Section 7.4.6, "PDU Header and Data Digests"
///   
/// Possible values:
/// - `digestDisabled`: Both header and data digest verification are disabled.
/// - `headerOnly`: Only header digest verification is enabled.
/// - `dataOnly`: Only data digest verification is enabled.
/// - `headerAndData`: Both header and data digest verification are enabled.
///   
/// ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostDigestVerificationSettingEnum {
    #[serde(rename = "digestDisabled")]
    #[strum(serialize = "digestDisabled")]
    DigestDisabled,
    #[serde(rename = "headerOnly")]
    #[strum(serialize = "headerOnly")]
    HeaderOnly,
    #[serde(rename = "dataOnly")]
    #[strum(serialize = "dataOnly")]
    DataOnly,
    #[serde(rename = "headerAndData")]
    #[strum(serialize = "headerAndData")]
    HeaderAndData,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// List of partition format types.
/// 
/// This denotes the partition table layout.
/// 
/// Possible values:
/// - `gpt`
/// - `mbr`
/// - `unknown`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostDiskPartitionInfoPartitionFormatEnum {
    #[serde(rename = "gpt")]
    #[strum(serialize = "gpt")]
    Gpt,
    #[serde(rename = "mbr")]
    #[strum(serialize = "mbr")]
    Mbr,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// List of symbol partition types
/// 
/// Possible values:
/// - `none`
/// - `vmfs`
/// - `linuxNative`
/// - `linuxSwap`
/// - `extended`
/// - `ntfs`
/// - `vmkDiagnostic`
/// - `vffs`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostDiskPartitionInfoTypeEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "vmfs")]
    #[strum(serialize = "vmfs")]
    Vmfs,
    #[serde(rename = "linuxNative")]
    #[strum(serialize = "linuxNative")]
    LinuxNative,
    #[serde(rename = "linuxSwap")]
    #[strum(serialize = "linuxSwap")]
    LinuxSwap,
    #[serde(rename = "extended")]
    #[strum(serialize = "extended")]
    Extended,
    #[serde(rename = "ntfs")]
    #[strum(serialize = "ntfs")]
    Ntfs,
    #[serde(rename = "vmkDiagnostic")]
    #[strum(serialize = "vmkDiagnostic")]
    VmkDiagnostic,
    #[serde(rename = "vffs")]
    #[strum(serialize = "vffs")]
    Vffs,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for
/// *HostFeatureVersionInfo.key*, which
/// is a unique key that identifies a feature.
/// 
/// Possible values:
/// - `faultTolerance`: VMware Fault Tolerance feature.
///   
///   For pre-4.1 hosts, the
///   version value reported will be empty in which case
///   *AboutInfo.build* should be used. For all
///   other hosts, the version number reported will be a component-specific
///   version identifier of the form X.Y.Z, where:
///   X refers to host agent Fault Tolerance version number,
///   Y refers to VMX Fault Tolerance version number,
///   Z refers to VMkernal Fault Tolerance version
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostFeatureVersionKeyEnum {
    #[serde(rename = "faultTolerance")]
    #[strum(serialize = "faultTolerance")]
    FaultTolerance,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The operating mode of the adapter.
/// 
/// Possible values:
/// - `fabric`
/// - `loop`
/// - `pointToPoint`
/// - `unknown`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum FibreChannelPortTypeEnum {
    #[serde(rename = "fabric")]
    #[strum(serialize = "fabric")]
    Fabric,
    #[serde(rename = "loop")]
    #[strum(serialize = "loop")]
    Loop,
    #[serde(rename = "pointToPoint")]
    #[strum(serialize = "pointToPoint")]
    PointToPoint,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Status of volume's support for vStorage hardware acceleration.
/// 
/// The ESX Server determines the status based on the capabilities
/// of the devices that support the file system volume.
/// When a host boots, the support status is unknown.
/// As the ESX host attempts hardware-accelerated operations,
/// it determines whether the storage device supports hardware
/// acceleration and sets the *HostFileSystemMountInfo.vStorageSupport*
/// property accordingly.
/// 
/// Possible values:
/// - `vStorageSupported`: Storage device supports hardware acceleration.
///   
///   The ESX host will use the feature to offload certain
///   storage-related operations to the device.
/// - `vStorageUnsupported`: Storage device does not support hardware acceleration.
///   
///   The ESX host will handle all storage-related operations.
/// - `vStorageUnknown`: Initial support status value.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum FileSystemMountInfoVStorageSupportStatusEnum {
    #[serde(rename = "vStorageSupported")]
    #[strum(serialize = "vStorageSupported")]
    VStorageSupported,
    #[serde(rename = "vStorageUnsupported")]
    #[strum(serialize = "vStorageUnsupported")]
    VStorageUnsupported,
    #[serde(rename = "vStorageUnknown")]
    #[strum(serialize = "vStorageUnknown")]
    VStorageUnknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Type of file system volume.
/// 
/// Possible values:
/// - `VMFS`: VMware File System (ESX Server only).
///   
///   If this is set,
///   the type of the file system volume is VMFS.
/// - `NFS`: Network file system v3 linux &amp; esx servers only.
///   
///   If this is
///   set, the type of the file system volume is NFS v3.
/// - `NFS41`: Network file system v4.1 linux &amp; esx servers only.
///   
///   If this is
///   set, the type of the file system volume is NFS v4.1 or later.
/// - `CIFS`: Common Internet File System.
///   
///   If this is set, the type of the
///   file system volume is Common Internet File System.
/// - `vsan`: VSAN File System (ESX Server only).
/// - `VFFS`: vFlash File System (ESX Server only).
///   
///   If this is set, the type of the file system volume is VFFS.
/// - `VVOL`: vvol File System (ESX Server only).
/// - `PMEM`: Persistent Memory File System (ESX Server only).
/// - `vsanD`: VSAN direct file system.
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `OTHER`: Used if the file system is not one of the specified file systems.
///   
///   Used mostly for reporting purposes. The other types are described
///   by the otherType property.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostFileSystemVolumeFileSystemTypeEnum {
    #[serde(rename = "VMFS")]
    #[strum(serialize = "VMFS")]
    Vmfs,
    #[serde(rename = "NFS")]
    #[strum(serialize = "NFS")]
    Nfs,
    #[serde(rename = "NFS41")]
    #[strum(serialize = "NFS41")]
    Nfs41,
    #[serde(rename = "CIFS")]
    #[strum(serialize = "CIFS")]
    Cifs,
    #[serde(rename = "vsan")]
    #[strum(serialize = "vsan")]
    Vsan,
    #[serde(rename = "VFFS")]
    #[strum(serialize = "VFFS")]
    Vffs,
    #[serde(rename = "VVOL")]
    #[strum(serialize = "VVOL")]
    Vvol,
    #[serde(rename = "PMEM")]
    #[strum(serialize = "PMEM")]
    Pmem,
    #[serde(rename = "vsanD")]
    #[strum(serialize = "vsanD")]
    VsanD,
    #[serde(rename = "OTHER")]
    #[strum(serialize = "OTHER")]
    Other,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// List of available firewall ruleset ids
/// 
/// Possible values:
/// - `faultTolerance`
/// - `fdm`
/// - `updateManager`
/// - `vpxHeartbeats`
/// 
/// ***Since:*** vSphere API Release 8.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostFirewallSystemRuleSetIdEnum {
    #[serde(rename = "faultTolerance")]
    #[strum(serialize = "faultTolerance")]
    FaultTolerance,
    #[serde(rename = "fdm")]
    #[strum(serialize = "fdm")]
    Fdm,
    #[serde(rename = "updateManager")]
    #[strum(serialize = "updateManager")]
    UpdateManager,
    #[serde(rename = "vpxHeartbeats")]
    #[strum(serialize = "vpxHeartbeats")]
    VpxHeartbeats,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// List of available service names
/// 
/// Possible values:
/// - `vpxa`
/// 
/// ***Since:*** vSphere API Release 8.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostFirewallSystemServiceNameEnum {
    #[serde(rename = "vpxa")]
    #[strum(serialize = "vpxa")]
    Vpxa,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The vendor definition for type of Field Replaceable Unit (FRU).
/// 
/// Possible values:
/// - `undefined`
/// - `board`
/// - `product`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostFruFruTypeEnum {
    #[serde(rename = "undefined")]
    #[strum(serialize = "undefined")]
    Undefined,
    #[serde(rename = "board")]
    #[strum(serialize = "board")]
    Board,
    #[serde(rename = "product")]
    #[strum(serialize = "product")]
    Product,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values for graphics type.
/// 
/// Possible values:
/// - `shared`: Shared graphics (ex.
///   
///   virtual shared graphics acceleration).
/// - `sharedDirect`: Shared direct graphics (ex.
///   
///   vendor vGPU shared passthrough).
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostGraphicsConfigGraphicsTypeEnum {
    #[serde(rename = "shared")]
    #[strum(serialize = "shared")]
    Shared,
    #[serde(rename = "sharedDirect")]
    #[strum(serialize = "sharedDirect")]
    SharedDirect,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values for shared passthrough assignment policy
/// 
/// Possible values:
/// - `performance`: Performance policy: assign VM to GPU with fewest VMs.
/// - `consolidation`: Consolidation policy: group like VMs on GPU until fully loaded.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostGraphicsConfigSharedPassthruAssignmentPolicyEnum {
    #[serde(rename = "performance")]
    #[strum(serialize = "performance")]
    Performance,
    #[serde(rename = "consolidation")]
    #[strum(serialize = "consolidation")]
    Consolidation,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values for graphics type.
/// 
/// Possible values:
/// - `basic`: Basic graphics when no host driver is available.
/// - `shared`: Shared graphics (ex.
///   
///   virtual shared graphics acceleration).
/// - `direct`: Direct graphics (ex.
///   
///   passthrough).
/// - `sharedDirect`: Shared direct graphics (ex.
///   
///   vGPU shared passthrough).
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostGraphicsInfoGraphicsTypeEnum {
    #[serde(rename = "basic")]
    #[strum(serialize = "basic")]
    Basic,
    #[serde(rename = "shared")]
    #[strum(serialize = "shared")]
    Shared,
    #[serde(rename = "direct")]
    #[strum(serialize = "direct")]
    Direct,
    #[serde(rename = "sharedDirect")]
    #[strum(serialize = "sharedDirect")]
    SharedDirect,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The current status of the hardware
/// 
/// Possible values:
/// - `Unknown`: The implementation cannot report on the current status of the
///   physical element
/// - `Green`: The physical element is functioning as expected
/// - `Yellow`: All functionality is available but some might be degraded.
/// - `Red`: The physical element is failing.
///   
///   It is possible that some or all
///   functionalities of this physical element is degraded or not working.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostHardwareElementStatusEnum {
    Unknown,
    Green,
    Yellow,
    Red,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines different access modes that a user may have on the host for
/// direct host connections.
/// 
/// The assumption here is that when the host is managed by vCenter,
/// we don't need fine-grained control on local user permissions like the
/// interface provided by *AuthorizationManager*.
/// 
/// Possible values:
/// - `accessNone`: Indicates that the user has no explicitly defined permissions or roles.
///   
///   This is used when we want to remove all permissions for some user.
///   
///   Note that this is not the same as *accessNoAccess*.
/// - `accessAdmin`: Describes a propagating Admin role on the root inventory object
///   (root folder) on the host, and no other non-Admin role on any other
///   object.
///   
///   The same permissions are needed to login to local or remote
///   shell (ESXiShell or SSH).
/// - `accessNoAccess`: Describes a propagating NoAccess role on the root inventory object
///   (root folder) on the host, and no other roles.
///   
///   Even if the user has another (redundant) NoAccess role on some other
///   inventory object, then the access mode for this user will be
///   classified as *accessOther*.
///   
///   This mode may be used to restrict a specific user account without
///   restricting the access mode for the group to which the user belongs.
/// - `accessReadOnly`: Describes a propagating ReadOnly role on the root inventory object
///   (root folder) on the host, and no other roles.
///   
///   Even if the user has another (redundant) ReadOnly role on some other
///   inventory object, then the access mode for this user will be
///   *accessOther*.
/// - `accessOther`: Describes a combination of one or more roles/permissions which are
///   none of the above.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostAccessModeEnum {
    #[serde(rename = "accessNone")]
    #[strum(serialize = "accessNone")]
    AccessNone,
    #[serde(rename = "accessAdmin")]
    #[strum(serialize = "accessAdmin")]
    AccessAdmin,
    #[serde(rename = "accessNoAccess")]
    #[strum(serialize = "accessNoAccess")]
    AccessNoAccess,
    #[serde(rename = "accessReadOnly")]
    #[strum(serialize = "accessReadOnly")]
    AccessReadOnly,
    #[serde(rename = "accessOther")]
    #[strum(serialize = "accessOther")]
    AccessOther,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines the possible states of lockdown mode.
/// 
/// Possible values:
/// - `lockdownDisabled`: Indicates that lockdown mode is disabled.
/// - `lockdownNormal`: Indicates that lockdown mode is enabled with service DCUI
///   (Direct Console User Interface) running.
/// - `lockdownStrict`: Indicates that lockdown mode is enabled with service DCUI stopped.
///   
///   If the host is in "strict" lockdown mode then no one will be able
///   to exit lockdown mode through DCUI in emergency situations,
///   i.e. when the connection to vCenter server is permanently lost.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostLockdownModeEnum {
    #[serde(rename = "lockdownDisabled")]
    #[strum(serialize = "lockdownDisabled")]
    LockdownDisabled,
    #[serde(rename = "lockdownNormal")]
    #[strum(serialize = "lockdownNormal")]
    LockdownNormal,
    #[serde(rename = "lockdownStrict")]
    #[strum(serialize = "lockdownStrict")]
    LockdownStrict,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Acceptance level definitions
/// 
/// Possible values:
/// - `vmware_certified`: "VMware-certified"
/// - `vmware_accepted`: "VMware-accepted"
/// - `partner`: "Partner-supported"
/// - `community`: "Community-supported"
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostImageAcceptanceLevelEnum {
    #[serde(rename = "vmware_certified")]
    #[strum(serialize = "vmware_certified")]
    VmwareCertified,
    #[serde(rename = "vmware_accepted")]
    #[strum(serialize = "vmware_accepted")]
    VmwareAccepted,
    #[serde(rename = "partner")]
    #[strum(serialize = "partner")]
    Partner,
    #[serde(rename = "community")]
    #[strum(serialize = "community")]
    Community,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The type of CHAP authentication setting to use.
/// 
/// prohibited : do not use CHAP.
/// preferred : use CHAP if successfully negotiated,
/// but allow non-CHAP connections as fallback
/// discouraged : use non-CHAP, but allow CHAP connectsion as fallback
/// required : use CHAP for connection strictly, and fail if CHAP
/// negotiation fails.
/// Defaults to preferred on first configuration if unspecified.
/// 
/// Possible values:
/// - `chapProhibited`
/// - `chapDiscouraged`
/// - `chapPreferred`
/// - `chapRequired`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostInternetScsiHbaChapAuthenticationTypeEnum {
    #[serde(rename = "chapProhibited")]
    #[strum(serialize = "chapProhibited")]
    ChapProhibited,
    #[serde(rename = "chapDiscouraged")]
    #[strum(serialize = "chapDiscouraged")]
    ChapDiscouraged,
    #[serde(rename = "chapPreferred")]
    #[strum(serialize = "chapPreferred")]
    ChapPreferred,
    #[serde(rename = "chapRequired")]
    #[strum(serialize = "chapRequired")]
    ChapRequired,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The type of integrity checks to use.
/// 
/// The digest setting for header
/// and data traffic can be separately configured.
/// prohibited : do not use digest.
/// preferred : use digest if successfully negotiated, but skip the use
/// of digest otherwise.
/// discouraged : do not use digest if target allows, otherwise use digest.
/// required : use digest strictly, and fail if target does not support
/// digest.
/// Defaults to preferred on first configuration if unspecified.
/// 
/// Possible values:
/// - `digestProhibited`
/// - `digestDiscouraged`
/// - `digestPreferred`
/// - `digestRequired`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostInternetScsiHbaDigestTypeEnum {
    #[serde(rename = "digestProhibited")]
    #[strum(serialize = "digestProhibited")]
    DigestProhibited,
    #[serde(rename = "digestDiscouraged")]
    #[strum(serialize = "digestDiscouraged")]
    DigestDiscouraged,
    #[serde(rename = "digestPreferred")]
    #[strum(serialize = "digestPreferred")]
    DigestPreferred,
    #[serde(rename = "digestRequired")]
    #[strum(serialize = "digestRequired")]
    DigestRequired,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The available iSNS discovery methods.
/// 
/// Possible values:
/// - `isnsStatic`
/// - `isnsDhcp`
/// - `isnsSlp`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum InternetScsiSnsDiscoveryMethodEnum {
    #[serde(rename = "isnsStatic")]
    #[strum(serialize = "isnsStatic")]
    IsnsStatic,
    #[serde(rename = "isnsDhcp")]
    #[strum(serialize = "isnsDhcp")]
    IsnsDhcp,
    #[serde(rename = "isnsSlp")]
    #[strum(serialize = "isnsSlp")]
    IsnsSlp,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The available SLP discovery methods.
/// 
/// Possible values:
/// - `slpDhcp`: Use DHCP to find the SLP DAs.
/// - `slpAutoUnicast`: Use broadcasting to find SLP DAs.
///   
///   Only DAs on the current subnet will be found.
/// - `slpAutoMulticast`: Use the well known multicast address to find DAs.
/// - `slpManual`: User specified address for a DA.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum SlpDiscoveryMethodEnum {
    #[serde(rename = "slpDhcp")]
    #[strum(serialize = "slpDhcp")]
    SlpDhcp,
    #[serde(rename = "slpAutoUnicast")]
    #[strum(serialize = "slpAutoUnicast")]
    SlpAutoUnicast,
    #[serde(rename = "slpAutoMulticast")]
    #[strum(serialize = "slpAutoMulticast")]
    SlpAutoMulticast,
    #[serde(rename = "slpManual")]
    #[strum(serialize = "slpManual")]
    SlpManual,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// enum listing possible IPv6 address configuration methods.
/// 
/// Possible values:
/// - `DHCP`: DHCP
/// - `AutoConfigured`: Auto configured.
///   
///   Auto configured Link local address and Router Advertisement addresses
///   would be of this type.
/// - `Static`: Static address.
///   
///   Typically user specified addresses will be static addresses.
///   User can specify link local address. Only Static addresses can be added or removed.
/// - `Other`: Other or unknown type.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostInternetScsiHbaIscsiIpv6AddressAddressConfigurationTypeEnum {
    #[serde(rename = "DHCP")]
    #[strum(serialize = "DHCP")]
    Dhcp,
    AutoConfigured,
    Static,
    Other,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// enum listing IPv6 address operations.
/// 
/// Possible values:
/// - `add`
/// - `remove`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostInternetScsiHbaIscsiIpv6AddressIPv6AddressOperationEnum {
    #[serde(rename = "add")]
    #[strum(serialize = "add")]
    Add,
    #[serde(rename = "remove")]
    #[strum(serialize = "remove")]
    Remove,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The binding mode of the adapter.
/// 
/// Possible values:
/// - `notsupported`
/// - `optional`
/// - `required`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostInternetScsiHbaNetworkBindingSupportTypeEnum {
    #[serde(rename = "notsupported")]
    #[strum(serialize = "notsupported")]
    Notsupported,
    #[serde(rename = "optional")]
    #[strum(serialize = "optional")]
    Optional,
    #[serde(rename = "required")]
    #[strum(serialize = "required")]
    Required,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The method of discovery of an iScsi target.
/// 
/// staticMethod: static discovery
/// sendTargetsMethod: sendtarget discovery
/// slpMethod: Service Location Protocol discovery
/// isnsMethod: Internet Storage Name Service discovery
/// unknownMethod: discovery method not identified by iscsi stack
/// 
/// Possible values:
/// - `staticMethod`
/// - `sendTargetMethod`
/// - `slpMethod`
/// - `isnsMethod`
/// - `unknownMethod`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostInternetScsiHbaStaticTargetTargetDiscoveryMethodEnum {
    #[serde(rename = "staticMethod")]
    #[strum(serialize = "staticMethod")]
    StaticMethod,
    #[serde(rename = "sendTargetMethod")]
    #[strum(serialize = "sendTargetMethod")]
    SendTargetMethod,
    #[serde(rename = "slpMethod")]
    #[strum(serialize = "slpMethod")]
    SlpMethod,
    #[serde(rename = "isnsMethod")]
    #[strum(serialize = "isnsMethod")]
    IsnsMethod,
    #[serde(rename = "unknownMethod")]
    #[strum(serialize = "unknownMethod")]
    UnknownMethod,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This specifies how the ipv6 address is configured for the interface.
/// 
/// We follow rfc4293 in defining the values for the configType.
/// 
/// Possible values:
/// - `other`: Any other type of address configuration other than the below
///   mentioned ones will fall under this category.
///   
///   For e.g., automatic
///   address configuration for the link local address falls under
///   this type.
/// - `manual`: The address is configured manually.
/// - `dhcp`: The address is configured through dhcp.
/// - `linklayer`: The address is obtained through stateless autoconfiguration.
/// - `random`: The address is chosen by the system at random
///   e.g., an IPv4 address within 169.254/16, or an RFC
///   3041 privacy address.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostIpConfigIpV6AddressConfigTypeEnum {
    #[serde(rename = "other")]
    #[strum(serialize = "other")]
    Other,
    #[serde(rename = "manual")]
    #[strum(serialize = "manual")]
    Manual,
    #[serde(rename = "dhcp")]
    #[strum(serialize = "dhcp")]
    Dhcp,
    #[serde(rename = "linklayer")]
    #[strum(serialize = "linklayer")]
    Linklayer,
    #[serde(rename = "random")]
    #[strum(serialize = "random")]
    Random,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `preferred`: Indicates that this is a valid address.
/// - `deprecated`: Indicates that this is a valid but deprecated address
///   that should no longer be used as a source address.
/// - `invalid`: Indicates that this isn't a valid.
/// - `inaccessible`: Indicates that the address is not accessible because
///   interface is not operational.
/// - `unknown`: Indicates that the status cannot be determined.
/// - `tentative`: Indicates that the uniqueness of the
///   address on the link is presently being verified.
/// - `duplicate`: Indicates the address has been determined to be non-unique
///   on the link, this address will not be reachable.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostIpConfigIpV6AddressStatusEnum {
    #[serde(rename = "preferred")]
    #[strum(serialize = "preferred")]
    Preferred,
    #[serde(rename = "deprecated")]
    #[strum(serialize = "deprecated")]
    Deprecated,
    #[serde(rename = "invalid")]
    #[strum(serialize = "invalid")]
    Invalid,
    #[serde(rename = "inaccessible")]
    #[strum(serialize = "inaccessible")]
    Inaccessible,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "tentative")]
    #[strum(serialize = "tentative")]
    Tentative,
    #[serde(rename = "duplicate")]
    #[strum(serialize = "duplicate")]
    Duplicate,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `notUsed`: There are no paths on this Virtual NIC
/// - `active`: All paths on this Virtual NIC are standby paths from SCSI stack
///   perspective.
/// - `standBy`: One or more paths on the Virtual NIC are active paths to
///   storage.
///   
///   Unbinding this Virtual NIC will cause storage path
///   transitions.
/// - `lastActive`: One or more paths on the Virtual NIC is the last active
///   path to a particular storage device.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum IscsiPortInfoPathStatusEnum {
    #[serde(rename = "notUsed")]
    #[strum(serialize = "notUsed")]
    NotUsed,
    #[serde(rename = "active")]
    #[strum(serialize = "active")]
    Active,
    #[serde(rename = "standBy")]
    #[strum(serialize = "standBy")]
    StandBy,
    #[serde(rename = "lastActive")]
    #[strum(serialize = "lastActive")]
    LastActive,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The Discovery Protocol operation.
/// 
/// Possible values:
/// - `none`: Don't listen for incoming discovery packets and don't sent discover
///   packets for the switch either.
/// - `listen`: Listen for incoming discovery packets but don't sent discovery packet
///   for the switch.
/// - `advertise`: Sent discovery packets for the switch, but don't listen for incoming
///   discovery packets.
/// - `both`: Sent discovery packets for the switch and listen for incoming
///   discovery packets.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum LinkDiscoveryProtocolConfigOperationTypeEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "listen")]
    #[strum(serialize = "listen")]
    Listen,
    #[serde(rename = "advertise")]
    #[strum(serialize = "advertise")]
    Advertise,
    #[serde(rename = "both")]
    #[strum(serialize = "both")]
    Both,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The Discovery Protocol types.
/// 
/// Possible values:
/// - `cdp`: Cisco Discovery Protocol
/// - `lldp`: Link Layer Discovery Protocol
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum LinkDiscoveryProtocolConfigProtocolTypeEnum {
    #[serde(rename = "cdp")]
    #[strum(serialize = "cdp")]
    Cdp,
    #[serde(rename = "lldp")]
    #[strum(serialize = "lldp")]
    Lldp,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This enum defines the possible types of file types that can be reserved
/// or deleted
/// 
/// Possible values:
/// - `File`
/// - `VirtualDisk`
/// - `Directory`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostLowLevelProvisioningManagerFileTypeEnum {
    File,
    VirtualDisk,
    Directory,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The target of the disk reload.
/// 
/// Possible values:
/// - `currentConfig`: Specifies the reload of the current config of the virtual machine.
/// - `snapshotConfig`: Specifies the reload of the snapshot config of the virtual machine.
///   
///   If the virtual machine has multiple snapshots, all of the snapshot's
///   config will be reloaded.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostLowLevelProvisioningManagerReloadTargetEnum {
    #[serde(rename = "currentConfig")]
    #[strum(serialize = "currentConfig")]
    CurrentConfig,
    #[serde(rename = "snapshotConfig")]
    #[strum(serialize = "snapshotConfig")]
    SnapshotConfig,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `hostUpgrade`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostMaintenanceSpecPurposeEnum {
    #[serde(rename = "hostUpgrade")]
    #[strum(serialize = "hostUpgrade")]
    HostUpgrade,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Means for allocating additional memory for virtual machines.
/// 
/// Possible values:
/// - `swapNone`: Fit all virtual machine memory into reserved host memory.
/// - `swapSome`: Allow some virtual machine memory to be swapped.
/// - `swapMost`: Allow most virtual machine memory to be swapped.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineMemoryAllocationPolicyEnum {
    #[serde(rename = "swapNone")]
    #[strum(serialize = "swapNone")]
    SwapNone,
    #[serde(rename = "swapSome")]
    #[strum(serialize = "swapSome")]
    SwapSome,
    #[serde(rename = "swapMost")]
    #[strum(serialize = "swapMost")]
    SwapMost,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enumeration of flags pertaining to a memory tier.
/// 
/// Here are some examples of what the flags will look like for various memory
/// configurations:
/// - Traditional memory (*noTiering*): The host has a DRAM tier
///   for the main memory and nothing else. The DRAM tier will have the
///   *memoryTier* flag.
/// - App Direct mode (*noTiering*): The host has a DRAM tier
///   and a PMem tier, but the two are independent and unrelated. The PMem tier is
///   non-volatile and is exposed as an NVDIMM device. Applications can decide whether to
///   direct the reads and writes to DRAM or PMem by using the appropriate system call. The
///   DRAM tier will have the *memoryTier* flag and the PMem tier will
///   have the *persistentTier* flag.
/// - Memory mode (*hardwareTiering*): The host has a DRAM tier
///   and a PMem tier, but the DRAM is hidden from applications and is just a cache
///   for the PMem main memory. The PMem tier is volatile, and is abstracted by the hardware
///   layer to look like traditional memory. Applications can read from/write to memory
///   using the traditional memory system calls. The memory controller in the hardware will
///   internally direct those to the DRAM cache first, and on a cache miss redirect them to
///   the PMem main memory. The DRAM tier will have the *cachingTier*
///   flag and the PMem tier will have the *memoryTier* flag.
///   
/// Possible values:
/// - `memoryTier`: Flag indicating that the tier is the primary memory tier visible from the
///   host.
/// - `persistentTier`: Flag indicating that the tier is used as non-volatile storage, e.g.
///   
///   PMem in
///   App Direct mode.
/// - `cachingTier`: Flag indicating that the tier is a cache for main memory.
///   
/// ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostMemoryTierFlagsEnum {
    #[serde(rename = "memoryTier")]
    #[strum(serialize = "memoryTier")]
    MemoryTier,
    #[serde(rename = "persistentTier")]
    #[strum(serialize = "persistentTier")]
    PersistentTier,
    #[serde(rename = "cachingTier")]
    #[strum(serialize = "cachingTier")]
    CachingTier,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enumeration of supported types of memory tiers.
/// 
/// Possible values:
/// - `DRAM`: Dynamic random-access memory.
/// - `PMem`: Persistent memory.
///   
/// ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostMemoryTierTypeEnum {
    #[serde(rename = "DRAM")]
    #[strum(serialize = "DRAM")]
    Dram,
    PMem,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enumeration of the supported kinds of memory tiering configurations.
/// 
/// Possible values:
/// - `noTiering`: The traditional memory configuration without any tiers.
/// - `hardwareTiering`: The memory configuration where a tier is hardware-controlled and invisible to
///   applications, e.g.
///   
///   Intel's Memory Mode.
/// 
/// ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostMemoryTieringTypeEnum {
    #[serde(rename = "noTiering")]
    #[strum(serialize = "noTiering")]
    NoTiering,
    #[serde(rename = "hardwareTiering")]
    #[strum(serialize = "hardwareTiering")]
    HardwareTiering,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines the access mode of the datastore.
/// 
/// Possible values:
/// - `readWrite`: The host system has read/write access to the file system.
/// - `readOnly`: The host system has read-only access to the file system.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostMountModeEnum {
    #[serde(rename = "readWrite")]
    #[strum(serialize = "readWrite")]
    ReadWrite,
    #[serde(rename = "readOnly")]
    #[strum(serialize = "readOnly")]
    ReadOnly,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// A datastore can become inaccessible due to a number of reasons as
/// defined in this enum *HostMountInfoInaccessibleReason_enum*.
/// 
/// The reason for a datastore being inaccessible is reported in
/// *HostMountInfo.inaccessibleReason*.
/// APD ("All Paths Down") is a condition where a SAN or NFS storage has
/// become inaccessible for unknown reasons. It only indicates loss of
/// connectivity and does not indicate storage device failure or
/// LUN removal (Permanent Device Loss or PDL)
/// A difference between APD and PDL is that APD may recover
/// in which case all use cases will start to work as before. In case of PDL
/// the failed datastore/device is unlikely to recover and hence the device
/// path information and data cache will be emptied. If the PDL condition
/// recovers, the failed datastores have to be added back to the host. Once
/// in PDL a datastore cannot be added back until there are no longer any
/// open files on the datastore.
/// PDL is not linked to the APD and can happen at any time with or without APD
/// preceding. If APD and PDL occur at the same time, APD will be reported first.
/// Once (and if) the APD condition clears, PermanentDataLoss will be reported if
/// PDL condition still exists.
/// 
/// Possible values:
/// - `AllPathsDown_Start`: AllPathsDown\_Start value is reported when all paths down state is detected
/// - `AllPathsDown_Timeout`: After a wait for a system default time (which is user modifiable)
///   to ascertain the state is indeed an APD, AllPathsDown\_Timeout property
///   is reported.
///   
///   The host advanced option used to set timeout period
///   is "/Misc/APDTimeout"
///   After the datastore property is set to AllPathsDown\_Timeout, all data i/o
///   to the datastore will be fast-failed (failed immediately).
/// - `PermanentDeviceLoss`: A PDL condition is reported as PermanentDeviceLoss.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostMountInfoInaccessibleReasonEnum {
    #[serde(rename = "AllPathsDown_Start")]
    #[strum(serialize = "AllPathsDown_Start")]
    AllPathsDownStart,
    #[serde(rename = "AllPathsDown_Timeout")]
    #[strum(serialize = "AllPathsDown_Timeout")]
    AllPathsDownTimeout,
    PermanentDeviceLoss,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// NFS mount request can be failed due to a number of reasons as
/// defined in this enum *HostMountInfoMountFailedReason_enum*.
/// 
/// The reason for the mount failure is reported in
/// *HostMountInfo.mountFailedReason*. This is applicable only for those
/// datastores to which mount retry is configured.
/// 
/// Possible values:
/// - `CONNECT_FAILURE`: Failed to get port or connect.
///   
///   Or MOUNT/FSINFO RPC failed.
/// - `MOUNT_NOT_SUPPORTED`: Server doesn't support MOUNT\_PROGRAM/MOUNT\_PROGRAM\_VERSION.
/// - `NFS_NOT_SUPPORTED`: Server doesn't support NFS\_PROGRAM/NFS\_PROGRAM\_VERSION.
/// - `MOUNT_DENIED`: No permission to mount the remote volume or it doesn't exist.
/// - `MOUNT_NOT_DIR`: Remote path not a directory.
/// - `VOLUME_LIMIT_EXCEEDED`: Maximum NFS volumes have been mounted.
/// - `CONN_LIMIT_EXCEEDED`: Maximum connections for NFS has been reached.
/// - `MOUNT_EXISTS`: Volume already mounted or a different mount exists with same label.
/// - `OTHERS`: Any other reason which is not present in above list.
///   
/// ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostMountInfoMountFailedReasonEnum {
    #[serde(rename = "CONNECT_FAILURE")]
    #[strum(serialize = "CONNECT_FAILURE")]
    ConnectFailure,
    #[serde(rename = "MOUNT_NOT_SUPPORTED")]
    #[strum(serialize = "MOUNT_NOT_SUPPORTED")]
    MountNotSupported,
    #[serde(rename = "NFS_NOT_SUPPORTED")]
    #[strum(serialize = "NFS_NOT_SUPPORTED")]
    NfsNotSupported,
    #[serde(rename = "MOUNT_DENIED")]
    #[strum(serialize = "MOUNT_DENIED")]
    MountDenied,
    #[serde(rename = "MOUNT_NOT_DIR")]
    #[strum(serialize = "MOUNT_NOT_DIR")]
    MountNotDir,
    #[serde(rename = "VOLUME_LIMIT_EXCEEDED")]
    #[strum(serialize = "VOLUME_LIMIT_EXCEEDED")]
    VolumeLimitExceeded,
    #[serde(rename = "CONN_LIMIT_EXCEEDED")]
    #[strum(serialize = "CONN_LIMIT_EXCEEDED")]
    ConnLimitExceeded,
    #[serde(rename = "MOUNT_EXISTS")]
    #[strum(serialize = "MOUNT_EXISTS")]
    MountExists,
    #[serde(rename = "OTHERS")]
    #[strum(serialize = "OTHERS")]
    Others,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of constants defining the possible states of a multipath path.
/// 
/// Possible values:
/// - `standby`
/// - `active`
/// - `disabled`
/// - `dead`
/// - `unknown`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum MultipathStateEnum {
    #[serde(rename = "standby")]
    #[strum(serialize = "standby")]
    Standby,
    #[serde(rename = "active")]
    #[strum(serialize = "active")]
    Active,
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "dead")]
    #[strum(serialize = "dead")]
    Dead,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Security type supported.
/// 
/// Possible values:
/// - `AUTH_SYS`: Authentication based on traditional UNIX identifiers (UID and GID).
///   
///   Server trusts the IDs sent by the client for each request and uses them
///   to perform access control. Current implementation only supports
///   AUTH\_SYS with root user.
/// - `SEC_KRB5`: Ensures RPC header authentication using Kerberos session keys.
///   
///   When
///   this option is enabled, the client uses the information specified in
///   *HostNasVolumeUserInfo* to establish shared keys with the server using
///   Kerberos. These shared keys are used to generate and verify message
///   authentication codes for RPC header of NFS requests and responses,
///   respectively. This method does not secure NFS file data.
/// - `SEC_KRB5I`: Extends SEC\_KRB5 to generate and verify message authentication codes
///   for the payload of NFS requests and responses respectively.
///   
///   This
///   ensures the integrity of the NFS file data.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostNasVolumeSecurityTypeEnum {
    #[serde(rename = "AUTH_SYS")]
    #[strum(serialize = "AUTH_SYS")]
    AuthSys,
    #[serde(rename = "SEC_KRB5")]
    #[strum(serialize = "SEC_KRB5")]
    SecKrb5,
    #[serde(rename = "SEC_KRB5I")]
    #[strum(serialize = "SEC_KRB5I")]
    SecKrb5I,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Define TCP congestion control algorithm used by an instance
/// 
/// Possible values:
/// - `newreno`: New Reno Algorithm.
///   
///   See http://tools.ietf.org/html/rfc3782 for detail.
/// - `cubic`: Cubic Algorithm.
///   
///   See http://tools.ietf.org/id/draft-rhee-tcp-cubic-00.txt for detail.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostNetStackInstanceCongestionControlAlgorithmTypeEnum {
    #[serde(rename = "newreno")]
    #[strum(serialize = "newreno")]
    Newreno,
    #[serde(rename = "cubic")]
    #[strum(serialize = "cubic")]
    Cubic,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Define the instance identifier for different traffic type
/// 
/// Possible values:
/// - `defaultTcpipStack`: The default stack used by applications
/// - `vmotion`: Stack key used for vMotion applications
/// - `vSphereProvisioning`: Stack key used for vSphere provisioning NFC traffic
/// - `mirror`: Stack key used for port mirroring
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `ops`: Stack key used for ops applications
///   
///   ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostNetStackInstanceSystemStackKeyEnum {
    #[serde(rename = "defaultTcpipStack")]
    #[strum(serialize = "defaultTcpipStack")]
    DefaultTcpipStack,
    #[serde(rename = "vmotion")]
    #[strum(serialize = "vmotion")]
    Vmotion,
    #[serde(rename = "vSphereProvisioning")]
    #[strum(serialize = "vSphereProvisioning")]
    VSphereProvisioning,
    #[serde(rename = "mirror")]
    #[strum(serialize = "mirror")]
    Mirror,
    #[serde(rename = "ops")]
    #[strum(serialize = "ops")]
    Ops,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Health state of the numeric sensor as reported by the sensor probes.
/// 
/// Same data reported using command line: esxcli hardware ipmi sdr list
/// 
/// Possible values:
/// - `unknown`: The implementation cannot report on the current health state of the
///   physical element
/// - `green`: The sensor is operating under normal conditions
/// - `yellow`: The sensor is operating under conditions that are non-critical.
/// - `red`: The sensor is operating under critical or fatal conditions.
///   
///   This may
///   directly affect the functioning of both the sensor and related
///   components.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostNumericSensorHealthStateEnum {
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "green")]
    #[strum(serialize = "green")]
    Green,
    #[serde(rename = "yellow")]
    #[strum(serialize = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    #[strum(serialize = "red")]
    Red,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Sensor Types for specific hardware component are either based on
/// class of sensor or what the sensor monitors to allow for grouping
/// 
/// Possible values:
/// - `fan`: Fan sensor
/// - `power`: Power sensor
/// - `temperature`: Temperature sensor
/// - `voltage`: Voltage Sensor
/// - `other`: Other sensor.
/// - `processor`: Processor sensor.
/// - `memory`: Memory sensor.
/// - `storage`: disk/storage sensor.
/// - `systemBoard`: system board sensor.
/// - `battery`: Battery sensor.
/// - `bios`: BIOS/firmware related sensor.
/// - `cable`: cable related sensor.
/// - `watchdog`: Watchdog related sensor.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostNumericSensorTypeEnum {
    #[serde(rename = "fan")]
    #[strum(serialize = "fan")]
    Fan,
    #[serde(rename = "power")]
    #[strum(serialize = "power")]
    Power,
    #[serde(rename = "temperature")]
    #[strum(serialize = "temperature")]
    Temperature,
    #[serde(rename = "voltage")]
    #[strum(serialize = "voltage")]
    Voltage,
    #[serde(rename = "other")]
    #[strum(serialize = "other")]
    Other,
    #[serde(rename = "processor")]
    #[strum(serialize = "processor")]
    Processor,
    #[serde(rename = "memory")]
    #[strum(serialize = "memory")]
    Memory,
    #[serde(rename = "storage")]
    #[strum(serialize = "storage")]
    Storage,
    #[serde(rename = "systemBoard")]
    #[strum(serialize = "systemBoard")]
    SystemBoard,
    #[serde(rename = "battery")]
    #[strum(serialize = "battery")]
    Battery,
    #[serde(rename = "bios")]
    #[strum(serialize = "bios")]
    Bios,
    #[serde(rename = "cable")]
    #[strum(serialize = "cable")]
    Cable,
    #[serde(rename = "watchdog")]
    #[strum(serialize = "watchdog")]
    Watchdog,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Overall state of NVDIMM
/// 
/// Possible values:
/// - `normal`: NVDIMM state is normal
/// - `error`: Error in NVDIMM state.
///   
///   Potential data loss.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NvdimmNvdimmHealthInfoStateEnum {
    #[serde(rename = "normal")]
    #[strum(serialize = "normal")]
    Normal,
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// State of interleave set
/// 
/// Possible values:
/// - `invalid`: Interleave set is invalid
/// - `active`: Interleave set is valid and active
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NvdimmInterleaveSetStateEnum {
    #[serde(rename = "invalid")]
    #[strum(serialize = "invalid")]
    Invalid,
    #[serde(rename = "active")]
    #[strum(serialize = "active")]
    Active,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Overall health state for a namespace
/// 
/// Possible values:
/// - `normal`: Namespace health is normal
/// - `missing`: Namespace health is missing
/// - `labelMissing`: Namespace health label is missing
/// - `interleaveBroken`: Namespace health interleave broken
/// - `labelInconsistent`: Namespace health label is inconsistent
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NvdimmNamespaceDetailsHealthStatusEnum {
    #[serde(rename = "normal")]
    #[strum(serialize = "normal")]
    Normal,
    #[serde(rename = "missing")]
    #[strum(serialize = "missing")]
    Missing,
    #[serde(rename = "labelMissing")]
    #[strum(serialize = "labelMissing")]
    LabelMissing,
    #[serde(rename = "interleaveBroken")]
    #[strum(serialize = "interleaveBroken")]
    InterleaveBroken,
    #[serde(rename = "labelInconsistent")]
    #[strum(serialize = "labelInconsistent")]
    LabelInconsistent,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// State of Namespace
/// 
/// Possible values:
/// - `invalid`: Namespace is invalid
/// - `notInUse`: Namespace is valid but not in use
/// - `inUse`: Namespace is valid and is in use
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NvdimmNamespaceDetailsStateEnum {
    #[serde(rename = "invalid")]
    #[strum(serialize = "invalid")]
    Invalid,
    #[serde(rename = "notInUse")]
    #[strum(serialize = "notInUse")]
    NotInUse,
    #[serde(rename = "inUse")]
    #[strum(serialize = "inUse")]
    InUse,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Overall health state for a namespace
/// 
/// Possible values:
/// - `normal`: Namespace health is normal
/// - `missing`: Namespace health is missing
/// - `labelMissing`: Namespace health label is missing
/// - `interleaveBroken`: Namespace health interleave broken
/// - `labelInconsistent`: Namespace health label is inconsistent
/// - `bttCorrupt`: Namespace health BTT is corrupt
/// - `badBlockSize`: Namespace health encountered bad block
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NvdimmNamespaceHealthStatusEnum {
    #[serde(rename = "normal")]
    #[strum(serialize = "normal")]
    Normal,
    #[serde(rename = "missing")]
    #[strum(serialize = "missing")]
    Missing,
    #[serde(rename = "labelMissing")]
    #[strum(serialize = "labelMissing")]
    LabelMissing,
    #[serde(rename = "interleaveBroken")]
    #[strum(serialize = "interleaveBroken")]
    InterleaveBroken,
    #[serde(rename = "labelInconsistent")]
    #[strum(serialize = "labelInconsistent")]
    LabelInconsistent,
    #[serde(rename = "bttCorrupt")]
    #[strum(serialize = "bttCorrupt")]
    BttCorrupt,
    #[serde(rename = "badBlockSize")]
    #[strum(serialize = "badBlockSize")]
    BadBlockSize,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// State of Namespace
/// 
/// Possible values:
/// - `invalid`: Namespace is invalid
/// - `notInUse`: Namespace is valid but not in use
/// - `inUse`: Namespace is valid and is in use
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NvdimmNamespaceStateEnum {
    #[serde(rename = "invalid")]
    #[strum(serialize = "invalid")]
    Invalid,
    #[serde(rename = "notInUse")]
    #[strum(serialize = "notInUse")]
    NotInUse,
    #[serde(rename = "inUse")]
    #[strum(serialize = "inUse")]
    InUse,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Type of namespace.
/// 
/// Possible values:
/// - `blockNamespace`: Block mode namespace
/// - `persistentNamespace`: Persistent mode namespace
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NvdimmNamespaceTypeEnum {
    #[serde(rename = "blockNamespace")]
    #[strum(serialize = "blockNamespace")]
    BlockNamespace,
    #[serde(rename = "persistentNamespace")]
    #[strum(serialize = "persistentNamespace")]
    PersistentNamespace,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// An indicator of how a memory range is being used
/// 
/// Possible values:
/// - `volatileRange`: Identifies the region to be volatile
/// - `persistentRange`: Identifies the region to be persistent
/// - `controlRange`: NVDIMM control region
/// - `blockRange`: NVDIMM block data window region
/// - `volatileVirtualDiskRange`: NVDIMM volatile virtual disk region
/// - `volatileVirtualCDRange`: NVDIMM volatile virtual CD region
/// - `persistentVirtualDiskRange`: NVDIMM persistent virtual disk region
/// - `persistentVirtualCDRange`: NVDIMM persistent virtual CD region
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NvdimmRangeTypeEnum {
    #[serde(rename = "volatileRange")]
    #[strum(serialize = "volatileRange")]
    VolatileRange,
    #[serde(rename = "persistentRange")]
    #[strum(serialize = "persistentRange")]
    PersistentRange,
    #[serde(rename = "controlRange")]
    #[strum(serialize = "controlRange")]
    ControlRange,
    #[serde(rename = "blockRange")]
    #[strum(serialize = "blockRange")]
    BlockRange,
    #[serde(rename = "volatileVirtualDiskRange")]
    #[strum(serialize = "volatileVirtualDiskRange")]
    VolatileVirtualDiskRange,
    #[serde(rename = "volatileVirtualCDRange")]
    #[strum(serialize = "volatileVirtualCDRange")]
    VolatileVirtualCdRange,
    #[serde(rename = "persistentVirtualDiskRange")]
    #[strum(serialize = "persistentVirtualDiskRange")]
    PersistentVirtualDiskRange,
    #[serde(rename = "persistentVirtualCDRange")]
    #[strum(serialize = "persistentVirtualCDRange")]
    PersistentVirtualCdRange,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This enum represents the supported NVM subsystem types.
/// 
/// Possible values:
/// - `discovery`: A Discovery service, composed of Discovery controllers.
/// - `nvm`: An NVM subsystem whose controllers may have attached namespaces.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostNvmeDiscoveryLogSubsystemTypeEnum {
    #[serde(rename = "discovery")]
    #[strum(serialize = "discovery")]
    Discovery,
    #[serde(rename = "nvm")]
    #[strum(serialize = "nvm")]
    Nvm,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This enum represents the supported types of transport requirements.
/// 
/// Possible values:
/// - `secureChannelRequired`: A fabric secure channel is required.
/// - `secureChannelNotRequired`: A fabric secure channel is not required.
/// - `requirementsNotSpecified`: Requirements are not specified
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostNvmeDiscoveryLogTransportRequirementsEnum {
    #[serde(rename = "secureChannelRequired")]
    #[strum(serialize = "secureChannelRequired")]
    SecureChannelRequired,
    #[serde(rename = "secureChannelNotRequired")]
    #[strum(serialize = "secureChannelNotRequired")]
    SecureChannelNotRequired,
    #[serde(rename = "requirementsNotSpecified")]
    #[strum(serialize = "requirementsNotSpecified")]
    RequirementsNotSpecified,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This enum specifies the supported address families for
/// NVME over Fabrics.
/// 
/// For details, see:
/// - "NVM Express over Fabrics 1.0", Section 5.3, Figure 34,
///   "Discovery Log Page Entry"
///   
/// Possible values:
/// - `ipv4`: IPv4 address, format specified in IETF RFC 791.
/// - `ipv6`: IPv6 address, format specified in IETF RFC 2373.
/// - `infiniBand`: InfiniBand address family.
/// - `fc`: Fibre Channel address family.
/// - `loopback`: Intra-host transport.
/// - `unknown`: Unrecognized address family.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostNvmeTransportParametersNvmeAddressFamilyEnum {
    #[serde(rename = "ipv4")]
    #[strum(serialize = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    #[strum(serialize = "ipv6")]
    Ipv6,
    #[serde(rename = "infiniBand")]
    #[strum(serialize = "infiniBand")]
    InfiniBand,
    #[serde(rename = "fc")]
    #[strum(serialize = "fc")]
    Fc,
    #[serde(rename = "loopback")]
    #[strum(serialize = "loopback")]
    Loopback,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The set of NVM Express over Fabrics transport types.
/// 
/// For details, see:
/// - "NVM Express over Fabrics 1.0", Section 1.5.1,
///   "Fabrics and Transports".
///   
/// Possible values:
/// - `pcie`: PCI Express transport type
/// - `fibreChannel`: Fibre Channel transport type
/// - `rdma`: Remote Direct Memory Access transport type
/// - `tcp`: Transmission Control Protocol transport type
///   
///   ***Since:*** vSphere API Release 7.0.3.0
/// - `loopback`: Intra-host transport.
/// - `unsupported`: The transport type is not among the currently supported ones.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostNvmeTransportTypeEnum {
    #[serde(rename = "pcie")]
    #[strum(serialize = "pcie")]
    Pcie,
    #[serde(rename = "fibreChannel")]
    #[strum(serialize = "fibreChannel")]
    FibreChannel,
    #[serde(rename = "rdma")]
    #[strum(serialize = "rdma")]
    Rdma,
    #[serde(rename = "tcp")]
    #[strum(serialize = "tcp")]
    Tcp,
    #[serde(rename = "loopback")]
    #[strum(serialize = "loopback")]
    Loopback,
    #[serde(rename = "unsupported")]
    #[strum(serialize = "unsupported")]
    Unsupported,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `up`: The opaque switch is up and running.
/// - `warning`: The opaque switch requires attention.
/// - `down`: The opaque switch is down.
/// - `maintenance`: The opaque switch is under upgrade.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostOpaqueSwitchOpaqueSwitchStateEnum {
    #[serde(rename = "up")]
    #[strum(serialize = "up")]
    Up,
    #[serde(rename = "warning")]
    #[strum(serialize = "warning")]
    Warning,
    #[serde(rename = "down")]
    #[strum(serialize = "down")]
    Down,
    #[serde(rename = "maintenance")]
    #[strum(serialize = "maintenance")]
    Maintenance,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The installation state if the update is installed on the server.
/// 
/// Possible values:
/// - `hostRestarted`: The server has been restarted since the update installation.
/// - `imageActive`: Indicates if the newly installed image is active on the server
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostPatchManagerInstallStateEnum {
    #[serde(rename = "hostRestarted")]
    #[strum(serialize = "hostRestarted")]
    HostRestarted,
    #[serde(rename = "imageActive")]
    #[strum(serialize = "imageActive")]
    ImageActive,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The integrity validation status.
/// 
/// Possible values:
/// - `validated`: The update is successfully validated.
/// - `keyNotFound`: The integrity can not be verified since a public key to
///   verify the update cannot be found.
/// - `keyRevoked`: A public key to verify the update has been revoked.
/// - `keyExpired`: A public key to verify the update is expired.
/// - `digestMismatch`: A digital signature of the update does not match.
/// - `notEnoughSignatures`: Not enough signed signatures on the update.
/// - `validationError`: The integrity validation failed.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostPatchManagerIntegrityStatusEnum {
    #[serde(rename = "validated")]
    #[strum(serialize = "validated")]
    Validated,
    #[serde(rename = "keyNotFound")]
    #[strum(serialize = "keyNotFound")]
    KeyNotFound,
    #[serde(rename = "keyRevoked")]
    #[strum(serialize = "keyRevoked")]
    KeyRevoked,
    #[serde(rename = "keyExpired")]
    #[strum(serialize = "keyExpired")]
    KeyExpired,
    #[serde(rename = "digestMismatch")]
    #[strum(serialize = "digestMismatch")]
    DigestMismatch,
    #[serde(rename = "notEnoughSignatures")]
    #[strum(serialize = "notEnoughSignatures")]
    NotEnoughSignatures,
    #[serde(rename = "validationError")]
    #[strum(serialize = "validationError")]
    ValidationError,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Reasons why an update is not applicable to the ESX host.
/// 
/// Possible values:
/// - `obsoleted`: The update is made obsolete by other patches installed on the host.
/// - `missingPatch`: The update depends on another update that is neither installed
///   nor in the scanned list of updates.
/// - `missingLib`: The update depends on certain libraries or RPMs that are not
///   available.
/// - `hasDependentPatch`: The update depends on an update that is not installed but is
///   in the scanned list of updates.
/// - `conflictPatch`: The update conflicts with certain updates that are already
///   installed on the host.
/// - `conflictLib`: The update conflicts with RPMs or libraries installed on the
///   host.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostPatchManagerReasonEnum {
    #[serde(rename = "obsoleted")]
    #[strum(serialize = "obsoleted")]
    Obsoleted,
    #[serde(rename = "missingPatch")]
    #[strum(serialize = "missingPatch")]
    MissingPatch,
    #[serde(rename = "missingLib")]
    #[strum(serialize = "missingLib")]
    MissingLib,
    #[serde(rename = "hasDependentPatch")]
    #[strum(serialize = "hasDependentPatch")]
    HasDependentPatch,
    #[serde(rename = "conflictPatch")]
    #[strum(serialize = "conflictPatch")]
    ConflictPatch,
    #[serde(rename = "conflictLib")]
    #[strum(serialize = "conflictLib")]
    ConflictLib,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `userOptOut`: Indicates that the user has opted out the Physical NIC from resource pool
///   based scheduling.
/// - `hardwareUnsupported`: Indicates that the NIC device does is not capable of resource pool
///   based scheduling.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum PhysicalNicResourcePoolSchedulerDisallowedReasonEnum {
    #[serde(rename = "userOptOut")]
    #[strum(serialize = "userOptOut")]
    UserOptOut,
    #[serde(rename = "hardwareUnsupported")]
    #[strum(serialize = "hardwareUnsupported")]
    HardwareUnsupported,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for *PhysicalNic.vmDirectPathGen2SupportedMode*.
/// 
/// Possible values:
/// - `upt`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum PhysicalNicVmDirectPathGen2SupportedModeEnum {
    #[serde(rename = "upt")]
    #[strum(serialize = "upt")]
    Upt,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The type of component connected to a port group.
/// 
/// Possible values:
/// - `virtualMachine`: A virtual machine is connected to this port group.
/// - `systemManagement`: A system management entity (service console)
///   is connected to this port group.
/// - `host`: The VMkernel is connected to this port group.
/// - `unknown`: This port group serves an entity of unspecified kind.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum PortGroupConnecteeTypeEnum {
    #[serde(rename = "virtualMachine")]
    #[strum(serialize = "virtualMachine")]
    VirtualMachine,
    #[serde(rename = "systemManagement")]
    #[strum(serialize = "systemManagement")]
    SystemManagement,
    #[serde(rename = "host")]
    #[strum(serialize = "host")]
    Host,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated from all vmodl version above @released("6.0").
/// 
/// ProtocolEndpoint Type.
/// 
/// Possible values:
/// - `block`
/// - `nas`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostProtocolEndpointPeTypeEnum {
    #[serde(rename = "block")]
    #[strum(serialize = "block")]
    Block,
    #[serde(rename = "nas")]
    #[strum(serialize = "nas")]
    Nas,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// ProtocolEndpoint type.
/// 
/// Possible values:
/// - `scsi`
/// - `nfs`
/// - `nfs4x`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostProtocolEndpointProtocolEndpointTypeEnum {
    #[serde(rename = "scsi")]
    #[strum(serialize = "scsi")]
    Scsi,
    #[serde(rename = "nfs")]
    #[strum(serialize = "nfs")]
    Nfs,
    #[serde(rename = "nfs4x")]
    #[strum(serialize = "nfs4x")]
    Nfs4X,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// PTP capable network device type.
/// 
/// Possible values:
/// - `none`: No device.
/// - `virtualNic`: Virtual network adapter.
/// - `pciPassthruNic`: A network PCI device capable of PTP hardware timestamping,
///   enabled for passthru.
///   
///   See *HostPciPassthruSystem*
///   for information on PCI devices enabled for passthru available
///   on the host.
/// 
/// ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostPtpConfigDeviceTypeEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "virtualNic")]
    #[strum(serialize = "virtualNic")]
    VirtualNic,
    #[serde(rename = "pciPassthruNic")]
    #[strum(serialize = "pciPassthruNic")]
    PciPassthruNic,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `nvmeQualifiedName`: The NVMe Qualified Name (NQN) of this host.
/// - `vvolNvmeQualifiedName`: The NVMe Qualified Name (NQN) of this host used by Vvol.
///   
///   ***Since:*** vSphere API Release 8.0.0.0
/// 
/// ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostQualifiedNameTypeEnum {
    #[serde(rename = "nvmeQualifiedName")]
    #[strum(serialize = "nvmeQualifiedName")]
    NvmeQualifiedName,
    #[serde(rename = "vvolNvmeQualifiedName")]
    #[strum(serialize = "vvolNvmeQualifiedName")]
    VvolNvmeQualifiedName,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible RDMA device connection states.
/// 
/// These correspond
/// to possible link states as defined by the
/// Infiniband (TM) specification.
/// 
/// Further details can be found in:
/// - "Infiniband (TM) Architecture Specification, Volume 1"
///   section 7.2 "Link states"
///   
/// Possible values:
/// - `unknown`: Connection state unknown.
///   
///   Indicates that the driver returned
///   unexpected or no connection state information.
/// - `down`: Device down.
///   
///   Indicates that both the logical link and
///   underlying physical link are down. Packets
///   are discarded.
/// - `init`: Device initializing.
///   
///   Indicates that the physical link is up, but
///   the logical link is still initializing.
///   Only subnet management and flow control link
///   packets can be received and transmitted.
/// - `armed`: Device armed.
///   
///   Indicates that the physical link is up, but
///   the logical link is not yet fully configured.
///   Packets can be received, but non-SMPs
///   (subnet management packets) to be sent are discarded.
/// - `active`: Device active.
///   
///   Indicates that both the physical and logical
///   link are up. Packets can be transmitted and received.
/// - `activeDefer`: Device in active defer state.
///   
///   Indicates that the logical link was active, but the
///   physical link has suffered a failure. If it recovers
///   within a timeout, the connection state will return to active,
///   otherwise it will move to down.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostRdmaDeviceConnectionStateEnum {
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "down")]
    #[strum(serialize = "down")]
    Down,
    #[serde(rename = "init")]
    #[strum(serialize = "init")]
    Init,
    #[serde(rename = "armed")]
    #[strum(serialize = "armed")]
    Armed,
    #[serde(rename = "active")]
    #[strum(serialize = "active")]
    Active,
    #[serde(rename = "activeDefer")]
    #[strum(serialize = "activeDefer")]
    ActiveDefer,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enumeration of port directions.
/// 
/// Possible values:
/// - `inbound`
/// - `outbound`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostFirewallRuleDirectionEnum {
    #[serde(rename = "inbound")]
    #[strum(serialize = "inbound")]
    Inbound,
    #[serde(rename = "outbound")]
    #[strum(serialize = "outbound")]
    Outbound,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enumeration of port types.
/// 
/// Possible values:
/// - `src`
/// - `dst`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostFirewallRulePortTypeEnum {
    #[serde(rename = "src")]
    #[strum(serialize = "src")]
    Src,
    #[serde(rename = "dst")]
    #[strum(serialize = "dst")]
    Dst,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of valid port protocols.
/// 
/// Possible values:
/// - `tcp`
/// - `udp`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostFirewallRuleProtocolEnum {
    #[serde(rename = "tcp")]
    #[strum(serialize = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    #[strum(serialize = "udp")]
    Udp,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Define the instance state type
/// 
/// Possible values:
/// - `inactive`: The instance is deleted or not running
/// - `active`: The instance is running
/// - `deactivating`: The instance is in the progress of asynchronous deletion
/// - `activating`: Reserved state for future proofing asynchronous creation
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostRuntimeInfoNetStackInstanceRuntimeInfoStateEnum {
    #[serde(rename = "inactive")]
    #[strum(serialize = "inactive")]
    Inactive,
    #[serde(rename = "active")]
    #[strum(serialize = "active")]
    Active,
    #[serde(rename = "deactivating")]
    #[strum(serialize = "deactivating")]
    Deactivating,
    #[serde(rename = "activating")]
    #[strum(serialize = "activating")]
    Activating,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Valid protection modes for persistent state encryption.
/// 
/// Possible values:
/// - `none`: Encryption is not protected.
/// - `tpm`: Encryption is TPM protected.
///   
/// ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostRuntimeInfoStateEncryptionInfoProtectionModeEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "tpm")]
    #[strum(serialize = "tpm")]
    Tpm,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines if the host is ready for NVDS to VDS migration.
/// 
/// Possible values:
/// - `ready`: The host is ready for NVDS to VDS migration.
/// - `notNeeded`: The host does not need NVDS to VDS migration
/// - `unknown`: The host is disconnected from VC.
///   
/// ***Since:*** vSphere API Release 7.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostRuntimeInfoStatelessNvdsMigrationStateEnum {
    #[serde(rename = "ready")]
    #[strum(serialize = "ready")]
    Ready,
    #[serde(rename = "notNeeded")]
    #[strum(serialize = "notNeeded")]
    NotNeeded,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The types of disk drives.
/// 
/// Possible values:
/// - `native512`: 512 native sector size drive.
/// - `emulated512`: 4K sector size drive in 512 emulation mode.
/// - `native4k`: 4K native sector size drive.
/// - `SoftwareEmulated4k`: Software emulated 4k.
/// - `unknown`: Unknown type.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ScsiDiskTypeEnum {
    #[serde(rename = "native512")]
    #[strum(serialize = "native512")]
    Native512,
    #[serde(rename = "emulated512")]
    #[strum(serialize = "emulated512")]
    Emulated512,
    #[serde(rename = "native4k")]
    #[strum(serialize = "native4k")]
    Native4K,
    #[serde(rename = "SoftwareEmulated4k")]
    #[strum(serialize = "SoftwareEmulated4k")]
    SoftwareEmulated4K,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// An indicator of the utility of Descriptor in being used as an
/// identifier that is stable, unique, and correlatable.
/// 
/// Possible values:
/// - `highQuality`: The Descriptor has an identifier that is useful for identification
///   and correlation across hosts.
/// - `mediumQuality`: The Descriptor has an identifier that may be used for identification
///   and correlation across hosts.
/// - `lowQuality`: The Descriptor has an identifier that should not be used for
///   identification and correlation across hosts.
/// - `unknownQuality`: The Descriptor has an identifier that may or may not be useful for
///   identification and correlation across hosts.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ScsiLunDescriptorQualityEnum {
    #[serde(rename = "highQuality")]
    #[strum(serialize = "highQuality")]
    HighQuality,
    #[serde(rename = "mediumQuality")]
    #[strum(serialize = "mediumQuality")]
    MediumQuality,
    #[serde(rename = "lowQuality")]
    #[strum(serialize = "lowQuality")]
    LowQuality,
    #[serde(rename = "unknownQuality")]
    #[strum(serialize = "unknownQuality")]
    UnknownQuality,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The list of Device Protocols.
/// 
/// Device protocol could be either NVMe or SCSI
/// 
/// Possible values:
/// - `NVMe`
/// - `SCSI`
/// 
/// ***Since:*** vSphere API Release 8.0.1.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DeviceProtocolEnum {
    #[serde(rename = "NVMe")]
    #[strum(serialize = "NVMe")]
    NvMe,
    #[serde(rename = "SCSI")]
    #[strum(serialize = "SCSI")]
    Scsi,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The list of SCSI device types.
/// 
/// These values correspond to values
/// published in the SCSI specification.
/// 
/// Possible values:
/// - `disk`
/// - `tape`
/// - `printer`
/// - `processor`
/// - `worm`
/// - `cdrom`
/// - `scanner`
/// - `opticalDevice`
/// - `mediaChanger`
/// - `communications`
/// - `storageArrayController`
/// - `enclosure`
/// - `unknown`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ScsiLunTypeEnum {
    #[serde(rename = "disk")]
    #[strum(serialize = "disk")]
    Disk,
    #[serde(rename = "tape")]
    #[strum(serialize = "tape")]
    Tape,
    #[serde(rename = "printer")]
    #[strum(serialize = "printer")]
    Printer,
    #[serde(rename = "processor")]
    #[strum(serialize = "processor")]
    Processor,
    #[serde(rename = "worm")]
    #[strum(serialize = "worm")]
    Worm,
    #[serde(rename = "cdrom")]
    #[strum(serialize = "cdrom")]
    Cdrom,
    #[serde(rename = "scanner")]
    #[strum(serialize = "scanner")]
    Scanner,
    #[serde(rename = "opticalDevice")]
    #[strum(serialize = "opticalDevice")]
    OpticalDevice,
    #[serde(rename = "mediaChanger")]
    #[strum(serialize = "mediaChanger")]
    MediaChanger,
    #[serde(rename = "communications")]
    #[strum(serialize = "communications")]
    Communications,
    #[serde(rename = "storageArrayController")]
    #[strum(serialize = "storageArrayController")]
    StorageArrayController,
    #[serde(rename = "enclosure")]
    #[strum(serialize = "enclosure")]
    Enclosure,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The Operational state of the LUN
/// 
/// Possible values:
/// - `unknownState`: The LUN state is unknown.
/// - `ok`: The LUN is on and available.
/// - `error`: The LUN is dead and/or not reachable.
/// - `off`: The LUN is off.
/// - `quiesced`: The LUN is inactive.
/// - `degraded`: One or more paths to the LUN are down, but I/O
///   is still possible.
///   
///   Further path failures may
///   result in lost connectivity.
/// - `lostCommunication`: No more paths are available to the LUN.
/// - `timeout`: All Paths have been down for the timeout condition
///   determined by a user-configurable host advanced option.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ScsiLunStateEnum {
    #[serde(rename = "unknownState")]
    #[strum(serialize = "unknownState")]
    UnknownState,
    #[serde(rename = "ok")]
    #[strum(serialize = "ok")]
    Ok,
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    #[serde(rename = "off")]
    #[strum(serialize = "off")]
    Off,
    #[serde(rename = "quiesced")]
    #[strum(serialize = "quiesced")]
    Quiesced,
    #[serde(rename = "degraded")]
    #[strum(serialize = "degraded")]
    Degraded,
    #[serde(rename = "lostCommunication")]
    #[strum(serialize = "lostCommunication")]
    LostCommunication,
    #[serde(rename = "timeout")]
    #[strum(serialize = "timeout")]
    Timeout,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Storage array hardware acceleration support status.
/// 
/// When a host boots, the support status is unknown.
/// As a host attempts hardware-accelerated operations,
/// it determines whether the storage device supports hardware acceleration
/// and sets the *ScsiLun.vStorageSupport* property accordingly.
/// 
/// Possible values:
/// - `vStorageSupported`: Storage device supports hardware acceleration.
///   
///   The ESX host will use the feature to offload certain
///   storage-related operations to the device.
/// - `vStorageUnsupported`: Storage device does not support hardware acceleration.
///   
///   The ESX host will handle all storage-related operations.
/// - `vStorageUnknown`: Initial support status value.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ScsiLunVStorageSupportStatusEnum {
    #[serde(rename = "vStorageSupported")]
    #[strum(serialize = "vStorageSupported")]
    VStorageSupported,
    #[serde(rename = "vStorageUnsupported")]
    #[strum(serialize = "vStorageUnsupported")]
    VStorageUnsupported,
    #[serde(rename = "vStorageUnknown")]
    #[strum(serialize = "vStorageUnknown")]
    VStorageUnknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of valid service policy strings.
/// 
/// Possible values:
/// - `on`: Service should be started when the host starts up.
/// - `automatic`: Service should run if and only if it has open firewall ports.
/// - `off`: Service should not be started when the host starts up.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostServicePolicyEnum {
    #[serde(rename = "on")]
    #[strum(serialize = "on")]
    On,
    #[serde(rename = "automatic")]
    #[strum(serialize = "automatic")]
    Automatic,
    #[serde(rename = "off")]
    #[strum(serialize = "off")]
    Off,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `uninitialized`
/// - `initialized`
/// - `working`
/// 
/// ***Since:*** vSphere API Release 7.0.1.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostSevInfoSevStateEnum {
    #[serde(rename = "uninitialized")]
    #[strum(serialize = "uninitialized")]
    Uninitialized,
    #[serde(rename = "initialized")]
    #[strum(serialize = "initialized")]
    Initialized,
    #[serde(rename = "working")]
    #[strum(serialize = "working")]
    Working,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Flexible Launch Enclave (FLC) modes.
/// 
/// Possible values:
/// - `off`: Flexible Launch Enclave (FLC) is not available on the host.
///   
///   The
///   "launch enclave MSRs" are initialized with Intel's public key hash.
/// - `locked`: FLC is available and the "launch Enclave MSRs" are locked and
///   initialized with the provided public key hash.
/// - `unlocked`: FLC is available and the "launch enclave MSRs" are writeable and
///   initialized with Intel's public key hash.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostSgxInfoFlcModesEnum {
    #[serde(rename = "off")]
    #[strum(serialize = "off")]
    Off,
    #[serde(rename = "locked")]
    #[strum(serialize = "locked")]
    Locked,
    #[serde(rename = "unlocked")]
    #[strum(serialize = "unlocked")]
    Unlocked,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Host SGX states.
/// 
/// Possible values:
/// - `notPresent`: SGX is not present in the CPU.
/// - `disabledBIOS`: SGX is disabled in the BIOS.
/// - `disabledCFW101`: SGX is disabled because CPU erratum CFW101 is present.
/// - `disabledCPUMismatch`: SGX is disabled due to a mismatch in the SGX capabilities
///   exposed by different CPUs.
/// - `disabledNoFLC`: SGX is disabled because the CPU does not support FLC.
/// - `disabledNUMAUnsup`: SGX is disabled because the host uses NUMA, which is not
///   supported with SGX.
/// - `disabledMaxEPCRegs`: SGX is disabled because the host exceeds the maximum supported
///   number of EPC regions.
/// - `enabled`: SGX is enabled.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostSgxInfoSgxStatesEnum {
    #[serde(rename = "notPresent")]
    #[strum(serialize = "notPresent")]
    NotPresent,
    #[serde(rename = "disabledBIOS")]
    #[strum(serialize = "disabledBIOS")]
    DisabledBios,
    #[serde(rename = "disabledCFW101")]
    #[strum(serialize = "disabledCFW101")]
    DisabledCfw101,
    #[serde(rename = "disabledCPUMismatch")]
    #[strum(serialize = "disabledCPUMismatch")]
    DisabledCpuMismatch,
    #[serde(rename = "disabledNoFLC")]
    #[strum(serialize = "disabledNoFLC")]
    DisabledNoFlc,
    #[serde(rename = "disabledNUMAUnsup")]
    #[strum(serialize = "disabledNUMAUnsup")]
    DisabledNumaUnsup,
    #[serde(rename = "disabledMaxEPCRegs")]
    #[strum(serialize = "disabledMaxEPCRegs")]
    DisabledMaxEpcRegs,
    #[serde(rename = "enabled")]
    #[strum(serialize = "enabled")]
    Enabled,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// SGX registration status for ESX host.
/// 
/// Possible values:
/// - `notApplicable`: SGX is not available or the host is unisocket.
/// - `incomplete`: SGX registration is incomplete.
/// - `complete`: SGX registration is complete.
///   
/// ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostSgxRegistrationInfoRegistrationStatusEnum {
    #[serde(rename = "notApplicable")]
    #[strum(serialize = "notApplicable")]
    NotApplicable,
    #[serde(rename = "incomplete")]
    #[strum(serialize = "incomplete")]
    Incomplete,
    #[serde(rename = "complete")]
    #[strum(serialize = "complete")]
    Complete,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// SGX host registration type.
/// 
/// Possible values:
/// - `manifest`: Indicates that an Initial Platform Establishment
///   or TCB recovery registration is pending.
/// - `addPackage`: Indicates that new CPU package was added.
///   
/// ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostSgxRegistrationInfoRegistrationTypeEnum {
    #[serde(rename = "manifest")]
    #[strum(serialize = "manifest")]
    Manifest,
    #[serde(rename = "addPackage")]
    #[strum(serialize = "addPackage")]
    AddPackage,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// SNMP Agent supported capabilities enum
/// 
/// Possible values:
/// - `COMPLETE`: Implements test notifications and allows agent configuration
/// - `DIAGNOSTICS`: Implements only test notification capability only
/// - `CONFIGURATION`: Allows for agent configuration only
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostSnmpAgentCapabilityEnum {
    #[serde(rename = "COMPLETE")]
    #[strum(serialize = "COMPLETE")]
    Complete,
    #[serde(rename = "DIAGNOSTICS")]
    #[strum(serialize = "DIAGNOSTICS")]
    Diagnostics,
    #[serde(rename = "CONFIGURATION")]
    #[strum(serialize = "CONFIGURATION")]
    Configuration,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// These are the constraint relationships between software packages.
/// 
/// Possible values:
/// - `equals`
/// - `lessThan`
/// - `lessThanEqual`
/// - `greaterThanEqual`
/// - `greaterThan`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum SoftwarePackageConstraintEnum {
    #[serde(rename = "equals")]
    #[strum(serialize = "equals")]
    Equals,
    #[serde(rename = "lessThan")]
    #[strum(serialize = "lessThan")]
    LessThan,
    #[serde(rename = "lessThanEqual")]
    #[strum(serialize = "lessThanEqual")]
    LessThanEqual,
    #[serde(rename = "greaterThanEqual")]
    #[strum(serialize = "greaterThanEqual")]
    GreaterThanEqual,
    #[serde(rename = "greaterThan")]
    #[strum(serialize = "greaterThan")]
    GreaterThan,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `bootbank`: This package is installed into bootbank in storage.
/// - `tools`: This package is installed into tools partition in storage.
/// - `meta`: This package contains install related data without
///   content to install.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum SoftwarePackageVibTypeEnum {
    #[serde(rename = "bootbank")]
    #[strum(serialize = "bootbank")]
    Bootbank,
    #[serde(rename = "tools")]
    #[strum(serialize = "tools")]
    Tools,
    #[serde(rename = "meta")]
    #[strum(serialize = "meta")]
    Meta,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The set of supported host bus adapter protocols.
/// 
/// Possible values:
/// - `scsi`: The Small Computer System Interface (SCSI) protocol.
/// - `nvme`: The Non-Volatile Memory Express (NVME) protocol.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostStorageProtocolEnum {
    #[serde(rename = "scsi")]
    #[strum(serialize = "scsi")]
    Scsi,
    #[serde(rename = "nvme")]
    #[strum(serialize = "nvme")]
    Nvme,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `AssetTag`: The Asset tag of the system
/// - `ServiceTag`: The Service tag of the system
/// - `OemSpecificString`: OEM specific string
/// - `EnclosureSerialNumberTag`: The Enclosure Serial Number tag of the system
/// - `SerialNumberTag`: The Serial Number tag of the system
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostSystemIdentificationInfoIdentifierEnum {
    AssetTag,
    ServiceTag,
    OemSpecificString,
    EnclosureSerialNumberTag,
    SerialNumberTag,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Status constants of TPM attestation.
/// 
/// Possible values:
/// - `notAccepted`: TPM attestation failed.
/// - `accepted`: TPM attestation succeeded.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostTpmAttestationInfoAcceptanceStatusEnum {
    #[serde(rename = "notAccepted")]
    #[strum(serialize = "notAccepted")]
    NotAccepted,
    #[serde(rename = "accepted")]
    #[strum(serialize = "accepted")]
    Accepted,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `attested`: Attestation succeeded.
/// - `notAttested`: Attestation failed.
/// - `unknown`: Attestation status is unknown.
///   
/// ***Since:*** vSphere API Release 7.0.1.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostTrustAuthorityAttestationInfoAttestationStatusEnum {
    #[serde(rename = "attested")]
    #[strum(serialize = "attested")]
    Attested,
    #[serde(rename = "notAttested")]
    #[strum(serialize = "notAttested")]
    NotAttested,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Reasons for identifying the disk extent
/// as copy of VMFS volume extent.
/// 
/// Possible values:
/// - `diskIdMismatch`: The VMFS detected 'diskid' does not match with
///   LVM detected 'diskId'
/// - `uuidConflict`: VMFS 'uuid' does not match
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostUnresolvedVmfsExtentUnresolvedReasonEnum {
    #[serde(rename = "diskIdMismatch")]
    #[strum(serialize = "diskIdMismatch")]
    DiskIdMismatch,
    #[serde(rename = "uuidConflict")]
    #[strum(serialize = "uuidConflict")]
    UuidConflict,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `resignature`: Resignature the Unresolved VMFS volume.
///   
///   In the event the volume to be resignatured contains multiple
///   extents but only a single copy of each extent exists, only the
///   head extent needs to be specified.
/// - `forceMount`: Keep the original Uuid of the VMFS volume and mount it
///   
///   In the event the volume to be force mounted contains multiple
///   extents but only a single copy of each extent exists, only the
///   head extent needs to be specified.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostUnresolvedVmfsResolutionSpecVmfsUuidResolutionEnum {
    #[serde(rename = "resignature")]
    #[strum(serialize = "resignature")]
    Resignature,
    #[serde(rename = "forceMount")]
    #[strum(serialize = "forceMount")]
    ForceMount,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `vmotion`: The VirtualNic is used for VMotion.
/// - `faultToleranceLogging`: The VirtualNic is used for Fault Tolerance logging.
/// - `vSphereReplication`: The VirtualNic is used for vSphere Replication LWD traffic
///   (i.e From the primary host to the VR server).
/// - `vSphereReplicationNFC`: The VirtualNic is used for vSphere Replication NFC traffic (i.e.
///   
///   From
///   the VR server to the secondary host).
/// - `management`: The VirtualNic is used for management network traffic .
///   
///   This nicType is available only when the system does not
///   support service console adapters.
///   
///   See also *HostNetCapabilities.usesServiceConsoleNic*.
/// - `vsan`: The VirtualNic is used for Virtual SAN data traffic.
///   
///   To enable or disable a VirtualNic for VSAN networking,
///   use *HostVsanSystem.UpdateVsan_Task*.
///   
///   See also *HostVsanSystem*, *HostVsanSystem.UpdateVsan_Task*, *ComputeResource.ReconfigureComputeResource_Task*.
/// - `vSphereProvisioning`: The VirtualNic is used for vSphere provisioning NFC traffic
///   (i.e.
///   
///   the NFC traffic between ESX hosts as a part of a VC initiated
///   provisioning operations like cold-migrations, clones, storage vmotion
///   with snapshots etc).
/// - `vsanWitness`: The VirtualNic is used for Virtual SAN witness traffic.
///   
///   Witness traffic vmknic is required for Virtual SAN stretched cluster,
///   to help on communication between Virtual SAN data node and witness
///   node.
///   To enable or disable a VirtualNic for Virtual SAN networking,
///   use *HostVsanSystem.UpdateVsan_Task*.
///   
///   See also *HostVsanSystem*, *HostVsanSystem.UpdateVsan_Task*.
/// - `vSphereBackupNFC`: The VirtualNic is used for vSphere backup NFC traffic
///   (i.e.
///   
///   the NFC traffic between backup appliance and ESX hosts).
/// - `ptp`: The VirtualNic is used for Precision Time Protocol (PTP).
/// - `nvmeTcp`: The VirtualNic is used for NVMe over TCP traffic.
///   
///   ***Since:*** vSphere API Release 7.0.3.0
/// - `nvmeRdma`: The VirtualNic is used for NVMe over RDMA traffic.
///   
///   ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostVirtualNicManagerNicTypeEnum {
    #[serde(rename = "vmotion")]
    #[strum(serialize = "vmotion")]
    Vmotion,
    #[serde(rename = "faultToleranceLogging")]
    #[strum(serialize = "faultToleranceLogging")]
    FaultToleranceLogging,
    #[serde(rename = "vSphereReplication")]
    #[strum(serialize = "vSphereReplication")]
    VSphereReplication,
    #[serde(rename = "vSphereReplicationNFC")]
    #[strum(serialize = "vSphereReplicationNFC")]
    VSphereReplicationNfc,
    #[serde(rename = "management")]
    #[strum(serialize = "management")]
    Management,
    #[serde(rename = "vsan")]
    #[strum(serialize = "vsan")]
    Vsan,
    #[serde(rename = "vSphereProvisioning")]
    #[strum(serialize = "vSphereProvisioning")]
    VSphereProvisioning,
    #[serde(rename = "vsanWitness")]
    #[strum(serialize = "vsanWitness")]
    VsanWitness,
    #[serde(rename = "vSphereBackupNFC")]
    #[strum(serialize = "vSphereBackupNFC")]
    VSphereBackupNfc,
    #[serde(rename = "ptp")]
    #[strum(serialize = "ptp")]
    Ptp,
    #[serde(rename = "nvmeTcp")]
    #[strum(serialize = "nvmeTcp")]
    NvmeTcp,
    #[serde(rename = "nvmeRdma")]
    #[strum(serialize = "nvmeRdma")]
    NvmeRdma,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for mode field in AccessSpec.
/// 
/// Possible values:
/// - `grant`: Grant access to specified services in addition to existing services.
/// - `replace`: Replace existing services with specified services.
/// - `revoke`: Revoke the specified services.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostVmciAccessManagerModeEnum {
    #[serde(rename = "grant")]
    #[strum(serialize = "grant")]
    Grant,
    #[serde(rename = "replace")]
    #[strum(serialize = "replace")]
    Replace,
    #[serde(rename = "revoke")]
    #[strum(serialize = "revoke")]
    Revoke,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// VMFS unmap bandwidth policy.
/// 
/// VMFS unmap reclaims unused storage space.
/// This specifies the bandwidth policy option of unmaps.
/// 
/// Possible values:
/// - `fixed`: Unmap bandwidth is a fixed value.
/// - `dynamic`: Unmaps bandwidth is a dynamic value with lower and upper limits
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostVmfsVolumeUnmapBandwidthPolicyEnum {
    #[serde(rename = "fixed")]
    #[strum(serialize = "fixed")]
    Fixed,
    #[serde(rename = "dynamic")]
    #[strum(serialize = "dynamic")]
    Dynamic,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// VMFS unmap priority.
/// 
/// VMFS unmap reclaims unused storage space.
/// This specifies the processing rate of unmaps.
/// 
/// Possible values:
/// - `none`: Unmap is disabled.
/// - `low`: Unmaps are processed at low rate.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostVmfsVolumeUnmapPriorityEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "low")]
    #[strum(serialize = "low")]
    Low,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This specifies how an IP address was obtained for a given interface.
/// 
/// See RFC 4293 IpAddressOriginTC.
/// 
/// Possible values:
/// - `other`: Any other type of address configuration other than the below
///   mentioned ones will fall under this category.
///   
///   For e.g., automatic
///   address configuration for the link local address falls under
///   this type.
/// - `manual`: The address is configured manually.
///   
///   The term 'static' is a synonym.
/// - `dhcp`: The address is configured through dhcp.
/// - `linklayer`: The address is obtained through stateless autoconfiguration (autoconf).
///   
///   See RFC 4862, IPv6 Stateless Address Autoconfiguration.
/// - `random`: The address is chosen by the system at random
///   e.g., an IPv4 address within 169.254/16, or an RFC 3041 privacy address.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NetIpConfigInfoIpAddressOriginEnum {
    #[serde(rename = "other")]
    #[strum(serialize = "other")]
    Other,
    #[serde(rename = "manual")]
    #[strum(serialize = "manual")]
    Manual,
    #[serde(rename = "dhcp")]
    #[strum(serialize = "dhcp")]
    Dhcp,
    #[serde(rename = "linklayer")]
    #[strum(serialize = "linklayer")]
    Linklayer,
    #[serde(rename = "random")]
    #[strum(serialize = "random")]
    Random,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `preferred`: Indicates that this is a valid address.
/// - `deprecated`: Indicates that this is a valid but deprecated address
///   that should no longer be used as a source address.
/// - `invalid`: Indicates that this isn't a valid.
/// - `inaccessible`: Indicates that the address is not accessible because
///   interface is not operational.
/// - `unknown`: Indicates that the status cannot be determined.
/// - `tentative`: Indicates that the uniqueness of the
///   address on the link is presently being verified.
/// - `duplicate`: Indicates the address has been determined to be non-unique
///   on the link, this address will not be reachable.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NetIpConfigInfoIpAddressStatusEnum {
    #[serde(rename = "preferred")]
    #[strum(serialize = "preferred")]
    Preferred,
    #[serde(rename = "deprecated")]
    #[strum(serialize = "deprecated")]
    Deprecated,
    #[serde(rename = "invalid")]
    #[strum(serialize = "invalid")]
    Invalid,
    #[serde(rename = "inaccessible")]
    #[strum(serialize = "inaccessible")]
    Inaccessible,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "tentative")]
    #[strum(serialize = "tentative")]
    Tentative,
    #[serde(rename = "duplicate")]
    #[strum(serialize = "duplicate")]
    Duplicate,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// IP Stack keeps state on entries in IpNetToMedia table to perform
/// physical address lookups for IP addresses.
/// 
/// Here are the standard
/// states per @see RFC 4293 ipNetToMediaType.
/// 
/// Possible values:
/// - `other`: This implementation is reporting something other than
///   what states are listed below.
/// - `invalid`: The IP Stack has marked this entry as not useable.
/// - `dynamic`: This entry has been learned using ARP or NDP.
/// - `manual`: This entry was set manually.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NetIpStackInfoEntryTypeEnum {
    #[serde(rename = "other")]
    #[strum(serialize = "other")]
    Other,
    #[serde(rename = "invalid")]
    #[strum(serialize = "invalid")]
    Invalid,
    #[serde(rename = "dynamic")]
    #[strum(serialize = "dynamic")]
    Dynamic,
    #[serde(rename = "manual")]
    #[strum(serialize = "manual")]
    Manual,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The set of values used to determine ordering of default routers.
/// 
/// See RFC 4293 ipDefaultRouterPreference.
/// 
/// Possible values:
/// - `reserved`
/// - `low`
/// - `medium`
/// - `high`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NetIpStackInfoPreferenceEnum {
    #[serde(rename = "reserved")]
    #[strum(serialize = "reserved")]
    Reserved,
    #[serde(rename = "low")]
    #[strum(serialize = "low")]
    Low,
    #[serde(rename = "medium")]
    #[strum(serialize = "medium")]
    Medium,
    #[serde(rename = "high")]
    #[strum(serialize = "high")]
    High,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// NetBIOS configuration mode.
/// 
/// Possible values:
/// - `unknown`: Mode of NetBIOS is unknown.
/// - `enabled`: NetBIOS is enabled.
/// - `disabled`: NetBIOS is disabled.
/// - `enabledViaDHCP`: DHCP server decides whether or not to use NetBIOS.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum NetBiosConfigInfoModeEnum {
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "enabled")]
    #[strum(serialize = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "enabledViaDHCP")]
    #[strum(serialize = "enabledViaDHCP")]
    EnabledViaDhcp,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This list specifies the type of operation being performed on the array.
/// 
/// Possible values:
/// - `add`: indicates an addition to the array.
/// - `remove`: indicates the removal of an element in the
///   array.
///   
///   In this case the key field must contain the key of the element
///   to be removed.
/// - `edit`: indicates changes to an element in the array.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ArrayUpdateOperationEnum {
    #[serde(rename = "add")]
    #[strum(serialize = "add")]
    Add,
    #[serde(rename = "remove")]
    #[strum(serialize = "remove")]
    Remove,
    #[serde(rename = "edit")]
    #[strum(serialize = "edit")]
    Edit,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `compliant`: Entity is in Compliance
/// - `nonCompliant`: Entity is out of Compliance
/// - `unknown`: Compliance status of the entity is not known
/// - `running`: Compliance check on this host is running.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ComplianceResultStatusEnum {
    #[serde(rename = "compliant")]
    #[strum(serialize = "compliant")]
    Compliant,
    #[serde(rename = "nonCompliant")]
    #[strum(serialize = "nonCompliant")]
    NonCompliant,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "running")]
    #[strum(serialize = "running")]
    Running,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enumerates different operations supported for comparing
/// numerical values.
/// 
/// Possible values:
/// - `lessThan`
/// - `lessThanEqual`
/// - `equal`
/// - `notEqual`
/// - `greaterThanEqual`
/// - `greaterThan`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ProfileNumericComparatorEnum {
    #[serde(rename = "lessThan")]
    #[strum(serialize = "lessThan")]
    LessThan,
    #[serde(rename = "lessThanEqual")]
    #[strum(serialize = "lessThanEqual")]
    LessThanEqual,
    #[serde(rename = "equal")]
    #[strum(serialize = "equal")]
    Equal,
    #[serde(rename = "notEqual")]
    #[strum(serialize = "notEqual")]
    NotEqual,
    #[serde(rename = "greaterThanEqual")]
    #[strum(serialize = "greaterThanEqual")]
    GreaterThanEqual,
    #[serde(rename = "greaterThan")]
    #[strum(serialize = "greaterThan")]
    GreaterThan,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The relation type to be supported.
/// 
/// Possible values:
/// - `dynamic_relation`: The relation to a subprofile or a parameter.
/// - `extensible_relation`: The values from sources other than the parameter/profile or the static
///   value list are allowed.
/// - `localizable_relation`: The value list contains localization keys instead of values.
/// - `static_relation`: The relation is defined by static valid value list.
/// - `validation_relation`: The relation is defined for validation purpose.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ProfileParameterMetadataRelationTypeEnum {
    #[serde(rename = "dynamic_relation")]
    #[strum(serialize = "dynamic_relation")]
    DynamicRelation,
    #[serde(rename = "extensible_relation")]
    #[strum(serialize = "extensible_relation")]
    ExtensibleRelation,
    #[serde(rename = "localizable_relation")]
    #[strum(serialize = "localizable_relation")]
    LocalizableRelation,
    #[serde(rename = "static_relation")]
    #[strum(serialize = "static_relation")]
    StaticRelation,
    #[serde(rename = "validation_relation")]
    #[strum(serialize = "validation_relation")]
    ValidationRelation,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Type of services for which Profile can be requested for
/// 
/// Possible values:
/// - `DRS`: Distributed Resource Scheduling
/// - `HA`: High Availability
/// - `DPM`: Distributed Power Management
/// - `FT`: Fault tolerance
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ClusterProfileServiceTypeEnum {
    #[serde(rename = "DRS")]
    #[strum(serialize = "DRS")]
    Drs,
    #[serde(rename = "HA")]
    #[strum(serialize = "HA")]
    Ha,
    #[serde(rename = "DPM")]
    #[strum(serialize = "DPM")]
    Dpm,
    #[serde(rename = "FT")]
    #[strum(serialize = "FT")]
    Ft,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines the result status values for a
/// *HostProfile*.*HostProfile.ExecuteHostProfile*
/// operation.
/// 
/// The result data is contained in the
/// *ProfileExecuteResult* data object.
/// 
/// Possible values:
/// - `success`: Profile execution was successful.
///   
///   You can use the output configuration data
///   to apply the profile to a host.
/// - `needInput`: Additional data is required to complete the operation.
///   
///   The data requirements are defined in the list of policy options for the profile
///   (*ApplyProfile*.*ApplyProfile.policy*\[\]).
/// - `error`: Profile execution generated an error.
///   
///   See *ProfileExecuteResult*.*ProfileExecuteResult.error*.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ProfileExecuteResultStatusEnum {
    #[serde(rename = "success")]
    #[strum(serialize = "success")]
    Success,
    #[serde(rename = "needInput")]
    #[strum(serialize = "needInput")]
    NeedInput,
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Types of host profile update.
/// 
/// Possible values:
/// - `HostBased`: Update host profile from host.
/// - `Import`: Import host profile.
/// - `Edit`: Edit host profile.
/// - `Compose`: Compose setting from host profile.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostProfileValidationFailureInfoUpdateTypeEnum {
    HostBased,
    Import,
    Edit,
    Compose,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This defines validation state values for host profile.
/// 
/// Possible values:
/// - `Ready`
/// - `Running`
/// - `Failed`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostProfileValidationStateEnum {
    Ready,
    Running,
    Failed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The *HostProfileManagerAnswerFileStatus_enum* enum
/// defines possible values for answer file status.
/// 
/// Possible values:
/// - `valid`: Answer file is valid.
/// - `invalid`: Answer file is not valid.
///   
///   The file is either missing or incomplete.
///   - To produce an answer file, pass host-specific data (user input) to the
///     *HostProfileManager*.*HostProfileManager.ApplyHostConfig_Task*
///     method.
///   - To produce a complete answer file, call the
///     *HostProfile*.*HostProfile.ExecuteHostProfile*
///     method and fill in any missing parameters in the returned
///     *ProfileExecuteResult*.*ProfileExecuteResult.requireInput*
///     list. After you execute the profile successfully, you can pass the complete required
///     input list to the apply method.
/// - `unknown`: Answer file status is not known.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostProfileManagerAnswerFileStatusEnum {
    #[serde(rename = "valid")]
    #[strum(serialize = "valid")]
    Valid,
    #[serde(rename = "invalid")]
    #[strum(serialize = "invalid")]
    Invalid,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `success`: Remediation succeeded.
/// - `failed`: Remediation failed.
/// - `reboot_failed`: Remediation succeeded but reboot after remediation failed.
///   
///   May treat this as a warning.
/// - `stateless_reboot_failed`: Stateless reboot for remediation failed.
/// - `check_compliance_failed`: Remediation and reboot succeeded but check compliance after reboot
///   failed.
///   
///   May treat this as a warning.
/// - `state_not_satisfied`: The required state is not satisfied so host profiel apply cannot
///   be done.
/// - `exit_maintenancemode_failed`: Exit maintenance mode failed.
/// - `canceled`: The remediation was canceled.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ApplyHostProfileConfigurationResultStatusEnum {
    #[serde(rename = "success")]
    #[strum(serialize = "success")]
    Success,
    #[serde(rename = "failed")]
    #[strum(serialize = "failed")]
    Failed,
    #[serde(rename = "reboot_failed")]
    #[strum(serialize = "reboot_failed")]
    RebootFailed,
    #[serde(rename = "stateless_reboot_failed")]
    #[strum(serialize = "stateless_reboot_failed")]
    StatelessRebootFailed,
    #[serde(rename = "check_compliance_failed")]
    #[strum(serialize = "check_compliance_failed")]
    CheckComplianceFailed,
    #[serde(rename = "state_not_satisfied")]
    #[strum(serialize = "state_not_satisfied")]
    StateNotSatisfied,
    #[serde(rename = "exit_maintenancemode_failed")]
    #[strum(serialize = "exit_maintenancemode_failed")]
    ExitMaintenancemodeFailed,
    #[serde(rename = "canceled")]
    #[strum(serialize = "canceled")]
    Canceled,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The composition status class.
/// 
/// Possible values:
/// - `success`
/// - `error`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostProfileManagerCompositionResultResultElementStatusEnum {
    #[serde(rename = "success")]
    #[strum(serialize = "success")]
    Success,
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The composition validation status class.
/// 
/// Possible values:
/// - `success`
/// - `error`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostProfileManagerCompositionValidationResultResultElementStatusEnum {
    #[serde(rename = "success")]
    #[strum(serialize = "success")]
    Success,
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The *HostProfileManagerTaskListRequirement_enum* enum
/// defines possible values for requirements when applying a *HostConfigSpec*
/// object returned as part of a <code>generateConfigTaskList</code>
/// operation.
/// 
/// Possible values:
/// - `maintenanceModeRequired`: The ESXi host must be in maintenance mode before the task list can be
///   applied.
/// - `rebootRequired`: The ESXi host must be rebooted after the task list is applied in order
///   for the new settings in the *HostConfigSpec* to take
///   effect on the host.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum HostProfileManagerTaskListRequirementEnum {
    #[serde(rename = "maintenanceModeRequired")]
    #[strum(serialize = "maintenanceModeRequired")]
    MaintenanceModeRequired,
    #[serde(rename = "rebootRequired")]
    #[strum(serialize = "rebootRequired")]
    RebootRequired,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines the result status values for a validating answer file.
/// 
/// Possible values:
/// - `success`: Answer File validation was successful.
/// - `failed`: Answer File validation failed.
/// - `failed_defaults`: Answer File validation failed to generate default.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum AnswerFileValidationInfoStatusEnum {
    #[serde(rename = "success")]
    #[strum(serialize = "success")]
    Success,
    #[serde(rename = "failed")]
    #[strum(serialize = "failed")]
    Failed,
    #[serde(rename = "failed_defaults")]
    #[strum(serialize = "failed_defaults")]
    FailedDefaults,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `sunday`
/// - `monday`
/// - `tuesday`
/// - `wednesday`
/// - `thursday`
/// - `friday`
/// - `saturday`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum DayOfWeekEnum {
    #[serde(rename = "sunday")]
    #[strum(serialize = "sunday")]
    Sunday,
    #[serde(rename = "monday")]
    #[strum(serialize = "monday")]
    Monday,
    #[serde(rename = "tuesday")]
    #[strum(serialize = "tuesday")]
    Tuesday,
    #[serde(rename = "wednesday")]
    #[strum(serialize = "wednesday")]
    Wednesday,
    #[serde(rename = "thursday")]
    #[strum(serialize = "thursday")]
    Thursday,
    #[serde(rename = "friday")]
    #[strum(serialize = "friday")]
    Friday,
    #[serde(rename = "saturday")]
    #[strum(serialize = "saturday")]
    Saturday,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `first`
/// - `second`
/// - `third`
/// - `fourth`
/// - `last`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum WeekOfMonthEnum {
    #[serde(rename = "first")]
    #[strum(serialize = "first")]
    First,
    #[serde(rename = "second")]
    #[strum(serialize = "second")]
    Second,
    #[serde(rename = "third")]
    #[strum(serialize = "third")]
    Third,
    #[serde(rename = "fourth")]
    #[strum(serialize = "fourth")]
    Fourth,
    #[serde(rename = "last")]
    #[strum(serialize = "last")]
    Last,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Rule scope determines conditions when an affinity rule is
/// satisfied.
/// 
/// The following uses affinity rule as example.
/// cluster: All Vms in the rule list are placed in a single cluster.
/// host: All Vms in the rule list are placed in a single host.
/// storagePod: All Vms in the rule list are placed in a single storagePod.
/// datastore: All Vms in the rule list are placed in a single datastore.
/// 
/// Possible values:
/// - `cluster`: clusters are the scope
/// - `host`: individual hosts are the scope
/// - `storagePod`: datastore cluster is teh scope
/// - `datastore`: individual datastores are the scope
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum PlacementAffinityRuleRuleScopeEnum {
    #[serde(rename = "cluster")]
    #[strum(serialize = "cluster")]
    Cluster,
    #[serde(rename = "host")]
    #[strum(serialize = "host")]
    Host,
    #[serde(rename = "storagePod")]
    #[strum(serialize = "storagePod")]
    StoragePod,
    #[serde(rename = "datastore")]
    #[strum(serialize = "datastore")]
    Datastore,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Rule type determines how the affinity rule is to be enforced:
/// affinity: Vms in the list are kept together within the rule
/// scope.
/// 
/// anti-affinity: Vms in the rule list are kept separate
/// across the objects in the rule scope.
/// soft rule: The enforcement is best effort.
/// 
/// Possible values:
/// - `affinity`: Affinity
/// - `antiAffinity`: Anti-Affinity
/// - `softAffinity`: Best-effort affinity
/// - `softAntiAffinity`: Best-effort anti-affinity
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum PlacementAffinityRuleRuleTypeEnum {
    #[serde(rename = "affinity")]
    #[strum(serialize = "affinity")]
    Affinity,
    #[serde(rename = "antiAffinity")]
    #[strum(serialize = "antiAffinity")]
    AntiAffinity,
    #[serde(rename = "softAffinity")]
    #[strum(serialize = "softAffinity")]
    SoftAffinity,
    #[serde(rename = "softAntiAffinity")]
    #[strum(serialize = "softAntiAffinity")]
    SoftAntiAffinity,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Storage DRS behavior.
/// 
/// Possible values:
/// - `manual`: Specifies that VirtualCenter should generate recommendations for
///   virtual disk migration and for placement with a datastore,
///   but should not execute the recommendations automatically.
/// - `automated`: Specifies that VirtualCenter should generate recommendations
///   for virtual disk migration and for placement with a
///   datastore.
///   
///   The recommendations for virtual disk migrations
///   will be executed automatically, but the placement
///   recommendations will be done manually.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum StorageDrsPodConfigInfoBehaviorEnum {
    #[serde(rename = "manual")]
    #[strum(serialize = "manual")]
    Manual,
    #[serde(rename = "automated")]
    #[strum(serialize = "automated")]
    Automated,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines the two ways a space utilization threshold can be specified.
/// 
/// Possible values:
/// - `utilization`: Default mode: threshold as a percentage of datastore capacity
/// - `freeSpace`: Threshold as an absolute value of free space in GBs
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum StorageDrsSpaceLoadBalanceConfigSpaceThresholdModeEnum {
    #[serde(rename = "utilization")]
    #[strum(serialize = "utilization")]
    Utilization,
    #[serde(rename = "freeSpace")]
    #[strum(serialize = "freeSpace")]
    FreeSpace,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Defines the storage placement operation type.
/// 
/// Possible values:
/// - `create`: Create a VM.
/// - `reconfigure`: Reconfigure a VM.
/// - `relocate`: Relocate a VM.
/// - `clone`: Clone a VM.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum StoragePlacementSpecPlacementTypeEnum {
    #[serde(rename = "create")]
    #[strum(serialize = "create")]
    Create,
    #[serde(rename = "reconfigure")]
    #[strum(serialize = "reconfigure")]
    Reconfigure,
    #[serde(rename = "relocate")]
    #[strum(serialize = "relocate")]
    Relocate,
    #[serde(rename = "clone")]
    #[strum(serialize = "clone")]
    Clone,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Rule type determines how the virtual disks in a vm can be grouped
/// together.
/// 
/// Possible values:
/// - `affinity`: Virtual disks in the list are grouped together and placed on
///   the same data store.
/// - `antiAffinity`: Virtual disks in the list are placed on different data stores.
/// - `disabled`: SDRS will be disabled for the disks in the list.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDiskRuleSpecRuleTypeEnum {
    #[serde(rename = "affinity")]
    #[strum(serialize = "affinity")]
    Affinity,
    #[serde(rename = "antiAffinity")]
    #[strum(serialize = "antiAffinity")]
    AntiAffinity,
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The cloned VMs can either be provisioned the same way as the VMs
/// they are a clone of, thin provisioned or thick provisioned, or
/// linked clones (i.e., using delta disks).
/// 
/// Possible values:
/// - `sameAsSource`: Each disk in the cloned virtual machines will have the same
///   type of disk as the source vApp.
/// - `thin`: Each disk in the cloned virtual machines is allocated in full
///   size now and committed on demand.
///   
///   This is only supported on
///   VMFS-3 and newer datastores. Other types of datastores may
///   create thick disks.
/// - `thick`: Each disk in the cloned virtual machines are allocated and
///   committed in full size immediately.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VAppCloneSpecProvisioningTypeEnum {
    #[serde(rename = "sameAsSource")]
    #[strum(serialize = "sameAsSource")]
    SameAsSource,
    #[serde(rename = "thin")]
    #[strum(serialize = "thin")]
    Thin,
    #[serde(rename = "thick")]
    #[strum(serialize = "thick")]
    Thick,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `none`: No action is taken for this virtual machine.
///   
///   This virtual machine is
///   not a part of the auto-start sequence. This can be used for both auto-start
///   and auto-start settings.
/// - `powerOn`: This virtual machine is powered on when it is next in the auto-start order.
/// - `powerOff`: This virtual machine is powered off when it is next in the auto-stop order.
///   
///   This is the default stopAction.
/// - `guestShutdown`: The guest operating system for a virtual machine is shut down when that
///   virtual machine in next in the auto-stop order.
/// - `suspend`: This virtual machine is suspended when it is next in the auto-stop order.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VAppAutoStartActionEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "powerOn")]
    #[strum(serialize = "powerOn")]
    PowerOn,
    #[serde(rename = "powerOff")]
    #[strum(serialize = "powerOff")]
    PowerOff,
    #[serde(rename = "guestShutdown")]
    #[strum(serialize = "guestShutdown")]
    GuestShutdown,
    #[serde(rename = "suspend")]
    #[strum(serialize = "suspend")]
    Suspend,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// IP allocation schemes supported by the guest.
/// 
/// Possible values:
/// - `dhcp`: The vApp supports DHCP to acquire IP configuration.
/// - `ovfenv`: The vApp supports setting the IP configuration through the
///   properties provided in the OVF environment.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VAppIpAssignmentInfoAllocationSchemesEnum {
    #[serde(rename = "dhcp")]
    #[strum(serialize = "dhcp")]
    Dhcp,
    #[serde(rename = "ovfenv")]
    #[strum(serialize = "ovfenv")]
    Ovfenv,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// IP allocation policy for a deployment.
/// 
/// Possible values:
/// - `dhcpPolicy`: Specifies that DHCP must be used to allocate IP addresses to the vApp
/// - `transientPolicy`: Specifies that IP allocation is done through the range managed by the
///   vSphere platform.
///   
///   The IP addresses are allocated when needed, typically at
///   power-on, and deallocated during power-off. There is no guarantee that a
///   vApp will get the same IP address when restarted.
/// - `fixedPolicy`: Specifies that IP addresses are configured manually when the vApp is deployed
///   and will be kept until reconfigured or the vApp destroyed.
///   
///   This will ensure
///   that a vApp gets a consistent IP for its life-time.
/// - `fixedAllocatedPolicy`: Specifies that IP allocation is done through the range managed by the VI
///   platform.
///   
///   The IP addresses are allocated at first power-on, and remain
///   allocated at power-off. This will ensure that a vApp gets a consistent
///   IP for its life-time.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VAppIpAssignmentInfoIpAllocationPolicyEnum {
    #[serde(rename = "dhcpPolicy")]
    #[strum(serialize = "dhcpPolicy")]
    DhcpPolicy,
    #[serde(rename = "transientPolicy")]
    #[strum(serialize = "transientPolicy")]
    TransientPolicy,
    #[serde(rename = "fixedPolicy")]
    #[strum(serialize = "fixedPolicy")]
    FixedPolicy,
    #[serde(rename = "fixedAllocatedPolicy")]
    #[strum(serialize = "fixedAllocatedPolicy")]
    FixedAllocatedPolicy,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// IP protocols supported by the guest.
/// 
/// Possible values:
/// - `IPv4`: The vApp supports IPv4 protocol.
/// - `IPv6`: The vApp supports IPv6 protocol.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VAppIpAssignmentInfoProtocolsEnum {
    IPv4,
    IPv6,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `configured`: VCHA cluster is configured.
/// - `notConfigured`: VCHA cluster is not configured.
/// - `invalid`: VCHA cluster is in an invalid/dirty state.
/// - `prepared`: VC appliance has been prepared for VCHA cluster configuration.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VchaStateEnum {
    #[serde(rename = "configured")]
    #[strum(serialize = "configured")]
    Configured,
    #[serde(rename = "notConfigured")]
    #[strum(serialize = "notConfigured")]
    NotConfigured,
    #[serde(rename = "invalid")]
    #[strum(serialize = "invalid")]
    Invalid,
    #[serde(rename = "prepared")]
    #[strum(serialize = "prepared")]
    Prepared,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// VchaClusterMode enum defines the possible modes for a VCHA Cluster.
/// 
/// Possible values:
/// - `enabled`: VCHA Cluster is enabled.
///   
///   State replication between the Active and
///   Passive node is enabled and automatic failover is allowed.
/// - `disabled`: VCHA Cluster is disabled.
///   
///   State replication between the Active and
///   Passive node is disabled and automatic failover is not allowed.
/// - `maintenance`: VCHA Cluster is in maintenance mode.
///   
///   State replication between the
///   Active and Passive node is enabled but automatic failover
///   is not allowed.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VchaClusterModeEnum {
    #[serde(rename = "enabled")]
    #[strum(serialize = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "maintenance")]
    #[strum(serialize = "maintenance")]
    Maintenance,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// VchaClusterState enum defines the possible states for a VCHA Cluster.
/// 
/// Possible values:
/// - `healthy`: All three nodes in a VCHA Cluster are healthy and connected.
///   
///   State
///   replication between Active and Passive node is working and both
///   nodes are in sync.
/// - `degraded`: A VCHA Cluster is said to be in a degraded state for
///   either or all of the following reasons:
///   \- There is a node loss.
///   
///   \- State replication between the Active and Passive node fails.
/// - `isolated`: All three nodes are isolated from each other.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VchaClusterStateEnum {
    #[serde(rename = "healthy")]
    #[strum(serialize = "healthy")]
    Healthy,
    #[serde(rename = "degraded")]
    #[strum(serialize = "degraded")]
    Degraded,
    #[serde(rename = "isolated")]
    #[strum(serialize = "isolated")]
    Isolated,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `active`: Node is having a role of Active.
///   
///   In this role, node runs a vCenter
///   Server that serves client requests.
/// - `passive`: Node is having a role of Passive.
///   
///   In this role node, runs as a standby
///   for the Active vCenter Server and receives state updates. This node
///   takes over the role of Active vCenter Server upon failover.
/// - `witness`: Node is having a role of Witness.
///   
///   In this role, node acts as a quorom
///   node for avoiding the classic split-brain problem.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VchaNodeRoleEnum {
    #[serde(rename = "active")]
    #[strum(serialize = "active")]
    Active,
    #[serde(rename = "passive")]
    #[strum(serialize = "passive")]
    Passive,
    #[serde(rename = "witness")]
    #[strum(serialize = "witness")]
    Witness,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// VchaNodeState enum defines possible state a node can be in a
/// VCHA Cluster.
/// 
/// Possible values:
/// - `up`: Node is up and has joined the VCHA Cluster.
/// - `down`: Node is down and has left the VCHA Cluster.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VchaNodeStateEnum {
    #[serde(rename = "up")]
    #[strum(serialize = "up")]
    Up,
    #[serde(rename = "down")]
    #[strum(serialize = "down")]
    Down,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `ipv4`: PXE (or Apple NetBoot) over IPv4.
///   
///   The default.
/// - `ipv6`: PXE over IPv6.
///   
///   Only meaningful for EFI virtual machines.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineBootOptionsNetworkBootProtocolTypeEnum {
    #[serde(rename = "ipv4")]
    #[strum(serialize = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    #[strum(serialize = "ipv6")]
    Ipv6,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of supported hash algorithms for thumbprints.
/// 
/// Possible values:
/// - `sha256`: SHA256
///   
/// ***Since:*** vSphere API Release 7.0.3.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineCertThumbprintHashAlgorithmEnum {
    #[serde(rename = "sha256")]
    #[strum(serialize = "sha256")]
    Sha256,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// TPM provisioning policies used when cloning a VM with a virtual TPM
/// device.
/// 
/// Possible values:
/// - `copy`: The virtual TPM is copied.
///   
///   The virtual machine clone will have access
///   to the original virtual machine's TPM secrets.
/// - `replace`: The virtual TPM is replaced with a new one.
///   
///   The virtual machine clone
///   will not have access to the original virtual machine's TPM secrets.
/// 
/// ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineCloneSpecTpmProvisionPolicyEnum {
    #[serde(rename = "copy")]
    #[strum(serialize = "copy")]
    Copy,
    #[serde(rename = "replace")]
    #[strum(serialize = "replace")]
    Replace,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The NPIV WWN source type.
/// 
/// Possible values:
/// - `vc`: This set of WWNs is generated by VC server.
/// - `host`: This set of WWNs is generated by Host Agent.
/// - `external`: This set of WWNs is provided by the client.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineConfigInfoNpivWwnTypeEnum {
    #[serde(rename = "vc")]
    #[strum(serialize = "vc")]
    Vc,
    #[serde(rename = "host")]
    #[strum(serialize = "host")]
    Host,
    #[serde(rename = "external")]
    #[strum(serialize = "external")]
    External,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Available choices for virtual machine swapfile placement policy.
/// 
/// This is
/// the set of legal values for the virtual machine configuration's
/// *swapPlacement* property. All
/// values except for "inherit" and "vmConfigured" are also valid values for
/// a compute resource configuration's
/// *vmSwapPlacement*
/// property.
/// 
/// Possible values:
/// - `inherit`: Honor the virtual machine swapfile placement policy of the compute
///   resource that contains this virtual machine.
/// - `vmDirectory`: Store the swapfile in the same directory as the virtual machine.
/// - `hostLocal`: Store the swapfile in the datastore specified by the
///   *localSwapDatastore*
///   property of the virtual machine's host, if that property is set and
///   indicates a datastore with sufficient free space.
///   
///   Otherwise store the
///   swapfile in the same directory as the virtual machine.
///   
///   Note: This setting may degrade VMotion performance.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineConfigInfoSwapPlacementTypeEnum {
    #[serde(rename = "inherit")]
    #[strum(serialize = "inherit")]
    Inherit,
    #[serde(rename = "vmDirectory")]
    #[strum(serialize = "vmDirectory")]
    VmDirectory,
    #[serde(rename = "hostLocal")]
    #[strum(serialize = "hostLocal")]
    HostLocal,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The set of valid encrypted Fault Tolerance modes for a VM.
/// 
/// If the VM is encrypted, its encrypted Fault Tolerance mode
/// will be required.
/// 
/// Possible values:
/// - `ftEncryptionDisabled`: Do not use encrypted Fault Tolerance, even if available.
/// - `ftEncryptionOpportunistic`: Use encrypted Fault Tolerance if source and destination hosts
///   support it, fall back to unencrypted Fault Tolerance otherwise.
///   
///   This is the default option.
/// - `ftEncryptionRequired`: Allow only encrypted Fault Tolerance.
///   
///   If either the source or
///   destination host does not support encrypted Fault Tolerance,
///   do not allow the Fault Tolerance to occur.
/// 
/// ***Since:*** vSphere API Release 7.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineConfigSpecEncryptedFtModesEnum {
    #[serde(rename = "ftEncryptionDisabled")]
    #[strum(serialize = "ftEncryptionDisabled")]
    FtEncryptionDisabled,
    #[serde(rename = "ftEncryptionOpportunistic")]
    #[strum(serialize = "ftEncryptionOpportunistic")]
    FtEncryptionOpportunistic,
    #[serde(rename = "ftEncryptionRequired")]
    #[strum(serialize = "ftEncryptionRequired")]
    FtEncryptionRequired,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The set of valid encrypted vMotion modes for a VM.
/// 
/// If the VM is encrypted, its encrypted vMotion mode will be required.
/// 
/// Possible values:
/// - `disabled`: Do not use encrypted vMotion, even if available.
/// - `opportunistic`: Use encrypted vMotion if source and destination hosts support it,
///   fall back to unencrypted vMotion otherwise.
///   
///   This is the default option.
/// - `required`: Allow only encrypted vMotion.
///   
///   If the source or destination host does
///   not support vMotion encryption, do not allow the vMotion to occur.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineConfigSpecEncryptedVMotionModesEnum {
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "opportunistic")]
    #[strum(serialize = "opportunistic")]
    Opportunistic,
    #[serde(rename = "required")]
    #[strum(serialize = "required")]
    Required,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The root WWN operation mode.
/// 
/// Possible values:
/// - `generate`: Generate a new set of WWNs and assign it to the virtual machine.
/// - `set`: Take a client-specified set of WWNs (specified in "wwn" property) and
///   assign them to the virtual machine.
///   
///   If the new WWN quntity are more
///   than existing then we will append them to the existing list of WWNs.
/// - `remove`: Remove the currently assigned WWNs from the virtual machine.
/// - `extend`: Generate a new set of WWNs and append them to the existing list
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineConfigSpecNpivWwnOpEnum {
    #[serde(rename = "generate")]
    #[strum(serialize = "generate")]
    Generate,
    #[serde(rename = "set")]
    #[strum(serialize = "set")]
    Set,
    #[serde(rename = "remove")]
    #[strum(serialize = "remove")]
    Remove,
    #[serde(rename = "extend")]
    #[strum(serialize = "extend")]
    Extend,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The list of possible default power operations available for the virtual machine
/// 
/// Possible values:
/// - `soft`
/// - `hard`
/// - `preset`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachinePowerOpTypeEnum {
    #[serde(rename = "soft")]
    #[strum(serialize = "soft")]
    Soft,
    #[serde(rename = "hard")]
    #[strum(serialize = "hard")]
    Hard,
    #[serde(rename = "preset")]
    #[strum(serialize = "preset")]
    Preset,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The list of possible standby actions that the virtual machine can take
/// for S1 ACPI.
/// 
/// Possible values:
/// - `checkpoint`
/// - `powerOnSuspend`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineStandbyActionTypeEnum {
    #[serde(rename = "checkpoint")]
    #[strum(serialize = "checkpoint")]
    Checkpoint,
    #[serde(rename = "powerOnSuspend")]
    #[strum(serialize = "powerOnSuspend")]
    PowerOnSuspend,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `vmNptIncompatibleHost`: The virtual machine's host does not support VMDirectPath Gen 2.
///   
///   See also *HostCapability.vmDirectPathGen2Supported*.
/// - `vmNptIncompatibleNetwork`: The configuration or state of the attached network prevents
///   VMDirectPath Gen 2.
///   
///   Refer to
///   *vmDirectPathGen2InactiveReasonNetwork*
///   and/or
///   *vmDirectPathGen2InactiveReasonExtended*
///   in the RuntimeInfo of the DistributedVirtualPort connected to this
///   device.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeStateVmDirectPathGen2InactiveReasonOtherEnum {
    #[serde(rename = "vmNptIncompatibleHost")]
    #[strum(serialize = "vmNptIncompatibleHost")]
    VmNptIncompatibleHost,
    #[serde(rename = "vmNptIncompatibleNetwork")]
    #[strum(serialize = "vmNptIncompatibleNetwork")]
    VmNptIncompatibleNetwork,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `vmNptIncompatibleGuest`: The virtual machine's guest OS does not support
///   VMDirectPath Gen 2.
/// - `vmNptIncompatibleGuestDriver`: The virtual machine's guest network driver does not support
///   VMDirectPath Gen 2.
/// - `vmNptIncompatibleAdapterType`: The device type does not support VMDirectPath Gen 2.
///   
///   See also *VirtualEthernetCardOption.vmDirectPathGen2Supported*.
/// - `vmNptDisabledOrDisconnectedAdapter`: The virtual machine's network adapter is disabled or
///   disconnected, and thus is not participating in VMDirectPath Gen 2.
/// - `vmNptIncompatibleAdapterFeatures`: The virtual machine's network adapter has features enabled
///   which preclude it participating in VMDirectPath Gen 2 such
///   as INT-x or PXE booting.
/// - `vmNptIncompatibleBackingType`: The device backing is not a DistributedVirtualPortBacking.
/// - `vmNptInsufficientMemoryReservation`: The virtual machine does not have full memory reservation
///   required to activate VMDirectPath Gen 2.
/// - `vmNptFaultToleranceOrRecordReplayConfigured`: 
///   
///   Deprecated as of vSphere API 6.0.
///   
///   The virtual machine is configured for Fault Tolerance or
///   Record &amp; Replay, which prevents VMDirectPath Gen 2.
/// - `vmNptConflictingIOChainConfigured`: Some networking feature has placed a conflicting IOChain on
///   the network adapter, which prevents VMDirectPath Gen 2.
///   
///   Examples
///   include DVFilter.
/// - `vmNptMonitorBlocks`: The virtual machine monitor is exercising functionality which
///   which prevents VMDirectPath Gen 2.
/// - `vmNptConflictingOperationInProgress`: VMDirectPath Gen 2 is temporarily suspended while the virtual
///   machine executes an operation such as suspend.
/// - `vmNptRuntimeError`: VMDirectPath Gen 2 is unavailable due to an unforeseen runtime error
///   in the virtualization platform (typically resource constraints.)
/// - `vmNptOutOfIntrVector`: VMDirectPath Gen 2 is unavailable due to host run out of intr
///   vector in host.
///   
///   Guest can configure the vNIC to use less rx/tx
///   queues or use MSI instead of MSIX.
/// - `vmNptVMCIActive`: VMDirectPath Gen 2 is unavailable due to Incompatibe feature
///   VMCI is active in the current VM.
///   
///   Kill the relevant VMCI
///   application(s) and restart the VM will allow the vNIC(s) to enter
///   passthrough mode.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeStateVmDirectPathGen2InactiveReasonVmEnum {
    #[serde(rename = "vmNptIncompatibleGuest")]
    #[strum(serialize = "vmNptIncompatibleGuest")]
    VmNptIncompatibleGuest,
    #[serde(rename = "vmNptIncompatibleGuestDriver")]
    #[strum(serialize = "vmNptIncompatibleGuestDriver")]
    VmNptIncompatibleGuestDriver,
    #[serde(rename = "vmNptIncompatibleAdapterType")]
    #[strum(serialize = "vmNptIncompatibleAdapterType")]
    VmNptIncompatibleAdapterType,
    #[serde(rename = "vmNptDisabledOrDisconnectedAdapter")]
    #[strum(serialize = "vmNptDisabledOrDisconnectedAdapter")]
    VmNptDisabledOrDisconnectedAdapter,
    #[serde(rename = "vmNptIncompatibleAdapterFeatures")]
    #[strum(serialize = "vmNptIncompatibleAdapterFeatures")]
    VmNptIncompatibleAdapterFeatures,
    #[serde(rename = "vmNptIncompatibleBackingType")]
    #[strum(serialize = "vmNptIncompatibleBackingType")]
    VmNptIncompatibleBackingType,
    #[serde(rename = "vmNptInsufficientMemoryReservation")]
    #[strum(serialize = "vmNptInsufficientMemoryReservation")]
    VmNptInsufficientMemoryReservation,
    #[serde(rename = "vmNptFaultToleranceOrRecordReplayConfigured")]
    #[strum(serialize = "vmNptFaultToleranceOrRecordReplayConfigured")]
    VmNptFaultToleranceOrRecordReplayConfigured,
    #[serde(rename = "vmNptConflictingIOChainConfigured")]
    #[strum(serialize = "vmNptConflictingIOChainConfigured")]
    VmNptConflictingIoChainConfigured,
    #[serde(rename = "vmNptMonitorBlocks")]
    #[strum(serialize = "vmNptMonitorBlocks")]
    VmNptMonitorBlocks,
    #[serde(rename = "vmNptConflictingOperationInProgress")]
    #[strum(serialize = "vmNptConflictingOperationInProgress")]
    VmNptConflictingOperationInProgress,
    #[serde(rename = "vmNptRuntimeError")]
    #[strum(serialize = "vmNptRuntimeError")]
    VmNptRuntimeError,
    #[serde(rename = "vmNptOutOfIntrVector")]
    #[strum(serialize = "vmNptOutOfIntrVector")]
    VmNptOutOfIntrVector,
    #[serde(rename = "vmNptVMCIActive")]
    #[strum(serialize = "vmNptVMCIActive")]
    VmNptVmciActive,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// File-type constants.
/// 
/// Possible values:
/// - `config`: Config (vmx) file.
/// - `extendedConfig`: Extended config (vmxf) file.
/// - `diskDescriptor`: Disk descriptor (vmdk) file.
/// - `diskExtent`: Disk extent (-flat/-delta/-s/-rdm/-rdmp.vmdk) file.
/// - `digestDescriptor`: Disk digest descriptor file.
/// - `digestExtent`: Disk digest extent file.
/// - `diskReplicationState`: Host based replicated disk persistent state (psf) file.
/// - `log`: Log (log) file.
/// - `stat`: Virtual machine statistics (stat) file.
/// - `namespaceData`: Namespace data file.
/// - `dataSetsDiskModeStore`: DataSets disk mode store (dsd) file.
///   
///   ***Since:*** vSphere API Release 8.0.0.0
/// - `dataSetsVmModeStore`: DataSets vm mode store (dsv) file.
///   
///   ***Since:*** vSphere API Release 8.0.0.0
/// - `nvram`: Non-volatile RAM (nvram) file.
/// - `snapshotData`: Snapshot data (vmsn) file.
/// - `snapshotMemory`: Snapshot memory (vmem) file.
/// - `snapshotList`: Snapshot metadata (vmsd) file.
/// - `snapshotManifestList`: Snapshot manifest metadata (-aux.xml) file.
///   
///   This file is still being created but is no longer necessary since
///   the manifest metadata is now available in the snapshot metadata
///   (vmsd) file in vSphere 5.0. This type will be deprecated when
///   vSphere 4.1 is no longer supported.
/// - `suspend`: Suspend (vmss) file.
/// - `suspendMemory`: Suspend (vmem) file.
/// - `swap`: Swap (vswp) file.
/// - `uwswap`: File generated by VMware ESX kernel for a running virtual
///   machine.
/// - `core`: Core (core) file.
/// - `screenshot`: Screenshot file.
/// - `ftMetadata`: Fault Tolerance metadata file.
/// - `guestCustomization`: Guest image customization file.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineFileLayoutExFileTypeEnum {
    #[serde(rename = "config")]
    #[strum(serialize = "config")]
    Config,
    #[serde(rename = "extendedConfig")]
    #[strum(serialize = "extendedConfig")]
    ExtendedConfig,
    #[serde(rename = "diskDescriptor")]
    #[strum(serialize = "diskDescriptor")]
    DiskDescriptor,
    #[serde(rename = "diskExtent")]
    #[strum(serialize = "diskExtent")]
    DiskExtent,
    #[serde(rename = "digestDescriptor")]
    #[strum(serialize = "digestDescriptor")]
    DigestDescriptor,
    #[serde(rename = "digestExtent")]
    #[strum(serialize = "digestExtent")]
    DigestExtent,
    #[serde(rename = "diskReplicationState")]
    #[strum(serialize = "diskReplicationState")]
    DiskReplicationState,
    #[serde(rename = "log")]
    #[strum(serialize = "log")]
    Log,
    #[serde(rename = "stat")]
    #[strum(serialize = "stat")]
    Stat,
    #[serde(rename = "namespaceData")]
    #[strum(serialize = "namespaceData")]
    NamespaceData,
    #[serde(rename = "dataSetsDiskModeStore")]
    #[strum(serialize = "dataSetsDiskModeStore")]
    DataSetsDiskModeStore,
    #[serde(rename = "dataSetsVmModeStore")]
    #[strum(serialize = "dataSetsVmModeStore")]
    DataSetsVmModeStore,
    #[serde(rename = "nvram")]
    #[strum(serialize = "nvram")]
    Nvram,
    #[serde(rename = "snapshotData")]
    #[strum(serialize = "snapshotData")]
    SnapshotData,
    #[serde(rename = "snapshotMemory")]
    #[strum(serialize = "snapshotMemory")]
    SnapshotMemory,
    #[serde(rename = "snapshotList")]
    #[strum(serialize = "snapshotList")]
    SnapshotList,
    #[serde(rename = "snapshotManifestList")]
    #[strum(serialize = "snapshotManifestList")]
    SnapshotManifestList,
    #[serde(rename = "suspend")]
    #[strum(serialize = "suspend")]
    Suspend,
    #[serde(rename = "suspendMemory")]
    #[strum(serialize = "suspendMemory")]
    SuspendMemory,
    #[serde(rename = "swap")]
    #[strum(serialize = "swap")]
    Swap,
    #[serde(rename = "uwswap")]
    #[strum(serialize = "uwswap")]
    Uwswap,
    #[serde(rename = "core")]
    #[strum(serialize = "core")]
    Core,
    #[serde(rename = "screenshot")]
    #[strum(serialize = "screenshot")]
    Screenshot,
    #[serde(rename = "ftMetadata")]
    #[strum(serialize = "ftMetadata")]
    FtMetadata,
    #[serde(rename = "guestCustomization")]
    #[strum(serialize = "guestCustomization")]
    GuestCustomization,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated as of vSphere API 6.7.
/// 
/// Set of possible values for *VirtualMachineFlagInfo.htSharing*.
/// 
/// Possible values:
/// - `any`: VCPUs may freely share cores at any time with any other
///   VCPUs (default for all virtual machines on a hyperthreaded
///   system).
/// - `none`: VCPUs should not share cores with each other or with VCPUs
///   from other virtual machines.
///   
///   That is, each VCPU from this
///   virtual machine should always get a whole core to itself,
///   with the other logical CPU on that core being placed into
///   the "halted" state.
/// - `internal`: Similar to "none", in that VCPUs from this virtual machine
///   will not be allowed to share cores with VCPUs from other
///   virtual machines.
///   
///   However, other VCPUs from the same virtual
///   machine will be allowed to share cores together. This
///   configuration option is only permitted for SMP virtual
///   machines. If applied to a uniprocessor virtual machine, it
///   will be converted to the "none" sharing option.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineHtSharingEnum {
    #[serde(rename = "any")]
    #[strum(serialize = "any")]
    Any,
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "internal")]
    #[strum(serialize = "internal")]
    Internal,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for *VirtualMachineFlagInfo.monitorType*.
/// 
/// Possible values:
/// - `release`: Run vmx in default mode, matching the build type of vmkernel.
/// - `debug`: Run vmx in debug mode.
/// - `stats`: Run vmx in stats mode.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineFlagInfoMonitorTypeEnum {
    #[serde(rename = "release")]
    #[strum(serialize = "release")]
    Release,
    #[serde(rename = "debug")]
    #[strum(serialize = "debug")]
    Debug,
    #[serde(rename = "stats")]
    #[strum(serialize = "stats")]
    Stats,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for *VirtualMachineFlagInfo.snapshotPowerOffBehavior*.
/// 
/// Possible values:
/// - `powerOff`: Just power off the virtual machine.
/// - `revert`: Revert to the snapshot.
/// - `prompt`: Prompt the user for instructions at power-off time.
/// - `take`: Take a new snapshot.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachinePowerOffBehaviorEnum {
    #[serde(rename = "powerOff")]
    #[strum(serialize = "powerOff")]
    PowerOff,
    #[serde(rename = "revert")]
    #[strum(serialize = "revert")]
    Revert,
    #[serde(rename = "prompt")]
    #[strum(serialize = "prompt")]
    Prompt,
    #[serde(rename = "take")]
    #[strum(serialize = "take")]
    Take,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for *VirtualMachineFlagInfo.virtualExecUsage*.
/// 
/// Possible values:
/// - `hvAuto`: Determine automatically whether to use hardware virtualization (HV) support.
/// - `hvOn`: Use hardware virtualization (HV) support if the physical hardware supports it.
/// - `hvOff`: Do not use hardware virtualization (HV) support.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineFlagInfoVirtualExecUsageEnum {
    #[serde(rename = "hvAuto")]
    #[strum(serialize = "hvAuto")]
    HvAuto,
    #[serde(rename = "hvOn")]
    #[strum(serialize = "hvOn")]
    HvOn,
    #[serde(rename = "hvOff")]
    #[strum(serialize = "hvOff")]
    HvOff,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for *VirtualMachineFlagInfo.virtualMmuUsage*.
/// 
/// Possible values:
/// - `automatic`: Determine automatically whether to use nested page table hardware support.
/// - `on`: Use nested paging hardware support if the physical hardware supports it.
/// - `off`: Do not use nested page table hardware support.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineFlagInfoVirtualMmuUsageEnum {
    #[serde(rename = "automatic")]
    #[strum(serialize = "automatic")]
    Automatic,
    #[serde(rename = "on")]
    #[strum(serialize = "on")]
    On,
    #[serde(rename = "off")]
    #[strum(serialize = "off")]
    Off,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Fork child type.
/// 
/// A child could be type of none, persistent, or
/// nonpersistent.
/// 
/// Possible values:
/// - `none`: The virtual machine is not a child.
/// - `persistent`: The virtual machine is a persistent child.
/// - `nonpersistent`: The virtual machine is a non-persistent child.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineForkConfigInfoChildTypeEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "persistent")]
    #[strum(serialize = "persistent")]
    Persistent,
    #[serde(rename = "nonpersistent")]
    #[strum(serialize = "nonpersistent")]
    Nonpersistent,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Application state type.
/// 
/// Possible values:
/// - `none`: The application state wasn't set from the guest by the application agent.
///   
///   This is the default.
/// - `appStateOk`: The guest's application agent declared its state as normal and doesn't
///   require any action
/// - `appStateNeedReset`: Guest's application agent asks for immediate reset
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum GuestInfoAppStateTypeEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "appStateOk")]
    #[strum(serialize = "appStateOk")]
    AppStateOk,
    #[serde(rename = "appStateNeedReset")]
    #[strum(serialize = "appStateNeedReset")]
    AppStateNeedReset,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Guest customization status
/// 
/// Possible values:
/// - `TOOLSDEPLOYPKG_IDLE`: No guest customizationSpec has been applied for the VM
/// - `TOOLSDEPLOYPKG_PENDING`: The guest customizationSpec has been applied for the VM,
///   but the customization process has not yet started inside the guest OS
/// - `TOOLSDEPLOYPKG_RUNNING`: The customization process is currently running inside the guest OS
/// - `TOOLSDEPLOYPKG_SUCCEEDED`: The customization process has completed successfully inside the
///   guest OS
/// - `TOOLSDEPLOYPKG_FAILED`: The customizatio process has failed inside the guest OS
///   
/// ***Since:*** vSphere API Release 7.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum GuestInfoCustomizationStatusEnum {
    #[serde(rename = "TOOLSDEPLOYPKG_IDLE")]
    #[strum(serialize = "TOOLSDEPLOYPKG_IDLE")]
    ToolsdeploypkgIdle,
    #[serde(rename = "TOOLSDEPLOYPKG_PENDING")]
    #[strum(serialize = "TOOLSDEPLOYPKG_PENDING")]
    ToolsdeploypkgPending,
    #[serde(rename = "TOOLSDEPLOYPKG_RUNNING")]
    #[strum(serialize = "TOOLSDEPLOYPKG_RUNNING")]
    ToolsdeploypkgRunning,
    #[serde(rename = "TOOLSDEPLOYPKG_SUCCEEDED")]
    #[strum(serialize = "TOOLSDEPLOYPKG_SUCCEEDED")]
    ToolsdeploypkgSucceeded,
    #[serde(rename = "TOOLSDEPLOYPKG_FAILED")]
    #[strum(serialize = "TOOLSDEPLOYPKG_FAILED")]
    ToolsdeploypkgFailed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The possible hints that the guest could display about current tasks
/// inside the guest.
/// 
/// Possible values:
/// - `running`
/// - `shuttingDown`
/// - `resetting`
/// - `standby`
/// - `notRunning`
/// - `unknown`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineGuestStateEnum {
    #[serde(rename = "running")]
    #[strum(serialize = "running")]
    Running,
    #[serde(rename = "shuttingDown")]
    #[strum(serialize = "shuttingDown")]
    ShuttingDown,
    #[serde(rename = "resetting")]
    #[strum(serialize = "resetting")]
    Resetting,
    #[serde(rename = "standby")]
    #[strum(serialize = "standby")]
    Standby,
    #[serde(rename = "notRunning")]
    #[strum(serialize = "notRunning")]
    NotRunning,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The installation type of tools in the VM.
/// 
/// Possible values:
/// - `guestToolsTypeUnknown`: Installation type is not known.
///   
///   Most likely tools have been
///   installed by OSPs or open-vm-tools, but a version that does
///   not report its install type or an install type that we do
///   not recognize.
/// - `guestToolsTypeMSI`: MSI is the installation type used for VMware Tools on Windows.
/// - `guestToolsTypeTar`: Tools have been installed by the tar installer.
/// - `guestToolsTypeOSP`: OSPs are RPM or Debian packages tailored for the OS in the VM.
///   
///   See http://packages.vmware.com
/// - `guestToolsTypeOpenVMTools`: open-vm-tools are the open-source version of VMware Tools, may have
///   been packaged by the OS vendor.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineToolsInstallTypeEnum {
    #[serde(rename = "guestToolsTypeUnknown")]
    #[strum(serialize = "guestToolsTypeUnknown")]
    GuestToolsTypeUnknown,
    #[serde(rename = "guestToolsTypeMSI")]
    #[strum(serialize = "guestToolsTypeMSI")]
    GuestToolsTypeMsi,
    #[serde(rename = "guestToolsTypeTar")]
    #[strum(serialize = "guestToolsTypeTar")]
    GuestToolsTypeTar,
    #[serde(rename = "guestToolsTypeOSP")]
    #[strum(serialize = "guestToolsTypeOSP")]
    GuestToolsTypeOsp,
    #[serde(rename = "guestToolsTypeOpenVMTools")]
    #[strum(serialize = "guestToolsTypeOpenVMTools")]
    GuestToolsTypeOpenVmTools,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Current running status of VMware Tools running in the guest
/// operating system.
/// 
/// Possible values:
/// - `guestToolsNotRunning`: VMware Tools is not running.
/// - `guestToolsRunning`: VMware Tools is running.
/// - `guestToolsExecutingScripts`: VMware Tools is starting.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineToolsRunningStatusEnum {
    #[serde(rename = "guestToolsNotRunning")]
    #[strum(serialize = "guestToolsNotRunning")]
    GuestToolsNotRunning,
    #[serde(rename = "guestToolsRunning")]
    #[strum(serialize = "guestToolsRunning")]
    GuestToolsRunning,
    #[serde(rename = "guestToolsExecutingScripts")]
    #[strum(serialize = "guestToolsExecutingScripts")]
    GuestToolsExecutingScripts,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated as of vSphere API 4.0 use *VirtualMachineToolsVersionStatus_enum*
/// and *VirtualMachineToolsRunningStatus_enum*.
/// 
/// Current status of VMware Tools running in the guest operating system.
/// 
/// Possible values:
/// - `toolsNotInstalled`: VMware Tools has never been installed
///   or has not run in the virtual machine.
/// - `toolsNotRunning`: VMware Tools is not running.
/// - `toolsOld`: VMware Tools is running, but the version is not current.
/// - `toolsOk`: VMware Tools is running and the version is current.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineToolsStatusEnum {
    #[serde(rename = "toolsNotInstalled")]
    #[strum(serialize = "toolsNotInstalled")]
    ToolsNotInstalled,
    #[serde(rename = "toolsNotRunning")]
    #[strum(serialize = "toolsNotRunning")]
    ToolsNotRunning,
    #[serde(rename = "toolsOld")]
    #[strum(serialize = "toolsOld")]
    ToolsOld,
    #[serde(rename = "toolsOk")]
    #[strum(serialize = "toolsOk")]
    ToolsOk,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Current version status of VMware Tools installed in the guest operating
/// system.
/// 
/// Possible values:
/// - `guestToolsNotInstalled`: VMware Tools has never been installed.
/// - `guestToolsNeedUpgrade`: 
///   
///   Deprecated as of vSphere API 5.1 value is not reported by
///   toolsVersionStatus2, instead more detailed status is reported.
///   
///   VMware Tools is installed, but the version is not current.
/// - `guestToolsCurrent`: VMware Tools is installed, and the version is current.
/// - `guestToolsUnmanaged`: VMware Tools is installed, but it is not managed by VMWare.
/// - `guestToolsTooOld`: VMware Tools is installed, but the version is too old.
/// - `guestToolsSupportedOld`: VMware Tools is installed, supported, but a newer version is available.
/// - `guestToolsSupportedNew`: VMware Tools is installed, supported, and newer
///   than the version available on the host.
/// - `guestToolsTooNew`: VMware Tools is installed, and the version is known to be
///   too new to work correctly with this virtual machine.
/// - `guestToolsBlacklisted`: VMware Tools is installed, but the installed version is
///   known to have a grave bug and should be immediately upgraded.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineToolsVersionStatusEnum {
    #[serde(rename = "guestToolsNotInstalled")]
    #[strum(serialize = "guestToolsNotInstalled")]
    GuestToolsNotInstalled,
    #[serde(rename = "guestToolsNeedUpgrade")]
    #[strum(serialize = "guestToolsNeedUpgrade")]
    GuestToolsNeedUpgrade,
    #[serde(rename = "guestToolsCurrent")]
    #[strum(serialize = "guestToolsCurrent")]
    GuestToolsCurrent,
    #[serde(rename = "guestToolsUnmanaged")]
    #[strum(serialize = "guestToolsUnmanaged")]
    GuestToolsUnmanaged,
    #[serde(rename = "guestToolsTooOld")]
    #[strum(serialize = "guestToolsTooOld")]
    GuestToolsTooOld,
    #[serde(rename = "guestToolsSupportedOld")]
    #[strum(serialize = "guestToolsSupportedOld")]
    GuestToolsSupportedOld,
    #[serde(rename = "guestToolsSupportedNew")]
    #[strum(serialize = "guestToolsSupportedNew")]
    GuestToolsSupportedNew,
    #[serde(rename = "guestToolsTooNew")]
    #[strum(serialize = "guestToolsTooNew")]
    GuestToolsTooNew,
    #[serde(rename = "guestToolsBlacklisted")]
    #[strum(serialize = "guestToolsBlacklisted")]
    GuestToolsBlacklisted,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Firmware types
/// 
/// Possible values:
/// - `bios`: BIOS firmware
/// - `efi`: Extensible Firmware Interface
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum GuestOsDescriptorFirmwareTypeEnum {
    #[serde(rename = "bios")]
    #[strum(serialize = "bios")]
    Bios,
    #[serde(rename = "efi")]
    #[strum(serialize = "efi")]
    Efi,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Guest operating system family constants.
/// 
/// Possible values:
/// - `windowsGuest`: Windows operating system
/// - `linuxGuest`: Linux operating system
/// - `netwareGuest`: Novell Netware
/// - `solarisGuest`: Solaris operating system
/// - `darwinGuestFamily`: Mac OS operating system
/// - `otherGuestFamily`: Other operating systems
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineGuestOsFamilyEnum {
    #[serde(rename = "windowsGuest")]
    #[strum(serialize = "windowsGuest")]
    WindowsGuest,
    #[serde(rename = "linuxGuest")]
    #[strum(serialize = "linuxGuest")]
    LinuxGuest,
    #[serde(rename = "netwareGuest")]
    #[strum(serialize = "netwareGuest")]
    NetwareGuest,
    #[serde(rename = "solarisGuest")]
    #[strum(serialize = "solarisGuest")]
    SolarisGuest,
    #[serde(rename = "darwinGuestFamily")]
    #[strum(serialize = "darwinGuestFamily")]
    DarwinGuestFamily,
    #[serde(rename = "otherGuestFamily")]
    #[strum(serialize = "otherGuestFamily")]
    OtherGuestFamily,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Guest operating system identifier.
/// 
/// Possible values:
/// - `dosGuest`: MS-DOS.
/// - `win31Guest`: Windows 3.1
/// - `win95Guest`: Windows 95
/// - `win98Guest`: Windows 98
/// - `winMeGuest`: Windows Millennium Edition
/// - `winNTGuest`: Windows NT 4
/// - `win2000ProGuest`: Windows 2000 Professional
/// - `win2000ServGuest`: Windows 2000 Server
/// - `win2000AdvServGuest`: Windows 2000 Advanced Server
/// - `winXPHomeGuest`: Windows XP Home Edition
/// - `winXPProGuest`: Windows XP Professional
/// - `winXPPro64Guest`: Windows XP Professional Edition (64 bit)
/// - `winNetWebGuest`: Windows Server 2003, Web Edition
/// - `winNetStandardGuest`: Windows Server 2003, Standard Edition
/// - `winNetEnterpriseGuest`: Windows Server 2003, Enterprise Edition
/// - `winNetDatacenterGuest`: Windows Server 2003, Datacenter Edition
/// - `winNetBusinessGuest`: Windows Small Business Server 2003
/// - `winNetStandard64Guest`: Windows Server 2003, Standard Edition (64 bit)
/// - `winNetEnterprise64Guest`: Windows Server 2003, Enterprise Edition (64 bit)
/// - `winLonghornGuest`: Windows Longhorn
/// - `winLonghorn64Guest`: Windows Longhorn (64 bit)
/// - `winNetDatacenter64Guest`: Windows Server 2003, Datacenter Edition (64 bit)
/// - `winVistaGuest`: Windows Vista
/// - `winVista64Guest`: Windows Vista (64 bit)
/// - `windows7Guest`: Windows 7
/// - `windows7_64Guest`: Windows 7 (64 bit)
/// - `windows7Server64Guest`: Windows Server 2008 R2 (64 bit)
/// - `windows8Guest`: Windows 8
/// - `windows8_64Guest`: Windows 8 (64 bit)
/// - `windows8Server64Guest`: Windows 8 Server (64 bit)
/// - `windows9Guest`: Windows 10
/// - `windows9_64Guest`: Windows 10 (64 bit)
/// - `windows9Server64Guest`: Windows 10 Server (64 bit)
/// - `windows11_64Guest`: Windows 11
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `windows12_64Guest`: Windows 12
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `windowsHyperVGuest`: Windows Hyper-V
/// - `windows2019srv_64Guest`: Windows Server 2019
/// - `windows2019srvNext_64Guest`: Windows Server 2022
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `windows2022srvNext_64Guest`: Windows Server 2025
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `freebsdGuest`: FreeBSD
/// - `freebsd64Guest`: FreeBSD x64
/// - `freebsd11Guest`: FreeBSD 11
/// - `freebsd11_64Guest`: FreeBSD 11 x64
/// - `freebsd12Guest`: FreeBSD 12
/// - `freebsd12_64Guest`: FreeBSD 12 x64
/// - `freebsd13Guest`: FreeBSD 13
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `freebsd13_64Guest`: FreeBSD 13 x64
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `freebsd14Guest`: FreeBSD 14
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `freebsd14_64Guest`: FreeBSD 14 x64
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `redhatGuest`: Red Hat Linux 2.1
/// - `rhel2Guest`: Red Hat Enterprise Linux 2
/// - `rhel3Guest`: Red Hat Enterprise Linux 3
/// - `rhel3_64Guest`: Red Hat Enterprise Linux 3 (64 bit)
/// - `rhel4Guest`: Red Hat Enterprise Linux 4
/// - `rhel4_64Guest`: Red Hat Enterprise Linux 4 (64 bit)
/// - `rhel5Guest`: Red Hat Enterprise Linux 5
/// - `rhel5_64Guest`: Red Hat Enterprise Linux 5 (64 bit)
/// - `rhel6Guest`: Red Hat Enterprise Linux 6
/// - `rhel6_64Guest`: Red Hat Enterprise Linux 6 (64 bit)
/// - `rhel7Guest`: Red Hat Enterprise Linux 7
/// - `rhel7_64Guest`: Red Hat Enterprise Linux 7 (64 bit)
/// - `rhel8_64Guest`: Red Hat Enterprise Linux 8 (64 bit)
/// - `rhel9_64Guest`: Red Hat Enterprise Linux 9 (64 bit)
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `centosGuest`: CentOS 4/5
/// - `centos64Guest`: CentOS 4/5 (64-bit)
/// - `centos6Guest`: CentOS 6
/// - `centos6_64Guest`: CentOS 6 (64-bit)
/// - `centos7Guest`: CentOS 7
/// - `centos7_64Guest`: CentOS 7 (64-bit)
/// - `centos8_64Guest`: CentOS 8 (64-bit)
/// - `centos9_64Guest`: CentOS 9 (64-bit)
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `oracleLinuxGuest`: Oracle Linux 4/5
/// - `oracleLinux64Guest`: Oracle Linux 4/5 (64-bit)
/// - `oracleLinux6Guest`: Oracle 6
/// - `oracleLinux6_64Guest`: Oracle 6 (64-bit)
/// - `oracleLinux7Guest`: Oracle 7
/// - `oracleLinux7_64Guest`: Oracle 7 (64-bit)
/// - `oracleLinux8_64Guest`: Oracle 8 (64-bit)
/// - `oracleLinux9_64Guest`: Oracle 9 (64-bit)
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `suseGuest`: Suse Linux
/// - `suse64Guest`: Suse Linux (64 bit)
/// - `slesGuest`: Suse Linux Enterprise Server 9
/// - `sles64Guest`: Suse Linux Enterprise Server 9 (64 bit)
/// - `sles10Guest`: Suse linux Enterprise Server 10
/// - `sles10_64Guest`: Suse Linux Enterprise Server 10 (64 bit)
/// - `sles11Guest`: Suse linux Enterprise Server 11
/// - `sles11_64Guest`: Suse Linux Enterprise Server 11 (64 bit)
/// - `sles12Guest`: Suse linux Enterprise Server 12
/// - `sles12_64Guest`: Suse Linux Enterprise Server 12 (64 bit)
/// - `sles15_64Guest`: Suse Linux Enterprise Server 15 (64 bit)
/// - `sles16_64Guest`: Suse Linux Enterprise Server 16 (64 bit)
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `nld9Guest`: Novell Linux Desktop 9
/// - `oesGuest`: Open Enterprise Server
/// - `sjdsGuest`: Sun Java Desktop System
/// - `mandrakeGuest`: Mandrake Linux
/// - `mandrivaGuest`: Mandriva Linux
/// - `mandriva64Guest`: Mandriva Linux (64 bit)
/// - `turboLinuxGuest`: Turbolinux
/// - `turboLinux64Guest`: Turbolinux (64 bit)
/// - `ubuntuGuest`: Ubuntu Linux
/// - `ubuntu64Guest`: Ubuntu Linux (64 bit)
/// - `debian4Guest`: Debian GNU/Linux 4
/// - `debian4_64Guest`: Debian GNU/Linux 4 (64 bit)
/// - `debian5Guest`: Debian GNU/Linux 5
/// - `debian5_64Guest`: Debian GNU/Linux 5 (64 bit)
/// - `debian6Guest`: Debian GNU/Linux 6
/// - `debian6_64Guest`: Debian GNU/Linux 6 (64 bit)
/// - `debian7Guest`: Debian GNU/Linux 7
/// - `debian7_64Guest`: Debian GNU/Linux 7 (64 bit)
/// - `debian8Guest`: Debian GNU/Linux 8
/// - `debian8_64Guest`: Debian GNU/Linux 8 (64 bit)
/// - `debian9Guest`: Debian GNU/Linux 9
/// - `debian9_64Guest`: Debian GNU/Linux 9 (64 bit)
/// - `debian10Guest`: Debian GNU/Linux 10
/// - `debian10_64Guest`: Debian GNU/Linux 10 (64 bit)
/// - `debian11Guest`: Debian GNU/Linux 11
/// - `debian11_64Guest`: Debian GNU/Linux 11 (64 bit)
/// - `debian12Guest`: Debian GNU/Linux 12
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `debian12_64Guest`: Debian GNU/Linux 12 (64 bit)
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `asianux3Guest`: Asianux Server 3
/// - `asianux3_64Guest`: Asianux Server 3 (64 bit)
/// - `asianux4Guest`: Asianux Server 4
/// - `asianux4_64Guest`: Asianux Server 4 (64 bit)
/// - `asianux5_64Guest`: Asianux Server 5 (64 bit)
/// - `asianux7_64Guest`: Asianux Server 7 (64 bit)
/// - `asianux8_64Guest`: Asianux Server 8 (64 bit)
/// - `asianux9_64Guest`: Asianux Server 9 (64 bit)
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `opensuseGuest`: OpenSUSE Linux
/// - `opensuse64Guest`: OpenSUSE Linux (64 bit)
/// - `fedoraGuest`: Fedora Linux
/// - `fedora64Guest`: Fedora Linux (64 bit)
/// - `coreos64Guest`: CoreOS Linux (64 bit)
/// - `vmwarePhoton64Guest`: VMware Photon (64 bit)
/// - `other24xLinuxGuest`: Linux 2.4x Kernel
/// - `other26xLinuxGuest`: Linux 2.6x Kernel
/// - `otherLinuxGuest`: Linux 2.2x Kernel
/// - `other3xLinuxGuest`: Linux 3.x Kernel
/// - `other4xLinuxGuest`: Linux 4.x Kernel
/// - `other5xLinuxGuest`: Linux 5.x Kernel
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `other6xLinuxGuest`: Linux 6.x Kernel
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `genericLinuxGuest`: Other Linux
/// - `other24xLinux64Guest`: Linux 2.4.x Kernel (64 bit)
/// - `other26xLinux64Guest`: Linux 2.6.x Kernel (64 bit)
/// - `other3xLinux64Guest`: Linux 3.x Kernel (64 bit)
/// - `other4xLinux64Guest`: Linux 4.x Kernel (64 bit)
/// - `other5xLinux64Guest`: Linux 5.x Kernel (64 bit)
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `other6xLinux64Guest`: Linux 6.x Kernel (64 bit)
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `otherLinux64Guest`: Linux (64 bit)
/// - `solaris6Guest`: Solaris 6
/// - `solaris7Guest`: Solaris 7
/// - `solaris8Guest`: Solaris 8
/// - `solaris9Guest`: Solaris 9
/// - `solaris10Guest`: Solaris 10 (32 bit)
/// - `solaris10_64Guest`: Solaris 10 (64 bit)
/// - `solaris11_64Guest`: Solaris 11 (64 bit)
/// - `os2Guest`: OS/2
/// - `eComStationGuest`: eComStation 1.x
/// - `eComStation2Guest`: eComStation 2.0
/// - `netware4Guest`: Novell NetWare 4
/// - `netware5Guest`: Novell NetWare 5.1
/// - `netware6Guest`: Novell NetWare 6.x
/// - `openServer5Guest`: SCO OpenServer 5
/// - `openServer6Guest`: SCO OpenServer 6
/// - `unixWare7Guest`: SCO UnixWare 7
/// - `darwinGuest`: Mac OS 10.5
/// - `darwin64Guest`: Mac OS 10.5 (64 bit)
/// - `darwin10Guest`: Mac OS 10.6
/// - `darwin10_64Guest`: Mac OS 10.6 (64 bit)
/// - `darwin11Guest`: Mac OS 10.7
/// - `darwin11_64Guest`: Mac OS 10.7 (64 bit)
/// - `darwin12_64Guest`: Mac OS 10.8 (64 bit)
/// - `darwin13_64Guest`: Mac OS 10.9 (64 bit)
/// - `darwin14_64Guest`: Mac OS 10.10 (64 bit)
/// - `darwin15_64Guest`: Mac OS 10.11 (64 bit)
/// - `darwin16_64Guest`: Mac OS 10.12 (64 bit)
/// - `darwin17_64Guest`: macOS 10.13 (64 bit)
/// - `darwin18_64Guest`: macOS 10.14 (64 bit)
/// - `darwin19_64Guest`: macOS 10.15 (64 bit)
/// - `darwin20_64Guest`: macOS 11 (64 bit)
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `darwin21_64Guest`: macOS 12 (64 bit)
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `darwin22_64Guest`: macOS 13 (64 bit)
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `darwin23_64Guest`: macOS 14 (64 bit)
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `vmkernelGuest`: VMware ESX 4
/// - `vmkernel5Guest`: VMware ESX 5
/// - `vmkernel6Guest`: VMware ESX 6
/// - `vmkernel65Guest`: VMware ESXi 6.5 AND ESXi 6.7.
/// - `vmkernel7Guest`: VMware ESX 7
/// - `vmkernel8Guest`: VMware ESX 8
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `amazonlinux2_64Guest`: Amazon Linux 2 (64 bit)
/// - `amazonlinux3_64Guest`: Amazon Linux 3 (64 bit)
///   
///   ***Since:*** vSphere API Release 7.0.1.0
/// - `crxPod1Guest`: CRX Pod 1
/// - `rockylinux_64Guest`: Rocky Linux (64-bit)
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `almalinux_64Guest`: AlmaLinux (64-bit)
///   
///   ***Since:*** vSphere API Release 8.0.0.1
/// - `otherGuest`: Other Operating System
/// - `otherGuest64`: Other Operating System (64 bit)
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineGuestOsIdentifierEnum {
    #[serde(rename = "dosGuest")]
    #[strum(serialize = "dosGuest")]
    DosGuest,
    #[serde(rename = "win31Guest")]
    #[strum(serialize = "win31Guest")]
    Win31Guest,
    #[serde(rename = "win95Guest")]
    #[strum(serialize = "win95Guest")]
    Win95Guest,
    #[serde(rename = "win98Guest")]
    #[strum(serialize = "win98Guest")]
    Win98Guest,
    #[serde(rename = "winMeGuest")]
    #[strum(serialize = "winMeGuest")]
    WinMeGuest,
    #[serde(rename = "winNTGuest")]
    #[strum(serialize = "winNTGuest")]
    WinNtGuest,
    #[serde(rename = "win2000ProGuest")]
    #[strum(serialize = "win2000ProGuest")]
    Win2000ProGuest,
    #[serde(rename = "win2000ServGuest")]
    #[strum(serialize = "win2000ServGuest")]
    Win2000ServGuest,
    #[serde(rename = "win2000AdvServGuest")]
    #[strum(serialize = "win2000AdvServGuest")]
    Win2000AdvServGuest,
    #[serde(rename = "winXPHomeGuest")]
    #[strum(serialize = "winXPHomeGuest")]
    WinXpHomeGuest,
    #[serde(rename = "winXPProGuest")]
    #[strum(serialize = "winXPProGuest")]
    WinXpProGuest,
    #[serde(rename = "winXPPro64Guest")]
    #[strum(serialize = "winXPPro64Guest")]
    WinXpPro64Guest,
    #[serde(rename = "winNetWebGuest")]
    #[strum(serialize = "winNetWebGuest")]
    WinNetWebGuest,
    #[serde(rename = "winNetStandardGuest")]
    #[strum(serialize = "winNetStandardGuest")]
    WinNetStandardGuest,
    #[serde(rename = "winNetEnterpriseGuest")]
    #[strum(serialize = "winNetEnterpriseGuest")]
    WinNetEnterpriseGuest,
    #[serde(rename = "winNetDatacenterGuest")]
    #[strum(serialize = "winNetDatacenterGuest")]
    WinNetDatacenterGuest,
    #[serde(rename = "winNetBusinessGuest")]
    #[strum(serialize = "winNetBusinessGuest")]
    WinNetBusinessGuest,
    #[serde(rename = "winNetStandard64Guest")]
    #[strum(serialize = "winNetStandard64Guest")]
    WinNetStandard64Guest,
    #[serde(rename = "winNetEnterprise64Guest")]
    #[strum(serialize = "winNetEnterprise64Guest")]
    WinNetEnterprise64Guest,
    #[serde(rename = "winLonghornGuest")]
    #[strum(serialize = "winLonghornGuest")]
    WinLonghornGuest,
    #[serde(rename = "winLonghorn64Guest")]
    #[strum(serialize = "winLonghorn64Guest")]
    WinLonghorn64Guest,
    #[serde(rename = "winNetDatacenter64Guest")]
    #[strum(serialize = "winNetDatacenter64Guest")]
    WinNetDatacenter64Guest,
    #[serde(rename = "winVistaGuest")]
    #[strum(serialize = "winVistaGuest")]
    WinVistaGuest,
    #[serde(rename = "winVista64Guest")]
    #[strum(serialize = "winVista64Guest")]
    WinVista64Guest,
    #[serde(rename = "windows7Guest")]
    #[strum(serialize = "windows7Guest")]
    Windows7Guest,
    #[serde(rename = "windows7_64Guest")]
    #[strum(serialize = "windows7_64Guest")]
    Windows764Guest,
    #[serde(rename = "windows7Server64Guest")]
    #[strum(serialize = "windows7Server64Guest")]
    Windows7Server64Guest,
    #[serde(rename = "windows8Guest")]
    #[strum(serialize = "windows8Guest")]
    Windows8Guest,
    #[serde(rename = "windows8_64Guest")]
    #[strum(serialize = "windows8_64Guest")]
    Windows864Guest,
    #[serde(rename = "windows8Server64Guest")]
    #[strum(serialize = "windows8Server64Guest")]
    Windows8Server64Guest,
    #[serde(rename = "windows9Guest")]
    #[strum(serialize = "windows9Guest")]
    Windows9Guest,
    #[serde(rename = "windows9_64Guest")]
    #[strum(serialize = "windows9_64Guest")]
    Windows964Guest,
    #[serde(rename = "windows9Server64Guest")]
    #[strum(serialize = "windows9Server64Guest")]
    Windows9Server64Guest,
    #[serde(rename = "windows11_64Guest")]
    #[strum(serialize = "windows11_64Guest")]
    Windows1164Guest,
    #[serde(rename = "windows12_64Guest")]
    #[strum(serialize = "windows12_64Guest")]
    Windows1264Guest,
    #[serde(rename = "windowsHyperVGuest")]
    #[strum(serialize = "windowsHyperVGuest")]
    WindowsHyperVGuest,
    #[serde(rename = "windows2019srv_64Guest")]
    #[strum(serialize = "windows2019srv_64Guest")]
    Windows2019Srv64Guest,
    #[serde(rename = "windows2019srvNext_64Guest")]
    #[strum(serialize = "windows2019srvNext_64Guest")]
    Windows2019SrvNext64Guest,
    #[serde(rename = "windows2022srvNext_64Guest")]
    #[strum(serialize = "windows2022srvNext_64Guest")]
    Windows2022SrvNext64Guest,
    #[serde(rename = "freebsdGuest")]
    #[strum(serialize = "freebsdGuest")]
    FreebsdGuest,
    #[serde(rename = "freebsd64Guest")]
    #[strum(serialize = "freebsd64Guest")]
    Freebsd64Guest,
    #[serde(rename = "freebsd11Guest")]
    #[strum(serialize = "freebsd11Guest")]
    Freebsd11Guest,
    #[serde(rename = "freebsd11_64Guest")]
    #[strum(serialize = "freebsd11_64Guest")]
    Freebsd1164Guest,
    #[serde(rename = "freebsd12Guest")]
    #[strum(serialize = "freebsd12Guest")]
    Freebsd12Guest,
    #[serde(rename = "freebsd12_64Guest")]
    #[strum(serialize = "freebsd12_64Guest")]
    Freebsd1264Guest,
    #[serde(rename = "freebsd13Guest")]
    #[strum(serialize = "freebsd13Guest")]
    Freebsd13Guest,
    #[serde(rename = "freebsd13_64Guest")]
    #[strum(serialize = "freebsd13_64Guest")]
    Freebsd1364Guest,
    #[serde(rename = "freebsd14Guest")]
    #[strum(serialize = "freebsd14Guest")]
    Freebsd14Guest,
    #[serde(rename = "freebsd14_64Guest")]
    #[strum(serialize = "freebsd14_64Guest")]
    Freebsd1464Guest,
    #[serde(rename = "redhatGuest")]
    #[strum(serialize = "redhatGuest")]
    RedhatGuest,
    #[serde(rename = "rhel2Guest")]
    #[strum(serialize = "rhel2Guest")]
    Rhel2Guest,
    #[serde(rename = "rhel3Guest")]
    #[strum(serialize = "rhel3Guest")]
    Rhel3Guest,
    #[serde(rename = "rhel3_64Guest")]
    #[strum(serialize = "rhel3_64Guest")]
    Rhel364Guest,
    #[serde(rename = "rhel4Guest")]
    #[strum(serialize = "rhel4Guest")]
    Rhel4Guest,
    #[serde(rename = "rhel4_64Guest")]
    #[strum(serialize = "rhel4_64Guest")]
    Rhel464Guest,
    #[serde(rename = "rhel5Guest")]
    #[strum(serialize = "rhel5Guest")]
    Rhel5Guest,
    #[serde(rename = "rhel5_64Guest")]
    #[strum(serialize = "rhel5_64Guest")]
    Rhel564Guest,
    #[serde(rename = "rhel6Guest")]
    #[strum(serialize = "rhel6Guest")]
    Rhel6Guest,
    #[serde(rename = "rhel6_64Guest")]
    #[strum(serialize = "rhel6_64Guest")]
    Rhel664Guest,
    #[serde(rename = "rhel7Guest")]
    #[strum(serialize = "rhel7Guest")]
    Rhel7Guest,
    #[serde(rename = "rhel7_64Guest")]
    #[strum(serialize = "rhel7_64Guest")]
    Rhel764Guest,
    #[serde(rename = "rhel8_64Guest")]
    #[strum(serialize = "rhel8_64Guest")]
    Rhel864Guest,
    #[serde(rename = "rhel9_64Guest")]
    #[strum(serialize = "rhel9_64Guest")]
    Rhel964Guest,
    #[serde(rename = "centosGuest")]
    #[strum(serialize = "centosGuest")]
    CentosGuest,
    #[serde(rename = "centos64Guest")]
    #[strum(serialize = "centos64Guest")]
    Centos64Guest,
    #[serde(rename = "centos6Guest")]
    #[strum(serialize = "centos6Guest")]
    Centos6Guest,
    #[serde(rename = "centos6_64Guest")]
    #[strum(serialize = "centos6_64Guest")]
    Centos664Guest,
    #[serde(rename = "centos7Guest")]
    #[strum(serialize = "centos7Guest")]
    Centos7Guest,
    #[serde(rename = "centos7_64Guest")]
    #[strum(serialize = "centos7_64Guest")]
    Centos764Guest,
    #[serde(rename = "centos8_64Guest")]
    #[strum(serialize = "centos8_64Guest")]
    Centos864Guest,
    #[serde(rename = "centos9_64Guest")]
    #[strum(serialize = "centos9_64Guest")]
    Centos964Guest,
    #[serde(rename = "oracleLinuxGuest")]
    #[strum(serialize = "oracleLinuxGuest")]
    OracleLinuxGuest,
    #[serde(rename = "oracleLinux64Guest")]
    #[strum(serialize = "oracleLinux64Guest")]
    OracleLinux64Guest,
    #[serde(rename = "oracleLinux6Guest")]
    #[strum(serialize = "oracleLinux6Guest")]
    OracleLinux6Guest,
    #[serde(rename = "oracleLinux6_64Guest")]
    #[strum(serialize = "oracleLinux6_64Guest")]
    OracleLinux664Guest,
    #[serde(rename = "oracleLinux7Guest")]
    #[strum(serialize = "oracleLinux7Guest")]
    OracleLinux7Guest,
    #[serde(rename = "oracleLinux7_64Guest")]
    #[strum(serialize = "oracleLinux7_64Guest")]
    OracleLinux764Guest,
    #[serde(rename = "oracleLinux8_64Guest")]
    #[strum(serialize = "oracleLinux8_64Guest")]
    OracleLinux864Guest,
    #[serde(rename = "oracleLinux9_64Guest")]
    #[strum(serialize = "oracleLinux9_64Guest")]
    OracleLinux964Guest,
    #[serde(rename = "suseGuest")]
    #[strum(serialize = "suseGuest")]
    SuseGuest,
    #[serde(rename = "suse64Guest")]
    #[strum(serialize = "suse64Guest")]
    Suse64Guest,
    #[serde(rename = "slesGuest")]
    #[strum(serialize = "slesGuest")]
    SlesGuest,
    #[serde(rename = "sles64Guest")]
    #[strum(serialize = "sles64Guest")]
    Sles64Guest,
    #[serde(rename = "sles10Guest")]
    #[strum(serialize = "sles10Guest")]
    Sles10Guest,
    #[serde(rename = "sles10_64Guest")]
    #[strum(serialize = "sles10_64Guest")]
    Sles1064Guest,
    #[serde(rename = "sles11Guest")]
    #[strum(serialize = "sles11Guest")]
    Sles11Guest,
    #[serde(rename = "sles11_64Guest")]
    #[strum(serialize = "sles11_64Guest")]
    Sles1164Guest,
    #[serde(rename = "sles12Guest")]
    #[strum(serialize = "sles12Guest")]
    Sles12Guest,
    #[serde(rename = "sles12_64Guest")]
    #[strum(serialize = "sles12_64Guest")]
    Sles1264Guest,
    #[serde(rename = "sles15_64Guest")]
    #[strum(serialize = "sles15_64Guest")]
    Sles1564Guest,
    #[serde(rename = "sles16_64Guest")]
    #[strum(serialize = "sles16_64Guest")]
    Sles1664Guest,
    #[serde(rename = "nld9Guest")]
    #[strum(serialize = "nld9Guest")]
    Nld9Guest,
    #[serde(rename = "oesGuest")]
    #[strum(serialize = "oesGuest")]
    OesGuest,
    #[serde(rename = "sjdsGuest")]
    #[strum(serialize = "sjdsGuest")]
    SjdsGuest,
    #[serde(rename = "mandrakeGuest")]
    #[strum(serialize = "mandrakeGuest")]
    MandrakeGuest,
    #[serde(rename = "mandrivaGuest")]
    #[strum(serialize = "mandrivaGuest")]
    MandrivaGuest,
    #[serde(rename = "mandriva64Guest")]
    #[strum(serialize = "mandriva64Guest")]
    Mandriva64Guest,
    #[serde(rename = "turboLinuxGuest")]
    #[strum(serialize = "turboLinuxGuest")]
    TurboLinuxGuest,
    #[serde(rename = "turboLinux64Guest")]
    #[strum(serialize = "turboLinux64Guest")]
    TurboLinux64Guest,
    #[serde(rename = "ubuntuGuest")]
    #[strum(serialize = "ubuntuGuest")]
    UbuntuGuest,
    #[serde(rename = "ubuntu64Guest")]
    #[strum(serialize = "ubuntu64Guest")]
    Ubuntu64Guest,
    #[serde(rename = "debian4Guest")]
    #[strum(serialize = "debian4Guest")]
    Debian4Guest,
    #[serde(rename = "debian4_64Guest")]
    #[strum(serialize = "debian4_64Guest")]
    Debian464Guest,
    #[serde(rename = "debian5Guest")]
    #[strum(serialize = "debian5Guest")]
    Debian5Guest,
    #[serde(rename = "debian5_64Guest")]
    #[strum(serialize = "debian5_64Guest")]
    Debian564Guest,
    #[serde(rename = "debian6Guest")]
    #[strum(serialize = "debian6Guest")]
    Debian6Guest,
    #[serde(rename = "debian6_64Guest")]
    #[strum(serialize = "debian6_64Guest")]
    Debian664Guest,
    #[serde(rename = "debian7Guest")]
    #[strum(serialize = "debian7Guest")]
    Debian7Guest,
    #[serde(rename = "debian7_64Guest")]
    #[strum(serialize = "debian7_64Guest")]
    Debian764Guest,
    #[serde(rename = "debian8Guest")]
    #[strum(serialize = "debian8Guest")]
    Debian8Guest,
    #[serde(rename = "debian8_64Guest")]
    #[strum(serialize = "debian8_64Guest")]
    Debian864Guest,
    #[serde(rename = "debian9Guest")]
    #[strum(serialize = "debian9Guest")]
    Debian9Guest,
    #[serde(rename = "debian9_64Guest")]
    #[strum(serialize = "debian9_64Guest")]
    Debian964Guest,
    #[serde(rename = "debian10Guest")]
    #[strum(serialize = "debian10Guest")]
    Debian10Guest,
    #[serde(rename = "debian10_64Guest")]
    #[strum(serialize = "debian10_64Guest")]
    Debian1064Guest,
    #[serde(rename = "debian11Guest")]
    #[strum(serialize = "debian11Guest")]
    Debian11Guest,
    #[serde(rename = "debian11_64Guest")]
    #[strum(serialize = "debian11_64Guest")]
    Debian1164Guest,
    #[serde(rename = "debian12Guest")]
    #[strum(serialize = "debian12Guest")]
    Debian12Guest,
    #[serde(rename = "debian12_64Guest")]
    #[strum(serialize = "debian12_64Guest")]
    Debian1264Guest,
    #[serde(rename = "asianux3Guest")]
    #[strum(serialize = "asianux3Guest")]
    Asianux3Guest,
    #[serde(rename = "asianux3_64Guest")]
    #[strum(serialize = "asianux3_64Guest")]
    Asianux364Guest,
    #[serde(rename = "asianux4Guest")]
    #[strum(serialize = "asianux4Guest")]
    Asianux4Guest,
    #[serde(rename = "asianux4_64Guest")]
    #[strum(serialize = "asianux4_64Guest")]
    Asianux464Guest,
    #[serde(rename = "asianux5_64Guest")]
    #[strum(serialize = "asianux5_64Guest")]
    Asianux564Guest,
    #[serde(rename = "asianux7_64Guest")]
    #[strum(serialize = "asianux7_64Guest")]
    Asianux764Guest,
    #[serde(rename = "asianux8_64Guest")]
    #[strum(serialize = "asianux8_64Guest")]
    Asianux864Guest,
    #[serde(rename = "asianux9_64Guest")]
    #[strum(serialize = "asianux9_64Guest")]
    Asianux964Guest,
    #[serde(rename = "opensuseGuest")]
    #[strum(serialize = "opensuseGuest")]
    OpensuseGuest,
    #[serde(rename = "opensuse64Guest")]
    #[strum(serialize = "opensuse64Guest")]
    Opensuse64Guest,
    #[serde(rename = "fedoraGuest")]
    #[strum(serialize = "fedoraGuest")]
    FedoraGuest,
    #[serde(rename = "fedora64Guest")]
    #[strum(serialize = "fedora64Guest")]
    Fedora64Guest,
    #[serde(rename = "coreos64Guest")]
    #[strum(serialize = "coreos64Guest")]
    Coreos64Guest,
    #[serde(rename = "vmwarePhoton64Guest")]
    #[strum(serialize = "vmwarePhoton64Guest")]
    VmwarePhoton64Guest,
    #[serde(rename = "other24xLinuxGuest")]
    #[strum(serialize = "other24xLinuxGuest")]
    Other24XLinuxGuest,
    #[serde(rename = "other26xLinuxGuest")]
    #[strum(serialize = "other26xLinuxGuest")]
    Other26XLinuxGuest,
    #[serde(rename = "otherLinuxGuest")]
    #[strum(serialize = "otherLinuxGuest")]
    OtherLinuxGuest,
    #[serde(rename = "other3xLinuxGuest")]
    #[strum(serialize = "other3xLinuxGuest")]
    Other3XLinuxGuest,
    #[serde(rename = "other4xLinuxGuest")]
    #[strum(serialize = "other4xLinuxGuest")]
    Other4XLinuxGuest,
    #[serde(rename = "other5xLinuxGuest")]
    #[strum(serialize = "other5xLinuxGuest")]
    Other5XLinuxGuest,
    #[serde(rename = "other6xLinuxGuest")]
    #[strum(serialize = "other6xLinuxGuest")]
    Other6XLinuxGuest,
    #[serde(rename = "genericLinuxGuest")]
    #[strum(serialize = "genericLinuxGuest")]
    GenericLinuxGuest,
    #[serde(rename = "other24xLinux64Guest")]
    #[strum(serialize = "other24xLinux64Guest")]
    Other24XLinux64Guest,
    #[serde(rename = "other26xLinux64Guest")]
    #[strum(serialize = "other26xLinux64Guest")]
    Other26XLinux64Guest,
    #[serde(rename = "other3xLinux64Guest")]
    #[strum(serialize = "other3xLinux64Guest")]
    Other3XLinux64Guest,
    #[serde(rename = "other4xLinux64Guest")]
    #[strum(serialize = "other4xLinux64Guest")]
    Other4XLinux64Guest,
    #[serde(rename = "other5xLinux64Guest")]
    #[strum(serialize = "other5xLinux64Guest")]
    Other5XLinux64Guest,
    #[serde(rename = "other6xLinux64Guest")]
    #[strum(serialize = "other6xLinux64Guest")]
    Other6XLinux64Guest,
    #[serde(rename = "otherLinux64Guest")]
    #[strum(serialize = "otherLinux64Guest")]
    OtherLinux64Guest,
    #[serde(rename = "solaris6Guest")]
    #[strum(serialize = "solaris6Guest")]
    Solaris6Guest,
    #[serde(rename = "solaris7Guest")]
    #[strum(serialize = "solaris7Guest")]
    Solaris7Guest,
    #[serde(rename = "solaris8Guest")]
    #[strum(serialize = "solaris8Guest")]
    Solaris8Guest,
    #[serde(rename = "solaris9Guest")]
    #[strum(serialize = "solaris9Guest")]
    Solaris9Guest,
    #[serde(rename = "solaris10Guest")]
    #[strum(serialize = "solaris10Guest")]
    Solaris10Guest,
    #[serde(rename = "solaris10_64Guest")]
    #[strum(serialize = "solaris10_64Guest")]
    Solaris1064Guest,
    #[serde(rename = "solaris11_64Guest")]
    #[strum(serialize = "solaris11_64Guest")]
    Solaris1164Guest,
    #[serde(rename = "os2Guest")]
    #[strum(serialize = "os2Guest")]
    Os2Guest,
    #[serde(rename = "eComStationGuest")]
    #[strum(serialize = "eComStationGuest")]
    EComStationGuest,
    #[serde(rename = "eComStation2Guest")]
    #[strum(serialize = "eComStation2Guest")]
    EComStation2Guest,
    #[serde(rename = "netware4Guest")]
    #[strum(serialize = "netware4Guest")]
    Netware4Guest,
    #[serde(rename = "netware5Guest")]
    #[strum(serialize = "netware5Guest")]
    Netware5Guest,
    #[serde(rename = "netware6Guest")]
    #[strum(serialize = "netware6Guest")]
    Netware6Guest,
    #[serde(rename = "openServer5Guest")]
    #[strum(serialize = "openServer5Guest")]
    OpenServer5Guest,
    #[serde(rename = "openServer6Guest")]
    #[strum(serialize = "openServer6Guest")]
    OpenServer6Guest,
    #[serde(rename = "unixWare7Guest")]
    #[strum(serialize = "unixWare7Guest")]
    UnixWare7Guest,
    #[serde(rename = "darwinGuest")]
    #[strum(serialize = "darwinGuest")]
    DarwinGuest,
    #[serde(rename = "darwin64Guest")]
    #[strum(serialize = "darwin64Guest")]
    Darwin64Guest,
    #[serde(rename = "darwin10Guest")]
    #[strum(serialize = "darwin10Guest")]
    Darwin10Guest,
    #[serde(rename = "darwin10_64Guest")]
    #[strum(serialize = "darwin10_64Guest")]
    Darwin1064Guest,
    #[serde(rename = "darwin11Guest")]
    #[strum(serialize = "darwin11Guest")]
    Darwin11Guest,
    #[serde(rename = "darwin11_64Guest")]
    #[strum(serialize = "darwin11_64Guest")]
    Darwin1164Guest,
    #[serde(rename = "darwin12_64Guest")]
    #[strum(serialize = "darwin12_64Guest")]
    Darwin1264Guest,
    #[serde(rename = "darwin13_64Guest")]
    #[strum(serialize = "darwin13_64Guest")]
    Darwin1364Guest,
    #[serde(rename = "darwin14_64Guest")]
    #[strum(serialize = "darwin14_64Guest")]
    Darwin1464Guest,
    #[serde(rename = "darwin15_64Guest")]
    #[strum(serialize = "darwin15_64Guest")]
    Darwin1564Guest,
    #[serde(rename = "darwin16_64Guest")]
    #[strum(serialize = "darwin16_64Guest")]
    Darwin1664Guest,
    #[serde(rename = "darwin17_64Guest")]
    #[strum(serialize = "darwin17_64Guest")]
    Darwin1764Guest,
    #[serde(rename = "darwin18_64Guest")]
    #[strum(serialize = "darwin18_64Guest")]
    Darwin1864Guest,
    #[serde(rename = "darwin19_64Guest")]
    #[strum(serialize = "darwin19_64Guest")]
    Darwin1964Guest,
    #[serde(rename = "darwin20_64Guest")]
    #[strum(serialize = "darwin20_64Guest")]
    Darwin2064Guest,
    #[serde(rename = "darwin21_64Guest")]
    #[strum(serialize = "darwin21_64Guest")]
    Darwin2164Guest,
    #[serde(rename = "darwin22_64Guest")]
    #[strum(serialize = "darwin22_64Guest")]
    Darwin2264Guest,
    #[serde(rename = "darwin23_64Guest")]
    #[strum(serialize = "darwin23_64Guest")]
    Darwin2364Guest,
    #[serde(rename = "vmkernelGuest")]
    #[strum(serialize = "vmkernelGuest")]
    VmkernelGuest,
    #[serde(rename = "vmkernel5Guest")]
    #[strum(serialize = "vmkernel5Guest")]
    Vmkernel5Guest,
    #[serde(rename = "vmkernel6Guest")]
    #[strum(serialize = "vmkernel6Guest")]
    Vmkernel6Guest,
    #[serde(rename = "vmkernel65Guest")]
    #[strum(serialize = "vmkernel65Guest")]
    Vmkernel65Guest,
    #[serde(rename = "vmkernel7Guest")]
    #[strum(serialize = "vmkernel7Guest")]
    Vmkernel7Guest,
    #[serde(rename = "vmkernel8Guest")]
    #[strum(serialize = "vmkernel8Guest")]
    Vmkernel8Guest,
    #[serde(rename = "amazonlinux2_64Guest")]
    #[strum(serialize = "amazonlinux2_64Guest")]
    Amazonlinux264Guest,
    #[serde(rename = "amazonlinux3_64Guest")]
    #[strum(serialize = "amazonlinux3_64Guest")]
    Amazonlinux364Guest,
    #[serde(rename = "crxPod1Guest")]
    #[strum(serialize = "crxPod1Guest")]
    CrxPod1Guest,
    #[serde(rename = "rockylinux_64Guest")]
    #[strum(serialize = "rockylinux_64Guest")]
    Rockylinux64Guest,
    #[serde(rename = "almalinux_64Guest")]
    #[strum(serialize = "almalinux_64Guest")]
    Almalinux64Guest,
    #[serde(rename = "otherGuest")]
    #[strum(serialize = "otherGuest")]
    OtherGuest,
    #[serde(rename = "otherGuest64")]
    #[strum(serialize = "otherGuest64")]
    OtherGuest64,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Guest OS support level
/// 
/// Possible values:
/// - `experimental`: This operating system is not supported,
///   but may be supported in the future.
/// - `legacy`: This operating system is not fully supported,
///   but may have been supported in the past.
/// - `terminated`: No longer supported.
/// - `supported`: Fully supported.
/// - `unsupported`: This operating system is not supported.
/// - `deprecated`: Support for this operating system will be terminated in the future.
///   
///   Please migrate to using a different operating system.
/// - `techPreview`: This operating system may not be supported yet,
///   please check VMware compatibility guide.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum GuestOsDescriptorSupportLevelEnum {
    #[serde(rename = "experimental")]
    #[strum(serialize = "experimental")]
    Experimental,
    #[serde(rename = "legacy")]
    #[strum(serialize = "legacy")]
    Legacy,
    #[serde(rename = "terminated")]
    #[strum(serialize = "terminated")]
    Terminated,
    #[serde(rename = "supported")]
    #[strum(serialize = "supported")]
    Supported,
    #[serde(rename = "unsupported")]
    #[strum(serialize = "unsupported")]
    Unsupported,
    #[serde(rename = "deprecated")]
    #[strum(serialize = "deprecated")]
    Deprecated,
    #[serde(rename = "techPreview")]
    #[strum(serialize = "techPreview")]
    TechPreview,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// End guest quiesce phase error types.
/// 
/// Possible values:
/// - `failure`: Fail the end phase of guest quiesce creation.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum GuestQuiesceEndGuestQuiesceErrorEnum {
    #[serde(rename = "failure")]
    #[strum(serialize = "failure")]
    Failure,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This enum represents the set of legal operations
/// 
/// Possible values:
/// - `Update`: Create or update the Metadata for the specified VM
/// - `Remove`: Remove the Metadata for the specified VM
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineMetadataManagerVmMetadataOpEnum {
    Update,
    Remove,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This enum contains a list of valid owner values for
/// the name field
/// 
/// Possible values:
/// - `ComVmwareVsphereHA`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineMetadataManagerVmMetadataOwnerOwnerEnum {
    #[serde(rename = "ComVmwareVsphereHA")]
    #[strum(serialize = "ComVmwareVsphereHA")]
    ComVmwareVsphereHa,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Specifies how a virtual disk is moved or copied to a
/// datastore.
/// 
/// In all cases after the move or copy the virtual machine's current running point
/// will be placed on the target datastore. The current running point is defined
/// as the disk backing which the virtual machine is currently
/// writing to. This end state can be achieved in multiple
/// ways, and the supported options are described in this
/// enumeration.
/// 
/// These options are only relevant when the backing of the
/// specified disk is a *file backing*.
/// 
/// Since disk backings may become shared as the result of
/// either a *clone operation* or
/// a *relocate operation*,
/// *VirtualMachine.PromoteDisks_Task* has been provided as
/// a way to unshare such disk backings.
/// 
/// See also *VirtualDiskSparseVer1BackingInfo.parent*, *VirtualDiskSparseVer2BackingInfo.parent*, *VirtualDiskFlatVer1BackingInfo.parent*, *VirtualDiskFlatVer2BackingInfo.parent*, *VirtualDiskRawDiskMappingVer1BackingInfo.parent*, *VirtualMachineRelocateSpec.diskMoveType*, *VirtualMachineRelocateSpecDiskLocator.diskMoveType*.
/// 
/// Possible values:
/// - `moveAllDiskBackingsAndAllowSharing`: All of the virtual disk's backings should be moved to the new datastore.
///   
///   If a disk backing is not the child-most backing of this virtual machine,
///   and there exists a read-only disk backing with the same content ID
///   on the target datastore, then this disk backing may not be copied. Instead
///   it is acceptable to attach to the read-only disk backing at the target
///   datastore. A read-only disk backing is defined as a virtual disk
///   backing which no virtual machine is currently writing to.
///   
///   See also *VirtualDiskSparseVer1BackingInfo.contentId*, *VirtualDiskSparseVer2BackingInfo.contentId*, *VirtualDiskFlatVer1BackingInfo.contentId*, *VirtualDiskFlatVer2BackingInfo.contentId*, *VirtualDiskRawDiskMappingVer1BackingInfo.contentId*.
/// - `moveAllDiskBackingsAndDisallowSharing`: All of the virtual disk's backings should be moved to the new datastore.
///   
///   It is not acceptable to attach to a disk backing with the same content ID
///   on the destination datastore. During a *clone operation* any delta disk backings will be consolidated.
/// - `moveChildMostDiskBacking`: Move only the child-most disk backing.
///   
///   Any parent disk backings should
///   be left in their current locations.
///   
///   This option only differs from *moveAllDiskBackingsAndAllowSharing* and
///   *moveAllDiskBackingsAndDisallowSharing* when the virtual
///   disk has a parent backing.
///   
///   Note that in the case of a *clone operation*,
///   this means that the parent disks will now be shared. This is safe as any
///   parent disks are always read-only.
///   Note that in the case of a *VirtualMachine.RelocateVM_Task* operation,
///   only the virtual disks in the current virtual machine configuration are moved.
/// - `createNewChildDiskBacking`: Create a new child disk backing on the destination datastore.
///   
///   None of the
///   virtual disk's existing files should be moved from their current locations.
///   
///   Note that in the case of a *clone operation*,
///   this means that the original virtual machine's disks are now all being shared.
///   This is only safe if the clone was taken from a snapshot point, because
///   snapshot points are always read-only. Thus for a clone this
///   option is only valid *when cloning from a snapshot*.
///   createNewChildDiskBacking is not a supported operation for
///   *VirtualMachine.RelocateVM_Task* operations unless all disks are moving.
/// - `moveAllDiskBackingsAndConsolidate`: All of the virtual disk's backings should be moved to the new datastore.
///   
///   During a *clone operation* or a
///   *VirtualMachine.MigrateVM_Task*, any delta disk backings will be
///   consolidated.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineRelocateDiskMoveOptionsEnum {
    #[serde(rename = "moveAllDiskBackingsAndAllowSharing")]
    #[strum(serialize = "moveAllDiskBackingsAndAllowSharing")]
    MoveAllDiskBackingsAndAllowSharing,
    #[serde(rename = "moveAllDiskBackingsAndDisallowSharing")]
    #[strum(serialize = "moveAllDiskBackingsAndDisallowSharing")]
    MoveAllDiskBackingsAndDisallowSharing,
    #[serde(rename = "moveChildMostDiskBacking")]
    #[strum(serialize = "moveChildMostDiskBacking")]
    MoveChildMostDiskBacking,
    #[serde(rename = "createNewChildDiskBacking")]
    #[strum(serialize = "createNewChildDiskBacking")]
    CreateNewChildDiskBacking,
    #[serde(rename = "moveAllDiskBackingsAndConsolidate")]
    #[strum(serialize = "moveAllDiskBackingsAndConsolidate")]
    MoveAllDiskBackingsAndConsolidate,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Deprecated as of vSphere API 5.0.
/// 
/// The set of tranformations that can be performed on the virtual disks
/// as part of the copy.
/// 
/// Possible values:
/// - `flat`
/// - `sparse`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineRelocateTransformationEnum {
    #[serde(rename = "flat")]
    #[strum(serialize = "flat")]
    Flat,
    #[serde(rename = "sparse")]
    #[strum(serialize = "sparse")]
    Sparse,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The policy setting used to determine when to perform scheduled
/// upgrades for a virtual machine.
/// 
/// Possible values:
/// - `never`: No scheduled upgrades.
/// - `onSoftPowerOff`: Run scheduled upgrades only on normal guest OS shutdown.
/// - `always`: Always run scheduled upgrades.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ScheduledHardwareUpgradeInfoHardwareUpgradePolicyEnum {
    #[serde(rename = "never")]
    #[strum(serialize = "never")]
    Never,
    #[serde(rename = "onSoftPowerOff")]
    #[strum(serialize = "onSoftPowerOff")]
    OnSoftPowerOff,
    #[serde(rename = "always")]
    #[strum(serialize = "always")]
    Always,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Status for last attempt to run scheduled hardware upgrade.
/// 
/// Possible values:
/// - `none`: No scheduled upgrade ever happened.
/// - `pending`: Upgrade is scheduled, but was not run yet.
/// - `success`: Upgrade succeeded.
/// - `failed`: Upgrade failed.
///   
///   For more information about the failure
///   
///   See also *ScheduledHardwareUpgradeInfo.fault*.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ScheduledHardwareUpgradeInfoHardwareUpgradeStatusEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "pending")]
    #[strum(serialize = "pending")]
    Pending,
    #[serde(rename = "success")]
    #[strum(serialize = "success")]
    Success,
    #[serde(rename = "failed")]
    #[strum(serialize = "failed")]
    Failed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible SCSI classes.
/// 
/// Possible values:
/// - `disk`
/// - `tape`
/// - `printer`
/// - `processor`
/// - `worm`
/// - `cdrom`
/// - `scanner`
/// - `optical`
/// - `media`
/// - `com`
/// - `raid`
/// - `unknown`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineScsiPassthroughTypeEnum {
    #[serde(rename = "disk")]
    #[strum(serialize = "disk")]
    Disk,
    #[serde(rename = "tape")]
    #[strum(serialize = "tape")]
    Tape,
    #[serde(rename = "printer")]
    #[strum(serialize = "printer")]
    Printer,
    #[serde(rename = "processor")]
    #[strum(serialize = "processor")]
    Processor,
    #[serde(rename = "worm")]
    #[strum(serialize = "worm")]
    Worm,
    #[serde(rename = "cdrom")]
    #[strum(serialize = "cdrom")]
    Cdrom,
    #[serde(rename = "scanner")]
    #[strum(serialize = "scanner")]
    Scanner,
    #[serde(rename = "optical")]
    #[strum(serialize = "optical")]
    Optical,
    #[serde(rename = "media")]
    #[strum(serialize = "media")]
    Media,
    #[serde(rename = "com")]
    #[strum(serialize = "com")]
    Com,
    #[serde(rename = "raid")]
    #[strum(serialize = "raid")]
    Raid,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Flexible Launch Enclave (FLC) modes.
/// 
/// Possible values:
/// - `locked`: FLC is available in the guest.
///   
///   The "launch Enclave MSRs" are locked and
///   initialized with the provided public key hash.
/// - `unlocked`: FLC is available in the guest.
///   
///   The "launch enclave MSRs" are writeable
///   and initialized with Intel's public key hash.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineSgxInfoFlcModesEnum {
    #[serde(rename = "locked")]
    #[strum(serialize = "locked")]
    Locked,
    #[serde(rename = "unlocked")]
    #[strum(serialize = "unlocked")]
    Unlocked,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Describes how widely the endpoint is available in a cluster.
/// 
/// Note that these fields are not necessarily mutual-exclusive.
/// 
/// Possible values:
/// - `compliant`: Indicates that this device is part of the cluster compliant
///   specification.
/// - `clusterWide`: Indicates that this is available for all hosts in the cluster.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineTargetInfoConfigurationTagEnum {
    #[serde(rename = "compliant")]
    #[strum(serialize = "compliant")]
    Compliant,
    #[serde(rename = "clusterWide")]
    #[strum(serialize = "clusterWide")]
    ClusterWide,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The policy setting used to determine when tools are auto-upgraded for
/// a virtual machine
/// 
/// Possible values:
/// - `manual`: No auto-upgrades for tools will be performed for this
///   virtual machine.
///   
///   Users must manually invoke the UpgradeTools
///   operation to update the tools.
/// - `upgradeAtPowerCycle`: When the virtual machine is power-cycled, the system checks
///   for a newer version of tools when the VM comes back up.
///   
///   If it
///   is available, a tools upgrade is automatically performed on the
///   virtual machine and it is rebooted if necessary.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum UpgradePolicyEnum {
    #[serde(rename = "manual")]
    #[strum(serialize = "manual")]
    Manual,
    #[serde(rename = "upgradeAtPowerCycle")]
    #[strum(serialize = "upgradeAtPowerCycle")]
    UpgradeAtPowerCycle,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Device class family.
/// 
/// Possible values:
/// - `audio`: Audio capable device.
/// - `hid`: Human interface device.
/// - `hid_bootable`: Bootable human interface device, this is a subset of HID devices.
/// - `physical`: Physical interface device.
/// - `communication`: Communication device.
/// - `imaging`: Still imaging device.
/// - `printer`: Printer device.
/// - `storage`: Mass storage device.
/// - `hub`: USB hubs.
/// - `smart_card`: Smart card device.
/// - `security`: Content security device.
/// - `video`: Video device.
/// - `wireless`: Wireless controller.
/// - `bluetooth`: Standard bluetooth adapter that uses HCI protocol,
///   this is a subset of wireless controllers.
/// - `wusb`: Wireless device related to the Wireless USB standard,
///   this is a subset of wireless controllers,
/// - `pda`: Palm PDA, and Micorsoft ActiveSync PDA.
/// - `vendor_specific`: Device that has an interface using a vendor-specific protocol.
/// - `other`: Other miscellaneous device.
/// - `unknownFamily`: There was an error in determining this device's classes
///   accurately.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineUsbInfoFamilyEnum {
    #[serde(rename = "audio")]
    #[strum(serialize = "audio")]
    Audio,
    #[serde(rename = "hid")]
    #[strum(serialize = "hid")]
    Hid,
    #[serde(rename = "hid_bootable")]
    #[strum(serialize = "hid_bootable")]
    HidBootable,
    #[serde(rename = "physical")]
    #[strum(serialize = "physical")]
    Physical,
    #[serde(rename = "communication")]
    #[strum(serialize = "communication")]
    Communication,
    #[serde(rename = "imaging")]
    #[strum(serialize = "imaging")]
    Imaging,
    #[serde(rename = "printer")]
    #[strum(serialize = "printer")]
    Printer,
    #[serde(rename = "storage")]
    #[strum(serialize = "storage")]
    Storage,
    #[serde(rename = "hub")]
    #[strum(serialize = "hub")]
    Hub,
    #[serde(rename = "smart_card")]
    #[strum(serialize = "smart_card")]
    SmartCard,
    #[serde(rename = "security")]
    #[strum(serialize = "security")]
    Security,
    #[serde(rename = "video")]
    #[strum(serialize = "video")]
    Video,
    #[serde(rename = "wireless")]
    #[strum(serialize = "wireless")]
    Wireless,
    #[serde(rename = "bluetooth")]
    #[strum(serialize = "bluetooth")]
    Bluetooth,
    #[serde(rename = "wusb")]
    #[strum(serialize = "wusb")]
    Wusb,
    #[serde(rename = "pda")]
    #[strum(serialize = "pda")]
    Pda,
    #[serde(rename = "vendor_specific")]
    #[strum(serialize = "vendor_specific")]
    VendorSpecific,
    #[serde(rename = "other")]
    #[strum(serialize = "other")]
    Other,
    #[serde(rename = "unknownFamily")]
    #[strum(serialize = "unknownFamily")]
    UnknownFamily,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Device speed.
/// 
/// Possible values:
/// - `low`: This device operates at low speed (1.5Mb/s).
/// - `full`: This device operates at full speed (12Mb/s).
/// - `high`: This device can operate at high speed (480Mb/s)
/// - `superSpeed`: This device can operate at super speed (4.8Gb/s)
/// - `superSpeedPlus`: This device can operate at super speed plus (10Gb/s)
/// - `superSpeed20Gbps`: This device can operate at super speed gen 2x2 (20Gb/s)
///   
///   ***Since:*** vSphere API Release 7.0.3.2
/// - `unknownSpeed`: This device's speed is unknown.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineUsbInfoSpeedEnum {
    #[serde(rename = "low")]
    #[strum(serialize = "low")]
    Low,
    #[serde(rename = "full")]
    #[strum(serialize = "full")]
    Full,
    #[serde(rename = "high")]
    #[strum(serialize = "high")]
    High,
    #[serde(rename = "superSpeed")]
    #[strum(serialize = "superSpeed")]
    SuperSpeed,
    #[serde(rename = "superSpeedPlus")]
    #[strum(serialize = "superSpeedPlus")]
    SuperSpeedPlus,
    #[serde(rename = "superSpeed20Gbps")]
    #[strum(serialize = "superSpeed20Gbps")]
    SuperSpeed20Gbps,
    #[serde(rename = "unknownSpeed")]
    #[strum(serialize = "unknownSpeed")]
    UnknownSpeed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Type of component device.
/// 
/// Possible values:
/// - `pciPassthru`
/// - `nvidiaVgpu`
/// - `sriovNic`
/// - `dvx`
/// 
/// ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineVendorDeviceGroupInfoComponentDeviceInfoComponentTypeEnum {
    #[serde(rename = "pciPassthru")]
    #[strum(serialize = "pciPassthru")]
    PciPassthru,
    #[serde(rename = "nvidiaVgpu")]
    #[strum(serialize = "nvidiaVgpu")]
    NvidiaVgpu,
    #[serde(rename = "sriovNic")]
    #[strum(serialize = "sriovNic")]
    SriovNic,
    #[serde(rename = "dvx")]
    #[strum(serialize = "dvx")]
    Dvx,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values for profile class.
/// 
/// Possible values:
/// - `compute`
/// - `quadro`
/// 
/// ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineVgpuProfileInfoProfileClassEnum {
    #[serde(rename = "compute")]
    #[strum(serialize = "compute")]
    Compute,
    #[serde(rename = "quadro")]
    #[strum(serialize = "quadro")]
    Quadro,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values for profile sharing.
/// 
/// Possible values:
/// - `timeSliced`: Time-sliced
/// - `mig`: Multi-instance GPU partitioning
///   
/// ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineVgpuProfileInfoProfileSharingEnum {
    #[serde(rename = "timeSliced")]
    #[strum(serialize = "timeSliced")]
    TimeSliced,
    #[serde(rename = "mig")]
    #[strum(serialize = "mig")]
    Mig,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `none`: No operation active.
/// - `scheduled`: Device swap will be performed on next restart.
/// - `inprogress`: Device swap is in progress.
/// - `failed`: Device swap failed.
/// - `completed`: Device swap successfully completed.
///   
/// ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineVirtualDeviceSwapDeviceSwapStatusEnum {
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    #[serde(rename = "scheduled")]
    #[strum(serialize = "scheduled")]
    Scheduled,
    #[serde(rename = "inprogress")]
    #[strum(serialize = "inprogress")]
    Inprogress,
    #[serde(rename = "failed")]
    #[strum(serialize = "failed")]
    Failed,
    #[serde(rename = "completed")]
    #[strum(serialize = "completed")]
    Completed,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Motherboard layout of the VM.
/// 
/// Possible values:
/// - `i440bxHostBridge`: Single i440BX host bridge.
/// - `acpiHostBridges`: Multiple ACPI host bridges.
///   
/// ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualHardwareMotherboardLayoutEnum {
    #[serde(rename = "i440bxHostBridge")]
    #[strum(serialize = "i440bxHostBridge")]
    I440BxHostBridge,
    #[serde(rename = "acpiHostBridges")]
    #[strum(serialize = "acpiHostBridges")]
    AcpiHostBridges,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The set of supported snapshot modes for VMs configured with NVDIMMs.
/// 
/// Possible values:
/// - `independent_persistent`: The data on virtual NVDIMMs are not affected by snapshot reverts.
///   
///   Writes to virtual NVDIMMs after a snapshot is taken cannot be
///   reverted to the snapshotted state.
/// - `independent_eraseonrevert`: Virtual NVDIMMs are erased and recreated upon snapshot reverts.
///   
/// ***Since:*** vSphere API Release 7.0.3.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineVirtualPMemSnapshotModeEnum {
    #[serde(rename = "independent_persistent")]
    #[strum(serialize = "independent_persistent")]
    IndependentPersistent,
    #[serde(rename = "independent_eraseonrevert")]
    #[strum(serialize = "independent_eraseonrevert")]
    IndependentEraseonrevert,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The VSS Snapshot Context
/// VSS\_SNAPSHOT\_CONTEXT values not listed below are not implemented.
/// 
/// Possible values:
/// - `ctx_auto`: The context value indicates auto selection of VSS snapshot context.
///   
///   The ctx\_backup may make Windows VSS-aware applications quiescing during
///   backup. The ctx\_auto makes VMTools select ctx\_file\_share\_backup context
///   if ctx\_backup is not available.
/// - `ctx_backup`: Indicate VSS\_CTX\_BACKUP.
/// - `ctx_file_share_backup`: Indicate VSS\_CTX\_FILE\_SHARE\_BACKUP.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineWindowsQuiesceSpecVssBackupContextEnum {
    #[serde(rename = "ctx_auto")]
    #[strum(serialize = "ctx_auto")]
    CtxAuto,
    #[serde(rename = "ctx_backup")]
    #[strum(serialize = "ctx_backup")]
    CtxBackup,
    #[serde(rename = "ctx_file_share_backup")]
    #[strum(serialize = "ctx_file_share_backup")]
    CtxFileShareBackup,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The types of tests which can requested by any of the methods in either
/// *VirtualMachineCompatibilityChecker* or *VirtualMachineProvisioningChecker*.
/// 
/// Possible values:
/// - `sourceTests`: Tests that examine only the configuration
///   of the virtual machine and its current host; the destination
///   resource pool and host or cluster are irrelevant.
/// - `hostTests`: Tests that examine both the virtual
///   machine and the destination host or cluster; the destination
///   resource pool is irrelevant.
///   
///   This set excludes tests that fall
///   into the datastoreTests group.
/// - `resourcePoolTests`: Tests that check that the destination resource
///   pool can support the virtual machine if it is powered on.
///   
///   The
///   destination host or cluster is relevant because it will affect the
///   amount of overhead memory required to run the virtual machine.
/// - `datastoreTests`: Tests that check that the
///   destination host or cluster can see the datastores where the virtual
///   machine's virtual disks are going to be located.
///   
///   The destination
///   resource pool is irrelevant.
/// - `networkTests`: Tests that check that the
///   destination host or cluster can see the networks that the virtual
///   machine's virtual nic devices are going to be connected.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum CheckTestTypeEnum {
    #[serde(rename = "sourceTests")]
    #[strum(serialize = "sourceTests")]
    SourceTests,
    #[serde(rename = "hostTests")]
    #[strum(serialize = "hostTests")]
    HostTests,
    #[serde(rename = "resourcePoolTests")]
    #[strum(serialize = "resourcePoolTests")]
    ResourcePoolTests,
    #[serde(rename = "datastoreTests")]
    #[strum(serialize = "datastoreTests")]
    DatastoreTests,
    #[serde(rename = "networkTests")]
    #[strum(serialize = "networkTests")]
    NetworkTests,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// NetBIOS setting for Windows.
/// 
/// Possible values:
/// - `enableNetBIOSViaDhcp`: DHCP server decides whether or not to use NetBIOS.
/// - `enableNetBIOS`: Always use NetBIOS.
/// - `disableNetBIOS`: Never use NetBIOS.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum CustomizationNetBiosModeEnum {
    #[serde(rename = "enableNetBIOSViaDhcp")]
    #[strum(serialize = "enableNetBIOSViaDhcp")]
    EnableNetBiosViaDhcp,
    #[serde(rename = "enableNetBIOS")]
    #[strum(serialize = "enableNetBIOS")]
    EnableNetBios,
    #[serde(rename = "disableNetBIOS")]
    #[strum(serialize = "disableNetBIOS")]
    DisableNetBios,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enumeration of AutoMode values.
/// 
/// Possible values:
/// - `perServer`: Indicates that client access licenses have been purchased for the server,
///   allowing a certain number of concurrent connections to the VirtualCenter
///   server.
/// - `perSeat`: Indicates that a client access license has been purchased for each computer
///   that accesses the VirtualCenter server.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum CustomizationLicenseDataModeEnum {
    #[serde(rename = "perServer")]
    #[strum(serialize = "perServer")]
    PerServer,
    #[serde(rename = "perSeat")]
    #[strum(serialize = "perSeat")]
    PerSeat,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// A enum constant specifying what should be done to the guest vm after running
/// sysprep.
/// 
/// Possible values:
/// - `reboot`: Reboot the machine after running sysprep.
///   
///   This will cause values
///   specified in the sysprep.xml to be applied immediately.
/// - `noreboot`: Take no action.
///   
///   Leave the guest os running after running sysprep. This
///   option can be used to look at values for debugging purposes after
///   running sysprep.
/// - `shutdown`: Shutdown the machine after running sysprep.
///   
///   This puts the vm in a
///   sealed state.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum CustomizationSysprepRebootOptionEnum {
    #[serde(rename = "reboot")]
    #[strum(serialize = "reboot")]
    Reboot,
    #[serde(rename = "noreboot")]
    #[strum(serialize = "noreboot")]
    Noreboot,
    #[serde(rename = "shutdown")]
    #[strum(serialize = "shutdown")]
    Shutdown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Contains information about connectable virtual devices when
/// the virtual machine restores from a migration.
/// 
/// Possible values:
/// - `connect`: Attempt to connect the virtual device when the virtual machine
///   restores from a migration.
///   
///   This property has no effect if it
///   is set on a device that is already connected.
/// - `disconnect`: Attempt to disconnect the virtual device when the virtual machine
///   restores from a migration.
///   
///   This property has no effect if it
///   is set on a device that is already disconnected.
/// - `unset`: Unset the property, which resets the device to its default state.
///   
///   Under most circumstances, a device will return to the same
///   connection state before the migration was initiated.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDeviceConnectInfoMigrateConnectOpEnum {
    #[serde(rename = "connect")]
    #[strum(serialize = "connect")]
    Connect,
    #[serde(rename = "disconnect")]
    #[strum(serialize = "disconnect")]
    Disconnect,
    #[serde(rename = "unset")]
    #[strum(serialize = "unset")]
    Unset,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Specifies the connectable virtual device status.
/// 
/// Possible values:
/// - `ok`: The device is working correctly.
/// - `recoverableError`: The device has reported a recoverable error.
///   
///   For example,
///   attempting to connect to floppy device that is being used by
///   another virtual machine or some other program would result in
///   this status.
/// - `unrecoverableError`: The device cannot be used.
///   
///   For example, attempting to connect to
///   a floppy device that does not exist would result in this status.
/// - `untried`: The device status is unknown, or it has not been requested to
///   connect when the VM is powered on.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDeviceConnectInfoStatusEnum {
    #[serde(rename = "ok")]
    #[strum(serialize = "ok")]
    Ok,
    #[serde(rename = "recoverableError")]
    #[strum(serialize = "recoverableError")]
    RecoverableError,
    #[serde(rename = "unrecoverableError")]
    #[strum(serialize = "unrecoverableError")]
    UnrecoverableError,
    #[serde(rename = "untried")]
    #[strum(serialize = "untried")]
    Untried,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// All known file extensions.
/// 
/// Valid ones are:
/// 
/// Possible values:
/// - `iso`: CD ISO Image backings
/// - `flp`: Floppy File Backings
/// - `vmdk`: virtual disks
/// - `dsk`: legacy virtual disks
/// - `rdm`: pre 3.0 virtual disks using Raw Disk Maps
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDeviceFileExtensionEnum {
    #[serde(rename = "iso")]
    #[strum(serialize = "iso")]
    Iso,
    #[serde(rename = "flp")]
    #[strum(serialize = "flp")]
    Flp,
    #[serde(rename = "vmdk")]
    #[strum(serialize = "vmdk")]
    Vmdk,
    #[serde(rename = "dsk")]
    #[strum(serialize = "dsk")]
    Dsk,
    #[serde(rename = "rdm")]
    #[strum(serialize = "rdm")]
    Rdm,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The <code>VirtualDeviceURIBackingOptionDirection</code> enum type
/// provides values for the direction of a network connection.
/// 
/// Possible values:
/// - `server`: Indicates that the virtual machine can listen for a connection
///   on the specified *VirtualDeviceURIBackingInfo.serviceURI*.
/// - `client`: Indicates that the virtual machine can initiate a connection
///   with a system on the network using the specified
///   *VirtualDeviceURIBackingInfo.serviceURI*.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDeviceUriBackingOptionDirectionEnum {
    #[serde(rename = "server")]
    #[strum(serialize = "server")]
    Server,
    #[serde(rename = "client")]
    #[strum(serialize = "client")]
    Client,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Describes the change mode of the device.
/// 
/// Applies only to virtual disks during VirtualDeviceSpec.Operation "add"
/// that have no VirtualDeviceSpec.FileOperation set.
/// 
/// Possible values:
/// - `fail`
/// - `skip`
/// 
/// ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDeviceConfigSpecChangeModeEnum {
    #[serde(rename = "fail")]
    #[strum(serialize = "fail")]
    Fail,
    #[serde(rename = "skip")]
    #[strum(serialize = "skip")]
    Skip,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The type of operation being performed on the backing of a virtual device.
/// 
/// Valid values are:
/// 
/// Possible values:
/// - `create`: Specifies the creation of the device backing; for example,
///   the creation of a virtual disk or floppy image file.
/// - `destroy`: Specifies the destruction of a device backing.
/// - `replace`: Specifies the deletion of the existing backing for a virtual device
///   and the creation of a new backing.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDeviceConfigSpecFileOperationEnum {
    #[serde(rename = "create")]
    #[strum(serialize = "create")]
    Create,
    #[serde(rename = "destroy")]
    #[strum(serialize = "destroy")]
    Destroy,
    #[serde(rename = "replace")]
    #[strum(serialize = "replace")]
    Replace,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The type of operation being performed on the specified virtual device.
/// 
/// Valid values are:
/// 
/// Possible values:
/// - `add`: Specifies the addition of a virtual device to the configuration.
/// - `remove`: Specifies the removal of a virtual device.
/// - `edit`: Specifies changes to the virtual device specification.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDeviceConfigSpecOperationEnum {
    #[serde(rename = "add")]
    #[strum(serialize = "add")]
    Add,
    #[serde(rename = "remove")]
    #[strum(serialize = "remove")]
    Remove,
    #[serde(rename = "edit")]
    #[strum(serialize = "edit")]
    Edit,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The delta disk format constants
/// 
/// Possible values:
/// - `redoLogFormat`: redo-log based format
/// - `nativeFormat`: native snapshot format
/// - `seSparseFormat`: Flex-SE redo-log based format
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDiskDeltaDiskFormatEnum {
    #[serde(rename = "redoLogFormat")]
    #[strum(serialize = "redoLogFormat")]
    RedoLogFormat,
    #[serde(rename = "nativeFormat")]
    #[strum(serialize = "nativeFormat")]
    NativeFormat,
    #[serde(rename = "seSparseFormat")]
    #[strum(serialize = "seSparseFormat")]
    SeSparseFormat,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The delta disk format variant constants
/// 
/// Possible values:
/// - `vmfsSparseVariant`: vmfsSparse based redo-log format
/// - `vsanSparseVariant`: vsanSparse based redo-log format
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDiskDeltaDiskFormatVariantEnum {
    #[serde(rename = "vmfsSparseVariant")]
    #[strum(serialize = "vmfsSparseVariant")]
    VmfsSparseVariant,
    #[serde(rename = "vsanSparseVariant")]
    #[strum(serialize = "vsanSparseVariant")]
    VsanSparseVariant,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The sharing mode of the virtual disk.
/// 
/// Setting the value to sharingMultiWriter means that multiple virtual
/// machines can write to the virtual disk. This sharing mode is allowed
/// only for eagerly zeroed thick virtual disks.
/// 
/// Possible values:
/// - `sharingNone`: The virtual disk is not shared.
/// - `sharingMultiWriter`: The virtual disk is shared between multiple virtual machines.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDiskSharingEnum {
    #[serde(rename = "sharingNone")]
    #[strum(serialize = "sharingNone")]
    SharingNone,
    #[serde(rename = "sharingMultiWriter")]
    #[strum(serialize = "sharingMultiWriter")]
    SharingMultiWriter,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Pre-defined constants for cache consistency types
/// 
/// Possible values:
/// - `strong`: With strong consistency, it ensures that
///   a crash will leave the cache data consistent.
/// - `weak`: Cache data consistency is not guaranteed after a crash.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDiskVFlashCacheConfigInfoCacheConsistencyTypeEnum {
    #[serde(rename = "strong")]
    #[strum(serialize = "strong")]
    Strong,
    #[serde(rename = "weak")]
    #[strum(serialize = "weak")]
    Weak,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Pre-defined constants for cache modes.
/// 
/// Possible values:
/// - `write_thru`: In write-through cache mode, writes to the cache cause writes
///   to the underlying storage.
///   
///   The cache acts as a facade to the underlying
///   storage.
/// - `write_back`: In write-back mode, writes to the cache do not go to the underlying storage
///   right away.
///   
///   Cache holds data temporarily till it can be permanently saved or
///   otherwise modified.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDiskVFlashCacheConfigInfoCacheModeEnum {
    #[serde(rename = "write_thru")]
    #[strum(serialize = "write_thru")]
    WriteThru,
    #[serde(rename = "write_back")]
    #[strum(serialize = "write_back")]
    WriteBack,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// All known compatibility modes for raw disk mappings.
/// 
/// Valid compatibility
/// modes are:
/// - virtualMode
/// - physicalMode
///   
/// Possible values:
/// - `virtualMode`: A disk device backed by a virtual compatibility mode raw disk mapping can
///   use disk modes.
///   
///   See also *VirtualDiskMode_enum*.
/// - `physicalMode`: A disk device backed by a physical compatibility mode raw disk mapping cannot
///   use disk modes, and commands are passed straight through to the LUN
///   indicated by the raw disk mapping.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDiskCompatibilityModeEnum {
    #[serde(rename = "virtualMode")]
    #[strum(serialize = "virtualMode")]
    VirtualMode,
    #[serde(rename = "physicalMode")]
    #[strum(serialize = "physicalMode")]
    PhysicalMode,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The list of known disk modes.
/// 
/// The list of supported disk modes varies by the backing type. The "persistent"
/// mode is supported by every backing type.
/// 
/// Possible values:
/// - `persistent`: Changes are immediately and permanently written to the virtual disk.
/// - `nonpersistent`: Changes to virtual disk are made to a redo log and discarded at power off.
/// - `undoable`: Changes are made to a redo log, but you are given the option to commit or undo.
/// - `independent_persistent`: Same as persistent, but not affected by snapshots.
/// - `independent_nonpersistent`: Same as nonpersistent, but not affected by snapshots.
/// - `append`: Changes are appended to the redo log; you revoke changes by removing the undo log.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualDiskModeEnum {
    #[serde(rename = "persistent")]
    #[strum(serialize = "persistent")]
    Persistent,
    #[serde(rename = "nonpersistent")]
    #[strum(serialize = "nonpersistent")]
    Nonpersistent,
    #[serde(rename = "undoable")]
    #[strum(serialize = "undoable")]
    Undoable,
    #[serde(rename = "independent_persistent")]
    #[strum(serialize = "independent_persistent")]
    IndependentPersistent,
    #[serde(rename = "independent_nonpersistent")]
    #[strum(serialize = "independent_nonpersistent")]
    IndependentNonpersistent,
    #[serde(rename = "append")]
    #[strum(serialize = "append")]
    Append,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible device names for legacy network backing option are listed below.
/// 
/// Note: This is not an exhaustive list. It is possible to specify
/// a specific device as well.
/// For example, on ESX hosts, the device name could be specified as "vmnic\[0-9\]"
/// or vmnet\_\[0-9\].
/// For VMware Server Windows hosts, the device name could be specified as "vmnet\[0-9\]"
/// and for VMware Server Linux hosts, the device name could be specified as "/dev/vmnet\[0-9\]"
/// depending on what devices are available on that particular host.
/// 
/// Possible values:
/// - `bridged`
/// - `nat`
/// - `hostonly`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualEthernetCardLegacyNetworkDeviceNameEnum {
    #[serde(rename = "bridged")]
    #[strum(serialize = "bridged")]
    Bridged,
    #[serde(rename = "nat")]
    #[strum(serialize = "nat")]
    Nat,
    #[serde(rename = "hostonly")]
    #[strum(serialize = "hostonly")]
    Hostonly,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The enumeration of all known valid MAC address types.
/// 
/// Possible values:
/// - `manual`: A statistically assigned MAC address.
/// - `generated`: An automatically generated MAC address.
/// - `assigned`: A MAC address assigned by VirtualCenter.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualEthernetCardMacTypeEnum {
    #[serde(rename = "manual")]
    #[strum(serialize = "manual")]
    Manual,
    #[serde(rename = "generated")]
    #[strum(serialize = "generated")]
    Generated,
    #[serde(rename = "assigned")]
    #[strum(serialize = "assigned")]
    Assigned,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `noSharing`
/// - `physicalSharing`
/// 
/// ***Since:*** vSphere API Release 8.0.2.0
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualNvmeControllerSharingEnum {
    #[serde(rename = "noSharing")]
    #[strum(serialize = "noSharing")]
    NoSharing,
    #[serde(rename = "physicalSharing")]
    #[strum(serialize = "physicalSharing")]
    PhysicalSharing,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The valid choices for host pointing devices are:
/// 
/// Possible values:
/// - `autodetect`: Automatically detects the host mouse type.
/// - `intellimouseExplorer`: The Microsoft IntelliMouse Explorer.
/// - `intellimousePs2`: The Microsoft Intellimouse with a PS2 connection.
/// - `logitechMouseman`: The Logitech MouseMan.
/// - `microsoft_serial`: The Microsoft Serial Mouse.
/// - `mouseSystems`: The Mouse Systems Mouse.
/// - `mousemanSerial`: The Logitech MouseMan Serial Bus Mouse.
/// - `ps2`: A generic mouse with a PS2 connection.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualPointingDeviceHostChoiceEnum {
    #[serde(rename = "autodetect")]
    #[strum(serialize = "autodetect")]
    Autodetect,
    #[serde(rename = "intellimouseExplorer")]
    #[strum(serialize = "intellimouseExplorer")]
    IntellimouseExplorer,
    #[serde(rename = "intellimousePs2")]
    #[strum(serialize = "intellimousePs2")]
    IntellimousePs2,
    #[serde(rename = "logitechMouseman")]
    #[strum(serialize = "logitechMouseman")]
    LogitechMouseman,
    #[serde(rename = "microsoft_serial")]
    #[strum(serialize = "microsoft_serial")]
    MicrosoftSerial,
    #[serde(rename = "mouseSystems")]
    #[strum(serialize = "mouseSystems")]
    MouseSystems,
    #[serde(rename = "mousemanSerial")]
    #[strum(serialize = "mousemanSerial")]
    MousemanSerial,
    #[serde(rename = "ps2")]
    #[strum(serialize = "ps2")]
    Ps2,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Sharing describes three possible ways of sharing the SCSI bus:
/// One of these values is assigned to the sharedBus object to determine
/// if or how the SCSI bus is shared.
/// 
/// Possible values:
/// - `noSharing`: The virtual SCSI bus is not shared.
/// - `virtualSharing`: The virtual SCSI bus is shared between two or more virtual machines.
///   
///   In this case, no physical machine is involved.
/// - `physicalSharing`: The virtual SCSI bus is shared between two or more virtual machines
///   residing on different physical hosts.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualScsiSharingEnum {
    #[serde(rename = "noSharing")]
    #[strum(serialize = "noSharing")]
    NoSharing,
    #[serde(rename = "virtualSharing")]
    #[strum(serialize = "virtualSharing")]
    VirtualSharing,
    #[serde(rename = "physicalSharing")]
    #[strum(serialize = "physicalSharing")]
    PhysicalSharing,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The <code>*VirtualSerialPortEndPoint_enum* enum defines
/// endpoint values for virtual serial port pipe backing.
/// 
/// When you use serial port pipe backing to connect a virtual machine
/// to another process, you must define the endpoints.
/// See the <code>*VirtualSerialPortPipeBackingInfo.endpoint*</code>
/// property for the virtual serial port pipe backing information data object.
/// 
/// The possible endpoint values are:
/// - client
/// - server
///   
/// For the supported choices, see the
/// <code>*VirtualSerialPortPipeBackingOption.endpoint*</code>
/// property for the virtual serial port pipe backing option data object.
/// 
/// Possible values:
/// - `client`
/// - `server`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualSerialPortEndPointEnum {
    #[serde(rename = "client")]
    #[strum(serialize = "client")]
    Client,
    #[serde(rename = "server")]
    #[strum(serialize = "server")]
    Server,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for action field in FilterSpec.
/// 
/// Determines whether traffic is allowed or denied.
/// 
/// Possible values:
/// - `allow`: Allow communication.
/// - `deny`: Deny communication.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineVmciDeviceActionEnum {
    #[serde(rename = "allow")]
    #[strum(serialize = "allow")]
    Allow,
    #[serde(rename = "deny")]
    #[strum(serialize = "deny")]
    Deny,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for direction field in FilterSpec.
/// 
/// Possible values:
/// - `guest`: from host to guest
/// - `host`: from guest to host
/// - `anyDirection`: all of the above
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineVmciDeviceDirectionEnum {
    #[serde(rename = "guest")]
    #[strum(serialize = "guest")]
    Guest,
    #[serde(rename = "host")]
    #[strum(serialize = "host")]
    Host,
    #[serde(rename = "anyDirection")]
    #[strum(serialize = "anyDirection")]
    AnyDirection,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for protocol field in FilterSpec.
/// 
/// Possible values:
/// - `hypervisor`: VMCI hypervisor datagram send op.
///   
///   Direction code is not applicable to this one.
/// - `doorbell`: VMCI doorbell notification
/// - `queuepair`: VMCI queue pair alloc operation.
///   
///   Direction code not applicable to this one.
/// - `datagram`: VMCI and VMCI Socket datagram send op.
///   
///   Since VMCI Socket datagrams map ports directly to resources,
///   there is no need to distinguish between the two.
/// - `stream`: VMCI Stream Socket connect op.
/// - `anyProtocol`: All of the above.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineVmciDeviceProtocolEnum {
    #[serde(rename = "hypervisor")]
    #[strum(serialize = "hypervisor")]
    Hypervisor,
    #[serde(rename = "doorbell")]
    #[strum(serialize = "doorbell")]
    Doorbell,
    #[serde(rename = "queuepair")]
    #[strum(serialize = "queuepair")]
    Queuepair,
    #[serde(rename = "datagram")]
    #[strum(serialize = "datagram")]
    Datagram,
    #[serde(rename = "stream")]
    #[strum(serialize = "stream")]
    Stream,
    #[serde(rename = "anyProtocol")]
    #[strum(serialize = "anyProtocol")]
    AnyProtocol,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Set of possible values for *VirtualMachineVideoCard.use3dRenderer*.
/// 
/// Possible values:
/// - `automatic`: Determine automatically whether to render 3D with software or hardware.
/// - `software`: Render 3D with software.
/// - `hardware`: Render 3D with graphics hardware.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualMachineVideoCardUse3DRendererEnum {
    #[serde(rename = "automatic")]
    #[strum(serialize = "automatic")]
    Automatic,
    #[serde(rename = "software")]
    #[strum(serialize = "software")]
    Software,
    #[serde(rename = "hardware")]
    #[strum(serialize = "hardware")]
    Hardware,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The enumeration of all known valid VRDMA device protocols.
/// 
/// Possible values:
/// - `rocev1`: A RoCEv1 device.
/// - `rocev2`: A RoCEv2 device.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VirtualVmxnet3VrdmaOptionDeviceProtocolsEnum {
    #[serde(rename = "rocev1")]
    #[strum(serialize = "rocev1")]
    Rocev1,
    #[serde(rename = "rocev2")]
    #[strum(serialize = "rocev2")]
    Rocev2,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `file`: Regular files, and on Posix filesystems, unix domain sockets
///   and devices.
/// - `directory`: directory
/// - `symlink`: symbolic link
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum GuestFileTypeEnum {
    #[serde(rename = "file")]
    #[strum(serialize = "file")]
    File,
    #[serde(rename = "directory")]
    #[strum(serialize = "directory")]
    Directory,
    #[serde(rename = "symlink")]
    #[strum(serialize = "symlink")]
    Symlink,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// This describes the bitness (32-bit or 64-bit) of a registry view in a
/// Windows OS that supports WOW64.
/// 
/// WOW64 (short for Windows 32-bit on Windows 64-bit) is the x86 emulator
/// that allows 32-bit Windows-based applications to run seamlessly on
/// 64-bit Windows. Please refer to these MSDN sites for more details:
/// http://msdn.microsoft.com/en-us/library/aa384249(v=vs.85).aspx and
/// http://msdn.microsoft.com/en-us/library/aa384253(v=vs.85).aspx
/// 
/// Possible values:
/// - `WOWNative`: Access the key from the native view of the
///   Registry (32-bit on 32-bit versions of Windows,
///   64-bit on 64-bit versions of Windows).
/// - `WOW32`: Access the key from the 32-bit view of the Registry.
/// - `WOW64`: Access the key from the 64-bit view of the Registry.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum GuestRegKeyWowSpecEnum {
    #[serde(rename = "WOWNative")]
    #[strum(serialize = "WOWNative")]
    WowNative,
    #[serde(rename = "WOW32")]
    #[strum(serialize = "WOW32")]
    Wow32,
    #[serde(rename = "WOW64")]
    #[strum(serialize = "WOW64")]
    Wow64,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The action to take with regard to storage objects upon decommissioning
/// a host from use with the VSAN service.
/// 
/// Possible values:
/// - `noAction`: No special action should take place regarding VSAN data.
/// - `ensureObjectAccessibility`: VSAN data reconfiguration should be performed to ensure storage
///   object accessibility.
/// - `evacuateAllData`: VSAN data evacuation should be performed such that all storage
///   object data is removed from the host.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VsanHostDecommissionModeObjectActionEnum {
    #[serde(rename = "noAction")]
    #[strum(serialize = "noAction")]
    NoAction,
    #[serde(rename = "ensureObjectAccessibility")]
    #[strum(serialize = "ensureObjectAccessibility")]
    EnsureObjectAccessibility,
    #[serde(rename = "evacuateAllData")]
    #[strum(serialize = "evacuateAllData")]
    EvacuateAllData,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Values used for indicating a disk's status for use by the VSAN service.
/// 
/// See also *VsanHostDiskResult.state*.
/// 
/// Possible values:
/// - `inUse`: Disk is currently in use by the VSAN service.
///   
///   A disk may be considered in use by the VSAN service regardless of
///   whether the VSAN service is enabled. As long as a disk is in use
///   by VSAN, it is reserved exclusively for VSAN and may not be used
///   for other purposes.
///   
///   See also *VsanHostDiskResult.error*.
/// - `eligible`: Disk is considered eligible for use by the VSAN service,
///   but is not currently in use.
/// - `ineligible`: Disk is considered ineligible for use by the VSAN service,
///   and is not currently in use.
///   
///   See also *VsanHostDiskResult.error*.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VsanHostDiskResultStateEnum {
    #[serde(rename = "inUse")]
    #[strum(serialize = "inUse")]
    InUse,
    #[serde(rename = "eligible")]
    #[strum(serialize = "eligible")]
    Eligible,
    #[serde(rename = "ineligible")]
    #[strum(serialize = "ineligible")]
    Ineligible,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// A *VsanHostHealthState_enum* represents the state of a participating
/// host in the VSAN service.
/// 
/// See also *VsanHostClusterStatus*.
/// 
/// Possible values:
/// - `unknown`: Node health is unknown.
/// - `healthy`: Node is considered healthy.
/// - `unhealthy`: Node is considered unhealthy.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VsanHostHealthStateEnum {
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    #[serde(rename = "healthy")]
    #[strum(serialize = "healthy")]
    Healthy,
    #[serde(rename = "unhealthy")]
    #[strum(serialize = "unhealthy")]
    Unhealthy,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// A *VsanHostNodeState_enum* represents the state of participation of a host
/// in the VSAN service.
/// 
/// See also *VsanHostClusterStatus*, *VsanHostClusterStatusState*.
/// 
/// Possible values:
/// - `error`: The node is enabled for the VSAN service but has some configuration
///   error which prevents participation.
/// - `disabled`: The node is disabled for the VSAN service.
/// - `agent`: The node is enabled for the VSAN service and is serving as an agent.
/// - `master`: The node is enabled for the VSAN service and is serving as the master.
/// - `backup`: The node is enabled for the VSAN service and is serving as the backup.
/// - `starting`: The node is starting the VSAN service; this state is considered
///   transitory.
/// - `stopping`: The node is stopping the VSAN service; this state is considered
///   transitory.
/// - `enteringMaintenanceMode`: The node is entering maintenance mode; this state is considered
///   transitory.
///   
///   See also *HostSystem.EnterMaintenanceMode_Task*.
/// - `exitingMaintenanceMode`: The node is exiting maintenance mode; this state is considered
///   transitory.
///   
///   See also *HostSystem.ExitMaintenanceMode_Task*.
/// - `decommissioning`: The node is being decommissioned from the VSAN service; this state is
///   considered transitory.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VsanHostNodeStateEnum {
    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
    #[serde(rename = "disabled")]
    #[strum(serialize = "disabled")]
    Disabled,
    #[serde(rename = "agent")]
    #[strum(serialize = "agent")]
    Agent,
    #[serde(rename = "master")]
    #[strum(serialize = "master")]
    Master,
    #[serde(rename = "backup")]
    #[strum(serialize = "backup")]
    Backup,
    #[serde(rename = "starting")]
    #[strum(serialize = "starting")]
    Starting,
    #[serde(rename = "stopping")]
    #[strum(serialize = "stopping")]
    Stopping,
    #[serde(rename = "enteringMaintenanceMode")]
    #[strum(serialize = "enteringMaintenanceMode")]
    EnteringMaintenanceMode,
    #[serde(rename = "exitingMaintenanceMode")]
    #[strum(serialize = "exitingMaintenanceMode")]
    ExitingMaintenanceMode,
    #[serde(rename = "decommissioning")]
    #[strum(serialize = "decommissioning")]
    Decommissioning,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// The list of disk issues.
/// 
/// Possible values:
/// - `nonExist`
/// - `stampMismatch`
/// - `unknown`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VsanDiskIssueTypeEnum {
    #[serde(rename = "nonExist")]
    #[strum(serialize = "nonExist")]
    NonExist,
    #[serde(rename = "stampMismatch")]
    #[strum(serialize = "stampMismatch")]
    StampMismatch,
    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Provisioning type constants.
/// 
/// Possible values:
/// - `thin`: Space required for thin-provisioned virtual disk is allocated
///   and zeroed on demand as the space is used.
/// - `eagerZeroedThick`: An eager zeroed thick virtual disk has all space allocated and
///   wiped clean of any previous contents on the physical media at
///   creation time.
///   
///   Such virtual disk may take longer time
///   during creation compared to other provisioning formats.
/// - `lazyZeroedThick`: A thick virtual disk has all space allocated at creation time.
///   
///   This space may contain stale data on the physical media.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum BaseConfigInfoDiskFileBackingInfoProvisioningTypeEnum {
    #[serde(rename = "thin")]
    #[strum(serialize = "thin")]
    Thin,
    #[serde(rename = "eagerZeroedThick")]
    #[strum(serialize = "eagerZeroedThick")]
    EagerZeroedThick,
    #[serde(rename = "lazyZeroedThick")]
    #[strum(serialize = "lazyZeroedThick")]
    LazyZeroedThick,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Flags representing the different information of a disk.
/// 
/// Possible values:
/// - `id`: ID of virtual storage object.
/// - `descriptorVersion`: The disk descriptor version of the virtual storage object.
///   
///   ***Since:*** vSphere API Release 8.0.1.0
/// - `backingObjectId`: ID of object backing the virtual storage object.
/// - `path`: File path of virtual storage object.
/// - `parentPath`: Parent file path of virtual storage object file.
/// - `name`: Name of virtual storage object.
/// - `deviceName`: Canonical name of the LUN used for virtual storage object.
/// - `capacity`: Logical size of virtual storage object.
/// - `allocated`: Size allocated by the file system for the virtual storage object
///   file/chain/link/extent only.
/// - `type`: Provisioning type of virtual storage object.
/// - `consumers`: IDs of the consumers of virtual storage object.
/// - `tentativeState`: If virtual storage object is in tentative state.
/// - `createTime`: Date and time of creation of virtual storage object.
/// - `ioFilter`: IOFilter associated with virtual storage object.
/// - `controlFlags`: Control flags of virtual storage object.
/// - `keepAfterVmDelete`: Deletion behaviour of virtual storage object after VM deletion.
/// - `relocationDisabled`: If relocation is disabled for virtual storage object.
/// - `keyId`: Key ID used to encrypt the virtual storage object.
/// - `keyProviderId`: Crypto key provider ID used to encrypt the virtual storage object.
/// - `nativeSnapshotSupported`: If virtual storage object supports native snapshot.
/// - `cbtEnabled`: If virtual storage object has changed block tracking enabled.
///   
/// ***Since:*** vSphere API Release 8.0.0.1
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VslmDiskInfoFlagEnum {
    #[serde(rename = "id")]
    #[strum(serialize = "id")]
    Id,
    #[serde(rename = "descriptorVersion")]
    #[strum(serialize = "descriptorVersion")]
    DescriptorVersion,
    #[serde(rename = "backingObjectId")]
    #[strum(serialize = "backingObjectId")]
    BackingObjectId,
    #[serde(rename = "path")]
    #[strum(serialize = "path")]
    Path,
    #[serde(rename = "parentPath")]
    #[strum(serialize = "parentPath")]
    ParentPath,
    #[serde(rename = "name")]
    #[strum(serialize = "name")]
    Name,
    #[serde(rename = "deviceName")]
    #[strum(serialize = "deviceName")]
    DeviceName,
    #[serde(rename = "capacity")]
    #[strum(serialize = "capacity")]
    Capacity,
    #[serde(rename = "allocated")]
    #[strum(serialize = "allocated")]
    Allocated,
    #[serde(rename = "type")]
    #[strum(serialize = "type")]
    Type,
    #[serde(rename = "consumers")]
    #[strum(serialize = "consumers")]
    Consumers,
    #[serde(rename = "tentativeState")]
    #[strum(serialize = "tentativeState")]
    TentativeState,
    #[serde(rename = "createTime")]
    #[strum(serialize = "createTime")]
    CreateTime,
    #[serde(rename = "ioFilter")]
    #[strum(serialize = "ioFilter")]
    IoFilter,
    #[serde(rename = "controlFlags")]
    #[strum(serialize = "controlFlags")]
    ControlFlags,
    #[serde(rename = "keepAfterVmDelete")]
    #[strum(serialize = "keepAfterVmDelete")]
    KeepAfterVmDelete,
    #[serde(rename = "relocationDisabled")]
    #[strum(serialize = "relocationDisabled")]
    RelocationDisabled,
    #[serde(rename = "keyId")]
    #[strum(serialize = "keyId")]
    KeyId,
    #[serde(rename = "keyProviderId")]
    #[strum(serialize = "keyProviderId")]
    KeyProviderId,
    #[serde(rename = "nativeSnapshotSupported")]
    #[strum(serialize = "nativeSnapshotSupported")]
    NativeSnapshotSupported,
    #[serde(rename = "cbtEnabled")]
    #[strum(serialize = "cbtEnabled")]
    CbtEnabled,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Consumption type constants.
/// 
/// Consumption type describes how the virtual storage object is connected and
/// consumed for data by the clients.
/// 
/// Possible values:
/// - `disk`: Disk type.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VStorageObjectConsumptionTypeEnum {
    #[serde(rename = "disk")]
    #[strum(serialize = "disk")]
    Disk,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Possible values:
/// - `keepAfterDeleteVm`: Choice of the deletion behavior of this virtual storage object.
///   
///   If not set, the default value is false.
/// - `disableRelocation`: Is virtual storage object relocation disabled.
///   
///   If not set, the default value is false.
/// - `enableChangedBlockTracking`: Is Virtual storage object has changed blocked tracking enabled.
///   
///   If not set, default value is false.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum VslmVStorageObjectControlFlagEnum {
    #[serde(rename = "keepAfterDeleteVm")]
    #[strum(serialize = "keepAfterDeleteVm")]
    KeepAfterDeleteVm,
    #[serde(rename = "disableRelocation")]
    #[strum(serialize = "disableRelocation")]
    DisableRelocation,
    #[serde(rename = "enableChangedBlockTracking")]
    #[strum(serialize = "enableChangedBlockTracking")]
    EnableChangedBlockTracking,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enumeration of possible changes to a property.
/// 
/// Possible values:
/// - `add`
/// - `remove`
/// - `assign`
/// - `indirectRemove`
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum PropertyChangeOpEnum {
    #[serde(rename = "add")]
    #[strum(serialize = "add")]
    Add,
    #[serde(rename = "remove")]
    #[strum(serialize = "remove")]
    Remove,
    #[serde(rename = "assign")]
    #[strum(serialize = "assign")]
    Assign,
    #[serde(rename = "indirectRemove")]
    #[strum(serialize = "indirectRemove")]
    IndirectRemove,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
/// Enumeration of different kinds of updates.
/// 
/// Possible values:
/// - `modify`: A property of the managed object changed its value.
/// - `enter`: A managed object became visible to a filter for the first time.
///   
///   For instance, this can happen if a virtual machine is added to a
///   folder.
/// - `leave`: A managed object left the set of objects visible to a filter.
///   
///   For
///   instance, this can happen when a virtual machine is destroyed.
#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::IntoStaticStr)]
pub enum ObjectUpdateKindEnum {
    #[serde(rename = "modify")]
    #[strum(serialize = "modify")]
    Modify,
    #[serde(rename = "enter")]
    #[strum(serialize = "enter")]
    Enter,
    #[serde(rename = "leave")]
    #[strum(serialize = "leave")]
    Leave,
    /// This variant handles values not known at compile time.
    #[serde(untagged)]
    #[strum(serialize = "__OTHER__")]
    Other_(String),
}
