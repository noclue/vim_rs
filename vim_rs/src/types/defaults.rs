//! Default trait implementations for vim_rs types.
//! This module is conditionally compiled when the `defaults` feature is enabled.

use super::enums;
use super::structs;
use super::traits;

impl Default for enums::MoTypesEnum {
    fn default() -> Self {
        Self::Alarm
    }
}

impl Default for enums::DpInvalidProtectionReasonEnum {
    fn default() -> Self {
        Self::ProtectionDegraded
    }
}

impl Default for enums::DpMigrationTypeEnum {
    fn default() -> Self {
        Self::ColdMigration
    }
}

impl Default for enums::DpProtectionStatusTypeEnum {
    fn default() -> Self {
        Self::PeProtected
    }
}

impl Default for enums::DpProtectionSupportTypeEnum {
    fn default() -> Self {
        Self::Supported
    }
}

impl Default for enums::DpSnapshotTypeEnum {
    fn default() -> Self {
        Self::CrashConsistent
    }
}

impl Default for enums::DpSyncTypeEnum {
    fn default() -> Self {
        Self::FullSync
    }
}

impl Default for enums::DpVSphereDataProtectionCapabilitiesEnum {
    fn default() -> Self {
        Self::QueryPeInfo
    }
}

impl Default for enums::DpVssBackupContextEnum {
    fn default() -> Self {
        Self::VssBackupContextAuto
    }
}

impl Default for enums::DpVssBackupTypeEnum {
    fn default() -> Self {
        Self::VssBackupTypeFull
    }
}

impl Default for enums::DpCapabilitySupportLevelEnum {
    fn default() -> Self {
        Self::Supported
    }
}

impl Default for enums::DpDrSrmWorkflowEnum {
    fn default() -> Self {
        Self::TestFailover
    }
}

impl Default for enums::AgencyVmPlacementPolicyVmAntiAffinityEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::AgencyVmPlacementPolicyVmDataAffinityEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::AgentConfigInfoAuthenticationSchemeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::AgentConfigInfoOvfDiskProvisioningEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::AgentVmHookVmStateEnum {
    fn default() -> Self {
        Self::Provisioned
    }
}

impl Default for enums::EamObjectRuntimeInfoGoalStateEnum {
    fn default() -> Self {
        Self::Enabled
    }
}

impl Default for enums::EamObjectRuntimeInfoStatusEnum {
    fn default() -> Self {
        Self::Green
    }
}

impl Default for enums::EsxAgentManagerMaintenanceModePolicyEnum {
    fn default() -> Self {
        Self::SingleHost
    }
}

impl Default for enums::HooksHookTypeEnum {
    fn default() -> Self {
        Self::PostProvisioning
    }
}

impl Default for enums::SolutionsInvalidReasonEnum {
    fn default() -> Self {
        Self::InvalidOvfDescriptor
    }
}

impl Default for enums::SolutionsNonComplianceReasonEnum {
    fn default() -> Self {
        Self::Working
    }
}

impl Default for enums::SolutionsVmDeploymentOptimizationEnum {
    fn default() -> Self {
        Self::AllClones
    }
}

impl Default for enums::SolutionsVmDiskProvisioningEnum {
    fn default() -> Self {
        Self::Thin
    }
}

impl Default for enums::SolutionsVmPlacementPolicyEnum {
    fn default() -> Self {
        Self::VmVmAntiAffinity
    }
}

impl Default for enums::PbmLoggingConfigurationComponentEnum {
    fn default() -> Self {
        Self::Pbm
    }
}

impl Default for enums::PbmLoggingConfigurationLogLevelEnum {
    fn default() -> Self {
        Self::Info
    }
}

impl Default for enums::PbmDebugManagerKeystoreNameEnum {
    fn default() -> Self {
        Self::Sms
    }
}

impl Default for enums::PbmObjectTypeEnum {
    fn default() -> Self {
        Self::VirtualMachine
    }
}

impl Default for enums::PbmVvolTypeEnum {
    fn default() -> Self {
        Self::Config
    }
}

impl Default for enums::PbmCapabilityOperatorEnum {
    fn default() -> Self {
        Self::Not
    }
}

impl Default for enums::PbmCapabilitySchemaCapabilityCategoryEnum {
    fn default() -> Self {
        Self::Common
    }
}

impl Default for enums::PbmLineOfServiceInfoLineOfServiceEnumEnum {
    fn default() -> Self {
        Self::Inspection
    }
}

impl Default for enums::PbmBuiltinGenericTypeEnum {
    fn default() -> Self {
        Self::VmwRange
    }
}

impl Default for enums::PbmBuiltinTypeEnum {
    fn default() -> Self {
        Self::XsdLong
    }
}

impl Default for enums::PbmCapabilityTimeUnitTypeEnum {
    fn default() -> Self {
        Self::Seconds
    }
}

impl Default for enums::PbmComplianceStatusEnum {
    fn default() -> Self {
        Self::Compliant
    }
}

impl Default for enums::PbmComplianceResultComplianceTaskStatusEnum {
    fn default() -> Self {
        Self::InProgress
    }
}

impl Default for enums::PbmHealthStatusForEntityEnum {
    fn default() -> Self {
        Self::Red
    }
}

impl Default for enums::PbmAssociateAndApplyPolicyStatusPolicyStatusEnum {
    fn default() -> Self {
        Self::Success
    }
}

impl Default for enums::PbmProfileCategoryEnumEnum {
    fn default() -> Self {
        Self::Requirement
    }
}

impl Default for enums::PbmSystemCreatedProfileTypeEnum {
    fn default() -> Self {
        Self::VsanDefaultProfile
    }
}

impl Default for enums::PbmOperationEnum {
    fn default() -> Self {
        Self::Create
    }
}

impl Default for enums::PbmIofilterInfoFilterTypeEnum {
    fn default() -> Self {
        Self::Inspection
    }
}

impl Default for enums::PbmPolicyAssociationVolumeAllocationTypeEnum {
    fn default() -> Self {
        Self::FullyInitialized
    }
}

impl Default for enums::PbmProfileResourceTypeEnumEnum {
    fn default() -> Self {
        Self::Storage
    }
}

impl Default for enums::PbmVmOperationEnum {
    fn default() -> Self {
        Self::Create
    }
}

impl Default for enums::EntityReferenceEntityTypeEnum {
    fn default() -> Self {
        Self::Datacenter
    }
}

impl Default for enums::SmsTaskStateEnum {
    fn default() -> Self {
        Self::Queued
    }
}

impl Default for enums::VpCategoryEnum {
    fn default() -> Self {
        Self::Internal
    }
}

impl Default for enums::VasaProviderCertificateStatusEnum {
    fn default() -> Self {
        Self::Valid
    }
}

impl Default for enums::ProviderProfileEnum {
    fn default() -> Self {
        Self::ProfileBasedManagement
    }
}

impl Default for enums::VpTypeEnum {
    fn default() -> Self {
        Self::Persistence
    }
}

impl Default for enums::VasaProviderProfileEnum {
    fn default() -> Self {
        Self::BlockDevice
    }
}

impl Default for enums::VasaProviderStatusEnum {
    fn default() -> Self {
        Self::Online
    }
}

impl Default for enums::VasaAuthenticationTypeEnum {
    fn default() -> Self {
        Self::LoginByToken
    }
}

impl Default for enums::SmsAlarmStatusEnum {
    fn default() -> Self {
        Self::Red
    }
}

impl Default for enums::AlarmTypeEnum {
    fn default() -> Self {
        Self::SpaceCapacityAlarm
    }
}

impl Default for enums::BackingStoragePoolTypeEnum {
    fn default() -> Self {
        Self::ThinProvisioningPool
    }
}

impl Default for enums::SmsEntityTypeEnum {
    fn default() -> Self {
        Self::StorageArrayEntity
    }
}

impl Default for enums::BlockDeviceInterfaceEnum {
    fn default() -> Self {
        Self::Fc
    }
}

impl Default for enums::FileSystemInterfaceEnum {
    fn default() -> Self {
        Self::Nfs
    }
}

impl Default for enums::VasaProfileEnum {
    fn default() -> Self {
        Self::BlockDevice
    }
}

impl Default for enums::StorageContainerVvolContainerTypeEnumEnum {
    fn default() -> Self {
        Self::Nfs
    }
}

impl Default for enums::FileSystemInterfaceVersionEnum {
    fn default() -> Self {
        Self::Nfsv30
    }
}

impl Default for enums::ThinProvisioningStatusEnum {
    fn default() -> Self {
        Self::Red
    }
}

impl Default for enums::ReplicationReplicationStateEnum {
    fn default() -> Self {
        Self::Source
    }
}

impl Default for enums::BatchResultResultEnum {
    fn default() -> Self {
        Self::Success
    }
}

impl Default for enums::ClusterComputeResourceHciWorkflowStateEnum {
    fn default() -> Self {
        Self::InProgress
    }
}

impl Default for enums::ClusterComputeResourceVcsHealthStatusEnum {
    fn default() -> Self {
        Self::Healthy
    }
}

impl Default for enums::ComputeResourceHostSpbmLicenseInfoHostSpbmLicenseStateEnum {
    fn default() -> Self {
        Self::Licensed
    }
}

impl Default for enums::ComputeResourceNetworkBootModeEnum {
    fn default() -> Self {
        Self::Bootstrap
    }
}

impl Default for enums::ConfigSpecOperationEnum {
    fn default() -> Self {
        Self::Add
    }
}

impl Default for enums::DatastoreAccessibleEnum {
    fn default() -> Self {
        Self::True
    }
}

impl Default for enums::DatastoreSectorFormatEnum {
    fn default() -> Self {
        Self::Native512
    }
}

impl Default for enums::DatastoreSummaryMaintenanceModeStateEnum {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for enums::DiagnosticManagerLogCreatorEnum {
    fn default() -> Self {
        Self::Vpxd
    }
}

impl Default for enums::DiagnosticManagerLogFormatEnum {
    fn default() -> Self {
        Self::Plain
    }
}

impl Default for enums::DistributedVirtualSwitchHostInfrastructureTrafficClassEnum {
    fn default() -> Self {
        Self::Management
    }
}

impl Default for enums::DistributedVirtualSwitchNetworkResourceControlVersionEnum {
    fn default() -> Self {
        Self::Version2
    }
}

impl Default for enums::DistributedVirtualSwitchNicTeamingPolicyModeEnum {
    fn default() -> Self {
        Self::LoadbalanceIp
    }
}

impl Default for enums::DistributedVirtualSwitchProductSpecOperationTypeEnum {
    fn default() -> Self {
        Self::PreInstall
    }
}

impl Default for enums::DrsInjectorWorkloadCorrelationStateEnum {
    fn default() -> Self {
        Self::Correlated
    }
}

impl Default for enums::FolderDesiredHostStateEnum {
    fn default() -> Self {
        Self::Maintenance
    }
}

impl Default for enums::FolderExternallyManagedFolderTypeEnum {
    fn default() -> Self {
        Self::ProjectRoot
    }
}

impl Default for enums::ReplicationVmStateEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::QuiesceModeEnum {
    fn default() -> Self {
        Self::Application
    }
}

impl Default for enums::HealthUpdateInfoComponentTypeEnum {
    fn default() -> Self {
        Self::Memory
    }
}

impl Default for enums::HostSystemConnectionStateEnum {
    fn default() -> Self {
        Self::Connected
    }
}

impl Default for enums::HostCryptoStateEnum {
    fn default() -> Self {
        Self::Incapable
    }
}

impl Default for enums::HostSystemPowerStateEnum {
    fn default() -> Self {
        Self::PoweredOn
    }
}

impl Default for enums::HostSystemRemediationStateStateEnum {
    fn default() -> Self {
        Self::RemediationReady
    }
}

impl Default for enums::HostStandbyModeEnum {
    fn default() -> Self {
        Self::Entering
    }
}

impl Default for enums::HttpNfcLeaseManifestEntryChecksumTypeEnum {
    fn default() -> Self {
        Self::Sha1
    }
}

impl Default for enums::HttpNfcLeaseModeEnum {
    fn default() -> Self {
        Self::PushOrGet
    }
}

impl Default for enums::HttpNfcLeaseStateEnum {
    fn default() -> Self {
        Self::Initializing
    }
}

impl Default for enums::IoFilterTypeEnum {
    fn default() -> Self {
        Self::Cache
    }
}

impl Default for enums::IoFilterOperationEnum {
    fn default() -> Self {
        Self::Install
    }
}

impl Default for enums::LatencySensitivitySensitivityLevelEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl Default for enums::LicenseFeatureInfoUnitEnum {
    fn default() -> Self {
        Self::Host
    }
}

impl Default for enums::LicenseFeatureInfoSourceRestrictionEnum {
    fn default() -> Self {
        Self::Unrestricted
    }
}

impl Default for enums::LicenseFeatureInfoStateEnum {
    fn default() -> Self {
        Self::Enabled
    }
}

impl Default for enums::HostLicensableResourceKeyEnum {
    fn default() -> Self {
        Self::NumCpuPackages
    }
}

impl Default for enums::LicenseManagerLicenseKeyEnum {
    fn default() -> Self {
        Self::EsxFull
    }
}

impl Default for enums::LicenseManagerStateEnum {
    fn default() -> Self {
        Self::Initializing
    }
}

impl Default for enums::LicenseReservationInfoStateEnum {
    fn default() -> Self {
        Self::NotUsed
    }
}

impl Default for enums::ManagedEntityStatusEnum {
    fn default() -> Self {
        Self::Gray
    }
}

impl Default for enums::OvfConsumerOstNodeTypeEnum {
    fn default() -> Self {
        Self::Envelope
    }
}

impl Default for enums::OvfCreateImportSpecParamsDiskProvisioningTypeEnum {
    fn default() -> Self {
        Self::MonolithicSparse
    }
}

impl Default for enums::PerfSummaryTypeEnum {
    fn default() -> Self {
        Self::Average
    }
}

impl Default for enums::PerfStatsTypeEnum {
    fn default() -> Self {
        Self::Absolute
    }
}

impl Default for enums::PerformanceManagerUnitEnum {
    fn default() -> Self {
        Self::Percent
    }
}

impl Default for enums::PerfFormatEnum {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for enums::PlaceVmsXClusterSpecPlacementTypeEnum {
    fn default() -> Self {
        Self::CreateAndPowerOn
    }
}

impl Default for enums::ResourceConfigSpecScaleSharesBehaviorEnum {
    fn default() -> Self {
        Self::Disabled
    }
}

impl Default for enums::VMotionCompatibilityTypeEnum {
    fn default() -> Self {
        Self::Cpu
    }
}

impl Default for enums::ValidateMigrationTestTypeEnum {
    fn default() -> Self {
        Self::SourceTests
    }
}

impl Default for enums::SessionManagerGenericServiceTicketTicketTypeEnum {
    fn default() -> Self {
        Self::HttpNfcServiceTicket
    }
}

impl Default for enums::SessionManagerHttpServiceRequestSpecMethodEnum {
    fn default() -> Self {
        Self::HttpOptions
    }
}

impl Default for enums::SharesLevelEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl Default for enums::SimpleCommandEncodingEnum {
    fn default() -> Self {
        Self::Csv
    }
}

impl Default for enums::StorageIormThresholdModeEnum {
    fn default() -> Self {
        Self::Automatic
    }
}

impl Default for enums::TaskFilterSpecRecursionOptionEnum {
    fn default() -> Self {
        Self::Self_
    }
}

impl Default for enums::TaskFilterSpecTimeOptionEnum {
    fn default() -> Self {
        Self::QueuedTime
    }
}

impl Default for enums::TaskInfoStateEnum {
    fn default() -> Self {
        Self::Queued
    }
}

impl Default for enums::VirtualAppVAppStateEnum {
    fn default() -> Self {
        Self::Started
    }
}

impl Default for enums::VirtualDiskAdapterTypeEnum {
    fn default() -> Self {
        Self::Ide
    }
}

impl Default for enums::VirtualDiskTypeEnum {
    fn default() -> Self {
        Self::Preallocated
    }
}

impl Default for enums::VirtualMachineAppHeartbeatStatusTypeEnum {
    fn default() -> Self {
        Self::AppStatusGray
    }
}

impl Default for enums::VirtualMachineConnectionStateEnum {
    fn default() -> Self {
        Self::Connected
    }
}

impl Default for enums::VirtualMachineCryptoStateEnum {
    fn default() -> Self {
        Self::Unlocked
    }
}

impl Default for enums::VirtualMachineFaultToleranceStateEnum {
    fn default() -> Self {
        Self::NotConfigured
    }
}

impl Default for enums::VirtualMachineFaultToleranceTypeEnum {
    fn default() -> Self {
        Self::Unset
    }
}

impl Default for enums::VirtualMachineMovePriorityEnum {
    fn default() -> Self {
        Self::LowPriority
    }
}

impl Default for enums::VirtualMachineNeedSecondaryReasonEnum {
    fn default() -> Self {
        Self::Initializing
    }
}

impl Default for enums::VirtualMachinePowerStateEnum {
    fn default() -> Self {
        Self::PoweredOff
    }
}

impl Default for enums::VirtualMachineRecordReplayStateEnum {
    fn default() -> Self {
        Self::Recording
    }
}

impl Default for enums::VirtualMachineTicketTypeEnum {
    fn default() -> Self {
        Self::Mks
    }
}

impl Default for enums::VsanCompositeConstraintConjoinerEnumEnum {
    fn default() -> Self {
        Self::And
    }
}

impl Default for enums::VsanMassCollectorObjectCollectionEnumEnum {
    fn default() -> Self {
        Self::AllHosts
    }
}

impl Default for enums::VsanPropertyConstraintComparatorEnumEnum {
    fn default() -> Self {
        Self::Equals
    }
}

impl Default for enums::VsanUpgradeSystemUpgradeHistoryDiskGroupOpTypeEnum {
    fn default() -> Self {
        Self::Add
    }
}

impl Default for enums::ActionParameterEnum {
    fn default() -> Self {
        Self::TargetName
    }
}

impl Default for enums::AlarmFilterSpecAlarmTypeByEntityEnum {
    fn default() -> Self {
        Self::EntityTypeAll
    }
}

impl Default for enums::AlarmFilterSpecAlarmTypeByTriggerEnum {
    fn default() -> Self {
        Self::TriggerTypeAll
    }
}

impl Default for enums::EventAlarmExpressionComparisonOperatorEnum {
    fn default() -> Self {
        Self::Equals
    }
}

impl Default for enums::MetricAlarmOperatorEnum {
    fn default() -> Self {
        Self::IsAbove
    }
}

impl Default for enums::StateAlarmOperatorEnum {
    fn default() -> Self {
        Self::IsEqual
    }
}

impl Default for enums::ActionTypeEnum {
    fn default() -> Self {
        Self::MigrationV1
    }
}

impl Default for enums::ClusterPowerStatusEnum {
    fn default() -> Self {
        Self::ClusterPoweredOn
    }
}

impl Default for enums::ClusterCryptoConfigInfoCryptoModeEnum {
    fn default() -> Self {
        Self::OnDemand
    }
}

impl Default for enums::ClusterDasAamNodeStateDasStateEnum {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl Default for enums::ClusterDasConfigInfoHbDatastoreCandidateEnum {
    fn default() -> Self {
        Self::UserSelectedDs
    }
}

impl Default for enums::ClusterDasConfigInfoServiceStateEnum {
    fn default() -> Self {
        Self::Disabled
    }
}

impl Default for enums::ClusterDasConfigInfoVmMonitoringStateEnum {
    fn default() -> Self {
        Self::VmMonitoringDisabled
    }
}

impl Default for enums::ClusterDasFdmAvailabilityStateEnum {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl Default for enums::DasVmPriorityEnum {
    fn default() -> Self {
        Self::Disabled
    }
}

impl Default for enums::ClusterDasVmSettingsIsolationResponseEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::ClusterDasVmSettingsRestartPriorityEnum {
    fn default() -> Self {
        Self::Disabled
    }
}

impl Default for enums::DpmBehaviorEnum {
    fn default() -> Self {
        Self::Manual
    }
}

impl Default for enums::DrsBehaviorEnum {
    fn default() -> Self {
        Self::Manual
    }
}

impl Default for enums::DrsRecommendationReasonCodeEnum {
    fn default() -> Self {
        Self::FairnessCpuAvg
    }
}

impl Default for enums::ClusterHostInfraUpdateHaModeActionOperationTypeEnum {
    fn default() -> Self {
        Self::EnterQuarantine
    }
}

impl Default for enums::HostPowerOperationTypeEnum {
    fn default() -> Self {
        Self::PowerOn
    }
}

impl Default for enums::ClusterInfraUpdateHaConfigInfoBehaviorTypeEnum {
    fn default() -> Self {
        Self::Manual
    }
}

impl Default for enums::ClusterInfraUpdateHaConfigInfoRemediationTypeEnum {
    fn default() -> Self {
        Self::QuarantineMode
    }
}

impl Default for enums::PlacementSpecPlacementTypeEnum {
    fn default() -> Self {
        Self::Create
    }
}

impl Default for enums::ClusterPowerOnVmOptionEnum {
    fn default() -> Self {
        Self::OverrideAutomationLevel
    }
}

impl Default for enums::RecommendationReasonCodeEnum {
    fn default() -> Self {
        Self::FairnessCpuAvg
    }
}

impl Default for enums::RecommendationTypeEnum {
    fn default() -> Self {
        Self::V1
    }
}

impl Default for enums::VsanStorageComplianceStatusEnum {
    fn default() -> Self {
        Self::Compliant
    }
}

impl Default for enums::ClusterSystemVMsConfigInfoDeploymentModeEnum {
    fn default() -> Self {
        Self::SystemManaged
    }
}

impl Default for enums::VimClusterVsanStretchedClusterConfigIssueEnumEnum {
    fn default() -> Self {
        Self::ClusterWithoutOneWitnessHost
    }
}

impl Default for enums::ClusterVmComponentProtectionSettingsStorageVmReactionEnum {
    fn default() -> Self {
        Self::Disabled
    }
}

impl Default for enums::ClusterVmComponentProtectionSettingsVmReactionOnApdClearedEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::ClusterVmReadinessReadyConditionEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::VsanBaselinePreferenceTypeEnum {
    fn default() -> Self {
        Self::LatestRelease
    }
}

impl Default for enums::VsanCapabilityStatusEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::VsanCapabilityTypeEnum {
    fn default() -> Self {
        Self::Capability
    }
}

impl Default for enums::VsanCapabilityType90Enum {
    fn default() -> Self {
        Self::Vsanreplication
    }
}

impl Default for enums::VsanClusterHealthActionIdEnumEnum {
    fn default() -> Self {
        Self::RepairClusterObjectsAction
    }
}

impl Default for enums::VsanClusterHealthCategoryEnumEnum {
    fn default() -> Self {
        Self::DataAvailability
    }
}

impl Default for enums::VsanDatastoreTypeEnum {
    fn default() -> Self {
        Self::Vsan
    }
}

impl Default for enums::VimClusterVsanDiskGroupCreationTypeEnum {
    fn default() -> Self {
        Self::Allflash
    }
}

impl Default for enums::VsanIoInsightInstanceStateEnum {
    fn default() -> Self {
        Self::Running
    }
}

impl Default for enums::VsanIscsiLunStatusEnum {
    fn default() -> Self {
        Self::Online
    }
}

impl Default for enums::VsanIscsiLunRuntimeStatusTypeEnum {
    fn default() -> Self {
        Self::Online
    }
}

impl Default for enums::VsanIscsiTargetAuthTypeEnum {
    fn default() -> Self {
        Self::NoAuth
    }
}

impl Default for enums::VsanIscsiTargetServiceProcessStatusEnum {
    fn default() -> Self {
        Self::Running
    }
}

impl Default for enums::VsanObjectTypeEnumEnum {
    fn default() -> Self {
        Self::Vmswap
    }
}

impl Default for enums::VsanObjectTypeEnum90Enum {
    fn default() -> Self {
        Self::DedupSharedUserData
    }
}

impl Default for enums::VsanPerfDiagnosticQueryTypeEnum {
    fn default() -> Self {
        Self::Tput
    }
}

impl Default for enums::VsanPerfStatsUnitTypeEnum {
    fn default() -> Self {
        Self::Number
    }
}

impl Default for enums::VsanPerfStatsTypeEnum {
    fn default() -> Self {
        Self::Absolute
    }
}

impl Default for enums::VsanPerfSummaryTypeEnum {
    fn default() -> Self {
        Self::Average
    }
}

impl Default for enums::VsanPerfThresholdDirectionTypeEnum {
    fn default() -> Self {
        Self::Upper
    }
}

impl Default for enums::VsanRelayoutObjectsErrorCodeEnum {
    fn default() -> Self {
        Self::OutOfResources
    }
}

impl Default for enums::VsanSpaceReportingEntityTypeEnum {
    fn default() -> Self {
        Self::Vm
    }
}

impl Default for enums::VsanHealthLogLevelEnumEnum {
    fn default() -> Self {
        Self::Info
    }
}

impl Default for enums::CnsClusterFlavorEnum {
    fn default() -> Self {
        Self::Vanilla
    }
}

impl Default for enums::CnsClusterTypeEnum {
    fn default() -> Self {
        Self::Kubernetes
    }
}

impl Default for enums::CnsKubernetesEntityTypeEnum {
    fn default() -> Self {
        Self::PersistentVolume
    }
}

impl Default for enums::MetricFormatEnum {
    fn default() -> Self {
        Self::Prometheus
    }
}

impl Default for enums::MetricTypeEnum {
    fn default() -> Self {
        Self::Volume
    }
}

impl Default for enums::QuerySelectionNameTypeEnum {
    fn default() -> Self {
        Self::VolumeMetadata
    }
}

impl Default for enums::CnsVolumeTypeEnum {
    fn default() -> Self {
        Self::Block
    }
}

impl Default for enums::DvsFilterOnFailureEnum {
    fn default() -> Self {
        Self::FailOpen
    }
}

impl Default for enums::DvPortStatusVmDirectPathGen2InactiveReasonNetworkEnum {
    fn default() -> Self {
        Self::PortNptIncompatibleDvs
    }
}

impl Default for enums::DvPortStatusVmDirectPathGen2InactiveReasonOtherEnum {
    fn default() -> Self {
        Self::PortNptIncompatibleHost
    }
}

impl Default for enums::DistributedVirtualPortgroupBackingTypeEnum {
    fn default() -> Self {
        Self::Standard
    }
}

impl Default for enums::DistributedVirtualPortgroupMetaTagNameEnum {
    fn default() -> Self {
        Self::DvsName
    }
}

impl Default for enums::DistributedVirtualPortgroupPortgroupTypeEnum {
    fn default() -> Self {
        Self::EarlyBinding
    }
}

impl Default for enums::EntityTypeEnum {
    fn default() -> Self {
        Self::DistributedVirtualSwitch
    }
}

impl Default for enums::EntityImportTypeEnum {
    fn default() -> Self {
        Self::CreateEntityWithNewIdentifier
    }
}

impl Default for enums::DvsFilterSpecLinkConfigEnum {
    fn default() -> Self {
        Self::Blocked
    }
}

impl Default for enums::DvsFilterSpecLinkStateEnum {
    fn default() -> Self {
        Self::Down
    }
}

impl Default for enums::HostDvsConfigSpecSwitchModeEnum {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for enums::HostDistributedVirtualSwitchManagerFailoverReasonEnum {
    fn default() -> Self {
        Self::Crash
    }
}

impl Default for enums::HostDistributedVirtualSwitchManagerFailoverStageEnum {
    fn default() -> Self {
        Self::Stage1
    }
}

impl Default for enums::DistributedVirtualSwitchHostMemberHostComponentStateEnum {
    fn default() -> Self {
        Self::Up
    }
}

impl Default for enums::DistributedVirtualSwitchHostMemberHostUplinkStateStateEnum {
    fn default() -> Self {
        Self::Active
    }
}

impl Default for enums::DistributedVirtualSwitchHostMemberTransportZoneTypeEnum {
    fn default() -> Self {
        Self::Vlan
    }
}

impl Default for enums::DistributedVirtualSwitchPortConnecteeConnecteeTypeEnum {
    fn default() -> Self {
        Self::Pnic
    }
}

impl Default for enums::DvsNetworkRuleDirectionTypeEnum {
    fn default() -> Self {
        Self::IncomingPackets
    }
}

impl Default for enums::VMwareDvsLacpApiVersionEnum {
    fn default() -> Self {
        Self::SingleLag
    }
}

impl Default for enums::VMwareDvsLacpLoadBalanceAlgorithmEnum {
    fn default() -> Self {
        Self::SrcMac
    }
}

impl Default for enums::DvsMacLimitPolicyTypeEnum {
    fn default() -> Self {
        Self::Allow
    }
}

impl Default for enums::VMwareDvsMulticastFilteringModeEnum {
    fn default() -> Self {
        Self::LegacyFiltering
    }
}

impl Default for enums::VmwareDistributedVirtualSwitchPvlanPortTypeEnum {
    fn default() -> Self {
        Self::Promiscuous
    }
}

impl Default for enums::VMwareDvsTeamingMatchStatusEnum {
    fn default() -> Self {
        Self::IphashMatch
    }
}

impl Default for enums::VMwareUplinkLacpModeEnum {
    fn default() -> Self {
        Self::Active
    }
}

impl Default for enums::VMwareUplinkLacpTimeoutModeEnum {
    fn default() -> Self {
        Self::Fast
    }
}

impl Default for enums::VMwareDvsVspanSessionEncapTypeEnum {
    fn default() -> Self {
        Self::Gre
    }
}

impl Default for enums::VMwareDvsVspanSessionTypeEnum {
    fn default() -> Self {
        Self::MixedDestMirror
    }
}

impl Default for enums::CryptoManagerHostKeyManagementTypeEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::CryptoManagerKmipCryptoKeyStatusKeyUnavailableReasonEnum {
    fn default() -> Self {
        Self::KeyStateMissingInCache
    }
}

impl Default for enums::KmipClusterInfoKeyTypeEnum {
    fn default() -> Self {
        Self::RawKey
    }
}

impl Default for enums::KmipClusterInfoKmsManagementTypeEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::CustomizationFailedReasonCodeEnum {
    fn default() -> Self {
        Self::UserDefinedScriptDisabled
    }
}

impl Default for enums::DvsEventPortBlockStateEnum {
    fn default() -> Self {
        Self::Unset
    }
}

impl Default for enums::EventEventSeverityEnum {
    fn default() -> Self {
        Self::Error
    }
}

impl Default for enums::EventCategoryEnum {
    fn default() -> Self {
        Self::Info
    }
}

impl Default for enums::EventFilterSpecRecursionOptionEnum {
    fn default() -> Self {
        Self::Self_
    }
}

impl Default for enums::HostDasErrorEventHostDasErrorReasonEnum {
    fn default() -> Self {
        Self::ConfigFailed
    }
}

impl Default for enums::HostDisconnectedEventReasonCodeEnum {
    fn default() -> Self {
        Self::SslThumbprintVerifyFailed
    }
}

impl Default for enums::VmDasBeingResetEventReasonCodeEnum {
    fn default() -> Self {
        Self::VmtoolsHeartbeatFailure
    }
}

impl Default for enums::VmFailedStartingSecondaryEventFailureReasonEnum {
    fn default() -> Self {
        Self::IncompatibleHost
    }
}

impl Default for enums::VmShutdownOnIsolationEventOperationEnum {
    fn default() -> Self {
        Self::Shutdown
    }
}

impl Default for enums::AffinityTypeEnum {
    fn default() -> Self {
        Self::Memory
    }
}

impl Default for enums::AgentInstallFailedReasonEnum {
    fn default() -> Self {
        Self::NotEnoughSpaceOnDevice
    }
}

impl Default for enums::CannotEnableVmcpForClusterReasonEnum {
    fn default() -> Self {
        Self::ApdTimeoutDisabled
    }
}

impl Default for enums::CannotMoveFaultToleranceVmMoveTypeEnum {
    fn default() -> Self {
        Self::ResourcePool
    }
}

impl Default for enums::CannotPowerOffVmInClusterOperationEnum {
    fn default() -> Self {
        Self::Suspend
    }
}

impl Default for enums::CannotUseNetworkReasonEnum {
    fn default() -> Self {
        Self::NetworkReservationNotSupported
    }
}

impl Default for enums::DasConfigFaultDasConfigFaultReasonEnum {
    fn default() -> Self {
        Self::HostNetworkMisconfiguration
    }
}

impl Default for enums::DeviceNotSupportedReasonEnum {
    fn default() -> Self {
        Self::Host
    }
}

impl Default for enums::DisallowedChangeByServiceDisallowedChangeEnum {
    fn default() -> Self {
        Self::HotExtendDisk
    }
}

impl Default for enums::FtIssuesOnHostHostSelectionTypeEnum {
    fn default() -> Self {
        Self::User
    }
}

impl Default for enums::HostHasComponentFailureHostComponentTypeEnum {
    fn default() -> Self {
        Self::Datastore
    }
}

impl Default for enums::HostIncompatibleForFaultToleranceReasonEnum {
    fn default() -> Self {
        Self::Product
    }
}

impl Default for enums::HostIncompatibleForRecordReplayReasonEnum {
    fn default() -> Self {
        Self::Product
    }
}

impl Default for enums::IncompatibleHostForVmReplicationIncompatibleReasonEnum {
    fn default() -> Self {
        Self::Rpo
    }
}

impl Default for enums::InvalidDasConfigArgumentEntryForInvalidArgumentEnum {
    fn default() -> Self {
        Self::AdmissionControl
    }
}

impl Default for enums::InvalidProfileReferenceHostReasonEnum {
    fn default() -> Self {
        Self::IncompatibleVersion
    }
}

impl Default for enums::LicenseAssignmentFailedReasonEnum {
    fn default() -> Self {
        Self::KeyEntityMismatch
    }
}

impl Default for enums::NotSupportedDeviceForFtDeviceTypeEnum {
    fn default() -> Self {
        Self::VirtualVmxnet3
    }
}

impl Default for enums::NumVirtualCpusIncompatibleReasonEnum {
    fn default() -> Self {
        Self::RecordReplay
    }
}

impl Default for enums::QuarantineModeFaultFaultTypeEnum {
    fn default() -> Self {
        Self::NoCompatibleNonQuarantinedHost
    }
}

impl Default for enums::ReplicationDiskConfigFaultReasonForFaultEnum {
    fn default() -> Self {
        Self::DiskNotFound
    }
}

impl Default for enums::ReplicationVmConfigFaultReasonForFaultEnum {
    fn default() -> Self {
        Self::IncompatibleHwVersion
    }
}

impl Default for enums::ReplicationVmFaultReasonForFaultEnum {
    fn default() -> Self {
        Self::NotConfigured
    }
}

impl Default for enums::ReplicationVmInProgressFaultActivityEnum {
    fn default() -> Self {
        Self::FullSync
    }
}

impl Default for enums::ThirdPartyLicenseAssignmentFailedReasonEnum {
    fn default() -> Self {
        Self::LicenseAssignmentFailed
    }
}

impl Default for enums::VFlashModuleNotSupportedReasonEnum {
    fn default() -> Self {
        Self::CacheModeNotSupported
    }
}

impl Default for enums::VmFaultToleranceConfigIssueReasonForIssueEnum {
    fn default() -> Self {
        Self::HaNotEnabled
    }
}

impl Default for enums::VmFaultToleranceInvalidFileBackingDeviceTypeEnum {
    fn default() -> Self {
        Self::VirtualFloppy
    }
}

impl Default for enums::WillLoseHaProtectionResolutionEnum {
    fn default() -> Self {
        Self::Svmotion
    }
}

impl Default for enums::HostActiveDirectoryAuthenticationCertificateDigestEnum {
    fn default() -> Self {
        Self::Sha1
    }
}

impl Default for enums::HostActiveDirectoryInfoDomainMembershipStatusEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::AutoStartActionEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::AutoStartWaitHeartbeatSettingEnum {
    fn default() -> Self {
        Self::Yes
    }
}

impl Default for enums::HostBiosInfoFirmwareTypeEnum {
    fn default() -> Self {
        Self::Bios
    }
}

impl Default for enums::HostCapabilityFtUnsupportedReasonEnum {
    fn default() -> Self {
        Self::VMotionNotLicensed
    }
}

impl Default for enums::HostReplayUnsupportedReasonEnum {
    fn default() -> Self {
        Self::IncompatibleProduct
    }
}

impl Default for enums::HostCapabilityUnmapMethodSupportedEnum {
    fn default() -> Self {
        Self::Priority
    }
}

impl Default for enums::HostCapabilityVmDirectPathGen2UnsupportedReasonEnum {
    fn default() -> Self {
        Self::HostNptIncompatibleProduct
    }
}

impl Default for enums::HostCertificateManagerCertificateInfoCertificateStatusEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::HostCertificateManagerCertificateKindEnum {
    fn default() -> Self {
        Self::Machine
    }
}

impl Default for enums::HostConfigChangeModeEnum {
    fn default() -> Self {
        Self::Modify
    }
}

impl Default for enums::HostConfigChangeOperationEnum {
    fn default() -> Self {
        Self::Add
    }
}

impl Default for enums::HostConfigChangeOwnerEnum {
    fn default() -> Self {
        Self::Nsx
    }
}

impl Default for enums::HostCpuPackageVendorEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::HostCpuPowerManagementInfoPolicyTypeEnum {
    fn default() -> Self {
        Self::Off
    }
}

impl Default for enums::HostCpuSchedulerInfoCpuSchedulerPolicyInfoEnum {
    fn default() -> Self {
        Self::SystemDefault
    }
}

impl Default for enums::HostDateTimeInfoProtocolEnum {
    fn default() -> Self {
        Self::Ntp
    }
}

impl Default for enums::DiagnosticPartitionTypeEnum {
    fn default() -> Self {
        Self::SingleHost
    }
}

impl Default for enums::DiagnosticPartitionStorageTypeEnum {
    fn default() -> Self {
        Self::DirectAttached
    }
}

impl Default for enums::HostDigestInfoDigestMethodTypeEnum {
    fn default() -> Self {
        Self::Sha1
    }
}

impl Default for enums::HostDigestVerificationSettingEnum {
    fn default() -> Self {
        Self::DigestDisabled
    }
}

impl Default for enums::HostDiskPartitionInfoPartitionFormatEnum {
    fn default() -> Self {
        Self::Gpt
    }
}

impl Default for enums::HostDiskPartitionInfoTypeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::HostFeatureVersionKeyEnum {
    fn default() -> Self {
        Self::FaultTolerance
    }
}

impl Default for enums::FibreChannelPortTypeEnum {
    fn default() -> Self {
        Self::Fabric
    }
}

impl Default for enums::FileSystemMountInfoVStorageSupportStatusEnum {
    fn default() -> Self {
        Self::VStorageSupported
    }
}

impl Default for enums::HostFileSystemVolumeFileSystemTypeEnum {
    fn default() -> Self {
        Self::Vmfs
    }
}

impl Default for enums::HostFirewallSystemRuleSetIdEnum {
    fn default() -> Self {
        Self::FaultTolerance
    }
}

impl Default for enums::HostFirewallSystemServiceNameEnum {
    fn default() -> Self {
        Self::Vpxa
    }
}

impl Default for enums::HostFruFruTypeEnum {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Default for enums::HostGraphicsConfigGraphicsTypeEnum {
    fn default() -> Self {
        Self::Shared
    }
}

impl Default for enums::HostGraphicsConfigSharedPassthruAssignmentPolicyEnum {
    fn default() -> Self {
        Self::Performance
    }
}

impl Default for enums::HostGraphicsConfigVgpuModeEnum {
    fn default() -> Self {
        Self::SameSize
    }
}

impl Default for enums::HostGraphicsInfoGraphicsTypeEnum {
    fn default() -> Self {
        Self::Basic
    }
}

impl Default for enums::HostGraphicsInfoVgpuModeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::HostHardwareElementStatusEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::HostAccessModeEnum {
    fn default() -> Self {
        Self::AccessNone
    }
}

impl Default for enums::HostLockdownModeEnum {
    fn default() -> Self {
        Self::LockdownDisabled
    }
}

impl Default for enums::HostImageAcceptanceLevelEnum {
    fn default() -> Self {
        Self::VmwareCertified
    }
}

impl Default for enums::HostInternetScsiHbaChapAuthenticationTypeEnum {
    fn default() -> Self {
        Self::ChapProhibited
    }
}

impl Default for enums::HostInternetScsiHbaDigestTypeEnum {
    fn default() -> Self {
        Self::DigestProhibited
    }
}

impl Default for enums::InternetScsiSnsDiscoveryMethodEnum {
    fn default() -> Self {
        Self::IsnsStatic
    }
}

impl Default for enums::SlpDiscoveryMethodEnum {
    fn default() -> Self {
        Self::SlpDhcp
    }
}

impl Default for enums::HostInternetScsiHbaIscsiIpv6AddressAddressConfigurationTypeEnum {
    fn default() -> Self {
        Self::Dhcp
    }
}

impl Default for enums::HostInternetScsiHbaIscsiIpv6AddressIPv6AddressOperationEnum {
    fn default() -> Self {
        Self::Add
    }
}

impl Default for enums::HostInternetScsiHbaNetworkBindingSupportTypeEnum {
    fn default() -> Self {
        Self::Notsupported
    }
}

impl Default for enums::HostInternetScsiHbaStaticTargetTargetDiscoveryMethodEnum {
    fn default() -> Self {
        Self::StaticMethod
    }
}

impl Default for enums::HostIpConfigIpV6AddressConfigTypeEnum {
    fn default() -> Self {
        Self::Other
    }
}

impl Default for enums::HostIpConfigIpV6AddressStatusEnum {
    fn default() -> Self {
        Self::Preferred
    }
}

impl Default for enums::IscsiPortInfoPathStatusEnum {
    fn default() -> Self {
        Self::NotUsed
    }
}

impl Default for enums::LinkDiscoveryProtocolConfigOperationTypeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::LinkDiscoveryProtocolConfigProtocolTypeEnum {
    fn default() -> Self {
        Self::Cdp
    }
}

impl Default for enums::HostLowLevelProvisioningManagerFileTypeEnum {
    fn default() -> Self {
        Self::File
    }
}

impl Default for enums::HostLowLevelProvisioningManagerReloadTargetEnum {
    fn default() -> Self {
        Self::CurrentConfig
    }
}

impl Default for enums::HostMaintenanceSpecPurposeEnum {
    fn default() -> Self {
        Self::HostUpgrade
    }
}

impl Default for enums::VirtualMachineMemoryAllocationPolicyEnum {
    fn default() -> Self {
        Self::SwapNone
    }
}

impl Default for enums::HostMemoryTierFlagsEnum {
    fn default() -> Self {
        Self::MemoryTier
    }
}

impl Default for enums::HostMemoryTierTypeEnum {
    fn default() -> Self {
        Self::Dram
    }
}

impl Default for enums::HostMemoryTieringTypeEnum {
    fn default() -> Self {
        Self::NoTiering
    }
}

impl Default for enums::HostMountModeEnum {
    fn default() -> Self {
        Self::ReadWrite
    }
}

impl Default for enums::HostMountInfoInaccessibleReasonEnum {
    fn default() -> Self {
        Self::AllPathsDownStart
    }
}

impl Default for enums::HostMountInfoMountFailedReasonEnum {
    fn default() -> Self {
        Self::ConnectFailure
    }
}

impl Default for enums::MultipathStateEnum {
    fn default() -> Self {
        Self::Standby
    }
}

impl Default for enums::HostNasVolumeSecurityTypeEnum {
    fn default() -> Self {
        Self::AuthSys
    }
}

impl Default for enums::HostNetStackInstanceCongestionControlAlgorithmTypeEnum {
    fn default() -> Self {
        Self::Newreno
    }
}

impl Default for enums::HostNetStackInstanceSystemStackKeyEnum {
    fn default() -> Self {
        Self::DefaultTcpipStack
    }
}

impl Default for enums::HostNumericSensorHealthStateEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::HostNumericSensorTypeEnum {
    fn default() -> Self {
        Self::Fan
    }
}

impl Default for enums::NvdimmNvdimmHealthInfoStateEnum {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for enums::NvdimmInterleaveSetStateEnum {
    fn default() -> Self {
        Self::Invalid
    }
}

impl Default for enums::NvdimmNamespaceDetailsHealthStatusEnum {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for enums::NvdimmNamespaceDetailsStateEnum {
    fn default() -> Self {
        Self::Invalid
    }
}

impl Default for enums::NvdimmNamespaceHealthStatusEnum {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for enums::NvdimmNamespaceStateEnum {
    fn default() -> Self {
        Self::Invalid
    }
}

impl Default for enums::NvdimmNamespaceTypeEnum {
    fn default() -> Self {
        Self::BlockNamespace
    }
}

impl Default for enums::NvdimmRangeTypeEnum {
    fn default() -> Self {
        Self::VolatileRange
    }
}

impl Default for enums::HostNvmeDiscoveryLogSubsystemTypeEnum {
    fn default() -> Self {
        Self::Discovery
    }
}

impl Default for enums::HostNvmeDiscoveryLogTransportRequirementsEnum {
    fn default() -> Self {
        Self::SecureChannelRequired
    }
}

impl Default for enums::HostNvmeTransportParametersNvmeAddressFamilyEnum {
    fn default() -> Self {
        Self::Ipv4
    }
}

impl Default for enums::HostNvmeTransportTypeEnum {
    fn default() -> Self {
        Self::Pcie
    }
}

impl Default for enums::HostOpaqueSwitchOpaqueSwitchStateEnum {
    fn default() -> Self {
        Self::Up
    }
}

impl Default for enums::HostPartialMaintenanceModeIdEnum {
    fn default() -> Self {
        Self::QuickPatchPartialMm
    }
}

impl Default for enums::HostPartialMaintenanceModeStatusEnum {
    fn default() -> Self {
        Self::NotInPartialMm
    }
}

impl Default for enums::HostPatchManagerInstallStateEnum {
    fn default() -> Self {
        Self::HostRestarted
    }
}

impl Default for enums::HostPatchManagerIntegrityStatusEnum {
    fn default() -> Self {
        Self::Validated
    }
}

impl Default for enums::HostPatchManagerReasonEnum {
    fn default() -> Self {
        Self::Obsoleted
    }
}

impl Default for enums::PhysicalNicResourcePoolSchedulerDisallowedReasonEnum {
    fn default() -> Self {
        Self::UserOptOut
    }
}

impl Default for enums::PhysicalNicVmDirectPathGen2SupportedModeEnum {
    fn default() -> Self {
        Self::Upt
    }
}

impl Default for enums::PortGroupConnecteeTypeEnum {
    fn default() -> Self {
        Self::VirtualMachine
    }
}

impl Default for enums::HostProtocolEndpointPeTypeEnum {
    fn default() -> Self {
        Self::Block
    }
}

impl Default for enums::HostProtocolEndpointProtocolEndpointTypeEnum {
    fn default() -> Self {
        Self::Scsi
    }
}

impl Default for enums::HostPtpConfigDeviceTypeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::HostQualifiedNameTypeEnum {
    fn default() -> Self {
        Self::NvmeQualifiedName
    }
}

impl Default for enums::HostRdmaDeviceConnectionStateEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::RdmaProtocolEnum {
    fn default() -> Self {
        Self::RoCEv2
    }
}

impl Default for enums::HostFirewallRuleDirectionEnum {
    fn default() -> Self {
        Self::Inbound
    }
}

impl Default for enums::HostFirewallRulePortTypeEnum {
    fn default() -> Self {
        Self::Src
    }
}

impl Default for enums::HostFirewallRuleProtocolEnum {
    fn default() -> Self {
        Self::Tcp
    }
}

impl Default for enums::HostRuntimeInfoNetStackInstanceRuntimeInfoStateEnum {
    fn default() -> Self {
        Self::Inactive
    }
}

impl Default for enums::HostRuntimeInfoStateEncryptionInfoProtectionModeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::HostRuntimeInfoStatelessNvdsMigrationStateEnum {
    fn default() -> Self {
        Self::Ready
    }
}

impl Default for enums::ScsiDiskTypeEnum {
    fn default() -> Self {
        Self::Native512
    }
}

impl Default for enums::ScsiLunDescriptorQualityEnum {
    fn default() -> Self {
        Self::HighQuality
    }
}

impl Default for enums::DeviceProtocolEnum {
    fn default() -> Self {
        Self::NvMe
    }
}

impl Default for enums::ScsiLunLunReservationStatusEnum {
    fn default() -> Self {
        Self::LunReservedUnknown
    }
}

impl Default for enums::ScsiLunTypeEnum {
    fn default() -> Self {
        Self::Disk
    }
}

impl Default for enums::ScsiLunStateEnum {
    fn default() -> Self {
        Self::UnknownState
    }
}

impl Default for enums::ScsiLunVStorageSupportStatusEnum {
    fn default() -> Self {
        Self::VStorageSupported
    }
}

impl Default for enums::HostServicePolicyEnum {
    fn default() -> Self {
        Self::On
    }
}

impl Default for enums::HostSevInfoSevStateEnum {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl Default for enums::HostSgxInfoFlcModesEnum {
    fn default() -> Self {
        Self::Off
    }
}

impl Default for enums::HostSgxInfoSgxStatesEnum {
    fn default() -> Self {
        Self::NotPresent
    }
}

impl Default for enums::HostSgxRegistrationInfoRegistrationStatusEnum {
    fn default() -> Self {
        Self::NotApplicable
    }
}

impl Default for enums::HostSgxRegistrationInfoRegistrationTypeEnum {
    fn default() -> Self {
        Self::Manifest
    }
}

impl Default for enums::HostSnmpAgentCapabilityEnum {
    fn default() -> Self {
        Self::Complete
    }
}

impl Default for enums::SoftwarePackageConstraintEnum {
    fn default() -> Self {
        Self::Equals
    }
}

impl Default for enums::SoftwarePackageVibTypeEnum {
    fn default() -> Self {
        Self::Bootbank
    }
}

impl Default for enums::HostStorageProtocolEnum {
    fn default() -> Self {
        Self::Scsi
    }
}

impl Default for enums::HostSystemIdentificationInfoIdentifierEnum {
    fn default() -> Self {
        Self::AssetTag
    }
}

impl Default for enums::HostTdxInfoTdxStateEnum {
    fn default() -> Self {
        Self::Initializing
    }
}

impl Default for enums::HostTpmAttestationInfoAcceptanceStatusEnum {
    fn default() -> Self {
        Self::NotAccepted
    }
}

impl Default for enums::HostTrustAuthorityAttestationInfoAttestationStatusEnum {
    fn default() -> Self {
        Self::Attested
    }
}

impl Default for enums::HostUnresolvedVmfsExtentUnresolvedReasonEnum {
    fn default() -> Self {
        Self::DiskIdMismatch
    }
}

impl Default for enums::HostUnresolvedVmfsResolutionSpecVmfsUuidResolutionEnum {
    fn default() -> Self {
        Self::Resignature
    }
}

impl Default for enums::HostVirtualNicManagerNicTypeEnum {
    fn default() -> Self {
        Self::Vmotion
    }
}

impl Default for enums::HostVmciAccessManagerModeEnum {
    fn default() -> Self {
        Self::Grant
    }
}

impl Default for enums::HostVmfsVolumeUnmapBandwidthPolicyEnum {
    fn default() -> Self {
        Self::Fixed
    }
}

impl Default for enums::HostVmfsVolumeUnmapPriorityEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::VsanControllerTypeEnum {
    fn default() -> Self {
        Self::NvMe
    }
}

impl Default for enums::VsanDiskBalanceStateEnum {
    fn default() -> Self {
        Self::Rebalanceoff
    }
}

impl Default for enums::VsanEncryptionIssueEnum {
    fn default() -> Self {
        Self::Enabledwhenclusterdisabled
    }
}

impl Default for enums::VsanHostQueryCheckLimitsOptionTypeEnum {
    fn default() -> Self {
        Self::LogicalCapacity
    }
}

impl Default for enums::VsanIoInsightStateEnum {
    fn default() -> Self {
        Self::Running
    }
}

impl Default for enums::VsanObjectHealthStateEnum {
    fn default() -> Self {
        Self::Inaccessible
    }
}

impl Default for enums::VsanPeerHostConnectivityHealthStateEnum {
    fn default() -> Self {
        Self::StateGood
    }
}

impl Default for enums::VsanSmartParameterTypeEnum {
    fn default() -> Self {
        Self::Smarthealthstatus
    }
}

impl Default for enums::NetIpConfigInfoIpAddressOriginEnum {
    fn default() -> Self {
        Self::Other
    }
}

impl Default for enums::NetIpConfigInfoIpAddressStatusEnum {
    fn default() -> Self {
        Self::Preferred
    }
}

impl Default for enums::NetIpStackInfoEntryTypeEnum {
    fn default() -> Self {
        Self::Other
    }
}

impl Default for enums::NetIpStackInfoPreferenceEnum {
    fn default() -> Self {
        Self::Reserved
    }
}

impl Default for enums::NetBiosConfigInfoModeEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::ArrayUpdateOperationEnum {
    fn default() -> Self {
        Self::Add
    }
}

impl Default for enums::ComplianceResultStatusEnum {
    fn default() -> Self {
        Self::Compliant
    }
}

impl Default for enums::ProfileNumericComparatorEnum {
    fn default() -> Self {
        Self::LessThan
    }
}

impl Default for enums::ProfileParameterMetadataRelationTypeEnum {
    fn default() -> Self {
        Self::DynamicRelation
    }
}

impl Default for enums::ClusterProfileServiceTypeEnum {
    fn default() -> Self {
        Self::Drs
    }
}

impl Default for enums::ProfileExecuteResultStatusEnum {
    fn default() -> Self {
        Self::Success
    }
}

impl Default for enums::HostProfileValidationFailureInfoUpdateTypeEnum {
    fn default() -> Self {
        Self::HostBased
    }
}

impl Default for enums::HostProfileValidationStateEnum {
    fn default() -> Self {
        Self::Ready
    }
}

impl Default for enums::HostProfileManagerAnswerFileStatusEnum {
    fn default() -> Self {
        Self::Valid
    }
}

impl Default for enums::ApplyHostProfileConfigurationResultStatusEnum {
    fn default() -> Self {
        Self::Success
    }
}

impl Default for enums::HostProfileManagerCompositionResultResultElementStatusEnum {
    fn default() -> Self {
        Self::Success
    }
}

impl Default for enums::HostProfileManagerCompositionValidationResultResultElementStatusEnum {
    fn default() -> Self {
        Self::Success
    }
}

impl Default for enums::HostProfileManagerTaskListRequirementEnum {
    fn default() -> Self {
        Self::MaintenanceModeRequired
    }
}

impl Default for enums::AnswerFileValidationInfoStatusEnum {
    fn default() -> Self {
        Self::Success
    }
}

impl Default for enums::DayOfWeekEnum {
    fn default() -> Self {
        Self::Sunday
    }
}

impl Default for enums::WeekOfMonthEnum {
    fn default() -> Self {
        Self::First
    }
}

impl Default for enums::PlacementAffinityRuleRuleScopeEnum {
    fn default() -> Self {
        Self::Cluster
    }
}

impl Default for enums::PlacementAffinityRuleRuleTypeEnum {
    fn default() -> Self {
        Self::Affinity
    }
}

impl Default for enums::StorageDrsPodConfigInfoBehaviorEnum {
    fn default() -> Self {
        Self::Manual
    }
}

impl Default for enums::StorageDrsSpaceLoadBalanceConfigSpaceThresholdModeEnum {
    fn default() -> Self {
        Self::Utilization
    }
}

impl Default for enums::StoragePlacementSpecPlacementTypeEnum {
    fn default() -> Self {
        Self::Create
    }
}

impl Default for enums::VirtualDiskRuleSpecRuleTypeEnum {
    fn default() -> Self {
        Self::Affinity
    }
}

impl Default for enums::VAppCloneSpecProvisioningTypeEnum {
    fn default() -> Self {
        Self::SameAsSource
    }
}

impl Default for enums::VAppAutoStartActionEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::VAppIpAssignmentInfoAllocationSchemesEnum {
    fn default() -> Self {
        Self::Dhcp
    }
}

impl Default for enums::VAppIpAssignmentInfoIpAllocationPolicyEnum {
    fn default() -> Self {
        Self::DhcpPolicy
    }
}

impl Default for enums::VAppIpAssignmentInfoProtocolsEnum {
    fn default() -> Self {
        Self::IPv4
    }
}

impl Default for enums::VchaStateEnum {
    fn default() -> Self {
        Self::Configured
    }
}

impl Default for enums::VchaClusterModeEnum {
    fn default() -> Self {
        Self::Enabled
    }
}

impl Default for enums::VchaClusterStateEnum {
    fn default() -> Self {
        Self::Healthy
    }
}

impl Default for enums::VchaNodeRoleEnum {
    fn default() -> Self {
        Self::Active
    }
}

impl Default for enums::VchaNodeStateEnum {
    fn default() -> Self {
        Self::Up
    }
}

impl Default for enums::VirtualMachineBootOptionsNetworkBootProtocolTypeEnum {
    fn default() -> Self {
        Self::Ipv4
    }
}

impl Default for enums::VirtualMachineCertThumbprintHashAlgorithmEnum {
    fn default() -> Self {
        Self::Sha256
    }
}

impl Default for enums::VirtualMachineCloneSpecTpmProvisionPolicyEnum {
    fn default() -> Self {
        Self::Copy
    }
}

impl Default for enums::VirtualMachineConfigInfoNpivWwnTypeEnum {
    fn default() -> Self {
        Self::Vc
    }
}

impl Default for enums::VirtualMachineConfigInfoSwapPlacementTypeEnum {
    fn default() -> Self {
        Self::Inherit
    }
}

impl Default for enums::VirtualMachineConfigSpecEncryptedFtModesEnum {
    fn default() -> Self {
        Self::FtEncryptionDisabled
    }
}

impl Default for enums::VirtualMachineConfigSpecEncryptedVMotionModesEnum {
    fn default() -> Self {
        Self::Disabled
    }
}

impl Default for enums::VirtualMachineConfigSpecNpivWwnOpEnum {
    fn default() -> Self {
        Self::Generate
    }
}

impl Default for enums::VirtualMachinePowerOpTypeEnum {
    fn default() -> Self {
        Self::Soft
    }
}

impl Default for enums::VirtualMachineStandbyActionTypeEnum {
    fn default() -> Self {
        Self::Checkpoint
    }
}

impl Default for enums::VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeStateVmDirectPathGen2InactiveReasonOtherEnum {
    fn default() -> Self {
        Self::VmNptIncompatibleHost
    }
}

impl Default for enums::VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeStateVmDirectPathGen2InactiveReasonVmEnum {
    fn default() -> Self {
        Self::VmNptIncompatibleGuest
    }
}

impl Default for enums::VirtualMachineFileLayoutExFileTypeEnum {
    fn default() -> Self {
        Self::Config
    }
}

impl Default for enums::VirtualMachineHtSharingEnum {
    fn default() -> Self {
        Self::Any
    }
}

impl Default for enums::VirtualMachineFlagInfoMonitorTypeEnum {
    fn default() -> Self {
        Self::Release
    }
}

impl Default for enums::VirtualMachinePowerOffBehaviorEnum {
    fn default() -> Self {
        Self::PowerOff
    }
}

impl Default for enums::VirtualMachineFlagInfoVirtualExecUsageEnum {
    fn default() -> Self {
        Self::HvAuto
    }
}

impl Default for enums::VirtualMachineFlagInfoVirtualMmuUsageEnum {
    fn default() -> Self {
        Self::Automatic
    }
}

impl Default for enums::VirtualMachineForkConfigInfoChildTypeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::GuestInfoAppStateTypeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::GuestInfoCustomizationStatusEnum {
    fn default() -> Self {
        Self::ToolsdeploypkgIdle
    }
}

impl Default for enums::VirtualMachineGuestStateEnum {
    fn default() -> Self {
        Self::Running
    }
}

impl Default for enums::VirtualMachineToolsInstallTypeEnum {
    fn default() -> Self {
        Self::GuestToolsTypeUnknown
    }
}

impl Default for enums::VirtualMachineToolsRunningStatusEnum {
    fn default() -> Self {
        Self::GuestToolsNotRunning
    }
}

impl Default for enums::VirtualMachineToolsStatusEnum {
    fn default() -> Self {
        Self::ToolsNotInstalled
    }
}

impl Default for enums::VirtualMachineToolsVersionStatusEnum {
    fn default() -> Self {
        Self::GuestToolsNotInstalled
    }
}

impl Default for enums::GuestOsDescriptorFirmwareTypeEnum {
    fn default() -> Self {
        Self::Bios
    }
}

impl Default for enums::VirtualMachineGuestOsFamilyEnum {
    fn default() -> Self {
        Self::WindowsGuest
    }
}

impl Default for enums::VirtualMachineGuestOsIdentifierEnum {
    fn default() -> Self {
        Self::DosGuest
    }
}

impl Default for enums::GuestOsDescriptorSupportLevelEnum {
    fn default() -> Self {
        Self::Experimental
    }
}

impl Default for enums::GuestQuiesceEndGuestQuiesceErrorEnum {
    fn default() -> Self {
        Self::Failure
    }
}

impl Default for enums::VirtualMachineMetadataManagerVmMetadataOpEnum {
    fn default() -> Self {
        Self::Update
    }
}

impl Default for enums::VirtualMachineMetadataManagerVmMetadataOwnerOwnerEnum {
    fn default() -> Self {
        Self::ComVmwareVsphereHa
    }
}

impl Default for enums::VirtualMachineRelocateDiskMoveOptionsEnum {
    fn default() -> Self {
        Self::MoveAllDiskBackingsAndAllowSharing
    }
}

impl Default for enums::VirtualMachineRelocateTransformationEnum {
    fn default() -> Self {
        Self::Flat
    }
}

impl Default for enums::ScheduledHardwareUpgradeInfoHardwareUpgradePolicyEnum {
    fn default() -> Self {
        Self::Never
    }
}

impl Default for enums::ScheduledHardwareUpgradeInfoHardwareUpgradeStatusEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::VirtualMachineScsiPassthroughTypeEnum {
    fn default() -> Self {
        Self::Disk
    }
}

impl Default for enums::VirtualMachineSgxInfoFlcModesEnum {
    fn default() -> Self {
        Self::Locked
    }
}

impl Default for enums::VirtualMachineTargetInfoConfigurationTagEnum {
    fn default() -> Self {
        Self::Compliant
    }
}

impl Default for enums::UpgradePolicyEnum {
    fn default() -> Self {
        Self::Manual
    }
}

impl Default for enums::VirtualMachineUsbInfoFamilyEnum {
    fn default() -> Self {
        Self::Audio
    }
}

impl Default for enums::VirtualMachineUsbInfoSpeedEnum {
    fn default() -> Self {
        Self::Low
    }
}

impl Default for enums::VirtualMachineVendorDeviceGroupInfoComponentDeviceInfoComponentTypeEnum {
    fn default() -> Self {
        Self::PciPassthru
    }
}

impl Default for enums::VirtualMachineVgpuProfileInfoProfileClassEnum {
    fn default() -> Self {
        Self::Compute
    }
}

impl Default for enums::VirtualMachineVgpuProfileInfoProfileSharingEnum {
    fn default() -> Self {
        Self::TimeSliced
    }
}

impl Default for enums::VirtualMachineVirtualDeviceSwapDeviceSwapStatusEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::VirtualHardwareMotherboardLayoutEnum {
    fn default() -> Self {
        Self::I440BxHostBridge
    }
}

impl Default for enums::VirtualMachineVirtualPMemSnapshotModeEnum {
    fn default() -> Self {
        Self::IndependentPersistent
    }
}

impl Default for enums::VirtualMachineWindowsQuiesceSpecVssBackupContextEnum {
    fn default() -> Self {
        Self::CtxAuto
    }
}

impl Default for enums::CheckTestTypeEnum {
    fn default() -> Self {
        Self::SourceTests
    }
}

impl Default for enums::CustomizationNetBiosModeEnum {
    fn default() -> Self {
        Self::EnableNetBiosViaDhcp
    }
}

impl Default for enums::CustomizationLicenseDataModeEnum {
    fn default() -> Self {
        Self::PerServer
    }
}

impl Default for enums::CustomizationSysprepRebootOptionEnum {
    fn default() -> Self {
        Self::Reboot
    }
}

impl Default for enums::VirtualDeviceConnectInfoMigrateConnectOpEnum {
    fn default() -> Self {
        Self::Connect
    }
}

impl Default for enums::VirtualDeviceConnectInfoStatusEnum {
    fn default() -> Self {
        Self::Ok
    }
}

impl Default for enums::VirtualDeviceFileExtensionEnum {
    fn default() -> Self {
        Self::Iso
    }
}

impl Default for enums::VirtualDeviceUriBackingOptionDirectionEnum {
    fn default() -> Self {
        Self::Server
    }
}

impl Default for enums::VirtualDeviceConfigSpecChangeModeEnum {
    fn default() -> Self {
        Self::Fail
    }
}

impl Default for enums::VirtualDeviceConfigSpecFileOperationEnum {
    fn default() -> Self {
        Self::Create
    }
}

impl Default for enums::VirtualDeviceConfigSpecOperationEnum {
    fn default() -> Self {
        Self::Add
    }
}

impl Default for enums::VirtualDiskDeltaDiskFormatEnum {
    fn default() -> Self {
        Self::RedoLogFormat
    }
}

impl Default for enums::VirtualDiskDeltaDiskFormatVariantEnum {
    fn default() -> Self {
        Self::VmfsSparseVariant
    }
}

impl Default for enums::VirtualDiskSharingEnum {
    fn default() -> Self {
        Self::SharingNone
    }
}

impl Default for enums::VirtualDiskVFlashCacheConfigInfoCacheConsistencyTypeEnum {
    fn default() -> Self {
        Self::Strong
    }
}

impl Default for enums::VirtualDiskVFlashCacheConfigInfoCacheModeEnum {
    fn default() -> Self {
        Self::WriteThru
    }
}

impl Default for enums::VirtualDiskCompatibilityModeEnum {
    fn default() -> Self {
        Self::VirtualMode
    }
}

impl Default for enums::VirtualDiskModeEnum {
    fn default() -> Self {
        Self::Persistent
    }
}

impl Default for enums::VirtualEthernetCardLegacyNetworkDeviceNameEnum {
    fn default() -> Self {
        Self::Bridged
    }
}

impl Default for enums::VirtualEthernetCardMacTypeEnum {
    fn default() -> Self {
        Self::Manual
    }
}

impl Default for enums::VirtualNvmeControllerSharingEnum {
    fn default() -> Self {
        Self::NoSharing
    }
}

impl Default for enums::VirtualPointingDeviceHostChoiceEnum {
    fn default() -> Self {
        Self::Autodetect
    }
}

impl Default for enums::VirtualScsiSharingEnum {
    fn default() -> Self {
        Self::NoSharing
    }
}

impl Default for enums::VirtualSerialPortEndPointEnum {
    fn default() -> Self {
        Self::Client
    }
}

impl Default for enums::VirtualMachineVmciDeviceActionEnum {
    fn default() -> Self {
        Self::Allow
    }
}

impl Default for enums::VirtualMachineVmciDeviceDirectionEnum {
    fn default() -> Self {
        Self::Guest
    }
}

impl Default for enums::VirtualMachineVmciDeviceProtocolEnum {
    fn default() -> Self {
        Self::Hypervisor
    }
}

impl Default for enums::VirtualMachineVideoCardUse3DRendererEnum {
    fn default() -> Self {
        Self::Automatic
    }
}

impl Default for enums::VirtualVmxnet3StrictLatencyConfigDisableOffloadEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::VirtualVmxnet3VrdmaOptionDeviceProtocolsEnum {
    fn default() -> Self {
        Self::Rocev1
    }
}

impl Default for enums::GuestFileTypeEnum {
    fn default() -> Self {
        Self::File
    }
}

impl Default for enums::GuestRegKeyWowSpecEnum {
    fn default() -> Self {
        Self::WowNative
    }
}

impl Default for enums::VsanCapacityReservationStateEnum {
    fn default() -> Self {
        Self::StateUnknown
    }
}

impl Default for enums::VsanFileServiceConfigOpTypeEnum {
    fn default() -> Self {
        Self::CleanAffinityLocation
    }
}

impl Default for enums::VsanFileServicePreflightCheckScopeEnum {
    fn default() -> Self {
        Self::Basic
    }
}

impl Default for enums::VsanFileServiceVmStatusEnum {
    fn default() -> Self {
        Self::Running
    }
}

impl Default for enums::VsanFileShareAccessTypeEnum {
    fn default() -> Self {
        Self::ReadOnly
    }
}

impl Default for enums::VsanFileShareManagingEntityEnum {
    fn default() -> Self {
        Self::Cns
    }
}

impl Default for enums::VsanFileShareNfsSecTypeEnum {
    fn default() -> Self {
        Self::Sys
    }
}

impl Default for enums::VsanFileProtocolEnum {
    fn default() -> Self {
        Self::NfSv3
    }
}

impl Default for enums::VsanFileShareSmbEncryptionTypeEnum {
    fn default() -> Self {
        Self::Disabled
    }
}

impl Default for enums::HciMeshClientOperationEnum {
    fn default() -> Self {
        Self::DryrunDs
    }
}

impl Default for enums::VsanIoDiagnosticsFailedCheckTypeEnum {
    fn default() -> Self {
        Self::Perfsvcdisabled
    }
}

impl Default for enums::VsanIoDiagnosticsInstanceEventTypeEnum {
    fn default() -> Self {
        Self::ObjectOwnerTransfer
    }
}

impl Default for enums::VsanIoDiagnosticsInstanceStateEnum {
    fn default() -> Self {
        Self::VsanIoDiagnosticsInstanceCompleted
    }
}

impl Default for enums::VsanIoDiagnosticsTargetTypeEnum {
    fn default() -> Self {
        Self::VirtualMachine
    }
}

impl Default for enums::VsanIoLatencyTypeEnum {
    fn default() -> Self {
        Self::DomOwnerLatency
    }
}

impl Default for enums::VimVsanLifecycleCheckOperationEnum {
    fn default() -> Self {
        Self::NoChecks
    }
}

impl Default for enums::VimVsanLifecycleClusterTypeEnum {
    fn default() -> Self {
        Self::Single
    }
}

impl Default for enums::VimVsanLifecyclePreCheckTypeEnum {
    fn default() -> Self {
        Self::SupportedWitnessVersion
    }
}

impl Default for enums::VsanModeEnum {
    fn default() -> Self {
        Self::ModeNone
    }
}

impl Default for enums::VimVsanMountPrecheckTypeEnum {
    fn default() -> Self {
        Self::SupportedConfiguration
    }
}

impl Default for enums::VsanPerfsvcRemediateActionEnum {
    fn default() -> Self {
        Self::Enable
    }
}

impl Default for enums::PrecheckDatastoreSourceOperationEnum {
    fn default() -> Self {
        Self::CheckCreateDs
    }
}

impl Default for enums::VsanRemoteVcLinkTypeEnum {
    fn default() -> Self {
        Self::Standalone
    }
}

impl Default for enums::RemoteVsanNetworkTopologyEnum {
    fn default() -> Self {
        Self::TopologyUnknown
    }
}

impl Default for enums::VsanResourceCheckComponentTypeEnum {
    fn default() -> Self {
        Self::VSan
    }
}

impl Default for enums::ResourceCheckDedupStoreHealthStateEnum {
    fn default() -> Self {
        Self::Inaccessible
    }
}

impl Default for enums::VsanResourceCheckStatusTypeEnum {
    fn default() -> Self {
        Self::ResourceCheckUninitialized
    }
}

impl Default for enums::VsanSnapshotCreatorEnum {
    fn default() -> Self {
        Self::SnapService
    }
}

impl Default for enums::VsanSnapshotTypeEnum {
    fn default() -> Self {
        Self::Managed
    }
}

impl Default for enums::VsanAnalyticsEventLocationTypeEnum {
    fn default() -> Self {
        Self::Cluster
    }
}

impl Default for enums::VsanAnalyticsEventSnapshotTypeEnum {
    fn default() -> Self {
        Self::ObjectSnapshot
    }
}

impl Default for enums::VsanAnalyticsEventTypeEnum {
    fn default() -> Self {
        Self::StorageObjectUnavailable
    }
}

impl Default for enums::VsanConfigTypeEnum {
    fn default() -> Self {
        Self::Vsan
    }
}

impl Default for enums::VsanDiskCompatibilityTypeEnum {
    fn default() -> Self {
        Self::DiskGroup
    }
}

impl Default for enums::VsanHealthPerspectiveEnum {
    fn default() -> Self {
        Self::DefaultView
    }
}

impl Default for enums::VsanHealthPerspective90Enum {
    fn default() -> Self {
        Self::SiteEnterMaintenance
    }
}

impl Default for enums::VsanHealthStatusTypeEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::VsanHealthThresholdTargetEnum {
    fn default() -> Self {
        Self::DiskspaceVsanDatastore
    }
}

impl Default for enums::VsanIoTripAnalyzerRecurrenceStatusEnum {
    fn default() -> Self {
        Self::RecurrenceEnabled
    }
}

impl Default for enums::VsanPolicyRegulationCheckOpEnumEnum {
    fn default() -> Self {
        Self::Equal
    }
}

impl Default for enums::VimVsanVsanScanObjectsIssueTypeEnum {
    fn default() -> Self {
        Self::BrokenChain
    }
}

impl Default for enums::VsanServiceStatusEnum {
    fn default() -> Self {
        Self::Started
    }
}

impl Default for enums::VsanSiteLocationTypeEnum {
    fn default() -> Self {
        Self::None
    }
}

impl Default for enums::VsanSnapHealthTypeEnum {
    fn default() -> Self {
        Self::ApplianceHealth
    }
}

impl Default for enums::VsanSnapStatsExpirationTypeEnum {
    fn default() -> Self {
        Self::AfterSet
    }
}

impl Default for enums::VsanSnapVmMembershipChangeStatusEnum {
    fn default() -> Self {
        Self::Added
    }
}

impl Default for enums::VsanSyncReasonEnum {
    fn default() -> Self {
        Self::Repair
    }
}

impl Default for enums::VsanSyncStatusEnum {
    fn default() -> Self {
        Self::Active
    }
}

impl Default for enums::VsanUpdateItemImpactTypeEnum {
    fn default() -> Self {
        Self::Reboot
    }
}

impl Default for enums::VsanUpdateItemTypeEnum {
    fn default() -> Self {
        Self::Vib
    }
}

impl Default for enums::VimVsanVsanVcsaDeploymentPhaseEnum {
    fn default() -> Self {
        Self::Initializing
    }
}

impl Default for enums::VsanVibTypeEnum {
    fn default() -> Self {
        Self::Tool
    }
}

impl Default for enums::VsanXvcQueryCriteriaOperatorEnum {
    fn default() -> Self {
        Self::Equal
    }
}

impl Default for enums::VsanXvcQueryFilterOperatorEnum {
    fn default() -> Self {
        Self::And
    }
}

impl Default for enums::VimVsanClusterComplianceResourceCheckStatusTypeEnum {
    fn default() -> Self {
        Self::InProgress
    }
}

impl Default for enums::VimVsanClusterVsanManagedStorageTypeEnum {
    fn default() -> Self {
        Self::Vsandirect
    }
}

impl Default for enums::ClusterPowerStateEnum {
    fn default() -> Self {
        Self::PoweredOn
    }
}

impl Default for enums::VsanComplianceStatusEnum {
    fn default() -> Self {
        Self::Compliant
    }
}

impl Default for enums::VsanHostDecommissionModeObjectActionEnum {
    fn default() -> Self {
        Self::NoAction
    }
}

impl Default for enums::VimVsanHostDiskMappingCreationTypeEnum {
    fn default() -> Self {
        Self::Hybrid
    }
}

impl Default for enums::VsanHostDiskResultStateEnum {
    fn default() -> Self {
        Self::InUse
    }
}

impl Default for enums::VsanEncryptionOperationEnum {
    fn default() -> Self {
        Self::Enablement
    }
}

impl Default for enums::VsanEncryptionTransitionStateEnum {
    fn default() -> Self {
        Self::Settled
    }
}

impl Default for enums::VsanHostHealthStateEnum {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for enums::VsanHostNodeStateEnum {
    fn default() -> Self {
        Self::Error
    }
}

impl Default for enums::VimVsanHostTrafficTypeEnum {
    fn default() -> Self {
        Self::Vsan
    }
}

impl Default for enums::VimVsanHostTrafficType90Enum {
    fn default() -> Self {
        Self::VsanExternal
    }
}

impl Default for enums::ServerNodeTypeEnum {
    fn default() -> Self {
        Self::NodeTypeUnknown
    }
}

impl Default for enums::VsanHostStatsTypeEnum {
    fn default() -> Self {
        Self::ResyncIopsInfo
    }
}

impl Default for enums::StoragePoolDiskTypeEnum {
    fn default() -> Self {
        Self::SingleTier
    }
}

impl Default for enums::TrimDiskTypeEnum {
    fn default() -> Self {
        Self::CacheDisk
    }
}

impl Default for enums::VsanDiskEvacReasonEnum {
    fn default() -> Self {
        Self::User
    }
}

impl Default for enums::VsanDiskTrimOptionEnum {
    fn default() -> Self {
        Self::MetaDataOnly
    }
}

impl Default for enums::VsanDiskTypeEnum {
    fn default() -> Self {
        Self::DiskGroup
    }
}

impl Default for enums::VsanDiskgroupCapabilityEnum {
    fn default() -> Self {
        Self::SupportLargerThan16Tb
    }
}

impl Default for enums::VsanDiskIssueTypeEnum {
    fn default() -> Self {
        Self::NonExist
    }
}

impl Default for enums::VsanHostWipeDiskEligibleEnum {
    fn default() -> Self {
        Self::WipeDiskEligibleUnknown
    }
}

impl Default for enums::VsanHostWipeDiskStateEnum {
    fn default() -> Self {
        Self::WipeDiskStateUnknown
    }
}

impl Default for enums::BaseConfigInfoDiskFileBackingInfoProvisioningTypeEnum {
    fn default() -> Self {
        Self::Thin
    }
}

impl Default for enums::VslmDiskInfoFlagEnum {
    fn default() -> Self {
        Self::Id
    }
}

impl Default for enums::VStorageObjectConsumptionTypeEnum {
    fn default() -> Self {
        Self::Disk
    }
}

impl Default for enums::VslmVStorageObjectControlFlagEnum {
    fn default() -> Self {
        Self::KeepAfterDeleteVm
    }
}

impl Default for enums::PropertyChangeOpEnum {
    fn default() -> Self {
        Self::Add
    }
}

impl Default for enums::ObjectUpdateKindEnum {
    fn default() -> Self {
        Self::Modify
    }
}

impl Default for enums::VslmTaskInfoStateEnum {
    fn default() -> Self {
        Self::Queued
    }
}

impl Default for enums::VslmEventTypeEnum {
    fn default() -> Self {
        Self::PreFcdMigrateEvent
    }
}

impl Default for enums::VslmEventVslmEventInfoStateEnum {
    fn default() -> Self {
        Self::Success
    }
}

impl Default for enums::VslmVsoVStorageObjectQuerySpecQueryFieldEnumEnum {
    fn default() -> Self {
        Self::Id
    }
}

impl Default for enums::VslmVsoVStorageObjectQuerySpecQueryOperatorEnumEnum {
    fn default() -> Self {
        Self::Equals
    }
}

impl Default for structs::ManagedObjectReference {
    fn default() -> Self {
        Self {
            r#type: enums::MoTypesEnum::default(),
            value: String::new(),
        }
    }
}

impl Default for structs::DataObject {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::AgencyConfigInfo {
    fn default() -> Self {
        Self {
            agent_config: None,
            scope: None,
            manually_mark_agent_vm_available_after_provisioning: None,
            manually_mark_agent_vm_available_after_power_on: None,
            optimized_deployment_enabled: None,
            agent_name: None,
            agency_name: None,
            use_uuid_vm_name: None,
            manually_provisioned: None,
            manually_monitored: None,
            bypass_vum_enabled: None,
            agent_vm_network: None,
            agent_vm_datastore: None,
            prefer_host_configuration: None,
            ip_pool: None,
            resource_pools: None,
            folders: None,
        }
    }
}

impl Default for structs::AgencyScope {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::AgencyComputeResourceScope {
    fn default() -> Self {
        Self {
            compute_resource: None,
        }
    }
}

impl Default for structs::AgencyVmFolder {
    fn default() -> Self {
        Self {
            folder_id: structs::ManagedObjectReference::default(),
            datacenter_id: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::AgencyVmResourcePool {
    fn default() -> Self {
        Self {
            resource_pool_id: structs::ManagedObjectReference::default(),
            compute_resource_id: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::AgentConfigInfo {
    fn default() -> Self {
        Self {
            product_line_id: None,
            host_version: None,
            ovf_package_url: None,
            authentication_scheme: None,
            ovf_ssl_trust: None,
            ovf_environment: None,
            vib_url: None,
            vib_ssl_trust: None,
            vib_matching_rules: None,
            vib_name: None,
            dv_filter_enabled: None,
            reboot_host_after_vib_uninstall: None,
            vmci_service: None,
            ovf_disk_provisioning: None,
            vm_storage_policies: None,
            vm_resource_configuration: None,
        }
    }
}

impl Default for structs::AgentOvfEnvironmentInfo {
    fn default() -> Self {
        Self {
            ovf_property: None,
        }
    }
}

impl Default for structs::AgentOvfEnvironmentInfoOvfProperty {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: String::new(),
        }
    }
}

impl Default for structs::AgentSslTrust {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::AgentAnyCertificate {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::AgentPinnedPemCertificate {
    fn default() -> Self {
        Self {
            ssl_certificate: String::new(),
        }
    }
}

impl Default for structs::AgentStoragePolicy {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::AgentVsanStoragePolicy {
    fn default() -> Self {
        Self {
            profile_id: String::new(),
        }
    }
}

impl Default for structs::AgentVibMatchingRule {
    fn default() -> Self {
        Self {
            vib_name_regex: String::new(),
            vib_version_regex: String::new(),
        }
    }
}

impl Default for structs::AgentVmHook {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            vm_state: String::new(),
        }
    }
}

impl Default for structs::EamObjectRuntimeInfo {
    fn default() -> Self {
        Self {
            status: String::new(),
            issue: None,
            goal_state: String::new(),
            entity: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::AgentRuntimeInfo {
    fn default() -> Self {
        Self {
            eam_object_runtime_info_: structs::EamObjectRuntimeInfo::default(),
            vm_power_state: enums::VirtualMachinePowerStateEnum::default(),
            receiving_heart_beat: false,
            host: None,
            vm: None,
            vm_ip: None,
            vm_name: String::new(),
            esx_agent_resource_pool: None,
            esx_agent_folder: None,
            installed_bulletin: None,
            installed_vibs: None,
            agency: None,
            vm_hook: None,
        }
    }
}

impl Default for structs::Issue {
    fn default() -> Self {
        Self {
            key: 0,
            description: String::new(),
            time: String::new(),
        }
    }
}

impl Default for structs::AgencyIssue {
    fn default() -> Self {
        Self {
            issue_: structs::Issue::default(),
            agency: structs::ManagedObjectReference::default(),
            agency_name: String::new(),
            solution_id: String::new(),
            solution_name: String::new(),
        }
    }
}

impl Default for structs::AgencyDisabled {
    fn default() -> Self {
        Self {
            agency_issue_: structs::AgencyIssue::default(),
        }
    }
}

impl Default for structs::AgentIssue {
    fn default() -> Self {
        Self {
            agency_issue_: structs::AgencyIssue::default(),
            agent: structs::ManagedObjectReference::default(),
            agent_name: String::new(),
            host: structs::ManagedObjectReference::default(),
            host_name: String::new(),
        }
    }
}

impl Default for structs::EamCertificateNotTrusted {
    fn default() -> Self {
        Self {
            agent_issue_: structs::AgentIssue::default(),
            url: String::new(),
        }
    }
}

impl Default for structs::HostInPartialMaintenanceMode {
    fn default() -> Self {
        Self {
            agent_issue_: structs::AgentIssue::default(),
            vm: None,
        }
    }
}

impl Default for structs::ManagedHostNotReachable {
    fn default() -> Self {
        Self {
            agent_issue_: structs::AgentIssue::default(),
        }
    }
}

impl Default for structs::MissingDvFilterSwitch {
    fn default() -> Self {
        Self {
            agent_issue_: structs::AgentIssue::default(),
        }
    }
}

impl Default for structs::OvfInvalidProperty {
    fn default() -> Self {
        Self {
            agent_issue_: structs::AgentIssue::default(),
            error: None,
        }
    }
}

impl Default for structs::TransitionFailed {
    fn default() -> Self {
        Self {
            agent_issue_: structs::AgentIssue::default(),
        }
    }
}

impl Default for structs::VibIssue {
    fn default() -> Self {
        Self {
            agent_issue_: structs::AgentIssue::default(),
        }
    }
}

impl Default for structs::ImmediateHostRebootRequired {
    fn default() -> Self {
        Self {
            vib_issue_: structs::VibIssue::default(),
        }
    }
}

impl Default for structs::VibCannotPutHostInMaintenanceMode {
    fn default() -> Self {
        Self {
            vib_issue_: structs::VibIssue::default(),
        }
    }
}

impl Default for structs::VibCannotPutHostOutOfMaintenanceMode {
    fn default() -> Self {
        Self {
            vib_issue_: structs::VibIssue::default(),
        }
    }
}

impl Default for structs::VibNotInstalled {
    fn default() -> Self {
        Self {
            vib_issue_: structs::VibIssue::default(),
        }
    }
}

impl Default for structs::CannotAccessAgentVib {
    fn default() -> Self {
        Self {
            vib_not_installed_: structs::VibNotInstalled::default(),
            download_url: String::new(),
        }
    }
}

impl Default for structs::VibDependenciesNotMetByHost {
    fn default() -> Self {
        Self {
            vib_not_installed_: structs::VibNotInstalled::default(),
        }
    }
}

impl Default for structs::VibInvalidFormat {
    fn default() -> Self {
        Self {
            vib_not_installed_: structs::VibNotInstalled::default(),
        }
    }
}

impl Default for structs::VibRequirementsNotMetByHost {
    fn default() -> Self {
        Self {
            vib_not_installed_: structs::VibNotInstalled::default(),
        }
    }
}

impl Default for structs::VibRequiresHostInMaintenanceMode {
    fn default() -> Self {
        Self {
            vib_issue_: structs::VibIssue::default(),
        }
    }
}

impl Default for structs::VibRequiresHostReboot {
    fn default() -> Self {
        Self {
            vib_issue_: structs::VibIssue::default(),
        }
    }
}

impl Default for structs::VibRequiresManualInstallation {
    fn default() -> Self {
        Self {
            vib_issue_: structs::VibIssue::default(),
            bulletin: Vec::new(),
        }
    }
}

impl Default for structs::VibRequiresManualUninstallation {
    fn default() -> Self {
        Self {
            vib_issue_: structs::VibIssue::default(),
            bulletin: Vec::new(),
        }
    }
}

impl Default for structs::VmIssue {
    fn default() -> Self {
        Self {
            agent_issue_: structs::AgentIssue::default(),
            vm: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::InvalidConfig {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
            error: Default::default(),
        }
    }
}

impl Default for structs::VmCorrupted {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
            missing_file: None,
        }
    }
}

impl Default for structs::VmDeployed {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
        }
    }
}

impl Default for structs::HostInMaintenanceMode {
    fn default() -> Self {
        Self {
            vm_deployed_: structs::VmDeployed::default(),
        }
    }
}

impl Default for structs::HostInStandbyMode {
    fn default() -> Self {
        Self {
            vm_deployed_: structs::VmDeployed::default(),
        }
    }
}

impl Default for structs::HostPoweredOff {
    fn default() -> Self {
        Self {
            vm_deployed_: structs::VmDeployed::default(),
        }
    }
}

impl Default for structs::VmHookFailed {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
        }
    }
}

impl Default for structs::VmHookTimedout {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
        }
    }
}

impl Default for structs::VmInaccessible {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
        }
    }
}

impl Default for structs::VmMarkedAsTemplate {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
        }
    }
}

impl Default for structs::VmOrphaned {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
        }
    }
}

impl Default for structs::VmPoweredOff {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
        }
    }
}

impl Default for structs::InsufficientIpAddresses {
    fn default() -> Self {
        Self {
            vm_powered_off_: structs::VmPoweredOff::default(),
            network: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::MissingAgentIpPool {
    fn default() -> Self {
        Self {
            vm_powered_off_: structs::VmPoweredOff::default(),
            network: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VmPoweredOn {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
        }
    }
}

impl Default for structs::VmProtected {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
        }
    }
}

impl Default for structs::VmSuspended {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
        }
    }
}

impl Default for structs::VmWrongFolder {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
            current_folder: structs::ManagedObjectReference::default(),
            required_folder: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VmWrongResourcePool {
    fn default() -> Self {
        Self {
            vm_issue_: structs::VmIssue::default(),
            current_resource_pool: structs::ManagedObjectReference::default(),
            required_resource_pool: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VmNotDeployed {
    fn default() -> Self {
        Self {
            agent_issue_: structs::AgentIssue::default(),
        }
    }
}

impl Default for structs::CannotAccessAgentOvf {
    fn default() -> Self {
        Self {
            vm_not_deployed_: structs::VmNotDeployed::default(),
            download_url: String::new(),
        }
    }
}

impl Default for structs::IncompatibleHostVersion {
    fn default() -> Self {
        Self {
            vm_not_deployed_: structs::VmNotDeployed::default(),
        }
    }
}

impl Default for structs::InsufficientResources {
    fn default() -> Self {
        Self {
            vm_not_deployed_: structs::VmNotDeployed::default(),
        }
    }
}

impl Default for structs::InsufficientSpace {
    fn default() -> Self {
        Self {
            vm_not_deployed_: structs::VmNotDeployed::default(),
        }
    }
}

impl Default for structs::NoAgentVmDatastore {
    fn default() -> Self {
        Self {
            vm_not_deployed_: structs::VmNotDeployed::default(),
        }
    }
}

impl Default for structs::NoCustomAgentVmDatastore {
    fn default() -> Self {
        Self {
            no_agent_vm_datastore_: structs::NoAgentVmDatastore::default(),
            custom_agent_vm_datastore: Vec::new(),
            custom_agent_vm_datastore_name: Vec::new(),
        }
    }
}

impl Default for structs::NoAgentVmNetwork {
    fn default() -> Self {
        Self {
            vm_not_deployed_: structs::VmNotDeployed::default(),
        }
    }
}

impl Default for structs::NoCustomAgentVmNetwork {
    fn default() -> Self {
        Self {
            no_agent_vm_network_: structs::NoAgentVmNetwork::default(),
            custom_agent_vm_network: Vec::new(),
            custom_agent_vm_network_name: Vec::new(),
        }
    }
}

impl Default for structs::NoDiscoverableAgentVmDatastore {
    fn default() -> Self {
        Self {
            vm_not_deployed_: structs::VmNotDeployed::default(),
        }
    }
}

impl Default for structs::NoDiscoverableAgentVmNetwork {
    fn default() -> Self {
        Self {
            vm_not_deployed_: structs::VmNotDeployed::default(),
        }
    }
}

impl Default for structs::OvfInvalidFormat {
    fn default() -> Self {
        Self {
            vm_not_deployed_: structs::VmNotDeployed::default(),
            error: None,
        }
    }
}

impl Default for structs::VmRequiresHostOutOfMaintenanceMode {
    fn default() -> Self {
        Self {
            vm_not_deployed_: structs::VmNotDeployed::default(),
        }
    }
}

impl Default for structs::PersonalityAgentPmIssue {
    fn default() -> Self {
        Self {
            agent_issue_: structs::AgentIssue::default(),
        }
    }
}

impl Default for structs::PersonalityAgentAwaitingPmRemediation {
    fn default() -> Self {
        Self {
            personality_agent_pm_issue_: structs::PersonalityAgentPmIssue::default(),
        }
    }
}

impl Default for structs::PersonalityAgentBlockedByAgencyOperation {
    fn default() -> Self {
        Self {
            personality_agent_pm_issue_: structs::PersonalityAgentPmIssue::default(),
        }
    }
}

impl Default for structs::OrphanedAgency {
    fn default() -> Self {
        Self {
            agency_issue_: structs::AgencyIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentAgentIssue {
    fn default() -> Self {
        Self {
            agency_issue_: structs::AgencyIssue::default(),
            agent: structs::ManagedObjectReference::default(),
            cluster: None,
        }
    }
}

impl Default for structs::ClusterAgentOvfInvalidProperty {
    fn default() -> Self {
        Self {
            cluster_agent_agent_issue_: structs::ClusterAgentAgentIssue::default(),
            error: None,
        }
    }
}

impl Default for structs::ClusterAgentTransitionFailed {
    fn default() -> Self {
        Self {
            cluster_agent_agent_issue_: structs::ClusterAgentAgentIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentVmIssue {
    fn default() -> Self {
        Self {
            cluster_agent_agent_issue_: structs::ClusterAgentAgentIssue::default(),
            vm: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::ClusterAgentHostInMaintenanceMode {
    fn default() -> Self {
        Self {
            cluster_agent_vm_issue_: structs::ClusterAgentVmIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentHostInPartialMaintenanceMode {
    fn default() -> Self {
        Self {
            cluster_agent_vm_issue_: structs::ClusterAgentVmIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentInvalidConfig {
    fn default() -> Self {
        Self {
            cluster_agent_vm_issue_: structs::ClusterAgentVmIssue::default(),
            error: Default::default(),
        }
    }
}

impl Default for structs::ClusterAgentVmHookFailed {
    fn default() -> Self {
        Self {
            cluster_agent_vm_issue_: structs::ClusterAgentVmIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentVmHookTimedout {
    fn default() -> Self {
        Self {
            cluster_agent_vm_issue_: structs::ClusterAgentVmIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentVmInaccessible {
    fn default() -> Self {
        Self {
            cluster_agent_vm_issue_: structs::ClusterAgentVmIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentVmNotRemoved {
    fn default() -> Self {
        Self {
            cluster_agent_vm_issue_: structs::ClusterAgentVmIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentVmPoweredOff {
    fn default() -> Self {
        Self {
            cluster_agent_vm_issue_: structs::ClusterAgentVmIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentInsufficientClusterResources {
    fn default() -> Self {
        Self {
            cluster_agent_vm_powered_off_: structs::ClusterAgentVmPoweredOff::default(),
        }
    }
}

impl Default for structs::ClusterAgentVmPoweredOn {
    fn default() -> Self {
        Self {
            cluster_agent_vm_issue_: structs::ClusterAgentVmIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentVmProtected {
    fn default() -> Self {
        Self {
            cluster_agent_vm_issue_: structs::ClusterAgentVmIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentVmSuspended {
    fn default() -> Self {
        Self {
            cluster_agent_vm_issue_: structs::ClusterAgentVmIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentVmNotDeployed {
    fn default() -> Self {
        Self {
            cluster_agent_agent_issue_: structs::ClusterAgentAgentIssue::default(),
        }
    }
}

impl Default for structs::ClusterAgentCertificateNotTrusted {
    fn default() -> Self {
        Self {
            cluster_agent_vm_not_deployed_: structs::ClusterAgentVmNotDeployed::default(),
            url: String::new(),
        }
    }
}

impl Default for structs::ClusterAgentInsufficientClusterSpace {
    fn default() -> Self {
        Self {
            cluster_agent_vm_not_deployed_: structs::ClusterAgentVmNotDeployed::default(),
        }
    }
}

impl Default for structs::ClusterAgentMissingClusterVmDatastore {
    fn default() -> Self {
        Self {
            cluster_agent_vm_not_deployed_: structs::ClusterAgentVmNotDeployed::default(),
            missing_datastores: None,
        }
    }
}

impl Default for structs::ClusterAgentMissingClusterVmNetwork {
    fn default() -> Self {
        Self {
            cluster_agent_vm_not_deployed_: structs::ClusterAgentVmNotDeployed::default(),
            missing_networks: None,
            network_names: None,
        }
    }
}

impl Default for structs::IntegrityAgencyVumIssue {
    fn default() -> Self {
        Self {
            agency_issue_: structs::AgencyIssue::default(),
        }
    }
}

impl Default for structs::IntegrityAgencyCannotDeleteSoftware {
    fn default() -> Self {
        Self {
            integrity_agency_vum_issue_: structs::IntegrityAgencyVumIssue::default(),
        }
    }
}

impl Default for structs::IntegrityAgencyCannotStageSoftware {
    fn default() -> Self {
        Self {
            integrity_agency_vum_issue_: structs::IntegrityAgencyVumIssue::default(),
        }
    }
}

impl Default for structs::IntegrityAgencyVumUnavailable {
    fn default() -> Self {
        Self {
            integrity_agency_vum_issue_: structs::IntegrityAgencyVumIssue::default(),
        }
    }
}

impl Default for structs::PersonalityAgencyPmIssue {
    fn default() -> Self {
        Self {
            agency_issue_: structs::AgencyIssue::default(),
        }
    }
}

impl Default for structs::PersonalityAgencyCannotConfigureSolutions {
    fn default() -> Self {
        Self {
            personality_agency_pm_issue_: structs::PersonalityAgencyPmIssue::default(),
            cr: structs::ManagedObjectReference::default(),
            solutions_to_modify: None,
            solutions_to_remove: None,
        }
    }
}

impl Default for structs::PersonalityAgencyDepotIssue {
    fn default() -> Self {
        Self {
            personality_agency_pm_issue_: structs::PersonalityAgencyPmIssue::default(),
            remote_depot_url: String::new(),
        }
    }
}

impl Default for structs::PersonalityAgencyCannotUploadDepot {
    fn default() -> Self {
        Self {
            personality_agency_depot_issue_: structs::PersonalityAgencyDepotIssue::default(),
            local_depot_url: String::new(),
        }
    }
}

impl Default for structs::PersonalityAgencyInaccessibleDepot {
    fn default() -> Self {
        Self {
            personality_agency_depot_issue_: structs::PersonalityAgencyDepotIssue::default(),
        }
    }
}

impl Default for structs::PersonalityAgencyInvalidDepot {
    fn default() -> Self {
        Self {
            personality_agency_depot_issue_: structs::PersonalityAgencyDepotIssue::default(),
        }
    }
}

impl Default for structs::PersonalityAgencyPmUnavailable {
    fn default() -> Self {
        Self {
            personality_agency_pm_issue_: structs::PersonalityAgencyPmIssue::default(),
        }
    }
}

impl Default for structs::ExtensibleIssue {
    fn default() -> Self {
        Self {
            issue_: structs::Issue::default(),
            type_id: String::new(),
            argument: None,
            target: None,
            agent: None,
            agency: None,
        }
    }
}

impl Default for structs::HostIssue {
    fn default() -> Self {
        Self {
            issue_: structs::Issue::default(),
            host: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::OrphanedDvFilterSwitch {
    fn default() -> Self {
        Self {
            host_issue_: structs::HostIssue::default(),
        }
    }
}

impl Default for structs::UnknownAgentVm {
    fn default() -> Self {
        Self {
            host_issue_: structs::HostIssue::default(),
            vm: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::HooksHookListSpec {
    fn default() -> Self {
        Self {
            solutions: None,
            hosts: None,
        }
    }
}

impl Default for structs::HooksMarkAsProcessedSpec {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            hook_type: String::new(),
            success: false,
        }
    }
}

impl Default for structs::SolutionsApplySpec {
    fn default() -> Self {
        Self {
            desired_state: None,
            solutions: None,
            hosts: None,
            deployment_units: None,
        }
    }
}

impl Default for structs::SolutionsClusterSolutionComplianceResult {
    fn default() -> Self {
        Self {
            solution: String::new(),
            compliant: false,
            deployment_units: None,
        }
    }
}

impl Default for structs::SolutionsComplianceResult {
    fn default() -> Self {
        Self {
            compliant: false,
            hosts: None,
            cluster_solutions_compliance: None,
        }
    }
}

impl Default for structs::SolutionsComplianceSpec {
    fn default() -> Self {
        Self {
            desired_state: None,
            solutions: None,
            hosts: None,
            deployment_units: None,
        }
    }
}

impl Default for structs::SolutionsDeploymentUnitComplianceResult {
    fn default() -> Self {
        Self {
            deployment_unit: String::new(),
            compliant: false,
            compliance: None,
        }
    }
}

impl Default for structs::SolutionsHookAcknowledgeConfig {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::SolutionsInteractiveHookAcknowledgeConfig {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::SolutionsHookConfig {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            acknowledgement: Default::default(),
            timeout: None,
        }
    }
}

impl Default for structs::SolutionsHookInfo {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            solution: String::new(),
            config: structs::SolutionsHookConfig::default(),
            raised_at: String::new(),
        }
    }
}

impl Default for structs::SolutionsHostComplianceResult {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            compliant: false,
            solutions: None,
        }
    }
}

impl Default for structs::SolutionsOvfProperty {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: String::new(),
        }
    }
}

impl Default for structs::SolutionsSolutionComplianceResult {
    fn default() -> Self {
        Self {
            solution: String::new(),
            compliant: false,
            non_compliance_reason: None,
            vm: None,
            upgrading_vm: None,
            hook: None,
            issues: None,
            solution_config: None,
        }
    }
}

impl Default for structs::SolutionsSolutionConfig {
    fn default() -> Self {
        Self {
            solution: String::new(),
            display_name: String::new(),
            display_version: String::new(),
            vm_source: Default::default(),
            prefix_vm_name: String::new(),
            uuid_vm_name: false,
            resource_pool: None,
            folder: None,
            ovf_properties: None,
            storage_policies: None,
            vm_disk_provisioning: None,
            vm_deployment_optimization: None,
            type_specific_config: Default::default(),
            hooks: None,
            vm_resource_spec: None,
        }
    }
}

impl Default for structs::SolutionsSolutionValidationResult {
    fn default() -> Self {
        Self {
            solution: String::new(),
            valid: false,
            invalid_reason: None,
        }
    }
}

impl Default for structs::SolutionsStoragePolicy {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::SolutionsProfileIdStoragePolicy {
    fn default() -> Self {
        Self {
            profile_id: String::new(),
        }
    }
}

impl Default for structs::SolutionsTransitionSpec {
    fn default() -> Self {
        Self {
            solution: String::new(),
            agency_id: String::new(),
        }
    }
}

impl Default for structs::SolutionsTypeSpecificSolutionConfig {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::SolutionsClusterBoundSolutionConfig {
    fn default() -> Self {
        Self {
            vm_count: 0,
            vm_placement_policies: None,
            vm_networks: None,
            datastores: None,
            devices: None,
        }
    }
}

impl Default for structs::SolutionsHostBoundSolutionConfig {
    fn default() -> Self {
        Self {
            prefer_host_configuration: None,
            networks: None,
            datastores: None,
            vmci: None,
        }
    }
}

impl Default for structs::SolutionsVmNetworkMapping {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::SolutionsVmSource {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::SolutionsUrlVmSource {
    fn default() -> Self {
        Self {
            ovf_url: String::new(),
            certificate_validation: None,
            certificate_pem: None,
        }
    }
}

impl Default for structs::SolutionsValidateSpec {
    fn default() -> Self {
        Self {
            desired_state: Vec::new(),
            transition_spec: None,
        }
    }
}

impl Default for structs::SolutionsValidationResult {
    fn default() -> Self {
        Self {
            valid: false,
            solution_result: None,
        }
    }
}

impl Default for structs::SolutionsVmResourceSpec {
    fn default() -> Self {
        Self {
            ovf_deployment_option: None,
        }
    }
}

impl Default for structs::VibVibInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            version: String::new(),
            vendor: String::new(),
            summary: String::new(),
            software_tags: None,
            release_date: String::new(),
        }
    }
}

impl Default for structs::VibVibInfoSoftwareTags {
    fn default() -> Self {
        Self {
            tags: None,
        }
    }
}

impl Default for structs::VibVibServicesSslTrust {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VibVibServicesAnyCertificate {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VibVibServicesPinnedPemCertificate {
    fn default() -> Self {
        Self {
            ssl_certificate: String::new(),
        }
    }
}

impl Default for structs::PbmAboutInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            instance_uuid: String::new(),
        }
    }
}

impl Default for structs::PbmExtendedElementDescription {
    fn default() -> Self {
        Self {
            label: String::new(),
            summary: String::new(),
            key: String::new(),
            message_catalog_key_prefix: String::new(),
            message_arg: None,
        }
    }
}

impl Default for structs::PbmLoggingConfiguration {
    fn default() -> Self {
        Self {
            component: String::new(),
            log_level: String::new(),
        }
    }
}

impl Default for structs::PbmServerObjectRef {
    fn default() -> Self {
        Self {
            object_type: String::new(),
            key: String::new(),
            server_uuid: None,
        }
    }
}

impl Default for structs::PbmServiceInstanceContent {
    fn default() -> Self {
        Self {
            about_info: structs::PbmAboutInfo::default(),
            session_manager: structs::ManagedObjectReference::default(),
            capability_metadata_manager: structs::ManagedObjectReference::default(),
            profile_manager: structs::ManagedObjectReference::default(),
            compliance_manager: structs::ManagedObjectReference::default(),
            placement_solver: structs::ManagedObjectReference::default(),
            replication_manager: None,
        }
    }
}

impl Default for structs::PbmCapabilityInstance {
    fn default() -> Self {
        Self {
            id: structs::PbmCapabilityMetadataUniqueId::default(),
            constraint: Vec::new(),
        }
    }
}

impl Default for structs::PbmCapabilityMetadata {
    fn default() -> Self {
        Self {
            id: structs::PbmCapabilityMetadataUniqueId::default(),
            summary: structs::PbmExtendedElementDescription::default(),
            mandatory: None,
            hint: None,
            key_id: None,
            allow_multiple_constraints: None,
            property_metadata: Vec::new(),
        }
    }
}

impl Default for structs::PbmCapabilityMetadataUniqueId {
    fn default() -> Self {
        Self {
            namespace: String::new(),
            id: String::new(),
        }
    }
}

impl Default for structs::PbmCapabilityConstraintInstance {
    fn default() -> Self {
        Self {
            property_instance: Vec::new(),
        }
    }
}

impl Default for structs::PbmCapabilityPropertyInstance {
    fn default() -> Self {
        Self {
            id: String::new(),
            operator: None,
            value: Default::default(),
        }
    }
}

impl Default for structs::PbmCapabilityPropertyMetadata {
    fn default() -> Self {
        Self {
            id: String::new(),
            summary: structs::PbmExtendedElementDescription::default(),
            mandatory: false,
            r#type: None,
            default_value: None,
            allowed_value: None,
            requirements_type_hint: None,
        }
    }
}

impl Default for structs::PbmCapabilityTypeInfo {
    fn default() -> Self {
        Self {
            type_name: String::new(),
        }
    }
}

impl Default for structs::PbmCapabilityGenericTypeInfo {
    fn default() -> Self {
        Self {
            pbm_capability_type_info_: structs::PbmCapabilityTypeInfo::default(),
            generic_type_name: String::new(),
        }
    }
}

impl Default for structs::PbmCapabilityMetadataPerCategory {
    fn default() -> Self {
        Self {
            sub_category: String::new(),
            capability_metadata: Vec::new(),
        }
    }
}

impl Default for structs::PbmCapabilitySchema {
    fn default() -> Self {
        Self {
            vendor_info: structs::PbmCapabilitySchemaVendorInfo::default(),
            namespace_info: structs::PbmCapabilityNamespaceInfo::default(),
            line_of_service: None,
            capability_metadata_per_category: Vec::new(),
            capability_category: None,
        }
    }
}

impl Default for structs::PbmCapabilityNamespaceInfo {
    fn default() -> Self {
        Self {
            version: String::new(),
            namespace: String::new(),
            info: None,
        }
    }
}

impl Default for structs::PbmCapabilitySchemaVendorInfo {
    fn default() -> Self {
        Self {
            vendor_uuid: String::new(),
            info: structs::PbmExtendedElementDescription::default(),
        }
    }
}

impl Default for structs::PbmCapabilityVendorNamespaceInfo {
    fn default() -> Self {
        Self {
            vendor_info: structs::PbmCapabilitySchemaVendorInfo::default(),
            namespace_info: structs::PbmCapabilityNamespaceInfo::default(),
        }
    }
}

impl Default for structs::PbmCapabilityVendorResourceTypeInfo {
    fn default() -> Self {
        Self {
            resource_type: String::new(),
            vendor_namespace_info: Vec::new(),
        }
    }
}

impl Default for structs::PbmLineOfServiceInfo {
    fn default() -> Self {
        Self {
            line_of_service: String::new(),
            name: structs::PbmExtendedElementDescription::default(),
            description: None,
        }
    }
}

impl Default for structs::PbmPersistenceBasedDataServiceInfo {
    fn default() -> Self {
        Self {
            pbm_line_of_service_info_: structs::PbmLineOfServiceInfo::default(),
            compatible_persistence_schema_namespace: None,
        }
    }
}

impl Default for structs::PbmVaioDataServiceInfo {
    fn default() -> Self {
        Self {
            pbm_line_of_service_info_: structs::PbmLineOfServiceInfo::default(),
        }
    }
}

impl Default for structs::PbmCapabilityDescription {
    fn default() -> Self {
        Self {
            description: structs::PbmExtendedElementDescription::default(),
            value: Default::default(),
        }
    }
}

impl Default for structs::PbmCapabilityDiscreteSet {
    fn default() -> Self {
        Self {
            values: Vec::new(),
        }
    }
}

impl Default for structs::PbmCapabilityRange {
    fn default() -> Self {
        Self {
            min: Default::default(),
            max: Default::default(),
        }
    }
}

impl Default for structs::PbmCapabilityTimeSpan {
    fn default() -> Self {
        Self {
            value: 0,
            unit: String::new(),
        }
    }
}

impl Default for structs::PbmComplianceResult {
    fn default() -> Self {
        Self {
            check_time: String::new(),
            entity: structs::PbmServerObjectRef::default(),
            profile: None,
            compliance_task_status: None,
            compliance_status: String::new(),
            mismatch: false,
            violated_policies: None,
            error_cause: None,
            operational_status: None,
            info: None,
        }
    }
}

impl Default for structs::PbmFetchEntityHealthStatusSpec {
    fn default() -> Self {
        Self {
            object_ref: structs::PbmServerObjectRef::default(),
            backing_id: None,
        }
    }
}

impl Default for structs::PbmComplianceOperationalStatus {
    fn default() -> Self {
        Self {
            healthy: None,
            operation_eta: None,
            operation_progress: None,
            transitional: None,
        }
    }
}

impl Default for structs::PbmCompliancePolicyStatus {
    fn default() -> Self {
        Self {
            expected_value: structs::PbmCapabilityInstance::default(),
            current_value: None,
        }
    }
}

impl Default for structs::PbmRollupComplianceResult {
    fn default() -> Self {
        Self {
            oldest_check_time: String::new(),
            entity: structs::PbmServerObjectRef::default(),
            overall_compliance_status: String::new(),
            overall_compliance_task_status: None,
            result: None,
            error_cause: None,
            profile_mismatch: false,
        }
    }
}

impl Default for structs::PbmFaultNoPermissionEntityPrivileges {
    fn default() -> Self {
        Self {
            profile_id: None,
            privilege_ids: None,
        }
    }
}

impl Default for structs::PbmPlacementCompatibilityResult {
    fn default() -> Self {
        Self {
            hub: structs::PbmPlacementHub::default(),
            matching_resources: None,
            how_many: None,
            utilization: None,
            warning: None,
            error: None,
        }
    }
}

impl Default for structs::PbmPlacementMatchingResources {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::PbmPlacementMatchingReplicationResources {
    fn default() -> Self {
        Self {
            replication_group: None,
        }
    }
}

impl Default for structs::PbmPlacementHub {
    fn default() -> Self {
        Self {
            hub_type: String::new(),
            hub_id: String::new(),
        }
    }
}

impl Default for structs::PbmPlacementRequirement {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::PbmPlacementCapabilityConstraintsRequirement {
    fn default() -> Self {
        Self {
            constraints: Default::default(),
        }
    }
}

impl Default for structs::PbmPlacementCapabilityProfileRequirement {
    fn default() -> Self {
        Self {
            profile_id: structs::PbmProfileId::default(),
        }
    }
}

impl Default for structs::PbmPlacementResourceUtilization {
    fn default() -> Self {
        Self {
            name: structs::PbmExtendedElementDescription::default(),
            description: structs::PbmExtendedElementDescription::default(),
            available_before: None,
            available_after: None,
            total: None,
        }
    }
}

impl Default for structs::PbmCapabilityProfileCreateSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: None,
            category: None,
            resource_type: structs::PbmProfileResourceType::default(),
            constraints: Default::default(),
        }
    }
}

impl Default for structs::PbmCapabilityProfileUpdateSpec {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            constraints: None,
        }
    }
}

impl Default for structs::PbmCapabilityConstraints {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::PbmCapabilitySubProfileConstraints {
    fn default() -> Self {
        Self {
            sub_profiles: Vec::new(),
        }
    }
}

impl Default for structs::PbmDataServiceToPoliciesMap {
    fn default() -> Self {
        Self {
            data_service_policy: structs::PbmProfileId::default(),
            parent_storage_policies: None,
            fault: None,
        }
    }
}

impl Default for structs::PbmDefaultProfileInfo {
    fn default() -> Self {
        Self {
            datastores: Vec::new(),
            default_profile: None,
            method_fault: None,
        }
    }
}

impl Default for structs::PbmProfile {
    fn default() -> Self {
        Self {
            profile_id: structs::PbmProfileId::default(),
            name: String::new(),
            description: None,
            creation_time: String::new(),
            created_by: String::new(),
            last_updated_time: String::new(),
            last_updated_by: String::new(),
        }
    }
}

impl Default for structs::PbmCapabilityProfile {
    fn default() -> Self {
        Self {
            pbm_profile_: structs::PbmProfile::default(),
            profile_category: String::new(),
            resource_type: structs::PbmProfileResourceType::default(),
            constraints: Default::default(),
            generation_id: None,
            is_default: false,
            system_created_profile_type: None,
            line_of_service: None,
        }
    }
}

impl Default for structs::PbmDefaultCapabilityProfile {
    fn default() -> Self {
        Self {
            pbm_capability_profile_: structs::PbmCapabilityProfile::default(),
            vvol_type: Vec::new(),
            container_id: String::new(),
        }
    }
}

impl Default for structs::PbmProfileId {
    fn default() -> Self {
        Self {
            unique_id: String::new(),
        }
    }
}

impl Default for structs::PbmProfileOperationOutcome {
    fn default() -> Self {
        Self {
            profile_id: structs::PbmProfileId::default(),
            fault: None,
        }
    }
}

impl Default for structs::PbmProfileType {
    fn default() -> Self {
        Self {
            unique_id: String::new(),
        }
    }
}

impl Default for structs::PbmQueryProfileResult {
    fn default() -> Self {
        Self {
            object: structs::PbmServerObjectRef::default(),
            profile_id: None,
            fault: None,
        }
    }
}

impl Default for structs::PbmProfileResourceType {
    fn default() -> Self {
        Self {
            resource_type: String::new(),
        }
    }
}

impl Default for structs::PbmCapabilitySubProfile {
    fn default() -> Self {
        Self {
            name: String::new(),
            capability: Vec::new(),
            force_provision: None,
        }
    }
}

impl Default for structs::PbmDatastoreSpaceStatistics {
    fn default() -> Self {
        Self {
            profile_id: None,
            physical_total_in_mb: 0,
            physical_free_in_mb: 0,
            physical_used_in_mb: 0,
            logical_limit_in_mb: None,
            logical_free_in_mb: 0,
            logical_used_in_mb: 0,
        }
    }
}

impl Default for structs::PbmQueryReplicationGroupResult {
    fn default() -> Self {
        Self {
            object: structs::PbmServerObjectRef::default(),
            replication_group_id: None,
            fault: None,
        }
    }
}

impl Default for structs::SmsAboutInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            full_name: String::new(),
            vendor: String::new(),
            api_version: String::new(),
            instance_uuid: String::new(),
            vasa_api_version: None,
        }
    }
}

impl Default for structs::EntityReference {
    fn default() -> Self {
        Self {
            id: String::new(),
            r#type: None,
        }
    }
}

impl Default for structs::FaultDomainFilter {
    fn default() -> Self {
        Self {
            provider_id: None,
        }
    }
}

impl Default for structs::ReplicationGroupFilter {
    fn default() -> Self {
        Self {
            group_id: None,
        }
    }
}

impl Default for structs::SmsTaskInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
            task: structs::ManagedObjectReference::default(),
            object: None,
            error: None,
            result: None,
            start_time: None,
            completion_time: None,
            state: String::new(),
            progress: None,
        }
    }
}

impl Default for structs::AlarmFilter {
    fn default() -> Self {
        Self {
            alarm_status: None,
            alarm_type: None,
            entity_type: None,
            entity_id: None,
            page_marker: None,
        }
    }
}

impl Default for structs::AlarmResult {
    fn default() -> Self {
        Self {
            storage_alarm: None,
            page_marker: None,
        }
    }
}

impl Default for structs::SmsProviderInfo {
    fn default() -> Self {
        Self {
            uid: String::new(),
            name: String::new(),
            description: None,
            version: None,
        }
    }
}

impl Default for structs::VasaProviderInfo {
    fn default() -> Self {
        Self {
            sms_provider_info_: structs::SmsProviderInfo::default(),
            url: String::new(),
            certificate: None,
            status: None,
            status_fault: None,
            vasa_version: None,
            namespace: None,
            last_sync_time: None,
            supported_vendor_model_mapping: None,
            supported_profile: None,
            supported_provider_profile: None,
            related_storage_array: None,
            provider_id: None,
            certificate_expiry_date: None,
            certificate_status: None,
            service_location: None,
            needs_explicit_activation: None,
            max_batch_size: None,
            retain_vasa_provider_certificate: None,
            array_independent_provider: None,
            r#type: None,
            category: None,
            priority: None,
            failover_group_id: None,
        }
    }
}

impl Default for structs::SmsProviderSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: None,
        }
    }
}

impl Default for structs::VasaProviderSpec {
    fn default() -> Self {
        Self {
            sms_provider_spec_: structs::SmsProviderSpec::default(),
            username: String::new(),
            password: String::new(),
            url: String::new(),
            certificate: None,
        }
    }
}

impl Default for structs::VasaProviderUpgradeSpec {
    fn default() -> Self {
        Self {
            provider_uid: String::new(),
            username: String::new(),
            password: String::new(),
        }
    }
}

impl Default for structs::RelatedStorageArray {
    fn default() -> Self {
        Self {
            array_id: String::new(),
            active: false,
            manageable: false,
            priority: 0,
        }
    }
}

impl Default for structs::SupportedVendorModelMapping {
    fn default() -> Self {
        Self {
            vendor_id: None,
            model_id: None,
        }
    }
}

impl Default for structs::BackingConfig {
    fn default() -> Self {
        Self {
            thin_provision_backing_identifier: None,
            deduplication_backing_identifier: None,
            auto_tiering_enabled: None,
            deduplication_efficiency: None,
            performance_optimization_interval: None,
        }
    }
}

impl Default for structs::BackingStoragePool {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            r#type: String::new(),
            capacity_in_mb: 0,
            used_space_in_mb: 0,
        }
    }
}

impl Default for structs::DatastoreBackingPoolMapping {
    fn default() -> Self {
        Self {
            datastore: Vec::new(),
            backing_storage_pool: None,
        }
    }
}

impl Default for structs::DatastorePair {
    fn default() -> Self {
        Self {
            datastore_1: structs::ManagedObjectReference::default(),
            datastore_2: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::DrsMigrationCapabilityResult {
    fn default() -> Self {
        Self {
            recommended_datastore_pair: None,
            non_recommended_datastore_pair: None,
        }
    }
}

impl Default for structs::FaultDomainProviderMapping {
    fn default() -> Self {
        Self {
            active_provider: structs::ManagedObjectReference::default(),
            fault_domain_id: None,
        }
    }
}

impl Default for structs::StorageFileSystemInfo {
    fn default() -> Self {
        Self {
            file_server_name: String::new(),
            file_system_path: String::new(),
            ip_address: None,
        }
    }
}

impl Default for structs::LunHbaAssociation {
    fn default() -> Self {
        Self {
            canonical_name: String::new(),
            hba: Vec::new(),
        }
    }
}

impl Default for structs::NameValuePair {
    fn default() -> Self {
        Self {
            parameter_name: String::new(),
            parameter_value: String::new(),
        }
    }
}

impl Default for structs::StorageAlarm {
    fn default() -> Self {
        Self {
            alarm_id: 0,
            alarm_type: String::new(),
            container_id: None,
            object_id: None,
            object_type: String::new(),
            status: String::new(),
            alarm_time_stamp: String::new(),
            message_id: String::new(),
            parameter_list: None,
            alarm_object: None,
        }
    }
}

impl Default for structs::StorageArray {
    fn default() -> Self {
        Self {
            name: String::new(),
            uuid: String::new(),
            vendor_id: String::new(),
            model_id: String::new(),
            firmware: None,
            alternate_name: None,
            supported_block_interface: None,
            supported_file_system_interface: None,
            supported_profile: None,
            priority: None,
            discovery_svc: None,
        }
    }
}

impl Default for structs::StorageCapability {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            name: String::new(),
            description: String::new(),
        }
    }
}

impl Default for structs::StorageContainer {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            name: String::new(),
            max_vvol_size_in_mb: 0,
            provider_id: Vec::new(),
            array_id: Vec::new(),
            vvol_container_type: None,
            stretched: None,
        }
    }
}

impl Default for structs::StorageContainerResult {
    fn default() -> Self {
        Self {
            storage_container: None,
            provider_info: None,
        }
    }
}

impl Default for structs::StorageContainerSpec {
    fn default() -> Self {
        Self {
            container_id: None,
        }
    }
}

impl Default for structs::StorageFileSystem {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            info: Vec::new(),
            native_snapshot_supported: false,
            thin_provisioning_status: String::new(),
            r#type: String::new(),
            version: String::new(),
            backing_config: None,
        }
    }
}

impl Default for structs::StorageLun {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            v_sphere_lun_identifier: String::new(),
            vendor_display_name: String::new(),
            capacity_in_mb: 0,
            used_space_in_mb: 0,
            lun_thin_provisioned: false,
            alternate_identifier: None,
            drs_management_permitted: false,
            thin_provisioning_status: String::new(),
            backing_config: None,
        }
    }
}

impl Default for structs::StoragePort {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            r#type: String::new(),
            alternate_name: None,
        }
    }
}

impl Default for structs::FcStoragePort {
    fn default() -> Self {
        Self {
            storage_port_: structs::StoragePort::default(),
            port_wwn: String::new(),
            node_wwn: String::new(),
        }
    }
}

impl Default for structs::FcoeStoragePort {
    fn default() -> Self {
        Self {
            storage_port_: structs::StoragePort::default(),
            port_wwn: String::new(),
            node_wwn: String::new(),
        }
    }
}

impl Default for structs::IscsiStoragePort {
    fn default() -> Self {
        Self {
            storage_port_: structs::StoragePort::default(),
            identifier: String::new(),
        }
    }
}

impl Default for structs::StorageProcessor {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            alternate_identifer: None,
        }
    }
}

impl Default for structs::DeviceId {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VVolId {
    fn default() -> Self {
        Self {
            id: String::new(),
        }
    }
}

impl Default for structs::VasaVirtualDiskId {
    fn default() -> Self {
        Self {
            disk_id: String::new(),
        }
    }
}

impl Default for structs::VirtualDiskKey {
    fn default() -> Self {
        Self {
            vm_instance_uuid: String::new(),
            device_key: 0,
        }
    }
}

impl Default for structs::VirtualDiskMoId {
    fn default() -> Self {
        Self {
            vc_uuid: None,
            vm_moid: String::new(),
            disk_key: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineId {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualMachineFilePath {
    fn default() -> Self {
        Self {
            vc_uuid: None,
            ds_url: String::new(),
            vmx_path: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineMoId {
    fn default() -> Self {
        Self {
            vc_uuid: None,
            vm_moid: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineUuid {
    fn default() -> Self {
        Self {
            vm_instance_uuid: String::new(),
        }
    }
}

impl Default for structs::FailoverParam {
    fn default() -> Self {
        Self {
            is_planned: false,
            check_only: false,
            replication_groups_to_failover: None,
            policy_associations: None,
        }
    }
}

impl Default for structs::TestFailoverParam {
    fn default() -> Self {
        Self {
            failover_param_: structs::FailoverParam::default(),
        }
    }
}

impl Default for structs::PolicyAssociation {
    fn default() -> Self {
        Self {
            id: Default::default(),
            policy_id: String::new(),
            datastore: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::ReplicationGroupData {
    fn default() -> Self {
        Self {
            group_id: structs::ReplicationGroupId::default(),
            pit_id: None,
        }
    }
}

impl Default for structs::RecoveredDevice {
    fn default() -> Self {
        Self {
            target_device_id: None,
            recovered_device_id: None,
            source_device_id: Default::default(),
            info: None,
            datastore: structs::ManagedObjectReference::default(),
            recovered_disk_info: None,
            error: None,
            warnings: None,
        }
    }
}

impl Default for structs::RecoveredDiskInfo {
    fn default() -> Self {
        Self {
            device_key: 0,
            ds_url: String::new(),
            disk_path: String::new(),
        }
    }
}

impl Default for structs::GroupInfo {
    fn default() -> Self {
        Self {
            group_id: structs::ReplicationGroupId::default(),
        }
    }
}

impl Default for structs::SourceGroupInfo {
    fn default() -> Self {
        Self {
            group_info_: structs::GroupInfo::default(),
            name: None,
            description: None,
            state: String::new(),
            replica: None,
            member_info: None,
        }
    }
}

impl Default for structs::TargetGroupInfo {
    fn default() -> Self {
        Self {
            group_info_: structs::GroupInfo::default(),
            source_info: structs::TargetToSourceInfo::default(),
            state: String::new(),
            devices: None,
            is_promote_capable: None,
            name: None,
        }
    }
}

impl Default for structs::GroupOperationResult {
    fn default() -> Self {
        Self {
            group_id: structs::ReplicationGroupId::default(),
            warning: None,
        }
    }
}

impl Default for structs::FailoverSuccessResult {
    fn default() -> Self {
        Self {
            group_operation_result_: structs::GroupOperationResult::default(),
            new_state: String::new(),
            pit_id: None,
            pit_id_before_failover: None,
            recovered_device_info: None,
            time_stamp: None,
        }
    }
}

impl Default for structs::GroupErrorResult {
    fn default() -> Self {
        Self {
            group_operation_result_: structs::GroupOperationResult::default(),
            error: Vec::new(),
        }
    }
}

impl Default for structs::QueryPointInTimeReplicaSuccessResult {
    fn default() -> Self {
        Self {
            group_operation_result_: structs::GroupOperationResult::default(),
            replica_info: None,
        }
    }
}

impl Default for structs::QueryPointInTimeReplicaSummaryResult {
    fn default() -> Self {
        Self {
            group_operation_result_: structs::GroupOperationResult::default(),
            interval_results: None,
        }
    }
}

impl Default for structs::QueryReplicationGroupSuccessResult {
    fn default() -> Self {
        Self {
            group_operation_result_: structs::GroupOperationResult::default(),
            rg_info: Default::default(),
        }
    }
}

impl Default for structs::ReverseReplicationSuccessResult {
    fn default() -> Self {
        Self {
            group_operation_result_: structs::GroupOperationResult::default(),
            new_group_id: structs::DeviceGroupId::default(),
        }
    }
}

impl Default for structs::SyncReplicationGroupSuccessResult {
    fn default() -> Self {
        Self {
            group_operation_result_: structs::GroupOperationResult::default(),
            time_stamp: String::new(),
            pit_id: None,
            pit_name: None,
        }
    }
}

impl Default for structs::PointInTimeReplicaId {
    fn default() -> Self {
        Self {
            id: String::new(),
        }
    }
}

impl Default for structs::PromoteParam {
    fn default() -> Self {
        Self {
            is_planned: false,
            replication_groups_to_promote: None,
        }
    }
}

impl Default for structs::QueryPointInTimeReplicaParam {
    fn default() -> Self {
        Self {
            replica_time_query_param: None,
            pit_name: None,
            tags: None,
            prefer_details: None,
        }
    }
}

impl Default for structs::ReplicaQueryIntervalParam {
    fn default() -> Self {
        Self {
            from_date: None,
            to_date: None,
            number: None,
        }
    }
}

impl Default for structs::PointInTimeReplicaInfo {
    fn default() -> Self {
        Self {
            id: structs::PointInTimeReplicaId::default(),
            pit_name: String::new(),
            time_stamp: String::new(),
            tags: None,
        }
    }
}

impl Default for structs::ReplicaIntervalQueryResult {
    fn default() -> Self {
        Self {
            from_date: String::new(),
            to_date: String::new(),
            number: 0,
        }
    }
}

impl Default for structs::QueryReplicationPeerResult {
    fn default() -> Self {
        Self {
            source_domain: Default::default(),
            target_domain: None,
            error: None,
            warning: None,
        }
    }
}

impl Default for structs::ReplicaId {
    fn default() -> Self {
        Self {
            id: String::new(),
        }
    }
}

impl Default for structs::ReplicationTargetInfo {
    fn default() -> Self {
        Self {
            target_group_id: structs::ReplicationGroupId::default(),
            replication_agreement_description: None,
        }
    }
}

impl Default for structs::SourceGroupMemberInfo {
    fn default() -> Self {
        Self {
            device_id: Default::default(),
            target_id: None,
        }
    }
}

impl Default for structs::TargetDeviceId {
    fn default() -> Self {
        Self {
            domain_id: Default::default(),
            device_id: structs::ReplicaId::default(),
        }
    }
}

impl Default for structs::TargetToSourceInfo {
    fn default() -> Self {
        Self {
            source_group_id: structs::ReplicationGroupId::default(),
            replication_agreement_description: None,
        }
    }
}

impl Default for structs::TargetGroupMemberInfo {
    fn default() -> Self {
        Self {
            replica_id: structs::ReplicaId::default(),
            source_id: Default::default(),
            target_datastore: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::RecoveredTargetGroupMemberInfo {
    fn default() -> Self {
        Self {
            target_group_member_info_: structs::TargetGroupMemberInfo::default(),
            recovered_device_id: None,
        }
    }
}

impl Default for structs::AboutInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            full_name: String::new(),
            vendor: String::new(),
            version: String::new(),
            patch_level: None,
            build: String::new(),
            locale_version: None,
            locale_build: None,
            os_type: String::new(),
            product_line_id: String::new(),
            api_type: String::new(),
            api_version: String::new(),
            instance_uuid: None,
            license_product_name: None,
            license_product_version: None,
        }
    }
}

impl Default for structs::AuthorizationDescription {
    fn default() -> Self {
        Self {
            privilege: Vec::new(),
            privilege_group: Vec::new(),
        }
    }
}

impl Default for structs::EntityPrivilege {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
            priv_availability: Vec::new(),
        }
    }
}

impl Default for structs::Permission {
    fn default() -> Self {
        Self {
            entity: None,
            principal: String::new(),
            group: false,
            role_id: 0,
            propagate: false,
        }
    }
}

impl Default for structs::AuthorizationPrivilege {
    fn default() -> Self {
        Self {
            priv_id: String::new(),
            on_parent: false,
            name: String::new(),
            priv_group_name: String::new(),
        }
    }
}

impl Default for structs::PrivilegeAvailability {
    fn default() -> Self {
        Self {
            priv_id: String::new(),
            is_granted: false,
        }
    }
}

impl Default for structs::AuthorizationRole {
    fn default() -> Self {
        Self {
            role_id: 0,
            system: false,
            name: String::new(),
            info: Default::default(),
            privilege: None,
        }
    }
}

impl Default for structs::UserPrivilegeResult {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
            privileges: None,
        }
    }
}

impl Default for structs::BatchResult {
    fn default() -> Self {
        Self {
            result: String::new(),
            host_key: String::new(),
            ds: None,
            fault: None,
        }
    }
}

impl Default for structs::Capability {
    fn default() -> Self {
        Self {
            provisioning_supported: false,
            multi_host_supported: false,
            user_shell_access_supported: false,
            supported_evc_mode: None,
            supported_evc_graphics_mode: None,
            network_backup_and_restore_supported: None,
            ft_drs_without_evc_supported: None,
            hci_workflow_supported: None,
            compute_policy_version: None,
            cluster_placement_supported: None,
            lifecycle_management_supported: None,
            host_seeding_supported: None,
            scalable_shares_supported: None,
            hadcs_supported: None,
            config_mgmt_supported: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceClusterConfigResult {
    fn default() -> Self {
        Self {
            failed_hosts: None,
            configured_hosts: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceCryptoModePolicy {
    fn default() -> Self {
        Self {
            key_id: None,
            provider_id: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceDvsSetting {
    fn default() -> Self {
        Self {
            dv_switch: structs::ManagedObjectReference::default(),
            pnic_devices: None,
            dv_portgroup_setting: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping {
    fn default() -> Self {
        Self {
            dv_portgroup: structs::ManagedObjectReference::default(),
            service: String::new(),
        }
    }
}

impl Default for structs::ClusterComputeResourceDvsProfile {
    fn default() -> Self {
        Self {
            dvs_name: None,
            dv_switch: None,
            pnic_devices: None,
            dv_portgroup_mapping: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping {
    fn default() -> Self {
        Self {
            dv_portgroup_spec: None,
            dv_portgroup: None,
            service: String::new(),
        }
    }
}

impl Default for structs::ClusterComputeResourceHciConfigInfo {
    fn default() -> Self {
        Self {
            workflow_state: String::new(),
            dvs_setting: None,
            configured_hosts: None,
            host_config_profile: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceHciConfigSpec {
    fn default() -> Self {
        Self {
            dvs_prof: None,
            host_config_profile: None,
            v_san_config_spec: None,
            vc_prof: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceHostConfigurationInput {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            host_vmk_nics: None,
            allowed_in_non_maintenance_mode: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceHostConfigurationProfile {
    fn default() -> Self {
        Self {
            date_time_config: None,
            lockdown_mode: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceHostEvacuationInfo {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            action: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceHostVmkNicInfo {
    fn default() -> Self {
        Self {
            nic_spec: structs::HostVirtualNicSpec::default(),
            service: String::new(),
        }
    }
}

impl Default for structs::ClusterComputeResourceMaintenanceInfo {
    fn default() -> Self {
        Self {
            partial_mm_id: None,
            host_evac_info: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceVcProfile {
    fn default() -> Self {
        Self {
            cluster_spec: None,
            evc_mode_key: None,
            evc_graphics_mode_key: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceValidationResultBase {
    fn default() -> Self {
        Self {
            info: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceDvsConfigurationValidation {
    fn default() -> Self {
        Self {
            cluster_compute_resource_validation_result_base_: structs::ClusterComputeResourceValidationResultBase::default(),
            is_dvs_valid: false,
            is_dvpg_valid: false,
        }
    }
}

impl Default for structs::ClusterComputeResourceHostConfigurationValidation {
    fn default() -> Self {
        Self {
            cluster_compute_resource_validation_result_base_: structs::ClusterComputeResourceValidationResultBase::default(),
            host: structs::ManagedObjectReference::default(),
            is_dvs_setting_valid: None,
            is_vmknic_setting_valid: None,
            is_ntp_setting_valid: None,
            is_lockdown_mode_valid: None,
        }
    }
}

impl Default for structs::VsanClusterConfigPrecheckItem {
    fn default() -> Self {
        Self {
            cluster_compute_resource_validation_result_base_: structs::ClusterComputeResourceValidationResultBase::default(),
            status: String::new(),
            description: None,
        }
    }
}

impl Default for structs::ClusterComputeResourceVcsSlots {
    fn default() -> Self {
        Self {
            system_id: None,
            host: structs::ManagedObjectReference::default(),
            datastore: None,
            total_slots: 0,
        }
    }
}

impl Default for structs::ComputeResourceConfigInfo {
    fn default() -> Self {
        Self {
            vm_swap_placement: String::new(),
            spbm_enabled: None,
            default_hardware_version_key: None,
            maximum_hardware_version_key: None,
        }
    }
}

impl Default for structs::ClusterConfigInfoEx {
    fn default() -> Self {
        Self {
            compute_resource_config_info_: structs::ComputeResourceConfigInfo::default(),
            system_v_ms_config: None,
            das_config: structs::ClusterDasConfigInfo::default(),
            das_vm_config: None,
            drs_config: structs::ClusterDrsConfigInfo::default(),
            drs_vm_config: None,
            rule: None,
            orchestration: None,
            vm_orchestration: None,
            dpm_config_info: None,
            dpm_host_config: None,
            vsan_config_info: None,
            vsan_host_config: None,
            group: None,
            infra_update_ha_config: None,
            proactive_drs_config: None,
            crypto_config: None,
            vsan_core_config: None,
        }
    }
}

impl Default for structs::ComputeResourceConfigSpec {
    fn default() -> Self {
        Self {
            vm_swap_placement: None,
            spbm_enabled: None,
            default_hardware_version_key: None,
            desired_software_spec: None,
            maximum_hardware_version_key: None,
            enable_config_manager: None,
            host_seed_spec: None,
            software_spec_id: None,
            network_boot_mode: None,
        }
    }
}

impl Default for structs::ClusterConfigSpecEx {
    fn default() -> Self {
        Self {
            compute_resource_config_spec_: structs::ComputeResourceConfigSpec::default(),
            system_v_ms_config: None,
            das_config: None,
            das_vm_config_spec: None,
            drs_config: None,
            drs_vm_config_spec: None,
            rules_spec: None,
            orchestration: None,
            vm_orchestration_spec: None,
            dpm_config: None,
            dpm_host_config_spec: None,
            vsan_config: None,
            vsan_host_config_spec: None,
            group_spec: None,
            infra_update_ha_config: None,
            proactive_drs_config: None,
            in_hci_workflow: None,
            crypto_config: None,
            vsan_core_config_spec: None,
        }
    }
}

impl Default for structs::ComputeResourceHostSpbmLicenseInfo {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            license_state: enums::ComputeResourceHostSpbmLicenseInfoHostSpbmLicenseStateEnum::default(),
        }
    }
}

impl Default for structs::ComputeResourceHostSeedSpec {
    fn default() -> Self {
        Self {
            single_host_spec: structs::ComputeResourceHostSeedSpecSingleHostSpec::default(),
        }
    }
}

impl Default for structs::ComputeResourceHostSeedSpecSingleHostSpec {
    fn default() -> Self {
        Self {
            new_host_cnx_spec: None,
            existing_host: None,
        }
    }
}

impl Default for structs::ComputeResourceSummary {
    fn default() -> Self {
        Self {
            total_cpu: 0,
            total_memory: 0,
            num_cpu_cores: 0,
            num_cpu_threads: 0,
            effective_cpu: 0,
            effective_memory: 0,
            num_hosts: 0,
            num_effective_hosts: 0,
            overall_status: enums::ManagedEntityStatusEnum::default(),
        }
    }
}

impl Default for structs::ClusterComputeResourceSummary {
    fn default() -> Self {
        Self {
            compute_resource_summary_: structs::ComputeResourceSummary::default(),
            current_failover_level: 0,
            admission_control_info: None,
            num_vmotions: 0,
            target_balance: None,
            current_balance: None,
            drs_score: None,
            num_vms_per_drs_score_bucket: None,
            usage_summary: None,
            current_evc_mode_key: None,
            current_evc_graphics_mode_key: None,
            das_data: None,
            cluster_maintenance_mode_status: None,
            vcs_health_status: None,
            vcs_slots: None,
        }
    }
}

impl Default for structs::CustomFieldDef {
    fn default() -> Self {
        Self {
            key: 0,
            name: String::new(),
            r#type: String::new(),
            managed_object_type: None,
            field_def_privileges: None,
            field_instance_privileges: None,
        }
    }
}

impl Default for structs::CustomFieldValue {
    fn default() -> Self {
        Self {
            key: 0,
        }
    }
}

impl Default for structs::CustomFieldStringValue {
    fn default() -> Self {
        Self {
            custom_field_value_: structs::CustomFieldValue::default(),
            value: String::new(),
        }
    }
}

impl Default for structs::CustomizationSpecInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            r#type: String::new(),
            change_version: None,
            last_update_time: None,
        }
    }
}

impl Default for structs::CustomizationSpecItem {
    fn default() -> Self {
        Self {
            info: structs::CustomizationSpecInfo::default(),
            spec: structs::CustomizationSpec::default(),
        }
    }
}

impl Default for structs::DatacenterBasicConnectInfo {
    fn default() -> Self {
        Self {
            hostname: None,
            error: None,
            server_ip: None,
            num_vm: None,
            num_powered_on_vm: None,
            host_product_info: None,
            hardware_vendor: None,
            hardware_model: None,
        }
    }
}

impl Default for structs::DatacenterConfigInfo {
    fn default() -> Self {
        Self {
            default_hardware_version_key: None,
            maximum_hardware_version_key: None,
        }
    }
}

impl Default for structs::DatacenterConfigSpec {
    fn default() -> Self {
        Self {
            default_hardware_version_key: None,
            maximum_hardware_version_key: None,
        }
    }
}

impl Default for structs::DatastoreCapability {
    fn default() -> Self {
        Self {
            directory_hierarchy_supported: false,
            raw_disk_mappings_supported: false,
            per_file_thin_provisioning_supported: false,
            storage_iorm_supported: false,
            native_snapshot_supported: false,
            top_level_directory_create_supported: None,
            se_sparse_supported: None,
            vmfs_sparse_supported: None,
            vsan_sparse_supported: None,
            upit_supported: None,
            vmdk_expand_supported: None,
            clustered_vmdk_supported: None,
        }
    }
}

impl Default for structs::DatastoreHostMount {
    fn default() -> Self {
        Self {
            key: structs::ManagedObjectReference::default(),
            mount_info: structs::HostMountInfo::default(),
        }
    }
}

impl Default for structs::DatastoreInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            url: String::new(),
            free_space: 0,
            max_file_size: 0,
            max_virtual_disk_capacity: None,
            max_memory_file_size: 0,
            timestamp: None,
            container_id: None,
            alias_of: None,
            supported_v_disk_formats: None,
            logical_sector_size: None,
            physical_sector_size: None,
        }
    }
}

impl Default for structs::LocalDatastoreInfo {
    fn default() -> Self {
        Self {
            datastore_info_: structs::DatastoreInfo::default(),
            path: None,
        }
    }
}

impl Default for structs::NasDatastoreInfo {
    fn default() -> Self {
        Self {
            datastore_info_: structs::DatastoreInfo::default(),
            nas: None,
        }
    }
}

impl Default for structs::PMemDatastoreInfo {
    fn default() -> Self {
        Self {
            datastore_info_: structs::DatastoreInfo::default(),
            pmem: structs::HostPMemVolume::default(),
        }
    }
}

impl Default for structs::VmfsDatastoreInfo {
    fn default() -> Self {
        Self {
            datastore_info_: structs::DatastoreInfo::default(),
            max_physical_rdm_file_size: 0,
            max_virtual_rdm_file_size: 0,
            vmfs: None,
        }
    }
}

impl Default for structs::VsanDatastoreInfo {
    fn default() -> Self {
        Self {
            datastore_info_: structs::DatastoreInfo::default(),
            membership_uuid: None,
            access_gen_no: None,
        }
    }
}

impl Default for structs::VvolDatastoreInfo {
    fn default() -> Self {
        Self {
            datastore_info_: structs::DatastoreInfo::default(),
            vvol_ds: None,
        }
    }
}

impl Default for structs::DatastoreMountPathDatastorePair {
    fn default() -> Self {
        Self {
            old_mount_path: String::new(),
            datastore: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::DatastoreSummary {
    fn default() -> Self {
        Self {
            datastore: None,
            name: String::new(),
            url: String::new(),
            capacity: 0,
            free_space: 0,
            uncommitted: None,
            accessible: false,
            multiple_host_access: None,
            r#type: String::new(),
            maintenance_mode: None,
        }
    }
}

impl Default for structs::DatastoreVVolContainerFailoverPair {
    fn default() -> Self {
        Self {
            src_container: None,
            tgt_container: String::new(),
            vvol_mapping: None,
        }
    }
}

impl Default for structs::DatastoreNamespaceManagerDirectoryInfo {
    fn default() -> Self {
        Self {
            capacity: 0,
            used: 0,
        }
    }
}

impl Default for structs::Description {
    fn default() -> Self {
        Self {
            label: String::new(),
            summary: String::new(),
        }
    }
}

impl Default for structs::ElementDescription {
    fn default() -> Self {
        Self {
            description_: structs::Description::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::EvcMode {
    fn default() -> Self {
        Self {
            element_description_: structs::ElementDescription::default(),
            guaranteed_cpu_features: None,
            feature_capability: None,
            feature_mask: None,
            feature_requirement: None,
            vendor: String::new(),
            track: Vec::new(),
            vendor_tier: 0,
        }
    }
}

impl Default for structs::ExtendedElementDescription {
    fn default() -> Self {
        Self {
            element_description_: structs::ElementDescription::default(),
            message_catalog_key_prefix: String::new(),
            message_arg: None,
        }
    }
}

impl Default for structs::FeatureEvcMode {
    fn default() -> Self {
        Self {
            element_description_: structs::ElementDescription::default(),
            mask: None,
            capability: None,
            requirement: None,
        }
    }
}

impl Default for structs::OptionDef {
    fn default() -> Self {
        Self {
            element_description_: structs::ElementDescription::default(),
            option_type: Default::default(),
        }
    }
}

impl Default for structs::ExtendedDescription {
    fn default() -> Self {
        Self {
            description_: structs::Description::default(),
            message_catalog_key_prefix: String::new(),
            message_arg: None,
        }
    }
}

impl Default for structs::MethodDescription {
    fn default() -> Self {
        Self {
            description_: structs::Description::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::TypeDescription {
    fn default() -> Self {
        Self {
            description_: structs::Description::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::ScheduledTaskDetail {
    fn default() -> Self {
        Self {
            type_description_: structs::TypeDescription::default(),
            frequency: String::new(),
        }
    }
}

impl Default for structs::DesiredSoftwareSpec {
    fn default() -> Self {
        Self {
            base_image_spec: structs::DesiredSoftwareSpecBaseImageSpec::default(),
            vendor_add_on_spec: None,
            components: None,
            removed_components: None,
        }
    }
}

impl Default for structs::DesiredSoftwareSpecBaseImageSpec {
    fn default() -> Self {
        Self {
            version: String::new(),
        }
    }
}

impl Default for structs::DesiredSoftwareSpecComponentSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: None,
        }
    }
}

impl Default for structs::DesiredSoftwareSpecVendorAddOnSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
        }
    }
}

impl Default for structs::DiagnosticManagerAuditRecordResult {
    fn default() -> Self {
        Self {
            records: None,
            next_token: String::new(),
        }
    }
}

impl Default for structs::DiagnosticManagerBundleInfo {
    fn default() -> Self {
        Self {
            system: None,
            url: String::new(),
        }
    }
}

impl Default for structs::DiagnosticManagerLogDescriptor {
    fn default() -> Self {
        Self {
            key: String::new(),
            file_name: String::new(),
            creator: String::new(),
            format: String::new(),
            mime_type: String::new(),
            info: Default::default(),
        }
    }
}

impl Default for structs::DiagnosticManagerLogHeader {
    fn default() -> Self {
        Self {
            line_start: 0,
            line_end: 0,
            line_text: None,
        }
    }
}

impl Default for structs::DirectPathProfileManagerCapacityQuerySpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DirectPathProfileManagerCapacityQueryByDeviceConfig {
    fn default() -> Self {
        Self {
            device_config: Default::default(),
        }
    }
}

impl Default for structs::DirectPathProfileManagerCapacityQueryById {
    fn default() -> Self {
        Self {
            id: String::new(),
        }
    }
}

impl Default for structs::DirectPathProfileManagerCapacityQueryByName {
    fn default() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Default for structs::DirectPathProfileManagerCapacityResult {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DirectPathProfileManagerCapacityInfo {
    fn default() -> Self {
        Self {
            profile: structs::DirectPathProfileInfo::default(),
            consumed: 0,
            remaining: 0,
            max: 0,
            unused_reservation: 0,
        }
    }
}

impl Default for structs::DirectPathProfileManagerCapacityUnknown {
    fn default() -> Self {
        Self {
            query_spec: Default::default(),
            fault_list: None,
        }
    }
}

impl Default for structs::DirectPathProfileManagerCreateSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: None,
            device_config: Default::default(),
        }
    }
}

impl Default for structs::DirectPathProfileManagerDirectPathConfig {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DirectPathProfileManagerDvxDirectPathConfig {
    fn default() -> Self {
        Self {
            dvx_backing: structs::VirtualPciPassthroughDvxBackingInfo::default(),
        }
    }
}

impl Default for structs::DirectPathProfileManagerDynamicDirectPathConfig {
    fn default() -> Self {
        Self {
            dynamic_direct_path_backing: structs::VirtualPciPassthroughDynamicBackingInfo::default(),
        }
    }
}

impl Default for structs::DirectPathProfileManagerVirtualDeviceGroupDirectPathConfig {
    fn default() -> Self {
        Self {
            device_group_name: String::new(),
        }
    }
}

impl Default for structs::DirectPathProfileManagerVmiopDirectPathConfig {
    fn default() -> Self {
        Self {
            vgpu_profile: String::new(),
        }
    }
}

impl Default for structs::DirectPathProfileInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: None,
            vendor_name: String::new(),
            device_config: Default::default(),
        }
    }
}

impl Default for structs::DirectPathProfileManagerFilterSpec {
    fn default() -> Self {
        Self {
            ids: None,
            names: None,
            clusters: None,
        }
    }
}

impl Default for structs::DirectPathProfileManagerTargetEntity {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DirectPathProfileManagerTargetCluster {
    fn default() -> Self {
        Self {
            cluster: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::DirectPathProfileManagerTargetHost {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::DirectPathProfileManagerUpdateSpec {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
        }
    }
}

impl Default for structs::DvsBackupRestoreCapability {
    fn default() -> Self {
        Self {
            backup_restore_supported: false,
        }
    }
}

impl Default for structs::DvsCapability {
    fn default() -> Self {
        Self {
            dvs_operation_supported: None,
            dv_port_group_operation_supported: None,
            dv_port_operation_supported: None,
            compatible_host_component_product_info: None,
            features_supported: None,
        }
    }
}

impl Default for structs::DvsConfigInfo {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            name: String::new(),
            num_standalone_ports: 0,
            num_ports: 0,
            max_ports: 0,
            uplink_port_policy: Default::default(),
            uplink_portgroup: None,
            default_port_config: Default::default(),
            host: None,
            product_info: structs::DistributedVirtualSwitchProductSpec::default(),
            target_info: None,
            extension_key: None,
            vendor_specific_config: None,
            policy: None,
            description: None,
            config_version: String::new(),
            contact: structs::DvsContactInfo::default(),
            switch_ip_address: None,
            create_time: String::new(),
            network_resource_management_enabled: false,
            default_proxy_switch_max_num_ports: None,
            health_check_config: None,
            infrastructure_traffic_resource_config: None,
            net_resource_pool_traffic_resource_config: None,
            network_resource_control_version: None,
            vm_vnic_network_resource_pool: None,
            pnic_capacity_ratio_for_reservation: None,
        }
    }
}

impl Default for structs::VMwareDvsConfigInfo {
    fn default() -> Self {
        Self {
            dvs_config_info_: structs::DvsConfigInfo::default(),
            vspan_session: None,
            pvlan_config: None,
            max_mtu: 0,
            link_discovery_protocol_config: None,
            ipfix_config: None,
            lacp_group_config: None,
            lacp_api_version: None,
            multicast_filtering_mode: None,
            network_offload_spec_id: None,
            network_offload_config: None,
            real_time_config: None,
        }
    }
}

impl Default for structs::DvsConfigSpec {
    fn default() -> Self {
        Self {
            dynamic_property: None,
            config_version: None,
            name: None,
            num_standalone_ports: None,
            max_ports: None,
            uplink_port_policy: None,
            uplink_portgroup: None,
            default_port_config: None,
            host: None,
            extension_key: None,
            description: None,
            policy: None,
            vendor_specific_config: None,
            contact: None,
            switch_ip_address: None,
            default_proxy_switch_max_num_ports: None,
            infrastructure_traffic_resource_config: None,
            net_resource_pool_traffic_resource_config: None,
            network_resource_control_version: None,
        }
    }
}

impl Default for structs::VMwareDvsConfigSpec {
    fn default() -> Self {
        Self {
            dvs_config_spec_: structs::DvsConfigSpec::default(),
            pvlan_config_spec: None,
            vspan_config_spec: None,
            max_mtu: None,
            link_discovery_protocol_config: None,
            ipfix_config: None,
            lacp_api_version: None,
            multicast_filtering_mode: None,
            network_offload_spec_id: None,
            network_offload_config: None,
            real_time_config: None,
        }
    }
}

impl Default for structs::DvsContactInfo {
    fn default() -> Self {
        Self {
            name: None,
            contact: None,
        }
    }
}

impl Default for structs::DvsCreateSpec {
    fn default() -> Self {
        Self {
            config_spec: Default::default(),
            product_info: None,
            capability: None,
        }
    }
}

impl Default for structs::DvsFeatureCapability {
    fn default() -> Self {
        Self {
            network_resource_management_supported: false,
            vm_direct_path_gen_2_supported: None,
            nic_teaming_policy: None,
            network_resource_pool_high_share_value: None,
            network_resource_management_capability: None,
            health_check_capability: None,
            rollback_capability: None,
            backup_restore_capability: None,
            network_filter_supported: None,
            mac_learning_supported: None,
        }
    }
}

impl Default for structs::VMwareDvsFeatureCapability {
    fn default() -> Self {
        Self {
            dvs_feature_capability_: structs::DvsFeatureCapability::default(),
            vspan_supported: None,
            lldp_supported: None,
            ipfix_supported: None,
            ipfix_capability: None,
            multicast_snooping_supported: None,
            vspan_capability: None,
            lacp_capability: None,
            dpu_capability: None,
            nsx_supported: None,
            mtu_capability: None,
            real_time_config_supported: None,
        }
    }
}

impl Default for structs::DvsHealthCheckConfig {
    fn default() -> Self {
        Self {
            enable: None,
            interval: None,
        }
    }
}

impl Default for structs::VMwareDvsHealthCheckConfig {
    fn default() -> Self {
        Self {
            dvs_health_check_config_: structs::DvsHealthCheckConfig::default(),
        }
    }
}

impl Default for structs::VMwareDvsTeamingHealthCheckConfig {
    fn default() -> Self {
        Self {
            v_mware_dvs_health_check_config_: structs::VMwareDvsHealthCheckConfig::default(),
        }
    }
}

impl Default for structs::VMwareDvsVlanMtuHealthCheckConfig {
    fn default() -> Self {
        Self {
            v_mware_dvs_health_check_config_: structs::VMwareDvsHealthCheckConfig::default(),
        }
    }
}

impl Default for structs::DvsHealthCheckCapability {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VMwareDvsHealthCheckCapability {
    fn default() -> Self {
        Self {
            vlan_mtu_supported: false,
            teaming_supported: false,
        }
    }
}

impl Default for structs::DvsHostInfrastructureTrafficResource {
    fn default() -> Self {
        Self {
            key: String::new(),
            description: None,
            allocation_info: structs::DvsHostInfrastructureTrafficResourceAllocation::default(),
        }
    }
}

impl Default for structs::DvsHostInfrastructureTrafficResourceAllocation {
    fn default() -> Self {
        Self {
            limit: None,
            shares: None,
            reservation: None,
        }
    }
}

impl Default for structs::DvsNetworkResourceManagementCapability {
    fn default() -> Self {
        Self {
            network_resource_management_supported: false,
            network_resource_pool_high_share_value: 0,
            qos_supported: false,
            user_defined_network_resource_pools_supported: false,
            network_resource_control_version_3_supported: None,
            user_defined_infra_traffic_pool_supported: None,
        }
    }
}

impl Default for structs::DvsResourceRuntimeInfo {
    fn default() -> Self {
        Self {
            capacity: None,
            usage: None,
            available: None,
            allocated_resource: None,
            vm_vnic_network_resource_pool_runtime: None,
        }
    }
}

impl Default for structs::DvsRollbackCapability {
    fn default() -> Self {
        Self {
            rollback_supported: false,
        }
    }
}

impl Default for structs::DvsRuntimeInfo {
    fn default() -> Self {
        Self {
            host_member_runtime: None,
            resource_runtime_info: None,
        }
    }
}

impl Default for structs::DvsSummary {
    fn default() -> Self {
        Self {
            name: String::new(),
            uuid: String::new(),
            num_ports: 0,
            product_info: None,
            host_member: None,
            vm: None,
            host: None,
            portgroup_name: None,
            description: None,
            contact: None,
            num_hosts: None,
        }
    }
}

impl Default for structs::DvsPolicy {
    fn default() -> Self {
        Self {
            auto_pre_install_allowed: None,
            auto_upgrade_allowed: None,
            partial_upgrade_allowed: None,
        }
    }
}

impl Default for structs::DvsUplinkPortPolicy {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DvsNameArrayUplinkPortPolicy {
    fn default() -> Self {
        Self {
            uplink_port_name: Vec::new(),
        }
    }
}

impl Default for structs::EnumDescription {
    fn default() -> Self {
        Self {
            key: String::new(),
            tags: Vec::new(),
        }
    }
}

impl Default for structs::EnvironmentBrowserConfigOptionQuerySpec {
    fn default() -> Self {
        Self {
            key: None,
            host: None,
            guest_id: None,
        }
    }
}

impl Default for structs::Extension {
    fn default() -> Self {
        Self {
            description: Default::default(),
            key: String::new(),
            company: None,
            r#type: None,
            version: String::new(),
            subject_name: None,
            server: None,
            client: None,
            task_list: None,
            event_list: None,
            fault_list: None,
            privilege_list: None,
            resource_list: None,
            last_heartbeat_time: String::new(),
            health_info: None,
            ovf_consumer_info: None,
            extended_product_info: None,
            managed_entity_info: None,
            shown_in_solution_manager: None,
            solution_manager_info: None,
        }
    }
}

impl Default for structs::ExtensionClientInfo {
    fn default() -> Self {
        Self {
            version: String::new(),
            description: Default::default(),
            company: String::new(),
            r#type: String::new(),
            url: String::new(),
        }
    }
}

impl Default for structs::ExtensionEventTypeInfo {
    fn default() -> Self {
        Self {
            event_id: String::new(),
            event_type_schema: None,
        }
    }
}

impl Default for structs::ExtensionFaultTypeInfo {
    fn default() -> Self {
        Self {
            fault_id: String::new(),
        }
    }
}

impl Default for structs::ExtensionHealthInfo {
    fn default() -> Self {
        Self {
            url: String::new(),
        }
    }
}

impl Default for structs::ExtensionOvfConsumerInfo {
    fn default() -> Self {
        Self {
            callback_url: String::new(),
            section_type: Vec::new(),
        }
    }
}

impl Default for structs::ExtensionPrivilegeInfo {
    fn default() -> Self {
        Self {
            priv_id: String::new(),
            priv_group_name: String::new(),
        }
    }
}

impl Default for structs::ExtensionResourceInfo {
    fn default() -> Self {
        Self {
            locale: String::new(),
            module: String::new(),
            data: Vec::new(),
        }
    }
}

impl Default for structs::ExtensionServerInfo {
    fn default() -> Self {
        Self {
            url: String::new(),
            description: Default::default(),
            company: String::new(),
            r#type: String::new(),
            admin_email: Vec::new(),
            server_thumbprint: None,
            server_certificate: None,
        }
    }
}

impl Default for structs::ExtensionTaskTypeInfo {
    fn default() -> Self {
        Self {
            task_id: String::new(),
        }
    }
}

impl Default for structs::ExtensionManagerIpAllocationUsage {
    fn default() -> Self {
        Self {
            extension_key: String::new(),
            num_addresses: 0,
        }
    }
}

impl Default for structs::FaultsByHost {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            faults: None,
        }
    }
}

impl Default for structs::FaultsByVm {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            faults: None,
        }
    }
}

impl Default for structs::FileLockInfo {
    fn default() -> Self {
        Self {
            file_path: String::new(),
            host: String::new(),
            mac: String::new(),
            id: String::new(),
            world_name: String::new(),
            owner_id: None,
            lock_mode: String::new(),
            acquired: None,
            heartbeat: None,
            ref_count: None,
        }
    }
}

impl Default for structs::FileLockInfoResult {
    fn default() -> Self {
        Self {
            lock_info: None,
            fault: None,
        }
    }
}

impl Default for structs::FolderBatchAddHostsToClusterResult {
    fn default() -> Self {
        Self {
            hosts_added_to_cluster: None,
            hosts_failed_inventory_add: None,
            hosts_failed_move_to_cluster: None,
        }
    }
}

impl Default for structs::FolderBatchAddStandaloneHostsResult {
    fn default() -> Self {
        Self {
            added_hosts: None,
            hosts_failed_inventory_add: None,
        }
    }
}

impl Default for structs::FolderExternallyManagedFolderInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            r#type: String::new(),
        }
    }
}

impl Default for structs::FolderFailedHostResult {
    fn default() -> Self {
        Self {
            host_name: None,
            host: None,
            context: structs::LocalizableMessage::default(),
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::FolderNewHostSpec {
    fn default() -> Self {
        Self {
            host_cnx_spec: structs::HostConnectSpec::default(),
            esx_license: None,
        }
    }
}

impl Default for structs::HbrManagerReplicationVmInfo {
    fn default() -> Self {
        Self {
            state: String::new(),
            progress_info: None,
            image_id: None,
            last_error: None,
        }
    }
}

impl Default for structs::ReplicationVmProgressInfo {
    fn default() -> Self {
        Self {
            progress: 0,
            bytes_transferred: 0,
            bytes_to_transfer: 0,
            checksum_total_bytes: None,
            checksum_compared_bytes: None,
        }
    }
}

impl Default for structs::HbrManagerVmReplicationCapability {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            supported_quiesce_mode: String::new(),
            compression_supported: false,
            max_supported_source_disk_capacity: 0,
            min_rpo: None,
            fault: None,
        }
    }
}

impl Default for structs::HbrReplicationTargetSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HbrTargetSpecReplacement {
    fn default() -> Self {
        Self {
            spec: None,
        }
    }
}

impl Default for structs::HbrTargetSpec {
    fn default() -> Self {
        Self {
            target_ip: String::new(),
            certificate: String::new(),
        }
    }
}

impl Default for structs::HealthUpdate {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
            health_update_info_id: String::new(),
            id: String::new(),
            status: enums::ManagedEntityStatusEnum::default(),
            remediation: String::new(),
        }
    }
}

impl Default for structs::HealthUpdateInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            component_type: String::new(),
            description: String::new(),
        }
    }
}

impl Default for structs::PerfInterval {
    fn default() -> Self {
        Self {
            key: 0,
            sampling_period: 0,
            name: String::new(),
            length: 0,
            level: None,
            enabled: false,
        }
    }
}

impl Default for structs::HostServiceTicket {
    fn default() -> Self {
        Self {
            host: None,
            port: None,
            ssl_thumbprint: None,
            ssl_certificate: None,
            service: String::new(),
            service_version: String::new(),
            session_id: String::new(),
        }
    }
}

impl Default for structs::HostSystemComplianceCheckState {
    fn default() -> Self {
        Self {
            state: String::new(),
            check_time: String::new(),
        }
    }
}

impl Default for structs::HostSystemReconnectSpec {
    fn default() -> Self {
        Self {
            sync_state: None,
        }
    }
}

impl Default for structs::HostSystemRemediationState {
    fn default() -> Self {
        Self {
            state: String::new(),
            operation_time: String::new(),
        }
    }
}

impl Default for structs::HttpNfcLeaseCapabilities {
    fn default() -> Self {
        Self {
            pull_mode_supported: false,
            cors_supported: false,
        }
    }
}

impl Default for structs::HttpNfcLeaseDatastoreLeaseInfo {
    fn default() -> Self {
        Self {
            datastore_key: String::new(),
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::HttpNfcLeaseDeviceUrl {
    fn default() -> Self {
        Self {
            key: String::new(),
            import_key: String::new(),
            url: String::new(),
            ssl_thumbprint: String::new(),
            ssl_certificate: None,
            disk: None,
            target_id: None,
            datastore_key: None,
            file_size: None,
        }
    }
}

impl Default for structs::HttpNfcLeaseHostInfo {
    fn default() -> Self {
        Self {
            url: String::new(),
            ssl_thumbprint: String::new(),
        }
    }
}

impl Default for structs::HttpNfcLeaseInfo {
    fn default() -> Self {
        Self {
            lease: structs::ManagedObjectReference::default(),
            entity: structs::ManagedObjectReference::default(),
            device_url: None,
            total_disk_capacity_in_kb: 0,
            lease_timeout: 0,
            host_map: None,
        }
    }
}

impl Default for structs::HttpNfcLeaseManifestEntry {
    fn default() -> Self {
        Self {
            key: String::new(),
            sha_1: String::new(),
            checksum: None,
            checksum_type: None,
            size: 0,
            disk: false,
            capacity: None,
            populated_size: None,
        }
    }
}

impl Default for structs::HttpNfcLeaseProbeResult {
    fn default() -> Self {
        Self {
            server_accessible: false,
        }
    }
}

impl Default for structs::HttpNfcLeaseSourceFile {
    fn default() -> Self {
        Self {
            target_device_id: String::new(),
            url: String::new(),
            member_name: None,
            create: false,
            ssl_thumbprint: None,
            ssl_certificate: None,
            http_headers: None,
            size: None,
        }
    }
}

impl Default for structs::ImportSpec {
    fn default() -> Self {
        Self {
            entity_config: None,
            instantiation_ost: None,
        }
    }
}

impl Default for structs::VirtualAppImportSpec {
    fn default() -> Self {
        Self {
            import_spec_: structs::ImportSpec::default(),
            name: String::new(),
            v_app_config_spec: structs::VAppConfigSpec::default(),
            resource_pool_spec: structs::ResourceConfigSpec::default(),
            child: None,
        }
    }
}

impl Default for structs::VirtualMachineImportSpec {
    fn default() -> Self {
        Self {
            import_spec_: structs::ImportSpec::default(),
            config_spec: structs::VirtualMachineConfigSpec::default(),
            res_pool_entity: None,
        }
    }
}

impl Default for structs::InheritablePolicy {
    fn default() -> Self {
        Self {
            inherited: false,
        }
    }
}

impl Default for structs::BoolPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            value: None,
        }
    }
}

impl Default for structs::IntPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            value: None,
        }
    }
}

impl Default for structs::LongPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            value: None,
        }
    }
}

impl Default for structs::StringPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            value: None,
        }
    }
}

impl Default for structs::DvsFilterConfig {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            key: None,
            agent_name: None,
            slot_number: None,
            parameters: None,
            on_failure: None,
        }
    }
}

impl Default for structs::DvsFilterConfigSpec {
    fn default() -> Self {
        Self {
            dvs_filter_config_: structs::DvsFilterConfig::default(),
            operation: String::new(),
        }
    }
}

impl Default for structs::DvsTrafficFilterConfig {
    fn default() -> Self {
        Self {
            dvs_filter_config_: structs::DvsFilterConfig::default(),
            traffic_ruleset: None,
        }
    }
}

impl Default for structs::DvsTrafficFilterConfigSpec {
    fn default() -> Self {
        Self {
            dvs_traffic_filter_config_: structs::DvsTrafficFilterConfig::default(),
            operation: String::new(),
        }
    }
}

impl Default for structs::DvsFilterPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            filter_config: None,
        }
    }
}

impl Default for structs::DvsTrafficShapingPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            enabled: None,
            average_bandwidth: None,
            peak_bandwidth: None,
            burst_size: None,
        }
    }
}

impl Default for structs::DvsVendorSpecificConfig {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            key_value: None,
        }
    }
}

impl Default for structs::DvsFailureCriteria {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            check_speed: None,
            speed: None,
            check_duplex: None,
            full_duplex: None,
            check_error_percent: None,
            percentage: None,
            check_beacon: None,
        }
    }
}

impl Default for structs::DvsMacLearningPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            enabled: false,
            allow_unicast_flooding: None,
            limit: None,
            limit_policy: None,
        }
    }
}

impl Default for structs::DvsMacManagementPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            allow_promiscuous: None,
            mac_changes: None,
            forged_transmits: None,
            mac_learning_policy: None,
        }
    }
}

impl Default for structs::DvsSecurityPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            allow_promiscuous: None,
            mac_changes: None,
            forged_transmits: None,
        }
    }
}

impl Default for structs::VMwareUplinkLacpPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            enable: None,
            mode: None,
        }
    }
}

impl Default for structs::VMwareUplinkPortOrderPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            active_uplink_port: None,
            standby_uplink_port: None,
        }
    }
}

impl Default for structs::VmwareUplinkPortTeamingPolicy {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
            policy: None,
            reverse_policy: None,
            notify_switches: None,
            rolling_order: None,
            failure_criteria: None,
            uplink_port_order: None,
        }
    }
}

impl Default for structs::VmwareDistributedVirtualSwitchVlanSpec {
    fn default() -> Self {
        Self {
            inheritable_policy_: structs::InheritablePolicy::default(),
        }
    }
}

impl Default for structs::VmwareDistributedVirtualSwitchPvlanSpec {
    fn default() -> Self {
        Self {
            vmware_distributed_virtual_switch_vlan_spec_: structs::VmwareDistributedVirtualSwitchVlanSpec::default(),
            pvlan_id: 0,
        }
    }
}

impl Default for structs::VmwareDistributedVirtualSwitchTrunkVlanSpec {
    fn default() -> Self {
        Self {
            vmware_distributed_virtual_switch_vlan_spec_: structs::VmwareDistributedVirtualSwitchVlanSpec::default(),
            vlan_id: None,
        }
    }
}

impl Default for structs::VmwareDistributedVirtualSwitchVlanIdSpec {
    fn default() -> Self {
        Self {
            vmware_distributed_virtual_switch_vlan_spec_: structs::VmwareDistributedVirtualSwitchVlanSpec::default(),
            vlan_id: 0,
        }
    }
}

impl Default for structs::IoFilterInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            vendor: String::new(),
            version: String::new(),
            r#type: None,
            summary: None,
            release_date: None,
        }
    }
}

impl Default for structs::ClusterIoFilterInfo {
    fn default() -> Self {
        Self {
            io_filter_info_: structs::IoFilterInfo::default(),
            op_type: String::new(),
            vib_url: None,
        }
    }
}

impl Default for structs::HostIoFilterInfo {
    fn default() -> Self {
        Self {
            io_filter_info_: structs::IoFilterInfo::default(),
            available: false,
        }
    }
}

impl Default for structs::IoFilterQueryIssueResult {
    fn default() -> Self {
        Self {
            op_type: String::new(),
            host_issue: None,
        }
    }
}

impl Default for structs::IoFilterHostIssue {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            issue: Vec::new(),
        }
    }
}

impl Default for structs::IoFilterManagerSslTrust {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::PinnedCertificate {
    fn default() -> Self {
        Self {
            ssl_certificate: String::new(),
        }
    }
}

impl Default for structs::UntrustedCertificate {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::IpPoolManagerIpAllocation {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            allocation_id: String::new(),
        }
    }
}

impl Default for structs::KeyValue {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: String::new(),
        }
    }
}

impl Default for structs::LatencySensitivity {
    fn default() -> Self {
        Self {
            level: enums::LatencySensitivitySensitivityLevelEnum::default(),
            sensitivity: None,
        }
    }
}

impl Default for structs::LicenseAssignmentManagerLicenseAssignment {
    fn default() -> Self {
        Self {
            entity_id: String::new(),
            scope: None,
            entity_display_name: None,
            assigned_license: structs::LicenseManagerLicenseInfo::default(),
            properties: None,
        }
    }
}

impl Default for structs::LicenseAvailabilityInfo {
    fn default() -> Self {
        Self {
            feature: structs::LicenseFeatureInfo::default(),
            total: 0,
            available: 0,
        }
    }
}

impl Default for structs::LicenseDiagnostics {
    fn default() -> Self {
        Self {
            source_last_changed: String::new(),
            source_lost: String::new(),
            source_latency: 0.0,
            license_requests: String::new(),
            license_request_failures: String::new(),
            license_feature_unknowns: String::new(),
            op_state: enums::LicenseManagerStateEnum::default(),
            last_status_update: String::new(),
            op_failure_message: String::new(),
        }
    }
}

impl Default for structs::LicenseManagerEvaluationInfo {
    fn default() -> Self {
        Self {
            properties: Vec::new(),
        }
    }
}

impl Default for structs::LicenseFeatureInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
            feature_name: String::new(),
            feature_description: None,
            state: None,
            cost_unit: String::new(),
            source_restriction: None,
            dependent_key: None,
            edition: None,
            expires_on: None,
        }
    }
}

impl Default for structs::HostLicensableResourceInfo {
    fn default() -> Self {
        Self {
            resource: Vec::new(),
        }
    }
}

impl Default for structs::LicenseManagerLicenseInfo {
    fn default() -> Self {
        Self {
            license_key: String::new(),
            edition_key: String::new(),
            name: String::new(),
            total: 0,
            used: None,
            cost_unit: String::new(),
            properties: None,
            labels: None,
        }
    }
}

impl Default for structs::LicenseSource {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::EvaluationLicenseSource {
    fn default() -> Self {
        Self {
            remaining_hours: None,
        }
    }
}

impl Default for structs::LicenseServerSource {
    fn default() -> Self {
        Self {
            license_server: String::new(),
        }
    }
}

impl Default for structs::LocalLicenseSource {
    fn default() -> Self {
        Self {
            license_keys: String::new(),
        }
    }
}

impl Default for structs::LicenseUsageInfo {
    fn default() -> Self {
        Self {
            source: Default::default(),
            source_available: false,
            reservation_info: None,
            feature_info: None,
        }
    }
}

impl Default for structs::LicenseReservationInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
            state: enums::LicenseReservationInfoStateEnum::default(),
            required: 0,
        }
    }
}

impl Default for structs::LocalizationManagerMessageCatalog {
    fn default() -> Self {
        Self {
            module_name: String::new(),
            catalog_name: String::new(),
            locale: String::new(),
            catalog_uri: String::new(),
            last_modified: None,
            md_5_sum: None,
            version: None,
        }
    }
}

impl Default for structs::NegatableExpression {
    fn default() -> Self {
        Self {
            negate: None,
        }
    }
}

impl Default for structs::IntExpression {
    fn default() -> Self {
        Self {
            negatable_expression_: structs::NegatableExpression::default(),
            value: None,
        }
    }
}

impl Default for structs::IpAddress {
    fn default() -> Self {
        Self {
            negatable_expression_: structs::NegatableExpression::default(),
        }
    }
}

impl Default for structs::IpRange {
    fn default() -> Self {
        Self {
            ip_address_: structs::IpAddress::default(),
            address_prefix: String::new(),
            prefix_length: None,
        }
    }
}

impl Default for structs::SingleIp {
    fn default() -> Self {
        Self {
            ip_address_: structs::IpAddress::default(),
            address: String::new(),
        }
    }
}

impl Default for structs::MacAddress {
    fn default() -> Self {
        Self {
            negatable_expression_: structs::NegatableExpression::default(),
        }
    }
}

impl Default for structs::MacRange {
    fn default() -> Self {
        Self {
            mac_address_: structs::MacAddress::default(),
            address: String::new(),
            mask: String::new(),
        }
    }
}

impl Default for structs::SingleMac {
    fn default() -> Self {
        Self {
            mac_address_: structs::MacAddress::default(),
            address: String::new(),
        }
    }
}

impl Default for structs::StringExpression {
    fn default() -> Self {
        Self {
            negatable_expression_: structs::NegatableExpression::default(),
            value: None,
        }
    }
}

impl Default for structs::DvsIpPort {
    fn default() -> Self {
        Self {
            negatable_expression_: structs::NegatableExpression::default(),
        }
    }
}

impl Default for structs::DvsIpPortRange {
    fn default() -> Self {
        Self {
            dvs_ip_port_: structs::DvsIpPort::default(),
            start_port_number: 0,
            end_port_number: 0,
        }
    }
}

impl Default for structs::DvsSingleIpPort {
    fn default() -> Self {
        Self {
            dvs_ip_port_: structs::DvsIpPort::default(),
            port_number: 0,
        }
    }
}

impl Default for structs::NetworkSummary {
    fn default() -> Self {
        Self {
            network: None,
            name: String::new(),
            accessible: false,
            ip_pool_name: String::new(),
            ip_pool_id: None,
        }
    }
}

impl Default for structs::OpaqueNetworkSummary {
    fn default() -> Self {
        Self {
            network_summary_: structs::NetworkSummary::default(),
            opaque_network_id: String::new(),
            opaque_network_type: String::new(),
        }
    }
}

impl Default for structs::NumericRange {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
        }
    }
}

impl Default for structs::OpaqueNetworkCapability {
    fn default() -> Self {
        Self {
            network_reservation_supported: false,
        }
    }
}

impl Default for structs::OvfConsumerOstNode {
    fn default() -> Self {
        Self {
            id: String::new(),
            r#type: String::new(),
            section: None,
            child: None,
            entity: None,
        }
    }
}

impl Default for structs::OvfConsumerOvfSection {
    fn default() -> Self {
        Self {
            line_number: 0,
            xml: String::new(),
        }
    }
}

impl Default for structs::OvfManagerCommonParams {
    fn default() -> Self {
        Self {
            locale: String::new(),
            deployment_option: String::new(),
            msg_bundle: None,
            import_option: None,
        }
    }
}

impl Default for structs::OvfCreateImportSpecParams {
    fn default() -> Self {
        Self {
            ovf_manager_common_params_: structs::OvfManagerCommonParams::default(),
            entity_name: String::new(),
            host_system: None,
            network_mapping: None,
            ip_allocation_policy: None,
            ip_protocol: None,
            property_mapping: None,
            resource_mapping: None,
            disk_provisioning: None,
            instantiation_ost: None,
        }
    }
}

impl Default for structs::OvfImportParams {
    fn default() -> Self {
        Self {
            ovf_create_import_spec_params_: structs::OvfCreateImportSpecParams::default(),
            push_mode: None,
            signature_required: None,
            skip_manifest_check: None,
            power_on: None,
            custom_http_headers: None,
            source_certificate: None,
            datastore_mappings: None,
            vm_profile: None,
            disk_profiles: None,
        }
    }
}

impl Default for structs::OvfParseDescriptorParams {
    fn default() -> Self {
        Self {
            ovf_manager_common_params_: structs::OvfManagerCommonParams::default(),
        }
    }
}

impl Default for structs::OvfValidateHostParams {
    fn default() -> Self {
        Self {
            ovf_manager_common_params_: structs::OvfManagerCommonParams::default(),
        }
    }
}

impl Default for structs::OvfCreateDescriptorParams {
    fn default() -> Self {
        Self {
            ovf_files: None,
            name: None,
            description: None,
            include_image_files: None,
            export_option: None,
            snapshot: None,
        }
    }
}

impl Default for structs::OvfCreateDescriptorResult {
    fn default() -> Self {
        Self {
            ovf_descriptor: String::new(),
            error: None,
            warning: None,
            include_image_files: None,
        }
    }
}

impl Default for structs::OvfCreateImportSpecResult {
    fn default() -> Self {
        Self {
            import_spec: None,
            file_item: None,
            warning: None,
            error: None,
        }
    }
}

impl Default for structs::OvfDatastoreMapping {
    fn default() -> Self {
        Self {
            disk_id: String::new(),
            datastore: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::OvfDeploymentOption {
    fn default() -> Self {
        Self {
            key: String::new(),
            label: String::new(),
            description: String::new(),
        }
    }
}

impl Default for structs::OvfFileItem {
    fn default() -> Self {
        Self {
            device_id: String::new(),
            path: String::new(),
            compression_method: None,
            chunk_size: None,
            size: None,
            cim_type: 0,
            create: false,
        }
    }
}

impl Default for structs::OvfNetworkInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
        }
    }
}

impl Default for structs::OvfNetworkMapping {
    fn default() -> Self {
        Self {
            name: String::new(),
            network: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::OvfFile {
    fn default() -> Self {
        Self {
            device_id: String::new(),
            path: String::new(),
            compression_method: None,
            chunk_size: None,
            size: 0,
            capacity: None,
            populated_size: None,
        }
    }
}

impl Default for structs::OvfOptionInfo {
    fn default() -> Self {
        Self {
            option: String::new(),
            description: structs::LocalizableMessage::default(),
        }
    }
}

impl Default for structs::OvfParseDescriptorResult {
    fn default() -> Self {
        Self {
            eula: None,
            network: None,
            ip_allocation_scheme: None,
            ip_protocols: None,
            property: None,
            product_info: None,
            annotation: String::new(),
            approximate_download_size: None,
            approximate_flat_deployment_size: None,
            approximate_sparse_deployment_size: None,
            default_entity_name: String::new(),
            virtual_app: false,
            deployment_option: None,
            default_deployment_option: String::new(),
            entity_name: None,
            annotated_ost: None,
            error: None,
            warning: None,
        }
    }
}

impl Default for structs::OvfResourceMap {
    fn default() -> Self {
        Self {
            source: String::new(),
            parent: None,
            resource_spec: None,
            datastore: None,
        }
    }
}

impl Default for structs::OvfStorageProfileMapping {
    fn default() -> Self {
        Self {
            disk_id: String::new(),
            storage_profile_id: String::new(),
        }
    }
}

impl Default for structs::OvfValidateHostResult {
    fn default() -> Self {
        Self {
            download_size: None,
            flat_deployment_size: None,
            sparse_deployment_size: None,
            error: None,
            warning: None,
            supported_disk_provisioning: None,
        }
    }
}

impl Default for structs::PasswordField {
    fn default() -> Self {
        Self {
            value: String::new(),
        }
    }
}

impl Default for structs::PerformanceDescription {
    fn default() -> Self {
        Self {
            counter_type: Vec::new(),
            stats_type: Vec::new(),
        }
    }
}

impl Default for structs::PerfCompositeMetric {
    fn default() -> Self {
        Self {
            entity: None,
            child_entity: None,
        }
    }
}

impl Default for structs::PerfCounterInfo {
    fn default() -> Self {
        Self {
            key: 0,
            name_info: Default::default(),
            group_info: Default::default(),
            unit_info: Default::default(),
            rollup_type: enums::PerfSummaryTypeEnum::default(),
            stats_type: enums::PerfStatsTypeEnum::default(),
            level: None,
            per_device_level: None,
            associated_counter_id: None,
        }
    }
}

impl Default for structs::PerformanceManagerCounterLevelMapping {
    fn default() -> Self {
        Self {
            counter_id: 0,
            aggregate_level: None,
            per_device_level: None,
        }
    }
}

impl Default for structs::PerfEntityMetricBase {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::PerfEntityMetric {
    fn default() -> Self {
        Self {
            perf_entity_metric_base_: structs::PerfEntityMetricBase::default(),
            sample_info: None,
            value: None,
        }
    }
}

impl Default for structs::PerfEntityMetricCsv {
    fn default() -> Self {
        Self {
            perf_entity_metric_base_: structs::PerfEntityMetricBase::default(),
            sample_info_csv: String::new(),
            value: None,
        }
    }
}

impl Default for structs::PerfMetricId {
    fn default() -> Self {
        Self {
            counter_id: 0,
            instance: String::new(),
        }
    }
}

impl Default for structs::PerfMetricSeries {
    fn default() -> Self {
        Self {
            id: structs::PerfMetricId::default(),
        }
    }
}

impl Default for structs::PerfMetricIntSeries {
    fn default() -> Self {
        Self {
            perf_metric_series_: structs::PerfMetricSeries::default(),
            value: None,
        }
    }
}

impl Default for structs::PerfMetricSeriesCsv {
    fn default() -> Self {
        Self {
            perf_metric_series_: structs::PerfMetricSeries::default(),
            value: None,
        }
    }
}

impl Default for structs::PerfProviderSummary {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
            current_supported: false,
            summary_supported: false,
            refresh_rate: None,
        }
    }
}

impl Default for structs::PerfQuerySpec {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
            start_time: None,
            end_time: None,
            max_sample: None,
            metric_id: None,
            interval_id: None,
            format: None,
        }
    }
}

impl Default for structs::PerfSampleInfo {
    fn default() -> Self {
        Self {
            timestamp: String::new(),
            interval: 0,
        }
    }
}

impl Default for structs::PrivilegePolicyDef {
    fn default() -> Self {
        Self {
            create_privilege: String::new(),
            read_privilege: String::new(),
            update_privilege: String::new(),
            delete_privilege: String::new(),
        }
    }
}

impl Default for structs::ResourceAllocationInfo {
    fn default() -> Self {
        Self {
            reservation: None,
            expandable_reservation: None,
            limit: None,
            shares: None,
            overhead_limit: None,
        }
    }
}

impl Default for structs::ResourceAllocationOption {
    fn default() -> Self {
        Self {
            shares_option: structs::SharesOption::default(),
        }
    }
}

impl Default for structs::ResourceConfigOption {
    fn default() -> Self {
        Self {
            cpu_allocation_option: structs::ResourceAllocationOption::default(),
            memory_allocation_option: structs::ResourceAllocationOption::default(),
        }
    }
}

impl Default for structs::ResourceConfigSpec {
    fn default() -> Self {
        Self {
            entity: None,
            change_version: None,
            last_modified: None,
            cpu_allocation: structs::ResourceAllocationInfo::default(),
            memory_allocation: structs::ResourceAllocationInfo::default(),
            scale_descendants_shares: None,
        }
    }
}

impl Default for structs::DatabaseSizeEstimate {
    fn default() -> Self {
        Self {
            size: 0,
        }
    }
}

impl Default for structs::DatabaseSizeParam {
    fn default() -> Self {
        Self {
            inventory_desc: structs::InventoryDescription::default(),
            perf_stats_desc: None,
        }
    }
}

impl Default for structs::InventoryDescription {
    fn default() -> Self {
        Self {
            num_hosts: 0,
            num_virtual_machines: 0,
            num_resource_pools: None,
            num_clusters: None,
            num_cpu_dev: None,
            num_net_dev: None,
            num_disk_dev: None,
            numv_cpu_dev: None,
            numv_net_dev: None,
            numv_disk_dev: None,
        }
    }
}

impl Default for structs::PerformanceStatisticsDescription {
    fn default() -> Self {
        Self {
            intervals: None,
        }
    }
}

impl Default for structs::ResourcePoolResourceUsage {
    fn default() -> Self {
        Self {
            reservation_used: 0,
            reservation_used_for_vm: 0,
            unreserved_for_pool: 0,
            unreserved_for_vm: 0,
            overall_usage: 0,
            max_usage: 0,
        }
    }
}

impl Default for structs::ResourcePoolRuntimeInfo {
    fn default() -> Self {
        Self {
            memory: structs::ResourcePoolResourceUsage::default(),
            cpu: structs::ResourcePoolResourceUsage::default(),
            overall_status: enums::ManagedEntityStatusEnum::default(),
            shares_scalable: None,
        }
    }
}

impl Default for structs::ResourcePoolSummary {
    fn default() -> Self {
        Self {
            name: String::new(),
            config: structs::ResourceConfigSpec::default(),
            runtime: structs::ResourcePoolRuntimeInfo::default(),
            quick_stats: None,
            configured_memory_mb: None,
        }
    }
}

impl Default for structs::VirtualAppSummary {
    fn default() -> Self {
        Self {
            resource_pool_summary_: structs::ResourcePoolSummary::default(),
            product: None,
            v_app_state: None,
            suspended: None,
            install_boot_required: None,
            instance_uuid: None,
        }
    }
}

impl Default for structs::ResourcePoolQuickStats {
    fn default() -> Self {
        Self {
            overall_cpu_usage: None,
            overall_cpu_demand: None,
            guest_memory_usage: None,
            host_memory_usage: None,
            distributed_cpu_entitlement: None,
            distributed_memory_entitlement: None,
            static_cpu_entitlement: None,
            static_memory_entitlement: None,
            private_memory: None,
            shared_memory: None,
            swapped_memory: None,
            ballooned_memory: None,
            overhead_memory: None,
            consumed_overhead_memory: None,
            compressed_memory: None,
        }
    }
}

impl Default for structs::SddcBase {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VimVsanReconfigSpec {
    fn default() -> Self {
        Self {
            vsan_cluster_config: None,
            data_efficiency_config: None,
            disk_mapping_spec: None,
            fault_domains_spec: None,
            modify: false,
            allow_reduced_redundancy: None,
            resync_iops_limit_config: None,
            iscsi_spec: None,
            data_encryption_config: None,
            extended_config: None,
            datastore_config: None,
            perfsvc_config: None,
            unmap_config: None,
            vum_config: None,
            metrics_config: None,
            file_service_config: None,
            rdma_config: None,
            data_in_transit_encryption_config: None,
            mode: None,
            vsan_health_config: None,
            vsan_esa_config: None,
            xvc_datastore_config: None,
            server_cluster_config: None,
            snap_service_config: None,
            deconverged_net_config: None,
        }
    }
}

impl Default for structs::SelectionSet {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DvPortgroupSelection {
    fn default() -> Self {
        Self {
            dvs_uuid: String::new(),
            portgroup_key: Vec::new(),
        }
    }
}

impl Default for structs::DvsSelection {
    fn default() -> Self {
        Self {
            dvs_uuid: String::new(),
        }
    }
}

impl Default for structs::HostVMotionCompatibility {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            compatibility: None,
        }
    }
}

impl Default for structs::ProductComponentInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            version: String::new(),
            release: 0,
        }
    }
}

impl Default for structs::ServiceContent {
    fn default() -> Self {
        Self {
            root_folder: structs::ManagedObjectReference::default(),
            property_collector: structs::ManagedObjectReference::default(),
            view_manager: None,
            about: structs::AboutInfo::default(),
            setting: None,
            user_directory: None,
            session_manager: None,
            authorization_manager: None,
            service_manager: None,
            perf_manager: None,
            scheduled_task_manager: None,
            alarm_manager: None,
            event_manager: None,
            task_manager: None,
            extension_manager: None,
            customization_spec_manager: None,
            guest_customization_manager: None,
            custom_fields_manager: None,
            account_manager: None,
            diagnostic_manager: None,
            license_manager: None,
            search_index: None,
            file_manager: None,
            datastore_namespace_manager: None,
            virtual_disk_manager: None,
            virtualization_manager: None,
            snmp_system: None,
            vm_provisioning_checker: None,
            vm_compatibility_checker: None,
            ovf_manager: None,
            ip_pool_manager: None,
            dv_switch_manager: None,
            host_profile_manager: None,
            cluster_profile_manager: None,
            compliance_manager: None,
            localization_manager: None,
            storage_resource_manager: None,
            guest_operations_manager: None,
            overhead_memory_manager: None,
            certificate_manager: None,
            io_filter_manager: None,
            v_storage_object_manager: None,
            host_spec_manager: None,
            crypto_manager: None,
            health_update_manager: None,
            failover_cluster_configurator: None,
            failover_cluster_manager: None,
            tenant_manager: None,
            site_info_manager: None,
            storage_query_manager: None,
            direct_path_profile_manager: None,
        }
    }
}

impl Default for structs::ServiceLocator {
    fn default() -> Self {
        Self {
            instance_uuid: String::new(),
            url: String::new(),
            credential: Default::default(),
            ssl_thumbprint: None,
            ssl_certificate: None,
        }
    }
}

impl Default for structs::ServiceLocatorCredential {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::ServiceLocatorNamePassword {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
        }
    }
}

impl Default for structs::ServiceLocatorSamlCredential {
    fn default() -> Self {
        Self {
            token: None,
        }
    }
}

impl Default for structs::ServiceManagerServiceInfo {
    fn default() -> Self {
        Self {
            service_name: String::new(),
            location: None,
            service: structs::ManagedObjectReference::default(),
            description: String::new(),
        }
    }
}

impl Default for structs::SessionManagerGenericServiceTicket {
    fn default() -> Self {
        Self {
            id: String::new(),
            host_name: None,
            ssl_thumbprint: None,
            cert_thumbprint_list: None,
            ssl_certificate: None,
            ticket_type: None,
        }
    }
}

impl Default for structs::SessionManagerLocalTicket {
    fn default() -> Self {
        Self {
            user_name: String::new(),
            password_file_path: String::new(),
        }
    }
}

impl Default for structs::SessionManagerServiceRequestSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::SessionManagerHttpServiceRequestSpec {
    fn default() -> Self {
        Self {
            method: None,
            url: String::new(),
        }
    }
}

impl Default for structs::SessionManagerVmomiServiceRequestSpec {
    fn default() -> Self {
        Self {
            method: String::new(),
        }
    }
}

impl Default for structs::SharesInfo {
    fn default() -> Self {
        Self {
            shares: 0,
            level: enums::SharesLevelEnum::default(),
        }
    }
}

impl Default for structs::SharesOption {
    fn default() -> Self {
        Self {
            shares_option: structs::IntOption::default(),
            default_level: enums::SharesLevelEnum::default(),
        }
    }
}

impl Default for structs::SiteInfo {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::StoragePodSummary {
    fn default() -> Self {
        Self {
            name: String::new(),
            capacity: 0,
            free_space: 0,
        }
    }
}

impl Default for structs::StorageIoAllocationInfo {
    fn default() -> Self {
        Self {
            limit: None,
            shares: None,
            reservation: None,
        }
    }
}

impl Default for structs::StorageIoAllocationOption {
    fn default() -> Self {
        Self {
            limit_option: structs::LongOption::default(),
            shares_option: structs::SharesOption::default(),
        }
    }
}

impl Default for structs::StorageIormInfo {
    fn default() -> Self {
        Self {
            enabled: false,
            congestion_threshold_mode: String::new(),
            congestion_threshold: 0,
            percent_of_peak_throughput: None,
            stats_collection_enabled: false,
            reservation_enabled: false,
            stats_aggregation_disabled: None,
            reservable_iops_threshold: None,
        }
    }
}

impl Default for structs::StorageIormConfigOption {
    fn default() -> Self {
        Self {
            enabled_option: structs::BoolOption::default(),
            congestion_threshold_option: structs::IntOption::default(),
            stats_collection_enabled_option: structs::BoolOption::default(),
            reservation_enabled_option: structs::BoolOption::default(),
        }
    }
}

impl Default for structs::StorageIormConfigSpec {
    fn default() -> Self {
        Self {
            enabled: None,
            congestion_threshold_mode: None,
            congestion_threshold: None,
            percent_of_peak_throughput: None,
            stats_collection_enabled: None,
            reservation_enabled: None,
            stats_aggregation_disabled: None,
            reservable_iops_threshold: None,
        }
    }
}

impl Default for structs::PodStorageDrsEntry {
    fn default() -> Self {
        Self {
            storage_drs_config: structs::StorageDrsConfigInfo::default(),
            recommendation: None,
            drs_fault: None,
            action_history: None,
        }
    }
}

impl Default for structs::StoragePerformanceSummary {
    fn default() -> Self {
        Self {
            interval: 0,
            percentile: Vec::new(),
            datastore_read_latency: Vec::new(),
            datastore_write_latency: Vec::new(),
            datastore_vm_latency: Vec::new(),
            datastore_read_iops: Vec::new(),
            datastore_write_iops: Vec::new(),
            sioc_activity_duration: 0,
        }
    }
}

impl Default for structs::StorageResourceManagerStorageProfileStatistics {
    fn default() -> Self {
        Self {
            profile_id: String::new(),
            total_space_mb: 0,
            used_space_mb: 0,
        }
    }
}

impl Default for structs::Tag {
    fn default() -> Self {
        Self {
            key: String::new(),
        }
    }
}

impl Default for structs::TaskDescription {
    fn default() -> Self {
        Self {
            method_info: Vec::new(),
            state: Vec::new(),
            reason: Vec::new(),
        }
    }
}

impl Default for structs::TaskFilterSpec {
    fn default() -> Self {
        Self {
            entity: None,
            time: None,
            user_name: None,
            activation_id: None,
            state: None,
            alarm: None,
            scheduled_task: None,
            event_chain_id: None,
            tag: None,
            parent_task_key: None,
            root_task_key: None,
        }
    }
}

impl Default for structs::TaskFilterSpecByEntity {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
            recursion: enums::TaskFilterSpecRecursionOptionEnum::default(),
        }
    }
}

impl Default for structs::TaskFilterSpecByTime {
    fn default() -> Self {
        Self {
            time_type: enums::TaskFilterSpecTimeOptionEnum::default(),
            begin_time: None,
            end_time: None,
        }
    }
}

impl Default for structs::TaskFilterSpecByUsername {
    fn default() -> Self {
        Self {
            system_user: false,
            user_list: None,
        }
    }
}

impl Default for structs::TaskInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
            task: structs::ManagedObjectReference::default(),
            description: None,
            name: None,
            description_id: String::new(),
            entity: None,
            entity_name: None,
            locked: None,
            state: enums::TaskInfoStateEnum::default(),
            cancelled: false,
            cancelable: false,
            error: None,
            result: None,
            progress: None,
            progress_details: None,
            reason: Default::default(),
            queue_time: String::new(),
            start_time: None,
            complete_time: None,
            event_chain_id: 0,
            change_tag: None,
            parent_task_key: None,
            root_task_key: None,
            activation_id: None,
        }
    }
}

impl Default for structs::TaskInfoFilterSpec {
    fn default() -> Self {
        Self {
            filter_task_results: None,
        }
    }
}

impl Default for structs::TaskInfoFilterSpecFilterTaskResults {
    fn default() -> Self {
        Self {
            remove_all: None,
            description_ids: None,
            filter_in: None,
        }
    }
}

impl Default for structs::TaskManagerTaskViewSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::TaskManagerViewByStartId {
    fn default() -> Self {
        Self {
            count: 0,
            start_id: String::new(),
        }
    }
}

impl Default for structs::TaskReason {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::TaskReasonAlarm {
    fn default() -> Self {
        Self {
            alarm_name: String::new(),
            alarm: structs::ManagedObjectReference::default(),
            entity_name: String::new(),
            entity: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::TaskReasonSchedule {
    fn default() -> Self {
        Self {
            name: String::new(),
            scheduled_task: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::TaskReasonSystem {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::TaskReasonUser {
    fn default() -> Self {
        Self {
            user_name: String::new(),
        }
    }
}

impl Default for structs::UpdateVirtualMachineFilesResult {
    fn default() -> Self {
        Self {
            failed_vm_file: None,
        }
    }
}

impl Default for structs::UpdateVirtualMachineFilesResultFailedVmFileInfo {
    fn default() -> Self {
        Self {
            vm_file: String::new(),
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::UserSearchResult {
    fn default() -> Self {
        Self {
            principal: String::new(),
            full_name: None,
            group: false,
        }
    }
}

impl Default for structs::PosixUserSearchResult {
    fn default() -> Self {
        Self {
            user_search_result_: structs::UserSearchResult::default(),
            id: 0,
            shell_access: None,
        }
    }
}

impl Default for structs::UserSession {
    fn default() -> Self {
        Self {
            key: String::new(),
            user_name: String::new(),
            full_name: String::new(),
            login_time: String::new(),
            last_active_time: String::new(),
            locale: String::new(),
            message_locale: String::new(),
            extension_session: false,
            ip_address: String::new(),
            user_agent: String::new(),
            call_count: 0,
        }
    }
}

impl Default for structs::VVolVmConfigFileUpdateResult {
    fn default() -> Self {
        Self {
            succeeded_vm_config_file: None,
            failed_vm_config_file: None,
        }
    }
}

impl Default for structs::VVolVmConfigFileUpdateResultFailedVmConfigFileInfo {
    fn default() -> Self {
        Self {
            target_config_v_vol_id: String::new(),
            ds_path: None,
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::VasaStorageArray {
    fn default() -> Self {
        Self {
            name: String::new(),
            uuid: String::new(),
            vendor_id: String::new(),
            model_id: String::new(),
            discovery_svc_info: None,
        }
    }
}

impl Default for structs::VasaStorageArrayDiscoveryFcTransport {
    fn default() -> Self {
        Self {
            node_wwn: String::new(),
            port_wwn: String::new(),
        }
    }
}

impl Default for structs::VasaStorageArrayDiscoveryIpTransport {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            port_number: None,
        }
    }
}

impl Default for structs::VasaStorageArrayDiscoverySvcInfo {
    fn default() -> Self {
        Self {
            port_type: String::new(),
            svc_nqn: String::new(),
            ip_info: None,
            fc_info: None,
        }
    }
}

impl Default for structs::VasaProviderContainerSpec {
    fn default() -> Self {
        Self {
            vasa_provider_info: None,
            sc_id: String::new(),
            deleted: false,
            stretched: None,
        }
    }
}

impl Default for structs::VimVasaProvider {
    fn default() -> Self {
        Self {
            uid: None,
            url: String::new(),
            name: None,
            self_signed_certificate: None,
            vhost_config: None,
            version_id: None,
        }
    }
}

impl Default for structs::VimVasaProviderStatePerArray {
    fn default() -> Self {
        Self {
            priority: 0,
            array_id: String::new(),
            active: false,
        }
    }
}

impl Default for structs::VimVasaProviderVirtualHostConfig {
    fn default() -> Self {
        Self {
            vhost_name: None,
            service_host: String::new(),
            service_port: None,
        }
    }
}

impl Default for structs::VimVasaProviderInfo {
    fn default() -> Self {
        Self {
            provider: structs::VimVasaProvider::default(),
            array_state: None,
        }
    }
}

impl Default for structs::VirtualAppLinkInfo {
    fn default() -> Self {
        Self {
            key: structs::ManagedObjectReference::default(),
            destroy_with_parent: None,
        }
    }
}

impl Default for structs::VirtualDiskSpec {
    fn default() -> Self {
        Self {
            disk_type: String::new(),
            adapter_type: String::new(),
        }
    }
}

impl Default for structs::DeviceBackedVirtualDiskSpec {
    fn default() -> Self {
        Self {
            virtual_disk_spec_: structs::VirtualDiskSpec::default(),
            device: String::new(),
        }
    }
}

impl Default for structs::FileBackedVirtualDiskSpec {
    fn default() -> Self {
        Self {
            virtual_disk_spec_: structs::VirtualDiskSpec::default(),
            capacity_kb: 0,
            profile: None,
            crypto: None,
            sector_format: None,
        }
    }
}

impl Default for structs::SeSparseVirtualDiskSpec {
    fn default() -> Self {
        Self {
            file_backed_virtual_disk_spec_: structs::FileBackedVirtualDiskSpec::default(),
            grain_size_kb: None,
        }
    }
}

impl Default for structs::VirtualMachineConnection {
    fn default() -> Self {
        Self {
            label: String::new(),
            client: String::new(),
            user_name: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineMksConnection {
    fn default() -> Self {
        Self {
            virtual_machine_connection_: structs::VirtualMachineConnection::default(),
        }
    }
}

impl Default for structs::DiskChangeInfo {
    fn default() -> Self {
        Self {
            start_offset: 0,
            length: 0,
            changed_area: None,
        }
    }
}

impl Default for structs::DiskChangeExtent {
    fn default() -> Self {
        Self {
            start: 0,
            length: 0,
        }
    }
}

impl Default for structs::VirtualMachineDisplayTopology {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

impl Default for structs::VirtualMachineMksTicket {
    fn default() -> Self {
        Self {
            ticket: String::new(),
            cfg_file: String::new(),
            host: None,
            port: None,
            ssl_thumbprint: None,
        }
    }
}

impl Default for structs::StorageRequirement {
    fn default() -> Self {
        Self {
            datastore: structs::ManagedObjectReference::default(),
            free_space_required_in_kb: 0,
        }
    }
}

impl Default for structs::VirtualMachineTicket {
    fn default() -> Self {
        Self {
            ticket: String::new(),
            cfg_file: String::new(),
            host: None,
            port: None,
            ssl_thumbprint: None,
            cert_thumbprint_list: None,
            ssl_certificate: None,
            url: None,
        }
    }
}

impl Default for structs::VirtualMachineWipeResult {
    fn default() -> Self {
        Self {
            disk_id: 0,
            shrinkable_disk_space: 0,
        }
    }
}

impl Default for structs::VsanComparator {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VsanJsonComparator {
    fn default() -> Self {
        Self {
            comparator: None,
            comparable_value: None,
        }
    }
}

impl Default for structs::VsanNestJsonComparator {
    fn default() -> Self {
        Self {
            nested_comparators: None,
            conjoiner: None,
        }
    }
}

impl Default for structs::VsanDataObfuscationRule {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VsanJsonFilterRule {
    fn default() -> Self {
        Self {
            filter_comparator: None,
            comparable_path: None,
            keys_with_str_val: None,
            property_name: None,
        }
    }
}

impl Default for structs::VsanMassCollectorPropertyParams {
    fn default() -> Self {
        Self {
            property_name: None,
            property_params: None,
        }
    }
}

impl Default for structs::VsanMassCollectorSpec {
    fn default() -> Self {
        Self {
            objects: None,
            object_collection: None,
            properties: Vec::new(),
            properties_params: None,
            constraint: None,
        }
    }
}

impl Default for structs::VsanObjectTypeRule {
    fn default() -> Self {
        Self {
            object_type: None,
            attributes: None,
        }
    }
}

impl Default for structs::VsanRegexBasedRule {
    fn default() -> Self {
        Self {
            rules: None,
        }
    }
}

impl Default for structs::VsanResourceConstraint {
    fn default() -> Self {
        Self {
            target_type: None,
        }
    }
}

impl Default for structs::VsanCompositeConstraint {
    fn default() -> Self {
        Self {
            vsan_resource_constraint_: structs::VsanResourceConstraint::default(),
            nested_constraints: None,
            conjoiner: None,
        }
    }
}

impl Default for structs::VsanPropertyConstraint {
    fn default() -> Self {
        Self {
            vsan_resource_constraint_: structs::VsanResourceConstraint::default(),
            property_name: None,
            comparator: None,
            comparable_value: None,
        }
    }
}

impl Default for structs::VsanUpgradeSystemNetworkPartitionInfo {
    fn default() -> Self {
        Self {
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemPreflightCheckIssue {
    fn default() -> Self {
        Self {
            msg: String::new(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemApiBrokenIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemAutoClaimEnabledOnHostsIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemHostsDisconnectedIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemMissingHostsInClusterIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemNetworkPartitionIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            partitions: Vec::new(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemNotEnoughFreeCapacityIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            reduced_redundancy_upgrade_possible: false,
        }
    }
}

impl Default for structs::VsanUpgradeSystemRogueHostsInClusterIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            uuids: Vec::new(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            uuids: Vec::new(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemWrongEsxVersionIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanBrokenDiskChainIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            uuids: Vec::new(),
        }
    }
}

impl Default for structs::VsanDisallowDataMovementIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
        }
    }
}

impl Default for structs::VsanDisallowEvacuateDataIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            hosts: None,
        }
    }
}

impl Default for structs::VsanDiskUnhealthIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            uuids: Vec::new(),
        }
    }
}

impl Default for structs::VsanHigherObjectsPresentDuringDowngradeIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            uuids: Vec::new(),
        }
    }
}

impl Default for structs::VsanHostPropertyRetrieveIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanHostWithHybridDiskgroupIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanHostsCompressionOnlyNotSupported {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanMixedEsxVersionInClientIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            cluster_name: String::new(),
        }
    }
}

impl Default for structs::VsanMixedEsxVersionIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
        }
    }
}

impl Default for structs::VsanObjectInaccessibleIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            uuids: Vec::new(),
        }
    }
}

impl Default for structs::VsanObjectPolicyIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            uuids: Vec::new(),
        }
    }
}

impl Default for structs::VsanRemoteClusterNotCompatible {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            compatibility_info: Vec::new(),
        }
    }
}

impl Default for structs::VsanUnknownScanIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            uuids: Vec::new(),
        }
    }
}

impl Default for structs::VsanUnsupportedHighDiskVersionIssue {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_issue_: structs::VsanUpgradeSystemPreflightCheckIssue::default(),
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemPreflightCheckResult {
    fn default() -> Self {
        Self {
            issues: None,
            disk_mapping_to_restore: None,
        }
    }
}

impl Default for structs::VsanDiskFormatConversionCheckResult {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_preflight_check_result_: structs::VsanUpgradeSystemPreflightCheckResult::default(),
            is_supported: false,
            target_version: None,
            is_data_movement_required: None,
            storage_pool_disk: None,
        }
    }
}

impl Default for structs::VsanUpgradeSystemUpgradeHistoryItem {
    fn default() -> Self {
        Self {
            timestamp: String::new(),
            host: None,
            message: String::new(),
            task: None,
        }
    }
}

impl Default for structs::VsanUpgradeSystemUpgradeHistoryDiskGroupOp {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_upgrade_history_item_: structs::VsanUpgradeSystemUpgradeHistoryItem::default(),
            operation: String::new(),
            disk_mapping: structs::VsanHostDiskMapping::default(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemUpgradeHistoryPreflightFail {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_upgrade_history_item_: structs::VsanUpgradeSystemUpgradeHistoryItem::default(),
            preflight_result: Default::default(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemUpgradeHistoryStoragePoolOp {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_upgrade_history_item_: structs::VsanUpgradeSystemUpgradeHistoryItem::default(),
            operation: String::new(),
            disk_info: structs::VimVsanHostStoragePoolDiskInfo::default(),
        }
    }
}

impl Default for structs::VsanUpgradeSystemUpgradeStatus {
    fn default() -> Self {
        Self {
            in_progress: false,
            history: None,
            aborted: None,
            completed: None,
            progress: None,
        }
    }
}

impl Default for structs::VsanUpgradeStatusEx {
    fn default() -> Self {
        Self {
            vsan_upgrade_system_upgrade_status_: structs::VsanUpgradeSystemUpgradeStatus::default(),
            is_precheck: None,
            precheck_result: None,
        }
    }
}

impl Default for structs::Action {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CreateTaskAction {
    fn default() -> Self {
        Self {
            task_type_id: String::new(),
            cancelable: false,
        }
    }
}

impl Default for structs::MethodAction {
    fn default() -> Self {
        Self {
            name: String::new(),
            argument: None,
        }
    }
}

impl Default for structs::RunScriptAction {
    fn default() -> Self {
        Self {
            script: String::new(),
        }
    }
}

impl Default for structs::SendEmailAction {
    fn default() -> Self {
        Self {
            to_list: String::new(),
            cc_list: String::new(),
            subject: String::new(),
            body: String::new(),
        }
    }
}

impl Default for structs::SendSnmpAction {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::MethodActionArgument {
    fn default() -> Self {
        Self {
            value: None,
        }
    }
}

impl Default for structs::AlarmAction {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::AlarmTriggeringAction {
    fn default() -> Self {
        Self {
            action: Default::default(),
            transition_specs: None,
            green_2_yellow: false,
            yellow_2_red: false,
            red_2_yellow: false,
            yellow_2_green: false,
        }
    }
}

impl Default for structs::GroupAlarmAction {
    fn default() -> Self {
        Self {
            action: Vec::new(),
        }
    }
}

impl Default for structs::AlarmDescription {
    fn default() -> Self {
        Self {
            expr: Vec::new(),
            state_operator: Vec::new(),
            metric_operator: Vec::new(),
            host_system_connection_state: Vec::new(),
            virtual_machine_power_state: Vec::new(),
            datastore_connection_state: Vec::new(),
            host_system_power_state: Vec::new(),
            virtual_machine_guest_heartbeat_status: Vec::new(),
            entity_status: Vec::new(),
            action: Vec::new(),
        }
    }
}

impl Default for structs::AlarmExpression {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::AndAlarmExpression {
    fn default() -> Self {
        Self {
            expression: Vec::new(),
        }
    }
}

impl Default for structs::EventAlarmExpression {
    fn default() -> Self {
        Self {
            comparisons: None,
            event_type: String::new(),
            event_type_id: None,
            object_type: None,
            status: None,
        }
    }
}

impl Default for structs::MetricAlarmExpression {
    fn default() -> Self {
        Self {
            operator: enums::MetricAlarmOperatorEnum::default(),
            r#type: String::new(),
            metric: structs::PerfMetricId::default(),
            yellow: None,
            yellow_interval: None,
            red: None,
            red_interval: None,
        }
    }
}

impl Default for structs::OrAlarmExpression {
    fn default() -> Self {
        Self {
            expression: Vec::new(),
        }
    }
}

impl Default for structs::StateAlarmExpression {
    fn default() -> Self {
        Self {
            operator: enums::StateAlarmOperatorEnum::default(),
            r#type: String::new(),
            state_path: String::new(),
            yellow: None,
            red: None,
        }
    }
}

impl Default for structs::AlarmFilterSpec {
    fn default() -> Self {
        Self {
            status: None,
            type_entity: None,
            type_trigger: None,
        }
    }
}

impl Default for structs::AlarmSetting {
    fn default() -> Self {
        Self {
            tolerance_range: 0,
            reporting_frequency: 0,
        }
    }
}

impl Default for structs::AlarmSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            system_name: None,
            description: String::new(),
            enabled: false,
            expression: Default::default(),
            action: None,
            action_frequency: None,
            setting: None,
        }
    }
}

impl Default for structs::AlarmInfo {
    fn default() -> Self {
        Self {
            alarm_spec_: structs::AlarmSpec::default(),
            key: String::new(),
            alarm: structs::ManagedObjectReference::default(),
            entity: structs::ManagedObjectReference::default(),
            last_modified_time: String::new(),
            last_modified_user: String::new(),
            creation_event_id: 0,
        }
    }
}

impl Default for structs::AlarmState {
    fn default() -> Self {
        Self {
            key: String::new(),
            entity: structs::ManagedObjectReference::default(),
            alarm: structs::ManagedObjectReference::default(),
            overall_status: enums::ManagedEntityStatusEnum::default(),
            time: String::new(),
            acknowledged: None,
            acknowledged_by_user: None,
            acknowledged_time: None,
            event_key: None,
            disabled: None,
        }
    }
}

impl Default for structs::AlarmTriggeringActionTransitionSpec {
    fn default() -> Self {
        Self {
            start_state: enums::ManagedEntityStatusEnum::default(),
            final_state: enums::ManagedEntityStatusEnum::default(),
            repeats: false,
        }
    }
}

impl Default for structs::EventAlarmExpressionComparison {
    fn default() -> Self {
        Self {
            attribute_name: String::new(),
            operator: String::new(),
            value: String::new(),
        }
    }
}

impl Default for structs::ClusterAction {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            target: None,
        }
    }
}

impl Default for structs::ClusterClusterInitialPlacementAction {
    fn default() -> Self {
        Self {
            cluster_action_: structs::ClusterAction::default(),
            target_host: None,
            pool: structs::ManagedObjectReference::default(),
            config_spec: None,
        }
    }
}

impl Default for structs::ClusterHostInfraUpdateHaModeAction {
    fn default() -> Self {
        Self {
            cluster_action_: structs::ClusterAction::default(),
            operation_type: String::new(),
        }
    }
}

impl Default for structs::ClusterHostPowerAction {
    fn default() -> Self {
        Self {
            cluster_action_: structs::ClusterAction::default(),
            operation_type: enums::HostPowerOperationTypeEnum::default(),
            power_consumption_watt: None,
            cpu_capacity_m_hz: None,
            mem_capacity_mb: None,
        }
    }
}

impl Default for structs::ClusterInitialPlacementAction {
    fn default() -> Self {
        Self {
            cluster_action_: structs::ClusterAction::default(),
            target_host: structs::ManagedObjectReference::default(),
            pool: None,
        }
    }
}

impl Default for structs::ClusterMigrationAction {
    fn default() -> Self {
        Self {
            cluster_action_: structs::ClusterAction::default(),
            drs_migration: None,
        }
    }
}

impl Default for structs::PlacementAction {
    fn default() -> Self {
        Self {
            cluster_action_: structs::ClusterAction::default(),
            vm: None,
            target_host: None,
            relocate_spec: None,
        }
    }
}

impl Default for structs::HbrDiskMigrationAction {
    fn default() -> Self {
        Self {
            cluster_action_: structs::ClusterAction::default(),
            collection_id: String::new(),
            collection_name: String::new(),
            disk_ids: Vec::new(),
            source: structs::ManagedObjectReference::default(),
            destination: structs::ManagedObjectReference::default(),
            size_transferred: 0,
            space_util_src_before: None,
            space_util_dst_before: None,
            space_util_src_after: None,
            space_util_dst_after: None,
            io_latency_src_before: None,
            io_latency_dst_before: None,
        }
    }
}

impl Default for structs::StorageMigrationAction {
    fn default() -> Self {
        Self {
            cluster_action_: structs::ClusterAction::default(),
            vm: structs::ManagedObjectReference::default(),
            relocate_spec: structs::VirtualMachineRelocateSpec::default(),
            source: structs::ManagedObjectReference::default(),
            destination: structs::ManagedObjectReference::default(),
            size_transferred: 0,
            space_util_src_before: None,
            space_util_dst_before: None,
            space_util_src_after: None,
            space_util_dst_after: None,
            io_latency_src_before: None,
            io_latency_dst_before: None,
        }
    }
}

impl Default for structs::StoragePlacementAction {
    fn default() -> Self {
        Self {
            cluster_action_: structs::ClusterAction::default(),
            vm: None,
            relocate_spec: structs::VirtualMachineRelocateSpec::default(),
            destination: structs::ManagedObjectReference::default(),
            space_util_before: None,
            space_demand_before: None,
            space_util_after: None,
            space_demand_after: None,
            io_latency_before: None,
        }
    }
}

impl Default for structs::ClusterActionHistory {
    fn default() -> Self {
        Self {
            action: Default::default(),
            time: String::new(),
        }
    }
}

impl Default for structs::ClusterAttemptedVmInfo {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            task: None,
        }
    }
}

impl Default for structs::ClusterPowerContext {
    fn default() -> Self {
        Self {
            current_cluster_power_status: String::new(),
            ordered_cluster_power_status: None,
            tracking_task: None,
            last_error_message: None,
            last_error_m_os: None,
        }
    }
}

impl Default for structs::ClusterConfigInfo {
    fn default() -> Self {
        Self {
            das_config: structs::ClusterDasConfigInfo::default(),
            das_vm_config: None,
            drs_config: structs::ClusterDrsConfigInfo::default(),
            drs_vm_config: None,
            rule: None,
        }
    }
}

impl Default for structs::ClusterConfigSpec {
    fn default() -> Self {
        Self {
            das_config: None,
            das_vm_config_spec: None,
            drs_config: None,
            drs_vm_config_spec: None,
            rules_spec: None,
        }
    }
}

impl Default for structs::ClusterCryptoConfigInfo {
    fn default() -> Self {
        Self {
            crypto_mode: None,
            policy: None,
        }
    }
}

impl Default for structs::ClusterDasAamNodeState {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            name: String::new(),
            config_state: String::new(),
            runtime_state: String::new(),
        }
    }
}

impl Default for structs::ClusterDasAdmissionControlInfo {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::ClusterFailoverHostAdmissionControlInfo {
    fn default() -> Self {
        Self {
            host_status: None,
        }
    }
}

impl Default for structs::ClusterFailoverLevelAdmissionControlInfo {
    fn default() -> Self {
        Self {
            current_failover_level: 0,
        }
    }
}

impl Default for structs::ClusterFailoverResourcesAdmissionControlInfo {
    fn default() -> Self {
        Self {
            current_cpu_failover_resources_percent: 0,
            current_memory_failover_resources_percent: 0,
            current_p_mem_failover_resources_percent: None,
        }
    }
}

impl Default for structs::ClusterDasAdmissionControlPolicy {
    fn default() -> Self {
        Self {
            resource_reduction_to_tolerate_percent: None,
            p_mem_admission_control_enabled: None,
        }
    }
}

impl Default for structs::ClusterFailoverHostAdmissionControlPolicy {
    fn default() -> Self {
        Self {
            cluster_das_admission_control_policy_: structs::ClusterDasAdmissionControlPolicy::default(),
            failover_hosts: None,
            failover_level: None,
        }
    }
}

impl Default for structs::ClusterFailoverLevelAdmissionControlPolicy {
    fn default() -> Self {
        Self {
            cluster_das_admission_control_policy_: structs::ClusterDasAdmissionControlPolicy::default(),
            failover_level: 0,
            slot_policy: None,
        }
    }
}

impl Default for structs::ClusterFailoverResourcesAdmissionControlPolicy {
    fn default() -> Self {
        Self {
            cluster_das_admission_control_policy_: structs::ClusterDasAdmissionControlPolicy::default(),
            cpu_failover_resources_percent: 0,
            memory_failover_resources_percent: 0,
            failover_level: None,
            auto_compute_percentages: None,
            p_mem_failover_resources_percent: None,
            auto_compute_p_mem_failover_resources_percent: None,
        }
    }
}

impl Default for structs::ClusterDasAdvancedRuntimeInfo {
    fn default() -> Self {
        Self {
            das_host_info: None,
            vmcp_supported: None,
            heartbeat_datastore_info: None,
        }
    }
}

impl Default for structs::ClusterDasFailoverLevelAdvancedRuntimeInfo {
    fn default() -> Self {
        Self {
            cluster_das_advanced_runtime_info_: structs::ClusterDasAdvancedRuntimeInfo::default(),
            slot_info: structs::ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo::default(),
            total_slots: 0,
            used_slots: 0,
            unreserved_slots: 0,
            total_vms: 0,
            total_hosts: 0,
            total_good_hosts: 0,
            host_slots: None,
            vms_requiring_multiple_slots: None,
        }
    }
}

impl Default for structs::DasHeartbeatDatastoreInfo {
    fn default() -> Self {
        Self {
            datastore: structs::ManagedObjectReference::default(),
            hosts: Vec::new(),
        }
    }
}

impl Default for structs::ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo {
    fn default() -> Self {
        Self {
            storage_apd_supported: false,
            storage_pdl_supported: false,
        }
    }
}

impl Default for structs::ClusterDasConfigInfo {
    fn default() -> Self {
        Self {
            enabled: None,
            vm_monitoring: None,
            host_monitoring: None,
            vm_component_protecting: None,
            failover_level: None,
            admission_control_policy: None,
            admission_control_enabled: None,
            default_vm_settings: None,
            option: None,
            heartbeat_datastore: None,
            h_b_datastore_candidate_policy: None,
        }
    }
}

impl Default for structs::ClusterDasData {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::ClusterDasDataSummary {
    fn default() -> Self {
        Self {
            host_list_version: 0,
            cluster_config_version: 0,
            compat_list_version: 0,
        }
    }
}

impl Default for structs::ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            slots: 0,
        }
    }
}

impl Default for structs::ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo {
    fn default() -> Self {
        Self {
            num_vcpus: 0,
            cpu_m_hz: 0,
            memory_mb: 0,
        }
    }
}

impl Default for structs::ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            slots: 0,
        }
    }
}

impl Default for structs::ClusterDasFdmHostState {
    fn default() -> Self {
        Self {
            state: String::new(),
            state_reporter: None,
        }
    }
}

impl Default for structs::ClusterDasHostInfo {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::ClusterDasAamHostInfo {
    fn default() -> Self {
        Self {
            host_das_state: None,
            primary_hosts: None,
        }
    }
}

impl Default for structs::ClusterDasHostRecommendation {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            drs_rating: None,
        }
    }
}

impl Default for structs::ClusterDasVmConfigInfo {
    fn default() -> Self {
        Self {
            key: structs::ManagedObjectReference::default(),
            restart_priority: None,
            power_off_on_isolation: None,
            das_settings: None,
        }
    }
}

impl Default for structs::ClusterDasVmSettings {
    fn default() -> Self {
        Self {
            restart_priority: None,
            restart_priority_timeout: None,
            isolation_response: None,
            vm_tools_monitoring_settings: None,
            vm_component_protection_settings: None,
        }
    }
}

impl Default for structs::ClusterDpmConfigInfo {
    fn default() -> Self {
        Self {
            enabled: None,
            default_dpm_behavior: None,
            host_power_action_rate: None,
            option: None,
        }
    }
}

impl Default for structs::ClusterDpmHostConfigInfo {
    fn default() -> Self {
        Self {
            key: structs::ManagedObjectReference::default(),
            enabled: None,
            behavior: None,
        }
    }
}

impl Default for structs::ClusterDrsConfigInfo {
    fn default() -> Self {
        Self {
            enabled: None,
            enable_vm_behavior_overrides: None,
            default_vm_behavior: None,
            vmotion_rate: None,
            scale_descendants_shares: None,
            option: None,
        }
    }
}

impl Default for structs::ClusterDrsFaults {
    fn default() -> Self {
        Self {
            reason: String::new(),
            faults_by_vm: Vec::new(),
        }
    }
}

impl Default for structs::ClusterDrsFaultsFaultsByVm {
    fn default() -> Self {
        Self {
            vm: None,
            fault: Vec::new(),
        }
    }
}

impl Default for structs::ClusterDrsFaultsFaultsByVirtualDisk {
    fn default() -> Self {
        Self {
            cluster_drs_faults_faults_by_vm_: structs::ClusterDrsFaultsFaultsByVm::default(),
            disk: None,
        }
    }
}

impl Default for structs::ClusterDrsMigration {
    fn default() -> Self {
        Self {
            key: String::new(),
            time: String::new(),
            vm: structs::ManagedObjectReference::default(),
            cpu_load: None,
            memory_load: None,
            source: structs::ManagedObjectReference::default(),
            source_cpu_load: None,
            source_memory_load: None,
            destination: structs::ManagedObjectReference::default(),
            destination_cpu_load: None,
            destination_memory_load: None,
        }
    }
}

impl Default for structs::ClusterDrsRecommendation {
    fn default() -> Self {
        Self {
            key: String::new(),
            rating: 0,
            reason: String::new(),
            reason_text: String::new(),
            migration_list: Vec::new(),
        }
    }
}

impl Default for structs::ClusterDrsVmConfigInfo {
    fn default() -> Self {
        Self {
            key: structs::ManagedObjectReference::default(),
            enabled: None,
            behavior: None,
        }
    }
}

impl Default for structs::ClusterEvcManagerCheckResult {
    fn default() -> Self {
        Self {
            evc_mode_key: String::new(),
            error: structs::MethodFault::default(),
            host: None,
        }
    }
}

impl Default for structs::ClusterEvcManagerEvcState {
    fn default() -> Self {
        Self {
            supported_evc_mode: Vec::new(),
            current_evc_mode_key: None,
            guaranteed_cpu_features: None,
            feature_capability: None,
            feature_mask: None,
            feature_requirement: None,
        }
    }
}

impl Default for structs::ClusterEnterMaintenanceResult {
    fn default() -> Self {
        Self {
            recommendations: None,
            fault: None,
        }
    }
}

impl Default for structs::ClusterFailoverHostAdmissionControlInfoHostStatus {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            status: enums::ManagedEntityStatusEnum::default(),
        }
    }
}

impl Default for structs::ClusterGroupInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            user_created: None,
            unique_id: None,
        }
    }
}

impl Default for structs::ClusterHostGroup {
    fn default() -> Self {
        Self {
            cluster_group_info_: structs::ClusterGroupInfo::default(),
            host: None,
        }
    }
}

impl Default for structs::ClusterVmGroup {
    fn default() -> Self {
        Self {
            cluster_group_info_: structs::ClusterGroupInfo::default(),
            vm: None,
        }
    }
}

impl Default for structs::ClusterHostRecommendation {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            rating: 0,
        }
    }
}

impl Default for structs::ClusterInfraUpdateHaConfigInfo {
    fn default() -> Self {
        Self {
            enabled: None,
            behavior: None,
            moderate_remediation: None,
            severe_remediation: None,
            providers: None,
        }
    }
}

impl Default for structs::ClusterNotAttemptedVmInfo {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::ClusterOrchestrationInfo {
    fn default() -> Self {
        Self {
            default_vm_readiness: None,
        }
    }
}

impl Default for structs::PerformClusterPowerActionSpec {
    fn default() -> Self {
        Self {
            target_power_status: String::new(),
            is_orchestration: None,
            initial_power_status: None,
            power_off_reason: None,
            infra_v_ms: None,
            infra_vm_uuids: None,
        }
    }
}

impl Default for structs::PlacementResult {
    fn default() -> Self {
        Self {
            recommendations: None,
            drs_fault: None,
        }
    }
}

impl Default for structs::PlacementSpec {
    fn default() -> Self {
        Self {
            priority: None,
            vm: None,
            config_spec: None,
            relocate_spec: None,
            hosts: None,
            datastores: None,
            storage_pods: None,
            disallow_prerequisite_moves: None,
            rules: None,
            key: None,
            placement_type: None,
            clone_spec: None,
            clone_name: None,
        }
    }
}

impl Default for structs::ClusterPowerOnVmResult {
    fn default() -> Self {
        Self {
            attempted: None,
            not_attempted: None,
            recommendations: None,
        }
    }
}

impl Default for structs::ClusterPreemptibleVmPairInfo {
    fn default() -> Self {
        Self {
            id: None,
            monitored_vm: structs::ManagedObjectReference::default(),
            preemptible_vm: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::ClusterProactiveDrsConfigInfo {
    fn default() -> Self {
        Self {
            enabled: None,
        }
    }
}

impl Default for structs::QueryVsanManagedStorageSpaceUsageSpec {
    fn default() -> Self {
        Self {
            datastore_types: Vec::new(),
        }
    }
}

impl Default for structs::ClusterRecommendation {
    fn default() -> Self {
        Self {
            key: String::new(),
            r#type: String::new(),
            time: String::new(),
            rating: 0,
            reason: String::new(),
            reason_text: String::new(),
            warning_text: None,
            warning_details: None,
            prerequisite: None,
            action: None,
            target: None,
        }
    }
}

impl Default for structs::ClusterResourceUsageSummary {
    fn default() -> Self {
        Self {
            cpu_used_m_hz: 0,
            cpu_capacity_m_hz: 0,
            mem_used_mb: 0,
            mem_capacity_mb: 0,
            p_mem_available_mb: None,
            p_mem_capacity_mb: None,
            storage_used_mb: 0,
            storage_capacity_mb: 0,
        }
    }
}

impl Default for structs::ClusterRuleInfo {
    fn default() -> Self {
        Self {
            key: None,
            status: None,
            enabled: None,
            name: None,
            mandatory: None,
            user_created: None,
            in_compliance: None,
            rule_uuid: None,
        }
    }
}

impl Default for structs::ClusterAffinityRuleSpec {
    fn default() -> Self {
        Self {
            cluster_rule_info_: structs::ClusterRuleInfo::default(),
            vm: Vec::new(),
        }
    }
}

impl Default for structs::ClusterAntiAffinityRuleSpec {
    fn default() -> Self {
        Self {
            cluster_rule_info_: structs::ClusterRuleInfo::default(),
            vm: Vec::new(),
        }
    }
}

impl Default for structs::ClusterDependencyRuleInfo {
    fn default() -> Self {
        Self {
            cluster_rule_info_: structs::ClusterRuleInfo::default(),
            vm_group: String::new(),
            depends_on_vm_group: String::new(),
        }
    }
}

impl Default for structs::ClusterFtVmHostRuleInfo {
    fn default() -> Self {
        Self {
            cluster_rule_info_: structs::ClusterRuleInfo::default(),
            vm_group_name: String::new(),
            host_group_name: None,
        }
    }
}

impl Default for structs::ClusterVmHostRuleInfo {
    fn default() -> Self {
        Self {
            cluster_rule_info_: structs::ClusterRuleInfo::default(),
            vm_group_name: None,
            affine_host_group_name: None,
            anti_affine_host_group_name: None,
        }
    }
}

impl Default for structs::VirtualDiskAntiAffinityRuleSpec {
    fn default() -> Self {
        Self {
            cluster_rule_info_: structs::ClusterRuleInfo::default(),
            disk_id: Vec::new(),
        }
    }
}

impl Default for structs::VirtualDiskRuleSpec {
    fn default() -> Self {
        Self {
            cluster_rule_info_: structs::ClusterRuleInfo::default(),
            disk_rule_type: String::new(),
            disk_id: None,
        }
    }
}

impl Default for structs::VsanSiteFaultDomain {
    fn default() -> Self {
        Self {
            hosts: None,
            name: String::new(),
        }
    }
}

impl Default for structs::VsanSiteFaultDomainConfig {
    fn default() -> Self {
        Self {
            site_fault_domains: None,
        }
    }
}

impl Default for structs::ClusterSlotPolicy {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::ClusterFixedSizeSlotPolicy {
    fn default() -> Self {
        Self {
            cpu: 0,
            memory: 0,
        }
    }
}

impl Default for structs::VsanStorageComplianceResult {
    fn default() -> Self {
        Self {
            check_time: None,
            profile: None,
            object_uuid: None,
            compliance_status: String::new(),
            mismatch: false,
            violated_policies: None,
            operational_status: None,
            obj_policy_generation_id: None,
        }
    }
}

impl Default for structs::VsanStorageOperationalStatus {
    fn default() -> Self {
        Self {
            healthy: None,
            operation_eta: None,
            operation_progress: None,
            transitional: None,
        }
    }
}

impl Default for structs::VsanStoragePolicyStatus {
    fn default() -> Self {
        Self {
            id: None,
            expected_value: None,
            current_value: None,
        }
    }
}

impl Default for structs::ClusterSystemVMsConfigInfo {
    fn default() -> Self {
        Self {
            allowed_datastores: None,
            not_allowed_datastores: None,
            ds_tag_categories_to_exclude: None,
            deployment_mode: None,
        }
    }
}

impl Default for structs::ClusterSystemVMsConfigSpec {
    fn default() -> Self {
        Self {
            allowed_datastores: None,
            not_allowed_datastores: None,
            ds_tag_categories_to_exclude: None,
            deployment_mode: None,
        }
    }
}

impl Default for structs::ClusterUsageSummary {
    fn default() -> Self {
        Self {
            total_cpu_capacity_mhz: 0,
            total_mem_capacity_mb: 0,
            cpu_reservation_mhz: 0,
            mem_reservation_mb: 0,
            powered_off_cpu_reservation_mhz: None,
            powered_off_mem_reservation_mb: None,
            cpu_demand_mhz: 0,
            mem_demand_mb: 0,
            stats_gen_number: 0,
            cpu_entitled_mhz: 0,
            mem_entitled_mb: 0,
            powered_off_vm_count: 0,
            total_vm_count: 0,
            tier_0_mem_capacity_mb: None,
            reserved_tier_0_mem_mb: None,
            unreserved_tier_0_mem_mb: None,
        }
    }
}

impl Default for structs::VimClusterVsanPreferredFaultDomainInfo {
    fn default() -> Self {
        Self {
            preferred_fault_domain_name: None,
            preferred_fault_domain_id: None,
        }
    }
}

impl Default for structs::VimClusterVsanStretchedClusterCapability {
    fn default() -> Self {
        Self {
            host_mo_id: String::new(),
            conn_status: None,
            is_supported: None,
            host_capability: None,
        }
    }
}

impl Default for structs::VimClusterVsanStretchedClusterFaultDomainConfig {
    fn default() -> Self {
        Self {
            first_fd_name: String::new(),
            first_fd_hosts: Vec::new(),
            second_fd_name: String::new(),
            second_fd_hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanStretchedClusterHostVirtualApplianceStatus {
    fn default() -> Self {
        Self {
            vc_cluster: None,
            is_virtual_app: None,
            vc_clusters: None,
            is_virtual_app_valid: None,
        }
    }
}

impl Default for structs::VimClusterVsanWitnessHostInfo {
    fn default() -> Self {
        Self {
            node_uuid: String::new(),
            fault_domain_name: None,
            preferred_fd_name: None,
            preferred_fd_uuid: None,
            unicast_agent_addr: None,
            host: None,
            metadata_mode: None,
        }
    }
}

impl Default for structs::ClusterVmComponentProtectionSettings {
    fn default() -> Self {
        Self {
            vm_storage_protection_for_apd: None,
            enable_apd_timeout_for_hosts: None,
            vm_terminate_delay_for_apd_sec: None,
            vm_reaction_on_apd_cleared: None,
            vm_storage_protection_for_pdl: None,
        }
    }
}

impl Default for structs::ClusterVmOrchestrationInfo {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            vm_readiness: structs::ClusterVmReadiness::default(),
        }
    }
}

impl Default for structs::ClusterVmReadiness {
    fn default() -> Self {
        Self {
            ready_condition: None,
            post_ready_delay: None,
        }
    }
}

impl Default for structs::ClusterVmToolsMonitoringSettings {
    fn default() -> Self {
        Self {
            enabled: None,
            vm_monitoring: None,
            cluster_settings: None,
            failure_interval: None,
            min_up_time: None,
            max_failures: None,
            max_failure_window: None,
        }
    }
}

impl Default for structs::VsanAttachToSrOperation {
    fn default() -> Self {
        Self {
            task: None,
            success: None,
            timestamp: None,
            sr_number: String::new(),
        }
    }
}

impl Default for structs::VsanCapability {
    fn default() -> Self {
        Self {
            target: None,
            capabilities: None,
            statuses: None,
        }
    }
}

impl Default for structs::VsanClusterAdvCfgSyncHostResult {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            value: String::new(),
            is_default: None,
        }
    }
}

impl Default for structs::VsanClusterAdvCfgSyncResult {
    fn default() -> Self {
        Self {
            in_sync: false,
            name: String::new(),
            host_values: None,
        }
    }
}

impl Default for structs::VsanClusterBalancePerDiskInfo {
    fn default() -> Self {
        Self {
            uuid: None,
            fullness: 0,
            variance: 0,
            fullness_above_threshold: 0,
            data_to_move_b: 0,
            comp_fullness: None,
            comp_variance: None,
        }
    }
}

impl Default for structs::VsanClusterBalanceSummary {
    fn default() -> Self {
        Self {
            variance_threshold: 0,
            disks: None,
        }
    }
}

impl Default for structs::VsanClusterClomdLivenessResult {
    fn default() -> Self {
        Self {
            clomd_liveness_result: None,
            issue_found: false,
        }
    }
}

impl Default for structs::VsanClusterConfig {
    fn default() -> Self {
        Self {
            config: Default::default(),
            name: String::new(),
            hosts: None,
            to_be_deleted: None,
        }
    }
}

impl Default for structs::VsanClusterCreateVmHealthTestResult {
    fn default() -> Self {
        Self {
            cluster_result: structs::VsanClusterProactiveTestResult::default(),
            host_results: None,
        }
    }
}

impl Default for structs::VsanClusterDitEncryptionHealthSummary {
    fn default() -> Self {
        Self {
            overall_health: String::new(),
            enabled: None,
            host_results: None,
        }
    }
}

impl Default for structs::VsanClusterEncryptionHealthSummary {
    fn default() -> Self {
        Self {
            overall_health: None,
            config_health: None,
            kms_health: None,
            vc_kms_result: None,
            host_results: None,
            aesni_health: None,
        }
    }
}

impl Default for structs::VsanClusterFileServiceHealthSummary {
    fn default() -> Self {
        Self {
            overall_health: None,
            host_results: None,
        }
    }
}

impl Default for structs::VsanClusterGlobalDedupHealthSummary {
    fn default() -> Self {
        Self {
            dedup_config_health: None,
            dedup_store_health: None,
        }
    }
}

impl Default for structs::VsanClusterHclInfo {
    fn default() -> Self {
        Self {
            hcl_db_last_update: None,
            hcl_db_age_health: None,
            host_results: None,
            update_items: None,
            hcl_db_absent: None,
        }
    }
}

impl Default for structs::VsanClusterHealthAction {
    fn default() -> Self {
        Self {
            action_id: String::new(),
            action_label: structs::LocalizableMessage::default(),
            action_description: structs::LocalizableMessage::default(),
            enabled: false,
            parameters: None,
        }
    }
}

impl Default for structs::VsanClusterHealthCheckInfo {
    fn default() -> Self {
        Self {
            test_id: String::new(),
            test_name: None,
            group_id: String::new(),
            group_name: None,
        }
    }
}

impl Default for structs::VsanClusterHealthConfigs {
    fn default() -> Self {
        Self {
            enable_vsan_telemetry: None,
            vsan_telemetry_interval: None,
            vsan_telemetry_proxy: None,
            configs: None,
        }
    }
}

impl Default for structs::VsanClusterHealthGroup {
    fn default() -> Self {
        Self {
            group_id: String::new(),
            group_name: String::new(),
            group_health: String::new(),
            group_tests: None,
            group_details: None,
            in_progress: None,
        }
    }
}

impl Default for structs::VsanClusterHealthLinkBase {
    fn default() -> Self {
        Self {
            label: None,
        }
    }
}

impl Default for structs::VsanClusterHealthLink {
    fn default() -> Self {
        Self {
            vsan_cluster_health_link_base_: structs::VsanClusterHealthLinkBase::default(),
            url: String::new(),
            category: None,
        }
    }
}

impl Default for structs::VsanClusterHealthQuerySpec {
    fn default() -> Self {
        Self {
            task: None,
            disk_names: None,
            include_health_remediation: None,
        }
    }
}

impl Default for structs::VsanClusterHealthResultBase {
    fn default() -> Self {
        Self {
            label: None,
        }
    }
}

impl Default for structs::VsanClusterHealthResultTable {
    fn default() -> Self {
        Self {
            vsan_cluster_health_result_base_: structs::VsanClusterHealthResultBase::default(),
            columns: None,
            rows: None,
        }
    }
}

impl Default for structs::VsanClusterHealthResultWithRemediation {
    fn default() -> Self {
        Self {
            vsan_cluster_health_result_base_: structs::VsanClusterHealthResultBase::default(),
            issue_description: None,
            issue_detail: None,
            troubleshooting: None,
            additional_resources: None,
        }
    }
}

impl Default for structs::VsanClusterHealthResultColumnInfo {
    fn default() -> Self {
        Self {
            label: String::new(),
            r#type: String::new(),
        }
    }
}

impl Default for structs::VsanClusterHealthResultKeyValuePair {
    fn default() -> Self {
        Self {
            key: None,
            value: None,
        }
    }
}

impl Default for structs::VsanClusterHealthResultRow {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            nested_rows: None,
            actions: None,
        }
    }
}

impl Default for structs::VsanClusterHealthSummary {
    fn default() -> Self {
        Self {
            cluster_status: None,
            timestamp: None,
            cluster_versions: None,
            object_health: None,
            vm_health: None,
            network_health: None,
            limit_health: None,
            adv_cfg_sync: None,
            create_vm_health: None,
            physical_disks_health: None,
            encryption_health: None,
            hcl_info: None,
            groups: None,
            overall_health: String::new(),
            overall_health_description: String::new(),
            clomd_liveness: None,
            disk_balance: None,
            generic_cluster: None,
            network_config: None,
            vsan_config: None,
            burn_in_test: None,
            perfsvc_health: None,
            cluster: None,
            file_service_health: None,
            dit_encryption_health: None,
            health_score: None,
            global_dedup_health: None,
        }
    }
}

impl Default for structs::VsanClusterHealthSystemObjectsRepairResult {
    fn default() -> Self {
        Self {
            in_repairing_queue_objects: None,
            failed_repair_objects: None,
            issue_found: false,
        }
    }
}

impl Default for structs::VsanClusterHealthSystemStatusResult {
    fn default() -> Self {
        Self {
            status: String::new(),
            goal_state: String::new(),
            untracked_hosts: None,
            tracked_hosts_status: None,
        }
    }
}

impl Default for structs::VsanClusterHealthSystemVersionResult {
    fn default() -> Self {
        Self {
            host_results: None,
            vc_version: None,
            issue_found: false,
            upgrade_possible: None,
            vc_build: None,
        }
    }
}

impl Default for structs::VsanClusterHealthTest {
    fn default() -> Self {
        Self {
            test_id: None,
            test_name: None,
            test_description: None,
            test_short_description: None,
            test_healthy_entities: None,
            test_all_entities: None,
            test_health: None,
            test_details: None,
            test_actions: None,
            historical_results: None,
            test_correlation: None,
            reduced_score: None,
            category: None,
            risk_if_not_fix: None,
            last_status_change_time: None,
        }
    }
}

impl Default for structs::VsanClusterHostVmknicMapping {
    fn default() -> Self {
        Self {
            host: String::new(),
            vmknic: String::new(),
        }
    }
}

impl Default for structs::VsanClusterLimitHealthResult {
    fn default() -> Self {
        Self {
            issue_found: false,
            component_limit_health: String::new(),
            disk_free_space_health: String::new(),
            rc_free_reservation_health: String::new(),
            host_results: None,
            whatif_host_failures: None,
            hosts_comm_failure: None,
        }
    }
}

impl Default for structs::VsanClusterNetworkHealthResult {
    fn default() -> Self {
        Self {
            host_results: None,
            issue_found: None,
            vsan_vmknic_present: None,
            matching_multicast_config: None,
            matching_ip_subnets: None,
            ping_test_success: None,
            large_ping_test_success: None,
            host_latency_check_success: None,
            potential_multicast_issue: None,
            other_hosts_in_vsan_cluster: None,
            partitions: None,
            hosts_with_vsan_disabled: None,
            hosts_disconnected: None,
            hosts_comm_failure: None,
            hosts_in_esx_maintenance_mode: None,
            hosts_in_vsan_maintenance_mode: None,
            info_about_unexpected_hosts: None,
            cluster_in_unicast_mode: None,
            cluster_in_rdma_mode: None,
        }
    }
}

impl Default for structs::VsanClusterNetworkLoadTestResult {
    fn default() -> Self {
        Self {
            cluster_result: structs::VsanClusterProactiveTestResult::default(),
            host_results: None,
        }
    }
}

impl Default for structs::VsanClusterNetworkPartitionInfo {
    fn default() -> Self {
        Self {
            hosts: None,
            partition_unknown: None,
        }
    }
}

impl Default for structs::VsanClusterNetworkPerfTaskSpec {
    fn default() -> Self {
        Self {
            cluster: None,
            duration_sec: None,
            owner_vc: None,
        }
    }
}

impl Default for structs::VsanClusterProactiveTestResult {
    fn default() -> Self {
        Self {
            overall_status: String::new(),
            overall_status_description: String::new(),
            timestamp: String::new(),
            health_test: None,
        }
    }
}

impl Default for structs::VsanClusterTelemetryProxyConfig {
    fn default() -> Self {
        Self {
            host: None,
            port: None,
            user: None,
            password: None,
            auto_discovered: None,
        }
    }
}

impl Default for structs::VsanClusterVMsHealthOverallResult {
    fn default() -> Self {
        Self {
            health_state_list: None,
            overall_health_state: None,
        }
    }
}

impl Default for structs::VsanClusterVMsHealthSummaryResult {
    fn default() -> Self {
        Self {
            num_v_ms: 0,
            state: None,
            health: String::new(),
            vm_instance_uuids: None,
        }
    }
}

impl Default for structs::VsanClusterVmdkLoadTestResult {
    fn default() -> Self {
        Self {
            task: None,
            cluster_result: None,
            host_results: None,
        }
    }
}

impl Default for structs::VsanClusterWhatifHostFailuresResult {
    fn default() -> Self {
        Self {
            num_failures: 0,
            total_used_capacity_b: 0,
            total_capacity_b: 0,
            total_rc_reservation_b: 0,
            total_rc_size_b: 0,
            used_components: 0,
            total_components: 0,
            component_limit_health: None,
            disk_free_space_health: None,
            rc_free_reservation_health: None,
            slack_space_cap_required: None,
            disk_space_threshold: None,
            capacity_reservation_info: None,
        }
    }
}

impl Default for structs::VsanConfigGeneration {
    fn default() -> Self {
        Self {
            vc_uuid: String::new(),
            gen_num: 0,
            gen_time: 0,
        }
    }
}

impl Default for structs::VsanDataDrivenApiAction {
    fn default() -> Self {
        Self {
            action_id: String::new(),
            action_label: structs::LocalizableMessage::default(),
            action_description: structs::LocalizableMessage::default(),
            enabled: false,
            parameters: None,
        }
    }
}

impl Default for structs::VsanDiagnosticsThreshold {
    fn default() -> Self {
        Self {
            entity_type: String::new(),
            metric: String::new(),
            yellow: None,
            red: None,
        }
    }
}

impl Default for structs::VsanDiskFormatConversionSpec {
    fn default() -> Self {
        Self {
            data_efficiency_config: None,
            data_encryption_config: None,
            skip_host_remediation: None,
            allow_data_movement: None,
        }
    }
}

impl Default for structs::VimClusterVsanDiskMappingsConfigSpec {
    fn default() -> Self {
        Self {
            host_disk_mappings: Vec::new(),
        }
    }
}

impl Default for structs::VsanEntitySpaceUsage {
    fn default() -> Self {
        Self {
            entity_id: None,
            space_usage_by_object_type: None,
            total_capacity_b: None,
            free_capacity_b: None,
            efficient_capacity: None,
        }
    }
}

impl Default for structs::VimClusterVsanFaultDomainSpec {
    fn default() -> Self {
        Self {
            hosts: None,
            name: String::new(),
        }
    }
}

impl Default for structs::VsanFaultDomainDestroySpec {
    fn default() -> Self {
        Self {
            vim_cluster_vsan_fault_domain_spec_: structs::VimClusterVsanFaultDomainSpec::default(),
        }
    }
}

impl Default for structs::VsanFaultDomainUpdateSpec {
    fn default() -> Self {
        Self {
            vim_cluster_vsan_fault_domain_spec_: structs::VimClusterVsanFaultDomainSpec::default(),
            operation: String::new(),
        }
    }
}

impl Default for structs::VimClusterVsanFaultDomainsConfigSpec {
    fn default() -> Self {
        Self {
            fault_domains: Vec::new(),
            witness: None,
        }
    }
}

impl Default for structs::VsanHealthActionBase {
    fn default() -> Self {
        Self {
            description: String::new(),
        }
    }
}

impl Default for structs::VsanHealthActionSteps {
    fn default() -> Self {
        Self {
            vsan_health_action_base_: structs::VsanHealthActionBase::default(),
            steps: None,
        }
    }
}

impl Default for structs::VsanHealthApiBasedAction {
    fn default() -> Self {
        Self {
            vsan_health_action_base_: structs::VsanHealthActionBase::default(),
            api_action: structs::VsanClusterHealthAction::default(),
        }
    }
}

impl Default for structs::VsanHealthCmdBasedAction {
    fn default() -> Self {
        Self {
            vsan_health_action_base_: structs::VsanHealthActionBase::default(),
            commands: Vec::new(),
        }
    }
}

impl Default for structs::VsanHealthDataDrivenAction {
    fn default() -> Self {
        Self {
            vsan_health_action_base_: structs::VsanHealthActionBase::default(),
            api_action: structs::VsanDataDrivenApiAction::default(),
            confirmation: None,
        }
    }
}

impl Default for structs::VsanHealthTxtBasedAction {
    fn default() -> Self {
        Self {
            vsan_health_action_base_: structs::VsanHealthActionBase::default(),
        }
    }
}

impl Default for structs::VsanHealthConfirmationDialog {
    fn default() -> Self {
        Self {
            title: String::new(),
            sub_title: None,
            content: String::new(),
            agree_label: None,
            close_label: None,
            is_warning: None,
        }
    }
}

impl Default for structs::VsanHealthCorrelation {
    fn default() -> Self {
        Self {
            primary_health_tests: None,
            related_health_tests: None,
            skipped_health_tests: None,
        }
    }
}

impl Default for structs::VsanHealthExtMgmtPreCheckResult {
    fn default() -> Self {
        Self {
            overall_result: false,
            esx_version_check_passed: None,
            drs_check_passed: None,
            eam_connection_check_passed: None,
            install_state_check_passed: None,
            results: Vec::new(),
            vum_registered: None,
        }
    }
}

impl Default for structs::VsanHealthTroubleshooting {
    fn default() -> Self {
        Self {
            diagnostic_steps: None,
            remediations: None,
        }
    }
}

impl Default for structs::VsanHistoricalHealthQuerySpec {
    fn default() -> Self {
        Self {
            clusters: Vec::new(),
            start: String::new(),
            end: None,
            test_id: None,
            group_id: None,
            include_health_remediation: None,
        }
    }
}

impl Default for structs::VsanHistoricalHealthTest {
    fn default() -> Self {
        Self {
            timestamp: String::new(),
            health: String::new(),
            test_details: None,
            test_correlation: None,
        }
    }
}

impl Default for structs::VsanHostClomdLivenessResult {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            clomd_stat: String::new(),
            error: None,
        }
    }
}

impl Default for structs::VsanHostCreateVmHealthTestResult {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            state: String::new(),
            fault: None,
        }
    }
}

impl Default for structs::VimClusterVsanHostDiskMapping {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            cache_disks: None,
            capacity_disks: None,
            r#type: String::new(),
        }
    }
}

impl Default for structs::VsanHostHealthSystemVersionResult {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            version: None,
            error: None,
            build: None,
        }
    }
}

impl Default for structs::VsanIoInsightInstance {
    fn default() -> Self {
        Self {
            run_name: String::new(),
            state: None,
            start_time: None,
            end_time: None,
            hosts_io_insight_info: None,
            host_uuids: None,
            vm_uuids: None,
        }
    }
}

impl Default for structs::VsanIoInsightInstanceQuerySpec {
    fn default() -> Self {
        Self {
            state: None,
            entity_ref_id: None,
        }
    }
}

impl Default for structs::VsanIscsiHomeObjectSpec {
    fn default() -> Self {
        Self {
            storage_policy: None,
            default_config: None,
        }
    }
}

impl Default for structs::VsanIscsiInitiatorGroup {
    fn default() -> Self {
        Self {
            name: String::new(),
            initiators: None,
            targets: None,
        }
    }
}

impl Default for structs::VsanIscsiLunCommonInfo {
    fn default() -> Self {
        Self {
            lun_id: None,
            alias: None,
            lun_size: 0,
            status: None,
        }
    }
}

impl Default for structs::VsanIscsiLun {
    fn default() -> Self {
        Self {
            vsan_iscsi_lun_common_info_: structs::VsanIscsiLunCommonInfo::default(),
            target_alias: String::new(),
            uuid: String::new(),
            actual_size: 0,
            object_information: None,
        }
    }
}

impl Default for structs::VsanIscsiLunSpec {
    fn default() -> Self {
        Self {
            vsan_iscsi_lun_common_info_: structs::VsanIscsiLunCommonInfo::default(),
            storage_policy: None,
            new_lun_id: None,
        }
    }
}

impl Default for structs::VsanIscsiTargetAuthSpec {
    fn default() -> Self {
        Self {
            auth_type: None,
            user_name_attach_to_target: None,
            user_secret_attach_to_target: None,
            user_name_attach_to_initiator: None,
            user_secret_attach_to_initiator: None,
        }
    }
}

impl Default for structs::VsanIscsiTargetBasicInfo {
    fn default() -> Self {
        Self {
            alias: String::new(),
            iqn: None,
        }
    }
}

impl Default for structs::VsanIscsiTargetCommonInfo {
    fn default() -> Self {
        Self {
            vsan_iscsi_target_basic_info_: structs::VsanIscsiTargetBasicInfo::default(),
            auth_spec: None,
            port: None,
            network_interface: None,
            affinity_location: None,
        }
    }
}

impl Default for structs::VsanIscsiTarget {
    fn default() -> Self {
        Self {
            vsan_iscsi_target_common_info_: structs::VsanIscsiTargetCommonInfo::default(),
            lun_count: None,
            object_information: None,
            io_owner_host: None,
            initiators: None,
            initiator_groups: None,
        }
    }
}

impl Default for structs::VsanIscsiTargetSpec {
    fn default() -> Self {
        Self {
            vsan_iscsi_target_common_info_: structs::VsanIscsiTargetCommonInfo::default(),
            storage_policy: None,
            new_alias: None,
        }
    }
}

impl Default for structs::VsanIscsiTargetServiceConfig {
    fn default() -> Self {
        Self {
            default_config: None,
            enabled: None,
            vip_configs: None,
        }
    }
}

impl Default for structs::VsanIscsiTargetServiceSpec {
    fn default() -> Self {
        Self {
            vsan_iscsi_target_service_config_: structs::VsanIscsiTargetServiceConfig::default(),
            home_object_storage_policy: None,
        }
    }
}

impl Default for structs::VsanIscsiTargetServiceDefaultConfigSpec {
    fn default() -> Self {
        Self {
            network_interface: None,
            port: None,
            iscsi_target_auth_spec: None,
        }
    }
}

impl Default for structs::VsanNetworkDiagnostics {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            event_type_id: String::new(),
            severity: String::new(),
            created_time: String::new(),
            arguments: None,
        }
    }
}

impl Default for structs::VsanObjIdentityQuerySpec {
    fn default() -> Self {
        Self {
            known_spbm_profile_uuids: None,
        }
    }
}

impl Default for structs::VsanClusterObjectExtAttrs {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            object_type: None,
            object_path: None,
            group_uuid: None,
            directory_name: None,
        }
    }
}

impl Default for structs::VsanObjectExtraAttributes {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            obj_path: String::new(),
            obj_class: 0,
            ufn: String::new(),
            is_hbr_cfg: false,
            owner_cluster_uuid: None,
        }
    }
}

impl Default for structs::VsanObjectIdentity {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            r#type: String::new(),
            vm_instance_uuid: None,
            vm_ns_object_uuid: None,
            vm: None,
            description: None,
            spbm_profile_uuid: None,
            metadatas: None,
            type_ext_id: None,
            spbm_profile_name: None,
        }
    }
}

impl Default for structs::VsanObjectIdentityAndHealth {
    fn default() -> Self {
        Self {
            identities: None,
            health: None,
            space_summary: None,
            raw_data: None,
        }
    }
}

impl Default for structs::VsanObjectInformation {
    fn default() -> Self {
        Self {
            directory_name: None,
            vsan_object_uuid: None,
            vsan_health: None,
            policy_attributes: None,
            spbm_profile_uuid: None,
            spbm_profile_generation_id: None,
            spbm_compliance_result: None,
        }
    }
}

impl Default for structs::VsanObjectQuerySpec {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            spbm_profile_generation_id: None,
        }
    }
}

impl Default for structs::VsanObjectSpaceSummary {
    fn default() -> Self {
        Self {
            obj_type: None,
            overhead_b: None,
            temporary_overhead_b: None,
            primary_capacity_b: None,
            provision_capacity_b: None,
            reserved_capacity_b: None,
            over_reserved_b: None,
            physical_used_b: None,
            used_b: None,
            obj_type_ext: None,
            obj_type_ext_desc: None,
            snapshot_used_b: None,
        }
    }
}

impl Default for structs::VsanPerfDiagnoseQuerySpec {
    fn default() -> Self {
        Self {
            start_time: String::new(),
            end_time: String::new(),
            query_type: String::new(),
            context: None,
        }
    }
}

impl Default for structs::VsanPerfDiagnosticException {
    fn default() -> Self {
        Self {
            exception_id: String::new(),
            exception_message: String::new(),
            exception_details: String::new(),
            exception_url: String::new(),
        }
    }
}

impl Default for structs::VsanPerfDiagnosticResult {
    fn default() -> Self {
        Self {
            exception_id: String::new(),
            recommendation: None,
            aggregation_function: None,
            aggregation_data: None,
            exception_data: Vec::new(),
        }
    }
}

impl Default for structs::VsanPerfEntityMetricCsv {
    fn default() -> Self {
        Self {
            entity_ref_id: String::new(),
            sample_info: None,
            value: None,
        }
    }
}

impl Default for structs::VsanPerfEntityType {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            graphs: Vec::new(),
            description: None,
            advanced_graphs: None,
            verbose_graphs: None,
        }
    }
}

impl Default for structs::VsanPerfGraph {
    fn default() -> Self {
        Self {
            id: String::new(),
            metrics: Vec::new(),
            unit: String::new(),
            threshold: None,
            name: None,
            description: None,
            second_graph: None,
        }
    }
}

impl Default for structs::VsanPerfHotspotEntitiesMetrics {
    fn default() -> Self {
        Self {
            entity_ref_id: String::new(),
            start_time: String::new(),
            end_time: String::new(),
            metrics_value: None,
        }
    }
}

impl Default for structs::VsanPerfHotspotQuerySpec {
    fn default() -> Self {
        Self {
            start_time: String::new(),
            end_time: String::new(),
            entity: String::new(),
            metric_id: String::new(),
            num_entities: None,
        }
    }
}

impl Default for structs::VsanPerfMasterInformation {
    fn default() -> Self {
        Self {
            sec_since_last_stats_write: None,
            sec_since_last_stats_collect: None,
            stats_interval_sec: 0,
            collection_failure_host_uuids: None,
            renamed_stats_directories: None,
            stats_directory_percent_free: None,
            verbose_mode: None,
            verbose_mode_last_update: None,
        }
    }
}

impl Default for structs::VsanPerfMemberInfo {
    fn default() -> Self {
        Self {
            thumbprint: String::new(),
            thumbprint_list: None,
            member_uuid: None,
            is_support_unicast: None,
            unicast_address_infos: None,
            hostname: None,
        }
    }
}

impl Default for structs::VsanPerfMetricId {
    fn default() -> Self {
        Self {
            label: String::new(),
            group: None,
            rollup_type: None,
            stats_type: None,
            name: None,
            description: None,
            metrics_collect_interval: None,
        }
    }
}

impl Default for structs::VsanPerfMetricSeriesCsv {
    fn default() -> Self {
        Self {
            metric_id: structs::VsanPerfMetricId::default(),
            threshold: None,
            num_exceptions: None,
            values: None,
        }
    }
}

impl Default for structs::VsanPerfNodeInformation {
    fn default() -> Self {
        Self {
            version: String::new(),
            hostname: None,
            error: None,
            is_cmmds_master: false,
            is_stats_master: false,
            vsan_master_uuid: None,
            vsan_node_uuid: None,
            master_info: None,
            diagnostic_mode: None,
        }
    }
}

impl Default for structs::VsanPerfQuerySpec {
    fn default() -> Self {
        Self {
            entity_ref_id: String::new(),
            start_time: None,
            end_time: None,
            group: None,
            labels: None,
            interval: None,
        }
    }
}

impl Default for structs::VsanPerfThreshold {
    fn default() -> Self {
        Self {
            direction: String::new(),
            yellow: None,
            red: None,
        }
    }
}

impl Default for structs::VsanPerfTimeRange {
    fn default() -> Self {
        Self {
            name: String::new(),
            start_time: String::new(),
            end_time: String::new(),
        }
    }
}

impl Default for structs::VsanPerfTimeRangeQuerySpec {
    fn default() -> Self {
        Self {
            name: None,
            start_time_from: None,
            start_time_to: None,
            end_time_from: None,
            end_time_to: None,
        }
    }
}

impl Default for structs::VsanPerfTopEntities {
    fn default() -> Self {
        Self {
            metric_id: structs::VsanPerfMetricId::default(),
            entities: Vec::new(),
        }
    }
}

impl Default for structs::VsanPerfTopEntity {
    fn default() -> Self {
        Self {
            entity_ref_id: String::new(),
            value: String::new(),
        }
    }
}

impl Default for structs::VsanPerfTopQuerySpec {
    fn default() -> Self {
        Self {
            time_stamp: String::new(),
            entity: String::new(),
            metric_id: String::new(),
            num_entities: None,
        }
    }
}

impl Default for structs::VsanPerfsvcConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            profile: None,
            diagnostic_mode: None,
            verbose_mode: None,
        }
    }
}

impl Default for structs::VsanRemoteClusterQuerySpec {
    fn default() -> Self {
        Self {
            start_time: None,
            end_time: None,
        }
    }
}

impl Default for structs::VsanSpaceQuerySpec {
    fn default() -> Self {
        Self {
            entity_type: String::new(),
            entity_ids: None,
        }
    }
}

impl Default for structs::VsanSpaceUsage {
    fn default() -> Self {
        Self {
            total_capacity_b: 0,
            free_capacity_b: None,
            space_overview: None,
            space_detail: None,
            efficient_capacity: None,
            whatif_capacities: None,
            uncommitted_b: None,
            capacity_health_threshold: None,
            space_efficiency_ratio: None,
        }
    }
}

impl Default for structs::VsanSpaceUsageDetailResult {
    fn default() -> Self {
        Self {
            space_usage_by_object_type: None,
        }
    }
}

impl Default for structs::VsanSpaceUsageWithDatastoreType {
    fn default() -> Self {
        Self {
            space_usage: None,
            datastore_type: None,
        }
    }
}

impl Default for structs::VsanStorageWorkloadType {
    fn default() -> Self {
        Self {
            specs: Vec::new(),
            type_id: String::new(),
            name: String::new(),
            description: String::new(),
            duration: None,
        }
    }
}

impl Default for structs::VsanStretchedClusterConfig {
    fn default() -> Self {
        Self {
            cluster: structs::ManagedObjectReference::default(),
            preferred_fd_name: None,
            fault_domain_config: None,
        }
    }
}

impl Default for structs::VsanSyncingObjectFilter {
    fn default() -> Self {
        Self {
            resync_type: None,
            resync_status: None,
            number_of_objects: None,
            offset: None,
            include_dedup_object: None,
        }
    }
}

impl Default for structs::VsanUnicastAddressInfo {
    fn default() -> Self {
        Self {
            address: String::new(),
            port: None,
            nic_type: None,
        }
    }
}

impl Default for structs::VsanVcKmipServersHealth {
    fn default() -> Self {
        Self {
            health: None,
            error: None,
            kms_provider_id: None,
            kms_health: None,
            client_cert_health: None,
            client_cert_expire_date: None,
            is_aws_kms: None,
            cmk_health: None,
            kek_expire_health: None,
            kek_expire_date: None,
            host_key_expire_health: None,
            host_key_expire_date: None,
        }
    }
}

impl Default for structs::VsanVcLifecycleCheckResult {
    fn default() -> Self {
        Self {
            status: String::new(),
            pre_check_results: None,
            config_details: structs::LifecycleConfigDetails::default(),
        }
    }
}

impl Default for structs::VsanVcLifecycleCheckSpec {
    fn default() -> Self {
        Self {
            operation: String::new(),
        }
    }
}

impl Default for structs::VsanVsanClusterPcapGroup {
    fn default() -> Self {
        Self {
            master: String::new(),
            members: None,
        }
    }
}

impl Default for structs::VsanVsanClusterPcapResult {
    fn default() -> Self {
        Self {
            pkts: None,
            groups: None,
            issues: None,
            host_results: None,
        }
    }
}

impl Default for structs::VsanVumSystemConfig {
    fn default() -> Self {
        Self {
            enabled: None,
            auto_check_interval: None,
            metadata_update_interval: None,
            release_db_last_update: None,
        }
    }
}

impl Default for structs::VsanWhatifCapacity {
    fn default() -> Self {
        Self {
            total_whatif_capacity_b: 0,
            free_whatif_capacity_b: 0,
            storage_policy: Default::default(),
            is_satisfiable: false,
        }
    }
}

impl Default for structs::VimClusterVsanWitnessSpec {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            preferred_fault_domain_name: String::new(),
            disk_mapping: None,
            storage_pool_spec: None,
        }
    }
}

impl Default for structs::CnsAccessControlSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CnsNfsAccessControlSpec {
    fn default() -> Self {
        Self {
            net_permission: structs::VsanFileShareNetPermission::default(),
            delete: None,
        }
    }
}

impl Default for structs::CnsBackingObjectDetails {
    fn default() -> Self {
        Self {
            capacity_in_mb: None,
        }
    }
}

impl Default for structs::CnsBlockBackingDetails {
    fn default() -> Self {
        Self {
            cns_backing_object_details_: structs::CnsBackingObjectDetails::default(),
            backing_disk_id: None,
            backing_disk_url_path: None,
            backing_disk_path: None,
            backing_disk_object_id: None,
            used_capacity_in_mb: None,
            aggregated_snapshot_capacity_in_mb: None,
        }
    }
}

impl Default for structs::CnsFileBackingDetails {
    fn default() -> Self {
        Self {
            cns_backing_object_details_: structs::CnsBackingObjectDetails::default(),
            backing_file_id: None,
        }
    }
}

impl Default for structs::CnsVsanFileShareBackingDetails {
    fn default() -> Self {
        Self {
            cns_file_backing_details_: structs::CnsFileBackingDetails::default(),
            name: None,
            access_points: None,
            permission: None,
        }
    }
}

impl Default for structs::CnsBaseCreateSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CnsFileCreateSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CnsVsanFileCreateSpec {
    fn default() -> Self {
        Self {
            soft_quota_in_mb: None,
            permission: None,
        }
    }
}

impl Default for structs::CnsContainerCluster {
    fn default() -> Self {
        Self {
            cluster_type: String::new(),
            cluster_id: String::new(),
            v_sphere_user: String::new(),
            cluster_flavor: None,
            cluster_distribution: None,
            delete: None,
        }
    }
}

impl Default for structs::CnsCursor {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 0,
            total_records: None,
        }
    }
}

impl Default for structs::CnsEntityMetadata {
    fn default() -> Self {
        Self {
            entity_name: String::new(),
            labels: None,
            delete: None,
            cluster_id: None,
        }
    }
}

impl Default for structs::CnsKubernetesEntityMetadata {
    fn default() -> Self {
        Self {
            cns_entity_metadata_: structs::CnsEntityMetadata::default(),
            entity_type: String::new(),
            namespace: None,
            referred_entity: None,
        }
    }
}

impl Default for structs::CnsKubernetesEntityReference {
    fn default() -> Self {
        Self {
            entity_type: String::new(),
            entity_name: String::new(),
            namespace: None,
            cluster_id: None,
        }
    }
}

impl Default for structs::CnsPlacementResult {
    fn default() -> Self {
        Self {
            datastore: structs::ManagedObjectReference::default(),
            placement_faults: None,
        }
    }
}

impl Default for structs::CnsQueryFilter {
    fn default() -> Self {
        Self {
            volume_ids: None,
            names: None,
            container_cluster_ids: None,
            storage_policy_id: None,
            datastores: None,
            labels: None,
            compliance_status: None,
            datastore_accessibility_status: None,
            cursor: None,
            health_status: None,
        }
    }
}

impl Default for structs::CnsKubernetesQueryFilter {
    fn default() -> Self {
        Self {
            cns_query_filter_: structs::CnsQueryFilter::default(),
            namespaces: None,
            pod_names: None,
            pvc_names: None,
            pv_names: None,
        }
    }
}

impl Default for structs::CnsQueryResult {
    fn default() -> Self {
        Self {
            volumes: None,
            cursor: structs::CnsCursor::default(),
        }
    }
}

impl Default for structs::CnsQuerySelection {
    fn default() -> Self {
        Self {
            names: None,
        }
    }
}

impl Default for structs::CnsSnapshotCreateSpec {
    fn default() -> Self {
        Self {
            volume_id: structs::CnsVolumeId::default(),
            description: String::new(),
        }
    }
}

impl Default for structs::CnsSnapshotDeleteSpec {
    fn default() -> Self {
        Self {
            volume_id: structs::CnsVolumeId::default(),
            snapshot_id: structs::CnsSnapshotId::default(),
        }
    }
}

impl Default for structs::CnsSnapshotId {
    fn default() -> Self {
        Self {
            id: String::new(),
        }
    }
}

impl Default for structs::CnsVolume {
    fn default() -> Self {
        Self {
            volume_id: structs::CnsVolumeId::default(),
            datastore_url: None,
            name: None,
            volume_type: None,
            storage_policy_id: None,
            metadata: None,
            backing_object_details: None,
            compliance_status: None,
            datastore_accessibility_status: None,
            health_status: None,
        }
    }
}

impl Default for structs::CnsVolumeAclConfigureSpec {
    fn default() -> Self {
        Self {
            volume_id: structs::CnsVolumeId::default(),
            access_control_spec_list: Vec::new(),
        }
    }
}

impl Default for structs::CnsVolumeAttachDetachSpec {
    fn default() -> Self {
        Self {
            volume_id: structs::CnsVolumeId::default(),
            vm: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::CnsVolumeCreateSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            volume_type: String::new(),
            datastores: None,
            metadata: None,
            backing_object_details: Default::default(),
            profile: None,
            create_spec: None,
            volume_source: None,
        }
    }
}

impl Default for structs::CnsVolumeExtendSpec {
    fn default() -> Self {
        Self {
            volume_id: structs::CnsVolumeId::default(),
            capacity_in_mb: 0,
        }
    }
}

impl Default for structs::CnsVolumeId {
    fn default() -> Self {
        Self {
            id: String::new(),
        }
    }
}

impl Default for structs::CnsVolumeMetadata {
    fn default() -> Self {
        Self {
            container_cluster: structs::CnsContainerCluster::default(),
            entity_metadata: None,
            container_cluster_array: None,
        }
    }
}

impl Default for structs::CnsVolumeMetadataUpdateSpec {
    fn default() -> Self {
        Self {
            volume_id: structs::CnsVolumeId::default(),
            metadata: structs::CnsVolumeMetadata::default(),
        }
    }
}

impl Default for structs::CnsVolumeOperationBatchResult {
    fn default() -> Self {
        Self {
            volume_results: None,
        }
    }
}

impl Default for structs::CnsVolumeOperationResult {
    fn default() -> Self {
        Self {
            volume_id: None,
            fault: None,
        }
    }
}

impl Default for structs::CnsAsyncQueryResult {
    fn default() -> Self {
        Self {
            cns_volume_operation_result_: structs::CnsVolumeOperationResult::default(),
            query_result: None,
        }
    }
}

impl Default for structs::CnsVolumeAttachResult {
    fn default() -> Self {
        Self {
            cns_volume_operation_result_: structs::CnsVolumeOperationResult::default(),
            disk_uuid: None,
        }
    }
}

impl Default for structs::CnsVolumeCreateResult {
    fn default() -> Self {
        Self {
            cns_volume_operation_result_: structs::CnsVolumeOperationResult::default(),
            name: None,
            placement_results: None,
        }
    }
}

impl Default for structs::CnsVolumePolicyReconfigSpec {
    fn default() -> Self {
        Self {
            volume_id: structs::CnsVolumeId::default(),
            profile: None,
        }
    }
}

impl Default for structs::CnsVolumeRelocateSpec {
    fn default() -> Self {
        Self {
            volume_id: structs::CnsVolumeId::default(),
            datastore: structs::ManagedObjectReference::default(),
            profile: None,
        }
    }
}

impl Default for structs::CnsBlockVolumeRelocateSpec {
    fn default() -> Self {
        Self {
            cns_volume_relocate_spec_: structs::CnsVolumeRelocateSpec::default(),
        }
    }
}

impl Default for structs::CnsVolumeSource {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CnsSnapshotVolumeSource {
    fn default() -> Self {
        Self {
            volume_id: None,
            snapshot_id: None,
        }
    }
}

impl Default for structs::DistributedVirtualPort {
    fn default() -> Self {
        Self {
            key: String::new(),
            config: structs::DvPortConfigInfo::default(),
            dvs_uuid: String::new(),
            portgroup_key: None,
            proxy_host: None,
            connectee: None,
            conflict: false,
            conflict_port_key: None,
            state: None,
            connection_cookie: None,
            last_status_change: String::new(),
            host_local_port: None,
            external_id: None,
            segment_port_id: None,
        }
    }
}

impl Default for structs::DvPortConfigInfo {
    fn default() -> Self {
        Self {
            name: None,
            scope: None,
            description: None,
            setting: None,
            config_version: String::new(),
        }
    }
}

impl Default for structs::DvPortConfigSpec {
    fn default() -> Self {
        Self {
            operation: String::new(),
            key: None,
            name: None,
            scope: None,
            description: None,
            setting: None,
            config_version: None,
        }
    }
}

impl Default for structs::DvsFilterParameter {
    fn default() -> Self {
        Self {
            parameters: None,
        }
    }
}

impl Default for structs::DvsHostLocalPortInfo {
    fn default() -> Self {
        Self {
            switch_uuid: String::new(),
            port_key: String::new(),
            setting: Default::default(),
            vnic: String::new(),
        }
    }
}

impl Default for structs::DvPortStatus {
    fn default() -> Self {
        Self {
            link_up: false,
            blocked: false,
            vlan_ids: None,
            trunking_mode: None,
            mtu: None,
            link_peer: None,
            mac_address: None,
            status_detail: None,
            vm_direct_path_gen_2_active: None,
            vm_direct_path_gen_2_inactive_reason_network: None,
            vm_direct_path_gen_2_inactive_reason_other: None,
            vm_direct_path_gen_2_inactive_reason_extended: None,
        }
    }
}

impl Default for structs::DvPortSetting {
    fn default() -> Self {
        Self {
            blocked: None,
            vm_direct_path_gen_2_allowed: None,
            in_shaping_policy: None,
            out_shaping_policy: None,
            vendor_specific_config: None,
            network_resource_pool_key: None,
            filter_policy: None,
        }
    }
}

impl Default for structs::VMwareDvsPortSetting {
    fn default() -> Self {
        Self {
            dv_port_setting_: structs::DvPortSetting::default(),
            vlan: None,
            qos_tag: None,
            uplink_teaming_policy: None,
            security_policy: None,
            ipfix_enabled: None,
            tx_uplink: None,
            lacp_policy: None,
            mac_management_policy: None,
            vni: None,
        }
    }
}

impl Default for structs::DvPortState {
    fn default() -> Self {
        Self {
            runtime_info: None,
            stats: structs::DistributedVirtualSwitchPortStatistics::default(),
            vendor_specific_state: None,
        }
    }
}

impl Default for structs::DvPortgroupConfigInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: String::new(),
            num_ports: 0,
            distributed_virtual_switch: None,
            default_port_config: None,
            description: None,
            r#type: String::new(),
            backing_type: None,
            policy: Default::default(),
            port_name_format: None,
            scope: None,
            vendor_specific_config: None,
            config_version: None,
            auto_expand: None,
            vm_vnic_network_resource_pool_key: None,
            uplink: None,
            transport_zone_uuid: None,
            transport_zone_name: None,
            logical_switch_uuid: None,
            segment_id: None,
            subnet_id: None,
        }
    }
}

impl Default for structs::DvPortgroupConfigSpec {
    fn default() -> Self {
        Self {
            dynamic_property: None,
            config_version: None,
            name: None,
            num_ports: None,
            port_name_format: None,
            default_port_config: None,
            description: None,
            r#type: None,
            backing_type: None,
            scope: None,
            policy: None,
            vendor_specific_config: None,
            auto_expand: None,
            vm_vnic_network_resource_pool_key: None,
            transport_zone_uuid: None,
            transport_zone_name: None,
            logical_switch_uuid: None,
            segment_id: None,
            subnet_id: None,
        }
    }
}

impl Default for structs::DistributedVirtualPortgroupNsxPortgroupOperationResult {
    fn default() -> Self {
        Self {
            portgroups: None,
            problems: None,
        }
    }
}

impl Default for structs::DvPortgroupPolicy {
    fn default() -> Self {
        Self {
            block_override_allowed: false,
            shaping_override_allowed: false,
            vendor_config_override_allowed: false,
            live_port_moving_allowed: false,
            port_config_reset_at_disconnect: false,
            network_resource_pool_override_allowed: None,
            traffic_filter_override_allowed: None,
        }
    }
}

impl Default for structs::VMwareDvsPortgroupPolicy {
    fn default() -> Self {
        Self {
            dv_portgroup_policy_: structs::DvPortgroupPolicy::default(),
            vlan_override_allowed: false,
            uplink_teaming_override_allowed: false,
            security_policy_override_allowed: false,
            ipfix_override_allowed: None,
            mac_management_override_allowed: None,
        }
    }
}

impl Default for structs::DistributedVirtualPortgroupProblem {
    fn default() -> Self {
        Self {
            logical_switch_uuid: String::new(),
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::DistributedVirtualPortgroupInfo {
    fn default() -> Self {
        Self {
            switch_name: String::new(),
            switch_uuid: String::new(),
            portgroup_name: String::new(),
            portgroup_key: String::new(),
            portgroup_type: String::new(),
            uplink_portgroup: false,
            portgroup: structs::ManagedObjectReference::default(),
            network_reservation_supported: None,
            backing_type: None,
            logical_switch_uuid: None,
            segment_id: None,
            subnet_id: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchInfo {
    fn default() -> Self {
        Self {
            switch_name: String::new(),
            switch_uuid: String::new(),
            distributed_virtual_switch: structs::ManagedObjectReference::default(),
            network_reservation_supported: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchManagerCompatibilityResult {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            error: None,
        }
    }
}

impl Default for structs::DvsManagerDvsConfigTarget {
    fn default() -> Self {
        Self {
            distributed_virtual_portgroup: None,
            distributed_virtual_switch: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchManagerDvsProductSpec {
    fn default() -> Self {
        Self {
            new_switch_product_spec: None,
            distributed_virtual_switch: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchManagerHostContainer {
    fn default() -> Self {
        Self {
            container: structs::ManagedObjectReference::default(),
            recursive: false,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchManagerHostDvsFilterSpec {
    fn default() -> Self {
        Self {
            inclusive: false,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchManagerHostArrayFilter {
    fn default() -> Self {
        Self {
            distributed_virtual_switch_manager_host_dvs_filter_spec_: structs::DistributedVirtualSwitchManagerHostDvsFilterSpec::default(),
            host: Vec::new(),
        }
    }
}

impl Default for structs::DistributedVirtualSwitchManagerHostContainerFilter {
    fn default() -> Self {
        Self {
            distributed_virtual_switch_manager_host_dvs_filter_spec_: structs::DistributedVirtualSwitchManagerHostDvsFilterSpec::default(),
            host_container: structs::DistributedVirtualSwitchManagerHostContainer::default(),
        }
    }
}

impl Default for structs::DistributedVirtualSwitchManagerHostDvsMembershipFilter {
    fn default() -> Self {
        Self {
            distributed_virtual_switch_manager_host_dvs_filter_spec_: structs::DistributedVirtualSwitchManagerHostDvsFilterSpec::default(),
            distributed_virtual_switch: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::DistributedVirtualSwitchManagerImportResult {
    fn default() -> Self {
        Self {
            distributed_virtual_switch: None,
            distributed_virtual_portgroup: None,
            import_fault: None,
        }
    }
}

impl Default for structs::DvsManagerPhysicalNicsList {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            physical_nics: None,
        }
    }
}

impl Default for structs::EntityBackup {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::EntityBackupConfig {
    fn default() -> Self {
        Self {
            entity_type: String::new(),
            config_blob: Vec::new(),
            key: None,
            name: None,
            container: None,
            config_version: None,
        }
    }
}

impl Default for structs::DvsFilterSpecConnecteeSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DvsFilterSpecPnicConnecteeSpec {
    fn default() -> Self {
        Self {
            pnic_name_spec: None,
        }
    }
}

impl Default for structs::DvsFilterSpecVmConnecteeSpec {
    fn default() -> Self {
        Self {
            vm_name_spec: None,
        }
    }
}

impl Default for structs::DvsFilterSpecVmknicConnecteeSpec {
    fn default() -> Self {
        Self {
            vmknic_name_spec: None,
        }
    }
}

impl Default for structs::DvsFilterSpecVlanSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DvsFilterSpecPvlanSpec {
    fn default() -> Self {
        Self {
            pvlan_id: None,
        }
    }
}

impl Default for structs::DvsFilterSpecTrunkVlanSpec {
    fn default() -> Self {
        Self {
            range: None,
        }
    }
}

impl Default for structs::DvsFilterSpecVlanIdSpec {
    fn default() -> Self {
        Self {
            vlan_id: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchHostMember {
    fn default() -> Self {
        Self {
            dynamic_property: None,
            runtime_state: None,
            config: structs::DistributedVirtualSwitchHostMemberConfigInfo::default(),
            product_info: None,
            uplink_port_key: None,
            status: String::new(),
            status_detail: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchHostMemberBacking {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DistributedVirtualSwitchHostMemberPnicBacking {
    fn default() -> Self {
        Self {
            pnic_spec: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchHostMemberConfigInfo {
    fn default() -> Self {
        Self {
            host: None,
            max_proxy_switch_ports: 0,
            vendor_specific_config: None,
            backing: Default::default(),
            nsx_switch: None,
            ens_enabled: None,
            ens_interrupt_enabled: None,
            transport_zones: None,
            nsxt_used_uplink_names: None,
            network_offloading_enabled: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchHostMemberConfigSpec {
    fn default() -> Self {
        Self {
            dynamic_property: None,
            operation: String::new(),
            host: structs::ManagedObjectReference::default(),
            backing: None,
            max_proxy_switch_ports: None,
            vendor_specific_config: None,
        }
    }
}

impl Default for structs::HostMemberHealthCheckResult {
    fn default() -> Self {
        Self {
            summary: None,
        }
    }
}

impl Default for structs::HostMemberUplinkHealthCheckResult {
    fn default() -> Self {
        Self {
            host_member_health_check_result_: structs::HostMemberHealthCheckResult::default(),
            uplink_port_key: String::new(),
        }
    }
}

impl Default for structs::VMwareDvsMtuHealthCheckResult {
    fn default() -> Self {
        Self {
            host_member_uplink_health_check_result_: structs::HostMemberUplinkHealthCheckResult::default(),
            mtu_mismatch: false,
            vlan_support_switch_mtu: None,
            vlan_not_support_switch_mtu: None,
        }
    }
}

impl Default for structs::VMwareDvsVlanHealthCheckResult {
    fn default() -> Self {
        Self {
            host_member_uplink_health_check_result_: structs::HostMemberUplinkHealthCheckResult::default(),
            trunked_vlan: None,
            untrunked_vlan: None,
        }
    }
}

impl Default for structs::VMwareDvsTeamingHealthCheckResult {
    fn default() -> Self {
        Self {
            host_member_health_check_result_: structs::HostMemberHealthCheckResult::default(),
            teaming_status: String::new(),
        }
    }
}

impl Default for structs::DistributedVirtualSwitchHostMemberHostUplinkState {
    fn default() -> Self {
        Self {
            uplink_name: String::new(),
            state: String::new(),
        }
    }
}

impl Default for structs::DistributedVirtualSwitchHostMemberPnicSpec {
    fn default() -> Self {
        Self {
            pnic_device: String::new(),
            uplink_port_key: None,
            uplink_portgroup_key: None,
            connection_cookie: None,
        }
    }
}

impl Default for structs::HostMemberRuntimeInfo {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            status: None,
            status_detail: None,
            nsxt_status: None,
            nsxt_status_detail: None,
            health_check_result: None,
            host_uplink_state: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchHostMemberRuntimeState {
    fn default() -> Self {
        Self {
            current_max_proxy_switch_ports: 0,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchHostMemberTransportZoneInfo {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            r#type: String::new(),
        }
    }
}

impl Default for structs::DistributedVirtualSwitchHostProductSpec {
    fn default() -> Self {
        Self {
            product_line_id: None,
            version: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchKeyedOpaqueBlob {
    fn default() -> Self {
        Self {
            key: String::new(),
            opaque_data: String::new(),
        }
    }
}

impl Default for structs::DistributedVirtualSwitchNetworkOffloadSpec {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: None,
            types: None,
        }
    }
}

impl Default for structs::DvsNetworkResourcePool {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: None,
            description: None,
            config_version: String::new(),
            allocation_info: structs::DvsNetworkResourcePoolAllocationInfo::default(),
        }
    }
}

impl Default for structs::DvsNetworkResourcePoolAllocationInfo {
    fn default() -> Self {
        Self {
            limit: None,
            shares: None,
            priority_tag: None,
        }
    }
}

impl Default for structs::DvsNetworkResourcePoolConfigSpec {
    fn default() -> Self {
        Self {
            dynamic_property: None,
            key: String::new(),
            config_version: None,
            allocation_info: None,
            name: None,
            description: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchPortConnectee {
    fn default() -> Self {
        Self {
            connected_entity: None,
            nic_key: None,
            r#type: None,
            address_hint: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchPortConnection {
    fn default() -> Self {
        Self {
            switch_uuid: String::new(),
            portgroup_key: None,
            port_key: None,
            connection_cookie: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchPortCriteria {
    fn default() -> Self {
        Self {
            dynamic_property: None,
            connected: None,
            active: None,
            uplink_port: None,
            nsx_port: None,
            scope: None,
            portgroup_key: None,
            inside: None,
            port_key: None,
            host: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchPortStatistics {
    fn default() -> Self {
        Self {
            packets_in_multicast: 0,
            packets_out_multicast: 0,
            bytes_in_multicast: 0,
            bytes_out_multicast: 0,
            packets_in_unicast: 0,
            packets_out_unicast: 0,
            bytes_in_unicast: 0,
            bytes_out_unicast: 0,
            packets_in_broadcast: 0,
            packets_out_broadcast: 0,
            bytes_in_broadcast: 0,
            bytes_out_broadcast: 0,
            packets_in_dropped: 0,
            packets_out_dropped: 0,
            packets_in_exception: 0,
            packets_out_exception: 0,
            bytes_in_from_pnic: None,
            bytes_out_to_pnic: None,
        }
    }
}

impl Default for structs::DistributedVirtualSwitchProductSpec {
    fn default() -> Self {
        Self {
            name: None,
            vendor: None,
            version: None,
            build: None,
            forwarding_class: None,
            bundle_id: None,
            bundle_url: None,
        }
    }
}

impl Default for structs::DvsTrafficRule {
    fn default() -> Self {
        Self {
            key: None,
            description: None,
            sequence: None,
            qualifier: None,
            action: None,
            direction: None,
        }
    }
}

impl Default for structs::DvsNetworkRuleAction {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DvsAcceptNetworkRuleAction {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DvsCopyNetworkRuleAction {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DvsDropNetworkRuleAction {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DvsGreEncapNetworkRuleAction {
    fn default() -> Self {
        Self {
            encapsulation_ip: structs::SingleIp::default(),
        }
    }
}

impl Default for structs::DvsLogNetworkRuleAction {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DvsMacRewriteNetworkRuleAction {
    fn default() -> Self {
        Self {
            rewrite_mac: String::new(),
        }
    }
}

impl Default for structs::DvsPuntNetworkRuleAction {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::DvsRateLimitNetworkRuleAction {
    fn default() -> Self {
        Self {
            packets_per_second: 0,
        }
    }
}

impl Default for structs::DvsUpdateTagNetworkRuleAction {
    fn default() -> Self {
        Self {
            qos_tag: None,
            dscp_tag: None,
        }
    }
}

impl Default for structs::DvsNetworkRuleQualifier {
    fn default() -> Self {
        Self {
            key: None,
        }
    }
}

impl Default for structs::DvsIpNetworkRuleQualifier {
    fn default() -> Self {
        Self {
            dvs_network_rule_qualifier_: structs::DvsNetworkRuleQualifier::default(),
            source_address: None,
            destination_address: None,
            protocol: None,
            source_ip_port: None,
            destination_ip_port: None,
            tcp_flags: None,
        }
    }
}

impl Default for structs::DvsMacNetworkRuleQualifier {
    fn default() -> Self {
        Self {
            dvs_network_rule_qualifier_: structs::DvsNetworkRuleQualifier::default(),
            source_address: None,
            destination_address: None,
            protocol: None,
            vlan_id: None,
        }
    }
}

impl Default for structs::DvsSystemTrafficNetworkRuleQualifier {
    fn default() -> Self {
        Self {
            dvs_network_rule_qualifier_: structs::DvsNetworkRuleQualifier::default(),
            type_of_system_traffic: None,
        }
    }
}

impl Default for structs::DvsTrafficRuleset {
    fn default() -> Self {
        Self {
            key: None,
            enabled: None,
            precedence: None,
            rules: None,
        }
    }
}

impl Default for structs::DvsVmVnicNetworkResourcePool {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: None,
            description: None,
            config_version: String::new(),
            allocation_info: None,
        }
    }
}

impl Default for structs::DvsVmVnicResourcePoolConfigSpec {
    fn default() -> Self {
        Self {
            operation: String::new(),
            key: None,
            config_version: None,
            allocation_info: None,
            name: None,
            description: None,
        }
    }
}

impl Default for structs::DvsVmVnicResourceAllocation {
    fn default() -> Self {
        Self {
            reservation_quota: None,
        }
    }
}

impl Default for structs::DvsVmVnicNetworkResourcePoolRuntimeInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: None,
            capacity: None,
            usage: None,
            available: None,
            status: String::new(),
            allocated_resource: None,
        }
    }
}

impl Default for structs::DvsVnicAllocatedResource {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            vnic_key: String::new(),
            reservation: None,
        }
    }
}

impl Default for structs::VmwareDistributedVirtualSwitchDpuFailoverPolicy {
    fn default() -> Self {
        Self {
            active_uplink: None,
            standby_uplink: None,
        }
    }
}

impl Default for structs::VMwareDvsDpuCapability {
    fn default() -> Self {
        Self {
            network_offload_supported: None,
            active_standby_mode_supported: None,
        }
    }
}

impl Default for structs::VMwareIpfixConfig {
    fn default() -> Self {
        Self {
            collector_ip_address: None,
            collector_port: None,
            observation_domain_id: None,
            active_flow_timeout: 0,
            idle_flow_timeout: None,
            sampling_rate: 0,
            internal_flows_only: false,
        }
    }
}

impl Default for structs::VMwareDvsIpfixCapability {
    fn default() -> Self {
        Self {
            ipfix_supported: None,
            ipv_6_for_ipfix_supported: None,
            observation_domain_id_supported: None,
        }
    }
}

impl Default for structs::VMwareDvsLacpCapability {
    fn default() -> Self {
        Self {
            lacp_supported: None,
            multi_lacp_group_supported: None,
            lacp_fast_mode_supported: None,
        }
    }
}

impl Default for structs::VMwareDvsLacpGroupConfig {
    fn default() -> Self {
        Self {
            key: None,
            name: None,
            mode: None,
            uplink_num: None,
            loadbalance_algorithm: None,
            vlan: None,
            ipfix: None,
            uplink_name: None,
            uplink_port_key: None,
            timeout_mode: None,
        }
    }
}

impl Default for structs::VMwareDvsLacpGroupSpec {
    fn default() -> Self {
        Self {
            lacp_group_config: structs::VMwareDvsLacpGroupConfig::default(),
            operation: String::new(),
        }
    }
}

impl Default for structs::VMwareDvsLagIpfixConfig {
    fn default() -> Self {
        Self {
            ipfix_enabled: None,
        }
    }
}

impl Default for structs::VMwareDvsLagVlanConfig {
    fn default() -> Self {
        Self {
            vlan_id: None,
        }
    }
}

impl Default for structs::VMwareDvsMtuCapability {
    fn default() -> Self {
        Self {
            min_mtu_supported: 0,
            max_mtu_supported: 0,
        }
    }
}

impl Default for structs::VmwareDistributedVirtualSwitchNetworkOffloadConfig {
    fn default() -> Self {
        Self {
            dpu_failover_policy: None,
        }
    }
}

impl Default for structs::VMwareDvsPvlanConfigSpec {
    fn default() -> Self {
        Self {
            pvlan_entry: structs::VMwareDvsPvlanMapEntry::default(),
            operation: String::new(),
        }
    }
}

impl Default for structs::VMwareDvsPvlanMapEntry {
    fn default() -> Self {
        Self {
            primary_vlan_id: 0,
            secondary_vlan_id: 0,
            pvlan_type: String::new(),
        }
    }
}

impl Default for structs::VmwareDistributedVirtualSwitchRealTimeConfig {
    fn default() -> Self {
        Self {
            allowed: None,
            lan_annotation: None,
        }
    }
}

impl Default for structs::VmwareDistributedVirtualSwitchRealTimeLanAnnotation {
    fn default() -> Self {
        Self {
            lan_a_uplink: None,
            lan_b_uplink: None,
        }
    }
}

impl Default for structs::VMwareDvsVspanConfigSpec {
    fn default() -> Self {
        Self {
            vspan_session: structs::VMwareVspanSession::default(),
            operation: String::new(),
        }
    }
}

impl Default for structs::VMwareDvsVspanCapability {
    fn default() -> Self {
        Self {
            mixed_dest_supported: false,
            dvport_supported: false,
            remote_source_supported: false,
            remote_dest_supported: false,
            encap_remote_source_supported: false,
            erspan_protocol_supported: None,
            mirror_netstack_supported: None,
        }
    }
}

impl Default for structs::VMwareVspanPort {
    fn default() -> Self {
        Self {
            port_key: None,
            uplink_port_name: None,
            wildcard_port_connectee_type: None,
            vlans: None,
            ip_address: None,
        }
    }
}

impl Default for structs::VMwareVspanSession {
    fn default() -> Self {
        Self {
            key: None,
            name: None,
            description: None,
            enabled: false,
            source_port_transmitted: None,
            source_port_received: None,
            destination_port: None,
            encapsulation_vlan_id: None,
            strip_original_vlan: false,
            mirrored_packet_length: None,
            normal_traffic_allowed: false,
            session_type: None,
            sampling_rate: None,
            encap_type: None,
            erspan_id: None,
            erspan_cos: None,
            erspan_gra_nanosec: None,
            netstack: None,
        }
    }
}

impl Default for structs::CryptoKeyId {
    fn default() -> Self {
        Self {
            key_id: String::new(),
            provider_id: None,
        }
    }
}

impl Default for structs::CryptoKeyPlain {
    fn default() -> Self {
        Self {
            key_id: structs::CryptoKeyId::default(),
            algorithm: String::new(),
            key_data: String::new(),
        }
    }
}

impl Default for structs::CryptoKeyResult {
    fn default() -> Self {
        Self {
            key_id: structs::CryptoKeyId::default(),
            success: false,
            reason: None,
            fault: None,
        }
    }
}

impl Default for structs::CryptoManagerHostKeyStatus {
    fn default() -> Self {
        Self {
            key_id: structs::CryptoKeyId::default(),
            present: false,
            management_type: None,
            access_granted: None,
        }
    }
}

impl Default for structs::CryptoManagerKmipCertSignRequest {
    fn default() -> Self {
        Self {
            common_name: None,
            organization: None,
            organization_unit: None,
            locality: None,
            state: None,
            country: None,
            email: None,
        }
    }
}

impl Default for structs::CryptoManagerKmipCertificateInfo {
    fn default() -> Self {
        Self {
            subject: String::new(),
            issuer: String::new(),
            serial_number: String::new(),
            not_before: String::new(),
            not_after: String::new(),
            fingerprint: String::new(),
            check_time: String::new(),
            seconds_since_valid: None,
            seconds_before_expire: None,
        }
    }
}

impl Default for structs::CryptoManagerKmipClusterStatus {
    fn default() -> Self {
        Self {
            cluster_id: structs::KeyProviderId::default(),
            overall_status: None,
            management_type: None,
            servers: Vec::new(),
            client_cert_info: None,
        }
    }
}

impl Default for structs::CryptoManagerKmipCryptoKeyStatus {
    fn default() -> Self {
        Self {
            key_id: structs::CryptoKeyId::default(),
            key_available: None,
            reason: None,
            key_info: None,
            encrypted_v_ms: None,
            affected_hosts: None,
            referenced_by_tags: None,
        }
    }
}

impl Default for structs::CryptoManagerKmipCryptoKeyStatusKeyInfo {
    fn default() -> Self {
        Self {
            key_id: String::new(),
        }
    }
}

impl Default for structs::CryptoManagerKmipCryptoKeyStatusWrappingKeyIdKeyInfo {
    fn default() -> Self {
        Self {
            crypto_manager_kmip_crypto_key_status_key_info_: structs::CryptoManagerKmipCryptoKeyStatusKeyInfo::default(),
            configured_time: None,
        }
    }
}

impl Default for structs::CryptoManagerKmipCryptoKeyStatusWrappingRotationIntervalKeyInfo {
    fn default() -> Self {
        Self {
            crypto_manager_kmip_crypto_key_status_key_info_: structs::CryptoManagerKmipCryptoKeyStatusKeyInfo::default(),
            create_time: None,
            rotate_time: None,
        }
    }
}

impl Default for structs::CryptoManagerKmipCustomAttributeSpec {
    fn default() -> Self {
        Self {
            attributes: None,
        }
    }
}

impl Default for structs::CryptoManagerKmipGenerateKeySpec {
    fn default() -> Self {
        Self {
            key_type: None,
        }
    }
}

impl Default for structs::CryptoManagerKmipServerCertInfo {
    fn default() -> Self {
        Self {
            certificate: String::new(),
            cert_info: None,
            client_trust_server: None,
        }
    }
}

impl Default for structs::CryptoManagerKmipServerStatus {
    fn default() -> Self {
        Self {
            name: String::new(),
            status: enums::ManagedEntityStatusEnum::default(),
            connection_status: String::new(),
            cert_info: None,
            client_trust_server: None,
            server_trust_client: None,
        }
    }
}

impl Default for structs::CryptoSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CryptoSpecDecrypt {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CryptoSpecDeepRecrypt {
    fn default() -> Self {
        Self {
            new_key_id: structs::CryptoKeyId::default(),
        }
    }
}

impl Default for structs::CryptoSpecEncrypt {
    fn default() -> Self {
        Self {
            crypto_key_id: structs::CryptoKeyId::default(),
        }
    }
}

impl Default for structs::CryptoSpecNoOp {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CryptoSpecRegister {
    fn default() -> Self {
        Self {
            crypto_key_id: structs::CryptoKeyId::default(),
        }
    }
}

impl Default for structs::CryptoSpecShallowRecrypt {
    fn default() -> Self {
        Self {
            new_key_id: structs::CryptoKeyId::default(),
        }
    }
}

impl Default for structs::KeyProviderId {
    fn default() -> Self {
        Self {
            id: String::new(),
        }
    }
}

impl Default for structs::KmipClusterInfo {
    fn default() -> Self {
        Self {
            cluster_id: structs::KeyProviderId::default(),
            servers: None,
            use_as_default: false,
            management_type: None,
            use_as_entity_default: None,
            has_backup: None,
            tpm_required: None,
            key_id: None,
            default_key_type: None,
            key_info: None,
        }
    }
}

impl Default for structs::KmipClusterInfoKeyInfo {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::KmipClusterInfoWrappingKeyIdKeyInfo {
    fn default() -> Self {
        Self {
            key_id: String::new(),
            configured_time: String::new(),
        }
    }
}

impl Default for structs::KmipClusterInfoWrappingRotationIntervalKeyInfo {
    fn default() -> Self {
        Self {
            key_id: None,
            rotation_interval: None,
            last_rotation: None,
        }
    }
}

impl Default for structs::KmipServerInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            address: String::new(),
            port: 0,
            proxy_address: None,
            proxy_port: None,
            reconnect: None,
            protocol: None,
            nbio: None,
            timeout: None,
            user_name: None,
        }
    }
}

impl Default for structs::KmipServerSpec {
    fn default() -> Self {
        Self {
            cluster_id: structs::KeyProviderId::default(),
            info: structs::KmipServerInfo::default(),
            password: None,
            default_key_type: None,
            key_spec: None,
        }
    }
}

impl Default for structs::KmipServerSpecKeySpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::KmipServerSpecWrappingKeyIdKeySpec {
    fn default() -> Self {
        Self {
            key_id: String::new(),
        }
    }
}

impl Default for structs::KmipServerSpecWrappingRotationIntervalKeySpec {
    fn default() -> Self {
        Self {
            rotation_interval: None,
        }
    }
}

impl Default for structs::KmipServerStatus {
    fn default() -> Self {
        Self {
            cluster_id: structs::KeyProviderId::default(),
            name: String::new(),
            status: enums::ManagedEntityStatusEnum::default(),
            description: String::new(),
        }
    }
}

impl Default for structs::ChangesInfoEventArgument {
    fn default() -> Self {
        Self {
            modified: None,
            added: None,
            deleted: None,
        }
    }
}

impl Default for structs::DvsOutOfSyncHostArgument {
    fn default() -> Self {
        Self {
            out_of_sync_host: structs::HostEventArgument::default(),
            config_paramters: Vec::new(),
        }
    }
}

impl Default for structs::Event {
    fn default() -> Self {
        Self {
            key: 0,
            chain_id: 0,
            created_time: String::new(),
            user_name: String::new(),
            datacenter: None,
            compute_resource: None,
            host: None,
            vm: None,
            ds: None,
            net: None,
            dvs: None,
            full_formatted_message: None,
            change_tag: None,
            type_: None,
            extra_fields_: std::collections::HashMap::new(),
        }
    }
}

impl Default for structs::EventArgument {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::EntityEventArgument {
    fn default() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Default for structs::AlarmEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            alarm: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::ComputeResourceEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            compute_resource: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::DatacenterEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            datacenter: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::DatastoreEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            datastore: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::DvsEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            dvs: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::FolderEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            folder: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::HostEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            host: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::ManagedEntityEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            entity: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::NetworkEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            network: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::ResourcePoolEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            resource_pool: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::ScheduledTaskEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            scheduled_task: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VmEventArgument {
    fn default() -> Self {
        Self {
            entity_event_argument_: structs::EntityEventArgument::default(),
            vm: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::ProfileEventArgument {
    fn default() -> Self {
        Self {
            profile: structs::ManagedObjectReference::default(),
            name: String::new(),
        }
    }
}

impl Default for structs::RoleEventArgument {
    fn default() -> Self {
        Self {
            role_id: 0,
            name: String::new(),
        }
    }
}

impl Default for structs::EventDescription {
    fn default() -> Self {
        Self {
            category: Vec::new(),
            event_info: Vec::new(),
            enumerated_types: None,
        }
    }
}

impl Default for structs::EventArgDesc {
    fn default() -> Self {
        Self {
            name: String::new(),
            r#type: String::new(),
            description: None,
        }
    }
}

impl Default for structs::EventDescriptionEventDetail {
    fn default() -> Self {
        Self {
            key: String::new(),
            description: None,
            category: String::new(),
            format_on_datacenter: String::new(),
            format_on_compute_resource: String::new(),
            format_on_host: String::new(),
            format_on_vm: String::new(),
            full_format: String::new(),
            long_description: None,
        }
    }
}

impl Default for structs::EventFilterSpec {
    fn default() -> Self {
        Self {
            entity: None,
            time: None,
            user_name: None,
            event_chain_id: None,
            alarm: None,
            scheduled_task: None,
            disable_full_message: None,
            category: None,
            r#type: None,
            tag: None,
            event_type_id: None,
            max_count: None,
            delayed_init: None,
        }
    }
}

impl Default for structs::EventFilterSpecByEntity {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
            recursion: enums::EventFilterSpecRecursionOptionEnum::default(),
        }
    }
}

impl Default for structs::EventFilterSpecByTime {
    fn default() -> Self {
        Self {
            begin_time: None,
            end_time: None,
        }
    }
}

impl Default for structs::EventFilterSpecByUsername {
    fn default() -> Self {
        Self {
            system_user: false,
            user_list: None,
        }
    }
}

impl Default for structs::EventManagerEventViewSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::EventManagerViewByStartId {
    fn default() -> Self {
        Self {
            start_event_id: 0,
            is_forward: false,
        }
    }
}

impl Default for structs::ExtendedEventPair {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: String::new(),
        }
    }
}

impl Default for structs::VnicPortArgument {
    fn default() -> Self {
        Self {
            vnic: String::new(),
            port: structs::DistributedVirtualSwitchPortConnection::default(),
        }
    }
}

impl Default for structs::ExtExtendedProductInfo {
    fn default() -> Self {
        Self {
            company_url: None,
            product_url: None,
            management_url: None,
            self_: None,
        }
    }
}

impl Default for structs::ManagedByInfo {
    fn default() -> Self {
        Self {
            extension_key: String::new(),
            r#type: String::new(),
        }
    }
}

impl Default for structs::ExtManagedEntityInfo {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            small_icon_url: None,
            icon_url: None,
            description: None,
        }
    }
}

impl Default for structs::ExtSolutionManagerInfo {
    fn default() -> Self {
        Self {
            tab: None,
            small_icon_url: None,
        }
    }
}

impl Default for structs::ExtSolutionManagerInfoTabInfo {
    fn default() -> Self {
        Self {
            label: String::new(),
            url: String::new(),
        }
    }
}

impl Default for structs::AnswerFileUpdateFailure {
    fn default() -> Self {
        Self {
            user_input_path: structs::ProfilePropertyPath::default(),
            err_msg: structs::LocalizableMessage::default(),
        }
    }
}

impl Default for structs::ConflictingConfigurationConfig {
    fn default() -> Self {
        Self {
            entity: None,
            property_path: String::new(),
        }
    }
}

impl Default for structs::DatacenterMismatchArgument {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
            input_datacenter: None,
        }
    }
}

impl Default for structs::DvsApplyOperationFaultFaultOnObject {
    fn default() -> Self {
        Self {
            object_id: String::new(),
            r#type: String::new(),
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::DvsOperationBulkFaultFaultOnHost {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::ImportOperationBulkFaultFaultOnImport {
    fn default() -> Self {
        Self {
            entity_type: None,
            key: None,
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::MultipleCertificatesVerifyFaultThumbprintData {
    fn default() -> Self {
        Self {
            port: 0,
            thumbprint: String::new(),
        }
    }
}

impl Default for structs::NoPermissionEntityPrivileges {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
            privilege_ids: None,
        }
    }
}

impl Default for structs::ProfileUpdateFailedUpdateFailure {
    fn default() -> Self {
        Self {
            profile_path: structs::ProfilePropertyPath::default(),
            err_msg: structs::LocalizableMessage::default(),
        }
    }
}

impl Default for structs::HostActiveDirectory {
    fn default() -> Self {
        Self {
            change_operation: String::new(),
            spec: None,
        }
    }
}

impl Default for structs::HostActiveDirectorySpec {
    fn default() -> Self {
        Self {
            domain_name: None,
            user_name: None,
            password: None,
            cam_server: None,
            thumbprint: None,
            certificate: None,
            smart_card_authentication_enabled: None,
            smart_card_trust_anchors: None,
        }
    }
}

impl Default for structs::HostAssignableHardwareBinding {
    fn default() -> Self {
        Self {
            instance_id: String::new(),
            vm: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::HostAssignableHardwareConfig {
    fn default() -> Self {
        Self {
            attribute_override: None,
        }
    }
}

impl Default for structs::HostAssignableHardwareConfigAttributeOverride {
    fn default() -> Self {
        Self {
            instance_id: String::new(),
            name: String::new(),
            value: None,
        }
    }
}

impl Default for structs::HostAuthenticationInfo {
    fn default() -> Self {
        Self {
            principal: String::new(),
            owner_tag: String::new(),
            ssl_certificates: None,
        }
    }
}

impl Default for structs::HostAuthenticationManagerInfo {
    fn default() -> Self {
        Self {
            auth_config: Vec::new(),
        }
    }
}

impl Default for structs::HostAuthenticationStoreInfo {
    fn default() -> Self {
        Self {
            enabled: false,
        }
    }
}

impl Default for structs::HostDirectoryStoreInfo {
    fn default() -> Self {
        Self {
            host_authentication_store_info_: structs::HostAuthenticationStoreInfo::default(),
        }
    }
}

impl Default for structs::HostActiveDirectoryInfo {
    fn default() -> Self {
        Self {
            host_directory_store_info_: structs::HostDirectoryStoreInfo::default(),
            joined_domain: None,
            trusted_domain: None,
            domain_membership_status: None,
            smart_card_authentication_enabled: None,
        }
    }
}

impl Default for structs::HostLocalAuthenticationInfo {
    fn default() -> Self {
        Self {
            host_authentication_store_info_: structs::HostAuthenticationStoreInfo::default(),
        }
    }
}

impl Default for structs::AutoStartPowerInfo {
    fn default() -> Self {
        Self {
            key: structs::ManagedObjectReference::default(),
            start_order: 0,
            start_delay: 0,
            wait_for_heartbeat: enums::AutoStartWaitHeartbeatSettingEnum::default(),
            start_action: String::new(),
            stop_delay: 0,
            stop_action: String::new(),
        }
    }
}

impl Default for structs::HostAutoStartManagerConfig {
    fn default() -> Self {
        Self {
            defaults: None,
            power_info: None,
        }
    }
}

impl Default for structs::AutoStartDefaults {
    fn default() -> Self {
        Self {
            enabled: None,
            start_delay: None,
            stop_delay: None,
            wait_for_heartbeat: None,
            stop_action: None,
        }
    }
}

impl Default for structs::HostBiosInfo {
    fn default() -> Self {
        Self {
            bios_version: None,
            release_date: None,
            vendor: None,
            major_release: None,
            minor_release: None,
            firmware_major_release: None,
            firmware_minor_release: None,
            firmware_type: None,
        }
    }
}

impl Default for structs::HostBootDeviceInfo {
    fn default() -> Self {
        Self {
            boot_devices: None,
            current_boot_device_key: None,
        }
    }
}

impl Default for structs::HostBootDevice {
    fn default() -> Self {
        Self {
            key: String::new(),
            description: String::new(),
        }
    }
}

impl Default for structs::HostCacheConfigurationInfo {
    fn default() -> Self {
        Self {
            key: structs::ManagedObjectReference::default(),
            swap_size: 0,
        }
    }
}

impl Default for structs::HostCacheConfigurationSpec {
    fn default() -> Self {
        Self {
            datastore: structs::ManagedObjectReference::default(),
            swap_size: 0,
        }
    }
}

impl Default for structs::HostCapability {
    fn default() -> Self {
        Self {
            recursive_resource_pools_supported: false,
            cpu_memory_resource_configuration_supported: false,
            reboot_supported: false,
            shutdown_supported: false,
            vmotion_supported: false,
            standby_supported: false,
            ipmi_supported: None,
            max_supported_v_ms: None,
            max_running_v_ms: None,
            max_supported_vcpus: None,
            max_registered_v_ms: None,
            datastore_principal_supported: false,
            san_supported: false,
            nfs_supported: false,
            iscsi_supported: false,
            vlan_tagging_supported: false,
            nic_teaming_supported: false,
            high_guest_mem_supported: false,
            maintenance_mode_supported: false,
            suspended_relocate_supported: false,
            restricted_snapshot_relocate_supported: false,
            per_vm_swap_files: false,
            local_swap_datastore_supported: false,
            unshared_swap_v_motion_supported: false,
            background_snapshots_supported: false,
            pre_assigned_pci_unit_numbers_supported: false,
            screenshot_supported: false,
            scaled_screenshot_supported: false,
            storage_v_motion_supported: false,
            vmotion_with_storage_v_motion_supported: false,
            vmotion_across_network_supported: None,
            max_num_disks_sv_motion: None,
            max_virtual_disk_desc_version_supported: None,
            hbr_nic_selection_supported: false,
            vr_nfc_nic_selection_supported: false,
            record_replay_supported: false,
            ft_supported: false,
            replay_unsupported_reason: None,
            replay_compatibility_issues: None,
            smp_ft_supported: false,
            ft_compatibility_issues: None,
            smp_ft_compatibility_issues: None,
            max_vcpus_per_ft_vm: None,
            login_by_ssl_thumbprint_supported: None,
            clone_from_snapshot_supported: false,
            delta_disk_backings_supported: false,
            per_vm_network_traffic_shaping_supported: false,
            tpm_supported: false,
            tpm_version: None,
            txt_enabled: None,
            supported_cpu_feature: None,
            virtual_exec_usage_supported: false,
            storage_iorm_supported: false,
            vm_direct_path_gen_2_supported: None,
            vm_direct_path_gen_2_unsupported_reason: None,
            vm_direct_path_gen_2_unsupported_reason_extended: None,
            supported_vmfs_major_version: None,
            v_storage_capable: false,
            snapshot_relayout_supported: false,
            firewall_ip_rules_supported: None,
            service_package_info_supported: None,
            max_host_running_vms: None,
            max_host_supported_vcpus: None,
            vmfs_datastore_mount_capable: false,
            eight_plus_host_vmfs_shared_access_supported: false,
            nested_hv_supported: false,
            v_pmc_supported: false,
            inter_vm_communication_through_vmci_supported: false,
            scheduled_hardware_upgrade_supported: None,
            feature_capabilities_supported: false,
            latency_sensitivity_supported: false,
            storage_policy_supported: None,
            accel_3_d_supported: false,
            reliable_memory_aware: None,
            multiple_network_stack_instance_supported: None,
            message_bus_proxy_supported: None,
            vsan_supported: None,
            v_flash_supported: None,
            host_access_manager_supported: None,
            provisioning_nic_selection_supported: false,
            nfs_41_supported: None,
            nfs_41_krb_5_i_supported: None,
            turn_disk_locator_led_supported: None,
            virtual_volume_datastore_supported: None,
            mark_as_ssd_supported: None,
            mark_as_local_supported: None,
            smart_card_authentication_supported: None,
            p_mem_supported: None,
            p_mem_snapshot_supported: None,
            crypto_supported: None,
            one_k_volume_ap_is_supported: None,
            gateway_on_nic_supported: None,
            upit_supported: None,
            cpu_hw_mmu_supported: None,
            encrypted_v_motion_supported: None,
            encryption_change_on_add_remove_supported: None,
            encryption_hot_operation_supported: None,
            encryption_with_snapshots_supported: None,
            encryption_fault_tolerance_supported: None,
            encryption_memory_save_supported: None,
            encryption_rdm_supported: None,
            encryption_v_flash_supported: None,
            encryption_cbrc_supported: None,
            encryption_hbr_supported: None,
            ft_efi_supported: None,
            unmap_method_supported: None,
            max_mem_mb_per_ft_vm: None,
            virtual_mmu_usage_ignored: None,
            virtual_exec_usage_ignored: None,
            vm_create_date_supported: None,
            vmfs_3_eol_supported: None,
            ft_vmcp_supported: None,
            quick_boot_supported: None,
            encrypted_ft_supported: None,
            assignable_hardware_supported: None,
            suspend_to_memory_supported: None,
            use_feature_reqs_for_old_h_wv: None,
            mark_perennially_reserved_supported: None,
            hpp_psp_supported: None,
            device_rebind_without_reboot_supported: None,
            storage_policy_change_supported: None,
            precision_time_protocol_supported: None,
            remote_device_v_motion_supported: None,
            max_supported_vm_memory: None,
            ah_device_hints_supported: None,
            nvme_over_tcp_supported: None,
            nvme_storage_fabric_services_supported: None,
            assign_hw_pci_config_supported: None,
            time_config_supported: None,
            nvme_batch_operations_supported: None,
            p_mem_failover_supported: None,
            host_config_encryption_supported: None,
            max_supported_simultaneous_threads: None,
            ptp_config_supported: None,
            max_supported_ptp_ports: None,
            sgx_registration_supported: None,
            p_mem_independent_snapshot_supported: None,
            iommu_sl_dirty_capable: None,
            vmknic_binding_supported: None,
            ultralow_fixed_unmap_supported: None,
            nvme_vvol_supported: None,
            fpt_hotplug_supported: None,
            mconnect_supported: None,
            vsan_nic_mgmt_supported: None,
            vvol_nqn_supported: None,
            stretched_sc_supported: None,
            vmknic_binding_on_nf_sv_41: None,
            vp_status_check_supported: None,
            e_2_e_4_kn_supported: None,
            vsan_dedicated_vmk_nic_supported: None,
            n_connect_supported: None,
            user_key_supported: None,
            ndcm_supported: None,
            uefi_secure_boot: None,
            vpxd_vmx_generation_supported: None,
            nfs_41_krb_5_p_supported: None,
            cim_supported: None,
            npiv_supported: None,
            entitlement_supported: None,
        }
    }
}

impl Default for structs::HostCertificateManagerCertificateInfo {
    fn default() -> Self {
        Self {
            kind: None,
            issuer: None,
            not_before: None,
            not_after: None,
            subject: None,
            status: String::new(),
        }
    }
}

impl Default for structs::HostCertificateManagerCertificateSpec {
    fn default() -> Self {
        Self {
            kind: String::new(),
            subject_alternative_names: None,
        }
    }
}

impl Default for structs::HostConfigChange {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostConfigInfo {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            product: structs::AboutInfo::default(),
            deployment_info: None,
            hyper_thread: None,
            cpu_scheduler: None,
            console_reservation: None,
            virtual_machine_reservation: None,
            storage_device: None,
            multipath_state: None,
            file_system_volume: None,
            system_file: None,
            network: None,
            vmotion: None,
            virtual_nic_manager_info: None,
            capabilities: None,
            datastore_capabilities: None,
            offload_capabilities: None,
            service: None,
            firewall: None,
            auto_start: None,
            active_diagnostic_partition: None,
            option: None,
            option_def: None,
            datastore_principal: None,
            local_swap_datastore: None,
            system_swap_configuration: None,
            system_resources: None,
            date_time_info: None,
            flags: None,
            admin_disabled: None,
            lockdown_mode: None,
            ipmi: None,
            ssl_thumbprint_info: None,
            ssl_thumbprint_data: None,
            authentication_data: None,
            certificate: None,
            pci_passthru_info: None,
            authentication_manager_info: None,
            feature_version: None,
            power_system_capability: None,
            power_system_info: None,
            cache_configuration_info: None,
            wake_on_lan_capable: None,
            feature_capability: None,
            masked_feature_capability: None,
            v_flash_config_info: None,
            vsan_host_config: None,
            domain_list: None,
            script_check_sum: None,
            host_config_check_sum: None,
            description_tree_check_sum: None,
            graphics_info: None,
            shared_passthru_gpu_types: None,
            graphics_config: None,
            shared_gpu_capabilities: None,
            io_filter_info: None,
            sriov_device_pool: None,
            assignable_hardware_binding: None,
            assignable_hardware_config: None,
        }
    }
}

impl Default for structs::HostConfigManager {
    fn default() -> Self {
        Self {
            cpu_scheduler: None,
            datastore_system: None,
            memory_manager: None,
            storage_system: None,
            network_system: None,
            vmotion_system: None,
            virtual_nic_manager: None,
            service_system: None,
            firewall_system: None,
            advanced_option: None,
            diagnostic_system: None,
            auto_start_manager: None,
            snmp_system: None,
            date_time_system: None,
            patch_manager: None,
            image_config_manager: None,
            boot_device_system: None,
            firmware_system: None,
            health_status_system: None,
            pci_passthru_system: None,
            license_manager: None,
            kernel_module_system: None,
            authentication_manager: None,
            power_system: None,
            cache_configuration_manager: None,
            esx_agent_host_manager: None,
            iscsi_manager: None,
            v_flash_manager: None,
            vsan_system: None,
            message_bus_proxy: None,
            user_directory: None,
            account_manager: None,
            host_access_manager: None,
            graphics_manager: None,
            vsan_internal_system: None,
            certificate_manager: None,
            crypto_manager: None,
            nvdimm_system: None,
            assignable_hardware_manager: None,
        }
    }
}

impl Default for structs::HostConfigSpec {
    fn default() -> Self {
        Self {
            nas_datastore: None,
            network: None,
            nic_type_selection: None,
            service: None,
            firewall: None,
            option: None,
            datastore_principal: None,
            datastore_principal_passwd: None,
            datetime: None,
            storage_device: None,
            license: None,
            security: None,
            user_account: None,
            usergroup_account: None,
            memory: None,
            active_directory: None,
            generic_config: None,
            graphics_config: None,
            assignable_hardware_config: None,
        }
    }
}

impl Default for structs::HostConnectInfo {
    fn default() -> Self {
        Self {
            server_ip: None,
            in_das_cluster: None,
            host: structs::HostListSummary::default(),
            vm: None,
            vim_account_name_required: None,
            cluster_supported: None,
            network: None,
            datastore: None,
            license: None,
            capability: None,
        }
    }
}

impl Default for structs::HostDatastoreConnectInfo {
    fn default() -> Self {
        Self {
            summary: structs::DatastoreSummary::default(),
        }
    }
}

impl Default for structs::HostDatastoreExistsConnectInfo {
    fn default() -> Self {
        Self {
            host_datastore_connect_info_: structs::HostDatastoreConnectInfo::default(),
            new_datastore_name: String::new(),
        }
    }
}

impl Default for structs::HostDatastoreNameConflictConnectInfo {
    fn default() -> Self {
        Self {
            host_datastore_connect_info_: structs::HostDatastoreConnectInfo::default(),
            new_datastore_name: String::new(),
        }
    }
}

impl Default for structs::HostLicenseConnectInfo {
    fn default() -> Self {
        Self {
            license: structs::LicenseManagerLicenseInfo::default(),
            evaluation: structs::LicenseManagerEvaluationInfo::default(),
            resource: None,
        }
    }
}

impl Default for structs::HostConnectInfoNetworkInfo {
    fn default() -> Self {
        Self {
            summary: Default::default(),
        }
    }
}

impl Default for structs::HostNewNetworkConnectInfo {
    fn default() -> Self {
        Self {
            host_connect_info_network_info_: structs::HostConnectInfoNetworkInfo::default(),
        }
    }
}

impl Default for structs::HostConnectSpec {
    fn default() -> Self {
        Self {
            host_name: None,
            port: None,
            ssl_thumbprint: None,
            ssl_certificate: None,
            user_name: None,
            password: None,
            vm_folder: None,
            force: false,
            vim_account_name: None,
            vim_account_password: None,
            management_ip: None,
            lockdown_mode: None,
            host_gateway: None,
        }
    }
}

impl Default for structs::HostCpuIdInfo {
    fn default() -> Self {
        Self {
            level: 0,
            vendor: None,
            eax: None,
            ebx: None,
            ecx: None,
            edx: None,
        }
    }
}

impl Default for structs::HostCpuInfo {
    fn default() -> Self {
        Self {
            num_cpu_packages: 0,
            num_cpu_cores: 0,
            num_cpu_threads: 0,
            hz: 0,
        }
    }
}

impl Default for structs::HostCpuPackage {
    fn default() -> Self {
        Self {
            index: 0,
            vendor: String::new(),
            hz: 0,
            bus_hz: 0,
            description: String::new(),
            thread_id: Vec::new(),
            cpu_feature: None,
            family: None,
            model: None,
            stepping: None,
        }
    }
}

impl Default for structs::HostCpuPowerManagementInfo {
    fn default() -> Self {
        Self {
            current_policy: None,
            hardware_support: None,
        }
    }
}

impl Default for structs::HostCpuSchedulerInfo {
    fn default() -> Self {
        Self {
            policy: String::new(),
        }
    }
}

impl Default for structs::HostHyperThreadScheduleInfo {
    fn default() -> Self {
        Self {
            available: false,
            active: false,
            config: false,
        }
    }
}

impl Default for structs::HostDataTransportConnectionInfo {
    fn default() -> Self {
        Self {
            static_memory_consumed: 0,
        }
    }
}

impl Default for structs::HostNfcConnectionInfo {
    fn default() -> Self {
        Self {
            host_data_transport_connection_info_: structs::HostDataTransportConnectionInfo::default(),
            streaming_memory_consumed: None,
        }
    }
}

impl Default for structs::FileInfo {
    fn default() -> Self {
        Self {
            path: String::new(),
            friendly_name: None,
            file_size: None,
            modification: None,
            owner: None,
        }
    }
}

impl Default for structs::FloppyImageFileInfo {
    fn default() -> Self {
        Self {
            file_info_: structs::FileInfo::default(),
        }
    }
}

impl Default for structs::FolderFileInfo {
    fn default() -> Self {
        Self {
            file_info_: structs::FileInfo::default(),
        }
    }
}

impl Default for structs::IsoImageFileInfo {
    fn default() -> Self {
        Self {
            file_info_: structs::FileInfo::default(),
        }
    }
}

impl Default for structs::VmConfigFileInfo {
    fn default() -> Self {
        Self {
            file_info_: structs::FileInfo::default(),
            config_version: None,
            encryption: None,
        }
    }
}

impl Default for structs::TemplateConfigFileInfo {
    fn default() -> Self {
        Self {
            vm_config_file_info_: structs::VmConfigFileInfo::default(),
        }
    }
}

impl Default for structs::VmDiskFileInfo {
    fn default() -> Self {
        Self {
            file_info_: structs::FileInfo::default(),
            disk_type: None,
            capacity_kb: None,
            hardware_version: None,
            controller_type: None,
            disk_extents: None,
            thin: None,
            encryption: None,
            sector_format: None,
        }
    }
}

impl Default for structs::VmLogFileInfo {
    fn default() -> Self {
        Self {
            file_info_: structs::FileInfo::default(),
        }
    }
}

impl Default for structs::VmNvramFileInfo {
    fn default() -> Self {
        Self {
            file_info_: structs::FileInfo::default(),
        }
    }
}

impl Default for structs::VmSnapshotFileInfo {
    fn default() -> Self {
        Self {
            file_info_: structs::FileInfo::default(),
        }
    }
}

impl Default for structs::FileQueryFlags {
    fn default() -> Self {
        Self {
            file_type: false,
            file_size: false,
            modification: false,
            file_owner: false,
        }
    }
}

impl Default for structs::FileQuery {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::FloppyImageFileQuery {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::FolderFileQuery {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::IsoImageFileQuery {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VmConfigFileQuery {
    fn default() -> Self {
        Self {
            filter: None,
            details: None,
        }
    }
}

impl Default for structs::TemplateConfigFileQuery {
    fn default() -> Self {
        Self {
            vm_config_file_query_: structs::VmConfigFileQuery::default(),
        }
    }
}

impl Default for structs::VmDiskFileQuery {
    fn default() -> Self {
        Self {
            filter: None,
            details: None,
        }
    }
}

impl Default for structs::VmLogFileQuery {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VmNvramFileQuery {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VmSnapshotFileQuery {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostDatastoreBrowserSearchResults {
    fn default() -> Self {
        Self {
            datastore: None,
            folder_path: None,
            file: None,
        }
    }
}

impl Default for structs::HostDatastoreBrowserSearchSpec {
    fn default() -> Self {
        Self {
            query: None,
            details: None,
            search_case_insensitive: None,
            match_pattern: None,
            sort_folders_first: None,
        }
    }
}

impl Default for structs::VmConfigFileEncryptionInfo {
    fn default() -> Self {
        Self {
            key_id: None,
        }
    }
}

impl Default for structs::VmConfigFileQueryFlags {
    fn default() -> Self {
        Self {
            config_version: false,
            encryption: None,
        }
    }
}

impl Default for structs::VmConfigFileQueryFilter {
    fn default() -> Self {
        Self {
            match_config_version: None,
            encrypted: None,
        }
    }
}

impl Default for structs::VmDiskFileEncryptionInfo {
    fn default() -> Self {
        Self {
            key_id: None,
        }
    }
}

impl Default for structs::VmDiskFileQueryFlags {
    fn default() -> Self {
        Self {
            disk_type: false,
            capacity_kb: false,
            hardware_version: false,
            controller_type: None,
            disk_extents: None,
            thin: None,
            encryption: None,
            sector_format: None,
        }
    }
}

impl Default for structs::VmDiskFileQueryFilter {
    fn default() -> Self {
        Self {
            disk_type: None,
            match_hardware_version: None,
            controller_type: None,
            thin: None,
            encrypted: None,
        }
    }
}

impl Default for structs::HostDatastoreSystemCapabilities {
    fn default() -> Self {
        Self {
            nfs_mount_creation_required: false,
            nfs_mount_creation_supported: false,
            local_datastore_supported: false,
            vmfs_extent_expansion_supported: false,
        }
    }
}

impl Default for structs::HostDatastoreSystemDatastoreResult {
    fn default() -> Self {
        Self {
            key: structs::ManagedObjectReference::default(),
            fault: None,
        }
    }
}

impl Default for structs::HostDatastoreSystemVvolDatastoreSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            sc_id: String::new(),
        }
    }
}

impl Default for structs::HostDateTimeConfig {
    fn default() -> Self {
        Self {
            time_zone: None,
            ntp_config: None,
            ptp_config: None,
            protocol: None,
            enabled: None,
            disable_events: None,
            disable_fallback: None,
            reset_to_factory_defaults: None,
        }
    }
}

impl Default for structs::HostDateTimeInfo {
    fn default() -> Self {
        Self {
            time_zone: structs::HostDateTimeSystemTimeZone::default(),
            system_clock_protocol: None,
            ntp_config: None,
            ptp_config: None,
            enabled: None,
            disable_events: None,
            disable_fallback: None,
            in_fallback_state: None,
            service_sync: None,
            last_sync_time: None,
            remote_ntp_server: None,
            ntp_run_time: None,
            ptp_run_time: None,
            ntp_duration: None,
            ptp_duration: None,
        }
    }
}

impl Default for structs::HostDateTimeSystemServiceTestResult {
    fn default() -> Self {
        Self {
            working_normally: false,
            report: None,
        }
    }
}

impl Default for structs::HostDateTimeSystemTimeZone {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: String::new(),
            description: String::new(),
            gmt_offset: 0,
        }
    }
}

impl Default for structs::HostDeploymentInfo {
    fn default() -> Self {
        Self {
            booted_from_stateless_cache: None,
        }
    }
}

impl Default for structs::HostDevice {
    fn default() -> Self {
        Self {
            device_name: String::new(),
            device_type: String::new(),
        }
    }
}

impl Default for structs::ScsiLun {
    fn default() -> Self {
        Self {
            host_device_: structs::HostDevice::default(),
            key: None,
            uuid: String::new(),
            descriptor: None,
            canonical_name: None,
            display_name: None,
            lun_type: String::new(),
            vendor: None,
            model: None,
            revision: None,
            scsi_level: None,
            serial_number: None,
            durable_name: None,
            alternate_name: None,
            standard_inquiry: None,
            queue_depth: None,
            operational_state: Vec::new(),
            capabilities: None,
            v_storage_support: None,
            protocol_endpoint: None,
            perennially_reserved: None,
            clustered_vmdk_supported: None,
            application_protocol: None,
            dispersed_ns: None,
            device_reservation: None,
        }
    }
}

impl Default for structs::HostScsiDisk {
    fn default() -> Self {
        Self {
            scsi_lun_: structs::ScsiLun::default(),
            capacity: structs::HostDiskDimensionsLba::default(),
            device_path: String::new(),
            ssd: None,
            local_disk: None,
            physical_location: None,
            emulated_dixdif_enabled: None,
            vsan_disk_info: None,
            scsi_disk_type: None,
            used_by_memory_tiering: None,
        }
    }
}

impl Default for structs::DevicePciId {
    fn default() -> Self {
        Self {
            vendor_id: 0,
            device_id: 0,
            sub_vendor_id: 0,
            sub_device_id: 0,
        }
    }
}

impl Default for structs::HostDhcpService {
    fn default() -> Self {
        Self {
            key: String::new(),
            spec: structs::HostDhcpServiceSpec::default(),
        }
    }
}

impl Default for structs::HostDhcpServiceConfig {
    fn default() -> Self {
        Self {
            change_operation: None,
            key: String::new(),
            spec: structs::HostDhcpServiceSpec::default(),
        }
    }
}

impl Default for structs::HostDhcpServiceSpec {
    fn default() -> Self {
        Self {
            virtual_switch: String::new(),
            default_lease_duration: 0,
            lease_begin_ip: String::new(),
            lease_end_ip: String::new(),
            max_lease_duration: 0,
            unlimited_lease: false,
            ip_subnet_addr: String::new(),
            ip_subnet_mask: String::new(),
        }
    }
}

impl Default for structs::HostDiagnosticPartition {
    fn default() -> Self {
        Self {
            storage_type: String::new(),
            diagnostic_type: String::new(),
            slots: 0,
            id: structs::HostScsiDiskPartition::default(),
        }
    }
}

impl Default for structs::HostDiagnosticPartitionCreateDescription {
    fn default() -> Self {
        Self {
            layout: structs::HostDiskPartitionLayout::default(),
            disk_uuid: String::new(),
            spec: structs::HostDiagnosticPartitionCreateSpec::default(),
        }
    }
}

impl Default for structs::HostDiagnosticPartitionCreateOption {
    fn default() -> Self {
        Self {
            storage_type: String::new(),
            diagnostic_type: String::new(),
            disk: structs::HostScsiDisk::default(),
        }
    }
}

impl Default for structs::HostDiagnosticPartitionCreateSpec {
    fn default() -> Self {
        Self {
            storage_type: String::new(),
            diagnostic_type: String::new(),
            id: structs::HostScsiDiskPartition::default(),
            partition: structs::HostDiskPartitionSpec::default(),
            active: None,
        }
    }
}

impl Default for structs::HostDigestInfo {
    fn default() -> Self {
        Self {
            digest_method: String::new(),
            digest_value: Vec::new(),
            object_name: None,
        }
    }
}

impl Default for structs::HostTpmDigestInfo {
    fn default() -> Self {
        Self {
            host_digest_info_: structs::HostDigestInfo::default(),
            pcr_number: 0,
        }
    }
}

impl Default for structs::HostDiskConfigurationResult {
    fn default() -> Self {
        Self {
            device_path: None,
            success: None,
            fault: None,
        }
    }
}

impl Default for structs::HostDiskDimensions {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostDiskDimensionsChs {
    fn default() -> Self {
        Self {
            cylinder: 0,
            head: 0,
            sector: 0,
        }
    }
}

impl Default for structs::HostDiskDimensionsLba {
    fn default() -> Self {
        Self {
            block_size: 0,
            block: 0,
        }
    }
}

impl Default for structs::HostDiskPartitionInfo {
    fn default() -> Self {
        Self {
            device_name: String::new(),
            spec: structs::HostDiskPartitionSpec::default(),
            layout: structs::HostDiskPartitionLayout::default(),
        }
    }
}

impl Default for structs::HostDiskPartitionBlockRange {
    fn default() -> Self {
        Self {
            partition: None,
            r#type: String::new(),
            start: structs::HostDiskDimensionsLba::default(),
            end: structs::HostDiskDimensionsLba::default(),
        }
    }
}

impl Default for structs::HostDiskPartitionLayout {
    fn default() -> Self {
        Self {
            total: None,
            partition: Vec::new(),
        }
    }
}

impl Default for structs::HostDiskPartitionAttributes {
    fn default() -> Self {
        Self {
            partition: 0,
            start_sector: 0,
            end_sector: 0,
            r#type: String::new(),
            guid: None,
            logical: false,
            attributes: 0,
            partition_alignment: None,
        }
    }
}

impl Default for structs::HostDiskPartitionSpec {
    fn default() -> Self {
        Self {
            partition_format: None,
            chs: None,
            total_sectors: None,
            partition: None,
            sector_size: None,
        }
    }
}

impl Default for structs::HostDnsConfig {
    fn default() -> Self {
        Self {
            dhcp: false,
            virtual_nic_device: None,
            ipv_6_virtual_nic_device: None,
            host_name: String::new(),
            domain_name: String::new(),
            address: None,
            search_domain: None,
        }
    }
}

impl Default for structs::HostDnsConfigSpec {
    fn default() -> Self {
        Self {
            host_dns_config_: structs::HostDnsConfig::default(),
            virtual_nic_connection: None,
            virtual_nic_connection_v_6: None,
        }
    }
}

impl Default for structs::HostDvxClass {
    fn default() -> Self {
        Self {
            device_class: String::new(),
            checkpoint_supported: false,
            sw_dma_tracing_supported: false,
            sriov_nic: false,
        }
    }
}

impl Default for structs::HostEnterMaintenanceResult {
    fn default() -> Self {
        Self {
            vm_faults: None,
            host_faults: None,
        }
    }
}

impl Default for structs::HostEsxAgentHostManagerConfigInfo {
    fn default() -> Self {
        Self {
            agent_vm_datastore: None,
            agent_vm_network: None,
        }
    }
}

impl Default for structs::HostFaultToleranceManagerComponentHealthInfo {
    fn default() -> Self {
        Self {
            is_storage_healthy: false,
            is_network_healthy: false,
        }
    }
}

impl Default for structs::FcoeConfig {
    fn default() -> Self {
        Self {
            priority_class: 0,
            source_mac: String::new(),
            vlan_range: Vec::new(),
            capabilities: structs::FcoeConfigFcoeCapabilities::default(),
            fcoe_active: false,
        }
    }
}

impl Default for structs::FcoeConfigFcoeCapabilities {
    fn default() -> Self {
        Self {
            priority_class: false,
            source_mac_address: false,
            vlan_range: false,
        }
    }
}

impl Default for structs::FcoeConfigFcoeSpecification {
    fn default() -> Self {
        Self {
            underlying_pnic: String::new(),
            priority_class: None,
            source_mac: None,
            vlan_range: None,
        }
    }
}

impl Default for structs::FcoeConfigVlanRange {
    fn default() -> Self {
        Self {
            vlan_low: 0,
            vlan_high: 0,
        }
    }
}

impl Default for structs::HostFeatureCapability {
    fn default() -> Self {
        Self {
            key: String::new(),
            feature_name: String::new(),
            value: String::new(),
        }
    }
}

impl Default for structs::HostFeatureMask {
    fn default() -> Self {
        Self {
            key: String::new(),
            feature_name: String::new(),
            value: String::new(),
        }
    }
}

impl Default for structs::HostFeatureVersionInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: String::new(),
        }
    }
}

impl Default for structs::HostFibreChannelOverEthernetHbaLinkInfo {
    fn default() -> Self {
        Self {
            vnport_mac: String::new(),
            fcf_mac: String::new(),
            vlan_id: 0,
        }
    }
}

impl Default for structs::HostFileAccess {
    fn default() -> Self {
        Self {
            who: String::new(),
            what: String::new(),
        }
    }
}

impl Default for structs::ModeInfo {
    fn default() -> Self {
        Self {
            browse: None,
            read: String::new(),
            modify: String::new(),
            r#use: String::new(),
            admin: None,
            full: String::new(),
        }
    }
}

impl Default for structs::HostFileSystemMountInfo {
    fn default() -> Self {
        Self {
            mount_info: structs::HostMountInfo::default(),
            volume: Default::default(),
            v_storage_support: None,
        }
    }
}

impl Default for structs::HostFileSystemVolume {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            name: String::new(),
            capacity: 0,
        }
    }
}

impl Default for structs::HostLocalFileSystemVolume {
    fn default() -> Self {
        Self {
            host_file_system_volume_: structs::HostFileSystemVolume::default(),
            device: String::new(),
        }
    }
}

impl Default for structs::HostNasVolume {
    fn default() -> Self {
        Self {
            host_file_system_volume_: structs::HostFileSystemVolume::default(),
            remote_host: String::new(),
            remote_path: String::new(),
            user_name: None,
            remote_host_names: None,
            security_type: None,
            protocol_endpoint: None,
        }
    }
}

impl Default for structs::HostPMemVolume {
    fn default() -> Self {
        Self {
            host_file_system_volume_: structs::HostFileSystemVolume::default(),
            uuid: String::new(),
            version: String::new(),
        }
    }
}

impl Default for structs::HostVfatVolume {
    fn default() -> Self {
        Self {
            host_file_system_volume_: structs::HostFileSystemVolume::default(),
        }
    }
}

impl Default for structs::HostVffsVolume {
    fn default() -> Self {
        Self {
            host_file_system_volume_: structs::HostFileSystemVolume::default(),
            major_version: 0,
            version: String::new(),
            uuid: String::new(),
            extent: Vec::new(),
        }
    }
}

impl Default for structs::HostVmfsVolume {
    fn default() -> Self {
        Self {
            host_file_system_volume_: structs::HostFileSystemVolume::default(),
            block_size_mb: 0,
            block_size: None,
            unmap_granularity: None,
            unmap_priority: None,
            unmap_bandwidth_spec: None,
            max_blocks: 0,
            major_version: 0,
            version: String::new(),
            uuid: String::new(),
            extent: Vec::new(),
            vmfs_upgradable: false,
            force_mounted_info: None,
            ssd: None,
            local: None,
            scsi_disk_type: None,
        }
    }
}

impl Default for structs::HostVvolVolume {
    fn default() -> Self {
        Self {
            host_file_system_volume_: structs::HostFileSystemVolume::default(),
            sc_id: String::new(),
            host_pe: None,
            host_vvol_nqn: None,
            vasa_provider_info: None,
            storage_array: None,
            protocol_endpoint_type: None,
            vvol_nqn_fields_available: None,
            stretched: None,
        }
    }
}

impl Default for structs::HostFileSystemVolumeInfo {
    fn default() -> Self {
        Self {
            volume_type_list: None,
            mount_info: None,
        }
    }
}

impl Default for structs::HostFirewallConfig {
    fn default() -> Self {
        Self {
            rule: None,
            default_blocking_policy: structs::HostFirewallDefaultPolicy::default(),
        }
    }
}

impl Default for structs::HostFirewallConfigRuleSetConfig {
    fn default() -> Self {
        Self {
            ruleset_id: String::new(),
            enabled: false,
            allowed_hosts: None,
        }
    }
}

impl Default for structs::HostFirewallInfo {
    fn default() -> Self {
        Self {
            default_policy: structs::HostFirewallDefaultPolicy::default(),
            ruleset: None,
        }
    }
}

impl Default for structs::HostFirewallDefaultPolicy {
    fn default() -> Self {
        Self {
            incoming_blocked: None,
            outgoing_blocked: None,
        }
    }
}

impl Default for structs::HostFlagInfo {
    fn default() -> Self {
        Self {
            background_snapshots_enabled: None,
        }
    }
}

impl Default for structs::HostForceMountedInfo {
    fn default() -> Self {
        Self {
            persist: false,
            mounted: false,
        }
    }
}

impl Default for structs::HostFru {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            part_name: String::new(),
            part_number: String::new(),
            manufacturer: String::new(),
            serial_number: None,
            mfg_time_stamp: None,
        }
    }
}

impl Default for structs::HostGatewaySpec {
    fn default() -> Self {
        Self {
            gateway_type: String::new(),
            gateway_id: None,
            trust_verification_token: None,
            host_auth_params: None,
        }
    }
}

impl Default for structs::HostGraphicsConfig {
    fn default() -> Self {
        Self {
            host_default_graphics_type: String::new(),
            shared_passthru_assignment_policy: String::new(),
            device_type: None,
        }
    }
}

impl Default for structs::HostGraphicsConfigDeviceType {
    fn default() -> Self {
        Self {
            device_id: String::new(),
            graphics_type: String::new(),
            vgpu_mode: None,
        }
    }
}

impl Default for structs::HostGraphicsInfo {
    fn default() -> Self {
        Self {
            device_name: String::new(),
            vendor_name: String::new(),
            pci_id: String::new(),
            graphics_type: String::new(),
            vgpu_mode: None,
            memory_size_in_kb: 0,
            vm: None,
        }
    }
}

impl Default for structs::HostHardwareInfo {
    fn default() -> Self {
        Self {
            system_info: structs::HostSystemInfo::default(),
            cpu_power_management_info: None,
            cpu_info: structs::HostCpuInfo::default(),
            cpu_pkg: Vec::new(),
            memory_size: 0,
            numa_info: None,
            smc_present: false,
            pci_device: None,
            dvx_classes: None,
            cpu_feature: None,
            bios_info: None,
            reliable_memory_info: None,
            persistent_memory_info: None,
            sgx_info: None,
            sev_info: None,
            memory_tiering_type: None,
            memory_tier_info: None,
            tdx_info: None,
        }
    }
}

impl Default for structs::HostHardwareStatusInfo {
    fn default() -> Self {
        Self {
            memory_status_info: None,
            cpu_status_info: None,
            storage_status_info: None,
            dpu_status_info: None,
        }
    }
}

impl Default for structs::DpuStatusInfoOperationalInfo {
    fn default() -> Self {
        Self {
            sensor_id: String::new(),
            health_state: None,
            reading: String::new(),
            units: None,
            time_stamp: None,
        }
    }
}

impl Default for structs::HostHardwareElementInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            status: Default::default(),
        }
    }
}

impl Default for structs::DpuStatusInfo {
    fn default() -> Self {
        Self {
            host_hardware_element_info_: structs::HostHardwareElementInfo::default(),
            dpu_id: String::new(),
            fru: None,
            sensors: None,
        }
    }
}

impl Default for structs::HostStorageElementInfo {
    fn default() -> Self {
        Self {
            host_hardware_element_info_: structs::HostHardwareElementInfo::default(),
            operational_info: None,
        }
    }
}

impl Default for structs::HostStorageOperationalInfo {
    fn default() -> Self {
        Self {
            property: String::new(),
            value: String::new(),
        }
    }
}

impl Default for structs::HostHbaCreateSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostTcpHbaCreateSpec {
    fn default() -> Self {
        Self {
            pnic: String::new(),
        }
    }
}

impl Default for structs::HealthSystemRuntime {
    fn default() -> Self {
        Self {
            system_health_info: None,
            hardware_status_info: None,
        }
    }
}

impl Default for structs::HostAccessControlEntry {
    fn default() -> Self {
        Self {
            principal: String::new(),
            group: false,
            access_mode: enums::HostAccessModeEnum::default(),
        }
    }
}

impl Default for structs::HostHostBusAdapter {
    fn default() -> Self {
        Self {
            key: None,
            device: String::new(),
            bus: 0,
            status: String::new(),
            model: String::new(),
            driver: None,
            pci: None,
            storage_protocol: None,
        }
    }
}

impl Default for structs::HostBlockHba {
    fn default() -> Self {
        Self {
            host_host_bus_adapter_: structs::HostHostBusAdapter::default(),
        }
    }
}

impl Default for structs::HostFibreChannelHba {
    fn default() -> Self {
        Self {
            host_host_bus_adapter_: structs::HostHostBusAdapter::default(),
            port_world_wide_name: 0,
            node_world_wide_name: 0,
            port_type: enums::FibreChannelPortTypeEnum::default(),
            speed: 0,
        }
    }
}

impl Default for structs::HostFibreChannelOverEthernetHba {
    fn default() -> Self {
        Self {
            host_fibre_channel_hba_: structs::HostFibreChannelHba::default(),
            underlying_nic: String::new(),
            link_info: structs::HostFibreChannelOverEthernetHbaLinkInfo::default(),
            is_software_fcoe: false,
            marked_for_removal: None,
        }
    }
}

impl Default for structs::HostInternetScsiHba {
    fn default() -> Self {
        Self {
            host_host_bus_adapter_: structs::HostHostBusAdapter::default(),
            is_software_based: false,
            can_be_disabled: None,
            network_binding_support: None,
            discovery_capabilities: structs::HostInternetScsiHbaDiscoveryCapabilities::default(),
            discovery_properties: structs::HostInternetScsiHbaDiscoveryProperties::default(),
            authentication_capabilities: structs::HostInternetScsiHbaAuthenticationCapabilities::default(),
            authentication_properties: structs::HostInternetScsiHbaAuthenticationProperties::default(),
            digest_capabilities: None,
            digest_properties: None,
            ip_capabilities: structs::HostInternetScsiHbaIpCapabilities::default(),
            ip_properties: structs::HostInternetScsiHbaIpProperties::default(),
            supported_advanced_options: None,
            advanced_options: None,
            i_scsi_name: String::new(),
            i_scsi_alias: None,
            configured_send_target: None,
            configured_static_target: None,
            max_speed_mb: None,
            current_speed_mb: None,
        }
    }
}

impl Default for structs::HostParallelScsiHba {
    fn default() -> Self {
        Self {
            host_host_bus_adapter_: structs::HostHostBusAdapter::default(),
        }
    }
}

impl Default for structs::HostPcieHba {
    fn default() -> Self {
        Self {
            host_host_bus_adapter_: structs::HostHostBusAdapter::default(),
        }
    }
}

impl Default for structs::HostRdmaHba {
    fn default() -> Self {
        Self {
            host_host_bus_adapter_: structs::HostHostBusAdapter::default(),
            associated_rdma_device: None,
        }
    }
}

impl Default for structs::HostSerialAttachedHba {
    fn default() -> Self {
        Self {
            host_host_bus_adapter_: structs::HostHostBusAdapter::default(),
            node_world_wide_name: String::new(),
        }
    }
}

impl Default for structs::HostTcpHba {
    fn default() -> Self {
        Self {
            host_host_bus_adapter_: structs::HostHostBusAdapter::default(),
            associated_pnic: None,
        }
    }
}

impl Default for structs::HostProxySwitch {
    fn default() -> Self {
        Self {
            dvs_uuid: String::new(),
            dvs_name: String::new(),
            key: String::new(),
            num_ports: 0,
            config_num_ports: None,
            num_ports_available: 0,
            uplink_port: None,
            mtu: None,
            pnic: None,
            spec: structs::HostProxySwitchSpec::default(),
            host_lag: None,
            network_reservation_supported: None,
            nsxt_enabled: None,
            ens_enabled: None,
            ens_interrupt_enabled: None,
            transport_zones: None,
            nsx_used_uplink_port: None,
            nsxt_status: None,
            nsxt_status_detail: None,
            ens_info: None,
            network_offloading_enabled: None,
            host_uplink_state: None,
        }
    }
}

impl Default for structs::HostProxySwitchConfig {
    fn default() -> Self {
        Self {
            change_operation: None,
            uuid: String::new(),
            spec: None,
        }
    }
}

impl Default for structs::HostProxySwitchEnsInfo {
    fn default() -> Self {
        Self {
            ops_version: 0,
            num_ps_ops: 0,
            num_lcore_ops: 0,
            error_status: 0,
            lcore_status: 0,
        }
    }
}

impl Default for structs::HostProxySwitchHostLagConfig {
    fn default() -> Self {
        Self {
            lag_key: String::new(),
            lag_name: None,
            uplink_port: None,
        }
    }
}

impl Default for structs::HostProxySwitchSpec {
    fn default() -> Self {
        Self {
            backing: None,
        }
    }
}

impl Default for structs::HostSpbmDatastoreInfo {
    fn default() -> Self {
        Self {
            datastore_url: String::new(),
            namespace: String::new(),
            default_profile_id: String::new(),
        }
    }
}

impl Default for structs::HostSpbmHashInfo {
    fn default() -> Self {
        Self {
            policy_info_hash: String::new(),
            datastore_info_hash: String::new(),
        }
    }
}

impl Default for structs::HostSpbmPolicyBlobInfo {
    fn default() -> Self {
        Self {
            policy_blob: String::new(),
            namespace: String::new(),
        }
    }
}

impl Default for structs::HostSpbmPolicyInfo {
    fn default() -> Self {
        Self {
            profile_id: String::new(),
            name: String::new(),
            description: None,
            generation_id: 0,
            policy_blob_info: Vec::new(),
        }
    }
}

impl Default for structs::HostImageProfileSummary {
    fn default() -> Self {
        Self {
            name: String::new(),
            vendor: String::new(),
        }
    }
}

impl Default for structs::HostInternetScsiHbaAuthenticationCapabilities {
    fn default() -> Self {
        Self {
            chap_auth_settable: false,
            krb_5_auth_settable: false,
            srp_auth_settable: false,
            spkm_auth_settable: false,
            mutual_chap_settable: None,
            target_chap_settable: None,
            target_mutual_chap_settable: None,
        }
    }
}

impl Default for structs::HostInternetScsiHbaAuthenticationProperties {
    fn default() -> Self {
        Self {
            chap_auth_enabled: false,
            chap_name: None,
            chap_secret: None,
            chap_authentication_type: None,
            chap_inherited: None,
            mutual_chap_name: None,
            mutual_chap_secret: None,
            mutual_chap_authentication_type: None,
            mutual_chap_inherited: None,
        }
    }
}

impl Default for structs::HostInternetScsiHbaDigestCapabilities {
    fn default() -> Self {
        Self {
            header_digest_settable: None,
            data_digest_settable: None,
            target_header_digest_settable: None,
            target_data_digest_settable: None,
        }
    }
}

impl Default for structs::HostInternetScsiHbaDigestProperties {
    fn default() -> Self {
        Self {
            header_digest_type: None,
            header_digest_inherited: None,
            data_digest_type: None,
            data_digest_inherited: None,
        }
    }
}

impl Default for structs::HostInternetScsiHbaDiscoveryCapabilities {
    fn default() -> Self {
        Self {
            i_sns_discovery_settable: false,
            slp_discovery_settable: false,
            static_target_discovery_settable: false,
            send_targets_discovery_settable: false,
        }
    }
}

impl Default for structs::HostInternetScsiHbaDiscoveryProperties {
    fn default() -> Self {
        Self {
            i_sns_discovery_enabled: false,
            i_sns_discovery_method: None,
            i_sns_host: None,
            slp_discovery_enabled: false,
            slp_discovery_method: None,
            slp_host: None,
            static_target_discovery_enabled: false,
            send_targets_discovery_enabled: false,
        }
    }
}

impl Default for structs::HostInternetScsiHbaIpCapabilities {
    fn default() -> Self {
        Self {
            address_settable: false,
            ip_configuration_method_settable: false,
            subnet_mask_settable: false,
            default_gateway_settable: false,
            primary_dns_server_address_settable: false,
            alternate_dns_server_address_settable: false,
            ipv_6_supported: None,
            arp_redirect_settable: None,
            mtu_settable: None,
            host_name_as_target_address: None,
            name_alias_settable: None,
            ipv_4_enable_settable: None,
            ipv_6_enable_settable: None,
            ipv_6_prefix_length_settable: None,
            ipv_6_prefix_length: None,
            ipv_6_dhcp_configuration_settable: None,
            ipv_6_link_local_auto_configuration_settable: None,
            ipv_6_router_advertisement_configuration_settable: None,
            ipv_6_default_gateway_settable: None,
            ipv_6_max_static_addresses_supported: None,
        }
    }
}

impl Default for structs::HostInternetScsiHbaIpProperties {
    fn default() -> Self {
        Self {
            mac: None,
            address: None,
            dhcp_configuration_enabled: false,
            subnet_mask: None,
            default_gateway: None,
            primary_dns_server_address: None,
            alternate_dns_server_address: None,
            ipv_6_address: None,
            ipv_6_subnet_mask: None,
            ipv_6_default_gateway: None,
            arp_redirect_enabled: None,
            mtu: None,
            jumbo_frames_enabled: None,
            ipv_4_enabled: None,
            ipv_6_enabled: None,
            ipv_6_properties: None,
        }
    }
}

impl Default for structs::HostInternetScsiHbaIPv6Properties {
    fn default() -> Self {
        Self {
            iscsi_ipv_6_address: None,
            ipv_6_dhcp_configuration_enabled: None,
            ipv_6_link_local_auto_configuration_enabled: None,
            ipv_6_router_advertisement_configuration_enabled: None,
            ipv_6_default_gateway: None,
        }
    }
}

impl Default for structs::HostInternetScsiHbaIscsiIpv6Address {
    fn default() -> Self {
        Self {
            address: String::new(),
            prefix_length: 0,
            origin: String::new(),
            operation: None,
        }
    }
}

impl Default for structs::HostInternetScsiHbaSendTarget {
    fn default() -> Self {
        Self {
            address: String::new(),
            port: None,
            authentication_properties: None,
            digest_properties: None,
            supported_advanced_options: None,
            advanced_options: None,
            parent: None,
        }
    }
}

impl Default for structs::HostInternetScsiHbaStaticTarget {
    fn default() -> Self {
        Self {
            address: String::new(),
            port: None,
            i_scsi_name: String::new(),
            discovery_method: None,
            authentication_properties: None,
            digest_properties: None,
            supported_advanced_options: None,
            advanced_options: None,
            parent: None,
        }
    }
}

impl Default for structs::HostInternetScsiHbaTargetSet {
    fn default() -> Self {
        Self {
            static_targets: None,
            send_targets: None,
        }
    }
}

impl Default for structs::HostIpConfig {
    fn default() -> Self {
        Self {
            dhcp: false,
            ip_address: None,
            subnet_mask: None,
            ip_v_6_config: None,
        }
    }
}

impl Default for structs::VsanFileServiceIpConfig {
    fn default() -> Self {
        Self {
            host_ip_config_: structs::HostIpConfig::default(),
            fqdn: None,
            is_primary: None,
            gateway: String::new(),
            affinity_location: None,
            ipv_6_gateway: None,
        }
    }
}

impl Default for structs::HostIpConfigIpV6Address {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            prefix_length: 0,
            origin: None,
            dad_state: None,
            lifetime: None,
            operation: None,
        }
    }
}

impl Default for structs::HostIpConfigIpV6AddressConfiguration {
    fn default() -> Self {
        Self {
            ip_v_6_address: None,
            auto_configuration_enabled: None,
            dhcp_v_6_enabled: None,
        }
    }
}

impl Default for structs::HostIpRouteConfig {
    fn default() -> Self {
        Self {
            default_gateway: None,
            gateway_device: None,
            ip_v_6_default_gateway: None,
            ip_v_6_gateway_device: None,
        }
    }
}

impl Default for structs::HostIpRouteConfigSpec {
    fn default() -> Self {
        Self {
            host_ip_route_config_: structs::HostIpRouteConfig::default(),
            gateway_device_connection: None,
            ip_v_6_gateway_device_connection: None,
        }
    }
}

impl Default for structs::HostIpRouteEntry {
    fn default() -> Self {
        Self {
            network: String::new(),
            prefix_length: 0,
            gateway: String::new(),
            device_name: None,
        }
    }
}

impl Default for structs::HostIpRouteOp {
    fn default() -> Self {
        Self {
            change_operation: String::new(),
            route: structs::HostIpRouteEntry::default(),
        }
    }
}

impl Default for structs::HostIpRouteTableConfig {
    fn default() -> Self {
        Self {
            ip_route: None,
            ipv_6_route: None,
        }
    }
}

impl Default for structs::HostIpRouteTableInfo {
    fn default() -> Self {
        Self {
            ip_route: None,
            ipv_6_route: None,
        }
    }
}

impl Default for structs::HostIpmiInfo {
    fn default() -> Self {
        Self {
            bmc_ip_address: None,
            bmc_mac_address: None,
            login: None,
            password: None,
        }
    }
}

impl Default for structs::IscsiDependencyEntity {
    fn default() -> Self {
        Self {
            pnic_device: String::new(),
            vnic_device: String::new(),
            vmhba_name: String::new(),
        }
    }
}

impl Default for structs::IscsiMigrationDependency {
    fn default() -> Self {
        Self {
            migration_allowed: false,
            disallow_reason: None,
            dependency: None,
        }
    }
}

impl Default for structs::IscsiPortInfo {
    fn default() -> Self {
        Self {
            vnic_device: None,
            vnic: None,
            pnic_device: None,
            pnic: None,
            switch_name: None,
            switch_uuid: None,
            portgroup_name: None,
            portgroup_key: None,
            port_key: None,
            opaque_network_id: None,
            opaque_network_type: None,
            opaque_network_name: None,
            external_id: None,
            compliance_status: None,
            path_status: None,
        }
    }
}

impl Default for structs::IscsiStatus {
    fn default() -> Self {
        Self {
            reason: None,
        }
    }
}

impl Default for structs::KernelModuleInfo {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            version: String::new(),
            filename: String::new(),
            option_string: String::new(),
            loaded: false,
            enabled: false,
            use_count: 0,
            read_only_section: structs::KernelModuleSectionInfo::default(),
            writable_section: structs::KernelModuleSectionInfo::default(),
            text_section: structs::KernelModuleSectionInfo::default(),
            data_section: structs::KernelModuleSectionInfo::default(),
            bss_section: structs::KernelModuleSectionInfo::default(),
        }
    }
}

impl Default for structs::KernelModuleSectionInfo {
    fn default() -> Self {
        Self {
            address: 0,
            length: None,
        }
    }
}

impl Default for structs::LacpInfo {
    fn default() -> Self {
        Self {
            dvs_name: String::new(),
            lags: None,
        }
    }
}

impl Default for structs::LagInfo {
    fn default() -> Self {
        Self {
            lag_name: String::new(),
            group_state: 0,
            vnics: None,
            uplinks: None,
        }
    }
}

impl Default for structs::LagUplinkInfo {
    fn default() -> Self {
        Self {
            uplink_name: String::new(),
            port_state: 0,
            bundle_state: String::new(),
        }
    }
}

impl Default for structs::HostLicenseSpec {
    fn default() -> Self {
        Self {
            source: None,
            edition_key: None,
            disabled_feature_key: None,
            enabled_feature_key: None,
        }
    }
}

impl Default for structs::LinkDiscoveryProtocolConfig {
    fn default() -> Self {
        Self {
            protocol: String::new(),
            operation: String::new(),
        }
    }
}

impl Default for structs::HostAccountSpec {
    fn default() -> Self {
        Self {
            id: String::new(),
            password: None,
            description: None,
        }
    }
}

impl Default for structs::HostPosixAccountSpec {
    fn default() -> Self {
        Self {
            host_account_spec_: structs::HostAccountSpec::default(),
            posix_id: None,
            shell_access: None,
        }
    }
}

impl Default for structs::HostLocalFileSystemVolumeSpec {
    fn default() -> Self {
        Self {
            device: String::new(),
            local_path: String::new(),
        }
    }
}

impl Default for structs::HostLowLevelProvisioningManagerDiskLayoutSpec {
    fn default() -> Self {
        Self {
            controller_type: String::new(),
            bus_number: 0,
            unit_number: 0,
            src_filename: String::new(),
            dst_filename: String::new(),
        }
    }
}

impl Default for structs::HostLowLevelProvisioningManagerFileDeleteResult {
    fn default() -> Self {
        Self {
            file_name: String::new(),
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::HostLowLevelProvisioningManagerFileDeleteSpec {
    fn default() -> Self {
        Self {
            file_name: String::new(),
            file_type: String::new(),
        }
    }
}

impl Default for structs::HostLowLevelProvisioningManagerFileReserveResult {
    fn default() -> Self {
        Self {
            base_name: String::new(),
            parent_dir: String::new(),
            reserved_name: String::new(),
        }
    }
}

impl Default for structs::HostLowLevelProvisioningManagerFileReserveSpec {
    fn default() -> Self {
        Self {
            base_name: String::new(),
            parent_dir: String::new(),
            file_type: String::new(),
            storage_profile: String::new(),
        }
    }
}

impl Default for structs::HostLowLevelProvisioningManagerSnapshotLayoutSpec {
    fn default() -> Self {
        Self {
            id: 0,
            src_filename: String::new(),
            dst_filename: String::new(),
            disk: None,
        }
    }
}

impl Default for structs::HostLowLevelProvisioningManagerVmMigrationStatus {
    fn default() -> Self {
        Self {
            migration_id: 0,
            r#type: String::new(),
            source: false,
            considered_successful: false,
        }
    }
}

impl Default for structs::HostLowLevelProvisioningManagerVmRecoveryInfo {
    fn default() -> Self {
        Self {
            version: String::new(),
            bios_uuid: String::new(),
            instance_uuid: String::new(),
            ft_info: None,
        }
    }
}

impl Default for structs::HostMaintenanceSpec {
    fn default() -> Self {
        Self {
            vsan_mode: None,
            purpose: None,
        }
    }
}

impl Default for structs::ServiceConsoleReservationInfo {
    fn default() -> Self {
        Self {
            service_console_reserved_cfg: 0,
            service_console_reserved: 0,
            unreserved: 0,
        }
    }
}

impl Default for structs::VirtualMachineMemoryReservationInfo {
    fn default() -> Self {
        Self {
            virtual_machine_min: 0,
            virtual_machine_max: 0,
            virtual_machine_reserved: 0,
            allocation_policy: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineMemoryReservationSpec {
    fn default() -> Self {
        Self {
            virtual_machine_reserved: None,
            allocation_policy: None,
        }
    }
}

impl Default for structs::HostMemorySpec {
    fn default() -> Self {
        Self {
            service_console_reservation: None,
        }
    }
}

impl Default for structs::HostMemoryTierInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            r#type: String::new(),
            flags: None,
            internal_flags: None,
            size: 0,
        }
    }
}

impl Default for structs::HostMountInfo {
    fn default() -> Self {
        Self {
            path: None,
            access_mode: String::new(),
            mounted: None,
            accessible: None,
            inaccessible_reason: None,
            vmknic_name: None,
            vmknic_active: None,
            mount_failed_reason: None,
            num_tcp_connections: None,
        }
    }
}

impl Default for structs::HostMultipathInfo {
    fn default() -> Self {
        Self {
            lun: None,
        }
    }
}

impl Default for structs::HostMultipathInfoLogicalUnit {
    fn default() -> Self {
        Self {
            key: String::new(),
            id: String::new(),
            lun: String::new(),
            path: Vec::new(),
            policy: Default::default(),
            storage_array_type_policy: None,
        }
    }
}

impl Default for structs::HostMultipathInfoLogicalUnitPolicy {
    fn default() -> Self {
        Self {
            policy: String::new(),
        }
    }
}

impl Default for structs::HostMultipathInfoFixedLogicalUnitPolicy {
    fn default() -> Self {
        Self {
            host_multipath_info_logical_unit_policy_: structs::HostMultipathInfoLogicalUnitPolicy::default(),
            prefer: String::new(),
        }
    }
}

impl Default for structs::HostMultipathInfoHppLogicalUnitPolicy {
    fn default() -> Self {
        Self {
            host_multipath_info_logical_unit_policy_: structs::HostMultipathInfoLogicalUnitPolicy::default(),
            bytes: None,
            iops: None,
            path: None,
            latency_eval_time: None,
            sampling_ios_per_path: None,
        }
    }
}

impl Default for structs::HostMultipathInfoLogicalUnitStorageArrayTypePolicy {
    fn default() -> Self {
        Self {
            policy: String::new(),
        }
    }
}

impl Default for structs::HostMultipathInfoPath {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: String::new(),
            path_state: String::new(),
            state: None,
            is_working_path: None,
            adapter: String::new(),
            lun: String::new(),
            transport: None,
        }
    }
}

impl Default for structs::HostMultipathStateInfo {
    fn default() -> Self {
        Self {
            path: None,
        }
    }
}

impl Default for structs::HostMultipathStateInfoPath {
    fn default() -> Self {
        Self {
            name: String::new(),
            path_state: String::new(),
        }
    }
}

impl Default for structs::HostNasVolumeConfig {
    fn default() -> Self {
        Self {
            change_operation: None,
            spec: None,
        }
    }
}

impl Default for structs::HostNasVolumeSpec {
    fn default() -> Self {
        Self {
            remote_host: String::new(),
            remote_path: String::new(),
            local_path: String::new(),
            access_mode: String::new(),
            r#type: None,
            user_name: None,
            password: None,
            remote_host_names: None,
            security_type: None,
            vmknic_to_bind: None,
            vmknic_bound: None,
            connections: None,
        }
    }
}

impl Default for structs::HostNasVolumeUserInfo {
    fn default() -> Self {
        Self {
            user: String::new(),
        }
    }
}

impl Default for structs::HostNatService {
    fn default() -> Self {
        Self {
            key: String::new(),
            spec: structs::HostNatServiceSpec::default(),
        }
    }
}

impl Default for structs::HostNatServiceConfig {
    fn default() -> Self {
        Self {
            change_operation: None,
            key: String::new(),
            spec: structs::HostNatServiceSpec::default(),
        }
    }
}

impl Default for structs::HostNatServiceNameServiceSpec {
    fn default() -> Self {
        Self {
            dns_auto_detect: false,
            dns_policy: String::new(),
            dns_retries: 0,
            dns_timeout: 0,
            dns_name_server: None,
            nbds_timeout: 0,
            nbns_retries: 0,
            nbns_timeout: 0,
        }
    }
}

impl Default for structs::HostNatServicePortForwardSpec {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            name: String::new(),
            host_port: 0,
            guest_port: 0,
            guest_ip_address: String::new(),
        }
    }
}

impl Default for structs::HostNatServiceSpec {
    fn default() -> Self {
        Self {
            virtual_switch: String::new(),
            active_ftp: false,
            allow_any_oui: false,
            config_port: false,
            ip_gateway_address: String::new(),
            udp_timeout: 0,
            port_forward: None,
            name_service: None,
        }
    }
}

impl Default for structs::HostNetCapabilities {
    fn default() -> Self {
        Self {
            can_set_physical_nic_link_speed: false,
            supports_nic_teaming: false,
            nic_teaming_policy: None,
            supports_vlan: false,
            uses_service_console_nic: false,
            supports_network_hints: false,
            max_port_groups_per_vswitch: None,
            vswitch_config_supported: false,
            vnic_config_supported: false,
            ip_route_config_supported: false,
            dns_config_supported: false,
            dhcp_on_vnic_supported: false,
            ip_v_6_supported: false,
            backup_nfc_nioc_supported: None,
        }
    }
}

impl Default for structs::HostNetOffloadCapabilities {
    fn default() -> Self {
        Self {
            csum_offload: None,
            tcp_segmentation: None,
            zero_copy_xmit: None,
        }
    }
}

impl Default for structs::HostNetStackInstance {
    fn default() -> Self {
        Self {
            key: None,
            name: None,
            dns_config: None,
            ip_route_config: None,
            requested_max_number_of_connections: None,
            congestion_control_algorithm: None,
            ip_v_6_enabled: None,
            route_table_config: None,
            owner: None,
        }
    }
}

impl Default for structs::HostNetworkConfig {
    fn default() -> Self {
        Self {
            vswitch: None,
            proxy_switch: None,
            portgroup: None,
            pnic: None,
            vnic: None,
            console_vnic: None,
            dns_config: None,
            ip_route_config: None,
            console_ip_route_config: None,
            route_table_config: None,
            dhcp: None,
            nat: None,
            ip_v_6_enabled: None,
            net_stack_spec: None,
            migration_status: None,
        }
    }
}

impl Default for structs::HostNetworkConfigNetStackSpec {
    fn default() -> Self {
        Self {
            net_stack_instance: structs::HostNetStackInstance::default(),
            operation: None,
        }
    }
}

impl Default for structs::HostNetworkConfigResult {
    fn default() -> Self {
        Self {
            vnic_device: None,
            console_vnic_device: None,
        }
    }
}

impl Default for structs::HostNetworkInfo {
    fn default() -> Self {
        Self {
            vswitch: None,
            proxy_switch: None,
            portgroup: None,
            pnic: None,
            rdma_device: None,
            vnic: None,
            console_vnic: None,
            dns_config: None,
            ip_route_config: None,
            console_ip_route_config: None,
            route_table_info: None,
            dhcp: None,
            nat: None,
            ip_v_6_enabled: None,
            at_boot_ip_v_6_enabled: None,
            net_stack_instance: None,
            opaque_switch: None,
            opaque_network: None,
            nsx_transport_node_id: None,
            nvds_to_vds_migration_required: None,
            migration_status: None,
        }
    }
}

impl Default for structs::HostNetworkPolicy {
    fn default() -> Self {
        Self {
            security: None,
            nic_teaming: None,
            offload_policy: None,
            shaping_policy: None,
        }
    }
}

impl Default for structs::HostNicFailureCriteria {
    fn default() -> Self {
        Self {
            check_speed: None,
            speed: None,
            check_duplex: None,
            full_duplex: None,
            check_error_percent: None,
            percentage: None,
            check_beacon: None,
        }
    }
}

impl Default for structs::HostNicOrderPolicy {
    fn default() -> Self {
        Self {
            active_nic: None,
            standby_nic: None,
        }
    }
}

impl Default for structs::HostNicTeamingPolicy {
    fn default() -> Self {
        Self {
            policy: None,
            reverse_policy: None,
            notify_switches: None,
            rolling_order: None,
            failure_criteria: None,
            nic_order: None,
        }
    }
}

impl Default for structs::HostNetworkSecurityPolicy {
    fn default() -> Self {
        Self {
            allow_promiscuous: None,
            mac_changes: None,
            forged_transmits: None,
        }
    }
}

impl Default for structs::HostNetworkTrafficShapingPolicy {
    fn default() -> Self {
        Self {
            enabled: None,
            average_bandwidth: None,
            peak_bandwidth: None,
            burst_size: None,
        }
    }
}

impl Default for structs::HostNtpConfig {
    fn default() -> Self {
        Self {
            server: None,
            config_file: None,
        }
    }
}

impl Default for structs::HostNumaInfo {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            num_nodes: 0,
            numa_node: None,
        }
    }
}

impl Default for structs::HostNumaNode {
    fn default() -> Self {
        Self {
            type_id: 0,
            cpu_id: Vec::new(),
            memory_size: None,
            memory_range_begin: 0,
            memory_range_length: 0,
            pci_id: None,
        }
    }
}

impl Default for structs::HostNumericSensorInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            health_state: None,
            current_reading: 0,
            unit_modifier: 0,
            base_units: String::new(),
            rate_units: None,
            sensor_type: String::new(),
            id: None,
            sensor_number: None,
            time_stamp: None,
            fru: None,
        }
    }
}

impl Default for structs::NvdimmDimmInfo {
    fn default() -> Self {
        Self {
            dimm_handle: 0,
            health_info: structs::NvdimmHealthInfo::default(),
            total_capacity: 0,
            persistent_capacity: 0,
            available_persistent_capacity: 0,
            volatile_capacity: 0,
            available_volatile_capacity: 0,
            block_capacity: 0,
            region_info: None,
            representation_string: String::new(),
        }
    }
}

impl Default for structs::NvdimmGuid {
    fn default() -> Self {
        Self {
            uuid: String::new(),
        }
    }
}

impl Default for structs::NvdimmHealthInfo {
    fn default() -> Self {
        Self {
            health_status: String::new(),
            health_information: String::new(),
            state_flag_info: None,
            dimm_temperature: 0,
            dimm_temperature_threshold: 0,
            spare_blocks_percentage: 0,
            spare_block_threshold: 0,
            dimm_lifespan_percentage: 0,
            es_temperature: None,
            es_temperature_threshold: None,
            es_lifespan_percentage: None,
        }
    }
}

impl Default for structs::NvdimmInterleaveSetInfo {
    fn default() -> Self {
        Self {
            set_id: 0,
            range_type: String::new(),
            base_address: 0,
            size: 0,
            available_size: 0,
            device_list: None,
            state: String::new(),
        }
    }
}

impl Default for structs::NvdimmNamespaceCreateSpec {
    fn default() -> Self {
        Self {
            friendly_name: None,
            block_size: 0,
            block_count: 0,
            r#type: String::new(),
            location_id: 0,
        }
    }
}

impl Default for structs::NvdimmNamespaceDeleteSpec {
    fn default() -> Self {
        Self {
            uuid: String::new(),
        }
    }
}

impl Default for structs::NvdimmNamespaceDetails {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            friendly_name: String::new(),
            size: 0,
            r#type: String::new(),
            namespace_health_status: String::new(),
            interleaveset_id: 0,
            state: String::new(),
        }
    }
}

impl Default for structs::NvdimmNamespaceInfo {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            friendly_name: String::new(),
            block_size: 0,
            block_count: 0,
            r#type: String::new(),
            namespace_health_status: String::new(),
            location_id: 0,
            state: String::new(),
        }
    }
}

impl Default for structs::NvdimmSystemInfo {
    fn default() -> Self {
        Self {
            summary: None,
            dimms: None,
            dimm_info: None,
            interleave_set: None,
            i_set_info: None,
            namespace: None,
            ns_info: None,
            ns_details: None,
        }
    }
}

impl Default for structs::NvdimmPMemNamespaceCreateSpec {
    fn default() -> Self {
        Self {
            friendly_name: None,
            size: 0,
            interleaveset_id: 0,
        }
    }
}

impl Default for structs::NvdimmRegionInfo {
    fn default() -> Self {
        Self {
            region_id: 0,
            set_id: 0,
            range_type: String::new(),
            start_addr: 0,
            size: 0,
            offset: 0,
        }
    }
}

impl Default for structs::NvdimmSummary {
    fn default() -> Self {
        Self {
            num_dimms: 0,
            health_status: String::new(),
            total_capacity: 0,
            persistent_capacity: 0,
            block_capacity: 0,
            available_capacity: 0,
            num_interleavesets: 0,
            num_namespaces: 0,
        }
    }
}

impl Default for structs::HostNvmeController {
    fn default() -> Self {
        Self {
            key: String::new(),
            controller_number: 0,
            subnqn: String::new(),
            name: String::new(),
            associated_adapter: String::new(),
            transport_type: String::new(),
            fused_operation_supported: false,
            number_of_queues: 0,
            queue_size: 0,
            attached_namespace: None,
            vendor_id: None,
            model: None,
            serial_number: None,
            firmware_version: None,
        }
    }
}

impl Default for structs::HostNvmeDisconnectSpec {
    fn default() -> Self {
        Self {
            hba_name: String::new(),
            subnqn: None,
            controller_number: None,
        }
    }
}

impl Default for structs::HostNvmeDiscoveryLog {
    fn default() -> Self {
        Self {
            entry: None,
            complete: false,
        }
    }
}

impl Default for structs::HostNvmeDiscoveryLogEntry {
    fn default() -> Self {
        Self {
            subnqn: String::new(),
            subsystem_type: String::new(),
            subsystem_port_id: 0,
            controller_id: 0,
            admin_queue_max_size: 0,
            transport_parameters: Default::default(),
            transport_requirements: String::new(),
            connected: false,
        }
    }
}

impl Default for structs::HostNvmeNamespace {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: String::new(),
            id: 0,
            block_size: 0,
            capacity_in_blocks: 0,
        }
    }
}

impl Default for structs::HostNvmeSpec {
    fn default() -> Self {
        Self {
            hba_name: String::new(),
            transport_parameters: Default::default(),
        }
    }
}

impl Default for structs::HostNvmeConnectSpec {
    fn default() -> Self {
        Self {
            host_nvme_spec_: structs::HostNvmeSpec::default(),
            subnqn: String::new(),
            controller_id: None,
            admin_queue_size: None,
            keep_alive_timeout: None,
        }
    }
}

impl Default for structs::HostNvmeDiscoverSpec {
    fn default() -> Self {
        Self {
            host_nvme_spec_: structs::HostNvmeSpec::default(),
            auto_connect: None,
            root_discovery_controller: None,
        }
    }
}

impl Default for structs::HostNvmeTopology {
    fn default() -> Self {
        Self {
            adapter: None,
        }
    }
}

impl Default for structs::HostNvmeTopologyInterface {
    fn default() -> Self {
        Self {
            key: String::new(),
            adapter: String::new(),
            connected_controller: None,
        }
    }
}

impl Default for structs::HostNvmeTransportParameters {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostNvmeOpaqueTransportParameters {
    fn default() -> Self {
        Self {
            trtype: String::new(),
            traddr: String::new(),
            adrfam: String::new(),
            trsvcid: String::new(),
            tsas: Vec::new(),
        }
    }
}

impl Default for structs::HostNvmeOverFibreChannelParameters {
    fn default() -> Self {
        Self {
            node_world_wide_name: 0,
            port_world_wide_name: 0,
        }
    }
}

impl Default for structs::HostNvmeOverRdmaParameters {
    fn default() -> Self {
        Self {
            address: String::new(),
            address_family: None,
            port_number: None,
        }
    }
}

impl Default for structs::HostNvmeOverTcpParameters {
    fn default() -> Self {
        Self {
            address: String::new(),
            port_number: None,
            digest_verification: None,
        }
    }
}

impl Default for structs::HostOpaqueNetworkInfo {
    fn default() -> Self {
        Self {
            dynamic_property: None,
            opaque_network_id: String::new(),
            opaque_network_name: String::new(),
            opaque_network_type: String::new(),
            pnic_zone: None,
            capability: None,
            extra_config: None,
        }
    }
}

impl Default for structs::HostOpaqueSwitch {
    fn default() -> Self {
        Self {
            dynamic_property: None,
            key: String::new(),
            name: None,
            pnic: None,
            pnic_zone: None,
            status: None,
            vtep: None,
            extra_config: None,
            feature_capability: None,
        }
    }
}

impl Default for structs::HostOpaqueSwitchPhysicalNicZone {
    fn default() -> Self {
        Self {
            key: String::new(),
            pnic_device: None,
        }
    }
}

impl Default for structs::HostPartialMaintenanceModeRuntimeInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
            host_status: String::new(),
        }
    }
}

impl Default for structs::HostPatchManagerLocator {
    fn default() -> Self {
        Self {
            url: String::new(),
            proxy: None,
        }
    }
}

impl Default for structs::HostPatchManagerPatchManagerOperationSpec {
    fn default() -> Self {
        Self {
            proxy: None,
            port: None,
            user_name: None,
            password: None,
            cmd_option: None,
        }
    }
}

impl Default for structs::HostPatchManagerResult {
    fn default() -> Self {
        Self {
            version: String::new(),
            status: None,
            xml_result: None,
        }
    }
}

impl Default for structs::HostPatchManagerStatus {
    fn default() -> Self {
        Self {
            id: String::new(),
            applicable: false,
            reason: None,
            integrity: None,
            installed: false,
            install_state: None,
            prerequisite_patch: None,
            restart_required: false,
            reconnect_required: false,
            vm_off_required: false,
            superseded_patch_ids: None,
        }
    }
}

impl Default for structs::HostPatchManagerStatusPrerequisitePatch {
    fn default() -> Self {
        Self {
            id: String::new(),
            install_state: None,
        }
    }
}

impl Default for structs::HostPathSelectionPolicyOption {
    fn default() -> Self {
        Self {
            policy: Default::default(),
        }
    }
}

impl Default for structs::HostPciDevice {
    fn default() -> Self {
        Self {
            id: String::new(),
            class_id: 0,
            bus: 0,
            slot: 0,
            physical_slot: None,
            slot_description: None,
            function: 0,
            vendor_id: 0,
            sub_vendor_id: 0,
            vendor_name: String::new(),
            device_id: 0,
            sub_device_id: 0,
            parent_bridge: None,
            device_name: String::new(),
            device_class_name: None,
        }
    }
}

impl Default for structs::HostPciPassthruConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            passthru_enabled: false,
            apply_now: None,
            hardware_label: None,
        }
    }
}

impl Default for structs::HostSriovConfig {
    fn default() -> Self {
        Self {
            host_pci_passthru_config_: structs::HostPciPassthruConfig::default(),
            sriov_enabled: false,
            num_virtual_function: 0,
        }
    }
}

impl Default for structs::HostPciPassthruInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            dependent_device: String::new(),
            passthru_enabled: false,
            passthru_capable: false,
            passthru_active: false,
            hardware_label: None,
        }
    }
}

impl Default for structs::HostSriovInfo {
    fn default() -> Self {
        Self {
            host_pci_passthru_info_: structs::HostPciPassthruInfo::default(),
            sriov_enabled: false,
            sriov_capable: false,
            sriov_active: false,
            num_virtual_function_requested: 0,
            num_virtual_function: 0,
            max_virtual_function_supported: 0,
        }
    }
}

impl Default for structs::HostPersistentMemoryInfo {
    fn default() -> Self {
        Self {
            capacity_in_mb: None,
            volume_uuid: None,
        }
    }
}

impl Default for structs::PhysicalNic {
    fn default() -> Self {
        Self {
            key: None,
            device: String::new(),
            pci: String::new(),
            driver: None,
            driver_version: None,
            firmware_version: None,
            link_speed: None,
            valid_link_specification: None,
            spec: structs::PhysicalNicSpec::default(),
            wake_on_lan_supported: false,
            mac: String::new(),
            fcoe_configuration: None,
            vm_direct_path_gen_2_supported: None,
            vm_direct_path_gen_2_supported_mode: None,
            resource_pool_scheduler_allowed: None,
            resource_pool_scheduler_disallowed_reason: None,
            auto_negotiate_supported: None,
            enhanced_networking_stack_supported: None,
            ens_interrupt_supported: None,
            rdma_device: None,
            dpu_id: None,
        }
    }
}

impl Default for structs::PhysicalNicCdpDeviceCapability {
    fn default() -> Self {
        Self {
            router: false,
            transparent_bridge: false,
            source_route_bridge: false,
            network_switch: false,
            host: false,
            igmp_enabled: false,
            repeater: false,
        }
    }
}

impl Default for structs::PhysicalNicCdpInfo {
    fn default() -> Self {
        Self {
            cdp_version: None,
            timeout: None,
            ttl: None,
            samples: None,
            dev_id: None,
            address: None,
            port_id: None,
            device_capability: None,
            software_version: None,
            hardware_platform: None,
            ip_prefix: None,
            ip_prefix_len: None,
            vlan: None,
            full_duplex: None,
            mtu: None,
            system_name: None,
            system_oid: None,
            mgmt_addr: None,
            location: None,
        }
    }
}

impl Default for structs::PhysicalNicConfig {
    fn default() -> Self {
        Self {
            device: String::new(),
            spec: structs::PhysicalNicSpec::default(),
        }
    }
}

impl Default for structs::PhysicalNicLinkInfo {
    fn default() -> Self {
        Self {
            speed_mb: 0,
            duplex: false,
        }
    }
}

impl Default for structs::LinkLayerDiscoveryProtocolInfo {
    fn default() -> Self {
        Self {
            chassis_id: String::new(),
            port_id: String::new(),
            time_to_live: 0,
            parameter: None,
        }
    }
}

impl Default for structs::PhysicalNicHintInfo {
    fn default() -> Self {
        Self {
            device: String::new(),
            subnet: None,
            network: None,
            connected_switch_port: None,
            lldp_info: None,
        }
    }
}

impl Default for structs::PhysicalNicHint {
    fn default() -> Self {
        Self {
            vlan_id: None,
        }
    }
}

impl Default for structs::PhysicalNicIpHint {
    fn default() -> Self {
        Self {
            physical_nic_hint_: structs::PhysicalNicHint::default(),
            ip_subnet: String::new(),
        }
    }
}

impl Default for structs::PhysicalNicNameHint {
    fn default() -> Self {
        Self {
            physical_nic_hint_: structs::PhysicalNicHint::default(),
            network: String::new(),
        }
    }
}

impl Default for structs::PhysicalNicSpec {
    fn default() -> Self {
        Self {
            ip: None,
            link_speed: None,
            enable_enhanced_networking_stack: None,
            ens_interrupt_enabled: None,
        }
    }
}

impl Default for structs::HostPlugStoreTopology {
    fn default() -> Self {
        Self {
            adapter: None,
            path: None,
            target: None,
            device: None,
            plugin: None,
        }
    }
}

impl Default for structs::HostPlugStoreTopologyAdapter {
    fn default() -> Self {
        Self {
            key: String::new(),
            adapter: String::new(),
            path: None,
        }
    }
}

impl Default for structs::HostPlugStoreTopologyDevice {
    fn default() -> Self {
        Self {
            key: String::new(),
            lun: String::new(),
            path: None,
        }
    }
}

impl Default for structs::HostPlugStoreTopologyPath {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: String::new(),
            channel_number: None,
            target_number: None,
            lun_number: None,
            adapter: None,
            target: None,
            device: None,
        }
    }
}

impl Default for structs::HostPlugStoreTopologyPlugin {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: String::new(),
            device: None,
            claimed_path: None,
        }
    }
}

impl Default for structs::HostPlugStoreTopologyTarget {
    fn default() -> Self {
        Self {
            key: String::new(),
            transport: None,
        }
    }
}

impl Default for structs::PnicTsoInfo {
    fn default() -> Self {
        Self {
            nic_name: String::new(),
            is_supported: false,
            is_enabled: false,
        }
    }
}

impl Default for structs::HostPortGroup {
    fn default() -> Self {
        Self {
            key: None,
            port: None,
            vswitch: None,
            computed_policy: structs::HostNetworkPolicy::default(),
            spec: structs::HostPortGroupSpec::default(),
        }
    }
}

impl Default for structs::HostPortGroupConfig {
    fn default() -> Self {
        Self {
            change_operation: None,
            spec: None,
        }
    }
}

impl Default for structs::HostPortGroupPort {
    fn default() -> Self {
        Self {
            key: None,
            mac: None,
            r#type: String::new(),
        }
    }
}

impl Default for structs::HostPortGroupSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            vlan_id: 0,
            vswitch_name: String::new(),
            policy: structs::HostNetworkPolicy::default(),
        }
    }
}

impl Default for structs::PowerSystemCapability {
    fn default() -> Self {
        Self {
            available_policy: Vec::new(),
        }
    }
}

impl Default for structs::PowerSystemInfo {
    fn default() -> Self {
        Self {
            current_policy: structs::HostPowerPolicy::default(),
        }
    }
}

impl Default for structs::HostPowerPolicy {
    fn default() -> Self {
        Self {
            key: 0,
            name: String::new(),
            short_name: String::new(),
            description: String::new(),
        }
    }
}

impl Default for structs::HostProtocolEndpoint {
    fn default() -> Self {
        Self {
            pe_type: String::new(),
            r#type: None,
            uuid: String::new(),
            host_key: None,
            storage_array: None,
            nfs_server: None,
            nfs_dir: None,
            nfs_server_scope: None,
            nfs_server_major: None,
            nfs_server_auth_type: None,
            nfs_server_user: None,
            device_id: None,
            used_by_stretched_container: None,
        }
    }
}

impl Default for structs::HostPtpConfig {
    fn default() -> Self {
        Self {
            domain: None,
            port: None,
        }
    }
}

impl Default for structs::HostPtpConfigPtpPort {
    fn default() -> Self {
        Self {
            index: 0,
            device_type: None,
            device: None,
            ip_config: None,
        }
    }
}

impl Default for structs::HostQualifiedName {
    fn default() -> Self {
        Self {
            value: String::new(),
            r#type: String::new(),
        }
    }
}

impl Default for structs::HostRdmaDevice {
    fn default() -> Self {
        Self {
            key: String::new(),
            device: String::new(),
            driver: None,
            description: None,
            backing: None,
            connection_info: structs::HostRdmaDeviceConnectionInfo::default(),
            capability: structs::HostRdmaDeviceCapability::default(),
        }
    }
}

impl Default for structs::HostRdmaDeviceBacking {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostRdmaDevicePnicBacking {
    fn default() -> Self {
        Self {
            paired_uplink: String::new(),
        }
    }
}

impl Default for structs::HostRdmaDeviceCapability {
    fn default() -> Self {
        Self {
            roce_v_1_capable: false,
            roce_v_2_capable: false,
            i_warp_capable: false,
        }
    }
}

impl Default for structs::HostRdmaDeviceConnectionInfo {
    fn default() -> Self {
        Self {
            state: String::new(),
            mtu: 0,
            speed_in_mbps: 0,
        }
    }
}

impl Default for structs::HostReliableMemoryInfo {
    fn default() -> Self {
        Self {
            memory_size: 0,
        }
    }
}

impl Default for structs::HostResignatureRescanResult {
    fn default() -> Self {
        Self {
            rescan: None,
            result: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::HostFirewallRuleset {
    fn default() -> Self {
        Self {
            key: String::new(),
            label: String::new(),
            required: false,
            rule: Vec::new(),
            service: None,
            enabled: false,
            allowed_hosts: None,
            user_controllable: None,
            ip_list_user_configurable: None,
        }
    }
}

impl Default for structs::HostFirewallRulesetIpList {
    fn default() -> Self {
        Self {
            ip_address: None,
            ip_network: None,
            all_ip: false,
        }
    }
}

impl Default for structs::HostFirewallRulesetIpNetwork {
    fn default() -> Self {
        Self {
            network: String::new(),
            prefix_length: 0,
        }
    }
}

impl Default for structs::HostFirewallRule {
    fn default() -> Self {
        Self {
            port: 0,
            end_port: None,
            direction: enums::HostFirewallRuleDirectionEnum::default(),
            port_type: None,
            protocol: String::new(),
        }
    }
}

impl Default for structs::HostFirewallRulesetRulesetSpec {
    fn default() -> Self {
        Self {
            allowed_hosts: structs::HostFirewallRulesetIpList::default(),
        }
    }
}

impl Default for structs::HostRuntimeInfo {
    fn default() -> Self {
        Self {
            connection_state: enums::HostSystemConnectionStateEnum::default(),
            power_state: enums::HostSystemPowerStateEnum::default(),
            standby_mode: None,
            in_maintenance_mode: false,
            in_quarantine_mode: None,
            boot_time: None,
            health_system_runtime: None,
            das_host_state: None,
            tpm_pcr_values: None,
            vsan_runtime_info: None,
            network_runtime_info: None,
            v_flash_resource_runtime_info: None,
            host_max_virtual_disk_capacity: None,
            crypto_state: None,
            crypto_key_id: None,
            stateless_nvds_migration_ready: None,
            partial_maintenance_mode: None,
            state_encryption: None,
        }
    }
}

impl Default for structs::HostRuntimeInfoNetStackInstanceRuntimeInfo {
    fn default() -> Self {
        Self {
            net_stack_instance_key: String::new(),
            state: None,
            vmknic_keys: None,
            max_number_of_connections: None,
            current_ip_v_6_enabled: None,
        }
    }
}

impl Default for structs::HostNetworkResourceRuntime {
    fn default() -> Self {
        Self {
            pnic_resource_info: Vec::new(),
        }
    }
}

impl Default for structs::HostRuntimeInfoNetworkRuntimeInfo {
    fn default() -> Self {
        Self {
            net_stack_instance_runtime_info: None,
            network_resource_runtime: None,
        }
    }
}

impl Default for structs::HostPlacedVirtualNicIdentifier {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            vnic_key: String::new(),
            reservation: None,
        }
    }
}

impl Default for structs::HostPnicNetworkResourceInfo {
    fn default() -> Self {
        Self {
            pnic_device: String::new(),
            available_bandwidth_for_vm_traffic: None,
            unused_bandwidth_for_vm_traffic: None,
            placed_virtual_nics: None,
        }
    }
}

impl Default for structs::HostRuntimeInfoStateEncryptionInfo {
    fn default() -> Self {
        Self {
            protection_mode: String::new(),
            require_secure_boot: None,
            require_exec_installed_only: None,
        }
    }
}

impl Default for structs::HostScsiDiskPartition {
    fn default() -> Self {
        Self {
            disk_name: String::new(),
            partition: 0,
        }
    }
}

impl Default for structs::ScsiLunCapabilities {
    fn default() -> Self {
        Self {
            update_display_name_supported: false,
        }
    }
}

impl Default for structs::ScsiLunDescriptor {
    fn default() -> Self {
        Self {
            quality: String::new(),
            id: String::new(),
        }
    }
}

impl Default for structs::ScsiLunDurableName {
    fn default() -> Self {
        Self {
            namespace: String::new(),
            namespace_id: 0,
            data: None,
        }
    }
}

impl Default for structs::HostScsiTopology {
    fn default() -> Self {
        Self {
            adapter: None,
        }
    }
}

impl Default for structs::HostScsiTopologyInterface {
    fn default() -> Self {
        Self {
            key: String::new(),
            adapter: String::new(),
            target: None,
        }
    }
}

impl Default for structs::HostScsiTopologyLun {
    fn default() -> Self {
        Self {
            key: String::new(),
            lun: 0,
            scsi_lun: String::new(),
        }
    }
}

impl Default for structs::HostScsiTopologyTarget {
    fn default() -> Self {
        Self {
            key: String::new(),
            target: 0,
            lun: None,
            transport: None,
        }
    }
}

impl Default for structs::HostSecuritySpec {
    fn default() -> Self {
        Self {
            admin_password: None,
            remove_permission: None,
            add_permission: None,
        }
    }
}

impl Default for structs::HostService {
    fn default() -> Self {
        Self {
            key: String::new(),
            label: String::new(),
            required: false,
            uninstallable: false,
            running: false,
            ruleset: None,
            policy: String::new(),
            source_package: None,
        }
    }
}

impl Default for structs::HostServiceSourcePackage {
    fn default() -> Self {
        Self {
            source_package_name: String::new(),
            description: String::new(),
        }
    }
}

impl Default for structs::HostServiceConfig {
    fn default() -> Self {
        Self {
            service_id: String::new(),
            startup_policy: String::new(),
        }
    }
}

impl Default for structs::HostServiceInfo {
    fn default() -> Self {
        Self {
            service: None,
        }
    }
}

impl Default for structs::HostSevInfo {
    fn default() -> Self {
        Self {
            sev_state: String::new(),
            max_sev_es_guests: 0,
            snp_state: None,
            snp_supported: None,
        }
    }
}

impl Default for structs::HostSgxInfo {
    fn default() -> Self {
        Self {
            sgx_state: String::new(),
            total_epc_memory: 0,
            flc_mode: String::new(),
            le_pub_key_hash: None,
            registration_info: None,
        }
    }
}

impl Default for structs::HostSgxRegistrationInfo {
    fn default() -> Self {
        Self {
            status: None,
            bios_error: None,
            registration_url: None,
            r#type: None,
            ppid: None,
            last_registered_time: None,
        }
    }
}

impl Default for structs::HostSharedGpuCapabilities {
    fn default() -> Self {
        Self {
            vgpu: String::new(),
            disk_snapshot_supported: false,
            memory_snapshot_supported: false,
            suspend_supported: false,
            migrate_supported: false,
        }
    }
}

impl Default for structs::HostSnmpSystemAgentLimits {
    fn default() -> Self {
        Self {
            max_read_only_communities: 0,
            max_trap_destinations: 0,
            max_community_length: 0,
            max_buffer_size: 0,
            capability: enums::HostSnmpAgentCapabilityEnum::default(),
        }
    }
}

impl Default for structs::HostSnmpConfigSpec {
    fn default() -> Self {
        Self {
            enabled: None,
            port: None,
            read_only_communities: None,
            trap_targets: None,
            option: None,
        }
    }
}

impl Default for structs::HostSnmpDestination {
    fn default() -> Self {
        Self {
            host_name: String::new(),
            port: 0,
            community: String::new(),
        }
    }
}

impl Default for structs::SoftwarePackage {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            r#type: String::new(),
            vendor: String::new(),
            acceptance_level: String::new(),
            summary: String::new(),
            description: String::new(),
            reference_url: None,
            creation_date: None,
            depends: None,
            conflicts: None,
            replaces: None,
            provides: None,
            maintenance_mode_required: None,
            hardware_platforms_required: None,
            capability: structs::SoftwarePackageCapability::default(),
            tag: None,
            payload: None,
        }
    }
}

impl Default for structs::SoftwarePackageCapability {
    fn default() -> Self {
        Self {
            live_install_allowed: None,
            live_remove_allowed: None,
            stateless_ready: None,
            overlay: None,
        }
    }
}

impl Default for structs::Relation {
    fn default() -> Self {
        Self {
            constraint: None,
            name: String::new(),
            version: None,
        }
    }
}

impl Default for structs::HostSriovDevicePoolInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
        }
    }
}

impl Default for structs::HostSriovNetworkDevicePoolInfo {
    fn default() -> Self {
        Self {
            host_sriov_device_pool_info_: structs::HostSriovDevicePoolInfo::default(),
            switch_key: None,
            switch_uuid: None,
            pnic: None,
        }
    }
}

impl Default for structs::HostSslThumbprintInfo {
    fn default() -> Self {
        Self {
            principal: String::new(),
            owner_tag: String::new(),
            ssl_thumbprints: None,
        }
    }
}

impl Default for structs::HostStorageArrayTypePolicyOption {
    fn default() -> Self {
        Self {
            policy: Default::default(),
        }
    }
}

impl Default for structs::HostStorageDeviceInfo {
    fn default() -> Self {
        Self {
            host_bus_adapter: None,
            scsi_lun: None,
            scsi_topology: None,
            nvme_topology: None,
            multipath_info: None,
            plug_store_topology: None,
            software_internet_scsi_enabled: false,
        }
    }
}

impl Default for structs::HostStorageSystemDiskLocatorLedResult {
    fn default() -> Self {
        Self {
            key: String::new(),
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::HostStorageSystemScsiLunResult {
    fn default() -> Self {
        Self {
            key: String::new(),
            fault: None,
        }
    }
}

impl Default for structs::HostStorageSystemVmfsVolumeResult {
    fn default() -> Self {
        Self {
            key: String::new(),
            fault: None,
        }
    }
}

impl Default for structs::HostListSummary {
    fn default() -> Self {
        Self {
            host: None,
            hardware: None,
            runtime: None,
            config: structs::HostConfigSummary::default(),
            quick_stats: structs::HostListSummaryQuickStats::default(),
            overall_status: enums::ManagedEntityStatusEnum::default(),
            reboot_required: false,
            custom_value: None,
            management_server_ip: None,
            max_evc_mode_key: None,
            current_evc_mode_key: None,
            current_evc_graphics_mode_key: None,
            gateway: None,
            tpm_attestation: None,
            trust_authority_attestation_infos: None,
        }
    }
}

impl Default for structs::HostConfigSummary {
    fn default() -> Self {
        Self {
            name: String::new(),
            port: 0,
            ssl_thumbprint: None,
            ssl_certificate: None,
            product: None,
            vmotion_enabled: false,
            fault_tolerance_enabled: false,
            feature_version: None,
            agent_vm_datastore: None,
            agent_vm_network: None,
        }
    }
}

impl Default for structs::HostListSummaryGatewaySummary {
    fn default() -> Self {
        Self {
            gateway_type: String::new(),
            gateway_id: String::new(),
        }
    }
}

impl Default for structs::HostHardwareSummary {
    fn default() -> Self {
        Self {
            vendor: String::new(),
            model: String::new(),
            family: None,
            uuid: String::new(),
            other_identifying_info: None,
            memory_size: 0,
            cpu_model: String::new(),
            cpu_mhz: 0,
            num_cpu_pkgs: 0,
            num_cpu_cores: 0,
            num_cpu_threads: 0,
            num_nics: 0,
            num_hb_as: 0,
        }
    }
}

impl Default for structs::HostListSummaryQuickStats {
    fn default() -> Self {
        Self {
            overall_cpu_usage: None,
            overall_memory_usage: None,
            distributed_cpu_fairness: None,
            distributed_memory_fairness: None,
            available_p_mem_capacity: None,
            uptime: None,
        }
    }
}

impl Default for structs::SystemEventInfo {
    fn default() -> Self {
        Self {
            record_id: 0,
            when: String::new(),
            sel_type: 0,
            message: String::new(),
            sensor_number: 0,
        }
    }
}

impl Default for structs::HostSystemHealthInfo {
    fn default() -> Self {
        Self {
            numeric_sensor_info: None,
        }
    }
}

impl Default for structs::HostSystemIdentificationInfo {
    fn default() -> Self {
        Self {
            identifier_value: String::new(),
            identifier_type: Default::default(),
        }
    }
}

impl Default for structs::HostSystemInfo {
    fn default() -> Self {
        Self {
            vendor: String::new(),
            model: String::new(),
            family: None,
            uuid: String::new(),
            other_identifying_info: None,
            serial_number: None,
            qualified_name: None,
            vvol_host_nqn: None,
            vvol_host_id: None,
            boot_command_line: None,
        }
    }
}

impl Default for structs::HostSystemResourceInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
            config: None,
            child: None,
        }
    }
}

impl Default for structs::HostSystemSwapConfiguration {
    fn default() -> Self {
        Self {
            option: None,
        }
    }
}

impl Default for structs::HostSystemSwapConfigurationSystemSwapOption {
    fn default() -> Self {
        Self {
            key: 0,
        }
    }
}

impl Default for structs::HostSystemSwapConfigurationDatastoreOption {
    fn default() -> Self {
        Self {
            host_system_swap_configuration_system_swap_option_: structs::HostSystemSwapConfigurationSystemSwapOption::default(),
            datastore: String::new(),
        }
    }
}

impl Default for structs::HostSystemSwapConfigurationDisabledOption {
    fn default() -> Self {
        Self {
            host_system_swap_configuration_system_swap_option_: structs::HostSystemSwapConfigurationSystemSwapOption::default(),
        }
    }
}

impl Default for structs::HostSystemSwapConfigurationHostCacheOption {
    fn default() -> Self {
        Self {
            host_system_swap_configuration_system_swap_option_: structs::HostSystemSwapConfigurationSystemSwapOption::default(),
        }
    }
}

impl Default for structs::HostSystemSwapConfigurationHostLocalSwapOption {
    fn default() -> Self {
        Self {
            host_system_swap_configuration_system_swap_option_: structs::HostSystemSwapConfigurationSystemSwapOption::default(),
        }
    }
}

impl Default for structs::HostTargetTransport {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostBlockAdapterTargetTransport {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostFibreChannelTargetTransport {
    fn default() -> Self {
        Self {
            port_world_wide_name: 0,
            node_world_wide_name: 0,
        }
    }
}

impl Default for structs::HostFibreChannelOverEthernetTargetTransport {
    fn default() -> Self {
        Self {
            host_fibre_channel_target_transport_: structs::HostFibreChannelTargetTransport::default(),
            vnport_mac: String::new(),
            fcf_mac: String::new(),
            vlan_id: 0,
        }
    }
}

impl Default for structs::HostInternetScsiTargetTransport {
    fn default() -> Self {
        Self {
            i_scsi_name: String::new(),
            i_scsi_alias: String::new(),
            address: None,
        }
    }
}

impl Default for structs::HostParallelScsiTargetTransport {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostPcieTargetTransport {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostRdmaTargetTransport {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostSerialAttachedTargetTransport {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostTcpTargetTransport {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostTdxInfo {
    fn default() -> Self {
        Self {
            tdx_state: String::new(),
            num_tdx_private_key_i_ds: 0,
        }
    }
}

impl Default for structs::HostTpmAttestationInfo {
    fn default() -> Self {
        Self {
            time: String::new(),
            status: enums::HostTpmAttestationInfoAcceptanceStatusEnum::default(),
            message: None,
        }
    }
}

impl Default for structs::HostTpmAttestationReport {
    fn default() -> Self {
        Self {
            tpm_pcr_values: Vec::new(),
            tpm_events: Vec::new(),
            tpm_log_reliable: false,
        }
    }
}

impl Default for structs::HostTpmEventDetails {
    fn default() -> Self {
        Self {
            data_hash: Vec::new(),
            data_hash_method: None,
        }
    }
}

impl Default for structs::HostTpmBootCompleteEventDetails {
    fn default() -> Self {
        Self {
            host_tpm_event_details_: structs::HostTpmEventDetails::default(),
        }
    }
}

impl Default for structs::HostTpmBootSecurityOptionEventDetails {
    fn default() -> Self {
        Self {
            host_tpm_event_details_: structs::HostTpmEventDetails::default(),
            boot_security_option: String::new(),
        }
    }
}

impl Default for structs::HostTpmNvTagEventDetails {
    fn default() -> Self {
        Self {
            host_tpm_boot_security_option_event_details_: structs::HostTpmBootSecurityOptionEventDetails::default(),
        }
    }
}

impl Default for structs::HostTpmSignerEventDetails {
    fn default() -> Self {
        Self {
            host_tpm_boot_security_option_event_details_: structs::HostTpmBootSecurityOptionEventDetails::default(),
        }
    }
}

impl Default for structs::HostTpmCommandEventDetails {
    fn default() -> Self {
        Self {
            host_tpm_event_details_: structs::HostTpmEventDetails::default(),
            command_line: String::new(),
        }
    }
}

impl Default for structs::HostTpmOptionEventDetails {
    fn default() -> Self {
        Self {
            host_tpm_event_details_: structs::HostTpmEventDetails::default(),
            options_file_name: String::new(),
            boot_options: None,
        }
    }
}

impl Default for structs::HostTpmSoftwareComponentEventDetails {
    fn default() -> Self {
        Self {
            host_tpm_event_details_: structs::HostTpmEventDetails::default(),
            component_name: String::new(),
            vib_name: String::new(),
            vib_version: String::new(),
            vib_vendor: String::new(),
        }
    }
}

impl Default for structs::HostTpmVersionEventDetails {
    fn default() -> Self {
        Self {
            host_tpm_event_details_: structs::HostTpmEventDetails::default(),
            version: Vec::new(),
        }
    }
}

impl Default for structs::HostTpmEventLogEntry {
    fn default() -> Self {
        Self {
            pcr_index: 0,
            event_details: Default::default(),
        }
    }
}

impl Default for structs::HostTrustAuthorityAttestationInfo {
    fn default() -> Self {
        Self {
            attestation_status: String::new(),
            service_id: None,
            attested_at: None,
            attested_until: None,
            messages: None,
        }
    }
}

impl Default for structs::HostUnresolvedVmfsExtent {
    fn default() -> Self {
        Self {
            device: structs::HostScsiDiskPartition::default(),
            device_path: String::new(),
            vmfs_uuid: String::new(),
            is_head_extent: false,
            ordinal: 0,
            start_block: 0,
            end_block: 0,
            reason: String::new(),
        }
    }
}

impl Default for structs::HostUnresolvedVmfsResignatureSpec {
    fn default() -> Self {
        Self {
            extent_device_path: Vec::new(),
        }
    }
}

impl Default for structs::HostUnresolvedVmfsResolutionResult {
    fn default() -> Self {
        Self {
            spec: structs::HostUnresolvedVmfsResolutionSpec::default(),
            vmfs: None,
            fault: None,
        }
    }
}

impl Default for structs::HostUnresolvedVmfsResolutionSpec {
    fn default() -> Self {
        Self {
            extent_device_path: Vec::new(),
            uuid_resolution: String::new(),
        }
    }
}

impl Default for structs::HostUnresolvedVmfsVolume {
    fn default() -> Self {
        Self {
            extent: Vec::new(),
            vmfs_label: String::new(),
            vmfs_uuid: String::new(),
            total_blocks: 0,
            resolve_status: structs::HostUnresolvedVmfsVolumeResolveStatus::default(),
        }
    }
}

impl Default for structs::HostUnresolvedVmfsVolumeResolveStatus {
    fn default() -> Self {
        Self {
            resolvable: false,
            incomplete_extents: None,
            multiple_copies: None,
        }
    }
}

impl Default for structs::HostVFlashManagerVFlashCacheConfigInfo {
    fn default() -> Self {
        Self {
            v_flash_module_config_option: None,
            default_v_flash_module: None,
            swap_cache_reservation_in_gb: None,
        }
    }
}

impl Default for structs::HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption {
    fn default() -> Self {
        Self {
            v_flash_module: String::new(),
            v_flash_module_version: String::new(),
            min_supported_module_version: String::new(),
            cache_consistency_type: structs::ChoiceOption::default(),
            cache_mode: structs::ChoiceOption::default(),
            block_size_in_kb_option: structs::LongOption::default(),
            reservation_in_mb_option: structs::LongOption::default(),
            max_disk_size_in_kb: 0,
        }
    }
}

impl Default for structs::HostVFlashManagerVFlashCacheConfigSpec {
    fn default() -> Self {
        Self {
            default_v_flash_module: String::new(),
            swap_cache_reservation_in_gb: 0,
        }
    }
}

impl Default for structs::HostVFlashManagerVFlashConfigInfo {
    fn default() -> Self {
        Self {
            v_flash_resource_config_info: None,
            v_flash_cache_config_info: None,
        }
    }
}

impl Default for structs::HostVFlashManagerVFlashResourceConfigInfo {
    fn default() -> Self {
        Self {
            vffs: None,
            capacity: 0,
        }
    }
}

impl Default for structs::HostVFlashManagerVFlashResourceConfigSpec {
    fn default() -> Self {
        Self {
            vffs_uuid: String::new(),
        }
    }
}

impl Default for structs::HostVFlashManagerVFlashResourceRunTimeInfo {
    fn default() -> Self {
        Self {
            usage: 0,
            capacity: 0,
            accessible: false,
            capacity_for_vm_cache: 0,
            free_for_vm_cache: 0,
        }
    }
}

impl Default for structs::HostVFlashResourceConfigurationResult {
    fn default() -> Self {
        Self {
            device_path: None,
            vffs: None,
            disk_configuration_result: None,
        }
    }
}

impl Default for structs::HostVMotionConfig {
    fn default() -> Self {
        Self {
            vmotion_nic_key: None,
            enabled: false,
        }
    }
}

impl Default for structs::HostVMotionInfo {
    fn default() -> Self {
        Self {
            net_config: None,
            ip_config: None,
        }
    }
}

impl Default for structs::HostVMotionManagerDstInstantCloneResult {
    fn default() -> Self {
        Self {
            dst_vm_id: None,
            start_time: None,
            cpt_load_time: None,
            cpt_load_done_time: None,
            replicate_mem_done_time: None,
            end_time: None,
            cpt_xfer_time: None,
            cpt_cache_used: None,
            dev_cpt_stream_size: None,
            dev_cpt_stream_time: None,
        }
    }
}

impl Default for structs::HostVMotionManagerSrcInstantCloneResult {
    fn default() -> Self {
        Self {
            start_time: None,
            quiesce_time: None,
            quiesce_done_time: None,
            resume_done_time: None,
            end_time: None,
        }
    }
}

impl Default for structs::HostVMotionNetConfig {
    fn default() -> Self {
        Self {
            candidate_vnic: None,
            selected_vnic: None,
        }
    }
}

impl Default for structs::VimHostVsanStretchedClusterHostCapability {
    fn default() -> Self {
        Self {
            feature_version: String::new(),
        }
    }
}

impl Default for structs::HostVffsSpec {
    fn default() -> Self {
        Self {
            device_path: String::new(),
            partition: None,
            major_version: 0,
            volume_name: String::new(),
        }
    }
}

impl Default for structs::HostVirtualNic {
    fn default() -> Self {
        Self {
            device: String::new(),
            key: String::new(),
            portgroup: String::new(),
            spec: structs::HostVirtualNicSpec::default(),
            port: None,
            owner: None,
        }
    }
}

impl Default for structs::HostVirtualNicConfig {
    fn default() -> Self {
        Self {
            change_operation: None,
            device: None,
            portgroup: String::new(),
            spec: None,
        }
    }
}

impl Default for structs::HostVirtualNicIpRouteSpec {
    fn default() -> Self {
        Self {
            ip_route_config: None,
        }
    }
}

impl Default for structs::HostVirtualNicOpaqueNetworkSpec {
    fn default() -> Self {
        Self {
            opaque_network_id: String::new(),
            opaque_network_type: String::new(),
        }
    }
}

impl Default for structs::HostVirtualNicSpec {
    fn default() -> Self {
        Self {
            dynamic_property: None,
            ip: None,
            mac: None,
            distributed_virtual_port: None,
            portgroup: None,
            mtu: None,
            tso_enabled: None,
            net_stack_instance_key: None,
            opaque_network: None,
            external_id: None,
            pinned_pnic: None,
            ip_route_spec: None,
            system_owned: None,
            dpu_id: None,
        }
    }
}

impl Default for structs::HostVirtualNicConnection {
    fn default() -> Self {
        Self {
            portgroup: None,
            dv_port: None,
            op_network: None,
        }
    }
}

impl Default for structs::VirtualNicManagerNetConfig {
    fn default() -> Self {
        Self {
            nic_type: String::new(),
            multi_select_allowed: false,
            candidate_vnic: None,
            selected_vnic: None,
        }
    }
}

impl Default for structs::HostVirtualNicManagerNicTypeSelection {
    fn default() -> Self {
        Self {
            vnic: structs::HostVirtualNicConnection::default(),
            nic_type: None,
        }
    }
}

impl Default for structs::HostVirtualNicManagerInfo {
    fn default() -> Self {
        Self {
            net_config: None,
        }
    }
}

impl Default for structs::HostVirtualSwitch {
    fn default() -> Self {
        Self {
            name: String::new(),
            key: String::new(),
            num_ports: 0,
            num_ports_available: 0,
            mtu: None,
            portgroup: None,
            pnic: None,
            spec: structs::HostVirtualSwitchSpec::default(),
        }
    }
}

impl Default for structs::HostVirtualSwitchBeaconConfig {
    fn default() -> Self {
        Self {
            interval: 0,
        }
    }
}

impl Default for structs::HostVirtualSwitchBridge {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::HostVirtualSwitchAutoBridge {
    fn default() -> Self {
        Self {
            excluded_nic_device: None,
        }
    }
}

impl Default for structs::HostVirtualSwitchBondBridge {
    fn default() -> Self {
        Self {
            nic_device: Vec::new(),
            beacon: None,
            link_discovery_protocol_config: None,
        }
    }
}

impl Default for structs::HostVirtualSwitchSimpleBridge {
    fn default() -> Self {
        Self {
            nic_device: String::new(),
        }
    }
}

impl Default for structs::HostVirtualSwitchConfig {
    fn default() -> Self {
        Self {
            change_operation: None,
            name: String::new(),
            spec: None,
        }
    }
}

impl Default for structs::HostVirtualSwitchSpec {
    fn default() -> Self {
        Self {
            num_ports: 0,
            bridge: None,
            policy: None,
            mtu: None,
        }
    }
}

impl Default for structs::HostVmciAccessManagerAccessSpec {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            services: None,
            mode: String::new(),
        }
    }
}

impl Default for structs::VmfsDatastoreOption {
    fn default() -> Self {
        Self {
            info: Default::default(),
            spec: Default::default(),
        }
    }
}

impl Default for structs::VmfsDatastoreBaseOption {
    fn default() -> Self {
        Self {
            layout: structs::HostDiskPartitionLayout::default(),
            partition_format_change: None,
        }
    }
}

impl Default for structs::VmfsDatastoreMultipleExtentOption {
    fn default() -> Self {
        Self {
            vmfs_datastore_base_option_: structs::VmfsDatastoreBaseOption::default(),
            vmfs_extent: Vec::new(),
        }
    }
}

impl Default for structs::VmfsDatastoreSingleExtentOption {
    fn default() -> Self {
        Self {
            vmfs_datastore_base_option_: structs::VmfsDatastoreBaseOption::default(),
            vmfs_extent: structs::HostDiskPartitionBlockRange::default(),
        }
    }
}

impl Default for structs::VmfsDatastoreAllExtentOption {
    fn default() -> Self {
        Self {
            vmfs_datastore_single_extent_option_: structs::VmfsDatastoreSingleExtentOption::default(),
        }
    }
}

impl Default for structs::VmfsDatastoreSpec {
    fn default() -> Self {
        Self {
            disk_uuid: String::new(),
        }
    }
}

impl Default for structs::VmfsDatastoreCreateSpec {
    fn default() -> Self {
        Self {
            vmfs_datastore_spec_: structs::VmfsDatastoreSpec::default(),
            partition: structs::HostDiskPartitionSpec::default(),
            vmfs: structs::HostVmfsSpec::default(),
            extent: None,
        }
    }
}

impl Default for structs::VmfsDatastoreExpandSpec {
    fn default() -> Self {
        Self {
            vmfs_datastore_spec_: structs::VmfsDatastoreSpec::default(),
            partition: structs::HostDiskPartitionSpec::default(),
            extent: structs::HostScsiDiskPartition::default(),
        }
    }
}

impl Default for structs::VmfsDatastoreExtendSpec {
    fn default() -> Self {
        Self {
            vmfs_datastore_spec_: structs::VmfsDatastoreSpec::default(),
            partition: structs::HostDiskPartitionSpec::default(),
            extent: Vec::new(),
        }
    }
}

impl Default for structs::HostVmfsRescanResult {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            fault: None,
        }
    }
}

impl Default for structs::VmfsConfigOption {
    fn default() -> Self {
        Self {
            block_size_option: 0,
            unmap_granularity_option: None,
            unmap_bandwidth_fixed_value: None,
            unmap_bandwidth_dynamic_min: None,
            unmap_bandwidth_dynamic_max: None,
            unmap_bandwidth_increment: None,
            unmap_bandwidth_ultra_low: None,
        }
    }
}

impl Default for structs::HostVmfsSpec {
    fn default() -> Self {
        Self {
            extent: structs::HostScsiDiskPartition::default(),
            block_size_mb: None,
            major_version: 0,
            volume_name: String::new(),
            block_size: None,
            unmap_granularity: None,
            unmap_priority: None,
            unmap_bandwidth_spec: None,
        }
    }
}

impl Default for structs::VmfsUnmapBandwidthSpec {
    fn default() -> Self {
        Self {
            policy: String::new(),
            fixed_value: 0,
            dynamic_min: 0,
            dynamic_max: 0,
        }
    }
}

impl Default for structs::VsanBasicDeviceInfo {
    fn default() -> Self {
        Self {
            device_name: String::new(),
            pci_id: None,
            fw_version: None,
            features: None,
        }
    }
}

impl Default for structs::VsanClusterMembershipInfo {
    fn default() -> Self {
        Self {
            cluster_uuid: None,
            health: None,
            membership_uuid: None,
            member_uuid: None,
        }
    }
}

impl Default for structs::VsanDaemonHealth {
    fn default() -> Self {
        Self {
            name: String::new(),
            alive: false,
            error: None,
        }
    }
}

impl Default for structs::VsanDiskEncryptionHealth {
    fn default() -> Self {
        Self {
            disk_health: None,
            encryption_issues: None,
        }
    }
}

impl Default for structs::VsanDiskRebalanceResult {
    fn default() -> Self {
        Self {
            status: String::new(),
            bytes_moving: None,
            remaining_bytes_to_move: None,
            disk_usage: None,
            max_disk_usage: None,
            min_disk_usage: None,
            avg_disk_usage: None,
            disk_comp_usage: None,
            max_disk_comp_usage: None,
            min_disk_comp_usage: None,
            avg_disk_comp_usage: None,
        }
    }
}

impl Default for structs::VsanDitEncryptionHealthSummary {
    fn default() -> Self {
        Self {
            hostname: None,
            health: None,
            reason: None,
            dit_encryption_info: None,
        }
    }
}

impl Default for structs::VsanEncryptionHealthSummary {
    fn default() -> Self {
        Self {
            hostname: None,
            encryption_info: None,
            overall_kms_health: String::new(),
            kms_health: None,
            encryption_issues: None,
            disk_results: None,
            error: None,
            aesni_enabled: None,
            inconsistently_encrypted_object_count: None,
            host_encryption_dek_id: None,
            kek_verifier_health: None,
            dek_verifier_health: None,
        }
    }
}

impl Default for structs::VsanFailedRepairObjectResult {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            err_message: None,
        }
    }
}

impl Default for structs::VsanFileServerHealthSummary {
    fn default() -> Self {
        Self {
            domain_name: None,
            file_server_ip: None,
            nfsd_health: None,
            network_health: None,
            rootfs_health: None,
            description: None,
            smb_connections: None,
            smb_daemon_health: None,
            ad_test_join_health: None,
            dns_lookup_health: None,
        }
    }
}

impl Default for structs::VsanFileServiceBalanceHealth {
    fn default() -> Self {
        Self {
            health: None,
            description: None,
        }
    }
}

impl Default for structs::VsanFileServiceHealthSummary {
    fn default() -> Self {
        Self {
            hostname: None,
            overall_health: None,
            enabled: None,
            vdfsd_status: None,
            fsvm_status: None,
            root_fs_status: None,
            file_server_health: None,
            file_share_health: None,
            balance_status: None,
            host_load_status: None,
        }
    }
}

impl Default for structs::VsanFileServiceRootFsHealth {
    fn default() -> Self {
        Self {
            created: None,
            health: None,
            description: None,
        }
    }
}

impl Default for structs::VsanFileServiceShareHealthSummary {
    fn default() -> Self {
        Self {
            overall_health: None,
            domain_name: None,
            share_uuid: None,
            share_name: None,
            object_health: None,
            description: None,
            extensible: None,
            spbm_profile_uuid: None,
            spbm_profile_generation_id: None,
            share_policy_mismatch: None,
        }
    }
}

impl Default for structs::VsanHclCommonDeviceInfo {
    fn default() -> Self {
        Self {
            device_name: String::new(),
            display_name: None,
            driver_name: None,
            driver_version: None,
            vendor_id: None,
            device_id: None,
            sub_vendor_id: None,
            sub_device_id: None,
            extra_info: None,
            device_on_hcl: None,
            release_supported: None,
            releases_on_hcl: None,
            driver_versions_on_hcl: None,
            driver_version_supported: None,
            fw_version_supported: None,
            fw_version_on_hcl: None,
            fw_version: None,
            drivers_on_hcl: None,
        }
    }
}

impl Default for structs::VsanHclNicInfo {
    fn default() -> Self {
        Self {
            vsan_hcl_common_device_info_: structs::VsanHclCommonDeviceInfo::default(),
            vmknic: None,
            use_by_vsan: None,
            rdma_config: None,
            vsan_host_compatibility: None,
            nic_link_speed_in_mbps: None,
        }
    }
}

impl Default for structs::VsanHclComputeResource {
    fn default() -> Self {
        Self {
            memory: structs::VsanHclMemInfo::default(),
        }
    }
}

impl Default for structs::VsanHclControllerInfo {
    fn default() -> Self {
        Self {
            device_name: String::new(),
            device_display_name: None,
            driver_name: None,
            driver_version: None,
            vendor_id: None,
            device_id: None,
            sub_vendor_id: None,
            sub_device_id: None,
            extra_info: None,
            device_on_hcl: None,
            release_supported: None,
            releases_on_hcl: None,
            driver_versions_on_hcl: None,
            driver_version_supported: None,
            fw_version_supported: None,
            fw_version_on_hcl: None,
            cache_config_supported: None,
            cache_config_on_hcl: None,
            raid_config_supported: None,
            raid_config_on_hcl: None,
            fw_version: None,
            raid_config: None,
            cache_config: None,
            cim_provider_info: None,
            used_by_vsan: None,
            disks: None,
            issues: None,
            remediable_issues: None,
            drivers_on_hcl: None,
            fw_aux_version: None,
            queue_depth: None,
            queue_depth_on_hcl: None,
            queue_depth_supported: None,
            disk_mode: None,
            disk_mode_on_hcl: None,
            disk_mode_supported: None,
            tool_name: None,
            tool_version: None,
            product_id: None,
            disk_capacity: None,
            vcg_entry_info: None,
            controller_type: None,
            user_selected_vcg_id: None,
            vsan_compatibility: None,
        }
    }
}

impl Default for structs::VsanHclDiskInfo {
    fn default() -> Self {
        Self {
            device_name: String::new(),
            model: None,
            is_ssd: None,
            vsan_disk: false,
            issues: None,
            remediable_issues: None,
            uuid: None,
            capacity: None,
            vsan_compatibility: None,
        }
    }
}

impl Default for structs::VsanHclFirmwareFile {
    fn default() -> Self {
        Self {
            file_type: String::new(),
            filename_or_url: String::new(),
            sha_1_sum: String::new(),
        }
    }
}

impl Default for structs::VsanHclFirmwareUpdateSpec {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            hba_device: String::new(),
            fw_files: Vec::new(),
            allow_downgrade: None,
            firmware_component: None,
        }
    }
}

impl Default for structs::VsanHclMemInfo {
    fn default() -> Self {
        Self {
            memory_size: None,
            vsan_host_compatibility: None,
        }
    }
}

impl Default for structs::VsanHealthQuerySpec {
    fn default() -> Self {
        Self {
            include_all_remote_clusters: None,
            remote_cluster_uuids: None,
            latency_only: None,
            mode: None,
        }
    }
}

impl Default for structs::VsanHostCimProviderInfo {
    fn default() -> Self {
        Self {
            cim_provider_supported: None,
            installed_cim_provider: None,
            cim_provider_on_hcl: None,
            cim_provider_links_on_hcl: None,
        }
    }
}

impl Default for structs::VsanHostEmmSummary {
    fn default() -> Self {
        Self {
            hostname: None,
            in_maintenance_mode: None,
            in_decom_state: None,
        }
    }
}

impl Default for structs::VsanHostFwComponent {
    fn default() -> Self {
        Self {
            name: String::new(),
            url: None,
            sha_1_sum: None,
            current_version: None,
            suggested_version: None,
            component_id: None,
        }
    }
}

impl Default for structs::VsanHostGlobalDedupConfigHealthSummary {
    fn default() -> Self {
        Self {
            hostname: None,
            health: None,
        }
    }
}

impl Default for structs::VsanHostHclInfo {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            hcl_checked: false,
            release_name: None,
            error: None,
            controllers: None,
            pnics: None,
            host: None,
            compute_resource: None,
            vsan_host_compatibility: None,
        }
    }
}

impl Default for structs::VsanHostHealthSystemStatusResult {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            status: String::new(),
            issues: None,
        }
    }
}

impl Default for structs::VsanHostHwDeviceId {
    fn default() -> Self {
        Self {
            pci_id: structs::DevicePciId::default(),
            product_id: None,
            disk_capacity: None,
        }
    }
}

impl Default for structs::VsanHostIoInsightInfo {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            ioinsight_world_id: None,
            fault_message: None,
            ioinsight_info: None,
        }
    }
}

impl Default for structs::VsanHostQueryCheckLimitsSpec {
    fn default() -> Self {
        Self {
            option_types: None,
            fetch_all: false,
        }
    }
}

impl Default for structs::VsanHostReference {
    fn default() -> Self {
        Self {
            hostname: String::new(),
        }
    }
}

impl Default for structs::VsanHostVirtualApplianceInfo {
    fn default() -> Self {
        Self {
            host_key: structs::ManagedObjectReference::default(),
            is_virtual_app: false,
            is_deployed_from_ovf: None,
        }
    }
}

impl Default for structs::VsanHostVmdkLoadTestResult {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            issue_found: false,
            fault_message: None,
            vmdk_results: None,
        }
    }
}

impl Default for structs::VsanHwToVcgInfoMapping {
    fn default() -> Self {
        Self {
            vsan_host_hw_device_id: structs::VsanHostHwDeviceId::default(),
            vcg_id: 0,
        }
    }
}

impl Default for structs::HostVsanInternalSystemCmmdsQuery {
    fn default() -> Self {
        Self {
            r#type: None,
            uuid: None,
            owner: None,
        }
    }
}

impl Default for structs::HostVsanInternalSystemDeleteVsanObjectsResult {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            success: false,
            failure_reason: None,
        }
    }
}

impl Default for structs::VsanNewPolicyBatch {
    fn default() -> Self {
        Self {
            size: None,
            policy: None,
        }
    }
}

impl Default for structs::VsanPolicyChangeBatch {
    fn default() -> Self {
        Self {
            uuid: None,
            policy: None,
        }
    }
}

impl Default for structs::VsanPolicyCost {
    fn default() -> Self {
        Self {
            change_data_size: None,
            current_data_size: None,
            temp_data_size: None,
            copy_data_size: None,
            change_flash_read_cache_size: None,
            current_flash_read_cache_size: None,
            current_disk_space_to_address_space_ratio: None,
            disk_space_to_address_space_ratio: None,
        }
    }
}

impl Default for structs::VsanPolicySatisfiability {
    fn default() -> Self {
        Self {
            uuid: None,
            is_satisfiable: false,
            reason: None,
            cost: None,
        }
    }
}

impl Default for structs::HostVsanInternalSystemVsanObjectOperationResult {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            failure_reason: None,
        }
    }
}

impl Default for structs::HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult {
    fn default() -> Self {
        Self {
            disk_uuid: String::new(),
            success: false,
            failure_reason: None,
        }
    }
}

impl Default for structs::VsanIoInsightInfo {
    fn default() -> Self {
        Self {
            state: None,
            monitored_v_ms: None,
        }
    }
}

impl Default for structs::VsanIperfClientSpec {
    fn default() -> Self {
        Self {
            reverse: false,
        }
    }
}

impl Default for structs::VsanKmsHealth {
    fn default() -> Self {
        Self {
            server_name: String::new(),
            health: String::new(),
            error: None,
            trust_health: None,
            cert_health: None,
            cert_expire_date: None,
        }
    }
}

impl Default for structs::VsanLimitHealthResult {
    fn default() -> Self {
        Self {
            hostname: None,
            issue_found: false,
            max_components: 0,
            free_components: 0,
            component_limit_health: String::new(),
            lowest_free_disk_space_pct: 0,
            used_disk_space_b: 0,
            total_disk_space_b: 0,
            disk_free_space_health: String::new(),
            reserved_rc_size_b: 0,
            total_rc_size_b: 0,
            rc_free_reservation_health: String::new(),
            total_logical_space_b: None,
            logical_space_used_b: None,
            dedup_metadata_size_b: None,
            disk_transient_capacity_used_b: None,
            dg_transient_capacity_used_b: None,
            slack_space_cap_required: None,
            resync_pause_threshold: None,
            space_efficiency_metadata_size_b: None,
            host_rebuild_capacity: None,
            min_space_required_for_vsan_op: None,
            enforce_cap_resrv_space: None,
            cd_reserved_size_b: None,
        }
    }
}

impl Default for structs::VsanNetworkDiagnosticsHealthInfo {
    fn default() -> Self {
        Self {
            vnic_info: None,
            pnic_tso_info: None,
            lacp_info: None,
        }
    }
}

impl Default for structs::VsanNetworkHealthResult {
    fn default() -> Self {
        Self {
            host: None,
            hostname: None,
            vsan_vmknic_present: None,
            ip_subnets: None,
            issue_found: None,
            peer_health: None,
            v_motion_health: None,
            multicast_config: None,
            unicast_config: None,
            in_unicast: None,
            rdma_enabled: None,
            rdt_conn_protocol: None,
            server_clusters: None,
            external_peer_health: None,
        }
    }
}

impl Default for structs::VsanNetworkLoadTestResult {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            status: None,
            client: false,
            bandwidth_bps: 0,
            total_bytes: 0,
            lost_datagrams: None,
            loss_pct: None,
            sent_datagrams: None,
            jitter_ms: None,
        }
    }
}

impl Default for structs::VsanNetworkPeerHealthResult {
    fn default() -> Self {
        Self {
            peer: None,
            peer_hostname: None,
            peer_vmknic_name: None,
            small_ping_test_success_pct: None,
            large_ping_test_success_pct: None,
            max_latency_us: None,
            on_same_ip_subnet: None,
            source_vmknic_name: None,
            connectivity_health_state: None,
            missing_heart_beat_count: None,
        }
    }
}

impl Default for structs::VsanNicRdmaInfo {
    fn default() -> Self {
        Self {
            rdma_capable: None,
            rdma_protocol_capable: None,
            dcb_enabled: None,
            dcb_mode: None,
            pfc_enabled: None,
            pfc_config: None,
        }
    }
}

impl Default for structs::VsanObjectHealth {
    fn default() -> Self {
        Self {
            num_objects: 0,
            health: None,
            obj_uuids: None,
            vsan_cluster_uuid: None,
        }
    }
}

impl Default for structs::VsanObjectOverallHealth {
    fn default() -> Self {
        Self {
            object_health_detail: None,
            objects_compliance_detail: None,
            object_version_compliance: None,
            object_format_change_required_uuids: None,
            objects_relayout_bytes: None,
            global_dedup_store_health: None,
        }
    }
}

impl Default for structs::VsanPhysicalDiskHealth {
    fn default() -> Self {
        Self {
            name: String::new(),
            uuid: String::new(),
            in_cmmds: false,
            in_vsi: false,
            dedup_scope: None,
            format_version: None,
            is_all_flash: None,
            congestion_value: None,
            congestion_area: None,
            congestion_health: None,
            metadata_health: None,
            operational_health_description: None,
            operational_health: None,
            dedup_usage_health: None,
            capacity_health: None,
            summary_health: String::new(),
            capacity: None,
            used_capacity: None,
            reserved_capacity: None,
            total_bytes: None,
            free_bytes: None,
            hashed_bytes: None,
            deduped_bytes: None,
            scsi_disk: None,
            used_components: None,
            max_components: None,
            comp_limit_health: None,
            encryption_enabled: None,
            kms_provider_id: None,
            kek_id: None,
            dek_generation_id: None,
            encrypted_unlocked: None,
            rebalance_result: None,
            dek_id: None,
            kek_verifier_health: None,
            dek_verifier_health: None,
            logical_capacity: None,
            logical_capacity_used: None,
            logical_capacity_health: None,
            vsan_disk_group_uuid: None,
            dg_layout_issue: None,
            used_metadata_components: None,
            max_metadata_components: None,
            pending_cluster_dek_id: None,
            dmek_verifier_health: None,
        }
    }
}

impl Default for structs::VsanPhysicalDiskHealthSummary {
    fn default() -> Self {
        Self {
            overall_health: String::new(),
            heaps_with_issues: None,
            slabs_with_issues: None,
            disks: None,
            components_with_issues: None,
            hostname: None,
            host_dedup_scope: None,
            error: None,
        }
    }
}

impl Default for structs::VsanProactiveRebalanceInfoEx {
    fn default() -> Self {
        Self {
            running: None,
            start_ts: None,
            stop_ts: None,
            variance_threshold: None,
            time_threshold: None,
            rate_threshold: None,
            hostname: None,
            error: None,
        }
    }
}

impl Default for structs::VsanQueryResultHostInfo {
    fn default() -> Self {
        Self {
            uuid: None,
            hostname_in_cmmds: None,
            vsan_ipv_4_addresses: None,
        }
    }
}

impl Default for structs::VsanRepairObjectsResult {
    fn default() -> Self {
        Self {
            in_queue_objects: None,
            failed_repair_objects: None,
            not_in_queue_objects: None,
        }
    }
}

impl Default for structs::VsanResourceHealth {
    fn default() -> Self {
        Self {
            resource: String::new(),
            health: String::new(),
            description: None,
        }
    }
}

impl Default for structs::VsanServerClusterInfo {
    fn default() -> Self {
        Self {
            cluster: None,
            peer_health: None,
            membership: None,
        }
    }
}

impl Default for structs::VsanSmartDiskStats {
    fn default() -> Self {
        Self {
            disk: String::new(),
            stats: None,
            error: None,
        }
    }
}

impl Default for structs::VsanSmartParameter {
    fn default() -> Self {
        Self {
            parameter: None,
            value: None,
            threshold: None,
            worst: None,
        }
    }
}

impl Default for structs::VsanSmartStatsHostSummary {
    fn default() -> Self {
        Self {
            hostname: None,
            smart_stats: None,
        }
    }
}

impl Default for structs::VsanVcgDeviceInfo {
    fn default() -> Self {
        Self {
            vcg_id: 0,
            vcg_model_name: None,
        }
    }
}

impl Default for structs::VsanVmdkIoLoadSpec {
    fn default() -> Self {
        Self {
            read_pct: 0,
            oio: 0,
            iosize_b: 0,
            data_size_mb: 0,
            random: false,
            start_offset_b: None,
        }
    }
}

impl Default for structs::VsanVmdkLoadTestResult {
    fn default() -> Self {
        Self {
            success: false,
            fault_message: None,
            spec: structs::VsanVmdkLoadTestSpec::default(),
            actual_duration_sec: None,
            total_bytes: None,
            iops: None,
            tput_bps: None,
            avg_latency_us: None,
            max_latency_us: None,
            num_io_above_latency_threshold: None,
        }
    }
}

impl Default for structs::VsanVmdkLoadTestSpec {
    fn default() -> Self {
        Self {
            vmdk_create_spec: None,
            vmdk_io_spec: None,
            vmdk_io_spec_sequence: None,
            step_duration_sec: None,
        }
    }
}

impl Default for structs::VsanVsanPcapResult {
    fn default() -> Self {
        Self {
            calltime: 0.0,
            vmknic: String::new(),
            tcpdump_filter: String::new(),
            snaplen: 0,
            pkts: None,
            pcap: None,
            error: None,
            hostname: None,
        }
    }
}

impl Default for structs::HostVvolNqn {
    fn default() -> Self {
        Self {
            target_nqn: String::new(),
            storage_array: String::new(),
            online: false,
        }
    }
}

impl Default for structs::VVolHostPe {
    fn default() -> Self {
        Self {
            key: structs::ManagedObjectReference::default(),
            protocol_endpoint: Vec::new(),
        }
    }
}

impl Default for structs::HostVvolVolumeHostVvolNqn {
    fn default() -> Self {
        Self {
            host: None,
            vvol_nqn: None,
        }
    }
}

impl Default for structs::HostVvolVolumeSpecification {
    fn default() -> Self {
        Self {
            max_size_in_mb: 0,
            volume_name: String::new(),
            vasa_provider_info: None,
            storage_array: None,
            uuid: String::new(),
            stretched: None,
        }
    }
}

impl Default for structs::NetDhcpConfigInfo {
    fn default() -> Self {
        Self {
            ipv_6: None,
            ipv_4: None,
        }
    }
}

impl Default for structs::NetDhcpConfigInfoDhcpOptions {
    fn default() -> Self {
        Self {
            enable: false,
            config: None,
        }
    }
}

impl Default for structs::NetDhcpConfigSpec {
    fn default() -> Self {
        Self {
            ipv_6: None,
            ipv_4: None,
        }
    }
}

impl Default for structs::NetDhcpConfigSpecDhcpOptionsSpec {
    fn default() -> Self {
        Self {
            enable: None,
            config: Vec::new(),
            operation: String::new(),
        }
    }
}

impl Default for structs::NetDnsConfigInfo {
    fn default() -> Self {
        Self {
            dhcp: false,
            host_name: String::new(),
            domain_name: String::new(),
            ip_address: None,
            search_domain: None,
        }
    }
}

impl Default for structs::NetDnsConfigSpec {
    fn default() -> Self {
        Self {
            dhcp: None,
            host_name: None,
            domain_name: None,
            ip_address: None,
            search_domain: None,
        }
    }
}

impl Default for structs::NetIpConfigInfo {
    fn default() -> Self {
        Self {
            ip_address: None,
            dhcp: None,
            auto_configuration_enabled: None,
        }
    }
}

impl Default for structs::NetIpConfigInfoIpAddress {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            prefix_length: 0,
            origin: None,
            state: None,
            lifetime: None,
        }
    }
}

impl Default for structs::NetIpConfigSpec {
    fn default() -> Self {
        Self {
            ip_address: None,
            dhcp: None,
            auto_configuration_enabled: None,
        }
    }
}

impl Default for structs::NetIpConfigSpecIpAddressSpec {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            prefix_length: 0,
            operation: String::new(),
        }
    }
}

impl Default for structs::NetIpRouteConfigInfo {
    fn default() -> Self {
        Self {
            ip_route: None,
        }
    }
}

impl Default for structs::NetIpRouteConfigInfoGateway {
    fn default() -> Self {
        Self {
            ip_address: None,
            device: None,
        }
    }
}

impl Default for structs::NetIpRouteConfigInfoIpRoute {
    fn default() -> Self {
        Self {
            network: String::new(),
            prefix_length: 0,
            gateway: structs::NetIpRouteConfigInfoGateway::default(),
        }
    }
}

impl Default for structs::NetIpRouteConfigSpec {
    fn default() -> Self {
        Self {
            ip_route: None,
        }
    }
}

impl Default for structs::NetIpRouteConfigSpecGatewaySpec {
    fn default() -> Self {
        Self {
            ip_address: None,
            device: None,
        }
    }
}

impl Default for structs::NetIpRouteConfigSpecIpRouteSpec {
    fn default() -> Self {
        Self {
            network: String::new(),
            prefix_length: 0,
            gateway: structs::NetIpRouteConfigSpecGatewaySpec::default(),
            operation: String::new(),
        }
    }
}

impl Default for structs::NetIpStackInfo {
    fn default() -> Self {
        Self {
            neighbor: None,
            default_router: None,
        }
    }
}

impl Default for structs::NetIpStackInfoDefaultRouter {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            device: String::new(),
            lifetime: String::new(),
            preference: String::new(),
        }
    }
}

impl Default for structs::NetIpStackInfoNetToMedia {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            physical_address: String::new(),
            device: String::new(),
            r#type: String::new(),
        }
    }
}

impl Default for structs::NetBiosConfigInfo {
    fn default() -> Self {
        Self {
            mode: String::new(),
        }
    }
}

impl Default for structs::WinNetBiosConfigInfo {
    fn default() -> Self {
        Self {
            net_bios_config_info_: structs::NetBiosConfigInfo::default(),
            primary_wins: String::new(),
            secondary_wins: None,
        }
    }
}

impl Default for structs::ArrayUpdateSpec {
    fn default() -> Self {
        Self {
            operation: enums::ArrayUpdateOperationEnum::default(),
            remove_key: None,
        }
    }
}

impl Default for structs::ClusterDasVmConfigSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::ClusterDatastoreUpdateSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            datastore: None,
        }
    }
}

impl Default for structs::ClusterDpmHostConfigSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::ClusterDrsVmConfigSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::ClusterGroupSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::ClusterPreemptibleVmPairSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::ClusterRuleSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::ClusterTagCategoryUpdateSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            category: None,
        }
    }
}

impl Default for structs::ClusterVmOrchestrationSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::StorageDrsOptionSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            option: None,
        }
    }
}

impl Default for structs::StorageDrsVmConfigSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::VAppOvfSectionSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::VAppProductSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::VAppPropertySpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::VirtualMachineCpuIdInfoSpec {
    fn default() -> Self {
        Self {
            array_update_spec_: structs::ArrayUpdateSpec::default(),
            info: None,
        }
    }
}

impl Default for structs::OptionType {
    fn default() -> Self {
        Self {
            value_is_readonly: None,
        }
    }
}

impl Default for structs::BoolOption {
    fn default() -> Self {
        Self {
            option_type_: structs::OptionType::default(),
            supported: false,
            default_value: false,
        }
    }
}

impl Default for structs::ChoiceOption {
    fn default() -> Self {
        Self {
            option_type_: structs::OptionType::default(),
            choice_info: Vec::new(),
            default_index: None,
        }
    }
}

impl Default for structs::FloatOption {
    fn default() -> Self {
        Self {
            option_type_: structs::OptionType::default(),
            min: 0.0,
            max: 0.0,
            default_value: 0.0,
        }
    }
}

impl Default for structs::IntOption {
    fn default() -> Self {
        Self {
            option_type_: structs::OptionType::default(),
            min: 0,
            max: 0,
            default_value: 0,
        }
    }
}

impl Default for structs::LongOption {
    fn default() -> Self {
        Self {
            option_type_: structs::OptionType::default(),
            min: 0,
            max: 0,
            default_value: 0,
        }
    }
}

impl Default for structs::StringOption {
    fn default() -> Self {
        Self {
            option_type_: structs::OptionType::default(),
            default_value: String::new(),
            valid_characters: None,
        }
    }
}

impl Default for structs::OptionValue {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: None,
        }
    }
}

impl Default for structs::HostInternetScsiHbaParamValue {
    fn default() -> Self {
        Self {
            option_value_: structs::OptionValue::default(),
            is_inherited: None,
        }
    }
}

impl Default for structs::ApplyProfile {
    fn default() -> Self {
        Self {
            enabled: false,
            policy: None,
            profile_type_name: None,
            profile_version: None,
            property: None,
            favorite: None,
            to_be_merged: None,
            to_replace_with: None,
            to_be_deleted: None,
            copy_enable_status: None,
            hidden: None,
        }
    }
}

impl Default for structs::ProfileApplyProfileElement {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::ActiveDirectoryProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
        }
    }
}

impl Default for structs::AuthenticationProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            active_directory: None,
        }
    }
}

impl Default for structs::DateTimeProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
        }
    }
}

impl Default for structs::DvsProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
            name: String::new(),
            uplink: None,
        }
    }
}

impl Default for structs::DvsVNicProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
            ip_config: structs::IpAddressProfile::default(),
        }
    }
}

impl Default for structs::DvsHostVNicProfile {
    fn default() -> Self {
        Self {
            dvs_v_nic_profile_: structs::DvsVNicProfile::default(),
        }
    }
}

impl Default for structs::DvsServiceConsoleVNicProfile {
    fn default() -> Self {
        Self {
            dvs_v_nic_profile_: structs::DvsVNicProfile::default(),
        }
    }
}

impl Default for structs::FirewallProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            ruleset: None,
        }
    }
}

impl Default for structs::FirewallProfileRulesetProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::HostApplyProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            memory: None,
            storage: None,
            network: None,
            datetime: None,
            firewall: None,
            security: None,
            service: None,
            option: None,
            user_account: None,
            usergroup_account: None,
            authentication: None,
        }
    }
}

impl Default for structs::HostMemoryProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
        }
    }
}

impl Default for structs::IpAddressProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
        }
    }
}

impl Default for structs::IpRouteProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            static_route: None,
        }
    }
}

impl Default for structs::NasStorageProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::NetStackInstanceProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
            dns_config: structs::NetworkProfileDnsConfigProfile::default(),
            ip_route_config: structs::IpRouteProfile::default(),
        }
    }
}

impl Default for structs::NetworkPolicyProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
        }
    }
}

impl Default for structs::NetworkProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            vswitch: None,
            vm_port_group: None,
            host_port_group: None,
            service_console_port_group: None,
            dns_config: None,
            ip_route_config: None,
            console_ip_route_config: None,
            pnic: None,
            dvswitch: None,
            dvs_service_console_nic: None,
            dvs_host_nic: None,
            nsx_host_nic: None,
            net_stack_instance: None,
            opaque_switch: None,
        }
    }
}

impl Default for structs::NetworkProfileDnsConfigProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
        }
    }
}

impl Default for structs::NsxHostVNicProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
            ip_config: structs::IpAddressProfile::default(),
        }
    }
}

impl Default for structs::OpaqueSwitchProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
        }
    }
}

impl Default for structs::OptionProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::PermissionProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::PhysicalNicProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::PnicUplinkProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::PortGroupProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
            name: String::new(),
            vlan: structs::VlanProfile::default(),
            vswitch: structs::VirtualSwitchSelectionProfile::default(),
            network_policy: structs::NetworkPolicyProfile::default(),
        }
    }
}

impl Default for structs::HostPortGroupProfile {
    fn default() -> Self {
        Self {
            port_group_profile_: structs::PortGroupProfile::default(),
            ip_config: structs::IpAddressProfile::default(),
        }
    }
}

impl Default for structs::ServiceConsolePortGroupProfile {
    fn default() -> Self {
        Self {
            port_group_profile_: structs::PortGroupProfile::default(),
            ip_config: structs::IpAddressProfile::default(),
        }
    }
}

impl Default for structs::VmPortGroupProfile {
    fn default() -> Self {
        Self {
            port_group_profile_: structs::PortGroupProfile::default(),
        }
    }
}

impl Default for structs::VirtualSwitchSelectionProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
        }
    }
}

impl Default for structs::VlanProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
        }
    }
}

impl Default for structs::SecurityProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            permission: None,
        }
    }
}

impl Default for structs::ServiceProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::StaticRouteProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: None,
        }
    }
}

impl Default for structs::StorageProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            nas_storage: None,
        }
    }
}

impl Default for structs::UserGroupProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::UserProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
        }
    }
}

impl Default for structs::VirtualSwitchProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
            key: String::new(),
            name: String::new(),
            link: structs::LinkProfile::default(),
            num_ports: structs::NumPortsProfile::default(),
            network_policy: structs::NetworkPolicyProfile::default(),
        }
    }
}

impl Default for structs::LinkProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
        }
    }
}

impl Default for structs::NumPortsProfile {
    fn default() -> Self {
        Self {
            apply_profile_: structs::ApplyProfile::default(),
        }
    }
}

impl Default for structs::ProfileApplyProfileProperty {
    fn default() -> Self {
        Self {
            property_name: String::new(),
            array: false,
            profile: None,
        }
    }
}

impl Default for structs::ComplianceLocator {
    fn default() -> Self {
        Self {
            expression_name: String::new(),
            apply_path: structs::ProfilePropertyPath::default(),
        }
    }
}

impl Default for structs::ComplianceProfile {
    fn default() -> Self {
        Self {
            expression: Vec::new(),
            root_expression: String::new(),
        }
    }
}

impl Default for structs::ComplianceResult {
    fn default() -> Self {
        Self {
            profile: None,
            compliance_status: String::new(),
            entity: None,
            check_time: None,
            failure: None,
        }
    }
}

impl Default for structs::ComplianceFailure {
    fn default() -> Self {
        Self {
            failure_type: String::new(),
            message: structs::LocalizableMessage::default(),
            expression_name: None,
            failure_values: None,
        }
    }
}

impl Default for structs::ComplianceFailureComplianceFailureValues {
    fn default() -> Self {
        Self {
            comparison_identifier: String::new(),
            profile_instance: None,
            host_value: None,
            profile_value: None,
        }
    }
}

impl Default for structs::ProfileDeferredPolicyOptionParameter {
    fn default() -> Self {
        Self {
            input_path: structs::ProfilePropertyPath::default(),
            parameter: None,
        }
    }
}

impl Default for structs::ProfileExpression {
    fn default() -> Self {
        Self {
            id: String::new(),
            display_name: String::new(),
            negated: false,
        }
    }
}

impl Default for structs::ProfileCompositeExpression {
    fn default() -> Self {
        Self {
            profile_expression_: structs::ProfileExpression::default(),
            operator: String::new(),
            expression_name: Vec::new(),
        }
    }
}

impl Default for structs::ProfileSimpleExpression {
    fn default() -> Self {
        Self {
            profile_expression_: structs::ProfileExpression::default(),
            expression_type: String::new(),
            parameter: None,
        }
    }
}

impl Default for structs::ProfileExpressionMetadata {
    fn default() -> Self {
        Self {
            expression_id: structs::ExtendedElementDescription::default(),
            parameter: None,
        }
    }
}

impl Default for structs::ProfileParameterMetadata {
    fn default() -> Self {
        Self {
            id: structs::ExtendedElementDescription::default(),
            r#type: String::new(),
            optional: false,
            default_value: None,
            hidden: None,
            security_sensitive: None,
            read_only: None,
            parameter_relations: None,
        }
    }
}

impl Default for structs::ProfileParameterMetadataParameterRelationMetadata {
    fn default() -> Self {
        Self {
            relation_types: None,
            values: None,
            path: None,
            min_count: 0,
            max_count: 0,
        }
    }
}

impl Default for structs::ProfilePolicy {
    fn default() -> Self {
        Self {
            id: String::new(),
            policy_option: Default::default(),
        }
    }
}

impl Default for structs::ProfilePolicyMetadata {
    fn default() -> Self {
        Self {
            id: structs::ExtendedElementDescription::default(),
            possible_option: Vec::new(),
        }
    }
}

impl Default for structs::PolicyOption {
    fn default() -> Self {
        Self {
            id: String::new(),
            parameter: None,
        }
    }
}

impl Default for structs::CompositePolicyOption {
    fn default() -> Self {
        Self {
            policy_option_: structs::PolicyOption::default(),
            option: None,
        }
    }
}

impl Default for structs::ProfilePolicyOptionMetadata {
    fn default() -> Self {
        Self {
            id: structs::ExtendedElementDescription::default(),
            parameter: None,
        }
    }
}

impl Default for structs::ProfileCompositePolicyOptionMetadata {
    fn default() -> Self {
        Self {
            profile_policy_option_metadata_: structs::ProfilePolicyOptionMetadata::default(),
            option: Vec::new(),
        }
    }
}

impl Default for structs::UserInputRequiredParameterMetadata {
    fn default() -> Self {
        Self {
            profile_policy_option_metadata_: structs::ProfilePolicyOptionMetadata::default(),
            user_input_parameter: None,
        }
    }
}

impl Default for structs::ProfileConfigInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            annotation: None,
            enabled: false,
        }
    }
}

impl Default for structs::ClusterProfileConfigInfo {
    fn default() -> Self {
        Self {
            profile_config_info_: structs::ProfileConfigInfo::default(),
            comply_profile: None,
        }
    }
}

impl Default for structs::HostProfileConfigInfo {
    fn default() -> Self {
        Self {
            profile_config_info_: structs::ProfileConfigInfo::default(),
            apply_profile: None,
            default_comply_profile: None,
            default_comply_locator: None,
            custom_comply_profile: None,
            disabled_expression_list: None,
            description: None,
        }
    }
}

impl Default for structs::ProfileCreateSpec {
    fn default() -> Self {
        Self {
            name: None,
            annotation: None,
            enabled: None,
        }
    }
}

impl Default for structs::ProfileSerializedCreateSpec {
    fn default() -> Self {
        Self {
            profile_create_spec_: structs::ProfileCreateSpec::default(),
            profile_config_string: String::new(),
        }
    }
}

impl Default for structs::HostProfileSerializedHostProfileSpec {
    fn default() -> Self {
        Self {
            profile_serialized_create_spec_: structs::ProfileSerializedCreateSpec::default(),
            validator_host: None,
            validating: None,
        }
    }
}

impl Default for structs::ClusterProfileCreateSpec {
    fn default() -> Self {
        Self {
            profile_create_spec_: structs::ProfileCreateSpec::default(),
        }
    }
}

impl Default for structs::ClusterProfileConfigSpec {
    fn default() -> Self {
        Self {
            cluster_profile_create_spec_: structs::ClusterProfileCreateSpec::default(),
        }
    }
}

impl Default for structs::ClusterProfileCompleteConfigSpec {
    fn default() -> Self {
        Self {
            cluster_profile_config_spec_: structs::ClusterProfileConfigSpec::default(),
            comply_profile: None,
        }
    }
}

impl Default for structs::ClusterProfileConfigServiceCreateSpec {
    fn default() -> Self {
        Self {
            cluster_profile_config_spec_: structs::ClusterProfileConfigSpec::default(),
            service_type: None,
        }
    }
}

impl Default for structs::HostProfileConfigSpec {
    fn default() -> Self {
        Self {
            profile_create_spec_: structs::ProfileCreateSpec::default(),
        }
    }
}

impl Default for structs::HostProfileCompleteConfigSpec {
    fn default() -> Self {
        Self {
            host_profile_config_spec_: structs::HostProfileConfigSpec::default(),
            apply_profile: None,
            custom_comply_profile: None,
            disabled_expression_list_changed: false,
            disabled_expression_list: None,
            validator_host: None,
            validating: None,
            host_config: None,
        }
    }
}

impl Default for structs::HostProfileHostBasedConfigSpec {
    fn default() -> Self {
        Self {
            host_profile_config_spec_: structs::HostProfileConfigSpec::default(),
            host: structs::ManagedObjectReference::default(),
            use_host_profile_engine: None,
        }
    }
}

impl Default for structs::ProfileDescription {
    fn default() -> Self {
        Self {
            section: Vec::new(),
        }
    }
}

impl Default for structs::ProfileDescriptionSection {
    fn default() -> Self {
        Self {
            description: structs::ExtendedElementDescription::default(),
            message: None,
        }
    }
}

impl Default for structs::ProfileMetadata {
    fn default() -> Self {
        Self {
            key: String::new(),
            profile_type_name: None,
            description: None,
            sort_spec: None,
            profile_category: None,
            profile_component: None,
            operation_messages: None,
        }
    }
}

impl Default for structs::ProfileMetadataProfileOperationMessage {
    fn default() -> Self {
        Self {
            operation_name: String::new(),
            message: structs::LocalizableMessage::default(),
        }
    }
}

impl Default for structs::ProfileMetadataProfileSortSpec {
    fn default() -> Self {
        Self {
            policy_id: String::new(),
            parameter: String::new(),
        }
    }
}

impl Default for structs::ProfilePropertyPath {
    fn default() -> Self {
        Self {
            profile_path: String::new(),
            policy_id: None,
            parameter_id: None,
            policy_option_id: None,
        }
    }
}

impl Default for structs::ProfileProfileStructure {
    fn default() -> Self {
        Self {
            profile_type_name: String::new(),
            child: None,
        }
    }
}

impl Default for structs::ProfileProfileStructureProperty {
    fn default() -> Self {
        Self {
            property_name: String::new(),
            array: false,
            element: structs::ProfileProfileStructure::default(),
        }
    }
}

impl Default for structs::AnswerFile {
    fn default() -> Self {
        Self {
            user_input: None,
            created_time: String::new(),
            modified_time: String::new(),
        }
    }
}

impl Default for structs::AnswerFileStatusResult {
    fn default() -> Self {
        Self {
            checked_time: String::new(),
            host: structs::ManagedObjectReference::default(),
            status: String::new(),
            error: None,
        }
    }
}

impl Default for structs::AnswerFileStatusError {
    fn default() -> Self {
        Self {
            user_input_path: structs::ProfilePropertyPath::default(),
            err_msg: structs::LocalizableMessage::default(),
        }
    }
}

impl Default for structs::ProfileExecuteResult {
    fn default() -> Self {
        Self {
            status: String::new(),
            config_spec: None,
            inapplicable_path: None,
            require_input: None,
            error: None,
        }
    }
}

impl Default for structs::ApplyHostProfileConfigurationSpec {
    fn default() -> Self {
        Self {
            profile_execute_result_: structs::ProfileExecuteResult::default(),
            host: structs::ManagedObjectReference::default(),
            task_list_requirement: None,
            task_description: None,
            reboot_stateless: None,
            reboot_host: None,
            fault_data: None,
        }
    }
}

impl Default for structs::ProfileExecuteError {
    fn default() -> Self {
        Self {
            path: None,
            message: structs::LocalizableMessage::default(),
        }
    }
}

impl Default for structs::HostProfileValidationFailureInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            annotation: String::new(),
            update_type: String::new(),
            host: None,
            apply_profile: None,
            failures: None,
            faults: None,
        }
    }
}

impl Default for structs::HostSpecification {
    fn default() -> Self {
        Self {
            created_time: String::new(),
            last_modified: None,
            host: structs::ManagedObjectReference::default(),
            sub_specs: None,
            change_id: None,
        }
    }
}

impl Default for structs::HostSubSpecification {
    fn default() -> Self {
        Self {
            name: String::new(),
            created_time: String::new(),
            data: None,
            binary_data: None,
        }
    }
}

impl Default for structs::AnswerFileCreateSpec {
    fn default() -> Self {
        Self {
            validating: None,
        }
    }
}

impl Default for structs::AnswerFileOptionsCreateSpec {
    fn default() -> Self {
        Self {
            answer_file_create_spec_: structs::AnswerFileCreateSpec::default(),
            user_input: None,
        }
    }
}

impl Default for structs::AnswerFileSerializedCreateSpec {
    fn default() -> Self {
        Self {
            answer_file_create_spec_: structs::AnswerFileCreateSpec::default(),
            answer_file_config_string: String::new(),
        }
    }
}

impl Default for structs::ApplyHostProfileConfigurationResult {
    fn default() -> Self {
        Self {
            start_time: String::new(),
            complete_time: String::new(),
            host: structs::ManagedObjectReference::default(),
            status: String::new(),
            errors: None,
        }
    }
}

impl Default for structs::HostProfileManagerCompositionResult {
    fn default() -> Self {
        Self {
            errors: None,
            results: None,
        }
    }
}

impl Default for structs::HostProfileManagerCompositionResultResultElement {
    fn default() -> Self {
        Self {
            target: structs::ManagedObjectReference::default(),
            status: String::new(),
            errors: None,
        }
    }
}

impl Default for structs::HostProfileManagerCompositionValidationResult {
    fn default() -> Self {
        Self {
            results: None,
            errors: None,
        }
    }
}

impl Default for structs::HostProfileManagerCompositionValidationResultResultElement {
    fn default() -> Self {
        Self {
            target: structs::ManagedObjectReference::default(),
            status: String::new(),
            errors: None,
            source_diff_for_to_be_merged: None,
            target_diff_for_to_be_merged: None,
            to_be_added: None,
            to_be_deleted: None,
            to_be_disabled: None,
            to_be_enabled: None,
            to_be_reenable_cc: None,
        }
    }
}

impl Default for structs::HostProfileManagerConfigTaskList {
    fn default() -> Self {
        Self {
            config_spec: None,
            task_description: None,
            task_list_requirement: None,
        }
    }
}

impl Default for structs::HostProfilesEntityCustomizations {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::StructuredCustomizations {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
            customizations: None,
        }
    }
}

impl Default for structs::HostProfileManagerHostToConfigSpecMap {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            config_spec: Default::default(),
        }
    }
}

impl Default for structs::ScheduledTaskDescription {
    fn default() -> Self {
        Self {
            action: Vec::new(),
            scheduler_info: Vec::new(),
            state: Vec::new(),
            day_of_week: Vec::new(),
            week_of_month: Vec::new(),
        }
    }
}

impl Default for structs::ScheduledTaskSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            enabled: false,
            scheduler: Default::default(),
            action: Default::default(),
            notification: None,
        }
    }
}

impl Default for structs::ScheduledTaskInfo {
    fn default() -> Self {
        Self {
            scheduled_task_spec_: structs::ScheduledTaskSpec::default(),
            scheduled_task: structs::ManagedObjectReference::default(),
            entity: structs::ManagedObjectReference::default(),
            last_modified_time: String::new(),
            last_modified_user: String::new(),
            next_run_time: None,
            prev_run_time: None,
            state: enums::TaskInfoStateEnum::default(),
            error: None,
            result: None,
            progress: None,
            active_task: None,
            task_object: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::TaskScheduler {
    fn default() -> Self {
        Self {
            active_time: None,
            expire_time: None,
        }
    }
}

impl Default for structs::AfterStartupTaskScheduler {
    fn default() -> Self {
        Self {
            task_scheduler_: structs::TaskScheduler::default(),
            minute: 0,
        }
    }
}

impl Default for structs::OnceTaskScheduler {
    fn default() -> Self {
        Self {
            task_scheduler_: structs::TaskScheduler::default(),
            run_at: None,
        }
    }
}

impl Default for structs::RecurrentTaskScheduler {
    fn default() -> Self {
        Self {
            task_scheduler_: structs::TaskScheduler::default(),
            interval: 0,
        }
    }
}

impl Default for structs::HourlyTaskScheduler {
    fn default() -> Self {
        Self {
            recurrent_task_scheduler_: structs::RecurrentTaskScheduler::default(),
            minute: 0,
        }
    }
}

impl Default for structs::DailyTaskScheduler {
    fn default() -> Self {
        Self {
            hourly_task_scheduler_: structs::HourlyTaskScheduler::default(),
            hour: 0,
        }
    }
}

impl Default for structs::MonthlyTaskScheduler {
    fn default() -> Self {
        Self {
            daily_task_scheduler_: structs::DailyTaskScheduler::default(),
        }
    }
}

impl Default for structs::MonthlyByDayTaskScheduler {
    fn default() -> Self {
        Self {
            monthly_task_scheduler_: structs::MonthlyTaskScheduler::default(),
            day: 0,
        }
    }
}

impl Default for structs::MonthlyByWeekdayTaskScheduler {
    fn default() -> Self {
        Self {
            monthly_task_scheduler_: structs::MonthlyTaskScheduler::default(),
            offset: enums::WeekOfMonthEnum::default(),
            weekday: enums::DayOfWeekEnum::default(),
        }
    }
}

impl Default for structs::WeeklyTaskScheduler {
    fn default() -> Self {
        Self {
            daily_task_scheduler_: structs::DailyTaskScheduler::default(),
            sunday: false,
            monday: false,
            tuesday: false,
            wednesday: false,
            thursday: false,
            friday: false,
            saturday: false,
        }
    }
}

impl Default for structs::ApplyStorageRecommendationResult {
    fn default() -> Self {
        Self {
            vm: None,
        }
    }
}

impl Default for structs::StorageDrsAutomationConfig {
    fn default() -> Self {
        Self {
            space_load_balance_automation_mode: None,
            io_load_balance_automation_mode: None,
            rule_enforcement_automation_mode: None,
            policy_enforcement_automation_mode: None,
            vm_evacuation_automation_mode: None,
        }
    }
}

impl Default for structs::StorageDrsConfigInfo {
    fn default() -> Self {
        Self {
            pod_config: structs::StorageDrsPodConfigInfo::default(),
            vm_config: None,
        }
    }
}

impl Default for structs::StorageDrsConfigSpec {
    fn default() -> Self {
        Self {
            pod_config_spec: None,
            vm_config_spec: None,
        }
    }
}

impl Default for structs::StorageDrsIoLoadBalanceConfig {
    fn default() -> Self {
        Self {
            reservable_percent_threshold: None,
            reservable_iops_threshold: None,
            reservable_threshold_mode: None,
            io_latency_threshold: None,
            io_load_imbalance_threshold: None,
        }
    }
}

impl Default for structs::PlacementAffinityRule {
    fn default() -> Self {
        Self {
            rule_type: String::new(),
            rule_scope: String::new(),
            vms: None,
            keys: None,
        }
    }
}

impl Default for structs::PlacementRankResult {
    fn default() -> Self {
        Self {
            key: String::new(),
            candidate: structs::ManagedObjectReference::default(),
            reserved_space_mb: 0,
            used_space_mb: 0,
            total_space_mb: 0,
            utilization: 0.0,
            faults: None,
        }
    }
}

impl Default for structs::PlacementRankSpec {
    fn default() -> Self {
        Self {
            specs: Vec::new(),
            clusters: Vec::new(),
            rules: None,
            placement_rank_by_vm: None,
        }
    }
}

impl Default for structs::StorageDrsPlacementRankVmSpec {
    fn default() -> Self {
        Self {
            vm_placement_spec: structs::PlacementSpec::default(),
            vm_clusters: Vec::new(),
        }
    }
}

impl Default for structs::StorageDrsPodConfigInfo {
    fn default() -> Self {
        Self {
            enabled: false,
            io_load_balance_enabled: false,
            default_vm_behavior: String::new(),
            load_balance_interval: None,
            default_intra_vm_affinity: None,
            space_load_balance_config: None,
            io_load_balance_config: None,
            automation_overrides: None,
            rule: None,
            option: None,
        }
    }
}

impl Default for structs::StorageDrsPodConfigSpec {
    fn default() -> Self {
        Self {
            enabled: None,
            io_load_balance_enabled: None,
            default_vm_behavior: None,
            load_balance_interval: None,
            default_intra_vm_affinity: None,
            space_load_balance_config: None,
            io_load_balance_config: None,
            automation_overrides: None,
            rule: None,
            option: None,
        }
    }
}

impl Default for structs::StorageDrsPodSelectionSpec {
    fn default() -> Self {
        Self {
            initial_vm_config: None,
            storage_pod: None,
        }
    }
}

impl Default for structs::PodDiskLocator {
    fn default() -> Self {
        Self {
            disk_id: 0,
            disk_move_type: None,
            disk_backing_info: None,
            profile: None,
        }
    }
}

impl Default for structs::VmPodConfigForPlacement {
    fn default() -> Self {
        Self {
            storage_pod: structs::ManagedObjectReference::default(),
            disk: None,
            vm_config: None,
            inter_vm_rule: None,
        }
    }
}

impl Default for structs::StorageDrsSpaceLoadBalanceConfig {
    fn default() -> Self {
        Self {
            space_threshold_mode: None,
            space_utilization_threshold: None,
            free_space_threshold_gb: None,
            min_space_utilization_difference: None,
        }
    }
}

impl Default for structs::StoragePlacementResult {
    fn default() -> Self {
        Self {
            recommendations: None,
            drs_fault: None,
            task: None,
        }
    }
}

impl Default for structs::StoragePlacementSpec {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            priority: None,
            vm: None,
            pod_selection_spec: structs::StorageDrsPodSelectionSpec::default(),
            clone_spec: None,
            clone_name: None,
            config_spec: None,
            relocate_spec: None,
            resource_pool: None,
            host: None,
            folder: None,
            disallow_prerequisite_moves: None,
            resource_lease_duration_sec: None,
        }
    }
}

impl Default for structs::StorageDrsVmConfigInfo {
    fn default() -> Self {
        Self {
            vm: None,
            enabled: None,
            behavior: None,
            intra_vm_affinity: None,
            intra_vm_anti_affinity: None,
            virtual_disk_rules: None,
        }
    }
}

impl Default for structs::VAppCloneSpec {
    fn default() -> Self {
        Self {
            location: structs::ManagedObjectReference::default(),
            host: None,
            resource_spec: None,
            vm_folder: None,
            network_mapping: None,
            property: None,
            resource_mapping: None,
            provisioning: None,
        }
    }
}

impl Default for structs::VAppCloneSpecNetworkMappingPair {
    fn default() -> Self {
        Self {
            source: structs::ManagedObjectReference::default(),
            destination: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VAppCloneSpecResourceMap {
    fn default() -> Self {
        Self {
            source: structs::ManagedObjectReference::default(),
            parent: None,
            resource_spec: None,
            location: None,
        }
    }
}

impl Default for structs::VAppEntityConfigInfo {
    fn default() -> Self {
        Self {
            key: None,
            tag: None,
            start_order: None,
            start_delay: None,
            waiting_for_guest: None,
            start_action: None,
            stop_delay: None,
            stop_action: None,
            destroy_with_parent: None,
        }
    }
}

impl Default for structs::VAppIpAssignmentInfo {
    fn default() -> Self {
        Self {
            supported_allocation_scheme: None,
            ip_allocation_policy: None,
            supported_ip_protocol: None,
            ip_protocol: None,
        }
    }
}

impl Default for structs::IpPool {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            ipv_4_config: None,
            ipv_6_config: None,
            dns_domain: None,
            dns_search_path: None,
            host_prefix: None,
            http_proxy: None,
            network_association: None,
            available_ipv_4_addresses: None,
            available_ipv_6_addresses: None,
            allocated_ipv_4_addresses: None,
            allocated_ipv_6_addresses: None,
        }
    }
}

impl Default for structs::IpPoolAssociation {
    fn default() -> Self {
        Self {
            network: None,
            network_name: String::new(),
        }
    }
}

impl Default for structs::IpPoolIpPoolConfigInfo {
    fn default() -> Self {
        Self {
            subnet_address: None,
            netmask: None,
            gateway: None,
            range: None,
            dns: None,
            dhcp_server_available: None,
            ip_pool_enabled: None,
        }
    }
}

impl Default for structs::VAppOvfSectionInfo {
    fn default() -> Self {
        Self {
            key: None,
            namespace: None,
            r#type: None,
            at_envelope_level: None,
            contents: None,
        }
    }
}

impl Default for structs::VAppProductInfo {
    fn default() -> Self {
        Self {
            key: 0,
            class_id: None,
            instance_id: None,
            name: None,
            vendor: None,
            version: None,
            full_version: None,
            vendor_url: None,
            product_url: None,
            app_url: None,
        }
    }
}

impl Default for structs::VAppPropertyInfo {
    fn default() -> Self {
        Self {
            key: 0,
            class_id: None,
            instance_id: None,
            id: None,
            category: None,
            label: None,
            r#type: None,
            type_reference: None,
            user_configurable: None,
            default_value: None,
            value: None,
            description: None,
        }
    }
}

impl Default for structs::VmConfigInfo {
    fn default() -> Self {
        Self {
            product: None,
            property: None,
            ip_assignment: structs::VAppIpAssignmentInfo::default(),
            eula: None,
            ovf_section: None,
            ovf_environment_transport: None,
            install_boot_required: false,
            install_boot_stop_delay: 0,
        }
    }
}

impl Default for structs::VAppConfigInfo {
    fn default() -> Self {
        Self {
            vm_config_info_: structs::VmConfigInfo::default(),
            entity_config: None,
            annotation: String::new(),
            instance_uuid: None,
            managed_by: None,
        }
    }
}

impl Default for structs::VmConfigSpec {
    fn default() -> Self {
        Self {
            product: None,
            property: None,
            ip_assignment: None,
            eula: None,
            ovf_section: None,
            ovf_environment_transport: None,
            install_boot_required: None,
            install_boot_stop_delay: None,
        }
    }
}

impl Default for structs::VAppConfigSpec {
    fn default() -> Self {
        Self {
            vm_config_spec_: structs::VmConfigSpec::default(),
            entity_config: None,
            annotation: None,
            instance_uuid: None,
            managed_by: None,
        }
    }
}

impl Default for structs::ClusterNetworkConfigSpec {
    fn default() -> Self {
        Self {
            network_port_group: structs::ManagedObjectReference::default(),
            ip_settings: structs::CustomizationIpSettings::default(),
        }
    }
}

impl Default for structs::FailoverNodeInfo {
    fn default() -> Self {
        Self {
            cluster_ip_settings: structs::CustomizationIpSettings::default(),
            failover_ip: None,
            bios_uuid: None,
        }
    }
}

impl Default for structs::NodeDeploymentSpec {
    fn default() -> Self {
        Self {
            esx_host: None,
            datastore: None,
            public_network_port_group: None,
            cluster_network_port_group: None,
            folder: structs::ManagedObjectReference::default(),
            resource_pool: None,
            management_vc: None,
            node_name: String::new(),
            ip_settings: structs::CustomizationIpSettings::default(),
        }
    }
}

impl Default for structs::PassiveNodeDeploymentSpec {
    fn default() -> Self {
        Self {
            node_deployment_spec_: structs::NodeDeploymentSpec::default(),
            failover_ip_settings: None,
        }
    }
}

impl Default for structs::NodeNetworkSpec {
    fn default() -> Self {
        Self {
            ip_settings: structs::CustomizationIpSettings::default(),
        }
    }
}

impl Default for structs::PassiveNodeNetworkSpec {
    fn default() -> Self {
        Self {
            node_network_spec_: structs::NodeNetworkSpec::default(),
            failover_ip_settings: None,
        }
    }
}

impl Default for structs::SourceNodeSpec {
    fn default() -> Self {
        Self {
            management_vc: structs::ServiceLocator::default(),
            active_vc: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VchaClusterConfigInfo {
    fn default() -> Self {
        Self {
            failover_node_info_1: None,
            failover_node_info_2: None,
            witness_node_info: None,
            state: String::new(),
        }
    }
}

impl Default for structs::VchaClusterConfigSpec {
    fn default() -> Self {
        Self {
            passive_ip: String::new(),
            witness_ip: String::new(),
        }
    }
}

impl Default for structs::VchaClusterDeploymentSpec {
    fn default() -> Self {
        Self {
            passive_deployment_spec: structs::PassiveNodeDeploymentSpec::default(),
            witness_deployment_spec: Default::default(),
            active_vc_spec: structs::SourceNodeSpec::default(),
            active_vc_network_config: None,
        }
    }
}

impl Default for structs::VchaClusterNetworkSpec {
    fn default() -> Self {
        Self {
            witness_network_spec: Default::default(),
            passive_network_spec: structs::PassiveNodeNetworkSpec::default(),
        }
    }
}

impl Default for structs::WitnessNodeInfo {
    fn default() -> Self {
        Self {
            ip_settings: structs::CustomizationIpSettings::default(),
            bios_uuid: None,
        }
    }
}

impl Default for structs::VchaClusterHealth {
    fn default() -> Self {
        Self {
            runtime_info: structs::VchaClusterRuntimeInfo::default(),
            health_messages: None,
            additional_information: None,
        }
    }
}

impl Default for structs::VchaClusterRuntimeInfo {
    fn default() -> Self {
        Self {
            cluster_state: String::new(),
            node_info: None,
            cluster_mode: String::new(),
        }
    }
}

impl Default for structs::VchaNodeRuntimeInfo {
    fn default() -> Self {
        Self {
            node_state: String::new(),
            node_role: String::new(),
            node_ip: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineAffinityInfo {
    fn default() -> Self {
        Self {
            affinity_set: None,
        }
    }
}

impl Default for structs::VirtualMachineBaseIndependentFilterSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualMachineEmptyIndependentFilterSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualMachineIndependentFilterSpec {
    fn default() -> Self {
        Self {
            filter_name: String::new(),
            filter_class: None,
            filter_capabilities: None,
        }
    }
}

impl Default for structs::VirtualMachineBootOptions {
    fn default() -> Self {
        Self {
            boot_delay: None,
            enter_bios_setup: None,
            efi_secure_boot_enabled: None,
            boot_retry_enabled: None,
            boot_retry_delay: None,
            boot_order: None,
            network_boot_protocol: None,
        }
    }
}

impl Default for structs::VirtualMachineBootOptionsBootableDevice {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualMachineBootOptionsBootableCdromDevice {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualMachineBootOptionsBootableDiskDevice {
    fn default() -> Self {
        Self {
            device_key: 0,
        }
    }
}

impl Default for structs::VirtualMachineBootOptionsBootableEthernetDevice {
    fn default() -> Self {
        Self {
            device_key: 0,
        }
    }
}

impl Default for structs::VirtualMachineBootOptionsBootableFloppyDevice {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualMachineCapability {
    fn default() -> Self {
        Self {
            snapshot_operations_supported: false,
            multiple_snapshots_supported: false,
            snapshot_config_supported: false,
            powered_off_snapshots_supported: false,
            memory_snapshots_supported: false,
            revert_to_snapshot_supported: false,
            quiesced_snapshots_supported: false,
            disable_snapshots_supported: false,
            lock_snapshots_supported: false,
            console_preferences_supported: false,
            cpu_feature_mask_supported: false,
            s_1_acpi_management_supported: false,
            setting_screen_resolution_supported: false,
            tools_auto_update_supported: false,
            vm_npiv_wwn_supported: false,
            npiv_wwn_on_non_rdm_vm_supported: false,
            vm_npiv_wwn_disable_supported: false,
            vm_npiv_wwn_update_supported: false,
            swap_placement_supported: false,
            tools_sync_time_supported: false,
            virtual_mmu_usage_supported: false,
            disk_shares_supported: false,
            boot_options_supported: false,
            boot_retry_options_supported: false,
            setting_video_ram_size_supported: false,
            setting_display_topology_supported: false,
            record_replay_supported: false,
            change_tracking_supported: false,
            multiple_cores_per_socket_supported: false,
            host_based_replication_supported: false,
            guest_auto_lock_supported: false,
            memory_reservation_lock_supported: false,
            feature_requirement_supported: false,
            powered_on_monitor_type_change_supported: false,
            se_sparse_disk_supported: false,
            nested_hv_supported: false,
            v_pmc_supported: false,
            secure_boot_supported: None,
            per_vm_evc_supported: None,
            virtual_mmu_usage_ignored: None,
            virtual_exec_usage_ignored: None,
            disk_only_snapshot_on_suspended_vm_supported: None,
            suspend_to_memory_supported: None,
            tools_sync_time_allow_supported: None,
            sev_supported: None,
            pmem_failover_supported: None,
            require_sgx_attestation_supported: None,
            change_mode_disks_supported: None,
            vendor_device_group_supported: None,
            sev_snp_supported: None,
            tdx_supported: None,
        }
    }
}

impl Default for structs::VirtualMachineCertThumbprint {
    fn default() -> Self {
        Self {
            thumbprint: String::new(),
            hash_algorithm: None,
        }
    }
}

impl Default for structs::VirtualMachineCloneSpec {
    fn default() -> Self {
        Self {
            location: structs::VirtualMachineRelocateSpec::default(),
            template: false,
            config: None,
            customization: None,
            power_on: false,
            snapshot: None,
            memory: None,
            tpm_provision_policy: None,
        }
    }
}

impl Default for structs::VirtualMachineConfigInfo {
    fn default() -> Self {
        Self {
            change_version: String::new(),
            modified: String::new(),
            name: String::new(),
            guest_full_name: String::new(),
            version: String::new(),
            uuid: String::new(),
            create_date: None,
            instance_uuid: None,
            npiv_node_world_wide_name: None,
            npiv_port_world_wide_name: None,
            npiv_world_wide_name_type: None,
            npiv_desired_node_wwns: None,
            npiv_desired_port_wwns: None,
            npiv_temporary_disabled: None,
            npiv_on_non_rdm_disks: None,
            location_id: None,
            template: false,
            guest_id: String::new(),
            alternate_guest_name: String::new(),
            annotation: None,
            files: structs::VirtualMachineFileInfo::default(),
            tools: None,
            flags: structs::VirtualMachineFlagInfo::default(),
            console_preferences: None,
            default_power_ops: structs::VirtualMachineDefaultPowerOpInfo::default(),
            reboot_power_off: None,
            hardware: structs::VirtualHardware::default(),
            vcpu_config: None,
            cpu_allocation: None,
            memory_allocation: None,
            latency_sensitivity: None,
            memory_hot_add_enabled: None,
            cpu_hot_add_enabled: None,
            cpu_hot_remove_enabled: None,
            hot_plug_memory_limit: None,
            hot_plug_memory_increment_size: None,
            cpu_affinity: None,
            memory_affinity: None,
            network_shaper: None,
            extra_config: None,
            cpu_feature_mask: None,
            datastore_url: None,
            swap_placement: None,
            boot_options: None,
            ft_info: None,
            rep_config: None,
            v_app_config: None,
            v_asserts_enabled: None,
            change_tracking_enabled: None,
            firmware: None,
            max_mks_connections: None,
            guest_auto_lock_enabled: None,
            managed_by: None,
            memory_reservation_locked_to_max: None,
            initial_overhead: None,
            nested_hv_enabled: None,
            v_pmc_enabled: None,
            scheduled_hardware_upgrade_info: None,
            fork_config_info: None,
            v_flash_cache_reservation: None,
            vmx_config_checksum: None,
            message_bus_tunnel_enabled: None,
            vm_storage_object_id: None,
            swap_storage_object_id: None,
            key_id: None,
            guest_integrity_info: None,
            migrate_encryption: None,
            sgx_info: None,
            content_lib_item_info: None,
            ft_encryption_mode: None,
            guest_monitoring_mode_info: None,
            sev_enabled: None,
            numa_info: None,
            pmem_failover_enabled: None,
            vmx_stats_collection_enabled: None,
            vm_op_notification_to_app_enabled: None,
            vm_op_notification_timeout: None,
            device_swap: None,
            pmem: None,
            device_groups: None,
            fixed_passthru_hot_plug_enabled: None,
            metro_ft_enabled: None,
            vmx_runtime_config: None,
            metro_ft_host_group: None,
            tdx_enabled: None,
            sev_snp_enabled: None,
        }
    }
}

impl Default for structs::VirtualMachineConfigInfoDatastoreUrlPair {
    fn default() -> Self {
        Self {
            name: String::new(),
            url: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineConfigInfoOverheadInfo {
    fn default() -> Self {
        Self {
            initial_memory_reservation: None,
            initial_swap_reservation: None,
        }
    }
}

impl Default for structs::VirtualMachineConfigOption {
    fn default() -> Self {
        Self {
            version: String::new(),
            description: String::new(),
            guest_os_descriptor: Vec::new(),
            guest_os_default_index: 0,
            hardware_options: structs::VirtualHardwareOption::default(),
            capabilities: structs::VirtualMachineCapability::default(),
            datastore: structs::DatastoreOption::default(),
            default_device: None,
            supported_monitor_type: Vec::new(),
            supported_ovf_environment_transport: None,
            supported_ovf_install_transport: None,
            property_relations: None,
        }
    }
}

impl Default for structs::VirtualMachineConfigOptionDescriptor {
    fn default() -> Self {
        Self {
            key: String::new(),
            description: None,
            host: None,
            create_supported: false,
            default_config_option: false,
            run_supported: false,
            upgrade_supported: false,
        }
    }
}

impl Default for structs::VirtualMachineConfigSpec {
    fn default() -> Self {
        Self {
            change_version: None,
            name: None,
            version: None,
            create_date: None,
            uuid: None,
            instance_uuid: None,
            npiv_node_world_wide_name: None,
            npiv_port_world_wide_name: None,
            npiv_world_wide_name_type: None,
            npiv_desired_node_wwns: None,
            npiv_desired_port_wwns: None,
            npiv_temporary_disabled: None,
            npiv_on_non_rdm_disks: None,
            npiv_world_wide_name_op: None,
            location_id: None,
            guest_id: None,
            alternate_guest_name: None,
            annotation: None,
            files: None,
            tools: None,
            flags: None,
            console_preferences: None,
            power_op_info: None,
            reboot_power_off: None,
            num_cp_us: None,
            vcpu_config: None,
            num_cores_per_socket: None,
            memory_mb: None,
            memory_hot_add_enabled: None,
            cpu_hot_add_enabled: None,
            cpu_hot_remove_enabled: None,
            virtual_ich_7_m_present: None,
            virtual_smc_present: None,
            device_change: None,
            cpu_allocation: None,
            memory_allocation: None,
            latency_sensitivity: None,
            cpu_affinity: None,
            memory_affinity: None,
            network_shaper: None,
            cpu_feature_mask: None,
            extra_config: None,
            swap_placement: None,
            boot_options: None,
            v_app_config: None,
            ft_info: None,
            rep_config: None,
            v_app_config_removed: None,
            v_asserts_enabled: None,
            change_tracking_enabled: None,
            firmware: None,
            max_mks_connections: None,
            guest_auto_lock_enabled: None,
            managed_by: None,
            memory_reservation_locked_to_max: None,
            nested_hv_enabled: None,
            v_pmc_enabled: None,
            scheduled_hardware_upgrade_info: None,
            vm_profile: None,
            message_bus_tunnel_enabled: None,
            crypto: None,
            migrate_encryption: None,
            sgx_info: None,
            ft_encryption_mode: None,
            guest_monitoring_mode_info: None,
            sev_enabled: None,
            virtual_numa: None,
            motherboard_layout: None,
            pmem_failover_enabled: None,
            vmx_stats_collection_enabled: None,
            vm_op_notification_to_app_enabled: None,
            vm_op_notification_timeout: None,
            device_swap: None,
            simultaneous_threads: None,
            pmem: None,
            device_groups: None,
            fixed_passthru_hot_plug_enabled: None,
            metro_ft_enabled: None,
            metro_ft_host_group: None,
            tdx_enabled: None,
            sev_snp_enabled: None,
        }
    }
}

impl Default for structs::ConfigTarget {
    fn default() -> Self {
        Self {
            num_cpus: 0,
            num_cpu_cores: 0,
            num_numa_nodes: 0,
            max_cpus_per_host: None,
            smc_present: false,
            datastore: None,
            network: None,
            opaque_network: None,
            distributed_virtual_portgroup: None,
            distributed_virtual_switch: None,
            subnet_info: None,
            cd_rom: None,
            serial: None,
            parallel: None,
            sound: None,
            usb: None,
            floppy: None,
            legacy_network_info: None,
            scsi_passthrough: None,
            scsi_disk: None,
            ide_disk: None,
            max_mem_mb_optimal_perf: 0,
            supported_max_mem_mb: None,
            resource_pool: None,
            auto_vmotion: None,
            pci_passthrough: None,
            sriov: None,
            v_flash_module: None,
            shared_gpu_passthrough_types: None,
            available_persistent_memory_reservation_mb: None,
            dynamic_passthrough: None,
            sgx_target_info: None,
            precision_clock_info: None,
            sev_supported: None,
            vgpu_device_info: None,
            vgpu_profile_info: None,
            vendor_device_group_info: None,
            max_simultaneous_threads: None,
            dvx_class_info: None,
            sev_snp_supported: None,
            tdx_supported: None,
        }
    }
}

impl Default for structs::VirtualMachineConsolePreferences {
    fn default() -> Self {
        Self {
            power_on_when_opened: None,
            enter_full_screen_on_power_on: None,
            close_on_power_off_or_suspend: None,
        }
    }
}

impl Default for structs::VirtualMachineContentLibraryItemInfo {
    fn default() -> Self {
        Self {
            content_library_item_uuid: String::new(),
            content_library_item_version: None,
        }
    }
}

impl Default for structs::DatastoreOption {
    fn default() -> Self {
        Self {
            unsupported_volumes: None,
        }
    }
}

impl Default for structs::VirtualMachineDatastoreVolumeOption {
    fn default() -> Self {
        Self {
            file_system_type: String::new(),
            major_version: None,
        }
    }
}

impl Default for structs::VirtualMachineDefaultPowerOpInfo {
    fn default() -> Self {
        Self {
            power_off_type: None,
            suspend_type: None,
            reset_type: None,
            default_power_off_type: None,
            default_suspend_type: None,
            default_reset_type: None,
            standby_action: None,
        }
    }
}

impl Default for structs::VirtualMachineDeviceRuntimeInfo {
    fn default() -> Self {
        Self {
            runtime_state: Default::default(),
            key: 0,
        }
    }
}

impl Default for structs::VirtualMachineDeviceRuntimeInfoDeviceRuntimeState {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState {
    fn default() -> Self {
        Self {
            vm_direct_path_gen_2_active: None,
            vm_direct_path_gen_2_inactive_reason_vm: None,
            vm_direct_path_gen_2_inactive_reason_other: None,
            vm_direct_path_gen_2_inactive_reason_extended: None,
            uptv_2_active: None,
            uptv_2_inactive_reason_vm: None,
            uptv_2_inactive_reason_other: None,
            reservation_status: None,
            attachment_status: None,
            feature_requirement: None,
        }
    }
}

impl Default for structs::VirtualMachineDvxClassInfo {
    fn default() -> Self {
        Self {
            device_class: Default::default(),
            vendor_name: String::new(),
            sriov_nic: false,
            config_params: None,
        }
    }
}

impl Default for structs::FaultToleranceConfigInfo {
    fn default() -> Self {
        Self {
            role: 0,
            instance_uuids: Vec::new(),
            config_paths: Vec::new(),
            orphaned: None,
        }
    }
}

impl Default for structs::FaultTolerancePrimaryConfigInfo {
    fn default() -> Self {
        Self {
            fault_tolerance_config_info_: structs::FaultToleranceConfigInfo::default(),
            secondaries: Vec::new(),
        }
    }
}

impl Default for structs::FaultToleranceSecondaryConfigInfo {
    fn default() -> Self {
        Self {
            fault_tolerance_config_info_: structs::FaultToleranceConfigInfo::default(),
            primary_vm: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::FaultToleranceConfigSpec {
    fn default() -> Self {
        Self {
            meta_data_path: None,
            secondary_vm_spec: None,
            metro_ft_enabled: None,
            metro_ft_host_group: None,
        }
    }
}

impl Default for structs::FaultToleranceMetaSpec {
    fn default() -> Self {
        Self {
            meta_data_datastore: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::FaultToleranceSecondaryOpResult {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            power_on_attempted: false,
            power_on_result: None,
        }
    }
}

impl Default for structs::FaultToleranceVmConfigSpec {
    fn default() -> Self {
        Self {
            vm_config: None,
            disks: None,
        }
    }
}

impl Default for structs::FaultToleranceDiskSpec {
    fn default() -> Self {
        Self {
            disk: Default::default(),
            datastore: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VirtualMachineFeatureRequirement {
    fn default() -> Self {
        Self {
            key: String::new(),
            feature_name: String::new(),
            value: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineFileInfo {
    fn default() -> Self {
        Self {
            vm_path_name: None,
            snapshot_directory: None,
            suspend_directory: None,
            log_directory: None,
            ft_metadata_directory: None,
        }
    }
}

impl Default for structs::VirtualMachineFileLayout {
    fn default() -> Self {
        Self {
            config_file: None,
            log_file: None,
            disk: None,
            snapshot: None,
            swap_file: None,
        }
    }
}

impl Default for structs::VirtualMachineFileLayoutDiskLayout {
    fn default() -> Self {
        Self {
            key: 0,
            disk_file: Vec::new(),
        }
    }
}

impl Default for structs::VirtualMachineFileLayoutSnapshotLayout {
    fn default() -> Self {
        Self {
            key: structs::ManagedObjectReference::default(),
            snapshot_file: Vec::new(),
        }
    }
}

impl Default for structs::VirtualMachineFileLayoutEx {
    fn default() -> Self {
        Self {
            file: None,
            disk: None,
            snapshot: None,
            timestamp: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineFileLayoutExDiskLayout {
    fn default() -> Self {
        Self {
            key: 0,
            virtual_disk_format: None,
            chain: None,
        }
    }
}

impl Default for structs::VirtualMachineFileLayoutExDiskUnit {
    fn default() -> Self {
        Self {
            file_key: Vec::new(),
        }
    }
}

impl Default for structs::VirtualMachineFileLayoutExFileInfo {
    fn default() -> Self {
        Self {
            key: 0,
            name: String::new(),
            r#type: String::new(),
            size: 0,
            unique_size: None,
            backing_object_id: None,
            accessible: None,
        }
    }
}

impl Default for structs::VirtualMachineFileLayoutExSnapshotLayout {
    fn default() -> Self {
        Self {
            key: structs::ManagedObjectReference::default(),
            data_key: 0,
            memory_key: 0,
            disk: None,
        }
    }
}

impl Default for structs::VirtualMachineFlagInfo {
    fn default() -> Self {
        Self {
            disable_acceleration: None,
            enable_logging: None,
            use_toe: None,
            run_with_debug_info: None,
            monitor_type: None,
            ht_sharing: None,
            snapshot_disabled: None,
            snapshot_locked: None,
            disk_uuid_enabled: None,
            virtual_mmu_usage: None,
            virtual_exec_usage: None,
            snapshot_power_off_behavior: None,
            record_replay_enabled: None,
            fault_tolerance_type: None,
            cbrc_cache_enabled: None,
            vvtd_enabled: None,
            vbs_enabled: None,
        }
    }
}

impl Default for structs::VirtualMachineForkConfigInfo {
    fn default() -> Self {
        Self {
            parent_enabled: None,
            child_fork_group_id: None,
            parent_fork_group_id: None,
            child_type: None,
        }
    }
}

impl Default for structs::GuestInfo {
    fn default() -> Self {
        Self {
            tools_status: None,
            tools_version_status: None,
            tools_version_status_2: None,
            tools_running_status: None,
            tools_version: None,
            tools_install_type: None,
            guest_id: None,
            guest_family: None,
            guest_full_name: None,
            guest_detailed_data: None,
            host_name: None,
            ip_address: None,
            net: None,
            ip_stack: None,
            disk: None,
            screen: None,
            guest_state: String::new(),
            app_heartbeat_status: None,
            guest_kernel_crashed: None,
            app_state: None,
            guest_operations_ready: None,
            interactive_guest_operations_ready: None,
            guest_state_change_supported: None,
            generation_info: None,
            hw_version: None,
            customization_info: None,
        }
    }
}

impl Default for structs::GuestInfoCustomizationInfo {
    fn default() -> Self {
        Self {
            customization_status: String::new(),
            start_time: None,
            end_time: None,
            error_msg: None,
        }
    }
}

impl Default for structs::GuestDiskInfo {
    fn default() -> Self {
        Self {
            disk_path: None,
            capacity: None,
            free_space: None,
            filesystem_type: None,
            mappings: None,
        }
    }
}

impl Default for structs::GuestInfoNamespaceGenerationInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
            generation_no: 0,
        }
    }
}

impl Default for structs::GuestNicInfo {
    fn default() -> Self {
        Self {
            network: None,
            ip_address: None,
            mac_address: None,
            connected: false,
            device_config_id: 0,
            dns_config: None,
            ip_config: None,
            net_bios_config: None,
        }
    }
}

impl Default for structs::GuestScreenInfo {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}

impl Default for structs::GuestStackInfo {
    fn default() -> Self {
        Self {
            dns_config: None,
            ip_route_config: None,
            ip_stack_config: None,
            dhcp_config: None,
        }
    }
}

impl Default for structs::GuestInfoVirtualDiskMapping {
    fn default() -> Self {
        Self {
            key: 0,
        }
    }
}

impl Default for structs::VirtualMachineGuestIntegrityInfo {
    fn default() -> Self {
        Self {
            enabled: None,
        }
    }
}

impl Default for structs::VirtualMachineGuestMonitoringModeInfo {
    fn default() -> Self {
        Self {
            gmm_file: None,
            gmm_appliance: None,
        }
    }
}

impl Default for structs::GuestOsDescriptor {
    fn default() -> Self {
        Self {
            id: String::new(),
            family: String::new(),
            full_name: String::new(),
            supported_max_cp_us: 0,
            num_supported_physical_sockets: 0,
            num_supported_cores_per_socket: 0,
            supported_min_mem_mb: 0,
            supported_max_mem_mb: 0,
            recommended_mem_mb: 0,
            recommended_color_depth: 0,
            supported_disk_controller_list: Vec::new(),
            recommended_scsi_controller: None,
            recommended_disk_controller: String::new(),
            supported_num_disks: 0,
            recommended_disk_size_mb: 0,
            recommended_cdrom_controller: String::new(),
            supported_ethernet_card: Vec::new(),
            recommended_ethernet_card: None,
            supports_slave_disk: None,
            cpu_feature_mask: None,
            smc_required: false,
            supports_wake_on_lan: false,
            supports_vmi: false,
            supports_memory_hot_add: false,
            supports_cpu_hot_add: false,
            supports_cpu_hot_remove: false,
            supported_firmware: Vec::new(),
            recommended_firmware: String::new(),
            supported_usb_controller_list: None,
            recommended_usb_controller: None,
            supports_3_d: false,
            recommended_3_d: false,
            smc_recommended: false,
            ich_7_m_recommended: false,
            usb_recommended: false,
            support_level: String::new(),
            supported_for_create: false,
            v_ram_size_in_kb: structs::IntOption::default(),
            num_supported_floppy_devices: 0,
            wake_on_lan_ethernet_card: None,
            supports_pvscsi_controller_for_boot: false,
            disk_uuid_enabled: false,
            supports_hot_plug_pci: false,
            supports_secure_boot: None,
            default_secure_boot: None,
            persistent_memory_supported: None,
            supported_min_persistent_memory_mb: None,
            supported_max_persistent_memory_mb: None,
            recommended_persistent_memory_mb: None,
            persistent_memory_hot_add_supported: None,
            persistent_memory_hot_remove_supported: None,
            persistent_memory_cold_growth_supported: None,
            persistent_memory_cold_growth_granularity_mb: None,
            persistent_memory_hot_growth_supported: None,
            persistent_memory_hot_growth_granularity_mb: None,
            num_recommended_physical_sockets: None,
            num_recommended_cores_per_socket: None,
            vvtd_supported: None,
            vbs_supported: None,
            vsgx_supported: None,
            vsgx_remote_attestation_supported: None,
            supports_tpm_20: None,
            recommended_tpm_20: None,
            vwdt_supported: None,
        }
    }
}

impl Default for structs::VirtualMachineGuestQuiesceSpec {
    fn default() -> Self {
        Self {
            timeout: None,
        }
    }
}

impl Default for structs::VirtualMachineWindowsQuiesceSpec {
    fn default() -> Self {
        Self {
            virtual_machine_guest_quiesce_spec_: structs::VirtualMachineGuestQuiesceSpec::default(),
            vss_backup_type: None,
            vss_bootable_system_state: None,
            vss_partial_file_support: None,
            vss_backup_context: None,
        }
    }
}

impl Default for structs::VirtualMachineIdeDiskDevicePartitionInfo {
    fn default() -> Self {
        Self {
            id: 0,
            capacity: 0,
        }
    }
}

impl Default for structs::VirtualMachineInstantCloneSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            location: structs::VirtualMachineRelocateSpec::default(),
            config: None,
            bios_uuid: None,
        }
    }
}

impl Default for structs::VirtualMachineLegacyNetworkSwitchInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineMessage {
    fn default() -> Self {
        Self {
            id: String::new(),
            argument: None,
            text: None,
        }
    }
}

impl Default for structs::VirtualMachineMetadataManagerVmMetadata {
    fn default() -> Self {
        Self {
            vm_id: String::new(),
            metadata: None,
        }
    }
}

impl Default for structs::VirtualMachineMetadataManagerVmMetadataInput {
    fn default() -> Self {
        Self {
            operation: String::new(),
            vm_metadata: structs::VirtualMachineMetadataManagerVmMetadata::default(),
        }
    }
}

impl Default for structs::VirtualMachineMetadataManagerVmMetadataOwner {
    fn default() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineMetadataManagerVmMetadataResult {
    fn default() -> Self {
        Self {
            vm_metadata: structs::VirtualMachineMetadataManagerVmMetadata::default(),
            error: None,
        }
    }
}

impl Default for structs::VirtualMachineNetworkShaperInfo {
    fn default() -> Self {
        Self {
            enabled: None,
            peak_bps: None,
            average_bps: None,
            burst_size: None,
        }
    }
}

impl Default for structs::VirtualMachineProfileDetails {
    fn default() -> Self {
        Self {
            profile: None,
            disk_profile_details: None,
        }
    }
}

impl Default for structs::VirtualMachineProfileDetailsDiskProfileDetails {
    fn default() -> Self {
        Self {
            disk_id: 0,
            profile: None,
        }
    }
}

impl Default for structs::VirtualMachineProfileRawData {
    fn default() -> Self {
        Self {
            extension_key: String::new(),
            object_data: None,
        }
    }
}

impl Default for structs::VirtualMachineProfileSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualMachineDefaultProfileSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualMachineDefinedProfileSpec {
    fn default() -> Self {
        Self {
            profile_id: String::new(),
            replication_spec: None,
            profile_data: None,
            profile_params: None,
        }
    }
}

impl Default for structs::VirtualMachineEmptyProfileSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualMachinePropertyRelation {
    fn default() -> Self {
        Self {
            key: structs::DynamicProperty::default(),
            relations: None,
        }
    }
}

impl Default for structs::VirtualMachineQuestionInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            text: String::new(),
            choice: structs::ChoiceOption::default(),
            message: None,
        }
    }
}

impl Default for structs::VirtualMachineRelocateSpec {
    fn default() -> Self {
        Self {
            service: None,
            folder: None,
            datastore: None,
            disk_move_type: None,
            pool: None,
            host: None,
            disk: None,
            transform: None,
            device_change: None,
            profile: None,
            crypto_spec: None,
        }
    }
}

impl Default for structs::VirtualMachineRelocateSpecDiskLocator {
    fn default() -> Self {
        Self {
            disk_id: 0,
            datastore: structs::ManagedObjectReference::default(),
            disk_move_type: None,
            disk_backing_info: None,
            profile: None,
            backing: None,
            filter_spec: None,
        }
    }
}

impl Default for structs::VirtualMachineRelocateSpecDiskLocatorBackingSpec {
    fn default() -> Self {
        Self {
            parent: None,
            crypto: None,
        }
    }
}

impl Default for structs::ReplicationConfigSpec {
    fn default() -> Self {
        Self {
            generation: 0,
            vm_replication_id: String::new(),
            destination: String::new(),
            port: 0,
            rpo: 0,
            quiesce_guest_enabled: false,
            paused: false,
            opp_updates_enabled: false,
            net_compression_enabled: None,
            net_encryption_enabled: None,
            encryption_destination: None,
            encryption_port: None,
            remote_certificate_thumbprint: None,
            data_sets_replication_enabled: None,
            disk: None,
        }
    }
}

impl Default for structs::ReplicationInfoDiskSettings {
    fn default() -> Self {
        Self {
            key: 0,
            disk_replication_id: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineRuntimeInfo {
    fn default() -> Self {
        Self {
            device: None,
            host: None,
            connection_state: enums::VirtualMachineConnectionStateEnum::default(),
            power_state: enums::VirtualMachinePowerStateEnum::default(),
            vm_failover_in_progress: None,
            fault_tolerance_state: enums::VirtualMachineFaultToleranceStateEnum::default(),
            das_vm_protection: None,
            tools_installer_mounted: false,
            suspend_time: None,
            boot_time: None,
            suspend_interval: None,
            question: None,
            memory_overhead: None,
            max_cpu_usage: None,
            max_memory_usage: None,
            num_mks_connections: 0,
            record_replay_state: enums::VirtualMachineRecordReplayStateEnum::default(),
            clean_power_off: None,
            need_secondary_reason: None,
            online_standby: false,
            min_required_evc_mode_key: None,
            consolidation_needed: false,
            offline_feature_requirement: None,
            feature_requirement: None,
            feature_mask: None,
            v_flash_cache_allocation: None,
            paused: None,
            snapshot_in_background: None,
            quiesced_fork_parent: None,
            instant_clone_frozen: None,
            crypto_state: None,
            suspended_to_memory: None,
            op_notification_timeout: None,
            iommu_active: None,
        }
    }
}

impl Default for structs::VirtualMachineRuntimeInfoDasProtectionState {
    fn default() -> Self {
        Self {
            das_protected: false,
        }
    }
}

impl Default for structs::ScheduledHardwareUpgradeInfo {
    fn default() -> Self {
        Self {
            upgrade_policy: None,
            version_key: None,
            scheduled_hardware_upgrade_status: None,
            fault: None,
        }
    }
}

impl Default for structs::VirtualMachineSgxInfo {
    fn default() -> Self {
        Self {
            epc_size: 0,
            flc_mode: None,
            le_pub_key_hash: None,
            require_attestation: None,
        }
    }
}

impl Default for structs::VirtualMachineSnapshotInfo {
    fn default() -> Self {
        Self {
            current_snapshot: None,
            root_snapshot_list: Vec::new(),
        }
    }
}

impl Default for structs::SnapshotSelectionSpec {
    fn default() -> Self {
        Self {
            retention_days: None,
        }
    }
}

impl Default for structs::VirtualMachineSnapshotTree {
    fn default() -> Self {
        Self {
            snapshot: structs::ManagedObjectReference::default(),
            vm: structs::ManagedObjectReference::default(),
            name: String::new(),
            description: String::new(),
            id: 0,
            create_time: String::new(),
            state: enums::VirtualMachinePowerStateEnum::default(),
            quiesced: false,
            backup_manifest: None,
            child_snapshot_list: None,
            replay_supported: None,
        }
    }
}

impl Default for structs::VirtualMachineSriovDevicePoolInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineSriovNetworkDevicePoolInfo {
    fn default() -> Self {
        Self {
            virtual_machine_sriov_device_pool_info_: structs::VirtualMachineSriovDevicePoolInfo::default(),
            switch_key: None,
            switch_uuid: None,
        }
    }
}

impl Default for structs::VirtualMachineStorageInfo {
    fn default() -> Self {
        Self {
            per_datastore_usage: None,
            timestamp: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineUsageOnDatastore {
    fn default() -> Self {
        Self {
            datastore: structs::ManagedObjectReference::default(),
            committed: 0,
            uncommitted: 0,
            unshared: 0,
        }
    }
}

impl Default for structs::SubnetInfoFolderInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            folder: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VirtualMachineSummary {
    fn default() -> Self {
        Self {
            vm: None,
            runtime: structs::VirtualMachineRuntimeInfo::default(),
            guest: None,
            config: structs::VirtualMachineConfigSummary::default(),
            storage: None,
            quick_stats: structs::VirtualMachineQuickStats::default(),
            overall_status: enums::ManagedEntityStatusEnum::default(),
            custom_value: None,
        }
    }
}

impl Default for structs::VirtualMachineConfigSummary {
    fn default() -> Self {
        Self {
            name: String::new(),
            template: false,
            vm_path_name: String::new(),
            memory_size_mb: None,
            cpu_reservation: None,
            memory_reservation: None,
            num_cpu: None,
            num_ethernet_cards: None,
            num_virtual_disks: None,
            uuid: None,
            instance_uuid: None,
            guest_id: None,
            guest_full_name: None,
            annotation: None,
            product: None,
            install_boot_required: None,
            ft_info: None,
            managed_by: None,
            tpm_present: None,
            num_vmiop_backings: None,
            hw_version: None,
        }
    }
}

impl Default for structs::VirtualMachineGuestSummary {
    fn default() -> Self {
        Self {
            guest_id: None,
            guest_full_name: None,
            tools_status: None,
            tools_version_status: None,
            tools_version_status_2: None,
            tools_running_status: None,
            host_name: None,
            ip_address: None,
            hw_version: None,
        }
    }
}

impl Default for structs::VirtualMachineQuickStats {
    fn default() -> Self {
        Self {
            overall_cpu_usage: None,
            overall_cpu_demand: None,
            overall_cpu_readiness: None,
            guest_memory_usage: None,
            host_memory_usage: None,
            guest_heartbeat_status: enums::ManagedEntityStatusEnum::default(),
            distributed_cpu_entitlement: None,
            distributed_memory_entitlement: None,
            static_cpu_entitlement: None,
            static_memory_entitlement: None,
            granted_memory: None,
            private_memory: None,
            shared_memory: None,
            swapped_memory: None,
            ballooned_memory: None,
            consumed_overhead_memory: None,
            ft_log_bandwidth: None,
            ft_secondary_latency: None,
            ft_latency_status: None,
            compressed_memory: None,
            uptime_seconds: None,
            ssd_swapped_memory: None,
            active_memory: None,
            memory_tier_stats: None,
        }
    }
}

impl Default for structs::VirtualMachineQuickStatsMemoryTierStats {
    fn default() -> Self {
        Self {
            memory_tier_type: String::new(),
            read_bandwidth: 0,
        }
    }
}

impl Default for structs::VirtualMachineStorageSummary {
    fn default() -> Self {
        Self {
            committed: 0,
            uncommitted: 0,
            unshared: 0,
            timestamp: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineTargetInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            configuration_tag: None,
        }
    }
}

impl Default for structs::VirtualMachineCdromInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            description: None,
        }
    }
}

impl Default for structs::VirtualMachineDatastoreInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            datastore: structs::DatastoreSummary::default(),
            capability: structs::DatastoreCapability::default(),
            max_file_size: 0,
            max_virtual_disk_capacity: None,
            max_physical_rdm_file_size: None,
            max_virtual_rdm_file_size: None,
            mode: String::new(),
            v_storage_support: None,
            supported_v_disk_formats: None,
        }
    }
}

impl Default for structs::VirtualMachineDiskDeviceInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            capacity: None,
            vm: None,
        }
    }
}

impl Default for structs::VirtualMachineIdeDiskDeviceInfo {
    fn default() -> Self {
        Self {
            virtual_machine_disk_device_info_: structs::VirtualMachineDiskDeviceInfo::default(),
            partition_table: None,
        }
    }
}

impl Default for structs::VirtualMachineScsiDiskDeviceInfo {
    fn default() -> Self {
        Self {
            virtual_machine_disk_device_info_: structs::VirtualMachineDiskDeviceInfo::default(),
            disk: None,
            transport_hint: None,
            lun_number: None,
        }
    }
}

impl Default for structs::VirtualMachineDynamicPassthroughInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            vendor_name: String::new(),
            device_name: String::new(),
            custom_label: None,
            vendor_id: 0,
            device_id: 0,
        }
    }
}

impl Default for structs::VirtualMachineFloppyInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
        }
    }
}

impl Default for structs::VirtualMachineNetworkInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            network: Default::default(),
            vswitch: None,
        }
    }
}

impl Default for structs::OpaqueNetworkTargetInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            network: structs::OpaqueNetworkSummary::default(),
            network_reservation_supported: None,
        }
    }
}

impl Default for structs::VirtualMachineParallelInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
        }
    }
}

impl Default for structs::VirtualMachinePciPassthroughInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            pci_device: structs::HostPciDevice::default(),
            system_id: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineSriovInfo {
    fn default() -> Self {
        Self {
            virtual_machine_pci_passthrough_info_: structs::VirtualMachinePciPassthroughInfo::default(),
            virtual_function: false,
            pnic: None,
            device_pool: None,
        }
    }
}

impl Default for structs::VirtualMachinePciSharedGpuPassthroughInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            vgpu: String::new(),
        }
    }
}

impl Default for structs::VirtualMachinePrecisionClockInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            system_clock_protocol: None,
        }
    }
}

impl Default for structs::VirtualMachineScsiPassthroughInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            scsi_class: String::new(),
            vendor: String::new(),
            physical_unit_number: 0,
        }
    }
}

impl Default for structs::VirtualMachineSerialInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
        }
    }
}

impl Default for structs::VirtualMachineSgxTargetInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            max_epc_size: 0,
            flc_modes: None,
            le_pub_key_hashes: None,
            require_attestation_supported: None,
        }
    }
}

impl Default for structs::VirtualMachineSoundInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
        }
    }
}

impl Default for structs::SubnetInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            id: String::new(),
            subnet_folder_info: structs::SubnetInfoFolderInfo::default(),
            vpc_folder_info: structs::SubnetInfoFolderInfo::default(),
            project_folder_info: None,
            root_folder_info: structs::SubnetInfoFolderInfo::default(),
        }
    }
}

impl Default for structs::VirtualMachineUsbInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            description: String::new(),
            vendor: 0,
            product: 0,
            physical_path: String::new(),
            family: None,
            speed: None,
            summary: None,
        }
    }
}

impl Default for structs::VirtualMachineVFlashModuleInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            v_flash_module: structs::HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption::default(),
        }
    }
}

impl Default for structs::VirtualMachineVMotionStunTimeInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            migration_bw: 0,
            stun_time: 0,
        }
    }
}

impl Default for structs::VirtualMachineVendorDeviceGroupInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            device_group_name: String::new(),
            device_group_description: None,
            component_device_info: None,
        }
    }
}

impl Default for structs::VirtualMachineVgpuDeviceInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            device_name: String::new(),
            device_vendor_id: 0,
            max_fb_size_in_gib: 0,
            time_sliced_capable: false,
            mig_capable: false,
            compute_profile_capable: false,
            quadro_profile_capable: false,
        }
    }
}

impl Default for structs::VirtualMachineVgpuProfileInfo {
    fn default() -> Self {
        Self {
            virtual_machine_target_info_: structs::VirtualMachineTargetInfo::default(),
            profile_name: String::new(),
            device_vendor_id: 0,
            fb_size_in_gib: 0,
            profile_sharing: String::new(),
            profile_class: String::new(),
            stun_time_estimates: None,
        }
    }
}

impl Default for structs::ToolsConfigInfo {
    fn default() -> Self {
        Self {
            tools_version: None,
            tools_install_type: None,
            after_power_on: None,
            after_resume: None,
            before_guest_standby: None,
            before_guest_shutdown: None,
            before_guest_reboot: None,
            tools_upgrade_policy: None,
            pending_customization: None,
            customization_key_id: None,
            sync_time_with_host_allowed: None,
            sync_time_with_host: None,
            last_install_info: None,
        }
    }
}

impl Default for structs::ToolsConfigInfoToolsLastInstallInfo {
    fn default() -> Self {
        Self {
            counter: 0,
            fault: None,
        }
    }
}

impl Default for structs::UsbScanCodeSpec {
    fn default() -> Self {
        Self {
            key_events: Vec::new(),
        }
    }
}

impl Default for structs::UsbScanCodeSpecKeyEvent {
    fn default() -> Self {
        Self {
            usb_hid_code: 0,
            modifiers: None,
        }
    }
}

impl Default for structs::UsbScanCodeSpecModifierType {
    fn default() -> Self {
        Self {
            left_control: None,
            left_shift: None,
            left_alt: None,
            left_gui: None,
            right_control: None,
            right_shift: None,
            right_alt: None,
            right_gui: None,
        }
    }
}

impl Default for structs::VirtualMachineVcpuConfig {
    fn default() -> Self {
        Self {
            latency_sensitivity: None,
        }
    }
}

impl Default for structs::VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            vendor_name: String::new(),
            device_name: String::new(),
            is_configurable: false,
            device: Default::default(),
        }
    }
}

impl Default for structs::VirtualMachineVirtualDeviceGroups {
    fn default() -> Self {
        Self {
            device_group: None,
        }
    }
}

impl Default for structs::VirtualMachineVirtualDeviceGroupsDeviceGroup {
    fn default() -> Self {
        Self {
            group_instance_key: 0,
            device_info: None,
        }
    }
}

impl Default for structs::VirtualMachineVirtualDeviceGroupsVendorDeviceGroup {
    fn default() -> Self {
        Self {
            virtual_machine_virtual_device_groups_device_group_: structs::VirtualMachineVirtualDeviceGroupsDeviceGroup::default(),
            device_group_name: String::new(),
        }
    }
}

impl Default for structs::VirtualMachineVirtualDeviceSwap {
    fn default() -> Self {
        Self {
            lsi_to_pvscsi: None,
        }
    }
}

impl Default for structs::VirtualMachineVirtualDeviceSwapDeviceSwapInfo {
    fn default() -> Self {
        Self {
            enabled: None,
            applicable: None,
            status: None,
        }
    }
}

impl Default for structs::VirtualHardware {
    fn default() -> Self {
        Self {
            num_cpu: 0,
            num_cores_per_socket: None,
            auto_cores_per_socket: None,
            memory_mb: 0,
            virtual_ich_7_m_present: None,
            virtual_smc_present: None,
            device: None,
            motherboard_layout: None,
            simultaneous_threads: None,
        }
    }
}

impl Default for structs::VirtualHardwareOption {
    fn default() -> Self {
        Self {
            hw_version: 0,
            virtual_device_option: Vec::new(),
            device_list_readonly: false,
            num_cpu: Vec::new(),
            num_cores_per_socket: structs::IntOption::default(),
            auto_cores_per_socket: None,
            num_cpu_readonly: false,
            memory_mb: structs::LongOption::default(),
            num_pci_controllers: structs::IntOption::default(),
            num_ide_controllers: structs::IntOption::default(),
            num_usb_controllers: structs::IntOption::default(),
            num_usbxhci_controllers: structs::IntOption::default(),
            num_sio_controllers: structs::IntOption::default(),
            num_ps_2_controllers: structs::IntOption::default(),
            licensing_limit: None,
            num_supported_wwn_ports: None,
            num_supported_wwn_nodes: None,
            resource_config_option: structs::ResourceConfigOption::default(),
            num_nvdimm_controllers: None,
            num_tpm_devices: None,
            num_wdt_devices: None,
            num_precision_clock_devices: None,
            epc_memory_mb: None,
            acpi_host_bridges_firmware: None,
            num_cpu_simultaneous_threads: None,
            num_numa_nodes: None,
            num_device_groups: None,
            device_group_types: None,
        }
    }
}

impl Default for structs::VirtualMachineVirtualNuma {
    fn default() -> Self {
        Self {
            cores_per_numa_node: None,
            expose_vnuma_on_cpu_hotadd: None,
        }
    }
}

impl Default for structs::VirtualMachineVirtualNumaInfo {
    fn default() -> Self {
        Self {
            cores_per_numa_node: None,
            auto_cores_per_numa_node: None,
            vnuma_on_cpu_hotadd_exposed: None,
        }
    }
}

impl Default for structs::VirtualMachineVirtualPMem {
    fn default() -> Self {
        Self {
            snapshot_mode: None,
        }
    }
}

impl Default for structs::CheckResult {
    fn default() -> Self {
        Self {
            vm: None,
            host: None,
            warning: None,
            error: None,
        }
    }
}

impl Default for structs::CustomizationAdapterMapping {
    fn default() -> Self {
        Self {
            mac_address: None,
            adapter: structs::CustomizationIpSettings::default(),
        }
    }
}

impl Default for structs::CustomizationGlobalIpSettings {
    fn default() -> Self {
        Self {
            dns_suffix_list: None,
            dns_server_list: None,
        }
    }
}

impl Default for structs::CustomizationGuiRunOnce {
    fn default() -> Self {
        Self {
            command_list: Vec::new(),
        }
    }
}

impl Default for structs::CustomizationGuiUnattended {
    fn default() -> Self {
        Self {
            password: None,
            time_zone: 0,
            auto_logon: false,
            auto_logon_count: 0,
        }
    }
}

impl Default for structs::CustomizationIpSettings {
    fn default() -> Self {
        Self {
            ip: Default::default(),
            subnet_mask: None,
            gateway: None,
            ip_v_6_spec: None,
            dns_server_list: None,
            dns_domain: None,
            primary_wins: None,
            secondary_wins: None,
            net_bios: None,
        }
    }
}

impl Default for structs::CustomizationIpSettingsIpV6AddressSpec {
    fn default() -> Self {
        Self {
            ip: Vec::new(),
            gateway: None,
        }
    }
}

impl Default for structs::CustomizationIdentification {
    fn default() -> Self {
        Self {
            join_workgroup: None,
            join_domain: None,
            domain_admin: None,
            domain_admin_password: None,
            domain_ou: None,
        }
    }
}

impl Default for structs::CustomizationIdentitySettings {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationCloudinitPrep {
    fn default() -> Self {
        Self {
            metadata: String::new(),
            userdata: None,
        }
    }
}

impl Default for structs::CustomizationLinuxPrep {
    fn default() -> Self {
        Self {
            host_name: Default::default(),
            domain: String::new(),
            time_zone: None,
            hw_clock_utc: None,
            script_text: None,
            compatible_customization_method: None,
        }
    }
}

impl Default for structs::CustomizationSysprep {
    fn default() -> Self {
        Self {
            gui_unattended: structs::CustomizationGuiUnattended::default(),
            user_data: structs::CustomizationUserData::default(),
            gui_run_once: None,
            identification: structs::CustomizationIdentification::default(),
            license_file_print_data: None,
        }
    }
}

impl Default for structs::CustomizationSysprepText {
    fn default() -> Self {
        Self {
            value: String::new(),
        }
    }
}

impl Default for structs::CustomizationIpGenerator {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationCustomIpGenerator {
    fn default() -> Self {
        Self {
            argument: None,
        }
    }
}

impl Default for structs::CustomizationDhcpIpGenerator {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationFixedIp {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
        }
    }
}

impl Default for structs::CustomizationUnknownIpGenerator {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationIpV6Generator {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationAutoIpV6Generator {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationCustomIpV6Generator {
    fn default() -> Self {
        Self {
            argument: None,
        }
    }
}

impl Default for structs::CustomizationDhcpIpV6Generator {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationFixedIpV6 {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            subnet_mask: 0,
        }
    }
}

impl Default for structs::CustomizationStatelessIpV6Generator {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationUnknownIpV6Generator {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationLicenseFilePrintData {
    fn default() -> Self {
        Self {
            auto_mode: enums::CustomizationLicenseDataModeEnum::default(),
            auto_users: None,
        }
    }
}

impl Default for structs::CustomizationName {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationCustomName {
    fn default() -> Self {
        Self {
            argument: None,
        }
    }
}

impl Default for structs::CustomizationFixedName {
    fn default() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Default for structs::CustomizationPrefixName {
    fn default() -> Self {
        Self {
            base: String::new(),
        }
    }
}

impl Default for structs::CustomizationUnknownName {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationVirtualMachineName {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationOptions {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationLinuxOptions {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::CustomizationWinOptions {
    fn default() -> Self {
        Self {
            change_sid: false,
            delete_accounts: false,
            reboot: None,
        }
    }
}

impl Default for structs::CustomizationPassword {
    fn default() -> Self {
        Self {
            value: String::new(),
            plain_text: false,
        }
    }
}

impl Default for structs::CustomizationSpec {
    fn default() -> Self {
        Self {
            options: None,
            identity: Default::default(),
            global_ip_settings: structs::CustomizationGlobalIpSettings::default(),
            nic_setting_map: None,
            encryption_key: None,
        }
    }
}

impl Default for structs::CustomizationUserData {
    fn default() -> Self {
        Self {
            full_name: String::new(),
            org_name: String::new(),
            computer_name: Default::default(),
            product_id: String::new(),
        }
    }
}

impl Default for structs::HostDiskMappingInfo {
    fn default() -> Self {
        Self {
            physical_partition: None,
            name: String::new(),
            exclusive: None,
        }
    }
}

impl Default for structs::HostDiskMappingPartitionInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            file_system: String::new(),
            capacity_in_kb: 0,
        }
    }
}

impl Default for structs::HostDiskMappingOption {
    fn default() -> Self {
        Self {
            physical_partition: None,
            name: String::new(),
        }
    }
}

impl Default for structs::HostDiskMappingPartitionOption {
    fn default() -> Self {
        Self {
            name: String::new(),
            file_system: String::new(),
            capacity_in_kb: 0,
        }
    }
}

impl Default for structs::VirtualDevice {
    fn default() -> Self {
        Self {
            key: 0,
            device_info: None,
            backing: None,
            connectable: None,
            slot_info: None,
            controller_key: None,
            unit_number: None,
            numa_node: None,
            device_group_info: None,
        }
    }
}

impl Default for structs::VirtualCdrom {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
        }
    }
}

impl Default for structs::VirtualController {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
            bus_number: 0,
            device: None,
        }
    }
}

impl Default for structs::VirtualIdeController {
    fn default() -> Self {
        Self {
            virtual_controller_: structs::VirtualController::default(),
        }
    }
}

impl Default for structs::VirtualNvdimmController {
    fn default() -> Self {
        Self {
            virtual_controller_: structs::VirtualController::default(),
        }
    }
}

impl Default for structs::VirtualNvmeController {
    fn default() -> Self {
        Self {
            virtual_controller_: structs::VirtualController::default(),
            shared_bus: None,
        }
    }
}

impl Default for structs::VirtualPciController {
    fn default() -> Self {
        Self {
            virtual_controller_: structs::VirtualController::default(),
        }
    }
}

impl Default for structs::VirtualPs2Controller {
    fn default() -> Self {
        Self {
            virtual_controller_: structs::VirtualController::default(),
        }
    }
}

impl Default for structs::VirtualSataController {
    fn default() -> Self {
        Self {
            virtual_controller_: structs::VirtualController::default(),
        }
    }
}

impl Default for structs::VirtualAhciController {
    fn default() -> Self {
        Self {
            virtual_sata_controller_: structs::VirtualSataController::default(),
        }
    }
}

impl Default for structs::VirtualScsiController {
    fn default() -> Self {
        Self {
            virtual_controller_: structs::VirtualController::default(),
            hot_add_remove: None,
            shared_bus: enums::VirtualScsiSharingEnum::default(),
            scsi_ctlr_unit_number: None,
        }
    }
}

impl Default for structs::ParaVirtualScsiController {
    fn default() -> Self {
        Self {
            virtual_scsi_controller_: structs::VirtualScsiController::default(),
        }
    }
}

impl Default for structs::VirtualBusLogicController {
    fn default() -> Self {
        Self {
            virtual_scsi_controller_: structs::VirtualScsiController::default(),
        }
    }
}

impl Default for structs::VirtualLsiLogicController {
    fn default() -> Self {
        Self {
            virtual_scsi_controller_: structs::VirtualScsiController::default(),
        }
    }
}

impl Default for structs::VirtualLsiLogicSasController {
    fn default() -> Self {
        Self {
            virtual_scsi_controller_: structs::VirtualScsiController::default(),
        }
    }
}

impl Default for structs::VirtualSioController {
    fn default() -> Self {
        Self {
            virtual_controller_: structs::VirtualController::default(),
        }
    }
}

impl Default for structs::VirtualUsbController {
    fn default() -> Self {
        Self {
            virtual_controller_: structs::VirtualController::default(),
            auto_connect_devices: None,
            ehci_enabled: None,
        }
    }
}

impl Default for structs::VirtualUsbxhciController {
    fn default() -> Self {
        Self {
            virtual_controller_: structs::VirtualController::default(),
            auto_connect_devices: None,
        }
    }
}

impl Default for structs::VirtualDisk {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
            capacity_in_kb: 0,
            capacity_in_bytes: None,
            shares: None,
            storage_io_allocation: None,
            disk_object_id: None,
            v_flash_cache_config_info: None,
            iofilter: None,
            v_disk_id: None,
            v_disk_version: None,
            virtual_disk_format: None,
            native_unmanaged_linked_clone: None,
            independent_filters: None,
            guest_read_only: None,
        }
    }
}

impl Default for structs::VirtualEthernetCard {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
            dynamic_property: None,
            address_type: None,
            mac_address: None,
            wake_on_lan_enabled: None,
            resource_allocation: None,
            external_id: None,
            upt_compatibility_enabled: None,
            subnet_id: None,
        }
    }
}

impl Default for structs::VirtualE1000 {
    fn default() -> Self {
        Self {
            virtual_ethernet_card_: structs::VirtualEthernetCard::default(),
        }
    }
}

impl Default for structs::VirtualE1000E {
    fn default() -> Self {
        Self {
            virtual_ethernet_card_: structs::VirtualEthernetCard::default(),
        }
    }
}

impl Default for structs::VirtualPcNet32 {
    fn default() -> Self {
        Self {
            virtual_ethernet_card_: structs::VirtualEthernetCard::default(),
        }
    }
}

impl Default for structs::VirtualSriovEthernetCard {
    fn default() -> Self {
        Self {
            virtual_ethernet_card_: structs::VirtualEthernetCard::default(),
            allow_guest_os_mtu_change: None,
            sriov_backing: None,
            dvx_backing_info: None,
        }
    }
}

impl Default for structs::VirtualVmxnet {
    fn default() -> Self {
        Self {
            virtual_ethernet_card_: structs::VirtualEthernetCard::default(),
        }
    }
}

impl Default for structs::VirtualVmxnet2 {
    fn default() -> Self {
        Self {
            virtual_vmxnet_: structs::VirtualVmxnet::default(),
        }
    }
}

impl Default for structs::VirtualVmxnet3 {
    fn default() -> Self {
        Self {
            virtual_vmxnet_: structs::VirtualVmxnet::default(),
            uptv_2_enabled: None,
            strict_latency_config: None,
        }
    }
}

impl Default for structs::VirtualVmxnet3Vrdma {
    fn default() -> Self {
        Self {
            virtual_vmxnet_3_: structs::VirtualVmxnet3::default(),
            device_protocol: None,
        }
    }
}

impl Default for structs::VirtualFloppy {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
        }
    }
}

impl Default for structs::VirtualKeyboard {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
        }
    }
}

impl Default for structs::VirtualNvdimm {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
            capacity_in_mb: 0,
            configured_capacity_in_mb: None,
        }
    }
}

impl Default for structs::VirtualPciPassthrough {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
        }
    }
}

impl Default for structs::VirtualParallelPort {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
        }
    }
}

impl Default for structs::VirtualPointingDevice {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
        }
    }
}

impl Default for structs::VirtualPrecisionClock {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
        }
    }
}

impl Default for structs::VirtualScsiPassthrough {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
        }
    }
}

impl Default for structs::VirtualSerialPort {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
            yield_on_poll: false,
        }
    }
}

impl Default for structs::VirtualSoundCard {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
        }
    }
}

impl Default for structs::VirtualEnsoniq1371 {
    fn default() -> Self {
        Self {
            virtual_sound_card_: structs::VirtualSoundCard::default(),
        }
    }
}

impl Default for structs::VirtualHdAudioCard {
    fn default() -> Self {
        Self {
            virtual_sound_card_: structs::VirtualSoundCard::default(),
        }
    }
}

impl Default for structs::VirtualSoundBlaster16 {
    fn default() -> Self {
        Self {
            virtual_sound_card_: structs::VirtualSoundCard::default(),
        }
    }
}

impl Default for structs::VirtualTpm {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
            endorsement_key_certificate_signing_request: None,
            endorsement_key_certificate: None,
        }
    }
}

impl Default for structs::VirtualUsb {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
            connected: false,
            vendor: None,
            product: None,
            family: None,
            speed: None,
        }
    }
}

impl Default for structs::VirtualMachineVmciDevice {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
            id: None,
            allow_unrestricted_communication: None,
            filter_enable: None,
            filter_info: None,
        }
    }
}

impl Default for structs::VirtualMachineVmirom {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
        }
    }
}

impl Default for structs::VirtualMachineVideoCard {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
            video_ram_size_in_kb: None,
            num_displays: None,
            use_auto_detect: None,
            enable_3_d_support: None,
            use_3_d_renderer: None,
            graphics_memory_size_in_kb: None,
        }
    }
}

impl Default for structs::VirtualWdt {
    fn default() -> Self {
        Self {
            virtual_device_: structs::VirtualDevice::default(),
            run_on_boot: false,
            running: false,
        }
    }
}

impl Default for structs::VirtualDeviceBackingInfo {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualDeviceDeviceBackingInfo {
    fn default() -> Self {
        Self {
            device_name: String::new(),
            use_auto_detect: None,
        }
    }
}

impl Default for structs::VirtualCdromAtapiBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualCdromPassthroughBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
            exclusive: false,
        }
    }
}

impl Default for structs::VirtualDiskRawDiskVer2BackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
            descriptor_file_name: String::new(),
            uuid: None,
            change_id: None,
            sharing: None,
        }
    }
}

impl Default for structs::VirtualDiskPartitionedRawDiskVer2BackingInfo {
    fn default() -> Self {
        Self {
            virtual_disk_raw_disk_ver_2_backing_info_: structs::VirtualDiskRawDiskVer2BackingInfo::default(),
            partition: Vec::new(),
        }
    }
}

impl Default for structs::VirtualEthernetCardLegacyNetworkBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualEthernetCardNetworkBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
            network: None,
            in_passthrough_mode: None,
        }
    }
}

impl Default for structs::VirtualFloppyDeviceBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualPciPassthroughDeviceBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
            id: String::new(),
            device_id: String::new(),
            system_id: String::new(),
            vendor_id: 0,
        }
    }
}

impl Default for structs::VirtualPciPassthroughDynamicBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
            allowed_device: None,
            custom_label: None,
            assigned_id: None,
        }
    }
}

impl Default for structs::VirtualParallelPortDeviceBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualPointingDeviceDeviceBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
            host_pointing_device: String::new(),
        }
    }
}

impl Default for structs::VirtualScsiPassthroughDeviceBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualSerialPortDeviceBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualSoundCardDeviceBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualUsbRemoteHostBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
            hostname: String::new(),
        }
    }
}

impl Default for structs::VirtualUsbusbBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_info_: structs::VirtualDeviceDeviceBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualDeviceFileBackingInfo {
    fn default() -> Self {
        Self {
            file_name: String::new(),
            datastore: None,
            backing_object_id: None,
        }
    }
}

impl Default for structs::VirtualCdromIsoBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualDiskFlatVer1BackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
            disk_mode: String::new(),
            split: None,
            write_through: None,
            content_id: None,
            parent: None,
        }
    }
}

impl Default for structs::VirtualDiskFlatVer2BackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
            disk_mode: String::new(),
            split: None,
            write_through: None,
            thin_provisioned: None,
            eagerly_scrub: None,
            uuid: None,
            content_id: None,
            change_id: None,
            parent: None,
            delta_disk_format: None,
            digest_enabled: None,
            delta_grain_size: None,
            delta_disk_format_variant: None,
            sharing: None,
            key_id: None,
        }
    }
}

impl Default for structs::VirtualDiskLocalPMemBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
            disk_mode: String::new(),
            uuid: None,
            volume_uuid: None,
            content_id: None,
        }
    }
}

impl Default for structs::VirtualDiskRawDiskMappingVer1BackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
            lun_uuid: None,
            device_name: None,
            compatibility_mode: None,
            disk_mode: None,
            uuid: None,
            content_id: None,
            change_id: None,
            parent: None,
            delta_disk_format: None,
            delta_grain_size: None,
            sharing: None,
        }
    }
}

impl Default for structs::VirtualDiskSeSparseBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
            disk_mode: String::new(),
            write_through: None,
            uuid: None,
            content_id: None,
            change_id: None,
            parent: None,
            delta_disk_format: None,
            digest_enabled: None,
            grain_size: None,
            key_id: None,
        }
    }
}

impl Default for structs::VirtualDiskSparseVer1BackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
            disk_mode: String::new(),
            split: None,
            write_through: None,
            space_used_in_kb: None,
            content_id: None,
            parent: None,
        }
    }
}

impl Default for structs::VirtualDiskSparseVer2BackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
            disk_mode: String::new(),
            split: None,
            write_through: None,
            space_used_in_kb: None,
            uuid: None,
            content_id: None,
            change_id: None,
            parent: None,
            key_id: None,
        }
    }
}

impl Default for structs::VirtualFloppyImageBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualNvdimmBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
            parent: None,
            change_id: None,
        }
    }
}

impl Default for structs::VirtualParallelPortFileBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualSerialPortFileBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_info_: structs::VirtualDeviceFileBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualDevicePipeBackingInfo {
    fn default() -> Self {
        Self {
            pipe_name: String::new(),
        }
    }
}

impl Default for structs::VirtualSerialPortPipeBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_pipe_backing_info_: structs::VirtualDevicePipeBackingInfo::default(),
            endpoint: String::new(),
            no_rx_loss: None,
        }
    }
}

impl Default for structs::VirtualDeviceRemoteDeviceBackingInfo {
    fn default() -> Self {
        Self {
            device_name: String::new(),
            use_auto_detect: None,
        }
    }
}

impl Default for structs::VirtualCdromRemoteAtapiBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_remote_device_backing_info_: structs::VirtualDeviceRemoteDeviceBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualCdromRemotePassthroughBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_remote_device_backing_info_: structs::VirtualDeviceRemoteDeviceBackingInfo::default(),
            exclusive: false,
        }
    }
}

impl Default for structs::VirtualFloppyRemoteDeviceBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_remote_device_backing_info_: structs::VirtualDeviceRemoteDeviceBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualUsbRemoteClientBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_remote_device_backing_info_: structs::VirtualDeviceRemoteDeviceBackingInfo::default(),
            hostname: String::new(),
        }
    }
}

impl Default for structs::VirtualDeviceUriBackingInfo {
    fn default() -> Self {
        Self {
            service_uri: String::new(),
            direction: String::new(),
            proxy_uri: None,
        }
    }
}

impl Default for structs::VirtualSerialPortUriBackingInfo {
    fn default() -> Self {
        Self {
            virtual_device_uri_backing_info_: structs::VirtualDeviceUriBackingInfo::default(),
        }
    }
}

impl Default for structs::VirtualEthernetCardDistributedVirtualPortBackingInfo {
    fn default() -> Self {
        Self {
            port: structs::DistributedVirtualSwitchPortConnection::default(),
        }
    }
}

impl Default for structs::VirtualEthernetCardOpaqueNetworkBackingInfo {
    fn default() -> Self {
        Self {
            opaque_network_id: String::new(),
            opaque_network_type: String::new(),
        }
    }
}

impl Default for structs::VirtualPciPassthroughDvxBackingInfo {
    fn default() -> Self {
        Self {
            device_class: None,
            config_params: None,
        }
    }
}

impl Default for structs::VirtualPciPassthroughPluginBackingInfo {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualPciPassthroughVmiopBackingInfo {
    fn default() -> Self {
        Self {
            vgpu: None,
            vgpu_migrate_data_size_mb: None,
            migrate_supported: None,
            enhanced_migrate_capability: None,
        }
    }
}

impl Default for structs::VirtualPrecisionClockSystemClockBackingInfo {
    fn default() -> Self {
        Self {
            protocol: None,
        }
    }
}

impl Default for structs::VirtualSerialPortThinPrintBackingInfo {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualSriovEthernetCardSriovBackingInfo {
    fn default() -> Self {
        Self {
            physical_function_backing: None,
            virtual_function_backing: None,
            virtual_function_index: None,
        }
    }
}

impl Default for structs::VirtualDeviceBusSlotInfo {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VirtualDevicePciBusSlotInfo {
    fn default() -> Self {
        Self {
            pci_slot_number: 0,
        }
    }
}

impl Default for structs::VirtualUsbControllerPciBusSlotInfo {
    fn default() -> Self {
        Self {
            virtual_device_pci_bus_slot_info_: structs::VirtualDevicePciBusSlotInfo::default(),
            ehci_pci_slot_number: None,
        }
    }
}

impl Default for structs::VirtualDeviceConnectInfo {
    fn default() -> Self {
        Self {
            migrate_connect: None,
            start_connected: false,
            allow_guest_control: false,
            connected: false,
            status: None,
        }
    }
}

impl Default for structs::VirtualDeviceDeviceGroupInfo {
    fn default() -> Self {
        Self {
            group_instance_key: 0,
            sequence_id: 0,
        }
    }
}

impl Default for structs::VirtualDeviceOption {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            connect_option: None,
            bus_slot_option: None,
            controller_type: None,
            auto_assign_controller: None,
            backing_option: None,
            default_backing_option_index: None,
            licensing_limit: None,
            deprecated: false,
            plug_and_play: false,
            hot_remove_supported: false,
            numa_supported: None,
        }
    }
}

impl Default for structs::VirtualCdromOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
        }
    }
}

impl Default for structs::VirtualControllerOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
            devices: structs::IntOption::default(),
            supported_device: None,
        }
    }
}

impl Default for structs::VirtualIdeControllerOption {
    fn default() -> Self {
        Self {
            virtual_controller_option_: structs::VirtualControllerOption::default(),
            num_ide_disks: structs::IntOption::default(),
            num_ide_cdroms: structs::IntOption::default(),
        }
    }
}

impl Default for structs::VirtualNvdimmControllerOption {
    fn default() -> Self {
        Self {
            virtual_controller_option_: structs::VirtualControllerOption::default(),
            num_nvdimm_controllers: structs::IntOption::default(),
        }
    }
}

impl Default for structs::VirtualNvmeControllerOption {
    fn default() -> Self {
        Self {
            virtual_controller_option_: structs::VirtualControllerOption::default(),
            num_nvme_disks: structs::IntOption::default(),
            sharing: None,
        }
    }
}

impl Default for structs::VirtualPciControllerOption {
    fn default() -> Self {
        Self {
            virtual_controller_option_: structs::VirtualControllerOption::default(),
            num_scsi_controllers: structs::IntOption::default(),
            num_ethernet_cards: structs::IntOption::default(),
            num_video_cards: structs::IntOption::default(),
            num_sound_cards: structs::IntOption::default(),
            num_vmi_roms: structs::IntOption::default(),
            num_vmci_devices: structs::IntOption::default(),
            num_pci_passthrough_devices: structs::IntOption::default(),
            num_sas_scsi_controllers: structs::IntOption::default(),
            num_vmxnet_3_ethernet_cards: structs::IntOption::default(),
            num_para_virtual_scsi_controllers: structs::IntOption::default(),
            num_sata_controllers: structs::IntOption::default(),
            num_nvme_controllers: None,
            num_vmxnet_3_vrdma_ethernet_cards: None,
        }
    }
}

impl Default for structs::VirtualPs2ControllerOption {
    fn default() -> Self {
        Self {
            virtual_controller_option_: structs::VirtualControllerOption::default(),
            num_keyboards: structs::IntOption::default(),
            num_pointing_devices: structs::IntOption::default(),
        }
    }
}

impl Default for structs::VirtualSataControllerOption {
    fn default() -> Self {
        Self {
            virtual_controller_option_: structs::VirtualControllerOption::default(),
            num_sata_disks: structs::IntOption::default(),
            num_sata_cdroms: structs::IntOption::default(),
        }
    }
}

impl Default for structs::VirtualAhciControllerOption {
    fn default() -> Self {
        Self {
            virtual_sata_controller_option_: structs::VirtualSataControllerOption::default(),
        }
    }
}

impl Default for structs::VirtualScsiControllerOption {
    fn default() -> Self {
        Self {
            virtual_controller_option_: structs::VirtualControllerOption::default(),
            num_scsi_disks: structs::IntOption::default(),
            num_scsi_cdroms: structs::IntOption::default(),
            num_scsi_passthrough: structs::IntOption::default(),
            sharing: Vec::new(),
            default_shared_index: 0,
            hot_add_remove: structs::BoolOption::default(),
            scsi_ctlr_unit_number: 0,
        }
    }
}

impl Default for structs::ParaVirtualScsiControllerOption {
    fn default() -> Self {
        Self {
            virtual_scsi_controller_option_: structs::VirtualScsiControllerOption::default(),
        }
    }
}

impl Default for structs::VirtualBusLogicControllerOption {
    fn default() -> Self {
        Self {
            virtual_scsi_controller_option_: structs::VirtualScsiControllerOption::default(),
        }
    }
}

impl Default for structs::VirtualLsiLogicControllerOption {
    fn default() -> Self {
        Self {
            virtual_scsi_controller_option_: structs::VirtualScsiControllerOption::default(),
        }
    }
}

impl Default for structs::VirtualLsiLogicSasControllerOption {
    fn default() -> Self {
        Self {
            virtual_scsi_controller_option_: structs::VirtualScsiControllerOption::default(),
        }
    }
}

impl Default for structs::VirtualSioControllerOption {
    fn default() -> Self {
        Self {
            virtual_controller_option_: structs::VirtualControllerOption::default(),
            num_floppy_drives: structs::IntOption::default(),
            num_serial_ports: structs::IntOption::default(),
            num_parallel_ports: structs::IntOption::default(),
        }
    }
}

impl Default for structs::VirtualUsbControllerOption {
    fn default() -> Self {
        Self {
            virtual_controller_option_: structs::VirtualControllerOption::default(),
            auto_connect_devices: structs::BoolOption::default(),
            ehci_supported: structs::BoolOption::default(),
            supported_speeds: Vec::new(),
        }
    }
}

impl Default for structs::VirtualUsbxhciControllerOption {
    fn default() -> Self {
        Self {
            virtual_controller_option_: structs::VirtualControllerOption::default(),
            auto_connect_devices: structs::BoolOption::default(),
            supported_speeds: Vec::new(),
        }
    }
}

impl Default for structs::VirtualDiskOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
            capacity_in_kb: structs::LongOption::default(),
            io_allocation_option: structs::StorageIoAllocationOption::default(),
            v_flash_cache_config_option: None,
        }
    }
}

impl Default for structs::VirtualEthernetCardOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
            supported_oui: structs::ChoiceOption::default(),
            mac_type: structs::ChoiceOption::default(),
            wake_on_lan_enabled: structs::BoolOption::default(),
            vm_direct_path_gen_2_supported: None,
            upt_compatibility_enabled: None,
        }
    }
}

impl Default for structs::VirtualE1000Option {
    fn default() -> Self {
        Self {
            virtual_ethernet_card_option_: structs::VirtualEthernetCardOption::default(),
        }
    }
}

impl Default for structs::VirtualE1000EOption {
    fn default() -> Self {
        Self {
            virtual_ethernet_card_option_: structs::VirtualEthernetCardOption::default(),
        }
    }
}

impl Default for structs::VirtualPcNet32Option {
    fn default() -> Self {
        Self {
            virtual_ethernet_card_option_: structs::VirtualEthernetCardOption::default(),
            supports_morphing: false,
        }
    }
}

impl Default for structs::VirtualSriovEthernetCardOption {
    fn default() -> Self {
        Self {
            virtual_ethernet_card_option_: structs::VirtualEthernetCardOption::default(),
        }
    }
}

impl Default for structs::VirtualVmxnetOption {
    fn default() -> Self {
        Self {
            virtual_ethernet_card_option_: structs::VirtualEthernetCardOption::default(),
        }
    }
}

impl Default for structs::VirtualVmxnet2Option {
    fn default() -> Self {
        Self {
            virtual_vmxnet_option_: structs::VirtualVmxnetOption::default(),
        }
    }
}

impl Default for structs::VirtualVmxnet3Option {
    fn default() -> Self {
        Self {
            virtual_vmxnet_option_: structs::VirtualVmxnetOption::default(),
            uptv_2_enabled: None,
            strict_latency_config_option: None,
        }
    }
}

impl Default for structs::VirtualVmxnet3VrdmaOption {
    fn default() -> Self {
        Self {
            virtual_vmxnet_3_option_: structs::VirtualVmxnet3Option::default(),
            device_protocol: None,
        }
    }
}

impl Default for structs::VirtualFloppyOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
        }
    }
}

impl Default for structs::VirtualKeyboardOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
        }
    }
}

impl Default for structs::VirtualNvdimmOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
            capacity_in_mb: structs::LongOption::default(),
            growable: false,
            hot_growable: false,
            granularity_in_mb: 0,
        }
    }
}

impl Default for structs::VirtualPciPassthroughOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
        }
    }
}

impl Default for structs::VirtualParallelPortOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
        }
    }
}

impl Default for structs::VirtualPointingDeviceOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
        }
    }
}

impl Default for structs::VirtualPrecisionClockOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
        }
    }
}

impl Default for structs::VirtualScsiPassthroughOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
        }
    }
}

impl Default for structs::VirtualSerialPortOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
            yield_on_poll: structs::BoolOption::default(),
        }
    }
}

impl Default for structs::VirtualSoundCardOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
        }
    }
}

impl Default for structs::VirtualEnsoniq1371Option {
    fn default() -> Self {
        Self {
            virtual_sound_card_option_: structs::VirtualSoundCardOption::default(),
        }
    }
}

impl Default for structs::VirtualHdAudioCardOption {
    fn default() -> Self {
        Self {
            virtual_sound_card_option_: structs::VirtualSoundCardOption::default(),
        }
    }
}

impl Default for structs::VirtualSoundBlaster16Option {
    fn default() -> Self {
        Self {
            virtual_sound_card_option_: structs::VirtualSoundCardOption::default(),
        }
    }
}

impl Default for structs::VirtualTpmOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
            supported_firmware: None,
        }
    }
}

impl Default for structs::VirtualUsbOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
        }
    }
}

impl Default for structs::VirtualMachineVmciDeviceOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
            allow_unrestricted_communication: structs::BoolOption::default(),
            filter_spec_option: None,
            filter_supported: None,
        }
    }
}

impl Default for structs::VirtualVmiromOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
        }
    }
}

impl Default for structs::VirtualVideoCardOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
            video_ram_size_in_kb: None,
            num_displays: None,
            use_auto_detect: None,
            support_3_d: None,
            use_3_d_renderer_supported: None,
            graphics_memory_size_in_kb: None,
            graphics_memory_size_supported: None,
        }
    }
}

impl Default for structs::VirtualWdtOption {
    fn default() -> Self {
        Self {
            virtual_device_option_: structs::VirtualDeviceOption::default(),
            run_on_boot: structs::BoolOption::default(),
        }
    }
}

impl Default for structs::VirtualDeviceBackingOption {
    fn default() -> Self {
        Self {
            r#type: String::new(),
        }
    }
}

impl Default for structs::VirtualDeviceDeviceBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
            auto_detect_available: structs::BoolOption::default(),
        }
    }
}

impl Default for structs::VirtualCdromAtapiBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualCdromPassthroughBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
            exclusive: structs::BoolOption::default(),
        }
    }
}

impl Default for structs::VirtualCdromRemoteAtapiBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualDiskRawDiskMappingVer1BackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
            descriptor_file_name_extensions: None,
            compatibility_mode: structs::ChoiceOption::default(),
            disk_mode: structs::ChoiceOption::default(),
            uuid: false,
            virtual_disk_format: None,
        }
    }
}

impl Default for structs::VirtualDiskRawDiskVer2BackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
            descriptor_file_name_extensions: structs::ChoiceOption::default(),
            uuid: false,
        }
    }
}

impl Default for structs::VirtualDiskPartitionedRawDiskVer2BackingOption {
    fn default() -> Self {
        Self {
            virtual_disk_raw_disk_ver_2_backing_option_: structs::VirtualDiskRawDiskVer2BackingOption::default(),
        }
    }
}

impl Default for structs::VirtualEthernetCardLegacyNetworkBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualEthernetCardNetworkBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualFloppyDeviceBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualPciPassthroughDeviceBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualPciPassthroughDynamicBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualParallelPortDeviceBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualPointingDeviceBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
            host_pointing_device: structs::ChoiceOption::default(),
        }
    }
}

impl Default for structs::VirtualScsiPassthroughDeviceBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualSerialPortDeviceBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualSoundCardDeviceBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualUsbRemoteHostBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualUsbusbBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_device_backing_option_: structs::VirtualDeviceDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualDeviceFileBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
            file_name_extensions: None,
        }
    }
}

impl Default for structs::VirtualCdromIsoBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_option_: structs::VirtualDeviceFileBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualDiskFlatVer1BackingOption {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_option_: structs::VirtualDeviceFileBackingOption::default(),
            disk_mode: structs::ChoiceOption::default(),
            split: structs::BoolOption::default(),
            write_through: structs::BoolOption::default(),
            growable: false,
        }
    }
}

impl Default for structs::VirtualDiskFlatVer2BackingOption {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_option_: structs::VirtualDeviceFileBackingOption::default(),
            disk_mode: structs::ChoiceOption::default(),
            split: structs::BoolOption::default(),
            write_through: structs::BoolOption::default(),
            growable: false,
            hot_growable: false,
            uuid: false,
            thin_provisioned: structs::BoolOption::default(),
            eagerly_scrub: structs::BoolOption::default(),
            delta_disk_format: structs::ChoiceOption::default(),
            delta_disk_formats_supported: Vec::new(),
            virtual_disk_format: None,
        }
    }
}

impl Default for structs::VirtualDiskLocalPMemBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_option_: structs::VirtualDeviceFileBackingOption::default(),
            disk_mode: structs::ChoiceOption::default(),
            growable: false,
            hot_growable: false,
            uuid: false,
        }
    }
}

impl Default for structs::VirtualDiskSeSparseBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_option_: structs::VirtualDeviceFileBackingOption::default(),
            disk_mode: structs::ChoiceOption::default(),
            write_through: structs::BoolOption::default(),
            growable: false,
            hot_growable: false,
            uuid: false,
            delta_disk_formats_supported: Vec::new(),
            virtual_disk_format: None,
        }
    }
}

impl Default for structs::VirtualDiskSparseVer1BackingOption {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_option_: structs::VirtualDeviceFileBackingOption::default(),
            disk_modes: structs::ChoiceOption::default(),
            split: structs::BoolOption::default(),
            write_through: structs::BoolOption::default(),
            growable: false,
        }
    }
}

impl Default for structs::VirtualDiskSparseVer2BackingOption {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_option_: structs::VirtualDeviceFileBackingOption::default(),
            disk_mode: structs::ChoiceOption::default(),
            split: structs::BoolOption::default(),
            write_through: structs::BoolOption::default(),
            growable: false,
            hot_growable: false,
            uuid: false,
            virtual_disk_format: None,
        }
    }
}

impl Default for structs::VirtualFloppyImageBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_option_: structs::VirtualDeviceFileBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualParallelPortFileBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_option_: structs::VirtualDeviceFileBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualSerialPortFileBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_file_backing_option_: structs::VirtualDeviceFileBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualDevicePipeBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualSerialPortPipeBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_pipe_backing_option_: structs::VirtualDevicePipeBackingOption::default(),
            endpoint: structs::ChoiceOption::default(),
            no_rx_loss: structs::BoolOption::default(),
        }
    }
}

impl Default for structs::VirtualDeviceRemoteDeviceBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
            auto_detect_available: structs::BoolOption::default(),
        }
    }
}

impl Default for structs::VirtualCdromRemotePassthroughBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_remote_device_backing_option_: structs::VirtualDeviceRemoteDeviceBackingOption::default(),
            exclusive: structs::BoolOption::default(),
        }
    }
}

impl Default for structs::VirtualFloppyRemoteDeviceBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_remote_device_backing_option_: structs::VirtualDeviceRemoteDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualUsbRemoteClientBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_remote_device_backing_option_: structs::VirtualDeviceRemoteDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualDeviceUriBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
            directions: structs::ChoiceOption::default(),
        }
    }
}

impl Default for structs::VirtualSerialPortUriBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_uri_backing_option_: structs::VirtualDeviceUriBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualEthernetCardDvPortBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualEthernetCardOpaqueNetworkBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualPciPassthroughDvxBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualPciPassthroughPluginBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualPciPassthroughVmiopBackingOption {
    fn default() -> Self {
        Self {
            virtual_pci_passthrough_plugin_backing_option_: structs::VirtualPciPassthroughPluginBackingOption::default(),
            vgpu: structs::StringOption::default(),
            max_instances: 0,
        }
    }
}

impl Default for structs::VirtualPrecisionClockSystemClockBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
            protocol: structs::ChoiceOption::default(),
        }
    }
}

impl Default for structs::VirtualSerialPortThinPrintBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualSriovEthernetCardSriovBackingOption {
    fn default() -> Self {
        Self {
            virtual_device_backing_option_: structs::VirtualDeviceBackingOption::default(),
        }
    }
}

impl Default for structs::VirtualDeviceBusSlotOption {
    fn default() -> Self {
        Self {
            r#type: String::new(),
        }
    }
}

impl Default for structs::VirtualDeviceConnectOption {
    fn default() -> Self {
        Self {
            start_connected: structs::BoolOption::default(),
            allow_guest_control: structs::BoolOption::default(),
        }
    }
}

impl Default for structs::VirtualDeviceConfigSpec {
    fn default() -> Self {
        Self {
            operation: None,
            file_operation: None,
            device: Default::default(),
            profile: None,
            backing: None,
            filter_spec: None,
            change_mode: None,
        }
    }
}

impl Default for structs::VirtualDiskConfigSpec {
    fn default() -> Self {
        Self {
            virtual_device_config_spec_: structs::VirtualDeviceConfigSpec::default(),
            disk_move_type: None,
            migrate_cache: None,
        }
    }
}

impl Default for structs::VirtualDeviceConfigSpecBackingSpec {
    fn default() -> Self {
        Self {
            parent: None,
            crypto: None,
        }
    }
}

impl Default for structs::VirtualDiskVFlashCacheConfigInfo {
    fn default() -> Self {
        Self {
            v_flash_module: None,
            reservation_in_mb: None,
            cache_consistency_type: None,
            cache_mode: None,
            block_size_in_kb: None,
        }
    }
}

impl Default for structs::VirtualDiskId {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            disk_id: 0,
        }
    }
}

impl Default for structs::VirtualDiskDeltaDiskFormatsSupported {
    fn default() -> Self {
        Self {
            datastore_type: String::new(),
            delta_disk_format: structs::ChoiceOption::default(),
        }
    }
}

impl Default for structs::VirtualDiskOptionVFlashCacheConfigOption {
    fn default() -> Self {
        Self {
            cache_consistency_type: structs::ChoiceOption::default(),
            cache_mode: structs::ChoiceOption::default(),
            reservation_in_mb: structs::LongOption::default(),
            block_size_in_kb: structs::LongOption::default(),
        }
    }
}

impl Default for structs::VirtualEthernetCardResourceAllocation {
    fn default() -> Self {
        Self {
            reservation: None,
            share: structs::SharesInfo::default(),
            limit: None,
        }
    }
}

impl Default for structs::VirtualPciPassthroughAllowedDevice {
    fn default() -> Self {
        Self {
            vendor_id: 0,
            device_id: 0,
            sub_vendor_id: None,
            sub_device_id: None,
            revision_id: None,
        }
    }
}

impl Default for structs::VirtualMachineVmciDeviceFilterInfo {
    fn default() -> Self {
        Self {
            filters: None,
        }
    }
}

impl Default for structs::VirtualMachineVmciDeviceFilterSpec {
    fn default() -> Self {
        Self {
            rank: 0,
            action: String::new(),
            protocol: String::new(),
            direction: String::new(),
            lower_dst_port_boundary: None,
            upper_dst_port_boundary: None,
        }
    }
}

impl Default for structs::VirtualMachineVmciDeviceOptionFilterSpecOption {
    fn default() -> Self {
        Self {
            action: structs::ChoiceOption::default(),
            protocol: structs::ChoiceOption::default(),
            direction: structs::ChoiceOption::default(),
            lower_dst_port_boundary: structs::LongOption::default(),
            upper_dst_port_boundary: structs::LongOption::default(),
        }
    }
}

impl Default for structs::VirtualVmxnet3StrictLatencyConfig {
    fn default() -> Self {
        Self {
            allowed: None,
            measure_latency: None,
            max_tx_queues: None,
            max_rx_queues: None,
            tx_data_ring_desc_size: None,
            rx_data_ring_desc_size: None,
            disable_offload: None,
        }
    }
}

impl Default for structs::VirtualVmxnet3OptionStrictLatencyConfigOption {
    fn default() -> Self {
        Self {
            allowed: structs::BoolOption::default(),
            measure_latency: structs::BoolOption::default(),
            max_tx_queues: structs::IntOption::default(),
            max_rx_queues: structs::IntOption::default(),
            tx_data_ring_desc_size: structs::IntOption::default(),
            rx_data_ring_desc_size: structs::IntOption::default(),
            disable_offload: structs::ChoiceOption::default(),
        }
    }
}

impl Default for structs::GuestAliases {
    fn default() -> Self {
        Self {
            base_64_cert: String::new(),
            aliases: Vec::new(),
        }
    }
}

impl Default for structs::GuestAuthAliasInfo {
    fn default() -> Self {
        Self {
            subject: Default::default(),
            comment: String::new(),
        }
    }
}

impl Default for structs::GuestAuthSubject {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::GuestAuthAnySubject {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::GuestAuthNamedSubject {
    fn default() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Default for structs::GuestMappedAliases {
    fn default() -> Self {
        Self {
            base_64_cert: String::new(),
            username: String::new(),
            subjects: Vec::new(),
        }
    }
}

impl Default for structs::GuestFileAttributes {
    fn default() -> Self {
        Self {
            modification_time: None,
            access_time: None,
            symlink_target: None,
        }
    }
}

impl Default for structs::GuestPosixFileAttributes {
    fn default() -> Self {
        Self {
            guest_file_attributes_: structs::GuestFileAttributes::default(),
            owner_id: None,
            group_id: None,
            permissions: None,
        }
    }
}

impl Default for structs::GuestWindowsFileAttributes {
    fn default() -> Self {
        Self {
            guest_file_attributes_: structs::GuestFileAttributes::default(),
            hidden: None,
            read_only: None,
            create_time: None,
        }
    }
}

impl Default for structs::GuestFileInfo {
    fn default() -> Self {
        Self {
            path: String::new(),
            r#type: String::new(),
            size: 0,
            attributes: Default::default(),
        }
    }
}

impl Default for structs::FileTransferInformation {
    fn default() -> Self {
        Self {
            attributes: Default::default(),
            size: 0,
            url: String::new(),
        }
    }
}

impl Default for structs::GuestListFileInfo {
    fn default() -> Self {
        Self {
            files: None,
            remaining: 0,
        }
    }
}

impl Default for structs::GuestAuthentication {
    fn default() -> Self {
        Self {
            interactive_session: false,
        }
    }
}

impl Default for structs::NamePasswordAuthentication {
    fn default() -> Self {
        Self {
            guest_authentication_: structs::GuestAuthentication::default(),
            username: String::new(),
            password: String::new(),
        }
    }
}

impl Default for structs::SamlTokenAuthentication {
    fn default() -> Self {
        Self {
            guest_authentication_: structs::GuestAuthentication::default(),
            token: String::new(),
            username: None,
        }
    }
}

impl Default for structs::SspiAuthentication {
    fn default() -> Self {
        Self {
            guest_authentication_: structs::GuestAuthentication::default(),
            sspi_token: String::new(),
        }
    }
}

impl Default for structs::TicketedSessionAuthentication {
    fn default() -> Self {
        Self {
            guest_authentication_: structs::GuestAuthentication::default(),
            ticket: String::new(),
        }
    }
}

impl Default for structs::GuestProcessInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            pid: 0,
            owner: String::new(),
            cmd_line: String::new(),
            start_time: String::new(),
            end_time: None,
            exit_code: None,
        }
    }
}

impl Default for structs::GuestProgramSpec {
    fn default() -> Self {
        Self {
            program_path: String::new(),
            arguments: String::new(),
            working_directory: None,
            env_variables: None,
        }
    }
}

impl Default for structs::GuestWindowsProgramSpec {
    fn default() -> Self {
        Self {
            guest_program_spec_: structs::GuestProgramSpec::default(),
            start_minimized: false,
        }
    }
}

impl Default for structs::GuestRegKeySpec {
    fn default() -> Self {
        Self {
            key_name: structs::GuestRegKeyNameSpec::default(),
            class_type: String::new(),
            last_written: String::new(),
        }
    }
}

impl Default for structs::GuestRegKeyNameSpec {
    fn default() -> Self {
        Self {
            registry_path: String::new(),
            wow_bitness: String::new(),
        }
    }
}

impl Default for structs::GuestRegKeyRecordSpec {
    fn default() -> Self {
        Self {
            key: structs::GuestRegKeySpec::default(),
            fault: None,
        }
    }
}

impl Default for structs::GuestRegValueSpec {
    fn default() -> Self {
        Self {
            name: structs::GuestRegValueNameSpec::default(),
            data: Default::default(),
        }
    }
}

impl Default for structs::GuestRegValueDataSpec {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::GuestRegValueBinarySpec {
    fn default() -> Self {
        Self {
            value: None,
        }
    }
}

impl Default for structs::GuestRegValueDwordSpec {
    fn default() -> Self {
        Self {
            value: 0,
        }
    }
}

impl Default for structs::GuestRegValueExpandStringSpec {
    fn default() -> Self {
        Self {
            value: None,
        }
    }
}

impl Default for structs::GuestRegValueMultiStringSpec {
    fn default() -> Self {
        Self {
            value: None,
        }
    }
}

impl Default for structs::GuestRegValueQwordSpec {
    fn default() -> Self {
        Self {
            value: 0,
        }
    }
}

impl Default for structs::GuestRegValueStringSpec {
    fn default() -> Self {
        Self {
            value: None,
        }
    }
}

impl Default for structs::GuestRegValueNameSpec {
    fn default() -> Self {
        Self {
            key_name: structs::GuestRegKeyNameSpec::default(),
            name: String::new(),
        }
    }
}

impl Default for structs::DeviceGroupId {
    fn default() -> Self {
        Self {
            id: String::new(),
        }
    }
}

impl Default for structs::FaultDomainId {
    fn default() -> Self {
        Self {
            id: String::new(),
        }
    }
}

impl Default for structs::FaultDomainInfo {
    fn default() -> Self {
        Self {
            fault_domain_id_: structs::FaultDomainId::default(),
            name: None,
            description: None,
            storage_array_id: None,
            children: None,
            provider: None,
        }
    }
}

impl Default for structs::ReplicationGroupId {
    fn default() -> Self {
        Self {
            fault_domain_id: Default::default(),
            device_group_id: structs::DeviceGroupId::default(),
        }
    }
}

impl Default for structs::ReplicationSpec {
    fn default() -> Self {
        Self {
            replication_group_id: structs::ReplicationGroupId::default(),
        }
    }
}

impl Default for structs::VsanCapacityReservationInfo {
    fn default() -> Self {
        Self {
            host_rebuild_threshold: None,
            vsan_op_space_threshold: None,
        }
    }
}

impl Default for structs::ClusterRuntimeInfo {
    fn default() -> Self {
        Self {
            cluster_uuid: String::new(),
            total_components_count: 0,
            cluster: None,
        }
    }
}

impl Default for structs::VsanCompatibilityCheckResult {
    fn default() -> Self {
        Self {
            status: String::new(),
            message: None,
        }
    }
}

impl Default for structs::VimVsanDataEfficiencyCapacityState {
    fn default() -> Self {
        Self {
            logical_capacity: None,
            logical_capacity_used: None,
            physical_capacity: None,
            physical_capacity_used: None,
            dedup_metadata_size: None,
            space_efficiency_metadata_size: None,
            esa_dedup_space_saving: None,
            esa_compression_space_saving: None,
            total_space_used_without_overhead: None,
        }
    }
}

impl Default for structs::VsanDataEfficiencyConfig {
    fn default() -> Self {
        Self {
            dedup_enabled: false,
            compression_enabled: None,
        }
    }
}

impl Default for structs::VsanDataEfficiencyConfigEx {
    fn default() -> Self {
        Self {
            vsan_data_efficiency_config_: structs::VsanDataEfficiencyConfig::default(),
            dedup_store_uuid: None,
            dedup_paused: None,
        }
    }
}

impl Default for structs::VsanDataEncryptionConfig {
    fn default() -> Self {
        Self {
            encryption_enabled: false,
            kms_provider_id: None,
            kek_id: None,
            host_key_id: None,
            dek_generation_id: None,
            changing: None,
            erase_disks_before_use: None,
            wrapped_dek: None,
            dek_id: None,
            old_wrapped_dek: None,
            old_dek_id: None,
            kek_verifier: None,
            dek_verifier: None,
            old_dek_verifier: None,
            iv: None,
            syncing: None,
        }
    }
}

impl Default for structs::VsanDataInTransitEncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: None,
            rekey_interval: None,
        }
    }
}

impl Default for structs::VsanDatastoreConfig {
    fn default() -> Self {
        Self {
            datastores: None,
        }
    }
}

impl Default for structs::VsanAdvancedDatastoreConfig {
    fn default() -> Self {
        Self {
            vsan_datastore_config_: structs::VsanDatastoreConfig::default(),
            remote_datastores: None,
        }
    }
}

impl Default for structs::VsanDatastoreSpec {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            name: String::new(),
        }
    }
}

impl Default for structs::VsanClientDatastoreConfig {
    fn default() -> Self {
        Self {
            vsan_datastore_spec_: structs::VsanDatastoreSpec::default(),
            clusters: Vec::new(),
        }
    }
}

impl Default for structs::VsanXvcClientConfig {
    fn default() -> Self {
        Self {
            vsan_datastore_spec_: structs::VsanDatastoreSpec::default(),
            xvc_clusters: None,
        }
    }
}

impl Default for structs::DefaultDatastorePolicySelectionInfo {
    fn default() -> Self {
        Self {
            enabled: false,
            default_policy_id: None,
            last_policy_selection_time: None,
        }
    }
}

impl Default for structs::VsanDirectoryServerConfig {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::ActiveVsanDirectoryServerConfig {
    fn default() -> Self {
        Self {
            active_directory_domain_name: None,
            username: None,
            password: None,
            organizational_unit: None,
            preferred_ad_servers: None,
        }
    }
}

impl Default for structs::DiskClaimConfiguration {
    fn default() -> Self {
        Self {
            disk_type: String::new(),
            disk_name_prefix: None,
            number_of_disks: None,
            disk_model: None,
            vendor: None,
            disk_capacity: None,
        }
    }
}

impl Default for structs::VsanEntityCompatibilityResult {
    fn default() -> Self {
        Self {
            entity: structs::ManagedObjectReference::default(),
            compatible: false,
            incompatible_reasons: None,
            extended_attributes: None,
        }
    }
}

impl Default for structs::EntityResourceCheckDetails {
    fn default() -> Self {
        Self {
            name: None,
            uuid: None,
            is_new: None,
            capacity: None,
            post_operation_capacity: None,
            used_capacity: None,
            post_operation_used_capacity: None,
            additional_required_capacity: None,
            max_components: None,
            components: None,
        }
    }
}

impl Default for structs::VsanDiskGroupResourceCheckResult {
    fn default() -> Self {
        Self {
            entity_resource_check_details_: structs::EntityResourceCheckDetails::default(),
            cache_tier_disk: None,
            capacity_tier_disks: None,
        }
    }
}

impl Default for structs::VsanDiskResourceCheckResult {
    fn default() -> Self {
        Self {
            entity_resource_check_details_: structs::EntityResourceCheckDetails::default(),
        }
    }
}

impl Default for structs::VsanStoragePoolDiskResourceCheckResult {
    fn default() -> Self {
        Self {
            vsan_disk_resource_check_result_: structs::VsanDiskResourceCheckResult::default(),
            disk_type: None,
        }
    }
}

impl Default for structs::VsanFaultDomainResourceCheckResult {
    fn default() -> Self {
        Self {
            entity_resource_check_details_: structs::EntityResourceCheckDetails::default(),
            hosts: None,
        }
    }
}

impl Default for structs::VsanHostResourceCheckResult {
    fn default() -> Self {
        Self {
            entity_resource_check_details_: structs::EntityResourceCheckDetails::default(),
            host: None,
            disk_groups: None,
            storage_pools: None,
        }
    }
}

impl Default for structs::VsanResourceCheckResult {
    fn default() -> Self {
        Self {
            entity_resource_check_details_: structs::EntityResourceCheckDetails::default(),
            timestamp: String::new(),
            status: String::new(),
            messages: None,
            fault_domains: None,
            data_to_move: None,
            non_compliant_objects: None,
            inaccessible_objects: None,
            capacity_threshold: None,
            health: None,
            data_to_resync: None,
            dedup_store_health: None,
        }
    }
}

impl Default for structs::VsanResourceCheckComponentResult {
    fn default() -> Self {
        Self {
            vsan_resource_check_result_: structs::VsanResourceCheckResult::default(),
            r#type: String::new(),
        }
    }
}

impl Default for structs::VsanResourceCheckDataPersistenceResult {
    fn default() -> Self {
        Self {
            vsan_resource_check_component_result_: structs::VsanResourceCheckComponentResult::default(),
            data_to_rebuild: None,
            inaccessible_instances: None,
            reduced_availability_instances: None,
            rebuild_instances: None,
        }
    }
}

impl Default for structs::VsanResourceCheckVsanResult {
    fn default() -> Self {
        Self {
            vsan_resource_check_component_result_: structs::VsanResourceCheckComponentResult::default(),
        }
    }
}

impl Default for structs::VsanStoragePoolResourceCheckResult {
    fn default() -> Self {
        Self {
            entity_resource_check_details_: structs::EntityResourceCheckDetails::default(),
            disks: None,
        }
    }
}

impl Default for structs::VsanFileServiceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            file_server_memory_mb: None,
            file_server_cpu_mhz: None,
            fsvm_memory_mb: None,
            fsvm_cpu: None,
            network: None,
            domains: None,
            file_analytics_enabled: None,
        }
    }
}

impl Default for structs::VsanFileServiceDomain {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            config: None,
        }
    }
}

impl Default for structs::VsanFileServiceDomainConfig {
    fn default() -> Self {
        Self {
            name: None,
            dns_server_addresses: None,
            dns_suffixes: None,
            file_server_ip_config: None,
            directory_server_config: None,
            version: None,
        }
    }
}

impl Default for structs::VsanFileServiceDomainQuerySpec {
    fn default() -> Self {
        Self {
            uuids: None,
            names: None,
        }
    }
}

impl Default for structs::VsanFileShare {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            config: None,
            runtime: None,
        }
    }
}

impl Default for structs::VsanFileShareConfig {
    fn default() -> Self {
        Self {
            name: None,
            domain_name: None,
            quota: None,
            soft_quota: None,
            labels: None,
            storage_policy: None,
            permission: None,
            protocols: None,
            smb_options: None,
            nfs_sec_type: None,
            affinity_location: None,
        }
    }
}

impl Default for structs::VsanFileShareNetPermission {
    fn default() -> Self {
        Self {
            ips: String::new(),
            permissions: None,
            allow_root: None,
        }
    }
}

impl Default for structs::VsanFileShareQueryProperties {
    fn default() -> Self {
        Self {
            include_basic: None,
            include_used_capacity: None,
            include_vsan_object_uuids: None,
            include_all_labels: None,
            label_keys: None,
        }
    }
}

impl Default for structs::FileShareQueryResult {
    fn default() -> Self {
        Self {
            file_shares: None,
            next_offset: None,
            total_share_count: None,
            max_share_count: None,
        }
    }
}

impl Default for structs::VsanFileShareQuerySpec {
    fn default() -> Self {
        Self {
            domain_name: None,
            uuids: None,
            names: None,
            offset: None,
            limit: None,
            managed_by: None,
            protocols: None,
            page_number: None,
            properties: None,
        }
    }
}

impl Default for structs::VsanFileShareRuntimeInfo {
    fn default() -> Self {
        Self {
            used_capacity: None,
            hostname: None,
            address: None,
            vsan_object_uuids: None,
            access_points: None,
            managed_by: None,
            file_server_fqdn: None,
        }
    }
}

impl Default for structs::VsanFileShareSmbOptions {
    fn default() -> Self {
        Self {
            encryption: None,
            access_based_enumeration: None,
        }
    }
}

impl Default for structs::VsanFileShareSnapshot {
    fn default() -> Self {
        Self {
            config: None,
            creation_time: None,
            used_capacity: None,
        }
    }
}

impl Default for structs::VsanFileShareSnapshotConfig {
    fn default() -> Self {
        Self {
            share_uuid: None,
            name: None,
        }
    }
}

impl Default for structs::VsanFileShareSnapshotQueryResult {
    fn default() -> Self {
        Self {
            snapshots: None,
            total_count: None,
        }
    }
}

impl Default for structs::VsanFileShareSnapshotQuerySpec {
    fn default() -> Self {
        Self {
            share_uuid: String::new(),
            snapshot_names: None,
            start_time: None,
            end_time: None,
            page_size: None,
            page_number: None,
        }
    }
}

impl Default for structs::VsanHciMeshDatastoreSource {
    fn default() -> Self {
        Self {
            vc_info: Default::default(),
        }
    }
}

impl Default for structs::VsanIoDiagnosticsFailedCheck {
    fn default() -> Self {
        Self {
            unsupported_type: String::new(),
            reason: structs::LocalizableMessage::default(),
        }
    }
}

impl Default for structs::VsanIoDiagnosticsInstance {
    fn default() -> Self {
        Self {
            name: String::new(),
            state: String::new(),
            events: None,
            targets: None,
            start_time: String::new(),
            end_time: String::new(),
            recurrence_name: None,
        }
    }
}

impl Default for structs::VsanIoDiagnosticsInstanceEvent {
    fn default() -> Self {
        Self {
            event_type: String::new(),
            event_time: String::new(),
            event_targets: None,
        }
    }
}

impl Default for structs::VsanIoDiagnosticsInstanceQuerySpec {
    fn default() -> Self {
        Self {
            targets: None,
            start_time: String::new(),
            end_time: None,
        }
    }
}

impl Default for structs::VsanIoDiagnosticsObjectLayout {
    fn default() -> Self {
        Self {
            layout: String::new(),
        }
    }
}

impl Default for structs::VsanIoDiagnosticsPrecheckResult {
    fn default() -> Self {
        Self {
            supported: false,
            failed_checks: None,
        }
    }
}

impl Default for structs::VsanIoDiagnosticsStats {
    fn default() -> Self {
        Self {
            objects_io_stats: None,
            start_time: String::new(),
            end_time: String::new(),
        }
    }
}

impl Default for structs::VsanIoDiagnosticsTarget {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            entity_id: String::new(),
            obj_uuids: None,
        }
    }
}

impl Default for structs::VsanIoDiagnosticsTargetStats {
    fn default() -> Self {
        Self {
            target: structs::VsanIoDiagnosticsTarget::default(),
            objects_io_diagnostics_stats: None,
        }
    }
}

impl Default for structs::VsanIoLatency {
    fn default() -> Self {
        Self {
            latency_type: String::new(),
            source_entity_uuid: String::new(),
            dest_entity_uuid: String::new(),
            read_latency_stats: structs::VsanIoLatencyMetrics::default(),
            write_latency_stats: structs::VsanIoLatencyMetrics::default(),
            detailed_info: None,
        }
    }
}

impl Default for structs::VsanIoLatencyMetrics {
    fn default() -> Self {
        Self {
            total_count: 0,
            average_latency: 0.0,
            stddev_latency: None,
        }
    }
}

impl Default for structs::LifecycleConfigDetails {
    fn default() -> Self {
        Self {
            cluster_type: String::new(),
            fault_domains_details: None,
            witness_hosts_details: None,
        }
    }
}

impl Default for structs::LifecycleFaultDomainDetails {
    fn default() -> Self {
        Self {
            is_preferred_fault_domain: None,
            name: None,
            hosts: None,
        }
    }
}

impl Default for structs::LifecyclePreCheckResult {
    fn default() -> Self {
        Self {
            r#type: None,
            description: None,
            status: String::new(),
            reason: None,
        }
    }
}

impl Default for structs::LifecycleWitnessDetails {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            is_virtual_appliance: false,
            shared_clusters: None,
        }
    }
}

impl Default for structs::VsanMetricProfile {
    fn default() -> Self {
        Self {
            auth_token: String::new(),
        }
    }
}

impl Default for structs::VsanMetricsConfig {
    fn default() -> Self {
        Self {
            profiles: None,
        }
    }
}

impl Default for structs::VsanMountPrecheckItem {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            description: structs::LocalizableMessage::default(),
            status: String::new(),
            reason: None,
            ignore_message: None,
        }
    }
}

impl Default for structs::VsanDatastoreSourcePrecheckItem {
    fn default() -> Self {
        Self {
            vsan_mount_precheck_item_: structs::VsanMountPrecheckItem::default(),
        }
    }
}

impl Default for structs::VsanMountPrecheckNetworkConnectivityResult {
    fn default() -> Self {
        Self {
            vsan_mount_precheck_item_: structs::VsanMountPrecheckItem::default(),
            details: None,
        }
    }
}

impl Default for structs::VsanMountPrecheckNetworkLatencyResult {
    fn default() -> Self {
        Self {
            vsan_mount_precheck_item_: structs::VsanMountPrecheckItem::default(),
            details: Vec::new(),
        }
    }
}

impl Default for structs::VsanMountPrecheckNetworkConnectivity {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            small_ping_test_success_pct: 0,
            large_ping_test_success_pct: 0,
            status: String::new(),
        }
    }
}

impl Default for structs::VsanMountPrecheckNetworkConnectivityDetail {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            network_connectivity: None,
        }
    }
}

impl Default for structs::VsanMountPrecheckNetworkLatency {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            network_latency: 0,
            status: String::new(),
        }
    }
}

impl Default for structs::VsanMountPrecheckNetworkLatencyDetail {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            network_latencies: None,
        }
    }
}

impl Default for structs::VsanMountPrecheckResult {
    fn default() -> Self {
        Self {
            result: None,
        }
    }
}

impl Default for structs::VsanDatastoreSourcePrecheckResult {
    fn default() -> Self {
        Self {
            vsan_mount_precheck_result_: structs::VsanMountPrecheckResult::default(),
        }
    }
}

impl Default for structs::VsanObjectHealthTelemetrySummary {
    fn default() -> Self {
        Self {
            healthy_object_count: 0,
            inaccessible_object_count: 0,
            need_retry_object_count: 0,
            pdl_object_count: 0,
            cluster_host_count: 0,
        }
    }
}

impl Default for structs::VsanObjectIoStats {
    fn default() -> Self {
        Self {
            backing_object_id: String::new(),
            io_latency_stats: None,
            object_layout: structs::VsanIoDiagnosticsObjectLayout::default(),
        }
    }
}

impl Default for structs::VsanProactiveRebalanceInfo {
    fn default() -> Self {
        Self {
            enabled: None,
            threshold: None,
        }
    }
}

impl Default for structs::VsanRdmaConfig {
    fn default() -> Self {
        Self {
            rdma_enabled: false,
        }
    }
}

impl Default for structs::VsanRemoteVcInfo {
    fn default() -> Self {
        Self {
            link_type: None,
            vc_host: String::new(),
        }
    }
}

impl Default for structs::VsanRemoteVcInfoStandalone {
    fn default() -> Self {
        Self {
            vsan_remote_vc_info_: structs::VsanRemoteVcInfo::default(),
            user: None,
            password: None,
            cert: None,
        }
    }
}

impl Default for structs::RemoteVsanSite {
    fn default() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Default for structs::RemoteVsanSiteAffinity {
    fn default() -> Self {
        Self {
            client_site: None,
            server_site: structs::RemoteVsanSite::default(),
        }
    }
}

impl Default for structs::RepairTimerInfo {
    fn default() -> Self {
        Self {
            max_time_to_repair: 0,
            min_time_to_repair: 0,
            object_count: 0,
            object_count_with_repair_timer: None,
        }
    }
}

impl Default for structs::VsanResourceCheckSpec {
    fn default() -> Self {
        Self {
            operation: String::new(),
            entities: None,
            maintenance_spec: None,
            parent: None,
        }
    }
}

impl Default for structs::VsanResourceCheckStatus {
    fn default() -> Self {
        Self {
            status: String::new(),
            result: None,
            task: None,
            parent_task: None,
            component_results: None,
        }
    }
}

impl Default for structs::VsanResourceCheckTaskDetails {
    fn default() -> Self {
        Self {
            task: structs::ManagedObjectReference::default(),
            host: None,
            host_uuid: None,
            maintenance_spec: None,
        }
    }
}

impl Default for structs::VsanDiskDataEvacuationResourceCheckTaskDetails {
    fn default() -> Self {
        Self {
            vsan_resource_check_task_details_: structs::VsanResourceCheckTaskDetails::default(),
            disk_uuid: None,
            is_capacity_tier: None,
        }
    }
}

impl Default for structs::ResyncIopsInfo {
    fn default() -> Self {
        Self {
            resync_iops: 0,
        }
    }
}

impl Default for structs::VsanRuntimeStatsHostMap {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            stats: None,
        }
    }
}

impl Default for structs::SsdEnduranceThresholdSpec {
    fn default() -> Self {
        Self {
            clustername: String::new(),
            clusternameop: None,
            hostname: None,
            hostnameop: None,
            diskname: None,
            disknameop: None,
            diskvendorname: None,
            diskvendorop: None,
            ssd_endurance_ptg: 0.0,
            severity: String::new(),
        }
    }
}

impl Default for structs::VsanServerHostUnicastInfo {
    fn default() -> Self {
        Self {
            host_uuid: String::new(),
            node_type: None,
            unicast_spec: None,
        }
    }
}

impl Default for structs::VsanSharedWitnessCompatibilityResult {
    fn default() -> Self {
        Self {
            witness_host_compatibility: structs::VsanEntityCompatibilityResult::default(),
            robo_cluster_compatibility: None,
        }
    }
}

impl Default for structs::VsanSnapServiceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
        }
    }
}

impl Default for structs::VcRemoteVsanServerClusterConfig {
    fn default() -> Self {
        Self {
            server_clusters: None,
        }
    }
}

impl Default for structs::VcRemoteVsanServerClusterInfo {
    fn default() -> Self {
        Self {
            cluster_uuid: String::new(),
            network_topology: None,
            site_affinity: None,
        }
    }
}

impl Default for structs::VsanIscsiVipConfigSpec {
    fn default() -> Self {
        Self {
            enabled: None,
            v_4_network_config: None,
            v_6_network_config: None,
            vswitch_config: None,
            distributed_switch_config: None,
        }
    }
}

impl Default for structs::VsanIscsiVipConfig {
    fn default() -> Self {
        Self {
            vsan_iscsi_vip_config_spec_: structs::VsanIscsiVipConfigSpec::default(),
            vmknic_name: None,
            owner: None,
            owner_host_uuid: None,
        }
    }
}

impl Default for structs::VsanIscsiVipDVswitchConfig {
    fn default() -> Self {
        Self {
            port_group: None,
            dvs_uuid: None,
        }
    }
}

impl Default for structs::VsanVipNetworkConfig {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            subnet: String::new(),
            gateway: None,
        }
    }
}

impl Default for structs::VsanIscsiVipVswitchConfig {
    fn default() -> Self {
        Self {
            vswitch_name: String::new(),
            vlan_id: None,
        }
    }
}

impl Default for structs::VsanBurnInTest {
    fn default() -> Self {
        Self {
            testname: String::new(),
            workload: None,
            duration: 0,
            result: String::new(),
        }
    }
}

impl Default for structs::VsanBurnInTestCheckResult {
    fn default() -> Self {
        Self {
            passed_tests: None,
            not_performed_tests: None,
            failed_tests: None,
        }
    }
}

impl Default for structs::VsanCloudHealthStatus {
    fn default() -> Self {
        Self {
            collector_running: None,
            last_sent_timestamp: None,
            internet_connectivity: None,
        }
    }
}

impl Default for structs::VsanClusterBurnInTestResultList {
    fn default() -> Self {
        Self {
            items: None,
            hosts: None,
        }
    }
}

impl Default for structs::VsanCompliantDriver {
    fn default() -> Self {
        Self {
            driver_name: String::new(),
            driver_version: String::new(),
            supported_features: None,
        }
    }
}

impl Default for structs::VsanCompliantFirmware {
    fn default() -> Self {
        Self {
            firmware_version: String::new(),
            compliant_drivers: Vec::new(),
        }
    }
}

impl Default for structs::VsanConfigBaseIssue {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VsanConfigNotAllDisksClaimedIssue {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            disks: Vec::new(),
        }
    }
}

impl Default for structs::VsanConfigCheckResult {
    fn default() -> Self {
        Self {
            vsan_enabled: false,
            issues: None,
        }
    }
}

impl Default for structs::VsanDatastoreDefaultPolicySelectionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
        }
    }
}

impl Default for structs::VsanDeconvergedNetConfig {
    fn default() -> Self {
        Self {
            enabled: false,
        }
    }
}

impl Default for structs::VsanDiskModelInfo {
    fn default() -> Self {
        Self {
            product_id: String::new(),
            vendor: String::new(),
            part_number: None,
        }
    }
}

impl Default for structs::VsanDownloadItem {
    fn default() -> Self {
        Self {
            url: String::new(),
            sha_1_sum: String::new(),
            format_type: None,
            item_id: None,
        }
    }
}

impl Default for structs::VsanEsaConfig {
    fn default() -> Self {
        Self {
            storage_pool_specs: None,
            hcl_disk_claim_enabled: None,
            datastore_default_policy_selection_config: None,
            disk_configuration: None,
        }
    }
}

impl Default for structs::VsanEsaConfigInfo {
    fn default() -> Self {
        Self {
            hcl_disk_claim_enabled: None,
            datastore_default_policy_selection_config: None,
            disk_configuration: None,
        }
    }
}

impl Default for structs::VsanEsaDiskConfiguration {
    fn default() -> Self {
        Self {
            disk_claim_configuration: None,
        }
    }
}

impl Default for structs::VsanExtendedConfig {
    fn default() -> Self {
        Self {
            object_repair_timer: None,
            disable_site_read_locality: None,
            enable_customized_swap_object: None,
            large_scale_cluster_support: None,
            proactive_rebalance_info: None,
            capacity_reservation_info: None,
        }
    }
}

impl Default for structs::VsanFileServiceOvfSpec {
    fn default() -> Self {
        Self {
            version: None,
            update_time: None,
            task: None,
        }
    }
}

impl Default for structs::VsanFileServicePreflightCheckResult {
    fn default() -> Self {
        Self {
            ovf_installed: None,
            fsvm_version: None,
            last_upgrade_date: None,
            ovf_mixed_mode_issue: None,
            host_version: None,
            mixed_mode_issue: None,
            network_partition_issue: None,
            vsan_datastore_issue: None,
            domain_config_issue: None,
            file_service_version: None,
            dvs_config_issue: None,
            domain_config_warning: None,
            ntp_config_warning: None,
            svs_config_issue: None,
        }
    }
}

impl Default for structs::VsanGenericClusterBaseIssue {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VsanGenericClusterBestPracticeHealth {
    fn default() -> Self {
        Self {
            drs_enabled: false,
            ha_enabled: false,
            issues: None,
        }
    }
}

impl Default for structs::VsanHclDeviceConstraint {
    fn default() -> Self {
        Self {
            pci_id: String::new(),
            vcg_link: None,
            similar_vcg_links: None,
            compliant_firmwares: None,
            vcg_id: None,
            model: None,
            partner: None,
            part_number: None,
            release: None,
        }
    }
}

impl Default for structs::VsanHclDiskConstraint {
    fn default() -> Self {
        Self {
            product_id: String::new(),
            vendor: String::new(),
            constraints: None,
            pcie_constraints: None,
            part_number: None,
        }
    }
}

impl Default for structs::VsanHclDriverInfo {
    fn default() -> Self {
        Self {
            driver_version: None,
            driver_link: None,
            fw_version: None,
            fw_links: None,
            tools_links: None,
            eula: None,
            driver_type: None,
            driver_name: None,
            disk_modes: None,
            supported_features: None,
        }
    }
}

impl Default for structs::VsanHclMinFwConstraint {
    fn default() -> Self {
        Self {
            vcg_id: 0,
            vcg_link: String::new(),
            model: String::new(),
            partner: String::new(),
            part_number: None,
            release: String::new(),
            firmware: String::new(),
        }
    }
}

impl Default for structs::VsanHclQuerySpec {
    fn default() -> Self {
        Self {
            include_only_vsan_controllers: None,
            cluster: None,
            hosts: None,
            vsan_storage_pool_eligible_disks_only: None,
        }
    }
}

impl Default for structs::VsanHclReleaseConstraint {
    fn default() -> Self {
        Self {
            cluster: structs::ManagedObjectReference::default(),
            release: String::new(),
            host_devices: None,
            constraints: None,
        }
    }
}

impl Default for structs::VsanHealthConfigSpec {
    fn default() -> Self {
        Self {
            health_check_threshold_spec: None,
            historical_health_config: None,
        }
    }
}

impl Default for structs::VsanHealthCustomizationSpec {
    fn default() -> Self {
        Self {
            ssd_endurance_spec: None,
        }
    }
}

impl Default for structs::VsanHealthThreshold {
    fn default() -> Self {
        Self {
            yellow_value: 0,
            red_value: 0,
            target: None,
            enabled: None,
        }
    }
}

impl Default for structs::VsanHistoricalHealthConfig {
    fn default() -> Self {
        Self {
            enabled: false,
        }
    }
}

impl Default for structs::VsanHostDeviceInfo {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            devices: None,
        }
    }
}

impl Default for structs::VsanHwToVcgInfoMappingSpec {
    fn default() -> Self {
        Self {
            entity: String::new(),
            vsan_hw_to_vcg_info_mappings: Vec::new(),
        }
    }
}

impl Default for structs::VsanIoTripAnalyzerConfig {
    fn default() -> Self {
        Self {
            recurrences: None,
        }
    }
}

impl Default for structs::VsanIoTripAnalyzerRecurrence {
    fn default() -> Self {
        Self {
            name: None,
            targets: Vec::new(),
            start_time: String::new(),
            end_time: None,
            duration: 0,
            interval: 0,
            status: String::new(),
        }
    }
}

impl Default for structs::VsanInternalExtendedConfig {
    fn default() -> Self {
        Self {
            vc_max_disk_version: None,
            stretched_client: None,
        }
    }
}

impl Default for structs::VsanNetworkConfigBaseIssue {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VsanNetworkConfigPnicSpeedInconsistencyIssue {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            vswitch_name: None,
            vds: None,
            speeds_mb: Vec::new(),
        }
    }
}

impl Default for structs::VsanNetworkConfigPortgroupWithNoRedundancyIssue {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            portgroup_name: None,
            vds: None,
            pg: None,
            num_pnics: 0,
        }
    }
}

impl Default for structs::VsanNetworkConfigVdsScopeIssue {
    fn default() -> Self {
        Self {
            vds: structs::ManagedObjectReference::default(),
            member_hosts: Vec::new(),
            non_member_hosts: Vec::new(),
        }
    }
}

impl Default for structs::VsanNetworkConfigVsanNotOnVdsIssue {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            vmknic: String::new(),
        }
    }
}

impl Default for structs::VsanNetworkConfigVswitchWithNoRedundancyIssue {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            vswitch_name: None,
            vds: None,
            num_pnics: 0,
        }
    }
}

impl Default for structs::VsanNetworkVMotionVmknicNotFountIssue {
    fn default() -> Self {
        Self {
            host_without_vmotion_vmknic: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VsanNetworkConfigBestPracticeHealth {
    fn default() -> Self {
        Self {
            vds_present: false,
            issues: None,
        }
    }
}

impl Default for structs::VsanObjSnapParams {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            creator: None,
            snapshot_type: None,
            cookie: None,
        }
    }
}

impl Default for structs::VsanObjectDetail {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            object_path: None,
            snapshots: None,
        }
    }
}

impl Default for structs::VsanObjectSnapshotId {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            snapshot_id: 0,
            cookie: None,
        }
    }
}

impl Default for structs::VimVsanVsanPMemConfig {
    fn default() -> Self {
        Self {
            enabled: false,
        }
    }
}

impl Default for structs::VsanPerfsvcHealthResult {
    fn default() -> Self {
        Self {
            stats_object_info: None,
            stats_object_consistent: None,
            stats_object_policy_consistent: None,
            datastore_compatible: None,
            enough_free_space: None,
            remediate_action: None,
            host_results: None,
            verbose_mode_status: None,
        }
    }
}

impl Default for structs::VsanPrepareVsanForVcsaSpec {
    fn default() -> Self {
        Self {
            vsan_disk_mapping_creation_spec: None,
            vsan_data_efficiency_config: None,
            task_id: None,
            vsan_data_encryption_config: None,
            vsan_add_storage_pool_disk_spec: None,
            create_native_key_provider_spec: None,
        }
    }
}

impl Default for structs::VsanSnapshotDetail {
    fn default() -> Self {
        Self {
            snapshot_id: 0,
            snapshot_type: String::new(),
            snapshot_path: None,
        }
    }
}

impl Default for structs::VsanSnapshotQueryResult {
    fn default() -> Self {
        Self {
            objects: None,
        }
    }
}

impl Default for structs::VsanSnapshotQuerySpec {
    fn default() -> Self {
        Self {
            datastore_uuid: String::new(),
            object_uuids: None,
            snapshot_type: None,
            creator: None,
            include_descriptor_path: None,
        }
    }
}

impl Default for structs::VsanSpaceEfficiencyMetadataSize {
    fn default() -> Self {
        Self {
            dedup_metadata_size: None,
            compression_metadata_size: None,
        }
    }
}

impl Default for structs::VsanSpaceEfficiencyRatio {
    fn default() -> Self {
        Self {
            overall_ratio: None,
            compression_ratio: None,
            dedup_ratio: None,
        }
    }
}

impl Default for structs::VsanUnmapConfig {
    fn default() -> Self {
        Self {
            enable: false,
        }
    }
}

impl Default for structs::VsanUpdateItem {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            r#type: String::new(),
            name: String::new(),
            version: String::new(),
            existing_version: None,
            present: false,
            vib_spec: None,
            vib_type: None,
            firmware_spec: None,
            download_info: None,
            eula: None,
            adapter: None,
            key: None,
            impact: None,
            firmware_unknown: None,
        }
    }
}

impl Default for structs::VsanVcPostDeployConfigSpec {
    fn default() -> Self {
        Self {
            dc_name: None,
            cluster_name: None,
            first_host: None,
            hosts_to_add: None,
            vsan_data_efficiency_config: None,
            vsan_license_key: None,
            host_license_key: None,
            task_id: None,
            vsan_data_encryption_config: None,
            create_native_key_provider_spec: None,
            vsan_cluster_mode: None,
            deconverged_net_config: None,
        }
    }
}

impl Default for structs::VsanVcStretchedClusterConfigSpec {
    fn default() -> Self {
        Self {
            witness_host: structs::ManagedObjectReference::default(),
            clusters: Vec::new(),
            witness_disk_mappings: None,
            witness_storage_pool_specs: None,
        }
    }
}

impl Default for structs::VsanVcsaDeploymentProgress {
    fn default() -> Self {
        Self {
            phase: String::new(),
            progress_pct: 0,
            message: String::new(),
            success: false,
            error: None,
            update_counter: 0,
            task_id: None,
            vm: None,
        }
    }
}

impl Default for structs::VsanVdsMigrationPlan {
    fn default() -> Self {
        Self {
            vds_spec: structs::DvsCreateSpec::default(),
            pgs: None,
            inaccessible_vms: None,
            infra_vms: None,
        }
    }
}

impl Default for structs::VsanVdsPgMigrationHostInfo {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            hostname: String::new(),
            vmknic_devices: None,
            vm_vnics: None,
        }
    }
}

impl Default for structs::VsanVdsPgMigrationSpec {
    fn default() -> Self {
        Self {
            vss_pg_name: String::new(),
            dv_pg_name: String::new(),
            vds_pg_setting: structs::VMwareDvsPortSetting::default(),
            vds_pg_type: String::new(),
            hosts: None,
            collision_rename: false,
        }
    }
}

impl Default for structs::VsanVdsPgMigrationVmInfo {
    fn default() -> Self {
        Self {
            vm: structs::ManagedObjectReference::default(),
            vnic_label: Vec::new(),
        }
    }
}

impl Default for structs::VsanVibInstallPreflightStatus {
    fn default() -> Self {
        Self {
            manual_vmotion_required: false,
            rolling_required: false,
        }
    }
}

impl Default for structs::VsanVibScanResult {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            vib_name: String::new(),
            vib_version: String::new(),
            existing_version: None,
            maintenance_mode_required: false,
            reboot_required: false,
            meets_system_req: false,
            pkg_deps_met_by_host: false,
        }
    }
}

impl Default for structs::VsanVibSpec {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            meta_url: None,
            meta_sha_1_sum: None,
            vib_url: String::new(),
            vib_sha_1_sum: String::new(),
        }
    }
}

impl Default for structs::VsanVmVdsMigrationSpec {
    fn default() -> Self {
        Self {
            vm_instance_uuid: String::new(),
            vnics: Vec::new(),
        }
    }
}

impl Default for structs::VsanVnicVdsMigrationSpec {
    fn default() -> Self {
        Self {
            key: 0,
            vds_backing: Default::default(),
        }
    }
}

impl Default for structs::VsanVumConfig {
    fn default() -> Self {
        Self {
            baseline_preference_type: String::new(),
        }
    }
}

impl Default for structs::VsanWitnessHostConfig {
    fn default() -> Self {
        Self {
            sub_cluster_uuid: String::new(),
            preferred_fault_domain_name: String::new(),
            metadata_mode: None,
        }
    }
}

impl Default for structs::VsanXvcClientInfo {
    fn default() -> Self {
        Self {
            cluster: structs::ManagedObjectReference::default(),
            cluster_name: String::new(),
            vsan_format_version: String::new(),
            owner_vc: String::new(),
            vc_uuid: None,
            cluster_uuid: None,
        }
    }
}

impl Default for structs::VsanXvcDatastoreConfig {
    fn default() -> Self {
        Self {
            xvc_datastores: None,
        }
    }
}

impl Default for structs::VsanXvcDatastoreInfo {
    fn default() -> Self {
        Self {
            datastore: structs::ManagedObjectReference::default(),
            owner_vc: String::new(),
        }
    }
}

impl Default for structs::VsanXvcClientInfoSpec {
    fn default() -> Self {
        Self {
            client_vc: String::new(),
            vc_uuid: None,
            vc_version: None,
            cluster: None,
            cluster_name: None,
            cluster_uuid: None,
            vsan_format_version: None,
            min_vsan_format_version: None,
            datastore: None,
        }
    }
}

impl Default for structs::VsanXvcQueryCriteria {
    fn default() -> Self {
        Self {
            property: String::new(),
            operator: None,
            comparable_value: None,
            comparable_list: None,
            ignore_case: None,
        }
    }
}

impl Default for structs::VsanXvcQueryFilter {
    fn default() -> Self {
        Self {
            criterias: None,
            operator: None,
        }
    }
}

impl Default for structs::VsanXvcQueryPropertyValue {
    fn default() -> Self {
        Self {
            value: None,
        }
    }
}

impl Default for structs::VsanXvcQueryResultSet {
    fn default() -> Self {
        Self {
            properties: None,
            result_items: None,
            total_count: None,
        }
    }
}

impl Default for structs::VsanXvcQuerySpec {
    fn default() -> Self {
        Self {
            object_model: None,
            properties: None,
            filter: None,
            offset: None,
            limit: None,
            return_total_count: None,
        }
    }
}

impl Default for structs::VsanXvcResultItem {
    fn default() -> Self {
        Self {
            property_values: None,
        }
    }
}

impl Default for structs::VsanClusterConfigInfo {
    fn default() -> Self {
        Self {
            enabled: None,
            default_config: None,
            vsan_esa_enabled: None,
        }
    }
}

impl Default for structs::VsanConfigInfoEx {
    fn default() -> Self {
        Self {
            vsan_cluster_config_info_: structs::VsanClusterConfigInfo::default(),
            data_efficiency_config: None,
            resync_iops_limit_config: None,
            iscsi_config: None,
            data_encryption_config: None,
            extended_config: None,
            datastore_config: None,
            perfsvc_config: None,
            unmap_config: None,
            vum_config: None,
            file_service_config: None,
            metrics_config: None,
            rdma_config: None,
            data_in_transit_encryption_config: None,
            vsan_health_config: None,
            mode: None,
            vsan_p_mem_config: None,
            vsan_esa_config_info: None,
            xvc_datastore_config: None,
            server_cluster_config: None,
            datastore_default_policy_selection_config: None,
            snap_service_config: None,
            deconverged_net_config: None,
            site_fault_domain_config: None,
        }
    }
}

impl Default for structs::VsanClusterConfigInfoHostDefaultInfo {
    fn default() -> Self {
        Self {
            uuid: None,
            auto_claim_storage: None,
            checksum_enabled: None,
        }
    }
}

impl Default for structs::VsanClusterCoreConfig {
    fn default() -> Self {
        Self {
            vsan_max_enabled: None,
        }
    }
}

impl Default for structs::VsanClusterCoreConfigSpec {
    fn default() -> Self {
        Self {
            vsan_max_enabled: None,
        }
    }
}

impl Default for structs::VsanHostAbortWipeDiskStatus {
    fn default() -> Self {
        Self {
            disk: String::new(),
            success: false,
            reason: None,
        }
    }
}

impl Default for structs::VsanHostAboutInfoEx {
    fn default() -> Self {
        Self {
            name: None,
            version: None,
            build: None,
            build_type: None,
            api_version: None,
        }
    }
}

impl Default for structs::VsanAddStoragePoolDiskSpec {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            disks: Vec::new(),
        }
    }
}

impl Default for structs::VsanHostClusterStatus {
    fn default() -> Self {
        Self {
            uuid: None,
            node_uuid: None,
            health: String::new(),
            node_state: structs::VsanHostClusterStatusState::default(),
            member_uuid: None,
        }
    }
}

impl Default for structs::VsanHostClusterStatusState {
    fn default() -> Self {
        Self {
            state: String::new(),
            completion: None,
        }
    }
}

impl Default for structs::VsanHostClusterStatusStateCompletionEstimate {
    fn default() -> Self {
        Self {
            complete_time: None,
            percent_complete: None,
        }
    }
}

impl Default for structs::VsanComplianceDetail {
    fn default() -> Self {
        Self {
            object_uuid: String::new(),
            compliance_status: String::new(),
            object_health: 0,
            violated_policies: None,
        }
    }
}

impl Default for structs::VsanComplianceResult {
    fn default() -> Self {
        Self {
            check_time: String::new(),
            policy_id: None,
            policy_gen: None,
            obj_compliance_detail: None,
        }
    }
}

impl Default for structs::VsanHostConfigInfo {
    fn default() -> Self {
        Self {
            enabled: None,
            host_system: None,
            cluster_info: None,
            storage_info: None,
            network_info: None,
            fault_domain_info: None,
            vsan_esa_enabled: None,
        }
    }
}

impl Default for structs::VsanHostConfigInfoEx {
    fn default() -> Self {
        Self {
            vsan_host_config_info_: structs::VsanHostConfigInfo::default(),
            encryption_info: None,
            data_efficiency_info: None,
            resync_iops_limit_info: None,
            extended_config: None,
            datastore_info: None,
            unmap_config: None,
            witness_host_config: None,
            internal_extended_config: None,
            metrics_config: None,
            unicast_config: None,
            rdma_config: None,
            data_in_transit_encryption_info: None,
            mode: None,
            server_cluster_configs: None,
            snap_service_config: None,
            deconverged_net_config: None,
        }
    }
}

impl Default for structs::VsanHostConfigInfoClusterInfo {
    fn default() -> Self {
        Self {
            uuid: None,
            node_uuid: None,
        }
    }
}

impl Default for structs::VsanHostFaultDomainInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Default for structs::VsanHostConfigInfoNetworkInfo {
    fn default() -> Self {
        Self {
            port: None,
        }
    }
}

impl Default for structs::VsanHostConfigInfoNetworkInfoPortConfig {
    fn default() -> Self {
        Self {
            ip_config: None,
            device: String::new(),
        }
    }
}

impl Default for structs::VsanHostPortConfigEx {
    fn default() -> Self {
        Self {
            vsan_host_config_info_network_info_port_config_: structs::VsanHostConfigInfoNetworkInfoPortConfig::default(),
            traffic_types: None,
        }
    }
}

impl Default for structs::VsanHostConfigInfoStorageInfo {
    fn default() -> Self {
        Self {
            auto_claim_storage: None,
            disk_mapping: None,
            disk_map_info: None,
            checksum_enabled: None,
        }
    }
}

impl Default for structs::VsanHostCreateNativeKeyProviderSpec {
    fn default() -> Self {
        Self {
            provider: String::new(),
            key_id: None,
            key_derivation_key: None,
            tpm_required: None,
        }
    }
}

impl Default for structs::VsanInTransitEncryptionInfo {
    fn default() -> Self {
        Self {
            enabled: None,
            rekey_interval: None,
            transition_state: None,
        }
    }
}

impl Default for structs::VsanHostDecommissionMode {
    fn default() -> Self {
        Self {
            object_action: String::new(),
        }
    }
}

impl Default for structs::VsanDeleteStoragePoolDiskSpec {
    fn default() -> Self {
        Self {
            disk_uuids: Vec::new(),
            maintenance_spec: structs::HostMaintenanceSpec::default(),
        }
    }
}

impl Default for structs::VsanHostDiskMapInfo {
    fn default() -> Self {
        Self {
            mapping: structs::VsanHostDiskMapping::default(),
            mounted: false,
        }
    }
}

impl Default for structs::VimVsanHostDiskMapInfoEx {
    fn default() -> Self {
        Self {
            mapping: structs::VsanHostDiskMapping::default(),
            is_mounted: false,
            unlocked_encrypted: None,
            is_all_flash: false,
            is_data_efficiency: None,
            encryption_info: None,
            data_efficiency_config: None,
            diskgroup_capability: None,
        }
    }
}

impl Default for structs::VsanHostDiskMapResult {
    fn default() -> Self {
        Self {
            mapping: structs::VsanHostDiskMapping::default(),
            disk_result: None,
            error: None,
        }
    }
}

impl Default for structs::VsanHostDiskMapping {
    fn default() -> Self {
        Self {
            ssd: structs::HostScsiDisk::default(),
            non_ssd: Vec::new(),
        }
    }
}

impl Default for structs::VimVsanHostDiskMappingCreationSpec {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            cache_disks: None,
            capacity_disks: None,
            creation_type: String::new(),
        }
    }
}

impl Default for structs::VsanHostDiskResult {
    fn default() -> Self {
        Self {
            disk: structs::HostScsiDisk::default(),
            state: String::new(),
            vsan_uuid: None,
            error: None,
            degraded: None,
        }
    }
}

impl Default for structs::VimVsanHostDiskResultEx {
    fn default() -> Self {
        Self {
            vsan_host_disk_result_: structs::VsanHostDiskResult::default(),
            vsan_direct_tagged: false,
            storage_pool_disk_state: None,
            storage_pool_disk_error: None,
            is_capacity_flash: None,
        }
    }
}

impl Default for structs::VsanHostDrsStats {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            stats: Vec::new(),
            read_locality_presented: None,
        }
    }
}

impl Default for structs::VsanHostEncryptionInfo {
    fn default() -> Self {
        Self {
            enabled: None,
            kek_id: None,
            host_key_id: None,
            kmip_servers: None,
            kms_server_certs: None,
            client_key: None,
            client_cert: None,
            dek_generation_id: None,
            changing: None,
            erase_disks_before_use: None,
            wrapped_dek: None,
            dek_id: None,
            old_wrapped_dek: None,
            old_dek_id: None,
            kek_verifier: None,
            dek_verifier: None,
            old_dek_verifier: None,
            iv: None,
            syncing: None,
        }
    }
}

impl Default for structs::VsanHostIpConfig {
    fn default() -> Self {
        Self {
            upstream_ip_address: String::new(),
            downstream_ip_address: String::new(),
        }
    }
}

impl Default for structs::VsanHostIpConfigEx {
    fn default() -> Self {
        Self {
            vsan_host_ip_config_: structs::VsanHostIpConfig::default(),
            upstream_ip_v_6_address: None,
            downstream_ip_v_6_address: None,
        }
    }
}

impl Default for structs::VsanHostMembershipInfo {
    fn default() -> Self {
        Self {
            node_uuid: String::new(),
            hostname: String::new(),
        }
    }
}

impl Default for structs::VsanPolicyStatus {
    fn default() -> Self {
        Self {
            id: String::new(),
            expected_value: String::new(),
            current_value: String::new(),
        }
    }
}

impl Default for structs::VimVsanHostQueryVsanDisksSpec {
    fn default() -> Self {
        Self {
            disk_name: None,
            vsan_disk_type: None,
        }
    }
}

impl Default for structs::RemoteVsanServerClusterConfig {
    fn default() -> Self {
        Self {
            cluster_uuid: String::new(),
            site_affinity: None,
        }
    }
}

impl Default for structs::VsanHostRuntimeStats {
    fn default() -> Self {
        Self {
            resync_iops_info: None,
            config_generation: None,
            supported_cluster_size: None,
            repair_timer_info: None,
            component_limit_per_cluster: None,
            max_witness_clusters: None,
        }
    }
}

impl Default for structs::VsanHostServerClusterUnicastConfig {
    fn default() -> Self {
        Self {
            remote_unicast_config: None,
        }
    }
}

impl Default for structs::VsanHostServerClusterUnicastInfo {
    fn default() -> Self {
        Self {
            cluster_uuid: String::new(),
            unicast_info: None,
        }
    }
}

impl Default for structs::SiteAffinityInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            site_id: None,
        }
    }
}

impl Default for structs::VsanStoragePoolDisk {
    fn default() -> Self {
        Self {
            disk_name: String::new(),
            disk_type: String::new(),
        }
    }
}

impl Default for structs::VimVsanHostStoragePoolDiskInfo {
    fn default() -> Self {
        Self {
            disk: structs::HostScsiDisk::default(),
            vsan_uuid: None,
            error: None,
            is_mounted: None,
            is_encrypted: None,
            dek_id: None,
            disk_type: None,
        }
    }
}

impl Default for structs::VimVsanHostStoragePoolInfo {
    fn default() -> Self {
        Self {
            storage_pool_disks: None,
        }
    }
}

impl Default for structs::VimVsanHostTrimDiskEntry {
    fn default() -> Self {
        Self {
            disk_name: String::new(),
            disk_type: None,
        }
    }
}

impl Default for structs::VimVsanHostTrimDiskSpec {
    fn default() -> Self {
        Self {
            disks: None,
        }
    }
}

impl Default for structs::VimVsanHostUpdateStoragePoolDiskSpec {
    fn default() -> Self {
        Self {
            disk_uuids: None,
            disk_format_version: None,
        }
    }
}

impl Default for structs::VsanHostAssociatedObjects {
    fn default() -> Self {
        Self {
            spbm_profile_id: String::new(),
            spbm_profile_generation_num: 0,
            vsan_objects: None,
        }
    }
}

impl Default for structs::VsanHostAssociatedObjectsResult {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            offset: 0,
            limit: 0,
        }
    }
}

impl Default for structs::VsanComplianceQuerySpec {
    fn default() -> Self {
        Self {
            uuids: None,
            spbm_profile_id: None,
            spbm_profile_generation_id: None,
        }
    }
}

impl Default for structs::VsanHostComponentSyncState {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            disk_uuid: String::new(),
            host_uuid: String::new(),
            bytes_to_sync: 0,
            recovery_eta: None,
            reasons: None,
        }
    }
}

impl Default for structs::VimVsanHostVsanDirectStorage {
    fn default() -> Self {
        Self {
            scsi_disks: None,
            tier: None,
        }
    }
}

impl Default for structs::VsanHostVsanDiskInfo {
    fn default() -> Self {
        Self {
            vsan_uuid: String::new(),
            format_version: 0,
        }
    }
}

impl Default for structs::VimVsanHostVsanDiskManagementSystemCapability {
    fn default() -> Self {
        Self {
            version: String::new(),
        }
    }
}

impl Default for structs::VimVsanHostVsanHostCapability {
    fn default() -> Self {
        Self {
            host: structs::ManagedObjectReference::default(),
            is_supported: false,
            is_licensed: false,
        }
    }
}

impl Default for structs::VimVsanHostVsanManagedDisksInfo {
    fn default() -> Self {
        Self {
            v_san_direct_disks: None,
            v_san_disk_map_info: None,
            v_sanp_mem_info: None,
            storage_pools: None,
        }
    }
}

impl Default for structs::VimVsanHostVsanManagedPMemInfo {
    fn default() -> Self {
        Self {
            local_p_mem_datastores: None,
        }
    }
}

impl Default for structs::VsanObjectProfileInfo {
    fn default() -> Self {
        Self {
            vsan_object_uuid: String::new(),
            spbm_profile_id: String::new(),
            spbm_profile_generation_num: 0,
        }
    }
}

impl Default for structs::VsanHostVsanObjectSyncState {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            components: Vec::new(),
        }
    }
}

impl Default for structs::VsanHostRuntimeInfo {
    fn default() -> Self {
        Self {
            membership_list: None,
            disk_issues: None,
            access_gen_no: None,
        }
    }
}

impl Default for structs::VsanHostRuntimeInfoDiskIssue {
    fn default() -> Self {
        Self {
            disk_id: String::new(),
            issue: String::new(),
        }
    }
}

impl Default for structs::VimVsanHostVsanScsiDisk {
    fn default() -> Self {
        Self {
            capacity: structs::HostDiskDimensionsLba::default(),
            used_capacity: None,
            device_path: String::new(),
            ssd: None,
            local_disk: None,
            scsi_disk_type: None,
            uuid: String::new(),
            operational_state: None,
            canonical_name: None,
            display_name: None,
            lun_type: String::new(),
            vendor: None,
            model: None,
            mount_info: None,
        }
    }
}

impl Default for structs::VsanHostVsanObjectSyncQueryResult {
    fn default() -> Self {
        Self {
            total_objects_to_sync: None,
            total_bytes_to_sync: None,
            total_recovery_eta: None,
            objects: None,
            syncing_object_recovery_details: None,
        }
    }
}

impl Default for structs::VsanSyncingObjectRecoveryDetails {
    fn default() -> Self {
        Self {
            actively_syncing_object_recovery_eta: None,
            queued_for_sync_object_recovery_eta: None,
            suspended_object_recovery_eta: None,
            active_objects_to_sync: None,
            queued_objects_to_sync: None,
            suspended_objects_to_sync: None,
            bytes_to_sync_for_active_objects: None,
            bytes_to_sync_for_queued_objects: None,
            bytes_to_sync_for_suspended_objects: None,
        }
    }
}

impl Default for structs::VsanWhatIfEvacDetail {
    fn default() -> Self {
        Self {
            success: None,
            bytes_to_sync: None,
            inaccessible_objects: None,
            incompliant_objects: None,
            extra_space_needed: None,
            failed_due_to_inaccessible_objects: None,
        }
    }
}

impl Default for structs::VsanWhatIfEvacResult {
    fn default() -> Self {
        Self {
            no_action: structs::VsanWhatIfEvacDetail::default(),
            ensure_access: structs::VsanWhatIfEvacDetail::default(),
            evac_all_data: structs::VsanWhatIfEvacDetail::default(),
        }
    }
}

impl Default for structs::VsanHostWipeDiskStatus {
    fn default() -> Self {
        Self {
            disk: String::new(),
            eligible: String::new(),
            ineligible_reason: None,
            wipe_state: None,
            percentage_completed: None,
            estimated_time: None,
            wipe_start_time: None,
            wipe_complete_time: None,
        }
    }
}

impl Default for structs::BaseConfigInfo {
    fn default() -> Self {
        Self {
            id: structs::Id::default(),
            name: String::new(),
            create_time: String::new(),
            keep_after_delete_vm: None,
            relocation_disabled: None,
            native_snapshot_supported: None,
            changed_block_tracking_enabled: None,
            backing: Default::default(),
            metadata: None,
            vclock: None,
            iofilter: None,
        }
    }
}

impl Default for structs::VStorageObjectConfigInfo {
    fn default() -> Self {
        Self {
            base_config_info_: structs::BaseConfigInfo::default(),
            descriptor_version: None,
            capacity_in_mb: 0,
            consumption_type: None,
            consumer_id: None,
            virtual_disk_format: None,
        }
    }
}

impl Default for structs::BaseConfigInfoBackingInfo {
    fn default() -> Self {
        Self {
            datastore: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::BaseConfigInfoFileBackingInfo {
    fn default() -> Self {
        Self {
            base_config_info_backing_info_: structs::BaseConfigInfoBackingInfo::default(),
            file_path: String::new(),
            backing_object_id: None,
            parent: None,
            delta_size_in_mb: None,
            key_id: None,
        }
    }
}

impl Default for structs::BaseConfigInfoDiskFileBackingInfo {
    fn default() -> Self {
        Self {
            base_config_info_file_backing_info_: structs::BaseConfigInfoFileBackingInfo::default(),
            provisioning_type: String::new(),
        }
    }
}

impl Default for structs::BaseConfigInfoRawDiskMappingBackingInfo {
    fn default() -> Self {
        Self {
            base_config_info_file_backing_info_: structs::BaseConfigInfoFileBackingInfo::default(),
            lun_uuid: String::new(),
            compatibility_mode: String::new(),
        }
    }
}

impl Default for structs::VslmCreateSpec {
    fn default() -> Self {
        Self {
            name: String::new(),
            keep_after_delete_vm: None,
            backing_spec: Default::default(),
            capacity_in_mb: 0,
            profile: None,
            crypto: None,
            metadata: None,
        }
    }
}

impl Default for structs::VslmCreateSpecBackingSpec {
    fn default() -> Self {
        Self {
            datastore: structs::ManagedObjectReference::default(),
            path: None,
        }
    }
}

impl Default for structs::VslmCreateSpecDiskFileBackingSpec {
    fn default() -> Self {
        Self {
            vslm_create_spec_backing_spec_: structs::VslmCreateSpecBackingSpec::default(),
            provisioning_type: None,
        }
    }
}

impl Default for structs::VslmCreateSpecRawDiskMappingBackingSpec {
    fn default() -> Self {
        Self {
            vslm_create_spec_backing_spec_: structs::VslmCreateSpecBackingSpec::default(),
            lun_uuid: String::new(),
            compatibility_mode: String::new(),
        }
    }
}

impl Default for structs::DiskCryptoSpec {
    fn default() -> Self {
        Self {
            parent: None,
            crypto: Default::default(),
        }
    }
}

impl Default for structs::Id {
    fn default() -> Self {
        Self {
            id: String::new(),
        }
    }
}

impl Default for structs::VslmInfrastructureObjectPolicy {
    fn default() -> Self {
        Self {
            name: String::new(),
            backing_object_id: String::new(),
            profile_id: String::new(),
            error: None,
        }
    }
}

impl Default for structs::VslmInfrastructureObjectPolicySpec {
    fn default() -> Self {
        Self {
            datastore: structs::ManagedObjectReference::default(),
            profile: None,
        }
    }
}

impl Default for structs::VslmMigrateSpec {
    fn default() -> Self {
        Self {
            backing_spec: Default::default(),
            profile: None,
            consolidate: None,
            disks_crypto: None,
            service: None,
        }
    }
}

impl Default for structs::VslmCloneSpec {
    fn default() -> Self {
        Self {
            vslm_migrate_spec_: structs::VslmMigrateSpec::default(),
            name: String::new(),
            keep_after_delete_vm: None,
            metadata: None,
        }
    }
}

impl Default for structs::VslmRelocateSpec {
    fn default() -> Self {
        Self {
            vslm_migrate_spec_: structs::VslmMigrateSpec::default(),
        }
    }
}

impl Default for structs::VStorageObjectReconcileResult {
    fn default() -> Self {
        Self {
            reconcile_details: None,
        }
    }
}

impl Default for structs::VStorageObjectReconcileResultInvalidDiskPath {
    fn default() -> Self {
        Self {
            path: String::new(),
            reason: String::new(),
        }
    }
}

impl Default for structs::VStorageObjectReconcileResultReconcileDetail {
    fn default() -> Self {
        Self {
            host_name: None,
            reconcile_report_path: None,
            is_reconciled: None,
            is_deep_scanned: None,
            number_of_reconcile_issues: None,
            number_of_fcds_before_reconcile: None,
            number_of_fcds_after_reconcile: None,
            invalid_disk_paths: None,
        }
    }
}

impl Default for structs::VStorageObjectReconcileSpec {
    fn default() -> Self {
        Self {
            datastore: structs::ManagedObjectReference::default(),
            include_disk_paths: None,
            exclude_disk_paths: None,
            deep_scan: None,
            dry_run: None,
            generate_report: None,
        }
    }
}

impl Default for structs::VStorageObjectStateInfo {
    fn default() -> Self {
        Self {
            tentative: None,
        }
    }
}

impl Default for structs::VslmTagEntry {
    fn default() -> Self {
        Self {
            tag_name: String::new(),
            parent_category_name: String::new(),
        }
    }
}

impl Default for structs::VslmVClockInfo {
    fn default() -> Self {
        Self {
            v_clock_time: 0,
        }
    }
}

impl Default for structs::VStorageObject {
    fn default() -> Self {
        Self {
            config: structs::VStorageObjectConfigInfo::default(),
        }
    }
}

impl Default for structs::VStorageObjectSnapshot {
    fn default() -> Self {
        Self {
            id: structs::Id::default(),
            vclock: structs::VslmVClockInfo::default(),
            used_capacity: None,
        }
    }
}

impl Default for structs::VStorageObjectSnapshotDetails {
    fn default() -> Self {
        Self {
            path: None,
            changed_block_tracking_id: None,
        }
    }
}

impl Default for structs::VStorageObjectSnapshotInfo {
    fn default() -> Self {
        Self {
            snapshots: None,
        }
    }
}

impl Default for structs::VStorageObjectSnapshotInfoVStorageObjectSnapshot {
    fn default() -> Self {
        Self {
            id: None,
            backing_object_id: None,
            create_time: String::new(),
            description: String::new(),
        }
    }
}

impl Default for structs::RetrieveVStorageObjSpec {
    fn default() -> Self {
        Self {
            id: structs::Id::default(),
            datastore: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VStorageObjectAssociations {
    fn default() -> Self {
        Self {
            id: structs::Id::default(),
            vm_disk_associations: None,
            fault: None,
        }
    }
}

impl Default for structs::VStorageObjectAssociationsVmDiskAssociations {
    fn default() -> Self {
        Self {
            vm_id: String::new(),
            disk_key: 0,
        }
    }
}

impl Default for structs::DynamicArray {
    fn default() -> Self {
        Self {
            val: Vec::new(),
        }
    }
}

impl Default for structs::DynamicProperty {
    fn default() -> Self {
        Self {
            name: String::new(),
            val: Default::default(),
        }
    }
}

impl Default for structs::KeyAnyValue {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: Default::default(),
        }
    }
}

impl Default for structs::LocalizableMessage {
    fn default() -> Self {
        Self {
            key: String::new(),
            arg: None,
            message: None,
        }
    }
}

impl Default for structs::LocalizedMethodFault {
    fn default() -> Self {
        Self {
            fault: structs::MethodFault::default(),
            localized_message: None,
        }
    }
}

impl Default for structs::PropertyChange {
    fn default() -> Self {
        Self {
            name: String::new(),
            op: enums::PropertyChangeOpEnum::default(),
            val: None,
        }
    }
}

impl Default for structs::PropertyFilterSpec {
    fn default() -> Self {
        Self {
            prop_set: Vec::new(),
            object_set: Vec::new(),
            report_missing_objects_in_results: None,
        }
    }
}

impl Default for structs::PropertyFilterUpdate {
    fn default() -> Self {
        Self {
            filter: structs::ManagedObjectReference::default(),
            object_set: None,
            missing_set: None,
        }
    }
}

impl Default for structs::MissingObject {
    fn default() -> Self {
        Self {
            obj: structs::ManagedObjectReference::default(),
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::MissingProperty {
    fn default() -> Self {
        Self {
            path: String::new(),
            fault: structs::MethodFault::default(),
        }
    }
}

impl Default for structs::ObjectContent {
    fn default() -> Self {
        Self {
            obj: structs::ManagedObjectReference::default(),
            prop_set: None,
            missing_set: None,
        }
    }
}

impl Default for structs::ObjectSpec {
    fn default() -> Self {
        Self {
            obj: structs::ManagedObjectReference::default(),
            skip: None,
            select_set: None,
        }
    }
}

impl Default for structs::ObjectUpdate {
    fn default() -> Self {
        Self {
            kind: enums::ObjectUpdateKindEnum::default(),
            obj: structs::ManagedObjectReference::default(),
            change_set: None,
            missing_set: None,
        }
    }
}

impl Default for structs::PropertySpec {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            all: None,
            path_set: None,
        }
    }
}

impl Default for structs::RetrieveOptions {
    fn default() -> Self {
        Self {
            max_objects: None,
        }
    }
}

impl Default for structs::RetrieveResult {
    fn default() -> Self {
        Self {
            token: None,
            objects: Vec::new(),
        }
    }
}

impl Default for structs::SelectionSpec {
    fn default() -> Self {
        Self {
            name: None,
        }
    }
}

impl Default for structs::TraversalSpec {
    fn default() -> Self {
        Self {
            selection_spec_: structs::SelectionSpec::default(),
            r#type: String::new(),
            path: String::new(),
            skip: None,
            select_set: None,
        }
    }
}

impl Default for structs::UpdateSet {
    fn default() -> Self {
        Self {
            version: String::new(),
            filter_set: None,
            truncated: None,
        }
    }
}

impl Default for structs::WaitOptions {
    fn default() -> Self {
        Self {
            max_wait_seconds: None,
            max_object_updates: None,
        }
    }
}

impl Default for structs::VslmAboutInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            full_name: String::new(),
            vendor: String::new(),
            api_version: String::new(),
            instance_uuid: String::new(),
        }
    }
}

impl Default for structs::VslmQueryDatastoreInfoResult {
    fn default() -> Self {
        Self {
            datacenter: structs::ManagedObjectReference::default(),
            datastore: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VslmServiceInstanceContent {
    fn default() -> Self {
        Self {
            about_info: structs::VslmAboutInfo::default(),
            session_manager: structs::ManagedObjectReference::default(),
            v_storage_object_manager: structs::ManagedObjectReference::default(),
            storage_lifecycle_manager: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VslmTaskInfo {
    fn default() -> Self {
        Self {
            key: String::new(),
            task: structs::ManagedObjectReference::default(),
            description: None,
            name: None,
            description_id: String::new(),
            entity: None,
            entity_name: None,
            locked: None,
            state: enums::VslmTaskInfoStateEnum::default(),
            cancelled: false,
            cancelable: false,
            error: None,
            result: None,
            progress: None,
            reason: Default::default(),
            queue_time: String::new(),
            start_time: None,
            complete_time: None,
            event_chain_id: 0,
            change_tag: None,
            parent_task_key: None,
            root_task_key: None,
            activation_id: None,
        }
    }
}

impl Default for structs::VslmTaskReason {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VslmTaskReasonAlarm {
    fn default() -> Self {
        Self {
            alarm_name: String::new(),
            alarm: structs::ManagedObjectReference::default(),
            entity_name: String::new(),
            entity: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VslmTaskReasonSchedule {
    fn default() -> Self {
        Self {
            name: String::new(),
            scheduled_task: structs::ManagedObjectReference::default(),
        }
    }
}

impl Default for structs::VslmTaskReasonSystem {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for structs::VslmTaskReasonUser {
    fn default() -> Self {
        Self {
            user_name: String::new(),
        }
    }
}

impl Default for structs::VslmDatastoreSyncStatus {
    fn default() -> Self {
        Self {
            datastore_url: String::new(),
            object_v_clock: 0,
            sync_v_clock: 0,
            sync_time: None,
            number_of_retries: None,
            error: None,
        }
    }
}

impl Default for structs::VslmVsoVStorageObjectAssociations {
    fn default() -> Self {
        Self {
            id: structs::Id::default(),
            vm_disk_association: None,
            fault: None,
        }
    }
}

impl Default for structs::VslmVsoVStorageObjectAssociationsVmDiskAssociation {
    fn default() -> Self {
        Self {
            vm_id: String::new(),
            disk_key: 0,
        }
    }
}

impl Default for structs::VslmVsoVStorageObjectQueryResult {
    fn default() -> Self {
        Self {
            all_records_returned: false,
            id: None,
            query_results: None,
        }
    }
}

impl Default for structs::VslmVsoVStorageObjectQuerySpec {
    fn default() -> Self {
        Self {
            query_field: String::new(),
            query_operator: String::new(),
            query_value: None,
        }
    }
}

impl Default for structs::VslmVsoVStorageObjectResult {
    fn default() -> Self {
        Self {
            id: structs::Id::default(),
            name: None,
            capacity_in_mb: 0,
            create_time: None,
            datastore_url: None,
            disk_path: None,
            used_capacity_in_mb: None,
            backing_object_id: None,
            snapshot_info: None,
            metadata: None,
            error: None,
        }
    }
}

impl Default for structs::VslmVsoVStorageObjectSnapshotResult {
    fn default() -> Self {
        Self {
            backing_object_id: structs::Id::default(),
            description: None,
            snapshot_id: None,
            disk_path: None,
        }
    }
}

impl Default for structs::MethodFault {
    fn default() -> Self {
        Self {
            fault_cause: None,
            fault_message: None,
            type_: None,
            extra_fields_: std::collections::HashMap::new(),
        }
    }
}

impl Default for Box<dyn traits::DataObjectTrait> {
    fn default() -> Self {
        Box::new(structs::DataObject::default())
    }
}

impl Default for Box<dyn traits::AgencyScopeTrait> {
    fn default() -> Self {
        Box::new(structs::AgencyScope::default())
    }
}

impl Default for Box<dyn traits::AgentSslTrustTrait> {
    fn default() -> Self {
        Box::new(structs::AgentSslTrust::default())
    }
}

impl Default for Box<dyn traits::AgentStoragePolicyTrait> {
    fn default() -> Self {
        Box::new(structs::AgentStoragePolicy::default())
    }
}

impl Default for Box<dyn traits::EamObjectRuntimeInfoTrait> {
    fn default() -> Self {
        Box::new(structs::EamObjectRuntimeInfo::default())
    }
}

impl Default for Box<dyn traits::IssueTrait> {
    fn default() -> Self {
        Box::new(structs::Issue::default())
    }
}

impl Default for Box<dyn traits::AgencyIssueTrait> {
    fn default() -> Self {
        Box::new(structs::AgencyIssue::default())
    }
}

impl Default for Box<dyn traits::AgentIssueTrait> {
    fn default() -> Self {
        Box::new(structs::AgentIssue::default())
    }
}

impl Default for Box<dyn traits::VibIssueTrait> {
    fn default() -> Self {
        Box::new(structs::VibIssue::default())
    }
}

impl Default for Box<dyn traits::VibNotInstalledTrait> {
    fn default() -> Self {
        Box::new(structs::VibNotInstalled::default())
    }
}

impl Default for Box<dyn traits::VmIssueTrait> {
    fn default() -> Self {
        Box::new(structs::VmIssue::default())
    }
}

impl Default for Box<dyn traits::VmDeployedTrait> {
    fn default() -> Self {
        Box::new(structs::VmDeployed::default())
    }
}

impl Default for Box<dyn traits::VmPoweredOffTrait> {
    fn default() -> Self {
        Box::new(structs::VmPoweredOff::default())
    }
}

impl Default for Box<dyn traits::VmNotDeployedTrait> {
    fn default() -> Self {
        Box::new(structs::VmNotDeployed::default())
    }
}

impl Default for Box<dyn traits::NoAgentVmDatastoreTrait> {
    fn default() -> Self {
        Box::new(structs::NoAgentVmDatastore::default())
    }
}

impl Default for Box<dyn traits::NoAgentVmNetworkTrait> {
    fn default() -> Self {
        Box::new(structs::NoAgentVmNetwork::default())
    }
}

impl Default for Box<dyn traits::PersonalityAgentPmIssueTrait> {
    fn default() -> Self {
        Box::new(structs::PersonalityAgentPmIssue::default())
    }
}

impl Default for Box<dyn traits::ClusterAgentAgentIssueTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterAgentAgentIssue::default())
    }
}

impl Default for Box<dyn traits::ClusterAgentVmIssueTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterAgentVmIssue::default())
    }
}

impl Default for Box<dyn traits::ClusterAgentVmPoweredOffTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterAgentVmPoweredOff::default())
    }
}

impl Default for Box<dyn traits::ClusterAgentVmNotDeployedTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterAgentVmNotDeployed::default())
    }
}

impl Default for Box<dyn traits::IntegrityAgencyVumIssueTrait> {
    fn default() -> Self {
        Box::new(structs::IntegrityAgencyVumIssue::default())
    }
}

impl Default for Box<dyn traits::PersonalityAgencyPmIssueTrait> {
    fn default() -> Self {
        Box::new(structs::PersonalityAgencyPmIssue::default())
    }
}

impl Default for Box<dyn traits::PersonalityAgencyDepotIssueTrait> {
    fn default() -> Self {
        Box::new(structs::PersonalityAgencyDepotIssue::default())
    }
}

impl Default for Box<dyn traits::HostIssueTrait> {
    fn default() -> Self {
        Box::new(structs::HostIssue::default())
    }
}

impl Default for Box<dyn traits::SolutionsHookAcknowledgeConfigTrait> {
    fn default() -> Self {
        Box::new(structs::SolutionsHookAcknowledgeConfig::default())
    }
}

impl Default for Box<dyn traits::SolutionsStoragePolicyTrait> {
    fn default() -> Self {
        Box::new(structs::SolutionsStoragePolicy::default())
    }
}

impl Default for Box<dyn traits::SolutionsTypeSpecificSolutionConfigTrait> {
    fn default() -> Self {
        Box::new(structs::SolutionsTypeSpecificSolutionConfig::default())
    }
}

impl Default for Box<dyn traits::SolutionsVmSourceTrait> {
    fn default() -> Self {
        Box::new(structs::SolutionsVmSource::default())
    }
}

impl Default for Box<dyn traits::VibVibServicesSslTrustTrait> {
    fn default() -> Self {
        Box::new(structs::VibVibServicesSslTrust::default())
    }
}

impl Default for Box<dyn traits::PbmCapabilityTypeInfoTrait> {
    fn default() -> Self {
        Box::new(structs::PbmCapabilityTypeInfo::default())
    }
}

impl Default for Box<dyn traits::PbmLineOfServiceInfoTrait> {
    fn default() -> Self {
        Box::new(structs::PbmLineOfServiceInfo::default())
    }
}

impl Default for Box<dyn traits::PbmPlacementMatchingResourcesTrait> {
    fn default() -> Self {
        Box::new(structs::PbmPlacementMatchingResources::default())
    }
}

impl Default for Box<dyn traits::PbmPlacementRequirementTrait> {
    fn default() -> Self {
        Box::new(structs::PbmPlacementRequirement::default())
    }
}

impl Default for Box<dyn traits::PbmCapabilityConstraintsTrait> {
    fn default() -> Self {
        Box::new(structs::PbmCapabilityConstraints::default())
    }
}

impl Default for Box<dyn traits::PbmProfileTrait> {
    fn default() -> Self {
        Box::new(structs::PbmProfile::default())
    }
}

impl Default for Box<dyn traits::PbmCapabilityProfileTrait> {
    fn default() -> Self {
        Box::new(structs::PbmCapabilityProfile::default())
    }
}

impl Default for Box<dyn traits::SmsProviderInfoTrait> {
    fn default() -> Self {
        Box::new(structs::SmsProviderInfo::default())
    }
}

impl Default for Box<dyn traits::SmsProviderSpecTrait> {
    fn default() -> Self {
        Box::new(structs::SmsProviderSpec::default())
    }
}

impl Default for Box<dyn traits::StoragePortTrait> {
    fn default() -> Self {
        Box::new(structs::StoragePort::default())
    }
}

impl Default for Box<dyn traits::DeviceIdTrait> {
    fn default() -> Self {
        Box::new(structs::DeviceId::default())
    }
}

impl Default for Box<dyn traits::VirtualMachineIdTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachineId::default())
    }
}

impl Default for Box<dyn traits::FailoverParamTrait> {
    fn default() -> Self {
        Box::new(structs::FailoverParam::default())
    }
}

impl Default for Box<dyn traits::GroupInfoTrait> {
    fn default() -> Self {
        Box::new(structs::GroupInfo::default())
    }
}

impl Default for Box<dyn traits::GroupOperationResultTrait> {
    fn default() -> Self {
        Box::new(structs::GroupOperationResult::default())
    }
}

impl Default for Box<dyn traits::TargetGroupMemberInfoTrait> {
    fn default() -> Self {
        Box::new(structs::TargetGroupMemberInfo::default())
    }
}

impl Default for Box<dyn traits::ClusterComputeResourceValidationResultBaseTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterComputeResourceValidationResultBase::default())
    }
}

impl Default for Box<dyn traits::ComputeResourceConfigInfoTrait> {
    fn default() -> Self {
        Box::new(structs::ComputeResourceConfigInfo::default())
    }
}

impl Default for Box<dyn traits::ComputeResourceConfigSpecTrait> {
    fn default() -> Self {
        Box::new(structs::ComputeResourceConfigSpec::default())
    }
}

impl Default for Box<dyn traits::ComputeResourceSummaryTrait> {
    fn default() -> Self {
        Box::new(structs::ComputeResourceSummary::default())
    }
}

impl Default for Box<dyn traits::CustomFieldValueTrait> {
    fn default() -> Self {
        Box::new(structs::CustomFieldValue::default())
    }
}

impl Default for Box<dyn traits::DatastoreInfoTrait> {
    fn default() -> Self {
        Box::new(structs::DatastoreInfo::default())
    }
}

impl Default for Box<dyn traits::DescriptionTrait> {
    fn default() -> Self {
        Box::new(structs::Description::default())
    }
}

impl Default for Box<dyn traits::ElementDescriptionTrait> {
    fn default() -> Self {
        Box::new(structs::ElementDescription::default())
    }
}

impl Default for Box<dyn traits::TypeDescriptionTrait> {
    fn default() -> Self {
        Box::new(structs::TypeDescription::default())
    }
}

impl Default for Box<dyn traits::DirectPathProfileManagerCapacityQuerySpecTrait> {
    fn default() -> Self {
        Box::new(structs::DirectPathProfileManagerCapacityQuerySpec::default())
    }
}

impl Default for Box<dyn traits::DirectPathProfileManagerCapacityResultTrait> {
    fn default() -> Self {
        Box::new(structs::DirectPathProfileManagerCapacityResult::default())
    }
}

impl Default for Box<dyn traits::DirectPathProfileManagerDirectPathConfigTrait> {
    fn default() -> Self {
        Box::new(structs::DirectPathProfileManagerDirectPathConfig::default())
    }
}

impl Default for Box<dyn traits::DirectPathProfileManagerTargetEntityTrait> {
    fn default() -> Self {
        Box::new(structs::DirectPathProfileManagerTargetEntity::default())
    }
}

impl Default for Box<dyn traits::DvsConfigInfoTrait> {
    fn default() -> Self {
        Box::new(structs::DvsConfigInfo::default())
    }
}

impl Default for Box<dyn traits::DvsConfigSpecTrait> {
    fn default() -> Self {
        Box::new(structs::DvsConfigSpec::default())
    }
}

impl Default for Box<dyn traits::DvsFeatureCapabilityTrait> {
    fn default() -> Self {
        Box::new(structs::DvsFeatureCapability::default())
    }
}

impl Default for Box<dyn traits::DvsHealthCheckConfigTrait> {
    fn default() -> Self {
        Box::new(structs::DvsHealthCheckConfig::default())
    }
}

impl Default for Box<dyn traits::VMwareDvsHealthCheckConfigTrait> {
    fn default() -> Self {
        Box::new(structs::VMwareDvsHealthCheckConfig::default())
    }
}

impl Default for Box<dyn traits::DvsHealthCheckCapabilityTrait> {
    fn default() -> Self {
        Box::new(structs::DvsHealthCheckCapability::default())
    }
}

impl Default for Box<dyn traits::DvsUplinkPortPolicyTrait> {
    fn default() -> Self {
        Box::new(structs::DvsUplinkPortPolicy::default())
    }
}

impl Default for Box<dyn traits::HbrReplicationTargetSpecTrait> {
    fn default() -> Self {
        Box::new(structs::HbrReplicationTargetSpec::default())
    }
}

impl Default for Box<dyn traits::ImportSpecTrait> {
    fn default() -> Self {
        Box::new(structs::ImportSpec::default())
    }
}

impl Default for Box<dyn traits::InheritablePolicyTrait> {
    fn default() -> Self {
        Box::new(structs::InheritablePolicy::default())
    }
}

impl Default for Box<dyn traits::DvsFilterConfigTrait> {
    fn default() -> Self {
        Box::new(structs::DvsFilterConfig::default())
    }
}

impl Default for Box<dyn traits::DvsTrafficFilterConfigTrait> {
    fn default() -> Self {
        Box::new(structs::DvsTrafficFilterConfig::default())
    }
}

impl Default for Box<dyn traits::VmwareDistributedVirtualSwitchVlanSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VmwareDistributedVirtualSwitchVlanSpec::default())
    }
}

impl Default for Box<dyn traits::IoFilterInfoTrait> {
    fn default() -> Self {
        Box::new(structs::IoFilterInfo::default())
    }
}

impl Default for Box<dyn traits::IoFilterManagerSslTrustTrait> {
    fn default() -> Self {
        Box::new(structs::IoFilterManagerSslTrust::default())
    }
}

impl Default for Box<dyn traits::LicenseSourceTrait> {
    fn default() -> Self {
        Box::new(structs::LicenseSource::default())
    }
}

impl Default for Box<dyn traits::NegatableExpressionTrait> {
    fn default() -> Self {
        Box::new(structs::NegatableExpression::default())
    }
}

impl Default for Box<dyn traits::IpAddressTrait> {
    fn default() -> Self {
        Box::new(structs::IpAddress::default())
    }
}

impl Default for Box<dyn traits::MacAddressTrait> {
    fn default() -> Self {
        Box::new(structs::MacAddress::default())
    }
}

impl Default for Box<dyn traits::DvsIpPortTrait> {
    fn default() -> Self {
        Box::new(structs::DvsIpPort::default())
    }
}

impl Default for Box<dyn traits::NetworkSummaryTrait> {
    fn default() -> Self {
        Box::new(structs::NetworkSummary::default())
    }
}

impl Default for Box<dyn traits::OvfManagerCommonParamsTrait> {
    fn default() -> Self {
        Box::new(structs::OvfManagerCommonParams::default())
    }
}

impl Default for Box<dyn traits::OvfCreateImportSpecParamsTrait> {
    fn default() -> Self {
        Box::new(structs::OvfCreateImportSpecParams::default())
    }
}

impl Default for Box<dyn traits::PerfEntityMetricBaseTrait> {
    fn default() -> Self {
        Box::new(structs::PerfEntityMetricBase::default())
    }
}

impl Default for Box<dyn traits::PerfMetricSeriesTrait> {
    fn default() -> Self {
        Box::new(structs::PerfMetricSeries::default())
    }
}

impl Default for Box<dyn traits::ResourcePoolSummaryTrait> {
    fn default() -> Self {
        Box::new(structs::ResourcePoolSummary::default())
    }
}

impl Default for Box<dyn traits::SddcBaseTrait> {
    fn default() -> Self {
        Box::new(structs::SddcBase::default())
    }
}

impl Default for Box<dyn traits::SelectionSetTrait> {
    fn default() -> Self {
        Box::new(structs::SelectionSet::default())
    }
}

impl Default for Box<dyn traits::ServiceLocatorCredentialTrait> {
    fn default() -> Self {
        Box::new(structs::ServiceLocatorCredential::default())
    }
}

impl Default for Box<dyn traits::SessionManagerServiceRequestSpecTrait> {
    fn default() -> Self {
        Box::new(structs::SessionManagerServiceRequestSpec::default())
    }
}

impl Default for Box<dyn traits::TaskManagerTaskViewSpecTrait> {
    fn default() -> Self {
        Box::new(structs::TaskManagerTaskViewSpec::default())
    }
}

impl Default for Box<dyn traits::TaskReasonTrait> {
    fn default() -> Self {
        Box::new(structs::TaskReason::default())
    }
}

impl Default for Box<dyn traits::UserSearchResultTrait> {
    fn default() -> Self {
        Box::new(structs::UserSearchResult::default())
    }
}

impl Default for Box<dyn traits::VirtualDiskSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDiskSpec::default())
    }
}

impl Default for Box<dyn traits::FileBackedVirtualDiskSpecTrait> {
    fn default() -> Self {
        Box::new(structs::FileBackedVirtualDiskSpec::default())
    }
}

impl Default for Box<dyn traits::VirtualMachineConnectionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachineConnection::default())
    }
}

impl Default for Box<dyn traits::VsanComparatorTrait> {
    fn default() -> Self {
        Box::new(structs::VsanComparator::default())
    }
}

impl Default for Box<dyn traits::VsanResourceConstraintTrait> {
    fn default() -> Self {
        Box::new(structs::VsanResourceConstraint::default())
    }
}

impl Default for Box<dyn traits::VsanUpgradeSystemPreflightCheckIssueTrait> {
    fn default() -> Self {
        Box::new(structs::VsanUpgradeSystemPreflightCheckIssue::default())
    }
}

impl Default for Box<dyn traits::VsanUpgradeSystemPreflightCheckResultTrait> {
    fn default() -> Self {
        Box::new(structs::VsanUpgradeSystemPreflightCheckResult::default())
    }
}

impl Default for Box<dyn traits::VsanUpgradeSystemUpgradeHistoryItemTrait> {
    fn default() -> Self {
        Box::new(structs::VsanUpgradeSystemUpgradeHistoryItem::default())
    }
}

impl Default for Box<dyn traits::VsanUpgradeSystemUpgradeStatusTrait> {
    fn default() -> Self {
        Box::new(structs::VsanUpgradeSystemUpgradeStatus::default())
    }
}

impl Default for Box<dyn traits::ActionTrait> {
    fn default() -> Self {
        Box::new(structs::Action::default())
    }
}

impl Default for Box<dyn traits::AlarmActionTrait> {
    fn default() -> Self {
        Box::new(structs::AlarmAction::default())
    }
}

impl Default for Box<dyn traits::AlarmExpressionTrait> {
    fn default() -> Self {
        Box::new(structs::AlarmExpression::default())
    }
}

impl Default for Box<dyn traits::AlarmSpecTrait> {
    fn default() -> Self {
        Box::new(structs::AlarmSpec::default())
    }
}

impl Default for Box<dyn traits::ClusterActionTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterAction::default())
    }
}

impl Default for Box<dyn traits::ClusterDasAdmissionControlInfoTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterDasAdmissionControlInfo::default())
    }
}

impl Default for Box<dyn traits::ClusterDasAdmissionControlPolicyTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterDasAdmissionControlPolicy::default())
    }
}

impl Default for Box<dyn traits::ClusterDasAdvancedRuntimeInfoTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterDasAdvancedRuntimeInfo::default())
    }
}

impl Default for Box<dyn traits::ClusterDasDataTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterDasData::default())
    }
}

impl Default for Box<dyn traits::ClusterDasHostInfoTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterDasHostInfo::default())
    }
}

impl Default for Box<dyn traits::ClusterDrsFaultsFaultsByVmTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterDrsFaultsFaultsByVm::default())
    }
}

impl Default for Box<dyn traits::ClusterGroupInfoTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterGroupInfo::default())
    }
}

impl Default for Box<dyn traits::ClusterRuleInfoTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterRuleInfo::default())
    }
}

impl Default for Box<dyn traits::ClusterSlotPolicyTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterSlotPolicy::default())
    }
}

impl Default for Box<dyn traits::VsanClusterHealthLinkBaseTrait> {
    fn default() -> Self {
        Box::new(structs::VsanClusterHealthLinkBase::default())
    }
}

impl Default for Box<dyn traits::VsanClusterHealthResultBaseTrait> {
    fn default() -> Self {
        Box::new(structs::VsanClusterHealthResultBase::default())
    }
}

impl Default for Box<dyn traits::VimClusterVsanFaultDomainSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VimClusterVsanFaultDomainSpec::default())
    }
}

impl Default for Box<dyn traits::VsanHealthActionBaseTrait> {
    fn default() -> Self {
        Box::new(structs::VsanHealthActionBase::default())
    }
}

impl Default for Box<dyn traits::VsanIscsiLunCommonInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VsanIscsiLunCommonInfo::default())
    }
}

impl Default for Box<dyn traits::VsanIscsiTargetBasicInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VsanIscsiTargetBasicInfo::default())
    }
}

impl Default for Box<dyn traits::VsanIscsiTargetCommonInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VsanIscsiTargetCommonInfo::default())
    }
}

impl Default for Box<dyn traits::VsanIscsiTargetServiceConfigTrait> {
    fn default() -> Self {
        Box::new(structs::VsanIscsiTargetServiceConfig::default())
    }
}

impl Default for Box<dyn traits::CnsAccessControlSpecTrait> {
    fn default() -> Self {
        Box::new(structs::CnsAccessControlSpec::default())
    }
}

impl Default for Box<dyn traits::CnsBackingObjectDetailsTrait> {
    fn default() -> Self {
        Box::new(structs::CnsBackingObjectDetails::default())
    }
}

impl Default for Box<dyn traits::CnsFileBackingDetailsTrait> {
    fn default() -> Self {
        Box::new(structs::CnsFileBackingDetails::default())
    }
}

impl Default for Box<dyn traits::CnsBaseCreateSpecTrait> {
    fn default() -> Self {
        Box::new(structs::CnsBaseCreateSpec::default())
    }
}

impl Default for Box<dyn traits::CnsFileCreateSpecTrait> {
    fn default() -> Self {
        Box::new(structs::CnsFileCreateSpec::default())
    }
}

impl Default for Box<dyn traits::CnsEntityMetadataTrait> {
    fn default() -> Self {
        Box::new(structs::CnsEntityMetadata::default())
    }
}

impl Default for Box<dyn traits::CnsQueryFilterTrait> {
    fn default() -> Self {
        Box::new(structs::CnsQueryFilter::default())
    }
}

impl Default for Box<dyn traits::CnsVolumeOperationResultTrait> {
    fn default() -> Self {
        Box::new(structs::CnsVolumeOperationResult::default())
    }
}

impl Default for Box<dyn traits::CnsVolumeRelocateSpecTrait> {
    fn default() -> Self {
        Box::new(structs::CnsVolumeRelocateSpec::default())
    }
}

impl Default for Box<dyn traits::CnsVolumeSourceTrait> {
    fn default() -> Self {
        Box::new(structs::CnsVolumeSource::default())
    }
}

impl Default for Box<dyn traits::DvPortSettingTrait> {
    fn default() -> Self {
        Box::new(structs::DvPortSetting::default())
    }
}

impl Default for Box<dyn traits::DvPortgroupPolicyTrait> {
    fn default() -> Self {
        Box::new(structs::DvPortgroupPolicy::default())
    }
}

impl Default for Box<dyn traits::DistributedVirtualSwitchManagerHostDvsFilterSpecTrait> {
    fn default() -> Self {
        Box::new(structs::DistributedVirtualSwitchManagerHostDvsFilterSpec::default())
    }
}

impl Default for Box<dyn traits::DvsFilterSpecConnecteeSpecTrait> {
    fn default() -> Self {
        Box::new(structs::DvsFilterSpecConnecteeSpec::default())
    }
}

impl Default for Box<dyn traits::DvsFilterSpecVlanSpecTrait> {
    fn default() -> Self {
        Box::new(structs::DvsFilterSpecVlanSpec::default())
    }
}

impl Default for Box<dyn traits::DistributedVirtualSwitchHostMemberBackingTrait> {
    fn default() -> Self {
        Box::new(structs::DistributedVirtualSwitchHostMemberBacking::default())
    }
}

impl Default for Box<dyn traits::HostMemberHealthCheckResultTrait> {
    fn default() -> Self {
        Box::new(structs::HostMemberHealthCheckResult::default())
    }
}

impl Default for Box<dyn traits::HostMemberUplinkHealthCheckResultTrait> {
    fn default() -> Self {
        Box::new(structs::HostMemberUplinkHealthCheckResult::default())
    }
}

impl Default for Box<dyn traits::DvsNetworkRuleActionTrait> {
    fn default() -> Self {
        Box::new(structs::DvsNetworkRuleAction::default())
    }
}

impl Default for Box<dyn traits::DvsNetworkRuleQualifierTrait> {
    fn default() -> Self {
        Box::new(structs::DvsNetworkRuleQualifier::default())
    }
}

impl Default for Box<dyn traits::CryptoManagerKmipCryptoKeyStatusKeyInfoTrait> {
    fn default() -> Self {
        Box::new(structs::CryptoManagerKmipCryptoKeyStatusKeyInfo::default())
    }
}

impl Default for Box<dyn traits::CryptoSpecTrait> {
    fn default() -> Self {
        Box::new(structs::CryptoSpec::default())
    }
}

impl Default for Box<dyn traits::CryptoSpecNoOpTrait> {
    fn default() -> Self {
        Box::new(structs::CryptoSpecNoOp::default())
    }
}

impl Default for Box<dyn traits::KmipClusterInfoKeyInfoTrait> {
    fn default() -> Self {
        Box::new(structs::KmipClusterInfoKeyInfo::default())
    }
}

impl Default for Box<dyn traits::KmipServerSpecKeySpecTrait> {
    fn default() -> Self {
        Box::new(structs::KmipServerSpecKeySpec::default())
    }
}

impl Default for Box<dyn traits::EventArgumentTrait> {
    fn default() -> Self {
        Box::new(structs::EventArgument::default())
    }
}

impl Default for Box<dyn traits::EntityEventArgumentTrait> {
    fn default() -> Self {
        Box::new(structs::EntityEventArgument::default())
    }
}

impl Default for Box<dyn traits::EventManagerEventViewSpecTrait> {
    fn default() -> Self {
        Box::new(structs::EventManagerEventViewSpec::default())
    }
}

impl Default for Box<dyn traits::HostAuthenticationStoreInfoTrait> {
    fn default() -> Self {
        Box::new(structs::HostAuthenticationStoreInfo::default())
    }
}

impl Default for Box<dyn traits::HostDirectoryStoreInfoTrait> {
    fn default() -> Self {
        Box::new(structs::HostDirectoryStoreInfo::default())
    }
}

impl Default for Box<dyn traits::HostDatastoreConnectInfoTrait> {
    fn default() -> Self {
        Box::new(structs::HostDatastoreConnectInfo::default())
    }
}

impl Default for Box<dyn traits::HostConnectInfoNetworkInfoTrait> {
    fn default() -> Self {
        Box::new(structs::HostConnectInfoNetworkInfo::default())
    }
}

impl Default for Box<dyn traits::HostDataTransportConnectionInfoTrait> {
    fn default() -> Self {
        Box::new(structs::HostDataTransportConnectionInfo::default())
    }
}

impl Default for Box<dyn traits::FileInfoTrait> {
    fn default() -> Self {
        Box::new(structs::FileInfo::default())
    }
}

impl Default for Box<dyn traits::VmConfigFileInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VmConfigFileInfo::default())
    }
}

impl Default for Box<dyn traits::FileQueryTrait> {
    fn default() -> Self {
        Box::new(structs::FileQuery::default())
    }
}

impl Default for Box<dyn traits::VmConfigFileQueryTrait> {
    fn default() -> Self {
        Box::new(structs::VmConfigFileQuery::default())
    }
}

impl Default for Box<dyn traits::HostDeviceTrait> {
    fn default() -> Self {
        Box::new(structs::HostDevice::default())
    }
}

impl Default for Box<dyn traits::ScsiLunTrait> {
    fn default() -> Self {
        Box::new(structs::ScsiLun::default())
    }
}

impl Default for Box<dyn traits::HostDigestInfoTrait> {
    fn default() -> Self {
        Box::new(structs::HostDigestInfo::default())
    }
}

impl Default for Box<dyn traits::HostDnsConfigTrait> {
    fn default() -> Self {
        Box::new(structs::HostDnsConfig::default())
    }
}

impl Default for Box<dyn traits::HostFileSystemVolumeTrait> {
    fn default() -> Self {
        Box::new(structs::HostFileSystemVolume::default())
    }
}

impl Default for Box<dyn traits::HostHardwareElementInfoTrait> {
    fn default() -> Self {
        Box::new(structs::HostHardwareElementInfo::default())
    }
}

impl Default for Box<dyn traits::HostHbaCreateSpecTrait> {
    fn default() -> Self {
        Box::new(structs::HostHbaCreateSpec::default())
    }
}

impl Default for Box<dyn traits::HostHostBusAdapterTrait> {
    fn default() -> Self {
        Box::new(structs::HostHostBusAdapter::default())
    }
}

impl Default for Box<dyn traits::HostFibreChannelHbaTrait> {
    fn default() -> Self {
        Box::new(structs::HostFibreChannelHba::default())
    }
}

impl Default for Box<dyn traits::HostIpConfigTrait> {
    fn default() -> Self {
        Box::new(structs::HostIpConfig::default())
    }
}

impl Default for Box<dyn traits::HostIpRouteConfigTrait> {
    fn default() -> Self {
        Box::new(structs::HostIpRouteConfig::default())
    }
}

impl Default for Box<dyn traits::HostAccountSpecTrait> {
    fn default() -> Self {
        Box::new(structs::HostAccountSpec::default())
    }
}

impl Default for Box<dyn traits::HostMultipathInfoLogicalUnitPolicyTrait> {
    fn default() -> Self {
        Box::new(structs::HostMultipathInfoLogicalUnitPolicy::default())
    }
}

impl Default for Box<dyn traits::HostNvmeSpecTrait> {
    fn default() -> Self {
        Box::new(structs::HostNvmeSpec::default())
    }
}

impl Default for Box<dyn traits::HostNvmeTransportParametersTrait> {
    fn default() -> Self {
        Box::new(structs::HostNvmeTransportParameters::default())
    }
}

impl Default for Box<dyn traits::HostPciPassthruConfigTrait> {
    fn default() -> Self {
        Box::new(structs::HostPciPassthruConfig::default())
    }
}

impl Default for Box<dyn traits::HostPciPassthruInfoTrait> {
    fn default() -> Self {
        Box::new(structs::HostPciPassthruInfo::default())
    }
}

impl Default for Box<dyn traits::PhysicalNicHintTrait> {
    fn default() -> Self {
        Box::new(structs::PhysicalNicHint::default())
    }
}

impl Default for Box<dyn traits::HostRdmaDeviceBackingTrait> {
    fn default() -> Self {
        Box::new(structs::HostRdmaDeviceBacking::default())
    }
}

impl Default for Box<dyn traits::HostSriovDevicePoolInfoTrait> {
    fn default() -> Self {
        Box::new(structs::HostSriovDevicePoolInfo::default())
    }
}

impl Default for Box<dyn traits::HostSystemSwapConfigurationSystemSwapOptionTrait> {
    fn default() -> Self {
        Box::new(structs::HostSystemSwapConfigurationSystemSwapOption::default())
    }
}

impl Default for Box<dyn traits::HostTargetTransportTrait> {
    fn default() -> Self {
        Box::new(structs::HostTargetTransport::default())
    }
}

impl Default for Box<dyn traits::HostFibreChannelTargetTransportTrait> {
    fn default() -> Self {
        Box::new(structs::HostFibreChannelTargetTransport::default())
    }
}

impl Default for Box<dyn traits::HostTpmEventDetailsTrait> {
    fn default() -> Self {
        Box::new(structs::HostTpmEventDetails::default())
    }
}

impl Default for Box<dyn traits::HostTpmBootSecurityOptionEventDetailsTrait> {
    fn default() -> Self {
        Box::new(structs::HostTpmBootSecurityOptionEventDetails::default())
    }
}

impl Default for Box<dyn traits::HostVirtualSwitchBridgeTrait> {
    fn default() -> Self {
        Box::new(structs::HostVirtualSwitchBridge::default())
    }
}

impl Default for Box<dyn traits::VmfsDatastoreBaseOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VmfsDatastoreBaseOption::default())
    }
}

impl Default for Box<dyn traits::VmfsDatastoreSingleExtentOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VmfsDatastoreSingleExtentOption::default())
    }
}

impl Default for Box<dyn traits::VmfsDatastoreSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VmfsDatastoreSpec::default())
    }
}

impl Default for Box<dyn traits::VsanHclCommonDeviceInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VsanHclCommonDeviceInfo::default())
    }
}

impl Default for Box<dyn traits::NetBiosConfigInfoTrait> {
    fn default() -> Self {
        Box::new(structs::NetBiosConfigInfo::default())
    }
}

impl Default for Box<dyn traits::ArrayUpdateSpecTrait> {
    fn default() -> Self {
        Box::new(structs::ArrayUpdateSpec::default())
    }
}

impl Default for Box<dyn traits::OptionTypeTrait> {
    fn default() -> Self {
        Box::new(structs::OptionType::default())
    }
}

impl Default for Box<dyn traits::OptionValueTrait> {
    fn default() -> Self {
        Box::new(structs::OptionValue::default())
    }
}

impl Default for Box<dyn traits::ApplyProfileTrait> {
    fn default() -> Self {
        Box::new(structs::ApplyProfile::default())
    }
}

impl Default for Box<dyn traits::DvsVNicProfileTrait> {
    fn default() -> Self {
        Box::new(structs::DvsVNicProfile::default())
    }
}

impl Default for Box<dyn traits::PortGroupProfileTrait> {
    fn default() -> Self {
        Box::new(structs::PortGroupProfile::default())
    }
}

impl Default for Box<dyn traits::ProfileExpressionTrait> {
    fn default() -> Self {
        Box::new(structs::ProfileExpression::default())
    }
}

impl Default for Box<dyn traits::PolicyOptionTrait> {
    fn default() -> Self {
        Box::new(structs::PolicyOption::default())
    }
}

impl Default for Box<dyn traits::ProfilePolicyOptionMetadataTrait> {
    fn default() -> Self {
        Box::new(structs::ProfilePolicyOptionMetadata::default())
    }
}

impl Default for Box<dyn traits::ProfileConfigInfoTrait> {
    fn default() -> Self {
        Box::new(structs::ProfileConfigInfo::default())
    }
}

impl Default for Box<dyn traits::ProfileCreateSpecTrait> {
    fn default() -> Self {
        Box::new(structs::ProfileCreateSpec::default())
    }
}

impl Default for Box<dyn traits::ProfileSerializedCreateSpecTrait> {
    fn default() -> Self {
        Box::new(structs::ProfileSerializedCreateSpec::default())
    }
}

impl Default for Box<dyn traits::ClusterProfileCreateSpecTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterProfileCreateSpec::default())
    }
}

impl Default for Box<dyn traits::ClusterProfileConfigSpecTrait> {
    fn default() -> Self {
        Box::new(structs::ClusterProfileConfigSpec::default())
    }
}

impl Default for Box<dyn traits::HostProfileConfigSpecTrait> {
    fn default() -> Self {
        Box::new(structs::HostProfileConfigSpec::default())
    }
}

impl Default for Box<dyn traits::ProfileExecuteResultTrait> {
    fn default() -> Self {
        Box::new(structs::ProfileExecuteResult::default())
    }
}

impl Default for Box<dyn traits::AnswerFileCreateSpecTrait> {
    fn default() -> Self {
        Box::new(structs::AnswerFileCreateSpec::default())
    }
}

impl Default for Box<dyn traits::HostProfilesEntityCustomizationsTrait> {
    fn default() -> Self {
        Box::new(structs::HostProfilesEntityCustomizations::default())
    }
}

impl Default for Box<dyn traits::ScheduledTaskSpecTrait> {
    fn default() -> Self {
        Box::new(structs::ScheduledTaskSpec::default())
    }
}

impl Default for Box<dyn traits::TaskSchedulerTrait> {
    fn default() -> Self {
        Box::new(structs::TaskScheduler::default())
    }
}

impl Default for Box<dyn traits::RecurrentTaskSchedulerTrait> {
    fn default() -> Self {
        Box::new(structs::RecurrentTaskScheduler::default())
    }
}

impl Default for Box<dyn traits::HourlyTaskSchedulerTrait> {
    fn default() -> Self {
        Box::new(structs::HourlyTaskScheduler::default())
    }
}

impl Default for Box<dyn traits::DailyTaskSchedulerTrait> {
    fn default() -> Self {
        Box::new(structs::DailyTaskScheduler::default())
    }
}

impl Default for Box<dyn traits::MonthlyTaskSchedulerTrait> {
    fn default() -> Self {
        Box::new(structs::MonthlyTaskScheduler::default())
    }
}

impl Default for Box<dyn traits::VmConfigInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VmConfigInfo::default())
    }
}

impl Default for Box<dyn traits::VmConfigSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VmConfigSpec::default())
    }
}

impl Default for Box<dyn traits::NodeDeploymentSpecTrait> {
    fn default() -> Self {
        Box::new(structs::NodeDeploymentSpec::default())
    }
}

impl Default for Box<dyn traits::NodeNetworkSpecTrait> {
    fn default() -> Self {
        Box::new(structs::NodeNetworkSpec::default())
    }
}

impl Default for Box<dyn traits::VirtualMachineBaseIndependentFilterSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachineBaseIndependentFilterSpec::default())
    }
}

impl Default for Box<dyn traits::VirtualMachineBootOptionsBootableDeviceTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachineBootOptionsBootableDevice::default())
    }
}

impl Default for Box<dyn traits::VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachineDeviceRuntimeInfoDeviceRuntimeState::default())
    }
}

impl Default for Box<dyn traits::FaultToleranceConfigInfoTrait> {
    fn default() -> Self {
        Box::new(structs::FaultToleranceConfigInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualMachineGuestQuiesceSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachineGuestQuiesceSpec::default())
    }
}

impl Default for Box<dyn traits::VirtualMachineProfileSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachineProfileSpec::default())
    }
}

impl Default for Box<dyn traits::VirtualMachineSriovDevicePoolInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachineSriovDevicePoolInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualMachineTargetInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachineTargetInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualMachineDiskDeviceInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachineDiskDeviceInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualMachinePciPassthroughInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachinePciPassthroughInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualMachineVirtualDeviceGroupsDeviceGroupTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualMachineVirtualDeviceGroupsDeviceGroup::default())
    }
}

impl Default for Box<dyn traits::CustomizationIdentitySettingsTrait> {
    fn default() -> Self {
        Box::new(structs::CustomizationIdentitySettings::default())
    }
}

impl Default for Box<dyn traits::CustomizationIpGeneratorTrait> {
    fn default() -> Self {
        Box::new(structs::CustomizationIpGenerator::default())
    }
}

impl Default for Box<dyn traits::CustomizationIpV6GeneratorTrait> {
    fn default() -> Self {
        Box::new(structs::CustomizationIpV6Generator::default())
    }
}

impl Default for Box<dyn traits::CustomizationNameTrait> {
    fn default() -> Self {
        Box::new(structs::CustomizationName::default())
    }
}

impl Default for Box<dyn traits::CustomizationOptionsTrait> {
    fn default() -> Self {
        Box::new(structs::CustomizationOptions::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDevice::default())
    }
}

impl Default for Box<dyn traits::VirtualControllerTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualController::default())
    }
}

impl Default for Box<dyn traits::VirtualSataControllerTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualSataController::default())
    }
}

impl Default for Box<dyn traits::VirtualScsiControllerTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualScsiController::default())
    }
}

impl Default for Box<dyn traits::VirtualEthernetCardTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualEthernetCard::default())
    }
}

impl Default for Box<dyn traits::VirtualVmxnetTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualVmxnet::default())
    }
}

impl Default for Box<dyn traits::VirtualVmxnet3Trait> {
    fn default() -> Self {
        Box::new(structs::VirtualVmxnet3::default())
    }
}

impl Default for Box<dyn traits::VirtualSoundCardTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualSoundCard::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceBackingInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceBackingInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceDeviceBackingInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceDeviceBackingInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualDiskRawDiskVer2BackingInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDiskRawDiskVer2BackingInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceFileBackingInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceFileBackingInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualDevicePipeBackingInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDevicePipeBackingInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceRemoteDeviceBackingInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceRemoteDeviceBackingInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceUriBackingInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceUriBackingInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualPciPassthroughPluginBackingInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualPciPassthroughPluginBackingInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceBusSlotInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceBusSlotInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualDevicePciBusSlotInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDevicePciBusSlotInfo::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceOption::default())
    }
}

impl Default for Box<dyn traits::VirtualControllerOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualControllerOption::default())
    }
}

impl Default for Box<dyn traits::VirtualSataControllerOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualSataControllerOption::default())
    }
}

impl Default for Box<dyn traits::VirtualScsiControllerOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualScsiControllerOption::default())
    }
}

impl Default for Box<dyn traits::VirtualEthernetCardOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualEthernetCardOption::default())
    }
}

impl Default for Box<dyn traits::VirtualVmxnetOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualVmxnetOption::default())
    }
}

impl Default for Box<dyn traits::VirtualVmxnet3OptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualVmxnet3Option::default())
    }
}

impl Default for Box<dyn traits::VirtualSoundCardOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualSoundCardOption::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceBackingOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceBackingOption::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceDeviceBackingOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceDeviceBackingOption::default())
    }
}

impl Default for Box<dyn traits::VirtualDiskRawDiskVer2BackingOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDiskRawDiskVer2BackingOption::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceFileBackingOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceFileBackingOption::default())
    }
}

impl Default for Box<dyn traits::VirtualDevicePipeBackingOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDevicePipeBackingOption::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceRemoteDeviceBackingOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceRemoteDeviceBackingOption::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceUriBackingOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceUriBackingOption::default())
    }
}

impl Default for Box<dyn traits::VirtualPciPassthroughPluginBackingOptionTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualPciPassthroughPluginBackingOption::default())
    }
}

impl Default for Box<dyn traits::VirtualDeviceConfigSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VirtualDeviceConfigSpec::default())
    }
}

impl Default for Box<dyn traits::GuestAuthSubjectTrait> {
    fn default() -> Self {
        Box::new(structs::GuestAuthSubject::default())
    }
}

impl Default for Box<dyn traits::GuestFileAttributesTrait> {
    fn default() -> Self {
        Box::new(structs::GuestFileAttributes::default())
    }
}

impl Default for Box<dyn traits::GuestAuthenticationTrait> {
    fn default() -> Self {
        Box::new(structs::GuestAuthentication::default())
    }
}

impl Default for Box<dyn traits::GuestProgramSpecTrait> {
    fn default() -> Self {
        Box::new(structs::GuestProgramSpec::default())
    }
}

impl Default for Box<dyn traits::GuestRegValueDataSpecTrait> {
    fn default() -> Self {
        Box::new(structs::GuestRegValueDataSpec::default())
    }
}

impl Default for Box<dyn traits::FaultDomainIdTrait> {
    fn default() -> Self {
        Box::new(structs::FaultDomainId::default())
    }
}

impl Default for Box<dyn traits::VsanDataEfficiencyConfigTrait> {
    fn default() -> Self {
        Box::new(structs::VsanDataEfficiencyConfig::default())
    }
}

impl Default for Box<dyn traits::VsanDatastoreConfigTrait> {
    fn default() -> Self {
        Box::new(structs::VsanDatastoreConfig::default())
    }
}

impl Default for Box<dyn traits::VsanDatastoreSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VsanDatastoreSpec::default())
    }
}

impl Default for Box<dyn traits::VsanDirectoryServerConfigTrait> {
    fn default() -> Self {
        Box::new(structs::VsanDirectoryServerConfig::default())
    }
}

impl Default for Box<dyn traits::EntityResourceCheckDetailsTrait> {
    fn default() -> Self {
        Box::new(structs::EntityResourceCheckDetails::default())
    }
}

impl Default for Box<dyn traits::VsanDiskResourceCheckResultTrait> {
    fn default() -> Self {
        Box::new(structs::VsanDiskResourceCheckResult::default())
    }
}

impl Default for Box<dyn traits::VsanResourceCheckResultTrait> {
    fn default() -> Self {
        Box::new(structs::VsanResourceCheckResult::default())
    }
}

impl Default for Box<dyn traits::VsanResourceCheckComponentResultTrait> {
    fn default() -> Self {
        Box::new(structs::VsanResourceCheckComponentResult::default())
    }
}

impl Default for Box<dyn traits::VsanMountPrecheckItemTrait> {
    fn default() -> Self {
        Box::new(structs::VsanMountPrecheckItem::default())
    }
}

impl Default for Box<dyn traits::VsanMountPrecheckResultTrait> {
    fn default() -> Self {
        Box::new(structs::VsanMountPrecheckResult::default())
    }
}

impl Default for Box<dyn traits::VsanRemoteVcInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VsanRemoteVcInfo::default())
    }
}

impl Default for Box<dyn traits::VsanResourceCheckTaskDetailsTrait> {
    fn default() -> Self {
        Box::new(structs::VsanResourceCheckTaskDetails::default())
    }
}

impl Default for Box<dyn traits::VsanIscsiVipConfigSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VsanIscsiVipConfigSpec::default())
    }
}

impl Default for Box<dyn traits::VsanConfigBaseIssueTrait> {
    fn default() -> Self {
        Box::new(structs::VsanConfigBaseIssue::default())
    }
}

impl Default for Box<dyn traits::VsanNetworkConfigBaseIssueTrait> {
    fn default() -> Self {
        Box::new(structs::VsanNetworkConfigBaseIssue::default())
    }
}

impl Default for Box<dyn traits::VsanClusterConfigInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VsanClusterConfigInfo::default())
    }
}

impl Default for Box<dyn traits::VsanHostConfigInfoTrait> {
    fn default() -> Self {
        Box::new(structs::VsanHostConfigInfo::default())
    }
}

impl Default for Box<dyn traits::VsanHostConfigInfoNetworkInfoPortConfigTrait> {
    fn default() -> Self {
        Box::new(structs::VsanHostConfigInfoNetworkInfoPortConfig::default())
    }
}

impl Default for Box<dyn traits::VsanHostDiskResultTrait> {
    fn default() -> Self {
        Box::new(structs::VsanHostDiskResult::default())
    }
}

impl Default for Box<dyn traits::VsanHostIpConfigTrait> {
    fn default() -> Self {
        Box::new(structs::VsanHostIpConfig::default())
    }
}

impl Default for Box<dyn traits::BaseConfigInfoTrait> {
    fn default() -> Self {
        Box::new(structs::BaseConfigInfo::default())
    }
}

impl Default for Box<dyn traits::BaseConfigInfoBackingInfoTrait> {
    fn default() -> Self {
        Box::new(structs::BaseConfigInfoBackingInfo::default())
    }
}

impl Default for Box<dyn traits::BaseConfigInfoFileBackingInfoTrait> {
    fn default() -> Self {
        Box::new(structs::BaseConfigInfoFileBackingInfo::default())
    }
}

impl Default for Box<dyn traits::VslmCreateSpecBackingSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VslmCreateSpecBackingSpec::default())
    }
}

impl Default for Box<dyn traits::VslmMigrateSpecTrait> {
    fn default() -> Self {
        Box::new(structs::VslmMigrateSpec::default())
    }
}

impl Default for Box<dyn traits::SelectionSpecTrait> {
    fn default() -> Self {
        Box::new(structs::SelectionSpec::default())
    }
}

impl Default for Box<dyn traits::VslmTaskReasonTrait> {
    fn default() -> Self {
        Box::new(structs::VslmTaskReason::default())
    }
}

