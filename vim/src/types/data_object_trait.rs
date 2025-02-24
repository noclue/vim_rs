use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This is the built-in base interface implemented by all
/// data objects.
pub trait DataObjectTrait : super::vim_object_trait::VimObjectTrait {
}
impl<'s> serde::Serialize for dyn DataObjectTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DataObjectTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DataObjectVisitor)
            }
        }

struct DataObjectVisitor;

impl<'de> de::Visitor<'de> for DataObjectVisitor {
    type Value = Box<dyn DataObjectTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DataObjectTrait JSON object with a _typeName field")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let deserializer = de::value::MapAccessDeserializer::new(&mut map);
        let any: VimAny = de::Deserialize::deserialize(deserializer)?;
        match any {
            VimAny::Object(obj) => Ok(CastFrom::from_box(obj)
                .map_err(|_| de::Error::custom("Internal error converting to trait type"))?),
            VimAny::Value(value) => Err(de::Error::custom(format!(
                "expected object not wrapped value: {:?}",
                value))),
        }
    }
}

impl DataObjectTrait for DataObject {
}
impl DataObjectTrait for AboutInfo {
}
impl DataObjectTrait for AuthorizationDescription {
}
impl DataObjectTrait for EntityPrivilege {
}
impl DataObjectTrait for Permission {
}
impl DataObjectTrait for AuthorizationPrivilege {
}
impl DataObjectTrait for PrivilegeAvailability {
}
impl DataObjectTrait for AuthorizationRole {
}
impl DataObjectTrait for UserPrivilegeResult {
}
impl DataObjectTrait for BatchResult {
}
impl DataObjectTrait for Capability {
}
impl DataObjectTrait for ClusterComputeResourceClusterConfigResult {
}
impl DataObjectTrait for ClusterComputeResourceDvsSetting {
}
impl DataObjectTrait for ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping {
}
impl DataObjectTrait for ClusterComputeResourceDvsProfile {
}
impl DataObjectTrait for ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping {
}
impl DataObjectTrait for ClusterComputeResourceHciConfigInfo {
}
impl DataObjectTrait for ClusterComputeResourceHciConfigSpec {
}
impl DataObjectTrait for ClusterComputeResourceHostConfigurationInput {
}
impl DataObjectTrait for ClusterComputeResourceHostConfigurationProfile {
}
impl DataObjectTrait for ClusterComputeResourceHostVmkNicInfo {
}
impl DataObjectTrait for ClusterComputeResourceVcProfile {
}
impl DataObjectTrait for ClusterComputeResourceValidationResultBase {
}
impl DataObjectTrait for ClusterComputeResourceDvsConfigurationValidation {
}
impl DataObjectTrait for ClusterComputeResourceHostConfigurationValidation {
}
impl DataObjectTrait for ClusterComputeResourceVcsSlots {
}
impl DataObjectTrait for ComputeResourceConfigInfo {
}
impl DataObjectTrait for ClusterConfigInfoEx {
}
impl DataObjectTrait for ComputeResourceConfigSpec {
}
impl DataObjectTrait for ClusterConfigSpecEx {
}
impl DataObjectTrait for ComputeResourceHostSpbmLicenseInfo {
}
impl DataObjectTrait for ComputeResourceSummary {
}
impl DataObjectTrait for ClusterComputeResourceSummary {
}
impl DataObjectTrait for CustomFieldDef {
}
impl DataObjectTrait for CustomFieldValue {
}
impl DataObjectTrait for CustomFieldStringValue {
}
impl DataObjectTrait for CustomizationSpecInfo {
}
impl DataObjectTrait for CustomizationSpecItem {
}
impl DataObjectTrait for DatacenterBasicConnectInfo {
}
impl DataObjectTrait for DatacenterConfigInfo {
}
impl DataObjectTrait for DatacenterConfigSpec {
}
impl DataObjectTrait for DatastoreCapability {
}
impl DataObjectTrait for DatastoreHostMount {
}
impl DataObjectTrait for DatastoreInfo {
}
impl DataObjectTrait for LocalDatastoreInfo {
}
impl DataObjectTrait for NasDatastoreInfo {
}
impl DataObjectTrait for PMemDatastoreInfo {
}
impl DataObjectTrait for VmfsDatastoreInfo {
}
impl DataObjectTrait for VsanDatastoreInfo {
}
impl DataObjectTrait for VvolDatastoreInfo {
}
impl DataObjectTrait for DatastoreMountPathDatastorePair {
}
impl DataObjectTrait for DatastoreSummary {
}
impl DataObjectTrait for DatastoreVVolContainerFailoverPair {
}
impl DataObjectTrait for DatastoreNamespaceManagerDirectoryInfo {
}
impl DataObjectTrait for Description {
}
impl DataObjectTrait for ElementDescription {
}
impl DataObjectTrait for EvcMode {
}
impl DataObjectTrait for ExtendedElementDescription {
}
impl DataObjectTrait for FeatureEvcMode {
}
impl DataObjectTrait for OptionDef {
}
impl DataObjectTrait for ExtendedDescription {
}
impl DataObjectTrait for MethodDescription {
}
impl DataObjectTrait for TypeDescription {
}
impl DataObjectTrait for ScheduledTaskDetail {
}
impl DataObjectTrait for DesiredSoftwareSpec {
}
impl DataObjectTrait for DesiredSoftwareSpecBaseImageSpec {
}
impl DataObjectTrait for DesiredSoftwareSpecComponentSpec {
}
impl DataObjectTrait for DesiredSoftwareSpecVendorAddOnSpec {
}
impl DataObjectTrait for DiagnosticManagerAuditRecordResult {
}
impl DataObjectTrait for DiagnosticManagerBundleInfo {
}
impl DataObjectTrait for DiagnosticManagerLogDescriptor {
}
impl DataObjectTrait for DiagnosticManagerLogHeader {
}
impl DataObjectTrait for DvsBackupRestoreCapability {
}
impl DataObjectTrait for DvsCapability {
}
impl DataObjectTrait for DvsConfigInfo {
}
impl DataObjectTrait for VMwareDvsConfigInfo {
}
impl DataObjectTrait for DvsConfigSpec {
}
impl DataObjectTrait for VMwareDvsConfigSpec {
}
impl DataObjectTrait for DvsContactInfo {
}
impl DataObjectTrait for DvsCreateSpec {
}
impl DataObjectTrait for DvsFeatureCapability {
}
impl DataObjectTrait for VMwareDvsFeatureCapability {
}
impl DataObjectTrait for DvsHealthCheckConfig {
}
impl DataObjectTrait for VMwareDvsHealthCheckConfig {
}
impl DataObjectTrait for VMwareDvsTeamingHealthCheckConfig {
}
impl DataObjectTrait for VMwareDvsVlanMtuHealthCheckConfig {
}
impl DataObjectTrait for DvsHealthCheckCapability {
}
impl DataObjectTrait for VMwareDvsHealthCheckCapability {
}
impl DataObjectTrait for DvsHostInfrastructureTrafficResource {
}
impl DataObjectTrait for DvsHostInfrastructureTrafficResourceAllocation {
}
impl DataObjectTrait for DvsNetworkResourceManagementCapability {
}
impl DataObjectTrait for DvsResourceRuntimeInfo {
}
impl DataObjectTrait for DvsRollbackCapability {
}
impl DataObjectTrait for DvsRuntimeInfo {
}
impl DataObjectTrait for DvsSummary {
}
impl DataObjectTrait for DvsPolicy {
}
impl DataObjectTrait for DvsUplinkPortPolicy {
}
impl DataObjectTrait for DvsNameArrayUplinkPortPolicy {
}
impl DataObjectTrait for EnumDescription {
}
impl DataObjectTrait for EnvironmentBrowserConfigOptionQuerySpec {
}
impl DataObjectTrait for Extension {
}
impl DataObjectTrait for ExtensionClientInfo {
}
impl DataObjectTrait for ExtensionEventTypeInfo {
}
impl DataObjectTrait for ExtensionFaultTypeInfo {
}
impl DataObjectTrait for ExtensionHealthInfo {
}
impl DataObjectTrait for ExtensionOvfConsumerInfo {
}
impl DataObjectTrait for ExtensionPrivilegeInfo {
}
impl DataObjectTrait for ExtensionResourceInfo {
}
impl DataObjectTrait for ExtensionServerInfo {
}
impl DataObjectTrait for ExtensionTaskTypeInfo {
}
impl DataObjectTrait for ExtensionManagerIpAllocationUsage {
}
impl DataObjectTrait for FaultsByHost {
}
impl DataObjectTrait for FaultsByVm {
}
impl DataObjectTrait for FileLockInfo {
}
impl DataObjectTrait for FileLockInfoResult {
}
impl DataObjectTrait for FolderBatchAddHostsToClusterResult {
}
impl DataObjectTrait for FolderBatchAddStandaloneHostsResult {
}
impl DataObjectTrait for FolderFailedHostResult {
}
impl DataObjectTrait for FolderNewHostSpec {
}
impl DataObjectTrait for HbrManagerReplicationVmInfo {
}
impl DataObjectTrait for ReplicationVmProgressInfo {
}
impl DataObjectTrait for HbrManagerVmReplicationCapability {
}
impl DataObjectTrait for HealthUpdate {
}
impl DataObjectTrait for HealthUpdateInfo {
}
impl DataObjectTrait for PerfInterval {
}
impl DataObjectTrait for HostServiceTicket {
}
impl DataObjectTrait for HostSystemComplianceCheckState {
}
impl DataObjectTrait for HostSystemReconnectSpec {
}
impl DataObjectTrait for HostSystemRemediationState {
}
impl DataObjectTrait for HttpNfcLeaseCapabilities {
}
impl DataObjectTrait for HttpNfcLeaseDatastoreLeaseInfo {
}
impl DataObjectTrait for HttpNfcLeaseDeviceUrl {
}
impl DataObjectTrait for HttpNfcLeaseHostInfo {
}
impl DataObjectTrait for HttpNfcLeaseInfo {
}
impl DataObjectTrait for HttpNfcLeaseManifestEntry {
}
impl DataObjectTrait for HttpNfcLeaseProbeResult {
}
impl DataObjectTrait for HttpNfcLeaseSourceFile {
}
impl DataObjectTrait for ImportSpec {
}
impl DataObjectTrait for VirtualAppImportSpec {
}
impl DataObjectTrait for VirtualMachineImportSpec {
}
impl DataObjectTrait for InheritablePolicy {
}
impl DataObjectTrait for BoolPolicy {
}
impl DataObjectTrait for IntPolicy {
}
impl DataObjectTrait for LongPolicy {
}
impl DataObjectTrait for StringPolicy {
}
impl DataObjectTrait for DvsFilterConfig {
}
impl DataObjectTrait for DvsFilterConfigSpec {
}
impl DataObjectTrait for DvsTrafficFilterConfig {
}
impl DataObjectTrait for DvsTrafficFilterConfigSpec {
}
impl DataObjectTrait for DvsFilterPolicy {
}
impl DataObjectTrait for DvsTrafficShapingPolicy {
}
impl DataObjectTrait for DvsVendorSpecificConfig {
}
impl DataObjectTrait for DvsFailureCriteria {
}
impl DataObjectTrait for DvsMacLearningPolicy {
}
impl DataObjectTrait for DvsMacManagementPolicy {
}
impl DataObjectTrait for DvsSecurityPolicy {
}
impl DataObjectTrait for VMwareUplinkLacpPolicy {
}
impl DataObjectTrait for VMwareUplinkPortOrderPolicy {
}
impl DataObjectTrait for VmwareUplinkPortTeamingPolicy {
}
impl DataObjectTrait for VmwareDistributedVirtualSwitchVlanSpec {
}
impl DataObjectTrait for VmwareDistributedVirtualSwitchPvlanSpec {
}
impl DataObjectTrait for VmwareDistributedVirtualSwitchTrunkVlanSpec {
}
impl DataObjectTrait for VmwareDistributedVirtualSwitchVlanIdSpec {
}
impl DataObjectTrait for IoFilterInfo {
}
impl DataObjectTrait for ClusterIoFilterInfo {
}
impl DataObjectTrait for HostIoFilterInfo {
}
impl DataObjectTrait for IoFilterQueryIssueResult {
}
impl DataObjectTrait for IoFilterHostIssue {
}
impl DataObjectTrait for IpPoolManagerIpAllocation {
}
impl DataObjectTrait for KeyValue {
}
impl DataObjectTrait for LatencySensitivity {
}
impl DataObjectTrait for LicenseAssignmentManagerLicenseAssignment {
}
impl DataObjectTrait for LicenseAvailabilityInfo {
}
impl DataObjectTrait for LicenseDiagnostics {
}
impl DataObjectTrait for LicenseManagerEvaluationInfo {
}
impl DataObjectTrait for LicenseFeatureInfo {
}
impl DataObjectTrait for HostLicensableResourceInfo {
}
impl DataObjectTrait for LicenseManagerLicenseInfo {
}
impl DataObjectTrait for LicenseSource {
}
impl DataObjectTrait for EvaluationLicenseSource {
}
impl DataObjectTrait for LicenseServerSource {
}
impl DataObjectTrait for LocalLicenseSource {
}
impl DataObjectTrait for LicenseUsageInfo {
}
impl DataObjectTrait for LicenseReservationInfo {
}
impl DataObjectTrait for LocalizationManagerMessageCatalog {
}
impl DataObjectTrait for NegatableExpression {
}
impl DataObjectTrait for IntExpression {
}
impl DataObjectTrait for IpAddress {
}
impl DataObjectTrait for IpRange {
}
impl DataObjectTrait for SingleIp {
}
impl DataObjectTrait for MacAddress {
}
impl DataObjectTrait for MacRange {
}
impl DataObjectTrait for SingleMac {
}
impl DataObjectTrait for StringExpression {
}
impl DataObjectTrait for DvsIpPort {
}
impl DataObjectTrait for DvsIpPortRange {
}
impl DataObjectTrait for DvsSingleIpPort {
}
impl DataObjectTrait for NetworkSummary {
}
impl DataObjectTrait for OpaqueNetworkSummary {
}
impl DataObjectTrait for NumericRange {
}
impl DataObjectTrait for OpaqueNetworkCapability {
}
impl DataObjectTrait for OvfConsumerOstNode {
}
impl DataObjectTrait for OvfConsumerOvfSection {
}
impl DataObjectTrait for OvfManagerCommonParams {
}
impl DataObjectTrait for OvfCreateImportSpecParams {
}
impl DataObjectTrait for OvfParseDescriptorParams {
}
impl DataObjectTrait for OvfValidateHostParams {
}
impl DataObjectTrait for OvfCreateDescriptorParams {
}
impl DataObjectTrait for OvfCreateDescriptorResult {
}
impl DataObjectTrait for OvfCreateImportSpecResult {
}
impl DataObjectTrait for OvfDeploymentOption {
}
impl DataObjectTrait for OvfFileItem {
}
impl DataObjectTrait for OvfNetworkInfo {
}
impl DataObjectTrait for OvfNetworkMapping {
}
impl DataObjectTrait for OvfFile {
}
impl DataObjectTrait for OvfOptionInfo {
}
impl DataObjectTrait for OvfParseDescriptorResult {
}
impl DataObjectTrait for OvfResourceMap {
}
impl DataObjectTrait for OvfValidateHostResult {
}
impl DataObjectTrait for PasswordField {
}
impl DataObjectTrait for PerformanceDescription {
}
impl DataObjectTrait for PerfCompositeMetric {
}
impl DataObjectTrait for PerfCounterInfo {
}
impl DataObjectTrait for PerformanceManagerCounterLevelMapping {
}
impl DataObjectTrait for PerfEntityMetricBase {
}
impl DataObjectTrait for PerfEntityMetric {
}
impl DataObjectTrait for PerfEntityMetricCsv {
}
impl DataObjectTrait for PerfMetricId {
}
impl DataObjectTrait for PerfMetricSeries {
}
impl DataObjectTrait for PerfMetricIntSeries {
}
impl DataObjectTrait for PerfMetricSeriesCsv {
}
impl DataObjectTrait for PerfProviderSummary {
}
impl DataObjectTrait for PerfQuerySpec {
}
impl DataObjectTrait for PerfSampleInfo {
}
impl DataObjectTrait for PrivilegePolicyDef {
}
impl DataObjectTrait for ResourceAllocationInfo {
}
impl DataObjectTrait for ResourceAllocationOption {
}
impl DataObjectTrait for ResourceConfigOption {
}
impl DataObjectTrait for ResourceConfigSpec {
}
impl DataObjectTrait for DatabaseSizeEstimate {
}
impl DataObjectTrait for DatabaseSizeParam {
}
impl DataObjectTrait for InventoryDescription {
}
impl DataObjectTrait for PerformanceStatisticsDescription {
}
impl DataObjectTrait for ResourcePoolResourceUsage {
}
impl DataObjectTrait for ResourcePoolRuntimeInfo {
}
impl DataObjectTrait for ResourcePoolSummary {
}
impl DataObjectTrait for VirtualAppSummary {
}
impl DataObjectTrait for ResourcePoolQuickStats {
}
impl DataObjectTrait for SddcBase {
}
impl DataObjectTrait for SelectionSet {
}
impl DataObjectTrait for DvPortgroupSelection {
}
impl DataObjectTrait for DvsSelection {
}
impl DataObjectTrait for HostVMotionCompatibility {
}
impl DataObjectTrait for ProductComponentInfo {
}
impl DataObjectTrait for ServiceContent {
}
impl DataObjectTrait for ServiceLocator {
}
impl DataObjectTrait for ServiceLocatorCredential {
}
impl DataObjectTrait for ServiceLocatorNamePassword {
}
impl DataObjectTrait for ServiceLocatorSamlCredential {
}
impl DataObjectTrait for ServiceManagerServiceInfo {
}
impl DataObjectTrait for SessionManagerGenericServiceTicket {
}
impl DataObjectTrait for SessionManagerLocalTicket {
}
impl DataObjectTrait for SessionManagerServiceRequestSpec {
}
impl DataObjectTrait for SessionManagerHttpServiceRequestSpec {
}
impl DataObjectTrait for SessionManagerVmomiServiceRequestSpec {
}
impl DataObjectTrait for SharesInfo {
}
impl DataObjectTrait for SharesOption {
}
impl DataObjectTrait for SiteInfo {
}
impl DataObjectTrait for StoragePodSummary {
}
impl DataObjectTrait for StorageIoAllocationInfo {
}
impl DataObjectTrait for StorageIoAllocationOption {
}
impl DataObjectTrait for StorageIormInfo {
}
impl DataObjectTrait for StorageIormConfigOption {
}
impl DataObjectTrait for StorageIormConfigSpec {
}
impl DataObjectTrait for PodStorageDrsEntry {
}
impl DataObjectTrait for StoragePerformanceSummary {
}
impl DataObjectTrait for StorageResourceManagerStorageProfileStatistics {
}
impl DataObjectTrait for Tag {
}
impl DataObjectTrait for TaskDescription {
}
impl DataObjectTrait for TaskFilterSpec {
}
impl DataObjectTrait for TaskFilterSpecByEntity {
}
impl DataObjectTrait for TaskFilterSpecByTime {
}
impl DataObjectTrait for TaskFilterSpecByUsername {
}
impl DataObjectTrait for TaskInfo {
}
impl DataObjectTrait for TaskReason {
}
impl DataObjectTrait for TaskReasonAlarm {
}
impl DataObjectTrait for TaskReasonSchedule {
}
impl DataObjectTrait for TaskReasonSystem {
}
impl DataObjectTrait for TaskReasonUser {
}
impl DataObjectTrait for UpdateVirtualMachineFilesResult {
}
impl DataObjectTrait for UpdateVirtualMachineFilesResultFailedVmFileInfo {
}
impl DataObjectTrait for UserSearchResult {
}
impl DataObjectTrait for PosixUserSearchResult {
}
impl DataObjectTrait for UserSession {
}
impl DataObjectTrait for VVolVmConfigFileUpdateResult {
}
impl DataObjectTrait for VVolVmConfigFileUpdateResultFailedVmConfigFileInfo {
}
impl DataObjectTrait for VasaStorageArray {
}
impl DataObjectTrait for VasaStorageArrayDiscoveryFcTransport {
}
impl DataObjectTrait for VasaStorageArrayDiscoveryIpTransport {
}
impl DataObjectTrait for VasaStorageArrayDiscoverySvcInfo {
}
impl DataObjectTrait for VasaProviderContainerSpec {
}
impl DataObjectTrait for VimVasaProvider {
}
impl DataObjectTrait for VimVasaProviderStatePerArray {
}
impl DataObjectTrait for VimVasaProviderVirtualHostConfig {
}
impl DataObjectTrait for VimVasaProviderInfo {
}
impl DataObjectTrait for VirtualAppLinkInfo {
}
impl DataObjectTrait for VirtualDiskSpec {
}
impl DataObjectTrait for DeviceBackedVirtualDiskSpec {
}
impl DataObjectTrait for FileBackedVirtualDiskSpec {
}
impl DataObjectTrait for SeSparseVirtualDiskSpec {
}
impl DataObjectTrait for VirtualMachineConnection {
}
impl DataObjectTrait for VirtualMachineMksConnection {
}
impl DataObjectTrait for DiskChangeInfo {
}
impl DataObjectTrait for DiskChangeExtent {
}
impl DataObjectTrait for VirtualMachineDisplayTopology {
}
impl DataObjectTrait for VirtualMachineMksTicket {
}
impl DataObjectTrait for StorageRequirement {
}
impl DataObjectTrait for VirtualMachineTicket {
}
impl DataObjectTrait for VirtualMachineWipeResult {
}
impl DataObjectTrait for VsanUpgradeSystemNetworkPartitionInfo {
}
impl DataObjectTrait for VsanUpgradeSystemPreflightCheckIssue {
}
impl DataObjectTrait for VsanUpgradeSystemApiBrokenIssue {
}
impl DataObjectTrait for VsanUpgradeSystemAutoClaimEnabledOnHostsIssue {
}
impl DataObjectTrait for VsanUpgradeSystemHostsDisconnectedIssue {
}
impl DataObjectTrait for VsanUpgradeSystemMissingHostsInClusterIssue {
}
impl DataObjectTrait for VsanUpgradeSystemNetworkPartitionIssue {
}
impl DataObjectTrait for VsanUpgradeSystemNotEnoughFreeCapacityIssue {
}
impl DataObjectTrait for VsanUpgradeSystemRogueHostsInClusterIssue {
}
impl DataObjectTrait for VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue {
}
impl DataObjectTrait for VsanUpgradeSystemWrongEsxVersionIssue {
}
impl DataObjectTrait for VsanUpgradeSystemPreflightCheckResult {
}
impl DataObjectTrait for VsanUpgradeSystemUpgradeHistoryItem {
}
impl DataObjectTrait for VsanUpgradeSystemUpgradeHistoryDiskGroupOp {
}
impl DataObjectTrait for VsanUpgradeSystemUpgradeHistoryPreflightFail {
}
impl DataObjectTrait for VsanUpgradeSystemUpgradeStatus {
}
impl DataObjectTrait for Action {
}
impl DataObjectTrait for CreateTaskAction {
}
impl DataObjectTrait for MethodAction {
}
impl DataObjectTrait for RunScriptAction {
}
impl DataObjectTrait for SendEmailAction {
}
impl DataObjectTrait for SendSnmpAction {
}
impl DataObjectTrait for MethodActionArgument {
}
impl DataObjectTrait for AlarmAction {
}
impl DataObjectTrait for AlarmTriggeringAction {
}
impl DataObjectTrait for GroupAlarmAction {
}
impl DataObjectTrait for AlarmDescription {
}
impl DataObjectTrait for AlarmExpression {
}
impl DataObjectTrait for AndAlarmExpression {
}
impl DataObjectTrait for EventAlarmExpression {
}
impl DataObjectTrait for MetricAlarmExpression {
}
impl DataObjectTrait for OrAlarmExpression {
}
impl DataObjectTrait for StateAlarmExpression {
}
impl DataObjectTrait for AlarmFilterSpec {
}
impl DataObjectTrait for AlarmSetting {
}
impl DataObjectTrait for AlarmSpec {
}
impl DataObjectTrait for AlarmInfo {
}
impl DataObjectTrait for AlarmState {
}
impl DataObjectTrait for AlarmTriggeringActionTransitionSpec {
}
impl DataObjectTrait for EventAlarmExpressionComparison {
}
impl DataObjectTrait for ClusterAction {
}
impl DataObjectTrait for ClusterClusterInitialPlacementAction {
}
impl DataObjectTrait for ClusterHostInfraUpdateHaModeAction {
}
impl DataObjectTrait for ClusterHostPowerAction {
}
impl DataObjectTrait for ClusterInitialPlacementAction {
}
impl DataObjectTrait for ClusterMigrationAction {
}
impl DataObjectTrait for PlacementAction {
}
impl DataObjectTrait for HbrDiskMigrationAction {
}
impl DataObjectTrait for StorageMigrationAction {
}
impl DataObjectTrait for StoragePlacementAction {
}
impl DataObjectTrait for ClusterActionHistory {
}
impl DataObjectTrait for ClusterAttemptedVmInfo {
}
impl DataObjectTrait for ClusterConfigInfo {
}
impl DataObjectTrait for ClusterConfigSpec {
}
impl DataObjectTrait for ClusterCryptoConfigInfo {
}
impl DataObjectTrait for ClusterDasAamNodeState {
}
impl DataObjectTrait for ClusterDasAdmissionControlInfo {
}
impl DataObjectTrait for ClusterFailoverHostAdmissionControlInfo {
}
impl DataObjectTrait for ClusterFailoverLevelAdmissionControlInfo {
}
impl DataObjectTrait for ClusterFailoverResourcesAdmissionControlInfo {
}
impl DataObjectTrait for ClusterDasAdmissionControlPolicy {
}
impl DataObjectTrait for ClusterFailoverHostAdmissionControlPolicy {
}
impl DataObjectTrait for ClusterFailoverLevelAdmissionControlPolicy {
}
impl DataObjectTrait for ClusterFailoverResourcesAdmissionControlPolicy {
}
impl DataObjectTrait for ClusterDasAdvancedRuntimeInfo {
}
impl DataObjectTrait for ClusterDasFailoverLevelAdvancedRuntimeInfo {
}
impl DataObjectTrait for DasHeartbeatDatastoreInfo {
}
impl DataObjectTrait for ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo {
}
impl DataObjectTrait for ClusterDasConfigInfo {
}
impl DataObjectTrait for ClusterDasData {
}
impl DataObjectTrait for ClusterDasDataSummary {
}
impl DataObjectTrait for ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots {
}
impl DataObjectTrait for ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo {
}
impl DataObjectTrait for ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots {
}
impl DataObjectTrait for ClusterDasFdmHostState {
}
impl DataObjectTrait for ClusterDasHostInfo {
}
impl DataObjectTrait for ClusterDasAamHostInfo {
}
impl DataObjectTrait for ClusterDasHostRecommendation {
}
impl DataObjectTrait for ClusterDasVmConfigInfo {
}
impl DataObjectTrait for ClusterDasVmSettings {
}
impl DataObjectTrait for ClusterDpmConfigInfo {
}
impl DataObjectTrait for ClusterDpmHostConfigInfo {
}
impl DataObjectTrait for ClusterDrsConfigInfo {
}
impl DataObjectTrait for ClusterDrsFaults {
}
impl DataObjectTrait for ClusterDrsFaultsFaultsByVm {
}
impl DataObjectTrait for ClusterDrsFaultsFaultsByVirtualDisk {
}
impl DataObjectTrait for ClusterDrsMigration {
}
impl DataObjectTrait for ClusterDrsRecommendation {
}
impl DataObjectTrait for ClusterDrsVmConfigInfo {
}
impl DataObjectTrait for ClusterEvcManagerCheckResult {
}
impl DataObjectTrait for ClusterEvcManagerEvcState {
}
impl DataObjectTrait for ClusterEnterMaintenanceResult {
}
impl DataObjectTrait for ClusterFailoverHostAdmissionControlInfoHostStatus {
}
impl DataObjectTrait for ClusterGroupInfo {
}
impl DataObjectTrait for ClusterHostGroup {
}
impl DataObjectTrait for ClusterVmGroup {
}
impl DataObjectTrait for ClusterHostRecommendation {
}
impl DataObjectTrait for ClusterInfraUpdateHaConfigInfo {
}
impl DataObjectTrait for ClusterNotAttemptedVmInfo {
}
impl DataObjectTrait for ClusterOrchestrationInfo {
}
impl DataObjectTrait for PlacementResult {
}
impl DataObjectTrait for PlacementSpec {
}
impl DataObjectTrait for ClusterPowerOnVmResult {
}
impl DataObjectTrait for ClusterPreemptibleVmPairInfo {
}
impl DataObjectTrait for ClusterProactiveDrsConfigInfo {
}
impl DataObjectTrait for ClusterRecommendation {
}
impl DataObjectTrait for ClusterResourceUsageSummary {
}
impl DataObjectTrait for ClusterRuleInfo {
}
impl DataObjectTrait for ClusterAffinityRuleSpec {
}
impl DataObjectTrait for ClusterAntiAffinityRuleSpec {
}
impl DataObjectTrait for ClusterDependencyRuleInfo {
}
impl DataObjectTrait for ClusterVmHostRuleInfo {
}
impl DataObjectTrait for VirtualDiskAntiAffinityRuleSpec {
}
impl DataObjectTrait for VirtualDiskRuleSpec {
}
impl DataObjectTrait for ClusterSlotPolicy {
}
impl DataObjectTrait for ClusterFixedSizeSlotPolicy {
}
impl DataObjectTrait for ClusterSystemVMsConfigInfo {
}
impl DataObjectTrait for ClusterSystemVMsConfigSpec {
}
impl DataObjectTrait for ClusterUsageSummary {
}
impl DataObjectTrait for ClusterVmComponentProtectionSettings {
}
impl DataObjectTrait for ClusterVmOrchestrationInfo {
}
impl DataObjectTrait for ClusterVmReadiness {
}
impl DataObjectTrait for ClusterVmToolsMonitoringSettings {
}
impl DataObjectTrait for DistributedVirtualPort {
}
impl DataObjectTrait for DvPortConfigInfo {
}
impl DataObjectTrait for DvPortConfigSpec {
}
impl DataObjectTrait for DvsFilterParameter {
}
impl DataObjectTrait for DvsHostLocalPortInfo {
}
impl DataObjectTrait for DvPortStatus {
}
impl DataObjectTrait for DvPortSetting {
}
impl DataObjectTrait for VMwareDvsPortSetting {
}
impl DataObjectTrait for DvPortState {
}
impl DataObjectTrait for DvPortgroupConfigInfo {
}
impl DataObjectTrait for DvPortgroupConfigSpec {
}
impl DataObjectTrait for DistributedVirtualPortgroupNsxPortgroupOperationResult {
}
impl DataObjectTrait for DvPortgroupPolicy {
}
impl DataObjectTrait for VMwareDvsPortgroupPolicy {
}
impl DataObjectTrait for DistributedVirtualPortgroupProblem {
}
impl DataObjectTrait for DistributedVirtualPortgroupInfo {
}
impl DataObjectTrait for DistributedVirtualSwitchInfo {
}
impl DataObjectTrait for DistributedVirtualSwitchManagerCompatibilityResult {
}
impl DataObjectTrait for DvsManagerDvsConfigTarget {
}
impl DataObjectTrait for DistributedVirtualSwitchManagerDvsProductSpec {
}
impl DataObjectTrait for DistributedVirtualSwitchManagerHostContainer {
}
impl DataObjectTrait for DistributedVirtualSwitchManagerHostDvsFilterSpec {
}
impl DataObjectTrait for DistributedVirtualSwitchManagerHostArrayFilter {
}
impl DataObjectTrait for DistributedVirtualSwitchManagerHostContainerFilter {
}
impl DataObjectTrait for DistributedVirtualSwitchManagerHostDvsMembershipFilter {
}
impl DataObjectTrait for DistributedVirtualSwitchManagerImportResult {
}
impl DataObjectTrait for DvsManagerPhysicalNicsList {
}
impl DataObjectTrait for EntityBackup {
}
impl DataObjectTrait for EntityBackupConfig {
}
impl DataObjectTrait for DistributedVirtualSwitchHostMember {
}
impl DataObjectTrait for DistributedVirtualSwitchHostMemberBacking {
}
impl DataObjectTrait for DistributedVirtualSwitchHostMemberPnicBacking {
}
impl DataObjectTrait for DistributedVirtualSwitchHostMemberConfigInfo {
}
impl DataObjectTrait for DistributedVirtualSwitchHostMemberConfigSpec {
}
impl DataObjectTrait for HostMemberHealthCheckResult {
}
impl DataObjectTrait for HostMemberUplinkHealthCheckResult {
}
impl DataObjectTrait for VMwareDvsMtuHealthCheckResult {
}
impl DataObjectTrait for VMwareDvsVlanHealthCheckResult {
}
impl DataObjectTrait for VMwareDvsTeamingHealthCheckResult {
}
impl DataObjectTrait for DistributedVirtualSwitchHostMemberPnicSpec {
}
impl DataObjectTrait for HostMemberRuntimeInfo {
}
impl DataObjectTrait for DistributedVirtualSwitchHostMemberRuntimeState {
}
impl DataObjectTrait for DistributedVirtualSwitchHostMemberTransportZoneInfo {
}
impl DataObjectTrait for DistributedVirtualSwitchHostProductSpec {
}
impl DataObjectTrait for DistributedVirtualSwitchKeyedOpaqueBlob {
}
impl DataObjectTrait for DistributedVirtualSwitchNetworkOffloadSpec {
}
impl DataObjectTrait for DvsNetworkResourcePool {
}
impl DataObjectTrait for DvsNetworkResourcePoolAllocationInfo {
}
impl DataObjectTrait for DvsNetworkResourcePoolConfigSpec {
}
impl DataObjectTrait for DistributedVirtualSwitchPortConnectee {
}
impl DataObjectTrait for DistributedVirtualSwitchPortConnection {
}
impl DataObjectTrait for DistributedVirtualSwitchPortCriteria {
}
impl DataObjectTrait for DistributedVirtualSwitchPortStatistics {
}
impl DataObjectTrait for DistributedVirtualSwitchProductSpec {
}
impl DataObjectTrait for DvsTrafficRule {
}
impl DataObjectTrait for DvsNetworkRuleAction {
}
impl DataObjectTrait for DvsAcceptNetworkRuleAction {
}
impl DataObjectTrait for DvsCopyNetworkRuleAction {
}
impl DataObjectTrait for DvsDropNetworkRuleAction {
}
impl DataObjectTrait for DvsGreEncapNetworkRuleAction {
}
impl DataObjectTrait for DvsLogNetworkRuleAction {
}
impl DataObjectTrait for DvsMacRewriteNetworkRuleAction {
}
impl DataObjectTrait for DvsPuntNetworkRuleAction {
}
impl DataObjectTrait for DvsRateLimitNetworkRuleAction {
}
impl DataObjectTrait for DvsUpdateTagNetworkRuleAction {
}
impl DataObjectTrait for DvsNetworkRuleQualifier {
}
impl DataObjectTrait for DvsIpNetworkRuleQualifier {
}
impl DataObjectTrait for DvsMacNetworkRuleQualifier {
}
impl DataObjectTrait for DvsSystemTrafficNetworkRuleQualifier {
}
impl DataObjectTrait for DvsTrafficRuleset {
}
impl DataObjectTrait for DvsVmVnicNetworkResourcePool {
}
impl DataObjectTrait for DvsVmVnicResourcePoolConfigSpec {
}
impl DataObjectTrait for DvsVmVnicResourceAllocation {
}
impl DataObjectTrait for DvsVmVnicNetworkResourcePoolRuntimeInfo {
}
impl DataObjectTrait for DvsVnicAllocatedResource {
}
impl DataObjectTrait for VMwareDvsDpuCapability {
}
impl DataObjectTrait for VMwareIpfixConfig {
}
impl DataObjectTrait for VMwareDvsIpfixCapability {
}
impl DataObjectTrait for VMwareDvsLacpCapability {
}
impl DataObjectTrait for VMwareDvsLacpGroupConfig {
}
impl DataObjectTrait for VMwareDvsLacpGroupSpec {
}
impl DataObjectTrait for VMwareDvsLagIpfixConfig {
}
impl DataObjectTrait for VMwareDvsLagVlanConfig {
}
impl DataObjectTrait for VMwareDvsMtuCapability {
}
impl DataObjectTrait for VMwareDvsPvlanConfigSpec {
}
impl DataObjectTrait for VMwareDvsPvlanMapEntry {
}
impl DataObjectTrait for VMwareDvsVspanConfigSpec {
}
impl DataObjectTrait for VMwareDvsVspanCapability {
}
impl DataObjectTrait for VMwareVspanPort {
}
impl DataObjectTrait for VMwareVspanSession {
}
impl DataObjectTrait for CryptoKeyId {
}
impl DataObjectTrait for CryptoKeyPlain {
}
impl DataObjectTrait for CryptoKeyResult {
}
impl DataObjectTrait for CryptoManagerHostKeyStatus {
}
impl DataObjectTrait for CryptoManagerKmipCertSignRequest {
}
impl DataObjectTrait for CryptoManagerKmipCertificateInfo {
}
impl DataObjectTrait for CryptoManagerKmipClusterStatus {
}
impl DataObjectTrait for CryptoManagerKmipCryptoKeyStatus {
}
impl DataObjectTrait for CryptoManagerKmipCustomAttributeSpec {
}
impl DataObjectTrait for CryptoManagerKmipServerCertInfo {
}
impl DataObjectTrait for CryptoManagerKmipServerStatus {
}
impl DataObjectTrait for CryptoSpec {
}
impl DataObjectTrait for CryptoSpecDecrypt {
}
impl DataObjectTrait for CryptoSpecDeepRecrypt {
}
impl DataObjectTrait for CryptoSpecEncrypt {
}
impl DataObjectTrait for CryptoSpecNoOp {
}
impl DataObjectTrait for CryptoSpecRegister {
}
impl DataObjectTrait for CryptoSpecShallowRecrypt {
}
impl DataObjectTrait for KeyProviderId {
}
impl DataObjectTrait for KmipClusterInfo {
}
impl DataObjectTrait for KmipServerInfo {
}
impl DataObjectTrait for KmipServerSpec {
}
impl DataObjectTrait for KmipServerStatus {
}
impl DataObjectTrait for ChangesInfoEventArgument {
}
impl DataObjectTrait for DvsOutOfSyncHostArgument {
}
impl DataObjectTrait for Event {
}
impl DataObjectTrait for AlarmEvent {
}
impl DataObjectTrait for AlarmAcknowledgedEvent {
}
impl DataObjectTrait for AlarmActionTriggeredEvent {
}
impl DataObjectTrait for AlarmClearedEvent {
}
impl DataObjectTrait for AlarmCreatedEvent {
}
impl DataObjectTrait for AlarmEmailCompletedEvent {
}
impl DataObjectTrait for AlarmEmailFailedEvent {
}
impl DataObjectTrait for AlarmReconfiguredEvent {
}
impl DataObjectTrait for AlarmRemovedEvent {
}
impl DataObjectTrait for AlarmScriptCompleteEvent {
}
impl DataObjectTrait for AlarmScriptFailedEvent {
}
impl DataObjectTrait for AlarmSnmpCompletedEvent {
}
impl DataObjectTrait for AlarmSnmpFailedEvent {
}
impl DataObjectTrait for AlarmStatusChangedEvent {
}
impl DataObjectTrait for AuthorizationEvent {
}
impl DataObjectTrait for PermissionEvent {
}
impl DataObjectTrait for PermissionAddedEvent {
}
impl DataObjectTrait for PermissionRemovedEvent {
}
impl DataObjectTrait for PermissionUpdatedEvent {
}
impl DataObjectTrait for RoleEvent {
}
impl DataObjectTrait for RoleAddedEvent {
}
impl DataObjectTrait for RoleRemovedEvent {
}
impl DataObjectTrait for RoleUpdatedEvent {
}
impl DataObjectTrait for ClusterEvent {
}
impl DataObjectTrait for ClusterComplianceCheckedEvent {
}
impl DataObjectTrait for ClusterCreatedEvent {
}
impl DataObjectTrait for ClusterDestroyedEvent {
}
impl DataObjectTrait for ClusterOvercommittedEvent {
}
impl DataObjectTrait for HostOvercommittedEvent {
}
impl DataObjectTrait for ClusterReconfiguredEvent {
}
impl DataObjectTrait for ClusterStatusChangedEvent {
}
impl DataObjectTrait for HostStatusChangedEvent {
}
impl DataObjectTrait for DasAdmissionControlDisabledEvent {
}
impl DataObjectTrait for DasAdmissionControlEnabledEvent {
}
impl DataObjectTrait for DasAgentFoundEvent {
}
impl DataObjectTrait for DasAgentUnavailableEvent {
}
impl DataObjectTrait for DasClusterIsolatedEvent {
}
impl DataObjectTrait for DasDisabledEvent {
}
impl DataObjectTrait for DasEnabledEvent {
}
impl DataObjectTrait for DasHostFailedEvent {
}
impl DataObjectTrait for DasHostIsolatedEvent {
}
impl DataObjectTrait for DrsDisabledEvent {
}
impl DataObjectTrait for DrsEnabledEvent {
}
impl DataObjectTrait for DrsInvocationFailedEvent {
}
impl DataObjectTrait for DrsRecoveredFromFailureEvent {
}
impl DataObjectTrait for FailoverLevelRestored {
}
impl DataObjectTrait for HostMonitoringStateChangedEvent {
}
impl DataObjectTrait for InsufficientFailoverResourcesEvent {
}
impl DataObjectTrait for VmHealthMonitoringStateChangedEvent {
}
impl DataObjectTrait for CustomFieldEvent {
}
impl DataObjectTrait for CustomFieldDefEvent {
}
impl DataObjectTrait for CustomFieldDefAddedEvent {
}
impl DataObjectTrait for CustomFieldDefRemovedEvent {
}
impl DataObjectTrait for CustomFieldDefRenamedEvent {
}
impl DataObjectTrait for CustomFieldValueChangedEvent {
}
impl DataObjectTrait for DvPortgroupEvent {
}
impl DataObjectTrait for DvPortgroupCreatedEvent {
}
impl DataObjectTrait for DvPortgroupDestroyedEvent {
}
impl DataObjectTrait for DvPortgroupReconfiguredEvent {
}
impl DataObjectTrait for DvPortgroupRenamedEvent {
}
impl DataObjectTrait for DvpgImportEvent {
}
impl DataObjectTrait for DvpgRestoreEvent {
}
impl DataObjectTrait for DatacenterEvent {
}
impl DataObjectTrait for DatacenterCreatedEvent {
}
impl DataObjectTrait for DatacenterRenamedEvent {
}
impl DataObjectTrait for DatastoreEvent {
}
impl DataObjectTrait for DatastoreCapacityIncreasedEvent {
}
impl DataObjectTrait for DatastoreDestroyedEvent {
}
impl DataObjectTrait for DatastoreDuplicatedEvent {
}
impl DataObjectTrait for DatastoreFileEvent {
}
impl DataObjectTrait for DatastoreFileCopiedEvent {
}
impl DataObjectTrait for DatastoreFileDeletedEvent {
}
impl DataObjectTrait for DatastoreFileMovedEvent {
}
impl DataObjectTrait for DatastoreIormReconfiguredEvent {
}
impl DataObjectTrait for DatastoreRenamedEvent {
}
impl DataObjectTrait for NonViWorkloadDetectedOnDatastoreEvent {
}
impl DataObjectTrait for DvsEvent {
}
impl DataObjectTrait for DvsCreatedEvent {
}
impl DataObjectTrait for DvsDestroyedEvent {
}
impl DataObjectTrait for DvsHostBackInSyncEvent {
}
impl DataObjectTrait for DvsHostJoinedEvent {
}
impl DataObjectTrait for DvsHostLeftEvent {
}
impl DataObjectTrait for DvsHostStatusUpdated {
}
impl DataObjectTrait for DvsHostWentOutOfSyncEvent {
}
impl DataObjectTrait for DvsImportEvent {
}
impl DataObjectTrait for DvsMergedEvent {
}
impl DataObjectTrait for DvsPortBlockedEvent {
}
impl DataObjectTrait for DvsPortConnectedEvent {
}
impl DataObjectTrait for DvsPortCreatedEvent {
}
impl DataObjectTrait for DvsPortDeletedEvent {
}
impl DataObjectTrait for DvsPortDisconnectedEvent {
}
impl DataObjectTrait for DvsPortEnteredPassthruEvent {
}
impl DataObjectTrait for DvsPortExitedPassthruEvent {
}
impl DataObjectTrait for DvsPortJoinPortgroupEvent {
}
impl DataObjectTrait for DvsPortLeavePortgroupEvent {
}
impl DataObjectTrait for DvsPortLinkDownEvent {
}
impl DataObjectTrait for DvsPortLinkUpEvent {
}
impl DataObjectTrait for DvsPortReconfiguredEvent {
}
impl DataObjectTrait for DvsPortRuntimeChangeEvent {
}
impl DataObjectTrait for DvsPortUnblockedEvent {
}
impl DataObjectTrait for DvsPortVendorSpecificStateChangeEvent {
}
impl DataObjectTrait for DvsReconfiguredEvent {
}
impl DataObjectTrait for DvsRenamedEvent {
}
impl DataObjectTrait for DvsRestoreEvent {
}
impl DataObjectTrait for DvsUpgradeAvailableEvent {
}
impl DataObjectTrait for DvsUpgradeInProgressEvent {
}
impl DataObjectTrait for DvsUpgradeRejectedEvent {
}
impl DataObjectTrait for DvsUpgradedEvent {
}
impl DataObjectTrait for HostLocalPortCreatedEvent {
}
impl DataObjectTrait for OutOfSyncDvsHost {
}
impl DataObjectTrait for RecoveryEvent {
}
impl DataObjectTrait for RollbackEvent {
}
impl DataObjectTrait for VmVnicPoolReservationViolationClearEvent {
}
impl DataObjectTrait for VmVnicPoolReservationViolationRaiseEvent {
}
impl DataObjectTrait for EventEx {
}
impl DataObjectTrait for GeneralEvent {
}
impl DataObjectTrait for ExtendedEvent {
}
impl DataObjectTrait for GeneralHostErrorEvent {
}
impl DataObjectTrait for GeneralHostInfoEvent {
}
impl DataObjectTrait for GeneralHostWarningEvent {
}
impl DataObjectTrait for GeneralUserEvent {
}
impl DataObjectTrait for GeneralVmErrorEvent {
}
impl DataObjectTrait for GeneralVmInfoEvent {
}
impl DataObjectTrait for GeneralVmWarningEvent {
}
impl DataObjectTrait for HealthStatusChangedEvent {
}
impl DataObjectTrait for HostEvent {
}
impl DataObjectTrait for AccountCreatedEvent {
}
impl DataObjectTrait for AccountRemovedEvent {
}
impl DataObjectTrait for AccountUpdatedEvent {
}
impl DataObjectTrait for AdminPasswordNotChangedEvent {
}
impl DataObjectTrait for CanceledHostOperationEvent {
}
impl DataObjectTrait for DatastoreDiscoveredEvent {
}
impl DataObjectTrait for DatastorePrincipalConfigured {
}
impl DataObjectTrait for DatastoreRemovedOnHostEvent {
}
impl DataObjectTrait for DatastoreRenamedOnHostEvent {
}
impl DataObjectTrait for DrsResourceConfigureFailedEvent {
}
impl DataObjectTrait for DrsResourceConfigureSyncedEvent {
}
impl DataObjectTrait for DuplicateIpDetectedEvent {
}
impl DataObjectTrait for DvsHealthStatusChangeEvent {
}
impl DataObjectTrait for MtuMatchEvent {
}
impl DataObjectTrait for MtuMismatchEvent {
}
impl DataObjectTrait for TeamingMatchEvent {
}
impl DataObjectTrait for TeamingMisMatchEvent {
}
impl DataObjectTrait for UplinkPortMtuNotSupportEvent {
}
impl DataObjectTrait for UplinkPortMtuSupportEvent {
}
impl DataObjectTrait for UplinkPortVlanTrunkedEvent {
}
impl DataObjectTrait for UplinkPortVlanUntrunkedEvent {
}
impl DataObjectTrait for EnteredMaintenanceModeEvent {
}
impl DataObjectTrait for EnteredStandbyModeEvent {
}
impl DataObjectTrait for DrsEnteredStandbyModeEvent {
}
impl DataObjectTrait for EnteringMaintenanceModeEvent {
}
impl DataObjectTrait for EnteringStandbyModeEvent {
}
impl DataObjectTrait for DrsEnteringStandbyModeEvent {
}
impl DataObjectTrait for ExitMaintenanceModeEvent {
}
impl DataObjectTrait for ExitStandbyModeFailedEvent {
}
impl DataObjectTrait for DrsExitStandbyModeFailedEvent {
}
impl DataObjectTrait for ExitedStandbyModeEvent {
}
impl DataObjectTrait for DrsExitedStandbyModeEvent {
}
impl DataObjectTrait for ExitingStandbyModeEvent {
}
impl DataObjectTrait for DrsExitingStandbyModeEvent {
}
impl DataObjectTrait for GhostDvsProxySwitchDetectedEvent {
}
impl DataObjectTrait for GhostDvsProxySwitchRemovedEvent {
}
impl DataObjectTrait for HostAddFailedEvent {
}
impl DataObjectTrait for HostAddedEvent {
}
impl DataObjectTrait for HostAdminDisableEvent {
}
impl DataObjectTrait for HostAdminEnableEvent {
}
impl DataObjectTrait for HostCnxFailedAccountFailedEvent {
}
impl DataObjectTrait for HostCnxFailedAlreadyManagedEvent {
}
impl DataObjectTrait for HostCnxFailedBadCcagentEvent {
}
impl DataObjectTrait for HostCnxFailedBadUsernameEvent {
}
impl DataObjectTrait for HostCnxFailedBadVersionEvent {
}
impl DataObjectTrait for HostCnxFailedCcagentUpgradeEvent {
}
impl DataObjectTrait for HostCnxFailedEvent {
}
impl DataObjectTrait for HostCnxFailedNetworkErrorEvent {
}
impl DataObjectTrait for HostCnxFailedNoAccessEvent {
}
impl DataObjectTrait for HostCnxFailedNoConnectionEvent {
}
impl DataObjectTrait for HostCnxFailedNoLicenseEvent {
}
impl DataObjectTrait for HostCnxFailedNotFoundEvent {
}
impl DataObjectTrait for HostCnxFailedTimeoutEvent {
}
impl DataObjectTrait for HostComplianceCheckedEvent {
}
impl DataObjectTrait for HostCompliantEvent {
}
impl DataObjectTrait for HostConfigAppliedEvent {
}
impl DataObjectTrait for HostConnectedEvent {
}
impl DataObjectTrait for HostConnectionLostEvent {
}
impl DataObjectTrait for HostDasDisabledEvent {
}
impl DataObjectTrait for HostDasDisablingEvent {
}
impl DataObjectTrait for HostDasEnabledEvent {
}
impl DataObjectTrait for HostDasEnablingEvent {
}
impl DataObjectTrait for HostDasErrorEvent {
}
impl DataObjectTrait for HostDasEvent {
}
impl DataObjectTrait for HostExtraNetworksEvent {
}
impl DataObjectTrait for HostIsolationIpPingFailedEvent {
}
impl DataObjectTrait for HostMissingNetworksEvent {
}
impl DataObjectTrait for HostNoAvailableNetworksEvent {
}
impl DataObjectTrait for HostNoHaEnabledPortGroupsEvent {
}
impl DataObjectTrait for HostNoRedundantManagementNetworkEvent {
}
impl DataObjectTrait for HostNotInClusterEvent {
}
impl DataObjectTrait for HostPrimaryAgentNotShortNameEvent {
}
impl DataObjectTrait for HostShortNameInconsistentEvent {
}
impl DataObjectTrait for HostDasOkEvent {
}
impl DataObjectTrait for HostDisconnectedEvent {
}
impl DataObjectTrait for HostEnableAdminFailedEvent {
}
impl DataObjectTrait for HostGetShortNameFailedEvent {
}
impl DataObjectTrait for HostInAuditModeEvent {
}
impl DataObjectTrait for HostIpChangedEvent {
}
impl DataObjectTrait for HostIpInconsistentEvent {
}
impl DataObjectTrait for HostIpToShortNameFailedEvent {
}
impl DataObjectTrait for HostNonCompliantEvent {
}
impl DataObjectTrait for HostProfileAppliedEvent {
}
impl DataObjectTrait for HostReconnectionFailedEvent {
}
impl DataObjectTrait for HostRemovedEvent {
}
impl DataObjectTrait for HostShortNameToIpFailedEvent {
}
impl DataObjectTrait for HostShutdownEvent {
}
impl DataObjectTrait for HostSpecificationChangedEvent {
}
impl DataObjectTrait for HostSpecificationRequireEvent {
}
impl DataObjectTrait for HostSpecificationUpdateEvent {
}
impl DataObjectTrait for HostSubSpecificationDeleteEvent {
}
impl DataObjectTrait for HostSubSpecificationUpdateEvent {
}
impl DataObjectTrait for HostSyncFailedEvent {
}
impl DataObjectTrait for HostUpgradeFailedEvent {
}
impl DataObjectTrait for HostUserWorldSwapNotEnabledEvent {
}
impl DataObjectTrait for HostVnicConnectedToCustomizedDvPortEvent {
}
impl DataObjectTrait for HostWwnChangedEvent {
}
impl DataObjectTrait for HostWwnConflictEvent {
}
impl DataObjectTrait for LocalDatastoreCreatedEvent {
}
impl DataObjectTrait for LocalTsmEnabledEvent {
}
impl DataObjectTrait for NasDatastoreCreatedEvent {
}
impl DataObjectTrait for NoDatastoresConfiguredEvent {
}
impl DataObjectTrait for RemoteTsmEnabledEvent {
}
impl DataObjectTrait for TimedOutHostOperationEvent {
}
impl DataObjectTrait for UpdatedAgentBeingRestartedEvent {
}
impl DataObjectTrait for UserAssignedToGroup {
}
impl DataObjectTrait for UserPasswordChanged {
}
impl DataObjectTrait for UserUnassignedFromGroup {
}
impl DataObjectTrait for VmfsDatastoreCreatedEvent {
}
impl DataObjectTrait for VmfsDatastoreExpandedEvent {
}
impl DataObjectTrait for VmfsDatastoreExtendedEvent {
}
impl DataObjectTrait for VcAgentUninstallFailedEvent {
}
impl DataObjectTrait for VcAgentUninstalledEvent {
}
impl DataObjectTrait for VcAgentUpgradeFailedEvent {
}
impl DataObjectTrait for VcAgentUpgradedEvent {
}
impl DataObjectTrait for VimAccountPasswordChangedEvent {
}
impl DataObjectTrait for IScsiBootFailureEvent {
}
impl DataObjectTrait for HostInventoryUnreadableEvent {
}
impl DataObjectTrait for LicenseEvent {
}
impl DataObjectTrait for AllVirtualMachinesLicensedEvent {
}
impl DataObjectTrait for HostInventoryFullEvent {
}
impl DataObjectTrait for HostLicenseExpiredEvent {
}
impl DataObjectTrait for IncorrectHostInformationEvent {
}
impl DataObjectTrait for InvalidEditionEvent {
}
impl DataObjectTrait for LicenseNonComplianceEvent {
}
impl DataObjectTrait for LicenseRestrictedEvent {
}
impl DataObjectTrait for LicenseServerAvailableEvent {
}
impl DataObjectTrait for LicenseServerUnavailableEvent {
}
impl DataObjectTrait for NoLicenseEvent {
}
impl DataObjectTrait for ServerLicenseExpiredEvent {
}
impl DataObjectTrait for UnlicensedVirtualMachinesEvent {
}
impl DataObjectTrait for UnlicensedVirtualMachinesFoundEvent {
}
impl DataObjectTrait for VMotionLicenseExpiredEvent {
}
impl DataObjectTrait for LicenseExpiredEvent {
}
impl DataObjectTrait for LockerMisconfiguredEvent {
}
impl DataObjectTrait for LockerReconfiguredEvent {
}
impl DataObjectTrait for NetworkRollbackEvent {
}
impl DataObjectTrait for ProfileEvent {
}
impl DataObjectTrait for ProfileAssociatedEvent {
}
impl DataObjectTrait for ProfileChangedEvent {
}
impl DataObjectTrait for ProfileCreatedEvent {
}
impl DataObjectTrait for ProfileDissociatedEvent {
}
impl DataObjectTrait for ProfileReferenceHostChangedEvent {
}
impl DataObjectTrait for ProfileRemovedEvent {
}
impl DataObjectTrait for ResourcePoolEvent {
}
impl DataObjectTrait for ResourcePoolCreatedEvent {
}
impl DataObjectTrait for ResourcePoolDestroyedEvent {
}
impl DataObjectTrait for ResourcePoolMovedEvent {
}
impl DataObjectTrait for ResourcePoolReconfiguredEvent {
}
impl DataObjectTrait for ResourceViolatedEvent {
}
impl DataObjectTrait for ScheduledTaskEvent {
}
impl DataObjectTrait for ScheduledTaskCompletedEvent {
}
impl DataObjectTrait for ScheduledTaskCreatedEvent {
}
impl DataObjectTrait for ScheduledTaskEmailCompletedEvent {
}
impl DataObjectTrait for ScheduledTaskEmailFailedEvent {
}
impl DataObjectTrait for ScheduledTaskFailedEvent {
}
impl DataObjectTrait for ScheduledTaskReconfiguredEvent {
}
impl DataObjectTrait for ScheduledTaskRemovedEvent {
}
impl DataObjectTrait for ScheduledTaskStartedEvent {
}
impl DataObjectTrait for SessionEvent {
}
impl DataObjectTrait for AlreadyAuthenticatedSessionEvent {
}
impl DataObjectTrait for BadUsernameSessionEvent {
}
impl DataObjectTrait for GlobalMessageChangedEvent {
}
impl DataObjectTrait for NoAccessUserEvent {
}
impl DataObjectTrait for ServerStartedSessionEvent {
}
impl DataObjectTrait for SessionTerminatedEvent {
}
impl DataObjectTrait for UserLoginSessionEvent {
}
impl DataObjectTrait for UserLogoutSessionEvent {
}
impl DataObjectTrait for TaskEvent {
}
impl DataObjectTrait for TaskTimeoutEvent {
}
impl DataObjectTrait for TemplateUpgradeEvent {
}
impl DataObjectTrait for TemplateBeingUpgradedEvent {
}
impl DataObjectTrait for TemplateUpgradeFailedEvent {
}
impl DataObjectTrait for TemplateUpgradedEvent {
}
impl DataObjectTrait for UpgradeEvent {
}
impl DataObjectTrait for ErrorUpgradeEvent {
}
impl DataObjectTrait for InfoUpgradeEvent {
}
impl DataObjectTrait for UserUpgradeEvent {
}
impl DataObjectTrait for WarningUpgradeEvent {
}
impl DataObjectTrait for VmEvent {
}
impl DataObjectTrait for CustomizationEvent {
}
impl DataObjectTrait for CustomizationFailed {
}
impl DataObjectTrait for CustomizationLinuxIdentityFailed {
}
impl DataObjectTrait for CustomizationNetworkSetupFailed {
}
impl DataObjectTrait for CustomizationSysprepFailed {
}
impl DataObjectTrait for CustomizationUnknownFailure {
}
impl DataObjectTrait for CustomizationStartedEvent {
}
impl DataObjectTrait for CustomizationSucceeded {
}
impl DataObjectTrait for DrsRuleComplianceEvent {
}
impl DataObjectTrait for DrsRuleViolationEvent {
}
impl DataObjectTrait for DrsSoftRuleViolationEvent {
}
impl DataObjectTrait for MigrationEvent {
}
impl DataObjectTrait for MigrationErrorEvent {
}
impl DataObjectTrait for MigrationHostErrorEvent {
}
impl DataObjectTrait for MigrationHostWarningEvent {
}
impl DataObjectTrait for MigrationResourceErrorEvent {
}
impl DataObjectTrait for MigrationResourceWarningEvent {
}
impl DataObjectTrait for MigrationWarningEvent {
}
impl DataObjectTrait for NoMaintenanceModeDrsRecommendationForVm {
}
impl DataObjectTrait for NotEnoughResourcesToStartVmEvent {
}
impl DataObjectTrait for VmAcquiredMksTicketEvent {
}
impl DataObjectTrait for VmAcquiredTicketEvent {
}
impl DataObjectTrait for VmAutoRenameEvent {
}
impl DataObjectTrait for VmBeingCreatedEvent {
}
impl DataObjectTrait for VmBeingDeployedEvent {
}
impl DataObjectTrait for VmBeingHotMigratedEvent {
}
impl DataObjectTrait for VmBeingMigratedEvent {
}
impl DataObjectTrait for VmCloneEvent {
}
impl DataObjectTrait for VmBeingClonedEvent {
}
impl DataObjectTrait for VmBeingClonedNoFolderEvent {
}
impl DataObjectTrait for VmCloneFailedEvent {
}
impl DataObjectTrait for VmClonedEvent {
}
impl DataObjectTrait for VmConfigMissingEvent {
}
impl DataObjectTrait for VmConnectedEvent {
}
impl DataObjectTrait for VmCreatedEvent {
}
impl DataObjectTrait for VmDasBeingResetEvent {
}
impl DataObjectTrait for VmDasBeingResetWithScreenshotEvent {
}
impl DataObjectTrait for VmDasResetFailedEvent {
}
impl DataObjectTrait for VmDasUpdateErrorEvent {
}
impl DataObjectTrait for VmDasUpdateOkEvent {
}
impl DataObjectTrait for VmDateRolledBackEvent {
}
impl DataObjectTrait for VmDeployFailedEvent {
}
impl DataObjectTrait for VmDeployedEvent {
}
impl DataObjectTrait for VmDisconnectedEvent {
}
impl DataObjectTrait for VmDiscoveredEvent {
}
impl DataObjectTrait for VmDiskFailedEvent {
}
impl DataObjectTrait for VmEmigratingEvent {
}
impl DataObjectTrait for VmEndRecordingEvent {
}
impl DataObjectTrait for VmEndReplayingEvent {
}
impl DataObjectTrait for VmFailedMigrateEvent {
}
impl DataObjectTrait for VmFailedRelayoutEvent {
}
impl DataObjectTrait for VmFailedRelayoutOnVmfs2DatastoreEvent {
}
impl DataObjectTrait for VmFailedStartingSecondaryEvent {
}
impl DataObjectTrait for VmFailedToPowerOffEvent {
}
impl DataObjectTrait for VmFailedToPowerOnEvent {
}
impl DataObjectTrait for VmFailedToRebootGuestEvent {
}
impl DataObjectTrait for VmFailedToResetEvent {
}
impl DataObjectTrait for VmFailedToShutdownGuestEvent {
}
impl DataObjectTrait for VmFailedToStandbyGuestEvent {
}
impl DataObjectTrait for VmFailedToSuspendEvent {
}
impl DataObjectTrait for VmFailedUpdatingSecondaryConfig {
}
impl DataObjectTrait for VmFailoverFailed {
}
impl DataObjectTrait for VmFaultToleranceStateChangedEvent {
}
impl DataObjectTrait for VmFaultToleranceTurnedOffEvent {
}
impl DataObjectTrait for VmFaultToleranceVmTerminatedEvent {
}
impl DataObjectTrait for VmGuestOsCrashedEvent {
}
impl DataObjectTrait for VmGuestRebootEvent {
}
impl DataObjectTrait for VmGuestShutdownEvent {
}
impl DataObjectTrait for VmGuestStandbyEvent {
}
impl DataObjectTrait for VmInstanceUuidAssignedEvent {
}
impl DataObjectTrait for VmInstanceUuidChangedEvent {
}
impl DataObjectTrait for VmInstanceUuidConflictEvent {
}
impl DataObjectTrait for VmMacAssignedEvent {
}
impl DataObjectTrait for VmMacChangedEvent {
}
impl DataObjectTrait for VmMacConflictEvent {
}
impl DataObjectTrait for VmMaxFtRestartCountReached {
}
impl DataObjectTrait for VmMaxRestartCountReached {
}
impl DataObjectTrait for VmMessageErrorEvent {
}
impl DataObjectTrait for VmMessageEvent {
}
impl DataObjectTrait for VmMessageWarningEvent {
}
impl DataObjectTrait for VmMigratedEvent {
}
impl DataObjectTrait for DrsVmMigratedEvent {
}
impl DataObjectTrait for VmNoCompatibleHostForSecondaryEvent {
}
impl DataObjectTrait for VmNoNetworkAccessEvent {
}
impl DataObjectTrait for VmOrphanedEvent {
}
impl DataObjectTrait for VmPoweredOffEvent {
}
impl DataObjectTrait for VmPowerOffOnIsolationEvent {
}
impl DataObjectTrait for VmShutdownOnIsolationEvent {
}
impl DataObjectTrait for VmPoweredOnEvent {
}
impl DataObjectTrait for DrsVmPoweredOnEvent {
}
impl DataObjectTrait for VmRestartedOnAlternateHostEvent {
}
impl DataObjectTrait for VmPoweringOnWithCustomizedDvPortEvent {
}
impl DataObjectTrait for VmPrimaryFailoverEvent {
}
impl DataObjectTrait for VmReconfiguredEvent {
}
impl DataObjectTrait for VmRegisteredEvent {
}
impl DataObjectTrait for VmRelayoutSuccessfulEvent {
}
impl DataObjectTrait for VmRelayoutUpToDateEvent {
}
impl DataObjectTrait for VmReloadFromPathEvent {
}
impl DataObjectTrait for VmReloadFromPathFailedEvent {
}
impl DataObjectTrait for VmRelocateSpecEvent {
}
impl DataObjectTrait for VmBeingRelocatedEvent {
}
impl DataObjectTrait for VmRelocateFailedEvent {
}
impl DataObjectTrait for VmRelocatedEvent {
}
impl DataObjectTrait for VmRemoteConsoleConnectedEvent {
}
impl DataObjectTrait for VmRemoteConsoleDisconnectedEvent {
}
impl DataObjectTrait for VmRemovedEvent {
}
impl DataObjectTrait for VmRenamedEvent {
}
impl DataObjectTrait for VmRequirementsExceedCurrentEvcModeEvent {
}
impl DataObjectTrait for VmResettingEvent {
}
impl DataObjectTrait for VmResourcePoolMovedEvent {
}
impl DataObjectTrait for VmResourceReallocatedEvent {
}
impl DataObjectTrait for VmResumingEvent {
}
impl DataObjectTrait for VmSecondaryAddedEvent {
}
impl DataObjectTrait for VmSecondaryDisabledBySystemEvent {
}
impl DataObjectTrait for VmSecondaryDisabledEvent {
}
impl DataObjectTrait for VmSecondaryEnabledEvent {
}
impl DataObjectTrait for VmSecondaryStartedEvent {
}
impl DataObjectTrait for VmStartRecordingEvent {
}
impl DataObjectTrait for VmStartReplayingEvent {
}
impl DataObjectTrait for VmStartingEvent {
}
impl DataObjectTrait for VmUnsupportedStartingEvent {
}
impl DataObjectTrait for VmStartingSecondaryEvent {
}
impl DataObjectTrait for VmStaticMacConflictEvent {
}
impl DataObjectTrait for VmStoppingEvent {
}
impl DataObjectTrait for VmSuspendedEvent {
}
impl DataObjectTrait for VmSuspendingEvent {
}
impl DataObjectTrait for VmTimedoutStartingSecondaryEvent {
}
impl DataObjectTrait for VmUpgradeCompleteEvent {
}
impl DataObjectTrait for VmUpgradeFailedEvent {
}
impl DataObjectTrait for VmUpgradingEvent {
}
impl DataObjectTrait for VmUuidAssignedEvent {
}
impl DataObjectTrait for VmUuidChangedEvent {
}
impl DataObjectTrait for VmUuidConflictEvent {
}
impl DataObjectTrait for VmWwnAssignedEvent {
}
impl DataObjectTrait for VmWwnChangedEvent {
}
impl DataObjectTrait for VmWwnConflictEvent {
}
impl DataObjectTrait for EventArgument {
}
impl DataObjectTrait for EntityEventArgument {
}
impl DataObjectTrait for AlarmEventArgument {
}
impl DataObjectTrait for ComputeResourceEventArgument {
}
impl DataObjectTrait for DatacenterEventArgument {
}
impl DataObjectTrait for DatastoreEventArgument {
}
impl DataObjectTrait for DvsEventArgument {
}
impl DataObjectTrait for FolderEventArgument {
}
impl DataObjectTrait for HostEventArgument {
}
impl DataObjectTrait for ManagedEntityEventArgument {
}
impl DataObjectTrait for NetworkEventArgument {
}
impl DataObjectTrait for ResourcePoolEventArgument {
}
impl DataObjectTrait for ScheduledTaskEventArgument {
}
impl DataObjectTrait for VmEventArgument {
}
impl DataObjectTrait for ProfileEventArgument {
}
impl DataObjectTrait for RoleEventArgument {
}
impl DataObjectTrait for EventDescription {
}
impl DataObjectTrait for EventArgDesc {
}
impl DataObjectTrait for EventDescriptionEventDetail {
}
impl DataObjectTrait for EventFilterSpec {
}
impl DataObjectTrait for EventFilterSpecByEntity {
}
impl DataObjectTrait for EventFilterSpecByTime {
}
impl DataObjectTrait for EventFilterSpecByUsername {
}
impl DataObjectTrait for ExtendedEventPair {
}
impl DataObjectTrait for VnicPortArgument {
}
impl DataObjectTrait for ExtExtendedProductInfo {
}
impl DataObjectTrait for ManagedByInfo {
}
impl DataObjectTrait for ExtManagedEntityInfo {
}
impl DataObjectTrait for ExtSolutionManagerInfo {
}
impl DataObjectTrait for ExtSolutionManagerInfoTabInfo {
}
impl DataObjectTrait for AnswerFileUpdateFailure {
}
impl DataObjectTrait for ConflictingConfigurationConfig {
}
impl DataObjectTrait for DatacenterMismatchArgument {
}
impl DataObjectTrait for DvsApplyOperationFaultFaultOnObject {
}
impl DataObjectTrait for DvsOperationBulkFaultFaultOnHost {
}
impl DataObjectTrait for ImportOperationBulkFaultFaultOnImport {
}
impl DataObjectTrait for MultipleCertificatesVerifyFaultThumbprintData {
}
impl DataObjectTrait for NoPermissionEntityPrivileges {
}
impl DataObjectTrait for ProfileUpdateFailedUpdateFailure {
}
impl DataObjectTrait for HostActiveDirectory {
}
impl DataObjectTrait for HostActiveDirectorySpec {
}
impl DataObjectTrait for HostAssignableHardwareBinding {
}
impl DataObjectTrait for HostAssignableHardwareConfig {
}
impl DataObjectTrait for HostAssignableHardwareConfigAttributeOverride {
}
impl DataObjectTrait for HostAuthenticationManagerInfo {
}
impl DataObjectTrait for HostAuthenticationStoreInfo {
}
impl DataObjectTrait for HostDirectoryStoreInfo {
}
impl DataObjectTrait for HostActiveDirectoryInfo {
}
impl DataObjectTrait for HostLocalAuthenticationInfo {
}
impl DataObjectTrait for AutoStartPowerInfo {
}
impl DataObjectTrait for HostAutoStartManagerConfig {
}
impl DataObjectTrait for AutoStartDefaults {
}
impl DataObjectTrait for HostBiosInfo {
}
impl DataObjectTrait for HostBootDeviceInfo {
}
impl DataObjectTrait for HostBootDevice {
}
impl DataObjectTrait for HostCacheConfigurationInfo {
}
impl DataObjectTrait for HostCacheConfigurationSpec {
}
impl DataObjectTrait for HostCapability {
}
impl DataObjectTrait for HostCertificateManagerCertificateInfo {
}
impl DataObjectTrait for HostCertificateManagerCertificateSpec {
}
impl DataObjectTrait for HostConfigChange {
}
impl DataObjectTrait for HostConfigInfo {
}
impl DataObjectTrait for HostConfigManager {
}
impl DataObjectTrait for HostConfigSpec {
}
impl DataObjectTrait for HostConnectInfo {
}
impl DataObjectTrait for HostDatastoreConnectInfo {
}
impl DataObjectTrait for HostDatastoreExistsConnectInfo {
}
impl DataObjectTrait for HostDatastoreNameConflictConnectInfo {
}
impl DataObjectTrait for HostLicenseConnectInfo {
}
impl DataObjectTrait for HostConnectInfoNetworkInfo {
}
impl DataObjectTrait for HostNewNetworkConnectInfo {
}
impl DataObjectTrait for HostConnectSpec {
}
impl DataObjectTrait for HostCpuIdInfo {
}
impl DataObjectTrait for HostCpuInfo {
}
impl DataObjectTrait for HostCpuPackage {
}
impl DataObjectTrait for HostCpuPowerManagementInfo {
}
impl DataObjectTrait for HostHyperThreadScheduleInfo {
}
impl DataObjectTrait for HostDataTransportConnectionInfo {
}
impl DataObjectTrait for HostNfcConnectionInfo {
}
impl DataObjectTrait for FileInfo {
}
impl DataObjectTrait for FloppyImageFileInfo {
}
impl DataObjectTrait for FolderFileInfo {
}
impl DataObjectTrait for IsoImageFileInfo {
}
impl DataObjectTrait for VmConfigFileInfo {
}
impl DataObjectTrait for TemplateConfigFileInfo {
}
impl DataObjectTrait for VmDiskFileInfo {
}
impl DataObjectTrait for VmLogFileInfo {
}
impl DataObjectTrait for VmNvramFileInfo {
}
impl DataObjectTrait for VmSnapshotFileInfo {
}
impl DataObjectTrait for FileQueryFlags {
}
impl DataObjectTrait for FileQuery {
}
impl DataObjectTrait for FloppyImageFileQuery {
}
impl DataObjectTrait for FolderFileQuery {
}
impl DataObjectTrait for IsoImageFileQuery {
}
impl DataObjectTrait for VmConfigFileQuery {
}
impl DataObjectTrait for TemplateConfigFileQuery {
}
impl DataObjectTrait for VmDiskFileQuery {
}
impl DataObjectTrait for VmLogFileQuery {
}
impl DataObjectTrait for VmNvramFileQuery {
}
impl DataObjectTrait for VmSnapshotFileQuery {
}
impl DataObjectTrait for HostDatastoreBrowserSearchResults {
}
impl DataObjectTrait for HostDatastoreBrowserSearchSpec {
}
impl DataObjectTrait for VmConfigFileEncryptionInfo {
}
impl DataObjectTrait for VmConfigFileQueryFlags {
}
impl DataObjectTrait for VmConfigFileQueryFilter {
}
impl DataObjectTrait for VmDiskFileEncryptionInfo {
}
impl DataObjectTrait for VmDiskFileQueryFlags {
}
impl DataObjectTrait for VmDiskFileQueryFilter {
}
impl DataObjectTrait for HostDatastoreSystemCapabilities {
}
impl DataObjectTrait for HostDatastoreSystemDatastoreResult {
}
impl DataObjectTrait for HostDatastoreSystemVvolDatastoreSpec {
}
impl DataObjectTrait for HostDateTimeConfig {
}
impl DataObjectTrait for HostDateTimeInfo {
}
impl DataObjectTrait for HostDateTimeSystemServiceTestResult {
}
impl DataObjectTrait for HostDateTimeSystemTimeZone {
}
impl DataObjectTrait for HostDeploymentInfo {
}
impl DataObjectTrait for HostDevice {
}
impl DataObjectTrait for ScsiLun {
}
impl DataObjectTrait for HostScsiDisk {
}
impl DataObjectTrait for HostDhcpService {
}
impl DataObjectTrait for HostDhcpServiceConfig {
}
impl DataObjectTrait for HostDhcpServiceSpec {
}
impl DataObjectTrait for HostDiagnosticPartition {
}
impl DataObjectTrait for HostDiagnosticPartitionCreateDescription {
}
impl DataObjectTrait for HostDiagnosticPartitionCreateOption {
}
impl DataObjectTrait for HostDiagnosticPartitionCreateSpec {
}
impl DataObjectTrait for HostDigestInfo {
}
impl DataObjectTrait for HostTpmDigestInfo {
}
impl DataObjectTrait for HostDiskConfigurationResult {
}
impl DataObjectTrait for HostDiskDimensions {
}
impl DataObjectTrait for HostDiskDimensionsChs {
}
impl DataObjectTrait for HostDiskDimensionsLba {
}
impl DataObjectTrait for HostDiskPartitionInfo {
}
impl DataObjectTrait for HostDiskPartitionBlockRange {
}
impl DataObjectTrait for HostDiskPartitionLayout {
}
impl DataObjectTrait for HostDiskPartitionAttributes {
}
impl DataObjectTrait for HostDiskPartitionSpec {
}
impl DataObjectTrait for HostDnsConfig {
}
impl DataObjectTrait for HostDnsConfigSpec {
}
impl DataObjectTrait for HostDvxClass {
}
impl DataObjectTrait for HostEnterMaintenanceResult {
}
impl DataObjectTrait for HostEsxAgentHostManagerConfigInfo {
}
impl DataObjectTrait for HostFaultToleranceManagerComponentHealthInfo {
}
impl DataObjectTrait for FcoeConfig {
}
impl DataObjectTrait for FcoeConfigFcoeCapabilities {
}
impl DataObjectTrait for FcoeConfigFcoeSpecification {
}
impl DataObjectTrait for FcoeConfigVlanRange {
}
impl DataObjectTrait for HostFeatureCapability {
}
impl DataObjectTrait for HostFeatureMask {
}
impl DataObjectTrait for HostFeatureVersionInfo {
}
impl DataObjectTrait for HostFibreChannelOverEthernetHbaLinkInfo {
}
impl DataObjectTrait for HostFileAccess {
}
impl DataObjectTrait for ModeInfo {
}
impl DataObjectTrait for HostFileSystemMountInfo {
}
impl DataObjectTrait for HostFileSystemVolume {
}
impl DataObjectTrait for HostLocalFileSystemVolume {
}
impl DataObjectTrait for HostNasVolume {
}
impl DataObjectTrait for HostPMemVolume {
}
impl DataObjectTrait for HostVfatVolume {
}
impl DataObjectTrait for HostVffsVolume {
}
impl DataObjectTrait for HostVmfsVolume {
}
impl DataObjectTrait for HostVvolVolume {
}
impl DataObjectTrait for HostFileSystemVolumeInfo {
}
impl DataObjectTrait for HostFirewallConfig {
}
impl DataObjectTrait for HostFirewallConfigRuleSetConfig {
}
impl DataObjectTrait for HostFirewallInfo {
}
impl DataObjectTrait for HostFirewallDefaultPolicy {
}
impl DataObjectTrait for HostFlagInfo {
}
impl DataObjectTrait for HostForceMountedInfo {
}
impl DataObjectTrait for HostFru {
}
impl DataObjectTrait for HostGatewaySpec {
}
impl DataObjectTrait for HostGraphicsConfig {
}
impl DataObjectTrait for HostGraphicsConfigDeviceType {
}
impl DataObjectTrait for HostGraphicsInfo {
}
impl DataObjectTrait for HostHardwareInfo {
}
impl DataObjectTrait for HostHardwareStatusInfo {
}
impl DataObjectTrait for DpuStatusInfoOperationalInfo {
}
impl DataObjectTrait for HostHardwareElementInfo {
}
impl DataObjectTrait for DpuStatusInfo {
}
impl DataObjectTrait for HostStorageElementInfo {
}
impl DataObjectTrait for HostStorageOperationalInfo {
}
impl DataObjectTrait for HostHbaCreateSpec {
}
impl DataObjectTrait for HostTcpHbaCreateSpec {
}
impl DataObjectTrait for HealthSystemRuntime {
}
impl DataObjectTrait for HostAccessControlEntry {
}
impl DataObjectTrait for HostHostBusAdapter {
}
impl DataObjectTrait for HostBlockHba {
}
impl DataObjectTrait for HostFibreChannelHba {
}
impl DataObjectTrait for HostFibreChannelOverEthernetHba {
}
impl DataObjectTrait for HostInternetScsiHba {
}
impl DataObjectTrait for HostParallelScsiHba {
}
impl DataObjectTrait for HostPcieHba {
}
impl DataObjectTrait for HostRdmaHba {
}
impl DataObjectTrait for HostSerialAttachedHba {
}
impl DataObjectTrait for HostTcpHba {
}
impl DataObjectTrait for HostProxySwitch {
}
impl DataObjectTrait for HostProxySwitchConfig {
}
impl DataObjectTrait for HostProxySwitchEnsInfo {
}
impl DataObjectTrait for HostProxySwitchHostLagConfig {
}
impl DataObjectTrait for HostProxySwitchSpec {
}
impl DataObjectTrait for HostImageProfileSummary {
}
impl DataObjectTrait for HostInternetScsiHbaAuthenticationCapabilities {
}
impl DataObjectTrait for HostInternetScsiHbaAuthenticationProperties {
}
impl DataObjectTrait for HostInternetScsiHbaDigestCapabilities {
}
impl DataObjectTrait for HostInternetScsiHbaDigestProperties {
}
impl DataObjectTrait for HostInternetScsiHbaDiscoveryCapabilities {
}
impl DataObjectTrait for HostInternetScsiHbaDiscoveryProperties {
}
impl DataObjectTrait for HostInternetScsiHbaIpCapabilities {
}
impl DataObjectTrait for HostInternetScsiHbaIpProperties {
}
impl DataObjectTrait for HostInternetScsiHbaIPv6Properties {
}
impl DataObjectTrait for HostInternetScsiHbaIscsiIpv6Address {
}
impl DataObjectTrait for HostInternetScsiHbaSendTarget {
}
impl DataObjectTrait for HostInternetScsiHbaStaticTarget {
}
impl DataObjectTrait for HostInternetScsiHbaTargetSet {
}
impl DataObjectTrait for HostIpConfig {
}
impl DataObjectTrait for HostIpConfigIpV6Address {
}
impl DataObjectTrait for HostIpConfigIpV6AddressConfiguration {
}
impl DataObjectTrait for HostIpRouteConfig {
}
impl DataObjectTrait for HostIpRouteConfigSpec {
}
impl DataObjectTrait for HostIpRouteEntry {
}
impl DataObjectTrait for HostIpRouteOp {
}
impl DataObjectTrait for HostIpRouteTableConfig {
}
impl DataObjectTrait for HostIpRouteTableInfo {
}
impl DataObjectTrait for HostIpmiInfo {
}
impl DataObjectTrait for IscsiDependencyEntity {
}
impl DataObjectTrait for IscsiMigrationDependency {
}
impl DataObjectTrait for IscsiPortInfo {
}
impl DataObjectTrait for IscsiStatus {
}
impl DataObjectTrait for KernelModuleInfo {
}
impl DataObjectTrait for KernelModuleSectionInfo {
}
impl DataObjectTrait for HostLicenseSpec {
}
impl DataObjectTrait for LinkDiscoveryProtocolConfig {
}
impl DataObjectTrait for HostAccountSpec {
}
impl DataObjectTrait for HostPosixAccountSpec {
}
impl DataObjectTrait for HostLocalFileSystemVolumeSpec {
}
impl DataObjectTrait for HostLowLevelProvisioningManagerDiskLayoutSpec {
}
impl DataObjectTrait for HostLowLevelProvisioningManagerFileDeleteResult {
}
impl DataObjectTrait for HostLowLevelProvisioningManagerFileDeleteSpec {
}
impl DataObjectTrait for HostLowLevelProvisioningManagerFileReserveResult {
}
impl DataObjectTrait for HostLowLevelProvisioningManagerFileReserveSpec {
}
impl DataObjectTrait for HostLowLevelProvisioningManagerSnapshotLayoutSpec {
}
impl DataObjectTrait for HostLowLevelProvisioningManagerVmMigrationStatus {
}
impl DataObjectTrait for HostLowLevelProvisioningManagerVmRecoveryInfo {
}
impl DataObjectTrait for HostMaintenanceSpec {
}
impl DataObjectTrait for ServiceConsoleReservationInfo {
}
impl DataObjectTrait for VirtualMachineMemoryReservationInfo {
}
impl DataObjectTrait for VirtualMachineMemoryReservationSpec {
}
impl DataObjectTrait for HostMemorySpec {
}
impl DataObjectTrait for HostMemoryTierInfo {
}
impl DataObjectTrait for HostMountInfo {
}
impl DataObjectTrait for HostMultipathInfo {
}
impl DataObjectTrait for HostMultipathInfoLogicalUnit {
}
impl DataObjectTrait for HostMultipathInfoLogicalUnitPolicy {
}
impl DataObjectTrait for HostMultipathInfoFixedLogicalUnitPolicy {
}
impl DataObjectTrait for HostMultipathInfoHppLogicalUnitPolicy {
}
impl DataObjectTrait for HostMultipathInfoLogicalUnitStorageArrayTypePolicy {
}
impl DataObjectTrait for HostMultipathInfoPath {
}
impl DataObjectTrait for HostMultipathStateInfo {
}
impl DataObjectTrait for HostMultipathStateInfoPath {
}
impl DataObjectTrait for HostNasVolumeConfig {
}
impl DataObjectTrait for HostNasVolumeSpec {
}
impl DataObjectTrait for HostNasVolumeUserInfo {
}
impl DataObjectTrait for HostNatService {
}
impl DataObjectTrait for HostNatServiceConfig {
}
impl DataObjectTrait for HostNatServiceNameServiceSpec {
}
impl DataObjectTrait for HostNatServicePortForwardSpec {
}
impl DataObjectTrait for HostNatServiceSpec {
}
impl DataObjectTrait for HostNetCapabilities {
}
impl DataObjectTrait for HostNetOffloadCapabilities {
}
impl DataObjectTrait for HostNetStackInstance {
}
impl DataObjectTrait for HostNetworkConfig {
}
impl DataObjectTrait for HostNetworkConfigNetStackSpec {
}
impl DataObjectTrait for HostNetworkConfigResult {
}
impl DataObjectTrait for HostNetworkInfo {
}
impl DataObjectTrait for HostNetworkPolicy {
}
impl DataObjectTrait for HostNicFailureCriteria {
}
impl DataObjectTrait for HostNicOrderPolicy {
}
impl DataObjectTrait for HostNicTeamingPolicy {
}
impl DataObjectTrait for HostNetworkSecurityPolicy {
}
impl DataObjectTrait for HostNetworkTrafficShapingPolicy {
}
impl DataObjectTrait for HostNtpConfig {
}
impl DataObjectTrait for HostNumaInfo {
}
impl DataObjectTrait for HostNumaNode {
}
impl DataObjectTrait for HostNumericSensorInfo {
}
impl DataObjectTrait for NvdimmDimmInfo {
}
impl DataObjectTrait for NvdimmGuid {
}
impl DataObjectTrait for NvdimmHealthInfo {
}
impl DataObjectTrait for NvdimmInterleaveSetInfo {
}
impl DataObjectTrait for NvdimmNamespaceCreateSpec {
}
impl DataObjectTrait for NvdimmNamespaceDeleteSpec {
}
impl DataObjectTrait for NvdimmNamespaceDetails {
}
impl DataObjectTrait for NvdimmNamespaceInfo {
}
impl DataObjectTrait for NvdimmSystemInfo {
}
impl DataObjectTrait for NvdimmPMemNamespaceCreateSpec {
}
impl DataObjectTrait for NvdimmRegionInfo {
}
impl DataObjectTrait for NvdimmSummary {
}
impl DataObjectTrait for HostNvmeController {
}
impl DataObjectTrait for HostNvmeDisconnectSpec {
}
impl DataObjectTrait for HostNvmeDiscoveryLog {
}
impl DataObjectTrait for HostNvmeDiscoveryLogEntry {
}
impl DataObjectTrait for HostNvmeNamespace {
}
impl DataObjectTrait for HostNvmeSpec {
}
impl DataObjectTrait for HostNvmeConnectSpec {
}
impl DataObjectTrait for HostNvmeDiscoverSpec {
}
impl DataObjectTrait for HostNvmeTopology {
}
impl DataObjectTrait for HostNvmeTopologyInterface {
}
impl DataObjectTrait for HostNvmeTransportParameters {
}
impl DataObjectTrait for HostNvmeOpaqueTransportParameters {
}
impl DataObjectTrait for HostNvmeOverFibreChannelParameters {
}
impl DataObjectTrait for HostNvmeOverRdmaParameters {
}
impl DataObjectTrait for HostNvmeOverTcpParameters {
}
impl DataObjectTrait for HostOpaqueNetworkInfo {
}
impl DataObjectTrait for HostOpaqueSwitch {
}
impl DataObjectTrait for HostOpaqueSwitchPhysicalNicZone {
}
impl DataObjectTrait for HostPatchManagerLocator {
}
impl DataObjectTrait for HostPatchManagerPatchManagerOperationSpec {
}
impl DataObjectTrait for HostPatchManagerResult {
}
impl DataObjectTrait for HostPatchManagerStatus {
}
impl DataObjectTrait for HostPatchManagerStatusPrerequisitePatch {
}
impl DataObjectTrait for HostPathSelectionPolicyOption {
}
impl DataObjectTrait for HostPciDevice {
}
impl DataObjectTrait for HostPciPassthruConfig {
}
impl DataObjectTrait for HostSriovConfig {
}
impl DataObjectTrait for HostPciPassthruInfo {
}
impl DataObjectTrait for HostSriovInfo {
}
impl DataObjectTrait for HostPersistentMemoryInfo {
}
impl DataObjectTrait for PhysicalNic {
}
impl DataObjectTrait for PhysicalNicCdpDeviceCapability {
}
impl DataObjectTrait for PhysicalNicCdpInfo {
}
impl DataObjectTrait for PhysicalNicConfig {
}
impl DataObjectTrait for PhysicalNicLinkInfo {
}
impl DataObjectTrait for LinkLayerDiscoveryProtocolInfo {
}
impl DataObjectTrait for PhysicalNicHintInfo {
}
impl DataObjectTrait for PhysicalNicHint {
}
impl DataObjectTrait for PhysicalNicIpHint {
}
impl DataObjectTrait for PhysicalNicNameHint {
}
impl DataObjectTrait for PhysicalNicSpec {
}
impl DataObjectTrait for HostPlugStoreTopology {
}
impl DataObjectTrait for HostPlugStoreTopologyAdapter {
}
impl DataObjectTrait for HostPlugStoreTopologyDevice {
}
impl DataObjectTrait for HostPlugStoreTopologyPath {
}
impl DataObjectTrait for HostPlugStoreTopologyPlugin {
}
impl DataObjectTrait for HostPlugStoreTopologyTarget {
}
impl DataObjectTrait for HostPortGroup {
}
impl DataObjectTrait for HostPortGroupConfig {
}
impl DataObjectTrait for HostPortGroupPort {
}
impl DataObjectTrait for HostPortGroupSpec {
}
impl DataObjectTrait for PowerSystemCapability {
}
impl DataObjectTrait for PowerSystemInfo {
}
impl DataObjectTrait for HostPowerPolicy {
}
impl DataObjectTrait for HostProtocolEndpoint {
}
impl DataObjectTrait for HostPtpConfig {
}
impl DataObjectTrait for HostPtpConfigPtpPort {
}
impl DataObjectTrait for HostQualifiedName {
}
impl DataObjectTrait for HostRdmaDevice {
}
impl DataObjectTrait for HostRdmaDeviceBacking {
}
impl DataObjectTrait for HostRdmaDevicePnicBacking {
}
impl DataObjectTrait for HostRdmaDeviceCapability {
}
impl DataObjectTrait for HostRdmaDeviceConnectionInfo {
}
impl DataObjectTrait for HostReliableMemoryInfo {
}
impl DataObjectTrait for HostResignatureRescanResult {
}
impl DataObjectTrait for HostFirewallRuleset {
}
impl DataObjectTrait for HostFirewallRulesetIpList {
}
impl DataObjectTrait for HostFirewallRulesetIpNetwork {
}
impl DataObjectTrait for HostFirewallRule {
}
impl DataObjectTrait for HostFirewallRulesetRulesetSpec {
}
impl DataObjectTrait for HostRuntimeInfo {
}
impl DataObjectTrait for HostRuntimeInfoNetStackInstanceRuntimeInfo {
}
impl DataObjectTrait for HostNetworkResourceRuntime {
}
impl DataObjectTrait for HostRuntimeInfoNetworkRuntimeInfo {
}
impl DataObjectTrait for HostPlacedVirtualNicIdentifier {
}
impl DataObjectTrait for HostPnicNetworkResourceInfo {
}
impl DataObjectTrait for HostRuntimeInfoStateEncryptionInfo {
}
impl DataObjectTrait for HostScsiDiskPartition {
}
impl DataObjectTrait for ScsiLunCapabilities {
}
impl DataObjectTrait for ScsiLunDescriptor {
}
impl DataObjectTrait for ScsiLunDurableName {
}
impl DataObjectTrait for HostScsiTopology {
}
impl DataObjectTrait for HostScsiTopologyInterface {
}
impl DataObjectTrait for HostScsiTopologyLun {
}
impl DataObjectTrait for HostScsiTopologyTarget {
}
impl DataObjectTrait for HostSecuritySpec {
}
impl DataObjectTrait for HostService {
}
impl DataObjectTrait for HostServiceSourcePackage {
}
impl DataObjectTrait for HostServiceConfig {
}
impl DataObjectTrait for HostServiceInfo {
}
impl DataObjectTrait for HostSevInfo {
}
impl DataObjectTrait for HostSgxInfo {
}
impl DataObjectTrait for HostSgxRegistrationInfo {
}
impl DataObjectTrait for HostSharedGpuCapabilities {
}
impl DataObjectTrait for HostSnmpSystemAgentLimits {
}
impl DataObjectTrait for HostSnmpConfigSpec {
}
impl DataObjectTrait for HostSnmpDestination {
}
impl DataObjectTrait for SoftwarePackage {
}
impl DataObjectTrait for SoftwarePackageCapability {
}
impl DataObjectTrait for Relation {
}
impl DataObjectTrait for HostSriovDevicePoolInfo {
}
impl DataObjectTrait for HostSriovNetworkDevicePoolInfo {
}
impl DataObjectTrait for HostSslThumbprintInfo {
}
impl DataObjectTrait for HostStorageArrayTypePolicyOption {
}
impl DataObjectTrait for HostStorageDeviceInfo {
}
impl DataObjectTrait for HostStorageSystemDiskLocatorLedResult {
}
impl DataObjectTrait for HostStorageSystemScsiLunResult {
}
impl DataObjectTrait for HostStorageSystemVmfsVolumeResult {
}
impl DataObjectTrait for HostListSummary {
}
impl DataObjectTrait for HostConfigSummary {
}
impl DataObjectTrait for HostListSummaryGatewaySummary {
}
impl DataObjectTrait for HostHardwareSummary {
}
impl DataObjectTrait for HostListSummaryQuickStats {
}
impl DataObjectTrait for SystemEventInfo {
}
impl DataObjectTrait for HostSystemHealthInfo {
}
impl DataObjectTrait for HostSystemIdentificationInfo {
}
impl DataObjectTrait for HostSystemInfo {
}
impl DataObjectTrait for HostSystemResourceInfo {
}
impl DataObjectTrait for HostSystemSwapConfiguration {
}
impl DataObjectTrait for HostSystemSwapConfigurationSystemSwapOption {
}
impl DataObjectTrait for HostSystemSwapConfigurationDatastoreOption {
}
impl DataObjectTrait for HostSystemSwapConfigurationDisabledOption {
}
impl DataObjectTrait for HostSystemSwapConfigurationHostCacheOption {
}
impl DataObjectTrait for HostSystemSwapConfigurationHostLocalSwapOption {
}
impl DataObjectTrait for HostTargetTransport {
}
impl DataObjectTrait for HostBlockAdapterTargetTransport {
}
impl DataObjectTrait for HostFibreChannelTargetTransport {
}
impl DataObjectTrait for HostFibreChannelOverEthernetTargetTransport {
}
impl DataObjectTrait for HostInternetScsiTargetTransport {
}
impl DataObjectTrait for HostParallelScsiTargetTransport {
}
impl DataObjectTrait for HostPcieTargetTransport {
}
impl DataObjectTrait for HostRdmaTargetTransport {
}
impl DataObjectTrait for HostSerialAttachedTargetTransport {
}
impl DataObjectTrait for HostTcpTargetTransport {
}
impl DataObjectTrait for HostTpmAttestationInfo {
}
impl DataObjectTrait for HostTpmAttestationReport {
}
impl DataObjectTrait for HostTpmEventDetails {
}
impl DataObjectTrait for HostTpmBootCompleteEventDetails {
}
impl DataObjectTrait for HostTpmBootSecurityOptionEventDetails {
}
impl DataObjectTrait for HostTpmNvTagEventDetails {
}
impl DataObjectTrait for HostTpmSignerEventDetails {
}
impl DataObjectTrait for HostTpmCommandEventDetails {
}
impl DataObjectTrait for HostTpmOptionEventDetails {
}
impl DataObjectTrait for HostTpmSoftwareComponentEventDetails {
}
impl DataObjectTrait for HostTpmVersionEventDetails {
}
impl DataObjectTrait for HostTpmEventLogEntry {
}
impl DataObjectTrait for HostTrustAuthorityAttestationInfo {
}
impl DataObjectTrait for HostUnresolvedVmfsExtent {
}
impl DataObjectTrait for HostUnresolvedVmfsResignatureSpec {
}
impl DataObjectTrait for HostUnresolvedVmfsResolutionResult {
}
impl DataObjectTrait for HostUnresolvedVmfsResolutionSpec {
}
impl DataObjectTrait for HostUnresolvedVmfsVolume {
}
impl DataObjectTrait for HostUnresolvedVmfsVolumeResolveStatus {
}
impl DataObjectTrait for HostVFlashManagerVFlashCacheConfigInfo {
}
impl DataObjectTrait for HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption {
}
impl DataObjectTrait for HostVFlashManagerVFlashCacheConfigSpec {
}
impl DataObjectTrait for HostVFlashManagerVFlashConfigInfo {
}
impl DataObjectTrait for HostVFlashManagerVFlashResourceConfigInfo {
}
impl DataObjectTrait for HostVFlashManagerVFlashResourceConfigSpec {
}
impl DataObjectTrait for HostVFlashManagerVFlashResourceRunTimeInfo {
}
impl DataObjectTrait for HostVFlashResourceConfigurationResult {
}
impl DataObjectTrait for HostVMotionConfig {
}
impl DataObjectTrait for HostVMotionInfo {
}
impl DataObjectTrait for HostVMotionManagerDstInstantCloneResult {
}
impl DataObjectTrait for HostVMotionManagerSrcInstantCloneResult {
}
impl DataObjectTrait for HostVMotionNetConfig {
}
impl DataObjectTrait for HostVffsSpec {
}
impl DataObjectTrait for HostVirtualNic {
}
impl DataObjectTrait for HostVirtualNicConfig {
}
impl DataObjectTrait for HostVirtualNicIpRouteSpec {
}
impl DataObjectTrait for HostVirtualNicOpaqueNetworkSpec {
}
impl DataObjectTrait for HostVirtualNicSpec {
}
impl DataObjectTrait for HostVirtualNicConnection {
}
impl DataObjectTrait for VirtualNicManagerNetConfig {
}
impl DataObjectTrait for HostVirtualNicManagerNicTypeSelection {
}
impl DataObjectTrait for HostVirtualNicManagerInfo {
}
impl DataObjectTrait for HostVirtualSwitch {
}
impl DataObjectTrait for HostVirtualSwitchBeaconConfig {
}
impl DataObjectTrait for HostVirtualSwitchBridge {
}
impl DataObjectTrait for HostVirtualSwitchAutoBridge {
}
impl DataObjectTrait for HostVirtualSwitchBondBridge {
}
impl DataObjectTrait for HostVirtualSwitchSimpleBridge {
}
impl DataObjectTrait for HostVirtualSwitchConfig {
}
impl DataObjectTrait for HostVirtualSwitchSpec {
}
impl DataObjectTrait for HostVmciAccessManagerAccessSpec {
}
impl DataObjectTrait for VmfsDatastoreOption {
}
impl DataObjectTrait for VmfsDatastoreBaseOption {
}
impl DataObjectTrait for VmfsDatastoreMultipleExtentOption {
}
impl DataObjectTrait for VmfsDatastoreSingleExtentOption {
}
impl DataObjectTrait for VmfsDatastoreAllExtentOption {
}
impl DataObjectTrait for VmfsDatastoreSpec {
}
impl DataObjectTrait for VmfsDatastoreCreateSpec {
}
impl DataObjectTrait for VmfsDatastoreExpandSpec {
}
impl DataObjectTrait for VmfsDatastoreExtendSpec {
}
impl DataObjectTrait for HostVmfsRescanResult {
}
impl DataObjectTrait for VmfsConfigOption {
}
impl DataObjectTrait for HostVmfsSpec {
}
impl DataObjectTrait for VmfsUnmapBandwidthSpec {
}
impl DataObjectTrait for HostVsanInternalSystemCmmdsQuery {
}
impl DataObjectTrait for HostVsanInternalSystemDeleteVsanObjectsResult {
}
impl DataObjectTrait for VsanNewPolicyBatch {
}
impl DataObjectTrait for VsanPolicyChangeBatch {
}
impl DataObjectTrait for VsanPolicyCost {
}
impl DataObjectTrait for VsanPolicySatisfiability {
}
impl DataObjectTrait for HostVsanInternalSystemVsanObjectOperationResult {
}
impl DataObjectTrait for HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult {
}
impl DataObjectTrait for HostVvolNqn {
}
impl DataObjectTrait for VVolHostPe {
}
impl DataObjectTrait for HostVvolVolumeHostVvolNqn {
}
impl DataObjectTrait for HostVvolVolumeSpecification {
}
impl DataObjectTrait for NetDhcpConfigInfo {
}
impl DataObjectTrait for NetDhcpConfigInfoDhcpOptions {
}
impl DataObjectTrait for NetDhcpConfigSpec {
}
impl DataObjectTrait for NetDhcpConfigSpecDhcpOptionsSpec {
}
impl DataObjectTrait for NetDnsConfigInfo {
}
impl DataObjectTrait for NetDnsConfigSpec {
}
impl DataObjectTrait for NetIpConfigInfo {
}
impl DataObjectTrait for NetIpConfigInfoIpAddress {
}
impl DataObjectTrait for NetIpConfigSpec {
}
impl DataObjectTrait for NetIpConfigSpecIpAddressSpec {
}
impl DataObjectTrait for NetIpRouteConfigInfo {
}
impl DataObjectTrait for NetIpRouteConfigInfoGateway {
}
impl DataObjectTrait for NetIpRouteConfigInfoIpRoute {
}
impl DataObjectTrait for NetIpRouteConfigSpec {
}
impl DataObjectTrait for NetIpRouteConfigSpecGatewaySpec {
}
impl DataObjectTrait for NetIpRouteConfigSpecIpRouteSpec {
}
impl DataObjectTrait for NetIpStackInfo {
}
impl DataObjectTrait for NetIpStackInfoDefaultRouter {
}
impl DataObjectTrait for NetIpStackInfoNetToMedia {
}
impl DataObjectTrait for NetBiosConfigInfo {
}
impl DataObjectTrait for WinNetBiosConfigInfo {
}
impl DataObjectTrait for ArrayUpdateSpec {
}
impl DataObjectTrait for ClusterDasVmConfigSpec {
}
impl DataObjectTrait for ClusterDatastoreUpdateSpec {
}
impl DataObjectTrait for ClusterDpmHostConfigSpec {
}
impl DataObjectTrait for ClusterDrsVmConfigSpec {
}
impl DataObjectTrait for ClusterGroupSpec {
}
impl DataObjectTrait for ClusterPreemptibleVmPairSpec {
}
impl DataObjectTrait for ClusterRuleSpec {
}
impl DataObjectTrait for ClusterTagCategoryUpdateSpec {
}
impl DataObjectTrait for ClusterVmOrchestrationSpec {
}
impl DataObjectTrait for StorageDrsOptionSpec {
}
impl DataObjectTrait for StorageDrsVmConfigSpec {
}
impl DataObjectTrait for VAppOvfSectionSpec {
}
impl DataObjectTrait for VAppProductSpec {
}
impl DataObjectTrait for VAppPropertySpec {
}
impl DataObjectTrait for VirtualMachineCpuIdInfoSpec {
}
impl DataObjectTrait for OptionType {
}
impl DataObjectTrait for BoolOption {
}
impl DataObjectTrait for ChoiceOption {
}
impl DataObjectTrait for FloatOption {
}
impl DataObjectTrait for IntOption {
}
impl DataObjectTrait for LongOption {
}
impl DataObjectTrait for StringOption {
}
impl DataObjectTrait for OptionValue {
}
impl DataObjectTrait for HostInternetScsiHbaParamValue {
}
impl DataObjectTrait for ApplyProfile {
}
impl DataObjectTrait for ProfileApplyProfileElement {
}
impl DataObjectTrait for ActiveDirectoryProfile {
}
impl DataObjectTrait for AuthenticationProfile {
}
impl DataObjectTrait for DateTimeProfile {
}
impl DataObjectTrait for DvsProfile {
}
impl DataObjectTrait for DvsVNicProfile {
}
impl DataObjectTrait for DvsHostVNicProfile {
}
impl DataObjectTrait for DvsServiceConsoleVNicProfile {
}
impl DataObjectTrait for FirewallProfile {
}
impl DataObjectTrait for FirewallProfileRulesetProfile {
}
impl DataObjectTrait for HostApplyProfile {
}
impl DataObjectTrait for HostMemoryProfile {
}
impl DataObjectTrait for IpAddressProfile {
}
impl DataObjectTrait for IpRouteProfile {
}
impl DataObjectTrait for NasStorageProfile {
}
impl DataObjectTrait for NetStackInstanceProfile {
}
impl DataObjectTrait for NetworkPolicyProfile {
}
impl DataObjectTrait for NetworkProfile {
}
impl DataObjectTrait for NetworkProfileDnsConfigProfile {
}
impl DataObjectTrait for NsxHostVNicProfile {
}
impl DataObjectTrait for OpaqueSwitchProfile {
}
impl DataObjectTrait for OptionProfile {
}
impl DataObjectTrait for PermissionProfile {
}
impl DataObjectTrait for PhysicalNicProfile {
}
impl DataObjectTrait for PnicUplinkProfile {
}
impl DataObjectTrait for PortGroupProfile {
}
impl DataObjectTrait for HostPortGroupProfile {
}
impl DataObjectTrait for ServiceConsolePortGroupProfile {
}
impl DataObjectTrait for VmPortGroupProfile {
}
impl DataObjectTrait for VirtualSwitchSelectionProfile {
}
impl DataObjectTrait for VlanProfile {
}
impl DataObjectTrait for SecurityProfile {
}
impl DataObjectTrait for ServiceProfile {
}
impl DataObjectTrait for StaticRouteProfile {
}
impl DataObjectTrait for StorageProfile {
}
impl DataObjectTrait for UserGroupProfile {
}
impl DataObjectTrait for UserProfile {
}
impl DataObjectTrait for VirtualSwitchProfile {
}
impl DataObjectTrait for LinkProfile {
}
impl DataObjectTrait for NumPortsProfile {
}
impl DataObjectTrait for ProfileApplyProfileProperty {
}
impl DataObjectTrait for ComplianceLocator {
}
impl DataObjectTrait for ComplianceProfile {
}
impl DataObjectTrait for ComplianceResult {
}
impl DataObjectTrait for ComplianceFailure {
}
impl DataObjectTrait for ComplianceFailureComplianceFailureValues {
}
impl DataObjectTrait for ProfileDeferredPolicyOptionParameter {
}
impl DataObjectTrait for ProfileExpression {
}
impl DataObjectTrait for ProfileCompositeExpression {
}
impl DataObjectTrait for ProfileSimpleExpression {
}
impl DataObjectTrait for ProfileExpressionMetadata {
}
impl DataObjectTrait for ProfileParameterMetadata {
}
impl DataObjectTrait for ProfileParameterMetadataParameterRelationMetadata {
}
impl DataObjectTrait for ProfilePolicy {
}
impl DataObjectTrait for ProfilePolicyMetadata {
}
impl DataObjectTrait for PolicyOption {
}
impl DataObjectTrait for CompositePolicyOption {
}
impl DataObjectTrait for ProfilePolicyOptionMetadata {
}
impl DataObjectTrait for ProfileCompositePolicyOptionMetadata {
}
impl DataObjectTrait for UserInputRequiredParameterMetadata {
}
impl DataObjectTrait for ProfileConfigInfo {
}
impl DataObjectTrait for ClusterProfileConfigInfo {
}
impl DataObjectTrait for HostProfileConfigInfo {
}
impl DataObjectTrait for ProfileCreateSpec {
}
impl DataObjectTrait for ProfileSerializedCreateSpec {
}
impl DataObjectTrait for HostProfileSerializedHostProfileSpec {
}
impl DataObjectTrait for ClusterProfileCreateSpec {
}
impl DataObjectTrait for ClusterProfileConfigSpec {
}
impl DataObjectTrait for ClusterProfileCompleteConfigSpec {
}
impl DataObjectTrait for ClusterProfileConfigServiceCreateSpec {
}
impl DataObjectTrait for HostProfileConfigSpec {
}
impl DataObjectTrait for HostProfileCompleteConfigSpec {
}
impl DataObjectTrait for HostProfileHostBasedConfigSpec {
}
impl DataObjectTrait for ProfileDescription {
}
impl DataObjectTrait for ProfileDescriptionSection {
}
impl DataObjectTrait for ProfileMetadata {
}
impl DataObjectTrait for ProfileMetadataProfileOperationMessage {
}
impl DataObjectTrait for ProfileMetadataProfileSortSpec {
}
impl DataObjectTrait for ProfilePropertyPath {
}
impl DataObjectTrait for ProfileProfileStructure {
}
impl DataObjectTrait for ProfileProfileStructureProperty {
}
impl DataObjectTrait for AnswerFile {
}
impl DataObjectTrait for AnswerFileStatusResult {
}
impl DataObjectTrait for AnswerFileStatusError {
}
impl DataObjectTrait for ProfileExecuteResult {
}
impl DataObjectTrait for ApplyHostProfileConfigurationSpec {
}
impl DataObjectTrait for ProfileExecuteError {
}
impl DataObjectTrait for HostProfileValidationFailureInfo {
}
impl DataObjectTrait for HostSpecification {
}
impl DataObjectTrait for HostSubSpecification {
}
impl DataObjectTrait for AnswerFileCreateSpec {
}
impl DataObjectTrait for AnswerFileOptionsCreateSpec {
}
impl DataObjectTrait for AnswerFileSerializedCreateSpec {
}
impl DataObjectTrait for ApplyHostProfileConfigurationResult {
}
impl DataObjectTrait for HostProfileManagerCompositionResult {
}
impl DataObjectTrait for HostProfileManagerCompositionResultResultElement {
}
impl DataObjectTrait for HostProfileManagerCompositionValidationResult {
}
impl DataObjectTrait for HostProfileManagerCompositionValidationResultResultElement {
}
impl DataObjectTrait for HostProfileManagerConfigTaskList {
}
impl DataObjectTrait for HostProfilesEntityCustomizations {
}
impl DataObjectTrait for StructuredCustomizations {
}
impl DataObjectTrait for HostProfileManagerHostToConfigSpecMap {
}
impl DataObjectTrait for ScheduledTaskDescription {
}
impl DataObjectTrait for ScheduledTaskSpec {
}
impl DataObjectTrait for ScheduledTaskInfo {
}
impl DataObjectTrait for TaskScheduler {
}
impl DataObjectTrait for AfterStartupTaskScheduler {
}
impl DataObjectTrait for OnceTaskScheduler {
}
impl DataObjectTrait for RecurrentTaskScheduler {
}
impl DataObjectTrait for HourlyTaskScheduler {
}
impl DataObjectTrait for DailyTaskScheduler {
}
impl DataObjectTrait for MonthlyTaskScheduler {
}
impl DataObjectTrait for MonthlyByDayTaskScheduler {
}
impl DataObjectTrait for MonthlyByWeekdayTaskScheduler {
}
impl DataObjectTrait for WeeklyTaskScheduler {
}
impl DataObjectTrait for ApplyStorageRecommendationResult {
}
impl DataObjectTrait for StorageDrsAutomationConfig {
}
impl DataObjectTrait for StorageDrsConfigInfo {
}
impl DataObjectTrait for StorageDrsConfigSpec {
}
impl DataObjectTrait for StorageDrsIoLoadBalanceConfig {
}
impl DataObjectTrait for PlacementAffinityRule {
}
impl DataObjectTrait for PlacementRankResult {
}
impl DataObjectTrait for PlacementRankSpec {
}
impl DataObjectTrait for StorageDrsPlacementRankVmSpec {
}
impl DataObjectTrait for StorageDrsPodConfigInfo {
}
impl DataObjectTrait for StorageDrsPodConfigSpec {
}
impl DataObjectTrait for StorageDrsPodSelectionSpec {
}
impl DataObjectTrait for PodDiskLocator {
}
impl DataObjectTrait for VmPodConfigForPlacement {
}
impl DataObjectTrait for StorageDrsSpaceLoadBalanceConfig {
}
impl DataObjectTrait for StoragePlacementResult {
}
impl DataObjectTrait for StoragePlacementSpec {
}
impl DataObjectTrait for StorageDrsVmConfigInfo {
}
impl DataObjectTrait for VAppCloneSpec {
}
impl DataObjectTrait for VAppCloneSpecNetworkMappingPair {
}
impl DataObjectTrait for VAppCloneSpecResourceMap {
}
impl DataObjectTrait for VAppEntityConfigInfo {
}
impl DataObjectTrait for VAppIpAssignmentInfo {
}
impl DataObjectTrait for IpPool {
}
impl DataObjectTrait for IpPoolAssociation {
}
impl DataObjectTrait for IpPoolIpPoolConfigInfo {
}
impl DataObjectTrait for VAppOvfSectionInfo {
}
impl DataObjectTrait for VAppProductInfo {
}
impl DataObjectTrait for VAppPropertyInfo {
}
impl DataObjectTrait for VmConfigInfo {
}
impl DataObjectTrait for VAppConfigInfo {
}
impl DataObjectTrait for VmConfigSpec {
}
impl DataObjectTrait for VAppConfigSpec {
}
impl DataObjectTrait for ClusterNetworkConfigSpec {
}
impl DataObjectTrait for FailoverNodeInfo {
}
impl DataObjectTrait for NodeDeploymentSpec {
}
impl DataObjectTrait for PassiveNodeDeploymentSpec {
}
impl DataObjectTrait for NodeNetworkSpec {
}
impl DataObjectTrait for PassiveNodeNetworkSpec {
}
impl DataObjectTrait for SourceNodeSpec {
}
impl DataObjectTrait for VchaClusterConfigInfo {
}
impl DataObjectTrait for VchaClusterConfigSpec {
}
impl DataObjectTrait for VchaClusterDeploymentSpec {
}
impl DataObjectTrait for VchaClusterNetworkSpec {
}
impl DataObjectTrait for WitnessNodeInfo {
}
impl DataObjectTrait for VchaClusterHealth {
}
impl DataObjectTrait for VchaClusterRuntimeInfo {
}
impl DataObjectTrait for VchaNodeRuntimeInfo {
}
impl DataObjectTrait for VirtualMachineAffinityInfo {
}
impl DataObjectTrait for VirtualMachineBaseIndependentFilterSpec {
}
impl DataObjectTrait for VirtualMachineEmptyIndependentFilterSpec {
}
impl DataObjectTrait for VirtualMachineIndependentFilterSpec {
}
impl DataObjectTrait for VirtualMachineBootOptions {
}
impl DataObjectTrait for VirtualMachineBootOptionsBootableDevice {
}
impl DataObjectTrait for VirtualMachineBootOptionsBootableCdromDevice {
}
impl DataObjectTrait for VirtualMachineBootOptionsBootableDiskDevice {
}
impl DataObjectTrait for VirtualMachineBootOptionsBootableEthernetDevice {
}
impl DataObjectTrait for VirtualMachineBootOptionsBootableFloppyDevice {
}
impl DataObjectTrait for VirtualMachineCapability {
}
impl DataObjectTrait for VirtualMachineCertThumbprint {
}
impl DataObjectTrait for VirtualMachineCloneSpec {
}
impl DataObjectTrait for VirtualMachineConfigInfo {
}
impl DataObjectTrait for VirtualMachineConfigInfoDatastoreUrlPair {
}
impl DataObjectTrait for VirtualMachineConfigInfoOverheadInfo {
}
impl DataObjectTrait for VirtualMachineConfigOption {
}
impl DataObjectTrait for VirtualMachineConfigOptionDescriptor {
}
impl DataObjectTrait for VirtualMachineConfigSpec {
}
impl DataObjectTrait for ConfigTarget {
}
impl DataObjectTrait for VirtualMachineConsolePreferences {
}
impl DataObjectTrait for VirtualMachineContentLibraryItemInfo {
}
impl DataObjectTrait for DatastoreOption {
}
impl DataObjectTrait for VirtualMachineDatastoreVolumeOption {
}
impl DataObjectTrait for VirtualMachineDefaultPowerOpInfo {
}
impl DataObjectTrait for VirtualMachineDeviceRuntimeInfo {
}
impl DataObjectTrait for VirtualMachineDeviceRuntimeInfoDeviceRuntimeState {
}
impl DataObjectTrait for VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState {
}
impl DataObjectTrait for VirtualMachineDvxClassInfo {
}
impl DataObjectTrait for FaultToleranceConfigInfo {
}
impl DataObjectTrait for FaultTolerancePrimaryConfigInfo {
}
impl DataObjectTrait for FaultToleranceSecondaryConfigInfo {
}
impl DataObjectTrait for FaultToleranceConfigSpec {
}
impl DataObjectTrait for FaultToleranceMetaSpec {
}
impl DataObjectTrait for FaultToleranceSecondaryOpResult {
}
impl DataObjectTrait for FaultToleranceVmConfigSpec {
}
impl DataObjectTrait for FaultToleranceDiskSpec {
}
impl DataObjectTrait for VirtualMachineFeatureRequirement {
}
impl DataObjectTrait for VirtualMachineFileInfo {
}
impl DataObjectTrait for VirtualMachineFileLayout {
}
impl DataObjectTrait for VirtualMachineFileLayoutDiskLayout {
}
impl DataObjectTrait for VirtualMachineFileLayoutSnapshotLayout {
}
impl DataObjectTrait for VirtualMachineFileLayoutEx {
}
impl DataObjectTrait for VirtualMachineFileLayoutExDiskLayout {
}
impl DataObjectTrait for VirtualMachineFileLayoutExDiskUnit {
}
impl DataObjectTrait for VirtualMachineFileLayoutExFileInfo {
}
impl DataObjectTrait for VirtualMachineFileLayoutExSnapshotLayout {
}
impl DataObjectTrait for VirtualMachineFlagInfo {
}
impl DataObjectTrait for VirtualMachineForkConfigInfo {
}
impl DataObjectTrait for GuestInfo {
}
impl DataObjectTrait for GuestInfoCustomizationInfo {
}
impl DataObjectTrait for GuestDiskInfo {
}
impl DataObjectTrait for GuestInfoNamespaceGenerationInfo {
}
impl DataObjectTrait for GuestNicInfo {
}
impl DataObjectTrait for GuestScreenInfo {
}
impl DataObjectTrait for GuestStackInfo {
}
impl DataObjectTrait for GuestInfoVirtualDiskMapping {
}
impl DataObjectTrait for VirtualMachineGuestIntegrityInfo {
}
impl DataObjectTrait for VirtualMachineGuestMonitoringModeInfo {
}
impl DataObjectTrait for GuestOsDescriptor {
}
impl DataObjectTrait for VirtualMachineGuestQuiesceSpec {
}
impl DataObjectTrait for VirtualMachineWindowsQuiesceSpec {
}
impl DataObjectTrait for VirtualMachineIdeDiskDevicePartitionInfo {
}
impl DataObjectTrait for VirtualMachineInstantCloneSpec {
}
impl DataObjectTrait for VirtualMachineLegacyNetworkSwitchInfo {
}
impl DataObjectTrait for VirtualMachineMessage {
}
impl DataObjectTrait for VirtualMachineMetadataManagerVmMetadata {
}
impl DataObjectTrait for VirtualMachineMetadataManagerVmMetadataInput {
}
impl DataObjectTrait for VirtualMachineMetadataManagerVmMetadataOwner {
}
impl DataObjectTrait for VirtualMachineMetadataManagerVmMetadataResult {
}
impl DataObjectTrait for VirtualMachineNetworkShaperInfo {
}
impl DataObjectTrait for VirtualMachineProfileDetails {
}
impl DataObjectTrait for VirtualMachineProfileDetailsDiskProfileDetails {
}
impl DataObjectTrait for VirtualMachineProfileRawData {
}
impl DataObjectTrait for VirtualMachineProfileSpec {
}
impl DataObjectTrait for VirtualMachineDefaultProfileSpec {
}
impl DataObjectTrait for VirtualMachineDefinedProfileSpec {
}
impl DataObjectTrait for VirtualMachineEmptyProfileSpec {
}
impl DataObjectTrait for VirtualMachinePropertyRelation {
}
impl DataObjectTrait for VirtualMachineQuestionInfo {
}
impl DataObjectTrait for VirtualMachineRelocateSpec {
}
impl DataObjectTrait for VirtualMachineRelocateSpecDiskLocator {
}
impl DataObjectTrait for VirtualMachineRelocateSpecDiskLocatorBackingSpec {
}
impl DataObjectTrait for ReplicationConfigSpec {
}
impl DataObjectTrait for ReplicationInfoDiskSettings {
}
impl DataObjectTrait for VirtualMachineRuntimeInfo {
}
impl DataObjectTrait for VirtualMachineRuntimeInfoDasProtectionState {
}
impl DataObjectTrait for ScheduledHardwareUpgradeInfo {
}
impl DataObjectTrait for VirtualMachineSgxInfo {
}
impl DataObjectTrait for VirtualMachineSnapshotInfo {
}
impl DataObjectTrait for VirtualMachineSnapshotTree {
}
impl DataObjectTrait for VirtualMachineSriovDevicePoolInfo {
}
impl DataObjectTrait for VirtualMachineSriovNetworkDevicePoolInfo {
}
impl DataObjectTrait for VirtualMachineStorageInfo {
}
impl DataObjectTrait for VirtualMachineUsageOnDatastore {
}
impl DataObjectTrait for VirtualMachineSummary {
}
impl DataObjectTrait for VirtualMachineConfigSummary {
}
impl DataObjectTrait for VirtualMachineGuestSummary {
}
impl DataObjectTrait for VirtualMachineQuickStats {
}
impl DataObjectTrait for VirtualMachineQuickStatsMemoryTierStats {
}
impl DataObjectTrait for VirtualMachineStorageSummary {
}
impl DataObjectTrait for VirtualMachineTargetInfo {
}
impl DataObjectTrait for VirtualMachineCdromInfo {
}
impl DataObjectTrait for VirtualMachineDatastoreInfo {
}
impl DataObjectTrait for VirtualMachineDiskDeviceInfo {
}
impl DataObjectTrait for VirtualMachineIdeDiskDeviceInfo {
}
impl DataObjectTrait for VirtualMachineScsiDiskDeviceInfo {
}
impl DataObjectTrait for VirtualMachineDynamicPassthroughInfo {
}
impl DataObjectTrait for VirtualMachineFloppyInfo {
}
impl DataObjectTrait for VirtualMachineNetworkInfo {
}
impl DataObjectTrait for OpaqueNetworkTargetInfo {
}
impl DataObjectTrait for VirtualMachineParallelInfo {
}
impl DataObjectTrait for VirtualMachinePciPassthroughInfo {
}
impl DataObjectTrait for VirtualMachineSriovInfo {
}
impl DataObjectTrait for VirtualMachinePciSharedGpuPassthroughInfo {
}
impl DataObjectTrait for VirtualMachinePrecisionClockInfo {
}
impl DataObjectTrait for VirtualMachineScsiPassthroughInfo {
}
impl DataObjectTrait for VirtualMachineSerialInfo {
}
impl DataObjectTrait for VirtualMachineSgxTargetInfo {
}
impl DataObjectTrait for VirtualMachineSoundInfo {
}
impl DataObjectTrait for VirtualMachineUsbInfo {
}
impl DataObjectTrait for VirtualMachineVFlashModuleInfo {
}
impl DataObjectTrait for VirtualMachineVMotionStunTimeInfo {
}
impl DataObjectTrait for VirtualMachineVendorDeviceGroupInfo {
}
impl DataObjectTrait for VirtualMachineVgpuDeviceInfo {
}
impl DataObjectTrait for VirtualMachineVgpuProfileInfo {
}
impl DataObjectTrait for ToolsConfigInfo {
}
impl DataObjectTrait for ToolsConfigInfoToolsLastInstallInfo {
}
impl DataObjectTrait for UsbScanCodeSpec {
}
impl DataObjectTrait for UsbScanCodeSpecKeyEvent {
}
impl DataObjectTrait for UsbScanCodeSpecModifierType {
}
impl DataObjectTrait for VirtualMachineVcpuConfig {
}
impl DataObjectTrait for VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo {
}
impl DataObjectTrait for VirtualMachineVirtualDeviceGroups {
}
impl DataObjectTrait for VirtualMachineVirtualDeviceGroupsDeviceGroup {
}
impl DataObjectTrait for VirtualMachineVirtualDeviceGroupsVendorDeviceGroup {
}
impl DataObjectTrait for VirtualMachineVirtualDeviceSwap {
}
impl DataObjectTrait for VirtualMachineVirtualDeviceSwapDeviceSwapInfo {
}
impl DataObjectTrait for VirtualHardware {
}
impl DataObjectTrait for VirtualHardwareOption {
}
impl DataObjectTrait for VirtualMachineVirtualNuma {
}
impl DataObjectTrait for VirtualMachineVirtualNumaInfo {
}
impl DataObjectTrait for VirtualMachineVirtualPMem {
}
impl DataObjectTrait for CheckResult {
}
impl DataObjectTrait for CustomizationAdapterMapping {
}
impl DataObjectTrait for CustomizationGlobalIpSettings {
}
impl DataObjectTrait for CustomizationGuiRunOnce {
}
impl DataObjectTrait for CustomizationGuiUnattended {
}
impl DataObjectTrait for CustomizationIpSettings {
}
impl DataObjectTrait for CustomizationIpSettingsIpV6AddressSpec {
}
impl DataObjectTrait for CustomizationIdentification {
}
impl DataObjectTrait for CustomizationIdentitySettings {
}
impl DataObjectTrait for CustomizationCloudinitPrep {
}
impl DataObjectTrait for CustomizationLinuxPrep {
}
impl DataObjectTrait for CustomizationSysprep {
}
impl DataObjectTrait for CustomizationSysprepText {
}
impl DataObjectTrait for CustomizationIpGenerator {
}
impl DataObjectTrait for CustomizationCustomIpGenerator {
}
impl DataObjectTrait for CustomizationDhcpIpGenerator {
}
impl DataObjectTrait for CustomizationFixedIp {
}
impl DataObjectTrait for CustomizationUnknownIpGenerator {
}
impl DataObjectTrait for CustomizationIpV6Generator {
}
impl DataObjectTrait for CustomizationAutoIpV6Generator {
}
impl DataObjectTrait for CustomizationCustomIpV6Generator {
}
impl DataObjectTrait for CustomizationDhcpIpV6Generator {
}
impl DataObjectTrait for CustomizationFixedIpV6 {
}
impl DataObjectTrait for CustomizationStatelessIpV6Generator {
}
impl DataObjectTrait for CustomizationUnknownIpV6Generator {
}
impl DataObjectTrait for CustomizationLicenseFilePrintData {
}
impl DataObjectTrait for CustomizationName {
}
impl DataObjectTrait for CustomizationCustomName {
}
impl DataObjectTrait for CustomizationFixedName {
}
impl DataObjectTrait for CustomizationPrefixName {
}
impl DataObjectTrait for CustomizationUnknownName {
}
impl DataObjectTrait for CustomizationVirtualMachineName {
}
impl DataObjectTrait for CustomizationOptions {
}
impl DataObjectTrait for CustomizationLinuxOptions {
}
impl DataObjectTrait for CustomizationWinOptions {
}
impl DataObjectTrait for CustomizationPassword {
}
impl DataObjectTrait for CustomizationSpec {
}
impl DataObjectTrait for CustomizationUserData {
}
impl DataObjectTrait for HostDiskMappingInfo {
}
impl DataObjectTrait for HostDiskMappingPartitionInfo {
}
impl DataObjectTrait for HostDiskMappingOption {
}
impl DataObjectTrait for HostDiskMappingPartitionOption {
}
impl DataObjectTrait for VirtualDevice {
}
impl DataObjectTrait for VirtualCdrom {
}
impl DataObjectTrait for VirtualController {
}
impl DataObjectTrait for VirtualIdeController {
}
impl DataObjectTrait for VirtualNvdimmController {
}
impl DataObjectTrait for VirtualNvmeController {
}
impl DataObjectTrait for VirtualPciController {
}
impl DataObjectTrait for VirtualPs2Controller {
}
impl DataObjectTrait for VirtualSataController {
}
impl DataObjectTrait for VirtualAhciController {
}
impl DataObjectTrait for VirtualScsiController {
}
impl DataObjectTrait for ParaVirtualScsiController {
}
impl DataObjectTrait for VirtualBusLogicController {
}
impl DataObjectTrait for VirtualLsiLogicController {
}
impl DataObjectTrait for VirtualLsiLogicSasController {
}
impl DataObjectTrait for VirtualSioController {
}
impl DataObjectTrait for VirtualUsbController {
}
impl DataObjectTrait for VirtualUsbxhciController {
}
impl DataObjectTrait for VirtualDisk {
}
impl DataObjectTrait for VirtualEthernetCard {
}
impl DataObjectTrait for VirtualE1000 {
}
impl DataObjectTrait for VirtualE1000E {
}
impl DataObjectTrait for VirtualPcNet32 {
}
impl DataObjectTrait for VirtualSriovEthernetCard {
}
impl DataObjectTrait for VirtualVmxnet {
}
impl DataObjectTrait for VirtualVmxnet2 {
}
impl DataObjectTrait for VirtualVmxnet3 {
}
impl DataObjectTrait for VirtualVmxnet3Vrdma {
}
impl DataObjectTrait for VirtualFloppy {
}
impl DataObjectTrait for VirtualKeyboard {
}
impl DataObjectTrait for VirtualNvdimm {
}
impl DataObjectTrait for VirtualPciPassthrough {
}
impl DataObjectTrait for VirtualParallelPort {
}
impl DataObjectTrait for VirtualPointingDevice {
}
impl DataObjectTrait for VirtualPrecisionClock {
}
impl DataObjectTrait for VirtualScsiPassthrough {
}
impl DataObjectTrait for VirtualSerialPort {
}
impl DataObjectTrait for VirtualSoundCard {
}
impl DataObjectTrait for VirtualEnsoniq1371 {
}
impl DataObjectTrait for VirtualHdAudioCard {
}
impl DataObjectTrait for VirtualSoundBlaster16 {
}
impl DataObjectTrait for VirtualTpm {
}
impl DataObjectTrait for VirtualUsb {
}
impl DataObjectTrait for VirtualMachineVmciDevice {
}
impl DataObjectTrait for VirtualMachineVmirom {
}
impl DataObjectTrait for VirtualMachineVideoCard {
}
impl DataObjectTrait for VirtualWdt {
}
impl DataObjectTrait for VirtualDeviceBackingInfo {
}
impl DataObjectTrait for VirtualDeviceDeviceBackingInfo {
}
impl DataObjectTrait for VirtualCdromAtapiBackingInfo {
}
impl DataObjectTrait for VirtualCdromPassthroughBackingInfo {
}
impl DataObjectTrait for VirtualDiskRawDiskVer2BackingInfo {
}
impl DataObjectTrait for VirtualDiskPartitionedRawDiskVer2BackingInfo {
}
impl DataObjectTrait for VirtualEthernetCardLegacyNetworkBackingInfo {
}
impl DataObjectTrait for VirtualEthernetCardNetworkBackingInfo {
}
impl DataObjectTrait for VirtualFloppyDeviceBackingInfo {
}
impl DataObjectTrait for VirtualPciPassthroughDeviceBackingInfo {
}
impl DataObjectTrait for VirtualPciPassthroughDynamicBackingInfo {
}
impl DataObjectTrait for VirtualParallelPortDeviceBackingInfo {
}
impl DataObjectTrait for VirtualPointingDeviceDeviceBackingInfo {
}
impl DataObjectTrait for VirtualScsiPassthroughDeviceBackingInfo {
}
impl DataObjectTrait for VirtualSerialPortDeviceBackingInfo {
}
impl DataObjectTrait for VirtualSoundCardDeviceBackingInfo {
}
impl DataObjectTrait for VirtualUsbRemoteHostBackingInfo {
}
impl DataObjectTrait for VirtualUsbusbBackingInfo {
}
impl DataObjectTrait for VirtualDeviceFileBackingInfo {
}
impl DataObjectTrait for VirtualCdromIsoBackingInfo {
}
impl DataObjectTrait for VirtualDiskFlatVer1BackingInfo {
}
impl DataObjectTrait for VirtualDiskFlatVer2BackingInfo {
}
impl DataObjectTrait for VirtualDiskLocalPMemBackingInfo {
}
impl DataObjectTrait for VirtualDiskRawDiskMappingVer1BackingInfo {
}
impl DataObjectTrait for VirtualDiskSeSparseBackingInfo {
}
impl DataObjectTrait for VirtualDiskSparseVer1BackingInfo {
}
impl DataObjectTrait for VirtualDiskSparseVer2BackingInfo {
}
impl DataObjectTrait for VirtualFloppyImageBackingInfo {
}
impl DataObjectTrait for VirtualNvdimmBackingInfo {
}
impl DataObjectTrait for VirtualParallelPortFileBackingInfo {
}
impl DataObjectTrait for VirtualSerialPortFileBackingInfo {
}
impl DataObjectTrait for VirtualDevicePipeBackingInfo {
}
impl DataObjectTrait for VirtualSerialPortPipeBackingInfo {
}
impl DataObjectTrait for VirtualDeviceRemoteDeviceBackingInfo {
}
impl DataObjectTrait for VirtualCdromRemoteAtapiBackingInfo {
}
impl DataObjectTrait for VirtualCdromRemotePassthroughBackingInfo {
}
impl DataObjectTrait for VirtualFloppyRemoteDeviceBackingInfo {
}
impl DataObjectTrait for VirtualUsbRemoteClientBackingInfo {
}
impl DataObjectTrait for VirtualDeviceUriBackingInfo {
}
impl DataObjectTrait for VirtualSerialPortUriBackingInfo {
}
impl DataObjectTrait for VirtualEthernetCardDistributedVirtualPortBackingInfo {
}
impl DataObjectTrait for VirtualEthernetCardOpaqueNetworkBackingInfo {
}
impl DataObjectTrait for VirtualPciPassthroughDvxBackingInfo {
}
impl DataObjectTrait for VirtualPciPassthroughPluginBackingInfo {
}
impl DataObjectTrait for VirtualPciPassthroughVmiopBackingInfo {
}
impl DataObjectTrait for VirtualPrecisionClockSystemClockBackingInfo {
}
impl DataObjectTrait for VirtualSerialPortThinPrintBackingInfo {
}
impl DataObjectTrait for VirtualSriovEthernetCardSriovBackingInfo {
}
impl DataObjectTrait for VirtualDeviceBusSlotInfo {
}
impl DataObjectTrait for VirtualDevicePciBusSlotInfo {
}
impl DataObjectTrait for VirtualUsbControllerPciBusSlotInfo {
}
impl DataObjectTrait for VirtualDeviceConnectInfo {
}
impl DataObjectTrait for VirtualDeviceDeviceGroupInfo {
}
impl DataObjectTrait for VirtualDeviceOption {
}
impl DataObjectTrait for VirtualCdromOption {
}
impl DataObjectTrait for VirtualControllerOption {
}
impl DataObjectTrait for VirtualIdeControllerOption {
}
impl DataObjectTrait for VirtualNvdimmControllerOption {
}
impl DataObjectTrait for VirtualNvmeControllerOption {
}
impl DataObjectTrait for VirtualPciControllerOption {
}
impl DataObjectTrait for VirtualPs2ControllerOption {
}
impl DataObjectTrait for VirtualSataControllerOption {
}
impl DataObjectTrait for VirtualAhciControllerOption {
}
impl DataObjectTrait for VirtualScsiControllerOption {
}
impl DataObjectTrait for ParaVirtualScsiControllerOption {
}
impl DataObjectTrait for VirtualBusLogicControllerOption {
}
impl DataObjectTrait for VirtualLsiLogicControllerOption {
}
impl DataObjectTrait for VirtualLsiLogicSasControllerOption {
}
impl DataObjectTrait for VirtualSioControllerOption {
}
impl DataObjectTrait for VirtualUsbControllerOption {
}
impl DataObjectTrait for VirtualUsbxhciControllerOption {
}
impl DataObjectTrait for VirtualDiskOption {
}
impl DataObjectTrait for VirtualEthernetCardOption {
}
impl DataObjectTrait for VirtualE1000Option {
}
impl DataObjectTrait for VirtualE1000EOption {
}
impl DataObjectTrait for VirtualPcNet32Option {
}
impl DataObjectTrait for VirtualSriovEthernetCardOption {
}
impl DataObjectTrait for VirtualVmxnetOption {
}
impl DataObjectTrait for VirtualVmxnet2Option {
}
impl DataObjectTrait for VirtualVmxnet3Option {
}
impl DataObjectTrait for VirtualVmxnet3VrdmaOption {
}
impl DataObjectTrait for VirtualFloppyOption {
}
impl DataObjectTrait for VirtualKeyboardOption {
}
impl DataObjectTrait for VirtualNvdimmOption {
}
impl DataObjectTrait for VirtualPciPassthroughOption {
}
impl DataObjectTrait for VirtualParallelPortOption {
}
impl DataObjectTrait for VirtualPointingDeviceOption {
}
impl DataObjectTrait for VirtualPrecisionClockOption {
}
impl DataObjectTrait for VirtualScsiPassthroughOption {
}
impl DataObjectTrait for VirtualSerialPortOption {
}
impl DataObjectTrait for VirtualSoundCardOption {
}
impl DataObjectTrait for VirtualEnsoniq1371Option {
}
impl DataObjectTrait for VirtualHdAudioCardOption {
}
impl DataObjectTrait for VirtualSoundBlaster16Option {
}
impl DataObjectTrait for VirtualTpmOption {
}
impl DataObjectTrait for VirtualUsbOption {
}
impl DataObjectTrait for VirtualMachineVmciDeviceOption {
}
impl DataObjectTrait for VirtualVmiromOption {
}
impl DataObjectTrait for VirtualVideoCardOption {
}
impl DataObjectTrait for VirtualWdtOption {
}
impl DataObjectTrait for VirtualDeviceBackingOption {
}
impl DataObjectTrait for VirtualDeviceDeviceBackingOption {
}
impl DataObjectTrait for VirtualCdromAtapiBackingOption {
}
impl DataObjectTrait for VirtualCdromPassthroughBackingOption {
}
impl DataObjectTrait for VirtualCdromRemoteAtapiBackingOption {
}
impl DataObjectTrait for VirtualDiskRawDiskMappingVer1BackingOption {
}
impl DataObjectTrait for VirtualDiskRawDiskVer2BackingOption {
}
impl DataObjectTrait for VirtualDiskPartitionedRawDiskVer2BackingOption {
}
impl DataObjectTrait for VirtualEthernetCardLegacyNetworkBackingOption {
}
impl DataObjectTrait for VirtualEthernetCardNetworkBackingOption {
}
impl DataObjectTrait for VirtualFloppyDeviceBackingOption {
}
impl DataObjectTrait for VirtualPciPassthroughDeviceBackingOption {
}
impl DataObjectTrait for VirtualPciPassthroughDynamicBackingOption {
}
impl DataObjectTrait for VirtualParallelPortDeviceBackingOption {
}
impl DataObjectTrait for VirtualPointingDeviceBackingOption {
}
impl DataObjectTrait for VirtualScsiPassthroughDeviceBackingOption {
}
impl DataObjectTrait for VirtualSerialPortDeviceBackingOption {
}
impl DataObjectTrait for VirtualSoundCardDeviceBackingOption {
}
impl DataObjectTrait for VirtualUsbRemoteHostBackingOption {
}
impl DataObjectTrait for VirtualUsbusbBackingOption {
}
impl DataObjectTrait for VirtualDeviceFileBackingOption {
}
impl DataObjectTrait for VirtualCdromIsoBackingOption {
}
impl DataObjectTrait for VirtualDiskFlatVer1BackingOption {
}
impl DataObjectTrait for VirtualDiskFlatVer2BackingOption {
}
impl DataObjectTrait for VirtualDiskLocalPMemBackingOption {
}
impl DataObjectTrait for VirtualDiskSeSparseBackingOption {
}
impl DataObjectTrait for VirtualDiskSparseVer1BackingOption {
}
impl DataObjectTrait for VirtualDiskSparseVer2BackingOption {
}
impl DataObjectTrait for VirtualFloppyImageBackingOption {
}
impl DataObjectTrait for VirtualParallelPortFileBackingOption {
}
impl DataObjectTrait for VirtualSerialPortFileBackingOption {
}
impl DataObjectTrait for VirtualDevicePipeBackingOption {
}
impl DataObjectTrait for VirtualSerialPortPipeBackingOption {
}
impl DataObjectTrait for VirtualDeviceRemoteDeviceBackingOption {
}
impl DataObjectTrait for VirtualCdromRemotePassthroughBackingOption {
}
impl DataObjectTrait for VirtualFloppyRemoteDeviceBackingOption {
}
impl DataObjectTrait for VirtualUsbRemoteClientBackingOption {
}
impl DataObjectTrait for VirtualDeviceUriBackingOption {
}
impl DataObjectTrait for VirtualSerialPortUriBackingOption {
}
impl DataObjectTrait for VirtualEthernetCardDvPortBackingOption {
}
impl DataObjectTrait for VirtualEthernetCardOpaqueNetworkBackingOption {
}
impl DataObjectTrait for VirtualPciPassthroughDvxBackingOption {
}
impl DataObjectTrait for VirtualPciPassthroughPluginBackingOption {
}
impl DataObjectTrait for VirtualPciPassthroughVmiopBackingOption {
}
impl DataObjectTrait for VirtualPrecisionClockSystemClockBackingOption {
}
impl DataObjectTrait for VirtualSerialPortThinPrintBackingOption {
}
impl DataObjectTrait for VirtualSriovEthernetCardSriovBackingOption {
}
impl DataObjectTrait for VirtualDeviceBusSlotOption {
}
impl DataObjectTrait for VirtualDeviceConnectOption {
}
impl DataObjectTrait for VirtualDeviceConfigSpec {
}
impl DataObjectTrait for VirtualDiskConfigSpec {
}
impl DataObjectTrait for VirtualDeviceConfigSpecBackingSpec {
}
impl DataObjectTrait for VirtualDiskVFlashCacheConfigInfo {
}
impl DataObjectTrait for VirtualDiskId {
}
impl DataObjectTrait for VirtualDiskDeltaDiskFormatsSupported {
}
impl DataObjectTrait for VirtualDiskOptionVFlashCacheConfigOption {
}
impl DataObjectTrait for VirtualEthernetCardResourceAllocation {
}
impl DataObjectTrait for VirtualPciPassthroughAllowedDevice {
}
impl DataObjectTrait for VirtualMachineVmciDeviceFilterInfo {
}
impl DataObjectTrait for VirtualMachineVmciDeviceFilterSpec {
}
impl DataObjectTrait for VirtualMachineVmciDeviceOptionFilterSpecOption {
}
impl DataObjectTrait for GuestAliases {
}
impl DataObjectTrait for GuestAuthAliasInfo {
}
impl DataObjectTrait for GuestAuthSubject {
}
impl DataObjectTrait for GuestAuthAnySubject {
}
impl DataObjectTrait for GuestAuthNamedSubject {
}
impl DataObjectTrait for GuestMappedAliases {
}
impl DataObjectTrait for GuestFileAttributes {
}
impl DataObjectTrait for GuestPosixFileAttributes {
}
impl DataObjectTrait for GuestWindowsFileAttributes {
}
impl DataObjectTrait for GuestFileInfo {
}
impl DataObjectTrait for FileTransferInformation {
}
impl DataObjectTrait for GuestListFileInfo {
}
impl DataObjectTrait for GuestAuthentication {
}
impl DataObjectTrait for NamePasswordAuthentication {
}
impl DataObjectTrait for SamlTokenAuthentication {
}
impl DataObjectTrait for SspiAuthentication {
}
impl DataObjectTrait for TicketedSessionAuthentication {
}
impl DataObjectTrait for GuestProcessInfo {
}
impl DataObjectTrait for GuestProgramSpec {
}
impl DataObjectTrait for GuestWindowsProgramSpec {
}
impl DataObjectTrait for GuestRegKeySpec {
}
impl DataObjectTrait for GuestRegKeyNameSpec {
}
impl DataObjectTrait for GuestRegKeyRecordSpec {
}
impl DataObjectTrait for GuestRegValueSpec {
}
impl DataObjectTrait for GuestRegValueDataSpec {
}
impl DataObjectTrait for GuestRegValueBinarySpec {
}
impl DataObjectTrait for GuestRegValueDwordSpec {
}
impl DataObjectTrait for GuestRegValueExpandStringSpec {
}
impl DataObjectTrait for GuestRegValueMultiStringSpec {
}
impl DataObjectTrait for GuestRegValueQwordSpec {
}
impl DataObjectTrait for GuestRegValueStringSpec {
}
impl DataObjectTrait for GuestRegValueNameSpec {
}
impl DataObjectTrait for DeviceGroupId {
}
impl DataObjectTrait for FaultDomainId {
}
impl DataObjectTrait for ReplicationGroupId {
}
impl DataObjectTrait for ReplicationSpec {
}
impl DataObjectTrait for VsanClusterConfigInfo {
}
impl DataObjectTrait for VsanClusterConfigInfoHostDefaultInfo {
}
impl DataObjectTrait for VsanHostClusterStatus {
}
impl DataObjectTrait for VsanHostClusterStatusState {
}
impl DataObjectTrait for VsanHostClusterStatusStateCompletionEstimate {
}
impl DataObjectTrait for VsanHostConfigInfo {
}
impl DataObjectTrait for VsanHostConfigInfoClusterInfo {
}
impl DataObjectTrait for VsanHostFaultDomainInfo {
}
impl DataObjectTrait for VsanHostConfigInfoNetworkInfo {
}
impl DataObjectTrait for VsanHostConfigInfoNetworkInfoPortConfig {
}
impl DataObjectTrait for VsanHostConfigInfoStorageInfo {
}
impl DataObjectTrait for VsanHostDecommissionMode {
}
impl DataObjectTrait for VsanHostDiskMapInfo {
}
impl DataObjectTrait for VsanHostDiskMapResult {
}
impl DataObjectTrait for VsanHostDiskMapping {
}
impl DataObjectTrait for VsanHostDiskResult {
}
impl DataObjectTrait for VsanHostIpConfig {
}
impl DataObjectTrait for VsanHostMembershipInfo {
}
impl DataObjectTrait for VsanHostVsanDiskInfo {
}
impl DataObjectTrait for VsanHostRuntimeInfo {
}
impl DataObjectTrait for VsanHostRuntimeInfoDiskIssue {
}
impl DataObjectTrait for BaseConfigInfo {
}
impl DataObjectTrait for VStorageObjectConfigInfo {
}
impl DataObjectTrait for BaseConfigInfoBackingInfo {
}
impl DataObjectTrait for BaseConfigInfoFileBackingInfo {
}
impl DataObjectTrait for BaseConfigInfoDiskFileBackingInfo {
}
impl DataObjectTrait for BaseConfigInfoRawDiskMappingBackingInfo {
}
impl DataObjectTrait for VslmCreateSpec {
}
impl DataObjectTrait for VslmCreateSpecBackingSpec {
}
impl DataObjectTrait for VslmCreateSpecDiskFileBackingSpec {
}
impl DataObjectTrait for VslmCreateSpecRawDiskMappingBackingSpec {
}
impl DataObjectTrait for DiskCryptoSpec {
}
impl DataObjectTrait for Id {
}
impl DataObjectTrait for VslmInfrastructureObjectPolicy {
}
impl DataObjectTrait for VslmInfrastructureObjectPolicySpec {
}
impl DataObjectTrait for VslmMigrateSpec {
}
impl DataObjectTrait for VslmCloneSpec {
}
impl DataObjectTrait for VslmRelocateSpec {
}
impl DataObjectTrait for VStorageObjectStateInfo {
}
impl DataObjectTrait for VslmTagEntry {
}
impl DataObjectTrait for VslmVClockInfo {
}
impl DataObjectTrait for VStorageObject {
}
impl DataObjectTrait for VStorageObjectSnapshot {
}
impl DataObjectTrait for VStorageObjectSnapshotDetails {
}
impl DataObjectTrait for VStorageObjectSnapshotInfo {
}
impl DataObjectTrait for VStorageObjectSnapshotInfoVStorageObjectSnapshot {
}
impl DataObjectTrait for RetrieveVStorageObjSpec {
}
impl DataObjectTrait for VStorageObjectAssociations {
}
impl DataObjectTrait for VStorageObjectAssociationsVmDiskAssociations {
}
impl DataObjectTrait for DynamicArray {
}
impl DataObjectTrait for DynamicProperty {
}
impl DataObjectTrait for KeyAnyValue {
}
impl DataObjectTrait for LocalizableMessage {
}
impl DataObjectTrait for LocalizedMethodFault {
}
impl DataObjectTrait for PropertyChange {
}
impl DataObjectTrait for PropertyFilterSpec {
}
impl DataObjectTrait for PropertyFilterUpdate {
}
impl DataObjectTrait for MissingObject {
}
impl DataObjectTrait for MissingProperty {
}
impl DataObjectTrait for ObjectContent {
}
impl DataObjectTrait for ObjectSpec {
}
impl DataObjectTrait for ObjectUpdate {
}
impl DataObjectTrait for PropertySpec {
}
impl DataObjectTrait for RetrieveOptions {
}
impl DataObjectTrait for RetrieveResult {
}
impl DataObjectTrait for SelectionSpec {
}
impl DataObjectTrait for TraversalSpec {
}
impl DataObjectTrait for UpdateSet {
}
impl DataObjectTrait for WaitOptions {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DataObjectTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DataObject => Some(from.as_any_ref().downcast_ref::<DataObject>()?),
            StructType::AboutInfo => Some(from.as_any_ref().downcast_ref::<AboutInfo>()?),
            StructType::AuthorizationDescription => Some(from.as_any_ref().downcast_ref::<AuthorizationDescription>()?),
            StructType::EntityPrivilege => Some(from.as_any_ref().downcast_ref::<EntityPrivilege>()?),
            StructType::Permission => Some(from.as_any_ref().downcast_ref::<Permission>()?),
            StructType::AuthorizationPrivilege => Some(from.as_any_ref().downcast_ref::<AuthorizationPrivilege>()?),
            StructType::PrivilegeAvailability => Some(from.as_any_ref().downcast_ref::<PrivilegeAvailability>()?),
            StructType::AuthorizationRole => Some(from.as_any_ref().downcast_ref::<AuthorizationRole>()?),
            StructType::UserPrivilegeResult => Some(from.as_any_ref().downcast_ref::<UserPrivilegeResult>()?),
            StructType::BatchResult => Some(from.as_any_ref().downcast_ref::<BatchResult>()?),
            StructType::Capability => Some(from.as_any_ref().downcast_ref::<Capability>()?),
            StructType::ClusterComputeResourceClusterConfigResult => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceClusterConfigResult>()?),
            StructType::ClusterComputeResourceDvsSetting => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceDvsSetting>()?),
            StructType::ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping>()?),
            StructType::ClusterComputeResourceDvsProfile => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceDvsProfile>()?),
            StructType::ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping>()?),
            StructType::ClusterComputeResourceHciConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceHciConfigInfo>()?),
            StructType::ClusterComputeResourceHciConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceHciConfigSpec>()?),
            StructType::ClusterComputeResourceHostConfigurationInput => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceHostConfigurationInput>()?),
            StructType::ClusterComputeResourceHostConfigurationProfile => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceHostConfigurationProfile>()?),
            StructType::ClusterComputeResourceHostVmkNicInfo => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceHostVmkNicInfo>()?),
            StructType::ClusterComputeResourceVcProfile => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceVcProfile>()?),
            StructType::ClusterComputeResourceValidationResultBase => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceValidationResultBase>()?),
            StructType::ClusterComputeResourceDvsConfigurationValidation => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceDvsConfigurationValidation>()?),
            StructType::ClusterComputeResourceHostConfigurationValidation => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceHostConfigurationValidation>()?),
            StructType::ClusterComputeResourceVcsSlots => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceVcsSlots>()?),
            StructType::ComputeResourceConfigInfo => Some(from.as_any_ref().downcast_ref::<ComputeResourceConfigInfo>()?),
            StructType::ClusterConfigInfoEx => Some(from.as_any_ref().downcast_ref::<ClusterConfigInfoEx>()?),
            StructType::ComputeResourceConfigSpec => Some(from.as_any_ref().downcast_ref::<ComputeResourceConfigSpec>()?),
            StructType::ClusterConfigSpecEx => Some(from.as_any_ref().downcast_ref::<ClusterConfigSpecEx>()?),
            StructType::ComputeResourceHostSpbmLicenseInfo => Some(from.as_any_ref().downcast_ref::<ComputeResourceHostSpbmLicenseInfo>()?),
            StructType::ComputeResourceSummary => Some(from.as_any_ref().downcast_ref::<ComputeResourceSummary>()?),
            StructType::ClusterComputeResourceSummary => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceSummary>()?),
            StructType::CustomFieldDef => Some(from.as_any_ref().downcast_ref::<CustomFieldDef>()?),
            StructType::CustomFieldValue => Some(from.as_any_ref().downcast_ref::<CustomFieldValue>()?),
            StructType::CustomFieldStringValue => Some(from.as_any_ref().downcast_ref::<CustomFieldStringValue>()?),
            StructType::CustomizationSpecInfo => Some(from.as_any_ref().downcast_ref::<CustomizationSpecInfo>()?),
            StructType::CustomizationSpecItem => Some(from.as_any_ref().downcast_ref::<CustomizationSpecItem>()?),
            StructType::DatacenterBasicConnectInfo => Some(from.as_any_ref().downcast_ref::<DatacenterBasicConnectInfo>()?),
            StructType::DatacenterConfigInfo => Some(from.as_any_ref().downcast_ref::<DatacenterConfigInfo>()?),
            StructType::DatacenterConfigSpec => Some(from.as_any_ref().downcast_ref::<DatacenterConfigSpec>()?),
            StructType::DatastoreCapability => Some(from.as_any_ref().downcast_ref::<DatastoreCapability>()?),
            StructType::DatastoreHostMount => Some(from.as_any_ref().downcast_ref::<DatastoreHostMount>()?),
            StructType::DatastoreInfo => Some(from.as_any_ref().downcast_ref::<DatastoreInfo>()?),
            StructType::LocalDatastoreInfo => Some(from.as_any_ref().downcast_ref::<LocalDatastoreInfo>()?),
            StructType::NasDatastoreInfo => Some(from.as_any_ref().downcast_ref::<NasDatastoreInfo>()?),
            StructType::PMemDatastoreInfo => Some(from.as_any_ref().downcast_ref::<PMemDatastoreInfo>()?),
            StructType::VmfsDatastoreInfo => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreInfo>()?),
            StructType::VsanDatastoreInfo => Some(from.as_any_ref().downcast_ref::<VsanDatastoreInfo>()?),
            StructType::VvolDatastoreInfo => Some(from.as_any_ref().downcast_ref::<VvolDatastoreInfo>()?),
            StructType::DatastoreMountPathDatastorePair => Some(from.as_any_ref().downcast_ref::<DatastoreMountPathDatastorePair>()?),
            StructType::DatastoreSummary => Some(from.as_any_ref().downcast_ref::<DatastoreSummary>()?),
            StructType::DatastoreVVolContainerFailoverPair => Some(from.as_any_ref().downcast_ref::<DatastoreVVolContainerFailoverPair>()?),
            StructType::DatastoreNamespaceManagerDirectoryInfo => Some(from.as_any_ref().downcast_ref::<DatastoreNamespaceManagerDirectoryInfo>()?),
            StructType::Description => Some(from.as_any_ref().downcast_ref::<Description>()?),
            StructType::ElementDescription => Some(from.as_any_ref().downcast_ref::<ElementDescription>()?),
            StructType::EvcMode => Some(from.as_any_ref().downcast_ref::<EvcMode>()?),
            StructType::ExtendedElementDescription => Some(from.as_any_ref().downcast_ref::<ExtendedElementDescription>()?),
            StructType::FeatureEvcMode => Some(from.as_any_ref().downcast_ref::<FeatureEvcMode>()?),
            StructType::OptionDef => Some(from.as_any_ref().downcast_ref::<OptionDef>()?),
            StructType::ExtendedDescription => Some(from.as_any_ref().downcast_ref::<ExtendedDescription>()?),
            StructType::MethodDescription => Some(from.as_any_ref().downcast_ref::<MethodDescription>()?),
            StructType::TypeDescription => Some(from.as_any_ref().downcast_ref::<TypeDescription>()?),
            StructType::ScheduledTaskDetail => Some(from.as_any_ref().downcast_ref::<ScheduledTaskDetail>()?),
            StructType::DesiredSoftwareSpec => Some(from.as_any_ref().downcast_ref::<DesiredSoftwareSpec>()?),
            StructType::DesiredSoftwareSpecBaseImageSpec => Some(from.as_any_ref().downcast_ref::<DesiredSoftwareSpecBaseImageSpec>()?),
            StructType::DesiredSoftwareSpecComponentSpec => Some(from.as_any_ref().downcast_ref::<DesiredSoftwareSpecComponentSpec>()?),
            StructType::DesiredSoftwareSpecVendorAddOnSpec => Some(from.as_any_ref().downcast_ref::<DesiredSoftwareSpecVendorAddOnSpec>()?),
            StructType::DiagnosticManagerAuditRecordResult => Some(from.as_any_ref().downcast_ref::<DiagnosticManagerAuditRecordResult>()?),
            StructType::DiagnosticManagerBundleInfo => Some(from.as_any_ref().downcast_ref::<DiagnosticManagerBundleInfo>()?),
            StructType::DiagnosticManagerLogDescriptor => Some(from.as_any_ref().downcast_ref::<DiagnosticManagerLogDescriptor>()?),
            StructType::DiagnosticManagerLogHeader => Some(from.as_any_ref().downcast_ref::<DiagnosticManagerLogHeader>()?),
            StructType::DvsBackupRestoreCapability => Some(from.as_any_ref().downcast_ref::<DvsBackupRestoreCapability>()?),
            StructType::DvsCapability => Some(from.as_any_ref().downcast_ref::<DvsCapability>()?),
            StructType::DvsConfigInfo => Some(from.as_any_ref().downcast_ref::<DvsConfigInfo>()?),
            StructType::VMwareDvsConfigInfo => Some(from.as_any_ref().downcast_ref::<VMwareDvsConfigInfo>()?),
            StructType::DvsConfigSpec => Some(from.as_any_ref().downcast_ref::<DvsConfigSpec>()?),
            StructType::VMwareDvsConfigSpec => Some(from.as_any_ref().downcast_ref::<VMwareDvsConfigSpec>()?),
            StructType::DvsContactInfo => Some(from.as_any_ref().downcast_ref::<DvsContactInfo>()?),
            StructType::DvsCreateSpec => Some(from.as_any_ref().downcast_ref::<DvsCreateSpec>()?),
            StructType::DvsFeatureCapability => Some(from.as_any_ref().downcast_ref::<DvsFeatureCapability>()?),
            StructType::VMwareDvsFeatureCapability => Some(from.as_any_ref().downcast_ref::<VMwareDvsFeatureCapability>()?),
            StructType::DvsHealthCheckConfig => Some(from.as_any_ref().downcast_ref::<DvsHealthCheckConfig>()?),
            StructType::VMwareDvsHealthCheckConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsHealthCheckConfig>()?),
            StructType::VMwareDvsTeamingHealthCheckConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsTeamingHealthCheckConfig>()?),
            StructType::VMwareDvsVlanMtuHealthCheckConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsVlanMtuHealthCheckConfig>()?),
            StructType::DvsHealthCheckCapability => Some(from.as_any_ref().downcast_ref::<DvsHealthCheckCapability>()?),
            StructType::VMwareDvsHealthCheckCapability => Some(from.as_any_ref().downcast_ref::<VMwareDvsHealthCheckCapability>()?),
            StructType::DvsHostInfrastructureTrafficResource => Some(from.as_any_ref().downcast_ref::<DvsHostInfrastructureTrafficResource>()?),
            StructType::DvsHostInfrastructureTrafficResourceAllocation => Some(from.as_any_ref().downcast_ref::<DvsHostInfrastructureTrafficResourceAllocation>()?),
            StructType::DvsNetworkResourceManagementCapability => Some(from.as_any_ref().downcast_ref::<DvsNetworkResourceManagementCapability>()?),
            StructType::DvsResourceRuntimeInfo => Some(from.as_any_ref().downcast_ref::<DvsResourceRuntimeInfo>()?),
            StructType::DvsRollbackCapability => Some(from.as_any_ref().downcast_ref::<DvsRollbackCapability>()?),
            StructType::DvsRuntimeInfo => Some(from.as_any_ref().downcast_ref::<DvsRuntimeInfo>()?),
            StructType::DvsSummary => Some(from.as_any_ref().downcast_ref::<DvsSummary>()?),
            StructType::DvsPolicy => Some(from.as_any_ref().downcast_ref::<DvsPolicy>()?),
            StructType::DvsUplinkPortPolicy => Some(from.as_any_ref().downcast_ref::<DvsUplinkPortPolicy>()?),
            StructType::DvsNameArrayUplinkPortPolicy => Some(from.as_any_ref().downcast_ref::<DvsNameArrayUplinkPortPolicy>()?),
            StructType::EnumDescription => Some(from.as_any_ref().downcast_ref::<EnumDescription>()?),
            StructType::EnvironmentBrowserConfigOptionQuerySpec => Some(from.as_any_ref().downcast_ref::<EnvironmentBrowserConfigOptionQuerySpec>()?),
            StructType::Extension => Some(from.as_any_ref().downcast_ref::<Extension>()?),
            StructType::ExtensionClientInfo => Some(from.as_any_ref().downcast_ref::<ExtensionClientInfo>()?),
            StructType::ExtensionEventTypeInfo => Some(from.as_any_ref().downcast_ref::<ExtensionEventTypeInfo>()?),
            StructType::ExtensionFaultTypeInfo => Some(from.as_any_ref().downcast_ref::<ExtensionFaultTypeInfo>()?),
            StructType::ExtensionHealthInfo => Some(from.as_any_ref().downcast_ref::<ExtensionHealthInfo>()?),
            StructType::ExtensionOvfConsumerInfo => Some(from.as_any_ref().downcast_ref::<ExtensionOvfConsumerInfo>()?),
            StructType::ExtensionPrivilegeInfo => Some(from.as_any_ref().downcast_ref::<ExtensionPrivilegeInfo>()?),
            StructType::ExtensionResourceInfo => Some(from.as_any_ref().downcast_ref::<ExtensionResourceInfo>()?),
            StructType::ExtensionServerInfo => Some(from.as_any_ref().downcast_ref::<ExtensionServerInfo>()?),
            StructType::ExtensionTaskTypeInfo => Some(from.as_any_ref().downcast_ref::<ExtensionTaskTypeInfo>()?),
            StructType::ExtensionManagerIpAllocationUsage => Some(from.as_any_ref().downcast_ref::<ExtensionManagerIpAllocationUsage>()?),
            StructType::FaultsByHost => Some(from.as_any_ref().downcast_ref::<FaultsByHost>()?),
            StructType::FaultsByVm => Some(from.as_any_ref().downcast_ref::<FaultsByVm>()?),
            StructType::FileLockInfo => Some(from.as_any_ref().downcast_ref::<FileLockInfo>()?),
            StructType::FileLockInfoResult => Some(from.as_any_ref().downcast_ref::<FileLockInfoResult>()?),
            StructType::FolderBatchAddHostsToClusterResult => Some(from.as_any_ref().downcast_ref::<FolderBatchAddHostsToClusterResult>()?),
            StructType::FolderBatchAddStandaloneHostsResult => Some(from.as_any_ref().downcast_ref::<FolderBatchAddStandaloneHostsResult>()?),
            StructType::FolderFailedHostResult => Some(from.as_any_ref().downcast_ref::<FolderFailedHostResult>()?),
            StructType::FolderNewHostSpec => Some(from.as_any_ref().downcast_ref::<FolderNewHostSpec>()?),
            StructType::HbrManagerReplicationVmInfo => Some(from.as_any_ref().downcast_ref::<HbrManagerReplicationVmInfo>()?),
            StructType::ReplicationVmProgressInfo => Some(from.as_any_ref().downcast_ref::<ReplicationVmProgressInfo>()?),
            StructType::HbrManagerVmReplicationCapability => Some(from.as_any_ref().downcast_ref::<HbrManagerVmReplicationCapability>()?),
            StructType::HealthUpdate => Some(from.as_any_ref().downcast_ref::<HealthUpdate>()?),
            StructType::HealthUpdateInfo => Some(from.as_any_ref().downcast_ref::<HealthUpdateInfo>()?),
            StructType::PerfInterval => Some(from.as_any_ref().downcast_ref::<PerfInterval>()?),
            StructType::HostServiceTicket => Some(from.as_any_ref().downcast_ref::<HostServiceTicket>()?),
            StructType::HostSystemComplianceCheckState => Some(from.as_any_ref().downcast_ref::<HostSystemComplianceCheckState>()?),
            StructType::HostSystemReconnectSpec => Some(from.as_any_ref().downcast_ref::<HostSystemReconnectSpec>()?),
            StructType::HostSystemRemediationState => Some(from.as_any_ref().downcast_ref::<HostSystemRemediationState>()?),
            StructType::HttpNfcLeaseCapabilities => Some(from.as_any_ref().downcast_ref::<HttpNfcLeaseCapabilities>()?),
            StructType::HttpNfcLeaseDatastoreLeaseInfo => Some(from.as_any_ref().downcast_ref::<HttpNfcLeaseDatastoreLeaseInfo>()?),
            StructType::HttpNfcLeaseDeviceUrl => Some(from.as_any_ref().downcast_ref::<HttpNfcLeaseDeviceUrl>()?),
            StructType::HttpNfcLeaseHostInfo => Some(from.as_any_ref().downcast_ref::<HttpNfcLeaseHostInfo>()?),
            StructType::HttpNfcLeaseInfo => Some(from.as_any_ref().downcast_ref::<HttpNfcLeaseInfo>()?),
            StructType::HttpNfcLeaseManifestEntry => Some(from.as_any_ref().downcast_ref::<HttpNfcLeaseManifestEntry>()?),
            StructType::HttpNfcLeaseProbeResult => Some(from.as_any_ref().downcast_ref::<HttpNfcLeaseProbeResult>()?),
            StructType::HttpNfcLeaseSourceFile => Some(from.as_any_ref().downcast_ref::<HttpNfcLeaseSourceFile>()?),
            StructType::ImportSpec => Some(from.as_any_ref().downcast_ref::<ImportSpec>()?),
            StructType::VirtualAppImportSpec => Some(from.as_any_ref().downcast_ref::<VirtualAppImportSpec>()?),
            StructType::VirtualMachineImportSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineImportSpec>()?),
            StructType::InheritablePolicy => Some(from.as_any_ref().downcast_ref::<InheritablePolicy>()?),
            StructType::BoolPolicy => Some(from.as_any_ref().downcast_ref::<BoolPolicy>()?),
            StructType::IntPolicy => Some(from.as_any_ref().downcast_ref::<IntPolicy>()?),
            StructType::LongPolicy => Some(from.as_any_ref().downcast_ref::<LongPolicy>()?),
            StructType::StringPolicy => Some(from.as_any_ref().downcast_ref::<StringPolicy>()?),
            StructType::DvsFilterConfig => Some(from.as_any_ref().downcast_ref::<DvsFilterConfig>()?),
            StructType::DvsFilterConfigSpec => Some(from.as_any_ref().downcast_ref::<DvsFilterConfigSpec>()?),
            StructType::DvsTrafficFilterConfig => Some(from.as_any_ref().downcast_ref::<DvsTrafficFilterConfig>()?),
            StructType::DvsTrafficFilterConfigSpec => Some(from.as_any_ref().downcast_ref::<DvsTrafficFilterConfigSpec>()?),
            StructType::DvsFilterPolicy => Some(from.as_any_ref().downcast_ref::<DvsFilterPolicy>()?),
            StructType::DvsTrafficShapingPolicy => Some(from.as_any_ref().downcast_ref::<DvsTrafficShapingPolicy>()?),
            StructType::DvsVendorSpecificConfig => Some(from.as_any_ref().downcast_ref::<DvsVendorSpecificConfig>()?),
            StructType::DvsFailureCriteria => Some(from.as_any_ref().downcast_ref::<DvsFailureCriteria>()?),
            StructType::DvsMacLearningPolicy => Some(from.as_any_ref().downcast_ref::<DvsMacLearningPolicy>()?),
            StructType::DvsMacManagementPolicy => Some(from.as_any_ref().downcast_ref::<DvsMacManagementPolicy>()?),
            StructType::DvsSecurityPolicy => Some(from.as_any_ref().downcast_ref::<DvsSecurityPolicy>()?),
            StructType::VMwareUplinkLacpPolicy => Some(from.as_any_ref().downcast_ref::<VMwareUplinkLacpPolicy>()?),
            StructType::VMwareUplinkPortOrderPolicy => Some(from.as_any_ref().downcast_ref::<VMwareUplinkPortOrderPolicy>()?),
            StructType::VmwareUplinkPortTeamingPolicy => Some(from.as_any_ref().downcast_ref::<VmwareUplinkPortTeamingPolicy>()?),
            StructType::VmwareDistributedVirtualSwitchVlanSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchPvlanSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchPvlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchTrunkVlanSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchTrunkVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchVlanIdSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchVlanIdSpec>()?),
            StructType::IoFilterInfo => Some(from.as_any_ref().downcast_ref::<IoFilterInfo>()?),
            StructType::ClusterIoFilterInfo => Some(from.as_any_ref().downcast_ref::<ClusterIoFilterInfo>()?),
            StructType::HostIoFilterInfo => Some(from.as_any_ref().downcast_ref::<HostIoFilterInfo>()?),
            StructType::IoFilterQueryIssueResult => Some(from.as_any_ref().downcast_ref::<IoFilterQueryIssueResult>()?),
            StructType::IoFilterHostIssue => Some(from.as_any_ref().downcast_ref::<IoFilterHostIssue>()?),
            StructType::IpPoolManagerIpAllocation => Some(from.as_any_ref().downcast_ref::<IpPoolManagerIpAllocation>()?),
            StructType::KeyValue => Some(from.as_any_ref().downcast_ref::<KeyValue>()?),
            StructType::LatencySensitivity => Some(from.as_any_ref().downcast_ref::<LatencySensitivity>()?),
            StructType::LicenseAssignmentManagerLicenseAssignment => Some(from.as_any_ref().downcast_ref::<LicenseAssignmentManagerLicenseAssignment>()?),
            StructType::LicenseAvailabilityInfo => Some(from.as_any_ref().downcast_ref::<LicenseAvailabilityInfo>()?),
            StructType::LicenseDiagnostics => Some(from.as_any_ref().downcast_ref::<LicenseDiagnostics>()?),
            StructType::LicenseManagerEvaluationInfo => Some(from.as_any_ref().downcast_ref::<LicenseManagerEvaluationInfo>()?),
            StructType::LicenseFeatureInfo => Some(from.as_any_ref().downcast_ref::<LicenseFeatureInfo>()?),
            StructType::HostLicensableResourceInfo => Some(from.as_any_ref().downcast_ref::<HostLicensableResourceInfo>()?),
            StructType::LicenseManagerLicenseInfo => Some(from.as_any_ref().downcast_ref::<LicenseManagerLicenseInfo>()?),
            StructType::LicenseSource => Some(from.as_any_ref().downcast_ref::<LicenseSource>()?),
            StructType::EvaluationLicenseSource => Some(from.as_any_ref().downcast_ref::<EvaluationLicenseSource>()?),
            StructType::LicenseServerSource => Some(from.as_any_ref().downcast_ref::<LicenseServerSource>()?),
            StructType::LocalLicenseSource => Some(from.as_any_ref().downcast_ref::<LocalLicenseSource>()?),
            StructType::LicenseUsageInfo => Some(from.as_any_ref().downcast_ref::<LicenseUsageInfo>()?),
            StructType::LicenseReservationInfo => Some(from.as_any_ref().downcast_ref::<LicenseReservationInfo>()?),
            StructType::LocalizationManagerMessageCatalog => Some(from.as_any_ref().downcast_ref::<LocalizationManagerMessageCatalog>()?),
            StructType::NegatableExpression => Some(from.as_any_ref().downcast_ref::<NegatableExpression>()?),
            StructType::IntExpression => Some(from.as_any_ref().downcast_ref::<IntExpression>()?),
            StructType::IpAddress => Some(from.as_any_ref().downcast_ref::<IpAddress>()?),
            StructType::IpRange => Some(from.as_any_ref().downcast_ref::<IpRange>()?),
            StructType::SingleIp => Some(from.as_any_ref().downcast_ref::<SingleIp>()?),
            StructType::MacAddress => Some(from.as_any_ref().downcast_ref::<MacAddress>()?),
            StructType::MacRange => Some(from.as_any_ref().downcast_ref::<MacRange>()?),
            StructType::SingleMac => Some(from.as_any_ref().downcast_ref::<SingleMac>()?),
            StructType::StringExpression => Some(from.as_any_ref().downcast_ref::<StringExpression>()?),
            StructType::DvsIpPort => Some(from.as_any_ref().downcast_ref::<DvsIpPort>()?),
            StructType::DvsIpPortRange => Some(from.as_any_ref().downcast_ref::<DvsIpPortRange>()?),
            StructType::DvsSingleIpPort => Some(from.as_any_ref().downcast_ref::<DvsSingleIpPort>()?),
            StructType::NetworkSummary => Some(from.as_any_ref().downcast_ref::<NetworkSummary>()?),
            StructType::OpaqueNetworkSummary => Some(from.as_any_ref().downcast_ref::<OpaqueNetworkSummary>()?),
            StructType::NumericRange => Some(from.as_any_ref().downcast_ref::<NumericRange>()?),
            StructType::OpaqueNetworkCapability => Some(from.as_any_ref().downcast_ref::<OpaqueNetworkCapability>()?),
            StructType::OvfConsumerOstNode => Some(from.as_any_ref().downcast_ref::<OvfConsumerOstNode>()?),
            StructType::OvfConsumerOvfSection => Some(from.as_any_ref().downcast_ref::<OvfConsumerOvfSection>()?),
            StructType::OvfManagerCommonParams => Some(from.as_any_ref().downcast_ref::<OvfManagerCommonParams>()?),
            StructType::OvfCreateImportSpecParams => Some(from.as_any_ref().downcast_ref::<OvfCreateImportSpecParams>()?),
            StructType::OvfParseDescriptorParams => Some(from.as_any_ref().downcast_ref::<OvfParseDescriptorParams>()?),
            StructType::OvfValidateHostParams => Some(from.as_any_ref().downcast_ref::<OvfValidateHostParams>()?),
            StructType::OvfCreateDescriptorParams => Some(from.as_any_ref().downcast_ref::<OvfCreateDescriptorParams>()?),
            StructType::OvfCreateDescriptorResult => Some(from.as_any_ref().downcast_ref::<OvfCreateDescriptorResult>()?),
            StructType::OvfCreateImportSpecResult => Some(from.as_any_ref().downcast_ref::<OvfCreateImportSpecResult>()?),
            StructType::OvfDeploymentOption => Some(from.as_any_ref().downcast_ref::<OvfDeploymentOption>()?),
            StructType::OvfFileItem => Some(from.as_any_ref().downcast_ref::<OvfFileItem>()?),
            StructType::OvfNetworkInfo => Some(from.as_any_ref().downcast_ref::<OvfNetworkInfo>()?),
            StructType::OvfNetworkMapping => Some(from.as_any_ref().downcast_ref::<OvfNetworkMapping>()?),
            StructType::OvfFile => Some(from.as_any_ref().downcast_ref::<OvfFile>()?),
            StructType::OvfOptionInfo => Some(from.as_any_ref().downcast_ref::<OvfOptionInfo>()?),
            StructType::OvfParseDescriptorResult => Some(from.as_any_ref().downcast_ref::<OvfParseDescriptorResult>()?),
            StructType::OvfResourceMap => Some(from.as_any_ref().downcast_ref::<OvfResourceMap>()?),
            StructType::OvfValidateHostResult => Some(from.as_any_ref().downcast_ref::<OvfValidateHostResult>()?),
            StructType::PasswordField => Some(from.as_any_ref().downcast_ref::<PasswordField>()?),
            StructType::PerformanceDescription => Some(from.as_any_ref().downcast_ref::<PerformanceDescription>()?),
            StructType::PerfCompositeMetric => Some(from.as_any_ref().downcast_ref::<PerfCompositeMetric>()?),
            StructType::PerfCounterInfo => Some(from.as_any_ref().downcast_ref::<PerfCounterInfo>()?),
            StructType::PerformanceManagerCounterLevelMapping => Some(from.as_any_ref().downcast_ref::<PerformanceManagerCounterLevelMapping>()?),
            StructType::PerfEntityMetricBase => Some(from.as_any_ref().downcast_ref::<PerfEntityMetricBase>()?),
            StructType::PerfEntityMetric => Some(from.as_any_ref().downcast_ref::<PerfEntityMetric>()?),
            StructType::PerfEntityMetricCsv => Some(from.as_any_ref().downcast_ref::<PerfEntityMetricCsv>()?),
            StructType::PerfMetricId => Some(from.as_any_ref().downcast_ref::<PerfMetricId>()?),
            StructType::PerfMetricSeries => Some(from.as_any_ref().downcast_ref::<PerfMetricSeries>()?),
            StructType::PerfMetricIntSeries => Some(from.as_any_ref().downcast_ref::<PerfMetricIntSeries>()?),
            StructType::PerfMetricSeriesCsv => Some(from.as_any_ref().downcast_ref::<PerfMetricSeriesCsv>()?),
            StructType::PerfProviderSummary => Some(from.as_any_ref().downcast_ref::<PerfProviderSummary>()?),
            StructType::PerfQuerySpec => Some(from.as_any_ref().downcast_ref::<PerfQuerySpec>()?),
            StructType::PerfSampleInfo => Some(from.as_any_ref().downcast_ref::<PerfSampleInfo>()?),
            StructType::PrivilegePolicyDef => Some(from.as_any_ref().downcast_ref::<PrivilegePolicyDef>()?),
            StructType::ResourceAllocationInfo => Some(from.as_any_ref().downcast_ref::<ResourceAllocationInfo>()?),
            StructType::ResourceAllocationOption => Some(from.as_any_ref().downcast_ref::<ResourceAllocationOption>()?),
            StructType::ResourceConfigOption => Some(from.as_any_ref().downcast_ref::<ResourceConfigOption>()?),
            StructType::ResourceConfigSpec => Some(from.as_any_ref().downcast_ref::<ResourceConfigSpec>()?),
            StructType::DatabaseSizeEstimate => Some(from.as_any_ref().downcast_ref::<DatabaseSizeEstimate>()?),
            StructType::DatabaseSizeParam => Some(from.as_any_ref().downcast_ref::<DatabaseSizeParam>()?),
            StructType::InventoryDescription => Some(from.as_any_ref().downcast_ref::<InventoryDescription>()?),
            StructType::PerformanceStatisticsDescription => Some(from.as_any_ref().downcast_ref::<PerformanceStatisticsDescription>()?),
            StructType::ResourcePoolResourceUsage => Some(from.as_any_ref().downcast_ref::<ResourcePoolResourceUsage>()?),
            StructType::ResourcePoolRuntimeInfo => Some(from.as_any_ref().downcast_ref::<ResourcePoolRuntimeInfo>()?),
            StructType::ResourcePoolSummary => Some(from.as_any_ref().downcast_ref::<ResourcePoolSummary>()?),
            StructType::VirtualAppSummary => Some(from.as_any_ref().downcast_ref::<VirtualAppSummary>()?),
            StructType::ResourcePoolQuickStats => Some(from.as_any_ref().downcast_ref::<ResourcePoolQuickStats>()?),
            StructType::SddcBase => Some(from.as_any_ref().downcast_ref::<SddcBase>()?),
            StructType::SelectionSet => Some(from.as_any_ref().downcast_ref::<SelectionSet>()?),
            StructType::DvPortgroupSelection => Some(from.as_any_ref().downcast_ref::<DvPortgroupSelection>()?),
            StructType::DvsSelection => Some(from.as_any_ref().downcast_ref::<DvsSelection>()?),
            StructType::HostVMotionCompatibility => Some(from.as_any_ref().downcast_ref::<HostVMotionCompatibility>()?),
            StructType::ProductComponentInfo => Some(from.as_any_ref().downcast_ref::<ProductComponentInfo>()?),
            StructType::ServiceContent => Some(from.as_any_ref().downcast_ref::<ServiceContent>()?),
            StructType::ServiceLocator => Some(from.as_any_ref().downcast_ref::<ServiceLocator>()?),
            StructType::ServiceLocatorCredential => Some(from.as_any_ref().downcast_ref::<ServiceLocatorCredential>()?),
            StructType::ServiceLocatorNamePassword => Some(from.as_any_ref().downcast_ref::<ServiceLocatorNamePassword>()?),
            StructType::ServiceLocatorSamlCredential => Some(from.as_any_ref().downcast_ref::<ServiceLocatorSamlCredential>()?),
            StructType::ServiceManagerServiceInfo => Some(from.as_any_ref().downcast_ref::<ServiceManagerServiceInfo>()?),
            StructType::SessionManagerGenericServiceTicket => Some(from.as_any_ref().downcast_ref::<SessionManagerGenericServiceTicket>()?),
            StructType::SessionManagerLocalTicket => Some(from.as_any_ref().downcast_ref::<SessionManagerLocalTicket>()?),
            StructType::SessionManagerServiceRequestSpec => Some(from.as_any_ref().downcast_ref::<SessionManagerServiceRequestSpec>()?),
            StructType::SessionManagerHttpServiceRequestSpec => Some(from.as_any_ref().downcast_ref::<SessionManagerHttpServiceRequestSpec>()?),
            StructType::SessionManagerVmomiServiceRequestSpec => Some(from.as_any_ref().downcast_ref::<SessionManagerVmomiServiceRequestSpec>()?),
            StructType::SharesInfo => Some(from.as_any_ref().downcast_ref::<SharesInfo>()?),
            StructType::SharesOption => Some(from.as_any_ref().downcast_ref::<SharesOption>()?),
            StructType::SiteInfo => Some(from.as_any_ref().downcast_ref::<SiteInfo>()?),
            StructType::StoragePodSummary => Some(from.as_any_ref().downcast_ref::<StoragePodSummary>()?),
            StructType::StorageIoAllocationInfo => Some(from.as_any_ref().downcast_ref::<StorageIoAllocationInfo>()?),
            StructType::StorageIoAllocationOption => Some(from.as_any_ref().downcast_ref::<StorageIoAllocationOption>()?),
            StructType::StorageIormInfo => Some(from.as_any_ref().downcast_ref::<StorageIormInfo>()?),
            StructType::StorageIormConfigOption => Some(from.as_any_ref().downcast_ref::<StorageIormConfigOption>()?),
            StructType::StorageIormConfigSpec => Some(from.as_any_ref().downcast_ref::<StorageIormConfigSpec>()?),
            StructType::PodStorageDrsEntry => Some(from.as_any_ref().downcast_ref::<PodStorageDrsEntry>()?),
            StructType::StoragePerformanceSummary => Some(from.as_any_ref().downcast_ref::<StoragePerformanceSummary>()?),
            StructType::StorageResourceManagerStorageProfileStatistics => Some(from.as_any_ref().downcast_ref::<StorageResourceManagerStorageProfileStatistics>()?),
            StructType::Tag => Some(from.as_any_ref().downcast_ref::<Tag>()?),
            StructType::TaskDescription => Some(from.as_any_ref().downcast_ref::<TaskDescription>()?),
            StructType::TaskFilterSpec => Some(from.as_any_ref().downcast_ref::<TaskFilterSpec>()?),
            StructType::TaskFilterSpecByEntity => Some(from.as_any_ref().downcast_ref::<TaskFilterSpecByEntity>()?),
            StructType::TaskFilterSpecByTime => Some(from.as_any_ref().downcast_ref::<TaskFilterSpecByTime>()?),
            StructType::TaskFilterSpecByUsername => Some(from.as_any_ref().downcast_ref::<TaskFilterSpecByUsername>()?),
            StructType::TaskInfo => Some(from.as_any_ref().downcast_ref::<TaskInfo>()?),
            StructType::TaskReason => Some(from.as_any_ref().downcast_ref::<TaskReason>()?),
            StructType::TaskReasonAlarm => Some(from.as_any_ref().downcast_ref::<TaskReasonAlarm>()?),
            StructType::TaskReasonSchedule => Some(from.as_any_ref().downcast_ref::<TaskReasonSchedule>()?),
            StructType::TaskReasonSystem => Some(from.as_any_ref().downcast_ref::<TaskReasonSystem>()?),
            StructType::TaskReasonUser => Some(from.as_any_ref().downcast_ref::<TaskReasonUser>()?),
            StructType::UpdateVirtualMachineFilesResult => Some(from.as_any_ref().downcast_ref::<UpdateVirtualMachineFilesResult>()?),
            StructType::UpdateVirtualMachineFilesResultFailedVmFileInfo => Some(from.as_any_ref().downcast_ref::<UpdateVirtualMachineFilesResultFailedVmFileInfo>()?),
            StructType::UserSearchResult => Some(from.as_any_ref().downcast_ref::<UserSearchResult>()?),
            StructType::PosixUserSearchResult => Some(from.as_any_ref().downcast_ref::<PosixUserSearchResult>()?),
            StructType::UserSession => Some(from.as_any_ref().downcast_ref::<UserSession>()?),
            StructType::VVolVmConfigFileUpdateResult => Some(from.as_any_ref().downcast_ref::<VVolVmConfigFileUpdateResult>()?),
            StructType::VVolVmConfigFileUpdateResultFailedVmConfigFileInfo => Some(from.as_any_ref().downcast_ref::<VVolVmConfigFileUpdateResultFailedVmConfigFileInfo>()?),
            StructType::VasaStorageArray => Some(from.as_any_ref().downcast_ref::<VasaStorageArray>()?),
            StructType::VasaStorageArrayDiscoveryFcTransport => Some(from.as_any_ref().downcast_ref::<VasaStorageArrayDiscoveryFcTransport>()?),
            StructType::VasaStorageArrayDiscoveryIpTransport => Some(from.as_any_ref().downcast_ref::<VasaStorageArrayDiscoveryIpTransport>()?),
            StructType::VasaStorageArrayDiscoverySvcInfo => Some(from.as_any_ref().downcast_ref::<VasaStorageArrayDiscoverySvcInfo>()?),
            StructType::VasaProviderContainerSpec => Some(from.as_any_ref().downcast_ref::<VasaProviderContainerSpec>()?),
            StructType::VimVasaProvider => Some(from.as_any_ref().downcast_ref::<VimVasaProvider>()?),
            StructType::VimVasaProviderStatePerArray => Some(from.as_any_ref().downcast_ref::<VimVasaProviderStatePerArray>()?),
            StructType::VimVasaProviderVirtualHostConfig => Some(from.as_any_ref().downcast_ref::<VimVasaProviderVirtualHostConfig>()?),
            StructType::VimVasaProviderInfo => Some(from.as_any_ref().downcast_ref::<VimVasaProviderInfo>()?),
            StructType::VirtualAppLinkInfo => Some(from.as_any_ref().downcast_ref::<VirtualAppLinkInfo>()?),
            StructType::VirtualDiskSpec => Some(from.as_any_ref().downcast_ref::<VirtualDiskSpec>()?),
            StructType::DeviceBackedVirtualDiskSpec => Some(from.as_any_ref().downcast_ref::<DeviceBackedVirtualDiskSpec>()?),
            StructType::FileBackedVirtualDiskSpec => Some(from.as_any_ref().downcast_ref::<FileBackedVirtualDiskSpec>()?),
            StructType::SeSparseVirtualDiskSpec => Some(from.as_any_ref().downcast_ref::<SeSparseVirtualDiskSpec>()?),
            StructType::VirtualMachineConnection => Some(from.as_any_ref().downcast_ref::<VirtualMachineConnection>()?),
            StructType::VirtualMachineMksConnection => Some(from.as_any_ref().downcast_ref::<VirtualMachineMksConnection>()?),
            StructType::DiskChangeInfo => Some(from.as_any_ref().downcast_ref::<DiskChangeInfo>()?),
            StructType::DiskChangeExtent => Some(from.as_any_ref().downcast_ref::<DiskChangeExtent>()?),
            StructType::VirtualMachineDisplayTopology => Some(from.as_any_ref().downcast_ref::<VirtualMachineDisplayTopology>()?),
            StructType::VirtualMachineMksTicket => Some(from.as_any_ref().downcast_ref::<VirtualMachineMksTicket>()?),
            StructType::StorageRequirement => Some(from.as_any_ref().downcast_ref::<StorageRequirement>()?),
            StructType::VirtualMachineTicket => Some(from.as_any_ref().downcast_ref::<VirtualMachineTicket>()?),
            StructType::VirtualMachineWipeResult => Some(from.as_any_ref().downcast_ref::<VirtualMachineWipeResult>()?),
            StructType::VsanUpgradeSystemNetworkPartitionInfo => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemNetworkPartitionInfo>()?),
            StructType::VsanUpgradeSystemPreflightCheckIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemPreflightCheckIssue>()?),
            StructType::VsanUpgradeSystemApiBrokenIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemApiBrokenIssue>()?),
            StructType::VsanUpgradeSystemAutoClaimEnabledOnHostsIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemAutoClaimEnabledOnHostsIssue>()?),
            StructType::VsanUpgradeSystemHostsDisconnectedIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemHostsDisconnectedIssue>()?),
            StructType::VsanUpgradeSystemMissingHostsInClusterIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemMissingHostsInClusterIssue>()?),
            StructType::VsanUpgradeSystemNetworkPartitionIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemNetworkPartitionIssue>()?),
            StructType::VsanUpgradeSystemNotEnoughFreeCapacityIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemNotEnoughFreeCapacityIssue>()?),
            StructType::VsanUpgradeSystemRogueHostsInClusterIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemRogueHostsInClusterIssue>()?),
            StructType::VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue>()?),
            StructType::VsanUpgradeSystemWrongEsxVersionIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemWrongEsxVersionIssue>()?),
            StructType::VsanUpgradeSystemPreflightCheckResult => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemPreflightCheckResult>()?),
            StructType::VsanUpgradeSystemUpgradeHistoryItem => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemUpgradeHistoryItem>()?),
            StructType::VsanUpgradeSystemUpgradeHistoryDiskGroupOp => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemUpgradeHistoryDiskGroupOp>()?),
            StructType::VsanUpgradeSystemUpgradeHistoryPreflightFail => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemUpgradeHistoryPreflightFail>()?),
            StructType::VsanUpgradeSystemUpgradeStatus => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemUpgradeStatus>()?),
            StructType::Action => Some(from.as_any_ref().downcast_ref::<Action>()?),
            StructType::CreateTaskAction => Some(from.as_any_ref().downcast_ref::<CreateTaskAction>()?),
            StructType::MethodAction => Some(from.as_any_ref().downcast_ref::<MethodAction>()?),
            StructType::RunScriptAction => Some(from.as_any_ref().downcast_ref::<RunScriptAction>()?),
            StructType::SendEmailAction => Some(from.as_any_ref().downcast_ref::<SendEmailAction>()?),
            StructType::SendSnmpAction => Some(from.as_any_ref().downcast_ref::<SendSnmpAction>()?),
            StructType::MethodActionArgument => Some(from.as_any_ref().downcast_ref::<MethodActionArgument>()?),
            StructType::AlarmAction => Some(from.as_any_ref().downcast_ref::<AlarmAction>()?),
            StructType::AlarmTriggeringAction => Some(from.as_any_ref().downcast_ref::<AlarmTriggeringAction>()?),
            StructType::GroupAlarmAction => Some(from.as_any_ref().downcast_ref::<GroupAlarmAction>()?),
            StructType::AlarmDescription => Some(from.as_any_ref().downcast_ref::<AlarmDescription>()?),
            StructType::AlarmExpression => Some(from.as_any_ref().downcast_ref::<AlarmExpression>()?),
            StructType::AndAlarmExpression => Some(from.as_any_ref().downcast_ref::<AndAlarmExpression>()?),
            StructType::EventAlarmExpression => Some(from.as_any_ref().downcast_ref::<EventAlarmExpression>()?),
            StructType::MetricAlarmExpression => Some(from.as_any_ref().downcast_ref::<MetricAlarmExpression>()?),
            StructType::OrAlarmExpression => Some(from.as_any_ref().downcast_ref::<OrAlarmExpression>()?),
            StructType::StateAlarmExpression => Some(from.as_any_ref().downcast_ref::<StateAlarmExpression>()?),
            StructType::AlarmFilterSpec => Some(from.as_any_ref().downcast_ref::<AlarmFilterSpec>()?),
            StructType::AlarmSetting => Some(from.as_any_ref().downcast_ref::<AlarmSetting>()?),
            StructType::AlarmSpec => Some(from.as_any_ref().downcast_ref::<AlarmSpec>()?),
            StructType::AlarmInfo => Some(from.as_any_ref().downcast_ref::<AlarmInfo>()?),
            StructType::AlarmState => Some(from.as_any_ref().downcast_ref::<AlarmState>()?),
            StructType::AlarmTriggeringActionTransitionSpec => Some(from.as_any_ref().downcast_ref::<AlarmTriggeringActionTransitionSpec>()?),
            StructType::EventAlarmExpressionComparison => Some(from.as_any_ref().downcast_ref::<EventAlarmExpressionComparison>()?),
            StructType::ClusterAction => Some(from.as_any_ref().downcast_ref::<ClusterAction>()?),
            StructType::ClusterClusterInitialPlacementAction => Some(from.as_any_ref().downcast_ref::<ClusterClusterInitialPlacementAction>()?),
            StructType::ClusterHostInfraUpdateHaModeAction => Some(from.as_any_ref().downcast_ref::<ClusterHostInfraUpdateHaModeAction>()?),
            StructType::ClusterHostPowerAction => Some(from.as_any_ref().downcast_ref::<ClusterHostPowerAction>()?),
            StructType::ClusterInitialPlacementAction => Some(from.as_any_ref().downcast_ref::<ClusterInitialPlacementAction>()?),
            StructType::ClusterMigrationAction => Some(from.as_any_ref().downcast_ref::<ClusterMigrationAction>()?),
            StructType::PlacementAction => Some(from.as_any_ref().downcast_ref::<PlacementAction>()?),
            StructType::HbrDiskMigrationAction => Some(from.as_any_ref().downcast_ref::<HbrDiskMigrationAction>()?),
            StructType::StorageMigrationAction => Some(from.as_any_ref().downcast_ref::<StorageMigrationAction>()?),
            StructType::StoragePlacementAction => Some(from.as_any_ref().downcast_ref::<StoragePlacementAction>()?),
            StructType::ClusterActionHistory => Some(from.as_any_ref().downcast_ref::<ClusterActionHistory>()?),
            StructType::ClusterAttemptedVmInfo => Some(from.as_any_ref().downcast_ref::<ClusterAttemptedVmInfo>()?),
            StructType::ClusterConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterConfigInfo>()?),
            StructType::ClusterConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterConfigSpec>()?),
            StructType::ClusterCryptoConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterCryptoConfigInfo>()?),
            StructType::ClusterDasAamNodeState => Some(from.as_any_ref().downcast_ref::<ClusterDasAamNodeState>()?),
            StructType::ClusterDasAdmissionControlInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasAdmissionControlInfo>()?),
            StructType::ClusterFailoverHostAdmissionControlInfo => Some(from.as_any_ref().downcast_ref::<ClusterFailoverHostAdmissionControlInfo>()?),
            StructType::ClusterFailoverLevelAdmissionControlInfo => Some(from.as_any_ref().downcast_ref::<ClusterFailoverLevelAdmissionControlInfo>()?),
            StructType::ClusterFailoverResourcesAdmissionControlInfo => Some(from.as_any_ref().downcast_ref::<ClusterFailoverResourcesAdmissionControlInfo>()?),
            StructType::ClusterDasAdmissionControlPolicy => Some(from.as_any_ref().downcast_ref::<ClusterDasAdmissionControlPolicy>()?),
            StructType::ClusterFailoverHostAdmissionControlPolicy => Some(from.as_any_ref().downcast_ref::<ClusterFailoverHostAdmissionControlPolicy>()?),
            StructType::ClusterFailoverLevelAdmissionControlPolicy => Some(from.as_any_ref().downcast_ref::<ClusterFailoverLevelAdmissionControlPolicy>()?),
            StructType::ClusterFailoverResourcesAdmissionControlPolicy => Some(from.as_any_ref().downcast_ref::<ClusterFailoverResourcesAdmissionControlPolicy>()?),
            StructType::ClusterDasAdvancedRuntimeInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasAdvancedRuntimeInfo>()?),
            StructType::ClusterDasFailoverLevelAdvancedRuntimeInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasFailoverLevelAdvancedRuntimeInfo>()?),
            StructType::DasHeartbeatDatastoreInfo => Some(from.as_any_ref().downcast_ref::<DasHeartbeatDatastoreInfo>()?),
            StructType::ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo>()?),
            StructType::ClusterDasConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasConfigInfo>()?),
            StructType::ClusterDasData => Some(from.as_any_ref().downcast_ref::<ClusterDasData>()?),
            StructType::ClusterDasDataSummary => Some(from.as_any_ref().downcast_ref::<ClusterDasDataSummary>()?),
            StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots => Some(from.as_any_ref().downcast_ref::<ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots>()?),
            StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo>()?),
            StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots => Some(from.as_any_ref().downcast_ref::<ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots>()?),
            StructType::ClusterDasFdmHostState => Some(from.as_any_ref().downcast_ref::<ClusterDasFdmHostState>()?),
            StructType::ClusterDasHostInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasHostInfo>()?),
            StructType::ClusterDasAamHostInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasAamHostInfo>()?),
            StructType::ClusterDasHostRecommendation => Some(from.as_any_ref().downcast_ref::<ClusterDasHostRecommendation>()?),
            StructType::ClusterDasVmConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasVmConfigInfo>()?),
            StructType::ClusterDasVmSettings => Some(from.as_any_ref().downcast_ref::<ClusterDasVmSettings>()?),
            StructType::ClusterDpmConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterDpmConfigInfo>()?),
            StructType::ClusterDpmHostConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterDpmHostConfigInfo>()?),
            StructType::ClusterDrsConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterDrsConfigInfo>()?),
            StructType::ClusterDrsFaults => Some(from.as_any_ref().downcast_ref::<ClusterDrsFaults>()?),
            StructType::ClusterDrsFaultsFaultsByVm => Some(from.as_any_ref().downcast_ref::<ClusterDrsFaultsFaultsByVm>()?),
            StructType::ClusterDrsFaultsFaultsByVirtualDisk => Some(from.as_any_ref().downcast_ref::<ClusterDrsFaultsFaultsByVirtualDisk>()?),
            StructType::ClusterDrsMigration => Some(from.as_any_ref().downcast_ref::<ClusterDrsMigration>()?),
            StructType::ClusterDrsRecommendation => Some(from.as_any_ref().downcast_ref::<ClusterDrsRecommendation>()?),
            StructType::ClusterDrsVmConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterDrsVmConfigInfo>()?),
            StructType::ClusterEvcManagerCheckResult => Some(from.as_any_ref().downcast_ref::<ClusterEvcManagerCheckResult>()?),
            StructType::ClusterEvcManagerEvcState => Some(from.as_any_ref().downcast_ref::<ClusterEvcManagerEvcState>()?),
            StructType::ClusterEnterMaintenanceResult => Some(from.as_any_ref().downcast_ref::<ClusterEnterMaintenanceResult>()?),
            StructType::ClusterFailoverHostAdmissionControlInfoHostStatus => Some(from.as_any_ref().downcast_ref::<ClusterFailoverHostAdmissionControlInfoHostStatus>()?),
            StructType::ClusterGroupInfo => Some(from.as_any_ref().downcast_ref::<ClusterGroupInfo>()?),
            StructType::ClusterHostGroup => Some(from.as_any_ref().downcast_ref::<ClusterHostGroup>()?),
            StructType::ClusterVmGroup => Some(from.as_any_ref().downcast_ref::<ClusterVmGroup>()?),
            StructType::ClusterHostRecommendation => Some(from.as_any_ref().downcast_ref::<ClusterHostRecommendation>()?),
            StructType::ClusterInfraUpdateHaConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterInfraUpdateHaConfigInfo>()?),
            StructType::ClusterNotAttemptedVmInfo => Some(from.as_any_ref().downcast_ref::<ClusterNotAttemptedVmInfo>()?),
            StructType::ClusterOrchestrationInfo => Some(from.as_any_ref().downcast_ref::<ClusterOrchestrationInfo>()?),
            StructType::PlacementResult => Some(from.as_any_ref().downcast_ref::<PlacementResult>()?),
            StructType::PlacementSpec => Some(from.as_any_ref().downcast_ref::<PlacementSpec>()?),
            StructType::ClusterPowerOnVmResult => Some(from.as_any_ref().downcast_ref::<ClusterPowerOnVmResult>()?),
            StructType::ClusterPreemptibleVmPairInfo => Some(from.as_any_ref().downcast_ref::<ClusterPreemptibleVmPairInfo>()?),
            StructType::ClusterProactiveDrsConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterProactiveDrsConfigInfo>()?),
            StructType::ClusterRecommendation => Some(from.as_any_ref().downcast_ref::<ClusterRecommendation>()?),
            StructType::ClusterResourceUsageSummary => Some(from.as_any_ref().downcast_ref::<ClusterResourceUsageSummary>()?),
            StructType::ClusterRuleInfo => Some(from.as_any_ref().downcast_ref::<ClusterRuleInfo>()?),
            StructType::ClusterAffinityRuleSpec => Some(from.as_any_ref().downcast_ref::<ClusterAffinityRuleSpec>()?),
            StructType::ClusterAntiAffinityRuleSpec => Some(from.as_any_ref().downcast_ref::<ClusterAntiAffinityRuleSpec>()?),
            StructType::ClusterDependencyRuleInfo => Some(from.as_any_ref().downcast_ref::<ClusterDependencyRuleInfo>()?),
            StructType::ClusterVmHostRuleInfo => Some(from.as_any_ref().downcast_ref::<ClusterVmHostRuleInfo>()?),
            StructType::VirtualDiskAntiAffinityRuleSpec => Some(from.as_any_ref().downcast_ref::<VirtualDiskAntiAffinityRuleSpec>()?),
            StructType::VirtualDiskRuleSpec => Some(from.as_any_ref().downcast_ref::<VirtualDiskRuleSpec>()?),
            StructType::ClusterSlotPolicy => Some(from.as_any_ref().downcast_ref::<ClusterSlotPolicy>()?),
            StructType::ClusterFixedSizeSlotPolicy => Some(from.as_any_ref().downcast_ref::<ClusterFixedSizeSlotPolicy>()?),
            StructType::ClusterSystemVMsConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterSystemVMsConfigInfo>()?),
            StructType::ClusterSystemVMsConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterSystemVMsConfigSpec>()?),
            StructType::ClusterUsageSummary => Some(from.as_any_ref().downcast_ref::<ClusterUsageSummary>()?),
            StructType::ClusterVmComponentProtectionSettings => Some(from.as_any_ref().downcast_ref::<ClusterVmComponentProtectionSettings>()?),
            StructType::ClusterVmOrchestrationInfo => Some(from.as_any_ref().downcast_ref::<ClusterVmOrchestrationInfo>()?),
            StructType::ClusterVmReadiness => Some(from.as_any_ref().downcast_ref::<ClusterVmReadiness>()?),
            StructType::ClusterVmToolsMonitoringSettings => Some(from.as_any_ref().downcast_ref::<ClusterVmToolsMonitoringSettings>()?),
            StructType::DistributedVirtualPort => Some(from.as_any_ref().downcast_ref::<DistributedVirtualPort>()?),
            StructType::DvPortConfigInfo => Some(from.as_any_ref().downcast_ref::<DvPortConfigInfo>()?),
            StructType::DvPortConfigSpec => Some(from.as_any_ref().downcast_ref::<DvPortConfigSpec>()?),
            StructType::DvsFilterParameter => Some(from.as_any_ref().downcast_ref::<DvsFilterParameter>()?),
            StructType::DvsHostLocalPortInfo => Some(from.as_any_ref().downcast_ref::<DvsHostLocalPortInfo>()?),
            StructType::DvPortStatus => Some(from.as_any_ref().downcast_ref::<DvPortStatus>()?),
            StructType::DvPortSetting => Some(from.as_any_ref().downcast_ref::<DvPortSetting>()?),
            StructType::VMwareDvsPortSetting => Some(from.as_any_ref().downcast_ref::<VMwareDvsPortSetting>()?),
            StructType::DvPortState => Some(from.as_any_ref().downcast_ref::<DvPortState>()?),
            StructType::DvPortgroupConfigInfo => Some(from.as_any_ref().downcast_ref::<DvPortgroupConfigInfo>()?),
            StructType::DvPortgroupConfigSpec => Some(from.as_any_ref().downcast_ref::<DvPortgroupConfigSpec>()?),
            StructType::DistributedVirtualPortgroupNsxPortgroupOperationResult => Some(from.as_any_ref().downcast_ref::<DistributedVirtualPortgroupNsxPortgroupOperationResult>()?),
            StructType::DvPortgroupPolicy => Some(from.as_any_ref().downcast_ref::<DvPortgroupPolicy>()?),
            StructType::VMwareDvsPortgroupPolicy => Some(from.as_any_ref().downcast_ref::<VMwareDvsPortgroupPolicy>()?),
            StructType::DistributedVirtualPortgroupProblem => Some(from.as_any_ref().downcast_ref::<DistributedVirtualPortgroupProblem>()?),
            StructType::DistributedVirtualPortgroupInfo => Some(from.as_any_ref().downcast_ref::<DistributedVirtualPortgroupInfo>()?),
            StructType::DistributedVirtualSwitchInfo => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchInfo>()?),
            StructType::DistributedVirtualSwitchManagerCompatibilityResult => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerCompatibilityResult>()?),
            StructType::DvsManagerDvsConfigTarget => Some(from.as_any_ref().downcast_ref::<DvsManagerDvsConfigTarget>()?),
            StructType::DistributedVirtualSwitchManagerDvsProductSpec => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerDvsProductSpec>()?),
            StructType::DistributedVirtualSwitchManagerHostContainer => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostContainer>()?),
            StructType::DistributedVirtualSwitchManagerHostDvsFilterSpec => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostDvsFilterSpec>()?),
            StructType::DistributedVirtualSwitchManagerHostArrayFilter => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostArrayFilter>()?),
            StructType::DistributedVirtualSwitchManagerHostContainerFilter => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostContainerFilter>()?),
            StructType::DistributedVirtualSwitchManagerHostDvsMembershipFilter => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostDvsMembershipFilter>()?),
            StructType::DistributedVirtualSwitchManagerImportResult => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerImportResult>()?),
            StructType::DvsManagerPhysicalNicsList => Some(from.as_any_ref().downcast_ref::<DvsManagerPhysicalNicsList>()?),
            StructType::EntityBackup => Some(from.as_any_ref().downcast_ref::<EntityBackup>()?),
            StructType::EntityBackupConfig => Some(from.as_any_ref().downcast_ref::<EntityBackupConfig>()?),
            StructType::DistributedVirtualSwitchHostMember => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMember>()?),
            StructType::DistributedVirtualSwitchHostMemberBacking => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberBacking>()?),
            StructType::DistributedVirtualSwitchHostMemberPnicBacking => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberPnicBacking>()?),
            StructType::DistributedVirtualSwitchHostMemberConfigInfo => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberConfigInfo>()?),
            StructType::DistributedVirtualSwitchHostMemberConfigSpec => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberConfigSpec>()?),
            StructType::HostMemberHealthCheckResult => Some(from.as_any_ref().downcast_ref::<HostMemberHealthCheckResult>()?),
            StructType::HostMemberUplinkHealthCheckResult => Some(from.as_any_ref().downcast_ref::<HostMemberUplinkHealthCheckResult>()?),
            StructType::VMwareDvsMtuHealthCheckResult => Some(from.as_any_ref().downcast_ref::<VMwareDvsMtuHealthCheckResult>()?),
            StructType::VMwareDvsVlanHealthCheckResult => Some(from.as_any_ref().downcast_ref::<VMwareDvsVlanHealthCheckResult>()?),
            StructType::VMwareDvsTeamingHealthCheckResult => Some(from.as_any_ref().downcast_ref::<VMwareDvsTeamingHealthCheckResult>()?),
            StructType::DistributedVirtualSwitchHostMemberPnicSpec => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberPnicSpec>()?),
            StructType::HostMemberRuntimeInfo => Some(from.as_any_ref().downcast_ref::<HostMemberRuntimeInfo>()?),
            StructType::DistributedVirtualSwitchHostMemberRuntimeState => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberRuntimeState>()?),
            StructType::DistributedVirtualSwitchHostMemberTransportZoneInfo => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberTransportZoneInfo>()?),
            StructType::DistributedVirtualSwitchHostProductSpec => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostProductSpec>()?),
            StructType::DistributedVirtualSwitchKeyedOpaqueBlob => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchKeyedOpaqueBlob>()?),
            StructType::DistributedVirtualSwitchNetworkOffloadSpec => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchNetworkOffloadSpec>()?),
            StructType::DvsNetworkResourcePool => Some(from.as_any_ref().downcast_ref::<DvsNetworkResourcePool>()?),
            StructType::DvsNetworkResourcePoolAllocationInfo => Some(from.as_any_ref().downcast_ref::<DvsNetworkResourcePoolAllocationInfo>()?),
            StructType::DvsNetworkResourcePoolConfigSpec => Some(from.as_any_ref().downcast_ref::<DvsNetworkResourcePoolConfigSpec>()?),
            StructType::DistributedVirtualSwitchPortConnectee => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchPortConnectee>()?),
            StructType::DistributedVirtualSwitchPortConnection => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchPortConnection>()?),
            StructType::DistributedVirtualSwitchPortCriteria => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchPortCriteria>()?),
            StructType::DistributedVirtualSwitchPortStatistics => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchPortStatistics>()?),
            StructType::DistributedVirtualSwitchProductSpec => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchProductSpec>()?),
            StructType::DvsTrafficRule => Some(from.as_any_ref().downcast_ref::<DvsTrafficRule>()?),
            StructType::DvsNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsNetworkRuleAction>()?),
            StructType::DvsAcceptNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsAcceptNetworkRuleAction>()?),
            StructType::DvsCopyNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsCopyNetworkRuleAction>()?),
            StructType::DvsDropNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsDropNetworkRuleAction>()?),
            StructType::DvsGreEncapNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsGreEncapNetworkRuleAction>()?),
            StructType::DvsLogNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsLogNetworkRuleAction>()?),
            StructType::DvsMacRewriteNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsMacRewriteNetworkRuleAction>()?),
            StructType::DvsPuntNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsPuntNetworkRuleAction>()?),
            StructType::DvsRateLimitNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsRateLimitNetworkRuleAction>()?),
            StructType::DvsUpdateTagNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsUpdateTagNetworkRuleAction>()?),
            StructType::DvsNetworkRuleQualifier => Some(from.as_any_ref().downcast_ref::<DvsNetworkRuleQualifier>()?),
            StructType::DvsIpNetworkRuleQualifier => Some(from.as_any_ref().downcast_ref::<DvsIpNetworkRuleQualifier>()?),
            StructType::DvsMacNetworkRuleQualifier => Some(from.as_any_ref().downcast_ref::<DvsMacNetworkRuleQualifier>()?),
            StructType::DvsSystemTrafficNetworkRuleQualifier => Some(from.as_any_ref().downcast_ref::<DvsSystemTrafficNetworkRuleQualifier>()?),
            StructType::DvsTrafficRuleset => Some(from.as_any_ref().downcast_ref::<DvsTrafficRuleset>()?),
            StructType::DvsVmVnicNetworkResourcePool => Some(from.as_any_ref().downcast_ref::<DvsVmVnicNetworkResourcePool>()?),
            StructType::DvsVmVnicResourcePoolConfigSpec => Some(from.as_any_ref().downcast_ref::<DvsVmVnicResourcePoolConfigSpec>()?),
            StructType::DvsVmVnicResourceAllocation => Some(from.as_any_ref().downcast_ref::<DvsVmVnicResourceAllocation>()?),
            StructType::DvsVmVnicNetworkResourcePoolRuntimeInfo => Some(from.as_any_ref().downcast_ref::<DvsVmVnicNetworkResourcePoolRuntimeInfo>()?),
            StructType::DvsVnicAllocatedResource => Some(from.as_any_ref().downcast_ref::<DvsVnicAllocatedResource>()?),
            StructType::VMwareDvsDpuCapability => Some(from.as_any_ref().downcast_ref::<VMwareDvsDpuCapability>()?),
            StructType::VMwareIpfixConfig => Some(from.as_any_ref().downcast_ref::<VMwareIpfixConfig>()?),
            StructType::VMwareDvsIpfixCapability => Some(from.as_any_ref().downcast_ref::<VMwareDvsIpfixCapability>()?),
            StructType::VMwareDvsLacpCapability => Some(from.as_any_ref().downcast_ref::<VMwareDvsLacpCapability>()?),
            StructType::VMwareDvsLacpGroupConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsLacpGroupConfig>()?),
            StructType::VMwareDvsLacpGroupSpec => Some(from.as_any_ref().downcast_ref::<VMwareDvsLacpGroupSpec>()?),
            StructType::VMwareDvsLagIpfixConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsLagIpfixConfig>()?),
            StructType::VMwareDvsLagVlanConfig => Some(from.as_any_ref().downcast_ref::<VMwareDvsLagVlanConfig>()?),
            StructType::VMwareDvsMtuCapability => Some(from.as_any_ref().downcast_ref::<VMwareDvsMtuCapability>()?),
            StructType::VMwareDvsPvlanConfigSpec => Some(from.as_any_ref().downcast_ref::<VMwareDvsPvlanConfigSpec>()?),
            StructType::VMwareDvsPvlanMapEntry => Some(from.as_any_ref().downcast_ref::<VMwareDvsPvlanMapEntry>()?),
            StructType::VMwareDvsVspanConfigSpec => Some(from.as_any_ref().downcast_ref::<VMwareDvsVspanConfigSpec>()?),
            StructType::VMwareDvsVspanCapability => Some(from.as_any_ref().downcast_ref::<VMwareDvsVspanCapability>()?),
            StructType::VMwareVspanPort => Some(from.as_any_ref().downcast_ref::<VMwareVspanPort>()?),
            StructType::VMwareVspanSession => Some(from.as_any_ref().downcast_ref::<VMwareVspanSession>()?),
            StructType::CryptoKeyId => Some(from.as_any_ref().downcast_ref::<CryptoKeyId>()?),
            StructType::CryptoKeyPlain => Some(from.as_any_ref().downcast_ref::<CryptoKeyPlain>()?),
            StructType::CryptoKeyResult => Some(from.as_any_ref().downcast_ref::<CryptoKeyResult>()?),
            StructType::CryptoManagerHostKeyStatus => Some(from.as_any_ref().downcast_ref::<CryptoManagerHostKeyStatus>()?),
            StructType::CryptoManagerKmipCertSignRequest => Some(from.as_any_ref().downcast_ref::<CryptoManagerKmipCertSignRequest>()?),
            StructType::CryptoManagerKmipCertificateInfo => Some(from.as_any_ref().downcast_ref::<CryptoManagerKmipCertificateInfo>()?),
            StructType::CryptoManagerKmipClusterStatus => Some(from.as_any_ref().downcast_ref::<CryptoManagerKmipClusterStatus>()?),
            StructType::CryptoManagerKmipCryptoKeyStatus => Some(from.as_any_ref().downcast_ref::<CryptoManagerKmipCryptoKeyStatus>()?),
            StructType::CryptoManagerKmipCustomAttributeSpec => Some(from.as_any_ref().downcast_ref::<CryptoManagerKmipCustomAttributeSpec>()?),
            StructType::CryptoManagerKmipServerCertInfo => Some(from.as_any_ref().downcast_ref::<CryptoManagerKmipServerCertInfo>()?),
            StructType::CryptoManagerKmipServerStatus => Some(from.as_any_ref().downcast_ref::<CryptoManagerKmipServerStatus>()?),
            StructType::CryptoSpec => Some(from.as_any_ref().downcast_ref::<CryptoSpec>()?),
            StructType::CryptoSpecDecrypt => Some(from.as_any_ref().downcast_ref::<CryptoSpecDecrypt>()?),
            StructType::CryptoSpecDeepRecrypt => Some(from.as_any_ref().downcast_ref::<CryptoSpecDeepRecrypt>()?),
            StructType::CryptoSpecEncrypt => Some(from.as_any_ref().downcast_ref::<CryptoSpecEncrypt>()?),
            StructType::CryptoSpecNoOp => Some(from.as_any_ref().downcast_ref::<CryptoSpecNoOp>()?),
            StructType::CryptoSpecRegister => Some(from.as_any_ref().downcast_ref::<CryptoSpecRegister>()?),
            StructType::CryptoSpecShallowRecrypt => Some(from.as_any_ref().downcast_ref::<CryptoSpecShallowRecrypt>()?),
            StructType::KeyProviderId => Some(from.as_any_ref().downcast_ref::<KeyProviderId>()?),
            StructType::KmipClusterInfo => Some(from.as_any_ref().downcast_ref::<KmipClusterInfo>()?),
            StructType::KmipServerInfo => Some(from.as_any_ref().downcast_ref::<KmipServerInfo>()?),
            StructType::KmipServerSpec => Some(from.as_any_ref().downcast_ref::<KmipServerSpec>()?),
            StructType::KmipServerStatus => Some(from.as_any_ref().downcast_ref::<KmipServerStatus>()?),
            StructType::ChangesInfoEventArgument => Some(from.as_any_ref().downcast_ref::<ChangesInfoEventArgument>()?),
            StructType::DvsOutOfSyncHostArgument => Some(from.as_any_ref().downcast_ref::<DvsOutOfSyncHostArgument>()?),
            StructType::Event => Some(from.as_any_ref().downcast_ref::<Event>()?),
            StructType::AlarmEvent => Some(from.as_any_ref().downcast_ref::<AlarmEvent>()?),
            StructType::AlarmAcknowledgedEvent => Some(from.as_any_ref().downcast_ref::<AlarmAcknowledgedEvent>()?),
            StructType::AlarmActionTriggeredEvent => Some(from.as_any_ref().downcast_ref::<AlarmActionTriggeredEvent>()?),
            StructType::AlarmClearedEvent => Some(from.as_any_ref().downcast_ref::<AlarmClearedEvent>()?),
            StructType::AlarmCreatedEvent => Some(from.as_any_ref().downcast_ref::<AlarmCreatedEvent>()?),
            StructType::AlarmEmailCompletedEvent => Some(from.as_any_ref().downcast_ref::<AlarmEmailCompletedEvent>()?),
            StructType::AlarmEmailFailedEvent => Some(from.as_any_ref().downcast_ref::<AlarmEmailFailedEvent>()?),
            StructType::AlarmReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<AlarmReconfiguredEvent>()?),
            StructType::AlarmRemovedEvent => Some(from.as_any_ref().downcast_ref::<AlarmRemovedEvent>()?),
            StructType::AlarmScriptCompleteEvent => Some(from.as_any_ref().downcast_ref::<AlarmScriptCompleteEvent>()?),
            StructType::AlarmScriptFailedEvent => Some(from.as_any_ref().downcast_ref::<AlarmScriptFailedEvent>()?),
            StructType::AlarmSnmpCompletedEvent => Some(from.as_any_ref().downcast_ref::<AlarmSnmpCompletedEvent>()?),
            StructType::AlarmSnmpFailedEvent => Some(from.as_any_ref().downcast_ref::<AlarmSnmpFailedEvent>()?),
            StructType::AlarmStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<AlarmStatusChangedEvent>()?),
            StructType::AuthorizationEvent => Some(from.as_any_ref().downcast_ref::<AuthorizationEvent>()?),
            StructType::PermissionEvent => Some(from.as_any_ref().downcast_ref::<PermissionEvent>()?),
            StructType::PermissionAddedEvent => Some(from.as_any_ref().downcast_ref::<PermissionAddedEvent>()?),
            StructType::PermissionRemovedEvent => Some(from.as_any_ref().downcast_ref::<PermissionRemovedEvent>()?),
            StructType::PermissionUpdatedEvent => Some(from.as_any_ref().downcast_ref::<PermissionUpdatedEvent>()?),
            StructType::RoleEvent => Some(from.as_any_ref().downcast_ref::<RoleEvent>()?),
            StructType::RoleAddedEvent => Some(from.as_any_ref().downcast_ref::<RoleAddedEvent>()?),
            StructType::RoleRemovedEvent => Some(from.as_any_ref().downcast_ref::<RoleRemovedEvent>()?),
            StructType::RoleUpdatedEvent => Some(from.as_any_ref().downcast_ref::<RoleUpdatedEvent>()?),
            StructType::ClusterEvent => Some(from.as_any_ref().downcast_ref::<ClusterEvent>()?),
            StructType::ClusterComplianceCheckedEvent => Some(from.as_any_ref().downcast_ref::<ClusterComplianceCheckedEvent>()?),
            StructType::ClusterCreatedEvent => Some(from.as_any_ref().downcast_ref::<ClusterCreatedEvent>()?),
            StructType::ClusterDestroyedEvent => Some(from.as_any_ref().downcast_ref::<ClusterDestroyedEvent>()?),
            StructType::ClusterOvercommittedEvent => Some(from.as_any_ref().downcast_ref::<ClusterOvercommittedEvent>()?),
            StructType::HostOvercommittedEvent => Some(from.as_any_ref().downcast_ref::<HostOvercommittedEvent>()?),
            StructType::ClusterReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<ClusterReconfiguredEvent>()?),
            StructType::ClusterStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<ClusterStatusChangedEvent>()?),
            StructType::HostStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<HostStatusChangedEvent>()?),
            StructType::DasAdmissionControlDisabledEvent => Some(from.as_any_ref().downcast_ref::<DasAdmissionControlDisabledEvent>()?),
            StructType::DasAdmissionControlEnabledEvent => Some(from.as_any_ref().downcast_ref::<DasAdmissionControlEnabledEvent>()?),
            StructType::DasAgentFoundEvent => Some(from.as_any_ref().downcast_ref::<DasAgentFoundEvent>()?),
            StructType::DasAgentUnavailableEvent => Some(from.as_any_ref().downcast_ref::<DasAgentUnavailableEvent>()?),
            StructType::DasClusterIsolatedEvent => Some(from.as_any_ref().downcast_ref::<DasClusterIsolatedEvent>()?),
            StructType::DasDisabledEvent => Some(from.as_any_ref().downcast_ref::<DasDisabledEvent>()?),
            StructType::DasEnabledEvent => Some(from.as_any_ref().downcast_ref::<DasEnabledEvent>()?),
            StructType::DasHostFailedEvent => Some(from.as_any_ref().downcast_ref::<DasHostFailedEvent>()?),
            StructType::DasHostIsolatedEvent => Some(from.as_any_ref().downcast_ref::<DasHostIsolatedEvent>()?),
            StructType::DrsDisabledEvent => Some(from.as_any_ref().downcast_ref::<DrsDisabledEvent>()?),
            StructType::DrsEnabledEvent => Some(from.as_any_ref().downcast_ref::<DrsEnabledEvent>()?),
            StructType::DrsInvocationFailedEvent => Some(from.as_any_ref().downcast_ref::<DrsInvocationFailedEvent>()?),
            StructType::DrsRecoveredFromFailureEvent => Some(from.as_any_ref().downcast_ref::<DrsRecoveredFromFailureEvent>()?),
            StructType::FailoverLevelRestored => Some(from.as_any_ref().downcast_ref::<FailoverLevelRestored>()?),
            StructType::HostMonitoringStateChangedEvent => Some(from.as_any_ref().downcast_ref::<HostMonitoringStateChangedEvent>()?),
            StructType::InsufficientFailoverResourcesEvent => Some(from.as_any_ref().downcast_ref::<InsufficientFailoverResourcesEvent>()?),
            StructType::VmHealthMonitoringStateChangedEvent => Some(from.as_any_ref().downcast_ref::<VmHealthMonitoringStateChangedEvent>()?),
            StructType::CustomFieldEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldEvent>()?),
            StructType::CustomFieldDefEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefEvent>()?),
            StructType::CustomFieldDefAddedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefAddedEvent>()?),
            StructType::CustomFieldDefRemovedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefRemovedEvent>()?),
            StructType::CustomFieldDefRenamedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefRenamedEvent>()?),
            StructType::CustomFieldValueChangedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldValueChangedEvent>()?),
            StructType::DvPortgroupEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupEvent>()?),
            StructType::DvPortgroupCreatedEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupCreatedEvent>()?),
            StructType::DvPortgroupDestroyedEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupDestroyedEvent>()?),
            StructType::DvPortgroupReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupReconfiguredEvent>()?),
            StructType::DvPortgroupRenamedEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupRenamedEvent>()?),
            StructType::DvpgImportEvent => Some(from.as_any_ref().downcast_ref::<DvpgImportEvent>()?),
            StructType::DvpgRestoreEvent => Some(from.as_any_ref().downcast_ref::<DvpgRestoreEvent>()?),
            StructType::DatacenterEvent => Some(from.as_any_ref().downcast_ref::<DatacenterEvent>()?),
            StructType::DatacenterCreatedEvent => Some(from.as_any_ref().downcast_ref::<DatacenterCreatedEvent>()?),
            StructType::DatacenterRenamedEvent => Some(from.as_any_ref().downcast_ref::<DatacenterRenamedEvent>()?),
            StructType::DatastoreEvent => Some(from.as_any_ref().downcast_ref::<DatastoreEvent>()?),
            StructType::DatastoreCapacityIncreasedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreCapacityIncreasedEvent>()?),
            StructType::DatastoreDestroyedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreDestroyedEvent>()?),
            StructType::DatastoreDuplicatedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreDuplicatedEvent>()?),
            StructType::DatastoreFileEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileEvent>()?),
            StructType::DatastoreFileCopiedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileCopiedEvent>()?),
            StructType::DatastoreFileDeletedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileDeletedEvent>()?),
            StructType::DatastoreFileMovedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileMovedEvent>()?),
            StructType::DatastoreIormReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DatastoreIormReconfiguredEvent>()?),
            StructType::DatastoreRenamedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreRenamedEvent>()?),
            StructType::NonViWorkloadDetectedOnDatastoreEvent => Some(from.as_any_ref().downcast_ref::<NonViWorkloadDetectedOnDatastoreEvent>()?),
            StructType::DvsEvent => Some(from.as_any_ref().downcast_ref::<DvsEvent>()?),
            StructType::DvsCreatedEvent => Some(from.as_any_ref().downcast_ref::<DvsCreatedEvent>()?),
            StructType::DvsDestroyedEvent => Some(from.as_any_ref().downcast_ref::<DvsDestroyedEvent>()?),
            StructType::DvsHostBackInSyncEvent => Some(from.as_any_ref().downcast_ref::<DvsHostBackInSyncEvent>()?),
            StructType::DvsHostJoinedEvent => Some(from.as_any_ref().downcast_ref::<DvsHostJoinedEvent>()?),
            StructType::DvsHostLeftEvent => Some(from.as_any_ref().downcast_ref::<DvsHostLeftEvent>()?),
            StructType::DvsHostStatusUpdated => Some(from.as_any_ref().downcast_ref::<DvsHostStatusUpdated>()?),
            StructType::DvsHostWentOutOfSyncEvent => Some(from.as_any_ref().downcast_ref::<DvsHostWentOutOfSyncEvent>()?),
            StructType::DvsImportEvent => Some(from.as_any_ref().downcast_ref::<DvsImportEvent>()?),
            StructType::DvsMergedEvent => Some(from.as_any_ref().downcast_ref::<DvsMergedEvent>()?),
            StructType::DvsPortBlockedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortBlockedEvent>()?),
            StructType::DvsPortConnectedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortConnectedEvent>()?),
            StructType::DvsPortCreatedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortCreatedEvent>()?),
            StructType::DvsPortDeletedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortDeletedEvent>()?),
            StructType::DvsPortDisconnectedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortDisconnectedEvent>()?),
            StructType::DvsPortEnteredPassthruEvent => Some(from.as_any_ref().downcast_ref::<DvsPortEnteredPassthruEvent>()?),
            StructType::DvsPortExitedPassthruEvent => Some(from.as_any_ref().downcast_ref::<DvsPortExitedPassthruEvent>()?),
            StructType::DvsPortJoinPortgroupEvent => Some(from.as_any_ref().downcast_ref::<DvsPortJoinPortgroupEvent>()?),
            StructType::DvsPortLeavePortgroupEvent => Some(from.as_any_ref().downcast_ref::<DvsPortLeavePortgroupEvent>()?),
            StructType::DvsPortLinkDownEvent => Some(from.as_any_ref().downcast_ref::<DvsPortLinkDownEvent>()?),
            StructType::DvsPortLinkUpEvent => Some(from.as_any_ref().downcast_ref::<DvsPortLinkUpEvent>()?),
            StructType::DvsPortReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DvsPortReconfiguredEvent>()?),
            StructType::DvsPortRuntimeChangeEvent => Some(from.as_any_ref().downcast_ref::<DvsPortRuntimeChangeEvent>()?),
            StructType::DvsPortUnblockedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortUnblockedEvent>()?),
            StructType::DvsPortVendorSpecificStateChangeEvent => Some(from.as_any_ref().downcast_ref::<DvsPortVendorSpecificStateChangeEvent>()?),
            StructType::DvsReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DvsReconfiguredEvent>()?),
            StructType::DvsRenamedEvent => Some(from.as_any_ref().downcast_ref::<DvsRenamedEvent>()?),
            StructType::DvsRestoreEvent => Some(from.as_any_ref().downcast_ref::<DvsRestoreEvent>()?),
            StructType::DvsUpgradeAvailableEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradeAvailableEvent>()?),
            StructType::DvsUpgradeInProgressEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradeInProgressEvent>()?),
            StructType::DvsUpgradeRejectedEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradeRejectedEvent>()?),
            StructType::DvsUpgradedEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradedEvent>()?),
            StructType::HostLocalPortCreatedEvent => Some(from.as_any_ref().downcast_ref::<HostLocalPortCreatedEvent>()?),
            StructType::OutOfSyncDvsHost => Some(from.as_any_ref().downcast_ref::<OutOfSyncDvsHost>()?),
            StructType::RecoveryEvent => Some(from.as_any_ref().downcast_ref::<RecoveryEvent>()?),
            StructType::RollbackEvent => Some(from.as_any_ref().downcast_ref::<RollbackEvent>()?),
            StructType::VmVnicPoolReservationViolationClearEvent => Some(from.as_any_ref().downcast_ref::<VmVnicPoolReservationViolationClearEvent>()?),
            StructType::VmVnicPoolReservationViolationRaiseEvent => Some(from.as_any_ref().downcast_ref::<VmVnicPoolReservationViolationRaiseEvent>()?),
            StructType::EventEx => Some(from.as_any_ref().downcast_ref::<EventEx>()?),
            StructType::GeneralEvent => Some(from.as_any_ref().downcast_ref::<GeneralEvent>()?),
            StructType::ExtendedEvent => Some(from.as_any_ref().downcast_ref::<ExtendedEvent>()?),
            StructType::GeneralHostErrorEvent => Some(from.as_any_ref().downcast_ref::<GeneralHostErrorEvent>()?),
            StructType::GeneralHostInfoEvent => Some(from.as_any_ref().downcast_ref::<GeneralHostInfoEvent>()?),
            StructType::GeneralHostWarningEvent => Some(from.as_any_ref().downcast_ref::<GeneralHostWarningEvent>()?),
            StructType::GeneralUserEvent => Some(from.as_any_ref().downcast_ref::<GeneralUserEvent>()?),
            StructType::GeneralVmErrorEvent => Some(from.as_any_ref().downcast_ref::<GeneralVmErrorEvent>()?),
            StructType::GeneralVmInfoEvent => Some(from.as_any_ref().downcast_ref::<GeneralVmInfoEvent>()?),
            StructType::GeneralVmWarningEvent => Some(from.as_any_ref().downcast_ref::<GeneralVmWarningEvent>()?),
            StructType::HealthStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<HealthStatusChangedEvent>()?),
            StructType::HostEvent => Some(from.as_any_ref().downcast_ref::<HostEvent>()?),
            StructType::AccountCreatedEvent => Some(from.as_any_ref().downcast_ref::<AccountCreatedEvent>()?),
            StructType::AccountRemovedEvent => Some(from.as_any_ref().downcast_ref::<AccountRemovedEvent>()?),
            StructType::AccountUpdatedEvent => Some(from.as_any_ref().downcast_ref::<AccountUpdatedEvent>()?),
            StructType::AdminPasswordNotChangedEvent => Some(from.as_any_ref().downcast_ref::<AdminPasswordNotChangedEvent>()?),
            StructType::CanceledHostOperationEvent => Some(from.as_any_ref().downcast_ref::<CanceledHostOperationEvent>()?),
            StructType::DatastoreDiscoveredEvent => Some(from.as_any_ref().downcast_ref::<DatastoreDiscoveredEvent>()?),
            StructType::DatastorePrincipalConfigured => Some(from.as_any_ref().downcast_ref::<DatastorePrincipalConfigured>()?),
            StructType::DatastoreRemovedOnHostEvent => Some(from.as_any_ref().downcast_ref::<DatastoreRemovedOnHostEvent>()?),
            StructType::DatastoreRenamedOnHostEvent => Some(from.as_any_ref().downcast_ref::<DatastoreRenamedOnHostEvent>()?),
            StructType::DrsResourceConfigureFailedEvent => Some(from.as_any_ref().downcast_ref::<DrsResourceConfigureFailedEvent>()?),
            StructType::DrsResourceConfigureSyncedEvent => Some(from.as_any_ref().downcast_ref::<DrsResourceConfigureSyncedEvent>()?),
            StructType::DuplicateIpDetectedEvent => Some(from.as_any_ref().downcast_ref::<DuplicateIpDetectedEvent>()?),
            StructType::DvsHealthStatusChangeEvent => Some(from.as_any_ref().downcast_ref::<DvsHealthStatusChangeEvent>()?),
            StructType::MtuMatchEvent => Some(from.as_any_ref().downcast_ref::<MtuMatchEvent>()?),
            StructType::MtuMismatchEvent => Some(from.as_any_ref().downcast_ref::<MtuMismatchEvent>()?),
            StructType::TeamingMatchEvent => Some(from.as_any_ref().downcast_ref::<TeamingMatchEvent>()?),
            StructType::TeamingMisMatchEvent => Some(from.as_any_ref().downcast_ref::<TeamingMisMatchEvent>()?),
            StructType::UplinkPortMtuNotSupportEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortMtuNotSupportEvent>()?),
            StructType::UplinkPortMtuSupportEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortMtuSupportEvent>()?),
            StructType::UplinkPortVlanTrunkedEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortVlanTrunkedEvent>()?),
            StructType::UplinkPortVlanUntrunkedEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortVlanUntrunkedEvent>()?),
            StructType::EnteredMaintenanceModeEvent => Some(from.as_any_ref().downcast_ref::<EnteredMaintenanceModeEvent>()?),
            StructType::EnteredStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<EnteredStandbyModeEvent>()?),
            StructType::DrsEnteredStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<DrsEnteredStandbyModeEvent>()?),
            StructType::EnteringMaintenanceModeEvent => Some(from.as_any_ref().downcast_ref::<EnteringMaintenanceModeEvent>()?),
            StructType::EnteringStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<EnteringStandbyModeEvent>()?),
            StructType::DrsEnteringStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<DrsEnteringStandbyModeEvent>()?),
            StructType::ExitMaintenanceModeEvent => Some(from.as_any_ref().downcast_ref::<ExitMaintenanceModeEvent>()?),
            StructType::ExitStandbyModeFailedEvent => Some(from.as_any_ref().downcast_ref::<ExitStandbyModeFailedEvent>()?),
            StructType::DrsExitStandbyModeFailedEvent => Some(from.as_any_ref().downcast_ref::<DrsExitStandbyModeFailedEvent>()?),
            StructType::ExitedStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<ExitedStandbyModeEvent>()?),
            StructType::DrsExitedStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<DrsExitedStandbyModeEvent>()?),
            StructType::ExitingStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<ExitingStandbyModeEvent>()?),
            StructType::DrsExitingStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<DrsExitingStandbyModeEvent>()?),
            StructType::GhostDvsProxySwitchDetectedEvent => Some(from.as_any_ref().downcast_ref::<GhostDvsProxySwitchDetectedEvent>()?),
            StructType::GhostDvsProxySwitchRemovedEvent => Some(from.as_any_ref().downcast_ref::<GhostDvsProxySwitchRemovedEvent>()?),
            StructType::HostAddFailedEvent => Some(from.as_any_ref().downcast_ref::<HostAddFailedEvent>()?),
            StructType::HostAddedEvent => Some(from.as_any_ref().downcast_ref::<HostAddedEvent>()?),
            StructType::HostAdminDisableEvent => Some(from.as_any_ref().downcast_ref::<HostAdminDisableEvent>()?),
            StructType::HostAdminEnableEvent => Some(from.as_any_ref().downcast_ref::<HostAdminEnableEvent>()?),
            StructType::HostCnxFailedAccountFailedEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedAccountFailedEvent>()?),
            StructType::HostCnxFailedAlreadyManagedEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedAlreadyManagedEvent>()?),
            StructType::HostCnxFailedBadCcagentEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedBadCcagentEvent>()?),
            StructType::HostCnxFailedBadUsernameEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedBadUsernameEvent>()?),
            StructType::HostCnxFailedBadVersionEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedBadVersionEvent>()?),
            StructType::HostCnxFailedCcagentUpgradeEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedCcagentUpgradeEvent>()?),
            StructType::HostCnxFailedEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedEvent>()?),
            StructType::HostCnxFailedNetworkErrorEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedNetworkErrorEvent>()?),
            StructType::HostCnxFailedNoAccessEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedNoAccessEvent>()?),
            StructType::HostCnxFailedNoConnectionEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedNoConnectionEvent>()?),
            StructType::HostCnxFailedNoLicenseEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedNoLicenseEvent>()?),
            StructType::HostCnxFailedNotFoundEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedNotFoundEvent>()?),
            StructType::HostCnxFailedTimeoutEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedTimeoutEvent>()?),
            StructType::HostComplianceCheckedEvent => Some(from.as_any_ref().downcast_ref::<HostComplianceCheckedEvent>()?),
            StructType::HostCompliantEvent => Some(from.as_any_ref().downcast_ref::<HostCompliantEvent>()?),
            StructType::HostConfigAppliedEvent => Some(from.as_any_ref().downcast_ref::<HostConfigAppliedEvent>()?),
            StructType::HostConnectedEvent => Some(from.as_any_ref().downcast_ref::<HostConnectedEvent>()?),
            StructType::HostConnectionLostEvent => Some(from.as_any_ref().downcast_ref::<HostConnectionLostEvent>()?),
            StructType::HostDasDisabledEvent => Some(from.as_any_ref().downcast_ref::<HostDasDisabledEvent>()?),
            StructType::HostDasDisablingEvent => Some(from.as_any_ref().downcast_ref::<HostDasDisablingEvent>()?),
            StructType::HostDasEnabledEvent => Some(from.as_any_ref().downcast_ref::<HostDasEnabledEvent>()?),
            StructType::HostDasEnablingEvent => Some(from.as_any_ref().downcast_ref::<HostDasEnablingEvent>()?),
            StructType::HostDasErrorEvent => Some(from.as_any_ref().downcast_ref::<HostDasErrorEvent>()?),
            StructType::HostDasEvent => Some(from.as_any_ref().downcast_ref::<HostDasEvent>()?),
            StructType::HostExtraNetworksEvent => Some(from.as_any_ref().downcast_ref::<HostExtraNetworksEvent>()?),
            StructType::HostIsolationIpPingFailedEvent => Some(from.as_any_ref().downcast_ref::<HostIsolationIpPingFailedEvent>()?),
            StructType::HostMissingNetworksEvent => Some(from.as_any_ref().downcast_ref::<HostMissingNetworksEvent>()?),
            StructType::HostNoAvailableNetworksEvent => Some(from.as_any_ref().downcast_ref::<HostNoAvailableNetworksEvent>()?),
            StructType::HostNoHaEnabledPortGroupsEvent => Some(from.as_any_ref().downcast_ref::<HostNoHaEnabledPortGroupsEvent>()?),
            StructType::HostNoRedundantManagementNetworkEvent => Some(from.as_any_ref().downcast_ref::<HostNoRedundantManagementNetworkEvent>()?),
            StructType::HostNotInClusterEvent => Some(from.as_any_ref().downcast_ref::<HostNotInClusterEvent>()?),
            StructType::HostPrimaryAgentNotShortNameEvent => Some(from.as_any_ref().downcast_ref::<HostPrimaryAgentNotShortNameEvent>()?),
            StructType::HostShortNameInconsistentEvent => Some(from.as_any_ref().downcast_ref::<HostShortNameInconsistentEvent>()?),
            StructType::HostDasOkEvent => Some(from.as_any_ref().downcast_ref::<HostDasOkEvent>()?),
            StructType::HostDisconnectedEvent => Some(from.as_any_ref().downcast_ref::<HostDisconnectedEvent>()?),
            StructType::HostEnableAdminFailedEvent => Some(from.as_any_ref().downcast_ref::<HostEnableAdminFailedEvent>()?),
            StructType::HostGetShortNameFailedEvent => Some(from.as_any_ref().downcast_ref::<HostGetShortNameFailedEvent>()?),
            StructType::HostInAuditModeEvent => Some(from.as_any_ref().downcast_ref::<HostInAuditModeEvent>()?),
            StructType::HostIpChangedEvent => Some(from.as_any_ref().downcast_ref::<HostIpChangedEvent>()?),
            StructType::HostIpInconsistentEvent => Some(from.as_any_ref().downcast_ref::<HostIpInconsistentEvent>()?),
            StructType::HostIpToShortNameFailedEvent => Some(from.as_any_ref().downcast_ref::<HostIpToShortNameFailedEvent>()?),
            StructType::HostNonCompliantEvent => Some(from.as_any_ref().downcast_ref::<HostNonCompliantEvent>()?),
            StructType::HostProfileAppliedEvent => Some(from.as_any_ref().downcast_ref::<HostProfileAppliedEvent>()?),
            StructType::HostReconnectionFailedEvent => Some(from.as_any_ref().downcast_ref::<HostReconnectionFailedEvent>()?),
            StructType::HostRemovedEvent => Some(from.as_any_ref().downcast_ref::<HostRemovedEvent>()?),
            StructType::HostShortNameToIpFailedEvent => Some(from.as_any_ref().downcast_ref::<HostShortNameToIpFailedEvent>()?),
            StructType::HostShutdownEvent => Some(from.as_any_ref().downcast_ref::<HostShutdownEvent>()?),
            StructType::HostSpecificationChangedEvent => Some(from.as_any_ref().downcast_ref::<HostSpecificationChangedEvent>()?),
            StructType::HostSpecificationRequireEvent => Some(from.as_any_ref().downcast_ref::<HostSpecificationRequireEvent>()?),
            StructType::HostSpecificationUpdateEvent => Some(from.as_any_ref().downcast_ref::<HostSpecificationUpdateEvent>()?),
            StructType::HostSubSpecificationDeleteEvent => Some(from.as_any_ref().downcast_ref::<HostSubSpecificationDeleteEvent>()?),
            StructType::HostSubSpecificationUpdateEvent => Some(from.as_any_ref().downcast_ref::<HostSubSpecificationUpdateEvent>()?),
            StructType::HostSyncFailedEvent => Some(from.as_any_ref().downcast_ref::<HostSyncFailedEvent>()?),
            StructType::HostUpgradeFailedEvent => Some(from.as_any_ref().downcast_ref::<HostUpgradeFailedEvent>()?),
            StructType::HostUserWorldSwapNotEnabledEvent => Some(from.as_any_ref().downcast_ref::<HostUserWorldSwapNotEnabledEvent>()?),
            StructType::HostVnicConnectedToCustomizedDvPortEvent => Some(from.as_any_ref().downcast_ref::<HostVnicConnectedToCustomizedDvPortEvent>()?),
            StructType::HostWwnChangedEvent => Some(from.as_any_ref().downcast_ref::<HostWwnChangedEvent>()?),
            StructType::HostWwnConflictEvent => Some(from.as_any_ref().downcast_ref::<HostWwnConflictEvent>()?),
            StructType::LocalDatastoreCreatedEvent => Some(from.as_any_ref().downcast_ref::<LocalDatastoreCreatedEvent>()?),
            StructType::LocalTsmEnabledEvent => Some(from.as_any_ref().downcast_ref::<LocalTsmEnabledEvent>()?),
            StructType::NasDatastoreCreatedEvent => Some(from.as_any_ref().downcast_ref::<NasDatastoreCreatedEvent>()?),
            StructType::NoDatastoresConfiguredEvent => Some(from.as_any_ref().downcast_ref::<NoDatastoresConfiguredEvent>()?),
            StructType::RemoteTsmEnabledEvent => Some(from.as_any_ref().downcast_ref::<RemoteTsmEnabledEvent>()?),
            StructType::TimedOutHostOperationEvent => Some(from.as_any_ref().downcast_ref::<TimedOutHostOperationEvent>()?),
            StructType::UpdatedAgentBeingRestartedEvent => Some(from.as_any_ref().downcast_ref::<UpdatedAgentBeingRestartedEvent>()?),
            StructType::UserAssignedToGroup => Some(from.as_any_ref().downcast_ref::<UserAssignedToGroup>()?),
            StructType::UserPasswordChanged => Some(from.as_any_ref().downcast_ref::<UserPasswordChanged>()?),
            StructType::UserUnassignedFromGroup => Some(from.as_any_ref().downcast_ref::<UserUnassignedFromGroup>()?),
            StructType::VmfsDatastoreCreatedEvent => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreCreatedEvent>()?),
            StructType::VmfsDatastoreExpandedEvent => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreExpandedEvent>()?),
            StructType::VmfsDatastoreExtendedEvent => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreExtendedEvent>()?),
            StructType::VcAgentUninstallFailedEvent => Some(from.as_any_ref().downcast_ref::<VcAgentUninstallFailedEvent>()?),
            StructType::VcAgentUninstalledEvent => Some(from.as_any_ref().downcast_ref::<VcAgentUninstalledEvent>()?),
            StructType::VcAgentUpgradeFailedEvent => Some(from.as_any_ref().downcast_ref::<VcAgentUpgradeFailedEvent>()?),
            StructType::VcAgentUpgradedEvent => Some(from.as_any_ref().downcast_ref::<VcAgentUpgradedEvent>()?),
            StructType::VimAccountPasswordChangedEvent => Some(from.as_any_ref().downcast_ref::<VimAccountPasswordChangedEvent>()?),
            StructType::IScsiBootFailureEvent => Some(from.as_any_ref().downcast_ref::<IScsiBootFailureEvent>()?),
            StructType::HostInventoryUnreadableEvent => Some(from.as_any_ref().downcast_ref::<HostInventoryUnreadableEvent>()?),
            StructType::LicenseEvent => Some(from.as_any_ref().downcast_ref::<LicenseEvent>()?),
            StructType::AllVirtualMachinesLicensedEvent => Some(from.as_any_ref().downcast_ref::<AllVirtualMachinesLicensedEvent>()?),
            StructType::HostInventoryFullEvent => Some(from.as_any_ref().downcast_ref::<HostInventoryFullEvent>()?),
            StructType::HostLicenseExpiredEvent => Some(from.as_any_ref().downcast_ref::<HostLicenseExpiredEvent>()?),
            StructType::IncorrectHostInformationEvent => Some(from.as_any_ref().downcast_ref::<IncorrectHostInformationEvent>()?),
            StructType::InvalidEditionEvent => Some(from.as_any_ref().downcast_ref::<InvalidEditionEvent>()?),
            StructType::LicenseNonComplianceEvent => Some(from.as_any_ref().downcast_ref::<LicenseNonComplianceEvent>()?),
            StructType::LicenseRestrictedEvent => Some(from.as_any_ref().downcast_ref::<LicenseRestrictedEvent>()?),
            StructType::LicenseServerAvailableEvent => Some(from.as_any_ref().downcast_ref::<LicenseServerAvailableEvent>()?),
            StructType::LicenseServerUnavailableEvent => Some(from.as_any_ref().downcast_ref::<LicenseServerUnavailableEvent>()?),
            StructType::NoLicenseEvent => Some(from.as_any_ref().downcast_ref::<NoLicenseEvent>()?),
            StructType::ServerLicenseExpiredEvent => Some(from.as_any_ref().downcast_ref::<ServerLicenseExpiredEvent>()?),
            StructType::UnlicensedVirtualMachinesEvent => Some(from.as_any_ref().downcast_ref::<UnlicensedVirtualMachinesEvent>()?),
            StructType::UnlicensedVirtualMachinesFoundEvent => Some(from.as_any_ref().downcast_ref::<UnlicensedVirtualMachinesFoundEvent>()?),
            StructType::VMotionLicenseExpiredEvent => Some(from.as_any_ref().downcast_ref::<VMotionLicenseExpiredEvent>()?),
            StructType::LicenseExpiredEvent => Some(from.as_any_ref().downcast_ref::<LicenseExpiredEvent>()?),
            StructType::LockerMisconfiguredEvent => Some(from.as_any_ref().downcast_ref::<LockerMisconfiguredEvent>()?),
            StructType::LockerReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<LockerReconfiguredEvent>()?),
            StructType::NetworkRollbackEvent => Some(from.as_any_ref().downcast_ref::<NetworkRollbackEvent>()?),
            StructType::ProfileEvent => Some(from.as_any_ref().downcast_ref::<ProfileEvent>()?),
            StructType::ProfileAssociatedEvent => Some(from.as_any_ref().downcast_ref::<ProfileAssociatedEvent>()?),
            StructType::ProfileChangedEvent => Some(from.as_any_ref().downcast_ref::<ProfileChangedEvent>()?),
            StructType::ProfileCreatedEvent => Some(from.as_any_ref().downcast_ref::<ProfileCreatedEvent>()?),
            StructType::ProfileDissociatedEvent => Some(from.as_any_ref().downcast_ref::<ProfileDissociatedEvent>()?),
            StructType::ProfileReferenceHostChangedEvent => Some(from.as_any_ref().downcast_ref::<ProfileReferenceHostChangedEvent>()?),
            StructType::ProfileRemovedEvent => Some(from.as_any_ref().downcast_ref::<ProfileRemovedEvent>()?),
            StructType::ResourcePoolEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolEvent>()?),
            StructType::ResourcePoolCreatedEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolCreatedEvent>()?),
            StructType::ResourcePoolDestroyedEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolDestroyedEvent>()?),
            StructType::ResourcePoolMovedEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolMovedEvent>()?),
            StructType::ResourcePoolReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolReconfiguredEvent>()?),
            StructType::ResourceViolatedEvent => Some(from.as_any_ref().downcast_ref::<ResourceViolatedEvent>()?),
            StructType::ScheduledTaskEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskEvent>()?),
            StructType::ScheduledTaskCompletedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskCompletedEvent>()?),
            StructType::ScheduledTaskCreatedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskCreatedEvent>()?),
            StructType::ScheduledTaskEmailCompletedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskEmailCompletedEvent>()?),
            StructType::ScheduledTaskEmailFailedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskEmailFailedEvent>()?),
            StructType::ScheduledTaskFailedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskFailedEvent>()?),
            StructType::ScheduledTaskReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskReconfiguredEvent>()?),
            StructType::ScheduledTaskRemovedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskRemovedEvent>()?),
            StructType::ScheduledTaskStartedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskStartedEvent>()?),
            StructType::SessionEvent => Some(from.as_any_ref().downcast_ref::<SessionEvent>()?),
            StructType::AlreadyAuthenticatedSessionEvent => Some(from.as_any_ref().downcast_ref::<AlreadyAuthenticatedSessionEvent>()?),
            StructType::BadUsernameSessionEvent => Some(from.as_any_ref().downcast_ref::<BadUsernameSessionEvent>()?),
            StructType::GlobalMessageChangedEvent => Some(from.as_any_ref().downcast_ref::<GlobalMessageChangedEvent>()?),
            StructType::NoAccessUserEvent => Some(from.as_any_ref().downcast_ref::<NoAccessUserEvent>()?),
            StructType::ServerStartedSessionEvent => Some(from.as_any_ref().downcast_ref::<ServerStartedSessionEvent>()?),
            StructType::SessionTerminatedEvent => Some(from.as_any_ref().downcast_ref::<SessionTerminatedEvent>()?),
            StructType::UserLoginSessionEvent => Some(from.as_any_ref().downcast_ref::<UserLoginSessionEvent>()?),
            StructType::UserLogoutSessionEvent => Some(from.as_any_ref().downcast_ref::<UserLogoutSessionEvent>()?),
            StructType::TaskEvent => Some(from.as_any_ref().downcast_ref::<TaskEvent>()?),
            StructType::TaskTimeoutEvent => Some(from.as_any_ref().downcast_ref::<TaskTimeoutEvent>()?),
            StructType::TemplateUpgradeEvent => Some(from.as_any_ref().downcast_ref::<TemplateUpgradeEvent>()?),
            StructType::TemplateBeingUpgradedEvent => Some(from.as_any_ref().downcast_ref::<TemplateBeingUpgradedEvent>()?),
            StructType::TemplateUpgradeFailedEvent => Some(from.as_any_ref().downcast_ref::<TemplateUpgradeFailedEvent>()?),
            StructType::TemplateUpgradedEvent => Some(from.as_any_ref().downcast_ref::<TemplateUpgradedEvent>()?),
            StructType::UpgradeEvent => Some(from.as_any_ref().downcast_ref::<UpgradeEvent>()?),
            StructType::ErrorUpgradeEvent => Some(from.as_any_ref().downcast_ref::<ErrorUpgradeEvent>()?),
            StructType::InfoUpgradeEvent => Some(from.as_any_ref().downcast_ref::<InfoUpgradeEvent>()?),
            StructType::UserUpgradeEvent => Some(from.as_any_ref().downcast_ref::<UserUpgradeEvent>()?),
            StructType::WarningUpgradeEvent => Some(from.as_any_ref().downcast_ref::<WarningUpgradeEvent>()?),
            StructType::VmEvent => Some(from.as_any_ref().downcast_ref::<VmEvent>()?),
            StructType::CustomizationEvent => Some(from.as_any_ref().downcast_ref::<CustomizationEvent>()?),
            StructType::CustomizationFailed => Some(from.as_any_ref().downcast_ref::<CustomizationFailed>()?),
            StructType::CustomizationLinuxIdentityFailed => Some(from.as_any_ref().downcast_ref::<CustomizationLinuxIdentityFailed>()?),
            StructType::CustomizationNetworkSetupFailed => Some(from.as_any_ref().downcast_ref::<CustomizationNetworkSetupFailed>()?),
            StructType::CustomizationSysprepFailed => Some(from.as_any_ref().downcast_ref::<CustomizationSysprepFailed>()?),
            StructType::CustomizationUnknownFailure => Some(from.as_any_ref().downcast_ref::<CustomizationUnknownFailure>()?),
            StructType::CustomizationStartedEvent => Some(from.as_any_ref().downcast_ref::<CustomizationStartedEvent>()?),
            StructType::CustomizationSucceeded => Some(from.as_any_ref().downcast_ref::<CustomizationSucceeded>()?),
            StructType::DrsRuleComplianceEvent => Some(from.as_any_ref().downcast_ref::<DrsRuleComplianceEvent>()?),
            StructType::DrsRuleViolationEvent => Some(from.as_any_ref().downcast_ref::<DrsRuleViolationEvent>()?),
            StructType::DrsSoftRuleViolationEvent => Some(from.as_any_ref().downcast_ref::<DrsSoftRuleViolationEvent>()?),
            StructType::MigrationEvent => Some(from.as_any_ref().downcast_ref::<MigrationEvent>()?),
            StructType::MigrationErrorEvent => Some(from.as_any_ref().downcast_ref::<MigrationErrorEvent>()?),
            StructType::MigrationHostErrorEvent => Some(from.as_any_ref().downcast_ref::<MigrationHostErrorEvent>()?),
            StructType::MigrationHostWarningEvent => Some(from.as_any_ref().downcast_ref::<MigrationHostWarningEvent>()?),
            StructType::MigrationResourceErrorEvent => Some(from.as_any_ref().downcast_ref::<MigrationResourceErrorEvent>()?),
            StructType::MigrationResourceWarningEvent => Some(from.as_any_ref().downcast_ref::<MigrationResourceWarningEvent>()?),
            StructType::MigrationWarningEvent => Some(from.as_any_ref().downcast_ref::<MigrationWarningEvent>()?),
            StructType::NoMaintenanceModeDrsRecommendationForVm => Some(from.as_any_ref().downcast_ref::<NoMaintenanceModeDrsRecommendationForVm>()?),
            StructType::NotEnoughResourcesToStartVmEvent => Some(from.as_any_ref().downcast_ref::<NotEnoughResourcesToStartVmEvent>()?),
            StructType::VmAcquiredMksTicketEvent => Some(from.as_any_ref().downcast_ref::<VmAcquiredMksTicketEvent>()?),
            StructType::VmAcquiredTicketEvent => Some(from.as_any_ref().downcast_ref::<VmAcquiredTicketEvent>()?),
            StructType::VmAutoRenameEvent => Some(from.as_any_ref().downcast_ref::<VmAutoRenameEvent>()?),
            StructType::VmBeingCreatedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingCreatedEvent>()?),
            StructType::VmBeingDeployedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingDeployedEvent>()?),
            StructType::VmBeingHotMigratedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingHotMigratedEvent>()?),
            StructType::VmBeingMigratedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingMigratedEvent>()?),
            StructType::VmCloneEvent => Some(from.as_any_ref().downcast_ref::<VmCloneEvent>()?),
            StructType::VmBeingClonedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingClonedEvent>()?),
            StructType::VmBeingClonedNoFolderEvent => Some(from.as_any_ref().downcast_ref::<VmBeingClonedNoFolderEvent>()?),
            StructType::VmCloneFailedEvent => Some(from.as_any_ref().downcast_ref::<VmCloneFailedEvent>()?),
            StructType::VmClonedEvent => Some(from.as_any_ref().downcast_ref::<VmClonedEvent>()?),
            StructType::VmConfigMissingEvent => Some(from.as_any_ref().downcast_ref::<VmConfigMissingEvent>()?),
            StructType::VmConnectedEvent => Some(from.as_any_ref().downcast_ref::<VmConnectedEvent>()?),
            StructType::VmCreatedEvent => Some(from.as_any_ref().downcast_ref::<VmCreatedEvent>()?),
            StructType::VmDasBeingResetEvent => Some(from.as_any_ref().downcast_ref::<VmDasBeingResetEvent>()?),
            StructType::VmDasBeingResetWithScreenshotEvent => Some(from.as_any_ref().downcast_ref::<VmDasBeingResetWithScreenshotEvent>()?),
            StructType::VmDasResetFailedEvent => Some(from.as_any_ref().downcast_ref::<VmDasResetFailedEvent>()?),
            StructType::VmDasUpdateErrorEvent => Some(from.as_any_ref().downcast_ref::<VmDasUpdateErrorEvent>()?),
            StructType::VmDasUpdateOkEvent => Some(from.as_any_ref().downcast_ref::<VmDasUpdateOkEvent>()?),
            StructType::VmDateRolledBackEvent => Some(from.as_any_ref().downcast_ref::<VmDateRolledBackEvent>()?),
            StructType::VmDeployFailedEvent => Some(from.as_any_ref().downcast_ref::<VmDeployFailedEvent>()?),
            StructType::VmDeployedEvent => Some(from.as_any_ref().downcast_ref::<VmDeployedEvent>()?),
            StructType::VmDisconnectedEvent => Some(from.as_any_ref().downcast_ref::<VmDisconnectedEvent>()?),
            StructType::VmDiscoveredEvent => Some(from.as_any_ref().downcast_ref::<VmDiscoveredEvent>()?),
            StructType::VmDiskFailedEvent => Some(from.as_any_ref().downcast_ref::<VmDiskFailedEvent>()?),
            StructType::VmEmigratingEvent => Some(from.as_any_ref().downcast_ref::<VmEmigratingEvent>()?),
            StructType::VmEndRecordingEvent => Some(from.as_any_ref().downcast_ref::<VmEndRecordingEvent>()?),
            StructType::VmEndReplayingEvent => Some(from.as_any_ref().downcast_ref::<VmEndReplayingEvent>()?),
            StructType::VmFailedMigrateEvent => Some(from.as_any_ref().downcast_ref::<VmFailedMigrateEvent>()?),
            StructType::VmFailedRelayoutEvent => Some(from.as_any_ref().downcast_ref::<VmFailedRelayoutEvent>()?),
            StructType::VmFailedRelayoutOnVmfs2DatastoreEvent => Some(from.as_any_ref().downcast_ref::<VmFailedRelayoutOnVmfs2DatastoreEvent>()?),
            StructType::VmFailedStartingSecondaryEvent => Some(from.as_any_ref().downcast_ref::<VmFailedStartingSecondaryEvent>()?),
            StructType::VmFailedToPowerOffEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToPowerOffEvent>()?),
            StructType::VmFailedToPowerOnEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToPowerOnEvent>()?),
            StructType::VmFailedToRebootGuestEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToRebootGuestEvent>()?),
            StructType::VmFailedToResetEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToResetEvent>()?),
            StructType::VmFailedToShutdownGuestEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToShutdownGuestEvent>()?),
            StructType::VmFailedToStandbyGuestEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToStandbyGuestEvent>()?),
            StructType::VmFailedToSuspendEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToSuspendEvent>()?),
            StructType::VmFailedUpdatingSecondaryConfig => Some(from.as_any_ref().downcast_ref::<VmFailedUpdatingSecondaryConfig>()?),
            StructType::VmFailoverFailed => Some(from.as_any_ref().downcast_ref::<VmFailoverFailed>()?),
            StructType::VmFaultToleranceStateChangedEvent => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceStateChangedEvent>()?),
            StructType::VmFaultToleranceTurnedOffEvent => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceTurnedOffEvent>()?),
            StructType::VmFaultToleranceVmTerminatedEvent => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceVmTerminatedEvent>()?),
            StructType::VmGuestOsCrashedEvent => Some(from.as_any_ref().downcast_ref::<VmGuestOsCrashedEvent>()?),
            StructType::VmGuestRebootEvent => Some(from.as_any_ref().downcast_ref::<VmGuestRebootEvent>()?),
            StructType::VmGuestShutdownEvent => Some(from.as_any_ref().downcast_ref::<VmGuestShutdownEvent>()?),
            StructType::VmGuestStandbyEvent => Some(from.as_any_ref().downcast_ref::<VmGuestStandbyEvent>()?),
            StructType::VmInstanceUuidAssignedEvent => Some(from.as_any_ref().downcast_ref::<VmInstanceUuidAssignedEvent>()?),
            StructType::VmInstanceUuidChangedEvent => Some(from.as_any_ref().downcast_ref::<VmInstanceUuidChangedEvent>()?),
            StructType::VmInstanceUuidConflictEvent => Some(from.as_any_ref().downcast_ref::<VmInstanceUuidConflictEvent>()?),
            StructType::VmMacAssignedEvent => Some(from.as_any_ref().downcast_ref::<VmMacAssignedEvent>()?),
            StructType::VmMacChangedEvent => Some(from.as_any_ref().downcast_ref::<VmMacChangedEvent>()?),
            StructType::VmMacConflictEvent => Some(from.as_any_ref().downcast_ref::<VmMacConflictEvent>()?),
            StructType::VmMaxFtRestartCountReached => Some(from.as_any_ref().downcast_ref::<VmMaxFtRestartCountReached>()?),
            StructType::VmMaxRestartCountReached => Some(from.as_any_ref().downcast_ref::<VmMaxRestartCountReached>()?),
            StructType::VmMessageErrorEvent => Some(from.as_any_ref().downcast_ref::<VmMessageErrorEvent>()?),
            StructType::VmMessageEvent => Some(from.as_any_ref().downcast_ref::<VmMessageEvent>()?),
            StructType::VmMessageWarningEvent => Some(from.as_any_ref().downcast_ref::<VmMessageWarningEvent>()?),
            StructType::VmMigratedEvent => Some(from.as_any_ref().downcast_ref::<VmMigratedEvent>()?),
            StructType::DrsVmMigratedEvent => Some(from.as_any_ref().downcast_ref::<DrsVmMigratedEvent>()?),
            StructType::VmNoCompatibleHostForSecondaryEvent => Some(from.as_any_ref().downcast_ref::<VmNoCompatibleHostForSecondaryEvent>()?),
            StructType::VmNoNetworkAccessEvent => Some(from.as_any_ref().downcast_ref::<VmNoNetworkAccessEvent>()?),
            StructType::VmOrphanedEvent => Some(from.as_any_ref().downcast_ref::<VmOrphanedEvent>()?),
            StructType::VmPoweredOffEvent => Some(from.as_any_ref().downcast_ref::<VmPoweredOffEvent>()?),
            StructType::VmPowerOffOnIsolationEvent => Some(from.as_any_ref().downcast_ref::<VmPowerOffOnIsolationEvent>()?),
            StructType::VmShutdownOnIsolationEvent => Some(from.as_any_ref().downcast_ref::<VmShutdownOnIsolationEvent>()?),
            StructType::VmPoweredOnEvent => Some(from.as_any_ref().downcast_ref::<VmPoweredOnEvent>()?),
            StructType::DrsVmPoweredOnEvent => Some(from.as_any_ref().downcast_ref::<DrsVmPoweredOnEvent>()?),
            StructType::VmRestartedOnAlternateHostEvent => Some(from.as_any_ref().downcast_ref::<VmRestartedOnAlternateHostEvent>()?),
            StructType::VmPoweringOnWithCustomizedDvPortEvent => Some(from.as_any_ref().downcast_ref::<VmPoweringOnWithCustomizedDvPortEvent>()?),
            StructType::VmPrimaryFailoverEvent => Some(from.as_any_ref().downcast_ref::<VmPrimaryFailoverEvent>()?),
            StructType::VmReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<VmReconfiguredEvent>()?),
            StructType::VmRegisteredEvent => Some(from.as_any_ref().downcast_ref::<VmRegisteredEvent>()?),
            StructType::VmRelayoutSuccessfulEvent => Some(from.as_any_ref().downcast_ref::<VmRelayoutSuccessfulEvent>()?),
            StructType::VmRelayoutUpToDateEvent => Some(from.as_any_ref().downcast_ref::<VmRelayoutUpToDateEvent>()?),
            StructType::VmReloadFromPathEvent => Some(from.as_any_ref().downcast_ref::<VmReloadFromPathEvent>()?),
            StructType::VmReloadFromPathFailedEvent => Some(from.as_any_ref().downcast_ref::<VmReloadFromPathFailedEvent>()?),
            StructType::VmRelocateSpecEvent => Some(from.as_any_ref().downcast_ref::<VmRelocateSpecEvent>()?),
            StructType::VmBeingRelocatedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingRelocatedEvent>()?),
            StructType::VmRelocateFailedEvent => Some(from.as_any_ref().downcast_ref::<VmRelocateFailedEvent>()?),
            StructType::VmRelocatedEvent => Some(from.as_any_ref().downcast_ref::<VmRelocatedEvent>()?),
            StructType::VmRemoteConsoleConnectedEvent => Some(from.as_any_ref().downcast_ref::<VmRemoteConsoleConnectedEvent>()?),
            StructType::VmRemoteConsoleDisconnectedEvent => Some(from.as_any_ref().downcast_ref::<VmRemoteConsoleDisconnectedEvent>()?),
            StructType::VmRemovedEvent => Some(from.as_any_ref().downcast_ref::<VmRemovedEvent>()?),
            StructType::VmRenamedEvent => Some(from.as_any_ref().downcast_ref::<VmRenamedEvent>()?),
            StructType::VmRequirementsExceedCurrentEvcModeEvent => Some(from.as_any_ref().downcast_ref::<VmRequirementsExceedCurrentEvcModeEvent>()?),
            StructType::VmResettingEvent => Some(from.as_any_ref().downcast_ref::<VmResettingEvent>()?),
            StructType::VmResourcePoolMovedEvent => Some(from.as_any_ref().downcast_ref::<VmResourcePoolMovedEvent>()?),
            StructType::VmResourceReallocatedEvent => Some(from.as_any_ref().downcast_ref::<VmResourceReallocatedEvent>()?),
            StructType::VmResumingEvent => Some(from.as_any_ref().downcast_ref::<VmResumingEvent>()?),
            StructType::VmSecondaryAddedEvent => Some(from.as_any_ref().downcast_ref::<VmSecondaryAddedEvent>()?),
            StructType::VmSecondaryDisabledBySystemEvent => Some(from.as_any_ref().downcast_ref::<VmSecondaryDisabledBySystemEvent>()?),
            StructType::VmSecondaryDisabledEvent => Some(from.as_any_ref().downcast_ref::<VmSecondaryDisabledEvent>()?),
            StructType::VmSecondaryEnabledEvent => Some(from.as_any_ref().downcast_ref::<VmSecondaryEnabledEvent>()?),
            StructType::VmSecondaryStartedEvent => Some(from.as_any_ref().downcast_ref::<VmSecondaryStartedEvent>()?),
            StructType::VmStartRecordingEvent => Some(from.as_any_ref().downcast_ref::<VmStartRecordingEvent>()?),
            StructType::VmStartReplayingEvent => Some(from.as_any_ref().downcast_ref::<VmStartReplayingEvent>()?),
            StructType::VmStartingEvent => Some(from.as_any_ref().downcast_ref::<VmStartingEvent>()?),
            StructType::VmUnsupportedStartingEvent => Some(from.as_any_ref().downcast_ref::<VmUnsupportedStartingEvent>()?),
            StructType::VmStartingSecondaryEvent => Some(from.as_any_ref().downcast_ref::<VmStartingSecondaryEvent>()?),
            StructType::VmStaticMacConflictEvent => Some(from.as_any_ref().downcast_ref::<VmStaticMacConflictEvent>()?),
            StructType::VmStoppingEvent => Some(from.as_any_ref().downcast_ref::<VmStoppingEvent>()?),
            StructType::VmSuspendedEvent => Some(from.as_any_ref().downcast_ref::<VmSuspendedEvent>()?),
            StructType::VmSuspendingEvent => Some(from.as_any_ref().downcast_ref::<VmSuspendingEvent>()?),
            StructType::VmTimedoutStartingSecondaryEvent => Some(from.as_any_ref().downcast_ref::<VmTimedoutStartingSecondaryEvent>()?),
            StructType::VmUpgradeCompleteEvent => Some(from.as_any_ref().downcast_ref::<VmUpgradeCompleteEvent>()?),
            StructType::VmUpgradeFailedEvent => Some(from.as_any_ref().downcast_ref::<VmUpgradeFailedEvent>()?),
            StructType::VmUpgradingEvent => Some(from.as_any_ref().downcast_ref::<VmUpgradingEvent>()?),
            StructType::VmUuidAssignedEvent => Some(from.as_any_ref().downcast_ref::<VmUuidAssignedEvent>()?),
            StructType::VmUuidChangedEvent => Some(from.as_any_ref().downcast_ref::<VmUuidChangedEvent>()?),
            StructType::VmUuidConflictEvent => Some(from.as_any_ref().downcast_ref::<VmUuidConflictEvent>()?),
            StructType::VmWwnAssignedEvent => Some(from.as_any_ref().downcast_ref::<VmWwnAssignedEvent>()?),
            StructType::VmWwnChangedEvent => Some(from.as_any_ref().downcast_ref::<VmWwnChangedEvent>()?),
            StructType::VmWwnConflictEvent => Some(from.as_any_ref().downcast_ref::<VmWwnConflictEvent>()?),
            StructType::EventArgument => Some(from.as_any_ref().downcast_ref::<EventArgument>()?),
            StructType::EntityEventArgument => Some(from.as_any_ref().downcast_ref::<EntityEventArgument>()?),
            StructType::AlarmEventArgument => Some(from.as_any_ref().downcast_ref::<AlarmEventArgument>()?),
            StructType::ComputeResourceEventArgument => Some(from.as_any_ref().downcast_ref::<ComputeResourceEventArgument>()?),
            StructType::DatacenterEventArgument => Some(from.as_any_ref().downcast_ref::<DatacenterEventArgument>()?),
            StructType::DatastoreEventArgument => Some(from.as_any_ref().downcast_ref::<DatastoreEventArgument>()?),
            StructType::DvsEventArgument => Some(from.as_any_ref().downcast_ref::<DvsEventArgument>()?),
            StructType::FolderEventArgument => Some(from.as_any_ref().downcast_ref::<FolderEventArgument>()?),
            StructType::HostEventArgument => Some(from.as_any_ref().downcast_ref::<HostEventArgument>()?),
            StructType::ManagedEntityEventArgument => Some(from.as_any_ref().downcast_ref::<ManagedEntityEventArgument>()?),
            StructType::NetworkEventArgument => Some(from.as_any_ref().downcast_ref::<NetworkEventArgument>()?),
            StructType::ResourcePoolEventArgument => Some(from.as_any_ref().downcast_ref::<ResourcePoolEventArgument>()?),
            StructType::ScheduledTaskEventArgument => Some(from.as_any_ref().downcast_ref::<ScheduledTaskEventArgument>()?),
            StructType::VmEventArgument => Some(from.as_any_ref().downcast_ref::<VmEventArgument>()?),
            StructType::ProfileEventArgument => Some(from.as_any_ref().downcast_ref::<ProfileEventArgument>()?),
            StructType::RoleEventArgument => Some(from.as_any_ref().downcast_ref::<RoleEventArgument>()?),
            StructType::EventDescription => Some(from.as_any_ref().downcast_ref::<EventDescription>()?),
            StructType::EventArgDesc => Some(from.as_any_ref().downcast_ref::<EventArgDesc>()?),
            StructType::EventDescriptionEventDetail => Some(from.as_any_ref().downcast_ref::<EventDescriptionEventDetail>()?),
            StructType::EventFilterSpec => Some(from.as_any_ref().downcast_ref::<EventFilterSpec>()?),
            StructType::EventFilterSpecByEntity => Some(from.as_any_ref().downcast_ref::<EventFilterSpecByEntity>()?),
            StructType::EventFilterSpecByTime => Some(from.as_any_ref().downcast_ref::<EventFilterSpecByTime>()?),
            StructType::EventFilterSpecByUsername => Some(from.as_any_ref().downcast_ref::<EventFilterSpecByUsername>()?),
            StructType::ExtendedEventPair => Some(from.as_any_ref().downcast_ref::<ExtendedEventPair>()?),
            StructType::VnicPortArgument => Some(from.as_any_ref().downcast_ref::<VnicPortArgument>()?),
            StructType::ExtExtendedProductInfo => Some(from.as_any_ref().downcast_ref::<ExtExtendedProductInfo>()?),
            StructType::ManagedByInfo => Some(from.as_any_ref().downcast_ref::<ManagedByInfo>()?),
            StructType::ExtManagedEntityInfo => Some(from.as_any_ref().downcast_ref::<ExtManagedEntityInfo>()?),
            StructType::ExtSolutionManagerInfo => Some(from.as_any_ref().downcast_ref::<ExtSolutionManagerInfo>()?),
            StructType::ExtSolutionManagerInfoTabInfo => Some(from.as_any_ref().downcast_ref::<ExtSolutionManagerInfoTabInfo>()?),
            StructType::AnswerFileUpdateFailure => Some(from.as_any_ref().downcast_ref::<AnswerFileUpdateFailure>()?),
            StructType::ConflictingConfigurationConfig => Some(from.as_any_ref().downcast_ref::<ConflictingConfigurationConfig>()?),
            StructType::DatacenterMismatchArgument => Some(from.as_any_ref().downcast_ref::<DatacenterMismatchArgument>()?),
            StructType::DvsApplyOperationFaultFaultOnObject => Some(from.as_any_ref().downcast_ref::<DvsApplyOperationFaultFaultOnObject>()?),
            StructType::DvsOperationBulkFaultFaultOnHost => Some(from.as_any_ref().downcast_ref::<DvsOperationBulkFaultFaultOnHost>()?),
            StructType::ImportOperationBulkFaultFaultOnImport => Some(from.as_any_ref().downcast_ref::<ImportOperationBulkFaultFaultOnImport>()?),
            StructType::MultipleCertificatesVerifyFaultThumbprintData => Some(from.as_any_ref().downcast_ref::<MultipleCertificatesVerifyFaultThumbprintData>()?),
            StructType::NoPermissionEntityPrivileges => Some(from.as_any_ref().downcast_ref::<NoPermissionEntityPrivileges>()?),
            StructType::ProfileUpdateFailedUpdateFailure => Some(from.as_any_ref().downcast_ref::<ProfileUpdateFailedUpdateFailure>()?),
            StructType::HostActiveDirectory => Some(from.as_any_ref().downcast_ref::<HostActiveDirectory>()?),
            StructType::HostActiveDirectorySpec => Some(from.as_any_ref().downcast_ref::<HostActiveDirectorySpec>()?),
            StructType::HostAssignableHardwareBinding => Some(from.as_any_ref().downcast_ref::<HostAssignableHardwareBinding>()?),
            StructType::HostAssignableHardwareConfig => Some(from.as_any_ref().downcast_ref::<HostAssignableHardwareConfig>()?),
            StructType::HostAssignableHardwareConfigAttributeOverride => Some(from.as_any_ref().downcast_ref::<HostAssignableHardwareConfigAttributeOverride>()?),
            StructType::HostAuthenticationManagerInfo => Some(from.as_any_ref().downcast_ref::<HostAuthenticationManagerInfo>()?),
            StructType::HostAuthenticationStoreInfo => Some(from.as_any_ref().downcast_ref::<HostAuthenticationStoreInfo>()?),
            StructType::HostDirectoryStoreInfo => Some(from.as_any_ref().downcast_ref::<HostDirectoryStoreInfo>()?),
            StructType::HostActiveDirectoryInfo => Some(from.as_any_ref().downcast_ref::<HostActiveDirectoryInfo>()?),
            StructType::HostLocalAuthenticationInfo => Some(from.as_any_ref().downcast_ref::<HostLocalAuthenticationInfo>()?),
            StructType::AutoStartPowerInfo => Some(from.as_any_ref().downcast_ref::<AutoStartPowerInfo>()?),
            StructType::HostAutoStartManagerConfig => Some(from.as_any_ref().downcast_ref::<HostAutoStartManagerConfig>()?),
            StructType::AutoStartDefaults => Some(from.as_any_ref().downcast_ref::<AutoStartDefaults>()?),
            StructType::HostBiosInfo => Some(from.as_any_ref().downcast_ref::<HostBiosInfo>()?),
            StructType::HostBootDeviceInfo => Some(from.as_any_ref().downcast_ref::<HostBootDeviceInfo>()?),
            StructType::HostBootDevice => Some(from.as_any_ref().downcast_ref::<HostBootDevice>()?),
            StructType::HostCacheConfigurationInfo => Some(from.as_any_ref().downcast_ref::<HostCacheConfigurationInfo>()?),
            StructType::HostCacheConfigurationSpec => Some(from.as_any_ref().downcast_ref::<HostCacheConfigurationSpec>()?),
            StructType::HostCapability => Some(from.as_any_ref().downcast_ref::<HostCapability>()?),
            StructType::HostCertificateManagerCertificateInfo => Some(from.as_any_ref().downcast_ref::<HostCertificateManagerCertificateInfo>()?),
            StructType::HostCertificateManagerCertificateSpec => Some(from.as_any_ref().downcast_ref::<HostCertificateManagerCertificateSpec>()?),
            StructType::HostConfigChange => Some(from.as_any_ref().downcast_ref::<HostConfigChange>()?),
            StructType::HostConfigInfo => Some(from.as_any_ref().downcast_ref::<HostConfigInfo>()?),
            StructType::HostConfigManager => Some(from.as_any_ref().downcast_ref::<HostConfigManager>()?),
            StructType::HostConfigSpec => Some(from.as_any_ref().downcast_ref::<HostConfigSpec>()?),
            StructType::HostConnectInfo => Some(from.as_any_ref().downcast_ref::<HostConnectInfo>()?),
            StructType::HostDatastoreConnectInfo => Some(from.as_any_ref().downcast_ref::<HostDatastoreConnectInfo>()?),
            StructType::HostDatastoreExistsConnectInfo => Some(from.as_any_ref().downcast_ref::<HostDatastoreExistsConnectInfo>()?),
            StructType::HostDatastoreNameConflictConnectInfo => Some(from.as_any_ref().downcast_ref::<HostDatastoreNameConflictConnectInfo>()?),
            StructType::HostLicenseConnectInfo => Some(from.as_any_ref().downcast_ref::<HostLicenseConnectInfo>()?),
            StructType::HostConnectInfoNetworkInfo => Some(from.as_any_ref().downcast_ref::<HostConnectInfoNetworkInfo>()?),
            StructType::HostNewNetworkConnectInfo => Some(from.as_any_ref().downcast_ref::<HostNewNetworkConnectInfo>()?),
            StructType::HostConnectSpec => Some(from.as_any_ref().downcast_ref::<HostConnectSpec>()?),
            StructType::HostCpuIdInfo => Some(from.as_any_ref().downcast_ref::<HostCpuIdInfo>()?),
            StructType::HostCpuInfo => Some(from.as_any_ref().downcast_ref::<HostCpuInfo>()?),
            StructType::HostCpuPackage => Some(from.as_any_ref().downcast_ref::<HostCpuPackage>()?),
            StructType::HostCpuPowerManagementInfo => Some(from.as_any_ref().downcast_ref::<HostCpuPowerManagementInfo>()?),
            StructType::HostHyperThreadScheduleInfo => Some(from.as_any_ref().downcast_ref::<HostHyperThreadScheduleInfo>()?),
            StructType::HostDataTransportConnectionInfo => Some(from.as_any_ref().downcast_ref::<HostDataTransportConnectionInfo>()?),
            StructType::HostNfcConnectionInfo => Some(from.as_any_ref().downcast_ref::<HostNfcConnectionInfo>()?),
            StructType::FileInfo => Some(from.as_any_ref().downcast_ref::<FileInfo>()?),
            StructType::FloppyImageFileInfo => Some(from.as_any_ref().downcast_ref::<FloppyImageFileInfo>()?),
            StructType::FolderFileInfo => Some(from.as_any_ref().downcast_ref::<FolderFileInfo>()?),
            StructType::IsoImageFileInfo => Some(from.as_any_ref().downcast_ref::<IsoImageFileInfo>()?),
            StructType::VmConfigFileInfo => Some(from.as_any_ref().downcast_ref::<VmConfigFileInfo>()?),
            StructType::TemplateConfigFileInfo => Some(from.as_any_ref().downcast_ref::<TemplateConfigFileInfo>()?),
            StructType::VmDiskFileInfo => Some(from.as_any_ref().downcast_ref::<VmDiskFileInfo>()?),
            StructType::VmLogFileInfo => Some(from.as_any_ref().downcast_ref::<VmLogFileInfo>()?),
            StructType::VmNvramFileInfo => Some(from.as_any_ref().downcast_ref::<VmNvramFileInfo>()?),
            StructType::VmSnapshotFileInfo => Some(from.as_any_ref().downcast_ref::<VmSnapshotFileInfo>()?),
            StructType::FileQueryFlags => Some(from.as_any_ref().downcast_ref::<FileQueryFlags>()?),
            StructType::FileQuery => Some(from.as_any_ref().downcast_ref::<FileQuery>()?),
            StructType::FloppyImageFileQuery => Some(from.as_any_ref().downcast_ref::<FloppyImageFileQuery>()?),
            StructType::FolderFileQuery => Some(from.as_any_ref().downcast_ref::<FolderFileQuery>()?),
            StructType::IsoImageFileQuery => Some(from.as_any_ref().downcast_ref::<IsoImageFileQuery>()?),
            StructType::VmConfigFileQuery => Some(from.as_any_ref().downcast_ref::<VmConfigFileQuery>()?),
            StructType::TemplateConfigFileQuery => Some(from.as_any_ref().downcast_ref::<TemplateConfigFileQuery>()?),
            StructType::VmDiskFileQuery => Some(from.as_any_ref().downcast_ref::<VmDiskFileQuery>()?),
            StructType::VmLogFileQuery => Some(from.as_any_ref().downcast_ref::<VmLogFileQuery>()?),
            StructType::VmNvramFileQuery => Some(from.as_any_ref().downcast_ref::<VmNvramFileQuery>()?),
            StructType::VmSnapshotFileQuery => Some(from.as_any_ref().downcast_ref::<VmSnapshotFileQuery>()?),
            StructType::HostDatastoreBrowserSearchResults => Some(from.as_any_ref().downcast_ref::<HostDatastoreBrowserSearchResults>()?),
            StructType::HostDatastoreBrowserSearchSpec => Some(from.as_any_ref().downcast_ref::<HostDatastoreBrowserSearchSpec>()?),
            StructType::VmConfigFileEncryptionInfo => Some(from.as_any_ref().downcast_ref::<VmConfigFileEncryptionInfo>()?),
            StructType::VmConfigFileQueryFlags => Some(from.as_any_ref().downcast_ref::<VmConfigFileQueryFlags>()?),
            StructType::VmConfigFileQueryFilter => Some(from.as_any_ref().downcast_ref::<VmConfigFileQueryFilter>()?),
            StructType::VmDiskFileEncryptionInfo => Some(from.as_any_ref().downcast_ref::<VmDiskFileEncryptionInfo>()?),
            StructType::VmDiskFileQueryFlags => Some(from.as_any_ref().downcast_ref::<VmDiskFileQueryFlags>()?),
            StructType::VmDiskFileQueryFilter => Some(from.as_any_ref().downcast_ref::<VmDiskFileQueryFilter>()?),
            StructType::HostDatastoreSystemCapabilities => Some(from.as_any_ref().downcast_ref::<HostDatastoreSystemCapabilities>()?),
            StructType::HostDatastoreSystemDatastoreResult => Some(from.as_any_ref().downcast_ref::<HostDatastoreSystemDatastoreResult>()?),
            StructType::HostDatastoreSystemVvolDatastoreSpec => Some(from.as_any_ref().downcast_ref::<HostDatastoreSystemVvolDatastoreSpec>()?),
            StructType::HostDateTimeConfig => Some(from.as_any_ref().downcast_ref::<HostDateTimeConfig>()?),
            StructType::HostDateTimeInfo => Some(from.as_any_ref().downcast_ref::<HostDateTimeInfo>()?),
            StructType::HostDateTimeSystemServiceTestResult => Some(from.as_any_ref().downcast_ref::<HostDateTimeSystemServiceTestResult>()?),
            StructType::HostDateTimeSystemTimeZone => Some(from.as_any_ref().downcast_ref::<HostDateTimeSystemTimeZone>()?),
            StructType::HostDeploymentInfo => Some(from.as_any_ref().downcast_ref::<HostDeploymentInfo>()?),
            StructType::HostDevice => Some(from.as_any_ref().downcast_ref::<HostDevice>()?),
            StructType::ScsiLun => Some(from.as_any_ref().downcast_ref::<ScsiLun>()?),
            StructType::HostScsiDisk => Some(from.as_any_ref().downcast_ref::<HostScsiDisk>()?),
            StructType::HostDhcpService => Some(from.as_any_ref().downcast_ref::<HostDhcpService>()?),
            StructType::HostDhcpServiceConfig => Some(from.as_any_ref().downcast_ref::<HostDhcpServiceConfig>()?),
            StructType::HostDhcpServiceSpec => Some(from.as_any_ref().downcast_ref::<HostDhcpServiceSpec>()?),
            StructType::HostDiagnosticPartition => Some(from.as_any_ref().downcast_ref::<HostDiagnosticPartition>()?),
            StructType::HostDiagnosticPartitionCreateDescription => Some(from.as_any_ref().downcast_ref::<HostDiagnosticPartitionCreateDescription>()?),
            StructType::HostDiagnosticPartitionCreateOption => Some(from.as_any_ref().downcast_ref::<HostDiagnosticPartitionCreateOption>()?),
            StructType::HostDiagnosticPartitionCreateSpec => Some(from.as_any_ref().downcast_ref::<HostDiagnosticPartitionCreateSpec>()?),
            StructType::HostDigestInfo => Some(from.as_any_ref().downcast_ref::<HostDigestInfo>()?),
            StructType::HostTpmDigestInfo => Some(from.as_any_ref().downcast_ref::<HostTpmDigestInfo>()?),
            StructType::HostDiskConfigurationResult => Some(from.as_any_ref().downcast_ref::<HostDiskConfigurationResult>()?),
            StructType::HostDiskDimensions => Some(from.as_any_ref().downcast_ref::<HostDiskDimensions>()?),
            StructType::HostDiskDimensionsChs => Some(from.as_any_ref().downcast_ref::<HostDiskDimensionsChs>()?),
            StructType::HostDiskDimensionsLba => Some(from.as_any_ref().downcast_ref::<HostDiskDimensionsLba>()?),
            StructType::HostDiskPartitionInfo => Some(from.as_any_ref().downcast_ref::<HostDiskPartitionInfo>()?),
            StructType::HostDiskPartitionBlockRange => Some(from.as_any_ref().downcast_ref::<HostDiskPartitionBlockRange>()?),
            StructType::HostDiskPartitionLayout => Some(from.as_any_ref().downcast_ref::<HostDiskPartitionLayout>()?),
            StructType::HostDiskPartitionAttributes => Some(from.as_any_ref().downcast_ref::<HostDiskPartitionAttributes>()?),
            StructType::HostDiskPartitionSpec => Some(from.as_any_ref().downcast_ref::<HostDiskPartitionSpec>()?),
            StructType::HostDnsConfig => Some(from.as_any_ref().downcast_ref::<HostDnsConfig>()?),
            StructType::HostDnsConfigSpec => Some(from.as_any_ref().downcast_ref::<HostDnsConfigSpec>()?),
            StructType::HostDvxClass => Some(from.as_any_ref().downcast_ref::<HostDvxClass>()?),
            StructType::HostEnterMaintenanceResult => Some(from.as_any_ref().downcast_ref::<HostEnterMaintenanceResult>()?),
            StructType::HostEsxAgentHostManagerConfigInfo => Some(from.as_any_ref().downcast_ref::<HostEsxAgentHostManagerConfigInfo>()?),
            StructType::HostFaultToleranceManagerComponentHealthInfo => Some(from.as_any_ref().downcast_ref::<HostFaultToleranceManagerComponentHealthInfo>()?),
            StructType::FcoeConfig => Some(from.as_any_ref().downcast_ref::<FcoeConfig>()?),
            StructType::FcoeConfigFcoeCapabilities => Some(from.as_any_ref().downcast_ref::<FcoeConfigFcoeCapabilities>()?),
            StructType::FcoeConfigFcoeSpecification => Some(from.as_any_ref().downcast_ref::<FcoeConfigFcoeSpecification>()?),
            StructType::FcoeConfigVlanRange => Some(from.as_any_ref().downcast_ref::<FcoeConfigVlanRange>()?),
            StructType::HostFeatureCapability => Some(from.as_any_ref().downcast_ref::<HostFeatureCapability>()?),
            StructType::HostFeatureMask => Some(from.as_any_ref().downcast_ref::<HostFeatureMask>()?),
            StructType::HostFeatureVersionInfo => Some(from.as_any_ref().downcast_ref::<HostFeatureVersionInfo>()?),
            StructType::HostFibreChannelOverEthernetHbaLinkInfo => Some(from.as_any_ref().downcast_ref::<HostFibreChannelOverEthernetHbaLinkInfo>()?),
            StructType::HostFileAccess => Some(from.as_any_ref().downcast_ref::<HostFileAccess>()?),
            StructType::ModeInfo => Some(from.as_any_ref().downcast_ref::<ModeInfo>()?),
            StructType::HostFileSystemMountInfo => Some(from.as_any_ref().downcast_ref::<HostFileSystemMountInfo>()?),
            StructType::HostFileSystemVolume => Some(from.as_any_ref().downcast_ref::<HostFileSystemVolume>()?),
            StructType::HostLocalFileSystemVolume => Some(from.as_any_ref().downcast_ref::<HostLocalFileSystemVolume>()?),
            StructType::HostNasVolume => Some(from.as_any_ref().downcast_ref::<HostNasVolume>()?),
            StructType::HostPMemVolume => Some(from.as_any_ref().downcast_ref::<HostPMemVolume>()?),
            StructType::HostVfatVolume => Some(from.as_any_ref().downcast_ref::<HostVfatVolume>()?),
            StructType::HostVffsVolume => Some(from.as_any_ref().downcast_ref::<HostVffsVolume>()?),
            StructType::HostVmfsVolume => Some(from.as_any_ref().downcast_ref::<HostVmfsVolume>()?),
            StructType::HostVvolVolume => Some(from.as_any_ref().downcast_ref::<HostVvolVolume>()?),
            StructType::HostFileSystemVolumeInfo => Some(from.as_any_ref().downcast_ref::<HostFileSystemVolumeInfo>()?),
            StructType::HostFirewallConfig => Some(from.as_any_ref().downcast_ref::<HostFirewallConfig>()?),
            StructType::HostFirewallConfigRuleSetConfig => Some(from.as_any_ref().downcast_ref::<HostFirewallConfigRuleSetConfig>()?),
            StructType::HostFirewallInfo => Some(from.as_any_ref().downcast_ref::<HostFirewallInfo>()?),
            StructType::HostFirewallDefaultPolicy => Some(from.as_any_ref().downcast_ref::<HostFirewallDefaultPolicy>()?),
            StructType::HostFlagInfo => Some(from.as_any_ref().downcast_ref::<HostFlagInfo>()?),
            StructType::HostForceMountedInfo => Some(from.as_any_ref().downcast_ref::<HostForceMountedInfo>()?),
            StructType::HostFru => Some(from.as_any_ref().downcast_ref::<HostFru>()?),
            StructType::HostGatewaySpec => Some(from.as_any_ref().downcast_ref::<HostGatewaySpec>()?),
            StructType::HostGraphicsConfig => Some(from.as_any_ref().downcast_ref::<HostGraphicsConfig>()?),
            StructType::HostGraphicsConfigDeviceType => Some(from.as_any_ref().downcast_ref::<HostGraphicsConfigDeviceType>()?),
            StructType::HostGraphicsInfo => Some(from.as_any_ref().downcast_ref::<HostGraphicsInfo>()?),
            StructType::HostHardwareInfo => Some(from.as_any_ref().downcast_ref::<HostHardwareInfo>()?),
            StructType::HostHardwareStatusInfo => Some(from.as_any_ref().downcast_ref::<HostHardwareStatusInfo>()?),
            StructType::DpuStatusInfoOperationalInfo => Some(from.as_any_ref().downcast_ref::<DpuStatusInfoOperationalInfo>()?),
            StructType::HostHardwareElementInfo => Some(from.as_any_ref().downcast_ref::<HostHardwareElementInfo>()?),
            StructType::DpuStatusInfo => Some(from.as_any_ref().downcast_ref::<DpuStatusInfo>()?),
            StructType::HostStorageElementInfo => Some(from.as_any_ref().downcast_ref::<HostStorageElementInfo>()?),
            StructType::HostStorageOperationalInfo => Some(from.as_any_ref().downcast_ref::<HostStorageOperationalInfo>()?),
            StructType::HostHbaCreateSpec => Some(from.as_any_ref().downcast_ref::<HostHbaCreateSpec>()?),
            StructType::HostTcpHbaCreateSpec => Some(from.as_any_ref().downcast_ref::<HostTcpHbaCreateSpec>()?),
            StructType::HealthSystemRuntime => Some(from.as_any_ref().downcast_ref::<HealthSystemRuntime>()?),
            StructType::HostAccessControlEntry => Some(from.as_any_ref().downcast_ref::<HostAccessControlEntry>()?),
            StructType::HostHostBusAdapter => Some(from.as_any_ref().downcast_ref::<HostHostBusAdapter>()?),
            StructType::HostBlockHba => Some(from.as_any_ref().downcast_ref::<HostBlockHba>()?),
            StructType::HostFibreChannelHba => Some(from.as_any_ref().downcast_ref::<HostFibreChannelHba>()?),
            StructType::HostFibreChannelOverEthernetHba => Some(from.as_any_ref().downcast_ref::<HostFibreChannelOverEthernetHba>()?),
            StructType::HostInternetScsiHba => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHba>()?),
            StructType::HostParallelScsiHba => Some(from.as_any_ref().downcast_ref::<HostParallelScsiHba>()?),
            StructType::HostPcieHba => Some(from.as_any_ref().downcast_ref::<HostPcieHba>()?),
            StructType::HostRdmaHba => Some(from.as_any_ref().downcast_ref::<HostRdmaHba>()?),
            StructType::HostSerialAttachedHba => Some(from.as_any_ref().downcast_ref::<HostSerialAttachedHba>()?),
            StructType::HostTcpHba => Some(from.as_any_ref().downcast_ref::<HostTcpHba>()?),
            StructType::HostProxySwitch => Some(from.as_any_ref().downcast_ref::<HostProxySwitch>()?),
            StructType::HostProxySwitchConfig => Some(from.as_any_ref().downcast_ref::<HostProxySwitchConfig>()?),
            StructType::HostProxySwitchEnsInfo => Some(from.as_any_ref().downcast_ref::<HostProxySwitchEnsInfo>()?),
            StructType::HostProxySwitchHostLagConfig => Some(from.as_any_ref().downcast_ref::<HostProxySwitchHostLagConfig>()?),
            StructType::HostProxySwitchSpec => Some(from.as_any_ref().downcast_ref::<HostProxySwitchSpec>()?),
            StructType::HostImageProfileSummary => Some(from.as_any_ref().downcast_ref::<HostImageProfileSummary>()?),
            StructType::HostInternetScsiHbaAuthenticationCapabilities => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaAuthenticationCapabilities>()?),
            StructType::HostInternetScsiHbaAuthenticationProperties => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaAuthenticationProperties>()?),
            StructType::HostInternetScsiHbaDigestCapabilities => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaDigestCapabilities>()?),
            StructType::HostInternetScsiHbaDigestProperties => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaDigestProperties>()?),
            StructType::HostInternetScsiHbaDiscoveryCapabilities => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaDiscoveryCapabilities>()?),
            StructType::HostInternetScsiHbaDiscoveryProperties => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaDiscoveryProperties>()?),
            StructType::HostInternetScsiHbaIpCapabilities => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaIpCapabilities>()?),
            StructType::HostInternetScsiHbaIpProperties => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaIpProperties>()?),
            StructType::HostInternetScsiHbaIPv6Properties => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaIPv6Properties>()?),
            StructType::HostInternetScsiHbaIscsiIpv6Address => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaIscsiIpv6Address>()?),
            StructType::HostInternetScsiHbaSendTarget => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaSendTarget>()?),
            StructType::HostInternetScsiHbaStaticTarget => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaStaticTarget>()?),
            StructType::HostInternetScsiHbaTargetSet => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaTargetSet>()?),
            StructType::HostIpConfig => Some(from.as_any_ref().downcast_ref::<HostIpConfig>()?),
            StructType::HostIpConfigIpV6Address => Some(from.as_any_ref().downcast_ref::<HostIpConfigIpV6Address>()?),
            StructType::HostIpConfigIpV6AddressConfiguration => Some(from.as_any_ref().downcast_ref::<HostIpConfigIpV6AddressConfiguration>()?),
            StructType::HostIpRouteConfig => Some(from.as_any_ref().downcast_ref::<HostIpRouteConfig>()?),
            StructType::HostIpRouteConfigSpec => Some(from.as_any_ref().downcast_ref::<HostIpRouteConfigSpec>()?),
            StructType::HostIpRouteEntry => Some(from.as_any_ref().downcast_ref::<HostIpRouteEntry>()?),
            StructType::HostIpRouteOp => Some(from.as_any_ref().downcast_ref::<HostIpRouteOp>()?),
            StructType::HostIpRouteTableConfig => Some(from.as_any_ref().downcast_ref::<HostIpRouteTableConfig>()?),
            StructType::HostIpRouteTableInfo => Some(from.as_any_ref().downcast_ref::<HostIpRouteTableInfo>()?),
            StructType::HostIpmiInfo => Some(from.as_any_ref().downcast_ref::<HostIpmiInfo>()?),
            StructType::IscsiDependencyEntity => Some(from.as_any_ref().downcast_ref::<IscsiDependencyEntity>()?),
            StructType::IscsiMigrationDependency => Some(from.as_any_ref().downcast_ref::<IscsiMigrationDependency>()?),
            StructType::IscsiPortInfo => Some(from.as_any_ref().downcast_ref::<IscsiPortInfo>()?),
            StructType::IscsiStatus => Some(from.as_any_ref().downcast_ref::<IscsiStatus>()?),
            StructType::KernelModuleInfo => Some(from.as_any_ref().downcast_ref::<KernelModuleInfo>()?),
            StructType::KernelModuleSectionInfo => Some(from.as_any_ref().downcast_ref::<KernelModuleSectionInfo>()?),
            StructType::HostLicenseSpec => Some(from.as_any_ref().downcast_ref::<HostLicenseSpec>()?),
            StructType::LinkDiscoveryProtocolConfig => Some(from.as_any_ref().downcast_ref::<LinkDiscoveryProtocolConfig>()?),
            StructType::HostAccountSpec => Some(from.as_any_ref().downcast_ref::<HostAccountSpec>()?),
            StructType::HostPosixAccountSpec => Some(from.as_any_ref().downcast_ref::<HostPosixAccountSpec>()?),
            StructType::HostLocalFileSystemVolumeSpec => Some(from.as_any_ref().downcast_ref::<HostLocalFileSystemVolumeSpec>()?),
            StructType::HostLowLevelProvisioningManagerDiskLayoutSpec => Some(from.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerDiskLayoutSpec>()?),
            StructType::HostLowLevelProvisioningManagerFileDeleteResult => Some(from.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerFileDeleteResult>()?),
            StructType::HostLowLevelProvisioningManagerFileDeleteSpec => Some(from.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerFileDeleteSpec>()?),
            StructType::HostLowLevelProvisioningManagerFileReserveResult => Some(from.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerFileReserveResult>()?),
            StructType::HostLowLevelProvisioningManagerFileReserveSpec => Some(from.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerFileReserveSpec>()?),
            StructType::HostLowLevelProvisioningManagerSnapshotLayoutSpec => Some(from.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerSnapshotLayoutSpec>()?),
            StructType::HostLowLevelProvisioningManagerVmMigrationStatus => Some(from.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerVmMigrationStatus>()?),
            StructType::HostLowLevelProvisioningManagerVmRecoveryInfo => Some(from.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerVmRecoveryInfo>()?),
            StructType::HostMaintenanceSpec => Some(from.as_any_ref().downcast_ref::<HostMaintenanceSpec>()?),
            StructType::ServiceConsoleReservationInfo => Some(from.as_any_ref().downcast_ref::<ServiceConsoleReservationInfo>()?),
            StructType::VirtualMachineMemoryReservationInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineMemoryReservationInfo>()?),
            StructType::VirtualMachineMemoryReservationSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineMemoryReservationSpec>()?),
            StructType::HostMemorySpec => Some(from.as_any_ref().downcast_ref::<HostMemorySpec>()?),
            StructType::HostMemoryTierInfo => Some(from.as_any_ref().downcast_ref::<HostMemoryTierInfo>()?),
            StructType::HostMountInfo => Some(from.as_any_ref().downcast_ref::<HostMountInfo>()?),
            StructType::HostMultipathInfo => Some(from.as_any_ref().downcast_ref::<HostMultipathInfo>()?),
            StructType::HostMultipathInfoLogicalUnit => Some(from.as_any_ref().downcast_ref::<HostMultipathInfoLogicalUnit>()?),
            StructType::HostMultipathInfoLogicalUnitPolicy => Some(from.as_any_ref().downcast_ref::<HostMultipathInfoLogicalUnitPolicy>()?),
            StructType::HostMultipathInfoFixedLogicalUnitPolicy => Some(from.as_any_ref().downcast_ref::<HostMultipathInfoFixedLogicalUnitPolicy>()?),
            StructType::HostMultipathInfoHppLogicalUnitPolicy => Some(from.as_any_ref().downcast_ref::<HostMultipathInfoHppLogicalUnitPolicy>()?),
            StructType::HostMultipathInfoLogicalUnitStorageArrayTypePolicy => Some(from.as_any_ref().downcast_ref::<HostMultipathInfoLogicalUnitStorageArrayTypePolicy>()?),
            StructType::HostMultipathInfoPath => Some(from.as_any_ref().downcast_ref::<HostMultipathInfoPath>()?),
            StructType::HostMultipathStateInfo => Some(from.as_any_ref().downcast_ref::<HostMultipathStateInfo>()?),
            StructType::HostMultipathStateInfoPath => Some(from.as_any_ref().downcast_ref::<HostMultipathStateInfoPath>()?),
            StructType::HostNasVolumeConfig => Some(from.as_any_ref().downcast_ref::<HostNasVolumeConfig>()?),
            StructType::HostNasVolumeSpec => Some(from.as_any_ref().downcast_ref::<HostNasVolumeSpec>()?),
            StructType::HostNasVolumeUserInfo => Some(from.as_any_ref().downcast_ref::<HostNasVolumeUserInfo>()?),
            StructType::HostNatService => Some(from.as_any_ref().downcast_ref::<HostNatService>()?),
            StructType::HostNatServiceConfig => Some(from.as_any_ref().downcast_ref::<HostNatServiceConfig>()?),
            StructType::HostNatServiceNameServiceSpec => Some(from.as_any_ref().downcast_ref::<HostNatServiceNameServiceSpec>()?),
            StructType::HostNatServicePortForwardSpec => Some(from.as_any_ref().downcast_ref::<HostNatServicePortForwardSpec>()?),
            StructType::HostNatServiceSpec => Some(from.as_any_ref().downcast_ref::<HostNatServiceSpec>()?),
            StructType::HostNetCapabilities => Some(from.as_any_ref().downcast_ref::<HostNetCapabilities>()?),
            StructType::HostNetOffloadCapabilities => Some(from.as_any_ref().downcast_ref::<HostNetOffloadCapabilities>()?),
            StructType::HostNetStackInstance => Some(from.as_any_ref().downcast_ref::<HostNetStackInstance>()?),
            StructType::HostNetworkConfig => Some(from.as_any_ref().downcast_ref::<HostNetworkConfig>()?),
            StructType::HostNetworkConfigNetStackSpec => Some(from.as_any_ref().downcast_ref::<HostNetworkConfigNetStackSpec>()?),
            StructType::HostNetworkConfigResult => Some(from.as_any_ref().downcast_ref::<HostNetworkConfigResult>()?),
            StructType::HostNetworkInfo => Some(from.as_any_ref().downcast_ref::<HostNetworkInfo>()?),
            StructType::HostNetworkPolicy => Some(from.as_any_ref().downcast_ref::<HostNetworkPolicy>()?),
            StructType::HostNicFailureCriteria => Some(from.as_any_ref().downcast_ref::<HostNicFailureCriteria>()?),
            StructType::HostNicOrderPolicy => Some(from.as_any_ref().downcast_ref::<HostNicOrderPolicy>()?),
            StructType::HostNicTeamingPolicy => Some(from.as_any_ref().downcast_ref::<HostNicTeamingPolicy>()?),
            StructType::HostNetworkSecurityPolicy => Some(from.as_any_ref().downcast_ref::<HostNetworkSecurityPolicy>()?),
            StructType::HostNetworkTrafficShapingPolicy => Some(from.as_any_ref().downcast_ref::<HostNetworkTrafficShapingPolicy>()?),
            StructType::HostNtpConfig => Some(from.as_any_ref().downcast_ref::<HostNtpConfig>()?),
            StructType::HostNumaInfo => Some(from.as_any_ref().downcast_ref::<HostNumaInfo>()?),
            StructType::HostNumaNode => Some(from.as_any_ref().downcast_ref::<HostNumaNode>()?),
            StructType::HostNumericSensorInfo => Some(from.as_any_ref().downcast_ref::<HostNumericSensorInfo>()?),
            StructType::NvdimmDimmInfo => Some(from.as_any_ref().downcast_ref::<NvdimmDimmInfo>()?),
            StructType::NvdimmGuid => Some(from.as_any_ref().downcast_ref::<NvdimmGuid>()?),
            StructType::NvdimmHealthInfo => Some(from.as_any_ref().downcast_ref::<NvdimmHealthInfo>()?),
            StructType::NvdimmInterleaveSetInfo => Some(from.as_any_ref().downcast_ref::<NvdimmInterleaveSetInfo>()?),
            StructType::NvdimmNamespaceCreateSpec => Some(from.as_any_ref().downcast_ref::<NvdimmNamespaceCreateSpec>()?),
            StructType::NvdimmNamespaceDeleteSpec => Some(from.as_any_ref().downcast_ref::<NvdimmNamespaceDeleteSpec>()?),
            StructType::NvdimmNamespaceDetails => Some(from.as_any_ref().downcast_ref::<NvdimmNamespaceDetails>()?),
            StructType::NvdimmNamespaceInfo => Some(from.as_any_ref().downcast_ref::<NvdimmNamespaceInfo>()?),
            StructType::NvdimmSystemInfo => Some(from.as_any_ref().downcast_ref::<NvdimmSystemInfo>()?),
            StructType::NvdimmPMemNamespaceCreateSpec => Some(from.as_any_ref().downcast_ref::<NvdimmPMemNamespaceCreateSpec>()?),
            StructType::NvdimmRegionInfo => Some(from.as_any_ref().downcast_ref::<NvdimmRegionInfo>()?),
            StructType::NvdimmSummary => Some(from.as_any_ref().downcast_ref::<NvdimmSummary>()?),
            StructType::HostNvmeController => Some(from.as_any_ref().downcast_ref::<HostNvmeController>()?),
            StructType::HostNvmeDisconnectSpec => Some(from.as_any_ref().downcast_ref::<HostNvmeDisconnectSpec>()?),
            StructType::HostNvmeDiscoveryLog => Some(from.as_any_ref().downcast_ref::<HostNvmeDiscoveryLog>()?),
            StructType::HostNvmeDiscoveryLogEntry => Some(from.as_any_ref().downcast_ref::<HostNvmeDiscoveryLogEntry>()?),
            StructType::HostNvmeNamespace => Some(from.as_any_ref().downcast_ref::<HostNvmeNamespace>()?),
            StructType::HostNvmeSpec => Some(from.as_any_ref().downcast_ref::<HostNvmeSpec>()?),
            StructType::HostNvmeConnectSpec => Some(from.as_any_ref().downcast_ref::<HostNvmeConnectSpec>()?),
            StructType::HostNvmeDiscoverSpec => Some(from.as_any_ref().downcast_ref::<HostNvmeDiscoverSpec>()?),
            StructType::HostNvmeTopology => Some(from.as_any_ref().downcast_ref::<HostNvmeTopology>()?),
            StructType::HostNvmeTopologyInterface => Some(from.as_any_ref().downcast_ref::<HostNvmeTopologyInterface>()?),
            StructType::HostNvmeTransportParameters => Some(from.as_any_ref().downcast_ref::<HostNvmeTransportParameters>()?),
            StructType::HostNvmeOpaqueTransportParameters => Some(from.as_any_ref().downcast_ref::<HostNvmeOpaqueTransportParameters>()?),
            StructType::HostNvmeOverFibreChannelParameters => Some(from.as_any_ref().downcast_ref::<HostNvmeOverFibreChannelParameters>()?),
            StructType::HostNvmeOverRdmaParameters => Some(from.as_any_ref().downcast_ref::<HostNvmeOverRdmaParameters>()?),
            StructType::HostNvmeOverTcpParameters => Some(from.as_any_ref().downcast_ref::<HostNvmeOverTcpParameters>()?),
            StructType::HostOpaqueNetworkInfo => Some(from.as_any_ref().downcast_ref::<HostOpaqueNetworkInfo>()?),
            StructType::HostOpaqueSwitch => Some(from.as_any_ref().downcast_ref::<HostOpaqueSwitch>()?),
            StructType::HostOpaqueSwitchPhysicalNicZone => Some(from.as_any_ref().downcast_ref::<HostOpaqueSwitchPhysicalNicZone>()?),
            StructType::HostPatchManagerLocator => Some(from.as_any_ref().downcast_ref::<HostPatchManagerLocator>()?),
            StructType::HostPatchManagerPatchManagerOperationSpec => Some(from.as_any_ref().downcast_ref::<HostPatchManagerPatchManagerOperationSpec>()?),
            StructType::HostPatchManagerResult => Some(from.as_any_ref().downcast_ref::<HostPatchManagerResult>()?),
            StructType::HostPatchManagerStatus => Some(from.as_any_ref().downcast_ref::<HostPatchManagerStatus>()?),
            StructType::HostPatchManagerStatusPrerequisitePatch => Some(from.as_any_ref().downcast_ref::<HostPatchManagerStatusPrerequisitePatch>()?),
            StructType::HostPathSelectionPolicyOption => Some(from.as_any_ref().downcast_ref::<HostPathSelectionPolicyOption>()?),
            StructType::HostPciDevice => Some(from.as_any_ref().downcast_ref::<HostPciDevice>()?),
            StructType::HostPciPassthruConfig => Some(from.as_any_ref().downcast_ref::<HostPciPassthruConfig>()?),
            StructType::HostSriovConfig => Some(from.as_any_ref().downcast_ref::<HostSriovConfig>()?),
            StructType::HostPciPassthruInfo => Some(from.as_any_ref().downcast_ref::<HostPciPassthruInfo>()?),
            StructType::HostSriovInfo => Some(from.as_any_ref().downcast_ref::<HostSriovInfo>()?),
            StructType::HostPersistentMemoryInfo => Some(from.as_any_ref().downcast_ref::<HostPersistentMemoryInfo>()?),
            StructType::PhysicalNic => Some(from.as_any_ref().downcast_ref::<PhysicalNic>()?),
            StructType::PhysicalNicCdpDeviceCapability => Some(from.as_any_ref().downcast_ref::<PhysicalNicCdpDeviceCapability>()?),
            StructType::PhysicalNicCdpInfo => Some(from.as_any_ref().downcast_ref::<PhysicalNicCdpInfo>()?),
            StructType::PhysicalNicConfig => Some(from.as_any_ref().downcast_ref::<PhysicalNicConfig>()?),
            StructType::PhysicalNicLinkInfo => Some(from.as_any_ref().downcast_ref::<PhysicalNicLinkInfo>()?),
            StructType::LinkLayerDiscoveryProtocolInfo => Some(from.as_any_ref().downcast_ref::<LinkLayerDiscoveryProtocolInfo>()?),
            StructType::PhysicalNicHintInfo => Some(from.as_any_ref().downcast_ref::<PhysicalNicHintInfo>()?),
            StructType::PhysicalNicHint => Some(from.as_any_ref().downcast_ref::<PhysicalNicHint>()?),
            StructType::PhysicalNicIpHint => Some(from.as_any_ref().downcast_ref::<PhysicalNicIpHint>()?),
            StructType::PhysicalNicNameHint => Some(from.as_any_ref().downcast_ref::<PhysicalNicNameHint>()?),
            StructType::PhysicalNicSpec => Some(from.as_any_ref().downcast_ref::<PhysicalNicSpec>()?),
            StructType::HostPlugStoreTopology => Some(from.as_any_ref().downcast_ref::<HostPlugStoreTopology>()?),
            StructType::HostPlugStoreTopologyAdapter => Some(from.as_any_ref().downcast_ref::<HostPlugStoreTopologyAdapter>()?),
            StructType::HostPlugStoreTopologyDevice => Some(from.as_any_ref().downcast_ref::<HostPlugStoreTopologyDevice>()?),
            StructType::HostPlugStoreTopologyPath => Some(from.as_any_ref().downcast_ref::<HostPlugStoreTopologyPath>()?),
            StructType::HostPlugStoreTopologyPlugin => Some(from.as_any_ref().downcast_ref::<HostPlugStoreTopologyPlugin>()?),
            StructType::HostPlugStoreTopologyTarget => Some(from.as_any_ref().downcast_ref::<HostPlugStoreTopologyTarget>()?),
            StructType::HostPortGroup => Some(from.as_any_ref().downcast_ref::<HostPortGroup>()?),
            StructType::HostPortGroupConfig => Some(from.as_any_ref().downcast_ref::<HostPortGroupConfig>()?),
            StructType::HostPortGroupPort => Some(from.as_any_ref().downcast_ref::<HostPortGroupPort>()?),
            StructType::HostPortGroupSpec => Some(from.as_any_ref().downcast_ref::<HostPortGroupSpec>()?),
            StructType::PowerSystemCapability => Some(from.as_any_ref().downcast_ref::<PowerSystemCapability>()?),
            StructType::PowerSystemInfo => Some(from.as_any_ref().downcast_ref::<PowerSystemInfo>()?),
            StructType::HostPowerPolicy => Some(from.as_any_ref().downcast_ref::<HostPowerPolicy>()?),
            StructType::HostProtocolEndpoint => Some(from.as_any_ref().downcast_ref::<HostProtocolEndpoint>()?),
            StructType::HostPtpConfig => Some(from.as_any_ref().downcast_ref::<HostPtpConfig>()?),
            StructType::HostPtpConfigPtpPort => Some(from.as_any_ref().downcast_ref::<HostPtpConfigPtpPort>()?),
            StructType::HostQualifiedName => Some(from.as_any_ref().downcast_ref::<HostQualifiedName>()?),
            StructType::HostRdmaDevice => Some(from.as_any_ref().downcast_ref::<HostRdmaDevice>()?),
            StructType::HostRdmaDeviceBacking => Some(from.as_any_ref().downcast_ref::<HostRdmaDeviceBacking>()?),
            StructType::HostRdmaDevicePnicBacking => Some(from.as_any_ref().downcast_ref::<HostRdmaDevicePnicBacking>()?),
            StructType::HostRdmaDeviceCapability => Some(from.as_any_ref().downcast_ref::<HostRdmaDeviceCapability>()?),
            StructType::HostRdmaDeviceConnectionInfo => Some(from.as_any_ref().downcast_ref::<HostRdmaDeviceConnectionInfo>()?),
            StructType::HostReliableMemoryInfo => Some(from.as_any_ref().downcast_ref::<HostReliableMemoryInfo>()?),
            StructType::HostResignatureRescanResult => Some(from.as_any_ref().downcast_ref::<HostResignatureRescanResult>()?),
            StructType::HostFirewallRuleset => Some(from.as_any_ref().downcast_ref::<HostFirewallRuleset>()?),
            StructType::HostFirewallRulesetIpList => Some(from.as_any_ref().downcast_ref::<HostFirewallRulesetIpList>()?),
            StructType::HostFirewallRulesetIpNetwork => Some(from.as_any_ref().downcast_ref::<HostFirewallRulesetIpNetwork>()?),
            StructType::HostFirewallRule => Some(from.as_any_ref().downcast_ref::<HostFirewallRule>()?),
            StructType::HostFirewallRulesetRulesetSpec => Some(from.as_any_ref().downcast_ref::<HostFirewallRulesetRulesetSpec>()?),
            StructType::HostRuntimeInfo => Some(from.as_any_ref().downcast_ref::<HostRuntimeInfo>()?),
            StructType::HostRuntimeInfoNetStackInstanceRuntimeInfo => Some(from.as_any_ref().downcast_ref::<HostRuntimeInfoNetStackInstanceRuntimeInfo>()?),
            StructType::HostNetworkResourceRuntime => Some(from.as_any_ref().downcast_ref::<HostNetworkResourceRuntime>()?),
            StructType::HostRuntimeInfoNetworkRuntimeInfo => Some(from.as_any_ref().downcast_ref::<HostRuntimeInfoNetworkRuntimeInfo>()?),
            StructType::HostPlacedVirtualNicIdentifier => Some(from.as_any_ref().downcast_ref::<HostPlacedVirtualNicIdentifier>()?),
            StructType::HostPnicNetworkResourceInfo => Some(from.as_any_ref().downcast_ref::<HostPnicNetworkResourceInfo>()?),
            StructType::HostRuntimeInfoStateEncryptionInfo => Some(from.as_any_ref().downcast_ref::<HostRuntimeInfoStateEncryptionInfo>()?),
            StructType::HostScsiDiskPartition => Some(from.as_any_ref().downcast_ref::<HostScsiDiskPartition>()?),
            StructType::ScsiLunCapabilities => Some(from.as_any_ref().downcast_ref::<ScsiLunCapabilities>()?),
            StructType::ScsiLunDescriptor => Some(from.as_any_ref().downcast_ref::<ScsiLunDescriptor>()?),
            StructType::ScsiLunDurableName => Some(from.as_any_ref().downcast_ref::<ScsiLunDurableName>()?),
            StructType::HostScsiTopology => Some(from.as_any_ref().downcast_ref::<HostScsiTopology>()?),
            StructType::HostScsiTopologyInterface => Some(from.as_any_ref().downcast_ref::<HostScsiTopologyInterface>()?),
            StructType::HostScsiTopologyLun => Some(from.as_any_ref().downcast_ref::<HostScsiTopologyLun>()?),
            StructType::HostScsiTopologyTarget => Some(from.as_any_ref().downcast_ref::<HostScsiTopologyTarget>()?),
            StructType::HostSecuritySpec => Some(from.as_any_ref().downcast_ref::<HostSecuritySpec>()?),
            StructType::HostService => Some(from.as_any_ref().downcast_ref::<HostService>()?),
            StructType::HostServiceSourcePackage => Some(from.as_any_ref().downcast_ref::<HostServiceSourcePackage>()?),
            StructType::HostServiceConfig => Some(from.as_any_ref().downcast_ref::<HostServiceConfig>()?),
            StructType::HostServiceInfo => Some(from.as_any_ref().downcast_ref::<HostServiceInfo>()?),
            StructType::HostSevInfo => Some(from.as_any_ref().downcast_ref::<HostSevInfo>()?),
            StructType::HostSgxInfo => Some(from.as_any_ref().downcast_ref::<HostSgxInfo>()?),
            StructType::HostSgxRegistrationInfo => Some(from.as_any_ref().downcast_ref::<HostSgxRegistrationInfo>()?),
            StructType::HostSharedGpuCapabilities => Some(from.as_any_ref().downcast_ref::<HostSharedGpuCapabilities>()?),
            StructType::HostSnmpSystemAgentLimits => Some(from.as_any_ref().downcast_ref::<HostSnmpSystemAgentLimits>()?),
            StructType::HostSnmpConfigSpec => Some(from.as_any_ref().downcast_ref::<HostSnmpConfigSpec>()?),
            StructType::HostSnmpDestination => Some(from.as_any_ref().downcast_ref::<HostSnmpDestination>()?),
            StructType::SoftwarePackage => Some(from.as_any_ref().downcast_ref::<SoftwarePackage>()?),
            StructType::SoftwarePackageCapability => Some(from.as_any_ref().downcast_ref::<SoftwarePackageCapability>()?),
            StructType::Relation => Some(from.as_any_ref().downcast_ref::<Relation>()?),
            StructType::HostSriovDevicePoolInfo => Some(from.as_any_ref().downcast_ref::<HostSriovDevicePoolInfo>()?),
            StructType::HostSriovNetworkDevicePoolInfo => Some(from.as_any_ref().downcast_ref::<HostSriovNetworkDevicePoolInfo>()?),
            StructType::HostSslThumbprintInfo => Some(from.as_any_ref().downcast_ref::<HostSslThumbprintInfo>()?),
            StructType::HostStorageArrayTypePolicyOption => Some(from.as_any_ref().downcast_ref::<HostStorageArrayTypePolicyOption>()?),
            StructType::HostStorageDeviceInfo => Some(from.as_any_ref().downcast_ref::<HostStorageDeviceInfo>()?),
            StructType::HostStorageSystemDiskLocatorLedResult => Some(from.as_any_ref().downcast_ref::<HostStorageSystemDiskLocatorLedResult>()?),
            StructType::HostStorageSystemScsiLunResult => Some(from.as_any_ref().downcast_ref::<HostStorageSystemScsiLunResult>()?),
            StructType::HostStorageSystemVmfsVolumeResult => Some(from.as_any_ref().downcast_ref::<HostStorageSystemVmfsVolumeResult>()?),
            StructType::HostListSummary => Some(from.as_any_ref().downcast_ref::<HostListSummary>()?),
            StructType::HostConfigSummary => Some(from.as_any_ref().downcast_ref::<HostConfigSummary>()?),
            StructType::HostListSummaryGatewaySummary => Some(from.as_any_ref().downcast_ref::<HostListSummaryGatewaySummary>()?),
            StructType::HostHardwareSummary => Some(from.as_any_ref().downcast_ref::<HostHardwareSummary>()?),
            StructType::HostListSummaryQuickStats => Some(from.as_any_ref().downcast_ref::<HostListSummaryQuickStats>()?),
            StructType::SystemEventInfo => Some(from.as_any_ref().downcast_ref::<SystemEventInfo>()?),
            StructType::HostSystemHealthInfo => Some(from.as_any_ref().downcast_ref::<HostSystemHealthInfo>()?),
            StructType::HostSystemIdentificationInfo => Some(from.as_any_ref().downcast_ref::<HostSystemIdentificationInfo>()?),
            StructType::HostSystemInfo => Some(from.as_any_ref().downcast_ref::<HostSystemInfo>()?),
            StructType::HostSystemResourceInfo => Some(from.as_any_ref().downcast_ref::<HostSystemResourceInfo>()?),
            StructType::HostSystemSwapConfiguration => Some(from.as_any_ref().downcast_ref::<HostSystemSwapConfiguration>()?),
            StructType::HostSystemSwapConfigurationSystemSwapOption => Some(from.as_any_ref().downcast_ref::<HostSystemSwapConfigurationSystemSwapOption>()?),
            StructType::HostSystemSwapConfigurationDatastoreOption => Some(from.as_any_ref().downcast_ref::<HostSystemSwapConfigurationDatastoreOption>()?),
            StructType::HostSystemSwapConfigurationDisabledOption => Some(from.as_any_ref().downcast_ref::<HostSystemSwapConfigurationDisabledOption>()?),
            StructType::HostSystemSwapConfigurationHostCacheOption => Some(from.as_any_ref().downcast_ref::<HostSystemSwapConfigurationHostCacheOption>()?),
            StructType::HostSystemSwapConfigurationHostLocalSwapOption => Some(from.as_any_ref().downcast_ref::<HostSystemSwapConfigurationHostLocalSwapOption>()?),
            StructType::HostTargetTransport => Some(from.as_any_ref().downcast_ref::<HostTargetTransport>()?),
            StructType::HostBlockAdapterTargetTransport => Some(from.as_any_ref().downcast_ref::<HostBlockAdapterTargetTransport>()?),
            StructType::HostFibreChannelTargetTransport => Some(from.as_any_ref().downcast_ref::<HostFibreChannelTargetTransport>()?),
            StructType::HostFibreChannelOverEthernetTargetTransport => Some(from.as_any_ref().downcast_ref::<HostFibreChannelOverEthernetTargetTransport>()?),
            StructType::HostInternetScsiTargetTransport => Some(from.as_any_ref().downcast_ref::<HostInternetScsiTargetTransport>()?),
            StructType::HostParallelScsiTargetTransport => Some(from.as_any_ref().downcast_ref::<HostParallelScsiTargetTransport>()?),
            StructType::HostPcieTargetTransport => Some(from.as_any_ref().downcast_ref::<HostPcieTargetTransport>()?),
            StructType::HostRdmaTargetTransport => Some(from.as_any_ref().downcast_ref::<HostRdmaTargetTransport>()?),
            StructType::HostSerialAttachedTargetTransport => Some(from.as_any_ref().downcast_ref::<HostSerialAttachedTargetTransport>()?),
            StructType::HostTcpTargetTransport => Some(from.as_any_ref().downcast_ref::<HostTcpTargetTransport>()?),
            StructType::HostTpmAttestationInfo => Some(from.as_any_ref().downcast_ref::<HostTpmAttestationInfo>()?),
            StructType::HostTpmAttestationReport => Some(from.as_any_ref().downcast_ref::<HostTpmAttestationReport>()?),
            StructType::HostTpmEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmEventDetails>()?),
            StructType::HostTpmBootCompleteEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmBootCompleteEventDetails>()?),
            StructType::HostTpmBootSecurityOptionEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmBootSecurityOptionEventDetails>()?),
            StructType::HostTpmNvTagEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmNvTagEventDetails>()?),
            StructType::HostTpmSignerEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmSignerEventDetails>()?),
            StructType::HostTpmCommandEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmCommandEventDetails>()?),
            StructType::HostTpmOptionEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmOptionEventDetails>()?),
            StructType::HostTpmSoftwareComponentEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmSoftwareComponentEventDetails>()?),
            StructType::HostTpmVersionEventDetails => Some(from.as_any_ref().downcast_ref::<HostTpmVersionEventDetails>()?),
            StructType::HostTpmEventLogEntry => Some(from.as_any_ref().downcast_ref::<HostTpmEventLogEntry>()?),
            StructType::HostTrustAuthorityAttestationInfo => Some(from.as_any_ref().downcast_ref::<HostTrustAuthorityAttestationInfo>()?),
            StructType::HostUnresolvedVmfsExtent => Some(from.as_any_ref().downcast_ref::<HostUnresolvedVmfsExtent>()?),
            StructType::HostUnresolvedVmfsResignatureSpec => Some(from.as_any_ref().downcast_ref::<HostUnresolvedVmfsResignatureSpec>()?),
            StructType::HostUnresolvedVmfsResolutionResult => Some(from.as_any_ref().downcast_ref::<HostUnresolvedVmfsResolutionResult>()?),
            StructType::HostUnresolvedVmfsResolutionSpec => Some(from.as_any_ref().downcast_ref::<HostUnresolvedVmfsResolutionSpec>()?),
            StructType::HostUnresolvedVmfsVolume => Some(from.as_any_ref().downcast_ref::<HostUnresolvedVmfsVolume>()?),
            StructType::HostUnresolvedVmfsVolumeResolveStatus => Some(from.as_any_ref().downcast_ref::<HostUnresolvedVmfsVolumeResolveStatus>()?),
            StructType::HostVFlashManagerVFlashCacheConfigInfo => Some(from.as_any_ref().downcast_ref::<HostVFlashManagerVFlashCacheConfigInfo>()?),
            StructType::HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption => Some(from.as_any_ref().downcast_ref::<HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption>()?),
            StructType::HostVFlashManagerVFlashCacheConfigSpec => Some(from.as_any_ref().downcast_ref::<HostVFlashManagerVFlashCacheConfigSpec>()?),
            StructType::HostVFlashManagerVFlashConfigInfo => Some(from.as_any_ref().downcast_ref::<HostVFlashManagerVFlashConfigInfo>()?),
            StructType::HostVFlashManagerVFlashResourceConfigInfo => Some(from.as_any_ref().downcast_ref::<HostVFlashManagerVFlashResourceConfigInfo>()?),
            StructType::HostVFlashManagerVFlashResourceConfigSpec => Some(from.as_any_ref().downcast_ref::<HostVFlashManagerVFlashResourceConfigSpec>()?),
            StructType::HostVFlashManagerVFlashResourceRunTimeInfo => Some(from.as_any_ref().downcast_ref::<HostVFlashManagerVFlashResourceRunTimeInfo>()?),
            StructType::HostVFlashResourceConfigurationResult => Some(from.as_any_ref().downcast_ref::<HostVFlashResourceConfigurationResult>()?),
            StructType::HostVMotionConfig => Some(from.as_any_ref().downcast_ref::<HostVMotionConfig>()?),
            StructType::HostVMotionInfo => Some(from.as_any_ref().downcast_ref::<HostVMotionInfo>()?),
            StructType::HostVMotionManagerDstInstantCloneResult => Some(from.as_any_ref().downcast_ref::<HostVMotionManagerDstInstantCloneResult>()?),
            StructType::HostVMotionManagerSrcInstantCloneResult => Some(from.as_any_ref().downcast_ref::<HostVMotionManagerSrcInstantCloneResult>()?),
            StructType::HostVMotionNetConfig => Some(from.as_any_ref().downcast_ref::<HostVMotionNetConfig>()?),
            StructType::HostVffsSpec => Some(from.as_any_ref().downcast_ref::<HostVffsSpec>()?),
            StructType::HostVirtualNic => Some(from.as_any_ref().downcast_ref::<HostVirtualNic>()?),
            StructType::HostVirtualNicConfig => Some(from.as_any_ref().downcast_ref::<HostVirtualNicConfig>()?),
            StructType::HostVirtualNicIpRouteSpec => Some(from.as_any_ref().downcast_ref::<HostVirtualNicIpRouteSpec>()?),
            StructType::HostVirtualNicOpaqueNetworkSpec => Some(from.as_any_ref().downcast_ref::<HostVirtualNicOpaqueNetworkSpec>()?),
            StructType::HostVirtualNicSpec => Some(from.as_any_ref().downcast_ref::<HostVirtualNicSpec>()?),
            StructType::HostVirtualNicConnection => Some(from.as_any_ref().downcast_ref::<HostVirtualNicConnection>()?),
            StructType::VirtualNicManagerNetConfig => Some(from.as_any_ref().downcast_ref::<VirtualNicManagerNetConfig>()?),
            StructType::HostVirtualNicManagerNicTypeSelection => Some(from.as_any_ref().downcast_ref::<HostVirtualNicManagerNicTypeSelection>()?),
            StructType::HostVirtualNicManagerInfo => Some(from.as_any_ref().downcast_ref::<HostVirtualNicManagerInfo>()?),
            StructType::HostVirtualSwitch => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitch>()?),
            StructType::HostVirtualSwitchBeaconConfig => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitchBeaconConfig>()?),
            StructType::HostVirtualSwitchBridge => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitchBridge>()?),
            StructType::HostVirtualSwitchAutoBridge => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitchAutoBridge>()?),
            StructType::HostVirtualSwitchBondBridge => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitchBondBridge>()?),
            StructType::HostVirtualSwitchSimpleBridge => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitchSimpleBridge>()?),
            StructType::HostVirtualSwitchConfig => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitchConfig>()?),
            StructType::HostVirtualSwitchSpec => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitchSpec>()?),
            StructType::HostVmciAccessManagerAccessSpec => Some(from.as_any_ref().downcast_ref::<HostVmciAccessManagerAccessSpec>()?),
            StructType::VmfsDatastoreOption => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreOption>()?),
            StructType::VmfsDatastoreBaseOption => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreBaseOption>()?),
            StructType::VmfsDatastoreMultipleExtentOption => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreMultipleExtentOption>()?),
            StructType::VmfsDatastoreSingleExtentOption => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreSingleExtentOption>()?),
            StructType::VmfsDatastoreAllExtentOption => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreAllExtentOption>()?),
            StructType::VmfsDatastoreSpec => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreSpec>()?),
            StructType::VmfsDatastoreCreateSpec => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreCreateSpec>()?),
            StructType::VmfsDatastoreExpandSpec => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreExpandSpec>()?),
            StructType::VmfsDatastoreExtendSpec => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreExtendSpec>()?),
            StructType::HostVmfsRescanResult => Some(from.as_any_ref().downcast_ref::<HostVmfsRescanResult>()?),
            StructType::VmfsConfigOption => Some(from.as_any_ref().downcast_ref::<VmfsConfigOption>()?),
            StructType::HostVmfsSpec => Some(from.as_any_ref().downcast_ref::<HostVmfsSpec>()?),
            StructType::VmfsUnmapBandwidthSpec => Some(from.as_any_ref().downcast_ref::<VmfsUnmapBandwidthSpec>()?),
            StructType::HostVsanInternalSystemCmmdsQuery => Some(from.as_any_ref().downcast_ref::<HostVsanInternalSystemCmmdsQuery>()?),
            StructType::HostVsanInternalSystemDeleteVsanObjectsResult => Some(from.as_any_ref().downcast_ref::<HostVsanInternalSystemDeleteVsanObjectsResult>()?),
            StructType::VsanNewPolicyBatch => Some(from.as_any_ref().downcast_ref::<VsanNewPolicyBatch>()?),
            StructType::VsanPolicyChangeBatch => Some(from.as_any_ref().downcast_ref::<VsanPolicyChangeBatch>()?),
            StructType::VsanPolicyCost => Some(from.as_any_ref().downcast_ref::<VsanPolicyCost>()?),
            StructType::VsanPolicySatisfiability => Some(from.as_any_ref().downcast_ref::<VsanPolicySatisfiability>()?),
            StructType::HostVsanInternalSystemVsanObjectOperationResult => Some(from.as_any_ref().downcast_ref::<HostVsanInternalSystemVsanObjectOperationResult>()?),
            StructType::HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult => Some(from.as_any_ref().downcast_ref::<HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult>()?),
            StructType::HostVvolNqn => Some(from.as_any_ref().downcast_ref::<HostVvolNqn>()?),
            StructType::VVolHostPe => Some(from.as_any_ref().downcast_ref::<VVolHostPe>()?),
            StructType::HostVvolVolumeHostVvolNqn => Some(from.as_any_ref().downcast_ref::<HostVvolVolumeHostVvolNqn>()?),
            StructType::HostVvolVolumeSpecification => Some(from.as_any_ref().downcast_ref::<HostVvolVolumeSpecification>()?),
            StructType::NetDhcpConfigInfo => Some(from.as_any_ref().downcast_ref::<NetDhcpConfigInfo>()?),
            StructType::NetDhcpConfigInfoDhcpOptions => Some(from.as_any_ref().downcast_ref::<NetDhcpConfigInfoDhcpOptions>()?),
            StructType::NetDhcpConfigSpec => Some(from.as_any_ref().downcast_ref::<NetDhcpConfigSpec>()?),
            StructType::NetDhcpConfigSpecDhcpOptionsSpec => Some(from.as_any_ref().downcast_ref::<NetDhcpConfigSpecDhcpOptionsSpec>()?),
            StructType::NetDnsConfigInfo => Some(from.as_any_ref().downcast_ref::<NetDnsConfigInfo>()?),
            StructType::NetDnsConfigSpec => Some(from.as_any_ref().downcast_ref::<NetDnsConfigSpec>()?),
            StructType::NetIpConfigInfo => Some(from.as_any_ref().downcast_ref::<NetIpConfigInfo>()?),
            StructType::NetIpConfigInfoIpAddress => Some(from.as_any_ref().downcast_ref::<NetIpConfigInfoIpAddress>()?),
            StructType::NetIpConfigSpec => Some(from.as_any_ref().downcast_ref::<NetIpConfigSpec>()?),
            StructType::NetIpConfigSpecIpAddressSpec => Some(from.as_any_ref().downcast_ref::<NetIpConfigSpecIpAddressSpec>()?),
            StructType::NetIpRouteConfigInfo => Some(from.as_any_ref().downcast_ref::<NetIpRouteConfigInfo>()?),
            StructType::NetIpRouteConfigInfoGateway => Some(from.as_any_ref().downcast_ref::<NetIpRouteConfigInfoGateway>()?),
            StructType::NetIpRouteConfigInfoIpRoute => Some(from.as_any_ref().downcast_ref::<NetIpRouteConfigInfoIpRoute>()?),
            StructType::NetIpRouteConfigSpec => Some(from.as_any_ref().downcast_ref::<NetIpRouteConfigSpec>()?),
            StructType::NetIpRouteConfigSpecGatewaySpec => Some(from.as_any_ref().downcast_ref::<NetIpRouteConfigSpecGatewaySpec>()?),
            StructType::NetIpRouteConfigSpecIpRouteSpec => Some(from.as_any_ref().downcast_ref::<NetIpRouteConfigSpecIpRouteSpec>()?),
            StructType::NetIpStackInfo => Some(from.as_any_ref().downcast_ref::<NetIpStackInfo>()?),
            StructType::NetIpStackInfoDefaultRouter => Some(from.as_any_ref().downcast_ref::<NetIpStackInfoDefaultRouter>()?),
            StructType::NetIpStackInfoNetToMedia => Some(from.as_any_ref().downcast_ref::<NetIpStackInfoNetToMedia>()?),
            StructType::NetBiosConfigInfo => Some(from.as_any_ref().downcast_ref::<NetBiosConfigInfo>()?),
            StructType::WinNetBiosConfigInfo => Some(from.as_any_ref().downcast_ref::<WinNetBiosConfigInfo>()?),
            StructType::ArrayUpdateSpec => Some(from.as_any_ref().downcast_ref::<ArrayUpdateSpec>()?),
            StructType::ClusterDasVmConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterDasVmConfigSpec>()?),
            StructType::ClusterDatastoreUpdateSpec => Some(from.as_any_ref().downcast_ref::<ClusterDatastoreUpdateSpec>()?),
            StructType::ClusterDpmHostConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterDpmHostConfigSpec>()?),
            StructType::ClusterDrsVmConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterDrsVmConfigSpec>()?),
            StructType::ClusterGroupSpec => Some(from.as_any_ref().downcast_ref::<ClusterGroupSpec>()?),
            StructType::ClusterPreemptibleVmPairSpec => Some(from.as_any_ref().downcast_ref::<ClusterPreemptibleVmPairSpec>()?),
            StructType::ClusterRuleSpec => Some(from.as_any_ref().downcast_ref::<ClusterRuleSpec>()?),
            StructType::ClusterTagCategoryUpdateSpec => Some(from.as_any_ref().downcast_ref::<ClusterTagCategoryUpdateSpec>()?),
            StructType::ClusterVmOrchestrationSpec => Some(from.as_any_ref().downcast_ref::<ClusterVmOrchestrationSpec>()?),
            StructType::StorageDrsOptionSpec => Some(from.as_any_ref().downcast_ref::<StorageDrsOptionSpec>()?),
            StructType::StorageDrsVmConfigSpec => Some(from.as_any_ref().downcast_ref::<StorageDrsVmConfigSpec>()?),
            StructType::VAppOvfSectionSpec => Some(from.as_any_ref().downcast_ref::<VAppOvfSectionSpec>()?),
            StructType::VAppProductSpec => Some(from.as_any_ref().downcast_ref::<VAppProductSpec>()?),
            StructType::VAppPropertySpec => Some(from.as_any_ref().downcast_ref::<VAppPropertySpec>()?),
            StructType::VirtualMachineCpuIdInfoSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineCpuIdInfoSpec>()?),
            StructType::OptionType => Some(from.as_any_ref().downcast_ref::<OptionType>()?),
            StructType::BoolOption => Some(from.as_any_ref().downcast_ref::<BoolOption>()?),
            StructType::ChoiceOption => Some(from.as_any_ref().downcast_ref::<ChoiceOption>()?),
            StructType::FloatOption => Some(from.as_any_ref().downcast_ref::<FloatOption>()?),
            StructType::IntOption => Some(from.as_any_ref().downcast_ref::<IntOption>()?),
            StructType::LongOption => Some(from.as_any_ref().downcast_ref::<LongOption>()?),
            StructType::StringOption => Some(from.as_any_ref().downcast_ref::<StringOption>()?),
            StructType::OptionValue => Some(from.as_any_ref().downcast_ref::<OptionValue>()?),
            StructType::HostInternetScsiHbaParamValue => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaParamValue>()?),
            StructType::ApplyProfile => Some(from.as_any_ref().downcast_ref::<ApplyProfile>()?),
            StructType::ProfileApplyProfileElement => Some(from.as_any_ref().downcast_ref::<ProfileApplyProfileElement>()?),
            StructType::ActiveDirectoryProfile => Some(from.as_any_ref().downcast_ref::<ActiveDirectoryProfile>()?),
            StructType::AuthenticationProfile => Some(from.as_any_ref().downcast_ref::<AuthenticationProfile>()?),
            StructType::DateTimeProfile => Some(from.as_any_ref().downcast_ref::<DateTimeProfile>()?),
            StructType::DvsProfile => Some(from.as_any_ref().downcast_ref::<DvsProfile>()?),
            StructType::DvsVNicProfile => Some(from.as_any_ref().downcast_ref::<DvsVNicProfile>()?),
            StructType::DvsHostVNicProfile => Some(from.as_any_ref().downcast_ref::<DvsHostVNicProfile>()?),
            StructType::DvsServiceConsoleVNicProfile => Some(from.as_any_ref().downcast_ref::<DvsServiceConsoleVNicProfile>()?),
            StructType::FirewallProfile => Some(from.as_any_ref().downcast_ref::<FirewallProfile>()?),
            StructType::FirewallProfileRulesetProfile => Some(from.as_any_ref().downcast_ref::<FirewallProfileRulesetProfile>()?),
            StructType::HostApplyProfile => Some(from.as_any_ref().downcast_ref::<HostApplyProfile>()?),
            StructType::HostMemoryProfile => Some(from.as_any_ref().downcast_ref::<HostMemoryProfile>()?),
            StructType::IpAddressProfile => Some(from.as_any_ref().downcast_ref::<IpAddressProfile>()?),
            StructType::IpRouteProfile => Some(from.as_any_ref().downcast_ref::<IpRouteProfile>()?),
            StructType::NasStorageProfile => Some(from.as_any_ref().downcast_ref::<NasStorageProfile>()?),
            StructType::NetStackInstanceProfile => Some(from.as_any_ref().downcast_ref::<NetStackInstanceProfile>()?),
            StructType::NetworkPolicyProfile => Some(from.as_any_ref().downcast_ref::<NetworkPolicyProfile>()?),
            StructType::NetworkProfile => Some(from.as_any_ref().downcast_ref::<NetworkProfile>()?),
            StructType::NetworkProfileDnsConfigProfile => Some(from.as_any_ref().downcast_ref::<NetworkProfileDnsConfigProfile>()?),
            StructType::NsxHostVNicProfile => Some(from.as_any_ref().downcast_ref::<NsxHostVNicProfile>()?),
            StructType::OpaqueSwitchProfile => Some(from.as_any_ref().downcast_ref::<OpaqueSwitchProfile>()?),
            StructType::OptionProfile => Some(from.as_any_ref().downcast_ref::<OptionProfile>()?),
            StructType::PermissionProfile => Some(from.as_any_ref().downcast_ref::<PermissionProfile>()?),
            StructType::PhysicalNicProfile => Some(from.as_any_ref().downcast_ref::<PhysicalNicProfile>()?),
            StructType::PnicUplinkProfile => Some(from.as_any_ref().downcast_ref::<PnicUplinkProfile>()?),
            StructType::PortGroupProfile => Some(from.as_any_ref().downcast_ref::<PortGroupProfile>()?),
            StructType::HostPortGroupProfile => Some(from.as_any_ref().downcast_ref::<HostPortGroupProfile>()?),
            StructType::ServiceConsolePortGroupProfile => Some(from.as_any_ref().downcast_ref::<ServiceConsolePortGroupProfile>()?),
            StructType::VmPortGroupProfile => Some(from.as_any_ref().downcast_ref::<VmPortGroupProfile>()?),
            StructType::VirtualSwitchSelectionProfile => Some(from.as_any_ref().downcast_ref::<VirtualSwitchSelectionProfile>()?),
            StructType::VlanProfile => Some(from.as_any_ref().downcast_ref::<VlanProfile>()?),
            StructType::SecurityProfile => Some(from.as_any_ref().downcast_ref::<SecurityProfile>()?),
            StructType::ServiceProfile => Some(from.as_any_ref().downcast_ref::<ServiceProfile>()?),
            StructType::StaticRouteProfile => Some(from.as_any_ref().downcast_ref::<StaticRouteProfile>()?),
            StructType::StorageProfile => Some(from.as_any_ref().downcast_ref::<StorageProfile>()?),
            StructType::UserGroupProfile => Some(from.as_any_ref().downcast_ref::<UserGroupProfile>()?),
            StructType::UserProfile => Some(from.as_any_ref().downcast_ref::<UserProfile>()?),
            StructType::VirtualSwitchProfile => Some(from.as_any_ref().downcast_ref::<VirtualSwitchProfile>()?),
            StructType::LinkProfile => Some(from.as_any_ref().downcast_ref::<LinkProfile>()?),
            StructType::NumPortsProfile => Some(from.as_any_ref().downcast_ref::<NumPortsProfile>()?),
            StructType::ProfileApplyProfileProperty => Some(from.as_any_ref().downcast_ref::<ProfileApplyProfileProperty>()?),
            StructType::ComplianceLocator => Some(from.as_any_ref().downcast_ref::<ComplianceLocator>()?),
            StructType::ComplianceProfile => Some(from.as_any_ref().downcast_ref::<ComplianceProfile>()?),
            StructType::ComplianceResult => Some(from.as_any_ref().downcast_ref::<ComplianceResult>()?),
            StructType::ComplianceFailure => Some(from.as_any_ref().downcast_ref::<ComplianceFailure>()?),
            StructType::ComplianceFailureComplianceFailureValues => Some(from.as_any_ref().downcast_ref::<ComplianceFailureComplianceFailureValues>()?),
            StructType::ProfileDeferredPolicyOptionParameter => Some(from.as_any_ref().downcast_ref::<ProfileDeferredPolicyOptionParameter>()?),
            StructType::ProfileExpression => Some(from.as_any_ref().downcast_ref::<ProfileExpression>()?),
            StructType::ProfileCompositeExpression => Some(from.as_any_ref().downcast_ref::<ProfileCompositeExpression>()?),
            StructType::ProfileSimpleExpression => Some(from.as_any_ref().downcast_ref::<ProfileSimpleExpression>()?),
            StructType::ProfileExpressionMetadata => Some(from.as_any_ref().downcast_ref::<ProfileExpressionMetadata>()?),
            StructType::ProfileParameterMetadata => Some(from.as_any_ref().downcast_ref::<ProfileParameterMetadata>()?),
            StructType::ProfileParameterMetadataParameterRelationMetadata => Some(from.as_any_ref().downcast_ref::<ProfileParameterMetadataParameterRelationMetadata>()?),
            StructType::ProfilePolicy => Some(from.as_any_ref().downcast_ref::<ProfilePolicy>()?),
            StructType::ProfilePolicyMetadata => Some(from.as_any_ref().downcast_ref::<ProfilePolicyMetadata>()?),
            StructType::PolicyOption => Some(from.as_any_ref().downcast_ref::<PolicyOption>()?),
            StructType::CompositePolicyOption => Some(from.as_any_ref().downcast_ref::<CompositePolicyOption>()?),
            StructType::ProfilePolicyOptionMetadata => Some(from.as_any_ref().downcast_ref::<ProfilePolicyOptionMetadata>()?),
            StructType::ProfileCompositePolicyOptionMetadata => Some(from.as_any_ref().downcast_ref::<ProfileCompositePolicyOptionMetadata>()?),
            StructType::UserInputRequiredParameterMetadata => Some(from.as_any_ref().downcast_ref::<UserInputRequiredParameterMetadata>()?),
            StructType::ProfileConfigInfo => Some(from.as_any_ref().downcast_ref::<ProfileConfigInfo>()?),
            StructType::ClusterProfileConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterProfileConfigInfo>()?),
            StructType::HostProfileConfigInfo => Some(from.as_any_ref().downcast_ref::<HostProfileConfigInfo>()?),
            StructType::ProfileCreateSpec => Some(from.as_any_ref().downcast_ref::<ProfileCreateSpec>()?),
            StructType::ProfileSerializedCreateSpec => Some(from.as_any_ref().downcast_ref::<ProfileSerializedCreateSpec>()?),
            StructType::HostProfileSerializedHostProfileSpec => Some(from.as_any_ref().downcast_ref::<HostProfileSerializedHostProfileSpec>()?),
            StructType::ClusterProfileCreateSpec => Some(from.as_any_ref().downcast_ref::<ClusterProfileCreateSpec>()?),
            StructType::ClusterProfileConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterProfileConfigSpec>()?),
            StructType::ClusterProfileCompleteConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterProfileCompleteConfigSpec>()?),
            StructType::ClusterProfileConfigServiceCreateSpec => Some(from.as_any_ref().downcast_ref::<ClusterProfileConfigServiceCreateSpec>()?),
            StructType::HostProfileConfigSpec => Some(from.as_any_ref().downcast_ref::<HostProfileConfigSpec>()?),
            StructType::HostProfileCompleteConfigSpec => Some(from.as_any_ref().downcast_ref::<HostProfileCompleteConfigSpec>()?),
            StructType::HostProfileHostBasedConfigSpec => Some(from.as_any_ref().downcast_ref::<HostProfileHostBasedConfigSpec>()?),
            StructType::ProfileDescription => Some(from.as_any_ref().downcast_ref::<ProfileDescription>()?),
            StructType::ProfileDescriptionSection => Some(from.as_any_ref().downcast_ref::<ProfileDescriptionSection>()?),
            StructType::ProfileMetadata => Some(from.as_any_ref().downcast_ref::<ProfileMetadata>()?),
            StructType::ProfileMetadataProfileOperationMessage => Some(from.as_any_ref().downcast_ref::<ProfileMetadataProfileOperationMessage>()?),
            StructType::ProfileMetadataProfileSortSpec => Some(from.as_any_ref().downcast_ref::<ProfileMetadataProfileSortSpec>()?),
            StructType::ProfilePropertyPath => Some(from.as_any_ref().downcast_ref::<ProfilePropertyPath>()?),
            StructType::ProfileProfileStructure => Some(from.as_any_ref().downcast_ref::<ProfileProfileStructure>()?),
            StructType::ProfileProfileStructureProperty => Some(from.as_any_ref().downcast_ref::<ProfileProfileStructureProperty>()?),
            StructType::AnswerFile => Some(from.as_any_ref().downcast_ref::<AnswerFile>()?),
            StructType::AnswerFileStatusResult => Some(from.as_any_ref().downcast_ref::<AnswerFileStatusResult>()?),
            StructType::AnswerFileStatusError => Some(from.as_any_ref().downcast_ref::<AnswerFileStatusError>()?),
            StructType::ProfileExecuteResult => Some(from.as_any_ref().downcast_ref::<ProfileExecuteResult>()?),
            StructType::ApplyHostProfileConfigurationSpec => Some(from.as_any_ref().downcast_ref::<ApplyHostProfileConfigurationSpec>()?),
            StructType::ProfileExecuteError => Some(from.as_any_ref().downcast_ref::<ProfileExecuteError>()?),
            StructType::HostProfileValidationFailureInfo => Some(from.as_any_ref().downcast_ref::<HostProfileValidationFailureInfo>()?),
            StructType::HostSpecification => Some(from.as_any_ref().downcast_ref::<HostSpecification>()?),
            StructType::HostSubSpecification => Some(from.as_any_ref().downcast_ref::<HostSubSpecification>()?),
            StructType::AnswerFileCreateSpec => Some(from.as_any_ref().downcast_ref::<AnswerFileCreateSpec>()?),
            StructType::AnswerFileOptionsCreateSpec => Some(from.as_any_ref().downcast_ref::<AnswerFileOptionsCreateSpec>()?),
            StructType::AnswerFileSerializedCreateSpec => Some(from.as_any_ref().downcast_ref::<AnswerFileSerializedCreateSpec>()?),
            StructType::ApplyHostProfileConfigurationResult => Some(from.as_any_ref().downcast_ref::<ApplyHostProfileConfigurationResult>()?),
            StructType::HostProfileManagerCompositionResult => Some(from.as_any_ref().downcast_ref::<HostProfileManagerCompositionResult>()?),
            StructType::HostProfileManagerCompositionResultResultElement => Some(from.as_any_ref().downcast_ref::<HostProfileManagerCompositionResultResultElement>()?),
            StructType::HostProfileManagerCompositionValidationResult => Some(from.as_any_ref().downcast_ref::<HostProfileManagerCompositionValidationResult>()?),
            StructType::HostProfileManagerCompositionValidationResultResultElement => Some(from.as_any_ref().downcast_ref::<HostProfileManagerCompositionValidationResultResultElement>()?),
            StructType::HostProfileManagerConfigTaskList => Some(from.as_any_ref().downcast_ref::<HostProfileManagerConfigTaskList>()?),
            StructType::HostProfilesEntityCustomizations => Some(from.as_any_ref().downcast_ref::<HostProfilesEntityCustomizations>()?),
            StructType::StructuredCustomizations => Some(from.as_any_ref().downcast_ref::<StructuredCustomizations>()?),
            StructType::HostProfileManagerHostToConfigSpecMap => Some(from.as_any_ref().downcast_ref::<HostProfileManagerHostToConfigSpecMap>()?),
            StructType::ScheduledTaskDescription => Some(from.as_any_ref().downcast_ref::<ScheduledTaskDescription>()?),
            StructType::ScheduledTaskSpec => Some(from.as_any_ref().downcast_ref::<ScheduledTaskSpec>()?),
            StructType::ScheduledTaskInfo => Some(from.as_any_ref().downcast_ref::<ScheduledTaskInfo>()?),
            StructType::TaskScheduler => Some(from.as_any_ref().downcast_ref::<TaskScheduler>()?),
            StructType::AfterStartupTaskScheduler => Some(from.as_any_ref().downcast_ref::<AfterStartupTaskScheduler>()?),
            StructType::OnceTaskScheduler => Some(from.as_any_ref().downcast_ref::<OnceTaskScheduler>()?),
            StructType::RecurrentTaskScheduler => Some(from.as_any_ref().downcast_ref::<RecurrentTaskScheduler>()?),
            StructType::HourlyTaskScheduler => Some(from.as_any_ref().downcast_ref::<HourlyTaskScheduler>()?),
            StructType::DailyTaskScheduler => Some(from.as_any_ref().downcast_ref::<DailyTaskScheduler>()?),
            StructType::MonthlyTaskScheduler => Some(from.as_any_ref().downcast_ref::<MonthlyTaskScheduler>()?),
            StructType::MonthlyByDayTaskScheduler => Some(from.as_any_ref().downcast_ref::<MonthlyByDayTaskScheduler>()?),
            StructType::MonthlyByWeekdayTaskScheduler => Some(from.as_any_ref().downcast_ref::<MonthlyByWeekdayTaskScheduler>()?),
            StructType::WeeklyTaskScheduler => Some(from.as_any_ref().downcast_ref::<WeeklyTaskScheduler>()?),
            StructType::ApplyStorageRecommendationResult => Some(from.as_any_ref().downcast_ref::<ApplyStorageRecommendationResult>()?),
            StructType::StorageDrsAutomationConfig => Some(from.as_any_ref().downcast_ref::<StorageDrsAutomationConfig>()?),
            StructType::StorageDrsConfigInfo => Some(from.as_any_ref().downcast_ref::<StorageDrsConfigInfo>()?),
            StructType::StorageDrsConfigSpec => Some(from.as_any_ref().downcast_ref::<StorageDrsConfigSpec>()?),
            StructType::StorageDrsIoLoadBalanceConfig => Some(from.as_any_ref().downcast_ref::<StorageDrsIoLoadBalanceConfig>()?),
            StructType::PlacementAffinityRule => Some(from.as_any_ref().downcast_ref::<PlacementAffinityRule>()?),
            StructType::PlacementRankResult => Some(from.as_any_ref().downcast_ref::<PlacementRankResult>()?),
            StructType::PlacementRankSpec => Some(from.as_any_ref().downcast_ref::<PlacementRankSpec>()?),
            StructType::StorageDrsPlacementRankVmSpec => Some(from.as_any_ref().downcast_ref::<StorageDrsPlacementRankVmSpec>()?),
            StructType::StorageDrsPodConfigInfo => Some(from.as_any_ref().downcast_ref::<StorageDrsPodConfigInfo>()?),
            StructType::StorageDrsPodConfigSpec => Some(from.as_any_ref().downcast_ref::<StorageDrsPodConfigSpec>()?),
            StructType::StorageDrsPodSelectionSpec => Some(from.as_any_ref().downcast_ref::<StorageDrsPodSelectionSpec>()?),
            StructType::PodDiskLocator => Some(from.as_any_ref().downcast_ref::<PodDiskLocator>()?),
            StructType::VmPodConfigForPlacement => Some(from.as_any_ref().downcast_ref::<VmPodConfigForPlacement>()?),
            StructType::StorageDrsSpaceLoadBalanceConfig => Some(from.as_any_ref().downcast_ref::<StorageDrsSpaceLoadBalanceConfig>()?),
            StructType::StoragePlacementResult => Some(from.as_any_ref().downcast_ref::<StoragePlacementResult>()?),
            StructType::StoragePlacementSpec => Some(from.as_any_ref().downcast_ref::<StoragePlacementSpec>()?),
            StructType::StorageDrsVmConfigInfo => Some(from.as_any_ref().downcast_ref::<StorageDrsVmConfigInfo>()?),
            StructType::VAppCloneSpec => Some(from.as_any_ref().downcast_ref::<VAppCloneSpec>()?),
            StructType::VAppCloneSpecNetworkMappingPair => Some(from.as_any_ref().downcast_ref::<VAppCloneSpecNetworkMappingPair>()?),
            StructType::VAppCloneSpecResourceMap => Some(from.as_any_ref().downcast_ref::<VAppCloneSpecResourceMap>()?),
            StructType::VAppEntityConfigInfo => Some(from.as_any_ref().downcast_ref::<VAppEntityConfigInfo>()?),
            StructType::VAppIpAssignmentInfo => Some(from.as_any_ref().downcast_ref::<VAppIpAssignmentInfo>()?),
            StructType::IpPool => Some(from.as_any_ref().downcast_ref::<IpPool>()?),
            StructType::IpPoolAssociation => Some(from.as_any_ref().downcast_ref::<IpPoolAssociation>()?),
            StructType::IpPoolIpPoolConfigInfo => Some(from.as_any_ref().downcast_ref::<IpPoolIpPoolConfigInfo>()?),
            StructType::VAppOvfSectionInfo => Some(from.as_any_ref().downcast_ref::<VAppOvfSectionInfo>()?),
            StructType::VAppProductInfo => Some(from.as_any_ref().downcast_ref::<VAppProductInfo>()?),
            StructType::VAppPropertyInfo => Some(from.as_any_ref().downcast_ref::<VAppPropertyInfo>()?),
            StructType::VmConfigInfo => Some(from.as_any_ref().downcast_ref::<VmConfigInfo>()?),
            StructType::VAppConfigInfo => Some(from.as_any_ref().downcast_ref::<VAppConfigInfo>()?),
            StructType::VmConfigSpec => Some(from.as_any_ref().downcast_ref::<VmConfigSpec>()?),
            StructType::VAppConfigSpec => Some(from.as_any_ref().downcast_ref::<VAppConfigSpec>()?),
            StructType::ClusterNetworkConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterNetworkConfigSpec>()?),
            StructType::FailoverNodeInfo => Some(from.as_any_ref().downcast_ref::<FailoverNodeInfo>()?),
            StructType::NodeDeploymentSpec => Some(from.as_any_ref().downcast_ref::<NodeDeploymentSpec>()?),
            StructType::PassiveNodeDeploymentSpec => Some(from.as_any_ref().downcast_ref::<PassiveNodeDeploymentSpec>()?),
            StructType::NodeNetworkSpec => Some(from.as_any_ref().downcast_ref::<NodeNetworkSpec>()?),
            StructType::PassiveNodeNetworkSpec => Some(from.as_any_ref().downcast_ref::<PassiveNodeNetworkSpec>()?),
            StructType::SourceNodeSpec => Some(from.as_any_ref().downcast_ref::<SourceNodeSpec>()?),
            StructType::VchaClusterConfigInfo => Some(from.as_any_ref().downcast_ref::<VchaClusterConfigInfo>()?),
            StructType::VchaClusterConfigSpec => Some(from.as_any_ref().downcast_ref::<VchaClusterConfigSpec>()?),
            StructType::VchaClusterDeploymentSpec => Some(from.as_any_ref().downcast_ref::<VchaClusterDeploymentSpec>()?),
            StructType::VchaClusterNetworkSpec => Some(from.as_any_ref().downcast_ref::<VchaClusterNetworkSpec>()?),
            StructType::WitnessNodeInfo => Some(from.as_any_ref().downcast_ref::<WitnessNodeInfo>()?),
            StructType::VchaClusterHealth => Some(from.as_any_ref().downcast_ref::<VchaClusterHealth>()?),
            StructType::VchaClusterRuntimeInfo => Some(from.as_any_ref().downcast_ref::<VchaClusterRuntimeInfo>()?),
            StructType::VchaNodeRuntimeInfo => Some(from.as_any_ref().downcast_ref::<VchaNodeRuntimeInfo>()?),
            StructType::VirtualMachineAffinityInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineAffinityInfo>()?),
            StructType::VirtualMachineBaseIndependentFilterSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineBaseIndependentFilterSpec>()?),
            StructType::VirtualMachineEmptyIndependentFilterSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineEmptyIndependentFilterSpec>()?),
            StructType::VirtualMachineIndependentFilterSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineIndependentFilterSpec>()?),
            StructType::VirtualMachineBootOptions => Some(from.as_any_ref().downcast_ref::<VirtualMachineBootOptions>()?),
            StructType::VirtualMachineBootOptionsBootableDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableDevice>()?),
            StructType::VirtualMachineBootOptionsBootableCdromDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableCdromDevice>()?),
            StructType::VirtualMachineBootOptionsBootableDiskDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableDiskDevice>()?),
            StructType::VirtualMachineBootOptionsBootableEthernetDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableEthernetDevice>()?),
            StructType::VirtualMachineBootOptionsBootableFloppyDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableFloppyDevice>()?),
            StructType::VirtualMachineCapability => Some(from.as_any_ref().downcast_ref::<VirtualMachineCapability>()?),
            StructType::VirtualMachineCertThumbprint => Some(from.as_any_ref().downcast_ref::<VirtualMachineCertThumbprint>()?),
            StructType::VirtualMachineCloneSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineCloneSpec>()?),
            StructType::VirtualMachineConfigInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineConfigInfo>()?),
            StructType::VirtualMachineConfigInfoDatastoreUrlPair => Some(from.as_any_ref().downcast_ref::<VirtualMachineConfigInfoDatastoreUrlPair>()?),
            StructType::VirtualMachineConfigInfoOverheadInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineConfigInfoOverheadInfo>()?),
            StructType::VirtualMachineConfigOption => Some(from.as_any_ref().downcast_ref::<VirtualMachineConfigOption>()?),
            StructType::VirtualMachineConfigOptionDescriptor => Some(from.as_any_ref().downcast_ref::<VirtualMachineConfigOptionDescriptor>()?),
            StructType::VirtualMachineConfigSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineConfigSpec>()?),
            StructType::ConfigTarget => Some(from.as_any_ref().downcast_ref::<ConfigTarget>()?),
            StructType::VirtualMachineConsolePreferences => Some(from.as_any_ref().downcast_ref::<VirtualMachineConsolePreferences>()?),
            StructType::VirtualMachineContentLibraryItemInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineContentLibraryItemInfo>()?),
            StructType::DatastoreOption => Some(from.as_any_ref().downcast_ref::<DatastoreOption>()?),
            StructType::VirtualMachineDatastoreVolumeOption => Some(from.as_any_ref().downcast_ref::<VirtualMachineDatastoreVolumeOption>()?),
            StructType::VirtualMachineDefaultPowerOpInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineDefaultPowerOpInfo>()?),
            StructType::VirtualMachineDeviceRuntimeInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineDeviceRuntimeInfo>()?),
            StructType::VirtualMachineDeviceRuntimeInfoDeviceRuntimeState => Some(from.as_any_ref().downcast_ref::<VirtualMachineDeviceRuntimeInfoDeviceRuntimeState>()?),
            StructType::VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState => Some(from.as_any_ref().downcast_ref::<VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState>()?),
            StructType::VirtualMachineDvxClassInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineDvxClassInfo>()?),
            StructType::FaultToleranceConfigInfo => Some(from.as_any_ref().downcast_ref::<FaultToleranceConfigInfo>()?),
            StructType::FaultTolerancePrimaryConfigInfo => Some(from.as_any_ref().downcast_ref::<FaultTolerancePrimaryConfigInfo>()?),
            StructType::FaultToleranceSecondaryConfigInfo => Some(from.as_any_ref().downcast_ref::<FaultToleranceSecondaryConfigInfo>()?),
            StructType::FaultToleranceConfigSpec => Some(from.as_any_ref().downcast_ref::<FaultToleranceConfigSpec>()?),
            StructType::FaultToleranceMetaSpec => Some(from.as_any_ref().downcast_ref::<FaultToleranceMetaSpec>()?),
            StructType::FaultToleranceSecondaryOpResult => Some(from.as_any_ref().downcast_ref::<FaultToleranceSecondaryOpResult>()?),
            StructType::FaultToleranceVmConfigSpec => Some(from.as_any_ref().downcast_ref::<FaultToleranceVmConfigSpec>()?),
            StructType::FaultToleranceDiskSpec => Some(from.as_any_ref().downcast_ref::<FaultToleranceDiskSpec>()?),
            StructType::VirtualMachineFeatureRequirement => Some(from.as_any_ref().downcast_ref::<VirtualMachineFeatureRequirement>()?),
            StructType::VirtualMachineFileInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineFileInfo>()?),
            StructType::VirtualMachineFileLayout => Some(from.as_any_ref().downcast_ref::<VirtualMachineFileLayout>()?),
            StructType::VirtualMachineFileLayoutDiskLayout => Some(from.as_any_ref().downcast_ref::<VirtualMachineFileLayoutDiskLayout>()?),
            StructType::VirtualMachineFileLayoutSnapshotLayout => Some(from.as_any_ref().downcast_ref::<VirtualMachineFileLayoutSnapshotLayout>()?),
            StructType::VirtualMachineFileLayoutEx => Some(from.as_any_ref().downcast_ref::<VirtualMachineFileLayoutEx>()?),
            StructType::VirtualMachineFileLayoutExDiskLayout => Some(from.as_any_ref().downcast_ref::<VirtualMachineFileLayoutExDiskLayout>()?),
            StructType::VirtualMachineFileLayoutExDiskUnit => Some(from.as_any_ref().downcast_ref::<VirtualMachineFileLayoutExDiskUnit>()?),
            StructType::VirtualMachineFileLayoutExFileInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineFileLayoutExFileInfo>()?),
            StructType::VirtualMachineFileLayoutExSnapshotLayout => Some(from.as_any_ref().downcast_ref::<VirtualMachineFileLayoutExSnapshotLayout>()?),
            StructType::VirtualMachineFlagInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineFlagInfo>()?),
            StructType::VirtualMachineForkConfigInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineForkConfigInfo>()?),
            StructType::GuestInfo => Some(from.as_any_ref().downcast_ref::<GuestInfo>()?),
            StructType::GuestInfoCustomizationInfo => Some(from.as_any_ref().downcast_ref::<GuestInfoCustomizationInfo>()?),
            StructType::GuestDiskInfo => Some(from.as_any_ref().downcast_ref::<GuestDiskInfo>()?),
            StructType::GuestInfoNamespaceGenerationInfo => Some(from.as_any_ref().downcast_ref::<GuestInfoNamespaceGenerationInfo>()?),
            StructType::GuestNicInfo => Some(from.as_any_ref().downcast_ref::<GuestNicInfo>()?),
            StructType::GuestScreenInfo => Some(from.as_any_ref().downcast_ref::<GuestScreenInfo>()?),
            StructType::GuestStackInfo => Some(from.as_any_ref().downcast_ref::<GuestStackInfo>()?),
            StructType::GuestInfoVirtualDiskMapping => Some(from.as_any_ref().downcast_ref::<GuestInfoVirtualDiskMapping>()?),
            StructType::VirtualMachineGuestIntegrityInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineGuestIntegrityInfo>()?),
            StructType::VirtualMachineGuestMonitoringModeInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineGuestMonitoringModeInfo>()?),
            StructType::GuestOsDescriptor => Some(from.as_any_ref().downcast_ref::<GuestOsDescriptor>()?),
            StructType::VirtualMachineGuestQuiesceSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineGuestQuiesceSpec>()?),
            StructType::VirtualMachineWindowsQuiesceSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineWindowsQuiesceSpec>()?),
            StructType::VirtualMachineIdeDiskDevicePartitionInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineIdeDiskDevicePartitionInfo>()?),
            StructType::VirtualMachineInstantCloneSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineInstantCloneSpec>()?),
            StructType::VirtualMachineLegacyNetworkSwitchInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineLegacyNetworkSwitchInfo>()?),
            StructType::VirtualMachineMessage => Some(from.as_any_ref().downcast_ref::<VirtualMachineMessage>()?),
            StructType::VirtualMachineMetadataManagerVmMetadata => Some(from.as_any_ref().downcast_ref::<VirtualMachineMetadataManagerVmMetadata>()?),
            StructType::VirtualMachineMetadataManagerVmMetadataInput => Some(from.as_any_ref().downcast_ref::<VirtualMachineMetadataManagerVmMetadataInput>()?),
            StructType::VirtualMachineMetadataManagerVmMetadataOwner => Some(from.as_any_ref().downcast_ref::<VirtualMachineMetadataManagerVmMetadataOwner>()?),
            StructType::VirtualMachineMetadataManagerVmMetadataResult => Some(from.as_any_ref().downcast_ref::<VirtualMachineMetadataManagerVmMetadataResult>()?),
            StructType::VirtualMachineNetworkShaperInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineNetworkShaperInfo>()?),
            StructType::VirtualMachineProfileDetails => Some(from.as_any_ref().downcast_ref::<VirtualMachineProfileDetails>()?),
            StructType::VirtualMachineProfileDetailsDiskProfileDetails => Some(from.as_any_ref().downcast_ref::<VirtualMachineProfileDetailsDiskProfileDetails>()?),
            StructType::VirtualMachineProfileRawData => Some(from.as_any_ref().downcast_ref::<VirtualMachineProfileRawData>()?),
            StructType::VirtualMachineProfileSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineProfileSpec>()?),
            StructType::VirtualMachineDefaultProfileSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineDefaultProfileSpec>()?),
            StructType::VirtualMachineDefinedProfileSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineDefinedProfileSpec>()?),
            StructType::VirtualMachineEmptyProfileSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineEmptyProfileSpec>()?),
            StructType::VirtualMachinePropertyRelation => Some(from.as_any_ref().downcast_ref::<VirtualMachinePropertyRelation>()?),
            StructType::VirtualMachineQuestionInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineQuestionInfo>()?),
            StructType::VirtualMachineRelocateSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineRelocateSpec>()?),
            StructType::VirtualMachineRelocateSpecDiskLocator => Some(from.as_any_ref().downcast_ref::<VirtualMachineRelocateSpecDiskLocator>()?),
            StructType::VirtualMachineRelocateSpecDiskLocatorBackingSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineRelocateSpecDiskLocatorBackingSpec>()?),
            StructType::ReplicationConfigSpec => Some(from.as_any_ref().downcast_ref::<ReplicationConfigSpec>()?),
            StructType::ReplicationInfoDiskSettings => Some(from.as_any_ref().downcast_ref::<ReplicationInfoDiskSettings>()?),
            StructType::VirtualMachineRuntimeInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineRuntimeInfo>()?),
            StructType::VirtualMachineRuntimeInfoDasProtectionState => Some(from.as_any_ref().downcast_ref::<VirtualMachineRuntimeInfoDasProtectionState>()?),
            StructType::ScheduledHardwareUpgradeInfo => Some(from.as_any_ref().downcast_ref::<ScheduledHardwareUpgradeInfo>()?),
            StructType::VirtualMachineSgxInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSgxInfo>()?),
            StructType::VirtualMachineSnapshotInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSnapshotInfo>()?),
            StructType::VirtualMachineSnapshotTree => Some(from.as_any_ref().downcast_ref::<VirtualMachineSnapshotTree>()?),
            StructType::VirtualMachineSriovDevicePoolInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSriovDevicePoolInfo>()?),
            StructType::VirtualMachineSriovNetworkDevicePoolInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSriovNetworkDevicePoolInfo>()?),
            StructType::VirtualMachineStorageInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineStorageInfo>()?),
            StructType::VirtualMachineUsageOnDatastore => Some(from.as_any_ref().downcast_ref::<VirtualMachineUsageOnDatastore>()?),
            StructType::VirtualMachineSummary => Some(from.as_any_ref().downcast_ref::<VirtualMachineSummary>()?),
            StructType::VirtualMachineConfigSummary => Some(from.as_any_ref().downcast_ref::<VirtualMachineConfigSummary>()?),
            StructType::VirtualMachineGuestSummary => Some(from.as_any_ref().downcast_ref::<VirtualMachineGuestSummary>()?),
            StructType::VirtualMachineQuickStats => Some(from.as_any_ref().downcast_ref::<VirtualMachineQuickStats>()?),
            StructType::VirtualMachineQuickStatsMemoryTierStats => Some(from.as_any_ref().downcast_ref::<VirtualMachineQuickStatsMemoryTierStats>()?),
            StructType::VirtualMachineStorageSummary => Some(from.as_any_ref().downcast_ref::<VirtualMachineStorageSummary>()?),
            StructType::VirtualMachineTargetInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineTargetInfo>()?),
            StructType::VirtualMachineCdromInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineCdromInfo>()?),
            StructType::VirtualMachineDatastoreInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineDatastoreInfo>()?),
            StructType::VirtualMachineDiskDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineDiskDeviceInfo>()?),
            StructType::VirtualMachineIdeDiskDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineIdeDiskDeviceInfo>()?),
            StructType::VirtualMachineScsiDiskDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineScsiDiskDeviceInfo>()?),
            StructType::VirtualMachineDynamicPassthroughInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineDynamicPassthroughInfo>()?),
            StructType::VirtualMachineFloppyInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineFloppyInfo>()?),
            StructType::VirtualMachineNetworkInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineNetworkInfo>()?),
            StructType::OpaqueNetworkTargetInfo => Some(from.as_any_ref().downcast_ref::<OpaqueNetworkTargetInfo>()?),
            StructType::VirtualMachineParallelInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineParallelInfo>()?),
            StructType::VirtualMachinePciPassthroughInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachinePciPassthroughInfo>()?),
            StructType::VirtualMachineSriovInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSriovInfo>()?),
            StructType::VirtualMachinePciSharedGpuPassthroughInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachinePciSharedGpuPassthroughInfo>()?),
            StructType::VirtualMachinePrecisionClockInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachinePrecisionClockInfo>()?),
            StructType::VirtualMachineScsiPassthroughInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineScsiPassthroughInfo>()?),
            StructType::VirtualMachineSerialInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSerialInfo>()?),
            StructType::VirtualMachineSgxTargetInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSgxTargetInfo>()?),
            StructType::VirtualMachineSoundInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineSoundInfo>()?),
            StructType::VirtualMachineUsbInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineUsbInfo>()?),
            StructType::VirtualMachineVFlashModuleInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVFlashModuleInfo>()?),
            StructType::VirtualMachineVMotionStunTimeInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVMotionStunTimeInfo>()?),
            StructType::VirtualMachineVendorDeviceGroupInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVendorDeviceGroupInfo>()?),
            StructType::VirtualMachineVgpuDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVgpuDeviceInfo>()?),
            StructType::VirtualMachineVgpuProfileInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVgpuProfileInfo>()?),
            StructType::ToolsConfigInfo => Some(from.as_any_ref().downcast_ref::<ToolsConfigInfo>()?),
            StructType::ToolsConfigInfoToolsLastInstallInfo => Some(from.as_any_ref().downcast_ref::<ToolsConfigInfoToolsLastInstallInfo>()?),
            StructType::UsbScanCodeSpec => Some(from.as_any_ref().downcast_ref::<UsbScanCodeSpec>()?),
            StructType::UsbScanCodeSpecKeyEvent => Some(from.as_any_ref().downcast_ref::<UsbScanCodeSpecKeyEvent>()?),
            StructType::UsbScanCodeSpecModifierType => Some(from.as_any_ref().downcast_ref::<UsbScanCodeSpecModifierType>()?),
            StructType::VirtualMachineVcpuConfig => Some(from.as_any_ref().downcast_ref::<VirtualMachineVcpuConfig>()?),
            StructType::VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo>()?),
            StructType::VirtualMachineVirtualDeviceGroups => Some(from.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceGroups>()?),
            StructType::VirtualMachineVirtualDeviceGroupsDeviceGroup => Some(from.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceGroupsDeviceGroup>()?),
            StructType::VirtualMachineVirtualDeviceGroupsVendorDeviceGroup => Some(from.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceGroupsVendorDeviceGroup>()?),
            StructType::VirtualMachineVirtualDeviceSwap => Some(from.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceSwap>()?),
            StructType::VirtualMachineVirtualDeviceSwapDeviceSwapInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceSwapDeviceSwapInfo>()?),
            StructType::VirtualHardware => Some(from.as_any_ref().downcast_ref::<VirtualHardware>()?),
            StructType::VirtualHardwareOption => Some(from.as_any_ref().downcast_ref::<VirtualHardwareOption>()?),
            StructType::VirtualMachineVirtualNuma => Some(from.as_any_ref().downcast_ref::<VirtualMachineVirtualNuma>()?),
            StructType::VirtualMachineVirtualNumaInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVirtualNumaInfo>()?),
            StructType::VirtualMachineVirtualPMem => Some(from.as_any_ref().downcast_ref::<VirtualMachineVirtualPMem>()?),
            StructType::CheckResult => Some(from.as_any_ref().downcast_ref::<CheckResult>()?),
            StructType::CustomizationAdapterMapping => Some(from.as_any_ref().downcast_ref::<CustomizationAdapterMapping>()?),
            StructType::CustomizationGlobalIpSettings => Some(from.as_any_ref().downcast_ref::<CustomizationGlobalIpSettings>()?),
            StructType::CustomizationGuiRunOnce => Some(from.as_any_ref().downcast_ref::<CustomizationGuiRunOnce>()?),
            StructType::CustomizationGuiUnattended => Some(from.as_any_ref().downcast_ref::<CustomizationGuiUnattended>()?),
            StructType::CustomizationIpSettings => Some(from.as_any_ref().downcast_ref::<CustomizationIpSettings>()?),
            StructType::CustomizationIpSettingsIpV6AddressSpec => Some(from.as_any_ref().downcast_ref::<CustomizationIpSettingsIpV6AddressSpec>()?),
            StructType::CustomizationIdentification => Some(from.as_any_ref().downcast_ref::<CustomizationIdentification>()?),
            StructType::CustomizationIdentitySettings => Some(from.as_any_ref().downcast_ref::<CustomizationIdentitySettings>()?),
            StructType::CustomizationCloudinitPrep => Some(from.as_any_ref().downcast_ref::<CustomizationCloudinitPrep>()?),
            StructType::CustomizationLinuxPrep => Some(from.as_any_ref().downcast_ref::<CustomizationLinuxPrep>()?),
            StructType::CustomizationSysprep => Some(from.as_any_ref().downcast_ref::<CustomizationSysprep>()?),
            StructType::CustomizationSysprepText => Some(from.as_any_ref().downcast_ref::<CustomizationSysprepText>()?),
            StructType::CustomizationIpGenerator => Some(from.as_any_ref().downcast_ref::<CustomizationIpGenerator>()?),
            StructType::CustomizationCustomIpGenerator => Some(from.as_any_ref().downcast_ref::<CustomizationCustomIpGenerator>()?),
            StructType::CustomizationDhcpIpGenerator => Some(from.as_any_ref().downcast_ref::<CustomizationDhcpIpGenerator>()?),
            StructType::CustomizationFixedIp => Some(from.as_any_ref().downcast_ref::<CustomizationFixedIp>()?),
            StructType::CustomizationUnknownIpGenerator => Some(from.as_any_ref().downcast_ref::<CustomizationUnknownIpGenerator>()?),
            StructType::CustomizationIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationIpV6Generator>()?),
            StructType::CustomizationAutoIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationAutoIpV6Generator>()?),
            StructType::CustomizationCustomIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationCustomIpV6Generator>()?),
            StructType::CustomizationDhcpIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationDhcpIpV6Generator>()?),
            StructType::CustomizationFixedIpV6 => Some(from.as_any_ref().downcast_ref::<CustomizationFixedIpV6>()?),
            StructType::CustomizationStatelessIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationStatelessIpV6Generator>()?),
            StructType::CustomizationUnknownIpV6Generator => Some(from.as_any_ref().downcast_ref::<CustomizationUnknownIpV6Generator>()?),
            StructType::CustomizationLicenseFilePrintData => Some(from.as_any_ref().downcast_ref::<CustomizationLicenseFilePrintData>()?),
            StructType::CustomizationName => Some(from.as_any_ref().downcast_ref::<CustomizationName>()?),
            StructType::CustomizationCustomName => Some(from.as_any_ref().downcast_ref::<CustomizationCustomName>()?),
            StructType::CustomizationFixedName => Some(from.as_any_ref().downcast_ref::<CustomizationFixedName>()?),
            StructType::CustomizationPrefixName => Some(from.as_any_ref().downcast_ref::<CustomizationPrefixName>()?),
            StructType::CustomizationUnknownName => Some(from.as_any_ref().downcast_ref::<CustomizationUnknownName>()?),
            StructType::CustomizationVirtualMachineName => Some(from.as_any_ref().downcast_ref::<CustomizationVirtualMachineName>()?),
            StructType::CustomizationOptions => Some(from.as_any_ref().downcast_ref::<CustomizationOptions>()?),
            StructType::CustomizationLinuxOptions => Some(from.as_any_ref().downcast_ref::<CustomizationLinuxOptions>()?),
            StructType::CustomizationWinOptions => Some(from.as_any_ref().downcast_ref::<CustomizationWinOptions>()?),
            StructType::CustomizationPassword => Some(from.as_any_ref().downcast_ref::<CustomizationPassword>()?),
            StructType::CustomizationSpec => Some(from.as_any_ref().downcast_ref::<CustomizationSpec>()?),
            StructType::CustomizationUserData => Some(from.as_any_ref().downcast_ref::<CustomizationUserData>()?),
            StructType::HostDiskMappingInfo => Some(from.as_any_ref().downcast_ref::<HostDiskMappingInfo>()?),
            StructType::HostDiskMappingPartitionInfo => Some(from.as_any_ref().downcast_ref::<HostDiskMappingPartitionInfo>()?),
            StructType::HostDiskMappingOption => Some(from.as_any_ref().downcast_ref::<HostDiskMappingOption>()?),
            StructType::HostDiskMappingPartitionOption => Some(from.as_any_ref().downcast_ref::<HostDiskMappingPartitionOption>()?),
            StructType::VirtualDevice => Some(from.as_any_ref().downcast_ref::<VirtualDevice>()?),
            StructType::VirtualCdrom => Some(from.as_any_ref().downcast_ref::<VirtualCdrom>()?),
            StructType::VirtualController => Some(from.as_any_ref().downcast_ref::<VirtualController>()?),
            StructType::VirtualIdeController => Some(from.as_any_ref().downcast_ref::<VirtualIdeController>()?),
            StructType::VirtualNvdimmController => Some(from.as_any_ref().downcast_ref::<VirtualNvdimmController>()?),
            StructType::VirtualNvmeController => Some(from.as_any_ref().downcast_ref::<VirtualNvmeController>()?),
            StructType::VirtualPciController => Some(from.as_any_ref().downcast_ref::<VirtualPciController>()?),
            StructType::VirtualPs2Controller => Some(from.as_any_ref().downcast_ref::<VirtualPs2Controller>()?),
            StructType::VirtualSataController => Some(from.as_any_ref().downcast_ref::<VirtualSataController>()?),
            StructType::VirtualAhciController => Some(from.as_any_ref().downcast_ref::<VirtualAhciController>()?),
            StructType::VirtualScsiController => Some(from.as_any_ref().downcast_ref::<VirtualScsiController>()?),
            StructType::ParaVirtualScsiController => Some(from.as_any_ref().downcast_ref::<ParaVirtualScsiController>()?),
            StructType::VirtualBusLogicController => Some(from.as_any_ref().downcast_ref::<VirtualBusLogicController>()?),
            StructType::VirtualLsiLogicController => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicController>()?),
            StructType::VirtualLsiLogicSasController => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicSasController>()?),
            StructType::VirtualSioController => Some(from.as_any_ref().downcast_ref::<VirtualSioController>()?),
            StructType::VirtualUsbController => Some(from.as_any_ref().downcast_ref::<VirtualUsbController>()?),
            StructType::VirtualUsbxhciController => Some(from.as_any_ref().downcast_ref::<VirtualUsbxhciController>()?),
            StructType::VirtualDisk => Some(from.as_any_ref().downcast_ref::<VirtualDisk>()?),
            StructType::VirtualEthernetCard => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCard>()?),
            StructType::VirtualE1000 => Some(from.as_any_ref().downcast_ref::<VirtualE1000>()?),
            StructType::VirtualE1000E => Some(from.as_any_ref().downcast_ref::<VirtualE1000E>()?),
            StructType::VirtualPcNet32 => Some(from.as_any_ref().downcast_ref::<VirtualPcNet32>()?),
            StructType::VirtualSriovEthernetCard => Some(from.as_any_ref().downcast_ref::<VirtualSriovEthernetCard>()?),
            StructType::VirtualVmxnet => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet>()?),
            StructType::VirtualVmxnet2 => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet2>()?),
            StructType::VirtualVmxnet3 => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3>()?),
            StructType::VirtualVmxnet3Vrdma => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3Vrdma>()?),
            StructType::VirtualFloppy => Some(from.as_any_ref().downcast_ref::<VirtualFloppy>()?),
            StructType::VirtualKeyboard => Some(from.as_any_ref().downcast_ref::<VirtualKeyboard>()?),
            StructType::VirtualNvdimm => Some(from.as_any_ref().downcast_ref::<VirtualNvdimm>()?),
            StructType::VirtualPciPassthrough => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthrough>()?),
            StructType::VirtualParallelPort => Some(from.as_any_ref().downcast_ref::<VirtualParallelPort>()?),
            StructType::VirtualPointingDevice => Some(from.as_any_ref().downcast_ref::<VirtualPointingDevice>()?),
            StructType::VirtualPrecisionClock => Some(from.as_any_ref().downcast_ref::<VirtualPrecisionClock>()?),
            StructType::VirtualScsiPassthrough => Some(from.as_any_ref().downcast_ref::<VirtualScsiPassthrough>()?),
            StructType::VirtualSerialPort => Some(from.as_any_ref().downcast_ref::<VirtualSerialPort>()?),
            StructType::VirtualSoundCard => Some(from.as_any_ref().downcast_ref::<VirtualSoundCard>()?),
            StructType::VirtualEnsoniq1371 => Some(from.as_any_ref().downcast_ref::<VirtualEnsoniq1371>()?),
            StructType::VirtualHdAudioCard => Some(from.as_any_ref().downcast_ref::<VirtualHdAudioCard>()?),
            StructType::VirtualSoundBlaster16 => Some(from.as_any_ref().downcast_ref::<VirtualSoundBlaster16>()?),
            StructType::VirtualTpm => Some(from.as_any_ref().downcast_ref::<VirtualTpm>()?),
            StructType::VirtualUsb => Some(from.as_any_ref().downcast_ref::<VirtualUsb>()?),
            StructType::VirtualMachineVmciDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineVmciDevice>()?),
            StructType::VirtualMachineVmirom => Some(from.as_any_ref().downcast_ref::<VirtualMachineVmirom>()?),
            StructType::VirtualMachineVideoCard => Some(from.as_any_ref().downcast_ref::<VirtualMachineVideoCard>()?),
            StructType::VirtualWdt => Some(from.as_any_ref().downcast_ref::<VirtualWdt>()?),
            StructType::VirtualDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceBackingInfo>()?),
            StructType::VirtualDeviceDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceDeviceBackingInfo>()?),
            StructType::VirtualCdromAtapiBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromAtapiBackingInfo>()?),
            StructType::VirtualCdromPassthroughBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromPassthroughBackingInfo>()?),
            StructType::VirtualDiskRawDiskVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskRawDiskVer2BackingInfo>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskPartitionedRawDiskVer2BackingInfo>()?),
            StructType::VirtualEthernetCardLegacyNetworkBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardLegacyNetworkBackingInfo>()?),
            StructType::VirtualEthernetCardNetworkBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardNetworkBackingInfo>()?),
            StructType::VirtualFloppyDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualFloppyDeviceBackingInfo>()?),
            StructType::VirtualPciPassthroughDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDeviceBackingInfo>()?),
            StructType::VirtualPciPassthroughDynamicBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDynamicBackingInfo>()?),
            StructType::VirtualParallelPortDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortDeviceBackingInfo>()?),
            StructType::VirtualPointingDeviceDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPointingDeviceDeviceBackingInfo>()?),
            StructType::VirtualScsiPassthroughDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualScsiPassthroughDeviceBackingInfo>()?),
            StructType::VirtualSerialPortDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortDeviceBackingInfo>()?),
            StructType::VirtualSoundCardDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSoundCardDeviceBackingInfo>()?),
            StructType::VirtualUsbRemoteHostBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualUsbRemoteHostBackingInfo>()?),
            StructType::VirtualUsbusbBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualUsbusbBackingInfo>()?),
            StructType::VirtualDeviceFileBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceFileBackingInfo>()?),
            StructType::VirtualCdromIsoBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromIsoBackingInfo>()?),
            StructType::VirtualDiskFlatVer1BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer1BackingInfo>()?),
            StructType::VirtualDiskFlatVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer2BackingInfo>()?),
            StructType::VirtualDiskLocalPMemBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskLocalPMemBackingInfo>()?),
            StructType::VirtualDiskRawDiskMappingVer1BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskRawDiskMappingVer1BackingInfo>()?),
            StructType::VirtualDiskSeSparseBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskSeSparseBackingInfo>()?),
            StructType::VirtualDiskSparseVer1BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer1BackingInfo>()?),
            StructType::VirtualDiskSparseVer2BackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer2BackingInfo>()?),
            StructType::VirtualFloppyImageBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualFloppyImageBackingInfo>()?),
            StructType::VirtualNvdimmBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualNvdimmBackingInfo>()?),
            StructType::VirtualParallelPortFileBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortFileBackingInfo>()?),
            StructType::VirtualSerialPortFileBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortFileBackingInfo>()?),
            StructType::VirtualDevicePipeBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDevicePipeBackingInfo>()?),
            StructType::VirtualSerialPortPipeBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortPipeBackingInfo>()?),
            StructType::VirtualDeviceRemoteDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceRemoteDeviceBackingInfo>()?),
            StructType::VirtualCdromRemoteAtapiBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromRemoteAtapiBackingInfo>()?),
            StructType::VirtualCdromRemotePassthroughBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualCdromRemotePassthroughBackingInfo>()?),
            StructType::VirtualFloppyRemoteDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualFloppyRemoteDeviceBackingInfo>()?),
            StructType::VirtualUsbRemoteClientBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualUsbRemoteClientBackingInfo>()?),
            StructType::VirtualDeviceUriBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceUriBackingInfo>()?),
            StructType::VirtualSerialPortUriBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortUriBackingInfo>()?),
            StructType::VirtualEthernetCardDistributedVirtualPortBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardDistributedVirtualPortBackingInfo>()?),
            StructType::VirtualEthernetCardOpaqueNetworkBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardOpaqueNetworkBackingInfo>()?),
            StructType::VirtualPciPassthroughDvxBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDvxBackingInfo>()?),
            StructType::VirtualPciPassthroughPluginBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughPluginBackingInfo>()?),
            StructType::VirtualPciPassthroughVmiopBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughVmiopBackingInfo>()?),
            StructType::VirtualPrecisionClockSystemClockBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPrecisionClockSystemClockBackingInfo>()?),
            StructType::VirtualSerialPortThinPrintBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortThinPrintBackingInfo>()?),
            StructType::VirtualSriovEthernetCardSriovBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSriovEthernetCardSriovBackingInfo>()?),
            StructType::VirtualDeviceBusSlotInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceBusSlotInfo>()?),
            StructType::VirtualDevicePciBusSlotInfo => Some(from.as_any_ref().downcast_ref::<VirtualDevicePciBusSlotInfo>()?),
            StructType::VirtualUsbControllerPciBusSlotInfo => Some(from.as_any_ref().downcast_ref::<VirtualUsbControllerPciBusSlotInfo>()?),
            StructType::VirtualDeviceConnectInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceConnectInfo>()?),
            StructType::VirtualDeviceDeviceGroupInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceDeviceGroupInfo>()?),
            StructType::VirtualDeviceOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceOption>()?),
            StructType::VirtualCdromOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromOption>()?),
            StructType::VirtualControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualControllerOption>()?),
            StructType::VirtualIdeControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualIdeControllerOption>()?),
            StructType::VirtualNvdimmControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualNvdimmControllerOption>()?),
            StructType::VirtualNvmeControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualNvmeControllerOption>()?),
            StructType::VirtualPciControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualPciControllerOption>()?),
            StructType::VirtualPs2ControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualPs2ControllerOption>()?),
            StructType::VirtualSataControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualSataControllerOption>()?),
            StructType::VirtualAhciControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualAhciControllerOption>()?),
            StructType::VirtualScsiControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualScsiControllerOption>()?),
            StructType::ParaVirtualScsiControllerOption => Some(from.as_any_ref().downcast_ref::<ParaVirtualScsiControllerOption>()?),
            StructType::VirtualBusLogicControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualBusLogicControllerOption>()?),
            StructType::VirtualLsiLogicControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicControllerOption>()?),
            StructType::VirtualLsiLogicSasControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicSasControllerOption>()?),
            StructType::VirtualSioControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualSioControllerOption>()?),
            StructType::VirtualUsbControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbControllerOption>()?),
            StructType::VirtualUsbxhciControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbxhciControllerOption>()?),
            StructType::VirtualDiskOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskOption>()?),
            StructType::VirtualEthernetCardOption => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardOption>()?),
            StructType::VirtualE1000Option => Some(from.as_any_ref().downcast_ref::<VirtualE1000Option>()?),
            StructType::VirtualE1000EOption => Some(from.as_any_ref().downcast_ref::<VirtualE1000EOption>()?),
            StructType::VirtualPcNet32Option => Some(from.as_any_ref().downcast_ref::<VirtualPcNet32Option>()?),
            StructType::VirtualSriovEthernetCardOption => Some(from.as_any_ref().downcast_ref::<VirtualSriovEthernetCardOption>()?),
            StructType::VirtualVmxnetOption => Some(from.as_any_ref().downcast_ref::<VirtualVmxnetOption>()?),
            StructType::VirtualVmxnet2Option => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet2Option>()?),
            StructType::VirtualVmxnet3Option => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3Option>()?),
            StructType::VirtualVmxnet3VrdmaOption => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3VrdmaOption>()?),
            StructType::VirtualFloppyOption => Some(from.as_any_ref().downcast_ref::<VirtualFloppyOption>()?),
            StructType::VirtualKeyboardOption => Some(from.as_any_ref().downcast_ref::<VirtualKeyboardOption>()?),
            StructType::VirtualNvdimmOption => Some(from.as_any_ref().downcast_ref::<VirtualNvdimmOption>()?),
            StructType::VirtualPciPassthroughOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughOption>()?),
            StructType::VirtualParallelPortOption => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortOption>()?),
            StructType::VirtualPointingDeviceOption => Some(from.as_any_ref().downcast_ref::<VirtualPointingDeviceOption>()?),
            StructType::VirtualPrecisionClockOption => Some(from.as_any_ref().downcast_ref::<VirtualPrecisionClockOption>()?),
            StructType::VirtualScsiPassthroughOption => Some(from.as_any_ref().downcast_ref::<VirtualScsiPassthroughOption>()?),
            StructType::VirtualSerialPortOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortOption>()?),
            StructType::VirtualSoundCardOption => Some(from.as_any_ref().downcast_ref::<VirtualSoundCardOption>()?),
            StructType::VirtualEnsoniq1371Option => Some(from.as_any_ref().downcast_ref::<VirtualEnsoniq1371Option>()?),
            StructType::VirtualHdAudioCardOption => Some(from.as_any_ref().downcast_ref::<VirtualHdAudioCardOption>()?),
            StructType::VirtualSoundBlaster16Option => Some(from.as_any_ref().downcast_ref::<VirtualSoundBlaster16Option>()?),
            StructType::VirtualTpmOption => Some(from.as_any_ref().downcast_ref::<VirtualTpmOption>()?),
            StructType::VirtualUsbOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbOption>()?),
            StructType::VirtualMachineVmciDeviceOption => Some(from.as_any_ref().downcast_ref::<VirtualMachineVmciDeviceOption>()?),
            StructType::VirtualVmiromOption => Some(from.as_any_ref().downcast_ref::<VirtualVmiromOption>()?),
            StructType::VirtualVideoCardOption => Some(from.as_any_ref().downcast_ref::<VirtualVideoCardOption>()?),
            StructType::VirtualWdtOption => Some(from.as_any_ref().downcast_ref::<VirtualWdtOption>()?),
            StructType::VirtualDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceBackingOption>()?),
            StructType::VirtualDeviceDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceDeviceBackingOption>()?),
            StructType::VirtualCdromAtapiBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromAtapiBackingOption>()?),
            StructType::VirtualCdromPassthroughBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromPassthroughBackingOption>()?),
            StructType::VirtualCdromRemoteAtapiBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromRemoteAtapiBackingOption>()?),
            StructType::VirtualDiskRawDiskMappingVer1BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskRawDiskMappingVer1BackingOption>()?),
            StructType::VirtualDiskRawDiskVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskRawDiskVer2BackingOption>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskPartitionedRawDiskVer2BackingOption>()?),
            StructType::VirtualEthernetCardLegacyNetworkBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardLegacyNetworkBackingOption>()?),
            StructType::VirtualEthernetCardNetworkBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardNetworkBackingOption>()?),
            StructType::VirtualFloppyDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualFloppyDeviceBackingOption>()?),
            StructType::VirtualPciPassthroughDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDeviceBackingOption>()?),
            StructType::VirtualPciPassthroughDynamicBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDynamicBackingOption>()?),
            StructType::VirtualParallelPortDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortDeviceBackingOption>()?),
            StructType::VirtualPointingDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPointingDeviceBackingOption>()?),
            StructType::VirtualScsiPassthroughDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualScsiPassthroughDeviceBackingOption>()?),
            StructType::VirtualSerialPortDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortDeviceBackingOption>()?),
            StructType::VirtualSoundCardDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSoundCardDeviceBackingOption>()?),
            StructType::VirtualUsbRemoteHostBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbRemoteHostBackingOption>()?),
            StructType::VirtualUsbusbBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbusbBackingOption>()?),
            StructType::VirtualDeviceFileBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceFileBackingOption>()?),
            StructType::VirtualCdromIsoBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromIsoBackingOption>()?),
            StructType::VirtualDiskFlatVer1BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer1BackingOption>()?),
            StructType::VirtualDiskFlatVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskFlatVer2BackingOption>()?),
            StructType::VirtualDiskLocalPMemBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskLocalPMemBackingOption>()?),
            StructType::VirtualDiskSeSparseBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskSeSparseBackingOption>()?),
            StructType::VirtualDiskSparseVer1BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer1BackingOption>()?),
            StructType::VirtualDiskSparseVer2BackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskSparseVer2BackingOption>()?),
            StructType::VirtualFloppyImageBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualFloppyImageBackingOption>()?),
            StructType::VirtualParallelPortFileBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortFileBackingOption>()?),
            StructType::VirtualSerialPortFileBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortFileBackingOption>()?),
            StructType::VirtualDevicePipeBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDevicePipeBackingOption>()?),
            StructType::VirtualSerialPortPipeBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortPipeBackingOption>()?),
            StructType::VirtualDeviceRemoteDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceRemoteDeviceBackingOption>()?),
            StructType::VirtualCdromRemotePassthroughBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromRemotePassthroughBackingOption>()?),
            StructType::VirtualFloppyRemoteDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualFloppyRemoteDeviceBackingOption>()?),
            StructType::VirtualUsbRemoteClientBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbRemoteClientBackingOption>()?),
            StructType::VirtualDeviceUriBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceUriBackingOption>()?),
            StructType::VirtualSerialPortUriBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortUriBackingOption>()?),
            StructType::VirtualEthernetCardDvPortBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardDvPortBackingOption>()?),
            StructType::VirtualEthernetCardOpaqueNetworkBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardOpaqueNetworkBackingOption>()?),
            StructType::VirtualPciPassthroughDvxBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughDvxBackingOption>()?),
            StructType::VirtualPciPassthroughPluginBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughPluginBackingOption>()?),
            StructType::VirtualPciPassthroughVmiopBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughVmiopBackingOption>()?),
            StructType::VirtualPrecisionClockSystemClockBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPrecisionClockSystemClockBackingOption>()?),
            StructType::VirtualSerialPortThinPrintBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortThinPrintBackingOption>()?),
            StructType::VirtualSriovEthernetCardSriovBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSriovEthernetCardSriovBackingOption>()?),
            StructType::VirtualDeviceBusSlotOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceBusSlotOption>()?),
            StructType::VirtualDeviceConnectOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceConnectOption>()?),
            StructType::VirtualDeviceConfigSpec => Some(from.as_any_ref().downcast_ref::<VirtualDeviceConfigSpec>()?),
            StructType::VirtualDiskConfigSpec => Some(from.as_any_ref().downcast_ref::<VirtualDiskConfigSpec>()?),
            StructType::VirtualDeviceConfigSpecBackingSpec => Some(from.as_any_ref().downcast_ref::<VirtualDeviceConfigSpecBackingSpec>()?),
            StructType::VirtualDiskVFlashCacheConfigInfo => Some(from.as_any_ref().downcast_ref::<VirtualDiskVFlashCacheConfigInfo>()?),
            StructType::VirtualDiskId => Some(from.as_any_ref().downcast_ref::<VirtualDiskId>()?),
            StructType::VirtualDiskDeltaDiskFormatsSupported => Some(from.as_any_ref().downcast_ref::<VirtualDiskDeltaDiskFormatsSupported>()?),
            StructType::VirtualDiskOptionVFlashCacheConfigOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskOptionVFlashCacheConfigOption>()?),
            StructType::VirtualEthernetCardResourceAllocation => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardResourceAllocation>()?),
            StructType::VirtualPciPassthroughAllowedDevice => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughAllowedDevice>()?),
            StructType::VirtualMachineVmciDeviceFilterInfo => Some(from.as_any_ref().downcast_ref::<VirtualMachineVmciDeviceFilterInfo>()?),
            StructType::VirtualMachineVmciDeviceFilterSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineVmciDeviceFilterSpec>()?),
            StructType::VirtualMachineVmciDeviceOptionFilterSpecOption => Some(from.as_any_ref().downcast_ref::<VirtualMachineVmciDeviceOptionFilterSpecOption>()?),
            StructType::GuestAliases => Some(from.as_any_ref().downcast_ref::<GuestAliases>()?),
            StructType::GuestAuthAliasInfo => Some(from.as_any_ref().downcast_ref::<GuestAuthAliasInfo>()?),
            StructType::GuestAuthSubject => Some(from.as_any_ref().downcast_ref::<GuestAuthSubject>()?),
            StructType::GuestAuthAnySubject => Some(from.as_any_ref().downcast_ref::<GuestAuthAnySubject>()?),
            StructType::GuestAuthNamedSubject => Some(from.as_any_ref().downcast_ref::<GuestAuthNamedSubject>()?),
            StructType::GuestMappedAliases => Some(from.as_any_ref().downcast_ref::<GuestMappedAliases>()?),
            StructType::GuestFileAttributes => Some(from.as_any_ref().downcast_ref::<GuestFileAttributes>()?),
            StructType::GuestPosixFileAttributes => Some(from.as_any_ref().downcast_ref::<GuestPosixFileAttributes>()?),
            StructType::GuestWindowsFileAttributes => Some(from.as_any_ref().downcast_ref::<GuestWindowsFileAttributes>()?),
            StructType::GuestFileInfo => Some(from.as_any_ref().downcast_ref::<GuestFileInfo>()?),
            StructType::FileTransferInformation => Some(from.as_any_ref().downcast_ref::<FileTransferInformation>()?),
            StructType::GuestListFileInfo => Some(from.as_any_ref().downcast_ref::<GuestListFileInfo>()?),
            StructType::GuestAuthentication => Some(from.as_any_ref().downcast_ref::<GuestAuthentication>()?),
            StructType::NamePasswordAuthentication => Some(from.as_any_ref().downcast_ref::<NamePasswordAuthentication>()?),
            StructType::SamlTokenAuthentication => Some(from.as_any_ref().downcast_ref::<SamlTokenAuthentication>()?),
            StructType::SspiAuthentication => Some(from.as_any_ref().downcast_ref::<SspiAuthentication>()?),
            StructType::TicketedSessionAuthentication => Some(from.as_any_ref().downcast_ref::<TicketedSessionAuthentication>()?),
            StructType::GuestProcessInfo => Some(from.as_any_ref().downcast_ref::<GuestProcessInfo>()?),
            StructType::GuestProgramSpec => Some(from.as_any_ref().downcast_ref::<GuestProgramSpec>()?),
            StructType::GuestWindowsProgramSpec => Some(from.as_any_ref().downcast_ref::<GuestWindowsProgramSpec>()?),
            StructType::GuestRegKeySpec => Some(from.as_any_ref().downcast_ref::<GuestRegKeySpec>()?),
            StructType::GuestRegKeyNameSpec => Some(from.as_any_ref().downcast_ref::<GuestRegKeyNameSpec>()?),
            StructType::GuestRegKeyRecordSpec => Some(from.as_any_ref().downcast_ref::<GuestRegKeyRecordSpec>()?),
            StructType::GuestRegValueSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueSpec>()?),
            StructType::GuestRegValueDataSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueDataSpec>()?),
            StructType::GuestRegValueBinarySpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueBinarySpec>()?),
            StructType::GuestRegValueDwordSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueDwordSpec>()?),
            StructType::GuestRegValueExpandStringSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueExpandStringSpec>()?),
            StructType::GuestRegValueMultiStringSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueMultiStringSpec>()?),
            StructType::GuestRegValueQwordSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueQwordSpec>()?),
            StructType::GuestRegValueStringSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueStringSpec>()?),
            StructType::GuestRegValueNameSpec => Some(from.as_any_ref().downcast_ref::<GuestRegValueNameSpec>()?),
            StructType::DeviceGroupId => Some(from.as_any_ref().downcast_ref::<DeviceGroupId>()?),
            StructType::FaultDomainId => Some(from.as_any_ref().downcast_ref::<FaultDomainId>()?),
            StructType::ReplicationGroupId => Some(from.as_any_ref().downcast_ref::<ReplicationGroupId>()?),
            StructType::ReplicationSpec => Some(from.as_any_ref().downcast_ref::<ReplicationSpec>()?),
            StructType::VsanClusterConfigInfo => Some(from.as_any_ref().downcast_ref::<VsanClusterConfigInfo>()?),
            StructType::VsanClusterConfigInfoHostDefaultInfo => Some(from.as_any_ref().downcast_ref::<VsanClusterConfigInfoHostDefaultInfo>()?),
            StructType::VsanHostClusterStatus => Some(from.as_any_ref().downcast_ref::<VsanHostClusterStatus>()?),
            StructType::VsanHostClusterStatusState => Some(from.as_any_ref().downcast_ref::<VsanHostClusterStatusState>()?),
            StructType::VsanHostClusterStatusStateCompletionEstimate => Some(from.as_any_ref().downcast_ref::<VsanHostClusterStatusStateCompletionEstimate>()?),
            StructType::VsanHostConfigInfo => Some(from.as_any_ref().downcast_ref::<VsanHostConfigInfo>()?),
            StructType::VsanHostConfigInfoClusterInfo => Some(from.as_any_ref().downcast_ref::<VsanHostConfigInfoClusterInfo>()?),
            StructType::VsanHostFaultDomainInfo => Some(from.as_any_ref().downcast_ref::<VsanHostFaultDomainInfo>()?),
            StructType::VsanHostConfigInfoNetworkInfo => Some(from.as_any_ref().downcast_ref::<VsanHostConfigInfoNetworkInfo>()?),
            StructType::VsanHostConfigInfoNetworkInfoPortConfig => Some(from.as_any_ref().downcast_ref::<VsanHostConfigInfoNetworkInfoPortConfig>()?),
            StructType::VsanHostConfigInfoStorageInfo => Some(from.as_any_ref().downcast_ref::<VsanHostConfigInfoStorageInfo>()?),
            StructType::VsanHostDecommissionMode => Some(from.as_any_ref().downcast_ref::<VsanHostDecommissionMode>()?),
            StructType::VsanHostDiskMapInfo => Some(from.as_any_ref().downcast_ref::<VsanHostDiskMapInfo>()?),
            StructType::VsanHostDiskMapResult => Some(from.as_any_ref().downcast_ref::<VsanHostDiskMapResult>()?),
            StructType::VsanHostDiskMapping => Some(from.as_any_ref().downcast_ref::<VsanHostDiskMapping>()?),
            StructType::VsanHostDiskResult => Some(from.as_any_ref().downcast_ref::<VsanHostDiskResult>()?),
            StructType::VsanHostIpConfig => Some(from.as_any_ref().downcast_ref::<VsanHostIpConfig>()?),
            StructType::VsanHostMembershipInfo => Some(from.as_any_ref().downcast_ref::<VsanHostMembershipInfo>()?),
            StructType::VsanHostVsanDiskInfo => Some(from.as_any_ref().downcast_ref::<VsanHostVsanDiskInfo>()?),
            StructType::VsanHostRuntimeInfo => Some(from.as_any_ref().downcast_ref::<VsanHostRuntimeInfo>()?),
            StructType::VsanHostRuntimeInfoDiskIssue => Some(from.as_any_ref().downcast_ref::<VsanHostRuntimeInfoDiskIssue>()?),
            StructType::BaseConfigInfo => Some(from.as_any_ref().downcast_ref::<BaseConfigInfo>()?),
            StructType::VStorageObjectConfigInfo => Some(from.as_any_ref().downcast_ref::<VStorageObjectConfigInfo>()?),
            StructType::BaseConfigInfoBackingInfo => Some(from.as_any_ref().downcast_ref::<BaseConfigInfoBackingInfo>()?),
            StructType::BaseConfigInfoFileBackingInfo => Some(from.as_any_ref().downcast_ref::<BaseConfigInfoFileBackingInfo>()?),
            StructType::BaseConfigInfoDiskFileBackingInfo => Some(from.as_any_ref().downcast_ref::<BaseConfigInfoDiskFileBackingInfo>()?),
            StructType::BaseConfigInfoRawDiskMappingBackingInfo => Some(from.as_any_ref().downcast_ref::<BaseConfigInfoRawDiskMappingBackingInfo>()?),
            StructType::VslmCreateSpec => Some(from.as_any_ref().downcast_ref::<VslmCreateSpec>()?),
            StructType::VslmCreateSpecBackingSpec => Some(from.as_any_ref().downcast_ref::<VslmCreateSpecBackingSpec>()?),
            StructType::VslmCreateSpecDiskFileBackingSpec => Some(from.as_any_ref().downcast_ref::<VslmCreateSpecDiskFileBackingSpec>()?),
            StructType::VslmCreateSpecRawDiskMappingBackingSpec => Some(from.as_any_ref().downcast_ref::<VslmCreateSpecRawDiskMappingBackingSpec>()?),
            StructType::DiskCryptoSpec => Some(from.as_any_ref().downcast_ref::<DiskCryptoSpec>()?),
            StructType::Id => Some(from.as_any_ref().downcast_ref::<Id>()?),
            StructType::VslmInfrastructureObjectPolicy => Some(from.as_any_ref().downcast_ref::<VslmInfrastructureObjectPolicy>()?),
            StructType::VslmInfrastructureObjectPolicySpec => Some(from.as_any_ref().downcast_ref::<VslmInfrastructureObjectPolicySpec>()?),
            StructType::VslmMigrateSpec => Some(from.as_any_ref().downcast_ref::<VslmMigrateSpec>()?),
            StructType::VslmCloneSpec => Some(from.as_any_ref().downcast_ref::<VslmCloneSpec>()?),
            StructType::VslmRelocateSpec => Some(from.as_any_ref().downcast_ref::<VslmRelocateSpec>()?),
            StructType::VStorageObjectStateInfo => Some(from.as_any_ref().downcast_ref::<VStorageObjectStateInfo>()?),
            StructType::VslmTagEntry => Some(from.as_any_ref().downcast_ref::<VslmTagEntry>()?),
            StructType::VslmVClockInfo => Some(from.as_any_ref().downcast_ref::<VslmVClockInfo>()?),
            StructType::VStorageObject => Some(from.as_any_ref().downcast_ref::<VStorageObject>()?),
            StructType::VStorageObjectSnapshot => Some(from.as_any_ref().downcast_ref::<VStorageObjectSnapshot>()?),
            StructType::VStorageObjectSnapshotDetails => Some(from.as_any_ref().downcast_ref::<VStorageObjectSnapshotDetails>()?),
            StructType::VStorageObjectSnapshotInfo => Some(from.as_any_ref().downcast_ref::<VStorageObjectSnapshotInfo>()?),
            StructType::VStorageObjectSnapshotInfoVStorageObjectSnapshot => Some(from.as_any_ref().downcast_ref::<VStorageObjectSnapshotInfoVStorageObjectSnapshot>()?),
            StructType::RetrieveVStorageObjSpec => Some(from.as_any_ref().downcast_ref::<RetrieveVStorageObjSpec>()?),
            StructType::VStorageObjectAssociations => Some(from.as_any_ref().downcast_ref::<VStorageObjectAssociations>()?),
            StructType::VStorageObjectAssociationsVmDiskAssociations => Some(from.as_any_ref().downcast_ref::<VStorageObjectAssociationsVmDiskAssociations>()?),
            StructType::DynamicArray => Some(from.as_any_ref().downcast_ref::<DynamicArray>()?),
            StructType::DynamicProperty => Some(from.as_any_ref().downcast_ref::<DynamicProperty>()?),
            StructType::KeyAnyValue => Some(from.as_any_ref().downcast_ref::<KeyAnyValue>()?),
            StructType::LocalizableMessage => Some(from.as_any_ref().downcast_ref::<LocalizableMessage>()?),
            StructType::LocalizedMethodFault => Some(from.as_any_ref().downcast_ref::<LocalizedMethodFault>()?),
            StructType::PropertyChange => Some(from.as_any_ref().downcast_ref::<PropertyChange>()?),
            StructType::PropertyFilterSpec => Some(from.as_any_ref().downcast_ref::<PropertyFilterSpec>()?),
            StructType::PropertyFilterUpdate => Some(from.as_any_ref().downcast_ref::<PropertyFilterUpdate>()?),
            StructType::MissingObject => Some(from.as_any_ref().downcast_ref::<MissingObject>()?),
            StructType::MissingProperty => Some(from.as_any_ref().downcast_ref::<MissingProperty>()?),
            StructType::ObjectContent => Some(from.as_any_ref().downcast_ref::<ObjectContent>()?),
            StructType::ObjectSpec => Some(from.as_any_ref().downcast_ref::<ObjectSpec>()?),
            StructType::ObjectUpdate => Some(from.as_any_ref().downcast_ref::<ObjectUpdate>()?),
            StructType::PropertySpec => Some(from.as_any_ref().downcast_ref::<PropertySpec>()?),
            StructType::RetrieveOptions => Some(from.as_any_ref().downcast_ref::<RetrieveOptions>()?),
            StructType::RetrieveResult => Some(from.as_any_ref().downcast_ref::<RetrieveResult>()?),
            StructType::SelectionSpec => Some(from.as_any_ref().downcast_ref::<SelectionSpec>()?),
            StructType::TraversalSpec => Some(from.as_any_ref().downcast_ref::<TraversalSpec>()?),
            StructType::UpdateSet => Some(from.as_any_ref().downcast_ref::<UpdateSet>()?),
            StructType::WaitOptions => Some(from.as_any_ref().downcast_ref::<WaitOptions>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DataObject => Ok(from.as_any_box().downcast::<DataObject>()?),
            StructType::AboutInfo => Ok(from.as_any_box().downcast::<AboutInfo>()?),
            StructType::AuthorizationDescription => Ok(from.as_any_box().downcast::<AuthorizationDescription>()?),
            StructType::EntityPrivilege => Ok(from.as_any_box().downcast::<EntityPrivilege>()?),
            StructType::Permission => Ok(from.as_any_box().downcast::<Permission>()?),
            StructType::AuthorizationPrivilege => Ok(from.as_any_box().downcast::<AuthorizationPrivilege>()?),
            StructType::PrivilegeAvailability => Ok(from.as_any_box().downcast::<PrivilegeAvailability>()?),
            StructType::AuthorizationRole => Ok(from.as_any_box().downcast::<AuthorizationRole>()?),
            StructType::UserPrivilegeResult => Ok(from.as_any_box().downcast::<UserPrivilegeResult>()?),
            StructType::BatchResult => Ok(from.as_any_box().downcast::<BatchResult>()?),
            StructType::Capability => Ok(from.as_any_box().downcast::<Capability>()?),
            StructType::ClusterComputeResourceClusterConfigResult => Ok(from.as_any_box().downcast::<ClusterComputeResourceClusterConfigResult>()?),
            StructType::ClusterComputeResourceDvsSetting => Ok(from.as_any_box().downcast::<ClusterComputeResourceDvsSetting>()?),
            StructType::ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping => Ok(from.as_any_box().downcast::<ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping>()?),
            StructType::ClusterComputeResourceDvsProfile => Ok(from.as_any_box().downcast::<ClusterComputeResourceDvsProfile>()?),
            StructType::ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping => Ok(from.as_any_box().downcast::<ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping>()?),
            StructType::ClusterComputeResourceHciConfigInfo => Ok(from.as_any_box().downcast::<ClusterComputeResourceHciConfigInfo>()?),
            StructType::ClusterComputeResourceHciConfigSpec => Ok(from.as_any_box().downcast::<ClusterComputeResourceHciConfigSpec>()?),
            StructType::ClusterComputeResourceHostConfigurationInput => Ok(from.as_any_box().downcast::<ClusterComputeResourceHostConfigurationInput>()?),
            StructType::ClusterComputeResourceHostConfigurationProfile => Ok(from.as_any_box().downcast::<ClusterComputeResourceHostConfigurationProfile>()?),
            StructType::ClusterComputeResourceHostVmkNicInfo => Ok(from.as_any_box().downcast::<ClusterComputeResourceHostVmkNicInfo>()?),
            StructType::ClusterComputeResourceVcProfile => Ok(from.as_any_box().downcast::<ClusterComputeResourceVcProfile>()?),
            StructType::ClusterComputeResourceValidationResultBase => Ok(from.as_any_box().downcast::<ClusterComputeResourceValidationResultBase>()?),
            StructType::ClusterComputeResourceDvsConfigurationValidation => Ok(from.as_any_box().downcast::<ClusterComputeResourceDvsConfigurationValidation>()?),
            StructType::ClusterComputeResourceHostConfigurationValidation => Ok(from.as_any_box().downcast::<ClusterComputeResourceHostConfigurationValidation>()?),
            StructType::ClusterComputeResourceVcsSlots => Ok(from.as_any_box().downcast::<ClusterComputeResourceVcsSlots>()?),
            StructType::ComputeResourceConfigInfo => Ok(from.as_any_box().downcast::<ComputeResourceConfigInfo>()?),
            StructType::ClusterConfigInfoEx => Ok(from.as_any_box().downcast::<ClusterConfigInfoEx>()?),
            StructType::ComputeResourceConfigSpec => Ok(from.as_any_box().downcast::<ComputeResourceConfigSpec>()?),
            StructType::ClusterConfigSpecEx => Ok(from.as_any_box().downcast::<ClusterConfigSpecEx>()?),
            StructType::ComputeResourceHostSpbmLicenseInfo => Ok(from.as_any_box().downcast::<ComputeResourceHostSpbmLicenseInfo>()?),
            StructType::ComputeResourceSummary => Ok(from.as_any_box().downcast::<ComputeResourceSummary>()?),
            StructType::ClusterComputeResourceSummary => Ok(from.as_any_box().downcast::<ClusterComputeResourceSummary>()?),
            StructType::CustomFieldDef => Ok(from.as_any_box().downcast::<CustomFieldDef>()?),
            StructType::CustomFieldValue => Ok(from.as_any_box().downcast::<CustomFieldValue>()?),
            StructType::CustomFieldStringValue => Ok(from.as_any_box().downcast::<CustomFieldStringValue>()?),
            StructType::CustomizationSpecInfo => Ok(from.as_any_box().downcast::<CustomizationSpecInfo>()?),
            StructType::CustomizationSpecItem => Ok(from.as_any_box().downcast::<CustomizationSpecItem>()?),
            StructType::DatacenterBasicConnectInfo => Ok(from.as_any_box().downcast::<DatacenterBasicConnectInfo>()?),
            StructType::DatacenterConfigInfo => Ok(from.as_any_box().downcast::<DatacenterConfigInfo>()?),
            StructType::DatacenterConfigSpec => Ok(from.as_any_box().downcast::<DatacenterConfigSpec>()?),
            StructType::DatastoreCapability => Ok(from.as_any_box().downcast::<DatastoreCapability>()?),
            StructType::DatastoreHostMount => Ok(from.as_any_box().downcast::<DatastoreHostMount>()?),
            StructType::DatastoreInfo => Ok(from.as_any_box().downcast::<DatastoreInfo>()?),
            StructType::LocalDatastoreInfo => Ok(from.as_any_box().downcast::<LocalDatastoreInfo>()?),
            StructType::NasDatastoreInfo => Ok(from.as_any_box().downcast::<NasDatastoreInfo>()?),
            StructType::PMemDatastoreInfo => Ok(from.as_any_box().downcast::<PMemDatastoreInfo>()?),
            StructType::VmfsDatastoreInfo => Ok(from.as_any_box().downcast::<VmfsDatastoreInfo>()?),
            StructType::VsanDatastoreInfo => Ok(from.as_any_box().downcast::<VsanDatastoreInfo>()?),
            StructType::VvolDatastoreInfo => Ok(from.as_any_box().downcast::<VvolDatastoreInfo>()?),
            StructType::DatastoreMountPathDatastorePair => Ok(from.as_any_box().downcast::<DatastoreMountPathDatastorePair>()?),
            StructType::DatastoreSummary => Ok(from.as_any_box().downcast::<DatastoreSummary>()?),
            StructType::DatastoreVVolContainerFailoverPair => Ok(from.as_any_box().downcast::<DatastoreVVolContainerFailoverPair>()?),
            StructType::DatastoreNamespaceManagerDirectoryInfo => Ok(from.as_any_box().downcast::<DatastoreNamespaceManagerDirectoryInfo>()?),
            StructType::Description => Ok(from.as_any_box().downcast::<Description>()?),
            StructType::ElementDescription => Ok(from.as_any_box().downcast::<ElementDescription>()?),
            StructType::EvcMode => Ok(from.as_any_box().downcast::<EvcMode>()?),
            StructType::ExtendedElementDescription => Ok(from.as_any_box().downcast::<ExtendedElementDescription>()?),
            StructType::FeatureEvcMode => Ok(from.as_any_box().downcast::<FeatureEvcMode>()?),
            StructType::OptionDef => Ok(from.as_any_box().downcast::<OptionDef>()?),
            StructType::ExtendedDescription => Ok(from.as_any_box().downcast::<ExtendedDescription>()?),
            StructType::MethodDescription => Ok(from.as_any_box().downcast::<MethodDescription>()?),
            StructType::TypeDescription => Ok(from.as_any_box().downcast::<TypeDescription>()?),
            StructType::ScheduledTaskDetail => Ok(from.as_any_box().downcast::<ScheduledTaskDetail>()?),
            StructType::DesiredSoftwareSpec => Ok(from.as_any_box().downcast::<DesiredSoftwareSpec>()?),
            StructType::DesiredSoftwareSpecBaseImageSpec => Ok(from.as_any_box().downcast::<DesiredSoftwareSpecBaseImageSpec>()?),
            StructType::DesiredSoftwareSpecComponentSpec => Ok(from.as_any_box().downcast::<DesiredSoftwareSpecComponentSpec>()?),
            StructType::DesiredSoftwareSpecVendorAddOnSpec => Ok(from.as_any_box().downcast::<DesiredSoftwareSpecVendorAddOnSpec>()?),
            StructType::DiagnosticManagerAuditRecordResult => Ok(from.as_any_box().downcast::<DiagnosticManagerAuditRecordResult>()?),
            StructType::DiagnosticManagerBundleInfo => Ok(from.as_any_box().downcast::<DiagnosticManagerBundleInfo>()?),
            StructType::DiagnosticManagerLogDescriptor => Ok(from.as_any_box().downcast::<DiagnosticManagerLogDescriptor>()?),
            StructType::DiagnosticManagerLogHeader => Ok(from.as_any_box().downcast::<DiagnosticManagerLogHeader>()?),
            StructType::DvsBackupRestoreCapability => Ok(from.as_any_box().downcast::<DvsBackupRestoreCapability>()?),
            StructType::DvsCapability => Ok(from.as_any_box().downcast::<DvsCapability>()?),
            StructType::DvsConfigInfo => Ok(from.as_any_box().downcast::<DvsConfigInfo>()?),
            StructType::VMwareDvsConfigInfo => Ok(from.as_any_box().downcast::<VMwareDvsConfigInfo>()?),
            StructType::DvsConfigSpec => Ok(from.as_any_box().downcast::<DvsConfigSpec>()?),
            StructType::VMwareDvsConfigSpec => Ok(from.as_any_box().downcast::<VMwareDvsConfigSpec>()?),
            StructType::DvsContactInfo => Ok(from.as_any_box().downcast::<DvsContactInfo>()?),
            StructType::DvsCreateSpec => Ok(from.as_any_box().downcast::<DvsCreateSpec>()?),
            StructType::DvsFeatureCapability => Ok(from.as_any_box().downcast::<DvsFeatureCapability>()?),
            StructType::VMwareDvsFeatureCapability => Ok(from.as_any_box().downcast::<VMwareDvsFeatureCapability>()?),
            StructType::DvsHealthCheckConfig => Ok(from.as_any_box().downcast::<DvsHealthCheckConfig>()?),
            StructType::VMwareDvsHealthCheckConfig => Ok(from.as_any_box().downcast::<VMwareDvsHealthCheckConfig>()?),
            StructType::VMwareDvsTeamingHealthCheckConfig => Ok(from.as_any_box().downcast::<VMwareDvsTeamingHealthCheckConfig>()?),
            StructType::VMwareDvsVlanMtuHealthCheckConfig => Ok(from.as_any_box().downcast::<VMwareDvsVlanMtuHealthCheckConfig>()?),
            StructType::DvsHealthCheckCapability => Ok(from.as_any_box().downcast::<DvsHealthCheckCapability>()?),
            StructType::VMwareDvsHealthCheckCapability => Ok(from.as_any_box().downcast::<VMwareDvsHealthCheckCapability>()?),
            StructType::DvsHostInfrastructureTrafficResource => Ok(from.as_any_box().downcast::<DvsHostInfrastructureTrafficResource>()?),
            StructType::DvsHostInfrastructureTrafficResourceAllocation => Ok(from.as_any_box().downcast::<DvsHostInfrastructureTrafficResourceAllocation>()?),
            StructType::DvsNetworkResourceManagementCapability => Ok(from.as_any_box().downcast::<DvsNetworkResourceManagementCapability>()?),
            StructType::DvsResourceRuntimeInfo => Ok(from.as_any_box().downcast::<DvsResourceRuntimeInfo>()?),
            StructType::DvsRollbackCapability => Ok(from.as_any_box().downcast::<DvsRollbackCapability>()?),
            StructType::DvsRuntimeInfo => Ok(from.as_any_box().downcast::<DvsRuntimeInfo>()?),
            StructType::DvsSummary => Ok(from.as_any_box().downcast::<DvsSummary>()?),
            StructType::DvsPolicy => Ok(from.as_any_box().downcast::<DvsPolicy>()?),
            StructType::DvsUplinkPortPolicy => Ok(from.as_any_box().downcast::<DvsUplinkPortPolicy>()?),
            StructType::DvsNameArrayUplinkPortPolicy => Ok(from.as_any_box().downcast::<DvsNameArrayUplinkPortPolicy>()?),
            StructType::EnumDescription => Ok(from.as_any_box().downcast::<EnumDescription>()?),
            StructType::EnvironmentBrowserConfigOptionQuerySpec => Ok(from.as_any_box().downcast::<EnvironmentBrowserConfigOptionQuerySpec>()?),
            StructType::Extension => Ok(from.as_any_box().downcast::<Extension>()?),
            StructType::ExtensionClientInfo => Ok(from.as_any_box().downcast::<ExtensionClientInfo>()?),
            StructType::ExtensionEventTypeInfo => Ok(from.as_any_box().downcast::<ExtensionEventTypeInfo>()?),
            StructType::ExtensionFaultTypeInfo => Ok(from.as_any_box().downcast::<ExtensionFaultTypeInfo>()?),
            StructType::ExtensionHealthInfo => Ok(from.as_any_box().downcast::<ExtensionHealthInfo>()?),
            StructType::ExtensionOvfConsumerInfo => Ok(from.as_any_box().downcast::<ExtensionOvfConsumerInfo>()?),
            StructType::ExtensionPrivilegeInfo => Ok(from.as_any_box().downcast::<ExtensionPrivilegeInfo>()?),
            StructType::ExtensionResourceInfo => Ok(from.as_any_box().downcast::<ExtensionResourceInfo>()?),
            StructType::ExtensionServerInfo => Ok(from.as_any_box().downcast::<ExtensionServerInfo>()?),
            StructType::ExtensionTaskTypeInfo => Ok(from.as_any_box().downcast::<ExtensionTaskTypeInfo>()?),
            StructType::ExtensionManagerIpAllocationUsage => Ok(from.as_any_box().downcast::<ExtensionManagerIpAllocationUsage>()?),
            StructType::FaultsByHost => Ok(from.as_any_box().downcast::<FaultsByHost>()?),
            StructType::FaultsByVm => Ok(from.as_any_box().downcast::<FaultsByVm>()?),
            StructType::FileLockInfo => Ok(from.as_any_box().downcast::<FileLockInfo>()?),
            StructType::FileLockInfoResult => Ok(from.as_any_box().downcast::<FileLockInfoResult>()?),
            StructType::FolderBatchAddHostsToClusterResult => Ok(from.as_any_box().downcast::<FolderBatchAddHostsToClusterResult>()?),
            StructType::FolderBatchAddStandaloneHostsResult => Ok(from.as_any_box().downcast::<FolderBatchAddStandaloneHostsResult>()?),
            StructType::FolderFailedHostResult => Ok(from.as_any_box().downcast::<FolderFailedHostResult>()?),
            StructType::FolderNewHostSpec => Ok(from.as_any_box().downcast::<FolderNewHostSpec>()?),
            StructType::HbrManagerReplicationVmInfo => Ok(from.as_any_box().downcast::<HbrManagerReplicationVmInfo>()?),
            StructType::ReplicationVmProgressInfo => Ok(from.as_any_box().downcast::<ReplicationVmProgressInfo>()?),
            StructType::HbrManagerVmReplicationCapability => Ok(from.as_any_box().downcast::<HbrManagerVmReplicationCapability>()?),
            StructType::HealthUpdate => Ok(from.as_any_box().downcast::<HealthUpdate>()?),
            StructType::HealthUpdateInfo => Ok(from.as_any_box().downcast::<HealthUpdateInfo>()?),
            StructType::PerfInterval => Ok(from.as_any_box().downcast::<PerfInterval>()?),
            StructType::HostServiceTicket => Ok(from.as_any_box().downcast::<HostServiceTicket>()?),
            StructType::HostSystemComplianceCheckState => Ok(from.as_any_box().downcast::<HostSystemComplianceCheckState>()?),
            StructType::HostSystemReconnectSpec => Ok(from.as_any_box().downcast::<HostSystemReconnectSpec>()?),
            StructType::HostSystemRemediationState => Ok(from.as_any_box().downcast::<HostSystemRemediationState>()?),
            StructType::HttpNfcLeaseCapabilities => Ok(from.as_any_box().downcast::<HttpNfcLeaseCapabilities>()?),
            StructType::HttpNfcLeaseDatastoreLeaseInfo => Ok(from.as_any_box().downcast::<HttpNfcLeaseDatastoreLeaseInfo>()?),
            StructType::HttpNfcLeaseDeviceUrl => Ok(from.as_any_box().downcast::<HttpNfcLeaseDeviceUrl>()?),
            StructType::HttpNfcLeaseHostInfo => Ok(from.as_any_box().downcast::<HttpNfcLeaseHostInfo>()?),
            StructType::HttpNfcLeaseInfo => Ok(from.as_any_box().downcast::<HttpNfcLeaseInfo>()?),
            StructType::HttpNfcLeaseManifestEntry => Ok(from.as_any_box().downcast::<HttpNfcLeaseManifestEntry>()?),
            StructType::HttpNfcLeaseProbeResult => Ok(from.as_any_box().downcast::<HttpNfcLeaseProbeResult>()?),
            StructType::HttpNfcLeaseSourceFile => Ok(from.as_any_box().downcast::<HttpNfcLeaseSourceFile>()?),
            StructType::ImportSpec => Ok(from.as_any_box().downcast::<ImportSpec>()?),
            StructType::VirtualAppImportSpec => Ok(from.as_any_box().downcast::<VirtualAppImportSpec>()?),
            StructType::VirtualMachineImportSpec => Ok(from.as_any_box().downcast::<VirtualMachineImportSpec>()?),
            StructType::InheritablePolicy => Ok(from.as_any_box().downcast::<InheritablePolicy>()?),
            StructType::BoolPolicy => Ok(from.as_any_box().downcast::<BoolPolicy>()?),
            StructType::IntPolicy => Ok(from.as_any_box().downcast::<IntPolicy>()?),
            StructType::LongPolicy => Ok(from.as_any_box().downcast::<LongPolicy>()?),
            StructType::StringPolicy => Ok(from.as_any_box().downcast::<StringPolicy>()?),
            StructType::DvsFilterConfig => Ok(from.as_any_box().downcast::<DvsFilterConfig>()?),
            StructType::DvsFilterConfigSpec => Ok(from.as_any_box().downcast::<DvsFilterConfigSpec>()?),
            StructType::DvsTrafficFilterConfig => Ok(from.as_any_box().downcast::<DvsTrafficFilterConfig>()?),
            StructType::DvsTrafficFilterConfigSpec => Ok(from.as_any_box().downcast::<DvsTrafficFilterConfigSpec>()?),
            StructType::DvsFilterPolicy => Ok(from.as_any_box().downcast::<DvsFilterPolicy>()?),
            StructType::DvsTrafficShapingPolicy => Ok(from.as_any_box().downcast::<DvsTrafficShapingPolicy>()?),
            StructType::DvsVendorSpecificConfig => Ok(from.as_any_box().downcast::<DvsVendorSpecificConfig>()?),
            StructType::DvsFailureCriteria => Ok(from.as_any_box().downcast::<DvsFailureCriteria>()?),
            StructType::DvsMacLearningPolicy => Ok(from.as_any_box().downcast::<DvsMacLearningPolicy>()?),
            StructType::DvsMacManagementPolicy => Ok(from.as_any_box().downcast::<DvsMacManagementPolicy>()?),
            StructType::DvsSecurityPolicy => Ok(from.as_any_box().downcast::<DvsSecurityPolicy>()?),
            StructType::VMwareUplinkLacpPolicy => Ok(from.as_any_box().downcast::<VMwareUplinkLacpPolicy>()?),
            StructType::VMwareUplinkPortOrderPolicy => Ok(from.as_any_box().downcast::<VMwareUplinkPortOrderPolicy>()?),
            StructType::VmwareUplinkPortTeamingPolicy => Ok(from.as_any_box().downcast::<VmwareUplinkPortTeamingPolicy>()?),
            StructType::VmwareDistributedVirtualSwitchVlanSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchPvlanSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchPvlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchTrunkVlanSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchTrunkVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchVlanIdSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchVlanIdSpec>()?),
            StructType::IoFilterInfo => Ok(from.as_any_box().downcast::<IoFilterInfo>()?),
            StructType::ClusterIoFilterInfo => Ok(from.as_any_box().downcast::<ClusterIoFilterInfo>()?),
            StructType::HostIoFilterInfo => Ok(from.as_any_box().downcast::<HostIoFilterInfo>()?),
            StructType::IoFilterQueryIssueResult => Ok(from.as_any_box().downcast::<IoFilterQueryIssueResult>()?),
            StructType::IoFilterHostIssue => Ok(from.as_any_box().downcast::<IoFilterHostIssue>()?),
            StructType::IpPoolManagerIpAllocation => Ok(from.as_any_box().downcast::<IpPoolManagerIpAllocation>()?),
            StructType::KeyValue => Ok(from.as_any_box().downcast::<KeyValue>()?),
            StructType::LatencySensitivity => Ok(from.as_any_box().downcast::<LatencySensitivity>()?),
            StructType::LicenseAssignmentManagerLicenseAssignment => Ok(from.as_any_box().downcast::<LicenseAssignmentManagerLicenseAssignment>()?),
            StructType::LicenseAvailabilityInfo => Ok(from.as_any_box().downcast::<LicenseAvailabilityInfo>()?),
            StructType::LicenseDiagnostics => Ok(from.as_any_box().downcast::<LicenseDiagnostics>()?),
            StructType::LicenseManagerEvaluationInfo => Ok(from.as_any_box().downcast::<LicenseManagerEvaluationInfo>()?),
            StructType::LicenseFeatureInfo => Ok(from.as_any_box().downcast::<LicenseFeatureInfo>()?),
            StructType::HostLicensableResourceInfo => Ok(from.as_any_box().downcast::<HostLicensableResourceInfo>()?),
            StructType::LicenseManagerLicenseInfo => Ok(from.as_any_box().downcast::<LicenseManagerLicenseInfo>()?),
            StructType::LicenseSource => Ok(from.as_any_box().downcast::<LicenseSource>()?),
            StructType::EvaluationLicenseSource => Ok(from.as_any_box().downcast::<EvaluationLicenseSource>()?),
            StructType::LicenseServerSource => Ok(from.as_any_box().downcast::<LicenseServerSource>()?),
            StructType::LocalLicenseSource => Ok(from.as_any_box().downcast::<LocalLicenseSource>()?),
            StructType::LicenseUsageInfo => Ok(from.as_any_box().downcast::<LicenseUsageInfo>()?),
            StructType::LicenseReservationInfo => Ok(from.as_any_box().downcast::<LicenseReservationInfo>()?),
            StructType::LocalizationManagerMessageCatalog => Ok(from.as_any_box().downcast::<LocalizationManagerMessageCatalog>()?),
            StructType::NegatableExpression => Ok(from.as_any_box().downcast::<NegatableExpression>()?),
            StructType::IntExpression => Ok(from.as_any_box().downcast::<IntExpression>()?),
            StructType::IpAddress => Ok(from.as_any_box().downcast::<IpAddress>()?),
            StructType::IpRange => Ok(from.as_any_box().downcast::<IpRange>()?),
            StructType::SingleIp => Ok(from.as_any_box().downcast::<SingleIp>()?),
            StructType::MacAddress => Ok(from.as_any_box().downcast::<MacAddress>()?),
            StructType::MacRange => Ok(from.as_any_box().downcast::<MacRange>()?),
            StructType::SingleMac => Ok(from.as_any_box().downcast::<SingleMac>()?),
            StructType::StringExpression => Ok(from.as_any_box().downcast::<StringExpression>()?),
            StructType::DvsIpPort => Ok(from.as_any_box().downcast::<DvsIpPort>()?),
            StructType::DvsIpPortRange => Ok(from.as_any_box().downcast::<DvsIpPortRange>()?),
            StructType::DvsSingleIpPort => Ok(from.as_any_box().downcast::<DvsSingleIpPort>()?),
            StructType::NetworkSummary => Ok(from.as_any_box().downcast::<NetworkSummary>()?),
            StructType::OpaqueNetworkSummary => Ok(from.as_any_box().downcast::<OpaqueNetworkSummary>()?),
            StructType::NumericRange => Ok(from.as_any_box().downcast::<NumericRange>()?),
            StructType::OpaqueNetworkCapability => Ok(from.as_any_box().downcast::<OpaqueNetworkCapability>()?),
            StructType::OvfConsumerOstNode => Ok(from.as_any_box().downcast::<OvfConsumerOstNode>()?),
            StructType::OvfConsumerOvfSection => Ok(from.as_any_box().downcast::<OvfConsumerOvfSection>()?),
            StructType::OvfManagerCommonParams => Ok(from.as_any_box().downcast::<OvfManagerCommonParams>()?),
            StructType::OvfCreateImportSpecParams => Ok(from.as_any_box().downcast::<OvfCreateImportSpecParams>()?),
            StructType::OvfParseDescriptorParams => Ok(from.as_any_box().downcast::<OvfParseDescriptorParams>()?),
            StructType::OvfValidateHostParams => Ok(from.as_any_box().downcast::<OvfValidateHostParams>()?),
            StructType::OvfCreateDescriptorParams => Ok(from.as_any_box().downcast::<OvfCreateDescriptorParams>()?),
            StructType::OvfCreateDescriptorResult => Ok(from.as_any_box().downcast::<OvfCreateDescriptorResult>()?),
            StructType::OvfCreateImportSpecResult => Ok(from.as_any_box().downcast::<OvfCreateImportSpecResult>()?),
            StructType::OvfDeploymentOption => Ok(from.as_any_box().downcast::<OvfDeploymentOption>()?),
            StructType::OvfFileItem => Ok(from.as_any_box().downcast::<OvfFileItem>()?),
            StructType::OvfNetworkInfo => Ok(from.as_any_box().downcast::<OvfNetworkInfo>()?),
            StructType::OvfNetworkMapping => Ok(from.as_any_box().downcast::<OvfNetworkMapping>()?),
            StructType::OvfFile => Ok(from.as_any_box().downcast::<OvfFile>()?),
            StructType::OvfOptionInfo => Ok(from.as_any_box().downcast::<OvfOptionInfo>()?),
            StructType::OvfParseDescriptorResult => Ok(from.as_any_box().downcast::<OvfParseDescriptorResult>()?),
            StructType::OvfResourceMap => Ok(from.as_any_box().downcast::<OvfResourceMap>()?),
            StructType::OvfValidateHostResult => Ok(from.as_any_box().downcast::<OvfValidateHostResult>()?),
            StructType::PasswordField => Ok(from.as_any_box().downcast::<PasswordField>()?),
            StructType::PerformanceDescription => Ok(from.as_any_box().downcast::<PerformanceDescription>()?),
            StructType::PerfCompositeMetric => Ok(from.as_any_box().downcast::<PerfCompositeMetric>()?),
            StructType::PerfCounterInfo => Ok(from.as_any_box().downcast::<PerfCounterInfo>()?),
            StructType::PerformanceManagerCounterLevelMapping => Ok(from.as_any_box().downcast::<PerformanceManagerCounterLevelMapping>()?),
            StructType::PerfEntityMetricBase => Ok(from.as_any_box().downcast::<PerfEntityMetricBase>()?),
            StructType::PerfEntityMetric => Ok(from.as_any_box().downcast::<PerfEntityMetric>()?),
            StructType::PerfEntityMetricCsv => Ok(from.as_any_box().downcast::<PerfEntityMetricCsv>()?),
            StructType::PerfMetricId => Ok(from.as_any_box().downcast::<PerfMetricId>()?),
            StructType::PerfMetricSeries => Ok(from.as_any_box().downcast::<PerfMetricSeries>()?),
            StructType::PerfMetricIntSeries => Ok(from.as_any_box().downcast::<PerfMetricIntSeries>()?),
            StructType::PerfMetricSeriesCsv => Ok(from.as_any_box().downcast::<PerfMetricSeriesCsv>()?),
            StructType::PerfProviderSummary => Ok(from.as_any_box().downcast::<PerfProviderSummary>()?),
            StructType::PerfQuerySpec => Ok(from.as_any_box().downcast::<PerfQuerySpec>()?),
            StructType::PerfSampleInfo => Ok(from.as_any_box().downcast::<PerfSampleInfo>()?),
            StructType::PrivilegePolicyDef => Ok(from.as_any_box().downcast::<PrivilegePolicyDef>()?),
            StructType::ResourceAllocationInfo => Ok(from.as_any_box().downcast::<ResourceAllocationInfo>()?),
            StructType::ResourceAllocationOption => Ok(from.as_any_box().downcast::<ResourceAllocationOption>()?),
            StructType::ResourceConfigOption => Ok(from.as_any_box().downcast::<ResourceConfigOption>()?),
            StructType::ResourceConfigSpec => Ok(from.as_any_box().downcast::<ResourceConfigSpec>()?),
            StructType::DatabaseSizeEstimate => Ok(from.as_any_box().downcast::<DatabaseSizeEstimate>()?),
            StructType::DatabaseSizeParam => Ok(from.as_any_box().downcast::<DatabaseSizeParam>()?),
            StructType::InventoryDescription => Ok(from.as_any_box().downcast::<InventoryDescription>()?),
            StructType::PerformanceStatisticsDescription => Ok(from.as_any_box().downcast::<PerformanceStatisticsDescription>()?),
            StructType::ResourcePoolResourceUsage => Ok(from.as_any_box().downcast::<ResourcePoolResourceUsage>()?),
            StructType::ResourcePoolRuntimeInfo => Ok(from.as_any_box().downcast::<ResourcePoolRuntimeInfo>()?),
            StructType::ResourcePoolSummary => Ok(from.as_any_box().downcast::<ResourcePoolSummary>()?),
            StructType::VirtualAppSummary => Ok(from.as_any_box().downcast::<VirtualAppSummary>()?),
            StructType::ResourcePoolQuickStats => Ok(from.as_any_box().downcast::<ResourcePoolQuickStats>()?),
            StructType::SddcBase => Ok(from.as_any_box().downcast::<SddcBase>()?),
            StructType::SelectionSet => Ok(from.as_any_box().downcast::<SelectionSet>()?),
            StructType::DvPortgroupSelection => Ok(from.as_any_box().downcast::<DvPortgroupSelection>()?),
            StructType::DvsSelection => Ok(from.as_any_box().downcast::<DvsSelection>()?),
            StructType::HostVMotionCompatibility => Ok(from.as_any_box().downcast::<HostVMotionCompatibility>()?),
            StructType::ProductComponentInfo => Ok(from.as_any_box().downcast::<ProductComponentInfo>()?),
            StructType::ServiceContent => Ok(from.as_any_box().downcast::<ServiceContent>()?),
            StructType::ServiceLocator => Ok(from.as_any_box().downcast::<ServiceLocator>()?),
            StructType::ServiceLocatorCredential => Ok(from.as_any_box().downcast::<ServiceLocatorCredential>()?),
            StructType::ServiceLocatorNamePassword => Ok(from.as_any_box().downcast::<ServiceLocatorNamePassword>()?),
            StructType::ServiceLocatorSamlCredential => Ok(from.as_any_box().downcast::<ServiceLocatorSamlCredential>()?),
            StructType::ServiceManagerServiceInfo => Ok(from.as_any_box().downcast::<ServiceManagerServiceInfo>()?),
            StructType::SessionManagerGenericServiceTicket => Ok(from.as_any_box().downcast::<SessionManagerGenericServiceTicket>()?),
            StructType::SessionManagerLocalTicket => Ok(from.as_any_box().downcast::<SessionManagerLocalTicket>()?),
            StructType::SessionManagerServiceRequestSpec => Ok(from.as_any_box().downcast::<SessionManagerServiceRequestSpec>()?),
            StructType::SessionManagerHttpServiceRequestSpec => Ok(from.as_any_box().downcast::<SessionManagerHttpServiceRequestSpec>()?),
            StructType::SessionManagerVmomiServiceRequestSpec => Ok(from.as_any_box().downcast::<SessionManagerVmomiServiceRequestSpec>()?),
            StructType::SharesInfo => Ok(from.as_any_box().downcast::<SharesInfo>()?),
            StructType::SharesOption => Ok(from.as_any_box().downcast::<SharesOption>()?),
            StructType::SiteInfo => Ok(from.as_any_box().downcast::<SiteInfo>()?),
            StructType::StoragePodSummary => Ok(from.as_any_box().downcast::<StoragePodSummary>()?),
            StructType::StorageIoAllocationInfo => Ok(from.as_any_box().downcast::<StorageIoAllocationInfo>()?),
            StructType::StorageIoAllocationOption => Ok(from.as_any_box().downcast::<StorageIoAllocationOption>()?),
            StructType::StorageIormInfo => Ok(from.as_any_box().downcast::<StorageIormInfo>()?),
            StructType::StorageIormConfigOption => Ok(from.as_any_box().downcast::<StorageIormConfigOption>()?),
            StructType::StorageIormConfigSpec => Ok(from.as_any_box().downcast::<StorageIormConfigSpec>()?),
            StructType::PodStorageDrsEntry => Ok(from.as_any_box().downcast::<PodStorageDrsEntry>()?),
            StructType::StoragePerformanceSummary => Ok(from.as_any_box().downcast::<StoragePerformanceSummary>()?),
            StructType::StorageResourceManagerStorageProfileStatistics => Ok(from.as_any_box().downcast::<StorageResourceManagerStorageProfileStatistics>()?),
            StructType::Tag => Ok(from.as_any_box().downcast::<Tag>()?),
            StructType::TaskDescription => Ok(from.as_any_box().downcast::<TaskDescription>()?),
            StructType::TaskFilterSpec => Ok(from.as_any_box().downcast::<TaskFilterSpec>()?),
            StructType::TaskFilterSpecByEntity => Ok(from.as_any_box().downcast::<TaskFilterSpecByEntity>()?),
            StructType::TaskFilterSpecByTime => Ok(from.as_any_box().downcast::<TaskFilterSpecByTime>()?),
            StructType::TaskFilterSpecByUsername => Ok(from.as_any_box().downcast::<TaskFilterSpecByUsername>()?),
            StructType::TaskInfo => Ok(from.as_any_box().downcast::<TaskInfo>()?),
            StructType::TaskReason => Ok(from.as_any_box().downcast::<TaskReason>()?),
            StructType::TaskReasonAlarm => Ok(from.as_any_box().downcast::<TaskReasonAlarm>()?),
            StructType::TaskReasonSchedule => Ok(from.as_any_box().downcast::<TaskReasonSchedule>()?),
            StructType::TaskReasonSystem => Ok(from.as_any_box().downcast::<TaskReasonSystem>()?),
            StructType::TaskReasonUser => Ok(from.as_any_box().downcast::<TaskReasonUser>()?),
            StructType::UpdateVirtualMachineFilesResult => Ok(from.as_any_box().downcast::<UpdateVirtualMachineFilesResult>()?),
            StructType::UpdateVirtualMachineFilesResultFailedVmFileInfo => Ok(from.as_any_box().downcast::<UpdateVirtualMachineFilesResultFailedVmFileInfo>()?),
            StructType::UserSearchResult => Ok(from.as_any_box().downcast::<UserSearchResult>()?),
            StructType::PosixUserSearchResult => Ok(from.as_any_box().downcast::<PosixUserSearchResult>()?),
            StructType::UserSession => Ok(from.as_any_box().downcast::<UserSession>()?),
            StructType::VVolVmConfigFileUpdateResult => Ok(from.as_any_box().downcast::<VVolVmConfigFileUpdateResult>()?),
            StructType::VVolVmConfigFileUpdateResultFailedVmConfigFileInfo => Ok(from.as_any_box().downcast::<VVolVmConfigFileUpdateResultFailedVmConfigFileInfo>()?),
            StructType::VasaStorageArray => Ok(from.as_any_box().downcast::<VasaStorageArray>()?),
            StructType::VasaStorageArrayDiscoveryFcTransport => Ok(from.as_any_box().downcast::<VasaStorageArrayDiscoveryFcTransport>()?),
            StructType::VasaStorageArrayDiscoveryIpTransport => Ok(from.as_any_box().downcast::<VasaStorageArrayDiscoveryIpTransport>()?),
            StructType::VasaStorageArrayDiscoverySvcInfo => Ok(from.as_any_box().downcast::<VasaStorageArrayDiscoverySvcInfo>()?),
            StructType::VasaProviderContainerSpec => Ok(from.as_any_box().downcast::<VasaProviderContainerSpec>()?),
            StructType::VimVasaProvider => Ok(from.as_any_box().downcast::<VimVasaProvider>()?),
            StructType::VimVasaProviderStatePerArray => Ok(from.as_any_box().downcast::<VimVasaProviderStatePerArray>()?),
            StructType::VimVasaProviderVirtualHostConfig => Ok(from.as_any_box().downcast::<VimVasaProviderVirtualHostConfig>()?),
            StructType::VimVasaProviderInfo => Ok(from.as_any_box().downcast::<VimVasaProviderInfo>()?),
            StructType::VirtualAppLinkInfo => Ok(from.as_any_box().downcast::<VirtualAppLinkInfo>()?),
            StructType::VirtualDiskSpec => Ok(from.as_any_box().downcast::<VirtualDiskSpec>()?),
            StructType::DeviceBackedVirtualDiskSpec => Ok(from.as_any_box().downcast::<DeviceBackedVirtualDiskSpec>()?),
            StructType::FileBackedVirtualDiskSpec => Ok(from.as_any_box().downcast::<FileBackedVirtualDiskSpec>()?),
            StructType::SeSparseVirtualDiskSpec => Ok(from.as_any_box().downcast::<SeSparseVirtualDiskSpec>()?),
            StructType::VirtualMachineConnection => Ok(from.as_any_box().downcast::<VirtualMachineConnection>()?),
            StructType::VirtualMachineMksConnection => Ok(from.as_any_box().downcast::<VirtualMachineMksConnection>()?),
            StructType::DiskChangeInfo => Ok(from.as_any_box().downcast::<DiskChangeInfo>()?),
            StructType::DiskChangeExtent => Ok(from.as_any_box().downcast::<DiskChangeExtent>()?),
            StructType::VirtualMachineDisplayTopology => Ok(from.as_any_box().downcast::<VirtualMachineDisplayTopology>()?),
            StructType::VirtualMachineMksTicket => Ok(from.as_any_box().downcast::<VirtualMachineMksTicket>()?),
            StructType::StorageRequirement => Ok(from.as_any_box().downcast::<StorageRequirement>()?),
            StructType::VirtualMachineTicket => Ok(from.as_any_box().downcast::<VirtualMachineTicket>()?),
            StructType::VirtualMachineWipeResult => Ok(from.as_any_box().downcast::<VirtualMachineWipeResult>()?),
            StructType::VsanUpgradeSystemNetworkPartitionInfo => Ok(from.as_any_box().downcast::<VsanUpgradeSystemNetworkPartitionInfo>()?),
            StructType::VsanUpgradeSystemPreflightCheckIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemPreflightCheckIssue>()?),
            StructType::VsanUpgradeSystemApiBrokenIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemApiBrokenIssue>()?),
            StructType::VsanUpgradeSystemAutoClaimEnabledOnHostsIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemAutoClaimEnabledOnHostsIssue>()?),
            StructType::VsanUpgradeSystemHostsDisconnectedIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemHostsDisconnectedIssue>()?),
            StructType::VsanUpgradeSystemMissingHostsInClusterIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemMissingHostsInClusterIssue>()?),
            StructType::VsanUpgradeSystemNetworkPartitionIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemNetworkPartitionIssue>()?),
            StructType::VsanUpgradeSystemNotEnoughFreeCapacityIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemNotEnoughFreeCapacityIssue>()?),
            StructType::VsanUpgradeSystemRogueHostsInClusterIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemRogueHostsInClusterIssue>()?),
            StructType::VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue>()?),
            StructType::VsanUpgradeSystemWrongEsxVersionIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemWrongEsxVersionIssue>()?),
            StructType::VsanUpgradeSystemPreflightCheckResult => Ok(from.as_any_box().downcast::<VsanUpgradeSystemPreflightCheckResult>()?),
            StructType::VsanUpgradeSystemUpgradeHistoryItem => Ok(from.as_any_box().downcast::<VsanUpgradeSystemUpgradeHistoryItem>()?),
            StructType::VsanUpgradeSystemUpgradeHistoryDiskGroupOp => Ok(from.as_any_box().downcast::<VsanUpgradeSystemUpgradeHistoryDiskGroupOp>()?),
            StructType::VsanUpgradeSystemUpgradeHistoryPreflightFail => Ok(from.as_any_box().downcast::<VsanUpgradeSystemUpgradeHistoryPreflightFail>()?),
            StructType::VsanUpgradeSystemUpgradeStatus => Ok(from.as_any_box().downcast::<VsanUpgradeSystemUpgradeStatus>()?),
            StructType::Action => Ok(from.as_any_box().downcast::<Action>()?),
            StructType::CreateTaskAction => Ok(from.as_any_box().downcast::<CreateTaskAction>()?),
            StructType::MethodAction => Ok(from.as_any_box().downcast::<MethodAction>()?),
            StructType::RunScriptAction => Ok(from.as_any_box().downcast::<RunScriptAction>()?),
            StructType::SendEmailAction => Ok(from.as_any_box().downcast::<SendEmailAction>()?),
            StructType::SendSnmpAction => Ok(from.as_any_box().downcast::<SendSnmpAction>()?),
            StructType::MethodActionArgument => Ok(from.as_any_box().downcast::<MethodActionArgument>()?),
            StructType::AlarmAction => Ok(from.as_any_box().downcast::<AlarmAction>()?),
            StructType::AlarmTriggeringAction => Ok(from.as_any_box().downcast::<AlarmTriggeringAction>()?),
            StructType::GroupAlarmAction => Ok(from.as_any_box().downcast::<GroupAlarmAction>()?),
            StructType::AlarmDescription => Ok(from.as_any_box().downcast::<AlarmDescription>()?),
            StructType::AlarmExpression => Ok(from.as_any_box().downcast::<AlarmExpression>()?),
            StructType::AndAlarmExpression => Ok(from.as_any_box().downcast::<AndAlarmExpression>()?),
            StructType::EventAlarmExpression => Ok(from.as_any_box().downcast::<EventAlarmExpression>()?),
            StructType::MetricAlarmExpression => Ok(from.as_any_box().downcast::<MetricAlarmExpression>()?),
            StructType::OrAlarmExpression => Ok(from.as_any_box().downcast::<OrAlarmExpression>()?),
            StructType::StateAlarmExpression => Ok(from.as_any_box().downcast::<StateAlarmExpression>()?),
            StructType::AlarmFilterSpec => Ok(from.as_any_box().downcast::<AlarmFilterSpec>()?),
            StructType::AlarmSetting => Ok(from.as_any_box().downcast::<AlarmSetting>()?),
            StructType::AlarmSpec => Ok(from.as_any_box().downcast::<AlarmSpec>()?),
            StructType::AlarmInfo => Ok(from.as_any_box().downcast::<AlarmInfo>()?),
            StructType::AlarmState => Ok(from.as_any_box().downcast::<AlarmState>()?),
            StructType::AlarmTriggeringActionTransitionSpec => Ok(from.as_any_box().downcast::<AlarmTriggeringActionTransitionSpec>()?),
            StructType::EventAlarmExpressionComparison => Ok(from.as_any_box().downcast::<EventAlarmExpressionComparison>()?),
            StructType::ClusterAction => Ok(from.as_any_box().downcast::<ClusterAction>()?),
            StructType::ClusterClusterInitialPlacementAction => Ok(from.as_any_box().downcast::<ClusterClusterInitialPlacementAction>()?),
            StructType::ClusterHostInfraUpdateHaModeAction => Ok(from.as_any_box().downcast::<ClusterHostInfraUpdateHaModeAction>()?),
            StructType::ClusterHostPowerAction => Ok(from.as_any_box().downcast::<ClusterHostPowerAction>()?),
            StructType::ClusterInitialPlacementAction => Ok(from.as_any_box().downcast::<ClusterInitialPlacementAction>()?),
            StructType::ClusterMigrationAction => Ok(from.as_any_box().downcast::<ClusterMigrationAction>()?),
            StructType::PlacementAction => Ok(from.as_any_box().downcast::<PlacementAction>()?),
            StructType::HbrDiskMigrationAction => Ok(from.as_any_box().downcast::<HbrDiskMigrationAction>()?),
            StructType::StorageMigrationAction => Ok(from.as_any_box().downcast::<StorageMigrationAction>()?),
            StructType::StoragePlacementAction => Ok(from.as_any_box().downcast::<StoragePlacementAction>()?),
            StructType::ClusterActionHistory => Ok(from.as_any_box().downcast::<ClusterActionHistory>()?),
            StructType::ClusterAttemptedVmInfo => Ok(from.as_any_box().downcast::<ClusterAttemptedVmInfo>()?),
            StructType::ClusterConfigInfo => Ok(from.as_any_box().downcast::<ClusterConfigInfo>()?),
            StructType::ClusterConfigSpec => Ok(from.as_any_box().downcast::<ClusterConfigSpec>()?),
            StructType::ClusterCryptoConfigInfo => Ok(from.as_any_box().downcast::<ClusterCryptoConfigInfo>()?),
            StructType::ClusterDasAamNodeState => Ok(from.as_any_box().downcast::<ClusterDasAamNodeState>()?),
            StructType::ClusterDasAdmissionControlInfo => Ok(from.as_any_box().downcast::<ClusterDasAdmissionControlInfo>()?),
            StructType::ClusterFailoverHostAdmissionControlInfo => Ok(from.as_any_box().downcast::<ClusterFailoverHostAdmissionControlInfo>()?),
            StructType::ClusterFailoverLevelAdmissionControlInfo => Ok(from.as_any_box().downcast::<ClusterFailoverLevelAdmissionControlInfo>()?),
            StructType::ClusterFailoverResourcesAdmissionControlInfo => Ok(from.as_any_box().downcast::<ClusterFailoverResourcesAdmissionControlInfo>()?),
            StructType::ClusterDasAdmissionControlPolicy => Ok(from.as_any_box().downcast::<ClusterDasAdmissionControlPolicy>()?),
            StructType::ClusterFailoverHostAdmissionControlPolicy => Ok(from.as_any_box().downcast::<ClusterFailoverHostAdmissionControlPolicy>()?),
            StructType::ClusterFailoverLevelAdmissionControlPolicy => Ok(from.as_any_box().downcast::<ClusterFailoverLevelAdmissionControlPolicy>()?),
            StructType::ClusterFailoverResourcesAdmissionControlPolicy => Ok(from.as_any_box().downcast::<ClusterFailoverResourcesAdmissionControlPolicy>()?),
            StructType::ClusterDasAdvancedRuntimeInfo => Ok(from.as_any_box().downcast::<ClusterDasAdvancedRuntimeInfo>()?),
            StructType::ClusterDasFailoverLevelAdvancedRuntimeInfo => Ok(from.as_any_box().downcast::<ClusterDasFailoverLevelAdvancedRuntimeInfo>()?),
            StructType::DasHeartbeatDatastoreInfo => Ok(from.as_any_box().downcast::<DasHeartbeatDatastoreInfo>()?),
            StructType::ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo => Ok(from.as_any_box().downcast::<ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo>()?),
            StructType::ClusterDasConfigInfo => Ok(from.as_any_box().downcast::<ClusterDasConfigInfo>()?),
            StructType::ClusterDasData => Ok(from.as_any_box().downcast::<ClusterDasData>()?),
            StructType::ClusterDasDataSummary => Ok(from.as_any_box().downcast::<ClusterDasDataSummary>()?),
            StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots => Ok(from.as_any_box().downcast::<ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots>()?),
            StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo => Ok(from.as_any_box().downcast::<ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo>()?),
            StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots => Ok(from.as_any_box().downcast::<ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots>()?),
            StructType::ClusterDasFdmHostState => Ok(from.as_any_box().downcast::<ClusterDasFdmHostState>()?),
            StructType::ClusterDasHostInfo => Ok(from.as_any_box().downcast::<ClusterDasHostInfo>()?),
            StructType::ClusterDasAamHostInfo => Ok(from.as_any_box().downcast::<ClusterDasAamHostInfo>()?),
            StructType::ClusterDasHostRecommendation => Ok(from.as_any_box().downcast::<ClusterDasHostRecommendation>()?),
            StructType::ClusterDasVmConfigInfo => Ok(from.as_any_box().downcast::<ClusterDasVmConfigInfo>()?),
            StructType::ClusterDasVmSettings => Ok(from.as_any_box().downcast::<ClusterDasVmSettings>()?),
            StructType::ClusterDpmConfigInfo => Ok(from.as_any_box().downcast::<ClusterDpmConfigInfo>()?),
            StructType::ClusterDpmHostConfigInfo => Ok(from.as_any_box().downcast::<ClusterDpmHostConfigInfo>()?),
            StructType::ClusterDrsConfigInfo => Ok(from.as_any_box().downcast::<ClusterDrsConfigInfo>()?),
            StructType::ClusterDrsFaults => Ok(from.as_any_box().downcast::<ClusterDrsFaults>()?),
            StructType::ClusterDrsFaultsFaultsByVm => Ok(from.as_any_box().downcast::<ClusterDrsFaultsFaultsByVm>()?),
            StructType::ClusterDrsFaultsFaultsByVirtualDisk => Ok(from.as_any_box().downcast::<ClusterDrsFaultsFaultsByVirtualDisk>()?),
            StructType::ClusterDrsMigration => Ok(from.as_any_box().downcast::<ClusterDrsMigration>()?),
            StructType::ClusterDrsRecommendation => Ok(from.as_any_box().downcast::<ClusterDrsRecommendation>()?),
            StructType::ClusterDrsVmConfigInfo => Ok(from.as_any_box().downcast::<ClusterDrsVmConfigInfo>()?),
            StructType::ClusterEvcManagerCheckResult => Ok(from.as_any_box().downcast::<ClusterEvcManagerCheckResult>()?),
            StructType::ClusterEvcManagerEvcState => Ok(from.as_any_box().downcast::<ClusterEvcManagerEvcState>()?),
            StructType::ClusterEnterMaintenanceResult => Ok(from.as_any_box().downcast::<ClusterEnterMaintenanceResult>()?),
            StructType::ClusterFailoverHostAdmissionControlInfoHostStatus => Ok(from.as_any_box().downcast::<ClusterFailoverHostAdmissionControlInfoHostStatus>()?),
            StructType::ClusterGroupInfo => Ok(from.as_any_box().downcast::<ClusterGroupInfo>()?),
            StructType::ClusterHostGroup => Ok(from.as_any_box().downcast::<ClusterHostGroup>()?),
            StructType::ClusterVmGroup => Ok(from.as_any_box().downcast::<ClusterVmGroup>()?),
            StructType::ClusterHostRecommendation => Ok(from.as_any_box().downcast::<ClusterHostRecommendation>()?),
            StructType::ClusterInfraUpdateHaConfigInfo => Ok(from.as_any_box().downcast::<ClusterInfraUpdateHaConfigInfo>()?),
            StructType::ClusterNotAttemptedVmInfo => Ok(from.as_any_box().downcast::<ClusterNotAttemptedVmInfo>()?),
            StructType::ClusterOrchestrationInfo => Ok(from.as_any_box().downcast::<ClusterOrchestrationInfo>()?),
            StructType::PlacementResult => Ok(from.as_any_box().downcast::<PlacementResult>()?),
            StructType::PlacementSpec => Ok(from.as_any_box().downcast::<PlacementSpec>()?),
            StructType::ClusterPowerOnVmResult => Ok(from.as_any_box().downcast::<ClusterPowerOnVmResult>()?),
            StructType::ClusterPreemptibleVmPairInfo => Ok(from.as_any_box().downcast::<ClusterPreemptibleVmPairInfo>()?),
            StructType::ClusterProactiveDrsConfigInfo => Ok(from.as_any_box().downcast::<ClusterProactiveDrsConfigInfo>()?),
            StructType::ClusterRecommendation => Ok(from.as_any_box().downcast::<ClusterRecommendation>()?),
            StructType::ClusterResourceUsageSummary => Ok(from.as_any_box().downcast::<ClusterResourceUsageSummary>()?),
            StructType::ClusterRuleInfo => Ok(from.as_any_box().downcast::<ClusterRuleInfo>()?),
            StructType::ClusterAffinityRuleSpec => Ok(from.as_any_box().downcast::<ClusterAffinityRuleSpec>()?),
            StructType::ClusterAntiAffinityRuleSpec => Ok(from.as_any_box().downcast::<ClusterAntiAffinityRuleSpec>()?),
            StructType::ClusterDependencyRuleInfo => Ok(from.as_any_box().downcast::<ClusterDependencyRuleInfo>()?),
            StructType::ClusterVmHostRuleInfo => Ok(from.as_any_box().downcast::<ClusterVmHostRuleInfo>()?),
            StructType::VirtualDiskAntiAffinityRuleSpec => Ok(from.as_any_box().downcast::<VirtualDiskAntiAffinityRuleSpec>()?),
            StructType::VirtualDiskRuleSpec => Ok(from.as_any_box().downcast::<VirtualDiskRuleSpec>()?),
            StructType::ClusterSlotPolicy => Ok(from.as_any_box().downcast::<ClusterSlotPolicy>()?),
            StructType::ClusterFixedSizeSlotPolicy => Ok(from.as_any_box().downcast::<ClusterFixedSizeSlotPolicy>()?),
            StructType::ClusterSystemVMsConfigInfo => Ok(from.as_any_box().downcast::<ClusterSystemVMsConfigInfo>()?),
            StructType::ClusterSystemVMsConfigSpec => Ok(from.as_any_box().downcast::<ClusterSystemVMsConfigSpec>()?),
            StructType::ClusterUsageSummary => Ok(from.as_any_box().downcast::<ClusterUsageSummary>()?),
            StructType::ClusterVmComponentProtectionSettings => Ok(from.as_any_box().downcast::<ClusterVmComponentProtectionSettings>()?),
            StructType::ClusterVmOrchestrationInfo => Ok(from.as_any_box().downcast::<ClusterVmOrchestrationInfo>()?),
            StructType::ClusterVmReadiness => Ok(from.as_any_box().downcast::<ClusterVmReadiness>()?),
            StructType::ClusterVmToolsMonitoringSettings => Ok(from.as_any_box().downcast::<ClusterVmToolsMonitoringSettings>()?),
            StructType::DistributedVirtualPort => Ok(from.as_any_box().downcast::<DistributedVirtualPort>()?),
            StructType::DvPortConfigInfo => Ok(from.as_any_box().downcast::<DvPortConfigInfo>()?),
            StructType::DvPortConfigSpec => Ok(from.as_any_box().downcast::<DvPortConfigSpec>()?),
            StructType::DvsFilterParameter => Ok(from.as_any_box().downcast::<DvsFilterParameter>()?),
            StructType::DvsHostLocalPortInfo => Ok(from.as_any_box().downcast::<DvsHostLocalPortInfo>()?),
            StructType::DvPortStatus => Ok(from.as_any_box().downcast::<DvPortStatus>()?),
            StructType::DvPortSetting => Ok(from.as_any_box().downcast::<DvPortSetting>()?),
            StructType::VMwareDvsPortSetting => Ok(from.as_any_box().downcast::<VMwareDvsPortSetting>()?),
            StructType::DvPortState => Ok(from.as_any_box().downcast::<DvPortState>()?),
            StructType::DvPortgroupConfigInfo => Ok(from.as_any_box().downcast::<DvPortgroupConfigInfo>()?),
            StructType::DvPortgroupConfigSpec => Ok(from.as_any_box().downcast::<DvPortgroupConfigSpec>()?),
            StructType::DistributedVirtualPortgroupNsxPortgroupOperationResult => Ok(from.as_any_box().downcast::<DistributedVirtualPortgroupNsxPortgroupOperationResult>()?),
            StructType::DvPortgroupPolicy => Ok(from.as_any_box().downcast::<DvPortgroupPolicy>()?),
            StructType::VMwareDvsPortgroupPolicy => Ok(from.as_any_box().downcast::<VMwareDvsPortgroupPolicy>()?),
            StructType::DistributedVirtualPortgroupProblem => Ok(from.as_any_box().downcast::<DistributedVirtualPortgroupProblem>()?),
            StructType::DistributedVirtualPortgroupInfo => Ok(from.as_any_box().downcast::<DistributedVirtualPortgroupInfo>()?),
            StructType::DistributedVirtualSwitchInfo => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchInfo>()?),
            StructType::DistributedVirtualSwitchManagerCompatibilityResult => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerCompatibilityResult>()?),
            StructType::DvsManagerDvsConfigTarget => Ok(from.as_any_box().downcast::<DvsManagerDvsConfigTarget>()?),
            StructType::DistributedVirtualSwitchManagerDvsProductSpec => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerDvsProductSpec>()?),
            StructType::DistributedVirtualSwitchManagerHostContainer => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerHostContainer>()?),
            StructType::DistributedVirtualSwitchManagerHostDvsFilterSpec => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerHostDvsFilterSpec>()?),
            StructType::DistributedVirtualSwitchManagerHostArrayFilter => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerHostArrayFilter>()?),
            StructType::DistributedVirtualSwitchManagerHostContainerFilter => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerHostContainerFilter>()?),
            StructType::DistributedVirtualSwitchManagerHostDvsMembershipFilter => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerHostDvsMembershipFilter>()?),
            StructType::DistributedVirtualSwitchManagerImportResult => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerImportResult>()?),
            StructType::DvsManagerPhysicalNicsList => Ok(from.as_any_box().downcast::<DvsManagerPhysicalNicsList>()?),
            StructType::EntityBackup => Ok(from.as_any_box().downcast::<EntityBackup>()?),
            StructType::EntityBackupConfig => Ok(from.as_any_box().downcast::<EntityBackupConfig>()?),
            StructType::DistributedVirtualSwitchHostMember => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchHostMember>()?),
            StructType::DistributedVirtualSwitchHostMemberBacking => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchHostMemberBacking>()?),
            StructType::DistributedVirtualSwitchHostMemberPnicBacking => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchHostMemberPnicBacking>()?),
            StructType::DistributedVirtualSwitchHostMemberConfigInfo => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchHostMemberConfigInfo>()?),
            StructType::DistributedVirtualSwitchHostMemberConfigSpec => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchHostMemberConfigSpec>()?),
            StructType::HostMemberHealthCheckResult => Ok(from.as_any_box().downcast::<HostMemberHealthCheckResult>()?),
            StructType::HostMemberUplinkHealthCheckResult => Ok(from.as_any_box().downcast::<HostMemberUplinkHealthCheckResult>()?),
            StructType::VMwareDvsMtuHealthCheckResult => Ok(from.as_any_box().downcast::<VMwareDvsMtuHealthCheckResult>()?),
            StructType::VMwareDvsVlanHealthCheckResult => Ok(from.as_any_box().downcast::<VMwareDvsVlanHealthCheckResult>()?),
            StructType::VMwareDvsTeamingHealthCheckResult => Ok(from.as_any_box().downcast::<VMwareDvsTeamingHealthCheckResult>()?),
            StructType::DistributedVirtualSwitchHostMemberPnicSpec => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchHostMemberPnicSpec>()?),
            StructType::HostMemberRuntimeInfo => Ok(from.as_any_box().downcast::<HostMemberRuntimeInfo>()?),
            StructType::DistributedVirtualSwitchHostMemberRuntimeState => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchHostMemberRuntimeState>()?),
            StructType::DistributedVirtualSwitchHostMemberTransportZoneInfo => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchHostMemberTransportZoneInfo>()?),
            StructType::DistributedVirtualSwitchHostProductSpec => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchHostProductSpec>()?),
            StructType::DistributedVirtualSwitchKeyedOpaqueBlob => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchKeyedOpaqueBlob>()?),
            StructType::DistributedVirtualSwitchNetworkOffloadSpec => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchNetworkOffloadSpec>()?),
            StructType::DvsNetworkResourcePool => Ok(from.as_any_box().downcast::<DvsNetworkResourcePool>()?),
            StructType::DvsNetworkResourcePoolAllocationInfo => Ok(from.as_any_box().downcast::<DvsNetworkResourcePoolAllocationInfo>()?),
            StructType::DvsNetworkResourcePoolConfigSpec => Ok(from.as_any_box().downcast::<DvsNetworkResourcePoolConfigSpec>()?),
            StructType::DistributedVirtualSwitchPortConnectee => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchPortConnectee>()?),
            StructType::DistributedVirtualSwitchPortConnection => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchPortConnection>()?),
            StructType::DistributedVirtualSwitchPortCriteria => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchPortCriteria>()?),
            StructType::DistributedVirtualSwitchPortStatistics => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchPortStatistics>()?),
            StructType::DistributedVirtualSwitchProductSpec => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchProductSpec>()?),
            StructType::DvsTrafficRule => Ok(from.as_any_box().downcast::<DvsTrafficRule>()?),
            StructType::DvsNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsNetworkRuleAction>()?),
            StructType::DvsAcceptNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsAcceptNetworkRuleAction>()?),
            StructType::DvsCopyNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsCopyNetworkRuleAction>()?),
            StructType::DvsDropNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsDropNetworkRuleAction>()?),
            StructType::DvsGreEncapNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsGreEncapNetworkRuleAction>()?),
            StructType::DvsLogNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsLogNetworkRuleAction>()?),
            StructType::DvsMacRewriteNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsMacRewriteNetworkRuleAction>()?),
            StructType::DvsPuntNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsPuntNetworkRuleAction>()?),
            StructType::DvsRateLimitNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsRateLimitNetworkRuleAction>()?),
            StructType::DvsUpdateTagNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsUpdateTagNetworkRuleAction>()?),
            StructType::DvsNetworkRuleQualifier => Ok(from.as_any_box().downcast::<DvsNetworkRuleQualifier>()?),
            StructType::DvsIpNetworkRuleQualifier => Ok(from.as_any_box().downcast::<DvsIpNetworkRuleQualifier>()?),
            StructType::DvsMacNetworkRuleQualifier => Ok(from.as_any_box().downcast::<DvsMacNetworkRuleQualifier>()?),
            StructType::DvsSystemTrafficNetworkRuleQualifier => Ok(from.as_any_box().downcast::<DvsSystemTrafficNetworkRuleQualifier>()?),
            StructType::DvsTrafficRuleset => Ok(from.as_any_box().downcast::<DvsTrafficRuleset>()?),
            StructType::DvsVmVnicNetworkResourcePool => Ok(from.as_any_box().downcast::<DvsVmVnicNetworkResourcePool>()?),
            StructType::DvsVmVnicResourcePoolConfigSpec => Ok(from.as_any_box().downcast::<DvsVmVnicResourcePoolConfigSpec>()?),
            StructType::DvsVmVnicResourceAllocation => Ok(from.as_any_box().downcast::<DvsVmVnicResourceAllocation>()?),
            StructType::DvsVmVnicNetworkResourcePoolRuntimeInfo => Ok(from.as_any_box().downcast::<DvsVmVnicNetworkResourcePoolRuntimeInfo>()?),
            StructType::DvsVnicAllocatedResource => Ok(from.as_any_box().downcast::<DvsVnicAllocatedResource>()?),
            StructType::VMwareDvsDpuCapability => Ok(from.as_any_box().downcast::<VMwareDvsDpuCapability>()?),
            StructType::VMwareIpfixConfig => Ok(from.as_any_box().downcast::<VMwareIpfixConfig>()?),
            StructType::VMwareDvsIpfixCapability => Ok(from.as_any_box().downcast::<VMwareDvsIpfixCapability>()?),
            StructType::VMwareDvsLacpCapability => Ok(from.as_any_box().downcast::<VMwareDvsLacpCapability>()?),
            StructType::VMwareDvsLacpGroupConfig => Ok(from.as_any_box().downcast::<VMwareDvsLacpGroupConfig>()?),
            StructType::VMwareDvsLacpGroupSpec => Ok(from.as_any_box().downcast::<VMwareDvsLacpGroupSpec>()?),
            StructType::VMwareDvsLagIpfixConfig => Ok(from.as_any_box().downcast::<VMwareDvsLagIpfixConfig>()?),
            StructType::VMwareDvsLagVlanConfig => Ok(from.as_any_box().downcast::<VMwareDvsLagVlanConfig>()?),
            StructType::VMwareDvsMtuCapability => Ok(from.as_any_box().downcast::<VMwareDvsMtuCapability>()?),
            StructType::VMwareDvsPvlanConfigSpec => Ok(from.as_any_box().downcast::<VMwareDvsPvlanConfigSpec>()?),
            StructType::VMwareDvsPvlanMapEntry => Ok(from.as_any_box().downcast::<VMwareDvsPvlanMapEntry>()?),
            StructType::VMwareDvsVspanConfigSpec => Ok(from.as_any_box().downcast::<VMwareDvsVspanConfigSpec>()?),
            StructType::VMwareDvsVspanCapability => Ok(from.as_any_box().downcast::<VMwareDvsVspanCapability>()?),
            StructType::VMwareVspanPort => Ok(from.as_any_box().downcast::<VMwareVspanPort>()?),
            StructType::VMwareVspanSession => Ok(from.as_any_box().downcast::<VMwareVspanSession>()?),
            StructType::CryptoKeyId => Ok(from.as_any_box().downcast::<CryptoKeyId>()?),
            StructType::CryptoKeyPlain => Ok(from.as_any_box().downcast::<CryptoKeyPlain>()?),
            StructType::CryptoKeyResult => Ok(from.as_any_box().downcast::<CryptoKeyResult>()?),
            StructType::CryptoManagerHostKeyStatus => Ok(from.as_any_box().downcast::<CryptoManagerHostKeyStatus>()?),
            StructType::CryptoManagerKmipCertSignRequest => Ok(from.as_any_box().downcast::<CryptoManagerKmipCertSignRequest>()?),
            StructType::CryptoManagerKmipCertificateInfo => Ok(from.as_any_box().downcast::<CryptoManagerKmipCertificateInfo>()?),
            StructType::CryptoManagerKmipClusterStatus => Ok(from.as_any_box().downcast::<CryptoManagerKmipClusterStatus>()?),
            StructType::CryptoManagerKmipCryptoKeyStatus => Ok(from.as_any_box().downcast::<CryptoManagerKmipCryptoKeyStatus>()?),
            StructType::CryptoManagerKmipCustomAttributeSpec => Ok(from.as_any_box().downcast::<CryptoManagerKmipCustomAttributeSpec>()?),
            StructType::CryptoManagerKmipServerCertInfo => Ok(from.as_any_box().downcast::<CryptoManagerKmipServerCertInfo>()?),
            StructType::CryptoManagerKmipServerStatus => Ok(from.as_any_box().downcast::<CryptoManagerKmipServerStatus>()?),
            StructType::CryptoSpec => Ok(from.as_any_box().downcast::<CryptoSpec>()?),
            StructType::CryptoSpecDecrypt => Ok(from.as_any_box().downcast::<CryptoSpecDecrypt>()?),
            StructType::CryptoSpecDeepRecrypt => Ok(from.as_any_box().downcast::<CryptoSpecDeepRecrypt>()?),
            StructType::CryptoSpecEncrypt => Ok(from.as_any_box().downcast::<CryptoSpecEncrypt>()?),
            StructType::CryptoSpecNoOp => Ok(from.as_any_box().downcast::<CryptoSpecNoOp>()?),
            StructType::CryptoSpecRegister => Ok(from.as_any_box().downcast::<CryptoSpecRegister>()?),
            StructType::CryptoSpecShallowRecrypt => Ok(from.as_any_box().downcast::<CryptoSpecShallowRecrypt>()?),
            StructType::KeyProviderId => Ok(from.as_any_box().downcast::<KeyProviderId>()?),
            StructType::KmipClusterInfo => Ok(from.as_any_box().downcast::<KmipClusterInfo>()?),
            StructType::KmipServerInfo => Ok(from.as_any_box().downcast::<KmipServerInfo>()?),
            StructType::KmipServerSpec => Ok(from.as_any_box().downcast::<KmipServerSpec>()?),
            StructType::KmipServerStatus => Ok(from.as_any_box().downcast::<KmipServerStatus>()?),
            StructType::ChangesInfoEventArgument => Ok(from.as_any_box().downcast::<ChangesInfoEventArgument>()?),
            StructType::DvsOutOfSyncHostArgument => Ok(from.as_any_box().downcast::<DvsOutOfSyncHostArgument>()?),
            StructType::Event => Ok(from.as_any_box().downcast::<Event>()?),
            StructType::AlarmEvent => Ok(from.as_any_box().downcast::<AlarmEvent>()?),
            StructType::AlarmAcknowledgedEvent => Ok(from.as_any_box().downcast::<AlarmAcknowledgedEvent>()?),
            StructType::AlarmActionTriggeredEvent => Ok(from.as_any_box().downcast::<AlarmActionTriggeredEvent>()?),
            StructType::AlarmClearedEvent => Ok(from.as_any_box().downcast::<AlarmClearedEvent>()?),
            StructType::AlarmCreatedEvent => Ok(from.as_any_box().downcast::<AlarmCreatedEvent>()?),
            StructType::AlarmEmailCompletedEvent => Ok(from.as_any_box().downcast::<AlarmEmailCompletedEvent>()?),
            StructType::AlarmEmailFailedEvent => Ok(from.as_any_box().downcast::<AlarmEmailFailedEvent>()?),
            StructType::AlarmReconfiguredEvent => Ok(from.as_any_box().downcast::<AlarmReconfiguredEvent>()?),
            StructType::AlarmRemovedEvent => Ok(from.as_any_box().downcast::<AlarmRemovedEvent>()?),
            StructType::AlarmScriptCompleteEvent => Ok(from.as_any_box().downcast::<AlarmScriptCompleteEvent>()?),
            StructType::AlarmScriptFailedEvent => Ok(from.as_any_box().downcast::<AlarmScriptFailedEvent>()?),
            StructType::AlarmSnmpCompletedEvent => Ok(from.as_any_box().downcast::<AlarmSnmpCompletedEvent>()?),
            StructType::AlarmSnmpFailedEvent => Ok(from.as_any_box().downcast::<AlarmSnmpFailedEvent>()?),
            StructType::AlarmStatusChangedEvent => Ok(from.as_any_box().downcast::<AlarmStatusChangedEvent>()?),
            StructType::AuthorizationEvent => Ok(from.as_any_box().downcast::<AuthorizationEvent>()?),
            StructType::PermissionEvent => Ok(from.as_any_box().downcast::<PermissionEvent>()?),
            StructType::PermissionAddedEvent => Ok(from.as_any_box().downcast::<PermissionAddedEvent>()?),
            StructType::PermissionRemovedEvent => Ok(from.as_any_box().downcast::<PermissionRemovedEvent>()?),
            StructType::PermissionUpdatedEvent => Ok(from.as_any_box().downcast::<PermissionUpdatedEvent>()?),
            StructType::RoleEvent => Ok(from.as_any_box().downcast::<RoleEvent>()?),
            StructType::RoleAddedEvent => Ok(from.as_any_box().downcast::<RoleAddedEvent>()?),
            StructType::RoleRemovedEvent => Ok(from.as_any_box().downcast::<RoleRemovedEvent>()?),
            StructType::RoleUpdatedEvent => Ok(from.as_any_box().downcast::<RoleUpdatedEvent>()?),
            StructType::ClusterEvent => Ok(from.as_any_box().downcast::<ClusterEvent>()?),
            StructType::ClusterComplianceCheckedEvent => Ok(from.as_any_box().downcast::<ClusterComplianceCheckedEvent>()?),
            StructType::ClusterCreatedEvent => Ok(from.as_any_box().downcast::<ClusterCreatedEvent>()?),
            StructType::ClusterDestroyedEvent => Ok(from.as_any_box().downcast::<ClusterDestroyedEvent>()?),
            StructType::ClusterOvercommittedEvent => Ok(from.as_any_box().downcast::<ClusterOvercommittedEvent>()?),
            StructType::HostOvercommittedEvent => Ok(from.as_any_box().downcast::<HostOvercommittedEvent>()?),
            StructType::ClusterReconfiguredEvent => Ok(from.as_any_box().downcast::<ClusterReconfiguredEvent>()?),
            StructType::ClusterStatusChangedEvent => Ok(from.as_any_box().downcast::<ClusterStatusChangedEvent>()?),
            StructType::HostStatusChangedEvent => Ok(from.as_any_box().downcast::<HostStatusChangedEvent>()?),
            StructType::DasAdmissionControlDisabledEvent => Ok(from.as_any_box().downcast::<DasAdmissionControlDisabledEvent>()?),
            StructType::DasAdmissionControlEnabledEvent => Ok(from.as_any_box().downcast::<DasAdmissionControlEnabledEvent>()?),
            StructType::DasAgentFoundEvent => Ok(from.as_any_box().downcast::<DasAgentFoundEvent>()?),
            StructType::DasAgentUnavailableEvent => Ok(from.as_any_box().downcast::<DasAgentUnavailableEvent>()?),
            StructType::DasClusterIsolatedEvent => Ok(from.as_any_box().downcast::<DasClusterIsolatedEvent>()?),
            StructType::DasDisabledEvent => Ok(from.as_any_box().downcast::<DasDisabledEvent>()?),
            StructType::DasEnabledEvent => Ok(from.as_any_box().downcast::<DasEnabledEvent>()?),
            StructType::DasHostFailedEvent => Ok(from.as_any_box().downcast::<DasHostFailedEvent>()?),
            StructType::DasHostIsolatedEvent => Ok(from.as_any_box().downcast::<DasHostIsolatedEvent>()?),
            StructType::DrsDisabledEvent => Ok(from.as_any_box().downcast::<DrsDisabledEvent>()?),
            StructType::DrsEnabledEvent => Ok(from.as_any_box().downcast::<DrsEnabledEvent>()?),
            StructType::DrsInvocationFailedEvent => Ok(from.as_any_box().downcast::<DrsInvocationFailedEvent>()?),
            StructType::DrsRecoveredFromFailureEvent => Ok(from.as_any_box().downcast::<DrsRecoveredFromFailureEvent>()?),
            StructType::FailoverLevelRestored => Ok(from.as_any_box().downcast::<FailoverLevelRestored>()?),
            StructType::HostMonitoringStateChangedEvent => Ok(from.as_any_box().downcast::<HostMonitoringStateChangedEvent>()?),
            StructType::InsufficientFailoverResourcesEvent => Ok(from.as_any_box().downcast::<InsufficientFailoverResourcesEvent>()?),
            StructType::VmHealthMonitoringStateChangedEvent => Ok(from.as_any_box().downcast::<VmHealthMonitoringStateChangedEvent>()?),
            StructType::CustomFieldEvent => Ok(from.as_any_box().downcast::<CustomFieldEvent>()?),
            StructType::CustomFieldDefEvent => Ok(from.as_any_box().downcast::<CustomFieldDefEvent>()?),
            StructType::CustomFieldDefAddedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefAddedEvent>()?),
            StructType::CustomFieldDefRemovedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefRemovedEvent>()?),
            StructType::CustomFieldDefRenamedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefRenamedEvent>()?),
            StructType::CustomFieldValueChangedEvent => Ok(from.as_any_box().downcast::<CustomFieldValueChangedEvent>()?),
            StructType::DvPortgroupEvent => Ok(from.as_any_box().downcast::<DvPortgroupEvent>()?),
            StructType::DvPortgroupCreatedEvent => Ok(from.as_any_box().downcast::<DvPortgroupCreatedEvent>()?),
            StructType::DvPortgroupDestroyedEvent => Ok(from.as_any_box().downcast::<DvPortgroupDestroyedEvent>()?),
            StructType::DvPortgroupReconfiguredEvent => Ok(from.as_any_box().downcast::<DvPortgroupReconfiguredEvent>()?),
            StructType::DvPortgroupRenamedEvent => Ok(from.as_any_box().downcast::<DvPortgroupRenamedEvent>()?),
            StructType::DvpgImportEvent => Ok(from.as_any_box().downcast::<DvpgImportEvent>()?),
            StructType::DvpgRestoreEvent => Ok(from.as_any_box().downcast::<DvpgRestoreEvent>()?),
            StructType::DatacenterEvent => Ok(from.as_any_box().downcast::<DatacenterEvent>()?),
            StructType::DatacenterCreatedEvent => Ok(from.as_any_box().downcast::<DatacenterCreatedEvent>()?),
            StructType::DatacenterRenamedEvent => Ok(from.as_any_box().downcast::<DatacenterRenamedEvent>()?),
            StructType::DatastoreEvent => Ok(from.as_any_box().downcast::<DatastoreEvent>()?),
            StructType::DatastoreCapacityIncreasedEvent => Ok(from.as_any_box().downcast::<DatastoreCapacityIncreasedEvent>()?),
            StructType::DatastoreDestroyedEvent => Ok(from.as_any_box().downcast::<DatastoreDestroyedEvent>()?),
            StructType::DatastoreDuplicatedEvent => Ok(from.as_any_box().downcast::<DatastoreDuplicatedEvent>()?),
            StructType::DatastoreFileEvent => Ok(from.as_any_box().downcast::<DatastoreFileEvent>()?),
            StructType::DatastoreFileCopiedEvent => Ok(from.as_any_box().downcast::<DatastoreFileCopiedEvent>()?),
            StructType::DatastoreFileDeletedEvent => Ok(from.as_any_box().downcast::<DatastoreFileDeletedEvent>()?),
            StructType::DatastoreFileMovedEvent => Ok(from.as_any_box().downcast::<DatastoreFileMovedEvent>()?),
            StructType::DatastoreIormReconfiguredEvent => Ok(from.as_any_box().downcast::<DatastoreIormReconfiguredEvent>()?),
            StructType::DatastoreRenamedEvent => Ok(from.as_any_box().downcast::<DatastoreRenamedEvent>()?),
            StructType::NonViWorkloadDetectedOnDatastoreEvent => Ok(from.as_any_box().downcast::<NonViWorkloadDetectedOnDatastoreEvent>()?),
            StructType::DvsEvent => Ok(from.as_any_box().downcast::<DvsEvent>()?),
            StructType::DvsCreatedEvent => Ok(from.as_any_box().downcast::<DvsCreatedEvent>()?),
            StructType::DvsDestroyedEvent => Ok(from.as_any_box().downcast::<DvsDestroyedEvent>()?),
            StructType::DvsHostBackInSyncEvent => Ok(from.as_any_box().downcast::<DvsHostBackInSyncEvent>()?),
            StructType::DvsHostJoinedEvent => Ok(from.as_any_box().downcast::<DvsHostJoinedEvent>()?),
            StructType::DvsHostLeftEvent => Ok(from.as_any_box().downcast::<DvsHostLeftEvent>()?),
            StructType::DvsHostStatusUpdated => Ok(from.as_any_box().downcast::<DvsHostStatusUpdated>()?),
            StructType::DvsHostWentOutOfSyncEvent => Ok(from.as_any_box().downcast::<DvsHostWentOutOfSyncEvent>()?),
            StructType::DvsImportEvent => Ok(from.as_any_box().downcast::<DvsImportEvent>()?),
            StructType::DvsMergedEvent => Ok(from.as_any_box().downcast::<DvsMergedEvent>()?),
            StructType::DvsPortBlockedEvent => Ok(from.as_any_box().downcast::<DvsPortBlockedEvent>()?),
            StructType::DvsPortConnectedEvent => Ok(from.as_any_box().downcast::<DvsPortConnectedEvent>()?),
            StructType::DvsPortCreatedEvent => Ok(from.as_any_box().downcast::<DvsPortCreatedEvent>()?),
            StructType::DvsPortDeletedEvent => Ok(from.as_any_box().downcast::<DvsPortDeletedEvent>()?),
            StructType::DvsPortDisconnectedEvent => Ok(from.as_any_box().downcast::<DvsPortDisconnectedEvent>()?),
            StructType::DvsPortEnteredPassthruEvent => Ok(from.as_any_box().downcast::<DvsPortEnteredPassthruEvent>()?),
            StructType::DvsPortExitedPassthruEvent => Ok(from.as_any_box().downcast::<DvsPortExitedPassthruEvent>()?),
            StructType::DvsPortJoinPortgroupEvent => Ok(from.as_any_box().downcast::<DvsPortJoinPortgroupEvent>()?),
            StructType::DvsPortLeavePortgroupEvent => Ok(from.as_any_box().downcast::<DvsPortLeavePortgroupEvent>()?),
            StructType::DvsPortLinkDownEvent => Ok(from.as_any_box().downcast::<DvsPortLinkDownEvent>()?),
            StructType::DvsPortLinkUpEvent => Ok(from.as_any_box().downcast::<DvsPortLinkUpEvent>()?),
            StructType::DvsPortReconfiguredEvent => Ok(from.as_any_box().downcast::<DvsPortReconfiguredEvent>()?),
            StructType::DvsPortRuntimeChangeEvent => Ok(from.as_any_box().downcast::<DvsPortRuntimeChangeEvent>()?),
            StructType::DvsPortUnblockedEvent => Ok(from.as_any_box().downcast::<DvsPortUnblockedEvent>()?),
            StructType::DvsPortVendorSpecificStateChangeEvent => Ok(from.as_any_box().downcast::<DvsPortVendorSpecificStateChangeEvent>()?),
            StructType::DvsReconfiguredEvent => Ok(from.as_any_box().downcast::<DvsReconfiguredEvent>()?),
            StructType::DvsRenamedEvent => Ok(from.as_any_box().downcast::<DvsRenamedEvent>()?),
            StructType::DvsRestoreEvent => Ok(from.as_any_box().downcast::<DvsRestoreEvent>()?),
            StructType::DvsUpgradeAvailableEvent => Ok(from.as_any_box().downcast::<DvsUpgradeAvailableEvent>()?),
            StructType::DvsUpgradeInProgressEvent => Ok(from.as_any_box().downcast::<DvsUpgradeInProgressEvent>()?),
            StructType::DvsUpgradeRejectedEvent => Ok(from.as_any_box().downcast::<DvsUpgradeRejectedEvent>()?),
            StructType::DvsUpgradedEvent => Ok(from.as_any_box().downcast::<DvsUpgradedEvent>()?),
            StructType::HostLocalPortCreatedEvent => Ok(from.as_any_box().downcast::<HostLocalPortCreatedEvent>()?),
            StructType::OutOfSyncDvsHost => Ok(from.as_any_box().downcast::<OutOfSyncDvsHost>()?),
            StructType::RecoveryEvent => Ok(from.as_any_box().downcast::<RecoveryEvent>()?),
            StructType::RollbackEvent => Ok(from.as_any_box().downcast::<RollbackEvent>()?),
            StructType::VmVnicPoolReservationViolationClearEvent => Ok(from.as_any_box().downcast::<VmVnicPoolReservationViolationClearEvent>()?),
            StructType::VmVnicPoolReservationViolationRaiseEvent => Ok(from.as_any_box().downcast::<VmVnicPoolReservationViolationRaiseEvent>()?),
            StructType::EventEx => Ok(from.as_any_box().downcast::<EventEx>()?),
            StructType::GeneralEvent => Ok(from.as_any_box().downcast::<GeneralEvent>()?),
            StructType::ExtendedEvent => Ok(from.as_any_box().downcast::<ExtendedEvent>()?),
            StructType::GeneralHostErrorEvent => Ok(from.as_any_box().downcast::<GeneralHostErrorEvent>()?),
            StructType::GeneralHostInfoEvent => Ok(from.as_any_box().downcast::<GeneralHostInfoEvent>()?),
            StructType::GeneralHostWarningEvent => Ok(from.as_any_box().downcast::<GeneralHostWarningEvent>()?),
            StructType::GeneralUserEvent => Ok(from.as_any_box().downcast::<GeneralUserEvent>()?),
            StructType::GeneralVmErrorEvent => Ok(from.as_any_box().downcast::<GeneralVmErrorEvent>()?),
            StructType::GeneralVmInfoEvent => Ok(from.as_any_box().downcast::<GeneralVmInfoEvent>()?),
            StructType::GeneralVmWarningEvent => Ok(from.as_any_box().downcast::<GeneralVmWarningEvent>()?),
            StructType::HealthStatusChangedEvent => Ok(from.as_any_box().downcast::<HealthStatusChangedEvent>()?),
            StructType::HostEvent => Ok(from.as_any_box().downcast::<HostEvent>()?),
            StructType::AccountCreatedEvent => Ok(from.as_any_box().downcast::<AccountCreatedEvent>()?),
            StructType::AccountRemovedEvent => Ok(from.as_any_box().downcast::<AccountRemovedEvent>()?),
            StructType::AccountUpdatedEvent => Ok(from.as_any_box().downcast::<AccountUpdatedEvent>()?),
            StructType::AdminPasswordNotChangedEvent => Ok(from.as_any_box().downcast::<AdminPasswordNotChangedEvent>()?),
            StructType::CanceledHostOperationEvent => Ok(from.as_any_box().downcast::<CanceledHostOperationEvent>()?),
            StructType::DatastoreDiscoveredEvent => Ok(from.as_any_box().downcast::<DatastoreDiscoveredEvent>()?),
            StructType::DatastorePrincipalConfigured => Ok(from.as_any_box().downcast::<DatastorePrincipalConfigured>()?),
            StructType::DatastoreRemovedOnHostEvent => Ok(from.as_any_box().downcast::<DatastoreRemovedOnHostEvent>()?),
            StructType::DatastoreRenamedOnHostEvent => Ok(from.as_any_box().downcast::<DatastoreRenamedOnHostEvent>()?),
            StructType::DrsResourceConfigureFailedEvent => Ok(from.as_any_box().downcast::<DrsResourceConfigureFailedEvent>()?),
            StructType::DrsResourceConfigureSyncedEvent => Ok(from.as_any_box().downcast::<DrsResourceConfigureSyncedEvent>()?),
            StructType::DuplicateIpDetectedEvent => Ok(from.as_any_box().downcast::<DuplicateIpDetectedEvent>()?),
            StructType::DvsHealthStatusChangeEvent => Ok(from.as_any_box().downcast::<DvsHealthStatusChangeEvent>()?),
            StructType::MtuMatchEvent => Ok(from.as_any_box().downcast::<MtuMatchEvent>()?),
            StructType::MtuMismatchEvent => Ok(from.as_any_box().downcast::<MtuMismatchEvent>()?),
            StructType::TeamingMatchEvent => Ok(from.as_any_box().downcast::<TeamingMatchEvent>()?),
            StructType::TeamingMisMatchEvent => Ok(from.as_any_box().downcast::<TeamingMisMatchEvent>()?),
            StructType::UplinkPortMtuNotSupportEvent => Ok(from.as_any_box().downcast::<UplinkPortMtuNotSupportEvent>()?),
            StructType::UplinkPortMtuSupportEvent => Ok(from.as_any_box().downcast::<UplinkPortMtuSupportEvent>()?),
            StructType::UplinkPortVlanTrunkedEvent => Ok(from.as_any_box().downcast::<UplinkPortVlanTrunkedEvent>()?),
            StructType::UplinkPortVlanUntrunkedEvent => Ok(from.as_any_box().downcast::<UplinkPortVlanUntrunkedEvent>()?),
            StructType::EnteredMaintenanceModeEvent => Ok(from.as_any_box().downcast::<EnteredMaintenanceModeEvent>()?),
            StructType::EnteredStandbyModeEvent => Ok(from.as_any_box().downcast::<EnteredStandbyModeEvent>()?),
            StructType::DrsEnteredStandbyModeEvent => Ok(from.as_any_box().downcast::<DrsEnteredStandbyModeEvent>()?),
            StructType::EnteringMaintenanceModeEvent => Ok(from.as_any_box().downcast::<EnteringMaintenanceModeEvent>()?),
            StructType::EnteringStandbyModeEvent => Ok(from.as_any_box().downcast::<EnteringStandbyModeEvent>()?),
            StructType::DrsEnteringStandbyModeEvent => Ok(from.as_any_box().downcast::<DrsEnteringStandbyModeEvent>()?),
            StructType::ExitMaintenanceModeEvent => Ok(from.as_any_box().downcast::<ExitMaintenanceModeEvent>()?),
            StructType::ExitStandbyModeFailedEvent => Ok(from.as_any_box().downcast::<ExitStandbyModeFailedEvent>()?),
            StructType::DrsExitStandbyModeFailedEvent => Ok(from.as_any_box().downcast::<DrsExitStandbyModeFailedEvent>()?),
            StructType::ExitedStandbyModeEvent => Ok(from.as_any_box().downcast::<ExitedStandbyModeEvent>()?),
            StructType::DrsExitedStandbyModeEvent => Ok(from.as_any_box().downcast::<DrsExitedStandbyModeEvent>()?),
            StructType::ExitingStandbyModeEvent => Ok(from.as_any_box().downcast::<ExitingStandbyModeEvent>()?),
            StructType::DrsExitingStandbyModeEvent => Ok(from.as_any_box().downcast::<DrsExitingStandbyModeEvent>()?),
            StructType::GhostDvsProxySwitchDetectedEvent => Ok(from.as_any_box().downcast::<GhostDvsProxySwitchDetectedEvent>()?),
            StructType::GhostDvsProxySwitchRemovedEvent => Ok(from.as_any_box().downcast::<GhostDvsProxySwitchRemovedEvent>()?),
            StructType::HostAddFailedEvent => Ok(from.as_any_box().downcast::<HostAddFailedEvent>()?),
            StructType::HostAddedEvent => Ok(from.as_any_box().downcast::<HostAddedEvent>()?),
            StructType::HostAdminDisableEvent => Ok(from.as_any_box().downcast::<HostAdminDisableEvent>()?),
            StructType::HostAdminEnableEvent => Ok(from.as_any_box().downcast::<HostAdminEnableEvent>()?),
            StructType::HostCnxFailedAccountFailedEvent => Ok(from.as_any_box().downcast::<HostCnxFailedAccountFailedEvent>()?),
            StructType::HostCnxFailedAlreadyManagedEvent => Ok(from.as_any_box().downcast::<HostCnxFailedAlreadyManagedEvent>()?),
            StructType::HostCnxFailedBadCcagentEvent => Ok(from.as_any_box().downcast::<HostCnxFailedBadCcagentEvent>()?),
            StructType::HostCnxFailedBadUsernameEvent => Ok(from.as_any_box().downcast::<HostCnxFailedBadUsernameEvent>()?),
            StructType::HostCnxFailedBadVersionEvent => Ok(from.as_any_box().downcast::<HostCnxFailedBadVersionEvent>()?),
            StructType::HostCnxFailedCcagentUpgradeEvent => Ok(from.as_any_box().downcast::<HostCnxFailedCcagentUpgradeEvent>()?),
            StructType::HostCnxFailedEvent => Ok(from.as_any_box().downcast::<HostCnxFailedEvent>()?),
            StructType::HostCnxFailedNetworkErrorEvent => Ok(from.as_any_box().downcast::<HostCnxFailedNetworkErrorEvent>()?),
            StructType::HostCnxFailedNoAccessEvent => Ok(from.as_any_box().downcast::<HostCnxFailedNoAccessEvent>()?),
            StructType::HostCnxFailedNoConnectionEvent => Ok(from.as_any_box().downcast::<HostCnxFailedNoConnectionEvent>()?),
            StructType::HostCnxFailedNoLicenseEvent => Ok(from.as_any_box().downcast::<HostCnxFailedNoLicenseEvent>()?),
            StructType::HostCnxFailedNotFoundEvent => Ok(from.as_any_box().downcast::<HostCnxFailedNotFoundEvent>()?),
            StructType::HostCnxFailedTimeoutEvent => Ok(from.as_any_box().downcast::<HostCnxFailedTimeoutEvent>()?),
            StructType::HostComplianceCheckedEvent => Ok(from.as_any_box().downcast::<HostComplianceCheckedEvent>()?),
            StructType::HostCompliantEvent => Ok(from.as_any_box().downcast::<HostCompliantEvent>()?),
            StructType::HostConfigAppliedEvent => Ok(from.as_any_box().downcast::<HostConfigAppliedEvent>()?),
            StructType::HostConnectedEvent => Ok(from.as_any_box().downcast::<HostConnectedEvent>()?),
            StructType::HostConnectionLostEvent => Ok(from.as_any_box().downcast::<HostConnectionLostEvent>()?),
            StructType::HostDasDisabledEvent => Ok(from.as_any_box().downcast::<HostDasDisabledEvent>()?),
            StructType::HostDasDisablingEvent => Ok(from.as_any_box().downcast::<HostDasDisablingEvent>()?),
            StructType::HostDasEnabledEvent => Ok(from.as_any_box().downcast::<HostDasEnabledEvent>()?),
            StructType::HostDasEnablingEvent => Ok(from.as_any_box().downcast::<HostDasEnablingEvent>()?),
            StructType::HostDasErrorEvent => Ok(from.as_any_box().downcast::<HostDasErrorEvent>()?),
            StructType::HostDasEvent => Ok(from.as_any_box().downcast::<HostDasEvent>()?),
            StructType::HostExtraNetworksEvent => Ok(from.as_any_box().downcast::<HostExtraNetworksEvent>()?),
            StructType::HostIsolationIpPingFailedEvent => Ok(from.as_any_box().downcast::<HostIsolationIpPingFailedEvent>()?),
            StructType::HostMissingNetworksEvent => Ok(from.as_any_box().downcast::<HostMissingNetworksEvent>()?),
            StructType::HostNoAvailableNetworksEvent => Ok(from.as_any_box().downcast::<HostNoAvailableNetworksEvent>()?),
            StructType::HostNoHaEnabledPortGroupsEvent => Ok(from.as_any_box().downcast::<HostNoHaEnabledPortGroupsEvent>()?),
            StructType::HostNoRedundantManagementNetworkEvent => Ok(from.as_any_box().downcast::<HostNoRedundantManagementNetworkEvent>()?),
            StructType::HostNotInClusterEvent => Ok(from.as_any_box().downcast::<HostNotInClusterEvent>()?),
            StructType::HostPrimaryAgentNotShortNameEvent => Ok(from.as_any_box().downcast::<HostPrimaryAgentNotShortNameEvent>()?),
            StructType::HostShortNameInconsistentEvent => Ok(from.as_any_box().downcast::<HostShortNameInconsistentEvent>()?),
            StructType::HostDasOkEvent => Ok(from.as_any_box().downcast::<HostDasOkEvent>()?),
            StructType::HostDisconnectedEvent => Ok(from.as_any_box().downcast::<HostDisconnectedEvent>()?),
            StructType::HostEnableAdminFailedEvent => Ok(from.as_any_box().downcast::<HostEnableAdminFailedEvent>()?),
            StructType::HostGetShortNameFailedEvent => Ok(from.as_any_box().downcast::<HostGetShortNameFailedEvent>()?),
            StructType::HostInAuditModeEvent => Ok(from.as_any_box().downcast::<HostInAuditModeEvent>()?),
            StructType::HostIpChangedEvent => Ok(from.as_any_box().downcast::<HostIpChangedEvent>()?),
            StructType::HostIpInconsistentEvent => Ok(from.as_any_box().downcast::<HostIpInconsistentEvent>()?),
            StructType::HostIpToShortNameFailedEvent => Ok(from.as_any_box().downcast::<HostIpToShortNameFailedEvent>()?),
            StructType::HostNonCompliantEvent => Ok(from.as_any_box().downcast::<HostNonCompliantEvent>()?),
            StructType::HostProfileAppliedEvent => Ok(from.as_any_box().downcast::<HostProfileAppliedEvent>()?),
            StructType::HostReconnectionFailedEvent => Ok(from.as_any_box().downcast::<HostReconnectionFailedEvent>()?),
            StructType::HostRemovedEvent => Ok(from.as_any_box().downcast::<HostRemovedEvent>()?),
            StructType::HostShortNameToIpFailedEvent => Ok(from.as_any_box().downcast::<HostShortNameToIpFailedEvent>()?),
            StructType::HostShutdownEvent => Ok(from.as_any_box().downcast::<HostShutdownEvent>()?),
            StructType::HostSpecificationChangedEvent => Ok(from.as_any_box().downcast::<HostSpecificationChangedEvent>()?),
            StructType::HostSpecificationRequireEvent => Ok(from.as_any_box().downcast::<HostSpecificationRequireEvent>()?),
            StructType::HostSpecificationUpdateEvent => Ok(from.as_any_box().downcast::<HostSpecificationUpdateEvent>()?),
            StructType::HostSubSpecificationDeleteEvent => Ok(from.as_any_box().downcast::<HostSubSpecificationDeleteEvent>()?),
            StructType::HostSubSpecificationUpdateEvent => Ok(from.as_any_box().downcast::<HostSubSpecificationUpdateEvent>()?),
            StructType::HostSyncFailedEvent => Ok(from.as_any_box().downcast::<HostSyncFailedEvent>()?),
            StructType::HostUpgradeFailedEvent => Ok(from.as_any_box().downcast::<HostUpgradeFailedEvent>()?),
            StructType::HostUserWorldSwapNotEnabledEvent => Ok(from.as_any_box().downcast::<HostUserWorldSwapNotEnabledEvent>()?),
            StructType::HostVnicConnectedToCustomizedDvPortEvent => Ok(from.as_any_box().downcast::<HostVnicConnectedToCustomizedDvPortEvent>()?),
            StructType::HostWwnChangedEvent => Ok(from.as_any_box().downcast::<HostWwnChangedEvent>()?),
            StructType::HostWwnConflictEvent => Ok(from.as_any_box().downcast::<HostWwnConflictEvent>()?),
            StructType::LocalDatastoreCreatedEvent => Ok(from.as_any_box().downcast::<LocalDatastoreCreatedEvent>()?),
            StructType::LocalTsmEnabledEvent => Ok(from.as_any_box().downcast::<LocalTsmEnabledEvent>()?),
            StructType::NasDatastoreCreatedEvent => Ok(from.as_any_box().downcast::<NasDatastoreCreatedEvent>()?),
            StructType::NoDatastoresConfiguredEvent => Ok(from.as_any_box().downcast::<NoDatastoresConfiguredEvent>()?),
            StructType::RemoteTsmEnabledEvent => Ok(from.as_any_box().downcast::<RemoteTsmEnabledEvent>()?),
            StructType::TimedOutHostOperationEvent => Ok(from.as_any_box().downcast::<TimedOutHostOperationEvent>()?),
            StructType::UpdatedAgentBeingRestartedEvent => Ok(from.as_any_box().downcast::<UpdatedAgentBeingRestartedEvent>()?),
            StructType::UserAssignedToGroup => Ok(from.as_any_box().downcast::<UserAssignedToGroup>()?),
            StructType::UserPasswordChanged => Ok(from.as_any_box().downcast::<UserPasswordChanged>()?),
            StructType::UserUnassignedFromGroup => Ok(from.as_any_box().downcast::<UserUnassignedFromGroup>()?),
            StructType::VmfsDatastoreCreatedEvent => Ok(from.as_any_box().downcast::<VmfsDatastoreCreatedEvent>()?),
            StructType::VmfsDatastoreExpandedEvent => Ok(from.as_any_box().downcast::<VmfsDatastoreExpandedEvent>()?),
            StructType::VmfsDatastoreExtendedEvent => Ok(from.as_any_box().downcast::<VmfsDatastoreExtendedEvent>()?),
            StructType::VcAgentUninstallFailedEvent => Ok(from.as_any_box().downcast::<VcAgentUninstallFailedEvent>()?),
            StructType::VcAgentUninstalledEvent => Ok(from.as_any_box().downcast::<VcAgentUninstalledEvent>()?),
            StructType::VcAgentUpgradeFailedEvent => Ok(from.as_any_box().downcast::<VcAgentUpgradeFailedEvent>()?),
            StructType::VcAgentUpgradedEvent => Ok(from.as_any_box().downcast::<VcAgentUpgradedEvent>()?),
            StructType::VimAccountPasswordChangedEvent => Ok(from.as_any_box().downcast::<VimAccountPasswordChangedEvent>()?),
            StructType::IScsiBootFailureEvent => Ok(from.as_any_box().downcast::<IScsiBootFailureEvent>()?),
            StructType::HostInventoryUnreadableEvent => Ok(from.as_any_box().downcast::<HostInventoryUnreadableEvent>()?),
            StructType::LicenseEvent => Ok(from.as_any_box().downcast::<LicenseEvent>()?),
            StructType::AllVirtualMachinesLicensedEvent => Ok(from.as_any_box().downcast::<AllVirtualMachinesLicensedEvent>()?),
            StructType::HostInventoryFullEvent => Ok(from.as_any_box().downcast::<HostInventoryFullEvent>()?),
            StructType::HostLicenseExpiredEvent => Ok(from.as_any_box().downcast::<HostLicenseExpiredEvent>()?),
            StructType::IncorrectHostInformationEvent => Ok(from.as_any_box().downcast::<IncorrectHostInformationEvent>()?),
            StructType::InvalidEditionEvent => Ok(from.as_any_box().downcast::<InvalidEditionEvent>()?),
            StructType::LicenseNonComplianceEvent => Ok(from.as_any_box().downcast::<LicenseNonComplianceEvent>()?),
            StructType::LicenseRestrictedEvent => Ok(from.as_any_box().downcast::<LicenseRestrictedEvent>()?),
            StructType::LicenseServerAvailableEvent => Ok(from.as_any_box().downcast::<LicenseServerAvailableEvent>()?),
            StructType::LicenseServerUnavailableEvent => Ok(from.as_any_box().downcast::<LicenseServerUnavailableEvent>()?),
            StructType::NoLicenseEvent => Ok(from.as_any_box().downcast::<NoLicenseEvent>()?),
            StructType::ServerLicenseExpiredEvent => Ok(from.as_any_box().downcast::<ServerLicenseExpiredEvent>()?),
            StructType::UnlicensedVirtualMachinesEvent => Ok(from.as_any_box().downcast::<UnlicensedVirtualMachinesEvent>()?),
            StructType::UnlicensedVirtualMachinesFoundEvent => Ok(from.as_any_box().downcast::<UnlicensedVirtualMachinesFoundEvent>()?),
            StructType::VMotionLicenseExpiredEvent => Ok(from.as_any_box().downcast::<VMotionLicenseExpiredEvent>()?),
            StructType::LicenseExpiredEvent => Ok(from.as_any_box().downcast::<LicenseExpiredEvent>()?),
            StructType::LockerMisconfiguredEvent => Ok(from.as_any_box().downcast::<LockerMisconfiguredEvent>()?),
            StructType::LockerReconfiguredEvent => Ok(from.as_any_box().downcast::<LockerReconfiguredEvent>()?),
            StructType::NetworkRollbackEvent => Ok(from.as_any_box().downcast::<NetworkRollbackEvent>()?),
            StructType::ProfileEvent => Ok(from.as_any_box().downcast::<ProfileEvent>()?),
            StructType::ProfileAssociatedEvent => Ok(from.as_any_box().downcast::<ProfileAssociatedEvent>()?),
            StructType::ProfileChangedEvent => Ok(from.as_any_box().downcast::<ProfileChangedEvent>()?),
            StructType::ProfileCreatedEvent => Ok(from.as_any_box().downcast::<ProfileCreatedEvent>()?),
            StructType::ProfileDissociatedEvent => Ok(from.as_any_box().downcast::<ProfileDissociatedEvent>()?),
            StructType::ProfileReferenceHostChangedEvent => Ok(from.as_any_box().downcast::<ProfileReferenceHostChangedEvent>()?),
            StructType::ProfileRemovedEvent => Ok(from.as_any_box().downcast::<ProfileRemovedEvent>()?),
            StructType::ResourcePoolEvent => Ok(from.as_any_box().downcast::<ResourcePoolEvent>()?),
            StructType::ResourcePoolCreatedEvent => Ok(from.as_any_box().downcast::<ResourcePoolCreatedEvent>()?),
            StructType::ResourcePoolDestroyedEvent => Ok(from.as_any_box().downcast::<ResourcePoolDestroyedEvent>()?),
            StructType::ResourcePoolMovedEvent => Ok(from.as_any_box().downcast::<ResourcePoolMovedEvent>()?),
            StructType::ResourcePoolReconfiguredEvent => Ok(from.as_any_box().downcast::<ResourcePoolReconfiguredEvent>()?),
            StructType::ResourceViolatedEvent => Ok(from.as_any_box().downcast::<ResourceViolatedEvent>()?),
            StructType::ScheduledTaskEvent => Ok(from.as_any_box().downcast::<ScheduledTaskEvent>()?),
            StructType::ScheduledTaskCompletedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskCompletedEvent>()?),
            StructType::ScheduledTaskCreatedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskCreatedEvent>()?),
            StructType::ScheduledTaskEmailCompletedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskEmailCompletedEvent>()?),
            StructType::ScheduledTaskEmailFailedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskEmailFailedEvent>()?),
            StructType::ScheduledTaskFailedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskFailedEvent>()?),
            StructType::ScheduledTaskReconfiguredEvent => Ok(from.as_any_box().downcast::<ScheduledTaskReconfiguredEvent>()?),
            StructType::ScheduledTaskRemovedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskRemovedEvent>()?),
            StructType::ScheduledTaskStartedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskStartedEvent>()?),
            StructType::SessionEvent => Ok(from.as_any_box().downcast::<SessionEvent>()?),
            StructType::AlreadyAuthenticatedSessionEvent => Ok(from.as_any_box().downcast::<AlreadyAuthenticatedSessionEvent>()?),
            StructType::BadUsernameSessionEvent => Ok(from.as_any_box().downcast::<BadUsernameSessionEvent>()?),
            StructType::GlobalMessageChangedEvent => Ok(from.as_any_box().downcast::<GlobalMessageChangedEvent>()?),
            StructType::NoAccessUserEvent => Ok(from.as_any_box().downcast::<NoAccessUserEvent>()?),
            StructType::ServerStartedSessionEvent => Ok(from.as_any_box().downcast::<ServerStartedSessionEvent>()?),
            StructType::SessionTerminatedEvent => Ok(from.as_any_box().downcast::<SessionTerminatedEvent>()?),
            StructType::UserLoginSessionEvent => Ok(from.as_any_box().downcast::<UserLoginSessionEvent>()?),
            StructType::UserLogoutSessionEvent => Ok(from.as_any_box().downcast::<UserLogoutSessionEvent>()?),
            StructType::TaskEvent => Ok(from.as_any_box().downcast::<TaskEvent>()?),
            StructType::TaskTimeoutEvent => Ok(from.as_any_box().downcast::<TaskTimeoutEvent>()?),
            StructType::TemplateUpgradeEvent => Ok(from.as_any_box().downcast::<TemplateUpgradeEvent>()?),
            StructType::TemplateBeingUpgradedEvent => Ok(from.as_any_box().downcast::<TemplateBeingUpgradedEvent>()?),
            StructType::TemplateUpgradeFailedEvent => Ok(from.as_any_box().downcast::<TemplateUpgradeFailedEvent>()?),
            StructType::TemplateUpgradedEvent => Ok(from.as_any_box().downcast::<TemplateUpgradedEvent>()?),
            StructType::UpgradeEvent => Ok(from.as_any_box().downcast::<UpgradeEvent>()?),
            StructType::ErrorUpgradeEvent => Ok(from.as_any_box().downcast::<ErrorUpgradeEvent>()?),
            StructType::InfoUpgradeEvent => Ok(from.as_any_box().downcast::<InfoUpgradeEvent>()?),
            StructType::UserUpgradeEvent => Ok(from.as_any_box().downcast::<UserUpgradeEvent>()?),
            StructType::WarningUpgradeEvent => Ok(from.as_any_box().downcast::<WarningUpgradeEvent>()?),
            StructType::VmEvent => Ok(from.as_any_box().downcast::<VmEvent>()?),
            StructType::CustomizationEvent => Ok(from.as_any_box().downcast::<CustomizationEvent>()?),
            StructType::CustomizationFailed => Ok(from.as_any_box().downcast::<CustomizationFailed>()?),
            StructType::CustomizationLinuxIdentityFailed => Ok(from.as_any_box().downcast::<CustomizationLinuxIdentityFailed>()?),
            StructType::CustomizationNetworkSetupFailed => Ok(from.as_any_box().downcast::<CustomizationNetworkSetupFailed>()?),
            StructType::CustomizationSysprepFailed => Ok(from.as_any_box().downcast::<CustomizationSysprepFailed>()?),
            StructType::CustomizationUnknownFailure => Ok(from.as_any_box().downcast::<CustomizationUnknownFailure>()?),
            StructType::CustomizationStartedEvent => Ok(from.as_any_box().downcast::<CustomizationStartedEvent>()?),
            StructType::CustomizationSucceeded => Ok(from.as_any_box().downcast::<CustomizationSucceeded>()?),
            StructType::DrsRuleComplianceEvent => Ok(from.as_any_box().downcast::<DrsRuleComplianceEvent>()?),
            StructType::DrsRuleViolationEvent => Ok(from.as_any_box().downcast::<DrsRuleViolationEvent>()?),
            StructType::DrsSoftRuleViolationEvent => Ok(from.as_any_box().downcast::<DrsSoftRuleViolationEvent>()?),
            StructType::MigrationEvent => Ok(from.as_any_box().downcast::<MigrationEvent>()?),
            StructType::MigrationErrorEvent => Ok(from.as_any_box().downcast::<MigrationErrorEvent>()?),
            StructType::MigrationHostErrorEvent => Ok(from.as_any_box().downcast::<MigrationHostErrorEvent>()?),
            StructType::MigrationHostWarningEvent => Ok(from.as_any_box().downcast::<MigrationHostWarningEvent>()?),
            StructType::MigrationResourceErrorEvent => Ok(from.as_any_box().downcast::<MigrationResourceErrorEvent>()?),
            StructType::MigrationResourceWarningEvent => Ok(from.as_any_box().downcast::<MigrationResourceWarningEvent>()?),
            StructType::MigrationWarningEvent => Ok(from.as_any_box().downcast::<MigrationWarningEvent>()?),
            StructType::NoMaintenanceModeDrsRecommendationForVm => Ok(from.as_any_box().downcast::<NoMaintenanceModeDrsRecommendationForVm>()?),
            StructType::NotEnoughResourcesToStartVmEvent => Ok(from.as_any_box().downcast::<NotEnoughResourcesToStartVmEvent>()?),
            StructType::VmAcquiredMksTicketEvent => Ok(from.as_any_box().downcast::<VmAcquiredMksTicketEvent>()?),
            StructType::VmAcquiredTicketEvent => Ok(from.as_any_box().downcast::<VmAcquiredTicketEvent>()?),
            StructType::VmAutoRenameEvent => Ok(from.as_any_box().downcast::<VmAutoRenameEvent>()?),
            StructType::VmBeingCreatedEvent => Ok(from.as_any_box().downcast::<VmBeingCreatedEvent>()?),
            StructType::VmBeingDeployedEvent => Ok(from.as_any_box().downcast::<VmBeingDeployedEvent>()?),
            StructType::VmBeingHotMigratedEvent => Ok(from.as_any_box().downcast::<VmBeingHotMigratedEvent>()?),
            StructType::VmBeingMigratedEvent => Ok(from.as_any_box().downcast::<VmBeingMigratedEvent>()?),
            StructType::VmCloneEvent => Ok(from.as_any_box().downcast::<VmCloneEvent>()?),
            StructType::VmBeingClonedEvent => Ok(from.as_any_box().downcast::<VmBeingClonedEvent>()?),
            StructType::VmBeingClonedNoFolderEvent => Ok(from.as_any_box().downcast::<VmBeingClonedNoFolderEvent>()?),
            StructType::VmCloneFailedEvent => Ok(from.as_any_box().downcast::<VmCloneFailedEvent>()?),
            StructType::VmClonedEvent => Ok(from.as_any_box().downcast::<VmClonedEvent>()?),
            StructType::VmConfigMissingEvent => Ok(from.as_any_box().downcast::<VmConfigMissingEvent>()?),
            StructType::VmConnectedEvent => Ok(from.as_any_box().downcast::<VmConnectedEvent>()?),
            StructType::VmCreatedEvent => Ok(from.as_any_box().downcast::<VmCreatedEvent>()?),
            StructType::VmDasBeingResetEvent => Ok(from.as_any_box().downcast::<VmDasBeingResetEvent>()?),
            StructType::VmDasBeingResetWithScreenshotEvent => Ok(from.as_any_box().downcast::<VmDasBeingResetWithScreenshotEvent>()?),
            StructType::VmDasResetFailedEvent => Ok(from.as_any_box().downcast::<VmDasResetFailedEvent>()?),
            StructType::VmDasUpdateErrorEvent => Ok(from.as_any_box().downcast::<VmDasUpdateErrorEvent>()?),
            StructType::VmDasUpdateOkEvent => Ok(from.as_any_box().downcast::<VmDasUpdateOkEvent>()?),
            StructType::VmDateRolledBackEvent => Ok(from.as_any_box().downcast::<VmDateRolledBackEvent>()?),
            StructType::VmDeployFailedEvent => Ok(from.as_any_box().downcast::<VmDeployFailedEvent>()?),
            StructType::VmDeployedEvent => Ok(from.as_any_box().downcast::<VmDeployedEvent>()?),
            StructType::VmDisconnectedEvent => Ok(from.as_any_box().downcast::<VmDisconnectedEvent>()?),
            StructType::VmDiscoveredEvent => Ok(from.as_any_box().downcast::<VmDiscoveredEvent>()?),
            StructType::VmDiskFailedEvent => Ok(from.as_any_box().downcast::<VmDiskFailedEvent>()?),
            StructType::VmEmigratingEvent => Ok(from.as_any_box().downcast::<VmEmigratingEvent>()?),
            StructType::VmEndRecordingEvent => Ok(from.as_any_box().downcast::<VmEndRecordingEvent>()?),
            StructType::VmEndReplayingEvent => Ok(from.as_any_box().downcast::<VmEndReplayingEvent>()?),
            StructType::VmFailedMigrateEvent => Ok(from.as_any_box().downcast::<VmFailedMigrateEvent>()?),
            StructType::VmFailedRelayoutEvent => Ok(from.as_any_box().downcast::<VmFailedRelayoutEvent>()?),
            StructType::VmFailedRelayoutOnVmfs2DatastoreEvent => Ok(from.as_any_box().downcast::<VmFailedRelayoutOnVmfs2DatastoreEvent>()?),
            StructType::VmFailedStartingSecondaryEvent => Ok(from.as_any_box().downcast::<VmFailedStartingSecondaryEvent>()?),
            StructType::VmFailedToPowerOffEvent => Ok(from.as_any_box().downcast::<VmFailedToPowerOffEvent>()?),
            StructType::VmFailedToPowerOnEvent => Ok(from.as_any_box().downcast::<VmFailedToPowerOnEvent>()?),
            StructType::VmFailedToRebootGuestEvent => Ok(from.as_any_box().downcast::<VmFailedToRebootGuestEvent>()?),
            StructType::VmFailedToResetEvent => Ok(from.as_any_box().downcast::<VmFailedToResetEvent>()?),
            StructType::VmFailedToShutdownGuestEvent => Ok(from.as_any_box().downcast::<VmFailedToShutdownGuestEvent>()?),
            StructType::VmFailedToStandbyGuestEvent => Ok(from.as_any_box().downcast::<VmFailedToStandbyGuestEvent>()?),
            StructType::VmFailedToSuspendEvent => Ok(from.as_any_box().downcast::<VmFailedToSuspendEvent>()?),
            StructType::VmFailedUpdatingSecondaryConfig => Ok(from.as_any_box().downcast::<VmFailedUpdatingSecondaryConfig>()?),
            StructType::VmFailoverFailed => Ok(from.as_any_box().downcast::<VmFailoverFailed>()?),
            StructType::VmFaultToleranceStateChangedEvent => Ok(from.as_any_box().downcast::<VmFaultToleranceStateChangedEvent>()?),
            StructType::VmFaultToleranceTurnedOffEvent => Ok(from.as_any_box().downcast::<VmFaultToleranceTurnedOffEvent>()?),
            StructType::VmFaultToleranceVmTerminatedEvent => Ok(from.as_any_box().downcast::<VmFaultToleranceVmTerminatedEvent>()?),
            StructType::VmGuestOsCrashedEvent => Ok(from.as_any_box().downcast::<VmGuestOsCrashedEvent>()?),
            StructType::VmGuestRebootEvent => Ok(from.as_any_box().downcast::<VmGuestRebootEvent>()?),
            StructType::VmGuestShutdownEvent => Ok(from.as_any_box().downcast::<VmGuestShutdownEvent>()?),
            StructType::VmGuestStandbyEvent => Ok(from.as_any_box().downcast::<VmGuestStandbyEvent>()?),
            StructType::VmInstanceUuidAssignedEvent => Ok(from.as_any_box().downcast::<VmInstanceUuidAssignedEvent>()?),
            StructType::VmInstanceUuidChangedEvent => Ok(from.as_any_box().downcast::<VmInstanceUuidChangedEvent>()?),
            StructType::VmInstanceUuidConflictEvent => Ok(from.as_any_box().downcast::<VmInstanceUuidConflictEvent>()?),
            StructType::VmMacAssignedEvent => Ok(from.as_any_box().downcast::<VmMacAssignedEvent>()?),
            StructType::VmMacChangedEvent => Ok(from.as_any_box().downcast::<VmMacChangedEvent>()?),
            StructType::VmMacConflictEvent => Ok(from.as_any_box().downcast::<VmMacConflictEvent>()?),
            StructType::VmMaxFtRestartCountReached => Ok(from.as_any_box().downcast::<VmMaxFtRestartCountReached>()?),
            StructType::VmMaxRestartCountReached => Ok(from.as_any_box().downcast::<VmMaxRestartCountReached>()?),
            StructType::VmMessageErrorEvent => Ok(from.as_any_box().downcast::<VmMessageErrorEvent>()?),
            StructType::VmMessageEvent => Ok(from.as_any_box().downcast::<VmMessageEvent>()?),
            StructType::VmMessageWarningEvent => Ok(from.as_any_box().downcast::<VmMessageWarningEvent>()?),
            StructType::VmMigratedEvent => Ok(from.as_any_box().downcast::<VmMigratedEvent>()?),
            StructType::DrsVmMigratedEvent => Ok(from.as_any_box().downcast::<DrsVmMigratedEvent>()?),
            StructType::VmNoCompatibleHostForSecondaryEvent => Ok(from.as_any_box().downcast::<VmNoCompatibleHostForSecondaryEvent>()?),
            StructType::VmNoNetworkAccessEvent => Ok(from.as_any_box().downcast::<VmNoNetworkAccessEvent>()?),
            StructType::VmOrphanedEvent => Ok(from.as_any_box().downcast::<VmOrphanedEvent>()?),
            StructType::VmPoweredOffEvent => Ok(from.as_any_box().downcast::<VmPoweredOffEvent>()?),
            StructType::VmPowerOffOnIsolationEvent => Ok(from.as_any_box().downcast::<VmPowerOffOnIsolationEvent>()?),
            StructType::VmShutdownOnIsolationEvent => Ok(from.as_any_box().downcast::<VmShutdownOnIsolationEvent>()?),
            StructType::VmPoweredOnEvent => Ok(from.as_any_box().downcast::<VmPoweredOnEvent>()?),
            StructType::DrsVmPoweredOnEvent => Ok(from.as_any_box().downcast::<DrsVmPoweredOnEvent>()?),
            StructType::VmRestartedOnAlternateHostEvent => Ok(from.as_any_box().downcast::<VmRestartedOnAlternateHostEvent>()?),
            StructType::VmPoweringOnWithCustomizedDvPortEvent => Ok(from.as_any_box().downcast::<VmPoweringOnWithCustomizedDvPortEvent>()?),
            StructType::VmPrimaryFailoverEvent => Ok(from.as_any_box().downcast::<VmPrimaryFailoverEvent>()?),
            StructType::VmReconfiguredEvent => Ok(from.as_any_box().downcast::<VmReconfiguredEvent>()?),
            StructType::VmRegisteredEvent => Ok(from.as_any_box().downcast::<VmRegisteredEvent>()?),
            StructType::VmRelayoutSuccessfulEvent => Ok(from.as_any_box().downcast::<VmRelayoutSuccessfulEvent>()?),
            StructType::VmRelayoutUpToDateEvent => Ok(from.as_any_box().downcast::<VmRelayoutUpToDateEvent>()?),
            StructType::VmReloadFromPathEvent => Ok(from.as_any_box().downcast::<VmReloadFromPathEvent>()?),
            StructType::VmReloadFromPathFailedEvent => Ok(from.as_any_box().downcast::<VmReloadFromPathFailedEvent>()?),
            StructType::VmRelocateSpecEvent => Ok(from.as_any_box().downcast::<VmRelocateSpecEvent>()?),
            StructType::VmBeingRelocatedEvent => Ok(from.as_any_box().downcast::<VmBeingRelocatedEvent>()?),
            StructType::VmRelocateFailedEvent => Ok(from.as_any_box().downcast::<VmRelocateFailedEvent>()?),
            StructType::VmRelocatedEvent => Ok(from.as_any_box().downcast::<VmRelocatedEvent>()?),
            StructType::VmRemoteConsoleConnectedEvent => Ok(from.as_any_box().downcast::<VmRemoteConsoleConnectedEvent>()?),
            StructType::VmRemoteConsoleDisconnectedEvent => Ok(from.as_any_box().downcast::<VmRemoteConsoleDisconnectedEvent>()?),
            StructType::VmRemovedEvent => Ok(from.as_any_box().downcast::<VmRemovedEvent>()?),
            StructType::VmRenamedEvent => Ok(from.as_any_box().downcast::<VmRenamedEvent>()?),
            StructType::VmRequirementsExceedCurrentEvcModeEvent => Ok(from.as_any_box().downcast::<VmRequirementsExceedCurrentEvcModeEvent>()?),
            StructType::VmResettingEvent => Ok(from.as_any_box().downcast::<VmResettingEvent>()?),
            StructType::VmResourcePoolMovedEvent => Ok(from.as_any_box().downcast::<VmResourcePoolMovedEvent>()?),
            StructType::VmResourceReallocatedEvent => Ok(from.as_any_box().downcast::<VmResourceReallocatedEvent>()?),
            StructType::VmResumingEvent => Ok(from.as_any_box().downcast::<VmResumingEvent>()?),
            StructType::VmSecondaryAddedEvent => Ok(from.as_any_box().downcast::<VmSecondaryAddedEvent>()?),
            StructType::VmSecondaryDisabledBySystemEvent => Ok(from.as_any_box().downcast::<VmSecondaryDisabledBySystemEvent>()?),
            StructType::VmSecondaryDisabledEvent => Ok(from.as_any_box().downcast::<VmSecondaryDisabledEvent>()?),
            StructType::VmSecondaryEnabledEvent => Ok(from.as_any_box().downcast::<VmSecondaryEnabledEvent>()?),
            StructType::VmSecondaryStartedEvent => Ok(from.as_any_box().downcast::<VmSecondaryStartedEvent>()?),
            StructType::VmStartRecordingEvent => Ok(from.as_any_box().downcast::<VmStartRecordingEvent>()?),
            StructType::VmStartReplayingEvent => Ok(from.as_any_box().downcast::<VmStartReplayingEvent>()?),
            StructType::VmStartingEvent => Ok(from.as_any_box().downcast::<VmStartingEvent>()?),
            StructType::VmUnsupportedStartingEvent => Ok(from.as_any_box().downcast::<VmUnsupportedStartingEvent>()?),
            StructType::VmStartingSecondaryEvent => Ok(from.as_any_box().downcast::<VmStartingSecondaryEvent>()?),
            StructType::VmStaticMacConflictEvent => Ok(from.as_any_box().downcast::<VmStaticMacConflictEvent>()?),
            StructType::VmStoppingEvent => Ok(from.as_any_box().downcast::<VmStoppingEvent>()?),
            StructType::VmSuspendedEvent => Ok(from.as_any_box().downcast::<VmSuspendedEvent>()?),
            StructType::VmSuspendingEvent => Ok(from.as_any_box().downcast::<VmSuspendingEvent>()?),
            StructType::VmTimedoutStartingSecondaryEvent => Ok(from.as_any_box().downcast::<VmTimedoutStartingSecondaryEvent>()?),
            StructType::VmUpgradeCompleteEvent => Ok(from.as_any_box().downcast::<VmUpgradeCompleteEvent>()?),
            StructType::VmUpgradeFailedEvent => Ok(from.as_any_box().downcast::<VmUpgradeFailedEvent>()?),
            StructType::VmUpgradingEvent => Ok(from.as_any_box().downcast::<VmUpgradingEvent>()?),
            StructType::VmUuidAssignedEvent => Ok(from.as_any_box().downcast::<VmUuidAssignedEvent>()?),
            StructType::VmUuidChangedEvent => Ok(from.as_any_box().downcast::<VmUuidChangedEvent>()?),
            StructType::VmUuidConflictEvent => Ok(from.as_any_box().downcast::<VmUuidConflictEvent>()?),
            StructType::VmWwnAssignedEvent => Ok(from.as_any_box().downcast::<VmWwnAssignedEvent>()?),
            StructType::VmWwnChangedEvent => Ok(from.as_any_box().downcast::<VmWwnChangedEvent>()?),
            StructType::VmWwnConflictEvent => Ok(from.as_any_box().downcast::<VmWwnConflictEvent>()?),
            StructType::EventArgument => Ok(from.as_any_box().downcast::<EventArgument>()?),
            StructType::EntityEventArgument => Ok(from.as_any_box().downcast::<EntityEventArgument>()?),
            StructType::AlarmEventArgument => Ok(from.as_any_box().downcast::<AlarmEventArgument>()?),
            StructType::ComputeResourceEventArgument => Ok(from.as_any_box().downcast::<ComputeResourceEventArgument>()?),
            StructType::DatacenterEventArgument => Ok(from.as_any_box().downcast::<DatacenterEventArgument>()?),
            StructType::DatastoreEventArgument => Ok(from.as_any_box().downcast::<DatastoreEventArgument>()?),
            StructType::DvsEventArgument => Ok(from.as_any_box().downcast::<DvsEventArgument>()?),
            StructType::FolderEventArgument => Ok(from.as_any_box().downcast::<FolderEventArgument>()?),
            StructType::HostEventArgument => Ok(from.as_any_box().downcast::<HostEventArgument>()?),
            StructType::ManagedEntityEventArgument => Ok(from.as_any_box().downcast::<ManagedEntityEventArgument>()?),
            StructType::NetworkEventArgument => Ok(from.as_any_box().downcast::<NetworkEventArgument>()?),
            StructType::ResourcePoolEventArgument => Ok(from.as_any_box().downcast::<ResourcePoolEventArgument>()?),
            StructType::ScheduledTaskEventArgument => Ok(from.as_any_box().downcast::<ScheduledTaskEventArgument>()?),
            StructType::VmEventArgument => Ok(from.as_any_box().downcast::<VmEventArgument>()?),
            StructType::ProfileEventArgument => Ok(from.as_any_box().downcast::<ProfileEventArgument>()?),
            StructType::RoleEventArgument => Ok(from.as_any_box().downcast::<RoleEventArgument>()?),
            StructType::EventDescription => Ok(from.as_any_box().downcast::<EventDescription>()?),
            StructType::EventArgDesc => Ok(from.as_any_box().downcast::<EventArgDesc>()?),
            StructType::EventDescriptionEventDetail => Ok(from.as_any_box().downcast::<EventDescriptionEventDetail>()?),
            StructType::EventFilterSpec => Ok(from.as_any_box().downcast::<EventFilterSpec>()?),
            StructType::EventFilterSpecByEntity => Ok(from.as_any_box().downcast::<EventFilterSpecByEntity>()?),
            StructType::EventFilterSpecByTime => Ok(from.as_any_box().downcast::<EventFilterSpecByTime>()?),
            StructType::EventFilterSpecByUsername => Ok(from.as_any_box().downcast::<EventFilterSpecByUsername>()?),
            StructType::ExtendedEventPair => Ok(from.as_any_box().downcast::<ExtendedEventPair>()?),
            StructType::VnicPortArgument => Ok(from.as_any_box().downcast::<VnicPortArgument>()?),
            StructType::ExtExtendedProductInfo => Ok(from.as_any_box().downcast::<ExtExtendedProductInfo>()?),
            StructType::ManagedByInfo => Ok(from.as_any_box().downcast::<ManagedByInfo>()?),
            StructType::ExtManagedEntityInfo => Ok(from.as_any_box().downcast::<ExtManagedEntityInfo>()?),
            StructType::ExtSolutionManagerInfo => Ok(from.as_any_box().downcast::<ExtSolutionManagerInfo>()?),
            StructType::ExtSolutionManagerInfoTabInfo => Ok(from.as_any_box().downcast::<ExtSolutionManagerInfoTabInfo>()?),
            StructType::AnswerFileUpdateFailure => Ok(from.as_any_box().downcast::<AnswerFileUpdateFailure>()?),
            StructType::ConflictingConfigurationConfig => Ok(from.as_any_box().downcast::<ConflictingConfigurationConfig>()?),
            StructType::DatacenterMismatchArgument => Ok(from.as_any_box().downcast::<DatacenterMismatchArgument>()?),
            StructType::DvsApplyOperationFaultFaultOnObject => Ok(from.as_any_box().downcast::<DvsApplyOperationFaultFaultOnObject>()?),
            StructType::DvsOperationBulkFaultFaultOnHost => Ok(from.as_any_box().downcast::<DvsOperationBulkFaultFaultOnHost>()?),
            StructType::ImportOperationBulkFaultFaultOnImport => Ok(from.as_any_box().downcast::<ImportOperationBulkFaultFaultOnImport>()?),
            StructType::MultipleCertificatesVerifyFaultThumbprintData => Ok(from.as_any_box().downcast::<MultipleCertificatesVerifyFaultThumbprintData>()?),
            StructType::NoPermissionEntityPrivileges => Ok(from.as_any_box().downcast::<NoPermissionEntityPrivileges>()?),
            StructType::ProfileUpdateFailedUpdateFailure => Ok(from.as_any_box().downcast::<ProfileUpdateFailedUpdateFailure>()?),
            StructType::HostActiveDirectory => Ok(from.as_any_box().downcast::<HostActiveDirectory>()?),
            StructType::HostActiveDirectorySpec => Ok(from.as_any_box().downcast::<HostActiveDirectorySpec>()?),
            StructType::HostAssignableHardwareBinding => Ok(from.as_any_box().downcast::<HostAssignableHardwareBinding>()?),
            StructType::HostAssignableHardwareConfig => Ok(from.as_any_box().downcast::<HostAssignableHardwareConfig>()?),
            StructType::HostAssignableHardwareConfigAttributeOverride => Ok(from.as_any_box().downcast::<HostAssignableHardwareConfigAttributeOverride>()?),
            StructType::HostAuthenticationManagerInfo => Ok(from.as_any_box().downcast::<HostAuthenticationManagerInfo>()?),
            StructType::HostAuthenticationStoreInfo => Ok(from.as_any_box().downcast::<HostAuthenticationStoreInfo>()?),
            StructType::HostDirectoryStoreInfo => Ok(from.as_any_box().downcast::<HostDirectoryStoreInfo>()?),
            StructType::HostActiveDirectoryInfo => Ok(from.as_any_box().downcast::<HostActiveDirectoryInfo>()?),
            StructType::HostLocalAuthenticationInfo => Ok(from.as_any_box().downcast::<HostLocalAuthenticationInfo>()?),
            StructType::AutoStartPowerInfo => Ok(from.as_any_box().downcast::<AutoStartPowerInfo>()?),
            StructType::HostAutoStartManagerConfig => Ok(from.as_any_box().downcast::<HostAutoStartManagerConfig>()?),
            StructType::AutoStartDefaults => Ok(from.as_any_box().downcast::<AutoStartDefaults>()?),
            StructType::HostBiosInfo => Ok(from.as_any_box().downcast::<HostBiosInfo>()?),
            StructType::HostBootDeviceInfo => Ok(from.as_any_box().downcast::<HostBootDeviceInfo>()?),
            StructType::HostBootDevice => Ok(from.as_any_box().downcast::<HostBootDevice>()?),
            StructType::HostCacheConfigurationInfo => Ok(from.as_any_box().downcast::<HostCacheConfigurationInfo>()?),
            StructType::HostCacheConfigurationSpec => Ok(from.as_any_box().downcast::<HostCacheConfigurationSpec>()?),
            StructType::HostCapability => Ok(from.as_any_box().downcast::<HostCapability>()?),
            StructType::HostCertificateManagerCertificateInfo => Ok(from.as_any_box().downcast::<HostCertificateManagerCertificateInfo>()?),
            StructType::HostCertificateManagerCertificateSpec => Ok(from.as_any_box().downcast::<HostCertificateManagerCertificateSpec>()?),
            StructType::HostConfigChange => Ok(from.as_any_box().downcast::<HostConfigChange>()?),
            StructType::HostConfigInfo => Ok(from.as_any_box().downcast::<HostConfigInfo>()?),
            StructType::HostConfigManager => Ok(from.as_any_box().downcast::<HostConfigManager>()?),
            StructType::HostConfigSpec => Ok(from.as_any_box().downcast::<HostConfigSpec>()?),
            StructType::HostConnectInfo => Ok(from.as_any_box().downcast::<HostConnectInfo>()?),
            StructType::HostDatastoreConnectInfo => Ok(from.as_any_box().downcast::<HostDatastoreConnectInfo>()?),
            StructType::HostDatastoreExistsConnectInfo => Ok(from.as_any_box().downcast::<HostDatastoreExistsConnectInfo>()?),
            StructType::HostDatastoreNameConflictConnectInfo => Ok(from.as_any_box().downcast::<HostDatastoreNameConflictConnectInfo>()?),
            StructType::HostLicenseConnectInfo => Ok(from.as_any_box().downcast::<HostLicenseConnectInfo>()?),
            StructType::HostConnectInfoNetworkInfo => Ok(from.as_any_box().downcast::<HostConnectInfoNetworkInfo>()?),
            StructType::HostNewNetworkConnectInfo => Ok(from.as_any_box().downcast::<HostNewNetworkConnectInfo>()?),
            StructType::HostConnectSpec => Ok(from.as_any_box().downcast::<HostConnectSpec>()?),
            StructType::HostCpuIdInfo => Ok(from.as_any_box().downcast::<HostCpuIdInfo>()?),
            StructType::HostCpuInfo => Ok(from.as_any_box().downcast::<HostCpuInfo>()?),
            StructType::HostCpuPackage => Ok(from.as_any_box().downcast::<HostCpuPackage>()?),
            StructType::HostCpuPowerManagementInfo => Ok(from.as_any_box().downcast::<HostCpuPowerManagementInfo>()?),
            StructType::HostHyperThreadScheduleInfo => Ok(from.as_any_box().downcast::<HostHyperThreadScheduleInfo>()?),
            StructType::HostDataTransportConnectionInfo => Ok(from.as_any_box().downcast::<HostDataTransportConnectionInfo>()?),
            StructType::HostNfcConnectionInfo => Ok(from.as_any_box().downcast::<HostNfcConnectionInfo>()?),
            StructType::FileInfo => Ok(from.as_any_box().downcast::<FileInfo>()?),
            StructType::FloppyImageFileInfo => Ok(from.as_any_box().downcast::<FloppyImageFileInfo>()?),
            StructType::FolderFileInfo => Ok(from.as_any_box().downcast::<FolderFileInfo>()?),
            StructType::IsoImageFileInfo => Ok(from.as_any_box().downcast::<IsoImageFileInfo>()?),
            StructType::VmConfigFileInfo => Ok(from.as_any_box().downcast::<VmConfigFileInfo>()?),
            StructType::TemplateConfigFileInfo => Ok(from.as_any_box().downcast::<TemplateConfigFileInfo>()?),
            StructType::VmDiskFileInfo => Ok(from.as_any_box().downcast::<VmDiskFileInfo>()?),
            StructType::VmLogFileInfo => Ok(from.as_any_box().downcast::<VmLogFileInfo>()?),
            StructType::VmNvramFileInfo => Ok(from.as_any_box().downcast::<VmNvramFileInfo>()?),
            StructType::VmSnapshotFileInfo => Ok(from.as_any_box().downcast::<VmSnapshotFileInfo>()?),
            StructType::FileQueryFlags => Ok(from.as_any_box().downcast::<FileQueryFlags>()?),
            StructType::FileQuery => Ok(from.as_any_box().downcast::<FileQuery>()?),
            StructType::FloppyImageFileQuery => Ok(from.as_any_box().downcast::<FloppyImageFileQuery>()?),
            StructType::FolderFileQuery => Ok(from.as_any_box().downcast::<FolderFileQuery>()?),
            StructType::IsoImageFileQuery => Ok(from.as_any_box().downcast::<IsoImageFileQuery>()?),
            StructType::VmConfigFileQuery => Ok(from.as_any_box().downcast::<VmConfigFileQuery>()?),
            StructType::TemplateConfigFileQuery => Ok(from.as_any_box().downcast::<TemplateConfigFileQuery>()?),
            StructType::VmDiskFileQuery => Ok(from.as_any_box().downcast::<VmDiskFileQuery>()?),
            StructType::VmLogFileQuery => Ok(from.as_any_box().downcast::<VmLogFileQuery>()?),
            StructType::VmNvramFileQuery => Ok(from.as_any_box().downcast::<VmNvramFileQuery>()?),
            StructType::VmSnapshotFileQuery => Ok(from.as_any_box().downcast::<VmSnapshotFileQuery>()?),
            StructType::HostDatastoreBrowserSearchResults => Ok(from.as_any_box().downcast::<HostDatastoreBrowserSearchResults>()?),
            StructType::HostDatastoreBrowserSearchSpec => Ok(from.as_any_box().downcast::<HostDatastoreBrowserSearchSpec>()?),
            StructType::VmConfigFileEncryptionInfo => Ok(from.as_any_box().downcast::<VmConfigFileEncryptionInfo>()?),
            StructType::VmConfigFileQueryFlags => Ok(from.as_any_box().downcast::<VmConfigFileQueryFlags>()?),
            StructType::VmConfigFileQueryFilter => Ok(from.as_any_box().downcast::<VmConfigFileQueryFilter>()?),
            StructType::VmDiskFileEncryptionInfo => Ok(from.as_any_box().downcast::<VmDiskFileEncryptionInfo>()?),
            StructType::VmDiskFileQueryFlags => Ok(from.as_any_box().downcast::<VmDiskFileQueryFlags>()?),
            StructType::VmDiskFileQueryFilter => Ok(from.as_any_box().downcast::<VmDiskFileQueryFilter>()?),
            StructType::HostDatastoreSystemCapabilities => Ok(from.as_any_box().downcast::<HostDatastoreSystemCapabilities>()?),
            StructType::HostDatastoreSystemDatastoreResult => Ok(from.as_any_box().downcast::<HostDatastoreSystemDatastoreResult>()?),
            StructType::HostDatastoreSystemVvolDatastoreSpec => Ok(from.as_any_box().downcast::<HostDatastoreSystemVvolDatastoreSpec>()?),
            StructType::HostDateTimeConfig => Ok(from.as_any_box().downcast::<HostDateTimeConfig>()?),
            StructType::HostDateTimeInfo => Ok(from.as_any_box().downcast::<HostDateTimeInfo>()?),
            StructType::HostDateTimeSystemServiceTestResult => Ok(from.as_any_box().downcast::<HostDateTimeSystemServiceTestResult>()?),
            StructType::HostDateTimeSystemTimeZone => Ok(from.as_any_box().downcast::<HostDateTimeSystemTimeZone>()?),
            StructType::HostDeploymentInfo => Ok(from.as_any_box().downcast::<HostDeploymentInfo>()?),
            StructType::HostDevice => Ok(from.as_any_box().downcast::<HostDevice>()?),
            StructType::ScsiLun => Ok(from.as_any_box().downcast::<ScsiLun>()?),
            StructType::HostScsiDisk => Ok(from.as_any_box().downcast::<HostScsiDisk>()?),
            StructType::HostDhcpService => Ok(from.as_any_box().downcast::<HostDhcpService>()?),
            StructType::HostDhcpServiceConfig => Ok(from.as_any_box().downcast::<HostDhcpServiceConfig>()?),
            StructType::HostDhcpServiceSpec => Ok(from.as_any_box().downcast::<HostDhcpServiceSpec>()?),
            StructType::HostDiagnosticPartition => Ok(from.as_any_box().downcast::<HostDiagnosticPartition>()?),
            StructType::HostDiagnosticPartitionCreateDescription => Ok(from.as_any_box().downcast::<HostDiagnosticPartitionCreateDescription>()?),
            StructType::HostDiagnosticPartitionCreateOption => Ok(from.as_any_box().downcast::<HostDiagnosticPartitionCreateOption>()?),
            StructType::HostDiagnosticPartitionCreateSpec => Ok(from.as_any_box().downcast::<HostDiagnosticPartitionCreateSpec>()?),
            StructType::HostDigestInfo => Ok(from.as_any_box().downcast::<HostDigestInfo>()?),
            StructType::HostTpmDigestInfo => Ok(from.as_any_box().downcast::<HostTpmDigestInfo>()?),
            StructType::HostDiskConfigurationResult => Ok(from.as_any_box().downcast::<HostDiskConfigurationResult>()?),
            StructType::HostDiskDimensions => Ok(from.as_any_box().downcast::<HostDiskDimensions>()?),
            StructType::HostDiskDimensionsChs => Ok(from.as_any_box().downcast::<HostDiskDimensionsChs>()?),
            StructType::HostDiskDimensionsLba => Ok(from.as_any_box().downcast::<HostDiskDimensionsLba>()?),
            StructType::HostDiskPartitionInfo => Ok(from.as_any_box().downcast::<HostDiskPartitionInfo>()?),
            StructType::HostDiskPartitionBlockRange => Ok(from.as_any_box().downcast::<HostDiskPartitionBlockRange>()?),
            StructType::HostDiskPartitionLayout => Ok(from.as_any_box().downcast::<HostDiskPartitionLayout>()?),
            StructType::HostDiskPartitionAttributes => Ok(from.as_any_box().downcast::<HostDiskPartitionAttributes>()?),
            StructType::HostDiskPartitionSpec => Ok(from.as_any_box().downcast::<HostDiskPartitionSpec>()?),
            StructType::HostDnsConfig => Ok(from.as_any_box().downcast::<HostDnsConfig>()?),
            StructType::HostDnsConfigSpec => Ok(from.as_any_box().downcast::<HostDnsConfigSpec>()?),
            StructType::HostDvxClass => Ok(from.as_any_box().downcast::<HostDvxClass>()?),
            StructType::HostEnterMaintenanceResult => Ok(from.as_any_box().downcast::<HostEnterMaintenanceResult>()?),
            StructType::HostEsxAgentHostManagerConfigInfo => Ok(from.as_any_box().downcast::<HostEsxAgentHostManagerConfigInfo>()?),
            StructType::HostFaultToleranceManagerComponentHealthInfo => Ok(from.as_any_box().downcast::<HostFaultToleranceManagerComponentHealthInfo>()?),
            StructType::FcoeConfig => Ok(from.as_any_box().downcast::<FcoeConfig>()?),
            StructType::FcoeConfigFcoeCapabilities => Ok(from.as_any_box().downcast::<FcoeConfigFcoeCapabilities>()?),
            StructType::FcoeConfigFcoeSpecification => Ok(from.as_any_box().downcast::<FcoeConfigFcoeSpecification>()?),
            StructType::FcoeConfigVlanRange => Ok(from.as_any_box().downcast::<FcoeConfigVlanRange>()?),
            StructType::HostFeatureCapability => Ok(from.as_any_box().downcast::<HostFeatureCapability>()?),
            StructType::HostFeatureMask => Ok(from.as_any_box().downcast::<HostFeatureMask>()?),
            StructType::HostFeatureVersionInfo => Ok(from.as_any_box().downcast::<HostFeatureVersionInfo>()?),
            StructType::HostFibreChannelOverEthernetHbaLinkInfo => Ok(from.as_any_box().downcast::<HostFibreChannelOverEthernetHbaLinkInfo>()?),
            StructType::HostFileAccess => Ok(from.as_any_box().downcast::<HostFileAccess>()?),
            StructType::ModeInfo => Ok(from.as_any_box().downcast::<ModeInfo>()?),
            StructType::HostFileSystemMountInfo => Ok(from.as_any_box().downcast::<HostFileSystemMountInfo>()?),
            StructType::HostFileSystemVolume => Ok(from.as_any_box().downcast::<HostFileSystemVolume>()?),
            StructType::HostLocalFileSystemVolume => Ok(from.as_any_box().downcast::<HostLocalFileSystemVolume>()?),
            StructType::HostNasVolume => Ok(from.as_any_box().downcast::<HostNasVolume>()?),
            StructType::HostPMemVolume => Ok(from.as_any_box().downcast::<HostPMemVolume>()?),
            StructType::HostVfatVolume => Ok(from.as_any_box().downcast::<HostVfatVolume>()?),
            StructType::HostVffsVolume => Ok(from.as_any_box().downcast::<HostVffsVolume>()?),
            StructType::HostVmfsVolume => Ok(from.as_any_box().downcast::<HostVmfsVolume>()?),
            StructType::HostVvolVolume => Ok(from.as_any_box().downcast::<HostVvolVolume>()?),
            StructType::HostFileSystemVolumeInfo => Ok(from.as_any_box().downcast::<HostFileSystemVolumeInfo>()?),
            StructType::HostFirewallConfig => Ok(from.as_any_box().downcast::<HostFirewallConfig>()?),
            StructType::HostFirewallConfigRuleSetConfig => Ok(from.as_any_box().downcast::<HostFirewallConfigRuleSetConfig>()?),
            StructType::HostFirewallInfo => Ok(from.as_any_box().downcast::<HostFirewallInfo>()?),
            StructType::HostFirewallDefaultPolicy => Ok(from.as_any_box().downcast::<HostFirewallDefaultPolicy>()?),
            StructType::HostFlagInfo => Ok(from.as_any_box().downcast::<HostFlagInfo>()?),
            StructType::HostForceMountedInfo => Ok(from.as_any_box().downcast::<HostForceMountedInfo>()?),
            StructType::HostFru => Ok(from.as_any_box().downcast::<HostFru>()?),
            StructType::HostGatewaySpec => Ok(from.as_any_box().downcast::<HostGatewaySpec>()?),
            StructType::HostGraphicsConfig => Ok(from.as_any_box().downcast::<HostGraphicsConfig>()?),
            StructType::HostGraphicsConfigDeviceType => Ok(from.as_any_box().downcast::<HostGraphicsConfigDeviceType>()?),
            StructType::HostGraphicsInfo => Ok(from.as_any_box().downcast::<HostGraphicsInfo>()?),
            StructType::HostHardwareInfo => Ok(from.as_any_box().downcast::<HostHardwareInfo>()?),
            StructType::HostHardwareStatusInfo => Ok(from.as_any_box().downcast::<HostHardwareStatusInfo>()?),
            StructType::DpuStatusInfoOperationalInfo => Ok(from.as_any_box().downcast::<DpuStatusInfoOperationalInfo>()?),
            StructType::HostHardwareElementInfo => Ok(from.as_any_box().downcast::<HostHardwareElementInfo>()?),
            StructType::DpuStatusInfo => Ok(from.as_any_box().downcast::<DpuStatusInfo>()?),
            StructType::HostStorageElementInfo => Ok(from.as_any_box().downcast::<HostStorageElementInfo>()?),
            StructType::HostStorageOperationalInfo => Ok(from.as_any_box().downcast::<HostStorageOperationalInfo>()?),
            StructType::HostHbaCreateSpec => Ok(from.as_any_box().downcast::<HostHbaCreateSpec>()?),
            StructType::HostTcpHbaCreateSpec => Ok(from.as_any_box().downcast::<HostTcpHbaCreateSpec>()?),
            StructType::HealthSystemRuntime => Ok(from.as_any_box().downcast::<HealthSystemRuntime>()?),
            StructType::HostAccessControlEntry => Ok(from.as_any_box().downcast::<HostAccessControlEntry>()?),
            StructType::HostHostBusAdapter => Ok(from.as_any_box().downcast::<HostHostBusAdapter>()?),
            StructType::HostBlockHba => Ok(from.as_any_box().downcast::<HostBlockHba>()?),
            StructType::HostFibreChannelHba => Ok(from.as_any_box().downcast::<HostFibreChannelHba>()?),
            StructType::HostFibreChannelOverEthernetHba => Ok(from.as_any_box().downcast::<HostFibreChannelOverEthernetHba>()?),
            StructType::HostInternetScsiHba => Ok(from.as_any_box().downcast::<HostInternetScsiHba>()?),
            StructType::HostParallelScsiHba => Ok(from.as_any_box().downcast::<HostParallelScsiHba>()?),
            StructType::HostPcieHba => Ok(from.as_any_box().downcast::<HostPcieHba>()?),
            StructType::HostRdmaHba => Ok(from.as_any_box().downcast::<HostRdmaHba>()?),
            StructType::HostSerialAttachedHba => Ok(from.as_any_box().downcast::<HostSerialAttachedHba>()?),
            StructType::HostTcpHba => Ok(from.as_any_box().downcast::<HostTcpHba>()?),
            StructType::HostProxySwitch => Ok(from.as_any_box().downcast::<HostProxySwitch>()?),
            StructType::HostProxySwitchConfig => Ok(from.as_any_box().downcast::<HostProxySwitchConfig>()?),
            StructType::HostProxySwitchEnsInfo => Ok(from.as_any_box().downcast::<HostProxySwitchEnsInfo>()?),
            StructType::HostProxySwitchHostLagConfig => Ok(from.as_any_box().downcast::<HostProxySwitchHostLagConfig>()?),
            StructType::HostProxySwitchSpec => Ok(from.as_any_box().downcast::<HostProxySwitchSpec>()?),
            StructType::HostImageProfileSummary => Ok(from.as_any_box().downcast::<HostImageProfileSummary>()?),
            StructType::HostInternetScsiHbaAuthenticationCapabilities => Ok(from.as_any_box().downcast::<HostInternetScsiHbaAuthenticationCapabilities>()?),
            StructType::HostInternetScsiHbaAuthenticationProperties => Ok(from.as_any_box().downcast::<HostInternetScsiHbaAuthenticationProperties>()?),
            StructType::HostInternetScsiHbaDigestCapabilities => Ok(from.as_any_box().downcast::<HostInternetScsiHbaDigestCapabilities>()?),
            StructType::HostInternetScsiHbaDigestProperties => Ok(from.as_any_box().downcast::<HostInternetScsiHbaDigestProperties>()?),
            StructType::HostInternetScsiHbaDiscoveryCapabilities => Ok(from.as_any_box().downcast::<HostInternetScsiHbaDiscoveryCapabilities>()?),
            StructType::HostInternetScsiHbaDiscoveryProperties => Ok(from.as_any_box().downcast::<HostInternetScsiHbaDiscoveryProperties>()?),
            StructType::HostInternetScsiHbaIpCapabilities => Ok(from.as_any_box().downcast::<HostInternetScsiHbaIpCapabilities>()?),
            StructType::HostInternetScsiHbaIpProperties => Ok(from.as_any_box().downcast::<HostInternetScsiHbaIpProperties>()?),
            StructType::HostInternetScsiHbaIPv6Properties => Ok(from.as_any_box().downcast::<HostInternetScsiHbaIPv6Properties>()?),
            StructType::HostInternetScsiHbaIscsiIpv6Address => Ok(from.as_any_box().downcast::<HostInternetScsiHbaIscsiIpv6Address>()?),
            StructType::HostInternetScsiHbaSendTarget => Ok(from.as_any_box().downcast::<HostInternetScsiHbaSendTarget>()?),
            StructType::HostInternetScsiHbaStaticTarget => Ok(from.as_any_box().downcast::<HostInternetScsiHbaStaticTarget>()?),
            StructType::HostInternetScsiHbaTargetSet => Ok(from.as_any_box().downcast::<HostInternetScsiHbaTargetSet>()?),
            StructType::HostIpConfig => Ok(from.as_any_box().downcast::<HostIpConfig>()?),
            StructType::HostIpConfigIpV6Address => Ok(from.as_any_box().downcast::<HostIpConfigIpV6Address>()?),
            StructType::HostIpConfigIpV6AddressConfiguration => Ok(from.as_any_box().downcast::<HostIpConfigIpV6AddressConfiguration>()?),
            StructType::HostIpRouteConfig => Ok(from.as_any_box().downcast::<HostIpRouteConfig>()?),
            StructType::HostIpRouteConfigSpec => Ok(from.as_any_box().downcast::<HostIpRouteConfigSpec>()?),
            StructType::HostIpRouteEntry => Ok(from.as_any_box().downcast::<HostIpRouteEntry>()?),
            StructType::HostIpRouteOp => Ok(from.as_any_box().downcast::<HostIpRouteOp>()?),
            StructType::HostIpRouteTableConfig => Ok(from.as_any_box().downcast::<HostIpRouteTableConfig>()?),
            StructType::HostIpRouteTableInfo => Ok(from.as_any_box().downcast::<HostIpRouteTableInfo>()?),
            StructType::HostIpmiInfo => Ok(from.as_any_box().downcast::<HostIpmiInfo>()?),
            StructType::IscsiDependencyEntity => Ok(from.as_any_box().downcast::<IscsiDependencyEntity>()?),
            StructType::IscsiMigrationDependency => Ok(from.as_any_box().downcast::<IscsiMigrationDependency>()?),
            StructType::IscsiPortInfo => Ok(from.as_any_box().downcast::<IscsiPortInfo>()?),
            StructType::IscsiStatus => Ok(from.as_any_box().downcast::<IscsiStatus>()?),
            StructType::KernelModuleInfo => Ok(from.as_any_box().downcast::<KernelModuleInfo>()?),
            StructType::KernelModuleSectionInfo => Ok(from.as_any_box().downcast::<KernelModuleSectionInfo>()?),
            StructType::HostLicenseSpec => Ok(from.as_any_box().downcast::<HostLicenseSpec>()?),
            StructType::LinkDiscoveryProtocolConfig => Ok(from.as_any_box().downcast::<LinkDiscoveryProtocolConfig>()?),
            StructType::HostAccountSpec => Ok(from.as_any_box().downcast::<HostAccountSpec>()?),
            StructType::HostPosixAccountSpec => Ok(from.as_any_box().downcast::<HostPosixAccountSpec>()?),
            StructType::HostLocalFileSystemVolumeSpec => Ok(from.as_any_box().downcast::<HostLocalFileSystemVolumeSpec>()?),
            StructType::HostLowLevelProvisioningManagerDiskLayoutSpec => Ok(from.as_any_box().downcast::<HostLowLevelProvisioningManagerDiskLayoutSpec>()?),
            StructType::HostLowLevelProvisioningManagerFileDeleteResult => Ok(from.as_any_box().downcast::<HostLowLevelProvisioningManagerFileDeleteResult>()?),
            StructType::HostLowLevelProvisioningManagerFileDeleteSpec => Ok(from.as_any_box().downcast::<HostLowLevelProvisioningManagerFileDeleteSpec>()?),
            StructType::HostLowLevelProvisioningManagerFileReserveResult => Ok(from.as_any_box().downcast::<HostLowLevelProvisioningManagerFileReserveResult>()?),
            StructType::HostLowLevelProvisioningManagerFileReserveSpec => Ok(from.as_any_box().downcast::<HostLowLevelProvisioningManagerFileReserveSpec>()?),
            StructType::HostLowLevelProvisioningManagerSnapshotLayoutSpec => Ok(from.as_any_box().downcast::<HostLowLevelProvisioningManagerSnapshotLayoutSpec>()?),
            StructType::HostLowLevelProvisioningManagerVmMigrationStatus => Ok(from.as_any_box().downcast::<HostLowLevelProvisioningManagerVmMigrationStatus>()?),
            StructType::HostLowLevelProvisioningManagerVmRecoveryInfo => Ok(from.as_any_box().downcast::<HostLowLevelProvisioningManagerVmRecoveryInfo>()?),
            StructType::HostMaintenanceSpec => Ok(from.as_any_box().downcast::<HostMaintenanceSpec>()?),
            StructType::ServiceConsoleReservationInfo => Ok(from.as_any_box().downcast::<ServiceConsoleReservationInfo>()?),
            StructType::VirtualMachineMemoryReservationInfo => Ok(from.as_any_box().downcast::<VirtualMachineMemoryReservationInfo>()?),
            StructType::VirtualMachineMemoryReservationSpec => Ok(from.as_any_box().downcast::<VirtualMachineMemoryReservationSpec>()?),
            StructType::HostMemorySpec => Ok(from.as_any_box().downcast::<HostMemorySpec>()?),
            StructType::HostMemoryTierInfo => Ok(from.as_any_box().downcast::<HostMemoryTierInfo>()?),
            StructType::HostMountInfo => Ok(from.as_any_box().downcast::<HostMountInfo>()?),
            StructType::HostMultipathInfo => Ok(from.as_any_box().downcast::<HostMultipathInfo>()?),
            StructType::HostMultipathInfoLogicalUnit => Ok(from.as_any_box().downcast::<HostMultipathInfoLogicalUnit>()?),
            StructType::HostMultipathInfoLogicalUnitPolicy => Ok(from.as_any_box().downcast::<HostMultipathInfoLogicalUnitPolicy>()?),
            StructType::HostMultipathInfoFixedLogicalUnitPolicy => Ok(from.as_any_box().downcast::<HostMultipathInfoFixedLogicalUnitPolicy>()?),
            StructType::HostMultipathInfoHppLogicalUnitPolicy => Ok(from.as_any_box().downcast::<HostMultipathInfoHppLogicalUnitPolicy>()?),
            StructType::HostMultipathInfoLogicalUnitStorageArrayTypePolicy => Ok(from.as_any_box().downcast::<HostMultipathInfoLogicalUnitStorageArrayTypePolicy>()?),
            StructType::HostMultipathInfoPath => Ok(from.as_any_box().downcast::<HostMultipathInfoPath>()?),
            StructType::HostMultipathStateInfo => Ok(from.as_any_box().downcast::<HostMultipathStateInfo>()?),
            StructType::HostMultipathStateInfoPath => Ok(from.as_any_box().downcast::<HostMultipathStateInfoPath>()?),
            StructType::HostNasVolumeConfig => Ok(from.as_any_box().downcast::<HostNasVolumeConfig>()?),
            StructType::HostNasVolumeSpec => Ok(from.as_any_box().downcast::<HostNasVolumeSpec>()?),
            StructType::HostNasVolumeUserInfo => Ok(from.as_any_box().downcast::<HostNasVolumeUserInfo>()?),
            StructType::HostNatService => Ok(from.as_any_box().downcast::<HostNatService>()?),
            StructType::HostNatServiceConfig => Ok(from.as_any_box().downcast::<HostNatServiceConfig>()?),
            StructType::HostNatServiceNameServiceSpec => Ok(from.as_any_box().downcast::<HostNatServiceNameServiceSpec>()?),
            StructType::HostNatServicePortForwardSpec => Ok(from.as_any_box().downcast::<HostNatServicePortForwardSpec>()?),
            StructType::HostNatServiceSpec => Ok(from.as_any_box().downcast::<HostNatServiceSpec>()?),
            StructType::HostNetCapabilities => Ok(from.as_any_box().downcast::<HostNetCapabilities>()?),
            StructType::HostNetOffloadCapabilities => Ok(from.as_any_box().downcast::<HostNetOffloadCapabilities>()?),
            StructType::HostNetStackInstance => Ok(from.as_any_box().downcast::<HostNetStackInstance>()?),
            StructType::HostNetworkConfig => Ok(from.as_any_box().downcast::<HostNetworkConfig>()?),
            StructType::HostNetworkConfigNetStackSpec => Ok(from.as_any_box().downcast::<HostNetworkConfigNetStackSpec>()?),
            StructType::HostNetworkConfigResult => Ok(from.as_any_box().downcast::<HostNetworkConfigResult>()?),
            StructType::HostNetworkInfo => Ok(from.as_any_box().downcast::<HostNetworkInfo>()?),
            StructType::HostNetworkPolicy => Ok(from.as_any_box().downcast::<HostNetworkPolicy>()?),
            StructType::HostNicFailureCriteria => Ok(from.as_any_box().downcast::<HostNicFailureCriteria>()?),
            StructType::HostNicOrderPolicy => Ok(from.as_any_box().downcast::<HostNicOrderPolicy>()?),
            StructType::HostNicTeamingPolicy => Ok(from.as_any_box().downcast::<HostNicTeamingPolicy>()?),
            StructType::HostNetworkSecurityPolicy => Ok(from.as_any_box().downcast::<HostNetworkSecurityPolicy>()?),
            StructType::HostNetworkTrafficShapingPolicy => Ok(from.as_any_box().downcast::<HostNetworkTrafficShapingPolicy>()?),
            StructType::HostNtpConfig => Ok(from.as_any_box().downcast::<HostNtpConfig>()?),
            StructType::HostNumaInfo => Ok(from.as_any_box().downcast::<HostNumaInfo>()?),
            StructType::HostNumaNode => Ok(from.as_any_box().downcast::<HostNumaNode>()?),
            StructType::HostNumericSensorInfo => Ok(from.as_any_box().downcast::<HostNumericSensorInfo>()?),
            StructType::NvdimmDimmInfo => Ok(from.as_any_box().downcast::<NvdimmDimmInfo>()?),
            StructType::NvdimmGuid => Ok(from.as_any_box().downcast::<NvdimmGuid>()?),
            StructType::NvdimmHealthInfo => Ok(from.as_any_box().downcast::<NvdimmHealthInfo>()?),
            StructType::NvdimmInterleaveSetInfo => Ok(from.as_any_box().downcast::<NvdimmInterleaveSetInfo>()?),
            StructType::NvdimmNamespaceCreateSpec => Ok(from.as_any_box().downcast::<NvdimmNamespaceCreateSpec>()?),
            StructType::NvdimmNamespaceDeleteSpec => Ok(from.as_any_box().downcast::<NvdimmNamespaceDeleteSpec>()?),
            StructType::NvdimmNamespaceDetails => Ok(from.as_any_box().downcast::<NvdimmNamespaceDetails>()?),
            StructType::NvdimmNamespaceInfo => Ok(from.as_any_box().downcast::<NvdimmNamespaceInfo>()?),
            StructType::NvdimmSystemInfo => Ok(from.as_any_box().downcast::<NvdimmSystemInfo>()?),
            StructType::NvdimmPMemNamespaceCreateSpec => Ok(from.as_any_box().downcast::<NvdimmPMemNamespaceCreateSpec>()?),
            StructType::NvdimmRegionInfo => Ok(from.as_any_box().downcast::<NvdimmRegionInfo>()?),
            StructType::NvdimmSummary => Ok(from.as_any_box().downcast::<NvdimmSummary>()?),
            StructType::HostNvmeController => Ok(from.as_any_box().downcast::<HostNvmeController>()?),
            StructType::HostNvmeDisconnectSpec => Ok(from.as_any_box().downcast::<HostNvmeDisconnectSpec>()?),
            StructType::HostNvmeDiscoveryLog => Ok(from.as_any_box().downcast::<HostNvmeDiscoveryLog>()?),
            StructType::HostNvmeDiscoveryLogEntry => Ok(from.as_any_box().downcast::<HostNvmeDiscoveryLogEntry>()?),
            StructType::HostNvmeNamespace => Ok(from.as_any_box().downcast::<HostNvmeNamespace>()?),
            StructType::HostNvmeSpec => Ok(from.as_any_box().downcast::<HostNvmeSpec>()?),
            StructType::HostNvmeConnectSpec => Ok(from.as_any_box().downcast::<HostNvmeConnectSpec>()?),
            StructType::HostNvmeDiscoverSpec => Ok(from.as_any_box().downcast::<HostNvmeDiscoverSpec>()?),
            StructType::HostNvmeTopology => Ok(from.as_any_box().downcast::<HostNvmeTopology>()?),
            StructType::HostNvmeTopologyInterface => Ok(from.as_any_box().downcast::<HostNvmeTopologyInterface>()?),
            StructType::HostNvmeTransportParameters => Ok(from.as_any_box().downcast::<HostNvmeTransportParameters>()?),
            StructType::HostNvmeOpaqueTransportParameters => Ok(from.as_any_box().downcast::<HostNvmeOpaqueTransportParameters>()?),
            StructType::HostNvmeOverFibreChannelParameters => Ok(from.as_any_box().downcast::<HostNvmeOverFibreChannelParameters>()?),
            StructType::HostNvmeOverRdmaParameters => Ok(from.as_any_box().downcast::<HostNvmeOverRdmaParameters>()?),
            StructType::HostNvmeOverTcpParameters => Ok(from.as_any_box().downcast::<HostNvmeOverTcpParameters>()?),
            StructType::HostOpaqueNetworkInfo => Ok(from.as_any_box().downcast::<HostOpaqueNetworkInfo>()?),
            StructType::HostOpaqueSwitch => Ok(from.as_any_box().downcast::<HostOpaqueSwitch>()?),
            StructType::HostOpaqueSwitchPhysicalNicZone => Ok(from.as_any_box().downcast::<HostOpaqueSwitchPhysicalNicZone>()?),
            StructType::HostPatchManagerLocator => Ok(from.as_any_box().downcast::<HostPatchManagerLocator>()?),
            StructType::HostPatchManagerPatchManagerOperationSpec => Ok(from.as_any_box().downcast::<HostPatchManagerPatchManagerOperationSpec>()?),
            StructType::HostPatchManagerResult => Ok(from.as_any_box().downcast::<HostPatchManagerResult>()?),
            StructType::HostPatchManagerStatus => Ok(from.as_any_box().downcast::<HostPatchManagerStatus>()?),
            StructType::HostPatchManagerStatusPrerequisitePatch => Ok(from.as_any_box().downcast::<HostPatchManagerStatusPrerequisitePatch>()?),
            StructType::HostPathSelectionPolicyOption => Ok(from.as_any_box().downcast::<HostPathSelectionPolicyOption>()?),
            StructType::HostPciDevice => Ok(from.as_any_box().downcast::<HostPciDevice>()?),
            StructType::HostPciPassthruConfig => Ok(from.as_any_box().downcast::<HostPciPassthruConfig>()?),
            StructType::HostSriovConfig => Ok(from.as_any_box().downcast::<HostSriovConfig>()?),
            StructType::HostPciPassthruInfo => Ok(from.as_any_box().downcast::<HostPciPassthruInfo>()?),
            StructType::HostSriovInfo => Ok(from.as_any_box().downcast::<HostSriovInfo>()?),
            StructType::HostPersistentMemoryInfo => Ok(from.as_any_box().downcast::<HostPersistentMemoryInfo>()?),
            StructType::PhysicalNic => Ok(from.as_any_box().downcast::<PhysicalNic>()?),
            StructType::PhysicalNicCdpDeviceCapability => Ok(from.as_any_box().downcast::<PhysicalNicCdpDeviceCapability>()?),
            StructType::PhysicalNicCdpInfo => Ok(from.as_any_box().downcast::<PhysicalNicCdpInfo>()?),
            StructType::PhysicalNicConfig => Ok(from.as_any_box().downcast::<PhysicalNicConfig>()?),
            StructType::PhysicalNicLinkInfo => Ok(from.as_any_box().downcast::<PhysicalNicLinkInfo>()?),
            StructType::LinkLayerDiscoveryProtocolInfo => Ok(from.as_any_box().downcast::<LinkLayerDiscoveryProtocolInfo>()?),
            StructType::PhysicalNicHintInfo => Ok(from.as_any_box().downcast::<PhysicalNicHintInfo>()?),
            StructType::PhysicalNicHint => Ok(from.as_any_box().downcast::<PhysicalNicHint>()?),
            StructType::PhysicalNicIpHint => Ok(from.as_any_box().downcast::<PhysicalNicIpHint>()?),
            StructType::PhysicalNicNameHint => Ok(from.as_any_box().downcast::<PhysicalNicNameHint>()?),
            StructType::PhysicalNicSpec => Ok(from.as_any_box().downcast::<PhysicalNicSpec>()?),
            StructType::HostPlugStoreTopology => Ok(from.as_any_box().downcast::<HostPlugStoreTopology>()?),
            StructType::HostPlugStoreTopologyAdapter => Ok(from.as_any_box().downcast::<HostPlugStoreTopologyAdapter>()?),
            StructType::HostPlugStoreTopologyDevice => Ok(from.as_any_box().downcast::<HostPlugStoreTopologyDevice>()?),
            StructType::HostPlugStoreTopologyPath => Ok(from.as_any_box().downcast::<HostPlugStoreTopologyPath>()?),
            StructType::HostPlugStoreTopologyPlugin => Ok(from.as_any_box().downcast::<HostPlugStoreTopologyPlugin>()?),
            StructType::HostPlugStoreTopologyTarget => Ok(from.as_any_box().downcast::<HostPlugStoreTopologyTarget>()?),
            StructType::HostPortGroup => Ok(from.as_any_box().downcast::<HostPortGroup>()?),
            StructType::HostPortGroupConfig => Ok(from.as_any_box().downcast::<HostPortGroupConfig>()?),
            StructType::HostPortGroupPort => Ok(from.as_any_box().downcast::<HostPortGroupPort>()?),
            StructType::HostPortGroupSpec => Ok(from.as_any_box().downcast::<HostPortGroupSpec>()?),
            StructType::PowerSystemCapability => Ok(from.as_any_box().downcast::<PowerSystemCapability>()?),
            StructType::PowerSystemInfo => Ok(from.as_any_box().downcast::<PowerSystemInfo>()?),
            StructType::HostPowerPolicy => Ok(from.as_any_box().downcast::<HostPowerPolicy>()?),
            StructType::HostProtocolEndpoint => Ok(from.as_any_box().downcast::<HostProtocolEndpoint>()?),
            StructType::HostPtpConfig => Ok(from.as_any_box().downcast::<HostPtpConfig>()?),
            StructType::HostPtpConfigPtpPort => Ok(from.as_any_box().downcast::<HostPtpConfigPtpPort>()?),
            StructType::HostQualifiedName => Ok(from.as_any_box().downcast::<HostQualifiedName>()?),
            StructType::HostRdmaDevice => Ok(from.as_any_box().downcast::<HostRdmaDevice>()?),
            StructType::HostRdmaDeviceBacking => Ok(from.as_any_box().downcast::<HostRdmaDeviceBacking>()?),
            StructType::HostRdmaDevicePnicBacking => Ok(from.as_any_box().downcast::<HostRdmaDevicePnicBacking>()?),
            StructType::HostRdmaDeviceCapability => Ok(from.as_any_box().downcast::<HostRdmaDeviceCapability>()?),
            StructType::HostRdmaDeviceConnectionInfo => Ok(from.as_any_box().downcast::<HostRdmaDeviceConnectionInfo>()?),
            StructType::HostReliableMemoryInfo => Ok(from.as_any_box().downcast::<HostReliableMemoryInfo>()?),
            StructType::HostResignatureRescanResult => Ok(from.as_any_box().downcast::<HostResignatureRescanResult>()?),
            StructType::HostFirewallRuleset => Ok(from.as_any_box().downcast::<HostFirewallRuleset>()?),
            StructType::HostFirewallRulesetIpList => Ok(from.as_any_box().downcast::<HostFirewallRulesetIpList>()?),
            StructType::HostFirewallRulesetIpNetwork => Ok(from.as_any_box().downcast::<HostFirewallRulesetIpNetwork>()?),
            StructType::HostFirewallRule => Ok(from.as_any_box().downcast::<HostFirewallRule>()?),
            StructType::HostFirewallRulesetRulesetSpec => Ok(from.as_any_box().downcast::<HostFirewallRulesetRulesetSpec>()?),
            StructType::HostRuntimeInfo => Ok(from.as_any_box().downcast::<HostRuntimeInfo>()?),
            StructType::HostRuntimeInfoNetStackInstanceRuntimeInfo => Ok(from.as_any_box().downcast::<HostRuntimeInfoNetStackInstanceRuntimeInfo>()?),
            StructType::HostNetworkResourceRuntime => Ok(from.as_any_box().downcast::<HostNetworkResourceRuntime>()?),
            StructType::HostRuntimeInfoNetworkRuntimeInfo => Ok(from.as_any_box().downcast::<HostRuntimeInfoNetworkRuntimeInfo>()?),
            StructType::HostPlacedVirtualNicIdentifier => Ok(from.as_any_box().downcast::<HostPlacedVirtualNicIdentifier>()?),
            StructType::HostPnicNetworkResourceInfo => Ok(from.as_any_box().downcast::<HostPnicNetworkResourceInfo>()?),
            StructType::HostRuntimeInfoStateEncryptionInfo => Ok(from.as_any_box().downcast::<HostRuntimeInfoStateEncryptionInfo>()?),
            StructType::HostScsiDiskPartition => Ok(from.as_any_box().downcast::<HostScsiDiskPartition>()?),
            StructType::ScsiLunCapabilities => Ok(from.as_any_box().downcast::<ScsiLunCapabilities>()?),
            StructType::ScsiLunDescriptor => Ok(from.as_any_box().downcast::<ScsiLunDescriptor>()?),
            StructType::ScsiLunDurableName => Ok(from.as_any_box().downcast::<ScsiLunDurableName>()?),
            StructType::HostScsiTopology => Ok(from.as_any_box().downcast::<HostScsiTopology>()?),
            StructType::HostScsiTopologyInterface => Ok(from.as_any_box().downcast::<HostScsiTopologyInterface>()?),
            StructType::HostScsiTopologyLun => Ok(from.as_any_box().downcast::<HostScsiTopologyLun>()?),
            StructType::HostScsiTopologyTarget => Ok(from.as_any_box().downcast::<HostScsiTopologyTarget>()?),
            StructType::HostSecuritySpec => Ok(from.as_any_box().downcast::<HostSecuritySpec>()?),
            StructType::HostService => Ok(from.as_any_box().downcast::<HostService>()?),
            StructType::HostServiceSourcePackage => Ok(from.as_any_box().downcast::<HostServiceSourcePackage>()?),
            StructType::HostServiceConfig => Ok(from.as_any_box().downcast::<HostServiceConfig>()?),
            StructType::HostServiceInfo => Ok(from.as_any_box().downcast::<HostServiceInfo>()?),
            StructType::HostSevInfo => Ok(from.as_any_box().downcast::<HostSevInfo>()?),
            StructType::HostSgxInfo => Ok(from.as_any_box().downcast::<HostSgxInfo>()?),
            StructType::HostSgxRegistrationInfo => Ok(from.as_any_box().downcast::<HostSgxRegistrationInfo>()?),
            StructType::HostSharedGpuCapabilities => Ok(from.as_any_box().downcast::<HostSharedGpuCapabilities>()?),
            StructType::HostSnmpSystemAgentLimits => Ok(from.as_any_box().downcast::<HostSnmpSystemAgentLimits>()?),
            StructType::HostSnmpConfigSpec => Ok(from.as_any_box().downcast::<HostSnmpConfigSpec>()?),
            StructType::HostSnmpDestination => Ok(from.as_any_box().downcast::<HostSnmpDestination>()?),
            StructType::SoftwarePackage => Ok(from.as_any_box().downcast::<SoftwarePackage>()?),
            StructType::SoftwarePackageCapability => Ok(from.as_any_box().downcast::<SoftwarePackageCapability>()?),
            StructType::Relation => Ok(from.as_any_box().downcast::<Relation>()?),
            StructType::HostSriovDevicePoolInfo => Ok(from.as_any_box().downcast::<HostSriovDevicePoolInfo>()?),
            StructType::HostSriovNetworkDevicePoolInfo => Ok(from.as_any_box().downcast::<HostSriovNetworkDevicePoolInfo>()?),
            StructType::HostSslThumbprintInfo => Ok(from.as_any_box().downcast::<HostSslThumbprintInfo>()?),
            StructType::HostStorageArrayTypePolicyOption => Ok(from.as_any_box().downcast::<HostStorageArrayTypePolicyOption>()?),
            StructType::HostStorageDeviceInfo => Ok(from.as_any_box().downcast::<HostStorageDeviceInfo>()?),
            StructType::HostStorageSystemDiskLocatorLedResult => Ok(from.as_any_box().downcast::<HostStorageSystemDiskLocatorLedResult>()?),
            StructType::HostStorageSystemScsiLunResult => Ok(from.as_any_box().downcast::<HostStorageSystemScsiLunResult>()?),
            StructType::HostStorageSystemVmfsVolumeResult => Ok(from.as_any_box().downcast::<HostStorageSystemVmfsVolumeResult>()?),
            StructType::HostListSummary => Ok(from.as_any_box().downcast::<HostListSummary>()?),
            StructType::HostConfigSummary => Ok(from.as_any_box().downcast::<HostConfigSummary>()?),
            StructType::HostListSummaryGatewaySummary => Ok(from.as_any_box().downcast::<HostListSummaryGatewaySummary>()?),
            StructType::HostHardwareSummary => Ok(from.as_any_box().downcast::<HostHardwareSummary>()?),
            StructType::HostListSummaryQuickStats => Ok(from.as_any_box().downcast::<HostListSummaryQuickStats>()?),
            StructType::SystemEventInfo => Ok(from.as_any_box().downcast::<SystemEventInfo>()?),
            StructType::HostSystemHealthInfo => Ok(from.as_any_box().downcast::<HostSystemHealthInfo>()?),
            StructType::HostSystemIdentificationInfo => Ok(from.as_any_box().downcast::<HostSystemIdentificationInfo>()?),
            StructType::HostSystemInfo => Ok(from.as_any_box().downcast::<HostSystemInfo>()?),
            StructType::HostSystemResourceInfo => Ok(from.as_any_box().downcast::<HostSystemResourceInfo>()?),
            StructType::HostSystemSwapConfiguration => Ok(from.as_any_box().downcast::<HostSystemSwapConfiguration>()?),
            StructType::HostSystemSwapConfigurationSystemSwapOption => Ok(from.as_any_box().downcast::<HostSystemSwapConfigurationSystemSwapOption>()?),
            StructType::HostSystemSwapConfigurationDatastoreOption => Ok(from.as_any_box().downcast::<HostSystemSwapConfigurationDatastoreOption>()?),
            StructType::HostSystemSwapConfigurationDisabledOption => Ok(from.as_any_box().downcast::<HostSystemSwapConfigurationDisabledOption>()?),
            StructType::HostSystemSwapConfigurationHostCacheOption => Ok(from.as_any_box().downcast::<HostSystemSwapConfigurationHostCacheOption>()?),
            StructType::HostSystemSwapConfigurationHostLocalSwapOption => Ok(from.as_any_box().downcast::<HostSystemSwapConfigurationHostLocalSwapOption>()?),
            StructType::HostTargetTransport => Ok(from.as_any_box().downcast::<HostTargetTransport>()?),
            StructType::HostBlockAdapterTargetTransport => Ok(from.as_any_box().downcast::<HostBlockAdapterTargetTransport>()?),
            StructType::HostFibreChannelTargetTransport => Ok(from.as_any_box().downcast::<HostFibreChannelTargetTransport>()?),
            StructType::HostFibreChannelOverEthernetTargetTransport => Ok(from.as_any_box().downcast::<HostFibreChannelOverEthernetTargetTransport>()?),
            StructType::HostInternetScsiTargetTransport => Ok(from.as_any_box().downcast::<HostInternetScsiTargetTransport>()?),
            StructType::HostParallelScsiTargetTransport => Ok(from.as_any_box().downcast::<HostParallelScsiTargetTransport>()?),
            StructType::HostPcieTargetTransport => Ok(from.as_any_box().downcast::<HostPcieTargetTransport>()?),
            StructType::HostRdmaTargetTransport => Ok(from.as_any_box().downcast::<HostRdmaTargetTransport>()?),
            StructType::HostSerialAttachedTargetTransport => Ok(from.as_any_box().downcast::<HostSerialAttachedTargetTransport>()?),
            StructType::HostTcpTargetTransport => Ok(from.as_any_box().downcast::<HostTcpTargetTransport>()?),
            StructType::HostTpmAttestationInfo => Ok(from.as_any_box().downcast::<HostTpmAttestationInfo>()?),
            StructType::HostTpmAttestationReport => Ok(from.as_any_box().downcast::<HostTpmAttestationReport>()?),
            StructType::HostTpmEventDetails => Ok(from.as_any_box().downcast::<HostTpmEventDetails>()?),
            StructType::HostTpmBootCompleteEventDetails => Ok(from.as_any_box().downcast::<HostTpmBootCompleteEventDetails>()?),
            StructType::HostTpmBootSecurityOptionEventDetails => Ok(from.as_any_box().downcast::<HostTpmBootSecurityOptionEventDetails>()?),
            StructType::HostTpmNvTagEventDetails => Ok(from.as_any_box().downcast::<HostTpmNvTagEventDetails>()?),
            StructType::HostTpmSignerEventDetails => Ok(from.as_any_box().downcast::<HostTpmSignerEventDetails>()?),
            StructType::HostTpmCommandEventDetails => Ok(from.as_any_box().downcast::<HostTpmCommandEventDetails>()?),
            StructType::HostTpmOptionEventDetails => Ok(from.as_any_box().downcast::<HostTpmOptionEventDetails>()?),
            StructType::HostTpmSoftwareComponentEventDetails => Ok(from.as_any_box().downcast::<HostTpmSoftwareComponentEventDetails>()?),
            StructType::HostTpmVersionEventDetails => Ok(from.as_any_box().downcast::<HostTpmVersionEventDetails>()?),
            StructType::HostTpmEventLogEntry => Ok(from.as_any_box().downcast::<HostTpmEventLogEntry>()?),
            StructType::HostTrustAuthorityAttestationInfo => Ok(from.as_any_box().downcast::<HostTrustAuthorityAttestationInfo>()?),
            StructType::HostUnresolvedVmfsExtent => Ok(from.as_any_box().downcast::<HostUnresolvedVmfsExtent>()?),
            StructType::HostUnresolvedVmfsResignatureSpec => Ok(from.as_any_box().downcast::<HostUnresolvedVmfsResignatureSpec>()?),
            StructType::HostUnresolvedVmfsResolutionResult => Ok(from.as_any_box().downcast::<HostUnresolvedVmfsResolutionResult>()?),
            StructType::HostUnresolvedVmfsResolutionSpec => Ok(from.as_any_box().downcast::<HostUnresolvedVmfsResolutionSpec>()?),
            StructType::HostUnresolvedVmfsVolume => Ok(from.as_any_box().downcast::<HostUnresolvedVmfsVolume>()?),
            StructType::HostUnresolvedVmfsVolumeResolveStatus => Ok(from.as_any_box().downcast::<HostUnresolvedVmfsVolumeResolveStatus>()?),
            StructType::HostVFlashManagerVFlashCacheConfigInfo => Ok(from.as_any_box().downcast::<HostVFlashManagerVFlashCacheConfigInfo>()?),
            StructType::HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption => Ok(from.as_any_box().downcast::<HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption>()?),
            StructType::HostVFlashManagerVFlashCacheConfigSpec => Ok(from.as_any_box().downcast::<HostVFlashManagerVFlashCacheConfigSpec>()?),
            StructType::HostVFlashManagerVFlashConfigInfo => Ok(from.as_any_box().downcast::<HostVFlashManagerVFlashConfigInfo>()?),
            StructType::HostVFlashManagerVFlashResourceConfigInfo => Ok(from.as_any_box().downcast::<HostVFlashManagerVFlashResourceConfigInfo>()?),
            StructType::HostVFlashManagerVFlashResourceConfigSpec => Ok(from.as_any_box().downcast::<HostVFlashManagerVFlashResourceConfigSpec>()?),
            StructType::HostVFlashManagerVFlashResourceRunTimeInfo => Ok(from.as_any_box().downcast::<HostVFlashManagerVFlashResourceRunTimeInfo>()?),
            StructType::HostVFlashResourceConfigurationResult => Ok(from.as_any_box().downcast::<HostVFlashResourceConfigurationResult>()?),
            StructType::HostVMotionConfig => Ok(from.as_any_box().downcast::<HostVMotionConfig>()?),
            StructType::HostVMotionInfo => Ok(from.as_any_box().downcast::<HostVMotionInfo>()?),
            StructType::HostVMotionManagerDstInstantCloneResult => Ok(from.as_any_box().downcast::<HostVMotionManagerDstInstantCloneResult>()?),
            StructType::HostVMotionManagerSrcInstantCloneResult => Ok(from.as_any_box().downcast::<HostVMotionManagerSrcInstantCloneResult>()?),
            StructType::HostVMotionNetConfig => Ok(from.as_any_box().downcast::<HostVMotionNetConfig>()?),
            StructType::HostVffsSpec => Ok(from.as_any_box().downcast::<HostVffsSpec>()?),
            StructType::HostVirtualNic => Ok(from.as_any_box().downcast::<HostVirtualNic>()?),
            StructType::HostVirtualNicConfig => Ok(from.as_any_box().downcast::<HostVirtualNicConfig>()?),
            StructType::HostVirtualNicIpRouteSpec => Ok(from.as_any_box().downcast::<HostVirtualNicIpRouteSpec>()?),
            StructType::HostVirtualNicOpaqueNetworkSpec => Ok(from.as_any_box().downcast::<HostVirtualNicOpaqueNetworkSpec>()?),
            StructType::HostVirtualNicSpec => Ok(from.as_any_box().downcast::<HostVirtualNicSpec>()?),
            StructType::HostVirtualNicConnection => Ok(from.as_any_box().downcast::<HostVirtualNicConnection>()?),
            StructType::VirtualNicManagerNetConfig => Ok(from.as_any_box().downcast::<VirtualNicManagerNetConfig>()?),
            StructType::HostVirtualNicManagerNicTypeSelection => Ok(from.as_any_box().downcast::<HostVirtualNicManagerNicTypeSelection>()?),
            StructType::HostVirtualNicManagerInfo => Ok(from.as_any_box().downcast::<HostVirtualNicManagerInfo>()?),
            StructType::HostVirtualSwitch => Ok(from.as_any_box().downcast::<HostVirtualSwitch>()?),
            StructType::HostVirtualSwitchBeaconConfig => Ok(from.as_any_box().downcast::<HostVirtualSwitchBeaconConfig>()?),
            StructType::HostVirtualSwitchBridge => Ok(from.as_any_box().downcast::<HostVirtualSwitchBridge>()?),
            StructType::HostVirtualSwitchAutoBridge => Ok(from.as_any_box().downcast::<HostVirtualSwitchAutoBridge>()?),
            StructType::HostVirtualSwitchBondBridge => Ok(from.as_any_box().downcast::<HostVirtualSwitchBondBridge>()?),
            StructType::HostVirtualSwitchSimpleBridge => Ok(from.as_any_box().downcast::<HostVirtualSwitchSimpleBridge>()?),
            StructType::HostVirtualSwitchConfig => Ok(from.as_any_box().downcast::<HostVirtualSwitchConfig>()?),
            StructType::HostVirtualSwitchSpec => Ok(from.as_any_box().downcast::<HostVirtualSwitchSpec>()?),
            StructType::HostVmciAccessManagerAccessSpec => Ok(from.as_any_box().downcast::<HostVmciAccessManagerAccessSpec>()?),
            StructType::VmfsDatastoreOption => Ok(from.as_any_box().downcast::<VmfsDatastoreOption>()?),
            StructType::VmfsDatastoreBaseOption => Ok(from.as_any_box().downcast::<VmfsDatastoreBaseOption>()?),
            StructType::VmfsDatastoreMultipleExtentOption => Ok(from.as_any_box().downcast::<VmfsDatastoreMultipleExtentOption>()?),
            StructType::VmfsDatastoreSingleExtentOption => Ok(from.as_any_box().downcast::<VmfsDatastoreSingleExtentOption>()?),
            StructType::VmfsDatastoreAllExtentOption => Ok(from.as_any_box().downcast::<VmfsDatastoreAllExtentOption>()?),
            StructType::VmfsDatastoreSpec => Ok(from.as_any_box().downcast::<VmfsDatastoreSpec>()?),
            StructType::VmfsDatastoreCreateSpec => Ok(from.as_any_box().downcast::<VmfsDatastoreCreateSpec>()?),
            StructType::VmfsDatastoreExpandSpec => Ok(from.as_any_box().downcast::<VmfsDatastoreExpandSpec>()?),
            StructType::VmfsDatastoreExtendSpec => Ok(from.as_any_box().downcast::<VmfsDatastoreExtendSpec>()?),
            StructType::HostVmfsRescanResult => Ok(from.as_any_box().downcast::<HostVmfsRescanResult>()?),
            StructType::VmfsConfigOption => Ok(from.as_any_box().downcast::<VmfsConfigOption>()?),
            StructType::HostVmfsSpec => Ok(from.as_any_box().downcast::<HostVmfsSpec>()?),
            StructType::VmfsUnmapBandwidthSpec => Ok(from.as_any_box().downcast::<VmfsUnmapBandwidthSpec>()?),
            StructType::HostVsanInternalSystemCmmdsQuery => Ok(from.as_any_box().downcast::<HostVsanInternalSystemCmmdsQuery>()?),
            StructType::HostVsanInternalSystemDeleteVsanObjectsResult => Ok(from.as_any_box().downcast::<HostVsanInternalSystemDeleteVsanObjectsResult>()?),
            StructType::VsanNewPolicyBatch => Ok(from.as_any_box().downcast::<VsanNewPolicyBatch>()?),
            StructType::VsanPolicyChangeBatch => Ok(from.as_any_box().downcast::<VsanPolicyChangeBatch>()?),
            StructType::VsanPolicyCost => Ok(from.as_any_box().downcast::<VsanPolicyCost>()?),
            StructType::VsanPolicySatisfiability => Ok(from.as_any_box().downcast::<VsanPolicySatisfiability>()?),
            StructType::HostVsanInternalSystemVsanObjectOperationResult => Ok(from.as_any_box().downcast::<HostVsanInternalSystemVsanObjectOperationResult>()?),
            StructType::HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult => Ok(from.as_any_box().downcast::<HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult>()?),
            StructType::HostVvolNqn => Ok(from.as_any_box().downcast::<HostVvolNqn>()?),
            StructType::VVolHostPe => Ok(from.as_any_box().downcast::<VVolHostPe>()?),
            StructType::HostVvolVolumeHostVvolNqn => Ok(from.as_any_box().downcast::<HostVvolVolumeHostVvolNqn>()?),
            StructType::HostVvolVolumeSpecification => Ok(from.as_any_box().downcast::<HostVvolVolumeSpecification>()?),
            StructType::NetDhcpConfigInfo => Ok(from.as_any_box().downcast::<NetDhcpConfigInfo>()?),
            StructType::NetDhcpConfigInfoDhcpOptions => Ok(from.as_any_box().downcast::<NetDhcpConfigInfoDhcpOptions>()?),
            StructType::NetDhcpConfigSpec => Ok(from.as_any_box().downcast::<NetDhcpConfigSpec>()?),
            StructType::NetDhcpConfigSpecDhcpOptionsSpec => Ok(from.as_any_box().downcast::<NetDhcpConfigSpecDhcpOptionsSpec>()?),
            StructType::NetDnsConfigInfo => Ok(from.as_any_box().downcast::<NetDnsConfigInfo>()?),
            StructType::NetDnsConfigSpec => Ok(from.as_any_box().downcast::<NetDnsConfigSpec>()?),
            StructType::NetIpConfigInfo => Ok(from.as_any_box().downcast::<NetIpConfigInfo>()?),
            StructType::NetIpConfigInfoIpAddress => Ok(from.as_any_box().downcast::<NetIpConfigInfoIpAddress>()?),
            StructType::NetIpConfigSpec => Ok(from.as_any_box().downcast::<NetIpConfigSpec>()?),
            StructType::NetIpConfigSpecIpAddressSpec => Ok(from.as_any_box().downcast::<NetIpConfigSpecIpAddressSpec>()?),
            StructType::NetIpRouteConfigInfo => Ok(from.as_any_box().downcast::<NetIpRouteConfigInfo>()?),
            StructType::NetIpRouteConfigInfoGateway => Ok(from.as_any_box().downcast::<NetIpRouteConfigInfoGateway>()?),
            StructType::NetIpRouteConfigInfoIpRoute => Ok(from.as_any_box().downcast::<NetIpRouteConfigInfoIpRoute>()?),
            StructType::NetIpRouteConfigSpec => Ok(from.as_any_box().downcast::<NetIpRouteConfigSpec>()?),
            StructType::NetIpRouteConfigSpecGatewaySpec => Ok(from.as_any_box().downcast::<NetIpRouteConfigSpecGatewaySpec>()?),
            StructType::NetIpRouteConfigSpecIpRouteSpec => Ok(from.as_any_box().downcast::<NetIpRouteConfigSpecIpRouteSpec>()?),
            StructType::NetIpStackInfo => Ok(from.as_any_box().downcast::<NetIpStackInfo>()?),
            StructType::NetIpStackInfoDefaultRouter => Ok(from.as_any_box().downcast::<NetIpStackInfoDefaultRouter>()?),
            StructType::NetIpStackInfoNetToMedia => Ok(from.as_any_box().downcast::<NetIpStackInfoNetToMedia>()?),
            StructType::NetBiosConfigInfo => Ok(from.as_any_box().downcast::<NetBiosConfigInfo>()?),
            StructType::WinNetBiosConfigInfo => Ok(from.as_any_box().downcast::<WinNetBiosConfigInfo>()?),
            StructType::ArrayUpdateSpec => Ok(from.as_any_box().downcast::<ArrayUpdateSpec>()?),
            StructType::ClusterDasVmConfigSpec => Ok(from.as_any_box().downcast::<ClusterDasVmConfigSpec>()?),
            StructType::ClusterDatastoreUpdateSpec => Ok(from.as_any_box().downcast::<ClusterDatastoreUpdateSpec>()?),
            StructType::ClusterDpmHostConfigSpec => Ok(from.as_any_box().downcast::<ClusterDpmHostConfigSpec>()?),
            StructType::ClusterDrsVmConfigSpec => Ok(from.as_any_box().downcast::<ClusterDrsVmConfigSpec>()?),
            StructType::ClusterGroupSpec => Ok(from.as_any_box().downcast::<ClusterGroupSpec>()?),
            StructType::ClusterPreemptibleVmPairSpec => Ok(from.as_any_box().downcast::<ClusterPreemptibleVmPairSpec>()?),
            StructType::ClusterRuleSpec => Ok(from.as_any_box().downcast::<ClusterRuleSpec>()?),
            StructType::ClusterTagCategoryUpdateSpec => Ok(from.as_any_box().downcast::<ClusterTagCategoryUpdateSpec>()?),
            StructType::ClusterVmOrchestrationSpec => Ok(from.as_any_box().downcast::<ClusterVmOrchestrationSpec>()?),
            StructType::StorageDrsOptionSpec => Ok(from.as_any_box().downcast::<StorageDrsOptionSpec>()?),
            StructType::StorageDrsVmConfigSpec => Ok(from.as_any_box().downcast::<StorageDrsVmConfigSpec>()?),
            StructType::VAppOvfSectionSpec => Ok(from.as_any_box().downcast::<VAppOvfSectionSpec>()?),
            StructType::VAppProductSpec => Ok(from.as_any_box().downcast::<VAppProductSpec>()?),
            StructType::VAppPropertySpec => Ok(from.as_any_box().downcast::<VAppPropertySpec>()?),
            StructType::VirtualMachineCpuIdInfoSpec => Ok(from.as_any_box().downcast::<VirtualMachineCpuIdInfoSpec>()?),
            StructType::OptionType => Ok(from.as_any_box().downcast::<OptionType>()?),
            StructType::BoolOption => Ok(from.as_any_box().downcast::<BoolOption>()?),
            StructType::ChoiceOption => Ok(from.as_any_box().downcast::<ChoiceOption>()?),
            StructType::FloatOption => Ok(from.as_any_box().downcast::<FloatOption>()?),
            StructType::IntOption => Ok(from.as_any_box().downcast::<IntOption>()?),
            StructType::LongOption => Ok(from.as_any_box().downcast::<LongOption>()?),
            StructType::StringOption => Ok(from.as_any_box().downcast::<StringOption>()?),
            StructType::OptionValue => Ok(from.as_any_box().downcast::<OptionValue>()?),
            StructType::HostInternetScsiHbaParamValue => Ok(from.as_any_box().downcast::<HostInternetScsiHbaParamValue>()?),
            StructType::ApplyProfile => Ok(from.as_any_box().downcast::<ApplyProfile>()?),
            StructType::ProfileApplyProfileElement => Ok(from.as_any_box().downcast::<ProfileApplyProfileElement>()?),
            StructType::ActiveDirectoryProfile => Ok(from.as_any_box().downcast::<ActiveDirectoryProfile>()?),
            StructType::AuthenticationProfile => Ok(from.as_any_box().downcast::<AuthenticationProfile>()?),
            StructType::DateTimeProfile => Ok(from.as_any_box().downcast::<DateTimeProfile>()?),
            StructType::DvsProfile => Ok(from.as_any_box().downcast::<DvsProfile>()?),
            StructType::DvsVNicProfile => Ok(from.as_any_box().downcast::<DvsVNicProfile>()?),
            StructType::DvsHostVNicProfile => Ok(from.as_any_box().downcast::<DvsHostVNicProfile>()?),
            StructType::DvsServiceConsoleVNicProfile => Ok(from.as_any_box().downcast::<DvsServiceConsoleVNicProfile>()?),
            StructType::FirewallProfile => Ok(from.as_any_box().downcast::<FirewallProfile>()?),
            StructType::FirewallProfileRulesetProfile => Ok(from.as_any_box().downcast::<FirewallProfileRulesetProfile>()?),
            StructType::HostApplyProfile => Ok(from.as_any_box().downcast::<HostApplyProfile>()?),
            StructType::HostMemoryProfile => Ok(from.as_any_box().downcast::<HostMemoryProfile>()?),
            StructType::IpAddressProfile => Ok(from.as_any_box().downcast::<IpAddressProfile>()?),
            StructType::IpRouteProfile => Ok(from.as_any_box().downcast::<IpRouteProfile>()?),
            StructType::NasStorageProfile => Ok(from.as_any_box().downcast::<NasStorageProfile>()?),
            StructType::NetStackInstanceProfile => Ok(from.as_any_box().downcast::<NetStackInstanceProfile>()?),
            StructType::NetworkPolicyProfile => Ok(from.as_any_box().downcast::<NetworkPolicyProfile>()?),
            StructType::NetworkProfile => Ok(from.as_any_box().downcast::<NetworkProfile>()?),
            StructType::NetworkProfileDnsConfigProfile => Ok(from.as_any_box().downcast::<NetworkProfileDnsConfigProfile>()?),
            StructType::NsxHostVNicProfile => Ok(from.as_any_box().downcast::<NsxHostVNicProfile>()?),
            StructType::OpaqueSwitchProfile => Ok(from.as_any_box().downcast::<OpaqueSwitchProfile>()?),
            StructType::OptionProfile => Ok(from.as_any_box().downcast::<OptionProfile>()?),
            StructType::PermissionProfile => Ok(from.as_any_box().downcast::<PermissionProfile>()?),
            StructType::PhysicalNicProfile => Ok(from.as_any_box().downcast::<PhysicalNicProfile>()?),
            StructType::PnicUplinkProfile => Ok(from.as_any_box().downcast::<PnicUplinkProfile>()?),
            StructType::PortGroupProfile => Ok(from.as_any_box().downcast::<PortGroupProfile>()?),
            StructType::HostPortGroupProfile => Ok(from.as_any_box().downcast::<HostPortGroupProfile>()?),
            StructType::ServiceConsolePortGroupProfile => Ok(from.as_any_box().downcast::<ServiceConsolePortGroupProfile>()?),
            StructType::VmPortGroupProfile => Ok(from.as_any_box().downcast::<VmPortGroupProfile>()?),
            StructType::VirtualSwitchSelectionProfile => Ok(from.as_any_box().downcast::<VirtualSwitchSelectionProfile>()?),
            StructType::VlanProfile => Ok(from.as_any_box().downcast::<VlanProfile>()?),
            StructType::SecurityProfile => Ok(from.as_any_box().downcast::<SecurityProfile>()?),
            StructType::ServiceProfile => Ok(from.as_any_box().downcast::<ServiceProfile>()?),
            StructType::StaticRouteProfile => Ok(from.as_any_box().downcast::<StaticRouteProfile>()?),
            StructType::StorageProfile => Ok(from.as_any_box().downcast::<StorageProfile>()?),
            StructType::UserGroupProfile => Ok(from.as_any_box().downcast::<UserGroupProfile>()?),
            StructType::UserProfile => Ok(from.as_any_box().downcast::<UserProfile>()?),
            StructType::VirtualSwitchProfile => Ok(from.as_any_box().downcast::<VirtualSwitchProfile>()?),
            StructType::LinkProfile => Ok(from.as_any_box().downcast::<LinkProfile>()?),
            StructType::NumPortsProfile => Ok(from.as_any_box().downcast::<NumPortsProfile>()?),
            StructType::ProfileApplyProfileProperty => Ok(from.as_any_box().downcast::<ProfileApplyProfileProperty>()?),
            StructType::ComplianceLocator => Ok(from.as_any_box().downcast::<ComplianceLocator>()?),
            StructType::ComplianceProfile => Ok(from.as_any_box().downcast::<ComplianceProfile>()?),
            StructType::ComplianceResult => Ok(from.as_any_box().downcast::<ComplianceResult>()?),
            StructType::ComplianceFailure => Ok(from.as_any_box().downcast::<ComplianceFailure>()?),
            StructType::ComplianceFailureComplianceFailureValues => Ok(from.as_any_box().downcast::<ComplianceFailureComplianceFailureValues>()?),
            StructType::ProfileDeferredPolicyOptionParameter => Ok(from.as_any_box().downcast::<ProfileDeferredPolicyOptionParameter>()?),
            StructType::ProfileExpression => Ok(from.as_any_box().downcast::<ProfileExpression>()?),
            StructType::ProfileCompositeExpression => Ok(from.as_any_box().downcast::<ProfileCompositeExpression>()?),
            StructType::ProfileSimpleExpression => Ok(from.as_any_box().downcast::<ProfileSimpleExpression>()?),
            StructType::ProfileExpressionMetadata => Ok(from.as_any_box().downcast::<ProfileExpressionMetadata>()?),
            StructType::ProfileParameterMetadata => Ok(from.as_any_box().downcast::<ProfileParameterMetadata>()?),
            StructType::ProfileParameterMetadataParameterRelationMetadata => Ok(from.as_any_box().downcast::<ProfileParameterMetadataParameterRelationMetadata>()?),
            StructType::ProfilePolicy => Ok(from.as_any_box().downcast::<ProfilePolicy>()?),
            StructType::ProfilePolicyMetadata => Ok(from.as_any_box().downcast::<ProfilePolicyMetadata>()?),
            StructType::PolicyOption => Ok(from.as_any_box().downcast::<PolicyOption>()?),
            StructType::CompositePolicyOption => Ok(from.as_any_box().downcast::<CompositePolicyOption>()?),
            StructType::ProfilePolicyOptionMetadata => Ok(from.as_any_box().downcast::<ProfilePolicyOptionMetadata>()?),
            StructType::ProfileCompositePolicyOptionMetadata => Ok(from.as_any_box().downcast::<ProfileCompositePolicyOptionMetadata>()?),
            StructType::UserInputRequiredParameterMetadata => Ok(from.as_any_box().downcast::<UserInputRequiredParameterMetadata>()?),
            StructType::ProfileConfigInfo => Ok(from.as_any_box().downcast::<ProfileConfigInfo>()?),
            StructType::ClusterProfileConfigInfo => Ok(from.as_any_box().downcast::<ClusterProfileConfigInfo>()?),
            StructType::HostProfileConfigInfo => Ok(from.as_any_box().downcast::<HostProfileConfigInfo>()?),
            StructType::ProfileCreateSpec => Ok(from.as_any_box().downcast::<ProfileCreateSpec>()?),
            StructType::ProfileSerializedCreateSpec => Ok(from.as_any_box().downcast::<ProfileSerializedCreateSpec>()?),
            StructType::HostProfileSerializedHostProfileSpec => Ok(from.as_any_box().downcast::<HostProfileSerializedHostProfileSpec>()?),
            StructType::ClusterProfileCreateSpec => Ok(from.as_any_box().downcast::<ClusterProfileCreateSpec>()?),
            StructType::ClusterProfileConfigSpec => Ok(from.as_any_box().downcast::<ClusterProfileConfigSpec>()?),
            StructType::ClusterProfileCompleteConfigSpec => Ok(from.as_any_box().downcast::<ClusterProfileCompleteConfigSpec>()?),
            StructType::ClusterProfileConfigServiceCreateSpec => Ok(from.as_any_box().downcast::<ClusterProfileConfigServiceCreateSpec>()?),
            StructType::HostProfileConfigSpec => Ok(from.as_any_box().downcast::<HostProfileConfigSpec>()?),
            StructType::HostProfileCompleteConfigSpec => Ok(from.as_any_box().downcast::<HostProfileCompleteConfigSpec>()?),
            StructType::HostProfileHostBasedConfigSpec => Ok(from.as_any_box().downcast::<HostProfileHostBasedConfigSpec>()?),
            StructType::ProfileDescription => Ok(from.as_any_box().downcast::<ProfileDescription>()?),
            StructType::ProfileDescriptionSection => Ok(from.as_any_box().downcast::<ProfileDescriptionSection>()?),
            StructType::ProfileMetadata => Ok(from.as_any_box().downcast::<ProfileMetadata>()?),
            StructType::ProfileMetadataProfileOperationMessage => Ok(from.as_any_box().downcast::<ProfileMetadataProfileOperationMessage>()?),
            StructType::ProfileMetadataProfileSortSpec => Ok(from.as_any_box().downcast::<ProfileMetadataProfileSortSpec>()?),
            StructType::ProfilePropertyPath => Ok(from.as_any_box().downcast::<ProfilePropertyPath>()?),
            StructType::ProfileProfileStructure => Ok(from.as_any_box().downcast::<ProfileProfileStructure>()?),
            StructType::ProfileProfileStructureProperty => Ok(from.as_any_box().downcast::<ProfileProfileStructureProperty>()?),
            StructType::AnswerFile => Ok(from.as_any_box().downcast::<AnswerFile>()?),
            StructType::AnswerFileStatusResult => Ok(from.as_any_box().downcast::<AnswerFileStatusResult>()?),
            StructType::AnswerFileStatusError => Ok(from.as_any_box().downcast::<AnswerFileStatusError>()?),
            StructType::ProfileExecuteResult => Ok(from.as_any_box().downcast::<ProfileExecuteResult>()?),
            StructType::ApplyHostProfileConfigurationSpec => Ok(from.as_any_box().downcast::<ApplyHostProfileConfigurationSpec>()?),
            StructType::ProfileExecuteError => Ok(from.as_any_box().downcast::<ProfileExecuteError>()?),
            StructType::HostProfileValidationFailureInfo => Ok(from.as_any_box().downcast::<HostProfileValidationFailureInfo>()?),
            StructType::HostSpecification => Ok(from.as_any_box().downcast::<HostSpecification>()?),
            StructType::HostSubSpecification => Ok(from.as_any_box().downcast::<HostSubSpecification>()?),
            StructType::AnswerFileCreateSpec => Ok(from.as_any_box().downcast::<AnswerFileCreateSpec>()?),
            StructType::AnswerFileOptionsCreateSpec => Ok(from.as_any_box().downcast::<AnswerFileOptionsCreateSpec>()?),
            StructType::AnswerFileSerializedCreateSpec => Ok(from.as_any_box().downcast::<AnswerFileSerializedCreateSpec>()?),
            StructType::ApplyHostProfileConfigurationResult => Ok(from.as_any_box().downcast::<ApplyHostProfileConfigurationResult>()?),
            StructType::HostProfileManagerCompositionResult => Ok(from.as_any_box().downcast::<HostProfileManagerCompositionResult>()?),
            StructType::HostProfileManagerCompositionResultResultElement => Ok(from.as_any_box().downcast::<HostProfileManagerCompositionResultResultElement>()?),
            StructType::HostProfileManagerCompositionValidationResult => Ok(from.as_any_box().downcast::<HostProfileManagerCompositionValidationResult>()?),
            StructType::HostProfileManagerCompositionValidationResultResultElement => Ok(from.as_any_box().downcast::<HostProfileManagerCompositionValidationResultResultElement>()?),
            StructType::HostProfileManagerConfigTaskList => Ok(from.as_any_box().downcast::<HostProfileManagerConfigTaskList>()?),
            StructType::HostProfilesEntityCustomizations => Ok(from.as_any_box().downcast::<HostProfilesEntityCustomizations>()?),
            StructType::StructuredCustomizations => Ok(from.as_any_box().downcast::<StructuredCustomizations>()?),
            StructType::HostProfileManagerHostToConfigSpecMap => Ok(from.as_any_box().downcast::<HostProfileManagerHostToConfigSpecMap>()?),
            StructType::ScheduledTaskDescription => Ok(from.as_any_box().downcast::<ScheduledTaskDescription>()?),
            StructType::ScheduledTaskSpec => Ok(from.as_any_box().downcast::<ScheduledTaskSpec>()?),
            StructType::ScheduledTaskInfo => Ok(from.as_any_box().downcast::<ScheduledTaskInfo>()?),
            StructType::TaskScheduler => Ok(from.as_any_box().downcast::<TaskScheduler>()?),
            StructType::AfterStartupTaskScheduler => Ok(from.as_any_box().downcast::<AfterStartupTaskScheduler>()?),
            StructType::OnceTaskScheduler => Ok(from.as_any_box().downcast::<OnceTaskScheduler>()?),
            StructType::RecurrentTaskScheduler => Ok(from.as_any_box().downcast::<RecurrentTaskScheduler>()?),
            StructType::HourlyTaskScheduler => Ok(from.as_any_box().downcast::<HourlyTaskScheduler>()?),
            StructType::DailyTaskScheduler => Ok(from.as_any_box().downcast::<DailyTaskScheduler>()?),
            StructType::MonthlyTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyTaskScheduler>()?),
            StructType::MonthlyByDayTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyByDayTaskScheduler>()?),
            StructType::MonthlyByWeekdayTaskScheduler => Ok(from.as_any_box().downcast::<MonthlyByWeekdayTaskScheduler>()?),
            StructType::WeeklyTaskScheduler => Ok(from.as_any_box().downcast::<WeeklyTaskScheduler>()?),
            StructType::ApplyStorageRecommendationResult => Ok(from.as_any_box().downcast::<ApplyStorageRecommendationResult>()?),
            StructType::StorageDrsAutomationConfig => Ok(from.as_any_box().downcast::<StorageDrsAutomationConfig>()?),
            StructType::StorageDrsConfigInfo => Ok(from.as_any_box().downcast::<StorageDrsConfigInfo>()?),
            StructType::StorageDrsConfigSpec => Ok(from.as_any_box().downcast::<StorageDrsConfigSpec>()?),
            StructType::StorageDrsIoLoadBalanceConfig => Ok(from.as_any_box().downcast::<StorageDrsIoLoadBalanceConfig>()?),
            StructType::PlacementAffinityRule => Ok(from.as_any_box().downcast::<PlacementAffinityRule>()?),
            StructType::PlacementRankResult => Ok(from.as_any_box().downcast::<PlacementRankResult>()?),
            StructType::PlacementRankSpec => Ok(from.as_any_box().downcast::<PlacementRankSpec>()?),
            StructType::StorageDrsPlacementRankVmSpec => Ok(from.as_any_box().downcast::<StorageDrsPlacementRankVmSpec>()?),
            StructType::StorageDrsPodConfigInfo => Ok(from.as_any_box().downcast::<StorageDrsPodConfigInfo>()?),
            StructType::StorageDrsPodConfigSpec => Ok(from.as_any_box().downcast::<StorageDrsPodConfigSpec>()?),
            StructType::StorageDrsPodSelectionSpec => Ok(from.as_any_box().downcast::<StorageDrsPodSelectionSpec>()?),
            StructType::PodDiskLocator => Ok(from.as_any_box().downcast::<PodDiskLocator>()?),
            StructType::VmPodConfigForPlacement => Ok(from.as_any_box().downcast::<VmPodConfigForPlacement>()?),
            StructType::StorageDrsSpaceLoadBalanceConfig => Ok(from.as_any_box().downcast::<StorageDrsSpaceLoadBalanceConfig>()?),
            StructType::StoragePlacementResult => Ok(from.as_any_box().downcast::<StoragePlacementResult>()?),
            StructType::StoragePlacementSpec => Ok(from.as_any_box().downcast::<StoragePlacementSpec>()?),
            StructType::StorageDrsVmConfigInfo => Ok(from.as_any_box().downcast::<StorageDrsVmConfigInfo>()?),
            StructType::VAppCloneSpec => Ok(from.as_any_box().downcast::<VAppCloneSpec>()?),
            StructType::VAppCloneSpecNetworkMappingPair => Ok(from.as_any_box().downcast::<VAppCloneSpecNetworkMappingPair>()?),
            StructType::VAppCloneSpecResourceMap => Ok(from.as_any_box().downcast::<VAppCloneSpecResourceMap>()?),
            StructType::VAppEntityConfigInfo => Ok(from.as_any_box().downcast::<VAppEntityConfigInfo>()?),
            StructType::VAppIpAssignmentInfo => Ok(from.as_any_box().downcast::<VAppIpAssignmentInfo>()?),
            StructType::IpPool => Ok(from.as_any_box().downcast::<IpPool>()?),
            StructType::IpPoolAssociation => Ok(from.as_any_box().downcast::<IpPoolAssociation>()?),
            StructType::IpPoolIpPoolConfigInfo => Ok(from.as_any_box().downcast::<IpPoolIpPoolConfigInfo>()?),
            StructType::VAppOvfSectionInfo => Ok(from.as_any_box().downcast::<VAppOvfSectionInfo>()?),
            StructType::VAppProductInfo => Ok(from.as_any_box().downcast::<VAppProductInfo>()?),
            StructType::VAppPropertyInfo => Ok(from.as_any_box().downcast::<VAppPropertyInfo>()?),
            StructType::VmConfigInfo => Ok(from.as_any_box().downcast::<VmConfigInfo>()?),
            StructType::VAppConfigInfo => Ok(from.as_any_box().downcast::<VAppConfigInfo>()?),
            StructType::VmConfigSpec => Ok(from.as_any_box().downcast::<VmConfigSpec>()?),
            StructType::VAppConfigSpec => Ok(from.as_any_box().downcast::<VAppConfigSpec>()?),
            StructType::ClusterNetworkConfigSpec => Ok(from.as_any_box().downcast::<ClusterNetworkConfigSpec>()?),
            StructType::FailoverNodeInfo => Ok(from.as_any_box().downcast::<FailoverNodeInfo>()?),
            StructType::NodeDeploymentSpec => Ok(from.as_any_box().downcast::<NodeDeploymentSpec>()?),
            StructType::PassiveNodeDeploymentSpec => Ok(from.as_any_box().downcast::<PassiveNodeDeploymentSpec>()?),
            StructType::NodeNetworkSpec => Ok(from.as_any_box().downcast::<NodeNetworkSpec>()?),
            StructType::PassiveNodeNetworkSpec => Ok(from.as_any_box().downcast::<PassiveNodeNetworkSpec>()?),
            StructType::SourceNodeSpec => Ok(from.as_any_box().downcast::<SourceNodeSpec>()?),
            StructType::VchaClusterConfigInfo => Ok(from.as_any_box().downcast::<VchaClusterConfigInfo>()?),
            StructType::VchaClusterConfigSpec => Ok(from.as_any_box().downcast::<VchaClusterConfigSpec>()?),
            StructType::VchaClusterDeploymentSpec => Ok(from.as_any_box().downcast::<VchaClusterDeploymentSpec>()?),
            StructType::VchaClusterNetworkSpec => Ok(from.as_any_box().downcast::<VchaClusterNetworkSpec>()?),
            StructType::WitnessNodeInfo => Ok(from.as_any_box().downcast::<WitnessNodeInfo>()?),
            StructType::VchaClusterHealth => Ok(from.as_any_box().downcast::<VchaClusterHealth>()?),
            StructType::VchaClusterRuntimeInfo => Ok(from.as_any_box().downcast::<VchaClusterRuntimeInfo>()?),
            StructType::VchaNodeRuntimeInfo => Ok(from.as_any_box().downcast::<VchaNodeRuntimeInfo>()?),
            StructType::VirtualMachineAffinityInfo => Ok(from.as_any_box().downcast::<VirtualMachineAffinityInfo>()?),
            StructType::VirtualMachineBaseIndependentFilterSpec => Ok(from.as_any_box().downcast::<VirtualMachineBaseIndependentFilterSpec>()?),
            StructType::VirtualMachineEmptyIndependentFilterSpec => Ok(from.as_any_box().downcast::<VirtualMachineEmptyIndependentFilterSpec>()?),
            StructType::VirtualMachineIndependentFilterSpec => Ok(from.as_any_box().downcast::<VirtualMachineIndependentFilterSpec>()?),
            StructType::VirtualMachineBootOptions => Ok(from.as_any_box().downcast::<VirtualMachineBootOptions>()?),
            StructType::VirtualMachineBootOptionsBootableDevice => Ok(from.as_any_box().downcast::<VirtualMachineBootOptionsBootableDevice>()?),
            StructType::VirtualMachineBootOptionsBootableCdromDevice => Ok(from.as_any_box().downcast::<VirtualMachineBootOptionsBootableCdromDevice>()?),
            StructType::VirtualMachineBootOptionsBootableDiskDevice => Ok(from.as_any_box().downcast::<VirtualMachineBootOptionsBootableDiskDevice>()?),
            StructType::VirtualMachineBootOptionsBootableEthernetDevice => Ok(from.as_any_box().downcast::<VirtualMachineBootOptionsBootableEthernetDevice>()?),
            StructType::VirtualMachineBootOptionsBootableFloppyDevice => Ok(from.as_any_box().downcast::<VirtualMachineBootOptionsBootableFloppyDevice>()?),
            StructType::VirtualMachineCapability => Ok(from.as_any_box().downcast::<VirtualMachineCapability>()?),
            StructType::VirtualMachineCertThumbprint => Ok(from.as_any_box().downcast::<VirtualMachineCertThumbprint>()?),
            StructType::VirtualMachineCloneSpec => Ok(from.as_any_box().downcast::<VirtualMachineCloneSpec>()?),
            StructType::VirtualMachineConfigInfo => Ok(from.as_any_box().downcast::<VirtualMachineConfigInfo>()?),
            StructType::VirtualMachineConfigInfoDatastoreUrlPair => Ok(from.as_any_box().downcast::<VirtualMachineConfigInfoDatastoreUrlPair>()?),
            StructType::VirtualMachineConfigInfoOverheadInfo => Ok(from.as_any_box().downcast::<VirtualMachineConfigInfoOverheadInfo>()?),
            StructType::VirtualMachineConfigOption => Ok(from.as_any_box().downcast::<VirtualMachineConfigOption>()?),
            StructType::VirtualMachineConfigOptionDescriptor => Ok(from.as_any_box().downcast::<VirtualMachineConfigOptionDescriptor>()?),
            StructType::VirtualMachineConfigSpec => Ok(from.as_any_box().downcast::<VirtualMachineConfigSpec>()?),
            StructType::ConfigTarget => Ok(from.as_any_box().downcast::<ConfigTarget>()?),
            StructType::VirtualMachineConsolePreferences => Ok(from.as_any_box().downcast::<VirtualMachineConsolePreferences>()?),
            StructType::VirtualMachineContentLibraryItemInfo => Ok(from.as_any_box().downcast::<VirtualMachineContentLibraryItemInfo>()?),
            StructType::DatastoreOption => Ok(from.as_any_box().downcast::<DatastoreOption>()?),
            StructType::VirtualMachineDatastoreVolumeOption => Ok(from.as_any_box().downcast::<VirtualMachineDatastoreVolumeOption>()?),
            StructType::VirtualMachineDefaultPowerOpInfo => Ok(from.as_any_box().downcast::<VirtualMachineDefaultPowerOpInfo>()?),
            StructType::VirtualMachineDeviceRuntimeInfo => Ok(from.as_any_box().downcast::<VirtualMachineDeviceRuntimeInfo>()?),
            StructType::VirtualMachineDeviceRuntimeInfoDeviceRuntimeState => Ok(from.as_any_box().downcast::<VirtualMachineDeviceRuntimeInfoDeviceRuntimeState>()?),
            StructType::VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState => Ok(from.as_any_box().downcast::<VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState>()?),
            StructType::VirtualMachineDvxClassInfo => Ok(from.as_any_box().downcast::<VirtualMachineDvxClassInfo>()?),
            StructType::FaultToleranceConfigInfo => Ok(from.as_any_box().downcast::<FaultToleranceConfigInfo>()?),
            StructType::FaultTolerancePrimaryConfigInfo => Ok(from.as_any_box().downcast::<FaultTolerancePrimaryConfigInfo>()?),
            StructType::FaultToleranceSecondaryConfigInfo => Ok(from.as_any_box().downcast::<FaultToleranceSecondaryConfigInfo>()?),
            StructType::FaultToleranceConfigSpec => Ok(from.as_any_box().downcast::<FaultToleranceConfigSpec>()?),
            StructType::FaultToleranceMetaSpec => Ok(from.as_any_box().downcast::<FaultToleranceMetaSpec>()?),
            StructType::FaultToleranceSecondaryOpResult => Ok(from.as_any_box().downcast::<FaultToleranceSecondaryOpResult>()?),
            StructType::FaultToleranceVmConfigSpec => Ok(from.as_any_box().downcast::<FaultToleranceVmConfigSpec>()?),
            StructType::FaultToleranceDiskSpec => Ok(from.as_any_box().downcast::<FaultToleranceDiskSpec>()?),
            StructType::VirtualMachineFeatureRequirement => Ok(from.as_any_box().downcast::<VirtualMachineFeatureRequirement>()?),
            StructType::VirtualMachineFileInfo => Ok(from.as_any_box().downcast::<VirtualMachineFileInfo>()?),
            StructType::VirtualMachineFileLayout => Ok(from.as_any_box().downcast::<VirtualMachineFileLayout>()?),
            StructType::VirtualMachineFileLayoutDiskLayout => Ok(from.as_any_box().downcast::<VirtualMachineFileLayoutDiskLayout>()?),
            StructType::VirtualMachineFileLayoutSnapshotLayout => Ok(from.as_any_box().downcast::<VirtualMachineFileLayoutSnapshotLayout>()?),
            StructType::VirtualMachineFileLayoutEx => Ok(from.as_any_box().downcast::<VirtualMachineFileLayoutEx>()?),
            StructType::VirtualMachineFileLayoutExDiskLayout => Ok(from.as_any_box().downcast::<VirtualMachineFileLayoutExDiskLayout>()?),
            StructType::VirtualMachineFileLayoutExDiskUnit => Ok(from.as_any_box().downcast::<VirtualMachineFileLayoutExDiskUnit>()?),
            StructType::VirtualMachineFileLayoutExFileInfo => Ok(from.as_any_box().downcast::<VirtualMachineFileLayoutExFileInfo>()?),
            StructType::VirtualMachineFileLayoutExSnapshotLayout => Ok(from.as_any_box().downcast::<VirtualMachineFileLayoutExSnapshotLayout>()?),
            StructType::VirtualMachineFlagInfo => Ok(from.as_any_box().downcast::<VirtualMachineFlagInfo>()?),
            StructType::VirtualMachineForkConfigInfo => Ok(from.as_any_box().downcast::<VirtualMachineForkConfigInfo>()?),
            StructType::GuestInfo => Ok(from.as_any_box().downcast::<GuestInfo>()?),
            StructType::GuestInfoCustomizationInfo => Ok(from.as_any_box().downcast::<GuestInfoCustomizationInfo>()?),
            StructType::GuestDiskInfo => Ok(from.as_any_box().downcast::<GuestDiskInfo>()?),
            StructType::GuestInfoNamespaceGenerationInfo => Ok(from.as_any_box().downcast::<GuestInfoNamespaceGenerationInfo>()?),
            StructType::GuestNicInfo => Ok(from.as_any_box().downcast::<GuestNicInfo>()?),
            StructType::GuestScreenInfo => Ok(from.as_any_box().downcast::<GuestScreenInfo>()?),
            StructType::GuestStackInfo => Ok(from.as_any_box().downcast::<GuestStackInfo>()?),
            StructType::GuestInfoVirtualDiskMapping => Ok(from.as_any_box().downcast::<GuestInfoVirtualDiskMapping>()?),
            StructType::VirtualMachineGuestIntegrityInfo => Ok(from.as_any_box().downcast::<VirtualMachineGuestIntegrityInfo>()?),
            StructType::VirtualMachineGuestMonitoringModeInfo => Ok(from.as_any_box().downcast::<VirtualMachineGuestMonitoringModeInfo>()?),
            StructType::GuestOsDescriptor => Ok(from.as_any_box().downcast::<GuestOsDescriptor>()?),
            StructType::VirtualMachineGuestQuiesceSpec => Ok(from.as_any_box().downcast::<VirtualMachineGuestQuiesceSpec>()?),
            StructType::VirtualMachineWindowsQuiesceSpec => Ok(from.as_any_box().downcast::<VirtualMachineWindowsQuiesceSpec>()?),
            StructType::VirtualMachineIdeDiskDevicePartitionInfo => Ok(from.as_any_box().downcast::<VirtualMachineIdeDiskDevicePartitionInfo>()?),
            StructType::VirtualMachineInstantCloneSpec => Ok(from.as_any_box().downcast::<VirtualMachineInstantCloneSpec>()?),
            StructType::VirtualMachineLegacyNetworkSwitchInfo => Ok(from.as_any_box().downcast::<VirtualMachineLegacyNetworkSwitchInfo>()?),
            StructType::VirtualMachineMessage => Ok(from.as_any_box().downcast::<VirtualMachineMessage>()?),
            StructType::VirtualMachineMetadataManagerVmMetadata => Ok(from.as_any_box().downcast::<VirtualMachineMetadataManagerVmMetadata>()?),
            StructType::VirtualMachineMetadataManagerVmMetadataInput => Ok(from.as_any_box().downcast::<VirtualMachineMetadataManagerVmMetadataInput>()?),
            StructType::VirtualMachineMetadataManagerVmMetadataOwner => Ok(from.as_any_box().downcast::<VirtualMachineMetadataManagerVmMetadataOwner>()?),
            StructType::VirtualMachineMetadataManagerVmMetadataResult => Ok(from.as_any_box().downcast::<VirtualMachineMetadataManagerVmMetadataResult>()?),
            StructType::VirtualMachineNetworkShaperInfo => Ok(from.as_any_box().downcast::<VirtualMachineNetworkShaperInfo>()?),
            StructType::VirtualMachineProfileDetails => Ok(from.as_any_box().downcast::<VirtualMachineProfileDetails>()?),
            StructType::VirtualMachineProfileDetailsDiskProfileDetails => Ok(from.as_any_box().downcast::<VirtualMachineProfileDetailsDiskProfileDetails>()?),
            StructType::VirtualMachineProfileRawData => Ok(from.as_any_box().downcast::<VirtualMachineProfileRawData>()?),
            StructType::VirtualMachineProfileSpec => Ok(from.as_any_box().downcast::<VirtualMachineProfileSpec>()?),
            StructType::VirtualMachineDefaultProfileSpec => Ok(from.as_any_box().downcast::<VirtualMachineDefaultProfileSpec>()?),
            StructType::VirtualMachineDefinedProfileSpec => Ok(from.as_any_box().downcast::<VirtualMachineDefinedProfileSpec>()?),
            StructType::VirtualMachineEmptyProfileSpec => Ok(from.as_any_box().downcast::<VirtualMachineEmptyProfileSpec>()?),
            StructType::VirtualMachinePropertyRelation => Ok(from.as_any_box().downcast::<VirtualMachinePropertyRelation>()?),
            StructType::VirtualMachineQuestionInfo => Ok(from.as_any_box().downcast::<VirtualMachineQuestionInfo>()?),
            StructType::VirtualMachineRelocateSpec => Ok(from.as_any_box().downcast::<VirtualMachineRelocateSpec>()?),
            StructType::VirtualMachineRelocateSpecDiskLocator => Ok(from.as_any_box().downcast::<VirtualMachineRelocateSpecDiskLocator>()?),
            StructType::VirtualMachineRelocateSpecDiskLocatorBackingSpec => Ok(from.as_any_box().downcast::<VirtualMachineRelocateSpecDiskLocatorBackingSpec>()?),
            StructType::ReplicationConfigSpec => Ok(from.as_any_box().downcast::<ReplicationConfigSpec>()?),
            StructType::ReplicationInfoDiskSettings => Ok(from.as_any_box().downcast::<ReplicationInfoDiskSettings>()?),
            StructType::VirtualMachineRuntimeInfo => Ok(from.as_any_box().downcast::<VirtualMachineRuntimeInfo>()?),
            StructType::VirtualMachineRuntimeInfoDasProtectionState => Ok(from.as_any_box().downcast::<VirtualMachineRuntimeInfoDasProtectionState>()?),
            StructType::ScheduledHardwareUpgradeInfo => Ok(from.as_any_box().downcast::<ScheduledHardwareUpgradeInfo>()?),
            StructType::VirtualMachineSgxInfo => Ok(from.as_any_box().downcast::<VirtualMachineSgxInfo>()?),
            StructType::VirtualMachineSnapshotInfo => Ok(from.as_any_box().downcast::<VirtualMachineSnapshotInfo>()?),
            StructType::VirtualMachineSnapshotTree => Ok(from.as_any_box().downcast::<VirtualMachineSnapshotTree>()?),
            StructType::VirtualMachineSriovDevicePoolInfo => Ok(from.as_any_box().downcast::<VirtualMachineSriovDevicePoolInfo>()?),
            StructType::VirtualMachineSriovNetworkDevicePoolInfo => Ok(from.as_any_box().downcast::<VirtualMachineSriovNetworkDevicePoolInfo>()?),
            StructType::VirtualMachineStorageInfo => Ok(from.as_any_box().downcast::<VirtualMachineStorageInfo>()?),
            StructType::VirtualMachineUsageOnDatastore => Ok(from.as_any_box().downcast::<VirtualMachineUsageOnDatastore>()?),
            StructType::VirtualMachineSummary => Ok(from.as_any_box().downcast::<VirtualMachineSummary>()?),
            StructType::VirtualMachineConfigSummary => Ok(from.as_any_box().downcast::<VirtualMachineConfigSummary>()?),
            StructType::VirtualMachineGuestSummary => Ok(from.as_any_box().downcast::<VirtualMachineGuestSummary>()?),
            StructType::VirtualMachineQuickStats => Ok(from.as_any_box().downcast::<VirtualMachineQuickStats>()?),
            StructType::VirtualMachineQuickStatsMemoryTierStats => Ok(from.as_any_box().downcast::<VirtualMachineQuickStatsMemoryTierStats>()?),
            StructType::VirtualMachineStorageSummary => Ok(from.as_any_box().downcast::<VirtualMachineStorageSummary>()?),
            StructType::VirtualMachineTargetInfo => Ok(from.as_any_box().downcast::<VirtualMachineTargetInfo>()?),
            StructType::VirtualMachineCdromInfo => Ok(from.as_any_box().downcast::<VirtualMachineCdromInfo>()?),
            StructType::VirtualMachineDatastoreInfo => Ok(from.as_any_box().downcast::<VirtualMachineDatastoreInfo>()?),
            StructType::VirtualMachineDiskDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineDiskDeviceInfo>()?),
            StructType::VirtualMachineIdeDiskDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineIdeDiskDeviceInfo>()?),
            StructType::VirtualMachineScsiDiskDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineScsiDiskDeviceInfo>()?),
            StructType::VirtualMachineDynamicPassthroughInfo => Ok(from.as_any_box().downcast::<VirtualMachineDynamicPassthroughInfo>()?),
            StructType::VirtualMachineFloppyInfo => Ok(from.as_any_box().downcast::<VirtualMachineFloppyInfo>()?),
            StructType::VirtualMachineNetworkInfo => Ok(from.as_any_box().downcast::<VirtualMachineNetworkInfo>()?),
            StructType::OpaqueNetworkTargetInfo => Ok(from.as_any_box().downcast::<OpaqueNetworkTargetInfo>()?),
            StructType::VirtualMachineParallelInfo => Ok(from.as_any_box().downcast::<VirtualMachineParallelInfo>()?),
            StructType::VirtualMachinePciPassthroughInfo => Ok(from.as_any_box().downcast::<VirtualMachinePciPassthroughInfo>()?),
            StructType::VirtualMachineSriovInfo => Ok(from.as_any_box().downcast::<VirtualMachineSriovInfo>()?),
            StructType::VirtualMachinePciSharedGpuPassthroughInfo => Ok(from.as_any_box().downcast::<VirtualMachinePciSharedGpuPassthroughInfo>()?),
            StructType::VirtualMachinePrecisionClockInfo => Ok(from.as_any_box().downcast::<VirtualMachinePrecisionClockInfo>()?),
            StructType::VirtualMachineScsiPassthroughInfo => Ok(from.as_any_box().downcast::<VirtualMachineScsiPassthroughInfo>()?),
            StructType::VirtualMachineSerialInfo => Ok(from.as_any_box().downcast::<VirtualMachineSerialInfo>()?),
            StructType::VirtualMachineSgxTargetInfo => Ok(from.as_any_box().downcast::<VirtualMachineSgxTargetInfo>()?),
            StructType::VirtualMachineSoundInfo => Ok(from.as_any_box().downcast::<VirtualMachineSoundInfo>()?),
            StructType::VirtualMachineUsbInfo => Ok(from.as_any_box().downcast::<VirtualMachineUsbInfo>()?),
            StructType::VirtualMachineVFlashModuleInfo => Ok(from.as_any_box().downcast::<VirtualMachineVFlashModuleInfo>()?),
            StructType::VirtualMachineVMotionStunTimeInfo => Ok(from.as_any_box().downcast::<VirtualMachineVMotionStunTimeInfo>()?),
            StructType::VirtualMachineVendorDeviceGroupInfo => Ok(from.as_any_box().downcast::<VirtualMachineVendorDeviceGroupInfo>()?),
            StructType::VirtualMachineVgpuDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineVgpuDeviceInfo>()?),
            StructType::VirtualMachineVgpuProfileInfo => Ok(from.as_any_box().downcast::<VirtualMachineVgpuProfileInfo>()?),
            StructType::ToolsConfigInfo => Ok(from.as_any_box().downcast::<ToolsConfigInfo>()?),
            StructType::ToolsConfigInfoToolsLastInstallInfo => Ok(from.as_any_box().downcast::<ToolsConfigInfoToolsLastInstallInfo>()?),
            StructType::UsbScanCodeSpec => Ok(from.as_any_box().downcast::<UsbScanCodeSpec>()?),
            StructType::UsbScanCodeSpecKeyEvent => Ok(from.as_any_box().downcast::<UsbScanCodeSpecKeyEvent>()?),
            StructType::UsbScanCodeSpecModifierType => Ok(from.as_any_box().downcast::<UsbScanCodeSpecModifierType>()?),
            StructType::VirtualMachineVcpuConfig => Ok(from.as_any_box().downcast::<VirtualMachineVcpuConfig>()?),
            StructType::VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo => Ok(from.as_any_box().downcast::<VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo>()?),
            StructType::VirtualMachineVirtualDeviceGroups => Ok(from.as_any_box().downcast::<VirtualMachineVirtualDeviceGroups>()?),
            StructType::VirtualMachineVirtualDeviceGroupsDeviceGroup => Ok(from.as_any_box().downcast::<VirtualMachineVirtualDeviceGroupsDeviceGroup>()?),
            StructType::VirtualMachineVirtualDeviceGroupsVendorDeviceGroup => Ok(from.as_any_box().downcast::<VirtualMachineVirtualDeviceGroupsVendorDeviceGroup>()?),
            StructType::VirtualMachineVirtualDeviceSwap => Ok(from.as_any_box().downcast::<VirtualMachineVirtualDeviceSwap>()?),
            StructType::VirtualMachineVirtualDeviceSwapDeviceSwapInfo => Ok(from.as_any_box().downcast::<VirtualMachineVirtualDeviceSwapDeviceSwapInfo>()?),
            StructType::VirtualHardware => Ok(from.as_any_box().downcast::<VirtualHardware>()?),
            StructType::VirtualHardwareOption => Ok(from.as_any_box().downcast::<VirtualHardwareOption>()?),
            StructType::VirtualMachineVirtualNuma => Ok(from.as_any_box().downcast::<VirtualMachineVirtualNuma>()?),
            StructType::VirtualMachineVirtualNumaInfo => Ok(from.as_any_box().downcast::<VirtualMachineVirtualNumaInfo>()?),
            StructType::VirtualMachineVirtualPMem => Ok(from.as_any_box().downcast::<VirtualMachineVirtualPMem>()?),
            StructType::CheckResult => Ok(from.as_any_box().downcast::<CheckResult>()?),
            StructType::CustomizationAdapterMapping => Ok(from.as_any_box().downcast::<CustomizationAdapterMapping>()?),
            StructType::CustomizationGlobalIpSettings => Ok(from.as_any_box().downcast::<CustomizationGlobalIpSettings>()?),
            StructType::CustomizationGuiRunOnce => Ok(from.as_any_box().downcast::<CustomizationGuiRunOnce>()?),
            StructType::CustomizationGuiUnattended => Ok(from.as_any_box().downcast::<CustomizationGuiUnattended>()?),
            StructType::CustomizationIpSettings => Ok(from.as_any_box().downcast::<CustomizationIpSettings>()?),
            StructType::CustomizationIpSettingsIpV6AddressSpec => Ok(from.as_any_box().downcast::<CustomizationIpSettingsIpV6AddressSpec>()?),
            StructType::CustomizationIdentification => Ok(from.as_any_box().downcast::<CustomizationIdentification>()?),
            StructType::CustomizationIdentitySettings => Ok(from.as_any_box().downcast::<CustomizationIdentitySettings>()?),
            StructType::CustomizationCloudinitPrep => Ok(from.as_any_box().downcast::<CustomizationCloudinitPrep>()?),
            StructType::CustomizationLinuxPrep => Ok(from.as_any_box().downcast::<CustomizationLinuxPrep>()?),
            StructType::CustomizationSysprep => Ok(from.as_any_box().downcast::<CustomizationSysprep>()?),
            StructType::CustomizationSysprepText => Ok(from.as_any_box().downcast::<CustomizationSysprepText>()?),
            StructType::CustomizationIpGenerator => Ok(from.as_any_box().downcast::<CustomizationIpGenerator>()?),
            StructType::CustomizationCustomIpGenerator => Ok(from.as_any_box().downcast::<CustomizationCustomIpGenerator>()?),
            StructType::CustomizationDhcpIpGenerator => Ok(from.as_any_box().downcast::<CustomizationDhcpIpGenerator>()?),
            StructType::CustomizationFixedIp => Ok(from.as_any_box().downcast::<CustomizationFixedIp>()?),
            StructType::CustomizationUnknownIpGenerator => Ok(from.as_any_box().downcast::<CustomizationUnknownIpGenerator>()?),
            StructType::CustomizationIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationIpV6Generator>()?),
            StructType::CustomizationAutoIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationAutoIpV6Generator>()?),
            StructType::CustomizationCustomIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationCustomIpV6Generator>()?),
            StructType::CustomizationDhcpIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationDhcpIpV6Generator>()?),
            StructType::CustomizationFixedIpV6 => Ok(from.as_any_box().downcast::<CustomizationFixedIpV6>()?),
            StructType::CustomizationStatelessIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationStatelessIpV6Generator>()?),
            StructType::CustomizationUnknownIpV6Generator => Ok(from.as_any_box().downcast::<CustomizationUnknownIpV6Generator>()?),
            StructType::CustomizationLicenseFilePrintData => Ok(from.as_any_box().downcast::<CustomizationLicenseFilePrintData>()?),
            StructType::CustomizationName => Ok(from.as_any_box().downcast::<CustomizationName>()?),
            StructType::CustomizationCustomName => Ok(from.as_any_box().downcast::<CustomizationCustomName>()?),
            StructType::CustomizationFixedName => Ok(from.as_any_box().downcast::<CustomizationFixedName>()?),
            StructType::CustomizationPrefixName => Ok(from.as_any_box().downcast::<CustomizationPrefixName>()?),
            StructType::CustomizationUnknownName => Ok(from.as_any_box().downcast::<CustomizationUnknownName>()?),
            StructType::CustomizationVirtualMachineName => Ok(from.as_any_box().downcast::<CustomizationVirtualMachineName>()?),
            StructType::CustomizationOptions => Ok(from.as_any_box().downcast::<CustomizationOptions>()?),
            StructType::CustomizationLinuxOptions => Ok(from.as_any_box().downcast::<CustomizationLinuxOptions>()?),
            StructType::CustomizationWinOptions => Ok(from.as_any_box().downcast::<CustomizationWinOptions>()?),
            StructType::CustomizationPassword => Ok(from.as_any_box().downcast::<CustomizationPassword>()?),
            StructType::CustomizationSpec => Ok(from.as_any_box().downcast::<CustomizationSpec>()?),
            StructType::CustomizationUserData => Ok(from.as_any_box().downcast::<CustomizationUserData>()?),
            StructType::HostDiskMappingInfo => Ok(from.as_any_box().downcast::<HostDiskMappingInfo>()?),
            StructType::HostDiskMappingPartitionInfo => Ok(from.as_any_box().downcast::<HostDiskMappingPartitionInfo>()?),
            StructType::HostDiskMappingOption => Ok(from.as_any_box().downcast::<HostDiskMappingOption>()?),
            StructType::HostDiskMappingPartitionOption => Ok(from.as_any_box().downcast::<HostDiskMappingPartitionOption>()?),
            StructType::VirtualDevice => Ok(from.as_any_box().downcast::<VirtualDevice>()?),
            StructType::VirtualCdrom => Ok(from.as_any_box().downcast::<VirtualCdrom>()?),
            StructType::VirtualController => Ok(from.as_any_box().downcast::<VirtualController>()?),
            StructType::VirtualIdeController => Ok(from.as_any_box().downcast::<VirtualIdeController>()?),
            StructType::VirtualNvdimmController => Ok(from.as_any_box().downcast::<VirtualNvdimmController>()?),
            StructType::VirtualNvmeController => Ok(from.as_any_box().downcast::<VirtualNvmeController>()?),
            StructType::VirtualPciController => Ok(from.as_any_box().downcast::<VirtualPciController>()?),
            StructType::VirtualPs2Controller => Ok(from.as_any_box().downcast::<VirtualPs2Controller>()?),
            StructType::VirtualSataController => Ok(from.as_any_box().downcast::<VirtualSataController>()?),
            StructType::VirtualAhciController => Ok(from.as_any_box().downcast::<VirtualAhciController>()?),
            StructType::VirtualScsiController => Ok(from.as_any_box().downcast::<VirtualScsiController>()?),
            StructType::ParaVirtualScsiController => Ok(from.as_any_box().downcast::<ParaVirtualScsiController>()?),
            StructType::VirtualBusLogicController => Ok(from.as_any_box().downcast::<VirtualBusLogicController>()?),
            StructType::VirtualLsiLogicController => Ok(from.as_any_box().downcast::<VirtualLsiLogicController>()?),
            StructType::VirtualLsiLogicSasController => Ok(from.as_any_box().downcast::<VirtualLsiLogicSasController>()?),
            StructType::VirtualSioController => Ok(from.as_any_box().downcast::<VirtualSioController>()?),
            StructType::VirtualUsbController => Ok(from.as_any_box().downcast::<VirtualUsbController>()?),
            StructType::VirtualUsbxhciController => Ok(from.as_any_box().downcast::<VirtualUsbxhciController>()?),
            StructType::VirtualDisk => Ok(from.as_any_box().downcast::<VirtualDisk>()?),
            StructType::VirtualEthernetCard => Ok(from.as_any_box().downcast::<VirtualEthernetCard>()?),
            StructType::VirtualE1000 => Ok(from.as_any_box().downcast::<VirtualE1000>()?),
            StructType::VirtualE1000E => Ok(from.as_any_box().downcast::<VirtualE1000E>()?),
            StructType::VirtualPcNet32 => Ok(from.as_any_box().downcast::<VirtualPcNet32>()?),
            StructType::VirtualSriovEthernetCard => Ok(from.as_any_box().downcast::<VirtualSriovEthernetCard>()?),
            StructType::VirtualVmxnet => Ok(from.as_any_box().downcast::<VirtualVmxnet>()?),
            StructType::VirtualVmxnet2 => Ok(from.as_any_box().downcast::<VirtualVmxnet2>()?),
            StructType::VirtualVmxnet3 => Ok(from.as_any_box().downcast::<VirtualVmxnet3>()?),
            StructType::VirtualVmxnet3Vrdma => Ok(from.as_any_box().downcast::<VirtualVmxnet3Vrdma>()?),
            StructType::VirtualFloppy => Ok(from.as_any_box().downcast::<VirtualFloppy>()?),
            StructType::VirtualKeyboard => Ok(from.as_any_box().downcast::<VirtualKeyboard>()?),
            StructType::VirtualNvdimm => Ok(from.as_any_box().downcast::<VirtualNvdimm>()?),
            StructType::VirtualPciPassthrough => Ok(from.as_any_box().downcast::<VirtualPciPassthrough>()?),
            StructType::VirtualParallelPort => Ok(from.as_any_box().downcast::<VirtualParallelPort>()?),
            StructType::VirtualPointingDevice => Ok(from.as_any_box().downcast::<VirtualPointingDevice>()?),
            StructType::VirtualPrecisionClock => Ok(from.as_any_box().downcast::<VirtualPrecisionClock>()?),
            StructType::VirtualScsiPassthrough => Ok(from.as_any_box().downcast::<VirtualScsiPassthrough>()?),
            StructType::VirtualSerialPort => Ok(from.as_any_box().downcast::<VirtualSerialPort>()?),
            StructType::VirtualSoundCard => Ok(from.as_any_box().downcast::<VirtualSoundCard>()?),
            StructType::VirtualEnsoniq1371 => Ok(from.as_any_box().downcast::<VirtualEnsoniq1371>()?),
            StructType::VirtualHdAudioCard => Ok(from.as_any_box().downcast::<VirtualHdAudioCard>()?),
            StructType::VirtualSoundBlaster16 => Ok(from.as_any_box().downcast::<VirtualSoundBlaster16>()?),
            StructType::VirtualTpm => Ok(from.as_any_box().downcast::<VirtualTpm>()?),
            StructType::VirtualUsb => Ok(from.as_any_box().downcast::<VirtualUsb>()?),
            StructType::VirtualMachineVmciDevice => Ok(from.as_any_box().downcast::<VirtualMachineVmciDevice>()?),
            StructType::VirtualMachineVmirom => Ok(from.as_any_box().downcast::<VirtualMachineVmirom>()?),
            StructType::VirtualMachineVideoCard => Ok(from.as_any_box().downcast::<VirtualMachineVideoCard>()?),
            StructType::VirtualWdt => Ok(from.as_any_box().downcast::<VirtualWdt>()?),
            StructType::VirtualDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceBackingInfo>()?),
            StructType::VirtualDeviceDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceDeviceBackingInfo>()?),
            StructType::VirtualCdromAtapiBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromAtapiBackingInfo>()?),
            StructType::VirtualCdromPassthroughBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromPassthroughBackingInfo>()?),
            StructType::VirtualDiskRawDiskVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskRawDiskVer2BackingInfo>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskPartitionedRawDiskVer2BackingInfo>()?),
            StructType::VirtualEthernetCardLegacyNetworkBackingInfo => Ok(from.as_any_box().downcast::<VirtualEthernetCardLegacyNetworkBackingInfo>()?),
            StructType::VirtualEthernetCardNetworkBackingInfo => Ok(from.as_any_box().downcast::<VirtualEthernetCardNetworkBackingInfo>()?),
            StructType::VirtualFloppyDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualFloppyDeviceBackingInfo>()?),
            StructType::VirtualPciPassthroughDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDeviceBackingInfo>()?),
            StructType::VirtualPciPassthroughDynamicBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDynamicBackingInfo>()?),
            StructType::VirtualParallelPortDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualParallelPortDeviceBackingInfo>()?),
            StructType::VirtualPointingDeviceDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualPointingDeviceDeviceBackingInfo>()?),
            StructType::VirtualScsiPassthroughDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualScsiPassthroughDeviceBackingInfo>()?),
            StructType::VirtualSerialPortDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortDeviceBackingInfo>()?),
            StructType::VirtualSoundCardDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualSoundCardDeviceBackingInfo>()?),
            StructType::VirtualUsbRemoteHostBackingInfo => Ok(from.as_any_box().downcast::<VirtualUsbRemoteHostBackingInfo>()?),
            StructType::VirtualUsbusbBackingInfo => Ok(from.as_any_box().downcast::<VirtualUsbusbBackingInfo>()?),
            StructType::VirtualDeviceFileBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceFileBackingInfo>()?),
            StructType::VirtualCdromIsoBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromIsoBackingInfo>()?),
            StructType::VirtualDiskFlatVer1BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer1BackingInfo>()?),
            StructType::VirtualDiskFlatVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer2BackingInfo>()?),
            StructType::VirtualDiskLocalPMemBackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskLocalPMemBackingInfo>()?),
            StructType::VirtualDiskRawDiskMappingVer1BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskRawDiskMappingVer1BackingInfo>()?),
            StructType::VirtualDiskSeSparseBackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskSeSparseBackingInfo>()?),
            StructType::VirtualDiskSparseVer1BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer1BackingInfo>()?),
            StructType::VirtualDiskSparseVer2BackingInfo => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer2BackingInfo>()?),
            StructType::VirtualFloppyImageBackingInfo => Ok(from.as_any_box().downcast::<VirtualFloppyImageBackingInfo>()?),
            StructType::VirtualNvdimmBackingInfo => Ok(from.as_any_box().downcast::<VirtualNvdimmBackingInfo>()?),
            StructType::VirtualParallelPortFileBackingInfo => Ok(from.as_any_box().downcast::<VirtualParallelPortFileBackingInfo>()?),
            StructType::VirtualSerialPortFileBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortFileBackingInfo>()?),
            StructType::VirtualDevicePipeBackingInfo => Ok(from.as_any_box().downcast::<VirtualDevicePipeBackingInfo>()?),
            StructType::VirtualSerialPortPipeBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortPipeBackingInfo>()?),
            StructType::VirtualDeviceRemoteDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceRemoteDeviceBackingInfo>()?),
            StructType::VirtualCdromRemoteAtapiBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromRemoteAtapiBackingInfo>()?),
            StructType::VirtualCdromRemotePassthroughBackingInfo => Ok(from.as_any_box().downcast::<VirtualCdromRemotePassthroughBackingInfo>()?),
            StructType::VirtualFloppyRemoteDeviceBackingInfo => Ok(from.as_any_box().downcast::<VirtualFloppyRemoteDeviceBackingInfo>()?),
            StructType::VirtualUsbRemoteClientBackingInfo => Ok(from.as_any_box().downcast::<VirtualUsbRemoteClientBackingInfo>()?),
            StructType::VirtualDeviceUriBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceUriBackingInfo>()?),
            StructType::VirtualSerialPortUriBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortUriBackingInfo>()?),
            StructType::VirtualEthernetCardDistributedVirtualPortBackingInfo => Ok(from.as_any_box().downcast::<VirtualEthernetCardDistributedVirtualPortBackingInfo>()?),
            StructType::VirtualEthernetCardOpaqueNetworkBackingInfo => Ok(from.as_any_box().downcast::<VirtualEthernetCardOpaqueNetworkBackingInfo>()?),
            StructType::VirtualPciPassthroughDvxBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDvxBackingInfo>()?),
            StructType::VirtualPciPassthroughPluginBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughPluginBackingInfo>()?),
            StructType::VirtualPciPassthroughVmiopBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughVmiopBackingInfo>()?),
            StructType::VirtualPrecisionClockSystemClockBackingInfo => Ok(from.as_any_box().downcast::<VirtualPrecisionClockSystemClockBackingInfo>()?),
            StructType::VirtualSerialPortThinPrintBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortThinPrintBackingInfo>()?),
            StructType::VirtualSriovEthernetCardSriovBackingInfo => Ok(from.as_any_box().downcast::<VirtualSriovEthernetCardSriovBackingInfo>()?),
            StructType::VirtualDeviceBusSlotInfo => Ok(from.as_any_box().downcast::<VirtualDeviceBusSlotInfo>()?),
            StructType::VirtualDevicePciBusSlotInfo => Ok(from.as_any_box().downcast::<VirtualDevicePciBusSlotInfo>()?),
            StructType::VirtualUsbControllerPciBusSlotInfo => Ok(from.as_any_box().downcast::<VirtualUsbControllerPciBusSlotInfo>()?),
            StructType::VirtualDeviceConnectInfo => Ok(from.as_any_box().downcast::<VirtualDeviceConnectInfo>()?),
            StructType::VirtualDeviceDeviceGroupInfo => Ok(from.as_any_box().downcast::<VirtualDeviceDeviceGroupInfo>()?),
            StructType::VirtualDeviceOption => Ok(from.as_any_box().downcast::<VirtualDeviceOption>()?),
            StructType::VirtualCdromOption => Ok(from.as_any_box().downcast::<VirtualCdromOption>()?),
            StructType::VirtualControllerOption => Ok(from.as_any_box().downcast::<VirtualControllerOption>()?),
            StructType::VirtualIdeControllerOption => Ok(from.as_any_box().downcast::<VirtualIdeControllerOption>()?),
            StructType::VirtualNvdimmControllerOption => Ok(from.as_any_box().downcast::<VirtualNvdimmControllerOption>()?),
            StructType::VirtualNvmeControllerOption => Ok(from.as_any_box().downcast::<VirtualNvmeControllerOption>()?),
            StructType::VirtualPciControllerOption => Ok(from.as_any_box().downcast::<VirtualPciControllerOption>()?),
            StructType::VirtualPs2ControllerOption => Ok(from.as_any_box().downcast::<VirtualPs2ControllerOption>()?),
            StructType::VirtualSataControllerOption => Ok(from.as_any_box().downcast::<VirtualSataControllerOption>()?),
            StructType::VirtualAhciControllerOption => Ok(from.as_any_box().downcast::<VirtualAhciControllerOption>()?),
            StructType::VirtualScsiControllerOption => Ok(from.as_any_box().downcast::<VirtualScsiControllerOption>()?),
            StructType::ParaVirtualScsiControllerOption => Ok(from.as_any_box().downcast::<ParaVirtualScsiControllerOption>()?),
            StructType::VirtualBusLogicControllerOption => Ok(from.as_any_box().downcast::<VirtualBusLogicControllerOption>()?),
            StructType::VirtualLsiLogicControllerOption => Ok(from.as_any_box().downcast::<VirtualLsiLogicControllerOption>()?),
            StructType::VirtualLsiLogicSasControllerOption => Ok(from.as_any_box().downcast::<VirtualLsiLogicSasControllerOption>()?),
            StructType::VirtualSioControllerOption => Ok(from.as_any_box().downcast::<VirtualSioControllerOption>()?),
            StructType::VirtualUsbControllerOption => Ok(from.as_any_box().downcast::<VirtualUsbControllerOption>()?),
            StructType::VirtualUsbxhciControllerOption => Ok(from.as_any_box().downcast::<VirtualUsbxhciControllerOption>()?),
            StructType::VirtualDiskOption => Ok(from.as_any_box().downcast::<VirtualDiskOption>()?),
            StructType::VirtualEthernetCardOption => Ok(from.as_any_box().downcast::<VirtualEthernetCardOption>()?),
            StructType::VirtualE1000Option => Ok(from.as_any_box().downcast::<VirtualE1000Option>()?),
            StructType::VirtualE1000EOption => Ok(from.as_any_box().downcast::<VirtualE1000EOption>()?),
            StructType::VirtualPcNet32Option => Ok(from.as_any_box().downcast::<VirtualPcNet32Option>()?),
            StructType::VirtualSriovEthernetCardOption => Ok(from.as_any_box().downcast::<VirtualSriovEthernetCardOption>()?),
            StructType::VirtualVmxnetOption => Ok(from.as_any_box().downcast::<VirtualVmxnetOption>()?),
            StructType::VirtualVmxnet2Option => Ok(from.as_any_box().downcast::<VirtualVmxnet2Option>()?),
            StructType::VirtualVmxnet3Option => Ok(from.as_any_box().downcast::<VirtualVmxnet3Option>()?),
            StructType::VirtualVmxnet3VrdmaOption => Ok(from.as_any_box().downcast::<VirtualVmxnet3VrdmaOption>()?),
            StructType::VirtualFloppyOption => Ok(from.as_any_box().downcast::<VirtualFloppyOption>()?),
            StructType::VirtualKeyboardOption => Ok(from.as_any_box().downcast::<VirtualKeyboardOption>()?),
            StructType::VirtualNvdimmOption => Ok(from.as_any_box().downcast::<VirtualNvdimmOption>()?),
            StructType::VirtualPciPassthroughOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughOption>()?),
            StructType::VirtualParallelPortOption => Ok(from.as_any_box().downcast::<VirtualParallelPortOption>()?),
            StructType::VirtualPointingDeviceOption => Ok(from.as_any_box().downcast::<VirtualPointingDeviceOption>()?),
            StructType::VirtualPrecisionClockOption => Ok(from.as_any_box().downcast::<VirtualPrecisionClockOption>()?),
            StructType::VirtualScsiPassthroughOption => Ok(from.as_any_box().downcast::<VirtualScsiPassthroughOption>()?),
            StructType::VirtualSerialPortOption => Ok(from.as_any_box().downcast::<VirtualSerialPortOption>()?),
            StructType::VirtualSoundCardOption => Ok(from.as_any_box().downcast::<VirtualSoundCardOption>()?),
            StructType::VirtualEnsoniq1371Option => Ok(from.as_any_box().downcast::<VirtualEnsoniq1371Option>()?),
            StructType::VirtualHdAudioCardOption => Ok(from.as_any_box().downcast::<VirtualHdAudioCardOption>()?),
            StructType::VirtualSoundBlaster16Option => Ok(from.as_any_box().downcast::<VirtualSoundBlaster16Option>()?),
            StructType::VirtualTpmOption => Ok(from.as_any_box().downcast::<VirtualTpmOption>()?),
            StructType::VirtualUsbOption => Ok(from.as_any_box().downcast::<VirtualUsbOption>()?),
            StructType::VirtualMachineVmciDeviceOption => Ok(from.as_any_box().downcast::<VirtualMachineVmciDeviceOption>()?),
            StructType::VirtualVmiromOption => Ok(from.as_any_box().downcast::<VirtualVmiromOption>()?),
            StructType::VirtualVideoCardOption => Ok(from.as_any_box().downcast::<VirtualVideoCardOption>()?),
            StructType::VirtualWdtOption => Ok(from.as_any_box().downcast::<VirtualWdtOption>()?),
            StructType::VirtualDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceBackingOption>()?),
            StructType::VirtualDeviceDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceDeviceBackingOption>()?),
            StructType::VirtualCdromAtapiBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromAtapiBackingOption>()?),
            StructType::VirtualCdromPassthroughBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromPassthroughBackingOption>()?),
            StructType::VirtualCdromRemoteAtapiBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromRemoteAtapiBackingOption>()?),
            StructType::VirtualDiskRawDiskMappingVer1BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskRawDiskMappingVer1BackingOption>()?),
            StructType::VirtualDiskRawDiskVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskRawDiskVer2BackingOption>()?),
            StructType::VirtualDiskPartitionedRawDiskVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskPartitionedRawDiskVer2BackingOption>()?),
            StructType::VirtualEthernetCardLegacyNetworkBackingOption => Ok(from.as_any_box().downcast::<VirtualEthernetCardLegacyNetworkBackingOption>()?),
            StructType::VirtualEthernetCardNetworkBackingOption => Ok(from.as_any_box().downcast::<VirtualEthernetCardNetworkBackingOption>()?),
            StructType::VirtualFloppyDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualFloppyDeviceBackingOption>()?),
            StructType::VirtualPciPassthroughDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDeviceBackingOption>()?),
            StructType::VirtualPciPassthroughDynamicBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDynamicBackingOption>()?),
            StructType::VirtualParallelPortDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualParallelPortDeviceBackingOption>()?),
            StructType::VirtualPointingDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualPointingDeviceBackingOption>()?),
            StructType::VirtualScsiPassthroughDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualScsiPassthroughDeviceBackingOption>()?),
            StructType::VirtualSerialPortDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortDeviceBackingOption>()?),
            StructType::VirtualSoundCardDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualSoundCardDeviceBackingOption>()?),
            StructType::VirtualUsbRemoteHostBackingOption => Ok(from.as_any_box().downcast::<VirtualUsbRemoteHostBackingOption>()?),
            StructType::VirtualUsbusbBackingOption => Ok(from.as_any_box().downcast::<VirtualUsbusbBackingOption>()?),
            StructType::VirtualDeviceFileBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceFileBackingOption>()?),
            StructType::VirtualCdromIsoBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromIsoBackingOption>()?),
            StructType::VirtualDiskFlatVer1BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer1BackingOption>()?),
            StructType::VirtualDiskFlatVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskFlatVer2BackingOption>()?),
            StructType::VirtualDiskLocalPMemBackingOption => Ok(from.as_any_box().downcast::<VirtualDiskLocalPMemBackingOption>()?),
            StructType::VirtualDiskSeSparseBackingOption => Ok(from.as_any_box().downcast::<VirtualDiskSeSparseBackingOption>()?),
            StructType::VirtualDiskSparseVer1BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer1BackingOption>()?),
            StructType::VirtualDiskSparseVer2BackingOption => Ok(from.as_any_box().downcast::<VirtualDiskSparseVer2BackingOption>()?),
            StructType::VirtualFloppyImageBackingOption => Ok(from.as_any_box().downcast::<VirtualFloppyImageBackingOption>()?),
            StructType::VirtualParallelPortFileBackingOption => Ok(from.as_any_box().downcast::<VirtualParallelPortFileBackingOption>()?),
            StructType::VirtualSerialPortFileBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortFileBackingOption>()?),
            StructType::VirtualDevicePipeBackingOption => Ok(from.as_any_box().downcast::<VirtualDevicePipeBackingOption>()?),
            StructType::VirtualSerialPortPipeBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortPipeBackingOption>()?),
            StructType::VirtualDeviceRemoteDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceRemoteDeviceBackingOption>()?),
            StructType::VirtualCdromRemotePassthroughBackingOption => Ok(from.as_any_box().downcast::<VirtualCdromRemotePassthroughBackingOption>()?),
            StructType::VirtualFloppyRemoteDeviceBackingOption => Ok(from.as_any_box().downcast::<VirtualFloppyRemoteDeviceBackingOption>()?),
            StructType::VirtualUsbRemoteClientBackingOption => Ok(from.as_any_box().downcast::<VirtualUsbRemoteClientBackingOption>()?),
            StructType::VirtualDeviceUriBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceUriBackingOption>()?),
            StructType::VirtualSerialPortUriBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortUriBackingOption>()?),
            StructType::VirtualEthernetCardDvPortBackingOption => Ok(from.as_any_box().downcast::<VirtualEthernetCardDvPortBackingOption>()?),
            StructType::VirtualEthernetCardOpaqueNetworkBackingOption => Ok(from.as_any_box().downcast::<VirtualEthernetCardOpaqueNetworkBackingOption>()?),
            StructType::VirtualPciPassthroughDvxBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughDvxBackingOption>()?),
            StructType::VirtualPciPassthroughPluginBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughPluginBackingOption>()?),
            StructType::VirtualPciPassthroughVmiopBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughVmiopBackingOption>()?),
            StructType::VirtualPrecisionClockSystemClockBackingOption => Ok(from.as_any_box().downcast::<VirtualPrecisionClockSystemClockBackingOption>()?),
            StructType::VirtualSerialPortThinPrintBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortThinPrintBackingOption>()?),
            StructType::VirtualSriovEthernetCardSriovBackingOption => Ok(from.as_any_box().downcast::<VirtualSriovEthernetCardSriovBackingOption>()?),
            StructType::VirtualDeviceBusSlotOption => Ok(from.as_any_box().downcast::<VirtualDeviceBusSlotOption>()?),
            StructType::VirtualDeviceConnectOption => Ok(from.as_any_box().downcast::<VirtualDeviceConnectOption>()?),
            StructType::VirtualDeviceConfigSpec => Ok(from.as_any_box().downcast::<VirtualDeviceConfigSpec>()?),
            StructType::VirtualDiskConfigSpec => Ok(from.as_any_box().downcast::<VirtualDiskConfigSpec>()?),
            StructType::VirtualDeviceConfigSpecBackingSpec => Ok(from.as_any_box().downcast::<VirtualDeviceConfigSpecBackingSpec>()?),
            StructType::VirtualDiskVFlashCacheConfigInfo => Ok(from.as_any_box().downcast::<VirtualDiskVFlashCacheConfigInfo>()?),
            StructType::VirtualDiskId => Ok(from.as_any_box().downcast::<VirtualDiskId>()?),
            StructType::VirtualDiskDeltaDiskFormatsSupported => Ok(from.as_any_box().downcast::<VirtualDiskDeltaDiskFormatsSupported>()?),
            StructType::VirtualDiskOptionVFlashCacheConfigOption => Ok(from.as_any_box().downcast::<VirtualDiskOptionVFlashCacheConfigOption>()?),
            StructType::VirtualEthernetCardResourceAllocation => Ok(from.as_any_box().downcast::<VirtualEthernetCardResourceAllocation>()?),
            StructType::VirtualPciPassthroughAllowedDevice => Ok(from.as_any_box().downcast::<VirtualPciPassthroughAllowedDevice>()?),
            StructType::VirtualMachineVmciDeviceFilterInfo => Ok(from.as_any_box().downcast::<VirtualMachineVmciDeviceFilterInfo>()?),
            StructType::VirtualMachineVmciDeviceFilterSpec => Ok(from.as_any_box().downcast::<VirtualMachineVmciDeviceFilterSpec>()?),
            StructType::VirtualMachineVmciDeviceOptionFilterSpecOption => Ok(from.as_any_box().downcast::<VirtualMachineVmciDeviceOptionFilterSpecOption>()?),
            StructType::GuestAliases => Ok(from.as_any_box().downcast::<GuestAliases>()?),
            StructType::GuestAuthAliasInfo => Ok(from.as_any_box().downcast::<GuestAuthAliasInfo>()?),
            StructType::GuestAuthSubject => Ok(from.as_any_box().downcast::<GuestAuthSubject>()?),
            StructType::GuestAuthAnySubject => Ok(from.as_any_box().downcast::<GuestAuthAnySubject>()?),
            StructType::GuestAuthNamedSubject => Ok(from.as_any_box().downcast::<GuestAuthNamedSubject>()?),
            StructType::GuestMappedAliases => Ok(from.as_any_box().downcast::<GuestMappedAliases>()?),
            StructType::GuestFileAttributes => Ok(from.as_any_box().downcast::<GuestFileAttributes>()?),
            StructType::GuestPosixFileAttributes => Ok(from.as_any_box().downcast::<GuestPosixFileAttributes>()?),
            StructType::GuestWindowsFileAttributes => Ok(from.as_any_box().downcast::<GuestWindowsFileAttributes>()?),
            StructType::GuestFileInfo => Ok(from.as_any_box().downcast::<GuestFileInfo>()?),
            StructType::FileTransferInformation => Ok(from.as_any_box().downcast::<FileTransferInformation>()?),
            StructType::GuestListFileInfo => Ok(from.as_any_box().downcast::<GuestListFileInfo>()?),
            StructType::GuestAuthentication => Ok(from.as_any_box().downcast::<GuestAuthentication>()?),
            StructType::NamePasswordAuthentication => Ok(from.as_any_box().downcast::<NamePasswordAuthentication>()?),
            StructType::SamlTokenAuthentication => Ok(from.as_any_box().downcast::<SamlTokenAuthentication>()?),
            StructType::SspiAuthentication => Ok(from.as_any_box().downcast::<SspiAuthentication>()?),
            StructType::TicketedSessionAuthentication => Ok(from.as_any_box().downcast::<TicketedSessionAuthentication>()?),
            StructType::GuestProcessInfo => Ok(from.as_any_box().downcast::<GuestProcessInfo>()?),
            StructType::GuestProgramSpec => Ok(from.as_any_box().downcast::<GuestProgramSpec>()?),
            StructType::GuestWindowsProgramSpec => Ok(from.as_any_box().downcast::<GuestWindowsProgramSpec>()?),
            StructType::GuestRegKeySpec => Ok(from.as_any_box().downcast::<GuestRegKeySpec>()?),
            StructType::GuestRegKeyNameSpec => Ok(from.as_any_box().downcast::<GuestRegKeyNameSpec>()?),
            StructType::GuestRegKeyRecordSpec => Ok(from.as_any_box().downcast::<GuestRegKeyRecordSpec>()?),
            StructType::GuestRegValueSpec => Ok(from.as_any_box().downcast::<GuestRegValueSpec>()?),
            StructType::GuestRegValueDataSpec => Ok(from.as_any_box().downcast::<GuestRegValueDataSpec>()?),
            StructType::GuestRegValueBinarySpec => Ok(from.as_any_box().downcast::<GuestRegValueBinarySpec>()?),
            StructType::GuestRegValueDwordSpec => Ok(from.as_any_box().downcast::<GuestRegValueDwordSpec>()?),
            StructType::GuestRegValueExpandStringSpec => Ok(from.as_any_box().downcast::<GuestRegValueExpandStringSpec>()?),
            StructType::GuestRegValueMultiStringSpec => Ok(from.as_any_box().downcast::<GuestRegValueMultiStringSpec>()?),
            StructType::GuestRegValueQwordSpec => Ok(from.as_any_box().downcast::<GuestRegValueQwordSpec>()?),
            StructType::GuestRegValueStringSpec => Ok(from.as_any_box().downcast::<GuestRegValueStringSpec>()?),
            StructType::GuestRegValueNameSpec => Ok(from.as_any_box().downcast::<GuestRegValueNameSpec>()?),
            StructType::DeviceGroupId => Ok(from.as_any_box().downcast::<DeviceGroupId>()?),
            StructType::FaultDomainId => Ok(from.as_any_box().downcast::<FaultDomainId>()?),
            StructType::ReplicationGroupId => Ok(from.as_any_box().downcast::<ReplicationGroupId>()?),
            StructType::ReplicationSpec => Ok(from.as_any_box().downcast::<ReplicationSpec>()?),
            StructType::VsanClusterConfigInfo => Ok(from.as_any_box().downcast::<VsanClusterConfigInfo>()?),
            StructType::VsanClusterConfigInfoHostDefaultInfo => Ok(from.as_any_box().downcast::<VsanClusterConfigInfoHostDefaultInfo>()?),
            StructType::VsanHostClusterStatus => Ok(from.as_any_box().downcast::<VsanHostClusterStatus>()?),
            StructType::VsanHostClusterStatusState => Ok(from.as_any_box().downcast::<VsanHostClusterStatusState>()?),
            StructType::VsanHostClusterStatusStateCompletionEstimate => Ok(from.as_any_box().downcast::<VsanHostClusterStatusStateCompletionEstimate>()?),
            StructType::VsanHostConfigInfo => Ok(from.as_any_box().downcast::<VsanHostConfigInfo>()?),
            StructType::VsanHostConfigInfoClusterInfo => Ok(from.as_any_box().downcast::<VsanHostConfigInfoClusterInfo>()?),
            StructType::VsanHostFaultDomainInfo => Ok(from.as_any_box().downcast::<VsanHostFaultDomainInfo>()?),
            StructType::VsanHostConfigInfoNetworkInfo => Ok(from.as_any_box().downcast::<VsanHostConfigInfoNetworkInfo>()?),
            StructType::VsanHostConfigInfoNetworkInfoPortConfig => Ok(from.as_any_box().downcast::<VsanHostConfigInfoNetworkInfoPortConfig>()?),
            StructType::VsanHostConfigInfoStorageInfo => Ok(from.as_any_box().downcast::<VsanHostConfigInfoStorageInfo>()?),
            StructType::VsanHostDecommissionMode => Ok(from.as_any_box().downcast::<VsanHostDecommissionMode>()?),
            StructType::VsanHostDiskMapInfo => Ok(from.as_any_box().downcast::<VsanHostDiskMapInfo>()?),
            StructType::VsanHostDiskMapResult => Ok(from.as_any_box().downcast::<VsanHostDiskMapResult>()?),
            StructType::VsanHostDiskMapping => Ok(from.as_any_box().downcast::<VsanHostDiskMapping>()?),
            StructType::VsanHostDiskResult => Ok(from.as_any_box().downcast::<VsanHostDiskResult>()?),
            StructType::VsanHostIpConfig => Ok(from.as_any_box().downcast::<VsanHostIpConfig>()?),
            StructType::VsanHostMembershipInfo => Ok(from.as_any_box().downcast::<VsanHostMembershipInfo>()?),
            StructType::VsanHostVsanDiskInfo => Ok(from.as_any_box().downcast::<VsanHostVsanDiskInfo>()?),
            StructType::VsanHostRuntimeInfo => Ok(from.as_any_box().downcast::<VsanHostRuntimeInfo>()?),
            StructType::VsanHostRuntimeInfoDiskIssue => Ok(from.as_any_box().downcast::<VsanHostRuntimeInfoDiskIssue>()?),
            StructType::BaseConfigInfo => Ok(from.as_any_box().downcast::<BaseConfigInfo>()?),
            StructType::VStorageObjectConfigInfo => Ok(from.as_any_box().downcast::<VStorageObjectConfigInfo>()?),
            StructType::BaseConfigInfoBackingInfo => Ok(from.as_any_box().downcast::<BaseConfigInfoBackingInfo>()?),
            StructType::BaseConfigInfoFileBackingInfo => Ok(from.as_any_box().downcast::<BaseConfigInfoFileBackingInfo>()?),
            StructType::BaseConfigInfoDiskFileBackingInfo => Ok(from.as_any_box().downcast::<BaseConfigInfoDiskFileBackingInfo>()?),
            StructType::BaseConfigInfoRawDiskMappingBackingInfo => Ok(from.as_any_box().downcast::<BaseConfigInfoRawDiskMappingBackingInfo>()?),
            StructType::VslmCreateSpec => Ok(from.as_any_box().downcast::<VslmCreateSpec>()?),
            StructType::VslmCreateSpecBackingSpec => Ok(from.as_any_box().downcast::<VslmCreateSpecBackingSpec>()?),
            StructType::VslmCreateSpecDiskFileBackingSpec => Ok(from.as_any_box().downcast::<VslmCreateSpecDiskFileBackingSpec>()?),
            StructType::VslmCreateSpecRawDiskMappingBackingSpec => Ok(from.as_any_box().downcast::<VslmCreateSpecRawDiskMappingBackingSpec>()?),
            StructType::DiskCryptoSpec => Ok(from.as_any_box().downcast::<DiskCryptoSpec>()?),
            StructType::Id => Ok(from.as_any_box().downcast::<Id>()?),
            StructType::VslmInfrastructureObjectPolicy => Ok(from.as_any_box().downcast::<VslmInfrastructureObjectPolicy>()?),
            StructType::VslmInfrastructureObjectPolicySpec => Ok(from.as_any_box().downcast::<VslmInfrastructureObjectPolicySpec>()?),
            StructType::VslmMigrateSpec => Ok(from.as_any_box().downcast::<VslmMigrateSpec>()?),
            StructType::VslmCloneSpec => Ok(from.as_any_box().downcast::<VslmCloneSpec>()?),
            StructType::VslmRelocateSpec => Ok(from.as_any_box().downcast::<VslmRelocateSpec>()?),
            StructType::VStorageObjectStateInfo => Ok(from.as_any_box().downcast::<VStorageObjectStateInfo>()?),
            StructType::VslmTagEntry => Ok(from.as_any_box().downcast::<VslmTagEntry>()?),
            StructType::VslmVClockInfo => Ok(from.as_any_box().downcast::<VslmVClockInfo>()?),
            StructType::VStorageObject => Ok(from.as_any_box().downcast::<VStorageObject>()?),
            StructType::VStorageObjectSnapshot => Ok(from.as_any_box().downcast::<VStorageObjectSnapshot>()?),
            StructType::VStorageObjectSnapshotDetails => Ok(from.as_any_box().downcast::<VStorageObjectSnapshotDetails>()?),
            StructType::VStorageObjectSnapshotInfo => Ok(from.as_any_box().downcast::<VStorageObjectSnapshotInfo>()?),
            StructType::VStorageObjectSnapshotInfoVStorageObjectSnapshot => Ok(from.as_any_box().downcast::<VStorageObjectSnapshotInfoVStorageObjectSnapshot>()?),
            StructType::RetrieveVStorageObjSpec => Ok(from.as_any_box().downcast::<RetrieveVStorageObjSpec>()?),
            StructType::VStorageObjectAssociations => Ok(from.as_any_box().downcast::<VStorageObjectAssociations>()?),
            StructType::VStorageObjectAssociationsVmDiskAssociations => Ok(from.as_any_box().downcast::<VStorageObjectAssociationsVmDiskAssociations>()?),
            StructType::DynamicArray => Ok(from.as_any_box().downcast::<DynamicArray>()?),
            StructType::DynamicProperty => Ok(from.as_any_box().downcast::<DynamicProperty>()?),
            StructType::KeyAnyValue => Ok(from.as_any_box().downcast::<KeyAnyValue>()?),
            StructType::LocalizableMessage => Ok(from.as_any_box().downcast::<LocalizableMessage>()?),
            StructType::LocalizedMethodFault => Ok(from.as_any_box().downcast::<LocalizedMethodFault>()?),
            StructType::PropertyChange => Ok(from.as_any_box().downcast::<PropertyChange>()?),
            StructType::PropertyFilterSpec => Ok(from.as_any_box().downcast::<PropertyFilterSpec>()?),
            StructType::PropertyFilterUpdate => Ok(from.as_any_box().downcast::<PropertyFilterUpdate>()?),
            StructType::MissingObject => Ok(from.as_any_box().downcast::<MissingObject>()?),
            StructType::MissingProperty => Ok(from.as_any_box().downcast::<MissingProperty>()?),
            StructType::ObjectContent => Ok(from.as_any_box().downcast::<ObjectContent>()?),
            StructType::ObjectSpec => Ok(from.as_any_box().downcast::<ObjectSpec>()?),
            StructType::ObjectUpdate => Ok(from.as_any_box().downcast::<ObjectUpdate>()?),
            StructType::PropertySpec => Ok(from.as_any_box().downcast::<PropertySpec>()?),
            StructType::RetrieveOptions => Ok(from.as_any_box().downcast::<RetrieveOptions>()?),
            StructType::RetrieveResult => Ok(from.as_any_box().downcast::<RetrieveResult>()?),
            StructType::SelectionSpec => Ok(from.as_any_box().downcast::<SelectionSpec>()?),
            StructType::TraversalSpec => Ok(from.as_any_box().downcast::<TraversalSpec>()?),
            StructType::UpdateSet => Ok(from.as_any_box().downcast::<UpdateSet>()?),
            StructType::WaitOptions => Ok(from.as_any_box().downcast::<WaitOptions>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
